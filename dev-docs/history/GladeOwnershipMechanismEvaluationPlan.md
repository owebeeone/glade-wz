# Glade ownership mechanisms — evaluation plan

Date: 2026-10-03. Status: **DRAFT controlling plan for a mechanism evaluation;
no exclusive-ownership mechanism, library, wire profile or production activation selected**.

## 1. Mandate and correction of decision status

The owner requested a documented plan and execution of the mechanism comparison.
The question is whether Glade needs Chubby-like coordination, and, if it does,
which concrete authority/enforcement mechanism fits its resource model.

The earlier [architecture comparison](GladeResourceHomeComparison.md) recommended
H1 only if home-dependent availability is acceptable. That availability policy
has not been accepted. The [stable-home design](GladeStableHomeDesign.md) is an
explored candidate, not the selection of a production mechanism. The earlier
instruction to proceed authorized design and evaluation work; it MUST NOT be
treated as acceptance of the pending outage/deployment policy. This plan governs
the new evaluation's interpretation of those historical documents.

Confirmed constraints: clients create resources/scopes dynamically, discover
each other dynamically, and use a default two-node development launcher. The
launcher is not a ruling that deployment MUST always have exactly two nodes or
that its processes are independent failure domains. No per-resource manual
placement mapping is the general solution. Scope/resource identity, authorization,
durability and exclusive effects MUST remain distinct from discovery routing.

## 2. Authority and scope

Read [build entry](GladeBuildEntry.md), [alternatives](GladeResourceHomeAlternatives.md),
[comparison](GladeResourceHomeComparison.md), [H1](GladeStableHomeDesign.md),
[multiwriter evaluation](GladeMultiwriterSettingsEvaluation.md),
[AuthzModel](glade/GladeAuthzModel.md), [WorkspaceDirectory](glade/GladeWorkspaceDirectory.md),
[DiscoveryModel](glade/GladeDiscoveryModel.md), [buy/build matrix](GladeBuyBuildMatrix.md),
and the committed member [substrate](../glade/dev-docs/GladeSubstrateV1.md) and
[cross-node plan](../glade/dev-docs/GladeCrossNodeWritesPlan.md).
Existing contracts remain authoritative. A candidate needing amendments MUST
identify the exact clauses; documenting a candidate does not amend them.

This tranche is documents and analytical traces only. It MUST NOT introduce
dependencies, public interfaces, implementation, model executables, new test
selection, migration, pushes, deployments, desktop rebuilds or runtime restarts.
Future implementation MUST begin with failing conformance tests and follow
[library policy](LibraryBoundaryAndTestingPolicy.md) and
[package architecture](GladePackageArchitecture.md). Analytical traces are
not executed tests, a formal proof, measurements or adapter evidence.

## 3. Separate the mechanisms before comparing them

The evaluation MUST distinguish: (a) resource/scope identity and namespace
bootstrap; (b) host eligibility and placement policy; (c) serialized ownership
acquisition and transition; (d) a coordination group's own leader election;
(e) enforcement at the real store/effect; and (f) data replication/acknowledgement.
Choosing Raft/Paxos, or a directory winner, does not answer the other layers.

Candidate families MUST receive equally explicit protocols:

| ID | Candidate | Required concrete profile |
| --- | --- | --- |
| M0 | Discovery-only deterministic winner | Negative control: show why partitioned local views cannot establish exclusive mutation across independent copies. |
| M1 | H1 stable home | Durable genesis/name gate, immutable binding, exact retry and store custody; no automatic replacement. |
| M2 | H2 cooperative transfer | Durable quiesce/cut/fence/activation/recovery sequence; no takeover when required old-side cooperation is absent. |
| M3 | Consensus authority with per-operation serialization | Ownership transitions and protected mutations have an ordered, enforceable boundary; distinguish this from a racy preflight quorum check. |
| M4 | Consensus-granted bounded authority leases | Explicit clock/process/restart assumptions, expiry/drain or effect-side exclusion, renewal and safe reacquisition; no guessed lease duration. |
| M5 | Shared authoritative resource arbiter | Atomic generation/operation enforcement at one common sink; availability and data guarantees depend on that sink, not independently advertised homes. |

M3/M4 MUST compare Raft versus Paxos/Multi-Paxos at the algorithm/design level,
and embedded scope/shard groups versus a separate coordination service. The
comparison MUST not select a Rust crate from familiarity or presume Chubby's
namespace/session API is Glade's API. M5 may itself require a replicated sink;
that cost cannot be omitted. Multiwriter is a separate conditional option for
mergeable preferences, not an exclusive-ownership candidate.

## 4. Common fault model and deployment profiles

