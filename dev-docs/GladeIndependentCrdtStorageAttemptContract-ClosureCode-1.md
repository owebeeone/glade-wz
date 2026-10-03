# Storage-attempt typed contract — focused CODE closure review, remediation 1

**Review object:** Merged remediation 1 of the DRAFT storage-attempt typed-contract/compiling-RED checkpoint, at workspace root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`.

**Baseline:**

- Workspace root: `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`
- Glade: `346d963f09089a0636a01fac8a257f067908147d`
- Glial: `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`
- Glade-discover: `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`
- External Gyld: `95a426595bba8e248a5f484272e483a070c73918`

**Originating review:** Root `a696f0eef38614fe0cfe2a6b053c352470e800b3`, Glade `3cf1fa79cd752012acd0d2ff66d595e293b3433c`.

**Date:** 2026-10-04

**Axis:** Originating Code counterexample verification. Independent, adversarial, inspection-only. This report covers Code P2-1, P2-2 and P2-3; it is not aggregate approval or the required fresh full review.

**Verdict: GO within focused scope** — all three originating Code findings are CLOSED for this typed-contract/compiling-RED checkpoint. No new blocking finding or architectural root was established in the focused inspection.

---

## 0. Evidence base

All five HEADs matched the prescribed remediation tuple at start and end. No files were written, tests/builds executed, Git state mutated, or current peer closure/fresh full reports read.

Read the complete RemPlan-1, Remediation1-Evidence, corrected Contract and original Code report. Inspected the nine-file Glade correction, actual development provider, fixture assembly and six closure consumers, migrated Records/recovery/STA consumers and every text-session call site.

Recorded execution evidence was audited against source, not rerun: API 26 GREEN; six assembly closures GREEN; Records 21, recovery six and STA 15 compiling behavioral failures against refusing `step`; exact ten text rows RED with the released-corpus positive control passing. Recorded compilation, lint, architecture, selection and process checks remain supporting evidence.

Independent read-only comparison confirmed the refusing kernel is byte-identical, SHA256 `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`. Assertion counts remain Records 52, recovery 55 and original STA 40; the two new STA consumers add six. Consumer diffs preserve the original assertions and loop obligations.

## 1. Originating finding dispositions

### Code P2-1 — CLOSED: authoritative policy observation is composed independently

The original sequence supplied core policy `granted-test-cut`, while the uninjected host fell back to `[8;32]`. Valid local Begin therefore encountered a different producer observation and returned NonCommit.

[FixtureReplica initialization](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/support/mod.rs:502) now constructs the explicitly seeded session and injects each fixture instance’s exact policy digest and time interval. Provider changes have a separate input path. The effect driver still passes Begin’s expectation to the port; it does not promote that expectation into producer authority.

The [closure journey](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/fixture_composition.rs:43) prepares and begins the original staged plan through the actual traits. It requires Committed with the original receipts and start cut. A separate replica independently changes its provider to policy `[9;32]` and time 22–23, then submits the unchanged original Begin and requires NonCommit at revision 0.

The host still compares caller expectation, staged cut and independently retained observation. This retraces both sides of the original counterexample. Recorded compiling fixture RED preceded its GREEN correction.

**Classification:** Original non-architectural composition root; no redesign required.

### Code P2-2 — CLOSED: logical replicas retain their storage sessions

Originally ABC’s first publication advanced host/kernel revision to 1, but the remote offer opened another empty host at revision 0. The expected-revision predicate necessarily failed; earlier mappings and outcomes also disappeared.

[ABC’s corrected calls](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/examples/text_admission_trace.rs:337) use the same left/right sessions for each local publication and subsequent opposite-direction offer. Buffering, rival orders, qualified-fork/fresh-origin/retry/refusal and isolation likewise retain one session per logical replica. There are zero text `drive(` calls and thirteen explicit retained-port calls. Original rival and post-fork Rust consumers received the same lifetime correction.

The [consecutive-publication regression](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/fixture_composition.rs:71) requires revisions 1 then 2, recovered revision 2 and retention of the first exact attempt/outcome.

Nonempty initial states now use explicit bounded `OwnedFixtureSeed` construction. The development provider retains immutable genesis custody bytes alongside recovery identities, reservations and outcomes. Seed validation rejects inconsistent issuance counters/indexes, unsupported revisions and terminal state lacking original receipt/accounting custody. The regression checks that seed custody survives both publications.

This resolves the empty-host pairing without bypassing revision checks. All ten text obligations remain RED against the actual refusing kernel.

**Classification:** Original non-architectural harness lifetime root.

### Code P2-3 — CLOSED: injected lookups carry authoritative full issuance history

Originally Inspect 40 existed only in the secondary `lookups` map. Success assertions demanded consumption or installation without the full issued request, while malformed-terminal tests could reject solely because issuance was absent.

[register_lookup](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/support/mod.rs:752) now validates owner, namespace, Inspect operation, complete retained attempt/binding, uniqueness, finite shared history and counter progression before inserting the exact request into both maps. Attempt setup retains consumed Prepare 7 and issued Begin 8.

The fixture regression requires full lookup 40, secondary-index equality and next counter 41; foreign namespace and exhausted capacity refuse unchanged. Existing injected-lookup and terminal-mutation consumers now start from this issued fixture. Restoration moves the original invalidated request into consumed full history.

The [paired kernel closures](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/storage_attempt_contract.rs:394) distinguish issued Pending consumption—with unchanged full-request retirement and retained uncertainty—from the identical unissued reply, which must preserve state and report CallbackMismatch. These consumers compile and remain intentionally RED; kernel consumption is not claimed implemented.

**Classification:** Original non-architectural fixture-precondition root.

## 2. Invariant analysis and root classification

The corrected assembly makes the original valid sequences satisfiable through the actual port while preserving independent policy authority, application revisions, original receipts, retained outcomes and full callback authentication. Fixture GREEN does not supply successful admission through an alternate model.

No new architectural root was established by this focused retrace. This report does not adjudicate the State finding concerning Recovery cardinality or supersede either object’s recorded architectural-root history.

## 3. Risks and next action

The development seed assumes trusted, already-owned test continuation. It does not qualify physical restart, ownership acquisition, clone exclusion, antirollback, storage encoding or durable fencing.

The three originating Code IDs may be recorded CLOSED. Fresh complete peer-blind Code/State review remains required because shared Recovery changed. Successful kernel implementation must await that aggregate gate; the deliberately refusing kernel remains the correct checkpoint here.