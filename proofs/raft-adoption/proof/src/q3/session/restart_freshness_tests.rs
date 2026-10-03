#[test]
fn partitioned_joint_exit_reopens_fresh_eligible_voter_and_preserves_full_originals() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{ConfigOutcome, QualificationSession, StoreLifecycle, conformance};
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-rem-freshness-{}", std::process::id()));
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
    let mut create = conformance::create();
    create.action = Action::Create {
        name: 40,
        home: 2,
        payload: 11,
    };
    let original =
        conformance::submit_accepted(&mut session, create, conformance::created_resource(2));
    let original_entry = session.applied_entry(original.index).unwrap();
    let joined = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    assert_eq!(joined.outcome, ConfigOutcome::Accepted);
    let joined_entry = session.applied_entry(joined.index).unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let joint = session
        .configure(conformance::intent(
            2,
            joined.index,
            Change::EnterJoint {
                voters: vec![2, 3, 4],
            },
        ))
        .unwrap()
        .unwrap();
    assert_eq!(joint.outcome, ConfigOutcome::Accepted);
    let joint_entry = session.applied_entry(joint.index).unwrap();
    session
        .control(Control::Disconnect { nodes: vec![2] })
        .unwrap();
    let left = session
        .configure(conformance::intent(3, joint.index, Change::LeaveJoint))
        .unwrap()
        .unwrap();
    assert_eq!(left.outcome, ConfigOutcome::Accepted);
    assert_eq!(left.configuration.voters, vec![2, 3, 4]);
    assert!(left.configuration.voters_outgoing.is_empty());
    let left_entry = session.nodes[&3].machine.history[&left.index].0.clone();
    assert_eq!(session.nodes[&2].state.image.applied, joint.index);
    assert_eq!(
        session.nodes[&2].state.image.suffix.last().unwrap().index,
        joint.index
    );
    for id in [1, 3, 4] {
        assert_eq!(session.nodes[&id].state.image.applied, left.index);
    }
    session.control(Control::Reconnect).unwrap();
    assert_eq!(
        session.nodes[&2].state.image.applied, joint.index,
        "reconnect has no eligible leader to repair stale node2"
    );
    drop(session);
    // Genuine reopen, not reconstruction from the former session's memory.
    let mut factory = V2StoreFactory::new(root.clone());
    let mut stores = BTreeMap::new();
    for id in [1, 2, 3, 4] {
        stores.insert(id, factory.open(instance(id), None).unwrap());
    }
    let mut recovered = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
    assert_eq!(
        recovered.view().unwrap().configuration,
        left.configuration,
        "available authorized fresh voters must recover a leader"
    );
    assert_eq!(
        recovered.leader(),
        Ok(3),
        "fresh log tie chooses lower eligible id, not stale2 or removed1"
    );
    assert_eq!(
        recovered.resource(100),
        Ok(Some(conformance::created_resource(2)))
    );
    assert_eq!(recovered.submit(create), Ok(Some(original)));
    assert_eq!(
        recovered.applied_entry(original.index),
        Ok(original_entry.clone())
    );
    assert_eq!(
        recovered.replay(original_entry),
        Ok(ReplayResult::Application(original))
    );
    for (receipt, entry) in [
        (joined, joined_entry),
        (joint, joint_entry),
        (left.clone(), left_entry),
    ] {
        assert_eq!(
            recovered.configure(receipt.intent.clone()),
            Ok(Some(receipt.clone()))
        );
        assert_eq!(recovered.applied_entry(receipt.index), Ok(entry.clone()));
        assert_eq!(
            recovered.replay(entry),
            Ok(ReplayResult::Configuration(receipt))
        );
    }
    let mut mutation = conformance::command(2, Action::Mutate { payload: 23 });
    mutation.home = 2;
    let mutated = conformance::submit_accepted(
        &mut recovered,
        mutation,
        glade_raft_q3_api::Resource {
            payload: 23,
            ..conformance::created_resource(2)
        },
    );
    let mutation_entry = recovered.applied_entry(mutated.index).unwrap();
    recovered.control(Control::Restart).unwrap();
    assert!(matches!(recovered.leader(), Ok(2..=4)));
    assert_eq!(recovered.configure(left.intent.clone()), Ok(Some(left)));
    assert_eq!(recovered.submit(create), Ok(Some(original)));
    assert_eq!(recovered.submit(mutation), Ok(Some(mutated)));
    assert_eq!(
        recovered.replay(mutation_entry),
        Ok(ReplayResult::Application(mutated))
    );
    recovered
        .control(Control::Disconnect { nodes: vec![2, 4] })
        .unwrap();
    recovered.control(Control::Restart).unwrap();
    assert_eq!(recovered.leader(), Err(Error::NoQuorum));
    let mut uncommittable = conformance::command(3, Action::Mutate { payload: 99 });
    uncommittable.home = 2;
    assert_eq!(recovered.submit(uncommittable), Ok(None));
    assert!([2, 3, 4].iter().all(|id| {
        recovered.nodes[id]
            .machine
            .application
            .resource(100)
            .unwrap()
            .payload
            == 23
    }));
    assert_eq!(
        recovered.nodes[&1]
            .machine
            .application
            .resource(100)
            .unwrap()
            .payload,
        11,
        "removed voter stays outside later quorum/application work"
    );
    drop(recovered);
    drop(directory);
}

