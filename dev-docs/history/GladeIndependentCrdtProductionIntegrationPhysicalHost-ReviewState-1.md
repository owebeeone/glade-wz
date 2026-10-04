# Combined IC-3B genuine authentication and physical persistence — STATE-AXIS REVIEW

**Review object:** Complete combined B component at Glade `d3fded040d6e3c459cd5f975d6c672873356cc23`, controlled by `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at workspace root `c8d778e7c24b68690e27bfac8ca4457f647e747f`. Corrected candidate; acceptance and originating closures remain pending.

**Baseline:**

| Repository | Exact revision, verified at START and END |
| --- | --- |
| Glade workspace root | `c8d778e7c24b68690e27bfac8ca4457f647e747f` |
| Glade | `d3fded040d6e3c459cd5f975d6c672873356cc23` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

Sources were inspected with `cat`, `sed`, `nl`, `rg`, scoped `git diff` and `git show`. Read-only Python verified committed content hashes and inspected compressed recorded evidence.

**Date:** 2026-10-04.

**Axis:** Durable-state semantics, publication ordering, restart legality, ownership and conservative recovery. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified, preserves the held invariants below, and supplies the required regression evidence.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| None owned by this reviewer | Fresh full review after material shared-boundary changes | Not applicable | This report does not replace originating B closure reports or deferred A2 closures |

The legitimate merged remediation plan was an input. Its implemented dispositions are not treated as independent closure. P2-2 identifies a remaining edge of B-R4; this fresh reviewer does not close or reopen an originating reviewer’s finding on their behalf.

## Changed-range analysis

The full combined range is `c69e6416f5f5155d4bb570bf272e796b2deae0a3..d3fded040d6e3c459cd5f975d6c672873356cc23`: 62 files, including genuine authentication, physical slots/floors, Records, utility ingress and consumers.

The correction range is `9dbc677a25d93af0c561817571ad7eb1990ddadc..d3fded040d6e3c459cd5f975d6c672873356cc23`: 24 files, 1,732 insertions and 46 deletions. It adds issuer-recorded received-result provenance, protected current-observation metadata, live/retired Prepare lookup, public retained-loss conversion and associated evidence. These changes warrant the fresh full review because the receive capability and physical recovery lifecycle changed.

Both findings are **nonarchitectural B implementation defects**. P2-1 fails an existing AUTH-004/start-barrier contract; correcting which validated observation controls Started requires no new public authority or interface. P2-2 incompletely implements the already-authorized compact-loss discharge route. Neither establishes a third typed architectural root or resets any object’s accounting. The ledger’s two completed typed remediations and one completed B remediation remain corrections, not closure.

## 0. Evidence base

The review inspected the workspace/member instructions, `AGENTS_GWZ.md`, review-loop skill and canonical template, BuildEntry, boundary/package policies and the committed review ledger. Controlling inputs included ProductionIntegration Design/Plan, TypedContract/TypedUsage, Authentication, Persistence/PersistenceUsage, AdmissionPlan, ResourceConsistencyProfiles and the storage-attempt contract, particularly its authoritative-start and fencing clauses.

The production call graph inspected comprises:

- `node/src/assembly.rs:312–334` and `independent/mod.rs`: explicit configured construction and unchanged default refusers.
- Complete production implementations in `independent/auth/{provider,operations,custody}.rs`, `namespace.rs` and `text_profile.rs`.
- `independent/disk.rs`, `disk/{floors,slots,metadata,paired}.rs` and `paired/validation.rs`: root locks, canonical private envelope, intent/image/selection publication and recovery.
- `independent/records.rs` and its authentication, bindings, genesis, validation, runtime, storage, guards, observations, persist, reserves, loss, host and evidence-recovery modules.
- `contracts/crdt-recovery-api/src/ingress.rs`, including completion recording and nonmutating eligibility.
- Core `lifecycle.rs:245–293` and `callbacks.rs`, which establish how policy observation issues fences and how authenticated terminal callbacks install custody.

Relevant test bodies inspected include the genuine fence/Started controls, native and observation-marker kill matrices, cancellation/X–Y independence, ingress capacity/discharge controls and external remediation consumers for substitutions, retained loss and monotonic observations. Recorded test execution was audited; no test was executed during this review.

Content verification found:

| Artifact | Verification result |
| --- | --- |
| B1 source manifest | All 14 hashes match its committed implementation |
| Initial B2 source manifest | All 56 hashes match `9dbc677…` |
| Corrected combined manifest | All 62 hashes match `d3fded…`; current source also matches |
| Corrected consumer documents | All five hashes match |
| B2 compressed chronology | Raw SHA256 `242646bd8a7f5f0574fe16bf77593275b8a3ead4eef35b4fa937134e2de49c5f`; gzip SHA256 `ba20c7c821e399916128e9588fc3bbaaacf458ff9601518bc3aa937139bad01c` |
| Correction chronology | Raw SHA256 `b276e0f2349a9536b2eae265b7b0bb4ac91d5433715ab51fe705135232b98f5b`; gzip SHA256 `fdb8fa4f9631e8299f0846cf72c72f7e128c2d2046b895f54c79f69d7e9c3862` |

The correction log records successful actual Records coverage—28 tests plus five ignored child helpers, 121.675 seconds wall—and physical foundation coverage—16 tests plus three ignored prerequisites. The corrected exact legacy selector actually ran one successful test; its preceding zero-selection invocation is not evidence. Public process-death controls and the final receive-only chained-policy witness also have successful recorded executions.

Both five-repository tuple checks matched. No files were modified, and no builds, tests, network or live actions were performed. Current peer/full/closure testimony was not read.

## 1. Findings

### [P2-1] Started uses an authorization decision made before a newer authoritative cut is selected

**Location:** `glade/node/src/independent/records/storage.rs:188–224`, especially `allow` at 194–202 and persistence at 203; `records/authentication.rs:10–42`. Supporting behavior: core `lifecycle.rs:268–288`.

**Violated invariant:** StorageAttemptContract lines 185–196 requires the latest authoritative observation and protected durable Started barrier to form one serialized transition. Design IC3-AUTH-004 requires an exact current cut before Started. A later learned revocation must not be treated as occurring after an already-protected start when Started has not yet been published.

**Credible sequence:**

1. Prepare a genuine local AcceptedBatch under signed policy A and interval T. Its exact issued Begin request remains Reserved.
2. Use a valid injected `PolicyClock` whose next observation returns A/T. `storage::begin` independently observes that cut and sets `allow=true`.
3. The immediately following `authentication::persist` calls `evidence.current()` again. This time the provider returns a properly chained policy B revoking the writer or admitting node, with monotonic time.
4. Persistence selects B into the kernel image and trusted issuance floor. `Event::ObservePolicy` also retains an issued Fence for the Reserved attempt; its returned effects are discarded by persistence.
5. `storage::begin` does not recalculate `allow` against B. It writes `Started { cut: A/T }`, then `terminal` selects Committed with the original receipt.

The kernel terminal callback accepts this genuinely issued, binding-correct terminal. Historical validation subsequently checks the original signed A admission, so restart does not discover that B was selected before Started.

A same-policy time advancement between these two reads similarly defeats exact interval matching.

**Impact:** A fresh local operation can acquire protected Started and a durable accepted receipt after the host has already learned and selected a cut that forbids that start. The existing Started-survives-later-revocation guarantee then preserves the incorrectly authorized result.

**Required correction:** Make the persisted validated cut and the start predicate agree at the serialized start boundary. Persist the exact observed cut without silently replacing it, or return/use the actual selected observation and re-evaluate the complete precondition before Started. Retain genuine earlier Started behavior and conservative marker handling on failure.

**Closure test:** A compiling genuine DiskHost regression must return A/T for the first prestart observation and chained revoked B for the persistence consultation. Assert no Started, Committed or AcceptedLocal under A; drive the original issued fence and verify immutable NonCommit. Repeat with changed time, then SIGKILL/reopen at the relevant observation/start boundaries. Keep the positive control where revocation occurs only after a legitimately selected Started.

**Root classification:** Nonarchitectural B ordering defect implementing the existing authoritative-start invariant.

### [P2-2] Oversized drained input cannot reach the public compact-loss discharge

**Location:** `glade/node/src/independent/records/loss.rs:109–115,140–141,175–183`; `records/persist.rs:110–112,157–164,237–260`; ingress utility completion recording at `contracts/crdt-recovery-api/src/ingress.rs:165–174`.

**Violated invariant:** Design IC3-GUARD-002 explicitly places dropped-overlimit bodies inside guarded consumption and requires bounded rejection/loss when full bytes cannot be retained. Persistence documents a public route from an already-consumed drained refusal to compact permanent loss. Critical reservation exists precisely to discharge such uncertainty without retaining the full rejected body.

**Credible sequence:**

1. Establish a valid guard with finite `max_bytes=N`; its compact-loss space is reserved.
2. Let the owned receive reach Ready with `Ok(Bytes)` of length `N+1`. The utility marks the permit drained and correctly records no successful bounded-result witness.
3. Submit those bytes in an Observation. Normal settlement refuses Capacity and retains the by-value permit/observation/checkpoint in `PendingIngress::Settle`.
4. Call `retain_pending_ingress_loss(&guard.id)`. It forwards the entire retained observation bytes to `retain_with`, whose line 140 rejects the same `N+1` length before constructing the compact marker.
5. The continuation becomes `PendingIngress::Loss` containing the same oversized bytes. Every retry repeats Capacity.

The public API cannot recover the consumed permit or provide a bounded prefix. `record_observation` cannot replace the retained immutable Observation, and after conversion its pending variant is Loss. Close remains Pending and holds both root locks. Killing/reopening turns this safely drained live round into an orphan Active guard with no reconstruction route.

**Impact:** Oversized input creates an avoidable permanent lifecycle stop even with healthy storage and sufficient reserved loss capacity. Safety remains conservative, but the promised drained-refusal→loss→Closed path is incomplete.

**Required correction:** Derive a compact conservative digest or bounded prefix from the host-owned retained bytes during loss conversion. Preserve original continuation custody through publication failure. Do not relax normal settlement bounds, accept caller-authored completion authority or manufacture orphan permits.

**Closure test:** Through public DiskSession APIs only, receive exactly `max_bytes+1`, obtain the normal refusal, then require committed pending-loss conversion and Closed. Reopen must retain permanent loss and `complete_local=false`. Add an oversized supplied-observation substitution case and a failed-loss-publication control retaining the original continuation/Active guard. Exercise actual child death around loss selection.

**Root classification:** Nonarchitectural remaining edge of B-R4, not a new typed architectural root.

## 2. Invariant analysis

The following attacks did not produce additional findings:

**Physical publication and fallback:** Intent precedes inactive-image publication; image sync precedes selected-floor replacement; floor file and directory sync precede acknowledgement. Unknown writes poison live custody. Missing-next fallback retains advanced issuance and old Active guards and records unmaterialized intent; it does not install proposed retirement or invent NonCommit. Complete pending-next images undergo authenticated validation before resync and selection.

**Protected observation:** The v2 marker precedes current-source consultation. Missing/torn Observation recovery refuses usable full/narrow reopen, and unrelated publications retain the marker. The explicitly superseded availability assertions are stronger conservative stops, not weakened safety proofs.

**Ingress provenance:** Issuer-owned length/digest recording prevents nonempty, malformed or Err receive results from being replaced by a separately signed empty inventory. Eligibility does not retire status. Failed selection retains owned continuations; successful retirement follows durable selection. Started or dropped pending work cannot qualify for abandonment.

**Native recovery:** Prepare matches complete live or retired issuance before immutable existing-attempt lookup. Resolve requests retain full bindings; original terminal custody leads callback replay rather than fabricated receipt reconstruction. Genuine contrary callbacks retain integrity diagnostics.

**Completeness and ownership:** Active guards, permanent loss, missing obligations, unmaterialized intents and kernel sticky uncertainty suppress combined completeness. Empty later rounds cannot erase them. Stable lock paths survive clean release, and registration binds canonical paths and directory/lock identities. The actual cancellation witness joins X’s task while independent Y commits.

## 3. Risks and next action

Qualification remains bounded Unix LocalProcessRestart. It does not establish machine/power-loss, quorum, simultaneous trusted-floor/data rollback or corruption-repair guarantees. Unclean orphan receives intentionally remain pending; that limitation is distinct from P2-2’s avoidable loss of a live drained continuation.

Recorded whole-node formatting remains 251 inherited hunks, and Clippy retains nine inherited warnings. These are baseline debt, not whole-package cleanliness. Automatic exchange, wider typed closure and activation remain excluded; the strict pre-remote gate correctly remains closed.

The next action is one ledger-accounted correction addressing these two nonarchitectural defects with test-first evidence, followed by independent re-verdict and the separately required originating closures on the same settled tuple. B acceptance and C qualification must remain blocked meanwhile.