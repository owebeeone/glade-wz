# Combined IC-3B genuine authentication and physical persistence/recovery — CODE-AXIS REVIEW

**Review object:** Complete IC-3B1/B2 component at Glade `d3fded040d6e3c459cd5f975d6c672873356cc23`, controlled by the DRAFT [production integration design](/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md) at root `c8d778e7c24b68690e27bfac8ca4457f647e747f`.

**Baseline:**

| Repository | Exact revision |
|---|---|
| Workspace root | `c8d778e7c24b68690e27bfac8ca4457f647e747f` |
| Glade | `d3fded040d6e3c459cd5f975d6c672873356cc23` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

All five revisions matched at START and END. Sources were inspected through read-only working-file reads, pinned Git inspection and scoped diffs. All 62 corrected-manifest source hashes and five consumer-document hashes matched.

**Date:** 2026-10-04  
**Axis:** Architecture, interfaces, actual call graphs and compatibility. Independent, adversarial, read-only. Nothing here relies on another current axis or originating closure report. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block acceptance. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified while preserving the verified invariants below.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
|---|---|---|---|
| — | Fresh full Code reviewer; no own prior finding | Not applicable | This report supplies no originating-reviewer closure. Initial B Code/State closure and deferred original A2 closure remain separately governed. |

The legitimate prior merged remediation plan informed the attack targets. Its correction claims were examined independently against current code; they were not treated as finding closure.

## Changed-range analysis

The complete reviewed Glade range is `c69e6416f5f5155d4bb570bf272e796b2deae0a3..d3fded040d6e3c459cd5f975d6c672873356cc23`: 62 changed files, approximately 12,203 insertions. The first merged B correction is `9dbc677a25d93af0c561817571ad7eb1990ddadc..d3fded040d6e3c459cd5f975d6c672873356cc23`: 24 changed files, 1,732 insertions and 46 deletions.

The correction changes the shared receive-result witness, current-observation lifecycle, private floor envelope, exact retired Prepare recovery and public pending-settlement loss continuation. I traced these changes through the genuine evidence provider, actual Records session, native paired publication and recovery validation, including unchanged historical-binding consumers. This is a full component review, not disposition verification alone.

P2-1 is a new **nonarchitectural B implementation root**: the existing authoritative pre-start invariant is implemented with two inconsistent observations. P2-2 is a further counterexample to existing **nonarchitectural B-R4**, concerning the completeness of its public loss continuation. Neither requires a new typed contract or architectural role. No third new typed architectural root is identified.

The recorded semantic, typed and B remediation accounting is not reset by this report. In particular, typed has two completed remediations and B has one; corrections remain distinct from independent finding closure.

## 0. Evidence base

No files were modified. No builds, tests, network requests or live actions were run.

| Evidence inspected | Relevant coverage |
|---|---|
| Root/member instructions, review-loop skill and canonical prompt | Read-only mandate, exact tuple, scope exclusions, severity, accounting and report format |
| Controlling design, authentication, persistence, persistence usage, typed contract/usage and StorageAttempt contract | Authentication scopes; authoritative pre-start observation; historical validity; exact issuance; protected guards; loss and close lifecycle |
| Build entry, package architecture, library/testing policy, admission plan and consistency profiles | Integration ownership, semantic-selected core dependency, finite boundaries and storage strength |
| [Genuine authentication implementation](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/auth/operations.rs), provider, custody, namespace, text-profile and strict signing paths | Complete signed identity, monotonic policy/time, one-use challenge and possession, retained session custody, exact verification/sealing |
| [Actual Records session](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records.rs) and its storage, authentication, runtime, bindings, genesis, validation, recovery, guard, observation, persistence and loss modules | Actual public consumers; complete original requests and physical attempts; full replay before availability; settlement and continuation ownership |
| [Native paired storage](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/disk/paired.rs), metadata, validation, floors, slots and root custody | Stable locks, closed metadata, physically reserved slots, ordered publication, exact old/next selection and unresolved observation refusal |
| Public remediation tests; observation, fence, native-cut and ingress-cut tests; ingress API controls | Substitution rejection, retained loss, retired Prepare replay, denied observation retention, terminal winners and actual child-death test construction |
| B1/B2 and remediation evidence, both source manifests and compressed chronological logs | Recorded command results, failed intermediate attempts, corrected selectors, finite budgets and compatibility witness |

