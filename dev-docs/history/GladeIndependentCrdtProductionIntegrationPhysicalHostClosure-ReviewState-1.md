# Combined IC-3B physical host, originating closure 1 — STATE-AXIS REVIEW

**Review object:** The same combined IC-3B1/B2 component, corrected at Glade `d3fded040d6e3c459cd5f975d6c672873356cc23`, with controlling documents at workspace root `c8d778e7c24b68690e27bfac8ca4457f647e747f`. First completed B remediation and second completed typed remediation; candidate acceptance remains pending.

**Baseline:** All five prescribed heads matched at START and END. Inspection used scoped source reads, committed diffs and read-only hash audits.

| Repository | START and END HEAD |
| --- | --- |
| Workspace root | `c8d778e7c24b68690e27bfac8ca4457f647e747f` |
| Glade | `d3fded040d6e3c459cd5f975d6c672873356cc23` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

**Date:** 2026-10-04  
**Axis:** Originating State closure of three prior counterexamples, including changed-range adversity and root classification. Independent, adversarial, read-only. Other required reviews run separately; nothing here relies on current peer testimony. Filed verbatim by the lane owner.

**Verdict: NO-GO** — all three original State findings close, but one newly identified P2 blocks qualification. I pre-commit to GO on a revision resolving P2-4 as specified, subject to exact-tuple verification and the required executable regression. This originating closure does not replace fresh full acceptance.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample independently verified | Status |
| --- | --- | --- | --- |
| State P2-1: settlement detached from receive result | B-R1 adds private issuer-owned successful bounded length/digest and normal-settlement comparison | `GuardedReceive` records only bounded successful completion. `persist::settle` requires `can_settle_received` before observation publication. Genuine nonempty→different signed empty, malformed prefix→signed empty and receive error→signed empty now refuse. External regressions and five-mode actual child-death witness preserve Active/incomplete custody. Exact empty and nonempty controls remain successful. | **CLOSED** |
| State P2-2: refused calls forget learned floors | B-R2 establishes a durable protected marker before consultation, persists valid learned cuts despite denial, and refuses unresolved recovery | Challenge/response no longer return before persistence on denial. Ingress advances its own observation, then revalidates guard bindings. Failed marker establishment prevents consultation; failed post-consultation retention blocks usable load/recover/reconstruction and both reopen forms. External denial, chained revocation and 26-cut observation witnesses establish retention or conservative refusal. | **CLOSED** |
| State P2-3: refused settlement hides its only loss capability | B-R4 adds public `retain_pending_ingress_loss` | The public operation retrieves the host’s existing drained permit and preserves original observation/checkpoint through failure. Malformed and capacity-refused settlement can select loss, close and reopen incomplete. Unknown publication retains ownership and cannot be bypassed. Closure witnesses use public APIs, not private-map extraction. | **CLOSED** |

These closures apply to the original root causes and counterexamples. P2-4 below concerns a distinct ordering failure between two independently obtained current cuts; it does not reopen the original omitted-retention finding.

## Changed-range analysis

I inspected the complete 24-file Glade correction range `9dbc677a25d93af0c561817571ad7eb1990ddadc..d3fded040d6e3c459cd5f975d6c672873356cc23`, including production changes, added public regressions, utility/source controls, observation crash tests and the source-qualified prior decoder fixture. The changed-file list matches the remediation manifest exactly.

The correction modifies three shared mechanisms:

1. Normal settlement now binds retained observation bytes to the issuer’s completed receive. Conservative loss retains drain eligibility separately.
2. The private floor becomes closed `Ic3DiskFloor/v2`, with a pre-observation marker carried through publication. Missing next Observation images now leave recovery unavailable rather than allowing old-image recovery to discharge unknown authority.
3. Host-owned refused continuations gain a public conservative loss route, while exact Prepare replay authenticates live or retired issuance.

I also inspected the Prepare correction and its all-kind/kill-matrix assertions because it shares native lifecycle context. I do not claim originating closure of another reviewer’s IDs.

The five revised consumer documents explain the changed capability, marker, bounded receive-only reload/retry and loss lifecycle. The two explicitly superseded availability expectations are identified rather than hidden: missing-next learned Observation recovery and oversized learned policy now refuse usable authority. Their selected-image, advanced-floor, critical-space and no-fabricated-receipt proofs remain.

Context retention is valid for the unchanged component: the original review inspected the full combined implementation, and the corrected manifest authenticates all 62 combined source files against current committed bytes. I retraced the changed mechanisms through unchanged authentication, core policy observation, native terminal installation, validation and public consumers. Shared changes receive this independent analysis; earlier conclusions are not treated as blanket acceptance.

