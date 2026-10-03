# Glade resource consistency profiles — owner requirement capture

Date: 2026-10-03. Status: **owner-confirmed product requirement; exact profiles
remain design work, unreviewed and unimplemented**. This addendum follows the
reviewed [Raft bootstrap proposal](GladeRaftBootstrapGrowthDesign.md) and the
[multiwriter evaluation](GladeMultiwriterSettingsEvaluation.md). Their earlier
review reports apply to their pinned checkpoints, not to this addendum.

## Confirmed direction

The owner confirmed that shared browser preferences such as theme color may
accept competing edits during disconnection and resolve conflicts after peers
reconnect. A resource supplied only by a failed provider may remain unavailable.
Glade MUST distinguish those expectations instead of applying exclusive-owner
failover semantics to every resource. This is a requirement, not selection of a
particular merge algorithm or permission to weaken an existing binding contract.

| Resource expectation | Proposed service behavior | Boundary |
| --- | --- | --- |
| Mergeable preferences | Authorized replicas accept local edits while disconnected; reconcile when delivery resumes. | Temporary disagreement is expected. The exact conflict rule, acknowledgement, authorization-freshness and retention profiles remain open. |
| Single-provider resource | Serve the source through its available provider or a separately qualified replica. | A timeout/election cannot recreate unavailable source data or its external capabilities. |
| Exclusively coordinated resource | Use the selected strong authority/consensus contract. | Raft still requires a committed-configuration quorum. Preference availability does not authorize unilateral voter removal or leader takeover. |

The proposed preference path needs no exclusive leader election. A timeout MAY
switch service to an already authorized local replica; it MUST NOT create rights,
change voter membership, invent a replacement resource identity or reinterpret a
strong binding as multiwriter. Resource shape alone does not select consistency.

## General CRDT resources and independent instances

Subsequent owner direction, GDL-055: CRDT resources MUST NOT be limited to
preferences or collaborative text. Those are candidate consumers, not the
general capability boundary. Distinct resources/scopes MUST support independent
CRDT instances without sharing an exclusive admission home merely because they
use the same engine. This is a design requirement, not an implemented guarantee.

The declaration MUST distinguish the general engine, its exact supported payload/
merge profile, and its consistency/admission/receipt contract. Each instance MUST
retain its own canonical identity, operation/causal history and policy context.
Using the same profile MUST NOT combine unrelated resource histories or confer
writer rights across scopes. Replication participants MAY differ by instance.

Text is the implemented payload profile. Maps/registers, counters, sets or other
profiles remain possibilities requiring their own valid-state and convergence
contracts; this direction does not claim those profiles already exist. Unsupported
profiles MUST refuse rather than falling back to text or whole-value semantics.
CRDT convergence does not by itself authorize creation, govern membership, promise
replica durability or coordinate external effects.

Future witnesses MUST cover two distinct instances using the same profile with
overlapping writer names, different scope policies and independent partition/
reconnect cycles, as well as profile mismatch and unsupported-profile refusal.
This extends the unreviewed requirement capture; earlier reviews remain pinned.

## Near-term delivery priority

The owner requested independent CRDT admission in the near-term timeline, rather
than leaving it indefinitely behind Raft integration. The
[IC-1–4 delivery plan](GladeIndependentCrdtAdmissionPlan.md) recommends doing the
contract/proof/real-node path before the first Raft production integration, while
carrier qualification continues independently. The owner subsequently authorized
the design/review/implementation lane (GDL-057); the exact admission profile and
implementation gates remain open.

## Design obligations and future witnesses

| ID | Obligation for the next design | Required future witness |
| --- | --- | --- |
| RC-001 | Bind a consistency profile/version to authenticated canonical resource identity and declaration. Equal names and browser origins MUST NOT merge unrelated roots. Unsupported or conflicting profiles MUST refuse affected writes. | Independent same-name roots, mismatched profiles and mixed-version peers. |
| RC-002 | A mergeable preference profile MUST permit already authorized replicas to accept edits without an exclusive home's availability or a Raft write quorum, subject to its declared permission-freshness and capacity rules. | One replica, two replicas with one permanently lost, and a partition with both accepting edits. |
| RC-003 | Valid competing edits MUST converge under the exact declared rule when the same valid operation set is eventually delivered. Per-field LWW remains a candidate, not an adopted rule. Coupled fields MUST have a valid atomic merge unit. | Concurrent same-field edits, independent fields, coupled fields and delivery permutations. |
| RC-004 | Local acceptance MUST disclose its actual storage/replication guarantee. It MUST NOT claim global agreement or independent-copy durability. | Lost reply/exact retry, restart and permanent loss before replication. Acceptable loss remains an owner/profile decision. |
| RC-005 | Authorization, private scope, signed origin attribution and revocation/freshness semantics MUST remain explicit. A serving replica MUST NOT grant itself rights. | Wrong principal, stale disconnected policy and reconnect with revocation evidence. |
| RC-006 | Reads MUST distinguish local/provisional state from stronger current-state claims. Defaults, reset/deletion and retained tombstones MUST follow the profile rather than silently replacing unknown history. | Partitioned reads, fresh browser startup, restored history and deletion followed by delayed delivery. |
| RC-007 | Identity bootstrap, profile changes and migration MUST be separate from local edit acceptance. Existing bindings MUST retain their contract until a reviewed transition is qualified. | Empty discovery for a known scope, unavailable bootstrap authority, legacy/new-profile coexistence and unsafe downgrade attempts. |

The existing MW-001–012 matrix remains the detailed evaluation starting point.
The next proposal MUST settle the initial eligible fields, exact merge semantics,
disconnected authorization, receipt/read vocabulary and storage-loss promise.
Cold-start identity resolution during a partition remains a separate obligation;
accepting edits to a known resource does not prove that an unknown resource can
be safely created or joined without authority evidence.

The previous proposed choice of appearance as the first Raft production consumer
MUST be reconsidered against this requirement. No replacement strong consumer,
new wire/API, production activation or canonical amendment is selected here.
Before implementation, reconcile canonical clauses and Gyld allocation, add
compiling behavioral RED consumers and perform the applicable reviews.
