#[test]
fn incomplete_original_genesis_learner_is_gated_before_votes_and_pre_votes() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    use raft::eraftpb::Message;
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    for (case, kind) in [MessageType::MsgRequestVote, MessageType::MsgRequestPreVote]
        .into_iter()
        .enumerate()
    {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-nonvoting-{}-{case}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let directory = Directory(root.clone());
        let mut factory = V2StoreFactory::new(root);
        let mut stores = BTreeMap::new();
        for id in [1, 2, 3] {
            stores.insert(
                id,
                factory.create(instance(id), recovery::initial()).unwrap(),
            );
        }
        let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
        let added = session
            .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
            .unwrap()
            .unwrap();
        assert_eq!(added.outcome, glade_raft_q3_api::ConfigOutcome::Accepted);
        // Pause an admitted new learner between creation and actual restoration.
        let mut store = session
            .lifecycle
            .create(instance(4), recovery::initial())
            .unwrap();
        let state = store.load().unwrap();
        session.nodes.insert(4, voter(store, state).unwrap());
        let mut message = Message::default();
        message.set_msg_type(kind);
        message.from = 1;
        message.to = 4;
        message.term = 2;
        session.messages.push_back(message);
        session.drain().unwrap();
        assert_eq!(
            session.nodes[&4].state.image.vote, 0,
            "incomplete learner must not persist a vote"
        );
        assert_eq!(
            session.nodes[&4].raft.raft.term, 0,
            "host excludes both vote kinds before even stepping the private learner"
        );
        drop(session);
        drop(directory);
    }
}

#[test]
fn pending_configuration_retry_keeps_unknown_and_rejects_changed_intent_bytes() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-pending-intent-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let directory = Directory(root.clone());
    let mut factory = V2StoreFactory::new(root);
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3] {
        stores.insert(
            id,
            factory.create(instance(id), recovery::initial()).unwrap(),
        );
    }
    let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
    session
        .control(Control::Disconnect { nodes: vec![2, 3] })
        .unwrap();
    let intent = conformance::intent(1, 0, Change::AddLearner { node: 4 });
    assert_eq!(session.configure(intent.clone()), Ok(None));
    assert_eq!(session.configure(intent.clone()), Ok(None));
    let mut changed = intent.clone();
    changed.change = Change::AddLearner { node: 3 };
    assert_eq!(session.configure(changed), Err(Error::RetryConflict));
    assert_eq!(session.configuration_outcome(intent.key), Ok(None));
    session.control(Control::Reconnect).unwrap();
    let original = session.configuration_outcome(intent.key).unwrap().unwrap();
    assert_eq!(original.intent, intent);
    assert_eq!(session.configure(intent), Ok(Some(original)));
    drop(session);
    drop(directory);
}

#[test]
fn a_missing_promoted_voter_store_blocks_recovery_instead_of_recreating() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-missing-voter-{}", std::process::id()));
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
    let added = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    session
        .configure(conformance::intent(
            2,
            added.configuration.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .unwrap()
        .unwrap();
    drop(session);
    let mut factory = V2StoreFactory::new(root);
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3] {
        stores.insert(id, factory.open(instance(id), None).unwrap());
    }
    assert!(matches!(
        Q3Session::recover(stores, Box::new(factory), vec![1, 2]),
        Err(Error::Missing)
    ));
    drop(directory);
}

#[test]
fn removing_leader_preserves_home_and_manual_restart_campaigns_an_eligible_voter() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-remove-leader-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let directory = Directory(root.clone());
    let mut factory = V2StoreFactory::new(root);
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
        home: 2,
        payload: 11,
    };
    let original =
        conformance::submit_accepted(&mut session, create, conformance::created_resource(2));
    let added = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let entered = session
        .configure(conformance::intent(
            2,
            added.index,
            Change::EnterJoint {
                voters: vec![2, 3, 4],
            },
        ))
        .unwrap()
        .unwrap();
    session
        .configure(conformance::intent(3, entered.index, Change::LeaveJoint))
        .unwrap()
        .unwrap();
    assert_eq!(
        session.leader(),
        Err(Error::NoQuorum),
        "removed carrier leader must be excluded from host serving"
    );
    session.control(Control::Restart).unwrap();
    assert_eq!(session.leader(), Ok(2));
    assert_eq!(session.submit(create), Ok(Some(original)));
    assert_eq!(session.resource(100).unwrap().unwrap().home, 2);
    assert_eq!(session.resource(100).unwrap().unwrap().generation, 1);
    let mut mutation = conformance::command(2, Action::Mutate { payload: 23 });
    mutation.home = 2;
    conformance::submit_accepted(
        &mut session,
        mutation,
        glade_raft_q3_api::Resource {
            payload: 23,
            ..conformance::created_resource(2)
        },
    );
    drop(session);
    drop(directory);
}

#[test]
fn an_empty_private_four_requires_external_retained_join_authority_on_recovery() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::StoreLifecycle;
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-unauthorized-empty-four-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let directory = Directory(root.clone());
    let mut factory = V2StoreFactory::new(root.clone());
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3, 4] {
        stores.insert(
            id,
            factory.create(instance(id), recovery::initial()).unwrap(),
        );
    }
    let original = std::fs::read(root.join("node-4")).unwrap();
    assert!(matches!(
        Q3Session::recover(stores, Box::new(factory), vec![1, 2]),
        Err(Error::Unauthorized)
    ));
    assert_eq!(
        std::fs::read(root.join("node-4")).unwrap(),
        original,
        "invalid newcomer is neither reset nor served"
    );
    drop(directory);
}
