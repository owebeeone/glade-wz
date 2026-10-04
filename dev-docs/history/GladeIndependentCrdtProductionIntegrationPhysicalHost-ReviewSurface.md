# Combined IC-3B physical-host/API consumer surface — SURFACE-AXIS REVIEW

**Review object:** Combined IC-3B physical-host/API cold consumer surface, including `IngressAuthority::can_settle` and `can_abandon`, at workspace root `db29535fe162356df93fbdc28c052185cfd9a0f7` and Glade `9dbc677a25d93af0c561817571ad7eb1990ddadc`. Controlling consumer guide: `dev-docs/GladeIndependentCrdtProductionIntegrationPersistenceUsage.md`, status B candidate.
**Baseline:** Workspace root `db29535fe162356df93fbdc28c052185cfd9a0f7`; Glade `9dbc677a25d93af0c561817571ad7eb1990ddadc`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `400cedcf1fff74128f366af758a0c389a23435d7`. Consumer documents were read through `git show db29535fe162356df93fbdc28c052185cfd9a0f7:path`.
**Date:** 2026-10-04.
**Axis:** Surface: cold discoverability, construction and defaults, lifecycle pairs, ownership, error handling, and compatibility expectations. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: GO** — No P0, P1, or P2 findings. One nonblocking P3 documentation finding concerning fresh-root legacy compatibility discoverability. This verdict qualifies the reviewed consumer documentation, not executable persistence or compatibility evidence.

---

## 0. Evidence base

I read the generated Surface prompt in full, the review-loop skill and its canonical prompt/report template, and applicable root/member `AGENTS.md` and `AGENTS_GWZ.md` instructions. The prompt’s narrower cold-consumer restriction governed the substantive review.

The complete authorized consumer-document set was read:

| Document | Lines | Material inspected |
| --- | --- | --- |
| `dev-docs/GladeIndependentCrdtProductionIntegrationPersistenceUsage.md` | 1–91 | Configured construction, fresh provisioning, exact trusted-floor reopen, narrower storage opening, local authentication and append, finite receive lifecycle, eligibility queries, persistence results, retry, close and restart limits |
| `dev-docs/GladeIndependentCrdtProductionIntegrationTypedUsage.md` | 1–121 | Package placement, explicit configuration and limits, default refusal, generic receive ownership, settlement/abandonment, shutdown handoff, native storage operations, combined close, representation versus trusted evidence |
| `dev-docs/GladeIndependentCrdtProductionIntegrationAuthentication.md` | 1–27 | Genuine provider construction, injected policy/time, fresh possession, one-use local verification, historical evidence, custody and qualification boundaries |

Line references below refer to those pinned document bytes, numbered with `nl -ba`. I also performed a targeted search of the pinned persistence guide for legacy compatibility, markers, downgrade, provisioning, defaults and close terminology. Its only marker description concerns permanent receive loss; it contains no fresh-root legacy compatibility explanation.

`git rev-parse HEAD` was run at START and END for all five repositories:

| Repository | START and END result |
| --- | --- |
| Workspace root | `db29535fe162356df93fbdc28c052185cfd9a0f7` |
| Glade | `9dbc677a25d93af0c561817571ad7eb1990ddadc` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

Every head matched the requested tuple and remained unchanged.

No implementation source, controlling design/plan, evidence artifact, original review or current peer report was read. No files were written, builds/tests run, network requests made, or live actions taken.

The persistence guide expressly states that B adds no CLI, binary ingress route or configuration-file format (line 3). Consequently there was no new helper CLI whose help needed inspection. The walkthrough was performed by following documented Rust API calls and outcomes.

## 1. Findings

### [P3-1] Fresh-root legacy compatibility refusal is absent from the consumer guide

**Root classification:** New nonarchitectural B consumer-documentation root. This is not a semantic or typed-contract counterexample, and it does not assert that the physical compatibility mechanism fails.

**Location:** `PersistenceUsage.md` lines 39–45, describing creation and reopen, and lines 81–91, describing release, restart and qualification. The complete authorized consumer-document set contains no explanation of the fresh disposable-root compatibility marker or the expected old-executable refusal named in this review’s scope.

