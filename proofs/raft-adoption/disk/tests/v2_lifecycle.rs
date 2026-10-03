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
