# Glade Raft adoption contract — protected application history

Date: 2026-10-03. Status: **draft scoped amendment and qualification contract**.
The owner selected Raft as the next qualification direction following the
[ownership evaluation](GladeOwnershipMechanismEvaluation.md) and its
[accepted analytical review](GladeOwnershipMechanismEvaluation-ReviewCycle.md).
This authorizes contract/proof work. It does not ratify production deployment,
select a production crate, amend frozen discovery bytes, or activate takeover.

## 1. Scope and provisional profile

The proposed new scope is M3-D: Raft orders the **actual protected application
commands and complete authoritative data**, including outcomes. It is not a
quorum check followed by an independent local append. One fixed known scope,
one authorized group and three data-bearing voters form the initial executable
profile. Three logical voters permit deterministic majority/failover experiments;
they are not an owner ruling that three machines, disks or operators suffice in
production. Memory persistence establishes protocol/application ordering only.

Canonical scope includes share identity and the declared zone/binding identity,
including authenticated private-principal derivation where applicable. Equal
labels never identify equal independent roots. The proof initially orders a small
complete byte payload rather than every Glade shape or supplier. Declaration,
configuration, policy and retry history belong to the protected state; arbitrary
external effects do not become safe because their intents enter Raft.

Faults are crash/restart, partitions, loss, duplication, reordering and delayed
work; network delay is unbounded. Progress requires an available authorized
quorum, complete data and eventually useful communication. Byzantine voters,
contradictory root signatures, cloned credentials and malicious storage are not
solved by crash-fault Raft. No lease, actor/lifecycle framework, generic lock
service, million-node placement design or production migration is introduced.

## 2. Normative requirements

The IDs below are stable. Their qualification phases and EM journey mappings are
in [the plan](GladeRaftQualificationPlan.md). A passing partial witness MUST NOT
be reported as satisfying the entire requirement.

| ID | Required behavior |
| --- | --- |
| RA-001 | Fresh genesis MUST use one authorized signed root intent binding canonical scope identity, declaration/profile, group identity, initial configuration and resource-to-group mapping. The issuer MUST retain the exact intent before transmission and MUST NOT issue conflicting genesis for that identity. A known-scope join MUST authenticate and recover this mapping; an empty local directory/store MUST NOT authorize genesis or reset. Conflicting same-scope mappings MUST quarantine, not rank by label, term or discovery epoch. |
| RA-002 | Group leadership MUST be separate from resource home and resource generation. A term identifies Raft leadership, not a grant, application generation or takeover permission. `Create`, `Mutate`, `BeginMove`, `Activate` and `Retire` MUST become application commands in one committed order. Concurrent known-scope creates MUST resolve through that order to one canonical name binding or an explicit conflict. |
| RA-003 | Every authoritative mutation MUST include the complete required canonical payload and reach a data-bearing quorum under the named persistence profile. An application receipt MUST follow ordered application and retention of its exact terminal outcome. Production acceptance MUST additionally satisfy the selected durable quorum/application guarantee. A proposal, leader observation, metadata majority or memory acknowledgment MUST NOT claim that guarantee. |
| RA-004 | Retry identity MUST bind canonical scope/resource/incarnation, authenticated requester principal, durable request ID and exact canonical command bytes/digest. Changed bytes under the same identity MUST fail. Exact retry MUST recover the original outcome before current-generation/precondition rejection, subject to current disclosure permission, without applying another mutation. Same IDs in another principal/scope namespace MUST NOT alias. Lost reply, timeout or cancellation MUST remain unknown until outcome recovery; refusal MUST NOT manufacture evidence of noncommit. |
| RA-005 | Ordered application MUST validate current resource generation, home, declaration, canonical operation/chain constraints and command eligibility. A delayed uncommitted old-home command ordered after activation MUST be refused. A committed pre-cut command MUST apply/recover in its original order. Home movement MUST close old-generation admission at a committed cut and activate only after verified successor application readiness through that cut, including policy and retry outcomes. A preflight check followed by independent append MUST be rejected by a counterexample test. |
| RA-006 | Authorization MUST remain creation-rooted and derive from authenticated signed governance and pure policy evaluation. Deterministic application MUST consume a complete explicitly identified policy/evidence frontier and explicit time inputs when relevant; it MUST NOT consult local discovery, ambient clocks or a live grant oracle. Governance changes affecting this profile MUST enter the same defined order/frontier. Eligibility to vote/host MUST NOT grant requester verbs. Read/disclosure checks remain local at every serving hop; private `self` scope derives from the authenticated principal. Remote unseen revocation MUST have an honest freshness boundary. |
| RA-007 | Available decision quorum MUST NOT imply available application data. A metadata-only witness MUST NOT count toward the stated data-bearing receipt; missing acknowledged history MUST block activation. Replicas/projections MUST NOT admit independent authoritative writes. Cached/readable projections MAY support permitted offline reads, but their receipt MUST identify their frontier; a linearizable read MUST use a separately qualified barrier. |
| RA-008 | External effects MUST remain outside M3-D acceptance unless a separately reviewed M3-E/sink contract atomically enforces generation/sequence, deduplicates execution and retains recoverable outcomes. A queued shell/build/Git action or independently cached fence MUST NOT become exclusive through Raft ordering alone. Unresolved effects at a cut MUST block effect-bearing activation. The initial proof MUST exercise rejection/exclusion, not claim a real sink implementation. |
| RA-009 | Retirement MUST be an ordered durable fence retaining identity, generation and exact outcomes. Later creates/mutations/moves for the retired incarnation MUST fail, while permitted exact retries recover their historical outcome. Retention/compaction MUST preserve retry, retirement, policy and configuration evidence needed for safety. Exhausted bounded capacity MUST refuse explicitly rather than silently evict that evidence or recreate an identity. |
| RA-010 | Membership MUST be authorized by the bound configuration, not the discovered peer set. Admission MUST first authenticate mapping and catch up as a nonvoter. Reconfiguration MUST preserve intersecting decision authority, including joint consensus where selected. Lost replies MUST recover the original configuration intent. Two-voter either-loss availability MUST NOT be claimed; lost quorum MUST NOT authorize unilateral reset. Membership change is deferred from the first executable profile. |
| RA-011 | Recovery/snapshots MUST preserve the committed/application frontier, complete payload/state, retry outcomes, generations, retirement, policy and configuration as one coherent recoverable history. Persisted hard state, entries and snapshot boundaries MUST obey the selected Raft adapter's ordering. Incomplete, rolled-back or conflicting recovery evidence MUST quarantine; recognizable keys/labels MUST NOT prove completeness. Storage faults and process versus power-loss guarantees require concrete adapter evidence. |
| RA-012 | Activation into an existing Glade identity MUST exclude every legacy authoritative writer, establish a verified baseline cut and retain the fence across rollback. A flag, new advertisement or upgraded reader MUST NOT authorize activation while a legacy path can still mutate. Divergent/unknown legacy history MUST be reconciled explicitly or block preserved-history claims. Production integration/migration remains a later gate. |

