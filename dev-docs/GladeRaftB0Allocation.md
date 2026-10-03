# Q4-B0 private compiler and behavioral RED allocation

Date: 2026-10-03. Status: **DRAFT for contract/allocation review. Compiling RED only; no adapted engine, source adaptation, automatic-election result, carrier selection or production activation is accepted.**

This object is the new [`proofs/raft-carrier-comparison`](../proofs/raft-carrier-comparison/README.md)
subtree plus this allocation. The controlling requirements remain
[the comparison draft](GladeRaftCarrierComparisonContract.md),
[Q4 integration](GladeRaftProductionIntegrationPlan.md),
[qualification](GladeRaftQualificationPlan.md), and
[the pinned source audit](GladeRaftQ4-CarrierAudit.md), subject to
[the library policy](LibraryBoundaryAndTestingPolicy.md) and
[package architecture](GladePackageArchitecture.md). The
[review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>) controls
independent review. This drafter has made no Git/GWZ operation. Other existing documents,
production code, `raft-adoption`, member repositories, and the separate production
profile review object are outside this draft's writes.

## Proposed concrete allocation

The comparison draft's §6 and this allocation propose the same four std-only
members. This object supplies the concrete compiler allocation, signatures and
RED evidence for their joint contract review; neither document by itself ratifies
an allocation or qualifies a carrier.

| Package | Proposed role and rationale | Actual declared edges |
| --- | --- | --- |
| `glade-carrier-api` | Contract: meaningful instance sources, actual owned-future work, election driving, opaque memory protocol state, transport and instance logging | None; standard library only |
| `glade-carrier-spec` | Harness/tool: shared normal consumer, independent invariant oracle, pure scripted sources/scheduler/queued RPC fixtures | Normal API only; dev-only both refusing provider crates |
| `glade-carrier-raft-rs` | Implementation boundary reservation; current object is solely `RefusingRaftRs`, not a carrier implementation | Normal API only |
| `glade-carrier-openraft` | Implementation boundary reservation; current object is solely `RefusingOpenRaft`, not a carrier implementation | Normal API only |

`architecture-policy.json` declares these **review-proposed** roles/edges and
compiler/conformance target names. A structural PASS does not ratify its roles,
meaningful contracts, or currently failing implementation conformance. There is
no artificial pure/service marker trait. The lockfile contains four local packages,
zero registry packages; neither upstream carrier/runtime/codec is installed here.
Rust edition 2024, floor 1.96 and resolver 3 match `raft-adoption`.

The compile graph is API <- spec, API <- each refusing provider, with spec's
**development** edges selecting those providers. Runtime cooperation is different:
caller-owned source/work/store/endpoint/logger handles enter each constructor;
only explicit external clock/work/message controls may produce engine progress
in a future actual provider. These in-process values define no cross-carrier wire,
restart, disk or production compatibility.

## Concrete signature and lifecycle proposals

The public Rust definitions are in [`api/src/lib.rs`](../proofs/raft-carrier-comparison/api/src/lib.rs).
They contain complete owned boundary types and required trait operations, without
an implementation of election, validation, source sampling, or scheduling.
`SourceContext::new` only assembles dependencies already supplied by the caller.
The separate constructor compiler target creates two fixture instances, constructs
their source/work objects before the provider constructors, and supplies them
separately to both concrete refusing types.

The following concrete supplements to draft §3 are proposed for review:

- `WorkSpec::Runnable(OwnedTask)` and `WorkSpec::At { deadline, task }` carry the
  **actual owned boxed standard-library future**, not a scheduling metadata stub.
  `ScheduledWork::poll`, `advance`, `inventory` and `stop` make real task lifetime,
  eligibility and cancellation visible. A registration/spawn MUST NOT poll inline;
  a wake MUST only mark eligibility; one selected poll drives at most one poll.
  Engine wrappers MUST own typed joins/results and cancellation guards within the
  node scope. Work completion/cancellation is terminal; a live pending task may
  be selected again only after it becomes eligible.
