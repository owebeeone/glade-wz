# Combined IC-3B genuine authentication and physical persistence — STATE-AXIS REVIEW

**Review object:** Same combined B component, second bounded implementation correction at Glade `a47691598df648eb8c9554b27f3d06b0cffcf596`, controlled by `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`. Focused originating re-verdict of this reviewer’s two P2 findings; acceptance remains subject to the required independent review gate.

**Baseline:**

| Repository | Exact revision verified at START and END |
| --- | --- |
| Glade workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

Sources were read with `cat`, `nl`, `sed`, `rg`, scoped `git diff` and `git show`. Read-only Python checked hashes and inspected the compressed chronological evidence.

**Date:** 2026-10-04.

**Axis:** Durable-state semantics, publication ordering, restart legality, original authority and conservative recovery. Independent, adversarial, read-only. Other reviews run in parallel; nothing here relies on their testimony. Filed verbatim by the lane owner.

**Verdict: GO** — both owned P2 findings are independently closed. No additional concrete finding was established. This verdict satisfies this reviewer’s previous conditional commitment; it does not replace fresh full-component reviews or other originating closures.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| P2-1: authorization predates selected cut | B-R6: use the exact retained full cut; select and drain original Fence custody | Genuine public regressions return valid policy/time for the first prestart consultation, then chained revocation or expired time for persistence. Recorded initial execution failed both cases; final execution produces no AcceptedLocal, retains one original NonCommit, closes and reopens. Three physical Fence death cuts preserve that same attempt and terminal. | **Closed** |
| P2-2: oversized drained input cannot discharge | Remaining B-R4 edge: compact host-owned pending-loss conversion | Public M+1 receive drains without a bounded received-result digest. Normal settlement still refuses Capacity; pending loss commits and close succeeds. Reopen retains permanent loss and incomplete state. Failed publication retains the exact original continuation and Active guard; actual process death exercises refused, uncertain and selected states. | **Closed** |

The dispositions in RemPlan-2 were inputs to verification, not closure evidence by themselves. The original Started-before-revocation positive control and earlier substitution, observation-marker, Prepare-replay and critical-reserve controls remain intact.

## Changed-range analysis

The complete second-correction range is `d3fded040d6e3c459cd5f975d6c672873356cc23..a47691598df648eb8c9554b27f3d06b0cffcf596`: eleven files, 826 insertions and 23 deletions. The source manifest’s changed-file list exactly matches the scoped Git diff.

Four production files change:

- `records/authentication.rs` returns the validated cut selected by the observation barrier. It couples newly issued Fence request accounting and issuance floors to that image, then drains the selected effects.
- `records/storage.rs` compares that retained policy digest and complete interval with the sealed precondition before allowing Started.
- `records.rs` authorizes ingress using the retained complete cut, including its upper time bound.
- `records/loss.rs` converts a retained settlement into a fixed digest token while preserving the original boxed observation and checkpoint.

The other seven files add public and internal regressions, process-death witnesses, source-check coverage and module wiring. The existing Fence fixture now locates its exact request across live and retired invocation custody; its winner, late-callback and contrary-terminal assertions remain.

The five consumer-document changes describe the selected-cut rule, full interval deadline check, compact loss custody and fallible utility retirement. No public signature, utility implementation, dependency, role policy, codec, floor format, lock scope or default assembly changes in this correction.

Retained full-component context remains valid for unchanged physical publication, provenance, bindings, reserves, completeness and default construction. The new post-selection Fence drain materially changes callback order. I therefore retraced its affected call graph rather than inheriting a claim that persistence was custody-only. `ObservePolicy` produces an original Fence for a still-Reserved attempt; resolve selects NonCommit, and the existing authenticated core callback retires that request. The NonCommit callback does not initiate candidate replay or another current-authority consultation. Failed observation selection returns before effect draining.

This supports the focused closure verdict. It also supports the ledger’s requirement for fresh full Code/State review on this tuple; the focused report does not waive that requirement.

