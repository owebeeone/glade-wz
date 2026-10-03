#[test]
fn correctly_versioned_nonjoint_leave_and_nested_joint_refuse_without_logging_or_outcome() {
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
        .join(format!("q3-rem-nested-leave-{}", std::process::id()));
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
    let wrong_leave = conformance::intent(
        1,
        session.view().unwrap().configuration.index,
        Change::LeaveJoint,
    );
    let before_view = session.view().unwrap();
    let before_states: Vec<_> = session
        .nodes
        .values()
        .map(|node| node.state.clone())
        .collect();
    let before_files: Vec<_> = session
        .nodes
        .keys()
        .map(|id| std::fs::read(root.join(format!("node-{id}"))).unwrap())
        .collect();
    assert_eq!(session.configure(wrong_leave.clone()), Err(Error::NotJoint));
    assert_eq!(session.configuration_outcome(wrong_leave.key), Ok(None));
    assert_eq!(session.view(), Ok(before_view));
    assert_eq!(
        session
            .nodes
            .values()
            .map(|node| node.state.clone())
            .collect::<Vec<_>>(),
        before_states
    );
    assert_eq!(
        session
            .nodes
            .keys()
            .map(|id| std::fs::read(root.join(format!("node-{id}"))).unwrap())
            .collect::<Vec<_>>(),
        before_files
    );
    let joined = session
        .configure(conformance::intent(2, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    assert_eq!(joined.outcome, ConfigOutcome::Accepted);
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
    assert_eq!(joint.outcome, ConfigOutcome::Accepted);
    let nested = conformance::intent(
        4,
        joint.index,
        Change::EnterJoint {
            voters: vec![1, 2, 3],
        },
    );
    let before_view = session.view().unwrap();
    let before_states: Vec<_> = session
        .nodes
        .values()
        .map(|node| node.state.clone())
        .collect();
    let before_files: Vec<_> = session
        .nodes
        .keys()
        .map(|id| std::fs::read(root.join(format!("node-{id}"))).unwrap())
        .collect();
    assert_eq!(
        session.configure(nested.clone()),
        Err(Error::JointInProgress)
    );
    assert_eq!(session.configuration_outcome(nested.key), Ok(None));
    assert_eq!(session.view(), Ok(before_view));
    assert_eq!(
        session
            .nodes
            .values()
            .map(|node| node.state.clone())
            .collect::<Vec<_>>(),
        before_states
    );
    assert_eq!(
        session
            .nodes
            .keys()
            .map(|id| std::fs::read(root.join(format!("node-{id}"))).unwrap())
            .collect::<Vec<_>>(),
        before_files
    );
    let left = session
        .configure(conformance::intent(5, joint.index, Change::LeaveJoint))
        .unwrap()
        .unwrap();
    assert_eq!(left.outcome, ConfigOutcome::Accepted);
    assert!(left.configuration.voters_outgoing.is_empty());
    assert_eq!(session.configuration_outcome(wrong_leave.key), Ok(None));
    assert_eq!(session.configuration_outcome(nested.key), Ok(None));
    assert_eq!(session.submit(conformance::create()), Ok(Some(original)));
    drop(session);
    drop(directory);
}