- `ElectionNode::inventory` exposes clocks, work, queued messages, pending RPCs
  and the declared timing boundary. `transport` selects explicit Release/Hold/Drop/
  Duplicate/Resolve/Timeout/Cancel actions. The separate transport port's request
  returns a real pending response future; its required `respond(rpc, kind, bytes)`
  issues a reply with explicit `MessageView.reply_to`/`Inbound.reply_to`. Requests
  use `rpc`; one-way messages use neither. The common consumer MUST Resolve only
  explicitly correlated replies on the caller endpoint, and receive requests and
  one-way raft-rs replies normally. The controlled fixture checks the actual issued
  pending request, consumed request, expected peer/current scope/incarnation,
  correlation, response kind and carrier before issuing/resolving. Issued bytes
  remain immutable through copying; semantic protocol payload validation remains
  the actual engine/adapter's obligation. A wrong/foreign/old/terminal reply MUST
  NOT consume the queue token or settle another RPC. A dropped message MUST NOT secretly
  resolve its RPC; only explicit resolution/timeout/cancellation does so.
- `ElectionNode::advance_by(delta_ns)` is required alongside `advance(to)`.
  It MUST check `now.nanos + delta_ns` before changing sources, engine or queues.
  B0-10 therefore tests guaranteed addition overflow after a positive clock advance,
  without inventing a policy that every valid absolute `u64::MAX` time must refuse.
- `ElectionEpoch { carrier, group, term }` is a separate observation key for the
  proposed standard one-leader-per-term comparison mode. Full `VoteIdentity`
  retains opaque candidate/committed semantics; `Role::PreCandidate` preserves
  raft-rs pre-vote observations separately from Candidate; it MUST NOT be replaced by the
  epoch or have candidate identity inserted into the uniqueness key. The mapping
  MUST be verified against the selected upstream mode before any GREEN claim.
- Fixed configuration is raw owned input with explicit authorized-fixture genesis.
  The future constructor MUST validate it before engine/source/task participation.
  No production identity/bootstrap authority follows from numeric group 7/voters
  1–3 or the fixture authorization flag. This API supplies no executable validator.

A live current caller MUST be able to select Timeout/Cancel for its exact issued
pending RPC independently of whether the stored remote peer is live. That action
removes only its local pending ownership and settles the response with TimedOut
or Cancelled; a saved wake marks eligibility, with no inline future execution.
Foreign, forged, stale-caller and terminal RPC controls MUST refuse unchanged.
Respond/Resolve and message delivery MUST still validate live remote scope,
incarnation and issued correlation; local termination grants no stale delivery
rights. A request's delivery state (held/consumed/dropped) does not transfer RPC
termination ownership to the peer. These lifecycle rules are the remediation 2
clarification of the supplied pending-future boundary, not new public signatures.

Tokens bind carrier/session/group/node/incarnation/domain and sequence. Public
values are **inputs**, not unforgeable capabilities; future drivers/endpoints MUST
check their live issued registries. Foreign/stale/consumed controls MUST refuse
without mutation. A duplicate allocates a fresh token for the same original bytes.
Clock domains/ranges/unit arithmetic MUST be checked. A source/task failure MUST
stop participation, clean construction work, and cancel pending task/RPC ownership
without creating a leader/quorum/terminal noncommit. Stop MUST cancel/join owned
work and exclude old callbacks; a replacement receives a fresh explicit context.
B0 exposes no application proposal/outcome/disk/configuration/snapshot API.

The refusing constructors merely retain their already-created `NodeInputs` and
return a refusing object, with **no source call or initialization**. Every driver
operation returns `NotQualified`. In B0-03 even an injected failed-source constructor
currently returns that object, and the assertion correctly fails the requirement
for the exact constructor source error (logging `observe() == Err(NotQualified)`).
This agrees with draft §6's constructor-error scaffold: no source probing or
validation is implemented before review. The required constructor error/cleanup
behavior remains RED.

## Proposed pinned engineering fixture, not implemented