Both owned roots retain their prior **nonarchitectural B implementation** classifications. P2-1 implements the existing authoritative-start boundary, and P2-2 completes the existing reserved-loss route. The ingress interval correction is another manifestation of B-R6’s stale retained-cut decision. No new root or architectural change was established.

The committed accounting remains semantic **2 architectural/1 nonarchitectural/1 completed remediation**, typed **2 architectural/5 prior nonarchitectural/2 completed remediations**, and combined B **0 architectural/6 nonarchitectural/2 completed remediations**. This review neither resets caps nor authorizes a third remediation. The third typed architectural-root STOP remains unchanged.

## 0. Evidence base

Controlling inputs included root/member instructions and `AGENTS_GWZ.md`, the review-loop skill and canonical template, BuildEntry, library/package policies, the committed review ledger, ProductionIntegration Design/Plan, Authentication, Persistence/PersistenceUsage, TypedContract/TypedUsage, AdmissionPlan, StorageAttemptContract and ResourceConsistencyProfiles.

I read the complete canonical PromptState-2, my complete PhysicalHost-ReviewState-1 and the legitimate merged RemPlan-2. Current peer/full/originating reports were not read.

The retained full review covered the combined component from `c69e6416f5f5155d4bb570bf272e796b2deae0a3`, including genuine authentication, disk layout/slots/floors/metadata/paired validation, Records helpers, ingress utility and assembly. This re-verdict inspected the complete second-correction production/test/document diff and affected consumers, particularly:

| Source | Reviewed obligation |
| --- | --- |
| `glade/node/src/independent/records/authentication.rs:5–102` | Marker, exact stable-cut branch, selected observation, Fence accounting and post-selection drain |
| `glade/node/src/independent/records/storage.rs:164–330` | Reserved authorization, denied terminal, genuine Started and immutable terminal publication |
| `glade/node/src/independent/records.rs:264–317` | Retained full interval, fresh guard recheck, floor-before-permit ordering |
| `glade/node/src/independent/records/loss.rs:86–195` | Original continuation, compact marker, direct-input bounds and failure custody |
| `glade/node/src/independent/records/runtime.rs:99–279` | Nested callback drain, exact request replay and restart reattachment |
| `glade/contracts/crdt-admission-core/src/lifecycle.rs:245–293` and `callbacks.rs:28–241` | Original Fence issuance, authenticated callback retirement and terminal immutability |
| New `ic3_disk_remediation2`, `retained_cut_crash` and `oversized_loss` tests | Original public counterexamples, actual death/reopen and exact failed-publication custody |

Independent content checks found:

| Artifact | Result |
| --- | --- |
| Current combined source manifest | All 65 hashes match both committed Glade source and working source |
| Current consumer documents | All five hashes match the exact root commit and working documents |
| Protected compatibility/canonical files | All 45 compatibility and 27 codec hashes match |
| Protected existing artifacts | All 23 hashes match; report bytes were hashed without reading testimony |
| Prior consumer documents | All five hashes match their original `c8d778…` root commit |
| Second-correction chronology | Raw SHA256 `4d95a09b37610a7e1908e0179c5eb3dd4f27d2d6c69881a44524c0e0c1d61476`; gzip SHA256 `28c07bef73a2b83848418998b2218a8017fea5037887d575ca08890779eb1c03` |

Recorded execution was audited, not rerun:

| Recorded selection | Result |
| --- | --- |
| Four initial public counterexamples | Compiling RED: all four failed |
| Final public disk consumers | 29 passed, three child helpers ignored; 6.729 seconds wall |
| Final affected Records selection | 29 passed, six child helpers ignored; 38.248 seconds wall |
| Affected physical cuts | 57 native, 65 ingress, 26 marker/observation and three new Fence cuts |
| New public death witness | Five actual child deaths covering oversized loss and corrected denials |
| Original Fence/close/Started controls | Three passed |
| Authentication/boundary/assembly controls | 16/6/30 passed |
| Contract, architecture, disabled-source and globals gates | Successful recorded selections; allowance unchanged |
| Representation gate | Retained positives, negatives and mutants passed |
| Strict pre-remote gate | Expected refusal; no acceptance claimed |

