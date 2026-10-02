# Glade resource home selection — alternatives for decision review

Date: 2026-10-03. Status: **DRAFT decision packet; no alternative selected or implementation authorized**.

Owner directive: clients MUST dynamically add resources and scopes and discover
one another. Manually configured resource-to-home mappings are a comparison
baseline, not a viable general solution. This review concerns that decision
alone. It does not freeze a wire/API, select a dependency, or approve migration.

## 1. The decision and its scope

Which mechanism assigns, advertises and, when permitted, changes the serving
home of a dynamically created resource? Which guarantees may a client rely on
during concurrent creation, partitions, restart and transfer?

Three concepts MUST remain distinct:

- **Authority owner:** the resource's root and its authorized delegation/policy.
  Placement or an election MUST NOT create ownership or new grant authority.
- **Serving home:** the node allowed to admit authoritative operations for a
  placement unit. This is a replaceable role, subject to authorization.
- **Replica/discovery participant:** a node holding data or placement records.
  Possession, reachability and a directory entry do not confer serving authority.

The current node selects a holder at workspace/share level. Settings use a
zone address `(share, glade_id, key)` within that share; browsers are writers,
not homes. The review MUST choose whether home units stay whole shares or
become explicit resource/shard units. It MUST NOT silently give every settings
key its own lock or make the transport endpoint ID the resource identity.

Three axes are independent: dynamic creation/discovery; relocation; automatic
failover. Supporting the first MUST NOT be described as implementing the others.
Bootstrap trust, invitations and authorized candidate discovery remain necessary
even when no per-resource home is preconfigured.

## 2. Authority and evidence base

Sources in the root repository are pinned by the review's root SHA; member
sources below use these exact commits. Working-tree changes are out of scope.

| Source | What it controls or establishes |
| --- | --- |
| [GladeAuthzModel.md §3a](glade/GladeAuthzModel.md) and [DecisionLog.md GDL-034](DecisionLog.md) | Creation roots authority; delegation/revocation rules. Governance M-of-N approval is distinct from crash-fault consensus. |
| [GladeWorkspaceDirectory.md §4 and WD-8](glade/GladeWorkspaceDirectory.md) | A physical working copy is fenced by its local lock; claims route traffic. Home-node availability is a role. The historical mechanism is not general multi-machine mutual exclusion. |
| [GladeDiscoveryModel.md §0/§3](glade/GladeDiscoveryModel.md) and [GladeBuyBuildMatrix.md D-06/R7/R9/R16](GladeBuyBuildMatrix.md) | Discovery is a local fold of replicated records; consensus discovery was excluded. Some design prose is historical; existing code is not inferred from it. |
| [GladeSubstrateV1.md §6](../glade/dev-docs/GladeSubstrateV1.md), Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b` | Holder-directed client writes and the limited meaning of `Ok`; no automatic data-transfer guarantee. |
| [GladeCrossNodeWritesPlan.md](../glade/dev-docs/GladeCrossNodeWritesPlan.md), same Glade commit | Equal-epoch ranking agrees for the same eligible live claims at a common evaluation time. Lease filtering remains reader-relative, so identical replicated records can yield different routing answers; ranking is not partition-safe acquisition or data repair. |
| [RegistryContractDraft.md](../glade-discover/dev-docs/RegistryContractDraft.md), discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851` | Local acceptance, exact retry, partial local resolution; candidate placement is not leadership or fencing. These are draft contracts, not proof of a production adapter. |
| [LibraryBoundaryAndTestingPolicy.md](LibraryBoundaryAndTestingPolicy.md) and [GladePackageArchitecture.md](GladePackageArchitecture.md) | Meaningful replaceable boundaries, pure protocol/state roles, TDD, deterministic fast tests and affected consumers. |

The cold-join settings failure and current correction discussed in this chat
motivate the review. Uncommitted CJ changes, launcher changes, discovery review
work and existing scratch reproductions are NOT the review object and are NOT
claimed as accepted implementation evidence here.

External precedent: Chubby separates coarse ownership decisions from operations,
uses a replicated coordination service, and gives holders sequencers that
receiving resources check. This motivates acquisition and fencing as separate
obligations; it does not select Chubby, its namespace API, or its deployment for
Glade. See [Burrows, OSDI 2006, §2.2/§2.4/§2.6](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/).

## 3. Requirements for every candidate

These are proposed review constraints, not amendments to the existing protocol.
A review-ready candidate MUST state how it satisfies each or why a requirement
requires a narrower declared guarantee. Failure to meet the owner directive
RH-01 disqualifies a general solution.

