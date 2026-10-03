//! Reusable bounded behavioral specifications, not a passing model.
use crate::{
    Action, Command, ConfigIntent, ConfigKey, Outcome, QualificationSession, Receipt, RequestId,
    Resource,
};

mod joint_exit;
mod membership;
mod replay;
mod snapshot;
mod store;
pub use joint_exit::*;
pub use membership::*;
pub use replay::*;
pub use snapshot::*;
pub use store::*;

pub fn command(sequence: u64, action: Action) -> Command {
    Command {
        request: RequestId {
            scope: 7,
            resource: 100,
            incarnation: 1,
            principal: 1,
            sequence,
        },
        generation: 1,
        home: 1,
        policy_frontier: 0,
        action,
    }
}

pub fn create() -> Command {
    let mut create = command(
        1,
        Action::Create {
            name: 40,
            home: 1,
            payload: 11,
        },
    );
    create.generation = 0;
    create.home = 0;
    create
}

pub fn intent(sequence: u64, configuration: u64, change: crate::Change) -> ConfigIntent {
    ConfigIntent {
        key: ConfigKey {
            scope: 7,
            group: 70,
            principal: 1,
            sequence,
        },
        expected_configuration: configuration,
        change,
    }
}

/// Complete independent expected resource for successful fixture creation.
pub fn created_resource(home: u64) -> Resource {
    Resource {
        id: 100,
        name: 40,
        incarnation: 1,
        generation: 1,
        home,
        payload: 11,
        retired: false,
    }
}

/// Require complete Accepted state for success setup; a terminal refusal is
/// never sufficient merely because a Receipt exists.
pub fn submit_accepted(
    session: &mut dyn QualificationSession,
    command: Command,
    expected: Resource,
) -> Receipt {
    let receipt = session
        .submit(command)
        .expect("accepted command")
        .expect("applied receipt");
    assert_eq!(receipt.request, command.request);
    assert!(receipt.index > 0);
    assert_eq!(receipt.outcome, Outcome::Accepted(expected));
    receipt
}

pub fn accepted_create(session: &mut dyn QualificationSession) -> Receipt {
    submit_accepted(session, create(), created_resource(1))
}
