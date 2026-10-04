# Independent CRDT storage-attempt typed contract — CODE-AXIS REVIEW

**Review object:** Redesigned internal typed-contract/allocation/compiling RED checkpoint, controlling DRAFT `dev-docs/GladeIndependentCrdtStorageAttemptContract.md`, at root `a696f0eef38614fe0cfe2a6b053c352470e800b3`.

**Baseline:**

- Workspace root: `a696f0eef38614fe0cfe2a6b053c352470e800b3`
- Glade: `3cf1fa79cd752012acd0d2ff66d595e293b3433c`
- Glial: `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`
- Glade-discover: `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`
- External Gyld: `95a426595bba8e248a5f484272e483a070c73918`

Sources were read directly and checked against exact-pin `git show`; scoped committed diffs were inspected.

**Date:** 2026-10-04

**Axis:** Architecture, interfaces, actual consumer composition and contract executability. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block; zero P0, P1 or P3 findings. I pre-commit to GO on a revision that resolves P2-1, P2-2 and P2-3 as specified, subject to verification of the corrected consumers and relevant evidence.

---

## 0. Evidence base

All five HEADs matched the prescribed tuple at start and end. Inspection-only commands were used; no writes, builds, tests, network actions or Git mutations occurred. No current peer report was accessed.

Read the complete canonical Code prompt, storage-attempt contract/evidence and lifecycle design; reviewed the completed design reports, accepted CRDT semantics, admission plan/resource requirements, historical IC-1 contract/evidence/remediation evidence, escalation, Safety-2, root classification and full ReviewCycle. Process sources included both AGENTS files, review-loop skill/template and QualificationPlan §6. Library/package policies, BuildEntry, DecisionLog GDL-054–058 and legacy seal coexistence were checked.

Source inspection covered:

- API `src/lib.rs:1–265`, development host `tests/support/mod.rs:1–676`, seventeen public journeys, six-mutant probe and syntax/shape guards.
- Core types, refusing `step`, all original 21 Records/six recovery consumers, thirteen STA consumers and shared driver.
- Actual Rust text trace and unchanged Glial ten-row released-Taut consumer/corpus.
- Manifests, twelve-member inventory/policy, both selectors, all-member ARCH002 positive/negative fixture and synthetic thirteenth-member regression.
- SnapshotStore, RecordsFile CAS/replace/bridge/decoding, sysdir ownership and Store open/append.
- Frozen external Gyld semantic/lifecycle declarations, ledgers, capture implementation and provenance-negative consumers.

Read-only comparisons found no pin mismatches across the five changed root files, nineteen changed Glade files, four changed Gyld files, sixteen controlling root documents and nine relevant retained Glade/Glial sources. Frozen lifecycle text equalled its canonical source-qualified document and hash; the semantic-ledger hash matched. Recorded RED/GREEN executions and timings were audited against source, not rerun.

## 1. Findings

### [P2-1] Core driver never supplies the host’s matching authoritative policy observation

**Location:** [core support](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/support/mod.rs:50), lines 175–178 and 260–267; [memory host](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs:367), lines 367–384 and 588–596.

**Invariant:** Contract §4 requires the producer’s independently injected current observation to match the plan and caller expectation before start. Valid local consumer fixtures must make that condition satisfiable.

**Sequence:** `initial` supplies policy bytes `granted-test-cut`, whose SHA256 is `4cf19c86…7fe5231f`. Neither `drive` nor `drive_with_port` injects a host observation. The host therefore uses `cut()`, whose policy is `[8;32]`. A correctly generated local plan and Begin expectation use the core policy digest; `current != observed` becomes true and the host returns terminal NonCommit.

Consequently, even a correct future kernel cannot produce the required receipt in `icd002_partitioned_admission_commits_without_a_holder` through this assembled fixture. The caller-owned-session regression has the same missing input.

**Impact:** Local-admission RED currently masks a separate broken test assembly; turning the kernel green alone cannot satisfy the advertised actual-port consumers.

**Correction:** Compose an explicit trusted policy/time provider with the session, initialize it consistently with the selected fixture state and deliver policy changes through that provider. Do not copy `BeginRequest.current` into authoritative observation.

**Closure:** Add a driver/port regression demonstrating matching injected observation permits the actual Begin journey and independently changed observation prevents it. Keep kernel admission assertions RED until authorized implementation.

**Root classification:** Non-architectural composition defect; the reviewed producer/consumer interface and intended trust separation are sufficient.

