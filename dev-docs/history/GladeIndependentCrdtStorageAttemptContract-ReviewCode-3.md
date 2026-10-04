# Redesigned CRDT storage-attempt typed contract — CODE-AXIS REVIEW

**Review object:** Originating full Code re-verdict after remediation2; controlling DRAFT `dev-docs/GladeIndependentCrdtStorageAttemptContract.md` at root `fb6d69ff5153865266782f0b5c11d7a00086552c`. Internal typed-contract/allocation/compiling RED checkpoint only.
**Baseline:** Root `fb6d69ff5153865266782f0b5c11d7a00086552c`; Glade `c6c4239beecb129aa0585fe74006cc287dff3b87`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources inspected directly, through scoped Git diffs and comparisons against pinned objects.
**Date:** 2026-10-04
**Axis:** Architecture, interfaces, call graphs, ownership and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — Code-2 P2-1, P2-2 and P2-3 are closed for this checkpoint. No new P0–P3 finding. This verdict accepts neither successful admission behavior nor physical storage qualification.

---

## 0. Evidence base

Read the complete Code-3 canonical prompt, RemPlan-2, Remediation2-Evidence and corrected Contract §§1–9. Retained the preceding full independent audit of repository instructions, review-loop authority, accepted lifecycle/admission designs, package/testing policies, historical failed-IC1 record, original consumers, production coexistence and frozen Gyld allocation.

Inspected the complete correction range:

- Root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539..fb6d69ff5153865266782f0b5c11d7a00086552c`: controlling contract changes, remediation documents, review-cycle additions and member-pin/integrity updates.
- Glade `346d963f09089a0636a01fac8a257f067908147d..c6c4239beecb129aa0585fe74006cc287dff3b87`: exactly four development source files.
- Complete provider `crdt-storage-attempt-api/tests/support/mod.rs:1–867`, all added public journeys `tests/public_contract.rs:599–854`, corrected historical consumer `records_host_contract.rs:914–1058`, added fixture guard `fixture_composition.rs:256–269`, and unchanged registration helper `tests/support/mod.rs:752–791`.

All four changed working sources matched their pinned Git objects. Read-only comparison confirmed:

- Shared API unchanged, SHA256 `86cf4baf80aa532848a9a016e3c18a34a8896fd39a266919b84cf24ca06ac03d`.
- Refusing kernel unchanged, SHA256 `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.
- All original kernel test names retained: Records 21→22, recovery 6→6, storage-attempt 15→15. Assertion counts are respectively 52→61, 55→55 and 46→46.
- Multiline lookup search found ordinary insertion only in `register_lookup`; the remaining secondary-only fixture intentionally requires corruption refusal.

Recorded final evidence reports API29 GREEN, core fixture/representation/source12 GREEN, selected compilation/fmt/clippy/architecture/source checks GREEN, and all43 kernel consumers plus ten actual-Taut rows compiling assertion RED. The canonical corpus control remains GREEN. I audited these records against source and did not execute them. Chronology explicitly distinguishes corrected test-authoring/compiler errors from settled behavioral RED.

No builds, tests, writes, Git mutations, network or live actions occurred. All five HEADs matched at review start and end. No current peer re-verdict was read.

## 1. Findings

No open or new findings.

The originating closure determinations are:

| Code-2 finding | Retraced correction and regression | Disposition |
| --- | --- | --- |
| **P2-1 — Unissued historical lookup required to succeed** | [Records consumer:975](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/records_host_contract.rs:975) now registers Inspect40 through `register_lookup` before advancing issuance. It asserts the secondary index, full authoritative request and next41. The original post-revocation `ExactRetry` assertion remains. The added test at1024 sends the identical reply first without issuance, requiring unchanged state and `CallbackMismatch`, then with issuance, requiring the original receipt. The source guard detects the precise old multiline insertion. These kernel assertions deliberately remain RED. | **CLOSED.** Fixture consistency is repaired without implementing callback handling or weakening authentication. Nonarchitectural. |
| **P2-2 — Revision mismatch fabricated NonCommit revision** | [Provider:397](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs:397) compares a new binding’s expected revision with retained actual revision before queued/Reserved exposure. Existing exact-plan deduplication and unresolved-owner checks retain precedence. The public regression at600 covers all four kinds, ahead/stale expectations, Begin/Fence follow-ups and held/unheld preparation:32 combinations. It checks unchanged attempts, queue, revisions and attempt floor; authentic restore; original terminal repetition; and valid revision2 publication. | **CLOSED.** The original current0/expected1 sequence now refuses preparation, so its fabricated terminal cannot be produced. Existing uncertainty survives. Nonarchitectural. |
| **P2-3 — Reopen accepted incompatible retained-data limits** | [Provider:236](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs:236) validates a cloned retained image under proposed bounds before changing ownership/floors, then checks genesis custody, observations and existing critical reservations. The public regression at792 exercises nine reductions: batch, receipt, window, name, per-attempt critical, aggregate critical, attempt, invocation-history and instance bounds. Each refuses, preserves the old image, and permits a compatible retry at the same generation with retained outcomes/floors and authentic restore. | **CLOSED.** The original nine-byte/one-byte reopen no longer creates inconsistent Validated recovery. Nonarchitectural. |

## 2. Invariant analysis

The revision guard is placed after complete retained-plan matching and unresolved-owner exclusion. It therefore cannot replace a retained attempt, reinterpret a changed binding, or discard uncertain work merely because a later request has a different expected revision. Rejected invocations may consume their finite issuance entry, as the corrected contract explicitly states; they create no application revision or terminal outcome.

Restoration additionally checks unique complete AttemptIds, one unresolved owner per instance across attempts and queued preparations, unresolved revision agreement, unique plans and the combined current/queued instance bound. Eight negative image mutations discriminate these cases. The positive control preserves multiple historical terminals plus current X while independent Y publishes. These checks reject inconsistent images rather than reconstructing or deleting history.

Open performs its potentially failing validation before assigning critical consumption, store/namespace, owner floor or active owner. Reservation reductions cannot silently release existing consumption. The same-generation retry demonstrates that failed validation does not consume ownership.

The historical callback correction now satisfies the authoritative issuance boundary. Secondary lookup entries remain indexes, not authentication. Complete request ownership, namespace, operation, attempt and binding checks remain unchanged.

No architectural/interface premise invalidated originating re-review. The only member changes are provider validation and consumer regressions. Shared DTOs/traits, package roles, dependencies, selectors, budgets, allowlists, wire/platform assumptions, assembly seams and application publication boundary are unchanged. The internal call from open to the existing validator introduces no new ownership or mutation abstraction.

The prior full audit’s lifecycle conclusions remain applicable: temporary absence stays Pending, lost preparation retains identity, fencing and publication repeat one retained terminal result, and Started preserves its historical cut. Original27 obligations, all42 previous kernel tests and exact ten corpus rows remain. Frozen Gyld allocation/provenance and twelve-package architecture inventory are unchanged.

All three closed Code findings remain **nonarchitectural**. No new architectural root was found. The redesigned object retains its recorded count of one architectural root and remediation round2; neither count nor cap is reset. The failed IC1 object’s separate three-root stop remains historical authority. This report does not self-close the other reviewer’s findings.

## 3. Risks and next action

The provider remains a bounded `VolatileTest` model. Physical publication, external-I/O fencing, restart/antirollback, genuine proof domains and live qualification remain later gates. The kernel still reports Unavailable; this GO provides no successful admission evidence.

The next action is to combine this originating Code GO with the independently filed State re-verdict on the same settled tuple. Only the required aggregate gate outcome can authorize the next kernel implementation tranche.