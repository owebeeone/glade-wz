# Q4-B0 remediation 3 — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT private contract/allocation/compiling RED object at root `8553bc71b1f6bc9fe203729e61567e8b9bd37be2`, controlled by `dev-docs/GladeRaftB0-RemPlan-3.md`. Original-finder re-verdict of the bounded scheduling correction, independent verification of the domain correction, and changed-range analysis.

**Baseline:** Root `8553bc71b1f6bc9fe203729e61567e8b9bd37be2`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were read through pinned `git show`, restricted diff and verified matching local bytes.

**Date:** 2026-10-03

**Axis:** Consistency: original-counterexample closure, impacted consumers, governing-contract preservation and correction integrity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — Consistency P2-5 closes; Safety P2-6’s correction independently verifies here. No new findings. This accepts the private contract/allocation/compiling RED object on this axis only; it establishes no engine, source-adaptation, runtime or production qualification.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Original Consistency P2-1 | Carrier-specific source-access cuts | Replayed constructor-only OpenRaft witness, extra-draw rejection and persistent-registration witness; unchanged consumers retain documented raft-rs reset selection | Closure retained |
| Original Safety P2-1 | Same cadence correction | Same counterexample retraced; impossible unconditional OpenRaft runtime entropy failure remains removed | Closure retained |
| Original Consistency P2-2 | Shared endpoint lifecycle authority | Replayed stale emit/request/control/take and shared/fresh-handle stop regressions | Closure retained |
| Original Safety P2-2 | Same ordinary-traffic correction | Replayed replaced issuer/destination refusals and valid replacement participation | Closure retained |
| Original Safety P2-3 | Reconcile stop during polling | Own-scope stop, exact-once destruction, terminal inventory and saved-wake exclusion pass | Closure retained |
| Original Consistency P2-3 | Destroy detached futures outside borrows | Replayed parent/child cancellation, bulk stop, rejected registration and completed-future cleanup | Closure retained |
| Original Safety P2-4 | Same destructor correction | Original reentrant cleanup counterexamples pass without borrow panic | Closure retained |
| Remediation-1 Consistency P2-4 | Caller-local termination independent of peer liveness | Eight original stop/replacement termination cells pass; exact defect mutant fails all eight | Closure retained |
| Remediation-1 Safety P2-5 | Same termination correction | Actual Pending futures, typed completion, negative ownership and late replies pass | Closure retained |
| Fresh Consistency P2-5 | Refresh selected work and establish actual timing prerequisites | Original pending-core/new-send chain reaches four separately selected polls; corrected B0-04 retraced; early/inline mutants and bounds pass | Closed by original finder |
| Fresh Safety P2-6 | Collision-free bounded domain encoding | Original node1/inc11 versus node2/inc1, bilateral foreign instants/deadlines, neighbors, limits and replay pass | Verified corrected here; formal finder closure belongs to Safety |

## Changed-range analysis

Since `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`, implementation changes are confined to B0-04 scheduling and fixture domain construction. New private point utilities/witnesses and domain regressions accompany them. Allocation/README and current evidence describe the correction. The termination mutation checker changes only its expected unrelated passing-unit count.

Public API, manifests, lockfile, package classifications, declared edges, allowlists, refusing providers and twenty case labels are unchanged. The comparison contract, adaptation inventory and all previously reviewed governing documents are byte-identical. Prior evidence is preserved.

**Architectural classification:** No new architectural root cause identified. The patch follows existing explicit-poll and domain-isolation contracts without changing ownership or interfaces. Both corrections remain non-architectural. Two architectural remediation rounds remain used; this permitted third correction does not trigger the architectural stop condition.

## 0. Evidence base

Read the full canonical prompt; SHA-256 matches `50f588a080c8acfbf3a1f9b7c1322c22d602cc4af5c463c0a38d4e1a1ec6be05`. Read RemPlan-3, both previous full reports and both remediation-2 originating-finder closures as legitimate prior inputs. No current peer report was read.

Retained the previous full governing-graph review only after independently verifying unchanged bytes for AGENTS files, BuildEntry, ProductionIntegrationPlan, QualificationPlan including §6, AdoptionContract, ImplementationEvaluation, Q4-CarrierAudit, library policy, package architecture, comparison contract and adaptation inventory.

Inspected the complete changed implementation and documentary ranges, particularly:

