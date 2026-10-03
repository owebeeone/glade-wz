use super::{command, create, intent};
use crate::{Action, Control, Error, QualificationSession};

/// QS-001/002/003: checkpoint keeps complete externally retained receipt/bytes.
pub fn snapshot_original_receipt(session: &mut dyn QualificationSession) {
    let original = session.submit(create()).expect("create").expect("receipt");
    let mutation = command(2, Action::Mutate { payload: 23 });
    let updated = session
        .submit(mutation)
        .expect("mutation")
        .expect("receipt");
    let original_entry = session
        .applied_entry(original.index)
        .expect("complete original envelope");
    let checkpoint = session.checkpoint().expect("complete checkpoint");
    assert!(!checkpoint.application.is_empty());
    assert_eq!(checkpoint.index, session.view().expect("view").committed);
    session
        .install(checkpoint)
        .expect("atomic install/compaction");
    session.control(Control::Restart).expect("reopen");
    assert_eq!(session.replay(original_entry.clone()), Ok(Some(original)));
    let mut changed_entry = original_entry;
    changed_entry.bytes.push(99);
    assert_eq!(
        session.replay(changed_entry),
        Err(Error::ConflictingReplay {
            index: original.index
        })
    );
    assert_eq!(session.outcome(original.request), Ok(Some(original)));
    assert_eq!(session.submit(create()), Ok(Some(original)));
    assert_eq!(session.outcome(updated.request), Ok(Some(updated)));
    assert_eq!(session.submit(mutation), Ok(Some(updated)));
    let changed = command(2, Action::Mutate { payload: 24 });
    assert_eq!(session.submit(changed), Err(Error::RetryConflict));
    assert_eq!(
        session
            .resource(100)
            .expect("resource")
            .expect("payload")
            .payload,
        23
    );
}

/// QS-004: current disclosure and retirement survive snapshot, original retired
/// create remains retryable for an authorized reader; reserved name cannot reset.
pub fn snapshot_policy_retirement(session: &mut dyn QualificationSession) {
    let original = session.submit(create()).expect("create").expect("receipt");
    let permission = command(
        2,
        Action::SetPermission {
            principal: 10,
            write: false,
            disclose: false,
        },
    );
    let revoked = session
        .submit(permission)
        .expect("ordered policy")
        .expect("policy receipt");
    let mut retire = command(3, Action::Retire);
    retire.policy_frontier = revoked.index;
    session.submit(retire).expect("retire").expect("receipt");
    let checkpoint = session.checkpoint().expect("checkpoint");
    session.install(checkpoint).expect("install");
    session.control(Control::Restart).expect("reopen");
    assert_eq!(session.submit(create()), Ok(Some(original)));
    let resource = session.resource(100).expect("resource").expect("tombstone");
    assert!(resource.retired);
    assert_eq!(resource.name, 40);
    let mut later_create = create();
    later_create.request.sequence = 4;
    later_create.policy_frontier = revoked.index;
    let refused = session
        .submit(later_create)
        .expect("ordered post-retire create")
        .expect("retained refusal");
    assert_eq!(
        refused.outcome,
        crate::Outcome::Rejected(crate::Rejection::Retired)
    );
    let mut reuse_name = later_create;
    reuse_name.request.resource = 101;
    reuse_name.request.sequence = 5;
    let refused = session
        .submit(reuse_name)
        .expect("ordered name reuse")
        .expect("retained refusal");
    assert_eq!(
        refused.outcome,
        crate::Outcome::Rejected(crate::Rejection::NameConflict)
    );
    let mut unauthorized = create().request;
    unauthorized.principal = 10;
    assert_eq!(session.outcome(unauthorized), Err(Error::Unauthorized));
}

/// QS-005/006: wrong cut/configuration/opaque bytes refuse before serving.
pub fn snapshot_mismatch(session: &mut dyn QualificationSession) {
    session.submit(create()).expect("create").expect("receipt");
    let original = session.checkpoint().expect("checkpoint");
    let mut foreign = original.clone();
    foreign.binding.group = 71;
    assert_eq!(session.install(foreign), Err(Error::WrongBinding));
    let mut wrong = original.clone();
    wrong.configuration.voters = vec![1, 4];
    assert_eq!(session.install(wrong), Err(Error::Quarantined));
    let mut corrupt = original.clone();
    corrupt.application.push(255);
    assert_eq!(session.install(corrupt), Err(Error::Quarantined));
    let mut bad_cut = original;
    bad_cut.index = u64::MAX;
    assert_eq!(session.install(bad_cut), Err(Error::CapacityExhausted));
}

/// QS-003: retained private movement envelope preserves old-generation fence.
pub fn snapshot_movement_fence(session: &mut dyn QualificationSession) {
    session.submit(create()).expect("create").expect("receipt");
    let cut = session.view().expect("view").committed;
    let moved = session
        .submit(command(
            2,
            Action::Move {
                home: 2,
                successor_applied: Some(cut),
            },
        ))
        .expect("host-verified full successor cut")
        .expect("move receipt");
    let snapshot = session.checkpoint().expect("checkpoint");
    session.install(snapshot).expect("install");
    session.control(Control::Restart).expect("restart");
    assert_eq!(session.outcome(moved.request), Ok(Some(moved)));
    let resource = session.resource(100).expect("resource").expect("moved");
    assert_eq!((resource.home, resource.generation), (2, 2));
    let delayed = session
        .submit(command(3, Action::Mutate { payload: 99 }))
        .expect("ordered old work")
        .expect("terminal refusal");
    assert_eq!(
        delayed.outcome,
        crate::Outcome::Rejected(crate::Rejection::StaleGeneration)
    );
    assert_eq!(session.resource(100), Ok(Some(resource)));
}

/// QS-007/QM-003: learner catch-up after source log compaction needs the complete
/// snapshot plus suffix, then retained original outcomes survive promotion.
pub fn snapshot_learner_catchup_and_membership(session: &mut dyn QualificationSession) {
    let original = session.submit(create()).expect("create").expect("receipt");
    let checkpoint = session.checkpoint().expect("complete checkpoint");
    session.install(checkpoint).expect("compact source log");
    let view = session.view().expect("view");
    let added = session
        .configure(intent(
            1,
            view.configuration.index,
            crate::Change::AddLearner { node: 4 },
        ))
        .expect("bound authorized learner")
        .expect("configuration receipt");
    let mutation = command(2, Action::Mutate { payload: 23 });
    let updated = session
        .submit(mutation)
        .expect("ordinary post-admission suffix")
        .expect("mutation receipt");
    session
        .control(Control::CatchUp { node: 4 })
        .expect("actual snapshot plus suffix transfer");
    let view = session.view().expect("full learner view");
    let cut = view
        .nodes
        .iter()
        .find(|cut| cut.node == 4)
        .expect("learner");
    assert!(cut.durable >= view.committed);
    assert_eq!(cut.applied, view.committed);
    let received_snapshot = cut.snapshot_index.expect("actual snapshot received");
    assert!(received_snapshot >= added.index);
    assert!(received_snapshot < view.committed);
    session
        .configure(intent(
            2,
            view.configuration.index,
            crate::Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .expect("verified complete restored cut")
        .expect("joint receipt");
    session
        .control(Control::Restart)
        .expect("joint restored snapshot configuration");
    assert_eq!(session.submit(create()), Ok(Some(original)));
    assert_eq!(session.submit(mutation), Ok(Some(updated)));
}
