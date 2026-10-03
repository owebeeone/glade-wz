//! Reusable bounded behavioral specifications, not a passing model.
use crate::{Action, Command, ConfigIntent, ConfigKey, RequestId};

mod membership;
mod snapshot;
mod store;
pub use membership::*;
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
    command(
        1,
        Action::Create {
            name: 40,
            home: 1,
            payload: 11,
        },
    )
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
