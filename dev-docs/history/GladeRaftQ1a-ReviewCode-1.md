# Glade Raft Q1a remediation1 — CODE-AXIS REVIEW

**Review object:** Scoped remediation at root `31bbea0cf1da3c6ae437cf482cb744d561693c08`, controlled by `dev-docs/GladeRaftQ1a-RemPlan-1.md` and the unchanged DRAFT qualification/adoption documents. Memory-only Q1a acceptance; no production activation.

**Baseline:** Previous root `5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`; revised root `31bbea0cf1da3c6ae437cf482cb744d561693c08`. Unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources read through pinned `git show` and scoped diff. All four HEADs matched at start and end.

**Date:** 2026-10-03  
**Axis:** Focused interface-conformance closure and changed-range regression review. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its current-round report. Filed verbatim by the lane owner.

**Verdict: GO** — P2-1 closed; zero open Code findings. This accepts the bounded memory-only Q1a implementation scope.

---

## 0. Evidence base

Read the remediation plan and scoped changes to Application, Cluster, README, qualification evidence and review ledger. Inspected:

- `application.rs:91–146`: unchanged private envelope application/replay checks.
- `application.rs:195–211`: unchanged first-application movement readiness enforcement.
- `application.rs:291–311`: corrected public adapter.
- `cluster.rs:17–92`: new actual-driver/public-interface regression.
- `cluster.rs:99–116`: actual driver still decodes retained readiness and calls the private application path.

Compared against the original finding and previous pinned implementation. The old public adapter supplied `None` to a replay check comparing retained `Some(Readiness)`; the original counterexample therefore remains independently supported by source inspection. The evidence records the regression’s pre-correction failure at index 3; I did not mutate source to recreate that RED run.

Independently ran the permitted commands with the specified compatible `PROTOC`:

- Filtered `public_replay_of_driver_attested_move_recovers_original_receipt`: **1 passed**.
- Full locked/offline proof workspace tests: **22 passed**—2 API witnesses, 2 codec tests, 1 new regression and 17 qualification tests.
- `proofs/raft-adoption/check.sh`: architecture, process-global, conditional-boundary and formatting checks passed.
- Clippy, all targets with warnings denied: passed.

No source writes, custom tests, Git mutations or current-round peer reports were used. The scoped working-tree proof diff was empty.

## 1. Prior-finding closure

| Finding | Status | Independent closure evidence |
| --- | --- | --- |
| P2-1 — Public exact replay fails for privately applied movement entries | **Closed** | The actual RawNode driver accepts Move; its real Application is then replayed through `&mut dyn CommittedMachine`, returning the identical original receipt. Changed command content still returns `ConflictingReplay`; a new unwitnessed Move still returns `IncompleteSuccessor`. |

The correction satisfies the specified remedy. Public `apply` first checks retained history at the requested index. Exact canonical Command equality returns the retained receipt without re-execution; differing content fails. Unseen indexes continue through `apply_committed(index, command, None)`.

The existing private full-envelope replay comparison remains unchanged. Public replay neither replaces retained readiness nor creates new readiness.

## 2. Changed-range and invariant analysis

The regression crosses the boundary previously missing from coverage: successful real-driver movement followed by public trait replay on the same Application. It is not a recorder or separately reconstructed state-machine witness.

The correction is confined to recovering an already applied result. It does not advance application progress, mutate resources, alter retry history or rerun authorization/readiness decisions. Returning historical results remains appropriate for this trusted host interface; client-serving accessors continue to enforce current disclosure permission separately.

New-index behavior remains unchanged, including contiguous ordering and witness-free public movement refusal. Existing forged-frontier, lagging-successor and intervening-mutation tests pass. Private committed replay continues to detect changes in either command or readiness envelope.

Public signatures, boundary types, manifests, dependency policies, codec, authority rules and Ready/LightReady processing are unchanged. The new source uses braced control flow and introduces no conditional attributes. No new architectural root cause or regression was found in the changed range.

Documentation now correctly distinguishes first-time unwitnessed application from exact replay. Recorded feedback measurements are explicitly tied to the earlier 21-test revision rather than presented as measurements of this revision.

## 3. Risks and next action

The original production deferrals remain intact: disk/restart and power-loss behavior, cryptography/bootstrap, metadata witnesses, two-stage movement, bounded retention, snapshots/membership, automatic-election RNG compliance, real sinks and legacy activation remain unqualified.

The next action is to combine the independent focused verdicts on this exact tuple and record Q1a acceptance if both return GO. No broader RA requirement or production readiness follows from this Code verdict.
