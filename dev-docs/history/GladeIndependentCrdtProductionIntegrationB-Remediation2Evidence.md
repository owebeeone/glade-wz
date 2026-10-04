# Combined IC-3B — merged remediation 2 evidence

Status: **source-frozen corrected candidate; independent full and originating verdicts pending**. GREEN implementation tests are not finding closure. Parent owns Git/GWZ settlement and acceptance. Initial and Remediation1 testimony, evidence, manifests and raw/compressed logs remain immutable. Original typed closures remain owner-deferred to wider ABC. C remains behind the genuine committed B prerequisite.

## Source and provenance

[SourcePins](GladeIndependentCrdtProductionIntegrationB-Remediation2SourcePins.json) covers the entire combined `c69e6416f5f5155d4bb570bf272e796b2deae0a3..a47691598df648eb8c9554b27f3d06b0cffcf596` range and a separate exact remediation range from `d3fded040d6e3c459cd5f975d6c672873356cc23`. Parent committed the exact 11-file patch at Glade `a47691598df648eb8c9554b27f3d06b0cffcf596` and verified all 65 committed source hashes. This is a corrected candidate, not independent acceptance. Later metadata/root-document commits may differ while implementation bytes stay fixed. `accepted_review_record` and canonical crypto qualification remain null.

Five current consumer hashes are separate. The original five Remediation1 consumer hashes are verified against their existing committed root `c8d778e7c24b68690e27bfac8ca4457f647e747f`; active documents have scoped new additions and do not falsely claim their old hashes. Prior manifests are not rewritten. Current accepted compatibility45, canonical27 and23 existing production-integration evidence/manifests/logs/review reports remain byte-exact. Cargo manifests/locks, external versions, roles/edges and process-global allowances remain unchanged.

## Findings and executed counterexamples

| Root / requirements | Compiling behavioral RED | Corrected witness |
| --- | --- | --- |
| B-R6 write start; IC3-AUTH-003/004, DISK-004/005/006 | Genuine public local submit consults valid `[20,21]` through Begin observe, then consultation7 learns a genuinely chained node revocation or expiry `[1001,1002]` during persistence. Both initially acknowledged AcceptedLocal under the newer invalidating cut. | Both now return no AcceptedLocal, retain exactly one original immutable NonCommit, no accepted head/receipt, clean close and exact reopen. No caller Begin.current trust or reminting. Genuine earlier protected Started-after-revocation positive remains GREEN. |
| B-R6 receive interval; IC3-AUTH-003, GUARD-001 | Public fresh genesis retains `[20,100]`; ingress consults `[20,21]`, persistence `[20,100]`, deadline50. Both cuts have scalar floor20 and same signed policy. Initial candidate issued a consumable permit. | Exact retained full interval straddles50 and refuses Ownership, with no permit/factory authority. Stable `[20,21]` deadline50 positive and existing changed-image bounded reload/retry control remain GREEN. |
| B-R4 remaining edge; IC3-GUARD-002/003, CUT-003 | Owned receive returns exactly M+1 bytes, with no bounded success witness. Normal settlement correctly returns Capacity, but pending loss also returned Capacity despite healthy reserved critical space. | Public normal Capacity → host-owned compact permanent loss Committed → Closed → reopen loss retained/complete_local=false. No oversized inbox admission or caller substitute. Direct loss-prefix and normal settlement bounds remain. Failed publication keeps original permit, Observation and Checkpoint, Active guard and Pending close. |
| B-R7; cold utility lifecycle | Surface P3 documented eligibility but omitted actual retirement names. No implementation defect or fabricated RED claimed. | Five cold guides identify existing `IngressAuthority::settle(&permit)` / `abandon(&permit) -> Result<(),Fault>`, query versus mutation, only selected Committed retirement, Refused/Unknown custody, unexpected retirement error and concrete-session ownership. Independent Surface re-verdict pending. |

