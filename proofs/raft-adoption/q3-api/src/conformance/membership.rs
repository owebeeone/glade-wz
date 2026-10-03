use super::{accepted_create, command, create, intent};
use crate::{Action, ConfigKey, Control, Error, QualificationSession};

pub(super) fn learner(session: &mut dyn QualificationSession) -> crate::ConfigReceipt {
    let view = session.view().expect("known configured scope");
    let receipt = session
        .configure(intent(
            1,
            view.configuration.index,
            crate::Change::AddLearner { node: 4 },
        ))
        .expect("authorized bound intent")
        .expect("applied configuration receipt");
    assert_eq!(receipt.outcome, crate::ConfigOutcome::Accepted);
    assert_eq!(receipt.configuration.voters, vec![1, 2, 3]);
    assert_eq!(receipt.configuration.learners, vec![4]);
    receipt
}

pub(super) fn joint(session: &mut dyn QualificationSession) -> crate::ConfigReceipt {
    learner(session);
    session
        .control(Control::CatchUp { node: 4 })
        .expect("real full data and applied catch-up");
    let view = session.view().expect("view");
    let receipt = session
        .configure(intent(
            2,
            view.configuration.index,
            crate::Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .expect("verified readiness")
        .expect("joint applied");
    assert_eq!(receipt.outcome, crate::ConfigOutcome::Accepted);
    assert_eq!(receipt.configuration.voters, vec![1, 2, 4]);
    assert_eq!(receipt.configuration.voters_outgoing, vec![1, 2, 3]);
    assert!(!receipt.configuration.auto_leave);
    receipt
}

/// QM-001/002: discovery/principal alone cannot authorize; changed bytes fail.
pub fn membership_authority_and_retry(session: &mut dyn QualificationSession) {
    let original = learner(session);
    assert_eq!(
        session.configure(original.intent.clone()),
        Ok(Some(original.clone()))
    );
    let mut changed = original.intent.clone();
    changed.change = crate::Change::AddLearner { node: 3 };
    assert_eq!(session.configure(changed), Err(Error::RetryConflict));
    let mut other_authority = intent(
        1,
        original.configuration.index,
        crate::Change::AddLearner { node: 4 },
    );
    other_authority.key.principal = 2;
    let separate = session
        .configure(other_authority.clone())
        .expect("second configured authority")
        .expect("distinct principal namespace");
    assert_eq!(separate.intent, other_authority);
    assert_ne!(separate.intent.key, original.intent.key);
    assert_eq!(
        session.configuration_outcome(separate.intent.key),
        Ok(Some(separate))
    );
    let mut denied = original.intent.clone();
    denied.key.principal = 10;
    assert_eq!(session.configure(denied), Err(Error::Unauthorized));
    assert_eq!(
        session.configuration_outcome(original.intent.key),
        Ok(Some(original.clone()))
    );
    assert_eq!(
        session.configuration_outcome(ConfigKey {
            principal: 10,
            ..original.intent.key
        }),
        Err(Error::Unauthorized)
    );
    let mut wrong = original.intent;
    wrong.key.scope = 8;
    assert_eq!(session.configure(wrong), Err(Error::WrongBinding));
}

/// QM-003/004: learner cannot vote, fake/unavailable catch-up cannot promote.
pub fn learner_unavailable_and_nonvoting(session: &mut dyn QualificationSession) {
    learner(session);
    session
        .control(Control::Disconnect { nodes: vec![4] })
        .expect("schedule");
    assert_eq!(
        session.control(Control::CatchUp { node: 4 }),
        Err(Error::IncompleteLearner)
    );
    let view = session.view().expect("view");
    assert_eq!(
        session.configure(intent(
            2,
            view.configuration.index,
            crate::Change::EnterJoint {
                voters: vec![1, 2, 4]
            }
        )),
        Err(Error::IncompleteLearner)
    );
    session.control(Control::Reconnect).expect("schedule");
    session
        .control(Control::Disconnect { nodes: vec![1, 2] })
        .expect("schedule");
    assert_eq!(session.submit(create()), Ok(None));
    assert_eq!(session.outcome(create().request), Ok(None));
}

/// QM-004/005: joint old and new majorities are separately required.
pub fn joint_authority(session: &mut dyn QualificationSession, isolated: Vec<u64>) {
    joint(session);
    session
        .control(Control::Disconnect { nodes: isolated })
        .expect("schedule");
    assert_eq!(session.submit(create()), Ok(None));
    assert_eq!(session.outcome(create().request), Ok(None));
    session.control(Control::Reconnect).expect("heal");
    session.control(Control::Drain).expect("drain");
    let applied_create = session
        .outcome(create().request)
        .expect("outcome")
        .expect("healed committed create");
    assert_eq!(
        applied_create.outcome,
        crate::Outcome::Accepted(super::created_resource(1))
    );
    let view = session.view().expect("view");
    let left = session
        .configure(intent(
            3,
            view.configuration.index,
            crate::Change::LeaveJoint,
        ))
        .expect("both majorities")
        .expect("stable receipt");
    assert_eq!(left.outcome, crate::ConfigOutcome::Accepted);
    assert_eq!(left.configuration.voters, vec![1, 2, 4]);
    assert!(left.configuration.voters_outgoing.is_empty());
}

/// QM-002/005/006: externally retain original intent and receipt across restart.
pub fn joint_lost_reply_restart(session: &mut dyn QualificationSession) {
    let original = joint(session);
    session
        .control(Control::Restart)
        .expect("recover disk images while joint");
    assert_eq!(
        session.view().expect("recovered").configuration,
        original.configuration
    );
    assert_eq!(
        session.configure(original.intent.clone()),
        Ok(Some(original.clone()))
    );
    let next = intent(3, original.configuration.index, crate::Change::LeaveJoint);
    session
        .control(Control::LoseNextConfigurationReply)
        .expect("reply fault");
    assert_eq!(session.configure(next.clone()), Ok(None));
    // Retain the complete first recovered receipt, then require exact identity
    // after a second restart. Q3 implementation additionally records pre-kill
    // receipt outside its worker; two fresh outputs alone are not that oracle.
    let receipt = session
        .configuration_outcome(next.key)
        .expect("lookup")
        .expect("committed lost reply");
    session.control(Control::Restart).expect("restart");
    assert_eq!(session.configure(next), Ok(Some(receipt.clone())));
    assert_eq!(
        session.configuration_outcome(receipt.intent.key),
        Ok(Some(receipt))
    );
}

/// QM-007: voter removal never silently rehomes or advances generation.
pub fn removal_preserves_home(session: &mut dyn QualificationSession) {
    accepted_create(session);
    let before = session.resource(100).expect("resource").expect("live home");
    learner(session);
    session
        .control(Control::CatchUp { node: 4 })
        .expect("catch-up");
    let view = session.view().expect("view");
    assert_eq!(
        session.configure(intent(
            2,
            view.configuration.index,
            crate::Change::EnterJoint {
                voters: vec![2, 3, 4]
            }
        )),
        Err(Error::HomeInUse)
    );
    assert_eq!(session.resource(100), Ok(Some(before)));
}

/// QM-003/008: a logged preproposal proof for cut C becomes stale when an
/// already admitted command precedes the configuration at C+2. Every replica
/// deterministically retains the same refusal; local liveness is irrelevant.
pub fn admitted_configuration_refusal_is_retained(session: &mut dyn QualificationSession) {
    accepted_create(session);
    learner(session);
    session
        .control(Control::CatchUp { node: 4 })
        .expect("catch-up");
    let before = session.view().expect("view").configuration;
    let proposal = intent(
        2,
        before.index,
        crate::Change::EnterJoint {
            voters: vec![1, 2, 4],
        },
    );
    session
        .control(Control::QueueApplicationBeforeNextConfiguration {
            command: command(2, Action::Mutate { payload: 23 }),
        })
        .expect("ordered queued command, no fabricated readiness");
    let refused = session
        .configure(proposal.clone())
        .expect("ordered revalidation")
        .expect("retained terminal refusal");
    assert_eq!(
        refused.outcome,
        crate::ConfigOutcome::Refused(crate::ConfigRejection::IncompleteLearner)
    );
    assert_eq!(refused.configuration, before);
    assert_eq!(session.configure(proposal), Ok(Some(refused.clone())));
    assert_eq!(
        session.configuration_outcome(refused.intent.key),
        Ok(Some(refused))
    );
    assert_eq!(session.view().expect("view").configuration, before);
}
