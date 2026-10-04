# Combined IC-3B1/B2 authentication and physical recovery component — STATE-AXIS REVIEW

**Review object:** Glade source `c69e6416f5f5155d4bb570bf272e796b2deae0a3..9dbc677a25d93af0c561817571ad7eb1990ddadc`, controlled by `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at root `db29535fe162356df93fbdc28c052185cfd9a0f7`. Committed candidate; combined B qualification pending.

**Baseline:** The following heads matched the prescribed tuple at both START and END. Source was inspected with scoped reads/diffs and checked against committed bytes and recorded SHA256 pins.

| Repository | START and END HEAD |
| --- | --- |
| Workspace root | `db29535fe162356df93fbdc28c052185cfd9a0f7` |
| Glade | `9dbc677a25d93af0c561817571ad7eb1990ddadc` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

**Date:** 2026-10-04  
**Axis:** Durable-state semantics, restart legality, custody and fail-closed recovery. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on their testimony. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block qualification. I pre-commit to GO on a revision resolving P2-1, P2-2 and P2-3 as specified, subject to exact-tuple verification, the required executable regressions and full review of any materially changed boundary.

---

## 0. Evidence base

I read the generated State prompt, root/member instructions, `AGENTS_GWZ.md`, the review-loop skill and canonical template. Controlling inputs inspected include BuildEntry, library/testing policy, package architecture, the complete production design/plan, typed contract/usage, authentication and persistence documents, persistence usage, admission plan, resource profiles and storage-attempt contract. I also inspected relevant archived admission/storage lifecycle clauses, the prior typed State report and merged remediation plan, and the production review ledger through the frozen B checkpoint. No current peer report was read.

Source inspection covered the combined component: namespace and genuine-authentication implementation; disk layout, locks, floor replacement, slots, metadata and paired recovery; Records host/session, genesis, authentication persistence, bindings, validation, reserves, runtime, native storage, guards, observations, settlement and loss; configured assembly and preserved default refusal; the ingress utility and public consumer controls. I inspected the corresponding authentication, physical, native-kill, ingress-kill, cancellation, capacity, restart, callback and external consumer tests.

The principal finding locations are:

| Finding | Exact source locations |
| --- | --- |
| P2-1 | `glade/contracts/crdt-recovery-api/src/ingress.rs:11–15,138–150`; `glade/node/src/independent/records.rs:297–321`; `records/observations.rs:5–60`; `records/persist.rs:92–163` |
| P2-2 | `glade/node/src/independent/records.rs:103–126,255–279`; `auth/operations.rs:90–147,204–215`; `records/authentication.rs:4–42` |
| P2-3 | `glade/node/src/independent/records/persist.rs:153–161,211–268`; `records.rs:49–89`; `records/observations.rs:63–90`; `records/loss.rs:38–43,79–132`; `records/tests/ingress.rs:207–245` |

All 56 recorded Glade source hashes and five consumer-document hashes matched working and committed bytes. The B2 compressed chronology matched SHA256 `ba20c7c821e399916128e9588fc3bbaaacf458ff9601518bc3aa937139bad01c`; its 722,988 decompressed bytes matched `242646bd8a7f5f0574fe16bf77593275b8a3ead4eef35b4fa937134e2de49c5f`. I read the B1/B2 evidence, source manifest, chronological command/result index and relevant raw outputs, including final selections and preserved failures.

Recorded execution evidence, rather than execution performed by this reviewer:

| Selection | Recorded result and limit |
| --- | --- |
| Contract production selection | Architecture, retained core/API/data/evidence/recovery/codec consumers, format and strict contract Clippy pass |
| Configured/default node consumers | Authentication 16, boundary 6, assembly 30 and final public disk 8 pass |
| Complete physical Records selection | 26 pass, four owned child helpers ignored in the parent selection; 109.196 seconds wall, 101.07 seconds test execution |
| Native/ingress crash witnesses | 228 native and 65 ingress real child-process kill cuts |
| Genuine legacy executable | Explicit ignored selection passes; fresh-root refusal witness, not existing-store migration |
| Disabled source and process globals | Four source controls pass; 148 files, unchanged three permanent allowances |
| Node architecture | Adopted gate and negative dependency fixtures pass |
| Formatting/Clippy | Exact affected formatting passes; whole-node formatting retains 251 historical hunks; node Clippy retains nine baseline warnings |
| Canonical remote prerequisite | Candidate vectors pass; strict pre-remote mode correctly refuses absent accepted B qualification |

Earlier failed selectors, RED attempts and other nonzero exits remain evidence, not successes. I ran no builds, tests, network or live actions and changed no files. The counterexamples below are source-derived public-call sequences; they are not claimed as newly executed regressions.

## 1. Findings

### [P2-1] A drained permit does not bind settlement to the receive result

**Location and invariant.** `GuardedReceive::poll` marks only `drained=true` and returns the supplied result. Neither permit nor issuer retains its bytes, digest, length or success/failure outcome. `DiskSession::settle_ingress` validates the separately supplied signed observation and checkpoint, then checks only settlement eligibility. IC3-GUARD-002 requires exact received observation custody and prohibits treating partial/rejected input as a complete empty manifest; IC3-CUT-003 requires honest accounting of consumed input.

**Credible sequence.** Start with the existing public empty-round test at `glade/node/tests/ic3_disk.rs:235–275`. Keep its genuine signed empty inventory, observation and checkpoint, but make the owned receive return different bytes—for example `Ok(Bytes::new(b"partial invalid header".to_vec()))`. Await the returned permit, then submit the unchanged signed empty observation.

Every implemented settlement predicate still passes: scope, guard, signature, expected entries, checkpoint and drain status. The differing returned bytes are never compared. The host selects the empty observation, retires the guard and can report `complete_local=true`. Reopen authenticates only the substituted retained observation. Returning `Err(Fault::Unavailable)` instead also marks drained and leaves the same normal-settlement path available.

This needs no detached callback or forged signature. A classifier pairing the wrong round result with an otherwise genuine observation is sufficient. Authentication of the supplied empty snapshot does not establish that it was the snapshot this receive consumed.

**Impact.** The physical host can forget consumed uncertainty and publish a complete cut. Correct file ordering cannot repair the missing association: it durably preserves the wrong observation.

**Required correction.** Establish issuer/host-owned completion provenance binding successful receive bytes to normal settlement. Normal settlement MUST match that exact successful result. Errors, partial input and unretainable input MUST remain guarded or use conservative loss/rejection custody. Do not replace this with caller flags or a caller-provided digest asserted as authority. Preserve nonmutating eligibility and retirement only after durable selection.

**Closure tests.** External public consumers must show that malformed-prefix→signed-empty, genuine-nonempty→different-signed-empty and receive-error→signed-empty cannot commit normal settlement or restore completeness. Verify conservative state across actual kill/reopen. Retain successful exact empty/nonempty controls.

**Root classification.** One new architectural root against the **typed ingress object**: the capability proves completion but carries no association with its completed result across the receive/settlement boundary. This is distinct from the earlier generic codec type-erasure root. The accepted semantic exact-observation requirement is adequate; no new semantic rule is required.

### [P2-2] Refused authentication and ingress calls can forget learned floors on restart

**Location and invariant.** `GenuineEvidence::current` advances `observed_policy` and `observed_time_floor` after validating the injected cut. The public physical session persists authentication only after `issue_challenge` or `authenticate` returns success. Their subsequent authorization/window failures therefore bypass persistence. `begin_ingress` likewise calls `current`, then returns `Unavailable` when that learned cut differs from the durable floor.

IC3-AUTH-003/005 and IC3-DISK-002 require retention of learned policy/time floors and prohibit recovering older authorization after observing expiry or revocation.

**Credible sequence.**

1. Provision the genuine fixture at time `[20,21]`; the durable time floor is 20.
2. Supply valid injected time `[1001,1002]`, retaining the signed policy. Call public `issue_challenge` with the fixture’s `[10,100]` request window.
3. `current` records floor 1001 in memory. Window validation returns `Integrity`; `records.rs:111` exits before persistence.
4. Close/reopen the original roots using fresh genuine evidence supplying `[20,21]`. The authoritative disk floor is still 20, so this lower observation passes. A challenge in the formerly expired window can again succeed.

A same-process rollback is refused; restart loses precisely that protection. A valid next-generation revoking policy produces the analogous sequence: it is learned, challenge/authentication is denied, but reopening against the older persisted policy forgets the revocation. No data or trusted-floor rollback is required.

`begin_ingress` has the same persistence gap when its independently observed policy/time advances; it refuses because the durable floor differs without retaining the advancement.

**Impact.** Restart can restore fresh authorization that the physical host already knew was expired or revoked. The recorded unavailable-verification floor witness exercises another path and does not cover these early returns.

**Required correction.** Retain every validated monotonic observation before returning from an enclosing authorization operation, including refusal. Use the already conservative floor-ahead recovery mechanism where full image persistence cannot complete. If retention fails, keep the host unavailable/uncertain; do not permit a later restart to serve the older floor as current authority. Preserve historical receipts and protected original Started cuts.

**Closure tests.** Exercise public challenge denial, response denial and ingress denial with advanced time and signed revocation. After actual child termination/reopen, lower time/older policy must refuse. Include persistence failure and critical-capacity controls, plus unchanged historical receipt/retry behavior.

**Root classification.** One nonarchitectural **B implementation** root: caller paths omit the required durable observation step. The controlling semantic/typed floor contract does not need redesign.

### [P2-3] A refused settlement consumes the only public route to permanent loss

**Location and invariant.** Refused/unknown settlement retains its permit in private `PendingIngress::Settle`. Public `retain_ingress_loss` requires that already-consumed permit. `retry_ingress` repeats the exact settlement; `record_observation` permits checkpoint correction only for the identical observation. No public operation converts the host-owned drained settlement continuation to loss.

IC3-GUARD-002/003 and IC3-DISK-006 require a live drained receive to support conservative classification/loss and clean lifecycle completion while retaining ownership.

**Credible sequence.** Retain legitimate earlier observations until aggregate inbox bytes leave insufficient space for another full observation. Guard establishment can still succeed: it checks per-round bounds and reserves compact critical loss, rather than the full ordinary observation. Drain a valid new inventory and call `settle_ingress`. `observations::retained` refuses when aggregate inbox bytes exceed the signed bound.

The host now owns the drained permit, observation and checkpoint. Exact retry repeats the permanent capacity refusal. Correcting the checkpoint cannot remove old history or change the retained observation. The caller cannot invoke loss because it no longer possesses the permit. Close remains `Pending`, retaining both physical locks. A malformed observation produces a similar permanently refused continuation.

The supplied test at `records/tests/ingress.rs:231–243` reaches loss by directly removing the private pending entry and extracting its permit. That route is unavailable to an external consumer.

**Impact.** A fully joined live receive enters a new stuck state despite pre-reserved loss capacity. Dropping/killing the session converts it into the documented orphan limitation; that is not clean discharge of a live owned continuation.

**Required correction.** Provide a host-owned route that conservatively converts an existing drained settlement continuation to loss, preserving the original guard and custody. It may be an explicit operation keyed to that retained continuation or an automatic conservative transition for permanent refusal. It MUST NOT remint a permit, infer absence, clear an undrained guard or bypass unresolved physical uncertainty.

**Closure tests.** Using public APIs only, force aggregate-capacity and malformed-observation refusals, convert to loss, reach `Closed`, and reopen with permanent loss and `complete_local=false`. Inject loss-write failure and prove the original guard remains authoritative. Private pending-map extraction cannot constitute closure.

**Root classification.** One nonarchitectural **B lifecycle implementation** root. Existing semantics already authorize drained conservative loss; the implementation hides its necessary retained capability.

## 2. Invariant analysis

The publication-order attack otherwise held. Paired transactions validate old/next state before intent, retain advanced issuance and active guards in the intent, write/sync the inactive slot, and select/sync the floor before acknowledgment. Recovery validates candidate state before completing selection. Missing/invalid next images preserve advanced floors and guards rather than inventing terminal NonCommit. Recorded native and ingress kill matrices genuinely terminate child processes.

Stable root custody, substitution refusal, finite allocated slots and critical reserves have concrete source and recorded physical controls. Independent X/Y custody is exercised; unresolved X does not impose a shared floor wait on Y. Data-only rollback and missing floor fail closed. Simultaneous rollback of trusted floor and data remains outside the stated trust profile.

Native replay preserves complete requests and original terminal receipts through the actual core callback path before availability. Reserved fencing, immutable terminal winners, duplicate/contrary callbacks and valid original Started cuts under later revocation have specific controls. All four request kinds retain their distinct custody/receipt semantics.

Ingress establishment precedes factory invocation. Foreign ownership refuses before work construction; started pending/drop does not prove drain or abandonment. Eligibility queries do not mutate status. These controls remain valuable, but do not bind completed bytes or provide the missing post-refusal loss operation.

| Controlling object | Accounting preserved by this review |
| --- | --- |
| Semantic | Existing two architectural/one nonarchitectural roots; one completed merged remediation; unchanged |
| Typed | Existing one architectural/five nonarchitectural roots; one completed remediation; add P2-1 as second architectural root |
| Combined B | Initially zero roots/rounds; add two nonarchitectural roots, P2-2/P2-3; zero completed remediation rounds |

No third architectural root is classified here. Owner-deferred original typed closures remain deferred; this report neither closes them nor treats deferral as B qualification.

## 3. Risks and next action

Qualification remains Unix LocalProcessRestart, not power-loss, machine-loss or quorum durability. Orphan Active guards after unclean death remain honestly pending; that documented limit is not a finding. Automatic duplex exchange, activation, existing-store migration and owner-excluded BOM work remain outside this review.

The next action is one merged, test-first remediation addressing all three roots, with accounting retained against their controlling objects. Re-freeze one tuple and obtain required independent reviews, including Surface for changed lifecycle shape. Preserve chronological failures, existing consumers, dependency classifications and allowlists. Keep the strict pre-remote gate refusing until genuine accepted B qualification exists.