| Item | raft-rs 0.7.0 proposal | OpenRaft 0.9.25 proposal |
| --- | --- | --- |
| Upstream identity | `10c6e9db6792b85c81784e44fc278f895d5f0ab0` | `8815cdba2826f74e848acef361ad03f93bb1c3f8` |
| Logical scheduling | One host tick = 1,000,000 ns | Logical instants in ns; upstream ticker interval mapped separately |
| Timeout draw range | Original ticks `[10,20)` | Original **milliseconds** `[10,20)`; no expansion into a nanosecond RNG range |
| Heartbeat / ticker | Heartbeat 1 tick; host tick quantum 1 ms | Heartbeat 2 ms; pinned `Raft::new` ticker `heartbeat * 3 / 2` = 3 ms |
| Guards / cadence | pre-vote=true, check-quorum=true; every upstream timeout-reset access, including constructor/state resets | election=true, heartbeat=true; initial EngineConfig sampling only, no per-election resampling |
| Boundary semantics requiring further tests | elapsed ticks, half-open endpoints, outstanding Ready/LightReady lifecycle | vote last-modified time; committed-vote lease 20 ms plus sampled timeout; greater-log extra delay 40 ms; voter/enabled guards |
| Resume proposal | Host withholds work; resume one current host tick without missed-tick burst | Actual pinned ticker resumes the single pending wake, then schedules from resumed `now + interval`; no invented catch-up elections |
| Proposed engine features | defaults=false, protobuf-codec; instance logger | defaults=false, storage-v2, **single-term-leader** and **singlethreaded**; private local runtime profile only |

The OpenRaft feature is verified in the audited pin: `Cargo.toml:63` declares
`single-term-leader = []`; `src/docs/data/vote.md:37–38` distinguishes its default
multiple-leaders-per-term mode from the standard mode;
`src/vote/leader_id/leader_id_std.rs` retains `term` and `voted_for` and partially
orders competing same-term candidates. This is a proposed private engineering
feature profile, **not** a production selection or voting-algorithm adaptation.
Default OpenRaft mode is unqualified by this initial standard-epoch oracle; a later
mode change MUST settle its own epoch/uniqueness mapping rather than incorrectly
rejecting legitimate default-mode sequential leaders.

Further inspected pinned timing sites:
`src/config/config.rs:257` samples `election_timeout_min..election_timeout_max`;
`src/engine/engine_config.rs:46–60` converts milliseconds and derives lease=max,
greater-log delay=2*max; `src/raft/mod.rs:249–261` constructs the ticker;
`src/core/tick.rs:81–105` schedules `now + interval`; and
`src/core/raft_core.rs:1440–1507` supplies vote-modified/lease/greater-log/guard
inequalities. This is source inspection, not executable engine evidence. Initial
zero-time/absent-vote semantics and checked duration subtraction MUST remain faithful
in the reviewed context adaptation; this scaffold does not invent a saturation rule.

Timeout scripts explicitly distinguish nodes and provide lower/interior/upper-minus-one
values. Equal initial scripts are separately supplied for split-vote scaffolding.
All fixtures start independently at logical zero. The 10,000-round fair suffix
advances live fixture clocks by 1 ms, drives eligible work in node/work-ID order,
and releases messages in emission order; paused-node time may advance with its
work withheld. This is a test termination bound, not a latency promise.

`OwnedTask`, `RpcFuture` and current Rc-based source fixtures are intentionally
local and do not promise `Send`/`Sync`. The private engineering profile proposes
OpenRaft **singlethreaded** alongside single-term-leader/storage-v2: the pin's
`Cargo.toml` declares that feature and `AsyncRuntime`/`RaftTypeConfig` use
`OptionalSend`/`OptionalSync`. This gives the proposed local ownership/runtime
mode an explicit allocation for review; it does not implement an owned runtime,
prove actual constructor compatibility, remove static runtime seams, or authorize
unsafe Send/Sync workarounds. Any production multi-threaded/Send-capable allocation
MUST receive its own reviewed signatures and witnesses. The reviewed instance
context/failure/logging/source adaptation is still a separate mandatory gate.

## Consumers, actual RED and pre-acceptance obligations

`spec/tests/b0_election.rs` selects B0-01–10 for **both** refusing providers as
20 ordinary default tests. Both labels call the same exported functions in
`spec/src/b0`. No case is ignored, filtered or reinterpreted as passing on refusal.
The separate compiler witness does not contaminate the intended-RED target count.

