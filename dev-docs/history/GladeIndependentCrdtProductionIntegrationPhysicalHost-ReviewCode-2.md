# Combined IC-3B genuine authentication and physical persistence/recovery — CODE-AXIS REVIEW

**Review object:** The same combined IC-3B component after its second bounded correction, at Glade `a47691598df648eb8c9554b27f3d06b0cffcf596`, controlled by the DRAFT [production integration design](/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md) at root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`.

**Baseline:**

| Repository | Exact revision |
|---|---|
| Workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

All five START and END revisions matched. Working sources and committed `git show` bytes independently matched all 65 source-manifest hashes.

**Date:** 2026-10-04  
**Axis:** Architecture, interfaces, call graphs and compatibility, with retained full-component context and independent verification of this reviewer’s two previous findings. Adversarial and read-only. Nothing here relies on current peer, full-review or originating closure testimony. Filed verbatim by the lane owner.

**Verdict: GO** — both own P2 findings are independently closed; no new finding was established. This is the required originating re-verdict. It does not replace the additional fresh full Code/State acceptance required by the changed callback order.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
|---|---|---|---|
| P2-1 — Started used a decision older than the subsequently retained cut | B-R6: authorize against the complete cut actually selected; resolve invalidated Reserved attempts through original Fence custody | Public revocation and expiry advance at the second pre-start consultation. Both compiled and failed before correction. Corrected tests require no AcceptedLocal, exactly one original NonCommit, no accepted head, clean close and exact reopen. Source now compares retained policy and full interval before Started. Three intermediate Fence deaths exercise original issuance recovery. | **Closed at this tuple** |
| P2-2 — oversized captured settlement could not discharge through public pending loss | B-R4: bounded conservative loss from the existing drained continuation, preserving original custody on failure | Exactly M+1 bytes are returned by the owned future; normal settlement refuses Capacity. The public pending-loss route now commits compact loss, closes and reopens permanently incomplete. Failed-publication inspection retains the original permit, Observation and Checkpoint; actual death controls preserve Active guard or selected loss. | **Closed at this tuple** |

Closure rests on the inspected implementation and recorded executable counterexamples, not the writer’s disposition claims. Separate originators remain responsible for their own findings.

## Changed-range analysis

The entire correction range `d3fded040d6e3c459cd5f975d6c672873356cc23..a47691598df648eb8c9554b27f3d06b0cffcf596` contains 11 files, 826 insertions and 23 deletions. I inspected the complete production/test diff, new test files, scoped consumer-document changes and every affected observation-helper consumer.

Production changes are confined to the private Records observation helper, Begin decision, ingress decision and pending-loss conversion. There is no changed public signature, package role, dependency, trusted authority, disk grammar, compatibility marker, default or physical publication primitive.

One behavioral boundary changes: selected ObservePolicy Fence effects now execute before the helper returns. Consequently, unchanged full-component proofs alone cannot qualify every callback ordering. The merged plan correctly requires fresh full affected Code/State review in addition to originating closures. I independently traced this path for my findings, including its durable request accounting, native terminal, exact core callback and reattachment.

The Fence fixture now finds the same original request in live **or retired** custody. Its immutable terminal, late Begin, contrary callback, integrity diagnostic and empty accepted-state assertions remain intact.

Both corrections remain nonarchitectural implementations of existing obligations: B-R6 for exact retained authorization and the remaining B-R4 oversized-loss edge. No additional root or third typed architectural root is established. Accounting remains semantic **2 architectural / 1 nonarchitectural / 1 completed remediation**, typed **2 architectural / 5 prior nonarchitectural / 2 completed remediations**, and B **0 architectural / 6 nonarchitectural / 2 completed remediations**. Completion and closure remain distinct; no cap is reset.

## 0. Evidence base

No files were modified. No builds, tests, Git mutations, network requests or live actions were performed.

| Evidence inspected | Coverage and result |
|---|---|
| Canonical Code-2 prompt, own Code-1 report, legitimate RemPlan-2 and committed review ledger | Exact original counterexamples, correction scope, accounting, fresh-full-review requirement and retained deferrals |
| Controlling contracts and retained full-component context; five current consumer-document diffs | Authoritative pre-start cut, complete conservative interval, guarded receive, compact loss, retirement and close obligations remain controlling |
| [authentication.rs](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/authentication.rs), complete 103 lines | Selected observation return, equality-confirming marker clear, complete Fence issuance accounting, publication-before-callback order |
| [storage.rs:134](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/storage.rs:134), through line 331 | Exact live/retired Resolve authority, final retained-cut comparison, NonCommit, immutable Started and terminal retrieval |
| [records.rs:264](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records.rs:264), through line 346; runtime and recovery-query consumers | Ingress uses retained full interval; submit, Verify/Seal, challenge/response and reattachment preserve protected observation handling |
| [loss.rs:86](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/loss.rs:86) and unchanged persistence/reserve consumers | Compact digest token, retained boxed continuation, direct-prefix bound, selection-before-retirement and Unknown custody |
| [Public remediation regressions](/Volumes/projects/limbo/glade-wz/glade/node/tests/ic3_disk_remediation2/mod.rs), complete 508 lines | Original revocation, expiry and M+1 counterexamples; interval and stable controls; Verify/Seal advancement; five actual process-death outcomes |
| [Oversized-loss custody test](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/tests/oversized_loss.rs) and [retained-cut death tests](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/tests/retained_cut_crash.rs), complete files | Exact failed-publication custody; deaths before Fence intent, after intent sync and after selection sync; original NonCommit replay without reminting |
| Remediation2 evidence, source manifest and decompressed chronological log | Commands, compilation, intended behavioral RED, intermediate failures, final GREEN, selectors, timings and explicitly omitted reruns |

Read-only inspection used `git rev-parse`, `git show`, scoped `git diff`, `rg`, `cat`, `nl`, and Python hashing/decompression. All 65 working and committed source hashes, five current consumer hashes, 45 protected compatibility hashes, 27 canonical codec hashes and 23 immutable artifact hashes matched.

The new log is 71,711 decompressed bytes. Its SHA-256 is `4d95a09b37610a7e1908e0179c5eb3dd4f27d2d6c69881a44524c0e0c1d61476`; compressed SHA-256 is `28c07bef73a2b83848418998b2218a8017fea5037887d575ca08890779eb1c03`.

The chronology contains compiling RED for all four initial public regressions. Intermediate fixes still failed clean close because Fence callbacks remained outstanding, and later failed reopen with Integrity because newly issued request accounting was incomplete. Those failures remain recorded. Final evidence reports:

| Recorded selection | Result |
|---|---|
| Public disk consumer | 29 passed, three helper entries ignored; 6.729 seconds wall |
| Records selection | 29 passed, six helper entries ignored; 38.248 seconds wall |
| Selected physical cuts | 57 native, 65 ingress, 26 observation and three new Fence cuts |
| Auth / boundary / assembly | 16 / 6 / 30 passed |
| Contract production check | Exit 0; retained core, StorageAttempt, data, evidence, recovery and codec consumers |
| Architecture / disabled-source / process globals | Passed; four source controls include disabled branches; unchanged three permanent global allowances |
| Affected formatting / Clippy | Affected files passed; Clippy exit 0 with the same nine existing warnings |
| Strict pre-remote | Expected exit 1; accepted genuine qualification remains absent |

The unchanged 171 candidate/security/fork native-kind cuts were **not rerun**. Their earlier pinned evidence remains applicable to unchanged physical grammar and fixed-cut paths; all-four-kind custody controls were rerun. No fresh execution by this reviewer is claimed.

## 2. Invariant analysis

**Exact retained start decision.** `persist_observed` returns the validated signed cut only after successful observation selection or equality-confirming marker clearance. Begin additionally compares that returned policy digest and entire interval to the sealed precondition. The original second-consultation revocation and expiry sequences therefore cannot select Started. If persistence fails, its error exits before start selection.

**Original Fence lifecycle.** ObservePolicy’s complete issued request, invocation count/high-water and effect floor are coupled into the Observation image before callbacks run. The helper then uses the existing native resolver and core callback path. The NonCommit callback performs no fresh source consultation or reminted verification. Its three new process-death controls recover the original attempt as exactly one NonCommit, without accepted receipts or live storage invocations, and close cleanly.

**Earlier Started remains protected.** Begin’s current checks remain inside its Reserved branch. Existing death/reopen evidence still commits the original receipt when revocation occurs after durable Started. Late exact and contrary callback controls remain preserved.

**Complete ingress interval.** The consume-permit decision uses the final retained cut, including its upper time bound. The `[20,100] → [20,21] → [20,100]`, deadline-50 counterexample refuses without a permit. Stable `[20,21]` remains usable. Changed-image retries, receive-only permission and denied knowledge retention remain covered by the unchanged public controls.

**Oversized conservative loss.** Pending conversion keeps the original boxed settlement and derives a fixed digest token rather than cloning all oversized bytes. The full-input bound remains for direct caller prefixes. The private host-owned continuation can select the existing compact loss representation without installing oversized inventory. Normal settlement’s exact bounded-success witness is unchanged.

**Failure ownership.** Failed loss publication retains the exact original permit, observation and checkpoint and leaves the floor guard Active. Unknown remains unavailable for retry and Pending for close. Successful loss alone retires authority. Restart cannot manufacture a drained capability or erase permanent incompleteness.

**Preserved boundaries.** Root/lock identity, fixed physical allocation, paired floor ordering, validated old/next selection, historical binding validation, exact retired Prepare replay, genuine signatures and canonical wire bytes remain unchanged. Compatibility and protected artifacts match their pinned hashes.

No new actionable finding emerged from these attacks.

## 3. Risks and next action

This GO closes this reviewer’s two findings at the exact frozen tuple. The private Fence callback-order change still requires the separately mandated fresh full Code/State acceptance and remaining originator closures before a committed B acceptance record.

Qualification remains LocalProcessRestart on the stated Unix filesystem profile. It supplies no power-loss, quorum, arbitrary simultaneous rollback or IC-3C duplex qualification. Existing formatting/Clippy debt and owner-deferred A2 closure remain explicitly separate.

The next action is to complete the required independent verdict set at this tuple, then record acceptance only if every blocking obligation is closed. Strict pre-remote success, automatic exchange and activation remain unavailable until their controlling prerequisites are satisfied.