The chronology includes intermediate compilation failures, a Pending-close regression and a Fence restart Integrity failure. The final implementation resolves the latter by retaining invocation accounting and floors with observation-issued Fence custody. The final successful selections run the affected tests; failed or zero-selection commands are not counted as qualification.

The 171 unchanged historical-kind fixed-cut witnesses were not rerun. Their prior evidence remains pinned, and all four-kind custody controls were rerun. No writes, builds, tests, network or live actions occurred during this review.

## 2. Invariant analysis

**The protected start uses the selected observation.** The original attack depended on two valid consultations disagreeing. The corrected Begin first evaluates provider-owned authorization, then requires the actually retained policy digest and complete interval to equal the sealed cut. Revocation or time advancement between reads makes `allow` false. The policy observation and its original Fence custody are selected before callbacks run. The returned earlier successful observation cannot authorize Started after a different retained cut.

The final public revocation and expiry regressions retain one NonCommit and no accepted operation. Death before Fence intent, after intent synchronization and after Fence selection reattaches the same original request without minting another attempt. Recovery invocation count, high-water and next-effect floor are consistent at the newly introduced intermediate selection.

**Historical Started remains protected.** The new predicate applies only to Reserved recovery attempts. Genuine earlier Started and terminal attempts continue through the immutable historical path. The recorded positive control kills after a legitimate protected Started, then observes revocation and still commits the original receipt. The fix therefore distinguishes denied new start from subsequent revocation of an already-protected start.

**Ingress uses the full conservative interval.** The exact same-scalar attack uses retained `[20,100]`, a transient first consultation `[20,21]` and deadline 50. Comparing only the issuance lower bound would miss it. The corrected permission check uses the returned retained interval and refuses its upper bound. No permit reaches the receive factory. A stable valid complete interval still admits, and a never-started guard can abandon and close.

**Oversized loss preserves custody and bounds.** M+1 normal settlement remains refused. The host-owned pending route hashes the immutable original observation and stores only the bounded marker; it does not install oversized bytes or trust replacement caller input. Direct caller-provided loss prefixes retain their original size check. The conversion avoids a second full byte clone and keeps the boxed original observation/checkpoint for failure recovery.

The failed-publication control retains the original permit, observation and checkpoint, a fixed 32-byte token and the Active floor guard. Unknown publication remains unavailable for live retry and leaves close Pending. Selected loss retires the permit only after durable selection; reopen retains permanent loss and reports incomplete state. After process death, an orphan Active guard does not acquire a fabricated permit.

**Earlier conservative invariants hold.** The unchanged paired-storage path still orders intent, image synchronization and selected-floor synchronization before acknowledgment. Uncertain publication poisons live custody. Observation markers precede consultation; missing observation images remain unusable. Complete live/retired request matching protects Prepare replay and callbacks. Active guards, permanent loss, missing obligations and sticky uncertainty suppress completeness. Existing substitution, critical-reserve, cancellation and independent X/Y controls remain successful in the recorded affected selections.

## 3. Risks and next action

Qualification remains bounded Unix LocalProcessRestart. The evidence establishes actual child death and reopen, not power-loss, quorum or simultaneous trusted-floor/data rollback guarantees. Unclean orphan receives remain conservative unresolved states.

Recorded formatting checks cover the eleven affected files; whole-node formatting still has inherited debt. Clippy retains nine inherited warnings. Strict remote acceptance and genuine crypto qualification metadata remain closed. Automatic exchange, wider A2 closure, activation and live/default changes remain outside this verdict.

The next action is to combine this owned closure with the separately required fresh full and originating verdicts at the identical frozen tuple. Only that complete gate can authorize accepted B metadata; this report authorizes no additional correction or downstream activation.