| Case | Current failing positive/typed invariant |
| --- | --- |
| B0-01 | Actual elected-leader observation required; `NotQualified` |
| B0-02 | Supplied constructor stream draws required; empty trace, refusing observation |
| B0-03 | Exact `EntropyFailed` constructor refusal required; refusing object observed |
| B0-04 | Recorded actual eligibility boundary required; `NotQualified` |
| B0-05 | Actual no-quorum-prefix observation required; `NotQualified` |
| B0-06 | Actual minority/quorum observation required; `NotQualified` |
| B0-07 | Actual initial election required; `NotQualified` |
| B0-08 | Actual initial automatic leader required; `NotQualified` |
| B0-09 | Actual independent initial state required; `NotQualified` |
| B0-10 | Valid explicit advance must succeed before invalid controls; `Err(NotQualified)` versus `Ok(())` |

These consumers compile all the later source failures, split/minority prefixes,
delayed duplicate/reordered replies, failover/resume, independent schedule replay,
foreign/stale/consumed controls, checked-add overflow and stop/replacement assertions.
They presently stop at their first stated RED assertion: downstream schedules have
**not** executed against a functioning provider. A real adapter MUST reuse these
same consumers and intended case selections; failures MUST NOT be skipped to claim
qualification. The seven oracle self-tests reject synthetic-leader/no-traffic,
fixed-script substitution, missing required draw, early eligibility, implicit other
clock progression, paused work and conflicting standard-epoch leaders. They exercise
invariant detectors with synthetic counterexamples, not real engine mutants.

The practical correlated-reply supplement followed an observed compiling RED
step: all four ordinary `rpc_reply` fixture tests failed against a deliberately
refusing `respond` method (not import/type errors), before controlled queue/RPC
mechanics were implemented. They now pass for success, terminal reuse, wrong RPC,
wrong peer, old incarnation/replaced peer, wrong carrier and wrong response kind;
actual response futures still advance only at an explicitly selected poll.
[`rpc-red.log`](../proofs/raft-carrier-comparison/evidence/rpc-red.log) and
[`rpc-green.log`](../proofs/raft-carrier-comparison/evidence/rpc-green.log) retain
that local fixture RED/GREEN distinction. This is pre-gate harness completeness,
not implementation of an engine or remediation of an accepted interface. Late
correlated replies to terminal RPCs receive typed refusal and an explicit harness
drop; they cannot be fed into a completed RPC future again.

Before B0 acceptance, additional reviewed source-call footprints, actual pinned
constructor/reset cadence, lower/upper boundary values, all constructor/runtime
fault cuts/cleanup, OpenRaft vote-modified/lease/greater-log/nonvoter/disabled guards,
actual async RPC/future/join behavior, real resume policy, Ready/LightReady ordering,
and mutations in the actual adapted paths MUST execute. A self-reported generic
TimingBoundary or synthetic detector test cannot establish those facts. Neither
source tracing nor the local empty global allowlist proves absence of ambient calls.

In particular, OpenRaft `raft/mod.rs:249–261` uses `tracing::Span::current()` during
construction before EngineConfig. Tracing macros/spans/`#[instrument]` and active
ambient dispatcher/thread-local/logger paths MUST be inventoried even with no
subscriber installed. Runtime, logging, diagnostic formatting, entropy/time,
environment/hooks/child-spawn and disabled-source/dependency branches all remain
source-adaptation obligations. Passing injected entropy/time alone cannot close
BC-003 or whole dependency no-global compliance. No exception is authorized.

## Explicit gate adoption, fast loop and measured limits

`check.sh` adopts the existing discovery architecture checker by a **read-only
manifest path** with build outputs redirected under this subtree. It runs the
existing process-global checker with **empty entries**, scans/parses every local
Rust source including non-active paths for explicit scope, runs five actual manifest
negative fixtures (unknown package plus forbidden normal/build/dev/optional-target-
renamed edges), and checks formatting. Its local no-conditional-attribute profile
is stricter than needed: no cfg/cfg_attr attribute is present. These checks are
local source/declared-edge checks, not macro expansion or third-party source audits.

Initial checkpoint commands, exact logs and machine/toolchain/timings are in
[the subtree README](../proofs/raft-carrier-comparison/README.md) and
[`evidence/measurements.json`](../proofs/raft-carrier-comparison/evidence/measurements.json).
Observed initial checkpoint: **19 passing compiler/fixture/oracle tests; 20 intended
behavioral failures; zero ignored/filtered intended B0 cases**. Structural gate,
source/global checks and all-four-package Clippy `--all-targets -- -D warnings`
pass. Initial lint findings were mechanical nested-if warnings and were corrected;
no allowlist or role relaxation was used to turn a check green.

