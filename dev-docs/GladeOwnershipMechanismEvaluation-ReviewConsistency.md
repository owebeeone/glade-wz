# GladeOwnershipMechanismEvaluation — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeOwnershipMechanismEvaluation.md`, controlling DRAFT plan, and DecisionLog delta from root `4785cf516d24dab2d49c3ff79dc517a839dc2f45` to `189516d88838854dd3291291576d8c226b5162ac`. DRAFT analytical decision packet dated 2026-10-03; no mechanism selected.

**Baseline:** Root `189516d88838854dd3291576d8c226b5162ac` is abbreviated incorrectly here only if read without the full pin below; the exact reviewed root is **`189516d88838854dd3291291576d8c226b5162ac`**. Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Sources were read using `git show <exact-pin>:<path>`, not member working files.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling document graph, including comparative/product fitness. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This accepts the packet’s fitness for an owner decision only. It selects no mechanism, deployment, acknowledgement guarantee, dependency or availability policy.

---

## 0. Evidence base

The three permitted `rev-parse HEAD` commands returned the required full pins at both the start and end of review. The tuple did not move.

I read `AGENTS.md`, `AGENTS_GWZ.md`, the canonical Round1-Consistency prompt, and the locally bound `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`. No files were written, Git state mutated, builds or tests run, executable models constructed, or subagents launched. Uncommitted member work supplied no evidence. Current sibling prompts and reports were not consulted; earlier reviewer rankings were not used as evidence.

The inspected pinned evidence was:

- **Evaluation, lines 1–197:** all candidate protocols, algorithm and packaging comparison, P1–P5 matrix, EM-01–12 schedules, canonical impacts and conditional choices.
- **Evaluation plan, lines 1–157:** correction of H1 decision status; six-layer separation; M0–M5 requirements; common faults, deployments, journeys and acceptance boundary.
- **DecisionLog, lines 63–94, plus the specified delta:** particularly GDL-050/051 and the existing acceptance/registry contract limitations.
- **ResourceHomeAlternatives, lines 1–262:** authority/home/replica distinctions, RH-01–12, H1–H3 obligations and analytical-review limits.
- **ResourceHomeComparison, §§1–6:** the unresolved availability requirement, dynamic creation, planned movement, two-node limitations, acknowledged-data distinction and owner questions.
- **StableHomeDesign, lines 45–279 and requirements at 281–304:** creation journal, custody assumption, namespace and retirement semantics, exact outcomes, stronger metadata durability, admission and migration.
- **MultiwriterSettingsEvaluation, §§1–7:** its bounded preference scope, creation dependencies, authorization and durability limits, and exclusion of exclusive effects.
- **BuyBuildMatrix, D-06/R7/R9/R16/Q12:** lines 34, 67–69, 86 and 159.
- **AuthzModel, §§1/3a/3b/4/4a/7a and related rulings:** lines 21–180, 243–272 and 370–410.
- **WorkspaceDirectory, §§1–4, deployment phases and WD-8:** especially lines 115–139, 209–216 and 274.
- **DiscoveryModel, §§0–5 and §7:** local fold, pure kernel, authority filtering, time-relative projection and historical takeover scenarios.
- **BuildEntry, lines 1–88; LibraryBoundaryAndTestingPolicy, lines 1–108; PackageArchitecture, lines 1–160.**
- **Pinned Glade SubstrateV1, §§2/5/6:** especially R1/R2/R7 and W1–W8, lines 295–644; **CrossNodeWritesPlan, §§2/3/7**, including its independently admitted-write counterexample.
- **Pinned RegistryContractDraft, lines 1–164:** atomic acceptance/recovery, current authorization, partial resolution, independent-port composition limits and fixture limitations.

I inspected the specified root diff. The object adds the plan and evaluation and adds GDL-051; it does not amend existing ownership or wire contracts.