| Controlling object | Accounting after this report |
| --- | --- |
| Semantic | Two architectural/one nonarchitectural roots; one completed remediation, unchanged |
| Typed | Two architectural/five prior nonarchitectural roots; two completed remediations, unchanged |
| Combined B | Zero architectural/four previously merged nonarchitectural roots; one completed remediation; P2-4 adds one distinct nonarchitectural implementation root |

No third typed architectural root is classified. Original A2 R1–R6 closures remain owner-deferred to wider ABC. Neither naming this B nor closing these State findings resets any cap.

## 0. Evidence base

I read the complete closure prompt, my original report, the legitimate merged remediation plan, corrected evidence/pins and relevant review-ledger entries. Root/member instructions, review-loop authority and controlling design/contracts from the initial review remain applicable. Revised authentication, persistence, typed contract/usage and persistence usage were inspected against their original versions. No current full-review, peer-closure or Surface testimony was read.

Principal source inspection included:

| Area | Source |
| --- | --- |
| Completion provenance | `contracts/crdt-recovery-api/src/ingress.rs`, especially private Status and `can_settle_received`; complete added public utility controls |
| Denial/floor coupling | `node/src/independent/records.rs`; complete `records/authentication.rs`; runtime/evidence-recovery consultation paths |
| Physical marker recovery | Complete `disk/metadata.rs` change, `disk/paired.rs`, paired validation and compatibility controls |
| Retained loss | Complete `records/loss.rs` and settlement/retry changes; external remediation `mod.rs`, `loss.rs`, `observations.rs` |
| Observation death/reopen | Complete `records/tests/marker_cuts.rs` and corrected `observation_restart.rs` |
| Start-order counterexample | `records/storage.rs:164–224,269–320`; `auth/provider.rs:159–181`; `auth/operations.rs:90–114`; core `lifecycle.rs:245–293` and terminal callback handling |
| Preserved consumers | Native-kind, original fence/restart and public disk controls, including added retired-Prepare assertions |

Read-only audits found no mismatches among 62 combined source files, five consumer documents, 45 retained compatibility files, 27 canonical source pins and nine immutable initial B artifacts. The correction’s exact 24-file range matches its manifest.

The correction chronology contains 581,222 raw bytes, SHA256 `b276e0f2349a9536b2eae265b7b0bb4ac91d5433715ab51fe705135232b98f5b`; gzip is 98,508 bytes, SHA256 `fdb8fa4f9631e8299f0846cf72c72f7e128c2d2046b895f54c79f69d7e9c3862`. Both matched. I inspected chronological results, meaningful RED failures and relevant final outputs.

| Recorded selection | Result inspected |
| --- | --- |
| External substitution regressions | Initial three intended assertion failures; corrected success with exact nonempty control |
| Public retained-loss seam | Absent-method compilation distinguished from compiling refusing-seam RED; corrected successful loss/close/reopen |
| Denial/receive-only regressions | Intended restart/convergence failures; corrected public controls and final chained-policy/time witness |
| Public disk/auth/boundary | 22 passing disk controls plus two owned child helpers; authentication 16 and boundary 6 pass |
| Actual public process deaths | Five substitution/failed-loss/selected-loss modes and chained-revocation witness pass |
| Records physical selection | 28 pass, five owned helpers; 121.675 seconds wall, 118.14 seconds test execution |
| Physical disk selection | 16 pass, three explicit prerequisites; corrected 6.408-second wall run |
| Retained crash matrices | 228 native, 65 ingress and 26 added observation cuts |
| Recovery utility/source | One unit, nine public contract and four source controls pass; strict recovery lint passes |
| Architecture/globals | Adopted gates and negative fixtures pass; 150 production files, unchanged three permanent allowances |
| Other limits | Affected formatting passes; whole-node formatting remains 251 historical hunks; node Clippy retains nine old warnings; strict pre-remote correctly refuses |

Historical “GREEN” labels with nonzero exits remain failures. Zero-selected legacy execution is not qualification; the corrected exact selection runs one actual refusal witness. I performed no builds, tests, writes, network or live actions. P2-4 is a source-derived counterexample, not a newly executed test.

## 1. Findings

### [P2-4] Native Begin uses an earlier start decision after retaining a newer current cut

