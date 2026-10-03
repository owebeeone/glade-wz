# Glade Raft bootstrap, growth and unstable networks

Date: 2026-10-03. Status: **DRAFT recommendation for Consistency/Safety review**.
The owner requires Glade to operate with one, two, three or more nodes, including
participants that start disconnected and later discover each other. Three is a
resilience recommendation, not a minimum. This selects the cardinality objective,
not root custody, a production carrier, receipt guarantees, wire schemas or activation.
This document proposes semantics; it changes no API, package, launcher or running node.

## 1. Sources, authority and evidence boundary

The baseline is workspace `9c0510690050e6edc928c09ecc7b414baba58ba1`, Glade
`c65a6e87f0c257c15de8db080c29d365a883af85`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, and external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`. Parent-managed review amendments
may accompany this draft; reviewers MUST bind their exact complete source tuple.

Controlling sources are [RA-001–012](GladeRaftAdoptionContract.md), especially
001/003/004/006/007/010/011/012; [Q3 configuration/snapshots](GladeRaftConfigurationSnapshotContract.md)
§§1–5; [production decisions](GladeRaftProductionProfileDecisions.md);
[production integration](GladeRaftProductionIntegrationPlan.md); and
[qualification](GladeRaftQualificationPlan.md). Q3 proves a private numeric 1–4
fixture with three initial voters, trusted configuration authority and controlled
campaigns. It does not prove genuine custody, automatic elections, arbitrary
cardinality, five voters, real signing, independent machines or this design.
Q4-A qualifies a limited legacy Store interlock; Q4-B0 accepts a refusing-carrier
contract/scaffold. Neither is production bootstrap or Raft activation evidence.

Allocation follows [BuildEntry](GladeBuildEntry.md), the external
[Gyld declaration](</Volumes/projects/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py>),
[architecture revision 3](arch1/GladeArchitecture.md),
[library policy](LibraryBoundaryAndTestingPolicy.md), and
[package architecture](GladePackageArchitecture.md). Canonical
[authorization](glade/GladeAuthzModel.md) §§3a/3b/4a/7a/7b and
[discovery](glade/GladeDiscoveryModel.md) §§0/2/3/4/7 remain controlling.
No test was added or executed for this document; the matrix below is future work.

## 2. Identities and what discovery can tell us

Canonical scope includes authenticated creation-root ancestry, share and
zone/binding identity; a protected resource additionally binds declaration,
parameters and incarnation. Labels, browser origins, tab IDs and equal settings
keys do not prove equal independent roots. Private `self` MUST derive from the
authenticated requester at every hop; caller-supplied principal text is insufficient.

| Concept | Meaning and boundary |
| --- | --- |
| Discovery candidate | An authenticated permitted route to investigate; not proof of membership, global absence or current authority. |
| Member | A node admitted by the retained group configuration; learner and voter roles differ. |
| Voter | An authorized complete-data replica counted by the named committed configuration, never by the visible peer count. |
| Leader | A temporary Raft role within that configuration; term changes do not grant requester rights or move resources. |
| Resource home | An application placement binding and generation changed only by ordered qualified movement. |
| Creator/root | The creation-root authority from which governance derives; a Raft majority cannot replace it. |

Dynamic resources/scopes MUST use authenticated retained resource-to-group
mappings, not configured per-resource homes. New resources in a known scope
resolve concurrent canonical names by ordered `Create` in its authorized history.
Discovery can carry validated mapping evidence or referrals, but discovery lease,
epoch and tie-break changes MUST NOT activate homes or rewrite those mappings.
Initial grouping MAY place several resources in one group; general split/merge
and fleet-scale placement are separate contracts, not an implied algorithm here.

## 3. Bootstrap: fresh roots versus known scopes

Recommended fresh bootstrap is one authorized creator custody lane per canonical
namespace. Before any transmission or exclusive store creation, it MUST durably
retain one exact signed genesis intent binding root/scope, declaration/profile,
group, initial voter configuration and resource-to-group mapping. A one-voter
initial configuration is permitted only when the selected receipt profile allows it.
The bootstrap store MUST be created exclusively; durable initialization and its
exact outcome MUST be recoverable before claiming usable authoritative state.

The custody lane MUST NOT issue conflicting genesis for that identity. Signing
alone cannot ensure uniqueness: multiple devices sharing a key need an actual
serialized issuance/custody mechanism. Recovery needs an independently trusted
antirollback intent/configuration floor outside the rollbackable local store.
Loss of custody or that evidence MUST block same-identity issuance/recovery;
it MUST NOT be repaired by electing among signed contradictory genesis records.
Cloned credentials, malicious/rolled-back custody and Byzantine voters exceed
crash-fault Raft's guarantee; their detection/containment boundaries MUST be stated.

If creation is interrupted or its reply is lost, bootstrap remains **unknown**.
Only lookup/retry of that exact retained genesis intent may recover its outcome.
A new group, new initial voters or changed bytes under the same identity MUST NOT
be substituted. Exclusive-create failure does not prove the prior intent failed.
Publication ordering, storage faults and custody rollback require concrete tests.

A participant that knows a scope MUST authenticate and recover its existing
genesis/mapping before joining. Missing files, empty discovery, an expired claim,
waiting several election timeouts or failure to contact the creator MUST NOT
authorize genesis, voter reset or reconstruction from a label. It stays pending
or quarantined according to the evidence, while permitted cached reads remain possible.

Two disconnected participants may each create genuinely **fresh independent
roots**; those resources remain distinct even if their labels match. If they both
intend the same canonical scope/name but lack its unique creation/naming authority,
their requests MUST remain pending. They MUST NOT each commit a replacement
singleton and later merge histories. A disconnected device may use an already
retained authorized genesis only under its nonconflicting custody contract.
Conflicting same-scope genesis/mappings MUST quarantine both recognition paths
pending explicit reconciliation; label ranking, higher term or first discovery
cannot pick a winner. Creating a new identity is a distinct user operation and
does not preserve the old identity/history or make its pending requests terminal.

## 4. Joining and growing the group

1. Discover a permitted route; authenticate peer, requester and bound genesis.
   Validate group/profile/configuration ancestry and hosting policy before transfer.
2. Submit an authorized exact configuration intent to admit the node as a learner.
   Retain that complete intent and its accepted/refused outcome in the ordered history.
   Admission requires the current voter quorum; discovering a peer does not admit it.
3. Transfer original complete log or coherent snapshot plus suffix, payloads,
   policy, retry outcomes, tombstones, configuration and pending history evidence.
   Durably publish, replay and cross-validate the actual state through a named cut.
4. Produce authenticated readiness evidence binding group/profile, node identity
   and incarnation, exact cut/term and complete durable/applied state. A counter,
   client assertion, stale snapshot or readiness from a different incarnation fails.
5. An authorized promotion intent names its expected configuration version and
   exact complete target voter set. At its ordered predecessor cut, revalidate
   policy, readiness and home constraints; changed eligibility becomes a retained
   refusal. Enter joint configuration, then explicitly leave it in a separate intent.

Learners cannot vote, supply a voter quorum or independently accept authoritative
writes. Their presence does not change a one-voter receipt into a two-copy receipt.
Read-only delivery from a learner requires complete validated state, current local
disclosure permission and an honest applied frontier; a current-state claim additionally
requires the qualified read barrier. Joining must not leak private data to an
unaccepted operator: host, voter and requester rights remain separately authorized.

Onboarding MAY be automatic: a signed creation-rooted growth policy can delegate
learner-admission/promotion proposals to a host when explicit operator, identity,
capacity and readiness predicates hold. Such proposals still require ordered
evidence/policy validation and the applicable configuration quorum; discovery
alone is never the predicate proof. No human confirmation per resource or node
is inherently required. Growth-policy defaults, target size and whether a second
node stays a learner are provisional choices for review/selection. A target of
three does not require finding three nodes before authorized singleton operation.

Configuration intent identity MUST bind scope/group/incarnation, authenticated
authority, durable request ID, expected configuration version and exact canonical
change bytes. Retain the full original intent, accepted/refused outcome, original
index and resulting full configuration. Exact retry recovers that original result
before stale-version/readiness checks, subject to current disclosure permission;
changed bytes and another principal's same ID do not alias. A timeout, dropped
reply or cancellation remains pending/unknown, not evidence of noncommit.
At most one transition is unresolved; retries recover it, incompatible nested
changes refuse. No timeout silently rolls back an admitted transition.

During joint authority, commitment MUST require a complete-data majority of both
outgoing and incoming voter sets. Learners cannot stand in for either majority.
The concrete carrier's reviewed configuration-activation boundary governs when
EnterJoint becomes committed/applied; the host MUST NOT invent a separate Raft
commit rule. An external transition receipt claiming the joint data guarantee
MUST additionally have verified the required complete durable retention in both
sets. If the carrier commits its entering entry under the predecessor configuration,
that alone MUST NOT claim the new guarantee; required additional retention evidence
and unknown/failure behavior belong in its adapter contract before implementation.
LeaveJoint commits under joint authority; only then does the stable target govern.
Application receipts issued in a joint phase MUST name and satisfy both sets.

For `{A}` → `{A,B}`, the joint majorities are A and A+B: both must participate.
For `{A,B}` → `{A,B,C}`, the outgoing majority is A+B and the incoming majority
is any two of A/B/C; C cannot replace a missing outgoing voter during transition.
The same rule applies to larger sets and replacements. A crash after entry commit
but before apply/reply MUST recover the exact stage/outcome and persisted hard state;
recovery while joint MUST retain both sets, without inferred auto-leave or rollback.

Preserve Q3's home-voter constraint: EnterJoint removing a live resource home
from the target MUST refuse `HomeInUse`. At the **actual LeaveJoint predecessor
cut**, recheck every live home against incoming voters, including placements queued
since admission. A stranded home produces an ordered retained refusal, unchanged
configuration version and no carrier configuration apply. Qualified movement to
a ready remaining voter, or retirement, must precede a **new** exit intent; exact
retry of the old refused intent still returns that refusal. Membership/leadership
MUST NOT silently change resource home or generation. Nonvoting homes need a
separate amended product contract and are not introduced here.

## 5. Cardinality, failures and network instability

Counts below describe **stable complete-data voter sets**, not discovered nodes.
Guarantees require retained authorized membership, compliant crash-fault replicas,
complete data, the named persistence profile and eventually useful communication.
Growth MUST NOT retroactively upgrade an earlier one-domain receipt. Any later
replicated-coverage claim MUST identify its verified catch-up frontier and named
guarantee separately from the original acceptance promise.

| Voters | Majority | Lost voter domains permitting continued writes | Honest consequence |
| --- | --- | --- | --- |
| 1 | 1 | 0 | Local election/self-vote; accepted history relies on one data domain. |
| 2 | 2 | 0 | Both required; either loss or separation stops authoritative writes. |
| 3 | 2 | 1 | Remaining majority can continue under the same selected guarantee. |
| 4 | 3 | 1 | No improvement in tolerated losses over three; larger quorum. |
| 5 | 3 | 2 | Two losses tolerated only if three complete independent domains remain available. |

For more voters, majority is `floor(n/2)+1`; deployment/node count alone never
proves independent failure domains. Receipts MUST name membership version/cut,
stable or joint quorum rule, complete-data retention and persistence/failure promise.
Machines sharing one disk/power/operator dependency require an honest declared
fault-domain profile; local processes are qualification fixtures, not independence.

Alternative for a two-node installation: retain A as the sole voter and B as a
complete learner. This can preserve A's write availability when B disconnects,
but B cannot take over if A disappears. It requires an **explicitly selected
one-voter receipt/failure profile**, not automatic degradation from two voters.
Demanding synchronous second-copy retention would additionally stop receipts on
B loss; it is a separate promise, not implied by learner catch-up. Recommend
three voters for one-loss progress; one/two voter operation remains supported.

Partitions may cause repeated automatic elections and split votes; election
timeouts affect progress, never authority or proof that other nodes do not exist.
Owned injected time/randomness/work/transport must drive carrier qualification.
A minority, isolated old leader or stale discovery route MUST NOT newly accept an
authoritative mutation. It MAY relay or recover the exact original outcome of a
previously committed request, subject to current disclosure permission; this does
not claim current state or require a fresh data quorum merely to disclose retained
history. Current-state claims still require the qualified barrier. Higher terms
alone grant no rights. After links heal, Raft
reconciles uncommitted suffixes and catches up committed data; it does not merge
independently committed genesis groups. Continuous instability may prevent progress
indefinitely; do not claim bounded completion without a stated timing/fairness profile.

With no quorum, clients may retain authenticated exact pending intents and permitted
cached reads labeled with their frontier. A current authoritative read needs a
qualified quorum-confirmed barrier and local apply through it; a discovery lease,
leader belief or stale prefix is insufficient. Exact retry after reconnection
recovers the saved accepted/refused result, or remains unknown until recoverable.
Data freshness does not imply unseen revocation freshness; every hop rechecks
locally known disclosure evidence and states that boundary.

Loss of one voter in a two-voter group MUST NOT trigger unilateral 2→1 shrink,
fresh genesis, identity-changing reset or promotion from the visible peer set.
Authorized shrink still needs the applicable old/joint quorums. Missing/corrupt/
rolled-back stores MUST quarantine; recognizable keys cannot prove complete state.
Rejoin needs authorized recovery into a new owned path/incarnation and verified
catch-up, with no cloned active voter identity. Snapshot/configuration recovery
must preserve exact histories and an independently trusted rollback floor.

## 6. Requirement-to-future-executable-test matrix

Every case MUST record exact inputs, deterministic seed/schedule, full original
intent/outcome oracle and assertions. Compiling specifications start against
refusing providers; observed behavioral RED precedes implementation. No row is
claimed implemented or passing by this draft.

| ID | Requirement | Required success / failure / edge witnesses |
| --- | --- | --- |
| BG-001 | Retained unique genesis; RA-001/011 | One-voter exact genesis succeeds; wrong signature/namespace refuses; kill before/after retention, exclusive create, sync and outcome then lose reply: only exact intent recovery, never changed initial voters. |
| BG-002 | Known scope is never empty-view bootstrap; RA-001/010 | Disconnected known device later joins; empty directory, missing store, expired claim and arbitrarily delayed election wakeups cannot create/reset; unsupported custody/recovery stays pending. |
| BG-003 | Independent roots and conflicts are distinct; RA-001/002 | Same-label fresh roots remain distinct after discovery; same-scope simultaneous creation without naming authority stays pending; conflicting signed mappings quarantine without term/label ranking or history merge. |
| BG-004 | Dynamic ordered resource mapping; RA-001/002 | Two concurrent creates in one known history yield one binding or explicit ordered conflict; stale referral cannot change group/home; a new scope is not proof of global absence. |
| BG-005 | Authenticated separate roles; RA-006 | Certified authorized learner joins manually or under signed automatic growth policy; absent/stale predicates, wrong private self, revoked device, unaccepted operator and valid-signature/no-membership grant refuse before data transfer; majority cannot mint root/admin/requester rights. |
| BG-006 | Complete durable learner readiness; RA-003/007/010 | Actual snapshot+suffix restores full state then promotion succeeds; unavailable, metadata-only, corrupt, stale-cut and wrong-incarnation proofs fail; delayed readiness/policy revocation before ordered promotion produces exact refusal. |
| BG-007 | Exact configuration intent/outcome; RA-004/010 | Lost accepted and refused replies recover full original results; changed bytes/principal collision fail; commit-before-apply/outcome-before-reply crashes remain recoverable or unknown, never fabricated noncommit. |
| BG-008 | Joint intersection and cardinality; RA-010 | Automatic-election actual carrier journeys 1→2→3→5; incoming-only/outgoing-only joint majority cannot receipt; stable 1/2/3/5 partitions obey table; unavailable promoted member and nested transitions preserve safe pending state. |
| BG-009 | Configuration receipt fault boundaries; RA-003/010/011 | Crash at entering/exit entry retention, commit, apply, outcome and reply; reject old-set-only evidence presented as new/joint data guarantee; restart reconstructs both sets and exact refused/accepted receipts, no auto-leave. |
| BG-010 | Home constraint survives membership; RA-005/010 | Move/retire then remove succeeds; entry home removal and placement queued before actual exit refuse HomeInUse identically on every replica; snapshot/joint restart and old refused retry retain unchanged home/generation/result. |
| BG-011 | Safety under unstable networks; RA-003/004/007 | Eventual heal permits catch-up/election/exact retry; repeated loss/dup/reorder/asymmetry/late work may remain pending; isolated old leader and two-voter either loss cannot newly accept; permitted original committed outcome recovery remains distinct from current-state claims; uncommitted suffix repair preserves all committed outcomes. |
| BG-012 | Explicit read/availability profiles; RA-003/006/007 | Cached allowed frontier reads work without quorum; current barrier waits/refuses; unseen revocation boundary stays honest; optional 1-voter+learner profile never promises automatic failover or silently counts learner as voter; later catch-up coverage cannot retroactively upgrade original one-domain acceptance. |
| BG-013 | Recovery without clone/reset; RA-001/011 | Authorized new-incarnation restoration succeeds; missing/corrupt/valid-old store, custody rollback, duplicate active identity and lost trusted floor quarantine; no unilateral 2→1 or changed identity masquerades as old history. |
| BG-014 | Legacy exclusion remains separate; RA-012 | Fresh identity tests cannot qualify migrated identity; real all-writer inventory, divergent baseline/pending-unknown reconciliation and old-binary/rollback exclusion must pass before activation; Store seal alone fails the closure claim. |

## 7. Proposed allocation, amendments and next gates

Binding owns canonical interpretation; Admission validates authenticated ingress,
genesis/join/configuration evidence; pure Policy evaluates supplied rooted evidence.
Records owns ordered mappings, complete outcomes, membership and recovery;
StorageAdapter owns physical coherent publication and hard-state sequencing.
Directory/DirectoryRules supplies permitted routes/coverage, not membership.
Delivery exposes honest frontiers; Runtime owns explicit work/time/randomness;
NodeAssembly injects providers/custody inputs without becoming a new policy oracle.
Supplier/application source retains effect fencing. This proposes responsibilities,
not new libraries; narrow contracts/classifications/Gyld allocation require review.

| Exact existing clause | Proposed reconciliation; no silent supersession |
| --- | --- |
| AdoptionContract §1 fixed three-voter executable profile; RA-001/010; QualificationPlan Q1/Q3 | Preserve historical private evidence; add separately qualified variable-cardinality production/bootstrap profile. Do not enlarge numeric Q3 claims. |
| ProductionProfileDecisions recommended deployment row and “What each alternative changes”; IntegrationPlan proposed first profile | Three independent voters becomes resilience recommendation; support 1/2/3/more with explicitly named voter/receipt promise. Other recommendations remain owner-unselected. |
| Q3 §§1/2/3 node universe, trusted genesis/readiness, config identity and HomeInUse | Replace fixture trust/numbers only through real canonical contracts; preserve complete exact outcomes, learner/joint rules, both home checks and no nonvoting-home inference. |
| DiscoveryModel §§0/3/7; AdoptionContract §3 routing amendment | Keep local discovery folds/claims; add candidate-versus-group evidence and forbid absence/epoch from authorizing genesis, voter change, home activation or current reads. |
| AuthzModel §§3a/3b/4a/7a/7b; AdoptionContract RA-006 | Preserve creation ancestry, signed governance, private self and operator policy; specify custody, ordered membership/policy frontier and supported authentication without majority ownership. |
| AdoptionContract RA-003/004/007/011/012; IntegrationPlan Q4-C/D/E | Bind variable/joint receipts, unknown/retry and recovery floors to exact canonical bytes; unchanged mandatory production storage, legacy cut and independent-domain gates. |

Next: peer-blind Consistency/Safety review of this semantic proposal and exact
amendment trace; then select semantic options without treating this draft as an
owner ruling. Specify custody/antirollback and initial authority recovery, authentic
genesis/config/readiness evidence, receipt profiles and canonical schemas. Amend
the actual Gyld allocation and Taut-generated versioned bytes, both clients and
affected consumers; compiling RED plus applicable Consistency/Safety/Surface
reviews precede implementing ports. Existing unactivated identities keep their
contracts. Qualify actual adapted carriers, disk/transport/crypto, cardinality
journeys and independent-domain failures before production acceptance; activation
still needs reconciled legacy cut, all-writer exclusion and separate approval.

Open owner inputs are custody/issuance/recovery, actual hosts/failure domains,
supported authentication/read service, optional learner-only second-node promise,
retention/capacity and migration/rollback cut. Canonical evidence schemas and the
carrier activation/retention boundary are design gates, not presumed answers.
No deployment, enrollment, group merge, disposal, rebuild, commit or push occurs
because this draft exists.
