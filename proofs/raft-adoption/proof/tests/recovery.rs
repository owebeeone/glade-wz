//! Q2 specifications: injected stores, actual RawNode, full history replay.
use std::collections::BTreeMap;
use std::path::PathBuf;

use glade_raft_adoption_api::{Action, Command, Outcome, Receipt, Rejection, RequestId};
use glade_raft_adoption_proof::Cluster;
use glade_raft_disk::DiskStore;
use glade_raft_durability_api::{
    Binding, DurableImage, DurableStore, StoreError, StoredEntry, StoredState,
};

fn binding(node: u64) -> Binding {
    Binding {
        scope: 7,
        node,
        voters: vec![1, 2, 3],
        application_profile: 1,
    }
}

fn request(principal: u64, sequence: u64) -> RequestId {
    RequestId {
        scope: 7,
        resource: 100,
        incarnation: 1,
        principal,
        sequence,
    }
}

fn create() -> Command {
    Command {
        request: request(1, 1),
        generation: 0,
        home: 0,
        policy_frontier: 0,
        action: Action::Create {
            name: 40,
            home: 1,
            payload: 11,
        },
    }
}

fn mutate(sequence: u64, payload: u64) -> Command {
    Command {
        request: request(10, sequence),
        generation: 1,
        home: 1,
        policy_frontier: 0,
        action: Action::Mutate { payload },
    }
}

fn apply(cluster: &mut Cluster, voter: u64, command: Command) -> Receipt {
    cluster.propose(voter, command);
    cluster.drain();
    cluster
        .reply(voter, command)
        .expect("Q2: applied retained receipt")
}

