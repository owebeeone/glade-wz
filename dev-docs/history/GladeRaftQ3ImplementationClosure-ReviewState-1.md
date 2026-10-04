# Q3 implementation remediation 1 closure — STATE-AXIS REVIEW

**Review object:** Focused originating State closure on corrected root `ce0876423e921cdd066106af9190ae7c6ec5d781`, including remediation diff from `b61197602e5594bdf89770bda069ce7d30fdb222`. Q3 implementation acceptance remains pending the separate fresh gate; no production activation.

**Baseline:** Original implementation `b61197602e5594bdf89770bda069ce7d30fdb222`; corrected root `ce0876423e921cdd066106af9190ae7c6ec5d781`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Pinned documents and original-to-corrected diffs were inspected; working-tree source reads had an empty scoped diff.

**Date:** 2026-10-03.

**Axis:** Durable-state semantics, restart legality, publication ordering and original counterexample closure. Independent, adversarial, read-only. Parallel reviews run separately; nothing here relies on their current reports. Filed verbatim by the lane owner.

**Verdict: GO** — originating State P2-1 and P2-2 are closed; no new P0–P3 finding or architectural root cause identified in this focused closure. This verdict does not replace fresh full-scope Code/State renewal.

---

## Prior-finding closure table

| Original State finding | Disposition | Independent closure evidence |
| --- | --- | --- |
| P2-1: stale first voter permanently strands restart | **CLOSED** | Executed the exact partitioned joint-exit/reopen regression. Fresh voter 3 becomes leader; complete original application/configuration receipts and Entry bytes survive; home 2/generation 1 remain; new mutation, repeated physical restart and genuine quorum loss are checked. |
| P2-2: future checkpoint silently succeeds without overlap validation | **CLOSED** | Executed both actual-session future-checkpoint variants. Conflicting acknowledged history returns Quarantined; nonconflicting unsupported future cut returns InvalidImage. Exact files, state, receipt and Entry remain unchanged; same-history compaction and physical restart succeed. |

The original reports and merged RemPlan-1 were legitimate prior-round inputs. No current fresh prompt/report or other originating closure prompt/report was read.

## Changed-range analysis

The complete original-to-corrected diff contains scoped host corrections, five new owning test modules, evidence/remediation documentation and historical review filing. Its substantive changes are:

- `session/lifecycle.rs:76–117` selects an available eligible incoming voter by actual durable last-log `(term,index)`, using the checkpoint when the suffix is empty and lowest ID only for equal-log ties. It retains local completeness, membership and terminal-term checks.
- `session/port.rs:199–222` restores the candidate and compares common committed history against every retained node before recipient selection. Empty recipient selection now refuses explicitly.
- `voter.rs:21–53` centralizes stopped-voter handling for store errors and malformed successful publication responses. The prior cached state is retained; the exact error is recorded.
- `session.rs:110–184` suppresses delivery from stopped sources and to stopped targets, and excludes stopped sources from snapshot-result callbacks.
- New tests exercise original counterexamples, direct-install/CatchUp publication faults, malformed success, coherent foreign checkpoints and correctly versioned configuration admission errors.
- The shared snapshot specification adds an accurate comment distinguishing its outer-only binding mismatch from the coherent foreign witness.

These changes modify shared mutation/failure and routing call paths. Fresh renewal is therefore appropriate under review-loop §5, even though public contracts and package boundaries remain unchanged.

Scoped comparison confirms no change to V2 disk implementation, deterministic Machine, recovery codec, carrier grammar, Storage, crash runners/workers, manifests, lockfile, architecture policy or controlling contracts. No dependency/classification relaxation, exception, interface amendment, platform expansion or test-selection weakening occurred. I found **no new architectural root cause**. This is remediation round 1 of the implementation object.

## 0. Evidence base

The complete canonical closure prompt was read before review. Prior context retained root instructions, AGENTS_GWZ, review-loop authority, architectural allocation and controlling contracts. For this closure I inspected both initial reports, RemPlan-1, corrected ImplementationEvidence, the complete substantive diff and all five added test modules.

Important inspected ranges include `restart_freshness_tests.rs:1–218`, `future_install_tests.rs:1–110`, `install_fault_tests.rs:1–357`, `foreign_checkpoint_tests.rs:1–127`, and `configuration_admission_tests.rs:1–130`. Corrected publication, routing, campaign and installation ranges are identified above. Original Raft vote freshness and commit-by-vote behavior remain the source basis for the election counterexample.

All Cargo commands used the specified cached PROTOC and locked/offline manifest.