Baseline: honest authorized crash-fault operators; duplicate/lost/reordered
messages, process crash/restart, partitions, delayed effects and uncertain
recovery. No bound on network delay is assumed. Each candidate MUST state any
additional storage, clock, identity-custody or shared-sink assumptions. Byzantine
equivocation/copied keys are not solved by crash-fault consensus; contradictions
MUST NOT become permission to mint a replacement identity.

Apply every candidate to these profiles, without inventing owner preferences:

- P1: one node; deliberate new independent genesis versus joining a known scope.
- P2: two independent authority/data nodes; loss of either, partition, reunion.
- P3: two data nodes plus an independent placement voter; missing data and
  post-failure acknowledgement policies examined separately.
- P4: three independent authority/data nodes; majority progress versus isolated
  minority; exact application-data commit guarantee still stated.
- P5: a common externally operated coordination/sink service; include its own
  failure domains and runtime dependency, not just the two Glade processes.

Two loopback processes do not witness machine-loss resilience. Progress MAY
depend on quorum/eventual communication; safety MUST not depend on a timeout
proving another participant dead. Existing service under a valid lease and new
ownership acquisition MUST be assessed separately.

## 5. Equal journeys and acceptance criteria

For each journey, record initial state, event order, commit point, legal receipts,
admission/effect fence, recovery/unknown state, permitted progress and future
closure test. Unsupported operation is a result, not an omitted row.

| ID | Journey |
| --- | --- |
| EM-01 | Two clients concurrently create one canonical resource in a known scope; contrast two independent same-label roots. |
| EM-02 | Lost create reply, restart and exact retry; empty local lookup does not establish absence. |
| EM-03 | Home fails; survivor attempts acquisition with each authority/data deployment profile. |
| EM-04 | Partition with delayed/stale claims; both sides attempt mutation and authority renewal. |
| EM-05 | Old home returns or a queued old-generation operation reaches the effect after takeover. |
| EM-06 | Crash/lost reply at each transfer/acquisition commit boundary; recover original intent and outcome. |
| EM-07 | Successor lacks acknowledged data; authority witness has no application copy. |
| EM-08 | New scope bootstrap, two purported groups for one known scope, joining, membership change and lost configuration replies. |
| EM-09 | Lease pause, drift, expiry, restart, delayed renewal and effect execution beyond a preflight check. |
| EM-10 | Hosting permission revoked, scope access denied, resource retired, then stale replay; identify permission freshness assumptions. |
| EM-11 | Permanent media loss, restored/cloned/rollback journal and uncertain completeness. |
| EM-12 | Legacy and proposed writers coexist; activate, roll back and preserve the enforcement boundary. |

Selection criteria, in order: (1) invariant/contract eligibility, (2) ability to
meet the chosen availability/data profile, (3) operational prerequisites and
failure domains, (4) recovery and migration complexity, (5) qualitative latency,
throughput and implementation cost. No numerical scores without measured evidence
or explicit owner weighting. A simpler candidate failing a mandatory invariant
MUST NOT win on simplicity. The same conditions apply to each candidate.

## 6. Outputs and review procedure

The drafter MUST produce `GladeOwnershipMechanismEvaluation.md`: protocols,
equal-scenario matrix, algorithm/service comparison, pros/cons, canonical impact,
disqualifying traces, conditional recommendations and smallest owner decisions.
Use primary Chubby/Raft/Paxos references, distinguish imported precedent from
Glade-specific inference, and state inaccessible sources. Do not call a proposal
proven or implemented. Main lane owner drafts this plan; a bounded fresh drafter
authors the evaluation without Git operations.

Apply the [review-loop skill](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
and its [canonical template](/Users/owebeeone/.claude/skills/review-loop/references/review-prompt-template.md).
The skill's GWZ program checkpoint/process files are absent in this repository;
this plan and the filed cycle ledger bind the process locally. Review tier is
dual: Consistency additionally evaluates comparative/product fitness; Safety
additionally evaluates protocol/recovery feasibility. Each reviewer MUST assess
all candidates and give their own conditional ranking; neither sees the other's
current-round report or is instructed to prefer one candidate. No public surface
is frozen, so no Surface gate is asserted.

Commit the plan/evaluation checkpoint through GWZ; pin root, Glade and
Glade-discover HEADs and document hashes; exclude existing unrelated working
changes. Generate and file exact template-derived prompts. Fresh read-only
reviewers verify pins at start/end. File full reports verbatim, merge verdicts,
and remediate blockers in one scoped patch per round with mapped closure traces.
At most two remediation rounds; changed architecture requires fresh reviewers
and the skill's architectural cap applies. GO/GO accepts only the evaluation's
fitness for an owner decision. The final ledger MUST state exact reviewed tuple,
findings/dispositions, round count, source/verification limits, and next decisions.
No silence or review GO is owner selection. The owner selects the required
availability/deployment profile and then the mechanism; formal/adapter evidence
belongs to its following contract and TDD tranche.
