# Q4-B0 contract/allocation/compiling RED — CONSISTENCY-AXIS REVIEW

**Review object:** Complete DRAFT comparison contract, B0 allocation, adaptation inventory and `proofs/raft-carrier-comparison/` at root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`. Fresh full acceptance review of the private contract/allocation/compiling RED object; no runtime or production qualification.

**Baseline:** Root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Documents were read through pinned `git show`; executable local files matched committed bytes.

**Date:** 2026-10-03

**Axis:** Consistency with governing contracts, pinned source behavior, ownership boundaries and executable consumers. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one new P2 finding blocks. All prior counterexamples verify corrected here. I pre-commit to GO on a revision resolving P2-5 as specified while preserving scope and passing the existing gates.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Original Consistency P2-1 | Carrier-specific source-access cuts | Retraced constructor-only OpenRaft sampling and deliberate raft-rs quorum-loss reset; executed once-only witness and extra-draw rejection | Verified closed here |
| Original Safety P2-1 | Same cadence correction | Same original impossible runtime-entropy requirement retraced and removed | Verified closed here |
| Original Consistency P2-2 | Shared stop/current-incarnation authority | Executed old-handle emit/request/control/take and shared/fresh-handle stop regressions; unchanged inventories and sequence allocation | Verified closed here |
| Original Safety P2-2 | Same endpoint correction | Executed separate-handle stop and stale issuer/destination ordinary-traffic counterexamples | Verified closed here |
| Original Safety P2-3 | Reconcile stop during selected polling | Executed future stopping its own scope and returning Pending; exact-once destruction, terminal inventory and saved-wake exclusion pass | Verified closed here |
| Original Consistency P2-3 | Detach futures before destruction | Executed parent cancellation and bulk-stop destructors cancelling/inspecting children without panic | Verified closed here |
| Original Safety P2-4 | Same destructor correction | Same counterexamples pass; rejected registration, overflow and completed-future cleanup also pass | Verified closed here |
| Remediation-1 Consistency P2-4 | Caller-owned termination independent of peer liveness | Executed held/consumed × stopped/replaced × Timeout/Cancel matrix, actual Pending futures, typed completion, negative ownership and late replies | Verified closed here |
| Remediation-1 Safety P2-5 | Same local termination correction | Same eight counterexamples pass; isolated peer-liveness mutant fails all eight | Verified closed here |

This fresh review verifies counterexamples independently. Required originating-finder testimony remains a separate filing; no current finder-closure or peer report was read.

## Changed-range analysis

Since `256be2dd0fd652b34dfffa753fcba481f5fb842b`, the scoped correction splits `Network::owned_rpc` from live-peer `Network::rpc`, changes Timeout/Cancel to the former, corrects two endpoint expectations, and adds termination regressions, mutant tooling and evidence. Allocation/README descriptions and measurement invocation are updated.

The cumulative correction since `544c83d8cd07165cfeec2f0db64a78c8417849f3` also repairs source-footprint consumers and endpoint/scheduler lifecycle enforcement. Public API, manifests, lockfile, refusing providers and comparison-contract text remain unchanged.

**New-root classification:** P2-5 below is **non-architectural**: an existing consumer’s scheduling loop fails to exercise the already coherent explicit-poll contract. Its remedy changes test scheduling, without changing signatures, ownership, upstream algorithms or package boundaries. It is not a third new architectural root cause and does not itself trigger the architectural stop cap.

## 0. Evidence base

Read the full canonical prompt; its SHA-256 matches `ef22ad46be97466fc571b3fe864ef394360bf134f5e7dd3d6dea81ef32155a23`. Read both original reports, both remediation-1 reports and both remediation plans.

Read comparison contract §§1–10, complete allocation and adaptation inventory; governing ProductionIntegrationPlan, QualificationPlan including §6, AdoptionContract RA-001–012, ImplementationEvaluation, Q4-CarrierAudit, LibraryBoundaryAndTestingPolicy, PackageArchitecture and BuildEntry; AGENTS files and review-loop skill. Inspected Gyld Records, Runtime, StorageAdapter and NodeAssembly.

Inspected API, providers, manifests/policy/lockfile, compiler witnesses, all B0 consumers, oracle, fixtures, lifecycle/RPC tests, check scripts, measurement tooling and evidence. Material ranges include:

- `spec/src/b0/elections.rs:17–63`
- `api/src/lib.rs:400–414`
- `spec/src/fixture/transport.rs:41–95,350–371`
- `spec/src/fixture/transport/termination.rs:1–341`
- `spec/src/fixture/work.rs:61–299`

Checked cached upstream timing/cadence and OpenRaft feature/runtime/vote paths, particularly `core/raft_core.rs:1056–1088,1440–1507`, `core/tick.rs:56–105`, and raft-rs reset/tick/timeout predicates.

Independently ran the README commands:

- API witnesses: **2 passed**; provider witnesses: **2 passed**.
- Spec unit/compiler/fixture/oracle/lifecycle witnesses: **41 passed**.
- Ordinary `b0_election`: exit **101**, **20 assertion failures**, zero passed/ignored/filtered.
- Structural/source/global/format gate: PASS; five architecture negatives rejected; 24 Rust files, 14 normal-source files, zero global exceptions.
- All-four-package all-target Clippy with denied warnings: PASS.
- Isolated termination mutant: compiled, **8 required failures and 6 passes**, zero ignored/filtered; wrapper correctly reported rejection.

Historical logs independently show RPC’s four compiling RED failures, remediation-1 regression RED, and remediation-2’s ten assertion failures. Historical RED was inspected, not reconstructed. `measure.py` was not run.

Before and after execution, reviewed bytes matched the pin. Final verification covered 105 committed object paths, 102 inventory entries and documentary-context hashes, with no mismatches or untracked non-output subtree files. Current inventory SHA-256 is `5415b79ebaed75de6c5bf49d7619b0661516651c6982a427aef70b6fbef8e789`. All four HEADs remained unchanged. Only authorized ignored build/mutant artifacts were produced.

## 1. Findings

### [P2-5] B0-04 requires outbound votes before selecting the work that emits them

**Location:** `proofs/raft-carrier-comparison/spec/src/b0/elections.rs:27–61`; required one-action semantics at `api/src/lib.rs:400–401` and allocation lines 58–65.

**Violated invariant:** The shared consumer must be satisfiable against pinned OpenRaft while preserving explicit scheduling and prohibiting inline spawn/poll progress.

**Counterexample:** At `first_eligible`, line 49 captures one inventory snapshot. Lines 50–54 drive only entries already marked Runnable in that snapshot. Line 55 immediately requires an emitted VoteRequest.

Pinned OpenRaft’s election processing invokes `spawn_parallel_vote_requests`. At `core/raft_core.rs:1082–1088`, each outbound vote operation is inside a **newly spawned async task**; `client.vote(req, option)` is reached only when that task is polled. These tasks are created during core processing, after the consumer’s snapshot, and therefore receive no selected drive in this sweep. Their registration cannot execute them inline.

The normal ticker-to-core wake introduces another skipped step: a core task Pending in the snapshot is still skipped by the copied `WorkState`, even if polling the ticker wakes it. Even granting an already-Runnable core and successful election processing, the newly spawned vote tasks remain unpolled. The assertion therefore fails on faithful scheduling.

**Impact:** An unchanged mandatory consumer rejects a valid OpenRaft adapter or pressures it to poll spawned work inline, conceal scheduler work, or move network emission outside the pinned task path. Present refusal RED stops before this defect and cannot validate the downstream schedule.

**Required correction:** Establish initialization and timing prerequisites explicitly. At each tested logical instant, repeatedly obtain current inventory and select eligible work through bounded, recorded drive actions, including newly awakened/spawned tasks, while holding time and message delivery fixed. Preserve one poll per action. Verify election eligibility separately from the subsequent scheduled network emission; do not redefine the boundary to hide missing polls.

**Closure test:** Add a carrier-free scheduling witness with ticker → awakened core → newly spawned vote task. Registration/wakes must remain inert until their own selected polls. The corrected common boundary consumer must traverse this chain and reject early election or inline-spawn mutants. Preserve both B0-04 selections and all twenty ordinary refusal failures. Later execute the same consumer against actual adapted OpenRaft.

## 2. Invariant analysis

The four std-only roles and complete declared edges remain coherent. Constructors retain separately supplied dependencies without source probing; actual boxed futures and typed ownership/errors are present. Future engines must leave spec’s development graph through reviewed runner allocation.

Caller-local termination now survives peer loss, while respond/resolve/delivery retain live-scope and issued-correlation validation. Invalid ownership controls preserve inventories; terminal reuse and stale/live-peer late replies refuse. Scheduler cleanup survives reentrant destruction and stop during polling.

Full opaque votes remain separate from candidate-free epochs. OpenRaft’s proposed single-term-leader/singlethreaded/storage-v2 mode and constructor-only sampling remain explicit. Signed internal time, lease/greater-log guards, Ready/LightReady, fatal-source propagation and actual task cleanup remain mandatory later proofs.

B1/B2 and full comparison before B3 selection remain mandatory; neither candidate disappears. No production authority, receipt reinterpretation, default activation, source adaptation or engine qualification is claimed.

## 3. Risks and next action

GREEN fixtures, synthetic detectors and measured scaffold costs establish no real-engine, dependency-source, durability or production assurance.

The next action is a bounded regression-first correction of P2-5’s consumer schedule, followed by renewed applicable review. The two architectural remediation rounds remain recorded; no third architectural patch is authorized.