| Executed verification | Result |
| --- | --- |
| Proof library filter `q3::session::restart_freshness_tests` | 2 PASS |
| Proof library filter `q3::session::future_install_tests` | 1 PASS |
| Default fixture workspace | 120 PASS; zero failures; four explicit tier ignores |
| Original shared Q3 consumers | 19/19 PASS, zero ignored or filtered |
| Explicit ignored Q2 library tier | 2 PASS |
| Architecture/source/format/process-global gate | PASS; 49 Rust files, zero exceptions |
| Workspace/all-target Clippy, warnings denied | PASS |
| Q2 oracle self-test | 4 PASS |
| Q3 oracle self-test | PASS; ten recovered-field mutations plus missing/changed parent originals rejected |
| Actual Q2 SIGKILL | `write-ack`, `write-cut`: PASS |
| Actual Q3 SIGKILL | `ack`, `joint-before-apply`, `snapshot-before-apply`: PASS |

Cargo `--no-run` reported `process_crash-304030f150fc974e` and `q3_process_crash-57c67e4971c8507c`; those exact workers were used. The full workspace also executed the additional fault/admission/foreign tests.

All four HEADs matched at start and end. Final scoped working-tree diff was empty. No source/report writes or Git mutations were performed. Reported historical behavioral RED observations were inspected as evidence, not independently rerun on historical source.

## 1. Findings

No open originating State findings and no new findings in this focused review.

## 2. Invariant analysis

**P2-1 original counterexample.** The regression reproduces home-2 Create, accepted learner admission and genuine catch-up, joint incoming `[2,3,4]`/outgoing `[1,2,3]`, disconnection of 2, and leave committed through 1/3/4. It establishes that node 2 remains at the joint log cut and reconnect cannot repair it after leader 1 is removed.

It then drops the session and physically opens all four files. Selection ranks fresh logs correctly and campaigns node 3 through RawNode. Assertions compare original create, learner, joint and leave receipts and original Entry bytes, including typed replay. New mutation succeeds without changing home/generation. Another physical Restart retains original outcomes. Isolating two current voters then yields unknown for new work and preserves payload. The removed voter stays outside subsequent application authority.

The second regression excludes a disconnected freshest voter and elects the available equal-log candidate. This prevents availability filtering from recreating the first-ID trap. Campaigning remains explicit; no term or applied-counter assignment manufactures eligibility.

**P2-2 original counterexample.** Two actual sessions produce A’s payload11@2 and B’s payload99@2 plus index3. B’s checkpoint passes internal restore. A now checks overlap before recipient filtering and returns Quarantined. The nonconflicting payload11 variant returns InvalidImage because no local compaction publication is possible. Both variants verify byte-identical files, unchanged view/resources, retained receipt and original Entry/replay. Valid same-history installation persists checkpoints and survives physical restart.

**Complete matrix preservation.** The required rows remain exercised against actual providers:

| Contract §7 row | Closure assessment |
| --- | --- |
| V2 lifecycle/compatibility | Actual create/open, locks, missing/empty/Q2/wrong binding and coherent foreign refusal remain covered. |
| Learner and dual quorum | Actual catch-up, incomplete vote/pre-vote/restart admission, both joint-majority failures and replica-identical home refusal pass. |
| Configuration interruption | Pending intent, lost reply, committed-before-apply recovery, typed original accepted/refused/configuration/noop replay remain intact. |
| Snapshot semantic adversaries | Full-history replay and materialized-map comparisons remain unchanged and passing. |
| Snapshot plus suffix | Actual Ready installation, separate fast-forward/rejection, learner-add cut plus later suffix and legitimate uncommitted overwrite pass. |
| Publication faults | Existing Ready/LightReady faults pass; new direct-install/CatchUp tests cover all five faults and stopped-source routing. |
| Actual termination | All original external cuts pass against parent-held complete originals or independently specified expected result. |
| Bounds/rollback | Whole-frame refusal, parser adversaries, term guards, historical vote and independent-floor/no-floor honesty remain passing. |
| Combined acceptance | Actual snapshot/suffix, joint recovery, original retry, current policy/retirement and explicit exit remain passing. |

Central publication failure handling now covers direct callers as well as Ready processing. Unknown errors and malformed success stop participation before memory receipts/resources or queued source influence can escape. Reopen yields exact prior/full evidence or torn-record quarantine; errors are not reinterpreted as noncommit.

## 3. Risks and next action

The accepted experiment still has append-only physical retention, complete-prefix startup costs, a 16 MiB refusal limit and rollback detection requiring independent trusted state. Manual campaigns are not automatic-election certification. SIGKILL keeps the kernel/storage stack running and does not establish power-loss durability.

No production transport, cryptography, failure-domain, legacy/effect exclusion or activation conclusion follows.

The next action is to merge this originating closure with the independently completed fresh Code/State verdicts on the same corrected tuple. My two original State blockers are closed; full implementation acceptance remains the separate gate’s responsibility.