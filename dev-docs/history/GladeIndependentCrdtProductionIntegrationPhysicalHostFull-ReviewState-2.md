# Combined IC-3B genuine authentication and physical persistence — STATE-AXIS REVIEW

**Review object:** Fresh full acceptance review of Glade `c69e6416f5f5155d4bb570bf272e796b2deae0a3..a47691598df648eb8c9554b27f3d06b0cffcf596`, including both bounded B remediations. Controlling DRAFT: `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`.

**Baseline:** Sources read directly from the frozen checkout, with scoped diffs and historical `git show` checks. All five repository HEADs were verified at START and END:

| Repository | START | END |
|---|---|---|
| Workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` | `400cedcf1fff74128f366af758a0c389a23435d7` |

**Date:** 2026-10-04

**Axis:** Durable-state semantics, restart legality, ownership, publication ordering and fail-closed adversity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Current full, focused and originating `-2` reports, and other `-1` testimony outside the merged dispositions, were not read. Filed verbatim by the lane owner.

**Verdict: GO** — No concrete P0, P1, P2 or P3 defect established in the bounded combined B object. This is fresh full State acceptance; it does not substitute for any originating reviewer’s closure or qualify C/IC4.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
|---|---|---|---|
| None belonging to this reviewer | Fresh full review requested because the corrected helper changes callback/publication order | The merged remediation counterexamples and their source/evidence were independently attacked as acceptance coverage | Fresh State GO; no originating closure claimed |

## Changed-range analysis

I inspected the complete combined B component, the changes from original reviewed Glade `9dbc677a25d93af0c561817571ad7eb1990ddadc`, and the exact second-remediation range from `d3fded040d6e3c459cd5f975d6c672873356cc23`. The current combined manifest covers 65 source files; the second patch changes 11 files.

The private `persist_observed` return value and its selected Fence drain materially affect the reviewed call graph. Consequently, earlier source reasoning was not treated as sufficient acceptance. I retraced observation selection, original request issuance, native fencing, core callback selection, delayed Begin, reopen and close.

The corrected behavior remains within the existing authority and lifecycle:

- B-R6 corrects use of an earlier consultation instead of the complete cut actually retained. It is nonarchitectural: the existing signed policy, full interval, protected start and immutable terminal contracts already require the behavior.
- The M+1 loss correction is the remaining B-R4 edge. It uses the existing host-owned continuation and permanent-loss grammar; it creates no new reconstruction or reset authority.
- B-R7 documents existing utility retirement operations. It does not add an implementation lifecycle.

No additional root was established. Accounting remains semantic **2 architectural / 1 nonarchitectural / 1 completed remediation**; typed **2 architectural / 5 prior nonarchitectural / 2 completed remediations**; B **0 architectural / 6 nonarchitectural / 2 completed remediations**. This verdict neither resets those counts nor authorizes a third correction. The third typed architectural-root STOP and bounded-remediation rules remain applicable.

## 0. Evidence base

All activity was inspection. I ran no builds, tests, executable witness, network operation or live action, and made no filesystem or Git mutation.

I read the canonical State prompt, root/member rules, `AGENTS_GWZ.md`, review-loop skill and canonical template; BuildEntry, library/package policy; the production design, plan, typed contract/usage, authentication, persistence/usage and review ledger; AdmissionPlan, StorageAttemptContract, ResourceConsistencyProfiles; and the linked accepted admission, storage-attempt and Pure implementation authorities. The two merged remediation plans were legitimate inputs.

The relevant source inspection included:

| Boundary | Principal locations inspected |
|---|---|
| Selected authority and protected admission | `records/authentication.rs:5–103`, `records/storage.rs:134–331`, `records.rs:264–345`; provider/custody/operations and observation handling |
| Effect publication and recovery | `records/runtime.rs:5–280`, core lifecycle and `callbacks.rs:6–244`; complete bindings, validation and evidence recovery |
| Physical ownership and selection | `disk.rs`, `disk/paired.rs:25–393`, floors, slots, metadata and paired validation |
| Owned ingress and loss | `records/persist.rs`, `records/loss.rs:79–195`, guards, reserves and recovery utilities |
| Complete component context | Genesis, namespace/profile validation, authentication configuration, factory/assembly and all four native-kind installation paths |

I inspected the actual public regression, cancellation, restart and child-process test implementations, including `ic3_disk_remediation2/mod.rs`, `retained_cut_crash.rs`, `oversized_loss.rs`, native/ingress/marker cut matrices, Fence controls, compatibility and structural corruption controls. Internal custody assertions were distinguished from public physical witnesses.

B1/B2 and both remediation evidence packets, manifests and compressed chronological logs were inspected. Read-only hash checks found no mismatch among current 65 combined source hashes, five current consumer hashes, compatibility 45 and canonical 27 pins. Historical B2’s 56 source hashes and Remediation1’s 62 were checked against their respective committed sources; the original five consumer hashes matched their historical commit.

The retained legacy executable’s local artifact hash matched `5da9c9da8eb48da7f1ac484674b0f2238b59735714960c3109eccf4e1b42b4d4`. Its recorded refusal witness uses an actual separately built old executable against the fresh marker. I did not execute it.

The following are **recorded results**, not reviewer executions:

| Evidence | Audited result and scope |
|---|---|
| Required second-remediation regressions | Four compiling behavioral RED failures before implementation: revoked/expired write start, receive interval drift, M+1 pending loss |
| Intermediate publication failures | Pending close after selected Fence and Integrity reopen at new Fence deaths were preserved in chronology and drove further corrections |
| Final public disk selection | 29 tests passing, with owned helpers and five new actual SIGKILL outcomes |
| Final Records selection | 29 tests passing; 151 selected cuts: 57 native, 65 ingress, 26 observation, three new Fence cuts |
| Earlier other-kind physical cuts | 171 candidate/security/fork cuts retained from pinned earlier evidence; explicitly not rerun in remediation 2 |
| Authentication/boundary/assembly | Recorded 16/6/30 passing |
| Contract selection | Recorded core 106, storage-attempt API 34 and affected representation/evidence/recovery consumers passing |
| Structural gates | Expanded disabled-source checks, adopted node architecture and unchanged process-global allowances passing |
| Representation/strict gate | Three-language representation controls pass; strict pre-remote intentionally refuses absent genuine B acceptance |
| Formatting/lint | Affected-file formatting passes; inherited whole-node formatting debt and nine existing Clippy warnings remain disclosed |

The Remediation2 compressed log hash matched `28c07bef73a2b83848418998b2218a8017fea5037887d575ca08890779eb1c03`; decompressed bytes matched `4d95a09b37610a7e1908e0179c5eb3dd4f27d2d6c69881a44524c0e0c1d61476`. Compiler, selector and fixture failures were not counted as behavioral RED or GREEN.

## 2. Invariant analysis

**Protected Begin uses the exact retained authorization cut.** I attacked a valid earlier consultation followed by genuinely revoked placement or expired time during persistence. `storage::begin` checks its initial observation, then compares the returned retained policy digest and complete interval with the sealed precondition before selecting Started. A failed comparison produces the original durable NonCommit path. Matching scalar time floors cannot conceal interval growth.

Ingress likewise applies holder permission, revocation, policy windows and the deadline to the returned retained cut. The `[20,21] → [20,100]` deadline-50 case refuses despite equal lower floors; the stable valid interval control succeeds. Critical reserve refusal precedes consultation. A consultation begins with a durable observation marker, so failed retention cannot reopen the older usable authority.

**Selected Fence issuance precedes callback publication.** `persist_observed` couples the ObservePolicy transition’s request accounting, invocation high-water and authoritative issuance floors into the Observation image before selection. Only after successful selection does `runtime::drain` execute the original Fence. Native resolution selects the original terminal; the exact core callback is then selected.

This ordering defeats the previously exposed crash window in which a selected request lacked recovery issuance evidence. The three actual deaths cover pre-intent, synced intent and synced selection. Reopen authenticates retained original requests and resolves one NonCommit, without accepted state, lingering storage invocation or fabricated absence.

I also followed partial-publication retries: durable native terminal with absent core callback is recoverable through the retained binding; callback installation is once-only. Live and retired issuance authenticate complete requests. The runtime’s finite effect limit remains, and the corrected NonCommit callback path does not introduce another current-policy consultation.

**A Fence cannot rewrite valid Started or terminal history.** The fresh authorization branch applies to physical Reserved. Existing Started resolves its immutable retained start and original result. Terminal observations recover the same receipt/result. Exact Prepare replay authenticates its original live or retired issuance before finding the existing binding, avoiding revision-based rejection of an authentic retry.

The Fence-winner fixture’s live-or-retired lookup preserves its original delayed Begin, contrary callback, integrity and no-accepted-state assertions. Core callback handling distinguishes duplicate retirement from contradictory terminals and retains the integrity stop rather than choosing arrival order. All four native kinds preserve complete original custody and genuine receipt semantics.

**Physical recovery never promotes uncertainty to success.** Paired publication marks uncertainty before intent I/O and uses protected floors, fixed preallocated slots and final selection. The selected image, application custody and terminal outcome are coupled. Failure cannot yield a memory-only NonCommit.

Open validates ownership, the closed floor/image grammar and full next-image authentication before completing an interrupted selection. A missing or invalid protected Observation stays unavailable. Other incomplete transitions preserve maxima, guards and explicit incompleteness; they do not imply absent attempts or complete history. The retained corruption controls check structural as well as cryptographic invalidity before recovery selection.

Root locks remain held through session ownership. Path, directory/lock identity and marker checks refuse copied, substituted or incompatible roots. Version-one and version-two floor grammars reject cross-opening. The fresh marker and old-executable refusal are within scope; existing-store migration is not claimed.

**Owned receive completion is distinct from normal-input authentication.** The private issuer witness records only successful bounded completion. Malformed, failed or different captured bytes cannot be replaced by a caller’s signed-empty observation. Dropping a future does not prove drain. Normal settlement checks exact received bytes before publication; cancellation retains authority until actual join/drain.

For M+1 completion, normal settlement returns Capacity and retains the original continuation. `pending` loss conversion requires the genuine host-owned drained continuation. It derives a fixed-size digest token while preserving the original permit, observation and checkpoint through failure. It does not install oversized input or accept an arbitrary caller prefix.

Loss and guard retirement select together. Utility retirement occurs only after Committed. Refused or Unknown conversion reinserts original pending custody and leaves the guard Active and close Pending. Actual death/reopen controls preserve either that conservative unresolved state or the selected permanent loss; neither route invents a complete cut or admitted inbox record.

**Isolation and completeness remain conservative.** Pending X prevents premature close and replacement authority while independent Y can progress. Checkpoint validation protects kernel, origin, session and loss custody. Capacity restoration, partial repair or restart cannot clear sticky incompleteness without the defined witness. No terminal/evidence GC or origin reminting was introduced.

Public roles, dependencies, defaults and process-global allowances remain unchanged. The disabled-source scanner includes the new subtree and retains previous negative fixtures. Cold guides now identify eligibility queries and actual retirement names, selected-Committed ordering and failure custody, without promising reconstruction of orphan permits.

## 3. Risks and next action

This GO is bounded to genuine authentication and local-process-restart physical persistence. Recorded process termination/reopen evidence does not establish OS-crash, power-loss or machine-loss durability. Finite retained history can exhaust capacity. An orphan Active receive guard can remain conservatively unresolved; no generic reset or orphan reconstruction is promised. Trusted-floor loss or simultaneous rollback is not repaired by assigning a fresh identity.

Historical physical evidence remains pinned where unchanged rather than universally rerun. Inherited formatting and Clippy debt is disclosed. No automatic duplex-node exchange, browser activation, live enrollment, existing-store transition or comprehensive BOM decoder-equivalence claim follows.

The single next action is for the lane owner to reconcile this fresh State GO with fresh full Code, all four originating counterexample closures and required Surface review at this identical tuple before recording B acceptance. Implementation GREEN and this fresh review cannot replace those separate closures; the owner-deferred A2 review and wider ABC gates remain intact.