The primary [Chubby paper](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/), [extended Raft paper](https://raft.github.io/raft.pdf), and [Paxos Made Simple](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf) were accessible. I checked the mechanisms attributed to them, including sequencers, lease uncertainty, Raft client deduplication and membership transition, and Paxos durable promises and ordered state-machine commands. The evaluation’s descriptions are paraphrases, not purported verbatim quotations. No implementation audit or performance conclusion follows from those publications.

## 2. Invariant analysis

### Decision authority and comparison eligibility

The attempt to turn the historical H1 direction into accepted outage behavior fails. Evaluation lines 3–5, plan §1 and GDL-051 explicitly preserve that distinction. StableHomeDesign’s earlier authorization remains authorization to explore the candidate. The owner still must choose whether unreachable-home edits may remain pending.

All six families receive an actual authority/enforcement story, rather than just algorithm names. M0 is an intentional negative control. M1 needs an immutable binding and exclusive custody; M2 adds cooperative irreversible transfer; M3 serializes protected operations; M4 separates bounded local execution from shared-sink enforcement; M5 requires a common resource transaction. Their prerequisites are counted before cost.

The matrices do not make every candidate promise the same progress. That is appropriate parity: an unsupported operation is visibly blocked. M1’s lack of takeover and M2’s need for the old side are decisive product limitations, not missing rows. M4-L is explicitly blocked pending evidence for its additional environment assumptions. Neither its hypothetical benefit nor its lease duration is presented as delivered capability.

### Identity, creation and group leadership

I attacked lost creation replies followed by an empty directory at a different node. M1’s worked sequence, common retry rules and EM-02 prevent that node from minting a replacement. M2 inherits the creation gate; M3 orders creation; M4’s journey uses group creation before granting a lease; M5 serializes creation at its sink. Independent same-label roots remain distinct rather than silently composing.

Changing a Raft leader or Paxos proposer cannot become resource takeover under the text. Evaluation lines 19–26 and 70 make acquisition an application decision. Section 4 requires a unique authorized group/configuration binding for a known scope and prohibits unilateral quorum reset. Thus discovering another process, or selecting a leader, supplies neither namespace authority nor successor data.

The custody exclusion is consequential. M1/M2 are not proofs against two independently running cloned journals or contradictory authorized genesis. The packet says so instead of claiming signatures establish uniqueness. Restore uncertainty goes to quarantine/unavailability. Future custody controls remain an essential dependency whose adequacy must be checked in the selected profile.

### Actual mutation and effect fencing

Three counterexamples were particularly useful.

First, A performs a linearizable ownership check, pauses, B activates, and A appends locally. Evaluation line 78 rejects this explicitly: it is not M3. A committed command must cover the protected mutation, or the actual sink must enforce it.

Second, A and B each keep their own highest generation. Both can accept their locally current generation. M2’s cut/fence sequence, M3’s ordered application or sink barrier, and M5’s common transaction do not treat independent counters as mutual exclusion.

Third, A dispatches an action before a transition and the action executes afterward. M1 restart requires outstanding-effect resolution; M2 cannot fence until effects are terminal or excluded; M3-E requires ordered sink outcome recovery and a barrier; M4-L needs terminal-effect bounds, while M4-S needs sink enforcement; M5 rejects a metadata transaction followed by an uncontrolled queue.

These conditions can sacrifice availability indefinitely for an opaque action with an unknowable outcome. That is an honest limitation. The packet does not claim consensus, a process lock, or lease expiry manufactures the missing effect contract.

M2’s lost fence-certificate reply also remains coherent. Before the irreversible fence, recorded abort is possible with successor exclusion. After it, A cannot resume the old generation; recovery finishes the same intent or waits. No analytical schedule requires unknown to mean aborted.

### Permission freshness and policy ordering

The evaluation preserves the distinction between authorization and serving placement. Creation-rooted authority, signed governance, private-principal derivation and accepted operator placement remain controlling. Becoming a voter, holder or sink client does not issue grants.

The strongest attack was a remote revocation unseen by a partitioned enforcement point. Evaluation line 38 and EM-10 explicitly decline instantaneous knowledge. Local complete applicable evidence must be checked at the enforcement boundary; stronger freshness requires a selected policy stream/barrier and its availability cost.

M3 additionally requires deterministic state-machine policy inputs, rather than each replica independently consulting local discovery or time. This is consistent with ordering protected operations while retaining local authorization evaluation. Its ordered policy context is not permission to replace every local read check with a live grant oracle. Section 7 names the authorization clauses requiring reconciliation.

Exact outcome recovery also does not override current disclosure permission. A committed binding can survive revocation while its receipt is withheld from a now-unauthorized requester. Denial therefore does not undo the original transaction or justify new genesis.

### Acknowledgements and deployment claims

The metadata-only witness attack fails decisively. Suppose A acknowledges x, B lacks x, and A is lost. A surviving authority majority does not contain x merely because it can select B. Evaluation lines 131 and 145 require blocked activation unless the selected data guarantee establishes a complete successor cut.

The current substrate interpretation is accurate: R2 is unsynced local holding with process-crash survival, while W4 adds holder and forwarder copies. Neither establishes an independent machine-loss commitment. Stronger metadata durability is explicitly proposed separately.

P1–P5 also distinguish service continuation from acquisition. M3 with two voters needs both for new committed commands. M4 may serve during an already valid conservative lease but cannot acquire or renew without quorum. Adding a placement witness changes authority availability, not application-copy completeness. Continuing to acknowledge with one remaining data copy changes a two-copy guarantee and needs approval.

Two loopback processes are never counted as independent machines or storage domains. A separately operated service’s voters, backup, trust and outage dependence are counted instead of disappearing behind the two Glade clients.

### Canonical impacts and future closure

Section 7 correctly identifies the controlling conflict locations rather than pretending the evaluation supersedes them. D-06 and R7 retain local signed-record discovery; R9/Q12 do not preapprove resource consensus; R16’s distributed-lock exclusion needs amendment if that route is chosen. Workspace locks retain their physical scope.

Substrate §2 and W1/W2/W7 require explicit activated-profile changes; R1/R2/R7 and W3–W6 require receipt, ordering and cut reconciliation. Historical DiscoveryModel takeover cannot move a stable binding by itself. Registry ports cannot compose into an unstated atomic resource transaction.

EM-01–12 are satisfiable as future obligations because they distinguish permitted success from required refusal, unknown or blocked outcomes. They cover overlapping creation, changed retries, every durable boundary, data loss, membership, pause/drift, revocation, restore and legacy overlap. They are explicitly analytical schedules, not reported passing tests.

EM-12 prevents a reader upgrade or rollout flag from licensing an unfenced old writer. Rollback must retain enforcement. The future work also preserves LBT classifications, affected-consumer checks, concrete adapter evidence and the adopting architecture gate. It claims no wider gate adoption.

### Independent conditional ranking

My ranking is requirement-dependent, with eligibility first:

| Required profile | Independent order and conditions |
| --- | --- |
| Home outages acceptable; movement optional; no existing approved common sink | **M1, then M2, then M3-D, then M5.** M3-E and M4-S add sink dependencies; M4-L is currently blocked; M0 is ineligible. |
| Identity-preserving planned movement; old home can cooperate | **M2 first.** M3-D or M5 follows according to existing deployment. M3-E is conditional on recoverable effects. M4-S adds lease complexity; M4-L remains blocked. M1 cannot meet movement; M0 cannot meet exclusivity. |
| Automatic takeover preserving acknowledged data | **M3-D first for self-operated replicated application history; M5 first when an approved common sink already owns that history and mutation.** M3-E needs its additional sink recovery contract. M4-S is conditional but supplies no missing data. M4-L becomes eligible only after its extra assumptions close. M1/M2 and M0 are ineligible. |
| Exactly two independent majority voters; either may disappear; no additional authority | **No family meets automatic exclusive takeover.** A reachable external sink changes this premise and must be counted. |

An existing approved sink can move M5 ahead of M1/M2 even without required failover: its authority is already an operational prerequisite. Conversely, unacceptable plaintext placement or external-service dependence disqualifies that route before simplicity is compared.

I conditionally prefer **Raft** for a newly designed M3 log because its presented log, application and configuration model maps directly to the required ordered commands. That is a design-organization judgment, not measured superiority. **Paxos/Multi-Paxos** ranks equally on eligibility and can move first when an existing reviewed implementation supplies complete durable ballot/slot, prefix-application, deduplication and reconfiguration contracts. The cited paper’s agreement result does not itself supply those Glade contracts. [Raft](https://raft.github.io/raft.pdf), [Paxos](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf).

I prefer **embedded groups** when authority and complete application history can share the same approved data-bearing electorate. I prefer a **separate service** when its independent electorate is acceptable and genuinely supplies needed availability. That flips back if only metadata is available or its runtime operation is unacceptable. Chubby illustrates the generic-electorate and receiving-resource fence distinction; it does not decide Glade’s packaging. [Chubby](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/).

## 3. Risks and next action

Residual uncertainty remains in the selected profile’s namespace/action mapping, custody, bounds, retirement/reuse, policy ordering, data receipts, membership, adapter recovery and migration. These are explicit downstream contract obligations. Their existence does not make an analytical comparison defective, and this GO does not discharge them.

Opaque external actions may make safe recovery permanently unavailable. Lease feasibility may differ materially between platform and effect classes. Service packaging can introduce substantial operation and trust requirements. None has measured cost or performance evidence here.

The single next action is to obtain the owner’s explicit availability/data/deployment decision using the choices at evaluation line 193. That decision must settle acceptable pending outages, cooperative movement versus automatic takeover, independent authority/data domains, post-failure acknowledgement guarantees, and acceptable common-service/effect dependencies. Then select a mechanism and prepare its canonical amendment and conformance tranche. Review GO supplies decision evidence; it is not owner selection.
