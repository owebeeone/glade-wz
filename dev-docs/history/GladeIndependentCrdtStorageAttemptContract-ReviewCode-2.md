# Redesigned CRDT storage-attempt typed contract — CODE-AXIS REVIEW

**Review object:** Internal typed-contract/allocation/compiling RED checkpoint; controlling DRAFT `dev-docs/GladeIndependentCrdtStorageAttemptContract.md` at root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`, after remediation1.
**Baseline:** Root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`; Glade `346d963f09089a0636a01fac8a257f067908147d`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources inspected directly, through scoped diffs and comparisons against pinned Git objects.
**Date:** 2026-10-04
**Axis:** Architecture, interfaces, call graphs, ownership and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block. No P0, P1 or P3 findings. I pre-commit to GO on a revision that resolves P2-1, P2-2 and P2-3 as specified, provided it introduces no new blocker.

---

## 0. Evidence base

Read the complete canonical Code-2 prompt, repository instructions, review-loop skill/template, lifecycle design and completed design reviews, current typed contract §§1–9, evidence, initial Code/State reports, merged remediation plan and remediation1 evidence. Checked the accepted admission semantics, resource profiles, boundary/package policies, build entry, decision history, qualification §6, complete review-cycle history and preserved failed-IC1 escalation/root classification.

Implementation inspection covered:

- Storage API `src/lib.rs:1–267`, provider `tests/support/mod.rs:1–787`, complete public conformance, mutant and source-boundary consumers.
- Core types `src/types.rs:1–324`, refusing kernel, support `tests/support/mod.rs:1–791`, complete Records `records_host_contract.rs:1–1017`, recovery `recovery_contract.rs:1–536`, storage `storage_attempt_contract.rs:1–436`, fixture-composition tests and text trace `examples/text_admission_trace.rs:1–377`.
- Complete released-Taut Glial consumer and compound-body guard; affected manifests, README, architecture inventory, selectors, architecture scripts and positive/negative framework fixtures.
- Persistence interface, RecordsFile publication `records_file.rs:90–281`, ownership `sysdir.rs:109–165`, legacy store `store.rs:185–350`, seal contract and coexistence limitations.
- Frozen Gyld semantic source/ledger, lifecycle declaration, capture host and provenance/negative tests.

Read-only comparisons matched all 21 changed Glade files, four changed Gyld files and 19 selected controlling root files to their pins. The original 21 Records and six recovery test names and assertion counts remain; the ten actual-Taut rows remain. Twelve package manifests match twelve classifications. Frozen semantic source, ledger and lifecycle-source hashes matched; refusing kernel and Glial consumer remain unchanged.

Recorded evidence reports 26 API GREEN tests, 11 core fixture/representation/source GREEN tests, 42 compiling core behavioral RED tests and ten text RED rows with their independent three-order control. These results were audited against source, not rerun. No builds, tests, writes, Git mutations, network or live actions occurred. All five HEADs matched at both review start and end. No current peer or closure report was read.

## 1. Findings

### [P2-1] A mandatory historical-recovery success fixture still supplies an unissued callback

**Location:** [records_host_contract.rs:972](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/records_host_contract.rs:972), particularly lines 976–979 and 1009–1015; support `register_attempt:436–471`; contract §§3, 5 and 8.

The delayed prior-cut test registers Prepare7 and Begin8, then inserts Inspect40 only into `State.lookups`. It never records that request in authoritative `storage_invocations`. Nevertheless, it requires the resulting `LookupResolved` to report the original `ExactRetry`.

A conforming kernel must reject this never-issued callback with `CallbackMismatch` and unchanged state. This mandatory success assertion therefore contradicts callback authentication. It also contradicts §8’s claim that every manually issued lookup uses `register_lookup`. Remediation migrated other fixtures but missed this one.

**Required correction:** Register Inspect40 through `register_lookup`, including its exact full request. Remove the premature `next_effect_id = 41` assignment so registration can advance the counter correctly.

**Closure test:** Preserve the issued delayed-commit recovery success after revocation; add or retain the otherwise identical unissued request’s rejection with unchanged state and reservations. Assert both authoritative and secondary indexes.

**Classification:** Nonarchitectural fixture defect; residual of the initial Code finding concerning lookup issuance. Existing interfaces already express the required history.

### [P2-2] Revision-mismatch finality fabricates an application revision and breaks authentic restoration

**Location:** [provider support:417](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs:417), especially lines 430–433; preparation 317–365; restoration 86–88.

