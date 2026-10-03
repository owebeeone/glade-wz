use glade_raft_disk::{FaultPoint, v2::V2DiskStore};
use glade_raft_q3_api::{CheckpointStore, Error, Image, Instance, conformance};
use std::path::PathBuf;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let name = std::thread::current()
            .name()
            .unwrap_or("v2")
            .replace("::", "-");
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("v2-{}-{name}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn path(&self) -> PathBuf {
        self.0.join("node-1")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
fn instance() -> Instance {
    Instance {
        scope: 7,
        group: 70,
        node: 1,
        application_profile: 2,
    }
}
fn genesis() -> Image {
    Image {
        term: 0,
        vote: 0,
        commit: 0,
        applied: 0,
        configuration: conformance::stable(),
        checkpoint: None,
        suffix: Vec::new(),
    }
}

#[test]
fn explicit_create_sync_reopen_and_exact_binding() {
    let fixture = Fixture::new();
    let mut store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
    let saved = store.publish(0, conformance::opaque_image()).unwrap();
    drop(store);
    let mut reopened =
        V2DiskStore::open(&fixture.path(), instance(), Some(saved.revision)).unwrap();
    assert_eq!(reopened.load(), Ok(saved));
}

#[test]
fn open_missing_never_creates() {
    let fixture = Fixture::new();
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), instance(), None),
        Err(Error::Missing)
    ));
    assert!(!fixture.path().exists());
}

#[test]
fn empty_old_format_and_wrong_instance_never_reset() {
    let fixture = Fixture::new();
    std::fs::write(fixture.path(), []).unwrap();
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), instance(), None),
        Err(Error::Quarantined)
    ));
    assert_eq!(std::fs::read(fixture.path()).unwrap(), Vec::<u8>::new());
    let old = b"GLADEQ2 old unsupported profile";
    std::fs::write(fixture.path(), old).unwrap();
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), instance(), None),
        Err(Error::IncompatibleVersion)
    ));
    assert_eq!(std::fs::read(fixture.path()).unwrap(), old);
    std::fs::remove_file(fixture.path()).unwrap();
    drop(V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap());
    let before = std::fs::read(fixture.path()).unwrap();
    let mut foreign = instance();
    foreign.group = 71;
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), foreign, None),
        Err(Error::WrongBinding)
    ));
    assert_eq!(std::fs::read(fixture.path()).unwrap(), before);
}

#[test]
fn held_lock_and_exclusive_create_prevent_overwrite() {
    let fixture = Fixture::new();
    let store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), instance(), None),
        Err(Error::Locked)
    ));
    assert!(matches!(
        V2DiskStore::create_new(&fixture.path(), instance(), genesis()),
        Err(Error::AlreadyExists)
    ));
    drop(store);
    assert!(V2DiskStore::open(&fixture.path(), instance(), None).is_ok());
}

#[test]
fn invalid_instance_is_refused_before_file_creation() {
    let fixture = Fixture::new();
    let mut invalid = instance();
    invalid.node = 5;
    assert!(matches!(
        V2DiskStore::create_new(&fixture.path(), invalid, genesis()),
        Err(Error::WrongBinding)
    ));
    assert!(!fixture.path().exists());
}

#[test]
fn trusted_floor_detects_valid_old_journal_without_claiming_intrinsic_detection() {
    let fixture = Fixture::new();
    let mut store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
    let old = std::fs::read(fixture.path()).unwrap();
    let latest = store.publish(0, conformance::opaque_image()).unwrap();
    drop(store);
    std::fs::write(fixture.path(), old).unwrap();
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), instance(), Some(latest.revision)),
        Err(Error::Quarantined)
    ));
    let mut without_floor = V2DiskStore::open(&fixture.path(), instance(), None).unwrap();
    assert_eq!(without_floor.load().unwrap().revision, 0);
}

#[test]
fn all_publication_faults_preserve_full_or_unknown_quarantined_history() {
    for (offset, fault) in [
        FaultPoint::BeforeWrite,
        FaultPoint::PartialWrite,
        FaultPoint::AfterWrite,
        FaultPoint::BeforeSync,
        FaultPoint::AfterSync,
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = Fixture::new();
        let path = fixture.0.join(format!("node-{offset}"));
        let mut store = V2DiskStore::create_new(&path, instance(), genesis()).unwrap();
        let before = store.load().unwrap();
        store.inject_once(fault);
        assert_eq!(
            store.publish(before.revision, conformance::opaque_image()),
            Err(Error::IoUnknown)
        );
        if fault == FaultPoint::BeforeWrite {
            assert_eq!(store.load(), Ok(before.clone()));
        } else {
            assert_eq!(store.load(), Err(Error::Poisoned));
        }
        drop(store);
        if fault == FaultPoint::PartialWrite {
            assert!(matches!(
                V2DiskStore::open(&path, instance(), None),
                Err(Error::Quarantined)
            ));
        } else {
            let mut reopened = V2DiskStore::open(&path, instance(), None).unwrap();
            let recovered = reopened.load().unwrap();
            if fault == FaultPoint::BeforeWrite {
                assert_eq!(recovered, before);
            } else {
                assert_eq!(recovered.image, conformance::opaque_image());
                assert_eq!(recovered.revision, 1);
            }
        }
    }
}

