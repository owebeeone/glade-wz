use glade_raft_disk::{DiskStore, FaultPoint};
use glade_raft_durability_api::{Binding, DurableImage, DurableStore, StoreError, conformance};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

struct Fixture(PathBuf);

impl Fixture {
    fn new(label: &str) -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target/q2-fault-tests")
            .join(format!("{}-{label}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path).expect("remove previous test-owned fixture");
        }
        fs::create_dir_all(&path).expect("explicit test-owned parent");
        Self(path)
    }

    fn path(&self) -> PathBuf {
        self.0.join("state.journal")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove test-owned fixture");
    }
}

fn binding() -> Binding {
    Binding {
        scope: 7,
        node: 1,
        voters: vec![1, 2, 3],
        application_profile: 1,
    }
}

#[test]
fn shared_contract_runs_against_real_disk_and_reopens_exact_image() {
    let fixture = Fixture::new("shared");
    let mut store = DiskStore::create_new(&fixture.path(), binding()).expect("explicit genesis");
    conformance::exercise_store(&mut store);
    let expected = store.load().unwrap();
    drop(store);
    let mut reopened = DiskStore::open(&fixture.path(), binding(), Some(3)).expect("reopen");
    assert_eq!(reopened.load(), Ok(expected));
}

#[test]
fn existing_missing_empty_and_wrong_binding_never_initialize() {
    let fixture = Fixture::new("binding");
    assert!(matches!(
        DiskStore::open(&fixture.path(), binding(), None),
        Err(StoreError::Missing)
    ));
    let store = DiskStore::create_new(&fixture.path(), binding()).expect("genesis");
    assert!(matches!(
        DiskStore::create_new(&fixture.path(), binding()),
        Err(StoreError::AlreadyExists)
    ));
    drop(store);
    let mut wrong = binding();
    wrong.node = 2;
    assert!(matches!(
        DiskStore::open(&fixture.path(), wrong, None),
        Err(StoreError::BindingMismatch)
    ));
    fs::write(fixture.path(), []).unwrap();
    assert!(matches!(
        DiskStore::open(&fixture.path(), binding(), None),
        Err(StoreError::Quarantined)
    ));
    assert!(matches!(
        DiskStore::create_new(&fixture.path(), binding()),
        Err(StoreError::AlreadyExists)
    ));
}

#[test]
fn unsupported_scope_profile_and_configuration_cannot_create_genesis() {
    let fixture = Fixture::new("invalid-binding");
    for bad in [
        Binding {
            scope: 8,
            ..binding()
        },
        Binding {
            application_profile: 2,
            ..binding()
        },
        Binding {
            node: 9,
            ..binding()
        },
        Binding {
            voters: vec![1, 1, 2],
            ..binding()
        },
        Binding {
            voters: vec![1],
            ..binding()
        },
    ] {
        assert!(matches!(
            DiskStore::create_new(&fixture.path(), bad),
            Err(StoreError::BindingMismatch)
        ));
        assert!(!fixture.path().exists());
    }
}

#[test]
fn held_lock_excludes_a_second_open_and_drop_releases_it() {
    let fixture = Fixture::new("lock");
    let store = DiskStore::create_new(&fixture.path(), binding()).expect("genesis");
    assert!(matches!(
        DiskStore::open(&fixture.path(), binding(), None),
        Err(StoreError::Locked)
    ));
    drop(store);
    assert!(DiskStore::open(&fixture.path(), binding(), None).is_ok());
}

#[test]
fn fault_errors_poison_after_write_and_never_claim_noncommit() {
    for fault in [
        FaultPoint::BeforeWrite,
        FaultPoint::PartialWrite,
        FaultPoint::AfterWrite,
        FaultPoint::BeforeSync,
        FaultPoint::AfterSync,
    ] {
        let fixture = Fixture::new(&format!("fault-{fault:?}"));
        let mut store = DiskStore::create_new(&fixture.path(), binding())
            .unwrap()
            .with_fault(fault);
        let next = conformance::image(1, 1, 1, &[1]);
        assert_eq!(store.persist(0, next.clone()), Err(StoreError::Io));
        if fault == FaultPoint::BeforeWrite {
            assert_eq!(store.load().unwrap().revision, 0);
            assert_eq!(store.persist(0, next).unwrap().revision, 1);
        } else {
            assert_eq!(store.load(), Err(StoreError::Poisoned));
            assert_eq!(
                store.persist(0, DurableImage::default()),
                Err(StoreError::Poisoned)
            );
        }
        drop(store);
        if fault == FaultPoint::PartialWrite {
            assert!(matches!(
                DiskStore::open(&fixture.path(), binding(), None),
                Err(StoreError::Quarantined)
            ));
        } else {
            let mut reopened = DiskStore::open(&fixture.path(), binding(), None).unwrap();
            assert_eq!(reopened.load().unwrap().revision, 1);
        }
    }
}

#[test]
fn torn_or_corrupt_tail_quarantines_instead_of_truncating() {
    for corrupt in [false, true] {
        let fixture = Fixture::new(&format!("tail-{corrupt}"));
        let mut store = DiskStore::create_new(&fixture.path(), binding()).unwrap();
        store.persist(0, conformance::image(1, 1, 1, &[1])).unwrap();
        drop(store);
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(fixture.path())
            .unwrap();
        if corrupt {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            let position = bytes.len() / 2;
            bytes[position] ^= 0x80;
            fs::write(fixture.path(), &bytes).unwrap();
        } else {
            file.write_all(&[0xFF]).unwrap();
        }
        drop(file);
        let before = fs::read(fixture.path()).unwrap();
        assert!(matches!(
            DiskStore::open(&fixture.path(), binding(), None),
            Err(StoreError::Quarantined)
        ));
        assert_eq!(fs::read(fixture.path()).unwrap(), before);
    }
}

#[test]
fn externally_trusted_floor_detects_an_otherwise_valid_old_image() {
    let fixture = Fixture::new("rollback");
    let mut store = DiskStore::create_new(&fixture.path(), binding()).unwrap();
    store.persist(0, conformance::image(1, 1, 1, &[1])).unwrap();
    let old = fs::read(fixture.path()).unwrap();
    store
        .persist(1, conformance::image(2, 2, 2, &[1, 2]))
        .unwrap();
    drop(store);
    fs::write(fixture.path(), old).unwrap();
    assert!(matches!(
        DiskStore::open(&fixture.path(), binding(), Some(2)),
        Err(StoreError::Quarantined)
    ));
    let mut no_floor = DiskStore::open(&fixture.path(), binding(), None).unwrap();
    assert_eq!(no_floor.load().unwrap().revision, 1);
}

#[test]
fn oversized_image_is_rejected_before_write_and_prior_state_stays_usable() {
    let fixture = Fixture::new("capacity");
    let mut store = DiskStore::create_new(&fixture.path(), binding()).unwrap();
    let before = fs::read(fixture.path()).unwrap();
    let mut oversized = conformance::image(1, 1, 1, &[1]);
    oversized.entries[0].bytes = vec![0; 16 * 1024 * 1024];
    assert_eq!(
        store.persist(0, oversized),
        Err(StoreError::CapacityExhausted)
    );
    assert_eq!(store.load().unwrap().revision, 0);
    assert_eq!(fs::read(fixture.path()).unwrap(), before);
    assert_eq!(
        store
            .persist(0, conformance::image(1, 1, 1, &[1]))
            .unwrap()
            .revision,
        1
    );
}
