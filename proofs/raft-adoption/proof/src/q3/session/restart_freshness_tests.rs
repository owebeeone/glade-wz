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