| ID | Required behavior |
| --- | --- |
| RH-01 | Authorized clients MUST create resources and scopes at runtime and discover their placement without manually configured per-resource mappings. A fixed bootstrap or initial trust anchor MAY exist; it MUST be identified separately. |
| RH-02 | Resource identity MUST remain stable across home changes. Canonical namespace/alias rules MUST distinguish two independently created resources from two competing attempts to create the same logical resource. Retry MUST preserve intent and identity. |
| RH-03 | Create, advertise, host, transfer and vote eligibility MUST derive from the appropriate resource/scope authority. A host or voter MUST NOT acquire grant-issuing authority merely by being selected. |
| RH-04 | A partial or empty local discovery result MUST NOT prove global absence or authorize creation of a replacement for a known identity. Unavailable, denied, conflict and unknown outcomes MUST remain distinguishable. |
| RH-05 | Candidates promising exclusive admission MUST identify the serialization/fencing point. Concurrent or stale actors MUST NOT obtain two valid authoritative acceptance paths for the same unit. Local locks fence only the physical resource they actually protect. |
| RH-06 | Every authority-sensitive acceptance/effect MUST check current applicable permission and the serving generation at its enforcement boundary. A signed stale claim, a lease check only at startup, or a router's stale cache MUST NOT suffice. Clock assumptions and inability to validate authority MUST be explicit. |
| RH-07 | `Ok` MUST name its durability/replication guarantee and failure domain. Transfer MUST preserve all operations covered by that guarantee, with a verifiable cut and successor readiness. Ownership quorum alone is NOT a data-replication receipt. Permanent loss of the only locally acknowledged copy MUST be described as data loss/unavailable, never successful empty recreation. |
| RH-08 | Lost replies, cancellation, crash and restart MUST recover exact create/transfer outcomes without duplicated identities or effects; an unknown outcome MUST NOT be treated as an abort. Generation counters and retry identities MUST survive restart. |
| RH-09 | Partitions and unavailable coordination MUST have stated read/write behavior. Browser-local pending edits MAY continue, but MUST NOT be reported as authoritative acceptance. Cached reads MUST state their freshness/completeness limits. |
| RH-10 | Scope/placement discovery MUST bound referrals, verification, retained metadata and failure retries, and obey metadata-disclosure policy. Unknown or unauthorized peers MUST NOT receive private directory content through discovery. |
| RH-11 | Old clients/nodes and persisted claims MUST have an activation/migration rule. A new authoritative acceptance path MUST NOT run alongside an unfenced legacy path for the same identity. |
| RH-12 | Scope deletion/retirement, namespace reuse and recreation MUST have explicit identity/epoch/tombstone rules so delayed claims or retries cannot resurrect a retired resource. Losing local files MUST NOT be mistaken for retirement. |

Discovery MAY remain a replicated local projection even if ownership transitions
use coordination. A discoverable location is a hint until the operation's
acceptance boundary validates its authority. Consensus here assumes a declared
crash-fault membership; it is NOT protection against malicious authorized voters.
Adversarial membership and proof policy require a separate explicit trust model.

## 4. Candidate mechanisms

### H0 — configured home (comparison baseline)

An operator assigns each placement unit a host. Signed advertisements permit
dynamic discovery of that host, but resource creation depends on provisioning
the mapping. It fails RH-01 as the general solution. It remains useful as a
development/recovery reference with no automatic takeover. Fixed voter/bootstrap
configuration is not H0 if resources themselves are assigned dynamically.

### H1 — dynamic creation with a stable home

An authorized creation operation chooses an eligible reachable host and durably
binds a newly minted identity to it. The scope's authorized naming/creation
authority serializes retries and aliases for resources under an existing scope;
new independent scope roots can mint distinct globally unique identities without
first consulting a global registry. The exact namespace profile is a design gate.

The host publishes authenticated placement records; peers discover and route
from local replicated views. A discovered claim does not itself authorize writes.
Creation's durable binding, recovery and an actual resource-enforcement point
must prevent a second host from treating the same identity as fresh. The owner
MAY change hosting permission, but H1 supplies no protocol to move an existing
identity; changing a record alone is not a handoff. A replacement identity is an
explicit fork/import, not restoration of the original resource.

Home loss/partition sacrifices authoritative write progress; clients keep pending
edits. There is no automatic failover. Creating under an unavailable existing
scope may wait; disconnected creation of an independent scope is a different
operation. Simultaneous independent genesis with the same human name yields
distinct identities or an explicit naming conflict, never an implicit merge.

H1 best preserves the current discovery shape. Its hard question is where the
creation binding is serialized and enforced; a local registry empty result and
deterministic claim tie-break do not answer that question.

