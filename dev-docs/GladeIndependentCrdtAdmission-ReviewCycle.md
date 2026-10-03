# Independent CRDT admission — review and delivery ledger

Date: 2026-10-04. Status: **semantic design remediation 1 pending; no implementation or
activation accepted**. GDL-057 authorizes design, review and implementation in
the [IC-1–4 lane](GladeIndependentCrdtAdmissionPlan.md), ahead of first strong
Raft production integration. The [design](GladeIndependentCrdtAdmissionDesign.md)
is the controlling draft. Existing Raft and legacy writer contracts remain.

## Process binding and review tiers

AGENTS.md/AGENTS_GWZ.md, the
[review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>) and its
canonical prompt template govern this lane. The workspace has no GWZ
CurrentProgramCheckpoint/AgentProcessRules/GwzProcessOptimization files; use this
local ledger, as the existing Q4 process does. All checkpoints use GWZ scoped
commits and exact root/member pins. No push, live seal, enrollment, launcher
change or activation follows from a gate verdict.

| Checkpoint | Tier and boundary |
| --- | --- |
| Semantic design | Two fresh peer-blind Consistency/Safety reviewers. Proposed semantics and amendment obligations only; no public wire/API freeze or implemented feature. |
| Typed internal contract / allocation / compiling RED | Consistency/Safety on exact Gyld allocation, package roles, internal event/effect semantics and executable RED evidence. Surface additionally required if the object freezes a user-facing API/schema. |
| Pure component acceptance | Code/State dual gate on deterministic admission/reconciliation and actual released text consumer evidence. Does not qualify fixture crypto, physical storage or live sync. |
| Real adapters / aggregate live feature | Dual Code/State at durable/aggregate boundaries, real auth/I/O/partition/restart evidence and affected consumers. Surface for user-facing freeze; separate activation gate. |

At most two architectural remediation rounds per object. Findings close only
through the originating reviewer's verification; preserve each complete report
verbatim. Record exact prompt/source hashes, findings, closure tests, scope and
remaining gates at each landing. Fresh axes are used after material architecture
or interface changes. No current peer report enters another reviewer's prompt.

## Starting evidence

Requirement/timeline checkpoint: root `9baa7b8cd1c876ff9e608ecdffdd7490b58ebe9b`;
Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.

Focused existing-merge baseline, run before new implementation:
`./node_modules/.bin/vitest run test/text_crdt_mount.test.ts` from `glial`.
Five tests passed; suite execution 21 ms, runner duration 968 ms on this machine.
This is existing text merge evidence only. No independent-node acceptance,
signed app operation, physical commit or app anti-entropy claim follows.

The design has seventeen future ICD requirements. Their test/adapter obligations,
IC-1–4 milestones and production parameter choices remain open.

## Initial semantic review

Reviewed root `bba04ad27311db50e3e6aedddb4d91780e4e4483`; all member pins above
unchanged. One drafter produced the design; fresh peer-blind Consistency/Safety
reviewers verified the tuple at start and end and performed inspection only.

| Axis | Prompt SHA-256 | Report SHA-256 / verdict |
| --- | --- | --- |
| [Consistency](GladeIndependentCrdtAdmissionDesign-ReviewConsistency.md) | [Prompt](GladeIndependentCrdtAdmissionDesign-PromptConsistency.md): `1a3e1d52c366b194e3bdbbad2495e65a917306019c7f197a4bc85fa9218c35e1` | `4c3a51f41a463eb18588b2dac96fcda5ae78ee503f275e044bc29e6ad8e03d0c`; NO-GO, one P2 |
| [Safety](GladeIndependentCrdtAdmissionDesign-ReviewSafety.md) | [Prompt](GladeIndependentCrdtAdmissionDesign-PromptSafety.md): `a087cad0576072976503f36be6264f57f72673d7787120207794cbdaf2bee16a` | `7302d84baa63b1375fe3c1dac65c8f40214e8d8013b4f475c781dd18673cf863`; NO-GO, two P2 |

Three finding IDs identify two root causes. Both axes independently found the
canonical origin/epoch collision. Safety additionally found that an unauthorized
signed rival could quarantine legitimate admitted history. No finding is closed.
The [merged remediation plan](GladeIndependentCrdtAdmissionDesign-RemPlan-1.md)
defines one correction, originating-reviewer closure and fresh full axes because
security eligibility and canonical identity are refined. Remediation count: first
architectural round in progress; no implementation escape or adapter evidence.
