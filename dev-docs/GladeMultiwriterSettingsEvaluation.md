# Glade Multiwriter — bounded appearance-settings evaluation

Date: 2026-10-03. Status: **DRAFT evaluation and next-proof recommendation**. The owner authorized H1 design
work and this separate evaluation. This document does not select H3, amend an existing contract, freeze an
API, or authorize code.

Subsequent owner confirmation, GDL-054: shared browser preferences such as theme
color may remain writable during disconnection, with competing edits resolved
after reconnection. The [resource-profile requirement capture](GladeResourceConsistencyProfiles.md)
records this direction. Home-outage acceptance and temporary disagreement are no
longer merely hypothetical product conditions; the exact field scope, conflict,
permission-freshness and receipt/loss profiles remain open. The pinned historical
appearance/layout scope below is evidence for this evaluation, not a current
ruling for every browser preference. Earlier reviews do not cover this addendum.

## 1. Recommendation and product conditions

Continue H1 for resources requiring one authoritative admission point. In parallel, evaluate independent
admission only for the already-defined appearance preferences: theme, UI zoom, font scale, wallpaper and its
theme toggle. Layout remains local under the 2026-09-24 owner ruling recorded in the historical appearance
plan. Any additional preference or coupled invariant requires its own scope decision.

**Conditionally prefer per-field LWW for independently mergeable preferences** if the owner accepts
deterministic loss of concurrent edits to the same field, the schema proves fields independent, and an exact
supported profile can be ratified. First prove whole-document LWW as the existing-shape baseline. If
silently losing same-field edits is unacceptable, evaluate a conflict-exposing register instead; its
conflict presentation and resolution are product work, not free convergence. No current capability
establishes either new preference profile.

Independent admission could let authorized replicas accept preference edits while the H1 home is
unavailable. It does not supply instantaneous revocation, globally complete reads, permanent-loss
durability, or agreement on creation. Keep those conditions pending: home-outage acceptance, disconnected
permission freshness, same-field conflict policy, acceptable data loss and independent storage domains.

This evaluation excludes grants, revocations, membership, creation authority, single-writer bindings,
physical working-copy mutation, and external effects. Changing wallpaper data does not authorize fetching a
URL, writing a file or executing anything: a consumer's external action keeps its own effect contract.
Multiwriter appearance is a possible exception to exclusive admission, not an exclusive-owner failover
mechanism or a replacement for H1 across all settings.

## 2. Immutable evidence and capability boundary

All observations below come from `git show` at this tuple; live CJ work is excluded.

| Repository | Evidence revision |
| --- | --- |
| Root | `be6fc8dd09c8fab01c10f2aaf02c58e5ef808480` |
| Glade | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |

The [comparison](GladeResourceHomeComparison.md) §5 recommends this investigation; the
[alternatives](GladeResourceHomeAlternatives.md) separates placement, authority and replication.
[Authz](glade/GladeAuthzModel.md) §1 permits receiving-replica append with accepting-hop and every-folder
validation; §3a/§3b retains creation roots and signed governance; §4/§4a/§7a retains policy closure,
membership, identity-bound private keys and operator-approved plaintext placement.

The [historical appearance plan](glial/GlialAppearanceSettingsPlan.md) §1/§6 proposes one whole `value`,
`gyld.appearance`, `ws-razel`, private `self:<principal>`, `latest`, with unknown fields preserved; §8
excludes layout and retention. Its old working-tree line references and planned codec are not implementation
evidence. The already-defined product fields bound this evaluation; their exact current codec, defaults,
ranges and dependency constraints remain a profile input. The plan's `ws-razel` choice is workspace-local,
while [Zones](../glade/dev-docs/GladeZones.md) calls account-domain app settings universal: the owner MUST
settle that product scope before any cross-domain migration.

[Substrate](../glade/dev-docs/GladeSubstrateV1.md) §2/§3 defines op union and folds, and describes whole
values, MV registers and structured per-field annotations. Runtime evidence is narrower:
[ShapeDispatch](../glade/dev-docs/GladeShapeDispatch.md) accepts durable `value/log/swmr/crdt`, rejects
`message`, and folds only `value/log`. [CRDT adapter](../glade/dev-docs/GladeCrdtAdapter.md) GCA-04..07
supports explicit `text_crdt.profile/v1`; it supplies no preference map/register profile. [Catalogue
adoption](TautShapeCatalogAdoption.md) GSC-03..08 forbids inventing a `message` capability or inferring
semantics from retention tokens.