The production persistence profile MUST state what an acknowledgment survives,
how many independent data domains hold the complete command/outcome, and whether
the guarantee continues after one domain is lost. These choices remain open.
Any reduced post-failure guarantee requires an explicit approved profile rather
than silently counting one surviving copy as the original quorum receipt.

## 3. Proposed canonical reconciliation

The following are **scoped amendment proposals**. This document does not edit or
assert ratification of the listed sources. Current contracts govern all
unactivated identities. An activation review MUST resolve each affected clause
and its consumer tests before introducing the new production path.

| Controlling source/clause | Proposed amendment boundary |
| --- | --- |
| [Buy/build](GladeBuyBuildMatrix.md) §1 D-06 and §2.B R7 | Preserve signed local discovery folds, renewable advertisements and the three discovery layers. Qualify “no Paxos/Raft cell” and “not on critical path” to discovery resolution: protected application acceptance gains a separately declared ordered authority dependency. Discovery results route; they never authorize takeover. |
| Buy/build §2.B R9 and §4 Q12 | Record M3-D application ordering as new scope beyond the deferred shard-reconfiguration question. Neither clause presently approves it. Keep registry growth/reconfiguration separate from fixed-group proof and later production group lifecycle. |
| Buy/build §2.D R16; [WorkspaceDirectory](glade/GladeWorkspaceDirectory.md) §4 and WD-8 | Keep local locks for one physical checkout and advertisements/repair for routing/retention. Explicitly qualify their sufficiency: separate physical copies need the added M3-D admission fence. Raft does not make filesystem/build effects safe. WD-8 `global` dedup and idempotent replica repair do not elect exclusive authority. |
| [DiscoveryModel](glade/GladeDiscoveryModel.md) §0/3/7 | Keep offline fold/routing behavior. Qualify higher-epoch/stale-holder scenarios for protected identities: an advertisement epoch cannot advance application generation or activate a successor. Add a routing-versus-acceptance consumer witness. |
| [SubstrateV1](../glade/dev-docs/GladeSubstrateV1.md) §2 and §6 W1/W2/W7; [CrossNodeWritesPlan](../glade/dev-docs/GladeCrossNodeWritesPlan.md) §3 | Qualify network-independent append, convergence-only correctness, live-claim admission and unclaimed-local fallback for the explicitly activated profile. A client may retain an offline pending intent; authoritative acceptance requires ordered application. The protected path MUST bind authenticated requester identity instead of silently inheriting W2's node-principal approximation. |
| SubstrateV1 §6 R1/R2/R7 and W3–W6 | Add distinct versioned receipt/outcome mappings, correlation, exact retry, application cuts and unknown recovery. Existing `Ok` is process-crash retention without fsync and forwarded `Ok` covers A/B; it MUST NOT silently mean a durable Raft/data quorum. Local replay completeness MUST NOT imply a group-wide committed/application frontier. |
| [Authz](glade/GladeAuthzModel.md) §1/3a/3b/4/4a/7a | Preserve root ownership, issuance ancestry, signed governance, local read checks, per-share policy, derived private scope and approved operator placement. Amend only ordered protected-write eligibility/freshness; a consensus majority is not an administrator or permission oracle. |

## 4. Architecture allocation and open decisions

The supplemental allocation follows [BuildEntry](GladeBuildEntry.md),
[candidate revision 3](arch1/GladeArchitecture.md) and its
[Gyld declaration](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py).
No external Gyld file is changed. Binding owns canonical scope/declaration;
Admission and Policy own identity/evidence validation; Directory and
DirectoryRules retain routing. Records is the proposed host for M3-D ordered
accepted history, outcomes and recovery through narrow injected persistence and
message ports. This qualifies its existing “source commits stay outside” clause
only for this new application-history profile. Supplier/Application retain real
source/effect fencing. Delivery consumes declared application frontiers; NodeAssembly
selects providers; Runtime owns work without becoming an authority source.

Port signatures, complete serialization, governance-frontier selection, root
custody, group cardinality/lifecycle, production failure domains, persistence
guarantees, linearizable reads, external sinks and production crate selection
remain decisions. Classification/dependency proposals require independent review
under [LBT-001–012](LibraryBoundaryAndTestingPolicy.md) and
[package architecture](GladePackageArchitecture.md), before implementation.
