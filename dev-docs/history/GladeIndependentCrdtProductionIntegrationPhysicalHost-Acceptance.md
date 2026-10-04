# Combined IC-3B genuine authentication and physical persistence — acceptance

Status: **accepted at root `f0b1c325f9e0eff27b4eefadac8084bff3505e73` /
Glade `a47691598df648eb8c9554b27f3d06b0cffcf596` after the seven reports
below returned GO; this accepts genuine authentication and bounded Unix
LocalProcessRestart persistence only.** Date: 2026-10-04.

Protected tuple: Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`,
discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`400cedcf1fff74128f366af758a0c389a23435d7`. Every reviewer verified all five
START/END pins. Source and controlling guides remained frozen during review.

## Independent verdicts and closure

| Filed verbatim report | Verdict | SHA256 |
| --- | --- | --- |
| [GladeIndependentCrdtProductionIntegrationPhysicalHostFull-ReviewCode-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHostFull-ReviewCode-2.md) | GO | `ddb1f632a49a2d8f486c0c6ed54a79801eeac6172f5aaf779deb7dfe66c8bd18` |
| [GladeIndependentCrdtProductionIntegrationPhysicalHostFull-ReviewState-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHostFull-ReviewState-2.md) | GO | `29985d46d38267900771f7938683c5066d48d0274fc3f015de87a52b3fbcc4cb` |
| [GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewCode-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewCode-2.md) | GO | `6883ecf62b4e48729de31aaf2e1cc8772df7e2b6eb5de9399abb6f6ff6875314` |
| [GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewState-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewState-2.md) | GO | `5d5f449f2043d6867d4198d433ad037c198b3e386933b287ef40fab4cc7499f5` |
| [GladeIndependentCrdtProductionIntegrationPhysicalHostClosure-ReviewCode-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHostClosure-ReviewCode-2.md) | GO | `7fcdb2deee9e6cb5c1af0792ebbf2ad3a110e8e9f8cd86aec91ac7638b2026d0` |
| [GladeIndependentCrdtProductionIntegrationPhysicalHostClosure-ReviewState-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHostClosure-ReviewState-2.md) | GO | `7a5a57916457144b007cf21813a40feda7aacac51458b7b0694f6669f7ab23a1` |
| [GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewSurface-2.md](GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewSurface-2.md) | GO | `418bb01b8d4932ed996caef4c058266e6ecd54a658f69bba14372802dadd3d7d` |

Fresh full Code/State independently accept the complete component including the
changed persisted-cut → original Fence → native terminal → core callback path.
The two preceding full reviewers close their own stale-start and oversized-loss
findings. Original B Code/State close their new exact-cut counterexamples and
confirm preservation of all six initial blocking closures. Surface closes its
nonblocking retirement-method omission. No new P0–P3 finding was established.

The [full Code factual erratum](GladeIndependentCrdtProductionIntegrationPhysicalHostFull-ReviewCode-2-Erratum.md), SHA256
`b291461f0c6c8023c98664cdfbfc697976397f399663665137bcdbc48064764c`, preserves GO and corrects its three new
crash labels to native **Fence** boundaries. The original report is unchanged.
These are additional to 26 observation cuts, not three new observation cuts.

## Evidence and exact scope

Remediation2 Evidence/SourcePins/RunLog pin the exact 11-file correction, all65
combined source files and five current guides. Previous guides are checked at
their original commit; compatibility45, canonical27 and23 earlier artifacts
remain byte-exact. Compiling RED covered stale policy/time start, complete
receive interval and M+1 pending loss. Final public disk29 and Records29 pass
with151 affected cuts; prior171 unchanged historical-kind cuts remain pinned
evidence rather than newly rerun. Auth16, boundary6, assembly30 and relevant
contract/architecture/source/globals/format checks pass. Clippy retains9 old
warnings; whole-node formatting remains inherited251-hunk debt.

Authorization uses the complete policy/time cut selected at its barrier.
Observation-issued Fence custody and recovery counters select together before
exact callbacks drain. Oversized drained refused input can retain bounded
permanent loss while preserving original custody on failure. Earlier valid
Started, exact receipt/retry, complete Prepare replay and conservative guard
ownership remain intact. No public type, format, dependency, allowance,
trusted authority or default changed in the second correction.

Qualification MUST remain limited to the stated Unix local-process-restart
profile and independently trusted floor. No power-loss/quorum or simultaneous
floor/data rollback guarantee is established. Generic orphan Active receives
remain pending/incomplete; restart MUST NOT fabricate a drained capability.
Finite history has no implicit GC and can exhaust.

## Accounting and downstream prerequisite

Semantic: **2 architectural / 1 nonarchitectural / 1 completed remediation**.
Typed: **2 architectural / 5 prior nonarchitectural / 2 completed remediations**.
Combined B: **0 architectural / 6 nonarchitectural / 2 completed remediations**.
Finding closure does not subtract roots, reset caps or authorize another patch.
A third typed architectural root still requires STOP.

This committed acceptance record permits parent to file genuine B1 qualification
metadata and pin it to real committed source/evidence. The strict pre-remote gate
MUST pass with that record before any C history-bearing exchange. Metadata MUST
NOT reuse a null, invented or unrelated accepted_review_record.

C actual binary routing/automatic duplex exchange and aggregate qualification
remain unbuilt. Original A2 originating closures remain owner-deferred to the
wider ABC gate. IC4, live roots/keys/floors, enrollment/migration, launcher
defaults, desk rebuild, Raft and push remain outside this acceptance.
