//! Complete typed results for original configuration/noop indexes.
use super::membership::learner;
use super::{accepted_create, command, intent};
use crate::{
    Action, Change, ConfigOutcome, ConfigRejection, Control, Error, QualificationSession,
    ReplayResult, StoredEntry,
};

/// QS-002/QM-008: original accepted/refused configuration results and real
/// committed noops survive snapshot/restart without conflating missing indexes.
pub fn snapshot_typed_configuration_and_noop_replay(session: &mut dyn QualificationSession) {
    let application = accepted_create(session);
    let accepted = learner(session);
    assert_eq!(accepted.outcome, ConfigOutcome::Accepted);
    session
        .control(Control::CatchUp { node: 4 })
        .expect("full learner cut");
    let view = session.view().expect("view");
    session
        .control(Control::QueueApplicationBeforeNextConfiguration {
            command: command(2, Action::Mutate { payload: 23 }),
        })
        .expect("logged proof becomes stale at actual index");
    let refused = session
        .configure(intent(
            2,
            view.configuration.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        ))
        .expect("ordered refusal")
        .expect("retained refused configuration receipt");
    assert_eq!(
        refused.outcome,
        ConfigOutcome::Refused(ConfigRejection::IncompleteLearner)
    );
    let accepted_entry = session
        .applied_entry(accepted.index)
        .expect("accepted original configuration envelope");
    let refused_entry = session
        .applied_entry(refused.index)
        .expect("refused original configuration envelope");
    let application_entry = session
        .applied_entry(application.index)
        .expect("application original envelope");
    // The bounded assembly performs an initial campaign and commits noop index 1.
    let noop = session
        .applied_entry(1)
        .expect("actual retained initial election noop");
    assert!(application.index > 1);
    let checkpoint = session.checkpoint().expect("all original typed history");
    session.install(checkpoint).expect("snapshot/compaction");
    session
        .control(Control::Restart)
        .expect("reopen full retained originals");
    for (entry, expected) in [
        (accepted_entry, ReplayResult::Configuration(accepted)),
        (refused_entry, ReplayResult::Configuration(refused)),
        (application_entry, ReplayResult::Application(application)),
        (noop, ReplayResult::Noop { index: 1 }),
    ] {
        assert_eq!(session.replay(entry.clone()), Ok(expected));
        let mut changed = entry;
        changed.bytes.push(99);
        assert_eq!(
            session.replay(changed.clone()),
            Err(Error::ConflictingReplay {
                index: changed.index
            })
        );
    }
    let missing = session
        .view()
        .expect("applied view")
        .committed
        .checked_add(1)
        .expect("bounded next index");
    assert_eq!(session.applied_entry(missing), Err(Error::Missing));
    assert_eq!(
        session.replay(StoredEntry {
            index: missing,
            term: 1,
            bytes: vec![1]
        }),
        Err(Error::Missing)
    );
}
