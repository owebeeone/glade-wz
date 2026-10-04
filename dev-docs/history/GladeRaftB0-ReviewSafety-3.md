# Q4-B0 remediation 3 — SAFETY-AXIS REVIEW

**Review object:** DRAFT private contract/allocation/compiling RED object at root `8553bc71b1f6bc9fe203729e61567e8b9bd37be2`, controlled by `dev-docs/GladeRaftB0-RemPlan-3.md`. Original-finder re-verdict of Safety P2-6, with independent analysis of the scheduling correction and changed range.

**Baseline:** Root `8553bc71b1f6bc9fe203729e61567e8b9bd37be2`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were read through pinned `git show`, scoped comparison and verified matching local bytes.

**Date:** 2026-10-03  
**Axis:** Safety: original-counterexample closure, clock isolation, explicit scheduling, lifecycle preservation and honest evidence boundaries. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — Safety P2-6 closes; the scheduling correction independently verifies corrected here; zero new findings. This accepts only the private contract/allocation/compiling RED scope.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Safety P2-1 | Preserve actual entropy cadence | Re-executed once-only OpenRaft witness and extra-read detector; unchanged B0-02/03 preserve constructor-only sampling and raft-rs quorum-loss reset selection. | Earlier closure retained |
| Consistency P2-1 | Same cadence correction | Original impossible runtime-entropy requirement remains removed; source-footprint witnesses pass. | Verified corrected |
| Safety P2-2 | Shared endpoint lifecycle authority | Re-executed separate/fresh-handle stop and replaced-issuer/destination ordinary-traffic regressions, with unchanged inventories and sequence allocation. | Earlier closure retained |
| Consistency P2-2 | Reject stale ordinary traffic | Original emit/request/control/take paths remain protected by shared current-scope validation. | Verified corrected |
| Safety P2-3 | Reconcile own-scope stop during poll | Future stopping its scope and returning Pending drops once; inventory is Cancelled; saved wake cannot revive it. | Earlier closure retained |
| Safety P2-4 | Detach futures before destruction | Parent cancellation, bulk stop, rejected registration, overflow and completed-future reentrant cleanup pass without borrow panic. | Earlier closure retained |
| Consistency P2-3 | Same destructor correction | Original child-cancellation/inspection destructor sequences re-executed successfully. | Verified corrected |
| Safety P2-5 | Restore caller-owned Timeout/Cancel | All eight held/consumed × stopped/replaced-peer × Timeout/Cancel cells pass; exact defect mutant rejects all eight. | Earlier finder closure retained |
| Consistency P2-4 | Same local termination correction | Typed completion, selected ownership removal, negative controls, late replies and continued caller traffic remain GREEN. | Earlier finder closure retained |
| Safety P2-6 | Collision-free bounded domains | Original node1/inc11 versus node2/inc1 pair is distinct; bilateral foreign source/scheduler instants and deadlines refuse unchanged; neighbors, limits, maximum construction and replay pass. | **Closed by originating finder** |
| Fresh Consistency P2-5 | Refresh boundary work selection | Executed ticker → awakened pending core → newly spawned send chain through the common drain; separate selected polls, fixed time, early/inline rejection and actual host-tick progression pass. | Verified corrected here; originating Consistency closure is separate |

## Changed-range analysis

The scoped patch changes fixture domain encoding and B0-04 scheduling, adds private domain/point regression modules, updates allocation/README/evidence, and mechanically updates the termination mutant’s unrelated passing-unit count.

Domain arithmetic now uses disjoint carrier/session/node/incarnation fields with explicit bounds checked before source/work assembly. Existing source/work ownership and typed foreign-domain refusal remain unchanged.

B0-04 initializes work at unchanged constructor time, refreshes timing afterward, advances through actual aligned logical points, and repeatedly obtains current inventory after each selected drive. Election eligibility is recorded separately from subsequent scheduled vote emission.

Verified unchanged governing documents, comparison contract, adaptation inventory, public API, package roles/edges, manifests, lockfile, refusing providers, source/work/transport implementations and all case labels. Previous full-review reasoning is retained only for those unchanged bytes; impacted consumers and lifecycle regressions were rerun.

**New-root classification:** No new architectural or non-architectural root cause identified. Both corrections remain implementation repairs within existing coherent contracts. Two architectural rounds remain used; this third confined non-architectural correction does not trigger the architectural stop cap.