### H2 — dynamic creation plus explicit fenced transfer

Creation/discovery begin as H1. An authorized transfer changes the home while
preserving the resource identity. A proposed safe sequence is: authorize intent;
quiesce/drain old admission; record an exact durable data/effect cut; prepare and
verify the successor; irrevocably fence the old generation at the resource's
actual enforcement point; activate the next generation; publish placement.
This is an obligation outline, not a proven transaction protocol. Every gap,
lost reply and crash requires a closed recovery grammar in the later design.

For one shared physical resource, its enforceable lock/generation mechanism can
serialize handoff. Separate stores on different machines require an authority
that can revoke the old acceptance path, or a coordinated transfer involving the
old home; independently incrementing a counter at each store is insufficient.
If the old home or necessary enforcement authority cannot participate, transfer
MUST remain blocked/unknown. H2 does not promise takeover of a permanently lost
home merely because an administrator requests it.

This permits planned moves and cooperative rebalance with localized complexity.
It requires explicit ownership/fence records and a caught-up successor. Recovery
without the old home is a separate protocol, possibly H3, not an H2 shortcut.

### H3 — dynamic creation and consensus-backed home selection

An authorized group serializes placement acquisition and generation changes for
a scope or placement shard. New resource entries are created dynamically; this
does not imply one consensus group per settings key or one global group. The
group supplies authoritative acquisition/transition receipts; discovery can
continue publishing signed records for local routing, subject to current
acceptance-time validation.

Automatic failover is conditional on available ownership quorum, successor data
readiness and an enforceable exclusion of the old generation. The later design
MUST choose how a partitioned old home loses its ability to acknowledge effects:
e.g. a correctly specified bounded authority lease, quorum-validated acceptance,
or a shared authoritative sink which validates the generation. These are design
branches, not interchangeable implementations with an already proven guarantee.

Membership/bootstrap, reconfiguration and root-authorized administration MUST
be stated. A two-voter majority requires both; tolerating one voter failure
normally requires three voters in appropriate failure domains. Ownership voters
may be separate from application replicas, but cannot certify absent data. A
new scope needs an authorized bootstrap/recovery route into an existing group or
a deliberately created new group; peers discovering each other does not safely
create overlapping voting groups for one existing scope identity.

H3 offers automatic relocation/failover within its fault assumptions and data
policy, with coordination availability/latency/deployment costs. It requires
owner review of D-06/R7/R9/R16; a separate transition service with projected
discovery has a different impact from replacing all discovery with a service.
No consensus library or service is selected by this packet.

## 5. Equal-footing comparison

| Question | H0 | H1 | H2 | H3 |
| --- | --- | --- | --- | --- |
| Dynamic per-resource assignment | No | Yes, through creation binding | Yes, same initial binding | Yes, through authorized group |
| Discovery | Published records | Published records | Published records + transfer generations | Published records, authoritative transition source separate |
| Stable identity across relocation | Mapping edit alone proves nothing | No relocation protocol | Yes, after safe cooperative handoff | Yes, after serialized transition and data readiness |
| Home unreachable | Unavailable | Unavailable | Unavailable until safe handoff possible | Conditional failover with quorum, fencing and data |
| Exclusive admission basis | Actual resource lock/enforcement | Immutable creation binding + actual enforcement | Recoverable transfer + resource fence | Serialized acquisition + resource fence/authority validation |
| Independent offline creation | Mapping needed | New distinct roots possible; existing-scope naming may wait | Same as H1 | Group/bootstrap policy; existing identities require authority |
| Acknowledged-data safety | Existing local guarantee only | Declared storage guarantee; no auto recovery | Preserve stated guarantee at verified cut | Separate data-commit/catch-up guarantee required |
| Main cost | Manual provisioning | Home dependence and naming authority | Recovery protocol and cooperative progress | Quorum operations, membership, proof/cache and data policy |
| Existing architecture impact | Development profile | Creation/identity/discovery integration | Explicit transfer and fencing amendment | Coordination choice; exact D-06 and trust/placement amendment |

H1/H2 are only eligible if a subsequent design proves their named serialization
and enforcement points. H3 is only eligible if quorum ownership and data safety
are separately specified. A deterministic claim fold over the same eligible
live records is a routing baseline, not a fourth exclusive-ownership protocol;
different lease-expiry evaluations can still yield different answers.

## 6. Shared adversarial journeys and future closure tests

These are design acceptance scenarios to trace in this review, not tests claimed
to have been implemented. Each later contract MUST expose a deterministic fast
state-machine/conformance path; real storage/network adapters also require crash
and affected-consumer checks. Source/API edits MUST start with failing tests.