### [P2-2] Required multi-call text journeys discard their storage ledger between calls

**Location:** [fresh-session wrapper](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/support/mod.rs:175); [text trace](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/examples/text_admission_trace.rs:315), lines 54–64, 195–199 and 315–324.

**Invariant:** Contract §§3/6/8 require retained attempt mappings, application revisions, outcomes and issuance history across continuations. Section 8 explicitly requires multi-call journeys to retain a session or inject qualified recovery.

**Sequence, independently of P2-1:** In ABC, each first local call must return a receipt. That publication advances its host and kernel application revision to 1. The opposite-direction replica offer then calls `drive` again. It constructs a new empty host; preparation initializes that instance’s host revision to 0. A correctly bound subsequent plan expects revision 1, so Begin returns NonCommit. The second host also lacks the first attempt and outcome history.

The same wrapper is repeatedly used in AB-buffer/AD and post-fork continuations. Adding `drive_with_port` did not migrate these existing required consumers.

**Impact:** The retained-state convergence obligations cannot be satisfied through the delivered assembly without either fixing the harness or weakening revision/lifecycle checks. These are current mandatory consumers, not optional future scenarios.

**Correction:** Give each logical replica/journey a caller-owned session and preserve it throughout. Seeded recovery fixtures must provide explicit consistent trusted host recovery. Reserve `drive` for genuinely independent empty-host fixtures.

**Closure:** Add a public-port consecutive-publication journey requiring revisions 1 then 2 and retained first outcome; verify ABC and post-fork consumers use those persistent sessions. Preserve all ten text assertions.

**Root classification:** Non-architectural harness lifetime defect; no port or lifecycle redesign is required.

### [P2-3] Lookup fixtures demand consumption without the mandatory issued-request record

**Location:** [STA lookup setup](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/storage_attempt_contract.rs:81), lines 81–94; [four-kind recovery setup](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/recovery_contract.rs:197), lines 197–208; [original lookup setup](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/records_host_contract.rs:770), lines 770–775.

**Invariant:** Contract §3 makes `storage_invocations` the owner of outstanding full issued requests and requires consumption to move them unchanged into `retired_storage_invocations`. Section 5 requires authenticated issuance before terminal authority.

**Sequence:** These fixtures insert Inspect invocation 40 only into `lookups`, then inject its reply and require retirement or committed installation. `reserved` leaves `storage_invocations` empty; `register_attempt` records only Begin invocation 8. Inspect 40 therefore has no corresponding issued-request record.

A consumer enforcing the new contract must reject that setup; satisfying the success assertions instead requires treating the secondary lookup map as an undocumented alternative issuance authority. Malformed-terminal tests using the same setup can also obtain CallbackMismatch merely because issuance is absent, without testing their intended payload mutation.

**Impact:** Several mandatory RED obligations conflict with the new continuation grammar, and negative cases are nondiscriminating.

**Correction:** Register every injected lookup through actual emission or a complete trusted fixture helper that populates both maps and preserves finite history/counters. Do not relax issuance authentication.

**Closure:** Pair an issued valid lookup that retires and preserves its full request with the identical unissued callback that leaves state unchanged; run each terminal-field mutation from the issued fixture.

**Root classification:** Non-architectural fixture-precondition defect; existing state representation already supports the correction.

## 2. Invariant analysis

The original absence-before-publication attack fails against the contract and port model: Inspect remains Pending; commit/fence observations repeat one retained outcome; late Begin cannot rebase past NonCommit. Lost preparation acknowledgement retains plan identity, and Started cuts survive later revocation.

The traits have meaningful required lifecycle operations. Contract/Pure roles and minimal dependency direction are appropriate; no marker trait, runtime dependency or universal semantic-data package was introduced. The six probe mutants have discriminating assertions.

Scoped supersession preserves accepted CRDT semantics and failed-object history. Original test bodies/loop obligations and all ten genuine released-Taut rows remain present. Frozen Gyld allocation preserves source-qualified semantic ownership and adds explicit lifecycle ownership without engine expansion or satisfaction claims.

No new architectural root was established. The historical IC-1 three-root stop remains intact.

## 3. Risks and next action

The refusing kernel and deferred physical host are not missing delivered implementation. Simulation does not qualify physical fencing, restart, rollback, quotas, crypto or live synchronization. Recorded timing margin and initial TDD deviation remain disclosed.

Next action: one bounded correction of these three consumer defects, test-first with the kernel still refusing, followed by originating verification and the applicable review gate.