**Violated invariant:** A cold physical-host consumer must be able to distinguish an intentional compatibility refusal from a recoverable ownership, floor, integrity or configuration failure. Provisioning documentation should identify the compatibility consequence attached to roots it creates.

**Reproduction by cold inspection:**

1. Follow the guide to provision a fresh disposable root with explicit identity, registration and trusted floor.
2. Complete work and obtain `Closed`.
3. Consider reopening those same roots using a retained older executable.
4. Consult the documented reopen and failure guidance to determine whether that executable is eligible, what the compatibility marker means, and whether removing a marker is a supported repair.

The documents explain wrong/missing floors, foreign ownership, competing locks, path substitution and rollback. They provide no answer for this compatibility case. The consumer must obtain the answer outside the permitted cold surface.

**Impact:** An expected compatibility refusal is harder to diagnose. A consumer may spend time treating it as an ordinary reopen configuration failure or seek an unsupported marker-removal workaround. The existing instructions to preserve unknown roots substantially limit the risk, so this is P3 rather than a blocking correctness finding. I have not witnessed or claimed an actual destructive retry.

**Required correction:** Add a short fresh-root compatibility paragraph alongside provisioning/reopen. It should identify the relevant marker and its ownership, state the supported compatibility direction and expected legacy refusal, and explain that deleting the marker does not establish a qualified downgrade. Keep this narrowly about newly provisioned disposable roots; do not imply existing-store migration, sealing or deployment activation.

**Closure/regression check:** A cold reader using the revised consumer documents must be able to answer the four questions in step 4 without source or evidence access. The lane owner should also ensure that the documented marker/refusal description agrees with the separately qualified old-executable witness. No additional architectural mechanism is requested.

## 2. Invariant analysis

### Construction and defaults

The first-day path has a discoverable entry point: the typed guide links to the persistence guide, which names `NodeAssembly::independent_configured` and demonstrates obtaining `DiskSession<P>` through `ReplicaRecoveryHost::open_replica`.

The package names correspond to their documented responsibilities: owned data, evidence contracts, recovery contracts and structural codec. The configured adapter belongs under the node’s `independent` surface. The unchanged default assembly is repeatedly described as refusing. I found no documented switch that silently turns the existing binary into a writable independent replica.

The three configured inputs have explicit responsibilities: `EvidenceConfig`, a `PolicyClock` implementation and `DiskRootConfig`. The guides require explicit keys, identities, policy/time, root paths, registration, capacities and limits. They state that public configuration inputs have no `Default`; development identities and paths are not production defaults. This defeats the attack of treating an omitted option as permission to choose a key, environment-derived path, clock or storage strength.

The guides describe configuration meanings rather than furnishing a complete signing application. That is consistent with the stated owned Rust API scope. It does not promise a runnable command-line setup or enrollment workflow.

### Provisioning and reopen

The two lifecycle branches are explicit and materially different.

For fresh provisioning, `provision=true` requires fresh roots and the zero expected floor. Existing or partial roots refuse. A lost creation reply does not authorize replacement identity or fresh reset.

For reopen, `provision=false` preserves root paths, registration, owner, plan/invocation namespaces and signed limits. The expected floor is the digest of the exact complete bytes of the independently trusted `replica.floor`, read within the signed bound. The guide expressly excludes digesting a re-encoded public `ReplicaFloor`, a peer value or a cached data image.

The narrow `StorageAttemptHost::open(OpenRequest)` path is separately explained. It accepts owner, invocation namespace and exact storage limits; its configured host already owns the remaining signed inputs. It derives a plan namespace only on fresh creation. Reopen restores the retained namespace under the root locks, including roots initially created by `open_replica`. The absence of caller plan-namespace and expected-floor fields is explained, rather than left as an undocumented reconstruction convention.

The single-use host/provider lifecycle is also explicit: a new qualified attempt requires a fresh host/provider after an open attempt. A session retains its roots until successful close. The documentation does not authorize silent provider or owner replacement.

### Authentication and append

