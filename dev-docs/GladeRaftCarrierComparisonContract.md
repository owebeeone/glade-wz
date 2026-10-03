# Q4-B carrier comparison — contract and ordered proof tranche

Date: 2026-10-03. Status: **DRAFT for Consistency/Safety review; no implementation, runtime verification, carrier selection or production activation accepted**.

This object specifies the next private comparison after the accepted Q3 source
`468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`. It is controlled by the
[Q4 integration plan](GladeRaftProductionIntegrationPlan.md),
[qualification plan](GladeRaftQualificationPlan.md),
[RA-001–012](GladeRaftAdoptionContract.md),
[implementation evaluation](GladeRaftImplementationEvaluation.md), and
[pinned source audit](GladeRaftQ4-CarrierAudit.md). Q3 acceptance establishes its
private manual-campaign/process-crash profile only. Source inspection identifies
work; neither that inspection nor Q3 selects the production crate.

Only `raft = 0.7.0` and `openraft = 0.9.25` are comparison candidates here. Both
MUST execute actual upstream engine algorithms through reviewed instance-owned
entropy/time adaptations. A wrapper selecting leaders, copying a leader field,
manual campaigns, or replacing the engine with an application simulation MUST
NOT satisfy this contract. The comparison remains outside production dependency
paths and preserves existing Q1–Q3 proofs and encodings.

## 1. Scope, order and independent exits

| Exit | Required object and proof | What acceptance establishes |
| --- | --- | --- |
| B0 — sources and automatic elections | Small common scheduling/observation contract; compiling behavioral RED consumers; reviewed adaptations for both pins; actual automatic-election success/failure/edge witnesses with memory protocol stores | Separately owned entropy, clocks and work reach constructors and runtime paths; actual automatic elections/failover under controlled schedules. No durable application or full comparison claim |
| B1 — complete application and real durability | Common application/retained-outcome consumer extension, carrier-neutral application envelope and carrier-specific protocol stores; same application, real-I/O, fault and restart matrix for both | Complete private application ordering and retained accepted/refused outcomes under the explicitly named storage profile |
| B2 — membership and complete snapshots | Common configuration/snapshot extension; learner/joint transitions, both-stage reply loss, complete application-cut readiness, install/compaction/recovery matrix for both | Private membership and snapshot parity, including preserved policy/retry/retirement/configuration evidence |
| B3 — comparison/selection record | B0+B1+B2 accepted for both, complete inventories and measured commands/costs; comparative review and explicit owner library decision | Evidence sufficient to decide a carrier for subsequent production integration; no production authority, deployment or activation approval |

B0 is the smallest useful first object. B1 and B2 are **mandatory independent
exits**, not optional follow-ups displaced by a successful election demonstration.
Before each new/recomposed boundary is implemented, its signatures, allocation,
compiling success/failure/edge RED consumers and review MUST settle. No B0 method
MAY quietly claim B1 durability; unsupported later operations MUST remain absent
from the B0 trait rather than return fabricated results. B3 MUST NOT select on an
incomplete comparison. A failed candidate stays a blocker and MUST NOT disappear
from the matrix. Rejecting it as a reason to abbreviate this mandate requires an
explicit owner decision and a reviewed amendment; this draft grants no shortcut
around the same application/fault journeys required by Q4.

Q4-C owns real canonical `(share, glade_id, key)`, declaration/profile/version/
parameters, creation root/incarnation, group/configuration, authenticated request
identity, complete canonical commands and ordered governance/evidence frontier.
This document MUST NOT choose those values, substitute numeric fixtures for
cryptography, or authorize production amendments. Q4-D/Q4-E still own real
transport/Records integration, independent domains, old-writer exclusion,
baseline cut and rollback fencing. `Ok`, hashes and `Heads` retain their existing
meaning. No launch, cold-join, default, desktop, enrollment or cutover change is
part of B0–B3. External effects remain excluded under RA-008.

## 2. Pins and permitted source adaptation

The inspected raft-rs crate VCS identity is
`10c6e9db6792b85c81784e44fc278f895d5f0ab0`; OpenRaft is
`8815cdba2826f74e848acef361ad03f93bb1c3f8` (`path_in_vcs = openraft`). The audited
OpenRaft archive SHA-256 is
`a97014fb78acb77be3a40ac2da305f6dd3a6b243f3a908ace87d29b3972eaafd`.
Implementation evidence MUST record original archive/checksum, exact adapted
source/tree digest, patch, features, compiler and lockfile. Version ranges are
not transitive pins. A codec/runtime/storage change is a separately reviewed
comparison change, not permission to change an accepted proof dependency.

