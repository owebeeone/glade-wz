# Q4-B0 remediation 3 — bounded consumer and identifier fixes

Date: 2026-10-03. Status: **non-architectural correction authorized; closure pending**.
Object root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`; unchanged Glade,
discover and external Gyld pins. Original-finder
[Consistency closure](GladeRaftB0-ClosureConsistency-2.md) and
[Safety closure](GladeRaftB0-ClosureSafety-2.md) independently closed the
completion-ownership defect. Fresh full
[Consistency](GladeRaftB0-ReviewConsistency-2.md) and
[Safety](GladeRaftB0-ReviewSafety-2.md) returned NO-GO with two bounded
NON-ARCHITECTURAL implementation defects. They identified no third new
architectural root cause.

The [review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>)
explicitly permits: "A third round confined to non-architectural corrections is
permitted; any architectural root cause found in it triggers the cap."
Two architectural rounds remain used. This third patch MUST remain confined to
existing coherent scheduling/domain contracts; no new architecture is authorized.

| Finding | Disposition | Regression/closure |
| --- | --- | --- |
| Fresh Consistency P2-5 | Correct B0-04's work selection. Establish initialization and progress through actual controlled time/tick prerequisites; at each tested instant repeatedly obtain CURRENT inventory and select eligible actions, including newly woken/spawned tasks, under an explicit bound. Each drive remains one poll; hold time and delivery fixed while draining a point. Separate election eligibility from later scheduled network emission; no inline progress or invented catch-up burst. | Compiling RED ticker -> awakened core -> newly spawned vote-task witness; corrected common boundary scheduling reaches each selected action. Registration/wakes inert until their own polls; early-election and inline-spawn mutants rejected. Also establish enough actual selected raft-rs ticks rather than treating a time jump as elapsed ticks. Retain both B0-04 selections/all 20 ordinary RED cases. Carrier-free mechanics are not real engine witnesses. |
| Fresh Safety P2-6 | Replace colliding domain arithmetic with collision-free issued-domain allocation or an explicitly bounded nonoverlapping deterministic encoding validated BEFORE source/work construction. No global allocator. | Compiling RED node1/incarnation11 versus node2/incarnation1. Distinct domains; each foreign instant/deadline refuses WrongDomain without clock/work mutation. Adjacent node/incarnation boundaries, chosen allocation limit and deterministic replay. Existing source/work ownership and public contract remain unchanged. |

Record actual compiling behavioral RED before implementation; preserve prior
logs and inventories. Correct only consumer scheduling/fixture identity and their
regressions/evidence. Public trait signatures, upstream algorithms, role/edge
allowlists, lockfile, both refusing providers and all case IDs remain intact.
Run all compiler/fixture/oracle regressions, 20 ordinary provider RED cases,
structural/global/source-scope/format and denied-warning all-target Clippy.
File current exact hashes/counts/measurements and limits.

The TWO fresh full reviewers who found these defects MUST independently verify
original counterexamples on the same settled patch and add closure tables and
changed-range analysis. Neither drafter nor owner self-closes findings. If an
implementation change requires a shared interface/architecture/ownership-contract
change, stop and identify it instead of redefining this bounded correction.
Any reviewer-classified new architectural root cause now stops this object for
owner redesign-or-accept. No B0 runtime, source adaptation, carrier selection,
production or activation exit is accepted by this correction.
