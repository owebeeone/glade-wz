use std::collections::{BTreeMap, BTreeSet, VecDeque};

use glade_raft_adoption_api::{Action, Command, CommittedMachine, Receipt, RequestId, Resource};
use glade_raft_durability_api::{Binding, DurableStore, StoreError};
use raft::eraftpb::{HardState, Message};
use raft::storage::MemStorage;
use raft::{Config, RawNode, StateRole};

use crate::application::{Application, Readiness};
use crate::codec;
use crate::recovery;
use crate::voter::Voter;

/// Proposing reports neither application acceptance nor terminal noncommit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proposal {
    Unknown,
}

mod light_ready_tests;

mod tests {
    #[test]
    fn q2_campaign_refuses_exhausted_live_term() {
        use glade_raft_durability_api::StoreError;
        let mut cluster = super::Cluster::new(&[1, 2, 3]);
        cluster.voters.get_mut(&1).unwrap().raft.raft.term = u64::MAX;
        cluster.campaign(1);
        assert_eq!(cluster.failure(1), Some(StoreError::CapacityExhausted));
    }

    #[test]
    fn public_replay_of_driver_attested_move_recovers_original_receipt() {
        use super::Cluster;
        use glade_raft_adoption_api::{
            Action, ApplyError, Command, CommittedMachine, Outcome, Rejection, RequestId,
        };

        let request = |sequence| RequestId {
            scope: 7,
            resource: 100,
            incarnation: 1,
            principal: 1,
            sequence,
        };
        let mut cluster = Cluster::new(&[1, 2, 3]);
        cluster.campaign(1);
        cluster.drain();
        let create = Command {
            request: request(1),
            generation: 0,
            home: 0,
            policy_frontier: 0,
            action: Action::Create {
                name: 40,
                home: 1,
                payload: 11,
            },
        };
        cluster.propose(1, create);
        cluster.drain();
        let moved = cluster.move_command(request(2), 1, 1, 2);
        cluster.propose(1, moved);
        cluster.drain();
        let original = cluster.reply(1, moved).unwrap();
        assert!(
            matches!(original.outcome, Outcome::Accepted(resource) if resource.home == 2 && resource.generation == 2)
        );

        let machine: &mut dyn CommittedMachine =
            &mut cluster.voters.get_mut(&1).unwrap().application;
        assert_eq!(
            machine.apply(original.index, Some(moved)),
            Ok(Some(original))
        );
        let changed = Command {
            action: Action::Mutate { payload: 99 },
            ..moved
        };
        assert_eq!(
            machine.apply(original.index, Some(changed)),
            Err(ApplyError::ConflictingReplay {
                index: original.index
            })
        );
        let unwitnessed = Command {
            request: request(3),
            generation: 2,
            home: 2,
            policy_frontier: 0,
            action: Action::Move {
                home: 3,
                successor_applied: Some(original.index),
            },
        };
        let refused = machine
            .apply(original.index + 1, Some(unwitnessed))
            .unwrap()
            .unwrap();
        assert_eq!(
            refused.outcome,
            Outcome::Rejected(Rejection::IncompleteSuccessor)
        );
        assert_eq!(machine.lookup(moved.request), Some(original));
    }
}

/// Explicit FIFO schedules over fixed, complete-data logical voters.
/// Memory creation and injected persistent recovery are separate profiles.
/// Campaigns and message-loss/healing are manual; no timer ticks or sleeps.
pub struct Cluster {
    voters: BTreeMap<u64, Voter>,
    messages: VecDeque<Message>,
    isolated: BTreeSet<u64>,
    readiness: Vec<(Command, Readiness)>,
}