The provider accepts a valid new Candidate binding with `expected_revision = 1` while the instance’s actual revision is zero. Begin detects the mismatch and produces terminal `NonCommit { revision: 1 }`, copying the expected revision instead of the unchanged application revision. Fence uses the same construction.

`recover()` then returns this authentic state as `Validated`, with current revision zero and terminal revision one. `MemoryHost::restore` rejects it as `Integrity`. No forged DTO, physical restart or alternate admission implementation is required.

This violates §4’s unchanged-revision NonCommit grammar and §6’s consistent validated restoration. The bounded trusted provider cannot faithfully demonstrate these obligations while generating states its own restoration rejects.

**Required correction:** Refuse an inadmissible revision binding before registration, or otherwise produce negative finality using the actual unchanged application revision while preserving the complete original binding. Preserve existing uncertain work when refusing.

**Closure test:** Exercise revision mismatch through public Prepare/Begin and Prepare/Fence, verify no fabricated revision, stable repeated terminal results and successful authentic recovery/restoration. Cover expected revision both above and below the actual revision.

**Classification:** Nonarchitectural provider implementation defect. The existing binding and terminal types can express the correct values.

### [P2-3] Reopening admits limits that cannot contain retained state

**Location:** [provider support:184](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs:184), especially 194–207; recovery 492–507; restoration 58 and 100–105.

Open checks several limits only for nonzero values. It does not validate retained bindings, history or reservations against the proposed limits before granting ownership.

A public-port reproduction is: prepare the ordinary nine-byte Candidate batch `exact:x:1`; fence it to NonCommit; close; reopen the same store/namespace under generation two with `max_batch_bytes = 1`. Open succeeds. Recovery declares the retained nine-byte binding `Validated` under the one-byte limit. Restoring that authentic recovery returns `Capacity`.

Shrinking retained-attempt, invocation-history or critical-reservation limits can likewise publish inconsistent validated recovery. This violates §6’s same-bound live/restored contract and retention requirements.

**Required correction:** Validate retained state and reservations against requested limits before changing ownership/floors, or retain the original compatible limits. Refuse incompatible reductions without deleting history.

**Closure test:** Close/reopen with each relevant bound below retained consumption; require refusal and unchanged retained state/floors. A compatible reopen must preserve outcomes, identities and reservations and produce a successful recovery/restoration round trip.

**Classification:** Nonarchitectural missing validation. No new interface or lifecycle abstraction is needed.

## 2. Invariant analysis

The absence-before-install attack is now represented by queued preparation and Pending observations. Inspect does not manufacture negative finality; fence establishes a retained terminal outcome, and subsequent resolution repeats it. Lost prepare acknowledgements retain plan identity. All four commit kinds participate in the lifecycle matrix.

Complete binding and echoed request identity remain explicit. The new independent invocation cardinality avoids interpreting sparse high-water values as retained-entry counts; sparse restoration coverage directly exercises that distinction. Callback retirement and retained consumption remain bounded.

Remediation provides a meaningful owned fixture continuation with retained custody and injected policy observations. The text driver now retains sessions across calls. These corrections address real producer/consumer assembly problems, although P2-1 leaves one required historical-recovery path inconsistent.

The refusing admission kernel remains intact. Fixture GREEN results do not claim domain acceptance or physical qualification. Original retention, receipt, fork and merge obligations remain, including genuine corpus payloads and post-fork AB rows. Package roles are explicit; the storage trait has lifecycle contracts rather than marker-only classification. Dependencies and architecture allowlists were not weakened.

Gyld preserves the frozen 31 allocations/124 obligations and adds the explicit 34/135 lifecycle overlay with Records/StorageAdapter ownership and provenance checks. Supplemental capture remains app-owned and source-qualified.

All three findings are nonarchitectural. They do not add an architectural root to this redesigned object; its previously classified cardinality root remains the one recorded architectural root. The failed IC1 object’s separate three-root history remains preserved.

## 3. Risks and next action

Physical fencing, submitted-I/O exclusion, crypto, durable publication, antirollback and live qualification remain deferred gates. Their absence is not a finding here. The provider remains `VolatileTest`. Recorded timing and test-first evidence retain their disclosed limits; this inspection establishes neither fresh performance measurements nor perfect historical TDD chronology.

The next action is one bounded, test-first remediation covering P2-1–P2-3, preserving the refusing kernel and all original RED obligations, followed by the required independent re-verdicts within the existing two-round limit.