The fixture is std-only today. Cargo can compile package dev-dependencies even for
unit selections: replacing either current dev-provider package with a real engine
would therefore pull that engine into spec's supposed fast loop. **Before that
change**, a reviewed runner/adapter-test allocation MUST move concrete selection
out of spec's development graph (or supply an equally explicit reviewed isolation
design), preserving all exported common consumers and all B0-01–10 selections for
both engines. A normal-edge-only inventory MUST NOT be claimed as future unit-loop
isolation. Public contract changes require both concrete compiler consumers and
all common behavioral consumers; actual engine/affected-adapter checks remain
mandatory once those packages exist. B1 real-I/O and B2 snapshot/configuration
matrices remain independent exits.

Initial measurements concern this small std-only scaffold on macOS 26.6.2/arm64,
Rust/Cargo 1.96.0. B0 execution alone took 0.014461 s; warm Cargo build-plus-RED
0.054268 s; fresh target test build 0.871023 s; warm structural gate 1.451806 s;
all-target Clippy 0.432752 s. These are observations, not accepted budgets,
engine timings, reliability rankings, license/security/notice audits, disk/process
crash evidence or production results. The source footprint, complete resolved
adapted graph, licenses/security/platform inventory and measured adaptation/upgrade
cost are still absent. No B0/B1/B2/B3 exit is closed.


## Remediation 1: scoped correction, original closure pending

The [merged plan](GladeRaftB0-RemPlan-1.md) maps all seven findings in
[Consistency](GladeRaftB0-ReviewConsistency.md) and
[Safety](GladeRaftB0-ReviewSafety.md) to four root causes. Three converged blindly.
This first patch intended to enforce the existing lifecycle contract; **no public API,
library role, dependency edge, engine, source adaptation or production code changed**.
The architecture proposal only adds the two focused conformance targets. No
allowlist entry or classification was relaxed. Original finders MUST independently
verify their counterexamples on a settled revision; this drafter closes no finding.

| Original IDs | Correction and executable coverage |
| --- | --- |
| Consistency P2-1; Safety P2-1 | B0-02/03 use a caller-owned access/state oracle. OpenRaft MUST retain constructor-only entropy access; post-constructor poison remains healthy with no attempted draw. A source/work-only once-sampling witness passes and an extra-runtime-draw mutant is rejected. raft-rs isolates the actual established leader and withholds inbound quorum activity to reach `tick_heartbeat`/MsgCheckQuorum -> `become_follower` -> `reset` (raft.rs 1095, 1126, 986), rather than assuming a follower pre-campaign resets. Both constructors retain exact entropy/clock/register failures. Runtime clock/task faults target actual owned advance/poll. Runtime Register failures are required only at an observed registration attempt; no attempt MUST mean unchanged footprint and healthy state. A once-registered persistent-future witness covers no artificial registrations; an attempted-registration/no-failure mutant is rejected. Unknown runtime errors and inventory failures are asserted, never ignored. |
| Consistency P2-2; Safety P2-2 | One network-owned current-scope/stopped registry governs all separately obtained endpoints. Emit/request/take/ordinary controls/respond/Resolve validate live issuer and destination plus actual registered message/RPC ownership before sequence/queue/RPC mutation. The first patch wrongly applied remote liveness to Timeout/Cancel too; remediation 2 below restores caller-owned local termination. Stop is shared and terminal; replacement invalidates old handles/tokens. Seven endpoint tests cover no-mutation refusals, no burned sequence, stopped peers/callers, stale RPC ownership and valid replacement participation; four existing correlation tests stay GREEN. |
| Safety P2-3 | Selected future metadata remains scheduler-visible. A stop or cancellation during poll is reconciled before reinsertion, leaves terminal inventory and drops the selected future once. A saved wake cannot revive it. |
| Consistency P2-3; Safety P2-4 | Cancel/stop detach owned futures and establish terminal state under the scheduler borrow; arbitrary future Drop runs after release. Register binds the owned future before its borrow so wrong-domain/sequence-overflow rejection releases it first. Reentrant parent/child cleanup, exact-once drops, rejected registrations and completed-future cleanup are executable. RPC mutation/state settlement also releases both borrows before saved wake; existing RPC tests cover this scoped refactor. |