The initial four required regressions compiled and failed at their intended assertions before implementation. Earlier Verify/Seal advancement controls exercise thresholds3/5 with both revocation and expiry and prove no stale query reaches Prepare/AcceptedLocal. Normal nonempty/empty, malformed/Err substitution negatives, receive-only empty-grant policy/time growth, retired Prepare replay, current denial retention and marker failures all remain in the final public suite.

## Exact private call-path and publication impact

`authentication::persist_observed` returns the complete validated signed policy/time observation actually selected at the protected barrier. Existing custody-only callers retain the private `persist -> Result<()>` wrapper. Begin keeps the initial provider observation check but additionally compares the final retained policy digest and full interval against the sealed precondition before Started. Ingress checks holder permission, policy window and deadline against the returned final retained cut, not the earlier interval or scalar lower floor.

A selected `ObservePolicy` may issue an original Fence for a Reserved attempt. The first patch correctly fenced native outcome but left that selected callback outstanding, causing the unchanged close assertion to remain Pending. The helper now couples the new request's Recovery invocation_count/high-water and authoritative invocation/effect issuance floors into the Observation image **before selection**, then executes:

`persist_observed → runtime::drain(selected transition.effects) → storage::resolve(original Fence) → selected native NonCommit → runtime::event(exact FenceResolved) → selected core callback`.

No callbacks run when observation selection fails. Existing marker-before-source and fail-closed failed publication/reopen behavior remain. This private call-graph/publication-order change requires fresh full affected Code/State in addition to all originating counterexample closures; writer classification is not acceptance.

Three new actual SIGKILL cuts stop before Fence intent, after Fence intent sync and after Fence selection sync. Initial compiled control reopened as Integrity because newly issued Fence request accounting was not yet coupled; the correction retains it in the selected Observation image. All3 now reopen through the original request/callback as exactly one NonCommit, no accepted receipt, no live storage invocation and clean close. No request/terminal is inferred from absence.

The existing Fence-winner fixture originally searched only live storage_invocations and failed after automatic callback retirement. Its lookup now recovers the same original request from live **or retired** custody. Every original terminal, late-Begin, contrary-callback, conflict and no-accepted-state assertion is unchanged and GREEN. This is a fixture traversal adaptation, not an availability exception. Remediation1's only two explicitly superseded availability expectations remain exactly those recorded there; this patch adds none.

Pending-loss conversion preserves the boxed original Observation/Checkpoint and derives only a fixed32-byte cached digest token, avoiding a second full oversized input allocation. The existing loss grammar records the original captured continuation's digest when no bounded success witness exists. The full oversized observation is never installed in the image. Only genuine host-owned pending settlement can take this route; direct caller prefixes retain the byte bound. Unknown does not retire utility status. Internal custody inspection supplements the public API failure/restart witness and is not substituted for it.

## Helper-consumer audit

| Consumer | Current/callback boundary |
| --- | --- |
| Begin and begin_ingress | Decisions now use the complete cut actually retained. Original selected Fence is drained before return; an already Started attempt bypasses fresh rechecking and remains immutable. |
| Challenge/authenticate | Provider result establishes a challenge/possession, not append permission. Pre-source marker and subsequent exact policy/time/session custody are retained even for denial. Submit still requires fresh current policy. Existing denial and consumed-session reopen controls remain GREEN. |
| submit_local and Verify/Seal | Submit retains current observation before selecting its query. Verify/Seal begin with durable marker, retain the provider knowledge before delivering the exact core reply, and core `local_current/current_matches` checks the full selected policy/time. New advancement controls prove stale replies create no native attempt. |
| Recovery reattach/query replay | Exact selected original requests, historical bindings and consumed session custody remain controlling. Markers precede any current consultation; failed/unretained authority is unavailable. Original protected terminal/receipt and callback replay controls remain. Three new intermediate Fence deaths prove actual selected request reattachment. |
| Historical binding/proof and prior Started | No new current permission is synthesized. Genuine historical validity, exact receipt retry and Started-after-revocation controls are unchanged and pass. |