**Location.** `glade/node/src/independent/records/storage.rs:188–224` computes `allow` from `EvidencePort::observe`, then calls `authentication::persist`. That helper independently calls `evidence.current()` at `records/authentication.rs:12`, potentially retaining a different cut. Begin does not compare that newly retained cut before changing Reserved to Started. `storage.rs:269–320` then publishes the original terminal.

**Violated invariant.** IC3-AUTH-004 requires final trusted start to match the sealed cut immediately before durable Started. Changes learned before this barrier apply to Reserved work. Valid Started may survive later revocation; a cut learned before Started cannot be treated as later.

**Credible sequence.**

1. Prepare a genuine local AcceptedBatch with immutable prestart policy and time `[20,21]`, and permit window `[10,100]`. Keep its exact issued Begin request; its physical attempt is Reserved.
2. Configure a trusted injected source to return `[20,21]` for Begin’s `observe`, followed by valid monotonic `[1001,1002]` for persistence. The policy can remain unchanged.
3. Begin’s first observation matches the sealed cut and window, so `allow=true`.
4. `authentication::persist` obtains the second interval, retains floor 1001 and selects a state whose current policy time is `[1001,1002]`. The core’s ObservePolicy detects the Reserved cut mismatch and issues a Fence effect; persistence retains only `.state`.
5. Begin continues with the earlier `allow=true`, selects `Started { cut: [20,21] }`, and `terminal` commits the batch with that original cut and receipt.

There is no competing thread, forged callback or clock regression. The injected source is expressly allowed to provide new observations on successive calls. The newer time is known and durably retained before Started. A valid chained revocation on the second consultation provides the corresponding policy variant.

Terminal validation checks the original binding and historical qualification. It does not reject this ordering failure. Once committed, subsequent fencing cannot undo the immutable terminal winner.

**Impact.** A Reserved local admission can start and commit after the host has already learned and retained a cut that invalidates its sealed prestart conditions. The observation marker prevents lost knowledge, but does not make the earlier decision current.

**Required correction.** Bind final eligibility and retained observation to one exact trusted cut under the instance owner. Either persist the exact independently observed cut without obtaining another decision-changing observation, or re-evaluate the complete prestart predicate against the final retained cut before Started. Any intervening mismatch must refuse/fence while preserving the original request and custody. Preserve the contrasting rule that revocation learned after an already valid durable Started cannot retroactively cancel it.

**Closure regression.** Use a genuine configured physical host and deterministic injected source: exact sealed cut on Begin’s first consultation, then advanced time outside the permit window or a valid chained revocation on the subsequent persistence consultation. Assert no Started/Committed success, no admitted head or receipt, and legal original-request recovery across actual kill/reopen. Retain stable-cut success and the existing valid-Started-then-revoked witness.

**Root classification.** One new nonarchitectural B implementation root: stale prestart decision across a second source consultation. This differs from omitted durable observation, missing receive provenance and retained-loss accessibility. Existing semantic/typed contracts already require the correct ordering; no new contract authority or architectural boundary is needed.

## 2. Invariant analysis

The original receive attack now fails because the completed result’s successful bounded length/digest belongs to private issuer state. Caller-authenticated substitute bytes cannot change that witness. Error completion remains eligible for conservative loss, never normal settlement.

The retained-loss attack now fails through public lifecycle operations. The host retrieves its original drained continuation, checks ownership and physical availability, and retires it only after selected loss. Failed loss keeps the permit and original settlement custody. Restart does not reconstruct an orphan permit.

The original forgotten-floor attack also fails. A bounded floor marker precedes consultation; establishment failure consults nothing. Unresolved authority blocks usable reads and both reopen forms. Complete authenticated next Observation recovery can clear the marker after validation/resync; missing-next fallback cannot. Unrelated checkpoint/floor publication preserves the marker, and the prior closed decoder refuses v2.

These are substantive closures. They do not prove protected start ordering: the newly retained second cut and earlier `allow` value remain distinct, which is why P2-4 survives the otherwise successful floor correction.

## 3. Risks and next action

The named unavailable marker recovery stop is intentional, not automatic repair. Orphan Active guards remain conservative after unclean death. Qualification remains bounded Unix LocalProcessRestart, without power-loss, quorum or simultaneous trusted-floor/data rollback guarantees. C, activation, existing-store migration and excluded BOM work remain outside scope.

The next action is a scoped test-first correction for P2-4, preserving all three closed State roots and cumulative accounting. Re-freeze the tuple and obtain required independent verification. Fresh full acceptance remains separate, and the strict pre-remote prerequisite must remain closed until actual accepted B qualification is committed.