#[test]
fn whole_frame_capacity_refusal_preserves_prior_usable_store() {
    let fixture = Fixture::new();
    let mut store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
    let prior = store.load().unwrap();
    let mut image = conformance::compacted();
    image.checkpoint.as_mut().unwrap().application = vec![0; 16 * 1024 * 1024];
    assert_eq!(store.publish(0, image), Err(Error::CapacityExhausted));
    assert_eq!(store.load(), Ok(prior));
}

#[test]
fn a_new_checkpoint_cannot_regress_the_compacted_term() {
    let fixture = Fixture::new();
    let mut store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
    let saved = store.publish(0, conformance::compacted()).unwrap();
    let mut next = saved.image.clone();
    let checkpoint = next.checkpoint.as_mut().unwrap();
    checkpoint.index = 3;
    checkpoint.term = 1;
    next.commit = 3;
    next.applied = 3;
    assert_eq!(
        store.publish(saved.revision, next),
        Err(Error::InvalidImage)
    );
    assert_eq!(store.load(), Ok(saved));
}

#[test]
fn real_q2_file_is_incompatible_and_preserved_byte_for_byte() {
    let fixture = Fixture::new();
    let q2 = glade_raft_disk::DiskStore::create_new(
        &fixture.path(),
        glade_raft_durability_api::Binding {
            scope: 7,
            node: 1,
            voters: vec![1, 2, 3],
            application_profile: 1,
        },
    )
    .unwrap();
    drop(q2);
    let bytes = std::fs::read(fixture.path()).unwrap();
    assert!(matches!(
        V2DiskStore::open(&fixture.path(), instance(), None),
        Err(Error::IncompatibleVersion)
    ));
    assert_eq!(std::fs::read(fixture.path()).unwrap(), bytes);
}

#[test]
fn an_older_applied_checkpoint_retains_a_later_term_committed_suffix() {
    let fixture = Fixture::new();
    let mut store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
    let full = conformance::opaque_image();
    let previous = store.publish(0, full.clone()).unwrap();
    let mut next = full.clone();
    next.checkpoint = Some(glade_raft_q3_api::Checkpoint {
        binding: glade_raft_q3_api::GroupIdentity {
            scope: 7,
            group: 70,
            application_profile: 2,
        },
        index: 1,
        term: 1,
        configuration: conformance::stable(),
        application: vec![1, 2, 3],
    });
    next.suffix.remove(0);
    let accepted = store
        .publish(previous.revision, next.clone())
        .expect("known term-1 applied cut plus original committed term-2 suffix");
    assert_eq!(accepted.image, next);
    drop(store);
    let mut reopened =
        V2DiskStore::open(&fixture.path(), instance(), Some(accepted.revision)).unwrap();
    assert_eq!(reopened.load(), Ok(accepted));
}

#[test]
fn all_publication_faults_keep_checkpoint_configuration_and_suffix_in_one_cut() {
    use glade_raft_q3_api::StoredEntry;
    for fault in [
        FaultPoint::BeforeWrite,
        FaultPoint::PartialWrite,
        FaultPoint::AfterWrite,
        FaultPoint::BeforeSync,
        FaultPoint::AfterSync,
    ] {
        let fixture = Fixture::new();
        let mut store = V2DiskStore::create_new(&fixture.path(), instance(), genesis()).unwrap();
        let mut prior = conformance::compacted();
        prior.checkpoint.as_mut().unwrap().index = 1;
        prior.checkpoint.as_mut().unwrap().term = 1;
        prior.suffix = vec![StoredEntry {
            index: 2,
            term: 2,
            bytes: vec![22],
        }];
        prior.configuration.voters = vec![1, 2, 4];
        prior.configuration.voters_outgoing = vec![1, 2, 3];
        prior.configuration.index = 2;
        let original = store.publish(0, prior).unwrap();
        let mut next = original.image.clone();
        next.term = 3;
        next.vote = 4;
        next.commit = 3;
        next.applied = 3;
        next.checkpoint.as_mut().unwrap().index = 2;
        next.checkpoint.as_mut().unwrap().term = 2;
        next.checkpoint.as_mut().unwrap().configuration = original.image.configuration.clone();
        next.checkpoint.as_mut().unwrap().application.push(33);
        next.configuration.voters_outgoing.clear();
        next.configuration.index = 3;
        next.suffix = vec![StoredEntry {
            index: 3,
            term: 3,
            bytes: vec![44],
        }];
        store.inject_once(fault);
        assert_eq!(
            store.publish(original.revision, next.clone()),
            Err(Error::IoUnknown)
        );
        assert_eq!(
            store.load(),
            if fault == FaultPoint::BeforeWrite {
                Ok(original.clone())
            } else {
                Err(Error::Poisoned)
            }
        );
        drop(store);
        match V2DiskStore::open(&fixture.path(), instance(), None) {
            Ok(mut reopened) => {
                let recovered = reopened.load().unwrap();
                assert_eq!(
                    recovered.image,
                    if fault == FaultPoint::BeforeWrite {
                        original.image
                    } else {
                        next
                    }
                );
            }
            Err(error) => {
                assert_eq!(fault, FaultPoint::PartialWrite);
                assert_eq!(error, Error::Quarantined);
            }
        }
    }
}