For raft-rs, `RawNode::tick()` supplies logical time, while reset invokes ambient
`rand::thread_rng()` before a later timeout override could help. The adaptation
MUST pass an owned bounded-range entropy source into construction and use it at
every upstream timeout-reset site, including constructor/state transitions.
The interval remains the configured half-open min/max interval and reset
frequency remains upstream behavior. The host MUST preserve the Ready/LightReady
lifecycle, including the prohibition on state-changing calls while Ready is
outstanding. See pinned [ticks](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raw_node.rs#L338),
[election handler](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L1068),
and [entropy reset](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L2807).

For OpenRaft, a runtime *type* and static methods are insufficient. The adaptation
MUST add a constructor-supplied owned context and propagate that handle through
storage initialization, engine/vote handlers, ticker, core, replication, timeout,
sleep and spawn paths. Initial timeout sampling MUST use that context. The
inspected pin samples during EngineConfig construction; this contract MUST NOT
introduce per-election resampling merely to resemble raft-rs. Vote modification
time, committed-vote leader lease, extra delay for a greater known log, voter
eligibility and election-enable behavior MUST remain upstream behavior. Diagnostic
instant formatting MUST use logical instants or an explicit supplied conversion,
without ambient wall/local time. See pinned [runtime](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/async_runtime.rs#L25),
[constructor](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/raft/mod.rs#L230),
[initial timer](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/engine/engine_config.rs#L44),
[ticker](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/core/tick.rs#L81),
[election handler](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/core/raft_core.rs#L1440),
and [formatting](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/display_ext/display_instant.rs#L39).

Changing context plumbing does not authorize changing vote comparison, log
freshness, quorum arithmetic, lease/deadline tests, message validation, persistence
ordering or membership algorithms. Necessary deviations MUST be enumerated and
reviewed before running comparative acceptance. Footprint is measured as files,
changed lines, affected call paths, upstream divergence, adaptation-specific
regressions and upgrade/rebase effort; it is not an inferred reliability score.

The supplemental [adaptation inventory](GladeRaftB0-AdaptationInventory.md)
records the exact fallibility gaps. raft-rs transitions and ticks currently lack
the required error channel, and tick handlers discard step errors. Adaptations
MUST propagate source failures to the host, acquire/validate a new timeout before
installing reset-owned fields, and prevent emission/continued participation after
a failure. A later failure following other legitimate engine mutation MUST NOT
be reported as complete rollback. OpenRaft's sleep/join, vote rejection, timeout,
core API and fatal-result paths likewise MUST distinguish source failure from
normal protocol refusal or elapsed deadline. Constructor failure MUST clean up
work registered before timeout sampling or storage initialization completes.
These are source-derived required adaptations, not implemented witnesses.

An external unsigned, zero-based clock does not authorize simplifying upstream
time arithmetic. OpenRaft computes a pre-epoch `now - lease - 1ms` during vote
handling. Its private instant representation/mapping MUST preserve that signed
ordering without wrapping, clamping the comparison or altering the lease test.
Any mapping of an already-past sleep deadline to runnable work MUST retain the
original comparison semantics. Domain/overflow failures MUST use the reviewed
fatal-source path; an infallible operator or hidden ambient Instant is not a
substitute for that proof. Exact arithmetic/result changes need boundary RED and
source-adaptation review before comparative implementation acceptance.

The later actual adapters require constructor/API mutations in the adapted
carrier, not merely wrapper controls: raft-rs `RawNode`/`Raft` construction MUST
accept and retain the explicit entropy capability before any reset; OpenRaft
`Raft::new`, EngineConfig initialization and the runtime/time consumers MUST
accept/retain the supplied instance context and invoke instance methods through
it. A hidden settable timeout, `Default`-constructed source, type-selected static
`thread_rng()`/`now()`/`spawn()` implementation or global bridge MUST NOT serve as
that API mutation. The concrete patch/call graph is a reviewed B0 object.

## 3. Instance-owned source and lifecycle contract (B0)

`NodeKey` identifies fixture group and node; logical time is checked `u64`
nanoseconds plus an instance clock-domain identifier. All range/unit conversions
MUST be checked. No boundary type exposes an upstream Instant, protobuf Entry,
Tokio handle, filesystem path or runtime implementation. Private protocol
messages remain opaque carrier-specific envelopes and MUST NOT be described as
cross-carrier wire compatibility.

Before any engine, logger, storage initializer or task is constructed, the caller
MUST supply that node's `SourceContext`, memory protocol store, endpoint, validated
fixed configuration and instance logger. A context contains separately owned
entropy-stream state, monotonic-clock state and an instance-owned scheduler/task
scope. Clones MAY share state within that node scope; they MUST NOT discover it
through process globals, thread-locals, environment, a global registry or a static
factory. Two nodes MUST receive independently supplied handles. Explicitly supplied
equal test scripts are legal for a split-vote case; secretly giving every node
the same fixed seed from a static factory is not injection.

The following are proposed required operations, with no default implementation.
Their concrete compiler declaration belongs to the B0 contract/spec tranche.

| Boundary | Meaningful required operations | Input/output/failure contract |
| --- | --- | --- |
| `ElectionSources` | `now(&self) -> Result<LogicalInstant, SourceError>`; `sample(&mut self, purpose: DrawPurpose, lower: u64, upper_exclusive: u64) -> Result<u64, SourceError>` | Now is monotonic within this node's domain. Sample is in the supplied nonempty half-open interval; purpose identifies constructor/reset use for traceability. Invalid range, overflow, script exhaustion or source failure is explicit, never a fallback to ambient randomness/time |
| `ScheduledWork` | `register(&mut self, work: WorkSpec) -> Result<WorkId, SourceError>`; `spawn(&mut self, task: OwnedTask) -> Result<WorkId, SourceError>`; `cancel(&mut self, work: WorkId) -> Result<(), SourceError>` | WorkSpec is immediate runnable work or an absolute logical deadline in this domain. OwnedTask is an owned boxed standard-library future with unit completion; adapter wrappers retain typed task results/join channels inside their node scope. Registration/spawn never executes work inline. Work only progresses through explicit drive actions; no fake executor or static spawn bridge |
| `ElectionNode` | `advance(&mut self, to: LogicalInstant) -> Result<(), DriverError>`; `drive(&mut self, work: WorkId) -> Result<Vec<Event>, DriverError>`; `receive(&mut self, token: MessageToken) -> Result<(), DriverError>`; `observe(&self) -> Result<ElectionView, DriverError>`; `stop(&mut self) -> Result<(), DriverError>` | Advance only changes supplied time/enables due work; drive performs one explicitly selected eligible action. Receive selects one message emitted into this instance's transport queue. Observe is trusted test inspection, never a leadership authority API. Stop cancels/joins owned work; subsequent drive/receive refuse |

`SourceContext` is explicit composition of these supplied capabilities, not an
empty marker trait. Construction is an ordinary instance constructor taking its
owned dependencies; any reusable assembly object is itself caller-owned and
receives the context before invoking constructors. A type-only runtime adapter
MUST NOT claim conformance. The compiler consumer MUST construct two real source
objects and pass them separately to the two concrete adapter constructors.

`WorkId` and `MessageToken` are unique within a session, bound to carrier/node/
incarnation and single-use. A stale, foreign or consumed token returns
`InvalidToken` without state change. `Event` records outbound destination/token,
work registration/cancellation, source read/draw and election-role/vote/log
observations. It does not let the harness assign a role or vote. `ElectionView`
contains node identity, running/stopped/failed state, actual engine role, actual
leadership/vote identity and protocol log frontier. Full engine-specific vote
identity is retained as an opaque diagnostic token; a normalized term alone
MUST NOT erase OpenRaft's vote semantics. Queue/task inventories MUST be visible
to the scheduler so hidden background work cannot create progress.

The harness owns all delivery/drop/duplicate/reorder decisions. Duplicate delivery
uses a newly issued copy token with the same original bytes, not reuse of a
consumed token. OpenRaft RPC calls, responses and timeouts MUST remain pending
until explicit scheduled actions resolve them. Actual asynchronous futures/tasks
run, but no real sleep or uncontrolled timer/executor creates election progress.
raft-rs advances through scheduled tick actions; the host MUST record its
ticks-to-time mapping. A pause advances the node's clock while withholding its
work. Resume delivers due work using the pinned ticker/host policy: no harness
invented backlog of missed elections. B0 MUST record the concrete policy for
both and test it against each upstream timing rule.

Backward time, cross-domain instants, overflow and invalid controls MUST fail
without changing the engine or queues. An invalid entropy result MUST be caught
before it is installed. A source/task failure during construction returns an
explicit failure and cleans up registered work; during operation it stops that
node's participation and leaves outstanding proposals unknown. It MUST NOT
manufacture a leader, quorum or terminal noncommit result. No stale callback from
the stopped incarnation may mutate a restarted instance. Restart receives a fresh
explicit context and recovery inputs; source state is not presumed persistent.
Liveness after failure is only tested after an explicit successful restart and
eventually useful quorum scheduling.

## 4. Common application and observation contract (B1/B2)

Both candidates MUST run the same application transition rules, complete command
values, request namespace, policy/time inputs, expected accepted/refused outcomes,
configured capacity and external test oracle. They MUST NOT implement subtly
different machines inside their storage adapters. The accepted private
`CommittedMachine::apply/lookup` and Q3 consumers are starting points, not a new
production authority. The comparison MUST retain success **and ordered refusal**
outcomes; a returned RPC result/channel is not retained retry history.

The existing fixture atomic `Move` remains explicitly partial. B1 requires a
reviewed private application extension with separate `BeginMove` and `Activate`
commands, retained admission cut and successor application-readiness envelope.
That extension MUST begin with compiling RED tests against both consumers and
be independently reviewed before implementation. No public command can assert
its own readiness. Q4-C later supplies real canonical evidence; B1 numeric
evidence cannot ratify that production profile or silently redefine Q3 Action.

| Extension | Required operations | Observable semantics |
| --- | --- | --- |
| B1 `ApplicationNode` | `submit(command) -> Result<Submission, DriverError>`; `outcome(request, disclosure) -> Result<Lookup, DriverError>`; `inspect_application() -> Result<ApplicationView, DriverError>`; `restart(recovery, sources) -> Result<(), DriverError>` | Submit admits work asynchronously and returns a correlation token, explicit preadmission refusal, or unknown. A scheduled reply event may carry a complete retained terminal receipt. Lookup is Found(original receipt), Unknown or DisclosureDenied. No missing outcome means noncommit. Inspection is a trusted oracle, not client read permission |
| B1 persistence port | `load(instance, trusted_floor) -> Result<RecoveryImage, StoreError>`; `publish(expected_revision, image) -> Result<Publication, StoreError>` | Inputs retain complete application/terminal history and carrier protocol state with coherent cuts. Complete synchronized publication precedes durable completion. Uncertain failure poisons participation until reopen; malformed/missing/conflicting/rolled-back images quarantine, not bootstrap |
| B2 `ConfigurationNode` | `configure(intent) -> Result<Submission, DriverError>`; `configuration_outcome(key, disclosure) -> Result<LookupConfig, DriverError>` | Complete original ConfigIntent, accepted/refused ConfigReceipt, configuration and actual applied index persist. Exact retries precede current-cut/precondition rejection subject to disclosure. Matching final membership is insufficient |
| B2 snapshot/replay | `checkpoint(cut) -> Result<SnapshotImage, DriverError>`; `install(image) -> Result<Submission, DriverError>`; `applied_entry(index) -> Result<AppliedEnvelope, DriverError>`; `replay(envelope) -> Result<ReplayResult, DriverError>` | Snapshot has identity/profile/version, complete coherent application/policy/outcomes/tombstones/configuration/frontiers. Original typed application/configuration/noop replay is preserved. Missing history is Missing, not noop; changed same-index content refuses |

These extension names are proposed allocation/signature obligations, not present
implemented APIs. Their contract freeze MUST define complete concrete boundary
types and compiler witnesses before implementation; B0 acceptance does not
freeze unimplemented B1/B2 serialization. Durable protocol stores are
carrier-specific: Q3 `StoredEntry` includes original raft-rs Entry bytes. A new
versioned private application envelope can be common, but original protocol
bytes MUST remain intact in a carrier-tagged recovery/replay section. No byte
translation, loss of original history, or cross-carrier restart is implied.

Comparison equality means the same request has the same complete application
outcome and state, with exact original receipt recovery *within each carrier*.
Internal terms, noops, log indexes, vote tokens, scheduling work and protocol
message bytes MAY differ. Tests MUST NOT require equal raw logs or compare two
freshly reconstructed outputs as their only oracle. They hold complete original
commands/results and an independent expected application history externally.
Every serving lookup/reply applies current disclosure checks; a delayed old
reply MUST NOT leak a now-forbidden result. Accepted and refused terminal
outcomes are distinct from preadmission failures, pending and lost replies.

## 5. Requirements and trace

| ID | Obligation | Required witnesses / controlling trace |
| --- | --- | --- |
| BC-001 | Both exact adapted pins run actual algorithms and owned sources before construction | B0-01/02/03; source call-path/constructor compiler witness; RA-002/006, LBT-006/008/009 |
| BC-002 | Explicit schedules preserve relevant pinned election/vote/time behavior | B0-04–09 and per-carrier boundary tests; RA-002/010, LBT-008 |
| BC-003 | No global/static/thread-local/fixed-factory source substitution or hidden work | B0-02/03/10; graph/source/disabled-cfg inventory; process-global checks |
| BC-004 | Same complete application, exact retry and accepted/refused terminal histories | B1-APP matrix; RA-001–009; shared consumer/independent originals |
| BC-005 | Data-bearing durable quorum, correct publication ordering, faults and unknown recovery | B1-DISK matrix; RA-003/004/007/011; actual disk and process cuts |
| BC-006 | Authorized learner/joint transition and complete successor application cut | B2-CONFIG matrix; RA-005/010; complete original configuration outcomes |
| BC-007 | Coherent complete snapshots/replay/retirement/policy/retention after installation and compaction | B2-SNAP matrix; RA-004/006/009/011; mutants and actual crash recovery |
| BC-008 | Boundary ownership, minimal edges and isolated measured loops remain enforced | Architecture negative fixtures, compiler/conformance/affected consumers; LBT-001–012 |
| BC-009 | Measured source, graph, license/security/platform/build/runtime evidence precedes selection | B3 record with exact pins/commands/results/limits; no comparative performance claim from source |
| BC-010 | Canonical authority, receipts, legacy exclusion and activation remain separate | Q4-C/D/E explicit open entries; RA-001/006/008/012; no production-path dependency |

## 6. Exact first RED specifications — B0

The concrete first specification is `proofs/raft-carrier-comparison/`, a new
independent Cargo workspace. [B0 allocation](GladeRaftB0Allocation.md) supplies
the exact signatures, proposed feature/timing profiles, compiler witnesses and
recorded compiling RED. Its four std-only members are:

```text
api/                         glade-carrier-api (contract, std-only)
  src/lib.rs                 concrete B0 types and required traits
  tests/public_contract.rs   exhaustive typed compiler/boundary consumers
spec/                        glade-carrier-spec (harness/tool)
  src/lib.rs                 reusable generic B0 scenario/oracle functions
  tests/b0_election.rs       invokes every scenario for both provider labels
raft-rs/                     glade-carrier-raft-rs, REFUSING provider only
openraft/                    glade-carrier-openraft, REFUSING provider only
architecture-policy.json     exact four roles and all declared edges
check.sh                     explicit local architecture/source/global adoption
```

The local manifest MUST declare the same edition/toolchain floor as the accepted
proof unless a reviewed change is necessary, pin its own lockfile and contain
no upstream raft, OpenRaft, runtime, filesystem, full node or production dependency
at this stage. `spec` has only a normal dependency on `api`; its development edges
select both std-only refusing provider packages, each normally depending only on
API. No model package is needed until B1 justifies it. Before either provider
acquires an engine dependency, a reviewed runner/test allocation MUST preserve
the common consumers and move concrete composition out of spec's development
graph (or establish equivalent reviewed isolation). Cargo unit selection alone
does not exclude development dependencies.

`RefusingRaftRs` and `RefusingOpenRaft` MUST each implement the actual B0 trait,
accept caller-owned context/store/endpoint/configuration/logger through ordinary
constructors, and refuse behavior with `NotQualified`. They MUST NOT perform
elections, sample defaults, assign counters or call campaigns. Both labels call
the same generic `run_b0_<case>(provider)` functions, with exactly B0-01–10 selected
and no ignored cases. The compiler consumer also demonstrates separately owned
context values, checked complete error/event matches, typed future/work handles
and foreign-domain/token rejection inputs. Refusal scaffolds are compiler/RED
witnesses, never evidence for either real carrier.

The owner implementing that scaffold MUST file the actual failing assertion for
each case and then use these proposed commands from the workspace root. They
are specifications of the next work; none was run for this draft:

```sh
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-api --test public_contract
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-spec --test b0_election
proofs/raft-carrier-comparison/check.sh
cargo clippy --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml --workspace --all-targets -- -D warnings
```

The API compiler/type tests and structural gate should pass; the shared behavioral
test command MUST fail for the intentionally refusing providers at the stated
success/failure/edge assertions. Both refusing constructors retain dependencies
without probing sources or initializing an engine. Constructor-error tests fail
against that refusing object instead of the required exact source failure; normal
scenarios fail their specified positive observation or typed control assertions.
A dependency/tool/compiler failure does
not count. After contract review, concrete adapted-provider packages replace
the two labels' composition in explicitly selected adapter tests while reusing
the unchanged shared scenario functions. Keep the refusing scaffold and its
intentional RED command separate from accepted real-adapter GREEN commands;
never report an intentionally failing scaffold as a production test failure or
change expected outcomes merely to turn it green.

The B0 owner MUST first compile a carrier-free shared consumer against the actual
contract types and a deliberately refusing provider. Returning `NotQualified`
MAY demonstrate behavioral RED; import/build/tool errors MUST NOT. Each test
MUST reach and fail its stated positive assertion, including failure/edge tests
that require the correct typed refusal or preserved-state observation. Once
adapters exist, the same consumer runs against each real engine. Test-specific
carrier boundary assertions are additional obligations, not replacements.

The initial fixture is authorized known group G with three complete-data voters
1, 2 and 3, independent clock domains starting at zero, instance loggers and
memory protocol stores. Construction uses explicit validated configuration;
empty stores alone never authorize genesis. Test scripts provide timeout samples
at lower, interior and upper-minus-one bounds. Configured timeout units/ranges,
heartbeat/ticker intervals, pre-vote/check-quorum settings, OpenRaft election and
heartbeat enables, constructor/reset draws and mapping to logical nanoseconds
MUST be declared in the fixture before RED is recorded. Zero/invalid bounds
MUST NOT be silently repaired. A common scenario schedule is a fixed sequence
of named external actions; each adapter expands them into recorded engine work.

For liveness cases, after the stated fault prefix the consumer uses a deterministic
fair schedule: advance each live node by one configured ticker quantum, drive
eligible work in node/work-ID order and deliver queued traffic in emission order.
Completion must occur within 10,000 such rounds. This is a test-run termination
bound, not a product latency promise. At failure it dumps clocks, source traces,
queues, votes, roles and frontiers. The boundary tests additionally drive times
relative to the exact recorded upstream deadline rather than assuming identical
deadline formulas. No wall-clock sleep or timeout counts toward acceptance.

| Case | Exact action and required assertion |
| --- | --- |
| B0-01 `automatic_election_without_campaign` (success) | Construct all three, run only clock/work/message actions, then fair delivery. Assert one actual elected leader observed by a quorum, no manual campaign/trigger API calls, and engine-generated election/vote traffic. Refusing provider fails elected-leader assertion |
| B0-02 `constructor_and_reset_use_supplied_streams` (success/edge) | Supply distinguishing valid timeout scripts and trace sinks to nodes; record every constructor/reset sample. Cause automatic candidate/follower transitions. Assert each draw uses its own source and requested half-open interval, with raft-rs reset cadence and OpenRaft construction cadence separately. Change only node 2's script: nodes 1/3's draws remain identical. Equal explicit scripts are separately allowed |
| B0-03 `source_failure_has_no_ambient_fallback` (failure) | Supply exhausted/failed entropy, out-of-range sample and failed clock/work registration in separate constructor/runtime cuts. Assert precise source error, no fallback draw/time/spawn, no node participation after failure, pending work unknown, and no orphan tasks after failed construction |
| B0-04 `deadline_and_vote_rules_preserved` (edge) | Hold messages/work around each recorded timeout. Before a pinned engine's eligibility boundary, assert no election; at its first eligible scheduled tick, assert its actual election action. For raft-rs verify min/upper-minus-one reset and elapsed-tick behavior; for OpenRaft verify vote-modified time, committed-vote lease plus timeout, greater-log extra delay, and disabled-election/nonvoter guards. Assertions follow the pin's inequality and units, not a shared invented timeout formula |
| B0-05 `split_vote_eventual_useful_schedule` (edge) | Explicitly supply equal initial samples, delay competing vote traffic to cause a split/no-quorum prefix, then release recorded traffic with independent schedules. Assert no fabricated leader in the prefix and eventual actual quorum-elected leader in fair suffix; retain all candidate/vote evidence |
| B0-06 `minority_cannot_elect` (failure) | Isolate node 1 from 2/3 bidirectionally, advance/drive it beyond several eligible election deadlines while dropping all cross-partition traffic. Assert no quorum-established leadership on 1; schedule 2/3 fairly and assert their quorum leader. An obsolete local role indication never counts as quorum establishment |
| B0-07 `delayed_duplicate_reordered_vote_append` (edge) | Elect, delay old vote/append replies, force automatic failover by isolation, then duplicate/reorder those old messages across the new election. Assert no two quorum-established incompatible leaders for the same carrier vote epoch and no regressed committed prefix; stale messages pass through real engine validation |
| B0-08 `paused_leader_automatic_failover_and_resume` (success/edge) | Pause leader work/traffic while advancing clocks and scheduling other two fairly. Assert automatic successor election. Resume old work/queued traffic: assert old node converges to current vote/leader without resetting stores/history or gaining authority from elapsed time. Verify each carrier's recorded resume policy and no invented missed-tick burst |
| B0-09 `independent_clock_schedules` (edge) | Advance only node 1 and drive its due work, then advance 2, then 3. Assert no implicit time/work advancement on others. Repeat the exact explicit sources/schedule in fresh instances and compare each carrier's own normalized trace. Different carrier traces need only satisfy common invariants |
| B0-10 `invalid_controls_and_stop_lifecycle` (failure/edge) | Try backward/cross-domain/overflow time and stale/foreign/consumed work/message tokens. Assert typed refusal and unchanged pre/post engine/queue state. Stop with pending RPC/timer work; assert cancellation/join, subsequent drive/receive refusal and stale callback exclusion from a fresh incarnation |

Oracle self-tests MUST fail mutants that return a synthetic leader without engine
traffic, use a static fixed stream, bypass the source on reset, force an early
OpenRaft election by removing lease/greater-log delay, advance all clocks when
one is advanced, and allow paused background work. Failure must be attributed
to the invariant assertion, not a mutant compile failure. Source tracing alone
cannot prove absence of ambient calls; the source inventories below are mandatory.

## 7. Mandatory B1 and B2 matrices before selection

B1-APP MUST run identical complete known-scope genesis/mapping fixtures, concurrent
Create/name conflict, independent roots/scope/principal/request namespaces,
Mutate, separate BeginMove/Activate, Retire, ordered policy grant/revocation,
external-effect refusal, bounded-capacity refusal and exact index replay against
both carriers. Exercise accepted and ordered refused terminal results, changed
bytes under one request ID, lost reply then leadership change, current disclosure
denial then permitted recovery, delayed old-home command after activation,
pre-cut committed replay, incomplete successor/policy/retry cut, offline projection
write refusal and metadata-only majority. Automatic elections MUST drive every
leadership change; a Q3 manual-campaign success is reusable evidence only where
the unchanged application assertion actually applies. A check-then-local-append
mutant and outcome-eviction mutant MUST fail the common oracle.

B1-DISK MUST name the actual filesystem, storage implementation, sync/error
semantics and process-crash guarantee for each adapter, without inferring
power-loss survival. Execute both at vote persistence, log publication before
durable completion, commit publication, application/outcome publication before
reply, acknowledged outcome, reply loss and restart. At each cut inject errors
before/after writing and before/after sync, torn/corrupt/incomplete histories,
capacity exhaustion and independent trusted-floor rollback detection. Unknown
must stay unknown until complete recovery; failed publication stops participation.
Restart/leader change MUST compare full externally held original accepted/refused
outcomes and payload/policy/fence state. Include actual supervised process kills
at durable-before-apply and acknowledged cuts for both. Early raft-rs dependent
message release, early OpenRaft `LogFlushed` completion, receipt before durable
application, and missing-outcome reconstruction mutants MUST fail. Memory success
does not substitute for these cuts. Independent physical domains remain Q4-D/E.

B2-CONFIG MUST preserve the existing complete ConfigKey/ConfigIntent/ConfigReceipt
semantics: unauthorized/conflicting/missing mapping, nonvoter catch-up through
actual store/application paths, lagging complete data versus mere replication
position, stale predecessor readiness, authorized explicit joint/uniform stages,
old/new quorum loss, lost replies before/after each stage, restart in joint,
accepted/refused configuration outcomes, exact retry after configuration change,
changed intent under one key, home-in-use removal, and empty-store reset refusal.
OpenRaft's automatic second-stage membership convenience MUST NOT erase the
durable logical intent or original outcome between joint and uniform stages.
Catch-up/readiness MUST observe complete application, policy, outcomes and cut,
not assign counters or treat `add_learner(blocking=true)` as application readiness.

B2-SNAP MUST cover local checkpoint and incoming install at a lagging node,
publication before serving, compaction then restart, old/new snapshot rejection,
wrong identity/profile/version, incompatible or incoherent configuration/cuts,
syntactically valid mutations to application/outcome/policy/tombstone/config maps,
missing originals, original typed app/config/noop replay and changed-index replay.
Preserve accepted/refused results, retirement fences and disclosure policy through
snapshot/install/reconstruction. Kill at durable-snapshot-before-install/apply and
after acknowledged snapshot-dependent results; hold originals externally.
Partial install/publication must quarantine or recover only the permitted coherent
history, never expose a partial serving candidate. Missing-history/noop and
digest-only/partial-state snapshot mutants MUST fail. Physical journal reclamation
or cross-carrier store migration is not implied by log compaction.

All matrices MUST record exact deterministic schedule, input, expected outcome,
carrier-specific observation mapping and result. Deferred canonical crypto,
physical power loss, independent domains, linearizable reads and external sinks
remain named gates; fixture authentication is never signing evidence.

## 8. Package allocation, graphs and verification tiers

Start alongside `proofs/raft-adoption`, with proposed private comparison packages
in a separately adopted nested workspace. Names below are proposals to be frozen
with compiler consumers; this contract does not create packages or edit policy.

| Proposed boundary | Role / rationale | Allowed conceptual compile-time edges |
| --- | --- | --- |
| `carrier-comparison-api` | Contract: meaningful source/work/election operations and later reviewed app/config/store operations; no concrete engine types | Small existing contract/data APIs only where reused; no concrete carrier/runtime/store/node |
| `carrier-comparison-model` | Pure: deterministic complete application transitions/typed envelopes from explicit time/policy inputs; no service trait is needed for a value transformation | Comparison API and minimal existing contract/data boundaries |
| `carrier-comparison-spec` | Harness/tool: shared normal consumer and independent oracle; no production dependencies | Normal API/model only; concrete adapters are dev-only selected test providers |
| `carrier-raft-rs` / `carrier-openraft` | Replaceable implementation: real engine drivers implement the named common contracts; justified concrete upstream/runtime edges remain at these boundaries | API/model plus its exact adapted carrier and strictly needed runtime/codec; no production Glade node |
| Concrete comparison disk adapters | Implementation: injected storage/lifecycle contracts with actual I/O conformance | Small persistence API plus required carrier-private data codecs; no full harness dependency |
| Comparison runners | Harness/tool: explicit schedule, I/O faults and owned worker processes | API/spec and selected adapters; remain outside every production path |

Source providers implement meaningful source/work contracts. The deterministic
application is pure; it MUST NOT acquire marker traits or a runtime/store adapter
dependency just to satisfy a classification checker. Different caller-owned
providers MAY share an explicit test scheduler only through separately scoped
node handles. Compile-time graph and runtime cooperation MUST be documented
separately; no universal common-types package or whole production node is needed.

The comparison workspace MUST explicitly adopt the existing architecture gate
with its own reviewed roles, all normal/build/dev/optional/target/renamed edges,
unknown-package and forbidden-edge negative fixtures, nonempty contract syntax
and actual compiler/conformance witnesses. Existing `proofs/raft-adoption/check.sh`
and `glade-discover/scripts/check-architecture.sh` do not automatically cover it.
Exact local command and affected-consumer selection MUST be filed before B0
implementation. Classification/allowlist weakening to pass is prohibited.

The normal B0 loop is API/model/shared consumer, owning adapter election tests,
local architecture gate, source-scope and process-global checks, then lint/type
checks for affected packages. A public boundary change additionally runs both
real adapters and affected consumers. B1 real disk/process cuts and B2 exhaustive
faults are separate explicit assurance commands, not silently omitted acceptance
tests or mandatory work on every pure edit. Small carrier-free tests MUST NOT
pull either engine, disk adapter or Glade node through transitive test helpers.
Publish actual package commands and distinguish execution, warm incremental
build-plus-test and cold build on a named machine/toolchain; no budget has yet
been measured or accepted.

Production allocation later follows the external
[Gyld declaration](</Volumes/projects/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py>)
`Records`, `StorageAdapter` and `NodeAssembly`, with
[build entry](GladeBuildEntry.md), [architecture notes](arch1/GladeArchitecture.md),
[library policy](LibraryBoundaryAndTestingPolicy.md) and
[package architecture](GladePackageArchitecture.md). Records remains the proposed
owner of protected accepted history/outcomes/recovery; narrow injected storage
and message ports supply I/O; NodeAssembly selects providers; policy/admission
and binding keep authority/identity ownership. The declaration currently says
source commits stay outside Records. Any supplemental M3-D allocation/amendment
MUST receive its own reviewed canonical consumer contract before production
implementation. This private harness MUST NOT become that allocation by accident.
The external Gyld file is read-only in this lane.

## 9. Mandatory inventories and comparative record

B0 MUST inventory adapted source plus full resolved dependency graph, enabled and
disabled features/platform branches, constructors and every entropy, monotonic/
wall-clock, scheduler/task, formatting, logger, environment, process-hook and
child-spawn seam. Syntax-aware source checks MUST inspect disabled branches
without compiling every variant, enforce explicit cfg boundaries and braced
control-flow bodies in new/modified code, and record broader upstream migration
debt separately. A local empty process-global allowlist is not a dependency audit.
Apply the repository process-global checker to local production-intended Rust;
extend inspection coverage to vendor/dependency sources with reported limitations.
No new exception, global runtime workaround or allowlist relaxation is authorized.

Dormant global/default feature paths MUST be reported with proof that the selected
graph cannot execute them; reporting them does not grant a production exemption
or claim a complete upstream migration. Required unresolved source access blocks
the relevant gate. Build helpers/child processes need explicit environment/tool
inputs and their own inventory; an omitted target dependency cannot become an
unreported platform guarantee.

B3 additionally MUST file the exact resolved graph for each candidate across
normal/build/dev/optional/target edges and features, licenses/notice obligations
and provenance, advisory/security scan tool/database date and unresolved findings,
generated/macro/platform coverage limits, compiler/protoc/runtime support and
tested storage assumptions. Manifest license fields alone are not a notice audit;
a scan finding no advisory is not a security proof. raft-rs's protobuf-build/protoc
portability cost and OpenRaft's mandatory runtime/dependency footprint and
pre-1.0 API/on-disk compatibility costs remain measured work, not predictions.

The comparative record MUST include each accepted exit tuple/review, RED/GREEN
and mutant results, adaptation files/lines/call paths, source-inventory closures,
adapter code/test work, elapsed engineering effort when actually recorded,
command timings, build/runtime resource observations and measurement conditions.
Repeated runs use equal application workload/fault journey and report each
carrier's own algorithm traces. Unknown performance/cost stays unknown. Selection
is an explicit owner decision after comparative review; no numerical score or
default crate choice is supplied here.

## 10. Review, stop conditions and exact remaining decisions

Use the [review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>):
settled exact document/source tuples, peer-blind Consistency/Safety before
contract/adaptation implementation, Code/State at implemented exits, verbatim
reports, owner verdict merge and bounded remediation. A future user-facing
configuration/API freeze also requires Surface review. B0 acceptance MUST say
“automatic-election/source boundary only”; B1/B2/B3 remain open until separately
accepted. Stop on invariant failure, unowned sources/work, incomplete payload/
outcome/policy/cut, unresolved publication, graph violation or conflicting mapping.
Repair from a regression and review; do not reset history or weaken a receipt.

The lane owner still needs to settle the following exact objects before their
dependent implementations:

- B0 package/workspace location, concrete type/signature declarations, instance
  runtime plumbing patch for both pins, feature/timing profiles and exact local
  gate/test commands. These are engineering choices for review, not production
  library selection. This draft proposes their semantics, not measured outcomes.
- B1 private full application envelope and separate BeginMove/Activate extension,
  per-carrier protocol recovery representation, concrete durable publication
  adapters and named filesystem/process-crash profile; compiler RED then review.
- B2 concrete complete configuration/snapshot/replay serialization and common
  consumer extension, including OpenRaft joint-stage intent recovery; compiler
  RED then review.
- B3 comparative evidence and owner production library choice; Q4-C canonical
  authority/profile amendments and Q4-D/E independent domains, storage guarantee,
  custody, disclosure/freshness, retention, baseline/rollback cut and activation
  remain separate required owner decisions.

This drafting task changes only this document. No build, test, fault run,
dependency/source change, Git/GWZ mutation or production verification is claimed.