The authentication guide supplies the steps omitted from the persistence guide’s shorter overview: trusted channel/session binding, fresh nonzero challenge/session nonces, the node-signed challenge, the client’s exact signed response and authentication on the same provider.

The text consistently distinguishes possession, resource permission and durability. A challenge is not append permission; transport authentication is not resource permission; a seal is not a receipt. `submit_local` follows fresh local authentication. Exact retry retains the original receipt and charges while still requiring fresh possession.

The stated storage promise is `LocalProcessRestart`. Quorum durability, power-loss assurance and arbitrary rollback protection are excluded explicitly. Historical custody cannot be read as fresh current authorization.

### Receive lifecycle and eligibility

The receive sequence exposes both its normal and exceptional lifecycle halves. A successful durable `begin_ingress` precedes application receive/parse construction. `receive_ingress` validates the permit before invoking the factory. The owned continuation returns the permit only with a drained result; spawned carrier work must be joined.

The guide gives distinct outcomes:

- Complete authenticated inventory: settle the exact observation and checkpoint.
- Drained partial, erroneous or unclassifiable receive: retain permanent loss with the owned drained permit and bounded available prefix.
- Never-started permit: abandon.
- Started, cancelled or orphaned receive: never use never-started abandonment.

This defeats treating a timeout, dropped future or empty later inventory as drain or evidence of absent history.

The new utility queries have a clearly stated role. `can_settle` and `can_abandon` are nonmutating `Result` eligibility checks. Foreign and closed capabilities refuse; settlement requires drain; abandonment requires never-started work. Host implementers check before disk I/O and retire utility status only after committed durable selection. Concrete disk consumers do not manually manipulate that authority.

The naming supports this distinction: “can” asks eligibility; settlement/abandonment completes the transition. I found no cold instruction equating a successful query with persisted settlement.

### Refusal, uncertainty and retry

The result table separates `Committed`, pre-I/O `Refused`, `Unknown` and unavailable recovery. A consumed permit/observation remains host-owned after refusal; `Unknown` preserves the registered scope and exact continuation.

`retry_ingress(&guard_id)` is explicitly limited to a continuation retained by that live session. `record_observation` can correct a pre-I/O checkpoint refusal only for the exact drained observation/permit still held by the host. Changed bytes and fresh unguarded observations refuse. These instructions do not suggest fabricating replacement capabilities or erasing uncertainty through a new checkpoint.

Validated recovery is distinguished from successful structural decoding. `reconstruct_cut` retains active guards, loss, missing obligations and sticky kernel uncertainty. A later successful or empty inventory cannot establish that older unknown history never existed.

### Close, release and restart

The explicit trait-qualified close call resolves the two inherited `close` names. Both are required to enforce the same combined lifecycle.

The first close stops fresh input. `Pending` retains the session and both root locks while original work remains. The consumer can continue already-owned callbacks and cooperative cancellation/drain/classification, settle the exact result or loss, and close again. Only `Closed` permits clean release.

This is a discoverable shutdown pair, not an implied “drop to close” convention. The guides deny that abort without joining is a fence or that ending a process is clean closure.

After receive-time SIGKILL, reopen retains the orphan active guard and incomplete state. The documented provider does not promise a reset or eventual completeness. That honest limitation is distinct from a missing cleanup command. The review did not re-litigate automatic exchange coordination or existing-store recovery mechanisms deferred to later scope.

## 3. Risks and next action

The review establishes documentary discoverability only. It does not establish actual disk ordering, compatibility refusal, Unix restart behavior or correspondence between API documentation and implementation. Those require the separately authorized executable review and evidence.

Recorded controlling-object accounting remains unchanged: the semantic object retains two architectural and one nonarchitectural roots with one merged remediation; the typed object retains one architectural and five nonarchitectural roots with one completed remediation and owner-deferred standalone closures. This review adds zero architectural roots and one nonarchitectural B surface-documentation root. It neither resets those objects nor closes their deferred findings.

The next action is to file this report verbatim and include P3-1 in the lane owner’s documentation follow-up. This Surface verdict introduces no blocking remediation round and supplies no authorization for remote exchange, live activation or existing-store migration.