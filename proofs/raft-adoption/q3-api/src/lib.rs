//! DRAFT Q3 private qualification boundaries, not a production API or wire freeze.
//! See GladeRaftConfigurationSnapshotContract.md. No adapter is qualified here.

pub use glade_raft_adoption_api::{
    Action, Command, Outcome, Receipt, Rejection, RequestId, Resource,
};
pub use glade_raft_durability_api::StoredEntry;

/// Portable identity excludes the local node and mutable membership.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupIdentity {
    pub scope: u64,
    pub group: u64,
    pub application_profile: u64,
}

/// Immutable existing-group identity. Mutable membership lives in Configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instance {
    pub scope: u64,
    pub group: u64,
    pub node: u64,
    pub application_profile: u64,
}

/// Complete raft ConfState equivalent, without carrier types. Lists are sorted,
/// unique, bounded to nodes 1..=4. Explicit joint transitions only in this profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Configuration {
    pub voters: Vec<u64>,
    pub learners: Vec<u64>,
    pub voters_outgoing: Vec<u64>,
    pub learners_next: Vec<u64>,
    pub auto_leave: bool,
    /// Index of the last ACCEPTED configuration mutation, not a Raft term.
    /// Ordered refusals advance application but do not change this version.
    pub index: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConfigKey {
    pub scope: u64,
    pub group: u64,
    pub principal: u64,
    pub sequence: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Change {
    AddLearner { node: u64 },
    EnterJoint { voters: Vec<u64> },
    LeaveJoint,
}

/// Numeric principals 1 and 2 are explicitly supplied trusted authority fixtures.
/// Exact equality binds every field; clients cannot supply catch-up evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigIntent {
    pub key: ConfigKey,
    pub expected_configuration: u64,
    pub change: Change,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigRejection {
    Unauthorized,
    StaleConfiguration,
    IncompleteLearner,
    HomeInUse,
    JointInProgress,
    NotJoint,
    InvalidChange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigOutcome {
    Accepted,
    Refused(ConfigRejection),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigReceipt {
    pub intent: ConfigIntent,
    pub outcome: ConfigOutcome,
    pub index: u64,
    pub configuration: Configuration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NotQualified,
    Unauthorized,
    WrongBinding,
    StaleConfiguration,
    RetryConflict,
    ConflictingReplay { index: u64 },
    IncompleteLearner,
    HomeInUse,
    JointInProgress,
    NotJoint,
    NoQuorum,
    Missing,
    AlreadyExists,
    Locked,
    IncompatibleVersion,
    InvalidImage,
    Quarantined,
    Poisoned,
    IoUnknown,
    CapacityExhausted,
    RevisionConflict { expected: u64, actual: u64 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeCut {
    pub node: u64,
    /// Actual durable snapshot cut received by this node, never caller evidence.
    pub snapshot_index: Option<u64>,
    pub durable: u64,
    pub applied: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct View {
    pub configuration: Configuration,
    pub committed: u64,
    pub nodes: Vec<NodeCut>,
}

/// Harness controls only: deterministic delivery and store lifecycle.
/// CatchUp must perform real protocol/store/application work in the eventual
/// provider. It MUST NOT assign counters or assert caller-supplied readiness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Control {
    Drain,
    Disconnect {
        nodes: Vec<u64>,
    },
    Reconnect,
    CatchUp {
        node: u64,
    },
    Restart,
    LoseNextConfigurationReply,
    /// Queue an already admitted application command ahead of the next config
    /// proposal, after its driver captures readiness at the old committed cut.
    /// At apply, the logged proof is stale for the true predecessor index. This
    /// is a deterministic schedule, never a mutable local availability oracle.
    QueueApplicationBeforeNextConfiguration {
        command: Command,
    },
}

/// Complete coherent checkpoint. Application bytes are the versioned private
/// full-state envelope, including original applied history; no digest proxy.
/// Storage validates framing/cuts; host validates/replays application semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub binding: GroupIdentity,
    pub index: u64,
    pub term: u64,
    pub configuration: Configuration,
    pub application: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    pub term: u64,
    pub vote: u64,
    pub commit: u64,
    /// Durably reconstructible, validated application cut. A privately restored
    /// incoming snapshot may lead the live serving counter until publication
    /// succeeds and the host installs that candidate; it cannot serve early.
    pub applied: u64,
    pub configuration: Configuration,
    pub checkpoint: Option<Checkpoint>,
    /// Complete original entries from checkpoint.index + 1, or 1 without one.
    pub suffix: Vec<StoredEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub revision: u64,
    pub instance: Instance,
    pub image: Image,
}

/// V2 single-writer local publication boundary, with full-image compare-and-sync.
/// After publication starts errors are unknown and poison until explicit reopen.
/// Successful load is structural storage admission, not authorization to serve.
pub trait CheckpointStore {
    fn load(&mut self) -> Result<State, Error>;
    fn publish(&mut self, expected_revision: u64, image: Image) -> Result<State, Error>;
}

/// Original result for one applied index. Missing history is Error::Missing,
/// never Noop. Configuration results include accepted AND refused receipts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplayResult {
    Application(Receipt),
    Configuration(ConfigReceipt),
    Noop { index: u64 },
}

/// Private replaceable experiment composition. All controls are trusted test
/// inputs; this interface is outside production paths. Result None is unknown,
/// not noncommit; all outcomes require current disclosure at serving boundaries.
pub trait QualificationSession {
    fn control(&mut self, input: Control) -> Result<(), Error>;
    fn view(&self) -> Result<View, Error>;
    fn configure(&mut self, intent: ConfigIntent) -> Result<Option<ConfigReceipt>, Error>;
    fn configuration_outcome(&self, key: ConfigKey) -> Result<Option<ConfigReceipt>, Error>;
    fn submit(&mut self, command: Command) -> Result<Option<Receipt>, Error>;
    fn outcome(&self, request: RequestId) -> Result<Option<Receipt>, Error>;
    fn resource(&self, id: u64) -> Result<Option<Resource>, Error>;
    fn checkpoint(&self) -> Result<Checkpoint, Error>;
    fn install(&mut self, checkpoint: Checkpoint) -> Result<(), Error>;
    /// Trusted committed exact-index replay; envelope includes original carrier
    /// Entry bytes and private readiness. Changed bytes MUST fail.
    /// Missing/unapplied indexes return Error::Missing, never Noop.
    fn applied_entry(&self, index: u64) -> Result<StoredEntry, Error>;
    fn replay(&mut self, entry: StoredEntry) -> Result<ReplayResult, Error>;
}

/// Opt-in reusable specifications. Store fixtures are deliberately opaque and
/// cannot certify application parsing, real I/O, Raft or physical durability.
pub mod conformance;
