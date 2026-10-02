//! Draft trusted committed-log boundary for the bounded Raft experiment.
//!
//! This std-only contract does not admit client proposals, authenticate callers,
//! freeze Glade wire bytes, or promise disk durability. Numeric identity and
//! policy inputs stand for already validated fixtures in scope 7. A production
//! bridge must establish canonical identity, signatures, and policy evidence.

/// Durable retry identity, including the complete identity namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestId {
    pub scope: u64,
    pub resource: u64,
    pub incarnation: u64,
    pub principal: u64,
    pub sequence: u64,
}

/// Complete fixture command; equality is exact canonical fixture equality.
///
/// `policy_frontier` identifies the committed policy used for new admission.
/// Exact retries are resolved before generation/policy precondition checks,
/// subject to the serving hop's current disclosure permission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Command {
    pub request: RequestId,
    pub generation: u64,
    pub home: u64,
    pub policy_frontier: u64,
    pub action: Action,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Create {
        name: u64,
        home: u64,
        payload: u64,
    },
    Mutate {
        payload: u64,
    },
    /// Atomic fixture movement is only a partial RA-002/005 witness.
    ///
    /// The harness supplies `successor_applied` after checking complete applied
    /// data. It is trusted evidence, never a client assertion of readiness.
    /// Separate BeginMove/Activate, signatures, and real readiness are deferred.
    Move {
        home: u64,
        successor_applied: Option<u64>,
    },
    Retire,
    /// Fixture governance is ordered; it does not prove root/signature ancestry.
    SetPermission {
        principal: u64,
        write: bool,
        disclose: bool,
    },
    /// Always excluded from this profile; no external sink is implemented.
    ExternalEffect {
        code: u64,
    },
}

/// Complete retained numeric payload and lifecycle state, not a digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resource {
    pub id: u64,
    pub name: u64,
    pub incarnation: u64,
    pub generation: u64,
    pub home: u64,
    pub payload: u64,
    pub retired: bool,
}

/// An applied, retained terminal outcome under the memory-only fixture profile.
///
/// A success is not evidence of power-loss survival or physical failure domains.
/// Exact retries MUST return the original receipt, including its original index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub request: RequestId,
    pub index: u64,
    pub outcome: Outcome,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Accepted(Resource),
    Rejected(Rejection),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rejection {
    WrongScope,
    UnknownResource,
    NameConflict,
    IncarnationConflict,
    RetryConflict,
    StaleGeneration,
    WrongHome,
    PolicyFrontier,
    Unauthorized,
    IncompleteSuccessor,
    UnsupportedEffect,
    Retired,
    CapacityExhausted,
}

/// Host ordering errors are distinct from applied application refusals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyError {
    IndexGap {
        expected: u64,
        received: u64,
    },
    ConflictingReplay {
        index: u64,
    },
    /// Q0's refusing provider must not be mistaken for an implementation.
    NotQualified,
}

/// Replaceable deterministic host of an already committed, ordered log.
///
/// The host MUST apply contiguous indexes, including noops, once. An exact
/// replay of an index MUST recover its original result; different content at an
/// applied index MUST fail. A command's terminal outcome MUST be retained before
/// a receipt is returned. The caller supplies committed history; this interface
/// does not establish consensus, client authorization, or persistence itself.
pub trait CommittedMachine {
    fn apply(
        &mut self,
        index: u64,
        command: Option<Command>,
    ) -> Result<Option<Receipt>, ApplyError>;

    /// Trusted host lookup of the retained original outcome.
    ///
    /// Serving a client MUST separately enforce current disclosure permission.
    /// `None` means no outcome is available here, never proof of noncommit.
    fn lookup(&self, request: RequestId) -> Option<Receipt>;
}
