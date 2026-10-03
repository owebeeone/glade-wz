#[test]
fn valid_future_checkpoints_validate_overlap_and_refuse_without_any_publication() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    fn session(root: &std::path::Path, payload: u64) -> (Q3Session, Receipt) {
        std::fs::create_dir(root).unwrap();
        let mut factory = V2StoreFactory::new(root.to_path_buf());
        let mut stores = BTreeMap::new();
        for id in [1, 2, 3] {
            stores.insert(
                id,
                factory.create(instance(id), recovery::initial()).unwrap(),
            );
        }
        let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
        let mut create = conformance::create();
        create.action = Action::Create {
            name: 40,
            home: 1,
            payload,
        };
        let receipt = conformance::submit_accepted(
            &mut session,
            create,
            glade_raft_q3_api::Resource {
                payload,
                ..conformance::created_resource(1)
            },
        );
        (session, receipt)
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-rem-future-install-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let directory = Directory(root.clone());
    for payload in [99, 11] {
        let a_root = root.join(format!("a-{payload}"));
        let b_root = root.join(format!("b-{payload}"));
        let (mut a, original) = session(&a_root, 11);
        let (mut b, _) = session(&b_root, payload);
        let original_entry = a.applied_entry(original.index).unwrap();
        let original_view = a.view().unwrap();
        let original_files: Vec<_> = (1..=3)
            .map(|id| std::fs::read(a_root.join(format!("node-{id}"))).unwrap())
            .collect();
        conformance::submit_accepted(
            &mut b,
            conformance::command(2, Action::Mutate { payload: 23 }),
            glade_raft_q3_api::Resource {
                payload: 23,
                ..conformance::created_resource(1)
            },
        );
        let future = b.checkpoint().unwrap();
        assert_eq!(future.index, 3);
        assert!(
            Machine::restore(&future).is_ok(),
            "internally complete actual future checkpoint"
        );
        assert!(a.nodes.values().all(|node| node.machine.applied() == 2));
        let expected = if payload == 99 {
            Error::Quarantined
        } else {
            Error::InvalidImage
        };
        assert_eq!(
            a.install(future),
            Err(expected),
            "an unsupported future install must neither bypass overlap nor report durable success"
        );
        assert_eq!(a.view(), Ok(original_view));
        assert_eq!(a.outcome(original.request), Ok(Some(original)));
        assert_eq!(a.applied_entry(original.index), Ok(original_entry.clone()));
        assert_eq!(
            a.replay(original_entry.clone()),
            Ok(ReplayResult::Application(original))
        );
        assert_eq!(a.resource(100), Ok(Some(conformance::created_resource(1))));
        for id in 1..=3 {
            assert_eq!(
                std::fs::read(a_root.join(format!("node-{id}"))).unwrap(),
                original_files[id - 1]
            );
        }
        let same_history = a.checkpoint().unwrap();
        a.install(same_history.clone()).unwrap();
        assert!(
            a.nodes
                .values()
                .all(|node| node.state.image.checkpoint == Some(same_history.clone()))
        );
        a.control(Control::Restart).unwrap();
        assert_eq!(a.submit(conformance::create()), Ok(Some(original)));
        assert_eq!(
            a.replay(original_entry),
            Ok(ReplayResult::Application(original))
        );
        drop(a);
        drop(b);
    }
    drop(directory);
}
