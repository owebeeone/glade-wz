//! Q0 refusing providers for compiling qualification specifications.
//!
//! This file deliberately implements no application, Raft driver, authentication,
//! transport, persistence, or readiness verification. All proposed mutations
//! remain unknown and no retained outcome exists. Q1 may replace these providers
//! only after the contract/consumer review gate. The numeric profile is scope 7,
//! administrator 1 and user 10; voters are fixed complete-data fixture members.

use glade_raft_adoption_api::{
    Action, ApplyError, Command, CommittedMachine, Receipt, RequestId, Resource,
};

/// Unknown cannot be interpreted as a terminal application refusal/noncommit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proposal {
    Unknown,
}

/// Temporary refusing committed-log provider; no data or outcomes are accepted.
pub struct Application;

impl Application {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl CommittedMachine for Application {
    fn apply(
        &mut self,
        _index: u64,
        _command: Option<Command>,
    ) -> Result<Option<Receipt>, ApplyError> {
        Err(ApplyError::NotQualified)
    }

    fn lookup(&self, _request: RequestId) -> Option<Receipt> {
        None
    }
}

/// Public deterministic schedule surface for the future real-Raft experiment.
///
/// Q0 methods compile but exercise no protocol. Q1 must use explicit delivery,
/// manual campaigns, and no sleeps or election ticks. Three voters are the
/// positive fixture, two are an intentional either-loss availability negative.
pub struct Cluster;

impl Cluster {
    pub fn new(_voters: &[u64]) -> Self {
        Self
    }

    pub fn campaign(&mut self, _voter: u64) {}

    pub fn drain(&mut self) {}

    pub fn isolate(&mut self, _voter: u64) {}

    pub fn heal(&mut self) {}

    pub fn leader(&self) -> Option<u64> {
        None
    }

    pub fn applied(&self, _voter: u64) -> u64 {
        0
    }

    pub fn propose(&mut self, _voter: u64, _command: Command) -> Proposal {
        Proposal::Unknown
    }

    /// Transient response to the exact attempted command, subject to disclosure.
    /// A changed retry returns RetryConflict here while trusted retained lookup
    /// MUST preserve the original outcome. This accessor is not durable history.
    pub fn reply(&self, _voter: u64, _command: Command) -> Option<Receipt> {
        None
    }

    /// Serving-hop outcome accessor MUST apply current disclosure permission;
    /// unlike trusted `CommittedMachine::lookup`, it may withhold retained data.
    pub fn outcome(&self, _voter: u64, _request: RequestId) -> Option<Receipt> {
        None
    }

    pub fn resource(&self, _voter: u64, _resource: u64) -> Option<Resource> {
        None
    }

    /// Builds a movement request from trusted observed successor readiness.
    ///
    /// Q0 cannot verify readiness, so supplies no witness. Q1 MUST derive it from
    /// the successor's complete applied frontier; raw client input cannot mint
    /// this evidence. Atomic movement remains partial against RA-002/005.
    pub fn move_command(
        &self,
        request: RequestId,
        generation: u64,
        home: u64,
        successor: u64,
    ) -> Command {
        Command {
            request,
            generation,
            home,
            policy_frontier: 0,
            action: Action::Move {
                home: successor,
                successor_applied: None,
            },
        }
    }
}
