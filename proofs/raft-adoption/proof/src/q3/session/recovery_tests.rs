#[test]
fn join_authority_missing_reopen_and_incomplete_restart_never_reset_or_vote() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    use raft::eraftpb::Message;
    for case in 0..3 {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-reopen-{}-{case}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let mut factory = V2StoreFactory::new(root.clone());
        let mut stores = BTreeMap::new();
        for id in [1, 2, 3] {
            stores.insert(
                id,
                factory.create(instance(id), recovery::initial()).unwrap(),
            );
        }
        let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
        assert_eq!(
            session.control(Control::CatchUp { node: 4 }),
            Err(Error::Unauthorized)
        );
        assert!(!root.join("node-4").exists());
        let mut foreign = conformance::intent(1, 0, Change::AddLearner { node: 4 });
        foreign.key.group = 71;
        assert_eq!(session.configure(foreign), Err(Error::WrongBinding));
        let mut unauthorized = conformance::intent(1, 0, Change::AddLearner { node: 4 });
        unauthorized.key.principal = 10;
        assert_eq!(session.configure(unauthorized), Err(Error::Unauthorized));
        assert!(
            !root.join("node-4").exists(),
            "admission refusals never invoke create"
        );
        let joined = session
            .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
            .unwrap()
            .unwrap();
        assert_eq!(joined.outcome, glade_raft_q3_api::ConfigOutcome::Accepted);
        if case == 0 {
            session.control(Control::CatchUp { node: 4 }).unwrap();
            std::fs::remove_file(root.join("node-4")).unwrap();
            assert_eq!(session.control(Control::Restart), Err(Error::Missing));
            assert!(!root.join("node-4").exists());
        } else {
            let mut store = session
                .lifecycle
                .create(instance(4), recovery::initial())
                .unwrap();
            let state = store.load().unwrap();
            session.nodes.insert(4, voter(store, state).unwrap());
            session.disconnected.insert(4);
            session.control(Control::Restart).unwrap();
            assert_eq!(session.nodes[&4].machine.applied(), 0);
            assert_eq!(session.nodes[&4].state.image.vote, 0);
            let kind = if case == 1 {
                MessageType::MsgRequestVote
            } else {
                MessageType::MsgRequestPreVote
            };
            let mut message = Message::default();
            message.set_msg_type(kind);
            message.from = 1;
            message.to = 4;
            message.term = 50;
            session.disconnected.clear();
            session.messages.push_back(message);
            session.drain().unwrap();
            assert_eq!(session.nodes[&4].state.image.vote, 0);
            assert_eq!(session.nodes[&4].raft.raft.term, 0);
            session.control(Control::CatchUp { node: 4 }).unwrap();
            assert_eq!(session.configure(joined.intent.clone()), Ok(Some(joined)));
        }
        drop(session);
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn uncommitted_configuration_restart_keeps_unknown_then_commits_exact_original_entry() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{ConfigOutcome, QualificationSession, StoreLifecycle, conformance};
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-uncommitted-restart-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let mut factory = V2StoreFactory::new(root.clone());
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3] {
        stores.insert(
            id,
            factory.create(instance(id), recovery::initial()).unwrap(),
        );
    }
    let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
    let created = conformance::accepted_create(&mut session);
    session
        .control(Control::Disconnect { nodes: vec![2, 3] })
        .unwrap();
    let intent = conformance::intent(1, 0, Change::AddLearner { node: 4 });
    assert_eq!(session.configure(intent.clone()), Ok(None));
    let entry = session.nodes[&1].state.image.suffix.last().unwrap().clone();
    assert_eq!(entry.index, 3);
    assert_eq!(session.nodes[&1].machine.applied(), 2);
    session.control(Control::Restart).unwrap();
    assert_eq!(session.nodes[&1].machine.applied(), 2);
    assert!(
        !session.nodes[&1]
            .machine
            .configurations
            .contains_key(&intent.key)
    );
    assert_eq!(
        session.nodes[&1]
            .state
            .image
            .suffix
            .iter()
            .find(|stored| stored.index == 3),
        Some(&entry)
    );
    session.control(Control::Reconnect).unwrap();
    session.campaign().unwrap();
    session.drain().unwrap();
    let original = session.configuration_outcome(intent.key).unwrap().unwrap();
    assert_eq!(original.intent, intent);
    assert_eq!(original.index, 3);
    assert_eq!(original.outcome, ConfigOutcome::Accepted);
    assert_eq!(
        session.replay(entry.clone()),
        Ok(ReplayResult::Configuration(original.clone()))
    );
    assert_eq!(session.applied_entry(3), Ok(entry));
    assert_eq!(session.configure(intent.clone()), Ok(Some(original)));
    let mut changed = intent;
    changed.expected_configuration = 3;
    assert_eq!(session.configure(changed), Err(Error::RetryConflict));
    assert_eq!(session.submit(conformance::create()), Ok(Some(created)));
    drop(session);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn ordered_stale_promotion_refusal_agrees_at_every_replica_and_demotion_is_invalid() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{
        ConfigOutcome, ConfigRejection, QualificationSession, StoreLifecycle, conformance,
    };
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-replica-refusal-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let mut factory = V2StoreFactory::new(root.clone());
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3] {
        stores.insert(
            id,
            factory.create(instance(id), recovery::initial()).unwrap(),
        );
    }
    let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
    conformance::accepted_create(&mut session);
    let joined = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    session
        .control(Control::QueueApplicationBeforeNextConfiguration {
            command: conformance::command(2, Action::Mutate { payload: 23 }),
        })
        .unwrap();
    let refused = session
        .configure(conformance::intent(
            2,
            joined.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .unwrap()
        .unwrap();
    assert_eq!(
        refused.outcome,
        ConfigOutcome::Refused(ConfigRejection::IncompleteLearner)
    );
    for node in session.nodes.values() {
        assert_eq!(node.machine.configurations[&refused.intent.key], refused);
        assert_eq!(node.machine.configuration, joined.configuration);
        assert_eq!(
            node.machine.history[&refused.index].1,
            ReplayResult::Configuration(refused.clone())
        );
    }
    // A separately replayed unsupported voter demotion deterministically refuses,
    // even when bypassing the proposal admission gate in this private unit test.
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let joint = session
        .configure(conformance::intent(
            3,
            joined.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .unwrap()
        .unwrap();
    let leader = session.leader().unwrap();
    let current = session.nodes[&leader].machine.checkpoint().unwrap();
    let mut machine = Machine::restore(&current).unwrap();
    let intent = conformance::intent(4, joint.index, Change::AddLearner { node: 4 });
    let carrier = machine::change(&intent.change, &machine.configuration);
    use protobuf::Message as ProtobufMessage;
    let entry = raft::eraftpb::Entry {
        index: current.index + 1,
        term: current.term,
        entry_type: raft::eraftpb::EntryType::EntryConfChangeV2,
        context: encoding::context(&intent, &[]).into(),
        data: carrier.write_to_bytes().unwrap().into(),
        ..raft::eraftpb::Entry::default()
    };
    let (result, carrier) = machine
        .apply(StoredEntry {
            index: entry.index,
            term: entry.term,
            bytes: entry.write_to_bytes().unwrap(),
        })
        .unwrap();
    assert!(carrier.is_none());
    assert!(matches!(
        result,
        ReplayResult::Configuration(ConfigReceipt {
            outcome: ConfigOutcome::Refused(ConfigRejection::InvalidChange),
            ..
        })
    ));
    drop(session);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn direct_and_queued_home_exit_refusals_are_retained_at_every_actual_replica() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{ConfigOutcome, ConfigRejection, StoreLifecycle, conformance};
    for (case, (queued, movement)) in [(false, false), (false, true), (true, false), (true, true)]
        .into_iter()
        .enumerate()
    {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-home-replicas-{}-{case}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let mut factory = V2StoreFactory::new(root.clone());
        let mut stores = BTreeMap::new();
        for id in [1, 2, 3] {
            stores.insert(
                id,
                factory.create(instance(id), recovery::initial()).unwrap(),
            );
        }
        let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
        conformance::joint_exit_rechecks_home(&mut session, queued, movement);
        let leader = session.leader().unwrap();
        let refused = session.nodes[&leader]
            .machine
            .configurations
            .values()
            .find(|receipt| receipt.outcome == ConfigOutcome::Refused(ConfigRejection::HomeInUse))
            .unwrap()
            .clone();
        for node in session.nodes.values() {
            assert_eq!(node.machine.configurations[&refused.intent.key], refused);
            assert_eq!(
                node.machine.history[&refused.index].1,
                ReplayResult::Configuration(refused.clone())
            );
        }
        drop(session);
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn terminal_term_and_predecessor_refuse_before_campaign_without_resetting_evidence() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::StoreLifecycle;
    for terminal in [u64::MAX - 1, u64::MAX] {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-terminal-{}-{terminal}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let mut factory = V2StoreFactory::new(root.clone());
        let mut stores = BTreeMap::new();
        for id in [1, 2, 3] {
            let mut initial = recovery::initial();
            initial.term = terminal;
            if terminal == u64::MAX {
                assert!(matches!(
                    factory.create(instance(id), initial),
                    Err(Error::CapacityExhausted)
                ));
                assert!(!root.join(format!("node-{id}")).exists());
            } else {
                stores.insert(id, factory.create(instance(id), initial).unwrap());
            }
        }
        if terminal != u64::MAX {
            let originals: Vec<_> = (1..=3)
                .map(|id| std::fs::read(root.join(format!("node-{id}"))).unwrap())
                .collect();
            assert!(matches!(
                Q3Session::recover(stores, Box::new(factory), vec![1, 2]),
                Err(Error::CapacityExhausted)
            ));
            for id in 1..=3 {
                assert_eq!(
                    std::fs::read(root.join(format!("node-{id}"))).unwrap(),
                    originals[id - 1]
                );
            }
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
