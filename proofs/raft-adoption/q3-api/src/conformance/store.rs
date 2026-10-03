use crate::{Checkpoint, CheckpointStore, Configuration, Error, Image, StoredEntry};

pub fn stable() -> Configuration {
    Configuration {
        voters: vec![1, 2, 3],
        learners: Vec::new(),
        voters_outgoing: Vec::new(),
        learners_next: Vec::new(),
        auto_leave: false,
        index: 0,
    }
}

pub fn opaque_image() -> Image {
    Image {
        term: 2,
        vote: 3,
        commit: 2,
        applied: 2,
        configuration: stable(),
        checkpoint: None,
        suffix: vec![
            StoredEntry {
                index: 1,
                term: 1,
                bytes: vec![11],
            },
            StoredEntry {
                index: 2,
                term: 2,
                bytes: vec![22],
            },
        ],
    }
}

pub fn compacted() -> Image {
    let mut image = opaque_image();
    image.checkpoint = Some(Checkpoint {
        binding: crate::GroupIdentity {
            scope: 7,
            group: 70,
            application_profile: 2,
        },
        index: 2,
        term: 2,
        configuration: stable(),
        application: vec![1, 7, 9, 11, 23, 40],
    });
    image.suffix.clear();
    image
}

/// QC-001/003/004: local publication, exact image and immutable checkpoint.
/// The opaque bytes intentionally do NOT claim semantic application validation.
pub fn store_snapshot_roundtrip(store: &mut dyn CheckpointStore) {
    let genesis = store.load().expect("explicit V2 genesis");
    assert_eq!(genesis.revision, 0);
    let full = store
        .publish(0, opaque_image())
        .expect("uncompacted prefix");
    assert_eq!(full.revision, 1);
    let image = compacted();
    let saved = store
        .publish(1, image.clone())
        .expect("coherent checkpoint");
    assert_eq!(saved.revision, 2);
    assert_eq!(saved.instance, genesis.instance);
    assert_eq!(saved.image, image);
    assert_eq!(store.load(), Ok(saved.clone()));
    assert_eq!(
        store.publish(1, image.clone()),
        Err(Error::RevisionConflict {
            expected: 1,
            actual: 2
        })
    );
    let mut rewrite = image;
    rewrite
        .checkpoint
        .as_mut()
        .expect("checkpoint")
        .application
        .push(99);
    assert_eq!(store.publish(2, rewrite), Err(Error::InvalidImage));
    assert_eq!(store.load(), Ok(saved));
}

/// QC-003/004/009: no gap/regression or overflow; suffix replacement stays legal.
pub fn store_suffix_and_bounds(store: &mut dyn CheckpointStore) {
    store.publish(0, opaque_image()).expect("complete prefix");
    let mut image = compacted();
    image.suffix.push(StoredEntry {
        index: 3,
        term: 2,
        bytes: vec![33],
    });
    let first = store.publish(1, image.clone()).expect("uncommitted suffix");
    image.term = 3;
    image.vote = 4;
    image.suffix[0] = StoredEntry {
        index: 3,
        term: 3,
        bytes: vec![44],
    };
    let replaced = store
        .publish(first.revision, image.clone())
        .expect("replace only uncommitted suffix");
    let mut gap = image.clone();
    gap.suffix[0].index = 4;
    assert_eq!(
        store.publish(replaced.revision, gap),
        Err(Error::InvalidImage)
    );
    let mut regression = image.clone();
    regression.commit = 1;
    assert_eq!(
        store.publish(replaced.revision, regression),
        Err(Error::InvalidImage)
    );
    let mut exhausted = image;
    exhausted.term = u64::MAX;
    assert_eq!(
        store.publish(replaced.revision, exhausted),
        Err(Error::CapacityExhausted)
    );
    assert_eq!(store.load(), Ok(replaced));
}

/// QC-006: removed voter is still a valid historical same-term vote.
pub fn store_removed_vote_is_not_rewritten(store: &mut dyn CheckpointStore) {
    store.publish(0, opaque_image()).expect("vote 3 at term 2");
    let mut image = compacted();
    image.suffix.push(StoredEntry {
        index: 3,
        term: 2,
        bytes: vec![33],
    });
    image.commit = 3;
    image.applied = 3;
    image.configuration = Configuration {
        voters: vec![1, 2, 4],
        learners: Vec::new(),
        voters_outgoing: Vec::new(),
        learners_next: Vec::new(),
        auto_leave: false,
        index: 3,
    };
    let saved = store
        .publish(1, image)
        .expect("historical vote survives removal");
    assert_eq!(saved.image.vote, 3);
    let mut cleared = saved.image.clone();
    cleared.vote = 0;
    assert_eq!(
        store.publish(saved.revision, cleared),
        Err(Error::InvalidImage)
    );
    assert_eq!(store.load(), Ok(saved));
}
