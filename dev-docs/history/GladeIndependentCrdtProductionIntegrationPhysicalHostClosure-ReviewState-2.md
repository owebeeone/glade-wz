# Combined IC-3B physical host, originating closure 2 — STATE-AXIS REVIEW

**Review object:** SAME combined IC-3B genuine authentication/physical persistence component, second bounded correction, at Glade `a47691598df648eb8c9554b27f3d06b0cffcf596`. Controlling DRAFT: `/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`.

**Baseline:** The following exact heads matched at START and END. Sources were inspected through pinned Git content, scoped diffs and current files checked against the committed source manifest.

| Repository | START = END |
|---|---|
| Workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

**Date:** 2026-10-04

**Axis:** Originating State closure of P2-4, preservation of three earlier closures, complete changed-range and boundary analysis, and root classification. Independent, adversarial, read-only. Fresh full reviews run separately; nothing here relies on current peer testimony. Filed verbatim by the lane owner.

**Verdict: GO** — all four own State counterexamples are CLOSED; no additional P0–P3 finding was established. This originating closure does not replace fresh full acceptance or open the accepted-B prerequisite.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample independently verified | Status |
|---|---|---|---|
| State P2-1: settlement detached from receive result | B-R1 retains private issuer-owned successful bounded length/digest and checks normal settlement against it | The second correction preserves `GuardedReceive` custody and the comparison before observation publication. Different signed substitution, malformed prefix and failed receive cannot become successful normal settlement. Original public refusal/death controls remain selected in the final recorded disk run. The oversized-loss change retains this distinction. | **CLOSED; preserved** |
| State P2-2: refused calls forget learned floors | B-R2 marks before consultation, retains valid learned knowledge despite denial and refuses unresolved recovery | Both consultations remain protected. The newly returned retained cut is the cut actually selected, and denial does not discard it. Original denied authentication/ingress, chained revocation and observation-marker cut controls remain selected. Changed ObservePolicy accounting and restart tests additionally verify that issued Fence requests remain loadable. | **CLOSED; preserved** |
| State P2-3: refused settlement hides its only loss capability | B-R4 supplies public `retain_pending_ingress_loss` | The operation still uses the host-owned original permit and observation/checkpoint. The M+1 extension now permits bounded permanent-loss publication after normal settlement refuses capacity. Failure leaves Unknown/pending custody intact; retry cannot bypass it. Recorded public kill/reopen cases cover refused, failed and selected outcomes without extracting private capabilities. | **CLOSED; preserved** |
| State P2-4: Begin uses an earlier allow decision after retaining a newer cut | B-R6 binds Begin authorization to the complete durably retained policy/time cut and drains selected Fence effects | The exact expiry and chained node-revocation sequences between consultations now produce no accepted head, one original TerminalNonCommit and clean close/reopen. `storage::begin` compares the retained policy digest and full interval with the sealed precondition before Started. Three new Fence publication/death cuts retain the original request and recover legally. | **CLOSED** |

Writer dispositions were treated as claims. Closure rests on the source ordering, original counterexample replay in recorded compiled regressions, and preservation of the surrounding invariants.

## Changed-range analysis

I inspected the complete Glade range `d3fded040d6e3c459cd5f975d6c672873356cc23..a47691598df648eb8c9554b27f3d06b0cffcf596`, including every production, test and source-boundary change. Retained context includes the original combined component and first correction, including the `c69e6416f5f5155d4bb570bf272e796b2deae0a3` and `9dbc677a25d93af0c561817571ad7eb1990ddadc` baselines. Preservation was checked against actual changed consumers rather than inferred from an unchanged public signature.

| Changed files under Glade | Review consequence |
|---|---|
| `node/src/independent/records/authentication.rs` | Returns the durably retained cut internally, couples invocation counters to ObservePolicy state, and drains effects after successful selection |
| `node/src/independent/records/storage.rs` | Requires complete retained-cut equality before protected Started |
| `node/src/independent/records.rs` | Uses retained policy and full interval for ingress authorization |
| `node/src/independent/records/loss.rs` | Compacts pending settlement digest custody and permits host-owned oversized refused settlement to reach bounded loss |
| `node/src/independent/records/tests.rs` and `tests/fences.rs` | Registers new coverage and permits lookup of the same original Fence in live or retired state |
| `node/src/independent/records/tests/oversized_loss.rs` | Checks retained original custody after failed M+1 loss publication |
| `node/src/independent/records/tests/retained_cut_crash.rs` | Exercises three actual Fence publication/death cuts |
| `node/tests/ic3_disk.rs` and `ic3_disk_remediation2/mod.rs` | Public expiry/revocation, ingress interval, oversized loss, stable-cut and child-death witnesses |
| `contracts/crdt-recovery-api/tests/source_boundaries.rs` | Extends disabled-branch/source selection coverage to the new test subtree |