#[test]
fn disconnected_freshest_voter_does_not_trap_manual_restart_with_an_available_quorum() {
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
        .join(format!("q3-rem-available-candidate-{}", std::process::id()));
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
    session
        .control(Control::Disconnect { nodes: vec![1] })
        .unwrap();
    session.control(Control::Restart).unwrap();
    assert_eq!(session.leader(), Ok(2));
    assert_eq!(session.submit(conformance::create()), Ok(Some(original)));
    let mutation = conformance::command(2, Action::Mutate { payload: 23 });
    conformance::submit_accepted(
        &mut session,
        mutation,
        glade_raft_q3_api::Resource {
            payload: 23,
            ..conformance::created_resource(1)
        },
    );
    drop(session);
    drop(directory);
}

#[test]
fn outgoing_only_fresh_voter_recovers_joint_unknown_after_actual_reopen() {
    use super::*;
    use glade_raft_disk::v2::V2StoreFactory;
    use glade_raft_q3_api::{
        ConfigOutcome, ConfigReceipt, Configuration, Outcome, QualificationSession, Receipt,
        StoreLifecycle, conformance,
    };
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    for (case, isolated) in [vec![4], vec![2, 3]].into_iter().enumerate() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-rem2-outgoing-{}-{case}", std::process::id()));
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
        assert_eq!(session.view().unwrap().committed, 1);
        let join_intent = conformance::intent(1, 0, Change::AddLearner { node: 4 });
        let joined = ConfigReceipt {
            intent: join_intent.clone(),
            index: 2,
            outcome: ConfigOutcome::Accepted,
            configuration: Configuration {
                voters: vec![1, 2, 3],
                learners: vec![4],
                voters_outgoing: vec![],
                learners_next: vec![],
                auto_leave: false,
                index: 2,
            },
        };
        assert_eq!(session.configure(join_intent), Ok(Some(joined.clone())));
        let joined_entry = session.applied_entry(2).unwrap();
        session.control(Control::CatchUp { node: 4 }).unwrap();
        let joint_intent = conformance::intent(2, 2, Change::EnterJoint { voters: vec![4] });
        let joint = ConfigReceipt {
            intent: joint_intent.clone(),
            index: 3,
            outcome: ConfigOutcome::Accepted,
            configuration: Configuration {
                voters: vec![4],
                learners: vec![],
                voters_outgoing: vec![1, 2, 3],
                learners_next: vec![],
                auto_leave: false,
                index: 3,
            },
        };
        assert_eq!(session.configure(joint_intent), Ok(Some(joint.clone())));
        let joint_entry = session.applied_entry(3).unwrap();
        let mut create = conformance::create();
        create.action = Action::Create {
            name: 40,
            home: 4,
            payload: 11,
        };
        let original = Receipt {
            request: create.request,
            index: 4,
            outcome: Outcome::Accepted(conformance::created_resource(4)),
        };
        assert_eq!(session.submit(create), Ok(Some(original)));
        let original_entry = session.applied_entry(4).unwrap();
        let mut unknown = conformance::command(2, Action::Mutate { payload: 23 });
        unknown.home = 4;
        let expected_unknown = Receipt {
            request: unknown.request,
            index: 5,
            outcome: Outcome::Accepted(glade_raft_q3_api::Resource {
                payload: 23,
                ..conformance::created_resource(4)
            }),
        };
        session
            .control(Control::Disconnect { nodes: vec![4] })
            .unwrap();
        assert_eq!(session.submit(unknown), Ok(None));
        assert_eq!(session.outcome(unknown.request), Ok(None));
        let unknown_entry = session.nodes[&1].state.image.suffix.last().unwrap().clone();
        assert_eq!(unknown_entry.index, 5);
        assert_eq!(
            machine::parse(&unknown_entry).unwrap().data.to_vec(),
            crate::codec::encode(unknown, None)
        );
        for id in [1, 2, 3] {
            let node = &session.nodes[&id];
            assert_eq!(node.state.image.suffix.last(), Some(&unknown_entry));
            assert_eq!(node.state.image.commit, 4);
            assert_eq!(node.machine.applied(), 4);
        }
        assert_eq!(
            session.nodes[&4].state.image.suffix.last().unwrap().index,
            4
        );
        assert_eq!(session.nodes[&4].machine.applied(), 4);
        // Do NOT reconnect the live leader: it would repair4 and erase the
        // exact stale incoming / fresh outgoing-only recovery counterexample.
        drop(session);
        let mut new_command = conformance::command(3, Action::Mutate { payload: 77 });
        new_command.home = 4;
        let mut new_original = None;
        let mut new_entry = None;
        for reopen in 0..2 {
            let mut factory = V2StoreFactory::new(root.clone());
            let mut stores = BTreeMap::new();
            for id in [1, 2, 3, 4] {
                let mut store = factory.open(instance(id), None).unwrap();
                if reopen == 0 {
                    let state = store.load().unwrap();
                    assert_eq!(
                        state.image.suffix.last().unwrap().index,
                        if id == 4 { 4 } else { 5 }
                    );
                }
                stores.insert(id, store);
            }
            let mut recovered = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
            assert_eq!(
                recovered.leader(),
                Ok(1),
                "fresh outgoing-only voter must lawfully recover joint authority"
            );
            assert_eq!(recovered.view().unwrap().configuration, joint.configuration);
            assert_eq!(recovered.submit(create), Ok(Some(original)));
            assert_eq!(
                recovered.applied_entry(original.index),
                Ok(original_entry.clone())
            );
            assert_eq!(
                recovered.replay(original_entry.clone()),
                Ok(ReplayResult::Application(original))
            );
            for (receipt, entry) in [(&joined, &joined_entry), (&joint, &joint_entry)] {
                assert_eq!(
                    recovered.configuration_outcome(receipt.intent.key),
                    Ok(Some(receipt.clone()))
                );
                assert_eq!(
                    recovered.configure(receipt.intent.clone()),
                    Ok(Some(receipt.clone()))
                );
                assert_eq!(recovered.applied_entry(receipt.index), Ok(entry.clone()));
                assert_eq!(
                    recovered.replay(entry.clone()),
                    Ok(ReplayResult::Configuration(receipt.clone()))
                );
            }
            // Expected original result was held before drop; actual new-term
            // noop/reconciliation must commit Entry5, not manufacture a reply.
            assert_eq!(
                recovered.outcome(unknown.request),
                Ok(Some(expected_unknown))
            );
            assert_eq!(recovered.submit(unknown), Ok(Some(expected_unknown)));
            assert_eq!(recovered.applied_entry(5), Ok(unknown_entry.clone()));
            assert_eq!(
                recovered.replay(unknown_entry.clone()),
                Ok(ReplayResult::Application(expected_unknown))
            );
            for id in [1, 2, 3, 4] {
                let node = &recovered.nodes[&id];
                assert!(node.state.image.commit >= 5);
                assert_eq!(
                    node.machine.history[&5],
                    (
                        unknown_entry.clone(),
                        ReplayResult::Application(expected_unknown)
                    )
                );
            }
            if reopen == 0 {
                let accepted = conformance::submit_accepted(
                    &mut recovered,
                    new_command,
                    glade_raft_q3_api::Resource {
                        payload: 77,
                        ..conformance::created_resource(4)
                    },
                );
                new_entry = Some(recovered.applied_entry(accepted.index).unwrap());
                new_original = Some(accepted);
            } else {
                let accepted = new_original.unwrap();
                let entry = new_entry.as_ref().unwrap();
                assert_eq!(recovered.submit(new_command), Ok(Some(accepted)));
                assert_eq!(recovered.applied_entry(accepted.index), Ok(entry.clone()));
                assert_eq!(
                    recovered.replay(entry.clone()),
                    Ok(ReplayResult::Application(accepted))
                );
                assert_eq!(
                    recovered.resource(100),
                    Ok(Some(glade_raft_q3_api::Resource {
                        payload: 77,
                        ..conformance::created_resource(4)
                    }))
                );
                recovered
                    .control(Control::Disconnect {
                        nodes: isolated.clone(),
                    })
                    .unwrap();
                let before = recovered.view().unwrap().committed;
                let mut denied = conformance::command(4, Action::Mutate { payload: 99 });
                denied.home = 4;
                assert_eq!(recovered.submit(denied), Ok(None));
                assert_eq!(recovered.outcome(denied.request), Ok(None));
                assert_eq!(recovered.view().unwrap().committed, before);
                for node in recovered.nodes.values() {
                    assert_eq!(
                        node.machine.application.resource(100),
                        Some(glade_raft_q3_api::Resource {
                            payload: 77,
                            ..conformance::created_resource(4)
                        })
                    );
                }
            }
            drop(recovered);
        }
        drop(directory);
    }
}
