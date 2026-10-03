# Q4-B0 contract/allocation/compiling RED — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT `dev-docs/GladeRaftCarrierComparisonContract.md`, `GladeRaftB0Allocation.md`, `GladeRaftB0-AdaptationInventory.md`, and `proofs/raft-carrier-comparison/` at root `544c83d8cd07165cfeec2f0db64a78c8417849f3`. This reviews the private contract/allocation/compiling RED gate only.

**Baseline:** Root `544c83d8cd07165cfeec2f0db64a78c8417849f3`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Governing documents were read with pinned `git show`; inspected fixture files were verified against committed bytes.

**Date:** 2026-10-03

**Axis:** Consistency with the controlling document graph, source modes, ownership contracts and executable consumers. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block. I pre-commit to GO on a revision that resolves P2-1, P2-2 and P2-3 as specified, preserves the reviewed scope, and passes their regression checks.

---

## 0. Evidence base

Read the comparison contract §§1–10; allocation through line 264; complete adaptation inventory; ProductionIntegrationPlan; QualificationPlan, including §6; AdoptionContract RA-001–012; ImplementationEvaluation; Q4-CarrierAudit; LibraryBoundaryAndTestingPolicy; GladePackageArchitecture; GladeBuildEntry; AGENTS files and review-loop skill. Checked the pinned Gyld Records, Runtime, StorageAdapter and NodeAssembly allocations.

Inspected the complete API, refusing providers, manifests, policy, compiler consumers, shared B0 consumers, oracle, source/work/transport fixtures, fixture tests, structural checks and recorded evidence. Particularly relevant ranges are:

- `spec/src/b0/sources.rs:88–176`
- `spec/src/fixture/transport.rs:47–86,172–243,327–344`
- `spec/src/fixture/work.rs:171–213`

Checked exact cached upstream OpenRaft construction, timeout sampling, ticker, vote-time handling, election guards, runtime bounds and standard leader identity; checked raft-rs tick/pre-vote behavior.

Independently executed all six README command groups, with their stated locked/offline selection:

- API compiler target: **2 passed**.
- Both provider compiler targets: **2 passed**.
- Constructor/fixture/oracle/RPC targets: **15 passed**.
- B0 election target: exit **101**, **20 ordinary behavioral assertion failures**, **0 passed, ignored or filtered**.
- `check.sh`: PASS, including five architecture negatives, explicit source boundaries and zero local global exceptions.
- All-four-package, all-target Clippy with `-D warnings`: PASS.

The committed `rpc-red.log` records four assertion failures against `NotQualified` before fixture response implementation; the current four RPC tests pass. I inspected that historical evidence, rather than recreating an earlier implementation.

At start and end, all four HEADs matched the tuple. All 50 inventory entries plus the inventory itself matched committed bytes and digests. The inventory digest is `d25e9045fa21f11cb513ca6e72712d57bdaeec49d2c0b5018312067f801b3a66`. The additional comparison contract and adaptation inventory also matched the pin. Only authorized ignored build artifacts were produced; no sources, tests, evidence or Git state were written.

## 1. Findings

### [P2-1] Runtime fault matrix requires entropy accesses forbidden by the selected OpenRaft cadence

**Location:** `proofs/raft-carrier-comparison/spec/src/b0/sources.rs:126–164`; comparison contract lines 75–82; allocation lines 122–123.

**Violated invariant:** Shared consumers must be satisfiable against the exact upstream behavior they require preserving.

**Counterexample:** Construct OpenRaft successfully and establish a leader, as lines 132–133 require. Inject `Fault::Entropy` into node 1. The consumer then advances/drives node 1 and requires `EntropyFailed` within 100 iterations. OpenRaft’s election timeout was already sampled during `EngineConfig::new`; the inspected generator has no subsequent election call site. The contract expressly prohibits introducing per-election resampling. A faithful adapter therefore never encounters this injected entropy failure and fails line 164.

The matrix’s comment correctly says faults are observed at their next actual use, but this particular next use does not exist.

**Impact:** A conforming OpenRaft adaptation cannot pass unchanged B0-03. Making it pass encourages an extra RNG probe or altered sampling cadence, contradicting the source-preservation contract.

**Required correction:** Make fault cuts depend on each carrier’s actual access footprint within the shared consumer. Preserve constructor entropy-failure coverage for both. For OpenRaft, post-construction entropy poisoning must demonstrate no extra sampling; runtime fatal-source assertions must target actual clock/work accesses. For raft-rs entropy failure, deliberately reach a reset path rather than assume isolated ticking necessarily does so.