[`rem1-red.log`](../proofs/raft-carrier-comparison/evidence/rem1-red.log) records
**12 compiling behavioral failures**, zero passes/ignored/filtered, before the
four corrections. The separate
[`rem1-red-overflow.log`](../proofs/raft-carrier-comparison/evidence/rem1-red-overflow.log)
adds one unique compiling overflow/Drop regression before its correction (the
three source tests repeat there). Thus **13 distinct new regressions observed RED**.
The first authoring compile error is retained separately and is explicitly not
RED evidence. Original logs and `files.sha256` remain unchanged. GREEN logs and
[`rem1-final-measurements.json`](../proofs/raft-carrier-comparison/evidence/rem1-final-measurements.json)
record the current commands, machine/toolchain and observed status.

Remediation 1 result: **31 spec/unit/fixture/oracle tests GREEN plus 4 API/provider
compiler tests GREEN = 35 total; all 20 ordinary B0 provider tests remain RED,
zero ignored/filtered**. The evidence gate verifies the precise 20-test failure
summary, preventing compiler failure or a changed selection from masquerading as
intended RED. The source-footprint witnesses are minimal pure test scaffolding,
not an election provider, real engine mutant, adapted source or concrete-runtime
cadence proof. Later pin-specific clock/work/registration source evidence remains
mandatory; no OpenRaft runtime registration allocation is invented here.

A direct reentrant std `Wake` callback capturing the fixture's local Rc endpoint
cannot satisfy `Wake`'s Send/Sync bound safely. No unsafe/global workaround or
fabricated RED wake witness was added. The existing selected-poll/saved-wake/RPC
checks stay GREEN; direct callback integration remains a source/runtime obligation.

The root-relative
[`rem1-files.sha256`](../proofs/raft-carrier-comparison/evidence/rem1-files.sha256)
inventory covers this allocation and the subtree (excluding target/cache outputs
and its own digest). The read-only documentary context is separately named in
`rem1-context.json`; it does not enlarge this patch's authorized file set. The
original reviewed root was `544c83d8cd07165cfeec2f0db64a78c8417849f3`; the owner
filed the reports/plan at documentary root
`9ac2fa4fa9c0f9b7e3d9aa112bfd5d993528882f`. No Git/GWZ operation was performed by
this drafter. This uses one architectural remediation round; no B0/B1/B2/B3,
source adaptation, canonical profile or production exit is accepted.

Remediation 1 measurements on macOS 26.6.2/arm64, Rust/Cargo 1.96.0:
B0 execution-only **0.013763 s**, warm build-plus-RED **0.033829 s**, fresh-target
B0 test build **1.018200 s**, structural gate **1.480033 s**, all-target denied-warning
Clippy **0.296296 s**. All are observations of this std-only scaffold, not engine
performance, accepted budgets or qualification. The source gate inspected all
**23 Rust files**; the global guard inspected **13 normal-source files**, with
**zero allowlisted items**. All five architecture negative fixtures were rejected.


## Remediation 2: caller-owned RPC termination, acceptance pending

The originating [Consistency](GladeRaftB0-ReviewConsistency-1.md) and
[Safety](GladeRaftB0-ReviewSafety-1.md) reviewers closed all initial findings at
root `256be2dd0fd652b34dfffa753fcba481f5fb842b`, then independently found the
same new architectural root cause: the first patch conflated remote delivery
validity with local RPC termination ownership. **Consistency P2-4 and Safety
P2-5 remain open for finder verification**, controlled by
[remediation plan 2](GladeRaftB0-RemPlan-2.md). This is the second bounded
architectural remediation; no third architectural patch is authorized. Fresh
peer-blind full Consistency/Safety review is required in addition to original-
finder counterexample closure. This drafter grants no acceptance or self-closure.

The minimal fixture correction separates `Network::owned_rpc` (exact issued
pending registry, caller identity, original request linkage and terminal state)
from `Network::rpc` (that ownership plus live remote-token validity). Timeout/Cancel
use the former after verifying the endpoint is current/live and RpcId belongs to
it. Respond/Resolve retain the latter; ordinary token controls/take retain the
same strict checks. The stored peer identity and immutable request bytes remain
unchanged. Settlement still removes local ownership and releases all borrows
before saved wake. No consensus/source behavior, public API, library classification,
dependency allowlist, package manifest, lockfile, provider or B0 case label changes.
The comparison contract already requires explicit pending RPC/timeout resolution;
its text needed no change. The two prior tests asserting peer-liveness refusal
were corrected rather than retained as a policy.

