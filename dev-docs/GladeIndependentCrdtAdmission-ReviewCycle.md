# Independent CRDT admission — review and delivery ledger

Date: 2026-10-04. Status: **semantic design accepted; typed contract/allocation/RED
remediation 1 is required; no admission implementation or activation accepted**. GDL-057 authorizes design, review and implementation in
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

## Semantic acceptance after remediation 1

Accepted tuple: root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`; Glade,
discovery and external Gyld remain at the starting pins above. This accepts
design §§2–8, the scoped canonical amendment obligations, bounded offline
permit historical-validity/trusted-admitter-time model, local custody/provisional
projection separation, full-history quotas and unique canonical origins. It
does not change legacy contracts for unactivated instances or select deployment
values. No wire/API, crypto, storage, live sync or activation is accepted.

| Evidence | SHA-256 / result |
| --- | --- |
| [Originating Consistency closure](GladeIndependentCrdtAdmissionDesign-ClosureConsistency-1.md) | `f05c55cf79f7300cf3151f957128f162637651d442410f2433122b1a42c3c342`; GO, P2-1 closed |
| [Originating Safety closure](GladeIndependentCrdtAdmissionDesign-ClosureSafety-1.md) | `648430ca90384eaa4fc0dc3dd4776209616f6b3a8b928e5f52a69d7fd5519f4f`; GO, P2-1/P2-2 closed |
| [Fresh Consistency prompt](GladeIndependentCrdtAdmissionDesign-PromptConsistency-2.md) | `5c4876ef83431b1853564c2451cd1bce2cf5b2e57008d763796b1d68a5aa2737` |
| [Fresh Consistency report](GladeIndependentCrdtAdmissionDesign-ReviewConsistency-2.md) | `ba578da033b334bee3133fb89258a5799a2d4c07ef8f344d82ac05ee43a18627`; GO, zero P0–P3 |
| [Fresh Safety prompt](GladeIndependentCrdtAdmissionDesign-PromptSafety-2.md) | `d9daf1c6aebe8563eeae26278dec770e27646f780c65888da18139a425500c66` |
| [Fresh Safety report](GladeIndependentCrdtAdmissionDesign-ReviewSafety-2.md) | `dcf6b27353f92224b9a2443c6f58fbb43af8c8ee47162ce476e537c4ae49d35a`; GO, zero P0–P3 |

One architectural remediation round completed; three finding IDs/two root causes
closed. No new architectural root cause or implementation escape found. Required
executable `AD`/`AB` witnesses remain future contract regressions. Next: sole
drafter prepares the internal event/effect contract, reviewed Pure package role,
Gyld source/allocation update and compiling behavioral RED consumers. Kernel
success behavior MUST wait for that checkpoint's review.

## IC-1 typed contract / allocation / compiling RED checkpoint

Root baseline `4d735893c8db0a9ac9b4de9cde01600873b20ce3`. Settled member pins:
Glade `885249a2a093e082aad6e1dc9936a7fd5c54052b`, Glial
`5fd46ba5180051eb20d7b5547f59f52c0f3ebe06`, external Gyld app
`64666e8b1caadde8922b9d42163afbab90655c65`; discovery unchanged
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`. The generated review prompts
will record the exact root revision carrying this checkpoint. All commits are
local; the scope remains IC-1, not an activated feature.

Controlling [internal contract](GladeIndependentCrdtAdmissionContract.md) and
[evidence/inventory](GladeIndependentCrdtAdmissionContract-Evidence.md) define
the event/effect/continuation semantics, proposed Pure role/minimal dependencies,
and frozen inherited Gyld allocation. Existing 24 allocation owners and 107
source-qualified obligations are retained; seven new capabilities link all 17
ICD requirements and four new journeys. No Gyld engine/evaluator or base policy
is changed. The new host reads only app-owned pinned sources.

The state-preserving refusing scaffold compiles. Twenty-one Rust behavioral
consumers fail assertions (exit101), and all eight mandatory released-Taut text
rows fail for absent kernel cuts/receipts (exit1); merge-only corpus reference
passes. This is intentional RED before implementation. Structural/source,
architecture, framework-refusal, selection, process, format and clippy checks pass.
The full Gyld affected runner passes 171 tests in 1.923s after test-first validated
baseline reuse; its prior 2.290s/2.156s failures are preserved, and the 2.0s budget
and selector remain unchanged. Six broader Mypy errors reproduce identically with
the pinned baseline; the new capture host passes. No suppression or relaxed policy.

Next gate: fresh peer-blind Consistency/Safety review of this exact typed contract,
allocation and RED evidence. Internal API only; user-facing schema/Surface remains
later. Successful kernel behavior MUST NOT begin before this gate. Fixture facts,
volatile commit replies and delivered sync events do not qualify real signatures,
physical restart receipts or automatic duplex app synchronization. IC-3/4 remain
required before the complete live feature and any separate activation.

### Initial IC-1 review — NO-GO

Reviewed root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`, member tuple above.

| Axis | Prompt SHA-256 | Report SHA-256 / result |
| --- | --- | --- |
| [Consistency](GladeIndependentCrdtAdmissionContract-ReviewConsistency.md) | `d8e686ecae732fd13f6bbb21962a9ae73e106f07e3ea8913cb922b3aebc2d2a1` | `9cea2abf9d2698bb0ea7dcdd44f7099623839c4b290bb5bda9b70503e9b0e890`; NO-GO, two P2 and one P3 |
| [Safety](GladeIndependentCrdtAdmissionContract-ReviewSafety.md) | `627a18df3cfc52ab1227fea8de52db80e48ad5235d8e6d1289fa93065e9c3061` | `ffbe0aace9804eefff5ea0e06d05eb8b496bd37afb0089e9de20310642f33442`; NO-GO, one P2 |

Three distinct blocking roots: lookup-invocation ambiguity, incomplete recovery
grammar for retention-only commits, and missing combined post-fork fresh-origin
text witness. No blind convergence in this round. Safety explicitly labels its
recovery-interface root architectural. One bounded
[merged remediation](GladeIndependentCrdtAdmissionContract-RemPlan-1.md) corrects
all findings, including delayed-callback wording. All findings remain open pending
originating verification; fresh full axes also required after the typed interface
change. First remediation round for the IC-1 object; no successful kernel started.

### IC-1 remediation 1 checkpoint

The one merged correction is committed at Glade
`6a0cc5a78da38a023615f6adc5fc354bba4af0b4` and Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`. Discovery and external Gyld
remain at the IC-1 pins above. The root revision will be recorded in generated
review prompts. [Remediation evidence](GladeIndependentCrdtAdmissionContract-Remediation1-Evidence.md)
records 27 compiling behavioral RED tests and ten released-Taut RED rows;
the implementation still refuses every event. All findings remain open.

The original reviewer processes were unavailable after the session interruption.
Their roles MUST be restored from their own verbatim filed reports and original
prompts, with the interruption disclosed in the closure reports. This is restored
context, not a claim that the original live processes survived. Each restored role
MUST verify its original counterexamples; neither the drafter nor lane owner may
self-close them. Two additional fresh peer-blind full reviews MUST independently
review the changed interface. Acceptance MUST identify this process discontinuity.
