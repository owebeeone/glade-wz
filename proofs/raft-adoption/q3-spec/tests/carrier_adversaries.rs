mod common;
use glade_raft_q3_api::{Action, Change, Control, Outcome, QualificationSession, conformance};
#[test]
fn dynamic_voter_four_can_own_new_resource_after_joint() {
    let _store_consumer: fn() -> common::OwnedStore = common::store;
    let mut session = common::session();
    let current = session.view().unwrap();
    session
        .configure(conformance::intent(
            1,
            current.configuration.index,
            Change::AddLearner { node: 4 },
        ))
        .unwrap()
        .unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let current = session.view().unwrap();
    session
        .configure(conformance::intent(
            2,
            current.configuration.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .unwrap()
        .unwrap();
    let mut command = conformance::create();
    command.action = Action::Create {
        name: 40,
        home: 4,
        payload: 11,
    };
    let receipt = session.submit(command).unwrap().unwrap();
    assert_eq!(
        receipt.outcome,
        Outcome::Accepted(glade_raft_q3_api::Resource {
            home: 4,
            ..conformance::created_resource(1)
        })
    );
}

#[test]
fn adding_promoted_voter_as_learner_refuses_before_proposal_and_keeps_group_usable() {
    use glade_raft_q3_api::Error;
    let mut session = common::session();
    let joined = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let joint = session
        .configure(conformance::intent(
            2,
            joined.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .unwrap()
        .unwrap();
    for (sequence, configuration) in [(3, joint.index), (5, joint.index + 1)] {
        let before = session.view().unwrap();
        assert_eq!(
            session.configure(conformance::intent(
                sequence,
                configuration,
                Change::AddLearner { node: 4 }
            )),
            Err(Error::InvalidImage)
        );
        assert_eq!(
            session.view().unwrap(),
            before,
            "unsupported demotion must not enter the log"
        );
        assert_eq!(
            session.configure(joint.intent.clone()),
            Ok(Some(joint.clone()))
        );
        if sequence == 3 {
            session
                .configure(conformance::intent(4, joint.index, Change::LeaveJoint))
                .unwrap()
                .unwrap();
        }
    }
    assert!(session.submit(conformance::create()).unwrap().is_some());
}

#[test]
fn learner_can_restore_actual_snapshot_after_source_compacted_beyond_join() {
    let mut session = common::session();
    let original = session.submit(conformance::create()).unwrap().unwrap();
    assert_eq!(
        original.outcome,
        Outcome::Accepted(conformance::created_resource(1))
    );
    let joined = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    let checkpoint = session.checkpoint().unwrap();
    assert_eq!(checkpoint.index, joined.index);
    session.install(checkpoint).unwrap();
    session.control(Control::Restart).unwrap();
    let checkpoint = session.checkpoint().unwrap();
    assert!(checkpoint.index > joined.index);
    session.install(checkpoint).unwrap();
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let view = session.view().unwrap();
    assert!(view.nodes.iter().any(|node| node.node == 4
        && node.applied == view.committed
        && node.durable == view.committed));
    assert_eq!(session.submit(conformance::create()), Ok(Some(original)));
    assert_eq!(session.configure(joined.intent.clone()), Ok(Some(joined)));
}

#[test]
fn explicit_joint_with_identical_voters_is_real_joint_then_can_leave() {
    use glade_raft_q3_api::ConfigOutcome;
    let mut session = common::session();
    let original = conformance::accepted_create(&mut session);
    let joint = session
        .configure(conformance::intent(
            1,
            0,
            Change::EnterJoint {
                voters: vec![1, 2, 3],
            },
        ))
        .unwrap()
        .unwrap();
    assert_eq!(joint.outcome, ConfigOutcome::Accepted);
    assert_eq!(joint.configuration.voters, vec![1, 2, 3]);
    assert_eq!(joint.configuration.voters_outgoing, vec![1, 2, 3]);
    let left = session
        .configure(conformance::intent(2, joint.index, Change::LeaveJoint))
        .unwrap()
        .unwrap();
    assert_eq!(left.outcome, ConfigOutcome::Accepted);
    assert!(left.configuration.voters_outgoing.is_empty());
    assert_eq!(
        session.configure(joint.intent.clone()),
        Ok(Some(joint.clone()))
    );
    assert_eq!(
        session.replay(session.applied_entry(joint.index).unwrap()),
        Ok(glade_raft_q3_api::ReplayResult::Configuration(joint))
    );
    assert_eq!(session.submit(conformance::create()), Ok(Some(original)));
}

#[test]
fn combined_actual_snapshot_suffix_joint_restart_policy_retirement_and_explicit_exit() {
    use glade_raft_q3_api::{ConfigOutcome, Error, Resource};
    let mut session = common::session();
    let original = conformance::accepted_create(&mut session);
    let original_entry = session.applied_entry(original.index).unwrap();
    let cp = session.checkpoint().unwrap();
    session.install(cp).unwrap();
    let joined = session
        .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
        .unwrap()
        .unwrap();
    let mutation = conformance::command(2, Action::Mutate { payload: 23 });
    let mutated = conformance::submit_accepted(
        &mut session,
        mutation,
        Resource {
            payload: 23,
            ..conformance::created_resource(1)
        },
    );
    session.control(Control::CatchUp { node: 4 }).unwrap();
    let view = session.view().unwrap();
    let learner = view.nodes.iter().find(|node| node.node == 4).unwrap();
    assert_eq!(learner.snapshot_index, Some(joined.index));
    assert!(learner.applied > joined.index);
    assert_eq!(learner.applied, view.committed);
    let joint = session
        .configure(conformance::intent(
            2,
            joined.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .unwrap()
        .unwrap();
    assert_eq!(joint.outcome, ConfigOutcome::Accepted);
    let revoked = conformance::submit_accepted(
        &mut session,
        conformance::command(
            3,
            Action::SetPermission {
                principal: 10,
                write: false,
                disclose: false,
            },
        ),
        Resource {
            payload: 23,
            ..conformance::created_resource(1)
        },
    );
    let mut retire = conformance::command(4, Action::Retire);
    retire.policy_frontier = revoked.index;
    let retired = conformance::submit_accepted(
        &mut session,
        retire,
        Resource {
            payload: 23,
            retired: true,
            ..conformance::created_resource(1)
        },
    );
    session.control(Control::Restart).unwrap();
    assert_eq!(session.submit(conformance::create()), Ok(Some(original)));
    assert_eq!(session.submit(mutation), Ok(Some(mutated)));
    assert_eq!(session.submit(retire), Ok(Some(retired)));
    let mut forbidden = original.request;
    forbidden.principal = 10;
    assert_eq!(session.outcome(forbidden), Err(Error::Unauthorized));
    assert_eq!(
        session.replay(original_entry),
        Ok(glade_raft_q3_api::ReplayResult::Application(original))
    );
    let exit = session
        .configure(conformance::intent(3, joint.index, Change::LeaveJoint))
        .unwrap()
        .unwrap();
    assert_eq!(exit.outcome, ConfigOutcome::Accepted);
    assert!(exit.configuration.voters_outgoing.is_empty());
    assert_eq!(
        session.resource(100),
        Ok(Some(Resource {
            payload: 23,
            retired: true,
            ..conformance::created_resource(1)
        }))
    );
}
