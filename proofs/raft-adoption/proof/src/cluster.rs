use std::collections::{BTreeMap, BTreeSet, VecDeque};

use glade_raft_adoption_api::{Action, Command, CommittedMachine, Receipt, RequestId, Resource};
use raft::eraftpb::{Entry, EntryType, Message};
use raft::storage::MemStorage;
use raft::{Config, RawNode, StateRole};

use crate::application::{Application, Readiness};
use crate::codec;

/// Proposing reports neither application acceptance nor terminal noncommit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proposal {
    Unknown,
}

struct Voter {
    raft: RawNode<MemStorage>,
    application: Application,
}

impl Voter {
    fn apply(&mut self, entries: Vec<Entry>) {
        for entry in entries {
            assert_eq!(
                entry.get_entry_type(),
                EntryType::EntryNormal,
                "configuration changes are outside Q1a"
            );
            let (command, readiness) = if entry.data.is_empty() {
                (None, None)
            } else {
                let (command, readiness) =
                    codec::decode(&entry.data).expect("invalid committed fixture encoding");
                (Some(command), readiness)
            };
            self.application
                .apply_committed(entry.index, command, readiness)
                .expect("committed application ordering violation");
        }
    }

    fn ready(&mut self) -> Vec<Message> {
        let mut outgoing = Vec::new();
        while self.raft.has_ready() {
            let mut ready = self.raft.ready();
            assert!(ready.snapshot().is_empty(), "snapshots are outside Q1a");
            assert!(
                ready.read_states().is_empty(),
                "read barriers are outside Q1a"
            );
            // Release both classes only after entries and HardState are readable
            // under the memory-only persistence profile. No fsync is claimed.
            let entries = ready.take_entries();
            self.raft
                .mut_store()
                .wl()
                .append(&entries)
                .expect("memory append failed");
            if let Some(hard_state) = ready.hs() {
                self.raft.mut_store().wl().set_hardstate(hard_state.clone());
            }
            outgoing.extend(ready.take_messages());
            outgoing.extend(ready.take_persisted_messages());
            self.apply(ready.take_committed_entries());
            let mut light = self.raft.advance(ready);
            if let Some(commit) = light.commit_index() {
                // commit_to rewrites term from the committed entry. A LightReady
                // commit-only update MUST instead preserve current term/vote.
                self.raft.mut_store().wl().mut_hard_state().commit = commit;
            }
            self.apply(light.take_committed_entries());
            outgoing.extend(light.take_messages());
            self.raft.advance_apply_to(self.application.applied());
        }
        outgoing
    }
}

/// Explicit FIFO schedules over fixed, complete-data logical voters in memory.
/// Campaigns and message-loss/healing are manual; no timer ticks or sleeps.
pub struct Cluster {
    voters: BTreeMap<u64, Voter>,
    messages: VecDeque<Message>,
    isolated: BTreeSet<u64>,
    readiness: Vec<(Command, Readiness)>,
}

impl Cluster {
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
        self.voters
            .get_mut(&voter)
            .expect("unknown fixed voter")
            .raft
            .campaign()
            .expect("campaign failed");
    }

    pub fn drain(&mut self) {
        for _ in 0..100_000 {
            let mut progressed = false;
            for voter in self.voters.values_mut() {
                if voter.raft.has_ready() {
                    self.messages.extend(voter.ready());
                    progressed = true;
                }
            }
            if let Some(message) = self.messages.pop_front() {
                progressed = true;
                if !self.isolated.contains(&message.from) && !self.isolated.contains(&message.to) {
                    self.voters
                        .get_mut(&message.to)
                        .expect("message outside bound configuration")
                        .raft
                        .step(message)
                        .expect("Raft message rejected");
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
            .filter(|(_, voter)| voter.raft.raft.state == StateRole::Leader)
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
        let application = &self.voters.get(&voter)?.application;
        if !application.may_disclose(command.request) {
            return None;
        }
        application.reply(command)
    }

    /// Original retained outcome with current local disclosure permission.
    /// Local policy may be stale; unseen remote revocation is not qualified.
    pub fn outcome(&self, voter: u64, request: RequestId) -> Option<Receipt> {
        let application = &self.voters.get(&voter)?.application;
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
