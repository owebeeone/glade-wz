# Glade ownership mechanisms — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeOwnershipMechanismEvaluation.md`, controlling `dev-docs/GladeOwnershipMechanismEvaluationPlan.md`, and their DecisionLog delta from `4785cf516d24dab2d49c3ff79dc517a839dc2f45`, at root `189516d88838854dd3291291576d8c226b5162ac`. DRAFT analytical decision packet, dated 2026-10-03.

**Baseline:** Root `189516d88838854dd3291291576d8c226b5162ac`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Local evidence was read from pinned Git objects using `git show`, with the specified root diff inspected. Working-tree implementation changes were excluded.

**Date:** 2026-10-03.

**Axis:** Safety and protocol feasibility: degraded operation, irreversible transitions, real mutation enforcement, recovery, data preservation, permissions, deployment and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This packet is fit for an owner decision. No mechanism, outage policy, library, deployment or production activation is selected or accepted by this verdict.

---

## 0. Evidence base

The following evidence was actually inspected:

- Evaluation, lines 1–197: shared assumptions and receipts; M0–M5 protocols; primary-source interpretation; group configuration; P1–P5 deployment matrix; EM-01–12 schedules and closures; canonical amendments; conditional choices.
- Controlling plan, lines 1–157: mandate and H1-status correction; equal protocol obligations; fault/deployment profiles; analytical-review boundary; required journeys and acceptance procedure.
- DecisionLog, GDL-050/051, and the specified three-document diff: the clarification adds no production selection and does not authorize canonical replacement.
- `GladeResourceHomeAlternatives.md`, particularly §§2–8 and RH-01–12; `GladeResourceHomeComparison.md`, particularly §§1–6; `GladeStableHomeDesign.md`, §§1–9; `GladeMultiwriterSettingsEvaluation.md`, §§1–7. Historical review conclusions and rankings were not evidence for this verdict.
- `glade/GladeAuthzModel.md`, §§1, 3a, 3b, 4, 4a and 7a: creation-rooted governance, signed sensitive operations, policy closure, forward-only revocation, private principal binding and operator-approved plaintext placement.
- `glade/GladeWorkspaceDirectory.md`, §§3–4 and WD-8; `glade/GladeDiscoveryModel.md`, §§0, 3–7: genesis versus joining, physical-copy locking, local routing, authorization and historical epoch/takeover language.
- `GladeBuyBuildMatrix.md`, D-06/R7/R9/R16/Q12; `GladeBuildEntry.md`; `LibraryBoundaryAndTestingPolicy.md`, LBT-001–012; `GladePackageArchitecture.md`, §§1–8.
- Pinned Glade `GladeSubstrateV1.md`, especially §2 and §6 R1/R2/R7/W1–W8; `GladeCrossNodeWritesPlan.md`, §§2–3 and 7. These establish the restricted current acknowledgement guarantees, holder-directed admission and unresolved historical split repair.
- Pinned Glade-discover `RegistryContractDraft.md`, particularly atomic acceptance/recovery and registry/trust/placement: exact retry, bounded retention, partial local knowledge and the limits of composing independent ports.

