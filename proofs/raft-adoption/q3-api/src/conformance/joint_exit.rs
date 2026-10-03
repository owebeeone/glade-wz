//! Ordered current-home protection through joint exit, not only joint entry.
use super::membership::joint;
use super::{command, create, created_resource, intent, submit_accepted};
use crate::{
    Action, Change, ConfigOutcome, ConfigRejection, Control, Error, QualificationSession, Resource,
};

/// QM-007/008: outgoing-only home is permitted while joint. Leave must recheck
/// the actual predecessor state, including placement queued after admission.
/// Restart, exact terminal retry, movement/retirement and a NEW exit intent are
/// separate obligations; resolving a blocker cannot rewrite the old refusal.
pub fn joint_exit_rechecks_home(
    session: &mut dyn QualificationSession,
    queued: bool,
    move_home: bool,
) {
    let entered = joint(session);
    let mut placement = create();
    placement.action = Action::Create {
        name: 40,
        home: 3,
        payload: 11,
    };
    let leave = intent(3, entered.configuration.index, Change::LeaveJoint);
    if queued {
        session
            .control(Control::QueueApplicationBeforeNextConfiguration { command: placement })
            .expect("placement ordered after leave admission, before actual apply");
    } else {
        submit_accepted(session, placement, created_resource(3));
        session
            .control(Control::Restart)
            .expect("joint restart with outgoing-only live home");
        assert_eq!(
            session.view().expect("joint view").configuration,
            entered.configuration
        );
    }
    let refused = session
        .configure(leave.clone())
        .expect("ordered leave eligibility")
        .expect("retained terminal refusal");
    assert_eq!(
        refused.outcome,
        ConfigOutcome::Refused(ConfigRejection::HomeInUse)
    );
    assert_eq!(refused.configuration, entered.configuration);
    assert_eq!(session.resource(100), Ok(Some(created_resource(3))));
    let placement_receipt = session
        .outcome(placement.request)
        .expect("placement outcome")
        .expect("original queued or direct placement");
    assert_eq!(
        placement_receipt.outcome,
        crate::Outcome::Accepted(created_resource(3))
    );
    assert!(placement_receipt.index < refused.index);
    session
        .control(Control::Restart)
        .expect("restart retained refusal while joint");
    assert_eq!(session.configure(leave.clone()), Ok(Some(refused.clone())));
    assert_eq!(
        session.configuration_outcome(leave.key),
        Ok(Some(refused.clone()))
    );
    assert_eq!(
        session.view().expect("still joint").configuration,
        entered.configuration
    );

    let mut resolution = command(2, Action::Retire);
    resolution.home = 3;
    if move_home {
        session
            .control(Control::CatchUp { node: 2 })
            .expect("driver verifies complete durable incoming successor");
        let ready = session.view().expect("complete successor cut");
        let cut = ready.committed;
        let successor = ready
            .nodes
            .iter()
            .find(|node| node.node == 2)
            .expect("incoming successor");
        assert!(successor.durable >= cut);
        assert_eq!(successor.applied, cut);
        resolution.action = Action::Move {
            home: 2,
            successor_applied: Some(cut),
        };
        submit_accepted(
            session,
            resolution,
            Resource {
                home: 2,
                generation: 2,
                ..created_resource(3)
            },
        );
    } else {
        submit_accepted(
            session,
            resolution,
            Resource {
                retired: true,
                ..created_resource(3)
            },
        );
    }
    // Same terminal key retains its original refusal even after eligibility changes.
    assert_eq!(session.configure(leave), Ok(Some(refused)));
    assert_eq!(
        session.configure(intent(4, 0, Change::LeaveJoint)),
        Err(Error::StaleConfiguration)
    );
    let exited = session
        .configure(intent(5, entered.configuration.index, Change::LeaveJoint))
        .expect("new authorized intent after resolved blocker")
        .expect("accepted stable exit");
    assert_eq!(exited.outcome, ConfigOutcome::Accepted);
    assert_eq!(exited.configuration.voters, vec![1, 2, 4]);
    assert!(exited.configuration.voters_outgoing.is_empty());
    assert!(exited.configuration.index > entered.configuration.index);
}