impl Cluster {
    /// Recover all configured stores before creating any participant.
    pub fn recover(
        voters: &[u64],
        mut stores: BTreeMap<u64, Box<dyn DurableStore>>,
    ) -> Result<Self, StoreError> {
        if !matches!(voters, [1, 2] | [1, 2, 3])
            || stores.keys().copied().collect::<Vec<_>>() != voters
        {
            return Err(StoreError::BindingMismatch);
        }
        let mut loaded = Vec::new();
        for id in voters {
            let mut store = stores.remove(id).ok_or(StoreError::Missing)?;
            let state = store.load()?;
            let expected = Binding {
                scope: 7,
                node: *id,
                voters: voters.to_vec(),
                application_profile: 1,
            };
            let entries = recovery::entries(&state, &expected)?;
            loaded.push((*id, store, state, entries));
        }
        for left in &loaded {
            for right in &loaded {
                let common = left.2.image.commit.min(right.2.image.commit) as usize;
                if left.2.image.entries[..common] != right.2.image.entries[..common] {
                    return Err(StoreError::Quarantined);
                }
            }
        }
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let mut nodes = BTreeMap::new();
        for (id, store, state, entries) in loaded {
            let application = recovery::application(voters, state.image.commit, &entries)?;
            let storage = MemStorage::new_with_conf_state((voters.to_vec(), Vec::<u64>::new()));
            storage
                .wl()
                .append(&entries)
                .map_err(|_| StoreError::Quarantined)?;
            let hard_state = HardState {
                term: state.image.term,
                vote: state.image.vote,
                commit: state.image.commit,
                ..HardState::default()
            };
            storage.wl().set_hardstate(hard_state);
            let config = Config {
                id,
                election_tick: 10,
                heartbeat_tick: 1,
                applied: state.image.commit,
                ..Config::default()
            };
            let raft =
                RawNode::new(&config, storage, &logger).map_err(|_| StoreError::Quarantined)?;
            nodes.insert(
                id,
                Voter {
                    raft,
                    application,
                    persistence: Some((store, state)),
                    failure: None,
                },
            );
        }
        Ok(Self {
            voters: nodes,
            messages: VecDeque::new(),
            isolated: BTreeSet::new(),
            readiness: Vec::new(),
        })
    }

    pub fn failure(&self, voter: u64) -> Option<StoreError> {
        self.voters.get(&voter)?.failure.clone()
    }

    pub fn new(voters: &[u64]) -> Self {
        assert!(
            matches!(voters, [1, 2] | [1, 2, 3]),
            "only reviewed fixed configurations are supported"
        );
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let nodes = voters
            .iter()
            .map(|id| {
                let config = Config {
                    id: *id,
                    election_tick: 10,
                    heartbeat_tick: 1,
                    ..Config::default()
                };
                let storage = MemStorage::new_with_conf_state((voters.to_vec(), Vec::<u64>::new()));
                let raft =
                    RawNode::new(&config, storage, &logger).expect("invalid fixed configuration");
                (
                    *id,
                    Voter {
                        raft,
                        application: Application::with_voters(voters),
                        persistence: None,
                        failure: None,
                    },
                )
            })
            .collect();
        Self {
            voters: nodes,
            messages: VecDeque::new(),
            isolated: BTreeSet::new(),
            readiness: Vec::new(),
        }
    }

    pub fn campaign(&mut self, voter: u64) {
        let voter = self.voters.get_mut(&voter).expect("unknown fixed voter");
        if voter.failure.is_none() {
            if voter.raft.raft.term >= u64::MAX - 1 {
                voter.failure = Some(StoreError::CapacityExhausted);
                return;
            }
            voter.raft.campaign().expect("campaign failed");
        }
    }

    pub fn drain(&mut self) {
        for _ in 0..100_000 {
            let mut progressed = false;
            for voter in self.voters.values_mut() {
                if voter.failure.is_none() && voter.raft.has_ready() {
                    match voter.ready() {
                        Ok(messages) => {
                            self.messages.extend(messages);
                        }
                        Err(error) => {
                            voter.failure = Some(error);
                        }
                    }
                    progressed = true;
                }
            }
            if let Some(message) = self.messages.pop_front() {
                progressed = true;
                if !self.isolated.contains(&message.from) && !self.isolated.contains(&message.to) {
                    let voter = self
                        .voters
                        .get_mut(&message.to)
                        .expect("message outside bound configuration");
                    if voter.failure.is_none() {
                        voter.raft.step(message).expect("Raft message rejected");
                    }
                }
            }
            if !progressed {
                return;
            }
        }
        panic!("bounded deterministic schedule failed to quiesce");
    }

    pub fn isolate(&mut self, voter: u64) {
        assert!(self.voters.contains_key(&voter), "unknown fixed voter");
        self.isolated.insert(voter);
    }

    pub fn heal(&mut self) {
        self.isolated.clear();
        if let Some(leader) = self.leader() {
            self.voters.get_mut(&leader).unwrap().raft.ping();
        }
    }

    /// Highest-term local leader observation, never a linearizable read barrier.
    pub fn leader(&self) -> Option<u64> {
        self.voters
            .iter()
            .filter(|(_, voter)| {
                voter.failure.is_none() && voter.raft.raft.state == StateRole::Leader
            })
            .max_by_key(|(_, voter)| voter.raft.raft.term)
            .map(|(id, _)| *id)
    }

    pub fn applied(&self, voter: u64) -> u64 {
        self.voters
            .get(&voter)
            .expect("unknown fixed voter")
            .application
            .applied()
    }

