# Glade Raft qualification plan — bounded executable proof

Date: 2026-10-03. Status: **staged qualification plan; no passing results asserted**.
The [adoption contract](GladeRaftAdoptionContract.md) defines RA-001–012.
The [ownership evaluation](GladeOwnershipMechanismEvaluation.md) defines EM-01–12.
Raft is the selected qualification direction; production library, availability,
deployment and migration approvals remain separate.

## 1. Proof boundary and review gate

Start alongside existing work at `proofs/raft-adoption`, a standalone Rust Cargo
workspace outside production dependency paths. It MUST explicitly adopt the
existing syntax/metadata architecture checker with its own package inventory,
policy and local command. `glade-discover`'s adoption does not cover this nested
workspace automatically. No new GWZ member or production dependency is implied.

Proposed packages are `glade-raft-adoption-api`, a zero-dependency contract with
meaningful required `CommittedMachine` apply/lookup operations, and
`glade-raft-adoption-proof`, a harness/tool implementing that boundary through a
deterministic application machine and an actual Raft implementation. The API is
a trusted committed-log host boundary; it is not client proposal admission or a
wire/API freeze. Boundary types MUST NOT expose Raft implementation types.
The pure machine's input/output semantics remain explicit inside the harness;
no separate pure package or marker trait is required for this experiment.
Each role, manifest edge and conformance target MUST receive independent review
before implementation. The checker MUST reject an unknown library and forbidden
dependency rather than relaxing policy to pass.

Q0 MUST produce compiling consumer/conformance specifications and observed
failing behavioral tests before implementation. A temporary intentionally refusing
contract provider MAY demonstrate RED; compiler/import errors are not behavioral
RED evidence. Two fresh peer-blind adversarial reviews MUST assess canonical
consistency/product scope and protocol/application/recovery safety respectively.
File exact objects, prompts, findings and closure evidence. Both gates must pass
before implementing the proposed boundaries, as required by BuildEntry. Report
unresolved owner decisions without promoting recommendations into rulings.

## 2. Ordered delivery phases

| Phase | Deliverable and exit gate |
| --- | --- |
| Q0 — contract and allocation | Scoped amendments, role/dependency declarations, compiling consumers and failing success/failure/edge witnesses; dual review closes blockers. No production source is amended by this proof. |
| Q1 — deterministic protocol/application proof | Actual synchronous `raft-rs` `RawNode` harness, explicit message queue/schedules and manual campaigns; fixed three complete-data voters and memory persistence. Pure application and harness conformance pass; counterexample mutants fail. Report protocol results and memory limitations separately. |
| Q2 — real durable recovery | Separately reviewed injected disk adapter; RED crash/fault tests precede implementation. Demonstrate hard-state/log/application/outcome persistence and torn/incomplete recovery behavior at each boundary. Pin actual fsync/power-loss assumptions. Memory Q1 success cannot close this gate. |
| Q3 — configuration and snapshots | Authorized learner catch-up, joint transitions, lost configuration replies, snapshot installation/compaction, retirement and complete retry/policy recovery. Exact consumer contracts and adapter witnesses precede implementation. No dynamic membership enters Q1. |
| Q4 — Glade integration and legacy exclusion | Review/ratify exact canonical amendments and production profile, bridge real cryptographic evidence and receipts, real independent failure domains, all legacy writer exclusion and baseline cut. Qualify actual external sinks separately. No production activation before these gates. |

The Q1 dependency proposal is `raft = 0.7.0` with default features disabled and
`protobuf-codec`, `protobuf = 2.28`, and injected `slog` logging with a discard
fixture. The private proof encoding is not a Glade wire/protobuf bridge. Numeric
bounded fixture scope/name values, requester principal/sequence IDs and verified
policy fixtures are sufficient for this controlled boundary; they do not prove
Glade canonical encoding, signatures or ingress authentication.

The Q1 library is an experiment carrier, not a production selection. Evaluate its
application/persistence obligations against OpenRaft and current primary
implementation documentation in the [companion implementation evaluation](GladeRaftImplementationEvaluation.md). Pin
the chosen proof dependency and lockfile; do not infer delivery speed or reliability
from algorithm precedent, crate activity or a compiling example. Source inspection
of this candidate found `rand::thread_rng()` in its automatic-election timeout
reset. Manual campaigns and no election ticks bound Q1's exercised behavior;
they MUST NOT be called injected randomness or full dependency compliance with
the no-process-global rule. Production qualification remains blocked on an
explicit randomness boundary/approved dependency treatment and automatic-election
tests. Neither a local allowlist edit nor this proof grants that approval.

## 3. Initial executable milestone and requirement-to-witness trace

The initial milestone in this tranche is **Q1a**, the fixed known-group,
complete-payload application witness. It does not close all Q1 requirements.
Authenticated bootstrap/conflicting mapping, metadata-only voter exclusion,
full BeginMove/Activate staging and bounded retention exhaustion MUST remain
explicitly unqualified where the fixture cannot represent them. No numerical
scope/principal fixture is evidence of real signing or private-scope derivation.
Q2–Q4 are subsequent gated work, not part of Q1a's memory claim.

