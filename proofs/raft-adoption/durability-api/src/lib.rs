//! Q2 experimental fixed-configuration persistence boundary. See the normative
//! GladeRaftPersistenceContract.md; successful persistence is local, not a quorum.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binding {
    pub scope: u64,
    pub node: u64,
    pub voters: Vec<u64>,
    pub application_profile: u64,
}

/// Original serialized Raft Entry bytes plus independently checked framing.
/// The host MUST validate both protobuf framing and private command semantics
/// before starting any recovered voter. The store does not interpret commands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredEntry {
    pub index: u64,
    pub term: u64,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DurableImage {
    pub term: u64,
    pub vote: u64,
    pub commit: u64,
    pub entries: Vec<StoredEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredState {
    pub revision: u64,
    pub binding: Binding,
    pub image: DurableImage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoreError {
    NotQualified,
    Missing,
    AlreadyExists,
    BindingMismatch,
    Locked,
    Quarantined,
    Poisoned,
    InvalidImage,
    RevisionConflict { expected: u64, actual: u64 },
    Io,
    CapacityExhausted,
}

/// A caller-owned single-writer store. Errors after a write begins poison the
/// instance. Success follows sync_all; a failure is never proof of noncommit.
/// Revision checks are local compare-and-persist, not distributed authority.
pub trait DurableStore {
    fn load(&mut self) -> Result<StoredState, StoreError>;
    fn persist(
        &mut self,
        expected_revision: u64,
        image: DurableImage,
    ) -> Result<StoredState, StoreError>;
}

/// Opt-in shared semantic assertions, used by both a test model and the disk
/// adapter. Opaque bytes are fixtures here; real host replay tests use protobuf.
pub mod conformance {
    use super::{DurableImage, DurableStore, StoreError, StoredEntry};

    pub fn image(term: u64, vote: u64, commit: u64, terms: &[u64]) -> DurableImage {
        DurableImage {
            term,
            vote,
            commit,
            entries: terms
                .iter()
                .enumerate()
                .map(|(offset, entry_term)| StoredEntry {
                    index: offset as u64 + 1,
                    term: *entry_term,
                    bytes: vec![*entry_term as u8, offset as u8],
                })
                .collect(),
        }
    }

    pub fn exercise_store(store: &mut dyn DurableStore) {
        let initial = store.load().expect("explicit genesis");
        assert_eq!(initial.revision, 0);
        assert_eq!(initial.image, DurableImage::default());
        let first = image(2, 1, 1, &[1, 2]);
        let saved = store
            .persist(0, first.clone())
            .expect("first durable image");
        assert_eq!(saved.revision, 1);
        assert_eq!(saved.binding, initial.binding);
        assert_eq!(saved.image, first);
        assert_eq!(store.load(), Ok(saved.clone()));
        assert_eq!(
            store.persist(0, first.clone()),
            Err(StoreError::RevisionConflict {
                expected: 0,
                actual: 1
            })
        );
        let replacement = image(3, 2, 1, &[1, 3]);
        let replaced = store
            .persist(1, replacement.clone())
            .expect("uncommitted replacement");
        assert_eq!(replaced.revision, 2);
        assert_eq!(replaced.image, replacement);
        let shortened = store
            .persist(2, image(3, 2, 1, &[1]))
            .expect("uncommitted suffix may shorten");
        assert_eq!(shortened.revision, 3);
        let extended = store.persist(3, replacement.clone()).expect("append again");
        assert_eq!(extended.revision, 4);
        let committed = image(3, 2, 2, &[1, 3]);
        let saved = store
            .persist(4, committed.clone())
            .expect("commit-only update");
        assert_eq!(saved.revision, 5);
        let mut changed = committed.clone();
        changed.entries[0].bytes.push(99);
        assert_eq!(store.persist(5, changed), Err(StoreError::InvalidImage));
        assert_eq!(
            store.persist(5, image(3, 2, 1, &[1, 3])),
            Err(StoreError::InvalidImage)
        );
        assert_eq!(
            store.persist(5, image(2, 1, 2, &[1, 2])),
            Err(StoreError::InvalidImage)
        );
        assert_eq!(
            store.persist(5, image(3, 1, 2, &[1, 3])),
            Err(StoreError::InvalidImage)
        );
        assert_eq!(
            store.persist(5, image(3, 0, 2, &[1, 3])),
            Err(StoreError::InvalidImage)
        );
        assert_eq!(
            store.persist(5, image(3, 99, 2, &[1, 3])),
            Err(StoreError::InvalidImage)
        );
        assert_eq!(
            store.persist(5, image(3, 2, 3, &[1, 3])),
            Err(StoreError::InvalidImage)
        );
        assert_eq!(
            store.persist(5, image(3, 2, 2, &[1])),
            Err(StoreError::InvalidImage)
        );
        let mut gap = committed.clone();
        gap.entries[1].index = 3;
        assert_eq!(store.persist(5, gap), Err(StoreError::InvalidImage));
        assert_eq!(store.load(), Ok(saved));
    }
}
