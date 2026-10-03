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
independent review. This drafter has made no Git/GWZ operation. Existing documents,
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

Current focused commands, exact logs and machine/toolchain/timings are in
[the subtree README](../proofs/raft-carrier-comparison/README.md) and
[`evidence/measurements.json`](../proofs/raft-carrier-comparison/evidence/measurements.json).
Observed final checkpoint: **19 passing compiler/fixture/oracle tests; 20 intended
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

Measurements concern this small std-only scaffold on macOS 26.6.2/arm64,
Rust/Cargo 1.96.0. B0 execution alone took 0.014461 s; warm Cargo build-plus-RED
0.054268 s; fresh target test build 0.871023 s; warm structural gate 1.451806 s;
all-target Clippy 0.432752 s. These are observations, not accepted budgets,
engine timings, reliability rankings, license/security/notice audits, disk/process
crash evidence or production results. The source footprint, complete resolved
adapted graph, licenses/security/platform inventory and measured adaptation/upgrade
cost are still absent. No B0/B1/B2/B3 exit is closed.