**Closure test:** Add a footprint-faithful witness that samples entropy once during construction and never again: it must satisfy the revised OpenRaft cut. Reject a witness that adds runtime sampling. Preserve all ten case selections for both labels and later execute the reviewed cuts against actual adapted sources.

### [P2-2] Ordinary transport paths accept replaced incarnation scopes

**Location:** `spec/src/fixture/transport.rs:47–86,172–193,201–243`; allocation lines 95–102; comparison contract lines 164–166 and 191–193.

**Violated invariant:** Stale incarnation controls must refuse without queue mutation; old callbacks must not participate in a replacement scope.

**Counterexample:** Retain an endpoint for node 2/incarnation 1. Call `fixture.inputs(2, 2)`, which replaces node 2’s current scope in `Network.scopes`. The old endpoint can still:

1. `emit` a one-way message to node 1: `emit_inner` checks its independent stopped flag, group and carrier, but never checks the issuer against the current scope registry.
2. Release that token: `control` checks only `token.issuer == self.scope`.
3. Have node 1 consume it: `take` checks destination and deliverability, but never checks the issuer’s current incarnation.

Each step returns success on the inspected branches. An already-issued old token has the same problem. The RPC `respond`/`Resolve` paths perform current-scope checks, but ordinary raft-rs traffic bypasses those protections.

**Impact:** Stale scheduled work can enqueue and deliver protocol bytes after replacement. The fixture ceases to enforce its claimed incarnation boundary, and engine payload validation cannot supply the missing fixture lifecycle check.

**Required correction:** Centralize live-scope and stopped-incarnation validation for emitting, requesting, consuming and controlling messages. Scope lifecycle state must be shared across endpoint handles, rather than recreated independently by each `Fixture::endpoint` call. Invalid operations must preserve sequence numbers, queues and pending RPCs.

**Closure test:** Retain old endpoint handles and held tokens, replace their incarnation, then exercise emit/request/release/duplicate/take. Require typed refusal and unchanged inventories. Also stop an incarnation and verify a newly obtained handle for that same incarnation cannot resume participation. Keep valid replacement traffic and existing RPC tests GREEN.

### [P2-3] Cancellation drops owned futures while holding the scheduler’s mutable borrow

**Location:** `spec/src/fixture/work.rs:171–188,204–213`; allocation lines 58–65 and 99–102.

**Violated invariant:** Owned futures may contain child cancellation/join guards, and scope cleanup must complete without panicking.

**Counterexample:** Register a child task. Register a parent future owning a guard whose `Drop` cancels that child through a cloned `WorkProbe`. Cancel the parent. Line 187 drops its future while `state` still holds `self.0.borrow_mut()`. The guard calls `cancel(child)`, which attempts another mutable borrow of the same `RefCell` and panics. `stop()` has the same failure at line 209.

This is a legal owned-future composition and directly matches the required cancellation-guard ownership model.

**Impact:** Normal parent cancellation or construction/fatal cleanup can panic instead of terminating scoped work. Existing fixture tests use futures without reentrant cleanup guards, so their GREEN result does not cover this lifecycle.

**Required correction:** Mark terminal state and detach futures while borrowing scheduler state, release that borrow, then drop futures. During scope stop, establish the stopped/terminal state before invoking destructors.

**Closure test:** Add parent/child futures with guards that cancel child work and inspect scheduler inventory during destruction. Both individual cancellation and scope stop must finish without panic, destroy each future once, leave terminal inventories, and refuse later work.

## 2. Invariant analysis

The four-package allocation is coherent for this stage: API has no implementation dependency; spec normally depends only on API; its two development providers remain std-only refusing scaffolds. Meaningful required methods, actual boxed futures, typed errors and separately constructed source contexts exist. Structural PASS is not represented as behavioral conformance.

The selected OpenRaft `single-term-leader` and `singlethreaded` features exist in the pin. Full opaque vote identity remains separate from the candidate-free epoch key. Signed internal time arithmetic, lease/greater-log guards, constructor cleanup, Ready/LightReady and fatal-source adaptations remain explicit future obligations.

B1 application/disk and B2 membership/snapshot exits remain mandatory before B3 selection. Neither candidate is silently dropped. The documents preserve RA obligations and distinguish fixture identity, memory retention, process-crash profiles, cryptography and independent domains. No production authority, receipt reinterpretation, default activation or carrier selection is claimed.

## 3. Risks and next action

The downstream B0 schedules have not executed against functioning engines; their present RED results establish compilation and first assertion reachability only. Source adaptation, dependency/global inventories and runtime qualification remain mandatory later gates.

The next action is one bounded remediation patch addressing P2-1–3, with failing regressions first, followed by focused independent re-review before accepting this contract/allocation/RED object.