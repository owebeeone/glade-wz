# Q4-B0 corrected contract/allocation/compiling RED — SAFETY-AXIS REVIEW

**Review object:** Complete DRAFT comparison contract, B0 allocation and `proofs/raft-carrier-comparison/` at root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`, controlled by `dev-docs/GladeRaftB0-RemPlan-2.md`. Private contract/allocation/compiling RED gate only.

**Baseline:** Root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were read through pinned `git show`, scoped comparison and verified matching local bytes.

**Date:** 2026-10-03  
**Axis:** Safety: attack source ownership, invalid-input refusal, terminal lifecycle, satisfiable consumers and honest qualification boundaries. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one new P2 finding blocks. All prior counterexamples checked here are corrected. I pre-commit to GO on a revision resolving P2-6 as specified while preserving the reviewed scope and existing gates.

---

## Prior-finding closure table

“Verified” below records this fresh reviewer’s independent execution/retrace. Required originating-finder closure remains a separate process record.

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Safety P2-1 | Preserve actual entropy cadence | Once-only OpenRaft witness and extra-read detector pass; B0-02/03 retain constructor-only sampling and target raft-rs leader quorum-loss reset. | Verified corrected |
| Consistency P2-1 | Same cadence correction | Retraced the original impossible post-constructor entropy-failure requirement; it is removed. | Verified corrected |
| Safety P2-2 | Shared endpoint lifecycle authority | Executed separate/fresh-handle stop and replaced-issuer/destination ordinary-traffic tests; refusal preserves inventories and sequence allocation. | Verified corrected |
| Consistency P2-2 | Reject stale ordinary traffic | Original emit/request/release/duplicate/take paths now validate shared current scope before mutation. | Verified corrected |
| Safety P2-3 | Reconcile own-scope stop during poll | Executed selected future stopping its scope and returning Pending: exact-once destruction, Cancelled inventory, saved wake cannot revive it. | Verified corrected |
| Safety P2-4 | Detach futures before destruction | Individual cancellation, bulk stop, rejected registration and completed-future reentrant cleanup all pass without borrow panic. | Verified corrected |
| Consistency P2-3 | Same destructor correction | Original parent destructor cancelling/inspecting child work was executed through the lifecycle regressions. | Verified corrected |
| Safety P2-5 | Restore caller-owned Timeout/Cancel | Executed all eight held/consumed × stopped/replaced-peer × Timeout/Cancel cells, including selected polling, typed results and unchanged remote state. Exact defect mutant rejected. | Verified corrected |
| Consistency P2-4 | Same local termination correction | Independently verified ownership removal, wake without inline execution, late-reply refusal, foreign/forged/stale-caller refusal and continued healthy traffic. | Verified corrected |

## Changed-range analysis

Since remediation 1, the functional correction splits exact local pending ownership (`owned_rpc`) from remote delivery validity (`rpc`). Timeout/Cancel use the former; Respond/Resolve retain the latter. Added matrix tests, the isolated mutant checker and current evidence accompany allocation/README clarification.

The complete comparison against the original checkpoint also includes source-attempt instrumentation, corrected fault consumers, shared endpoint lifecycle enforcement, scheduler reconciliation and destruction outside borrows.

Public API, manifests, lockfile, refusing providers, ten case selections per provider and comparison-contract text remain unchanged. No engine, production dependency or source adaptation was installed.

**NEW NON-ARCHITECTURAL ROOT CAUSE: P2-6.** The unchanged fixture’s deterministic clock-domain numbering admits collisions. This is an implementation defect in identifier allocation, fixable without changing source/work ownership or trait semantics. I identify no third new architectural root cause; the architectural stop condition is not triggered by this finding.

## 0. Evidence base

Read the complete canonical Safety prompt; its SHA-256 matches `6f840b6e6316db1aae7fc2cf8f7006bb6a69d4dbed7020e2c1ddf2f573ee4929`. Read AGENTS authority, review-loop skill, both original and remediation-1 reports, remediation plans, full comparison contract/allocation/adaptation inventory, and all named governing documents, including QualificationPlan §6 and AdoptionContract RA-001–012. No current peer or separate finder-closure report was read.

Inspected API `lib.rs:1–415`, provider/compiler witnesses, all B0 consumers and oracle, fixture assembly/source/work/transport implementations, all fixture integration tests, manifests, policy and check/measurement scripts. Material lifecycle ranges include:

- `spec/src/fixture/transport.rs:44–100,263–430`
- `spec/src/fixture/transport/termination.rs:1–341`
- `spec/src/fixture/work.rs:61–265`
- `spec/tests/endpoint_lifecycle.rs:10–256`
- `spec/tests/scheduler_lifecycle.rs:12–215`

Checked cached pinned upstream timeout/reset, election, ticker, signed pre-epoch vote arithmetic and feature/runtime constraints. No network was required.

Independently executed the README’s three GREEN commands, ordinary RED command, `check.sh`, all-four-package all-target Clippy with denied warnings, and authorized isolated termination mutant:

- **45 GREEN:** 41 spec/unit/compiler/fixture/oracle tests plus four API/provider witnesses.
- **20 ordinary B0 assertion failures:** exit 101; zero passed, ignored or filtered.
- Structural/source/format/global checks PASS: 24 Rust files, 14 normal-source files, zero global exceptions, five rejected architecture negatives.
- Clippy PASS.
- Mutant compiled and produced exactly eight matrix assertion failures plus six passing units; no ignored/filtered cases.

Inspected historical RPC compiling RED, remediation-1 RED and overflow supplement, and remediation-2’s ten compiling failures. These historical executions were inspected, not reconstructed. Current measurements distinguish scaffold execution, warm build, fresh build and expected failure honestly.

All four HEADs matched at start/end. Verified 102 current inventory entries plus manifest, documentary-context hashes and matching non-output subtree bytes. Manifest SHA-256: `5415b79ebaed75de6c5bf49d7619b0661516651c6982a427aef70b6fbef8e789`. Scoped diff remained empty. Only authorized ignored build/mutant outputs were produced.

## 1. Findings

### [P2-6] Clock-domain numbering aliases different node incarnations

**Location:** `proofs/raft-carrier-comparison/spec/src/fixture.rs:71–84`; accepted by `fixture/sources.rs:86–98` and `fixture/work.rs:108–119`.

**Violated invariant:** Separately supplied node clocks require independent domain identities. Foreign-domain time and deadlines must refuse before source/scheduler mutation; see comparison contract §3 and its initial independent-clock-domain fixture.

**Concrete source-traced reproduction:** In `Fixture::new(Carrier::OpenRaft, 1)`, construct `inputs(1, 11)` and `inputs(2, 1)`. Both calls are accepted; no incarnation bound rejects either. Their domain calculation is:

- Node 1/incarnation 11: `100 + 10 + 11 = 121`.
- Node 2/incarnation 1: `100 + 20 + 1 = 121`.

The resulting source states are separately owned, but both carry `ClockDomain(121)`. Obtain node 2’s clock, set its nanos to a later value, and pass it to node 1/incarnation 11’s `SourceProbe::advance`. The only ownership discriminator is domain equality; backward/fault checks also pass. The foreign timestamp therefore returns `Ok(())` and changes node 1’s clock.

The same timestamp is accepted by that incarnation’s `WorkProbe::advance`; a deadline originating from the other node is likewise accepted by `register`. Checked arithmetic prevents numeric wrap but does not prevent this alias.

This counterexample follows committed branches directly. No additional test or unauthorized build was created during review.

**Impact:** After sufficiently many incarnations—or direct construction of these supported values—the fixture ceases to enforce its advertised clock isolation. Cross-node time/deadline mistakes can silently advance time or make work eligible, weakening downstream invalid-control evidence.

**Required correction:** Allocate collision-free domains for issued scopes, or explicitly validate a bounded nonoverlapping encoding before constructing source/work state. Merely checking addition overflow is insufficient. Preserve deterministic replay and instance ownership; introduce no process-global allocator.

**Closure test:** Construct node 1/incarnation 11 and node 2/incarnation 1 in the same fixture. Require distinct domains. Passing either node’s instant/deadline to the other’s source and scheduler must yield `WrongDomain` with unchanged clock/work inventories. Cover adjacent incarnation/node boundaries and the chosen allocation limit, then rerun the existing GREEN, ordinary RED, structural and lint gates.

## 2. Invariant analysis

The prior lifecycle attacks now fail: shared stop blocks independently obtained handles; replacement blocks stale ordinary traffic; selected futures cannot survive scope stop; arbitrary future destruction occurs outside scheduler borrows.

Caller-owned termination survives remote stop/replacement without granting stale delivery rights. Correlation, peer/incarnation, kind/carrier, late replies and terminal reuse remain checked. Registration and wake do not poll inline.

The four std-only roles and declared edges remain meaningful. Constructors retain separately created dependencies; both labels select identical exported B0-01–10 consumers. Opaque full votes remain separate from candidate-free standard epochs.

OpenRaft’s private single-term-leader/singlethreaded/storage-v2 profile, millisecond sampling, ticker cadence, lease/greater-log guards and signed internal arithmetic remain source-derived mandatory adaptation obligations. Ready/LightReady, fatal-source propagation and real task cleanup are not falsely qualified by these fixtures.

B1/B2 and complete B3 comparison remain mandatory; neither candidate is silently dropped. No production authority, receipt reinterpretation or default activation is claimed.

## 3. Risks and next action

GREEN fixtures and synthetic detectors establish no actual-engine, adapted-source, durability, dependency-security or production qualification. The separate SourceAdaptationPlan remains out of scope.

The next action is a regression-first bounded correction of P2-6 on a newly settled tuple, followed by its independent closure and the required acceptance records. No third architectural patch is implied.