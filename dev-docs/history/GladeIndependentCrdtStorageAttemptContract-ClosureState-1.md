# Storage-attempt typed contract — STATE focused closure 1

**Review object:** Remediation 1 of `dev-docs/GladeIndependentCrdtStorageAttemptContract.md`, DRAFT at root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`.

**Baseline:** Glade `346d963f09089a0636a01fac8a257f067908147d`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Original reviewed root: `a696f0eef38614fe0cfe2a6b053c352470e800b3`, Glade `3cf1fa79cd752012acd0d2ff66d595e293b3433c`.

**Date:** 2026-10-04

**Axis and scope:** Originating State reviewer, with original context retained. Independent, read-only verification of State P2-1/P2-2 only. No current closure or fresh full peer report was accessed. Filed verbatim by the lane owner.

**Verdict: GO within focused closure scope** — State P2-1 and P2-2 are independently verified closed. This is not aggregate approval of the changed shared interface or the complete remediation object.

---

## Prior-finding closure table

| ID | Disposition claimed | Independently verified evidence | Status |
| --- | --- | --- | --- |
| State P2-1 | Preserve actual invocation cardinality independently of identity high-water | Shared Recovery carries both fields; producer exports and restores them separately; sparse-ID regression reproduces the original sequence and tests equal capacity through exhaustion | CLOSED |
| State P2-2 | Configure the producer from trusted fixture policy/time, independently of Begin expectations | Actual driver constructs configured FixtureReplica; public-port regression publishes the core fixture’s original receipt and rejects the same stale Begin after an independent provider change | CLOSED |

## 0. Evidence base

All five HEADs matched the stated tuple at both start and end.

I read the complete RemPlan-1, Remediation1-Evidence, corrected controlling Contract and original State report. Inspection covered the scoped Glade diff, Recovery declaration, provider restoration/invocation/recovery paths, added public regressions, fixture assembly and composition tests, actual effect driver and relevant persistent text-consumer call sites.

Read-only exact-pin comparisons confirmed all nine changed Glade files match the settled revision. The controlling Contract, remediation plan and evidence also match their root pin. The refusing kernel remains byte-identical to the original checkpoint, SHA256 `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.

No tests, builds, writes, network actions or Git mutations were performed. Executed results below are recorded drafter evidence audited against source, not reviewer reruns.

## 1. Exact counterexample verification

### State P2-1 — sparse invocation restoration

API `src/lib.rs:220–231` now retains `invocation_count` separately from `invocation_high_water`. Provider `tests/support/mod.rs:39–52` validates count against history capacity, scalar floor and minimum retained preparation/attempt consumption. Restoration at lines 119–121 assigns each retained value to its corresponding ledger field. Recovery at lines 497–506 exports the actual count.

Retracing my original sequence: prepare Candidate with invocation 128 under history limit 128. Live invocation handling at lines 265–277 increments the count once, producing count one and floor 128. Authentic restoration therefore preserves count one rather than manufacturing count 128. Stale invocation 128 fails identity validation before consuming capacity. Fresh inspect 129 consumes the second entry and returns Pending; fence 130 consumes the third and installs terminal NonCommit. The original unfenceable recovery stop is removed.

`public_contract.rs:476–529` executes this exact public-port sequence, compares the live and restored branches, and checks retained terminal results for invocations 131–255. These consume exactly 128 total entries: prepare, inspect, fence and 125 subsequent observations. Invocation 256 then returns Capacity in both branches. Lines 532–555 add corrupt count/floor/history-limit negatives.

Remediation evidence records the original compiling sparse regression failing with restored Capacity versus live Pending before the correction, followed by the corrected API suite’s 20 public journeys passing. Source assertions discriminate the original defect.

**Closure:** CLOSED. Its architectural classification remains historical; correction does not erase that root.

### State P2-2 — mismatched trusted policy observation

Core `tests/support/mod.rs:175–177` now constructs `FixtureReplica` instead of opening an unconfigured empty host. `FixtureReplica::new`, lines 502–513, injects each explicit fixture instance’s policy digest and policy-time interval into the producer.

Retracing the original isolated-local-edit fixture: initial State still uses `granted-test-cut`; staged local intent still binds its digest and interval `[20,21]`. The configured producer now observes that same trusted fixture cut. A correctly emitted Begin can satisfy both the immutable precondition and the independent producer observation.

The host’s validation remains intact at API provider lines 410–428: caller expectation must equal both the plan cut and the producer’s observation, and the complete interval must fit every permit window. The driver’s Begin arm, core lines 265–266, simply executes the request; it does not copy caller data into authority.

`fixture_composition.rs:43–68` prepares and begins the exact core staged plan through the actual port. It checks the original receipt list and retained start cut. A separate fixture then receives independently injected policy `[9;32]` and interval `[22,23]`; the unchanged stale Begin must produce NonCommit at revision zero. This rejects a correction that merely trusts or echoes Begin.

Remediation evidence records compiling fixture failures before successful assembly behavior, then six composition closures GREEN. Original kernel/domain assertions remain RED, with the exact ten released-Taut rows and corpus control preserved.

**Closure:** CLOSED. Classification remains non-architectural fixture wiring.

## 2. Changed-range analysis

The shared production API change is the cardinality field. Fixture changes also address the merged plan’s session lifetime, explicit seed and issued-lookup work; those are legitimate context, but this report does not adjudicate the other reviewer’s finding IDs.

No additional architectural root was established during this focused verification. The existing architectural count is preserved. Model restoration and owned fixture seeds still do not qualify physical restart, ownership floors, clone exclusion or antirollback.

## 3. Risks and next action

Both original State counterexamples are closed at the typed/development-fixture tier. Physical storage finality, cryptography and successful admission behavior remain unqualified.

The next action is completion and merge of the separate fresh full Code/State reviews at this tuple. Successful kernel work must remain stopped until the required aggregate gate passes.