## 0. Evidence base

Read the full canonical remediation-3 prompt; SHA-256 matches `096d58d26654b992aabde7f475057fade6e5e21d332be7665588cd50fdabf4b0`. Read RemPlan-3, both prior full reports and both prior original-finder closure reports. Earlier original/remediation-1 reports and governing-source review remain available from the preceding full review. No current peer report was read.

Verified that AGENTS authority, ProductionIntegrationPlan, QualificationPlan including §6, AdoptionContract, ImplementationEvaluation, Q4-CarrierAudit, library policy, package architecture, BuildEntry, comparison contract and adaptation inventory are byte-identical to the prior full-review pin.

Material changed ranges inspected:

- `spec/src/fixture.rs:47–110`
- `spec/src/fixture/domains.rs:1–149`
- `spec/src/b0/elections.rs:17–145`
- `spec/src/b0/point.rs:1–46`
- `spec/src/b0/point/witness.rs:1–301`
- allocation’s remediation-3 section and current README/evidence

Rechecked pinned OpenRaft ticker and spawned vote-task paths, particularly `core/tick.rs:81–105` and `core/raft_core.rs:1056–1100`, against the corrected scheduling sequence.

Independently executed the exact README commands and authorized termination mutant:

- **58 GREEN:** 54 spec/unit/compiler/fixture/oracle tests plus four API/provider compiler witnesses; zero ignored/filtered.
- Ordinary B0 target: exit **101**, **20 assertion failures**, zero passed/ignored/filtered.
- Structural/source/global/format checks: PASS; 27 Rust files, 17 normal-source files, zero global exceptions, five rejected architecture negatives.
- Four-package all-target Clippy with denied warnings: PASS.
- Isolated termination mutant: compiled; exactly **eight required failures and 19 other passing units**, zero ignored/filtered.

Inspected historical `rem3-red.log`: ten compiling failures, including `ClockDomain(121)` aliasing and accepted foreign controls. The time supplement adds one distinct host-tick regression, producing eleven failures. Authoring compile errors remain separate. Historical RED was inspected, not reconstructed. Current measurements distinguish scaffold execution, warm build and fresh build without qualification claims. `measure.py` was not run.

All four HEADs matched at start and end. Verified **136 inventory entries plus manifest**, documentary-context hashes and all non-output proof files against committed bytes. Inventory SHA-256:

`bf01cd40a7b2c9a4e1eb45abd03c60d7b62aaadbe2830d7ba0665d26c6276fb7`

The scoped diff remained empty. Only authorized ignored build/mutant artifacts were produced.

## 2. Invariant analysis

The original P2-6 counterexample no longer aliases: carrier uses bit 63, session bits 32–62, node bits 30–31, and incarnation bits 0–29. Checked field limits prevent overlap. Both supported carriers produce distinct domains for node1/inc11 and node2/inc1.

Bilateral foreign source advances preserve clocks and trace. Foreign scheduler advances preserve inventory and cannot enable waiting work; foreign deadlines register no work. Invalid coordinates refuse before probe/source/work assembly. Maximum issued coordinates construct and run actual owned source/work, including the valid `u64::MAX` domain identity. Deterministic replay needs no global allocator.

The scheduling witness traverses startup, ticker, awakened core and newly registered send through four distinct selected actions. Registration and wake remain inert until polling. Time and delivery remain fixed inside each drain; action/point bounds fail explicitly. Actual selected host ticks occur at separate logical points rather than being inferred from a time jump.

These witnesses exercise carrier-free mechanics. They do not establish actual engine elections, upstream eligibility mapping or adapted-runtime behavior.

Prior endpoint/RPC/scheduler safety properties remain intact. Both labels retain all ten common consumers. B1/B2/full B3 comparison remain mandatory, with neither candidate dropped and no production authority, receipt reinterpretation or default activation claim.

## 3. Risks and next action

Real engines, source adaptations, Ready/LightReady, fatal-source propagation, real runtime cleanup, durability and production qualification remain unexecuted mandatory gates. The separate SourceAdaptationPlan remains out of scope.

File this originating Safety closure and combine it with the independent Consistency re-verdict on this exact tuple to determine aggregate contract/allocation/compiling RED acceptance.