The initial machine MAY use one atomic fixture Move after the harness verifies
successor application through the current committed frontier. This is only a
controlled witness of old-generation fencing, not the production two-stage
move protocol. Its readiness observation is harness-trusted; production requires
verified retained cut evidence and ordered admission closure.

## 4. Full requirement-to-witness trace

Each named case MUST record its deterministic schedule and concrete assertion.
Phases in parentheses close obligations excluded from Q1 rather than hiding them.

| Requirements | EM coverage | Required witnesses and closure |
| --- | --- | --- |
| RA-001/002 | EM-01/02/08 | Known-scope concurrent create has one binding; independent same-label roots stay distinct; empty known-scope join blocks; conflicting root/group mapping quarantines. Changing leader does not silently change home/generation. Q1 uses explicit authenticated-evidence fixtures; genuine signing/custody closes in Q4. |
| RA-003/004 | EM-02/03/04/06 | Commit then drop reply; exact retry after leadership change returns one saved outcome; changed bytes fail; principal/scope IDs do not collide. Minority cannot accept; quorum loss reports pending/unknown honestly. Crash before/after every real persistence/application/reply boundary closes in Q2. |
| RA-005 | EM-04/05/06/09 | Queue old-home work, apply a move cut/activation, then deliver it: new old-generation mutation fails; committed pre-cut work replays once. A check-then-local-append mutant must violate the invariant. Activation without a complete successor frontier fails. |
| RA-006 | EM-01/10 | Unauthorized create/write/host, wrong scope/private principal and inconsistent policy frontier fail. Ordered revocation then delayed write refuses; earlier committed exact retry recovers only with current disclosure permission. Q4 adds genuine signatures and governed policy-frontier integration. |
| RA-007/008 | EM-03/07/09 | Remove the sole payload holder while a metadata-only majority remains: no complete-data receipt/activation. Offline projections cannot accept new authoritative mutation. Opaque external effect is excluded; delayed-effect execution claims fail. Real sink closure is a separate later profile. |
| RA-009 | EM-02/10/11 | Retire then replay create/mutate/move: denied; permitted exact historical retry returns saved outcome. Exhaustion refuses explicitly. Snapshot/compaction retains necessary tombstone/retry evidence (Q3). |
| RA-010/011 | EM-06/08/11 | Fixed configuration cannot be reset from discovery or empty storage (Q1). Real disk loss/incomplete prefix quarantines (Q2). Learner, joint membership, lost replies and full snapshot restoration require Q3. |
| RA-012 | EM-12 | Synthetic legacy-bypass attempt fails at proof boundary (Q1); enumerate and exclude actual production writers, complete baseline cut and rollback fence in Q4. Synthetic exclusion is not migration evidence. |

A sequence number or digest alone cannot stand in for stored complete application
payload. If Q1 cannot represent a requirement faithfully, mark it deferred/blocking
and keep the trace intact. No phase completion means all RA IDs are qualified.

## 5. Verification, evidence and stop conditions

Pure application tests MUST consume explicit policy, time and messages without sleeps,
network services, ambient environment or process-global state. Run the owning
package/conformance tests and the local architecture gate on the normal loop;
contract changes additionally run affected consumers. Use syntax-aware checks
for braced control flow and explicit conditional-compilation scopes, including
disabled branches. Apply the process-global checker with a reviewed local
allowlist; do not add exceptions to make failures disappear.

Measure test execution, warm build-plus-test and cold build separately on the
recorded toolchain/machine after commands exist. Publish actual commands and
durations, not an invented budget or “fast” claim. Q2 disk faults and Q3/Q4 broader
assurance stay explicit tiers outside the pure-library edit loop. Whole-workspace
tests MUST NOT substitute for affected-package/consumer verification.

Q2's first concrete witness MAY retain a disk checkpoint containing entries,
hard state and configuration, reopen it and replay application outcomes. That
proves only its exercised file/reopen path. Power-kill, torn writes, fsync/storage
ordering and certified snapshot recovery remain explicit further Q2/Q3 gates.

Evidence MUST distinguish modeled storage from real disk, controlled delivery
from real transport, supplied trust decisions from cryptography, logical voters
from independent machines, and analytical review from executable outcomes.
Stop a phase on invariant failure, missing required payload/policy frontier,
unrecoverable unknown, architecture violation or conflicting bootstrap evidence.
Repair through a failing regression and renewed applicable review; never reset
history, loosen a dependency/classification policy or weaken a receipt to turn
the phase green. Final reporting MUST name passing cases, residual gates and
the exact profile actually demonstrated.

## 6. Local process binding

The review-loop skill at `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`
controls dual peer-blind gates and bounded remediation. This workspace has no
`CurrentProgramCheckpoint.md`, `AgentProcessRules.md` or
`GwzProcessOptimization.md`; the controlling object is this plan plus the
adoption contract and local review/evidence ledger. Existing root/member work
is named out of scope. Review exact source checkpoints; preserve reports verbatim.
Use GWZ scoped add/commit. No push, production cutover or desktop rebuild is
part of this qualification milestone.

Q0 is a dual Consistency/Safety draft gate. Q1a is a dual Code/State acceptance
gate because it introduces ordered retry/retirement transitions. These do not
freeze a user-facing interface or file format; the private fixture API/codec
has no production compatibility promise. A future production freeze needs its
own applicable review tier and consumer tests.