struct Directory(PathBuf);
impl Directory {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("glade-q2-recovery-{}-{name}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn stores(&self, create: bool) -> BTreeMap<u64, Box<dyn DurableStore>> {
        (1..=3)
            .map(|node| {
                let path = self.0.join(format!("voter-{node}"));
                let store = if create {
                    DiskStore::create_new(&path, binding(node))
                } else {
                    DiskStore::open(&path, binding(node), None)
                }
                .unwrap();
                (node, Box::new(store) as Box<dyn DurableStore>)
            })
            .collect()
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

// A loaded fixture is a compiler/consumer witness, not evidence of disk retention.
struct Loaded(StoredState);
impl DurableStore for Loaded {
    fn load(&mut self) -> Result<StoredState, StoreError> {
        Ok(self.0.clone())
    }
    fn persist(
        &mut self,
        expected_revision: u64,
        image: DurableImage,
    ) -> Result<StoredState, StoreError> {
        if expected_revision != self.0.revision {
            return Err(StoreError::RevisionConflict {
                expected: expected_revision,
                actual: self.0.revision,
            });
        }
        self.0.revision += 1;
        self.0.image = image;
        Ok(self.0.clone())
    }
}
fn loaded_stores() -> BTreeMap<u64, Box<dyn DurableStore>> {
    (1..=3)
        .map(|node| {
            (
                node,
                Box::new(Loaded(StoredState {
                    revision: 0,
                    binding: binding(node),
                    image: DurableImage {
                        term: 0,
                        vote: 0,
                        commit: 0,
                        entries: vec![],
                    },
                })) as Box<dyn DurableStore>,
            )
        })
        .collect()
}

#[test]
fn q2_injected_consumer_recovers_fixed_empty_genesis() {
    let mut cluster =
        Cluster::recover(&[1, 2, 3], loaded_stores()).expect("reviewed port accepts valid genesis");
    cluster.campaign(1);
    cluster.drain();
    assert!(matches!(
        apply(&mut cluster, 1, create()).outcome,
        Outcome::Accepted(_)
    ));
}

#[test]
fn q2_recovery_refuses_bad_bytes_binding_and_incomplete_history() {
    for case in 0..3 {
        let mut stores = loaded_stores();
        let mut state = stores.get_mut(&2).unwrap().load().unwrap();
        match case {
            0 => {
                state.image = DurableImage {
                    term: 1,
                    vote: 2,
                    commit: 1,
                    entries: vec![StoredEntry {
                        index: 1,
                        term: 1,
                        bytes: vec![255],
                    }],
                };
            }
            1 => {
                state.binding.scope = 8;
            }
            _ => {
                state.image.commit = 1;
            }
        }
        stores.insert(2, Box::new(Loaded(state)));
        assert!(Cluster::recover(&[1, 2, 3], stores).is_err());
    }
}

#[test]
fn q2_disk_replay_recovers_receipts_move_fences_and_changed_retry() {
    let dir = Directory::new("move");
    let mut cluster = Cluster::recover(&[1, 2, 3], dir.stores(true)).unwrap();
    cluster.campaign(1);
    cluster.drain();
    apply(&mut cluster, 1, create());
    let original_command = mutate(1, 23);
    let original = apply(&mut cluster, 1, original_command);
    let moved = cluster.move_command(request(1, 2), 1, 1, 2);
    let movement = apply(&mut cluster, 1, moved);
    assert!(matches!(movement.outcome, Outcome::Accepted(resource) if resource.generation == 2));
    drop(cluster);
    let mut recovered = Cluster::recover(&[1, 2, 3], dir.stores(false)).unwrap();
    for voter in 1..=3 {
        assert_eq!(recovered.outcome(voter, original.request), Some(original));
        assert_eq!(recovered.outcome(voter, moved.request), Some(movement));
        let resource = recovered.resource(voter, 100).unwrap();
        assert_eq!(
            (resource.home, resource.generation, resource.payload),
            (2, 2, 23)
        );
    }
    recovered.campaign(2);
    recovered.drain();
    assert_eq!(apply(&mut recovered, 2, original_command), original);
    assert_eq!(
        apply(&mut recovered, 2, mutate(1, 99)).outcome,
        Outcome::Rejected(Rejection::RetryConflict)
    );
    assert_eq!(
        apply(&mut recovered, 2, mutate(2, 99)).outcome,
        Outcome::Rejected(Rejection::StaleGeneration)
    );
}

#[test]
fn q2_disk_replay_recovers_policy_tombstone_and_exact_create_outcome() {
    let dir = Directory::new("policy-retire");
    let mut cluster = Cluster::recover(&[1, 2, 3], dir.stores(true)).unwrap();
    cluster.campaign(1);
    cluster.drain();
    let original_create = apply(&mut cluster, 1, create());
    let original = apply(&mut cluster, 1, mutate(1, 23));
    let policy = Command {
        request: request(1, 2),
        generation: 1,
        home: 1,
        policy_frontier: 0,
        action: Action::SetPermission {
            principal: 10,
            write: false,
            disclose: false,
        },
    };
    let frontier = apply(&mut cluster, 1, policy).index;
    let retire = Command {
        request: request(1, 3),
        policy_frontier: frontier,
        action: Action::Retire,
        ..policy
    };
    apply(&mut cluster, 1, retire);
    drop(cluster);
    let mut recovered = Cluster::recover(&[1, 2, 3], dir.stores(false)).unwrap();
    assert!(recovered.resource(2, 100).unwrap().retired);
    assert!(recovered.outcome(2, original.request).is_none());
    recovered.campaign(2);
    recovered.drain();
    assert_eq!(apply(&mut recovered, 2, create()), original_create);
    let new = Command {
        request: request(1, 4),
        policy_frontier: frontier,
        action: Action::Mutate { payload: 99 },
        ..retire
    };
    assert_eq!(
        apply(&mut recovered, 2, new).outcome,
        Outcome::Rejected(Rejection::Retired)
    );
}

#[test]
fn q2_disk_reopen_preserves_uncommitted_suffix_until_real_raft_overwrite() {
    let dir = Directory::new("suffix");
    let mut cluster = Cluster::recover(&[1, 2, 3], dir.stores(true)).unwrap();
    cluster.campaign(1);
    cluster.drain();
    apply(&mut cluster, 1, create());
    cluster.isolate(1);
    let old = mutate(1, 99);
    cluster.propose(1, old);
    cluster.drain();
    assert!(cluster.outcome(1, old.request).is_none());
    drop(cluster);
    let mut recovered = Cluster::recover(&[1, 2, 3], dir.stores(false)).unwrap();
    recovered.isolate(1);
    recovered.campaign(2);
    recovered.drain();
    apply(&mut recovered, 2, mutate(2, 23));
    recovered.heal();
    recovered.drain();
    assert!(recovered.outcome(1, old.request).is_none());
    assert_eq!(recovered.resource(1, 100).unwrap().payload, 23);
}

struct SwitchStore {
    inner: DiskStore,
    fail: std::rc::Rc<std::cell::Cell<bool>>,
    poisoned: bool,
}
impl DurableStore for SwitchStore {
    fn load(&mut self) -> Result<StoredState, StoreError> {
        if self.poisoned {
            return Err(StoreError::Poisoned);
        }
        self.inner.load()
    }
    fn persist(&mut self, revision: u64, image: DurableImage) -> Result<StoredState, StoreError> {
        if self.poisoned {
            return Err(StoreError::Poisoned);
        }
        if self.fail.get() {
            self.poisoned = true;
            return Err(StoreError::Io);
        }
        self.inner.persist(revision, image)
    }
}

#[test]
fn q2_leader_storage_failure_releases_no_new_application_or_message() {
    let dir = Directory::new("leader-fail");
    let mut stores = dir.stores(true);
    // Replace exactly one store after relinquishing its exclusive handle.
    stores.remove(&1);
    let fail = std::rc::Rc::new(std::cell::Cell::new(false));
    stores.insert(
        1,
        Box::new(SwitchStore {
            inner: DiskStore::open(&dir.0.join("voter-1"), binding(1), None).unwrap(),
            fail: fail.clone(),
            poisoned: false,
        }),
    );
    let mut cluster = Cluster::recover(&[1, 2, 3], stores).unwrap();
    cluster.campaign(1);
    cluster.drain();
    apply(&mut cluster, 1, create());
    let cut = cluster.applied(1);
    fail.set(true);
    let command = mutate(1, 99);
    cluster.propose(1, command);
    cluster.drain();
    assert_eq!(cluster.failure(1), Some(StoreError::Io));
    assert_eq!(cluster.applied(1), cut);
    for voter in 1..=3 {
        assert!(cluster.outcome(voter, command.request).is_none());
    }
    assert!(
        cluster.reply(1, create()).is_none(),
        "failed node stops all serving"
    );
}

#[test]
fn q2_follower_storage_failure_cannot_supply_a_data_quorum_ack() {
    let dir = Directory::new("follower-fail");
    let mut stores = dir.stores(true);
    stores.remove(&2);
    let fail = std::rc::Rc::new(std::cell::Cell::new(false));
    stores.insert(
        2,
        Box::new(SwitchStore {
            inner: DiskStore::open(&dir.0.join("voter-2"), binding(2), None).unwrap(),
            fail: fail.clone(),
            poisoned: false,
        }),
    );
    let mut cluster = Cluster::recover(&[1, 2, 3], stores).unwrap();
    cluster.campaign(1);
    cluster.drain();
    apply(&mut cluster, 1, create());
    cluster.isolate(3);
    fail.set(true);
    let command = mutate(1, 99);
    cluster.propose(1, command);
    cluster.drain();
    assert_eq!(cluster.failure(2), Some(StoreError::Io));
    assert!(cluster.outcome(1, command.request).is_none());
    assert_eq!(cluster.resource(1, 100).unwrap().payload, 11);
}

#[test]
fn q2_recovery_rejects_conflicting_committed_voter_prefixes() {
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::Entry;
    let mut stores = loaded_stores();
    for node in [1, 2] {
        let term = node;
        let entry = Entry {
            index: 1,
            term,
            ..Entry::default()
        };
        let state = StoredState {
            revision: 1,
            binding: binding(node),
            image: DurableImage {
                term,
                vote: node,
                commit: 1,
                entries: vec![StoredEntry {
                    index: 1,
                    term,
                    bytes: entry.write_to_bytes().unwrap(),
                }],
            },
        };
        stores.insert(node, Box::new(Loaded(state)));
    }
    assert!(matches!(
        Cluster::recover(&[1, 2, 3], stores),
        Err(StoreError::Quarantined)
    ));
}

#[test]
fn q2_recovery_closes_protobuf_and_private_command_grammar() {
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::{Entry, EntryType};
    for case in 0..5 {
        let mut stores = loaded_stores();
        let mut entry = Entry {
            index: 1,
            term: 1,
            ..Entry::default()
        };
        match case {
            0 => {
                entry.data = vec![255].into();
            }
            1 => {
                entry.index = 2;
            }
            2 => {
                entry.set_entry_type(EntryType::EntryConfChange);
            }
            3 => {
                entry.context = vec![1].into();
            }
            _ => {
                entry.sync_log = true;
            }
        }
        let state = StoredState {
            revision: 1,
            binding: binding(2),
            image: DurableImage {
                term: 1,
                vote: 2,
                commit: 0,
                entries: vec![StoredEntry {
                    index: 1,
                    term: 1,
                    bytes: entry.write_to_bytes().unwrap(),
                }],
            },
        };
        stores.insert(2, Box::new(Loaded(state)));
        assert!(matches!(
            Cluster::recover(&[1, 2, 3], stores),
            Err(StoreError::Quarantined)
        ));
    }
}

#[test]
fn q2_exhausted_raft_term_is_refused_before_a_wrapping_campaign() {
    let mut stores = loaded_stores();
    let mut state = stores.get_mut(&1).unwrap().load().unwrap();
    state.image.term = u64::MAX;
    stores.insert(1, Box::new(Loaded(state)));
    assert!(matches!(
        Cluster::recover(&[1, 2, 3], stores),
        Err(StoreError::CapacityExhausted)
    ));
}

#[test]
fn q2_actual_disk_valid_frame_with_malformed_command_never_starts_a_voter() {
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::Entry;
    for commit in [0, 1] {
        let dir = Directory::new(&format!("invalid-command-{commit}"));
        let mut stores = dir.stores(true);
        let entry = Entry {
            index: 1,
            term: 1,
            data: vec![255].into(),
            ..Entry::default()
        };
        stores
            .get_mut(&2)
            .unwrap()
            .persist(
                0,
                DurableImage {
                    term: 1,
                    vote: 2,
                    commit,
                    entries: vec![StoredEntry {
                        index: 1,
                        term: 1,
                        bytes: entry.write_to_bytes().unwrap(),
                    }],
                },
            )
            .unwrap();
        drop(stores);
        assert!(matches!(
            Cluster::recover(&[1, 2, 3], dir.stores(false)),
            Err(StoreError::Quarantined)
        ));
    }
}

#[test]
fn q2_actual_after_sync_error_is_unknown_and_can_later_commit_without_losing_old_receipt() {
    use glade_raft_disk::FaultPoint;
    struct Armed {
        inner: Option<DiskStore>,
        fault: std::rc::Rc<std::cell::Cell<Option<FaultPoint>>>,
    }
    impl DurableStore for Armed {
        fn load(&mut self) -> Result<StoredState, StoreError> {
            self.inner.as_mut().unwrap().load()
        }
        fn persist(
            &mut self,
            revision: u64,
            image: DurableImage,
        ) -> Result<StoredState, StoreError> {
            let mut inner = self.inner.take().unwrap();
            if let Some(fault) = self.fault.take() {
                inner = inner.with_fault(fault);
            }
            let result = inner.persist(revision, image);
            self.inner = Some(inner);
            result
        }
    }
    let dir = Directory::new("unknown-after-sync");
    let mut stores = dir.stores(true);
    stores.remove(&1);
    let fault = std::rc::Rc::new(std::cell::Cell::new(None));
    stores.insert(
        1,
        Box::new(Armed {
            inner: Some(DiskStore::open(&dir.0.join("voter-1"), binding(1), None).unwrap()),
            fault: fault.clone(),
        }),
    );
    let mut cluster = Cluster::recover(&[1, 2, 3], stores).unwrap();
    cluster.campaign(1);
    cluster.drain();
    apply(&mut cluster, 1, create());
    let original = apply(&mut cluster, 1, mutate(1, 23));
    fault.set(Some(FaultPoint::AfterSync));
    let unknown = mutate(2, 99);
    cluster.propose(1, unknown);
    cluster.drain();
    assert_eq!(cluster.failure(1), Some(StoreError::Io));
    assert!(cluster.reply(1, unknown).is_none());
    assert_eq!(cluster.resource(1, 100).unwrap().payload, 23);
    drop(cluster);
    let mut recovered = Cluster::recover(&[1, 2, 3], dir.stores(false)).unwrap();
    assert_eq!(recovered.outcome(1, original.request), Some(original));
    assert!(recovered.outcome(1, unknown.request).is_none());
    recovered.campaign(1);
    recovered.drain();
    let settled = recovered
        .outcome(1, unknown.request)
        .expect("unknown persisted suffix can later commit");
    assert!(matches!(settled.outcome, Outcome::Accepted(resource) if resource.payload == 99));
    assert_eq!(recovered.outcome(1, original.request), Some(original));
    assert_eq!(apply(&mut recovered, 1, unknown), settled);
}