Pinned `glade/client-ts/src/fold.ts::foldValue` chooses max `(lamport, origin)`; `foldLog` additionally
orders by seq. `session.ts::append` advances a logical clock, copies causal heads only for `crdt`, and keeps
per-zone origin seq/prev. `Session.restore` loads operations into a new session with clock initially zero:
clock recovery is an open witness, not an achieved guarantee. `node/src/store.rs` validates chains and
SWMR/CRDT shape conflicts, but establishes some shape/writer facts from local history. `node/src/accept.rs`
routes client writes to the holder.

[Substrate §6 W1..W7](../glade/dev-docs/GladeSubstrateV1.md) deliberately admits through one holder;
[CrossNodeWrites §2 option B](../glade/dev-docs/GladeCrossNodeWritesPlan.md) rejects dual admission because
one node can acknowledge what another refuses. App ops are unsigned and the holder checks the forwarding
node rather than an end-to-end client principal (W2). Hash identity is not authentication. Existing folds
and multi-origin CRDT transport do not prove safe independent admission.

## 3. Compare the three merge choices

| Choice | Same concurrent situation | Evidence and cost | Conditional suitability |
| --- | --- | --- | --- |
| Whole-document LWW | A changes theme; B changes zoom from the same old document. One winning document discards the other's change. | Existing `value` fold; historical appearance choice. Reading the latest document before writing only helps when that update has arrived. | Smallest baseline if whole-preference replacement and its lost edits are accepted. |
| Per-field LWW | A's theme and B's zoom both survive; competing theme writes have one deterministic winner. | Needs field operations or a map carrying per-field identities/order, exact codec/fold/recovery and unknown-field policy. `message` is unsupported; splitting surfaces also changes declarations and chain/transaction semantics. | Preferred candidate only after proving independence, including wallpaper/toggle coupling and schema rules. |
| Conflict-exposing register / declared merge profile | Concurrent values remain visible until an explicit resolution observes the competing versions. | Needs causal context, conflict representation, deterministic projection and resolution operation. An MV mention in §3 is not a built runtime profile; text CRDT is not a preference register. | Appropriate if same-field intent must remain visible; higher UI, history and metadata obligations. |

Whole-value defaults and codec fallback cannot be used to silently rewrite invalid or unknown remote
content. Validation and projection are different: reject an invalid operation under the pinned profile;
present a declared local fallback without publishing it. Per-field merge MUST NOT yield an invalid
combination. If wallpaper and its toggle must change together, declare one compound register or atomic
operation and test it; do not claim all five grips are independent.

