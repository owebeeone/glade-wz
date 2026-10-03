#[test]
fn coherent_foreign_checkpoint_envelope_history_and_binding_refuse_without_publication() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{Image, QualificationSession, StoreLifecycle, conformance};
    use protobuf::Message as ProtobufMessage;
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-rem-coherent-foreign-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let directory = Directory(root.clone());
    let mut factory = V2StoreFactory::new(root.clone());
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3] {
        stores.insert(
            id,
            factory.create(instance(id), recovery::initial()).unwrap(),
        );
    }
    let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
    let original = conformance::accepted_create(&mut session);
    let original_entry = session.applied_entry(original.index).unwrap();
    let checkpoint = session.checkpoint().unwrap();
    assert_eq!(checkpoint.index, 2);
    assert_eq!(checkpoint.configuration.index, 0);
    let mut foreign = checkpoint.clone();
    foreign.binding.group = 71;
    foreign.application[24..32].copy_from_slice(&71_u64.to_le_bytes());
    // Independently inspect the foreign portable envelope, replay every exact
    // original normal Entry/result, and compare the full materialized tail.
    // This two-entry history has no configuration intents/group70 contexts.
    let mut input = encoding::Reader(&foreign.application);
    assert_eq!(input.bytes().unwrap(), b"GQ3APP02");
    assert_eq!(
        [
            input.word().unwrap(),
            input.word().unwrap(),
            input.word().unwrap()
        ],
        [
            foreign.binding.scope,
            foreign.binding.group,
            foreign.binding.application_profile
        ]
    );
    let count = input.word().unwrap();
    assert_eq!(count, foreign.index);
    let mut replayed = Machine::new();
    for index in 1..=count {
        let entry = StoredEntry {
            index: input.word().unwrap(),
            term: input.word().unwrap(),
            bytes: input.bytes().unwrap(),
        };
        assert_eq!(entry.index, index);
        let carrier = raft::eraftpb::Entry::parse_from_bytes(&entry.bytes).unwrap();
        assert_eq!(
            carrier.get_entry_type(),
            raft::eraftpb::EntryType::EntryNormal
        );
        assert!(carrier.context.is_empty());
        let (result, change) = replayed.apply(entry).unwrap();
        assert!(change.is_none());
        let mut expected = Vec::new();
        machine::result_bytes(&mut expected, &result);
        assert_eq!(input.take(expected.len()).unwrap(), expected);
    }
    assert_eq!(replayed.configuration, foreign.configuration);
    assert_eq!(replayed.history[&foreign.index].0.term, foreign.term);
    let mut tail = Vec::new();
    machine::configuration_bytes(&mut tail, &replayed.configuration);
    encoding::bytes(&mut tail, &replayed.application.complete_evidence());
    assert_eq!(input.0, tail);
    assert_eq!(
        replayed.application.reply(conformance::create()),
        Some(original)
    );
    // Normalizing only the consistent group binding/envelope restores the exact
    // original fixed-profile fixture, independently confirming internal content.
    let mut normalized = foreign.clone();
    normalized.binding.group = 70;
    normalized.application[24..32].copy_from_slice(&70_u64.to_le_bytes());
    assert_eq!(normalized, checkpoint);
    assert!(Machine::restore(&normalized).is_ok());
    let view = session.view().unwrap();
    let images: Vec<Image> = session
        .nodes
        .values()
        .map(|node| node.state.image.clone())
        .collect();
    let files: Vec<_> = (1..=3)
        .map(|id| std::fs::read(root.join(format!("node-{id}"))).unwrap())
        .collect();
    assert_eq!(session.install(foreign), Err(Error::WrongBinding));
    assert_eq!(session.view(), Ok(view));
    assert_eq!(session.outcome(original.request), Ok(Some(original)));
    assert_eq!(
        session.applied_entry(original.index),
        Ok(original_entry.clone())
    );
    assert_eq!(
        session.replay(original_entry),
        Ok(ReplayResult::Application(original))
    );
    assert_eq!(
        session
            .nodes
            .values()
            .map(|node| node.state.image.clone())
            .collect::<Vec<_>>(),
        images
    );
    for id in 1..=3 {
        assert_eq!(
            std::fs::read(root.join(format!("node-{id}"))).unwrap(),
            files[id - 1]
        );
    }
    drop(session);
    drop(directory);
}