ObservePolicy's drained Fence callbacks retire exact original requests and report the terminal; the audited Pure NonCommit callback path does not consult current policy again or remint a verification. Existing finite runtime effect limits still apply.

## Exact gates and retained proof scope

| Selection | Result |
| --- | --- |
| Initial four external regressions | Expected compiling RED4;14.702s including build |
| Final full public `--test ic3_disk` | GREEN29 tests +3 owned helpers;6.729s wall; new five SIGKILL outcomes included |
| Final Records selection excluding unchanged171 historical native-kind cuts | GREEN29 tests +6 owned helpers;38.248s wall;57 native +65 ingress +26 observation +3 new Fence cuts =151 selected cuts, plus existing owned restart/terminal/X-Y controls |
| Existing Fence-winner/close/protected Started selection | GREEN3;3.949s after exact-request fixture adaptation, preserved original assertions |
| Auth/boundary/assembly | GREEN16/6/30;8.015s |
| Contract `check.sh crdt-production` | GREEN10.078s; retained core106 (original105 plus existing boundary control), STA34, released encoder/text, data/evidence/recovery/codec consumers, architecture, format and strict lint |
| Expanded disabled-source gate | Initial coverage assertion RED; final GREEN4,0.222s. New subtree and all old disabled branch fixtures retained |
| Node architecture adopter | GREEN0.764s; no role/edge/dependency relaxation |
| Final process-global guard | GREEN152 files, unchanged3 permanent/0debt,0.678s; actual children env_clear |
| Affected formatting and provenance | GREEN10 node +1 contract files, package editions and skip_children; protected45/27/23 and old committed5 consumer hashes verified |
| All-features/all-targets node Clippy | Exit0, exactly unchanged9 old warnings,4.539s; no new warning |
| Three-language candidate representation | GREEN21 semantic-input positives/18 negatives plus6 fail-closed mutants,0.904s; bytes unchanged, no crypto qualification claim |
| Strict pre-remote | Expected exit1: genuine B1 crypto qualification absent. No accepted record or pin relaxation |

The unchanged171 candidate/security/fork native-kind kill cuts were **not rerun**. Their earlier committed evidence remains pinned. Physical slot/floor/pair grammar and bytes are unchanged; their stable fixed current-cut path creates no ObservePolicy Fence effect. Their actual four-kind custody/terminal controls were rerun. The changed auth/Fence path instead has three dedicated deaths plus the retained57 native/65 ingress/26 observation matrices. No repeated319-cut whole survey is claimed. Prior whole-node format debt251 remains documented in immutable Remediation1 evidence; this task claims only affected-file formatting GREEN. Clippy's old9 debt is separately reported, not suppressed or fixed.

## Chronology and limits

The [new compressed run log](GladeIndependentCrdtProductionIntegrationB-Remediation2RunLog.md.gz) preserves exact commands, outputs, exits and wall times, including all failures. Labels describe intent, never override a nonzero exit. Separate compile/tool/fixture failures are not behavioral RED: missing TrustedCut import, partially moved Option, wrong assembly_di selector, Fence fixture live-only lookup and the final lifecycle/issuance failures all remain in chronological evidence. Public close Pending and intermediate SIGKILL Integrity were actual behavioral failures and drove bounded fixes. No zero-selected run supplies evidence.

This is a candidate second B correction of the same object: semantic2 architectural/1 nonarchitectural/1 completed remediation; typed2 architectural/5 prior nonarchitectural/2 completed remediations; B0 architectural/6 nonarchitectural/1 prior completed remediation. No writer closes findings or resets caps. Fresh full and originator acceptance remains mandatory. No public contract, private floor format, dependency, allowance, trusted authority, default, live root, external source, desk, push, C history transport, IC4 activation or owner-excluded BOM work changed.
