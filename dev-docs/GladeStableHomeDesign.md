# Glade H1 — dynamic creation with a stable serving home

Date: 2026-10-03. Status: **DRAFT design for review; H1 design direction authorized,
production implementation and wire/API ratification pending**.

The owner directed proceeding with the [recommended design](GladeResourceHomeComparison.md)
and evaluating [multiwriter settings](GladeMultiwriterSettingsEvaluation.md).
This document makes H1's creation, identity and admission mechanism concrete.
It does not implement H2 transfer, H3 failover or multiwriter acceptance.
H1's declared profile permits cached reads and pending edits while the home is
unreachable; it promises no replacement of that home. This is the proposed H1
availability contract, not a claim about shipped settings behavior.

## 1. Scope and source authority

The placement unit is a **share**, containing declared `(glade_id, key)` zones.
A browser is a client, not a serving home. Settings retain their app/scope/
principal address; a session ID, port or transport endpoint is not that address.
Runtime creation needs no manual per-resource mapping. Bootstrap trust, identity
and an authenticated invite remain required.

Sources were examined at root `be6fc8dd09c8fab01c10f2aaf02c58e5ef808480`,
Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, and Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`. The review root pin is in the ledger.
Uncommitted cold-join work, launcher/discovery lanes and scratch scripts are not
evidence for this design. Root authority includes:

- [GladeAuthzModel](glade/GladeAuthzModel.md) §1/3a/3b/4/4a/7a:
  creation-rooted ownership, signed governance, local permission checks and
  operator-authorized hosting. A home does not gain grant-issuing authority.
- [WorkspaceDirectory](glade/GladeWorkspaceDirectory.md) §4/WD-8,
  [DiscoveryModel](glade/GladeDiscoveryModel.md) §0/3 and
  [buy/build matrix](GladeBuyBuildMatrix.md) D-06/R7/R9/R16:
  replicated discovery is a projection; a physical-copy lock fences that copy.
- [SubstrateV1](../glade/dev-docs/GladeSubstrateV1.md) §2/6 and
  [CrossNodeWritesPlan](../glade/dev-docs/GladeCrossNodeWritesPlan.md) W1–W8:
  current holder-directed admission and limited data acknowledgements.
- [RegistryContractDraft](../glade-discover/dev-docs/RegistryContractDraft.md):
  exact retry and partial local knowledge; its journal/registry ports do not
  provide this whole creation transaction or exclusive resource acquisition.
- [Build entry](GladeBuildEntry.md), [library policy](LibraryBoundaryAndTestingPolicy.md)
  and [package architecture](GladePackageArchitecture.md): contract-first staging,
  framework-free logic, narrow ports and deterministic conformance.

## 2. Concrete profile and trust boundary

**One creation/admission journal at the share's home is the serialization point.**
Names created inside that share and their zone declarations are serialized there.
All zones inherit the share's fixed home. Selecting different homes for individual
zones is not part of H1; a separately created share is the placement boundary.

An authorized creator of a **new independent root** proposes a fresh share identity,
its root authority, initial declaration and an eligible reachable home. That home
accepts the signed genesis and durably commits it. It may be the creator's node.
Two independent roots with the same display label are different identities.
No global name reservation is claimed. A second device joins through the existing
root/invite, not by minting another root from that label.

For an **existing share**, creation requests go only to the bound home. Its
authorized namespace policy serializes canonical names. The initial profile
requires owner or authorized administrative creation/declaration authority;
ordinary write permission alone does not grant it. Exact create/retire action
encoding and attenuation coverage require the contract tranche's authorization
corpus; this document does not invent a shipped grant verb.

An administrator MAY preauthorize a declaration template for a user's derived
private settings address. Instantiation still passes the same naming/creation
gate and verifies that authenticated principal against the template. This is
explicit delegated creation policy, not an arbitrary client's first write
declaring a shape. Account-owner implicit rights and workspace membership's
private-zone rules remain inputs to that mapping.

The fault model is honest authorized operators, crash/restart, lost/duplicated/
reordered messages and partitions. It excludes a root intentionally signing
contradictory genesis, or an operator cloning its home identity/journal to run
two independent instances. Signatures prove provenance, not non-equivocation.
Conflicting genesis or binding evidence MUST cause quarantine/conflict, not a
winner by epoch or node ID. Byzantine-owner recovery needs a different profile.

The home MUST exclusively own its journal and served store. Its lifecycle gate
holds an actual storage/process lock before admitting operations and drains
before releasing it. Copied credentials on another machine are not a safe
restart or transfer. A restored store with uncertain completeness, missing
journal, identity mismatch or detected rollback MUST remain unavailable.
Preventing malicious storage rollback or copied keys is outside this crash model;
operator restore procedures MUST NOT claim to be automatic failover.

## 3. Semantic records and identities

These are semantic fields for a future taut profile, not new wire structs.
No user command, public API or binary encoding is frozen here.

| Record | Required contents and invariant |
| --- | --- |
| Root genesis | Fresh stable share ID, authority-root reference, fixed home principal/node binding, initial signed declarations and profile/version. Creator authorization and home operator eligibility verified before acceptance. |
| Zone declaration | Stable `(share, glade_id, key)` address, canonical namespace name, shape/schema/profile digest, relevant principal/zone policy, active/retired state and declaration revision. It cannot acquire a new shape from whichever write arrives first. |
| Creation intent | Authenticated requester + scope + durable request ID, exact signed draft/digest, proposed identity/address and expected namespace revision. Reusing the request ID for different intent is an error. |
| Binding receipt | Original accepted intent, immutable home and identity, committed namespace revision, profile and metadata-storage guarantee. This is not an application-data receipt or a grant. |
| Retry/outbox entry | Original receipt/outcome and bounded pending advertisement. Delivery acknowledgement cannot erase deduplication or create a new binding. |
| Retirement entry | Original identity/address, durable retirement revision and retained replay fence. Name reuse, if admitted, MUST name a distinct identity/address incarnation; the old one never becomes active again. |

For initial canonical names, use validated typed identifiers and exact canonical
taut bytes, with case-sensitive byte equality. Display labels are separate.
No fuzzy Unicode/case/path aliasing is supported. Zone/private keys MUST be
derived from authenticated scope/principal by the agreed profile, never trusted
from a caller string. Two different encodings claiming one canonical identifier
are invalid; normalized aliases require an explicit later namespace profile.

Exact existing addresses such as textual `ws-razel` are not freshly minted root
identities. They require the migration procedure in §8. IDs and stable home
references MUST survive restart independently of transient connection IDs.
Admission generation is initially fixed for the binding; retirement revokes it.
Generation fields reserve a semantic migration seam, not permission to increment
locally and take over. An H2/H3 upgrade requires a separate reviewed transition.

## 4. Creation and crash recovery

The home validates signed intent, complete available authority, operator policy,
canonical address and declared profile before entering its creation gate. Checks
are revalidated against the gate's local policy/declaration revision at commit.
One atomic durable transaction retains the root/name binding or zone declaration,
exact intent/receipt, namespace revision and advertisement outbox. Neither a
network reply nor successful gossip is the commit boundary.

| State/event | Required outcome |
| --- | --- |
| No committed intent; valid fresh create | Commit exactly one active binding/declaration, receipt and outbox. |
| Exact same intent retried | After current access validation, return the original outcome before stale expected-revision rejection; do not reallocate, reseed or extend a lease. |
| Same request ID, different signed draft | Reject retry mismatch with no mutation. |
| Different intent, same occupied canonical name | Return conflict with an authorized reference to the existing identity; never silently merge roots or replace the binding. |
| Stale expected revision, no exact committed retry | Conflict; caller resolves and submits a new explicit intent. |
| Crash before transaction commit | No partial root/name/receipt/revision/outbox becomes visible as accepted. Retry may commit the original intent. |
| Crash after commit before reply/publication | Reopen returns exact outcome; outbox replays idempotently. No client may interpret missing reply as abort. |
| Corrupt/unreadable/missing journal for known share | Unavailable/quarantined; no empty initialization or identity reuse. |
| Retire races with create/write | One gate orders declaration/retirement and authoritative admission; no write may commit after the local retirement fence. |

Metadata acceptance MUST survive node-process crash and OS crash/power loss on
retained healthy storage. This is a **new proposed metadata guarantee**, stronger
than current R2 application `Ok`. A concrete adapter MUST prove atomic commit,
file/directory durability where required, recovery and corruption detection;
plain flush or current unsynced app-store writes are insufficient evidence.
Permanent medium loss remains loss/unavailable. Application data retains its
separately declared acknowledgement guarantee; metadata durability does not
upgrade settings data by implication.

Outcome lookup may return accepted/conflict/denied/unavailable/unknown, with
privacy-filtered evidence. Permission revocation can prevent disclosing an old
receipt but cannot undo its committed binding or turn its intent into a new one.
Cancelled calls remain queryable after possible commit. Capacity exhaustion
MUST fail explicitly before commit; request/retirement history cannot be silently
evicted to reclaim names. A bounded initial profile may stop admitting new
creates instead of compacting unproven identity/replay evidence.

## 5. Discovery and authoritative admission

Invites or an authenticated namespace response give a stable root/address and
its verified home binding. Published records make it discoverable; transport
discovery finds that node's current endpoint. Lease expiry reports availability,
not binding retirement. A local directory result is always partial.

The first browser creates explicitly or opens an existing reference. The second
uses the same reference, subscribes through its contacted node, and resolves to
the bound home. Unknown placement/history means unavailable/unknown; it does
not authorize defaults, legacy migration or another local store. New independent
scope creation remains an explicit user/authorized application intent.

Every H1 authoritative append or effect MUST pass an **admission gate** at the
bound home's real storage/effect boundary: verify identity/binding, active
declaration/incarnation, local current applicable permission, declared shape/
schema and origin-chain/equivocation constraints, then commit through the gate.
Retirement and local policy updates share that ordering. A queued effect MUST
revalidate immediately at execution or carry a capability the actual effect
sink enforces; an earlier route check is insufficient. Physical-copy mutations
also require their actual checkout lock; H1 does not replace that lock.

A non-home may relay or retain an authenticated replica copy after accepted
operations; it MUST NOT acknowledge a new authoritative append independently.
The admission gate compares the committed binding, not whichever claim wins a
local expiry/epoch fold. A forged/stale advertisement can misroute or deny service
but cannot create another valid acceptance path within the declared fault model.
Calls on a retired address never create a replacement zone.

Permission means current **locally validated replicated policy**, not globally
instantaneous revocation. Unknown/invalid/expired proof or uncertain required
clock fails closed. Unseen remote revocation can remain effective only once its
evidence propagates, as Authz §4 already states. H1 MUST NOT advertise an atomic
global revocation barrier or compensate for an unavailable proof by guessing.
If a stronger freshness barrier is required, its availability cost is a distinct
contract choice. Placement/hosting permission changes can stop home admission;
they do not grant another node the same binding.

## 6. Reads, defaults, reset and loss

H1 does not centralize local authorization decisions. Authorized replicas MAY
serve cached reads with explicit completeness/freshness limits; a missing local
zone is not confirmed empty. Live authoritative subscription/replay is from the
home and identifies a verified cut under the selected session contract.
Default values may be local placeholders. Persisting a default or importing
legacy settings requires an explicit intent after confirmed authoritative
absence at that cut and the declaration gate; stale browsers cannot reseed.
If conditional initialization is not representable on the existing wire, it
MUST wait for the contract tranche rather than simulate CAS using a read/write gap.

Reset is an ordinary authorized update to declared defaults, with normal origin
identity/retry; it is not retirement. Deleting a resource requires signed
administrative retirement. Name reuse is initially unsupported; retained
tombstones prevent delayed create/append retries from resurrecting it. A later
reuse profile requires a new stable incarnation and explicit alias semantics.

While home is unreachable, local edits remain pending under the client's actual
persistence guarantee. Memory-only edits must not be called durable. After
reconnect, replay settles original op identities; denial, chain gaps, conflict
or Retention are not silently converted into success. Home data loss is explicit
loss/unavailable; a fork/import has a different identity and requires intent.
The default two nodes add no automatic takeover or permanent-loss guarantee.

## 7. Boundaries and staged delivery

| Proposed role/boundary | Responsibilities |
| --- | --- |
| Pure creation/admission rules | Deterministic transition inputs/outputs, canonical intent matching, namespace conflicts, declaration and retirement ordering; injected policy/clock/storage outcomes. No artificial service trait. |
| CreationLedger contract and adapter | Atomic create/declare/retire transaction, exact outcome lookup, durable reopen/outbox replay and explicit capacity/corruption. Meaningful replaceable storage operations; no runtime/DB types in the contract. |
| Policy/identity verification ports | Existing trust/signature seams where adequate; bind exact action, requester, operator and complete proof closure. No marker interface or assumed distributed atomicity. |
| Admission/storage/effect ports | Enforce the committed identity/incarnation and declaration at append/effect commit; distinguish accepted data from pending and replica copies. |
| Node integration | Route translation, lock lifetime, injected providers, bounded dissemination/retry, status and graceful drain; Shaku/sdax remain at their existing assembly/runtime ownership. |

These are responsibilities, not a new mandatory crate per row. First map them
onto the existing Gyld architecture responsibilities per BuildEntry, then settle
compiling consumer/conformance contracts. The current independent registry
ports MUST NOT be composed as if separate successful calls create one atomic
namespace transaction. No dependency or package extraction is selected here.

The proposed allocation uses the existing `DirectoryRules`/`Directory` for
namespace transitions and projection, `Admission` for validation, `Records` and
`StorageAdapter` for metadata acceptance/recovery, and the runtime host for
locking/drain. The [source-qualified capture](/Volumes/projects/limbo/gyld-wz/dev-docs/case-studies/glade/ProblemSpaceCapture.md)
is a problem map, not new authority. Before implementing recomposed ports, the
[Gyld declaration](/Volumes/projects/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py)
MUST record the actual allocation and compiling consumer witnesses required by
BuildEntry. Neither external model file is changed or claimed ratified here.

Delivery: (1) review this semantic design and amendments; (2) TDD pure rules and
consumer/adapter contract witnesses, settle taut/authz/version corpus; (3) prove
real storage/lock/signature adapters and overlapping races; (4) migrate one
declared development share behind exclusive activation; (5) verify the two-node
and browser journeys. Measure fast paths at each package; wider tests follow
affected contracts. No production implementation is authorized by this document
alone, and no current process-global allowlist or dependency gate is relaxed.

## 8. Migration and exact amendment obligations

H1 runs only for explicitly activated new-format shares; **one address cannot
have a legacy admission path and an H1 path simultaneously**. The contract
tranche MUST freeze a profile/version discriminator enforced by all serving
adapters. A deployment option alone is not that protection.

For existing `ws-razel` or account-like textual identities: quiesce every
authoritative legacy path, inventory all acknowledged history/possible splits,
choose the intended root and one eligible home through authorized administration,
reconcile or report unrecoverable history, commit a verified binding/declaration
and baseline cut, then enable H1 only on nodes with its gate. If unavailable old
writers or unaccounted history prevent exclusion, migration stays blocked.
No deterministic live claim winner proves this procedure complete. Rollback
MUST preserve the binding/retirement gate; it cannot reactivate unfenced legacy
writers. Copying an old data directory does not create a new authorized home.

| Controlling clause | Proposed reconciliation required before code activation |
| --- | --- |
| SubstrateV1 §2, §6 W1/W2/W7 | H1 share placement uses the committed stable binding instead of the current live-claim winner; unclaimed-local fallback disabled for H1; shape/schema fixed by declaration rather than first arrival. Current W1–W8 remain controlling for legacy shares. |
| SubstrateV1 §6 R1/R2/W4 | New creation/metadata outcomes and stronger metadata durability are separate from unchanged application `Ok`; no implied data quorum or restored-history receipt. Wire mappings require separate corpus/consumer review. |
| WorkspaceDirectory §4/WD-8; DiscoveryModel §3 routing/home roles and §7 epoch-fence/takeover scenario | H1 advertisements project availability only; lease lapse cannot move a binding. Local working-copy locking still governs the physical resource. Add an H1 test rejecting epoch-based takeover while retaining §7's legacy scenario for unactivated shares; historical takeover cannot be generalized to H1 stores. |
| Authz §1/3a/3b/4/4a/7a; GDL-031/034 | Keep local permission checks, signed administrative/name operations and operator eligibility. Explicit creation/declaration/retirement authority mapping and forward-only freshness limits need conformance; no new owner by hosting. |
| GDL-036/037/038/043/045; matrix D-06/R7/R9/R16 | Discovery stays a local signed-record projection; journal/admission is resource-local, not a consensus registry. Name/binding metadata and receipts need explicit record-profile amendments. H3 stays out of scope. |
| Historical GlialAppearanceSettingsPlan §1/§3 migration | Its location/migration recommendations are not global-absence proofs. H1 initialization must use declaration/conditional admission and verified replay; per-user/app identity and local layout ruling are retained. |

These are proposed qualified amendments, not edits to frozen discovery bytes or
canonical rulings. The contract tranche MUST enumerate exact wire/schema clauses
and affected consumers before freezing them. [GDL-050](DecisionLog.md) records
the present owner direction and unresolved review choices.

## 9. Requirements and future closure tests

No implementation tests are claimed here. Each scenario MUST first fail against
the implementation under construction; pure traces use explicit time/randomness/
delivery, concrete adapters add actual I/O/crash/crypto/lock evidence.

| ID | Invariant / required closure |
| --- | --- |
| SH-001 | Fresh independent genesis chooses an eligible runtime home; duplicate human labels remain distinct; wrong creator/operator proof rejects without mutation. |
| SH-002 | Overlapping same-name requests at one existing scope have one winner/conflict; same/different intent retries and cancelled/lost postcommit replies recover exact outcome. |
| SH-003 | Kill before/after every metadata commit boundary; reopen exact identity, receipt and outbox; OS/power-loss adapter durability separately exercised. Corruption/rollback/missing journal never initializes empty. |
| SH-004 | Empty partial lookup, cold join and delayed advertisements cannot create another binding/default. Confirmed-empty conditional seed races with a real update without overwriting it. |
| SH-005 | Partition readers disagree on claims/clock; only the bound home gate admits. Non-home legacy fallback, copied/wrong binding and stale declaration are rejected. |
| SH-006 | Retire/update/admission overlap through one gate; queued effect rechecks its actual sink/checkout lock; restart retains fence. Host permission loss stops admission without takeover. |
| SH-007 | Same-zone shape/schema conflict, origin equivocation/gap and wrong private principal reject identically on client/relay/home; accepted-versus-pending statuses stay honest. |
| SH-008 | No home: cached read/pending edit versus authoritative acceptance distinguished; reconnect settles same intent; permanent missing history is loss/unavailable, not defaults. |
| SH-009 | Bounded names/requests/proofs/outbox; capacity/clock uncertainty fails explicitly; retry/retirement evidence never silently evicted; no private referral disclosure. |
| SH-010 | Mixed-version activation and rollback cannot create overlapping admission; blocked migration with offline legacy writer/history gap; valid migrated share keeps identity and cut. |

Review uses [review-loop](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
with Consistency and Safety on the settled H1 + multiwriter packet. The ledger
records exact tuple, prompts, verbatim verdicts, dispositions and remaining
contract/wire decisions. Design review GO accepts this design scope only; it
does not claim executable interfaces, completed migration or distributed proof.