These are established register tradeoffs, not a selected new library. The primary [CRDT study, §3.2 and
§4](https://www.lip6.fr/Marc.Shapiro/papers/2011/Comprehensive-CRDTs-RR7506-2011-01.pdf) distinguishes LWW
from multi-value registers and discusses metadata collection. Its convergence conditions do not establish
Glade authorization or durability.

## 4. Identity, bootstrap and the limits of convergence

Replicas MUST refer to the same canonical share/domain, zone key, surface and profile version before
merging. H1's creation/binding and alias authority remain necessary: two disconnected creators naming
“appearance” cannot infer one resource from its display name. Retrying creation MUST recover the original
binding; a lost response, unknown scope or partial empty lookup MUST NOT trigger new genesis.
[WorkspaceDirectory](glade/GladeWorkspaceDirectory.md) §3 distinguishes initial genesis from another device
joining; §4's local working-copy lock is unrelated. [DiscoveryModel](glade/GladeDiscoveryModel.md) §0/§3
describes local replicated resolution, and
[RegistryContractDraft](../glade-discover/dev-docs/RegistryContractDraft.md) explicitly says an empty local
result does not prove global absence.

Appearance initialization MUST separate “not yet known”, confirmed initialized absence, an explicit default
value, and a user edit. Rendering defaults MUST NOT append defaults at every boot. Legacy import MUST have
canonical eligibility and an idempotent seed intent tied to the initial binding; two browsers importing
different legacy values require a declared conflict outcome. Multiwriter does not make unknown-as-empty
migration safe or repair pre-existing split histories.

Convergence means: **same canonical operation set, same validation/profile and same deterministic fold give
the same materialized state**. Eventual delivery and sufficient retained causal/chain evidence are
additional prerequisites. Different replicas may have different provisional views during a partition. The
proof MUST include policy evidence and validity decisions, not just payloads; otherwise identical stored
payloads can produce different authorized folds.

Each tab/session MUST keep a distinct origin chain even when principals match. An origin is not a principal,
node identity or share identity. A restored origin MUST recover its seq, predecessor and clock watermark
before appending; parallel use of one origin MUST be prevented or detected as equivocation, never tie-broken
into valid history. Exact retry preserves the original canonical op and identity.

Logical order MUST be specified independently of wall time. Current LWW's `(lamport, origin)` is
deterministic but is not “last physical click wins”. [Lamport's primary
paper](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/)
distinguishes causality from a chosen total order. A new profile MUST define causal observation, canonical
origin ordering across Rust/TS, same-origin ties, clock restoration, numeric bounds and overflow. A
forged/huge logical timestamp MUST have a declared outcome; a wall-clock skew heuristic is not a merge
proof.

Missing predecessors/dependencies MUST yield bounded pending recovery or refusal, never valid folding past
the gap. A rejected predecessor blocks its descendants; the client must recover a valid frontier before
generating replacement intent. Current `Session.refuse` removes the own tail and requires resume; that is
useful evidence, not sufficient when several replicas have already acknowledged it. Fork evidence and
byte-different retries MUST be retained/reported, and detection on one replica MUST converge to the same
quarantine outcome on all others.

## 5. Authorization, acknowledgement and retention

Independent accepting nodes and every folder MUST validate identical scope, profile, payload, chain and
authenticated origin/principal evidence. Identity- bound `self` resolution applies to append, subscribe,
replay and forwarding; workspace membership remains entitlement, and account-owner access remains the AZ-17
carve-out. Placement MUST retain `replica.hold`/operator approval. No new access entitlement may be inferred
from possessing a preference replica.

Disconnected permission freshness needs an owner-selected policy: a stale replica can accept locally before
learning revocation, or it can block until required fresh evidence is available. Instant revocation at an
isolated replica is not promised. The profile MUST define whether such acceptance is provisional, which
policy evidence decides later fold eligibility, and what a user sees on rejection. Forward-only revocation
(§4) stops new flow as evidence arrives; it **does not imply retroactive erasure** of already-replicated
history. A rule invalidating earlier accepted edits is a separate owner amendment, not an inferred
consequence. Keep governance validation fail-closed and distinct from appearance merge rules.

A future receipt MUST state local holding, process-crash persistence, synchronized storage or independent
replica confirmation separately. Current R2 `Ok` lacks fsync and survives process crash only; W4 adds
holder+forwarder copies, not an arbitrary machine-loss guarantee. Independent local success MUST NOT retain
W4's two-node promise by name. Lost replies require outcome lookup/exact retry; permanent loss before
replication cannot be repaired by convergence. Storage failure MUST leave no success receipt or partial
visible commit. Adapter claims need actual reopen, crash and fault evidence, beyond deterministic models.

Reset is a new attributed preference operation under the chosen conflict rule, not journal deletion. The
profile MUST distinguish explicit default, field absence/inheritance, deletion and whole-appearance reset;
reset racing an offline edit needs a deterministic outcome. A deletion/tombstone MUST prevent obsolete state
resurrecting after delayed delivery. Safe tombstone/operation collection requires a reviewed
frontier/checkpoint and rejoin policy, not elapsed local time.

`latest` currently describes projection, not implemented bounded storage. Before activation, bound
operation/payload bytes, fields, origins, dependency/pending queues, retry identity retention, conflict sets
and evidence. Capacity refusal MUST be explicit and MUST NOT evict chain/retry/tombstone evidence needed by
an offline replica. Long-disconnected rejoin may require an explicit refresh or retirement decision.
Retention, checkpoint trust and cursor expiry are blockers.

## 6. Narrow next proof tranche — proposed requirements and TDD scenarios

These MW requirements govern the proposed proof object only; they are not a ratified wire/API. Write each
failing scenario before implementation and retain success, failure and edge cases as deterministic data with
injected inputs.

| ID | Proposed requirement | Deterministic RED scenario / concrete adapter obligation |
| --- | --- | --- |
| MW-001 | MUST admit only the bounded appearance surface under one canonical binding/profile. | Same principal/two origins succeeds; wrong share/key/profile and literal other-self fail; lost-create-reply/alias race never mint a second binding. Binding adapter verifies authoritative outcome recovery. |
| MW-002 | MUST pin codec, defaults, ranges, unknown fields and coupled invariants. | Valid round-trip; malformed/unknown-version reject before mutation; missing field renders without a write; wallpaper/toggle race stays valid. Rust/TS codec corpus agrees. |
| MW-003 | MUST prove the selected merge policy on one validated op-set. | Partition A-theme/B-zoom, competing theme, duplicate and every causal delivery permutation; assert whole-LWW loss versus per-field preservation or visible MV conflict. Use a rejecting mutant and shared fold corpus. |
| MW-004 | MUST preserve origin chain, logical order and causal context across recovery. | Two tabs share principal, not origin; restore then append orders after observed history; numeric limit and same-origin fork fail. Origin/store adapter proves restart and concurrent-origin custody. |
| MW-005 | MUST recover gaps and quarantine equivocation without folding invalid descendants. | Deliver successor first, later predecessor, rejected predecessor, fork delivered in opposite orders; bounded pending state and identical final eligibility. Sync adapter requests exact missing data and reports evidence. |
| MW-006 | MUST make exact retry idempotent with honest outcome uncertainty. | Commit/lost reply/retry returns original identity; changed bytes fail; restart or retention boundary cannot re-mint. Storage adapter tests interrupted commit and receipt recovery. |
| MW-007 | MUST apply the selected authorization freshness rule at admission and fold. | Granted edit succeeds; unknown device, wrong principal/operator and missing policy fail; partition/revoke/heal yields declared provisional/accepted/rejected history without retroactive-erasure claims. Security adapter supplies real authentication/proof corpus. |
| MW-008 | MUST distinguish defaults, initialization and legacy import. | Unknown replay never seeds; confirmed initial absence imports once; two stale imports and explicit default/edit races have declared outcomes. Migration adapter retains seed intent across reload and lost response. |
| MW-009 | MUST specify reset/delete/tombstone behavior. | Default-set differs from absent; reset races update; old replica returns after delete; repeated resolution/reset dedups. Checkpoint adapter proves no resurrection after allowed collection. |
| MW-010 | MUST report only achieved storage/replication guarantees. | Local receipt before partition; replica loss, disk failure and lost acknowledgement do not upgrade it. Real adapter demonstrates stated fsync/reopen/failure-domain guarantees separately. |
| MW-011 | MUST enforce reviewed bounds and recovery floors. | Exact capacity limit, oversized payload/frontier, many origins/conflicts, retry below floor and retired offline replica fail explicitly. Storage/sync adapters test bounded work and refresh path. |
| MW-012 | MUST prevent mixed-version reinterpretation or simultaneous legacy/profile admission. | Old reader/writer meets new profile: reject before mutation; upgrade restart, rollback and delayed old op preserve one activation generation. Declaration/transport adapters test exact capability negotiation. |

Build the proof as pure profile/validation data and deterministic state transitions, with existing
whole-value fixtures as baseline. Do not create a new library or dependency merely for this document. Any
later extraction MUST declare its role under [LBT](LibraryBoundaryAndTestingPolicy.md), keep
transport/storage outside the pure fold, run shared conformance against replacements, check affected node,
Rust/TS client and Glial consumers, and run the adopting architecture gate. Measured fast commands, genuine
crypto and actual I/O tests remain separate gates.

## 7. Canonical amendments and owner decisions before activation

An owner-reviewed amendment MUST name the exact exception to Substrate §6 **W1** “writes follow the read
route”, **W2** “the holder decides”/“no node holds an op another refuses”, **W3** holder-first storage,
**W4** two-node `Ok`, **W5** unplaced retry and **W7** holder-established shape. CrossNodeWrites §2 option
B/§3 W1..W7 and §7's no-independent-offline-admission boundary must be reconciled with it. Preserve the
existing path for every unactivated binding.

Per-field/MV selection additionally needs an exact profile and corpus under Catalogue adoption
GSC-04/05/07/08, ShapeDispatch's capability table and CRDT adapter GCA-04..07 where reused; no `message`
activation by textual inference. Authz §1/§4 needs explicit disconnected acceptance/validity semantics if
changed; §3a/§3b, AZ-16/AZ-17 and §7a MUST remain intact. Zones' universal-account example and the
historical plan §1's workspace scope/import proposal require a recorded scope decision; the historical plan
§8 layout ruling is retained.

Pending decision log: choose whole/per-field/MV and atomic field groups; choose workspace versus account
scope and canonical bootstrap/import authority; choose disconnected freshness and any provisional receipt;
choose storage guarantee, bounds/rejoin floors and same-field/reset UX. Activate only after the amendment,
profile conformance, adapter evidence and dual review pass on one committed tuple. Roll out readers before
writers, explicitly exclude legacy admission, and require a migration/rollback plan that cannot reinterpret
new history as whole-value LWW.

Verification for this draft: source inspection and primary-paper lookup only. No implementation, dependency,
test-selection or runtime change; no claim that the current code safely implements Multiwriter preferences.