| ID | Scenario and required distinction | Requirements |
| --- | --- | --- |
| HF-01 | Two authorized creators concurrently request the same canonical resource under one existing scope, with different hosts; require one binding or explicit conflict/unknown, plus exact lost-reply retry. Independently minted roots sharing a label remain distinct. | RH-01/02/04/05/08 |
| HF-02 | A previously uninvolved authorized client learns an invite/root reference, discovers the new scope/resource and finds its home; a partial empty view cannot recreate it. Unauthorized peers get no private metadata. | RH-01/03/04/10 |
| HF-03 | Partition the owner/candidates and deliver stale signed claims; each candidate states which writes can be authoritatively acknowledged and why competing acceptance cannot occur. | RH-05/06/09 |
| HF-04 | Pause an old holder, activate a successor where the candidate permits it, then deliver the old holder's delayed operation/reply. Trace actual effect/store rejection, not just the directory winner. | RH-05/06/07 |
| HF-05 | Replica lacks one acknowledged op when asked to become home. It cannot advertise successful preservation or initialize defaults as if history were empty. Local-only physical loss remains an explicit loss/unavailable outcome. | RH-04/07/09 |
| HF-06 | Crash after each handoff boundary, lose each receipt, restart and retry the original intent; establish old/new/blocked/unknown states without duplicate effect or rollback of committed fencing. H1 answers unsupported transfer. | RH-07/08 |
| HF-07 | Revoke candidate/voter permission, restart with old proofs, roll the clock back, or delay renewal. State security-sensitive validation and clock assumptions, plus limits on revocation freshness under partition. | RH-03/06/09 |
| HF-08 | Bring up one/two/three voters; lose quorum and reconfigure membership while messages are delayed. Trace H3 authority and data availability separately; H1/H2 have no voting promise. | RH-05/07/08/09 |
| HF-09 | Old node accepts legacy claims while a new node tries fenced placement. Activation MUST block overlapping authoritative paths and state rollback/recovery preconditions. | RH-06/11 |
| HF-10 | Retire a scope/resource, reuse its human name, then replay old advertisements/create intents. The new identity/generation cannot silently resurrect or take over the old one. | RH-02/08/12 |

## 7. Questions the review must answer

1. Is automatic failover required now, or is dynamic creation plus stable or
   cooperatively movable homes sufficient? The owner has required dynamic
   creation/discovery; automatic failover has NOT yet been ruled mandatory.
2. What is the placement unit and identity/namespace model? Are independent
   roots with the same human name acceptable, and who serializes existing-scope
   naming? How do clients distinguish lookup from authorized creation?
3. What acknowledgement guarantee must survive a home change or failure?
   Existing `Ok` is not a quorum data-durability promise. Would the selected
   alternative require a changed storage/ack contract?
4. What exact boundary can fence stale effects for each resource class:
   replicated settings, a physical working copy, and an external supplier?
5. Which availability, trust/fault and deployment assumptions are acceptable,
   and which existing clauses must be explicitly amended?

The drafter has no ratified preference. A staged H1 then H2 approach appears
smaller under cooperative movement; H3 is a serious candidate when automatic
failover is required. Reviewers MUST challenge that hypothesis and each give an
independent conditional recommendation and the decisive tradeoff.

## 8. Review process and acceptance boundary

Apply [review-loop SKILL.md](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
and its canonical prompt template: owner drafts; exactly two independent
peer-blind READ-ONLY agents attack Consistency and Safety; owner files reports
verbatim, merges findings and remediates in one patch, at most two remediation
rounds. The GWZ program-specific checkpoint/process files do not exist in this
Glade root; this packet and its separate review ledger bind the process here.
Use GWZ for scoped local commits. No push, merge, runtime restart, code change
or dependency change is authorized by this document task.

Review GO means the comparison is fit for an OWNER DECISION, not that H1/H2/H3
is implemented, build-ready, protocol-frozen or approved. A NO-GO MUST name a
concrete defect in this comparison (contradictory guarantee, excluded owner
requirement, omitted decisive failure path, or false source/evidence claim).
An explicitly identified downstream design choice is a decision input, not an
automatic defect for lacking implementation detail in this packet.

The final ledger MUST contain exact document/source tuples, prompt hashes,
verbatim report links, every blocking disposition and original-counterexample
re-verification, independent recommendations, remaining P3s and owner questions.
Any later selected design MUST record exact amended clauses and pass its own
contract/design review before implementation. Existing canonical documents
and recorded rulings remain unchanged by this alternatives review.