Read-only commands included `git rev-parse HEAD`, scoped `git diff`, `git show`, `rg`, `cat`, `sed`, `nl` and Python hash/log inspection.

Recorded evidence, rather than fresh execution, reports 228 native and 65 ingress process-death cuts, 26 marker cuts, successful targeted suites and an actual legacy-executable refusal test. Child helpers explicitly clear inherited environments. The final Records run reports 28 passing tests plus five ignored helper entry points, approximately 118 seconds of test time.

The logs retain earlier failures and zero-match selectors. The strict pre-remote gate remains an expected refusal with no accepted B record. Format and Clippy output retain identified baseline debt; I do not describe those checks as blanket green.

The B2 decompressed log SHA-256 is `242646bd8a7f5f0574fe16bf77593275b8a3ead4eef35b4fa937134e2de49c5f`. The remediation decompressed log SHA-256 is `b276e0f2349a9536b2eae265b7b0bb4ac91d5433715ab51fe705135232b98f5b`.

## 1. Findings

### [P2-1] Started uses an authorization decision older than the cut retained immediately before it

**Location:** [storage.rs:194](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/storage.rs:194), through line 209; supporting [authentication.rs:12](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/authentication.rs:12) and lines 34–66.

**Violated invariant:** IC3-AUTH-004 and StorageAttempt contract §4 require Records to match its latest authoritative observation to the plan before the protected start barrier. Observation and durable Started must form one serialized decision. Only a valid already-selected Started cut survives later revocation.

**Credible sequence:** Use the configured public DiskHost/DiskSession path with an injected scripted PolicyClock:

1. Retain valid signed policy P0 and time T0 through genuine challenge, possession, verification, sealing and preparation.
2. The first pre-start `EvidencePort::observe` returns P0/T0. Lines 194–202 set `allow = true`.
3. The immediately following `authentication::persist` calls `evidence.current()` again. Return a valid successor P1 that revokes the writer, or a monotonic time interval outside the sealed authorization window.
4. Persistence selects this newer policy/time in the actual image and trusted floor. Its ObservePolicy transition can also issue a Fence, but only the transition state is retained here.
5. `begin` never recomputes `allow`. It selects `Started` with the original P0/T0 cut, then `terminal` can publish Committed using the original historical binding.

No concurrent caller or untrusted policy is needed. The provider may legitimately advance between two consultations in one synchronous operation. The newer cut is learned and durably selected **before** Started. Historical receipt verification consequently cannot justify treating this as revocation learned after protected start.

**Impact:** A fresh local operation can acquire durable Started and a committed receipt after the host has already retained an authoritative cut that invalidates its pre-start authorization. Existing tests change policy before begin or after Started; those endpoints miss this internal ordering.

**Required correction:** Derive the start decision from the exact validated cut actually retained at the barrier. Avoid the second unaccounted consultation, or return and compare its retained cut before selecting Started. A newer mismatching cut must prevent start and follow the contract’s refusal/fencing behavior. Preserve the durable pre-observation marker and immutable outcomes for genuinely earlier Started attempts.

**Closure test:** Through the actual configured session, advance policy specifically between the first pre-start observe and persistence’s second source read. Repeat with time advancement. Assert no Started, Committed or AcceptedLocal result under the invalidated cut, retained current authority, and correct original-attempt resolution. Retain the existing positive control where revocation occurs after durable Started.

**Classification:** Nonarchitectural B implementation defect under an existing protected-start obligation.

### [P2-2] Public pending-loss discharge permanently rejects an oversized result it already owns

**Location:** [loss.rs:114](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/loss.rs:114), and lines 140–142; supporting [persist.rs:110](/Volumes/projects/limbo/glade-wz/glade/node/src/independent/records/persist.rs:110), lines 157–164 and 249–266.