- `spec/src/b0/elections.rs:17–147`
- `spec/src/b0/point.rs:1–46`
- `spec/src/b0/point/witness.rs`, complete
- `spec/src/fixture.rs:18–19,49–94,108–138`
- `spec/src/fixture/domains.rs:1–149`
- Allocation’s remediation-3 section, README, mutation checker and current evidence/context inventories

Independently executed the README’s three GREEN commands, ordinary RED command, structural gate, all-four-package all-target Clippy with denied warnings, and authorized isolated termination mutation:

- **58 GREEN:** 54 spec/unit/compiler/fixture/oracle witnesses and four API/provider witnesses.
- **20 ordinary B0 assertion failures:** exit 101; zero passed, ignored or filtered.
- Structural/global/source/format gate PASS: 27 Rust files, 17 normal-source files, zero allowlisted items; five architecture negatives rejected.
- Clippy PASS.
- Termination mutant compiled and produced exactly **eight required failures and nineteen unrelated passes**, zero ignored/filtered.

Historical `rem3-red.log` records compilation followed by ten failures and fourteen passes. `rem3-red-time.log` adds one distinct host-tick regression: eleven failures and fourteen passes. Independently counted eleven unique failed regression names. Initial GREEN records twenty-five unit passes. Authoring compiler errors remain separately identified and are not RED. Historical executions were inspected, not reconstructed; `measure.py` was not run.

Current measurements distinguish expected RED, execution-only, warm build, fresh build and gates. Their scaffold limitations remain explicit.

At start and end, all four HEADs matched. Before/after checks found no committed-byte mismatch, context-hash mismatch or untracked non-output subtree file. Final verification covered 139 object paths and all 136 current inventory entries. Inventory SHA-256 is `bf01cd40a7b2c9a4e1eb45abd03c60d7b62aaadbe2830d7ba0665d26c6276fb7`. Only authorized ignored build/mutant artifacts were produced.

## 2. Invariant analysis

The original scheduling counterexample is corrected. Startup explicitly parks the pending core at unchanged time. At the deadline, the selected ticker wakes that core; a refreshed inventory selects the core; another refresh selects the newly registered send future. The witness records `startup → tick → eligible → send` as four distinct selected actions. Registration remains inert, clocks stay fixed inside each drain, and no protocol delivery occurs.

The actual B0-04 consumer uses this same drain utility. It refreshes inventory after every selected drive, initializes before refreshing the timing boundary, and advances through aligned logical points plus exact partial boundaries. It records the first Candidate/PreCandidate observation separately from subsequent vote emission. The previous stale snapshot no longer suppresses awakened or newly spawned work.

The host-tick witness selects four real actions at four separate millisecond points; a time jump is not treated as four elapsed ticks. Early-eligibility and inline-send mutants are rejected through external action/time records. Perpetually self-woken work reaches the explicit 4096-action bound; domain, monotonicity, partial points and time-point exhaustion are checked. These remain carrier-free mechanics, with no synthetic ElectionNode or engine result.

Domain construction is injective over its documented supported coordinates: disjoint carrier/session/node/incarnation widths are 1/31/2/30 bits, with fixed group 7. Bounds precede source/work construction. The original collision now yields distinct domains; bilateral foreign source/scheduler instants and deadlines return `WrongDomain` without changing clocks, traces or work inventories. Neighbor, session/carrier and maximum-coordinate tests pass, including maximum issued construction. Domain identity `u64::MAX` is correctly distinct from time-nanos overflow. Deterministic replay uses explicit caller namespaces without global allocation.

All earlier lifecycle regressions remain GREEN. Caller-local termination still survives remote loss, strict delivery/correlation checks remain intact, and arbitrary future destruction occurs outside scheduler borrows.

The unchanged four-package allocation, identical exported consumers, candidate-free epochs with retained opaque votes, mandatory B1/B2/full B3 comparison and separate Q4-C/D/E gates retain their prior consistency proof. No candidate is dropped or production receipt/default/authority claim introduced.

## 3. Risks and next action

Actual adapted engines must still execute B0-04 and the other common consumers, including pinned timing, fatal-source, Ready/LightReady and task-cleanup obligations. Fixture GREEN and synthetic detectors establish no source adaptation, real election, durability, dependency-security or production qualification. The separate SourceAdaptationPlan remains unadopted.

The next action is to file this originating Consistency closure and combine it with the independent Safety verdict on the same tuple for the private contract/allocation/compiling RED gate.