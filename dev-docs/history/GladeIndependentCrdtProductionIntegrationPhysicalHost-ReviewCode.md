# Combined IC-3B1/B2 Physical Host — CODE-AXIS REVIEW

**Review object:** Genuine authentication and complete physical persistence/recovery component, Glade source range `c69e6416f5f5155d4bb570bf272e796b2deae0a3..9dbc677a25d93af0c561817571ad7eb1990ddadc`, controlled by DRAFT `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at workspace root `db29535fe162356df93fbdc28c052185cfd9a0f7`, reviewed 2026-10-04. The candidate `IngressAuthority::can_settle/can_abandon` extension is included.

**Baseline:** Workspace root `db29535fe162356df93fbdc28c052185cfd9a0f7`; Glade `9dbc677a25d93af0c561817571ad7eb1990ddadc`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `400cedcf1fff74128f366af758a0c389a23435d7`. All five heads matched at START and END. Sources were inspected through read-only file reads, scoped Git inspection and read-only Python evidence auditing.

**Date:** 2026-10-04

**Axis:** Code — architecture, interfaces, call graphs and compatibility reality. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block qualification.

---

## 0. Evidence base

I read the complete review prompt, applicable root/member instructions, review-loop skill and canonical template. Controlling inputs included the Build Entry, library/testing policy, package architecture, production integration Design/Plan, Typed Contract/Usage, Authentication, Persistence/Usage, Admission Plan, Storage Attempt Contract, resource consistency profiles, relevant linked contracts and review-cycle accounting. The previous typed remediation plan supplied historical root classification; no current peer testimony was read.

Source inspection covered the combined component: `independent/auth` and namespace/text validation; physical disk, slot, floor, metadata, paired publication and validation modules; complete Records implementation and authentication, binding, recovery, genesis, guard, host, loss, observation, persistence, reserve, runtime, storage and validation modules; the recovery API’s ingress utility; assembly, package and architecture-policy changes; associated integration, unit, child-process and consumer tests. In particular, the finding paths were traced through:

- `contracts/crdt-recovery-api/src/ingress.rs:11–149`;
- `node/src/independent/records/persist.rs:92–163`;
- `records/observations.rs:5–61` and `records/guards.rs:130–164`;
- `records/storage.rs:3–34` and `records/bindings.rs:5–137`;
- `records.rs:255–295`, `auth/operations.rs:90–147`, `records/authentication.rs:4–42`, and `records/runtime.rs:186–199`.

I audited B1/B2 evidence, the source-pin manifest and the complete chronological compressed run log. All 56 implementation-file hashes and five consumer-document hashes matched the manifest. The compressed log SHA-256 was `ba20c7c821e399916128e9588fc3bbaaacf458ff9601518bc3aa937139bad01c`; its uncompressed SHA-256 was `242646bd8a7f5f0574fe16bf77593275b8a3ead4eef35b4fa937134e2de49c5f`. It contains 14,972 lines and preserves unsuccessful commands and subsequent corrections.

Recorded final evidence includes the contract gate passing; Records tests with 26 passed and four ignored helpers; 228 native publication cuts and 65 ingress cuts; genuine authentication, assembly, boundary and disk tests; disabled-platform source checks; adopted architecture checks and negative controls; and process-global checking of 148 files with no new debt. Targeted formatting passed. Whole-node formatting remained baseline debt, and Clippy retained nine pre-existing warnings. These are recorded results, not commands I reran.

The legacy-refusal witness uses an actual old executable from `e8097861c9559ce9753b75231ee2aac8f9e28064`, SHA-256 `5da9c9da8eb48da7f1ac484674b0f2238b59735714960c3109eccf4e1b42b4d4`. Physical tests launch actual test executables with `env_clear()`, kill them at publication cuts and reopen real disposable disk roots. They establish component process-restart evidence; they do not establish IC-3C duplex-node transport or machine/power-loss durability.

No files were modified. No builds, tests, network or live actions were performed.

## 1. Findings

### [P2-1] Settlement authority does not bind the observation to the receive result

**Location:** `contracts/crdt-recovery-api/src/ingress.rs:11–15,138–149`; `node/src/independent/records/persist.rs:109–141`.

`GuardedReceive` records only started/drained/closed status. Any `Poll::Ready`, including `Err`, sets `drained = true`. Neither the permit nor its issuer retains the successful result’s bytes, length or digest. `settle_ingress` independently authenticates its caller-supplied Observation, then asks only whether the permit was drained.

This violates Design IC3-GUARD-002: settlement must retain the exact authenticated received observation bytes, and a rejected or partial message cannot become a complete empty inventory.

**Counterexample:** Start from the existing genuine finite-empty-round fixture and establish its authorized durable guard. Let its owned receive finish with partial invalid inventory bytes, or `Err(Fault::Unavailable)`. Alternatively, receive a valid signed nonempty inventory. Take the returned permit and supply a separately valid signed empty Observation for the same guard/source/channel, with the corresponding checkpoint. All implemented settlement predicates can pass. The host selects the empty observation, retires the guard and leaves no obligations. On a previously complete kernel cut, `guards::reconstruct` returns `complete_local = true`; reopening preserves that selected result.

No signature forgery, foreign permit, private-field construction or caller-authored “drained” flag is required. Authenticating the substituted observation does not authenticate its relationship to the receive.

**Impact:** The host can discard evidence of consumed nonempty, rejected or partial input and durably report completeness.

**Required correction:** Preserve issuer-owned evidence of the receive outcome and exact successful bytes, or an equally strong bounded identity, in the owned continuation. Settlement must compare that witness with the Observation before I/O. Failed or unclassifiable results must follow conservative loss handling. Refusal and unknown publication must preserve the original owned continuation.

**Closure tests:** Through actual DiskHost/session consumers, test nonempty-to-empty substitution, partial-to-empty substitution and receive-error-to-empty substitution. Each must retain incompleteness or refuse settlement. Include exact empty and nonempty positive controls, and kill/reopen around selection.

**Root classification:** New **architectural typed-contract root**. The capability representation lacks custody of the result it purports to authorize; the B implementation exposes that omission. This is distinct from the previous generic physical-type identity root. Charge it to the typed object, advancing its architectural count from one to two. The semantic requirement is already correct.

### [P2-2] Exact original prepare recovery stops working when its invocation retires

**Location:** `node/src/independent/records/storage.rs:8–34`; both session `prepare` and `recover_plan` delegate to this function.

The issuance check searches only `State.storage_invocations`. After the genuine core consumes the prepare reply, it moves the unchanged request to `retired_storage_invocations`. The check then returns `CallbackMismatch` before reaching the retained PlanKey/Binding deduplication branch.

Storage Attempt Contract §3 requires repeated prepare/recover_plan for the exact PlanKey to recover the same attempt, and explicitly separates attempt lifetime from invocation retirement. Refusing a replayed callback at the consumer does not justify refusing retrieval of an already retained native attempt.

**Counterexample:** Submit a genuine local operation and let its preparation and commit complete. Obtain the unchanged original Prepare request from the retained retired map. Call `prepare(original)` or `recover_plan(original)` on the same session. Repeat after clean close/reopen. The host refuses although the original exact attempt remains in Recovery. The same defect applies once the prepare callback retires while the attempt remains unresolved.

**Impact:** An executor recovering an original request/reply cannot retrieve its already allocated attempt through the contracted host command. Original request custody exists but is unusable for this purpose.

**Required correction:** Authenticate the complete original issued request against the live or retired request map, then perform exact existing-plan recovery. Preserve rejection of changed, foreign and unissued requests. Do not introduce AttemptId-only or high-water authorization, duplicate allocation, or fresh revision checks that override exact existing-plan recovery.

**Closure tests:** For all four request kinds, replay the exact original prepare/recover request before and after callback retirement and process reopen. Assert the original AttemptBinding, unchanged counters and charges, and no second attempt. Mutated and unissued requests must still refuse.

**Root classification:** New **nonarchitectural B implementation root**. The necessary original request map and immutable attempt mapping already exist; the lookup omits one existing custody state.

### [P2-3] Ordinary trusted-time advancement can permanently disable receive-only ingress

**Location:** `node/src/independent/records.rs:262–268`; `auth/operations.rs:90–114`; `records/runtime.rs:193–199`.

`begin_ingress` obtains the current provider-owned policy/time observation, advancing the evidence object’s observed floor. It then requires that observation to equal the durable issuance floor and returns `Unavailable` on any difference. It does not persist the new observation.

Successful challenge/authentication/local submission can persist observations, but those paths require local append authorization. Reattachment only persists when cached image policy/time differs from the external durable floor; it does not resolve the ordinary case where both retained values are old.

**Counterexample:** Create a genuine signed genesis policy admitting the local node and authorizing the remote replica holder, with no append grants. Provision trusted time `[20,21]`, producing durable floor 20. Later provide valid monotonic time `[22,23]`, with the same policy and valid guard deadline. `begin_ingress` advances the in-memory floor to 22 and refuses because the durable floor remains 20. Repeating the call cannot converge. Clean close/reopen retains matching cached/durable floor 20, so reattachment does not refresh it; ingress refuses again. A fabricated local challenge cannot repair this receive-only scope because challenge issuance requires an append grant.

**Impact:** A legitimate replica-holder path becomes unavailable after normal time advancement. A policy update preserving replica-hold permission can produce the corresponding failure. This conflates independent replica-hold and append rights described in Design §3.3.

**Required correction:** Provide durable advancement of the provider-owned observation in the ingress path, using the existing selected-image/floor transaction discipline. If advancement changes the guard’s base selection, a bounded reload/retry must converge under a stable current cut. Consumption must still wait for durable guard establishment; uncertainty and revocation must remain fail-closed.

**Closure tests:** Use a genuine receive-only policy with empty append grants. Advance valid trusted time, then valid chained policy, and establish ingress without issuing a local append challenge. Test bounded retry convergence, restart retention, revocation and time-regression rejection, and observation-publication failure returning no consumable permit.

**Root classification:** New **nonarchitectural B integration root**. The trusted observation transaction already exists; ingress lacks the necessary call path.

## 2. Invariant analysis

The inspected implementation withstands several other concrete attacks.

Cryptographic facts derive from existing Ed25519 custom-domain verification and closed bounded bodies. Namespace validation checks complete declaration/schema/corpus identity. Fresh challenge/response custody and retained original query/facts prevent generic restored authentication success from becoming a new local seal. Historical admission verification retains original cuts rather than applying later expiry or revocation retroactively.

Native storage bindings decode and compare exact stage contents, receipt lists, preconditions, charges and qualified-fork facts. Recovery validates retained terminal/application coherence and replays actual pure-core callbacks before returning a usable session. Exact resolve requests authenticate against retained issuance custody. Existing attempt identity and immutable terminal handling resist changed-binding and contrary-callback attacks.

Disk custody uses stable OS locks and bound root identities. Publication keeps advanced issuance and guards in the floor intent, writes/synchronizes the inactive image, then replaces and synchronizes the selected floor before acknowledgement. Missing-next recovery preserves advanced metadata and active guards without inventing NonCommit. Intact next images require validation before selection.

The finite slots are physically written and checked for allocation. Ordinary writes preserve critical reserve, including native/callback history and active-guard loss requirements. Exhaustion refuses rather than collecting history silently. Genuine process-cut tests support these claims; structural fixture tests are separately identifiable.

Close retains ownership while ingress, preparation, queries, callbacks or unresolved attempts remain. A started unjoined receive cannot be abandoned. Failed observation/loss persistence retains the guard, and permanent loss survives later empty observations. The eligibility extension is nonmutating and retirement follows durable selection. Those protections do not resolve P2-1’s missing receive-result binding.

Default legacy paths remain intact. Fresh compatibility-marker refusal has actual old-executable evidence. Architecture/process-global checks and source checks inspect the affected adoption and disabled-platform branches; baseline formatting and Clippy debt are recorded honestly.

Accounting must remain attached to controlling objects:

| Object | New roots in this report | Retained accounting after classification |
|---|---|---|
| Semantic | None | Two architectural, one nonarchitectural; one merged remediation |
| Typed | P2-1: one architectural | Two architectural, five prior nonarchitectural; one completed remediation |
| Combined B | P2-2 and P2-3: two nonarchitectural | Zero architectural, two nonarchitectural; zero merged remediation so far |

No third architectural root is classified here. The previous typed R1–R6 closure deferral remains intact; this report does not claim their independent closure.

## 3. Risks and next action

Recorded durability remains LocalProcessRestart. Simultaneous trusted-floor/data rollback, machine power loss, automatic duplex transport and live-store migration are outside this qualification. Synchronous physical operations will also require appropriate scheduling when integrated into IC-3C. These limits are not additional findings.

The next action is a bounded merged remediation addressing P2-1–P2-3, preserving the accounting above and adding the specified genuine-host regressions. The corrected typed capability representation requires fresh independent review alongside the corrected B component; passing recorded tests alone cannot qualify the frozen revision.