I also inspected all five active document changes: Authentication, Persistence, PersistenceUsage, TypedContract and TypedUsage. They describe retained-cut authorization and host-owned loss custody consistently. The cold utility retirement clarification does not supply a replacement pending-loss capability or weaken Unknown refusal.

The helper’s internal contract and callback graph changed materially. I inspected challenge/response persistence, submit/query, Verify/Seal completion, protected Begin, ingress consumption, runtime draining and recovery reattachment. ObservePolicy can issue Fence and reporting effects; its selected Fence uses the existing NonCommit callback and historical binding validation. That callback does not obtain a new current cut or mint replacement Verify/Prepare work. This supports retained State proofs, while the shared callback change still requires the separately commissioned fresh full affected reviews.

No serialized grammar, public loss capability, historical receipt identity, Prepare replay contract, compatibility marker or package dependency was changed by this range.

| Controlling object | Classification and preserved accounting |
|---|---|
| Semantic object | `2 architectural / 1 nonarchitectural / 1 completed remediation`, unchanged |
| Typed object | `2 architectural / 5 nonarchitectural / 2 completed remediations`, unchanged; third architectural root remains STOP |
| Combined B object | `0 architectural / 6 nonarchitectural / 2 completed remediations`, as committed in the ledger |
| Own P2-4 | Nonarchitectural B implementation ordering root: an existing sealed-start obligation was evaluated against the wrong consultation |
| Additional roots | None established; no count increment or exceptional remediation authorization |

The accounting does not convert implementation GREEN into reviewer closure. The maximum two merged remediation rounds remains binding; any further round requires the recorded explicit exception, and architectural STOP rules remain intact.

## 0. Evidence base

The principal current evidence resides under `/Volumes/projects/limbo/glade-wz/dev-docs/history/`:

| Evidence inspected | Result |
|---|---|
| Full `PhysicalHostClosure-PromptState-2.md`, own original State report, own closure-1 report, merged RemPlan-1/2 | Exact mandate and original counterexamples established; no current peer reports read |
| Full Remediation2Evidence, SourcePins and decompressed chronological RunLog | Failures, corrections, final selections and limitations inspected |
| Current source manifest | All 65 source hashes matched working bytes and committed HEAD; changed-file set exactly matched the 11-file range |
| Compatibility/canonical/immutable manifests | 45 compatibility, 27 canonical and 23 immutable-artifact entries matched |
| Current and prior document pins | Five current hashes matched; five prior remediation document hashes matched their pinned root content |
| Recorded raw log and compressed artifact | Raw: 71,711 bytes, SHA-256 `4d95a09b37610a7e1908e0179c5eb3dd4f27d2d6c69881a44524c0e0c1d61476`; gzip: 14,622 bytes, SHA-256 `28c07bef73a2b83848418998b2218a8017fea5037887d575ca08890779eb1c03` |
| Production ordering | Full `records/authentication.rs`; `records/storage.rs` Begin and terminal paths; `records.rs` ingress; full changed loss path; affected runtime/recovery and core ObservePolicy/Fence consumers |
| New regression source | Full 508-line public remediation module, 158-line retained-cut crash module and 68-line oversized-loss module |

The recorded initial compiled regressions failed on the intended revocation, expiry, ingress full-interval and M+1 pending-loss assertions. Import/type compilation failures were not treated as behavioural RED evidence.

The first compiled correction still failed clean-close expectations on two start cases, prompting selected-effect draining. The first new Fence crash execution then failed reopen with Integrity, exposing missing issuance accounting. The final source couples that accounting to the selected image; the recorded three-cut execution passes. The fence fixture adjustment retrieves the same original request from live or retired storage and preserves its terminal and contrary-callback assertions.

Final recorded verification includes:

- Public disk suite: 29 passed, three ignored helpers.
- Selected records suite: 29 passed, six ignored helpers; the unchanged 171-cut candidate/security/fork matrix explicitly skipped.
- Selected physical cuts: 57 native, 65 ingress, 26 observation-marker and three new Fence cuts, plus five new public child-death outcomes.
- Authentication, boundary and assembly tests: 16, six and 30 passed.
- Production contract check: retained core, STA, data/evidence/recovery/codec controls passed.
- Disabled-source checks, architecture gate, process-global check, affected-file formatting/audit, strict lint and candidate vectors passed.

The process-global check inspected 152 files with the unchanged three permanent entries and zero debt. Clippy retained nine existing warnings. Whole-node formatting debt was not requalified as clean. The unchanged 171-cut matrix and old-executable compatibility witness retain their prior pinned execution evidence; they were not newly rerun by this correction.

Strict pre-remote qualification exits nonzero as expected while genuine accepted qualification is absent. Accepted-review metadata remains null.

I performed inspection and hash verification only: no builds, tests, writes, Git mutations, network or live actions.

## 2. Invariant analysis

**Protected Begin uses durably retained authority.** The original attack exploits two individually valid consultations: the first allows the sealed start; the second reports expiry or node revocation and is retained. Previously, the earlier `allow` survived. Now Begin receives the exact cut returned by persistence and requires both its policy digest and complete conservative interval to equal the sealed precondition. A changed second cut therefore defeats Started. A first refusal followed by a permissive second cut remains conservative.

The recorded public cases establish the expected terminal result, while source inspection establishes that the new comparison precedes the Reserved-to-Started branch. Already Started attempts preserve their original terminal authority after later revocation; that immutable-start rule was not replaced with current-policy reinterpretation.

**ObservePolicy publication preserves recoverable request identity.** The selected image now records live-plus-retired invocation count, invocation high-water and corresponding floor/effect high-water values from the transition. These are selected before runtime drains the exact issued Fence. A crash cannot leave a selected request outside recovery’s accounting grammar.

The three new cuts cover Fence intent before temporary publication, intent-directory synchronization and selection-directory synchronization. Reopen finds one original TerminalNonCommit, no accepted head and no live invocation. This is physical process-death evidence at those hooks, not a fixture-only claim or a universal power-loss guarantee.

**Ingress checks the full retained interval.** The first narrow `20..21` interval and later retained `20..100` interval share a lower bound; comparing only the lower floor would miss a deadline-50 violation. Ingress now evaluates the complete returned interval and final policy after persistence. The public counterexample refuses Ownership. Stable-cut positive control still succeeds, and bounded guard reload retains its existing limit.

**Denied knowledge and protected uncertainty remain durable.** Marker establishment precedes consultation. Failed establishment prevents consultation; failed retention leaves conservative unresolved state. Successful selection can drain Fence effects, but no effect is drained before the corresponding image is durable. Historical callback binding does not consult current authority, avoiding another independent-cut authorization window.

**Oversized refusal cannot invent successful observation.** Normal M+1 settlement still refuses capacity. Only the existing host-owned pending settlement can proceed through the pending-loss route above that normal bound. Its marker uses a compact digest token while retaining the exact original observation/checkpoint and permit.

Failed publication preserves Active/pending custody and Unknown cannot be bypassed. Successful permanent loss closes and reopens incomplete. Direct caller-supplied oversized prefixes remain bounded; modifying caller observation bytes does not substitute for the retained host-owned observation. Source and public witnesses therefore preserve the original receive-provenance closure while extending loss progress.

**Compatibility and earlier physical proofs remain bounded.** Canonical/compatibility pins and serialized recovery boundaries are unchanged. Original floor, marker, receipt, Prepare and actual-old-executable proofs remain applicable where their controlling code and data are unchanged. The newly selected Fence/death evidence covers the changed accounting boundary. No comprehensive decoder equivalence, BOM-specific qualification or execution of the skipped matrix is claimed.

## 3. Risks and next action

This GO closes my originating findings on the exact frozen tuple. It does not qualify aggregate IC-3C, IC4, live migration/enrollment, existing-store seals, launcher defaults, desk, Raft or push. Original A2 originator closures remain owner-deferred to wider ABC.

The single next action is to merge this originating closure with the separately required fresh full affected reviews through the canonical acceptance process. Accepted-B metadata and strict qualification must remain closed until that process establishes genuine acceptance.