| Required closure surface | Executable evidence |
| --- | --- |
| Held/consumed request × peer stop/replacement × Timeout/Cancel | Eight ordinary unit tests in `spec/src/fixture/transport/termination.rs`, visible in the normal spec `--lib` command. Each actual response is polled Pending before the fault and local control. Only the selected RPC leaves caller inventory; unrelated caller and peer ownership remain unchanged. |
| Wake and terminal result | Control leaves the observing future unexecuted, marks work Runnable, then the next selected poll returns Complete and records precisely TimedOut/Cancelled once. |
| Immutable protocol and remote state | Read-only private byte snapshots compare every original issued message before/after lifecycle and local control. Peer/replacement message, pending RPC and work inventories are unchanged by local control. The caller continues valid traffic to another live endpoint. |
| Negative ownership and late reply | Foreign and forged controls, a replaced caller with an actual Pending future, repeated terminal controls, stale remote replies, and a still-live peer's late reply all refuse without consuming queues or settling unrelated RPCs. |
| Exact defect mutant | `check-termination-mutant.py` copies the std-only fixture under ignored target, changes only Timeout/Cancel back to `network.rpc`, and verifies all eight matrix assertions fail after compilation while six other unit checks pass, zero ignored/filtered. No engine mutation is claimed. |

[`rem2-red.log`](../proofs/raft-carrier-comparison/evidence/rem2-red.log) records
compilation followed by **10 behavioral failures and 10 passes**, zero ignored/
filtered, before the validator correction: all eight matrix cells and the two
corrected prior expectations fail. The Pending assertion executes before each
matrix failure. The initial
[`rem2-green.log`](../proofs/raft-carrier-comparison/evidence/rem2-green.log)
records **20 passing focused tests** after correction. Two additional GREEN
edge witnesses (replaced-caller Pending future and live-peer late reply) are not
claimed as observed RED. Original/remediation-1 logs and inventories are preserved.

Current result: **41 spec/unit/compiler/fixture/oracle tests GREEN plus 4 API/
provider compiler witnesses = 45 GREEN total**. The ten unchanged cases for each
refusing provider remain **20 ordinary behavioral RED assertions**, zero passing,
ignored or filtered; no refusal is reinterpreted as qualification. Architecture,
source scope, empty global allowlist, five architecture negative fixtures,
formatting and all-target Clippy with denied warnings PASS. The source scan covers
**24 Rust files**; the globals guard covers **14 normal-source files**, with
**zero allowlisted items**. `measure.py --prefix <fresh-label>` replays these gates
and the exact eight-cell mutant rejection without overwriting older checkpoint
names. This remains a std-only fixture/compiler/consumer object, with no engine,
source-adaptation, runtime, profile, production or B0/B1/B2/B3 exit acceptance.

Current commands, logs and actual machine/toolchain are in
[`rem2-final-measurements.json`](../proofs/raft-carrier-comparison/evidence/rem2-final-measurements.json).
On macOS 26.6.2/arm64 with Rust/Cargo 1.96.0: B0 execution-only **0.014054 s**,
warm build-plus-RED **0.039291 s**, fresh-target test build **0.848734 s**,
structural **1.644862 s**, denied-warning Clippy **0.360469 s**, and isolated
mutant build/run/check **1.226936 s**. These are observed scaffold timings,
not accepted budgets or engine performance.

The exact root-relative packet is
[`rem2-files.sha256`](../proofs/raft-carrier-comparison/evidence/rem2-files.sha256),
with changes from the preserved remediation-1 inventory named in `rem2-changes.json`.
`rem2-context.json` labels original implementation-review root
`256be2dd0fd652b34dfffa753fcba481f5fb842b` separately from owner documentary root
`fe8b25bc736c1366be9c1aeecd0ca883d8e208e1`; member/external pins remain unchanged.
Read-only reports/plans are context, not patch writes. No Git/GWZ operation was
performed by this drafter.
