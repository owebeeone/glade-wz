//! Private B0 proposed contracts. No carrier, executor, election or durable store.
//! Observations are trusted test diagnostics; they grant no leadership authority.
use std::future::Future;
use std::pin::Pin;

pub type OwnedTask = Pin<Box<dyn Future<Output = ()> + 'static>>;
pub type RpcFuture = Pin<Box<dyn Future<Output = Result<ProtocolMessage, RpcError>> + 'static>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Carrier {
    RaftRs,
    OpenRaft,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeKey {
    pub group: u64,
    pub node: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClockDomain(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeScope {
    pub carrier: Carrier,
    pub session: u64,
    pub node: NodeKey,
    pub incarnation: u64,
    pub domain: ClockDomain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInstant {
    pub domain: ClockDomain,
    pub nanos: u64,
}
/// Raw range input; providers MUST reject invalid bounds before mutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SampleRange {
    pub lower: u64,
    pub upper_exclusive: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawPurpose {
    Constructor,
    TimeoutReset,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceError {
    InvalidRange,
    Overflow,
    ScriptExhausted,
    OutOfRange,
    BackwardTime,
    WrongDomain,
    InvalidToken,
    Stopped,
    NotDue,
    RegistrationFailed,
    ClockFailed,
    EntropyFailed,
    TaskFailed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriverError {
    NotQualified,
    InvalidConfiguration,
    InvalidToken,
    Stopped,
    NotDue,
    Protocol,
    Source(SourceError),
}
impl From<SourceError> for DriverError {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

/// Caller-supplied instance capability. No ambient fallback is permitted.
pub trait ElectionSources {
    fn now(&self) -> Result<LogicalInstant, SourceError>;
    fn sample(
        &mut self,
        purpose: DrawPurpose,
        lower: u64,
        upper_exclusive: u64,
    ) -> Result<u64, SourceError>;
    /// Only the explicit drive API may advance this clock. Validation precedes mutation.
    fn advance(&mut self, to: LogicalInstant) -> Result<(), SourceError>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkId {
    pub scope: NodeScope,
    pub sequence: u64,
}
/// Every registration carries its actual owned future, including delayed work.
/// No registration or spawn may poll inline. Wrappers own typed joins/results.
pub enum WorkSpec {
    Runnable(OwnedTask),
    At {
        deadline: LogicalInstant,
        task: OwnedTask,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkState {
    WaitingDeadline,
    Runnable,
    Pending,
    Complete,
    Cancelled,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkView {
    pub id: WorkId,
    pub deadline: Option<LogicalInstant>,
    pub state: WorkState,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkPoll {
    Pending,
    Complete,
}
/// Instance scheduler consumes real futures; a wake marks eligibility, never polls.
/// One `poll` action performs at most one poll of that future. Completed/cancelled
/// IDs are terminal. Cancellation drops the future and its owned joins/RPC guards.
pub trait ScheduledWork {
    fn register(&mut self, work: WorkSpec) -> Result<WorkId, SourceError>;
    fn spawn(&mut self, task: OwnedTask) -> Result<WorkId, SourceError>;
    fn advance(&mut self, to: LogicalInstant) -> Result<(), SourceError>;
    fn poll(&mut self, work: WorkId) -> Result<WorkPoll, SourceError>;
    fn cancel(&mut self, work: WorkId) -> Result<(), SourceError>;
    fn inventory(&self) -> Vec<WorkView>;
    fn stop(&mut self) -> Result<(), SourceError>;
}
pub struct SourceContext {
    pub scope: NodeScope,
    pub sources: Box<dyn ElectionSources>,
    pub work: Box<dyn ScheduledWork>,
}
impl SourceContext {
    /// No engine/logger/store initializer runs here; dependencies already exist.
    pub fn new(
        scope: NodeScope,
        sources: Box<dyn ElectionSources>,
        work: Box<dyn ScheduledWork>,
    ) -> Self {
        Self {
            scope,
            sources,
            work,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageToken {
    pub issuer: NodeScope,
    pub destination: NodeScope,
    pub sequence: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RpcId {
    pub scope: NodeScope,
    pub sequence: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolMessage {
    pub carrier: Carrier,
    pub bytes: Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolKind {
    VoteRequest,
    VoteResponse,
    AppendRequest,
    AppendResponse,
    Other,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RpcError {
    Dropped,
    TimedOut,
    Cancelled,
    InvalidToken,
    Stopped,
}
pub struct PendingRpc {
    pub id: RpcId,
    pub message: MessageToken,
    pub response: RpcFuture,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inbound {
    pub token: MessageToken,
    pub message: ProtocolMessage,
    pub rpc: Option<RpcId>,
    /// Explicit issued RPC reply correlation; requests and one-way messages are None.
    pub reply_to: Option<RpcId>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageState {
    Held,
    Deliverable,
    Consumed,
    Dropped,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MessageView {
    pub token: MessageToken,
    pub kind: ProtocolKind,
    pub state: MessageState,
    pub rpc: Option<RpcId>,
    /// Explicit issued RPC reply correlation; requests and one-way messages are None.
    pub reply_to: Option<RpcId>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportAction {
    Release(MessageToken),
    Hold(MessageToken),
    Drop(MessageToken),
    Duplicate(MessageToken),
    Resolve { rpc: RpcId, reply: MessageToken },
    Timeout(RpcId),
    Cancel(RpcId),
}
/// RPC futures remain pending until an explicit Resolve/Timeout/Cancel action.
/// Drop of a message does not resolve an RPC. Duplicate allocates a fresh token
/// preserving original bytes; terminal tokens and wrong scopes refuse unchanged.
pub trait TransportEndpoint {
    fn emit(
        &mut self,
        to: NodeKey,
        kind: ProtocolKind,
        message: ProtocolMessage,
    ) -> Result<MessageToken, DriverError>;
    fn request(
        &mut self,
        to: NodeKey,
        kind: ProtocolKind,
        message: ProtocolMessage,
    ) -> Result<PendingRpc, DriverError>;
    fn respond(
        &mut self,
        rpc: RpcId,
        kind: ProtocolKind,
        message: ProtocolMessage,
    ) -> Result<MessageToken, DriverError>;
    fn take(&mut self, token: MessageToken) -> Result<Inbound, DriverError>;
    fn control(&mut self, action: TransportAction) -> Result<Vec<Event>, DriverError>;
    fn messages(&self) -> Vec<MessageView>;
    fn pending_rpcs(&self) -> Vec<RpcId>;
    fn stop(&mut self) -> Result<(), DriverError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolImage {
    pub carrier: Carrier,
    pub opaque: Vec<u8>,
}
/// B0-only memory protocol state; successful publication makes no disk claim.
pub trait MemoryProtocolStore {
    fn load(&self) -> Result<ProtocolImage, DriverError>;
    fn publish(&mut self, image: ProtocolImage) -> Result<(), DriverError>;
}
pub trait InstanceLogger {
    fn record(&mut self, event: Event);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimingProfile {
    RaftRs {
        quantum_ns: u64,
        election_ticks: u64,
        max_election_ticks_exclusive: u64,
        heartbeat_ticks: u64,
        pre_vote: bool,
        check_quorum: bool,
    },
    OpenRaft {
        ticker_ns: u64,
        min_election_ms: u64,
        max_election_ms_exclusive: u64,
        heartbeat_ms: u64,
        election_enabled: bool,
        heartbeat_enabled: bool,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedConfiguration {
    pub group: u64,
    pub voters: [u64; 3],
    pub authorized_genesis: bool,
    pub timing: TimingProfile,
}
pub struct NodeInputs {
    pub sources: SourceContext,
    pub store: Box<dyn MemoryProtocolStore>,
    pub endpoint: Box<dyn TransportEndpoint>,
    pub configuration: FixedConfiguration,
    pub logger: Box<dyn InstanceLogger>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeState {
    Running,
    Stopped,
    Failed(SourceError),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    PreCandidate,
    Leader,
    Learner,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoteIdentity {
    pub carrier: Carrier,
    pub opaque: Vec<u8>,
}
/// Initial comparison uses standard single-leader-per-term modes only.
/// Full candidate/committed vote identity remains separately retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElectionEpoch {
    pub carrier: Carrier,
    pub group: u64,
    pub term: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogMark {
    pub index: u64,
    pub opaque_identity: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectionView {
    pub scope: NodeScope,
    pub state: NodeState,
    pub role: Role,
    pub leader: Option<NodeKey>,
    pub vote: Option<VoteIdentity>,
    pub epoch: Option<ElectionEpoch>,
    pub committed: Vec<LogMark>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumePolicy {
    HostSingleTick,
    PinnedTickerSingleWake,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimingBoundary {
    pub before: LogicalInstant,
    pub first_eligible: LogicalInstant,
    pub sampled: u64,
    pub resume: ResumePolicy,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeInventory {
    pub clock: LogicalInstant,
    pub work: Vec<WorkView>,
    pub messages: Vec<MessageView>,
    pub pending_rpcs: Vec<RpcId>,
    pub timing: TimingBoundary,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    SourceRead {
        scope: NodeScope,
        now: LogicalInstant,
    },
    SourceDraw {
        scope: NodeScope,
        purpose: DrawPurpose,
        range: SampleRange,
        value: u64,
    },
    WorkRegistered {
        work: WorkView,
    },
    WorkPolled {
        work: WorkId,
        result: WorkPoll,
    },
    WorkCancelled {
        work: WorkId,
    },
    Outbound {
        message: MessageView,
    },
    RpcPending {
        rpc: RpcId,
    },
    RpcResolved {
        rpc: RpcId,
    },
    Observation(ElectionView),
    SourceFailed(SourceError),
    Stopped {
        scope: NodeScope,
    },
}
/// All driver state mutation is explicit. Advance changes time/enables work only;
/// drive polls one eligible owned future or executes one scheduled engine tick.
/// Tokens are checked against live registries (public values alone confer no
/// capability). Source failure stops participation. Stop cancels all scoped work
/// and pending RPCs, drops/joins tasks and excludes old-incarnation callbacks.
pub trait ElectionNode {
    fn advance(&mut self, to: LogicalInstant) -> Result<(), DriverError>;
    /// Checked now + delta; overflow MUST refuse before any time/engine mutation.
    fn advance_by(&mut self, delta_ns: u64) -> Result<(), DriverError>;
    fn drive(&mut self, work: WorkId) -> Result<Vec<Event>, DriverError>;
    fn receive(&mut self, token: MessageToken) -> Result<(), DriverError>;
    fn transport(&mut self, action: TransportAction) -> Result<Vec<Event>, DriverError>;
    fn observe(&self) -> Result<ElectionView, DriverError>;
    fn inventory(&self) -> Result<NodeInventory, DriverError>;
    fn stop(&mut self) -> Result<(), DriverError>;
}
