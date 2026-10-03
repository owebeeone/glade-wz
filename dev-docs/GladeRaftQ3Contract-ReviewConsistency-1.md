# Glade Raft Q3 contract — CONSISTENCY-AXIS REVIEW

**Review object:** Root diff `14c347612678e76affd561abd31728adac6a9043..ed243db983c485e46a27aa870ec745de16a56d7a`, principally `dev-docs/GladeRaftConfigurationSnapshotContract.md` and compiling RED specifications under `proofs/raft-adoption`. Status: DRAFT remediation 1 contract gate; no Q3 implementation qualification or production ratification.

**Baseline:** Root `ed243db983c485e46a27aa870ec745de16a56d7a`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` `ca04499a360d910fbf8ee2540ed446facd051b35`. Root sources were read through pinned `git show` and working-tree reads verified against an empty scoped diff. Member controlling sources and the external declaration were read at their pinned revisions.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling contract/design graph, internal invariants, interface satisfiability and evidence claims. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero new P0, P1, P2 or P3 findings. The corrected contract and compiling specifications satisfy this axis. This verdict does not qualify actual membership, snapshots, V2 persistence or production behavior.

---

## 0. Evidence base

All four HEADs matched the specified tuple at review start and end. The final scoped working-tree diff was empty. No files or repository state were changed.

Read:

- Root instructions, `AGENTS_GWZ.md`, the canonical Consistency remediation prompt and review-loop skill.
- Configuration/snapshot contract, lines 1–116; Q3 evidence, including historical RED/GREEN distinctions and remediation evidence.
- Both initial Q3 reports and merged `GladeRaftQ3Contract-RemPlan-1.md` as authorized prior-round inputs. No current-round peer prompt/report was read.
- BuildEntry; LibraryBoundaryAndTestingPolicy §§1–6; PackageArchitecture; architecture revision 3; AdoptionContract §§1–4; QualificationPlan §§1–6; PersistenceContract §§1–7 and amendments; qualification evidence and review ledger.
- Pinned Gyld declarations for Admission, Policy, Records, StorageAdapter, NodeAssembly and responsibility allocation.
- Canonical reconciliation sources: Buy/build D-06/R7/R9/R16/Q12; WorkspaceDirectory lock/claim rules and WD-8; DiscoveryModel routing/authority clauses; Authz creation-rooted authority, signed governance, serving-hop and placement rules; pinned member SubstrateV1 and CrossNodeWritesPlan.
- All q3-api/q3-spec source, conformance modules, compiler consumers and refusing providers; fixture compatibility tests; manifests, lockfile, inventory and source-check changes.
- Existing Application create/replay semantics; local raft-rs 0.7.0 proposal/configuration APIs, pending-configuration handling and snapshot restore paths.

Executed independently, using the permitted compatible PROTOC:

```sh
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64
```

Each Cargo invocation supplied that value explicitly:

- Q3 API/spec `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml ... --no-run`: PASS.
- Targeted q3-api `public_contract` and proof `q3_fixture_compatibility`: four tests PASS, none ignored.
- q3-spec `configuration_snapshot`: 19 failures, all at `NotQualified`; exit 101, none ignored.
- `proofs/raft-adoption/check.sh`: PASS; architecture/dependencies/traits, formatting and token-aware conditional boundaries; 21 owned sources, zero allowlisted items.
- All-target Clippy with `-D warnings`: PASS.

Existing Q2 implementation/process-crash evidence was inspected. No redundant broad Q2 build or new process-kill qualification was performed.

## 1. Prior-finding closure verification

| Prior finding | Independent verification at this tuple | Disposition for this axis |
| --- | --- | --- |
| Initial Consistency report | It reported no findings. The entire corrected package was reviewed afresh rather than inheriting its GO. | No prior Consistency blocker. |
| Initial Safety P2-1: invalid Create preconditions | `conformance::create()` now sets generation/home to 0/0 while retaining `Action::Create.home=1`. The exact-command regression and unchanged Application compatibility test pass; nonzero generation/home still produce the original refusals. Success helpers require complete Accepted Resource equality. | Correction verified for the contract/specification gate. |
| Initial Safety P2-2: outgoing home stranded through joint exit | Contract §3 requires deterministic current-home validation at the actual LeaveJoint predecessor cut. `joint_exit.rs:13–123` specifies direct and queued placement, joint restart, retained refusal, exact retry, movement/retirement resolution and a new successful exit intent. | Original permitted sequence is excluded by the corrected normative rule; compiling closure specifications verified. Actual carrier behavior remains RED/unimplemented. |
| Initial Safety P2-3: configuration result absent from index replay | `q3-api/src/lib.rs:196–222` exposes complete application/configuration/noop variants. The exhaustive consumer compiles. `replay.rs:12–88` retains accepted/refused configuration originals, an application original and the actual initial noop across checkpoint/install/restart; changed envelopes and missing indexes have distinct errors. | Interface impossibility corrected; behavioral implementation remains unqualified. |

These are independent Consistency checks. They do not replace the originating Safety reviewer’s required closure verification or the lane owner’s combined verdict.

## 2. Invariant analysis

**Allocation and dependencies.** Q3 retains the explicitly proposed M3-D qualification of Records’ source-commit exclusion. It does not silently ratify production amendments or move arbitrary effects into Records. Storage owns physical publication; Admission/Policy owns validated authority; discovery supplies routes. The proposed contract depends only on existing application and durability contracts. The new proof-to-q3-api edge is explicitly development-only, used by the two compatibility regressions, and introduces no contract-to-implementation dependency, third-party package or classification relaxation.

**Lifecycle and recoverable history.** Explicit create_new/open separation, exclusive creation, held locking, expected immutable identity and trusted revision floor preserve Q2’s recovery direction. Missing or empty stores cannot authorize genesis. Structurally accepted storage cannot authorize serving. Complete semantic replay and comparisons of overlapping committed histories remain required before startup. Checkpoint omission must be covered by retained original history; legitimate uncommitted suffix replacement remains permitted while committed evidence stays immutable.

**Joint authority and home independence.** The partition specifications distinguish outgoing-only and incoming-only majorities. Learner catch-up must traverse actual storage/application paths. Configuration readiness is retained evidence checked against the ordered predecessor, rather than local availability. The corrected LeaveJoint rule additionally checks current homes at that predecessor. Refusal preserves ConfState and its successful-mutation version while advancing applied history. Exact retry preserves the old refusal after resolution. Local carrier documentation permits refusing a committed configuration without calling apply_conf_change; pending-change handling follows application progress.

**Snapshots and replay.** Typed replay now makes the complete-history promise representable at the declared boundary. Missing history cannot masquerade as a noop. Complete serialized entry identity remains required for every kind. Snapshot envelopes retain resources, names, tombstones, policy, original commands/results and configuration history; semantic reconstruction must cross-check every materialized map.

The incoming-snapshot private candidate avoids publishing checkpoint S with applied A<S. Whole-image synchronized publication precedes exposure, messages and receipts. LightReady ordering retains Q2’s obligations. Learner catch-up after compaction requires a regenerated authorized snapshot containing the receiver, followed by a genuine suffix; rewriting an old cut’s ConfState is prohibited.

**Evidence honesty.** Unknown post-write results poison participation, partial tails quarantine, rollback detection requires independent trusted state, and complete-record exhaustion refuses. Compiler/fixture success is separated from 19 intentionally RED behavioral specifications. Synthetic faults, actual SIGKILL and power loss remain distinct claims.

## 3. Risks and next action

Implementation fidelity remains the principal risk: assertions beyond initial `NotQualified` failures have not executed. Actual ConfChangeV2 handling, V2 framing, semantic adversaries, snapshot/suffix installation, publication faults and externally checked process termination remain mandatory later witnesses. Complete-history retention also preserves the stated growth and capacity limits.

The next action is to merge the independent contract verdicts and originating Safety closure verification. Combined acceptance permits staged Q3 implementation beginning with actual host/adapter RED tests; Code/State acceptance and the complete implementation exit matrix remain required.