Primary web inspection covered [Chubby](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/), especially locks/sequencers and session/failover handling; [Raft](https://raft.github.io/raft.pdf), including joint configuration and client retry/read safeguards; and [Paxos Made Simple](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf), including stable acceptor state, competing proposers and ordered state-machine application. All three were accessible. Their descriptions supply algorithm precedent, not evidence that Glade adapters satisfy these proposals.

At both START and END, `git rev-parse HEAD`, `git -C glade rev-parse HEAD`, and `git -C glade-discover rev-parse HEAD` returned the exact tuple above. It did not move.

Process instructions inspected were AGENTS.md, AGENTS_GWZ.md, the canonical Safety prompt and the locally bound review-loop skill. No sibling current-round prompt/report was consulted. No files were written, Git state mutated, builds/tests run, executable models evaluated or runtime restarted.

## 2. Invariant analysis

### Identity, bootstrap and leadership

EM-01, EM-02 and EM-08 distinguish deliberate independent genesis from acquisition inside a known canonical scope. Equal labels do not establish equal identity. Conversely, a missing local directory entry does not permit a replacement for an already known identity.

I attacked this with two clients choosing different homes for the same known name, followed by a lost successful reply and a retry at an uninformed node. M1 serializes the name at the existing home; M2 inherits that creation gate; M3 orders Create; M4’s journey explicitly places Create at the group before leasing; M5 uses the common sink transaction. The uninformed node cannot turn uncertainty into genesis. Exact request identity, changed-payload rejection and original-outcome recovery are common requirements at lines 32–36.

A second attack bootstrapped disjoint voting groups for one known scope from empty discovery views. Lines 114 and 146 require authenticated binding to one configuration and resource mapping, nonvoting catch-up, safe transition authority and exact recovery of configuration intent. A lost quorum cannot be repaired through unilateral reset. The packet therefore does not mistake dynamic discovery for group formation.

Group leader election remains separate from resource home changes at lines 26 and 70. Electing a proposer cannot itself create an application generation, issue grants or establish successor data. These attacks failed under the proposed rules; configuration implementation remains a future contract obligation.

### Acquisition and actual effect enforcement

M0 is correctly excluded from exclusive admission across independent copies. Its EM-04 counterexample is reproducible analytically: A renews unseen, B expires its local observation, each considers itself eligible, and both mutate. Healing and deterministic ranking cannot retract completed effects. No claim convergence argument repairs this.

M1’s exclusion rests on an immutable binding and exclusive store/journal custody, not on advertisements. Lines 50–56 require durable genesis intent, atomic binding/retry state, an actual lifecycle/storage lock and execution-boundary checks. I tried restarting A while previously dispatched work remained active. The text requires resolving outstanding effects before conflicting admission; inability to establish exclusion blocks the affected effect. Another independent store cannot take over merely by raising its counter.

For M2, I retraced the irreversible boundary:

1. A closes admission and obtains terminal outcomes for admitted/dispatched work.
2. C includes the history promised by the selected acknowledgement guarantee.
3. B verifies and durably prepares C while inactive.
4. A durably fences g.
5. Only recoverable fence evidence permits B’s recoverable activation.

A crash after step 4 but before delivery does not authorize A to reopen g, nor B to assume the certificate exists. Recovery completes the original T or remains unavailable. Permanent loss of required evidence may strand the resource; that is stated rather than hidden behind an administrative retry. A delayed old effect must already be terminal or irrevocably excluded before fencing. Lines 60–66 close the tempting “fence metadata now, drain later” shortcut.

M3 passes the mutation-order attack because its eligible data profile places actual authoritative application operations in the ordered history. Independent projection writes are prohibited. The packet explicitly disqualifies a linearizable “still home?” query followed by a paused local append at line 78.

M3-E adds the essential external sink rather than treating consensus as effect execution. A committed q can stall before dispatch; BeginMove closes admission, recovery resolves the committed prefix through k, the sink installs its barrier, and Activate follows barrier verification and successor readiness. A delayed q then recovers a saved outcome or fails enforcement. An opaque action with no recoverable terminal outcome blocks activation. This is a real limitation on eligible effects, not a proof supplied by a log entry.

M5 survives both orders of the same race: old q commits before the transition and enters its cut/outcome, or arrives afterward and is rejected atomically with mutation. A metadata row beside an uncontrolled queue is expressly insufficient. Its common-sink requirement is decisive; separate stores retaining their own highest generations cannot be relabeled M5.

### Leases, delayed execution and restart

EM-09 supplies the strongest attack on M4: A validates, pauses, and leaves work queued; authority expires; B obtains the next generation; A’s queued mutation completes afterward.

M4-L does not claim this schedule safe merely because clocks are monotonic. Lines 82–88 require durable outstanding promises, conservative relative-rate assumptions, suspension-aware clocks and an admission-to-terminal-effect bound or irreversible prevention. Restart invalidates the old session; delayed renewals do not resurrect it; new leadership cannot erase outstanding horizons. The profile is explicitly blocked for Glade deployment until those assumptions have evidence.

That blocking qualification matters. General desktop scheduling, suspension and opaque remote effects have not been shown to support the required bound. This packet does not represent them as eligible today.

M4-S requires atomic sink enforcement and exclusion of old admitted operations across the successor barrier. Its feasibility consequently approaches M5. The packet fairly retains that operational dependency instead of presenting leases as free offline exclusivity. Chubby’s resource-checked sequencers and conservative session handling support the separation of coordination and enforcement; they do not supply Glade’s terminal-effect proof. [Chubby §§2.4, 2.8–2.9](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/).

### Acknowledged data and deployment

EM-03/07 and P1–P5 keep authority availability separate from data completeness.

P1 permits deliberate singleton genesis, with no independent failure tolerance. In P2, two majority voters require both for new consensus decisions. M1 can continue after non-home loss subject to permissions; loss of the home blocks it. M2 cannot substitute survivor preference for old-side cooperation. Existing M4 service may continue only inside its proven valid interval.

In P3, B plus a placement witness may retain authority but cannot reconstruct x acknowledged solely at A. Lines 131 and 145 expressly block truthful successor activation in that state. Requiring both data nodes to retain x before acknowledgement changes the protection available, but cannot sustain the same two-copy acknowledgement rule after either copy disappears. Reduced post-failure guarantees require a separate owner decision.

P4 offers majority progress only with a declared data-bearing durable commit rule. Three metadata voters plus asynchronous application copies are insufficient. A witness storing complete command payloads becomes data-bearing and changes the stated profile.

P5 includes the external service’s own electorate, storage, permissions and outage dependency. The two Glade clients do not become the service’s failure domains. An unreplicated sink may lose data and stops service when unavailable. These qualifications defeat the “coordination quorum means preserved application history” attack.

Current unsynced application `Ok` is not silently upgraded by stronger binding/log/sink durability. Lines 36 and 180 require explicit receipt mapping and consumer amendments. Lost replies remain unknown until exact recovery, rather than permission to repeat an irreversible action.

### Permissions, retirement, privacy and mixed versions

EM-10 is honest about local freshness. Every enforcement boundary checks complete applicable policy available there; unavailable proof fails closed. An isolated node cannot know an unseen revocation instantly, and a lease does not exempt it from reevaluation. M3 must make its canonical policy decisions deterministic in order; a future implementation cannot validate each replica against unrelated discovery/time snapshots and still claim that profile.

Retirement/incarnation and retry history remain retained. Exact retry may recover an old committed outcome subject to current disclosure permission; it does not reactivate the resource. More detailed namespace-reuse policy is an explicit following-contract decision, not an implicit promise of safe reuse today.

Privacy did not become authority by replication. The impact table retains authenticated private keys, local read enforcement and operator-approved placement. M5 names plaintext/operator approval as a cost. An M3 data-bearing electorate would likewise need approved payload custody; group membership alone cannot substitute for `replica.hold`. Bounds, proof/referral work and retained evidence require the following profile to refuse capacity explicitly.

EM-11 rejects restoration from uncertain journals, incomplete cuts, cloned custody or rollback evidence. Familiar keys do not establish completeness. EM-12 rejects activation beside any unfenced legacy writer, including an offline writer that can later mutate. Rollback must preserve enforcement. No “never worse than the status quo” claim licenses bypass; the packet requires reconciliation or an honest loss report before preserved-history receipts.

## 3. Risks and next action

My independent conditional ranking is:

| Required profile | Ranking and conditions |
| --- | --- |
| Home-dependent pending edits accepted; movement optional; no approved common sink already present | **M1, M2, M3-D, M5, M3-E, M4-S**. M4-L remains blocked; M0 is ineligible. Additional authority/integration is not justified by this requirement alone. |
| Cooperative identity-preserving movement required | **M2 first**. M3-D follows for an application already needing replicated command order; M5 follows when a common sink is practical. M3-E/M4-S inherit sink recovery work. M1 is ineligible for movement; M4-L remains blocked; M0 is ineligible. |
| Automatic takeover preserving acknowledged data; self-operated independent data domains accepted | **M3-D first**, then M5 if its data/effect contract is suitable. M3-E follows where external effects require both application order and a sink; M4-S is a conditional optimization. M1/M2 cannot meet lost-home takeover; M4-L is blocked; M0 is ineligible. |
| Approved common sink already owns all relevant data and effects | **M5 first**. M3-E becomes preferable only when an additional ordered application protocol supplies a required invariant. M4-S follows only where its lease semantics justify the extra lifecycle assumptions. |

These orders flip with existing operational authority and the actual effect class. Proven lease environments and a demonstrated need to reduce per-operation coordination could promote M4; no such evidence exists here. Exactly two independent voters, no extra authority, either-loss progress and exclusive writes leave no qualifying takeover candidate.

For M3/M4, I conditionally rank **Raft ahead of Multi-Paxos for a new ordered-log design** because the cited design provides one coherent account of log recovery, configuration transition and client interaction. This is a specification/integration preference, not a safety or performance superiority claim. Multi-Paxos can take first place when the team has a complete reviewed reconfiguration/recovery implementation and conformance evidence; agreement alone is insufficient. [Raft §§6, 8](https://raft.github.io/raft.pdf); [Paxos Made Simple §§2.5, 3](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf).

I rank **embedded scope/shard M3-D groups first for self-operated application-data ordering**, and **a separate service first when its independently operated authority and approved sink already satisfy the requirement**. Packaging cannot fix absent data, unsafe clocks or unfenced stores. Bounded group lifecycle and authorized mapping are prerequisites to embedded scaling.

Residual risks remain substantial but explicit: custody and durable atomic transitions, canonical authorization/time inputs, terminal-effect recovery, safe membership and snapshots, privacy/capacity profiles, and comprehensive legacy exclusion. None is operationally proven by this review. Missing measurements, executable models or formal proofs alone do not block this analytical decision packet.

The single next action is the owner decision described at evaluation line 193: select the required outage/movement behavior, authority/data failure domains and acknowledgement guarantee, and acceptable common-service/effect dependency. The resulting mechanism then needs its own contract, canonical amendments, failing conformance schedules, adapter evidence and review. This GO supplies decision evidence only.