    pub fn propose(&mut self, voter: u64, command: Command) -> Proposal {
        if self
            .voters
            .get(&voter)
            .expect("unknown fixed voter")
            .failure
            .is_some()
        {
            return Proposal::Unknown;
        }
        let witness = self.readiness.iter().rev().find_map(|(minted, witness)| {
            if *minted == command {
                Some(*witness)
            } else {
                None
            }
        });
        // An admission error is not evidence of noncommit. Exact historical
        // outcomes remain recoverable before readiness/generation checks.
        let _ = self
            .voters
            .get_mut(&voter)
            .expect("unknown fixed voter")
            .raft
            .propose(Vec::new(), codec::encode(command, witness));
        Proposal::Unknown
    }

    /// Transient attempted-command response, protected by current disclosure.
    pub fn reply(&self, voter: u64, command: Command) -> Option<Receipt> {
        let node = self.voters.get(&voter)?;
        if node.failure.is_some() {
            return None;
        }
        let application = &node.application;
        if !application.may_disclose(command.request) {
            return None;
        }
        application.reply(command)
    }

    /// Original retained outcome with current local disclosure permission.
    /// Local policy may be stale; unseen remote revocation is not qualified.
    pub fn outcome(&self, voter: u64, request: RequestId) -> Option<Receipt> {
        let node = self.voters.get(&voter)?;
        if node.failure.is_some() {
            return None;
        }
        let application = &node.application;
        if !application.may_disclose(request) {
            return None;
        }
        application.lookup(request)
    }

    /// Trusted test observation, not a client read or disclosure endpoint.
    pub fn resource(&self, voter: u64, resource: u64) -> Option<Resource> {
        self.voters.get(&voter)?.application.resource(resource)
    }

    /// Captures verified complete successor application for atomic fixture Move.
    /// Ordered application rechecks against the actual preceding log cut.
    pub fn move_command(
        &mut self,
        request: RequestId,
        generation: u64,
        home: u64,
        successor: u64,
    ) -> Command {
        let frontier = self
            .voters
            .get(&successor)
            .filter(|voter| voter.failure.is_none())
            .map(|voter| voter.application.applied());
        let policy_frontier = self
            .leader()
            .map(|leader| self.voters[&leader].application.policy_frontier())
            .unwrap_or(0);
        let command = Command {
            request,
            generation,
            home,
            policy_frontier,
            action: Action::Move {
                home: successor,
                successor_applied: frontier,
            },
        };
        if let Some(frontier) = frontier {
            self.readiness.push((
                command,
                Readiness {
                    home: successor,
                    frontier,
                },
            ));
        }
        command
    }
}

#[test]
#[ignore = "Q2 real-disk terminal-term publication tier; execute explicitly"]
fn q2_real_disk_reserved_term_is_refused_before_publication() {
    use glade_raft_disk::DiskStore;
    use raft::eraftpb::{Message, MessageType};
    for voters in [vec![1, 2], vec![1, 2, 3]] {
        let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!(
                "q2-term-message-{}-{}",
                std::process::id(),
                voters.len()
            ));
        std::fs::create_dir(&directory).unwrap();
        let binding = |node| glade_raft_durability_api::Binding {
            scope: 7,
            node,
            voters: voters.clone(),
            application_profile: 1,
        };
        let stores = voters
            .iter()
            .map(|node| {
                (
                    *node,
                    Box::new(
                        DiskStore::create_new(
                            &directory.join(format!("node-{node}")),
                            binding(*node),
                        )
                        .unwrap(),
                    ) as Box<dyn DurableStore>,
                )
            })
            .collect();
        let mut cluster = Cluster::recover(&voters, stores).unwrap();
        let voter = cluster.voters.get_mut(&1).unwrap();
        let before = voter.persistence.as_ref().unwrap().1.clone();
        let mut incoming = Message::default();
        incoming.set_msg_type(MessageType::MsgHeartbeat);
        incoming.from = 2;
        incoming.to = 1;
        incoming.term = u64::MAX;
        voter.raft.step(incoming).unwrap();
        let result = voter.ready();
        // Test the real publication barrier, including terms learned from peers.
        assert_eq!(result, Err(StoreError::CapacityExhausted));
        assert_eq!(voter.persistence.as_ref().unwrap().1, before);
        drop(cluster);
        let stores = voters
            .iter()
            .map(|node| {
                (
                    *node,
                    Box::new(
                        DiskStore::open(
                            &directory.join(format!("node-{node}")),
                            binding(*node),
                            None,
                        )
                        .unwrap(),
                    ) as Box<dyn DurableStore>,
                )
            })
            .collect();
        let recovered = Cluster::recover(&voters, stores).unwrap();
        drop(recovered);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
