# Glade Raft Q3 Contract Remediation 1 — SAFETY-AXIS REVIEW

**Review object:** DRAFT configuration/snapshot contract and compiling RED specifications at root `ed243db983c485e46a27aa870ec745de16a56d7a`. Originating Safety focused closure of P2-1, P2-2 and P2-3 from the initial review at `fa1ff8b9fd0be53932300730ff925d0e41c76b1a`. Contract/specification gate only; no Q3 implementation qualification.

**Baseline:** Root `ed243db983c485e46a27aa870ec745de16a56d7a`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Reviewed the committed remediation diff and scoped working-tree reads with an empty scoped diff. All four HEADs matched at start and end.

**Date:** 2026-10-03

**Axis:** Safety, focused on verifying the original fixture, joint-exit and exact-index replay counterexamples. Independent, adversarial, read-only. Fresh reviewers assess the changed shared interface separately; nothing here relies on their current reports. Filed verbatim by the lane owner.

**Verdict: GO** — all three original P2 findings are closed at the contract/specification level; zero open findings in this focused review. This verdict does not replace the required fresh dual review or qualify the unimplemented Q3 algorithms.

---

## 0. Evidence base

Read the canonical Safety closure prompt, merged `GladeRaftQ3Contract-RemPlan-1.md`, and both initial review reports. The initial Consistency report is a permitted prior-round input; no current fresh-review prompt or report was read.

Examined the committed remediation against the initial root, principally:

- `GladeRaftConfigurationSnapshotContract.md`, revised §§1, 3–4 and requirement/implementation traces.
- `GladeRaftQ3ContractEvidence.md`, historical results and remediation evidence.
- `q3-api/src/conformance.rs`, especially corrected Create and complete Accepted setup helpers.
- `q3-api/src/conformance/joint_exit.rs:1–123`, all three joint-exit schedules.
- `q3-api/src/conformance/replay.rs:1–88`, typed application/configuration/noop replay and missing-history checks.
- `q3-api/src/lib.rs:196–222`, `ReplayResult` and revised dyn-compatible replay signature.
- Compiler consumers, amended snapshot/membership specifications, refusing providers and all 19 behavioral test registrations.
- `proof/tests/q3_fixture_compatibility.rs`, proof manifest, lockfile and architecture-policy changes.

The committed diff for existing `proof/src`, `disk/src`, `api/src` and `durability-api/src` was empty. The fixture correction therefore did not alter accepted application or Q2 implementation semantics.

Executed independently, with the permitted pinned PROTOC:

```sh
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api -p glade-raft-adoption-proof --test public_contract --test q3_fixture_compatibility

PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot

proofs/raft-adoption/check.sh
```

Results: two API compiler/fixture tests PASS; two existing-Application compatibility tests PASS; all 19 Q3 behavioral specifications compile and fail at `NotQualified`, exit 101, zero ignored. Structural architecture, formatting and explicit-source-boundary checks PASS; 21 owned sources, zero process-global exceptions.

Historical pre-correction RED results were inspected in committed evidence, not recreated by modifying sources. No files or repository state were changed.

## 1. Prior-finding closure

| Original finding | Disposition | Independently verified closure |
|---|---|---|
| P2-1 — Invalid shared Create preconditions | **Closed** | Exact command regression and unchanged-Application success test pass; nonzero generation/home retain their original refusals. Success helpers require complete Accepted state. |
| P2-2 — Home protection ends at EnterJoint | **Closed** | Contract requires deterministic current-home validation at actual LeaveJoint application. RED specifications cover direct placement, restart, admission race, retained refusal, movement/retirement and successful new exit. |
| P2-3 — Replay cannot represent configuration results | **Closed** | Typed replay represents complete application/configuration results and noops. Exhaustive dyn consumer compiles; RED specification retains accepted/refused originals before snapshot and checks exact replay afterward. |

No new finding arose within this focused closure.

## 2. Invariant analysis

**P2-1: the original invalid-create counterexample no longer survives.** The old helper supplied generation/home 1/1 for creation, conflicting with the retained application’s zero preconditions. `create()` now explicitly sets both to zero while retaining `Action::Create.home=1`. The exact whole-command regression prevents accidentally correcting only one field or confusing the command’s precondition home with the selected new home.

More importantly, the compatibility test invokes the unchanged real `Application` through `CommittedMachine`, compares the complete Accepted Resource and confirms retained lookup. The paired negative test restores nonzero generation or home separately and still obtains `StaleGeneration` or `WrongHome`. Thus closure comes from compatibility with existing behavior, not a weakened implementation.

`submit_accepted` compares the full request and full Accepted Resource; corrected snapshot, removal and movement setup uses that helper. A terminal refusal can no longer silently masquerade as successful setup merely because a receipt exists.

The new proof-to-q3-api edge is development-only, explicitly declared and narrowly justified by this compatibility target. It adds no normal contract-to-implementation dependency, carrier type leakage, role reclassification or process-global exception. The architecture gate independently accepts the declared graph.

**P2-2: the original outgoing-home sequence now has an explicit ordered refusal.** Contract lines 41–43 permit placement on an outgoing-only voter while joint, then require every replica to check current live homes against the resulting incoming voters at the actual LeaveJoint predecessor cut. Stranding a home must retain `Refused(HomeInUse)`, preserve the entire ConfState and configuration version, and avoid applying the carrier configuration change. An otherwise valid LeaveJoint cannot substitute an admission-only HomeInUse error for this ordered decision.

The new conformance function reconstructs the original sequence: enter `[1,2,4]`/`[1,2,3]`, create home 3, restart while joint, then attempt exit. It asserts complete placement state, refusal, unchanged configuration and original terminal retry after another restart.

The queued variant additionally places Create between exit admission and application. It requires the placement index to precede the refusal, addressing the original preflight race. Retirement and qualified movement variants resolve the blocker, preserve the old key’s refusal, reject a stale expected configuration version and require a new authorized exit to succeed. Movement checks observed durable/applied successor cuts and retains the contract’s private-readiness obligation.

These assertions remain unexecuted beyond the initial refusing boundary. That is appropriate for this contract gate: the unsafe permitted sequence has been removed from the normative rules and captured by compiling regression specifications.

**P2-3: the original result is now representable and distinguishable.** `ReplayResult` carries `Application(Receipt)`, `Configuration(ConfigReceipt)` or `Noop { index }`. The amended replay method returns this type directly. Missing/unapplied history is `Error::Missing`, so it cannot become a fabricated noop or absent application result.

The exhaustive compiler consumer accesses complete configuration intent, outcome and configuration fields. The new replay specification externally retains accepted and refused ConfigReceipts and original Entry envelopes before checkpoint/install/restart. It then compares complete typed originals, including an application receipt and the explicitly specified initial election noop. Changed bytes are tested for every retained entry kind; missing indexes are tested separately.

The correction preserves Q1a’s existing application boundary and full original-history retention. It does not weaken replay into keyed outcome lookup or latest-state reconstruction.

## 3. Risks and next action

Actual Q3 membership, V2 storage, snapshot encoding/installation and crash recovery remain unimplemented. The 19 expected RED failures establish compiling specifications, not executed safety behavior. Later Code/State acceptance must exercise these assertions against actual providers, including replica-wide deterministic refusals and original-result recovery after real publication/crash cuts.

The next action is to merge this originating closure with the required fresh dual contract reviews on the same revised tuple. Implementation remains conditional on the combined gate. All four end HEADs were unchanged, and the final scoped working-tree diff was empty.