**Violated invariant:** IC3-GUARD-002 explicitly requires bounded rejection/loss for oversized or partial messages and permanent observation loss when full necessary bytes cannot be retained. Persistence’s public contract identifies `retain_pending_ingress_loss` as the concrete route for an already-consumed, drained refused settlement.

**Credible sequence:**

1. Establish a valid guard with finite maximum M and sufficient reserved critical capacity.
2. Its owned receive becomes Ready with M+1 bytes. The capability is genuinely drained, although normal bounded-success provenance is absent.
3. Pass those bytes to normal settlement using an otherwise correctly bound checkpoint. `can_settle_received` returns Capacity, and the session captures the original permit, observation and checkpoint by value.
4. Invoke `retain_pending_ingress_loss`. It copies the complete oversized observation into `observed`, then rejects `observed.len() > max_bytes` before constructing the compact loss marker.
5. The refusal stores a PendingIngress::Loss containing those same oversized bytes. Every retry repeats Capacity.

The caller cannot recover the consumed permit to invoke the direct loss method with a bounded prefix. `record_observation` rejects changed bytes, and after conversion it refuses the pending Loss variant. Publication uncertainty or exhausted physical capacity is unnecessary.

**Impact:** A drained oversized input leaves the session permanently unable to close or admit a new same-instance receive, despite healthy storage and reserved compact-loss capacity. Restart conservatively preserves the Active guard but cannot recreate its live capability. Safety remains conservative; the promised live discharge lifecycle is incomplete.

**Required correction:** Let the owned pending-loss conversion produce bounded conservative loss evidence without requiring the entire refused observation to fit the receive bound. Preserve the original settlement custody on failed publication. Do not relax normal settlement provenance or infer an empty receive.

**Closure test:** Receive M+1 bytes through the public owned future, capture the refused settlement, then call the public pending-loss route. With sufficient critical capacity, require committed loss, clean close and reopen with permanent incompleteness. Add a failed-publication counterpart proving that the original permit/observation/checkpoint and Active guard remain retained.

**Classification:** Nonarchitectural incomplete B-R4 implementation; no new typed architectural root.

## 2. Invariant analysis

Several concrete attacks failed against the settled source:

- Normal settlement checks issuer-owned successful bytes before inventory authentication. A signed empty inventory cannot replace nonempty, malformed or Err input. Eligibility queries do not mutate status; retirement follows durable selection.
- Pre-observation markers precede current-source consultation. Failed retention leaves protected unavailable authority; unrelated publications cannot clear the marker. Receive-only holders can retain advanced cuts without append grants.
- Exact original Prepare/recover-plan issuance is checked in live or retired maps before fresh revision/allocation checks. PlanKey replay returns the retained immutable attempt; mutated or unissued requests fail.
- Native terminals retain exact bindings, receipts, revisions and start cuts. Contrary authenticated callbacks cannot replace the sole terminal. Historical receipt validation does not consult fresh authorization.
- Floor intent preserves advanced issuance and Active guards before image publication. Missing-next fallback cannot manufacture NonCommit or completeness. Full semantic validation precedes usable recovery selection.
- Root custody checks stable lock and directory identities. Slots have fixed capacity and actual backing allocation; critical capacity includes guard discharge. The actual old executable is exercised against the fresh compatibility marker.
- Reattachment retains finite request/session histories and reconstructs projection through actual core callbacks. Orphan guards and permanent loss continue to suppress completeness; a later empty inventory cannot erase them.

These checks support the bounded component design. They do not close either finding: neither the second pre-start source read nor the oversized captured continuation is covered by the relevant successful controls.

## 3. Risks and next action

Qualification remains LocalProcessRestart on the stated Unix filesystem profile. It supplies no machine/power-loss, quorum, arbitrary simultaneous rollback or IC-3C duplex qualification. Owner-excluded BOM work and wider original A2 closure are not findings here.

The next action is one merged, test-first correction for P2-1 and P2-2 within the existing accounting, followed by independent full review and separately required originating B counterexample closure at the resulting exact tuple. Accepted-review metadata and the strict pre-remote gate must remain closed until those requirements are satisfied.