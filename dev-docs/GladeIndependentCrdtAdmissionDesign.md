# Glade independent CRDT admission design

Date: 2026-10-04. Status: **DRAFT — semantic design for Consistency/Safety review; unimplemented, not a wire/API freeze or runtime activation**.

GDL-054–057 authorize the resource-specific requirement and the sequence: design,
review, implementation ahead of first Raft production integration. Authorized
Glade nodes MUST be able to accept edits to an already known CRDT instance while
partitioned, without an exclusive claim holder or write quorum, and reconcile
application operations when connected again. Text is the first supported witness,
not the product's resource boundary. This design proposes the exact conditions
under which that acceptance is sound; it does not promise unlimited offline rights,
instant revocation, globally current reads or survival of the only replica's loss.

## 1. Authority, baseline and implementation evidence

The requirement/plan baseline is root
`9baa7b8cd1c876ff9e608ecdffdd7490b58ebe9b`, Glade
`c65a6e87f0c257c15de8db080c29d365a883af85`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`. Review MUST identify its complete
tuple, including the revision containing this new file. This is source inspection,
not an executed qualification or an assertion that all linked working drafts are
ratified. Canonical rulings remain authoritative until their explicit amendments
are reviewed; this draft identifies those amendments below.

Sources:

- [Resource consistency requirements](GladeResourceConsistencyProfiles.md),
  [IC-1–4 plan](GladeIndependentCrdtAdmissionPlan.md), and
  [Decision log](DecisionLog.md), GDL-054–057.
- [Build entry](GladeBuildEntry.md), the source-qualified
  [problem capture](</Volumes/projects/limbo/gyld-wz/dev-docs/case-studies/glade/ProblemSpaceCapture.md>),
  [Gyld declaration](</Volumes/projects/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py>),
  [component allocation](arch1/Components.md),
  [library policy](LibraryBoundaryAndTestingPolicy.md), and
  [package architecture](GladePackageArchitecture.md).
- [Substrate](../glade/dev-docs/GladeSubstrateV1.md) §6 R1–R8/W1–W8;
  [cross-node writes](../glade/dev-docs/GladeCrossNodeWritesPlan.md) §§2/3/7/8;
  [authorization](glade/GladeAuthzModel.md) §§1/3a/3b/4/4a/6/7a/7b/11;
  [node signing](../glade/dev-docs/GladeNodeSigning.md), D4/D5/D7–D9;
  [CRDT adapter](../glade/dev-docs/GladeCrdtAdapter.md), GCA-01–09;
  [catalogue adoption](TautShapeCatalogAdoption.md), GSC-04/05/07/08.
- [Multiwriter evaluation](GladeMultiwriterSettingsEvaluation.md), especially
  MW-004–007/010–012; [Raft bootstrap design](GladeRaftBootstrapGrowthDesign.md);
  [application contracts](../glade/dev-docs/ApplicationContractDraft.md).

Present implementation is narrower than the requested feature:

| Observed boundary | Current behavior and consequence |
| --- | --- |
| `node/src/accept.rs::placement` | Client writes follow `route_subscribe`; forward ingress requires the live holder. Changing only Store does not enable independent admission. |
| `node/src/accept.rs::write_granted` | Forwarded writes check the authenticated forwarding node; client enforcement is switchable. This is not end-to-end requester authorization. |
| `node/src/store.rs` | CRDT supports several origins, but local first history establishes shape, application predecessor checks remain lenient, and non-home replay lacks the new proof/profile validation. |
| `node/src/envelope.rs`, `signing.rs` | Real Ed25519 signs home records; app operations are unsigned. Existing home envelopes are not an application proof profile. |
| `node/src/server.rs` client Hello | Principal text is self-claimed on the current client path; it cannot supply authenticated B3 attribution without possession/certification checks. |
| `node/src/mesh/serve.rs`, `mesh/route.rs` | Home pull/push and gap recovery exist; app interest follows holder routes. No general bidirectional app anti-entropy after isolated admission exists. |
| Rust/TS client sessions | Own per-zone seq/prev and CRDT refs exist. Current refusal drops own tail; timeout retry and client heads do not represent durable independent admission or origin custody. |
| `glial/src/text_crdt.ts` | Uses released `CrdtNode` and text projection; `baseSequences` derives offsets from first retained observations. Independent truncated views cannot use those observations as new origins. |
| `node/src/store/legacy_seal.rs` | A root lock and marker exclude legacy writes and replay for the entire Store root. It is not a per-binding activation switch. |

The released engine is `crdt.oracle/v1`, first payload profile
`text_crdt.profile/v1`, with `taut-shape/corpus/crdt.v1.json`,
`crdt.convergence.v1.json`, and `text_crdt.profile.v1.json`. Taut's older
`taut/src/taut/crdt/engine.py::ReferenceDoc` arithmetic fixture MUST NOT substitute
for that released engine or establish a new map/counter capability.

## 2. Proposed contract and identity

The independent profile name in this draft is
`glade.crdt.independent-admission/v1`. It is a proposed admission capability,
not another delivery shape. A binding MUST authenticate this tuple before opening
an instance or accepting an operation:

| Field | Exact proposed meaning |
| --- | --- |
| Resource identity | Creation-root ancestry and canonical resource ID, with retained exact genesis intent/hash. Human names, browser origins and discovery entries are not identity. |
| Instance identity | Resource incarnation + canonical share + glade ID + canonical zone/key bytes. Canonical parameter/key schema version is pinned. |
| Declaration identity | Authoritative declaration hash/version, immutable for this incarnation; shape MUST be `crdt`. |
| Merge capability | Exact engine, payload profile and corpus versions. First capability is `crdt.oracle/v1` + `text_crdt.profile/v1`; unsupported profiles refuse. |
| Admission capability | Exact independent-admission version; separate from legacy holder admission and from strong-profile consensus. |
| Authorization capability | Exact offline authorization profile/version, trusted roots, accepted authentication strength, issued permit semantics, and placement authority. |
| Storage/recovery capability | Receipt durability class, bounds, retention/rejoin policy, initial chain base and migration generation. |

The full authenticated tuple is the instance's namespace. Each operation,
origin certificate, permit, receipt, proof, cursor, cache, pending request and
replication session MUST bind it, directly or by collision-resistant canonical
descriptor hash. Equal `glade_id`, keys, actor display names or engine names
MUST NOT join histories across roots or incarnations. Canonical serialization
and hashing MUST have Rust/TS/Python vectors before remote use; private in-memory
values are insufficient evidence of canonical bytes.

Every certified origin epoch MUST have a fresh, immutable canonical `Op.origin`
within its instance, never reused by another epoch. The epoch distinction MUST
already be embodied in that identifier before the inner op is signed or hashed;
an epoch field in an outer certificate cannot disambiguate identical inner
`Op.origin` values. Certification MUST bind the exact canonical origin, epoch,
instance and writer key. A certificate proposing a new epoch with an already
bound canonical origin MUST refuse before admission or store mutation. Display
names MAY repeat; they are not canonical writer identities. All refs/frontiers,
chain slots, retry identities, quarantine keys and text element `actor_id`s MUST
use this same canonical origin. The unchanged GCA mapping remains
`CrdtOp.origin = Op.origin`; no extra epoch field or replica-local remapping is
added to the canonical engine. Histories and refs from old epochs keep their
original identities permanently.

Each initial origin chain MUST start at Glade seq 0, with no predecessor, and
use positive Taut seq `Glade seq + 1`. Later ops MUST carry exactly the prior
canonical op hash; refs are sorted unique per-origin frontiers in this instance.
Glade seq and refs MUST be integers in `0..Number.MAX_SAFE_INTEGER-1`; lamport
MUST be a nonnegative safe integer. Overflow refuses before mutation. The first
profile has no truncated bootstrap or app checkpoint: no replica-dependent
renormalization from its first observed seq is permitted. Full history or an
explicit incomplete-history result is required. A future signed checkpoint
profile must bind fixed chain bases and exact Taut bootstrap semantics separately.

Declaration/shape comes from authenticated identity, never the first local op,
local peer count, a `ServeClaim` timeout or an opportunistic `value` arrival.
Known legacy/strong bindings retain their own contracts. Merely seeing shape
`crdt` does not opt a legacy binding into this admission profile.

### Genesis and discovery

Creation remains a privileged, uniquely custodied authority operation. Before
publication the creator MUST retain the exact signed genesis intent and an
antirollback issuance floor; retries recover that intent, not a new resource.
Two truly fresh roots with the same name remain independent. Two disconnected
participants intending one known identity cannot both issue its genesis because
lookup is empty. Naming/alias authority must resolve that case separately.

Joining a known instance requires its existing authenticated descriptor and
policy evidence. Unknown authority, empty discovery, missing local files or an
unreachable creator yields `IdentityPending`/`Unavailable`, never initialized
empty state or permission to bootstrap. Discovery locates eligible replicas and
carries evidence; it supplies neither global absence nor new write rights.
One authorized replica can operate without peers. Two or more replicas may join
the same known identity from previously retained evidence while initially
disconnected. Replica participation is instance-specific, not Raft voting.

## 3. Signed requester, origin and offline authorization

### End-to-end operation attribution

Proposed application proof data MUST preserve the canonical operation unchanged:

1. A certified writer key/origin certificate binds the origin ID, requester B3
   principal/attenuation ancestry, instance descriptor, origin epoch and key.
   It MUST bind the fresh canonical `Op.origin` specified in §2, not merely a
   display label plus epoch. Tabs sharing a principal have distinct canonical
   origins; a newly certified recovery epoch also has a distinct canonical
   origin even when its display label or writer key repeats. A node may be a
   writer only through its own authorized principal/origin; it cannot borrow
   a caller name. Permits, admission records and signed causal refs MUST bind
   that same certified canonical origin.
2. The writer signs the exact canonical app op and descriptor hash under a
   separately specified application-op domain. Scope, seq/prev, refs, shape,
   payload, permit reference and requester ancestry are covered by the proof.
3. The accepting node's admission record binds those bytes/hash, certificate,
   permit, policy evidence/frontier, acceptance time evidence, node identity,
   storage class and stable receipt identity. Its signature is a statement of
   local admission, not the writer's signature or an independent-copy receipt.

Client ingress MUST prove possession of the certified writer key and bind the
session to that principal. Each forwarding/replication hop MUST preserve the
original requester and proof, never replace it with peer HELLO identity.
Peer authentication authorizes the channel, not the operation. `self` is
resolved from the authenticated principal at append, subscribe, replay and
forwarding; mismatched literal `self:<other>` refuses. Membership, AZ-17 owner
access and operator-approved `replica.hold` remain separate checks. Receiving
payload does not authorize serving it to another user.

The first real-node slice SHOULD use explicit certified development roots and
key-signed sessions only. They are real keys and narrowly issued authority,
not an allow-all switch or invented production enrollment. Operator-vouched
sessions remain unavailable for independent writes until a separately reviewed
attenuation profile exists. Neither app signatures nor this design permit
client-created governance or privileged effect records (H-R3).

### Proposed bounded offline permit semantics

This draft recommends `bounded-offline-permit/v1` to make the availability/
revocation tradeoff reviewable. A permit is an authenticated attenuation of
existing authority, issued ahead of time, not a per-edit oracle decision. It
binds instance, subject/origin certificate, `write.append`, accepted admitting
nodes/operators, policy generation, not-before/not-after bounds and optional
finite per-origin sequence range. Issuance cannot widen source authority. Each
permit MUST have a finite expiry; no implicit infinite offline entitlement.

At local admission the node MUST validate all of the following from supplied,
identified evidence before mutation:

- Valid creation/declaration, writer possession/certificate, signature and
  narrowing requester chain; supported exact capabilities and private scope.
- Current local policy permits this subject and node's placement/admission.
  Missing, unreadable, known revoked or contradictory governance fails closed.
  Legacy unsigned governance is history, never authority.
- A permit covers the exact operation and origin sequence, and trusted time's
  entire uncertainty interval lies within its validity interval. Clock rollback
  below the durable time floor or unknown uncertainty yields `PolicyPending`,
  not success. Trust/time inputs MUST be injected, not process globals.
- Complete locally validated strict predecessor and causal closure for local
  generated admission; payload conforms to the exact profile, and capacity is
  reserved for the whole commit and retry/evidence record.

On learning a valid revocation the node MUST stop new admissions under the
revoked chain and cut newly denied sessions/serve paths. Revocation-wins governs
current grants, and the revoked subtree cannot issue a new permit. It MUST NOT
erase a prior local admission record. A valid admission made by a still unaware
permitted node within the finite offline window remains historically eligible
after reconnection, even when revocation was concurrent. Import validates the
recorded admission evidence/time under the permit; it does not apply today's
grant fold as a retroactive veto of already admitted history. After expiry,
new intent requires renewed authority. An expired permit does not prevent
recovery or validation of its earlier admitted records; serving still requires
current access.

This explicitly settles the disconnected scenario: A receives revocation and
refuses subsequent edits; isolated B accepts within its remaining permit and
capacity; B stops when it learns the revocation or its time window ends; the
valid B admission remains in history. If B has no qualifying evidence/window,
its edit stays local client intent with `PolicyPending` or `Denied`. It MUST NOT
receive the accepted receipt. A locally refused edit is not laundered by replay;
obtaining another valid admission requires that other node's independent valid
permit and checks, and changes no existing receipt identity.

A writer signature alone, including a signature made after revocation or permit
expiry, is attribution rather than qualifying historical admission. A rival
without the qualifying admission evidence defined in §5 MUST NOT revoke or
quarantine a previously valid operation or its dependents. Retain the rival as
bounded attributable security evidence and preserve the valid projection in
either arrival order. Conversely, a genuine earlier qualifying admission is
not disqualified merely because its writer or permit is now revoked/expired.

**Review-significant amendment:** Authz §4's forward-only revocation does not
already specify historical write eligibility. This proposal separates historical
admission validity from current serve/admit rights. The permit is not a proof of
global nonrevocation. The accepting node attests local knowledge and trusted
time; a malicious node with a colluding revoked writer can lie about that time
or view. Origin signatures still prevent carrier forgery, but offline admission
freshness relies on the declared trusted-admitter/time model in addition to
placement trust. This MUST be explicitly accepted or replaced during Safety
review, not inferred from operator approval alone. No crash-only algorithm can
prove instantaneous revocation to a disconnected participant. If the deployment
requires that guarantee or rejects this trust model, it MUST choose fresh-policy
admission and refuse partitioned writes for that resource. This is a resource
profile choice, not a silent change to all CRDTs.

## 4. Admission, custody, projection and honest outcomes

One pure decision component evaluates the supplied authenticated descriptor,
proof results, policy/time evidence, frontier, budget and candidate operation.
It returns a bounded commit plan or an explicit denial/pending result. It
performs no I/O and creates no cryptographic fact. The Records host serializes
instance commit, executes the plan against storage, and reports acceptance
only after the declared storage barrier. The same validation is reused for
client, peer, disk replay and restore; disk location is not authentication.

Profile validation MAY inspect opaque payload through the canonical profile's
validator; the node remains projection/fold-agnostic. Validation results MUST
bind the exact descriptor and immutable bytes, not confer forever-valid policy.

| Outcome | Meaning and required client behavior |
| --- | --- |
| `AcceptedLocal` | Exact signed op + admission/retry record held locally at the named storage class; no global agreement, remote copy or permanent-loss promise. Intent can leave its unsent queue only according to its own durable recovery policy. |
| Exact retry | Original stable receipt and op identity, with current custody/projection status separately reported. No new seq, resigning with changed bytes, replay fan-out or second charge. Current denial may prevent new access but cannot rewrite the stored historical outcome. |
| `OutcomeUnknown` | Commit/reply boundary uncertain; exact lookup/retry only. The node MUST NOT issue a fresh equivalent operation. |
| `MissingHistory` / `PolicyPending` / `IdentityPending` | No admitted local edit. Preserve intent and canonical bytes; request missing evidence. A retained peer candidate is explicitly provisional, not `AcceptedLocal`. |
| `Denied` / `Invalid` / `Unsupported` / `Capacity` | Known no-commit result for local intent; keep intent for explicit correction, renewal or authorized resubmission. Existing independently accepted records MUST NOT be silently dropped by this result. |
| `IntegrityConflict` | Same-slot fork with two historically qualifying admission records (§5), or proven incompatible identity evidence; retain evidence and expose degraded projection. A bare signed nonqualifying rival is security evidence and MUST NOT degrade legitimate projection. It is not an arrival-order winner. |
| `ReplicaRetained` | Named peer attests its actual retained record and storage class. It is additional custody evidence, not a consensus decision or proof of independent failure domains. |

The proposed first storage class is `local-process-restart/v1`: successful local
acceptance survives qualified node-process termination/reopen, but does not claim
OS crash, power loss or machine loss. It preserves current R2's limited promise,
without W4's second-node promise. A later synced storage class requires actual
file/directory barriers and interruption evidence. Permanently losing the only
copy can lose acknowledged edits under the first class; this fact is part of the
receipt and must be accepted for deployment. A fake store receipt cannot qualify
either storage class. A known write failure MUST leave no success/visible partial
commit; uncertain partial I/O produces `OutcomeUnknown` and recoverable framing.

Custody and projection are separate. Accepted admission records stay retained
even if subsequent integrity evidence makes their projection ineligible. The
client MUST NOT apply the current blanket `Session.refuse` tail deletion to an
independently admitted operation. It must retain outcome/evidence and rebuild
the projection from eligible history, with a visible degraded/pending state.
An unaccepted provisional local editor view MAY show intent, but MUST label it
as unsent/unadmitted. A user-facing vocabulary/UX freeze requires Surface review.

### Reads and complete local cuts

A read names descriptor/generation, local accepted frontier, eligible frontier,
pending dependencies, integrity quarantine and observed peer frontiers. The
rendered text is local/provisional relative to unseen remote edits; it is not
globally current. `complete-local-cut` means the finite announced cut has all
its required data/evidence and can be projected, not that no other peer has
accepted edits. A completed sync round means the named two cuts reconcile;
it never certifies every participant or future operation.

Unknown history MUST NOT render as authenticated initialized emptiness. Empty
text is valid only under the known descriptor and complete required cut. UI
fallback/default rendering MUST NOT publish initialization. Text deletes retain
canonical tombstones and cursor anchors; reset is a profile operation, never
journal deletion. Preference defaults/import/reset/coupled-field rules require
their own supported profile and are outside the first text witness.

## 5. Gaps, forks and deterministic eligibility

The eligibility function is over the complete immutable descriptor plus known
signed operation/admission/policy/fork evidence. Equal complete evidence sets
MUST produce equal eligible operation sets before the canonical Taut merge.
Capacity and arrival order MUST NOT choose a winner. Intermediate projections
may differ while evidence is incomplete; the read must state that condition.

Local admission requires its strict own predecessor and valid causal dependencies.
A successor delivered first by replication MUST yield a bounded pending request
for exact predecessor/dependency identities and hashes, not permanent refusal
or a falsely advanced head. Pending candidate bytes can be durably retained in a
separate candidate area, with a candidate receipt, but do not advance the accepted
contiguous head or enter Taut. Unknown verifying key/policy is deferred, not
invalid. Malformed or proven invalid ancestors quarantine dependent candidates.
The sender's original local receipt remains custody evidence; the receiving node
must report unresolved transfer, never claim convergence while excluding it.

Before deriving fork quarantine, each rival MUST independently establish a
**historically qualifying admission record**. Qualification requires the exact
authenticated descriptor/profile and valid payload; the unique certified
canonical origin/epoch and narrowing requester chain; the writer signature over
the bound immutable operation; a valid signed admission record from a node
authorized to admit that instance under the issued permit; the finite permit's
scope/sequence/time bounds and recorded trusted acceptance-time evidence; and
the recorded local policy evidence allowing that admission at that cut, with
strict predecessor/hash and causal proof closure. Current revocation or permit
expiry is not a retrospective qualification veto. Missing proof/key/closure
leaves qualification pending; a proven invalid proof, payload or admission makes
the rival nonqualifying.

This historical proof predicate MUST be evaluated over finite acyclic structural
and authorization evidence, independently of the subsequently derived projection
quarantine. Its supporting predecessor/dependency records must themselves have
qualifying historical evidence, but need not survive derived fork exclusion.
At a disputed cross-origin ref slot, the required historical support is a
qualifying record at that canonical origin/seq, not an arrival-selected projected
winner. Own predecessor hashes still bind the exact predecessor bytes. Neither
"currently projection-eligible" nor the absence of a derived fork may be an
input to qualification. Thus two fully qualifying branches remain capable of
proving their conflict after both have been excluded; invalidating one through
that exclusion cannot undo the proof or create a circular verdict.

Only two distinct operations with independently qualifying admission records
at identical `(instance, canonical Op.origin, seq)` convict a projection-affecting
fork. Their certificates necessarily bind the same epoch; a purported other
epoch reusing the canonical origin is an invalid certificate, not another valid
slot. Proof is a canonical unordered pair of the operations plus qualifying
admission evidence; pair order cannot affect validity. A bare signed unauthorized,
expired-without-prior-admission, malformed or unknown-admission rival MUST NOT
convict or invalidate legitimate history. Retain it as bounded attributable
security evidence (or a pending candidate when qualification is unresolved),
without advancing the accepted head or feeding that rival to Taut. A missing
predecessor is not a fork proof, and an unsigned rival cannot convict either.

For a qualifying same-slot fork, quarantine the canonical instance-origin chain
from its earliest fork seq onward, both competing operations and every transitive
causal dependent. A valid common prefix before that seq stays eligible. A later
discovered earlier qualifying fork moves the quarantine floor backward
monotonically. Both branches, prior receipts and dependent records remain
retained as evidence; no arbitrary hash/arrival choice returns one branch to
eligible history. Reconstruct the projection from the eligible closure, notifying
consumers of the integrity change. Unresolved qualification alone MUST NOT remove
the established valid projection or manufacture a complete-convergence claim.

This is an admission/security eligibility rule, not a new CRDT merge algorithm.
Released Taut's own equivocation behavior/corpus remains unchanged; this adapter
passes only the security-eligible set through `CrdtNode` and `text_crdt`.
It MUST test prefix/dependency filtering against opposite fork-delivery orders
and complete set convergence. A same key used concurrently/rolled back cannot
be made safe by deterministic text conflict resolution. Continuing after a
fork requires a separately authorized fresh origin epoch and explicit intent
recovery with a fresh canonical `Op.origin` under §2; old quarantined bytes MUST
NOT be automatically recast as new edits. The valid old prefix remains under
its old canonical origin, and a new-origin edit can causally reference it.

Exact future text witnesses for ICD-T06/T07/T09:

- Origin E0's canonical ID `writer-e0`, Glade seq0/Taut seq1, validly admits an
  insertion with atom ID `atom-a` and text `A`. A different valid origin then
  admits an insertion `atom-d` containing `D` after `atom-a`, causally referencing
  `(writer-e0, 0)` in Glade coordinates. After E0's writer is revoked and its
  permit expires, it signs a different seq0 rival inserting `X` but supplies
  no qualifying admission. Both evidence orders MUST retain the rival as
  security evidence, keep the exact two legitimate operation identities
  eligible and project exactly `AD` through released `CrdtNode`/`text_crdt`.
  The invalid rival MUST NOT enter the engine or degrade those legitimate
  receipts. Use the same outcome for a rival with proven invalid payload or
  permit; unresolved evidence remains pending without conviction.
- Separately, retain E0's valid `A` prefix and two genuinely qualifying rival
  admissions at E0 seq1. Both orders MUST quarantine those two rivals and their
  causal descendants while preserving E0 seq0. Authority certifies E1 with
  canonical ID `writer-e1` (the same display name is permitted). Its initial
  Glade seq0/Taut seq1 insertion `atom-b` containing `B` after `atom-a` references
  `(writer-e0, 0)` and uses text `actor_id = writer-e1`. Both orders of prefix/
  recovery delivery, buffering the dependency when necessary, MUST keep the
  eligible set exactly `{(writer-e0,0),(writer-e1,0)}` in Glade coordinates,
  map it to distinct Taut identities, and project exactly `AB`. E0 receipts/
  retries retain E0 identities; E1's counters start only under `writer-e1`.
  A proposed E1 certificate naming `writer-e0` MUST fail before mutation.

## 6. Bidirectional application synchronization and bounds

The independent path MUST have an instance-scoped app reconciliation session,
separate from home metadata pull/push and from the legacy claim-holder forward.
Both endpoints authenticate node/operator placement and current transfer access,
verify matching descriptor/generation/profile, exchange contiguous seq+hash
frontiers and pending/evidence summaries, and request/send exact missing app
operations, admission records and required proof/policy closure in both directions.
A can recover its loss from B without a client manually resending history.
An absent or stale `ServeClaim` does not stop already authorized local admission
or local reads under this profile; claims remain discovery/routing hints for it.

Replica retention interest MUST survive browser unsubscribe/restart according
to the declared instance enrollment. Otherwise two nodes that accepted offline
but have no open browser will never reconcile. Sync is not whole-share indiscriminate
replication: only enrolled, authorized instances/zone keys flow. A peer authorized
for instance X does not obtain Y's private history. Node replication checks and
per-requester serving checks remain distinct at every hop. Removed placement
stops future transfer, preserving the honest offline-cache caveat.

Frontiers advance only after complete validation and custody; max observed seq
is insufficient. Same-seq different hash starts proof reconciliation. Delivery
ordering optimizes strict-chain ingress but correctness tolerates reordering,
duplicates, interruption and reconnect through bounded pending recovery. Live
traffic cannot overtake a declared replay cut unnoticed; subscription ack/gap/
live frames retain the applicable R4/R5 cut semantics. Transport backpressure
and cancellation report incomplete progress, not successful convergence.

The first profile deliberately retains all app ops/tombstones, genesis, origin
certificates, permits, receipts, policy evidence, fork proofs and retry identities
for its incarnation. It has finite reviewed per-instance quotas; full-history
retention plus refusal is bounded storage, not unlimited append availability.
No TTL-based collection or `latest` truncation is permitted. Required bounds
include operation/frame bytes, origin count, refs per op, accepted journal bytes,
candidate/pending bytes and count, evidence bytes/count, outstanding requests,
sync page bytes/items, concurrent sessions and per-round work. Overflow refuses
before acceptance. Already accepted history MUST NOT be evicted to make room.

Reserve control/recovery/evidence capacity separately before exhausting normal
edit quota; otherwise fork proofs or predecessors could never be recorded.
Bounded pending overflow leaves the sender custodian and returns `Capacity` with
an exact retry/request cursor; it must not drop the only acknowledged copy.
If even reserved recovery capacity is unavailable, the instance is degraded and
sync remains incomplete. Progress requires capacity restoration or reviewed
instance retirement, not an implicit stronger success. Bounds are explicit
deployment parameters, not unspecified magic constants.

Rejoin during this initial profile requires full retained genesis-to-head evidence.
A retired incarnation or unknown recovery floor yields explicit `HistoryUnavailable`
and no new same-identity genesis. Later collection needs a versioned signed
checkpoint/bootstrap/rejoin contract that handles offline members and tombstones;
it is not required to invent that algorithm for the first useful bounded release.
Finite quotas mean sufficiently long disconnection can stop new admissions.

## 7. Restart, origin custody and physical storage

Client origin recovery MUST retain certified canonical origin/epoch, next seq, prev hash,
lamport watermark, observed causal frontier and exact outstanding intent/op/receipt
bytes together. Restore must recover maxima before appending, and exact retry
must reuse bytes. Parallel holders of one writable origin MUST be excluded by
actual origin custody or contained as forks. A tab label, saved clock or mutex
in another process does not prove custody. If a restored backup lacks a trusted
antirollback floor, its old origin becomes read-only/recovery-pending; establish
a fresh certified origin epoch with a new never-reused canonical `Op.origin`
through authority before new edits. Resetting seq0 under the old canonical
origin is forbidden even with a new certificate/epoch. Old history, refs, text
actor identities and exact retry/outcome keys remain bound to the old origin;
new edits/counters use the new origin and may reference the eligible old prefix.
Custody must retain enough origin-issuance evidence to refuse canonical ID reuse;
missing or contradictory issuance evidence leaves recovery pending, not a
new certificate inferred from a repeated label. Offline
creation of fresh writer keys is allowed only when already covered by reviewed
certificate/delegation issuance authority, not merely because data is CRDT.

Node replay MUST recover descriptor, journal, admission/retry state, policy/time
floors, pending candidates and fork quarantine before serving. Receipt publication
follows one physical commit unit covering canonical op, admission record, retry
identity and recoverable synchronization handoff. A separate best-effort outbox
written after `AcceptedLocal` would strand acknowledged records after restart;
instead handoff may be derived deterministically from accepted history/enrollment.
Tests must kill actual processes at framing/commit/reply boundaries and reopen.

Proposed layout places independent data in an explicitly versioned, separate
physical root with an exclusive writer lock and fail-closed format marker;
its path is declared configuration. It MUST NOT be a legacy Store subdirectory
that old Store scanning treats as a share. Instance journals and proof/custody
metadata can share this root, but retain namespace isolation and atomic record
framing. `records.json` snapshot CAS and current `ReplicaSync` do not alone prove
the required coupled commit or journal capability. Engine/vendor selection is
separate; no new database/library is selected here.

Current `ReplicaSync::ingest` requires an atomic durable bounded batch with
no mutation on known `Conflict`/`Gap`/other errors. Current `Store::append`
is unsynced single-record application retention, verifies only `home`, and
`envelope::verify` requires `home`/`log`: none qualifies that contract for this
application profile. A new candidate/evidence path MUST NOT mutate a batch and
return the existing `Conflict` as if its no-mutation contract held. Use separate
explicit candidate/evidence retention outcomes and an atomic accepted-batch
path, or review a precisely versioned interface amendment. Quarantine proof
retention is a declared successful evidence commit, not a hidden side effect
of a no-commit application refusal. Existing snapshot-only RecordsFile tests
do not qualify this operation ledger.

Legacy Store root and new root MUST have explicit resource writer inventory and
ownership. An activation record binds instance incarnation, source root, target
root, format/admission/profile versions and admission exclusion generation.
Old binaries MUST fail opening new roots. Separate directories alone cannot
prevent an old node admitting the same resource under legacy rules: protocol
exclusion, controlled enrollment and per-instance admission fencing are also
mandatory. The earliest real witness SHOULD use fresh independent identities,
leaving existing resources untouched.

## 8. Migration, whole-store seal and compatibility

Legacy or strong resources MUST NOT switch profile on timeout, first CRDT op,
configuration reload or discovery. A reviewed transition MUST freeze old writers,
inventory every admission path/consumer, recover all accepted/unknown/pending
history and origin outcomes, retain a backup, validate a deterministic import
under an explicit new incarnation, publish the activation/exclusion evidence,
then enable only qualified writers. In-place history rehash/resign/shape reinterpretation
is prohibited. Unknown old tails block a claim of complete import. Old late ops
either belong to a declared included cut or remain explicit unresolved evidence;
they do not silently become new-incarnation writes. Retrying old outcomes remains
possible without reopening old admission.

Q4's `legacy-store.sealed` disables writes AND replay across its whole legacy
root, including unrelated app and home records. Installing it is not the CRDT
per-instance switch. If Raft retirement seals a shared root, all remaining
independent/legacy/home writers must first move through their qualified transition
or use an explicitly separated retained store. Neither deleting the marker nor
pointing a new writer inside that root is an acceptable bypass. CRDT work MUST
not weaken LS seal tests or Raft RA-001–012/Q4 migration gates.

The new capability MUST negotiate descriptor, admission/proof, merge, receipt and
recovery versions before any new operation or provisional payload is routed.
Existing `Welcome.protocol=1` and NodeHello/ALPN distinctions do not express this
agreement. Old peers/readers/writers cannot receive new-envelope ops as ordinary
CRDT payloads. Readers are deployed and qualified before writers. Unsupported or
conflicting tuples fail closed with no Store mutation. Rollback before any new
acceptance can follow the reviewed exclusion reversal; after new acceptance it
requires a qualified migration, not reopening a legacy journal or downgrading to
whole-value LWW. Strong and independent bindings MAY coexist in one node with
separate routes, stores and governance; no profile conversion is implied.

### Exact canonical supersession required

These are proposed scoped amendments, not changes performed by this file:

| Source clause | Replacement ONLY for an activated independent instance | Retained behavior |
| --- | --- | --- |
| Substrate §6 W1 first/second points, and §2/§5 amendments making holder reachability necessary | Authenticated descriptor selects independent local admission and local authorized read; `ServeClaim` is not the admission home. Unknown identity remains pending. | Legacy W1, including Local/Forward/Absent, and strong authority remain intact. |
| W2 holder-only/one judgment/first-writer claims and node-ID write check | Each qualified replica validates the same admission profile and immutable requester/origin proof. Current transfer policy differs from historical op eligibility. | Legacy holder checks; SWMR's exclusive writer; effects at their authority. |
| W3 holder-first persistence and "keeps nothing" on another node's refusal | Local valid admission is retained before receipt independent of remote judgment. Peer pending/refusal has explicit recovery/quarantine status and cannot delete admitted history. | Legacy forwarding remains holder-first. |
| W4 two-node `Ok` and holder loss requiring client resend | New named local receipt states one-node storage class. Bidirectional app recovery offers retained copies automatically; replica receipts are additional evidence. | Legacy W4's two-node limited promise is unchanged. |
| W5 `UnknownShare` for no holder/12-second forwarding and no node outbox | Independent identity/policy/history/transport uncertainty is distinct. A holder timeout does not unplace a valid local commit; exact durable outcome lookup/retry follows local receipt identity. | Legacy `UnknownShare` retry/backoff rules remain. |
| W6 order-dependent gap refusal | Independent reconciliation buffers/requests bounded gaps and verifies hashes; per-zone receipt correlation/order is retained. | Legacy W6 and R4 replay cut ordering remain required. |
| W7 shape/writer inferred from first holder op | Authenticated declaration fixes exact CRDT/admission/profile before any op. Taut/Glial still own merge. | Other durable shapes, SWMR and unsupported stream/exchange distinctions remain. |
| W8 / H-R3 | No independent application path accepts `home` governance or privileged effects. | W8/H-R3 remain, without exception. |
| R1/R2/R3/R7 and client answer 4 | Independent receipt/read/status capability distinguishes custody, projection, pending, unknown and quarantine; no blanket drop of acknowledged own tail. Heads are validated contiguous cuts. | Legacy Error/Ok/corr/session semantics remain for legacy sessions. |
| CrossNodeWrites §2 option B flaw/§3 W1–W7/§7 "no wire change/no offline writes/no outbox/no repair/no principal/app unsigned" | A declared independent path qualifies the missing shared policy, authenticated requester, signed op, fixed shape, origin custody and bounded bidirectional recovery. Its new schema is explicit. | Option A remains the existing safe path; option B is not enabled generically. CJ-1–4 remain legacy cold-join protection. |
| Authz §1 accepting-hop/every-folder; §4 current fold/forward-only; §7a integrity/trust | Historical eligibility verifies issued permit + signed origin + attested admission; current revocation governs new admission/serve. Declare trusted-admitter/time assumption and finite offline window. | Creation/B5 governance, narrowing, revocation ancestry, AZ-16/17, current fan-out and placement rules remain. |
| NodeSigning D5 unsigned app slice/D4 payload home envelope | Introduce separately versioned app proof, no substitution of home `SignedRecord` or unsigned attribution. | Existing home signing and its canonical hash history stay unchanged. |
| GCA-02/03/04/05/06/07 and GSC-04/05/07/08 | Extend conformance for declaration-bound independent proof/eligibility/full-history sequence mapping; keep exact engine/profile and dispatch. | Frozen shape numbers (`crdt` wire 4, declaration 7), text semantics/cursor rules and other catalogue restrictions remain. |
| BindingResolver/ReplicaSync/Signer/Clock draft contracts | Identify new descriptor/evidence/time/commit semantics with compiling consumer conformance; existing descriptive binding/cursor/signature outcomes are not upgraded by inference. | Snapshot CAS, atomic batch ingest, local cursor and current-access contracts remain until explicitly amended. |

Appending an optional signature field to frozen Op changes canonical encoding and
hash history (NodeSigning D4). Hiding proof bytes inside old `Op.payload` makes
old text clients decode an envelope as a text edit. Neither is an acceptable
shortcut. The schema tranche MUST select a versioned transfer container/proof
representation preserving canonical inner app Op bytes and freezing new hash/
signature domains, before real-node mutation. This is a known schema gate, not
permission to freeze an unreviewed public interface during semantic review.

## 9. Gyld allocation, consumers and package proposal

The existing revision-3 `GladeArchitecture` MUST carry the new requirements and
journeys in an acknowledged update before new/recomposed production ports.
It is an allocation update, not a request to extend the Gyld engine or demand
new crates for all responsibilities. The original captured responsibilities
remain intact; no implementation claim follows from graph checking.

| Responsibility | Lead / required cooperation for this lane |
| --- | --- |
| `bootstrap`, `compose_runtime` | NodeAssembly owns explicit initial descriptor/trust/store/provider construction and both composition roots; no new creator rights from directory. |
| `resolve_definitions`, `canonicalize_scope`, `author_declarations` | Binding + declaration toolchain authenticate exact instance/profile/key schema; application supplies product declarations. |
| `verify_identity`, `enforce_policy`, `validate_ingress` | Admission orchestrates Binding/Policy/Signature/Clock evidence; pure admission kernel supplies decisions, same rules on wire/disk/replay. |
| `retain_replica` | Records owns accepted history, stable retry and coupled handoff; StorageAdapter supplies qualified physical commit. |
| `resume_data`, `deliver_interest` | Delivery + Records/ReplicaPort own enrolled instance anti-entropy, missing-data recovery and read cuts; ShapeAdapters/Glial project canonical merge. |
| `peer_connectivity`, `client_sessions`, `schedule_traffic`, `own_lifecycle` | Iroh/Sessions/Runtime own authenticated duplex framing, version negotiation, bounded work and shutdown/restart custody. |
| `reconcile_metadata`, `locate_providers` | Directory/Records carry governance and eligible routes. Metadata convergence does not complete app reconciliation. |
| `assemble_consumer` | Glial uses released Taut/text, preserves local intent/receipts and element-ID anchors, rebuilds after eligibility changes. Grip remains protocol-free. |
| `verify_integration`, `expose_management` | Harness/conformance + system suppliers expose evidence, quota, pending/fork and exact receipts; management is not a parallel authority source. |
| `fence_source`, `invoke_source`, `repair_publication`, `attach_suppliers` | Existing SupplierHost/Invocation/source boundaries remain. CRDT acceptance cannot claim external effects, saved-file replacement or source failover. |

Affected consumers include Glade's acceptance/server/session/store/mesh and both
node roots; wire/declaration generators and IR/corpora in Taut; Rust/TS clients
and durable outboxes/origin recovery; Glial instance-store/mount/text assembly;
demo editor; any vendored IR/client in gryth-ui; glade-gyld/glade-gwz/gryth launchers
when they negotiate new readiness/roots; application binding/sync/signer/clock
contract implementers; Raft resource mapping/migration/seal inventory. Discovery
is affected only where evidence/descriptor routing changes; it must not import
the node or acquire CRDT fold dependencies.

Proposed first production boundary is a small **pure** independent-admission
kernel under Glade's explicitly adopted architecture gate, with typed input/state/
decision/commit-outcome/recovery effects. Its required behaviors are `plan local
admission`, `classify exact retry`, `plan peer recovery`, and `derive eligible
closure/read cut`; package name is provisional. No marker trait is required for
this deterministic kernel. Narrow **contract** ports are justified only for
replaceable verification/evidence, coupled commit and transport capabilities;
an **integration** host consumes them and the kernel. It MUST NOT depend on a
concrete database/Iroh/Tokio or call environment/time itself. Helpers are pure
development dependencies; text conformance uses released Taut adapters at the
consumer/harness boundary. No new merge engine, universal framework or global
types crate is proposed. Exact package/classification/dependency entries require
reviewed adoption, not an allowlist relaxation to make tests pass.

## 10. Stable requirements and future RED witnesses

All tests below are future obligations. Write each behavioral RED consumer before
implementation; a placeholder compile failure or assertion against its own helper
does not prove a useful behavior. `ICD-Tnn` maps to `ICD-nnn`; preserve these IDs
through protocol/schema allocation. Shared corpora test exact output/op identities,
not merely equal length or absence of crashes.

| ID | Normative requirement | Meaningful future RED success / failure / edge witness |
| --- | --- | --- |
| ICD-001 | MUST authenticate immutable identity/descriptor and separate discovery/genesis. | T01 known instance joins; equal-name roots remain separate; lost genesis reply/empty discovery never re-genesis. |
| ICD-002 | MUST select independent admission by exact binding capability; retain legacy/strong routes. | T02 one replica admits without claim; two partitioned nodes both admit; legacy missing holder stays unplaced and strong minority stays blocked. |
| ICD-003 | MUST reuse exact supported canonical engine/payload profile. | T03 actual concurrent text insert/delete permutations converge in op set and projection through released engine; unknown profile/wrong shape/malformed text fails before mutation. Reject a mutant returning empty state or routing CRDT to value. |
| ICD-004 | MUST preserve instance isolation across identity, proof, history and policy. | T04 two same-profile instances with overlapping display origins and different policies partition/heal independently; foreign cursor/permit/refs/private-self fail and bytes never cross. |
| ICD-005 | MUST authenticate original requester and writer end-to-end. | T05 certified principal succeeds across two hops; forged Hello/op, forwarded node-ID substitution, changed scope/payload and unknown device defer/refuse appropriately. Real cryptographic corpus separately. |
| ICD-006 | MUST apply finite offline authorization and qualifying historical admission, separately from current revocation/serving. | T06 isolated permitted B admits; A observes revoke and denies; B's earlier valid admission remains eligible after heal; expiry/clock rollback/unknown policy yield no acceptance. A later bare signed revoked/expired rival has no conviction power: both orders retain security evidence and legitimate dependent text `AD` (§5). Current serve denial still applies. |
| ICD-007 | MUST retain strict origin chain/causal/logical custody with a unique canonical origin per certified epoch. | T07 seq0 and linked successor succeed; missing prev, numeric overflow and rollback fail; safe restore/retry preserves exact old bytes. Fresh E1 with repeated display name MUST use a new canonical origin; retained E0 `A` plus E1 `B` after it projects exactly `AB` in both orders (§5). E1 reusing E0's canonical origin fails before mutation. Actual concurrent-handle custody test. |
| ICD-008 | MUST recover gaps with bounded provisional state. | T08 successor-before-predecessor requests exact missing bytes and later projects; invalid ancestor excludes descendants; queue full never advances a false head or drops sender custody. |
| ICD-009 | MUST derive fork/dependency quarantine only from two independently historically qualifying rival admissions, without circular qualification or arrival-order winners. | T09 two qualifying rivals give equal prefix/quarantine/dependents and degraded retained receipts; earlier qualifying fork moves floor backward. Bare signed unauthorized/expired/invalid rival preserves legitimate text `AD` and receipts, retaining security evidence in either order. Distinct canonical E1 recovery after E0 fork preserves exact eligible identities and text `AB` (§5); unsigned/unknown-admission rival cannot convict. |
| ICD-010 | MUST make exact retry/outcome lookup stable across interruption. | T10 commit/reply loss/retry yields one original op/receipt; changed bytes same identity conflicts; quota/restart/expiry cannot remint. Physical interruption proof distinct from copied fixture state. |
| ICD-011 | MUST state achieved custody and read completeness precisely. | T11 local receipt survives declared process restart; no peer copy/global-current claim; complete local empty cut differs from missing history; permanent sole-copy loss produces loss/unavailable, no fabricated recovery. |
| ICD-012 | MUST reconcile actual app operations in both directions automatically. | T12 isolate/edit each real node/reconnect with no browser resubscribe/manual ferry; exact signed text ops and proof closure converge; holder's erased tail is offered from follower; home-only mutant fails. |
| ICD-013 | MUST enforce finite retention/capacity/rejoin rules. | T13 exact byte/origin/ref/pending/evidence boundaries; reserved recovery capacity works; accepted/tombstone/retry evidence is never evicted; retired incarnation/cursor yields explicit history unavailable. |
| ICD-014 | MUST recover coupled physical commit/handoff without false success. | T14 real write/termination/reopen at journal, record, outcome and reply cuts; known failures no partial visible commit; uncertain result exact recovery. Storage class tested at its actual promised boundary. |
| ICD-015 | MUST exclude legacy/new/mixed-version admission and retain seal safety. | T15 wrong capability/old binary/new root/delayed legacy op/rollback reject; safe fresh-identity coexistence; whole-store seal closes all writers and cannot masquerade as per-instance conversion. |
| ICD-016 | MUST preserve application/editor semantics independent of admission. | T16 text cursor/tombstone anchors survive remote and quarantine rebuild; defaults never publish; external effect invocation and saved-file authority are not manufactured by accepted text. |
| ICD-017 | MUST provide compiling boundaries, affected-consumer and fast checks. | T17 actual consumer compiles required methods, contract-faithful refusing doubles and behavior mutants fail; classified package gate refuses concrete runtime dependency; fast selection includes relevant rows and wider adapter journeys separately. |

RC-001–007 and GCA-01–09 remain traceable: RC-001/007 map to ICD-001/002/004/015;
RC-002 to 002/006/013; RC-003 to 003/008/009; RC-004 to 010/011/014;
RC-005 to 005/006; RC-006 to 008/011/016. GCA's merge/profile/cursor obligations
map to 003/007/008/016. MW's appearance-only restrictions and preference codec/
reset rows do not narrow the general independent-admission capability.

## 11. Smallest useful implementation and gates

### IC-1 completion and first code tranche

The smallest useful post-review implementation is **the production-bound pure
admission/reconciliation kernel with compiling consumers, exercised with genuine
supported text operations for two independent authenticated instances**. This
implements an actual reusable component; it is IC-2's deterministic proof,
not the complete live Glade feature. It can deliver useful decisions and missing-op
plans to the future node host without depending on real crypto/network/storage.
It MUST accept valid local edits for either isolated replica without asking a
holder, exchange candidate histories/evidence when connectivity is delivered as
an event, produce identical eligible op sets, and drive existing Taut/Glial text
projection to equal results. Merely implementing a private numerical sync fixture,
mocking the projection, or only modifying `Store` is insufficient.

Concrete minimum witness: import the existing canonical text
`concurrent_siblings` corpus's `a:1` insertion of `A` (Glade `a:0`) into both
replicas of instance X. Partition them; one locally admits `b:1` inserting `B`
after `a`, the other `c:1` inserting `C` after `a`, both referencing `a:1` in
Taut coordinates. Each gets its local receipt without a holder. Deliver the
recovery events in opposite orders; both retain exactly `{a:1,b:1,c:1}` and
released text projection is exactly `ABC`, as pinned by the existing corpus.
Repeat over independent instance Y with the same display origins but different
authenticated namespace and policy; Y's denied writer yields no admission and
X's permit/history cannot be imported. Also include a missing ancestor, a fork,
lost commit reply/exact retry, window expiry and edit-quota boundary. These use
explicit verified test evidence/commit outcomes; they do not claim physical
receipts or genuine certification. The initial kernel's consumed Glade zero-base
mapping and byte-preserving payload adapter must be tested against that corpus.

Mandatory prerequisites before that code:

1. Consistency/Safety review accepts or corrects §§2–8, especially historical
   revocation validity, trusted-admitter/time assumption, fork quarantine,
   local receipt and full-history-with-capacity refusal. Apply accepted
   findings and record precise canonical amendments/decision dispositions.
2. Add the ICD journeys/allocation to the existing Gyld declaration and pin
   its checked source tuple; no Gyld engine extension is required. Preserve
   captured responsibility ownership and current discovery transaction.
3. Pin typed internal input/output semantics for descriptor, writer/permit/
   evidence proof results, commit outcomes and recovery effects. Compile
   real downstream consumer/conformance specifications, run them RED before
   implementation, and review package roles/dependency directions. Do not
   force new public traits where an explicit pure event/effect contract fits.
4. Select the actual supported text corpus and deterministic test limits/time
   inputs. Verification results supplied to the pure kernel are labelled
   trusted test inputs; no signature or physical durability claim follows.

A public wire/API field allocation is **not** a blocker to this pure component
once its internal semantic contract is reviewed. Canonical semantics and the
consumer input/output shape **are** blockers; they cannot be postponed while
implementing unrestricted successful admission. Production owner choices about
keys, permit duration, time source, replicas and live paths **do not block** this
deterministic component: it consumes explicit parameterized evidence and tests
finite, named examples without passing them off as deployment decisions.

### Following tranches and completion

IC-3 must select/qualify canonical transfer/proof schema and signature/hash domains,
real development custody/certification and scoped authentication, offline permit/
clock adapter, coupled process-restart storage, instance enrollment, duplex app
sync, and node/client negotiation. Start with fresh text identities, isolated
test roots and real Ed25519/fixed peer Iroh routes; two actual nodes partition,
both accept, restart/exact retry, reconnect and exchange app operations both ways.
Tests include one surviving replica when its partner is permanently gone, with
no consensus timeout. This is the smallest useful **live feature** witness.

IC-4 qualifies Rust/TS clients, Glial/durable local intent, separate instances,
declared capacity and process-loss receipts, protocol exclusion/mixed versions,
whole-store-seal coexistence and affected consumers on one review tuple. A later
preference map/register profile needs exact canonical semantics/corpora and its
product scope/reset/conflict review; no new engine capability is implied by the
text witness. Raft carrier qualification can proceed independently; strong
production integration follows its own preserved gates and consumer selection.

| Decision/input | Can be proposed/reviewed now | Blocks |
| --- | --- | --- |
| Identity tuple; scoped supersession; permit historical validity; fork/read/receipt/bounds semantics | Yes, this semantic gate must settle them. | Kernel implementation if unresolved. |
| Pure contract/classification/Gyld allocation and required RED consumers | Yes; bounded next tranche, no universal API freeze. | Kernel implementation. |
| Canonical proof/container encoding, domains, generated compatibility corpus, negotiation | Yes, engineer/design tranche after semantic agreement. | Real node/client mutation, not kernel proof. |
| Account/operator roots, admitted nodes, actual key/recovery custody and antirollback locations | Actual owner/deployment inputs, never inferred from fixture keys. | That live deployment/activation; development witnesses use explicit isolated authority. |
| Permit lifetime/time trust/uncertainty, loss tolerance, quotas and enrolled replica operators | Parameterized semantics can be reviewed now; production values require owner selection. | Production activation, plus actual adapter qualification for chosen values. |
| Existing-resource import cut/profile choice and root-seal inventory | Fresh identity witness can proceed without migrating existing data. Owner's affected stores/resources require concrete inventory and transition review. | Existing-store migration/activation. |
| User-facing new receipts/conflict/quarantine UX and unsupported preference profile | Later Surface/product review on concrete interfaces. | User-facing freeze/that profile activation, not pure internal kernel. |

No production default MUST be installed to fill a missing owner input. No running
node/store/launcher/enrollment/whole-store seal is changed by this design.

### Fast feedback and qualification selection

For the new pure package, add a bounded command selecting ICD-T02–10/13 and one
two-instance released-text witness, with independent deterministic events and
no sockets, sleeps, ambient clock or application runtime. Record package name,
toolchain/machine, test execution, warm build+test and cold build separately;
no achieved budget is claimed here. Broader delivery permutations/fuzz remain
explicit system-assurance jobs, not an ever-growing mandatory minor-edit loop.

Existing relevant commands/selection are `sh glade/contracts/check.sh binding`,
`sync`, `signer`, `clock` (only those affected),
`sh glade/contracts/test-selection.sh`; focused node acceptance/store/mesh
tests under both roots; `npm test --prefix glade/client-ts` and focused Rust
session tests; from `glial`, `./node_modules/.bin/vitest run
test/text_crdt_mount.test.ts test/shapes.test.ts` plus `npm run typecheck` when modified.
Owning-package checks precede affected-consumer checks. Run the adopted node/
contracts architecture gates at boundary checkpoints; changes touching discovery
also run `glade-discover/scripts/check-architecture.sh`. Record `sh glade/node/check.sh`
as wider checkpoint verification, not the normal pure edit loop.

The existing architecture checker reports cfg-disabled-source blind spots;
host compilation cannot claim all-platform enforcement. New/modified code MUST
keep every C-style control-flow body braced and conditional Rust sections inside
`cfg_if!` or enclosing modules. Add/use a syntax-aware source check which examines
disabled branches for this tranche, and record remaining broader migrations.
Run each repository's process-globals checker against its existing allowlist;
do not loosen classifications/allowlists/dependencies to turn a failing check green.

This file's verification is source inspection and design traceability only.
No code, test, dependency, schema, Gyld declaration or runtime has changed here;
all RED/adapter/compatibility evidence above remains required future work.
