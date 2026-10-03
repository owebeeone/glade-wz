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

// Independent private-format adversarial records; checksum recomputation ensures
// semantic recovery checks, rather than merely checksum detection, are exercised.
fn test_record(revision: u64, image: &DurableImage) -> Vec<u8> {
    let mut payload = Vec::new();
    let bound = binding();
    for word in [
        revision,
        bound.scope,
        bound.node,
        bound.application_profile,
        3,
        1,
        2,
        3,
        image.term,
        image.vote,
        image.commit,
        image.entries.len() as u64,
    ] {
        payload.extend(word.to_le_bytes());
    }
    for entry in &image.entries {
        for word in [entry.index, entry.term, entry.bytes.len() as u64] {
            payload.extend(word.to_le_bytes());
        }
        payload.extend(&entry.bytes);
    }
    let mut record = b"GQ2JRNL1".to_vec();
    record.extend((payload.len() as u64 + 24).to_le_bytes());
    record.extend(payload);
    test_checksum(&mut record);
    record
}

fn test_checksum(record: &mut Vec<u8>) {
    let mut crc = 0_u64;
    for byte in &*record {
        crc ^= u64::from(*byte) << 56;
        for _ in 0..8 {
            crc = if crc & (1 << 63) != 0 {
                (crc << 1) ^ 0x42F0_E1EB_A9EA_3693
            } else {
                crc << 1
            };
        }
    }
    record.extend(crc.to_le_bytes());
}

fn rejects_without_repair(label: &str, bytes: &[u8]) {
    let fixture = Fixture::new(label);
    // Ensure the adapter accepts actual genesis before forging recovery records.
    let store = DiskStore::create_new(&fixture.path(), binding()).unwrap();
    drop(store);
    fs::write(fixture.path(), bytes).unwrap();
    assert!(matches!(
        DiskStore::open(&fixture.path(), binding(), None),
        Err(StoreError::Quarantined)
    ));
    assert_eq!(fs::read(fixture.path()).unwrap(), bytes);
}

#[test]
fn malformed_length_version_and_count_are_bounded_and_quarantined() {
    for size in [0_u64, 23, 16 * 1024 * 1024 + 1, u64::MAX] {
        let mut record = b"GQ2JRNL1".to_vec();
        record.extend(size.to_le_bytes());
        rejects_without_repair(&format!("length-{size}"), &record);
    }
    let mut wrong_version = test_record(0, &DurableImage::default());
    wrong_version[7] = b'2';
    wrong_version.truncate(wrong_version.len() - 8);
    test_checksum(&mut wrong_version);
    rejects_without_repair("version", &wrong_version);
    let mut huge_count = test_record(0, &DurableImage::default());
    huge_count[104..112].copy_from_slice(&u64::MAX.to_le_bytes());
    huge_count.truncate(huge_count.len() - 8);
    test_checksum(&mut huge_count);
    rejects_without_repair("entry-count", &huge_count);
}

#[test]
fn revision_gaps_duplicates_and_missing_genesis_quarantine() {
    let image = conformance::image(1, 1, 1, &[1]);
    for revision in [0, 2, u64::MAX] {
        let mut journal = test_record(0, &DurableImage::default());
        journal.extend(test_record(revision, &image));
        rejects_without_repair(&format!("revision-{revision}"), &journal);
    }
    rejects_without_repair("missing-genesis", &test_record(1, &image));
}

#[test]
fn valid_checksums_cannot_hide_conflicting_committed_history_or_binding() {
    let first = conformance::image(1, 1, 1, &[1]);
    let mut changed = first.clone();
    changed.entries[0].bytes.push(99);
    let mut journal = test_record(0, &DurableImage::default());
    journal.extend(test_record(1, &first));
    journal.extend(test_record(2, &changed));
    rejects_without_repair("conflicting-prefix", &journal);
    let mut wrong_binding = test_record(1, &first);
    wrong_binding[32..40].copy_from_slice(&2_u64.to_le_bytes());
    wrong_binding.truncate(wrong_binding.len() - 8);
    test_checksum(&mut wrong_binding);
    let mut journal = test_record(0, &DurableImage::default());
    journal.extend(wrong_binding);
    rejects_without_repair("changed-binding", &journal);
}

#[test]
fn genesis_must_be_revision_zero_exactly_empty_and_canonical() {
    rejects_without_repair(
        "genesis-nonempty",
        &test_record(0, &conformance::image(1, 1, 1, &[1])),
    );
    let mut trailing = test_record(0, &DurableImage::default());
    trailing.truncate(trailing.len() - 8);
    trailing.extend(0_u64.to_le_bytes());
    let size = trailing.len() as u64 + 8;
    trailing[8..16].copy_from_slice(&size.to_le_bytes());
    test_checksum(&mut trailing);
    rejects_without_repair("genesis-trailing-word", &trailing);
}
