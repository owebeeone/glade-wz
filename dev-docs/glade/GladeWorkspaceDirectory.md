# Glade Workspace Directory — who owns the list, and how a node finds it

Status: working draft — **design direction + phased plan, not yet a contract**

Purpose: the substrate (`glade/dev-docs/GladeSubstrateV1.md`, M-LIMP done)
makes shares converge between sessions that are already connected. The
unwritten layer is what happens *before* that: a grazel/gryth node cold-starts
and must answer "what workspaces does this user have, which nodes serve them,
and am I allowed to?" — and someone must own that list. This document pins
where the list lives, who may write it, and how a node holding nothing but
keys finds it.

It specializes the open root decisions **GDL-008** (genesis authority),
**GDL-013** (bootstrap), **GDL-016** (provisioning authority) and **GDL-017**
(manager coordination) to the v1 case: **one user, N devices, M workspaces,
over iroh**. It consumes `GladeBootstrapModel.md` (genesis, authority bands),
`GladeDistributedControlPlane.md` (no live global master; signed leased
records), and the constraints of
`glade/dev-docs/GladeGrythSecurityModelAnalysisPrompt.md` (zero-infra default,
local-first survives, punt-without-foreclosing).

## 1. The problem, decomposed (and the chubby question)

"A p2p node discovers all workspaces the user has access to" is four separate
problems. Chubby is the reflex because Chubby bundled all four; this design
splits them, because at user scale each has a cheaper owner:

| Sub-problem | What Chubby did | What this design does |
| --- | --- | --- |
| Store the list (names, config) | small strongly-consistent file store | **a share** — the *home share*; the directory is data on the existing rails |
| Decide who may write it | ACLs on a central service | **cryptography** — ops valid iff signed under the user's root-of-trust chain; no service arbitrates |
| Keep it available | 5-node Paxos cell | **replication + rendezvous** — every device holds a replica (offline-first is already free); a durable *home node* + iroh relays for reachability |
| Exclusive things (locks, election) | lock service, master leases | **leased claim records fenced by local locks** — needed only where a real single-writer resource exists (serving a working copy) |

The load-bearing split: **authority is cryptographic, availability is
replication.** Chubby needed a quorum cell because its authority *was* its
availability — a live service had to answer "who holds the lock". Once
authority is signatures over a converging op-set, no cell is needed at
cell-size = one user's devices. Convergence never requires coordination
(SubstrateV1 §2: leases/roles are optimizations, not correctness).

The honest residue where chubby-*shaped* exclusivity survives is §4 (who
serves a working copy) — and it reduces to commodity lease-takeover, fenced by
a filesystem lock. The real risk in this design is integration surface (iroh
discovery reliability, key custody UX), not algorithmic novelty.

## 2. The home share — directory as data

Every user has a **home share**: the user-scale instance of the *system
declaration space* from `GladeBootstrapModel.md`. It is an ordinary share on
the substrate (per-origin logs, folds, destinations) whose record kinds are
directory records, taut-declared:

| Record | Content | Fold semantics |
| --- | --- | --- |
| `PrincipalDecl` | user root, device keys (cert chain root→device), agent principals | set-union, revocation-aware |
| `CapabilityGrant` / `CapabilityRevocation` | who may read/write/serve what (workspace / share / binding / verb — the units from the security prompt §3.3) | set-union; revocation wins |
| `WorkspaceEntry` | workspace id, human name, gwz manifest identity, eligible host nodes | LWW per field (taut `merge`) |
| `ServeClaim` | node X serves workspace W, lease expiry, epoch | leased observed record (§4) |
| `NodeHint` | last-known relay/addr hints per node (optimization only — iroh owns truth) | LWW |

Rules:

- **Validity filters before the fold.** An op that fails signature/capability
  verification is excluded from the op-set; the fold itself stays a pure
  deterministic function of the (valid) op-set. Folds never verify.
- **Time never enters the fold.** Lease expiry is evaluated at projection/read
  time against the reader's clock, not inside the fold — same op-set must fold
  byte-identically on every peer regardless of when it folds.
- The home share sits in the `system` authority band; workspace content shares
  stay `application` band. The bootstrap kernel + genesis bundle sit below it,
  exactly per `GladeBootstrapModel.md` — this doc adds no new band.

**Who gets to manage the list (GDL-008/-016 at user scale):** the user's root
key, created once at the genesis ceremony, is the trust anchor of the home
share. Device keys are certified under it; agents get attenuated grants under
a sponsoring device/user (shape owned by the security analysis). "Management
rights" = holding an unrevoked capability chain to the root — a fact checked
by every replica independently, not a role granted by a service. There is no
election to *mutate* the directory: writes from valid chains converge; writes
from invalid ones are ignored everywhere.

## 3. Discovery and bootstrap (GDL-013)

Strict separation of layers — do not rebuild what iroh already owns:

- The **home share** maps *user → workspaces → node ids (+ capabilities)*.
- **iroh** maps *node id → dialable address* (pkarr/DNS discovery, relays,
  local mDNS). Node id = ed25519 key = machine identity, per the security
  prompt §2.

Cold-start ladder, in order of what a node has:

1. **Steady state — local replica.** Every device is a destination of the home
   share; the list is readable offline before any network. This is the common
   case and it is already how the substrate works (SubstrateV1 §5).
2. **First device ever — genesis ceremony.** Create user root key, genesis
   bundle (per `GladeBootstrapModel.md` §Genesis: trust anchors, share space
   id, signature schemes, policy version), home share, first device cert.
3. **New device — invite ticket.** An existing device mints an out-of-band
   ticket (QR/link): iroh NodeAddr(s) to dial + home share id + a signed
   device-cert grant. New device dials, syncs the home share, is now a
   replica. This is the security prompt's "adding a machine" ceremony.
4. **Re-rendezvous — all else lost but keys.** Resolve the user's published
   node set via iroh discovery (pkarr under known node keys) and/or the home
   node's stable address.

**The home node.** One gryth node with better uptime — the canonical glade
server role from SubstrateV1 §6 — acts as durable replica + relay + ticket
target. It is **availability, not authority**: it holds no special keys and
can be rebuilt from any replica + the genesis bundle. Recommended (a fresh
device can't bootstrap while every other device is asleep), never required
for correctness.

## 4. Serving a workspace — the exclusivity residue

A workspace = a gwz multi-repo working tree + the services grazel mounts over
it (razel build, gwz-core ops, glade workspace bindings per SubstrateV1 §6).
A given *working copy* is a genuine single-writer resource — two nodes must
not run mutations/builds against the same checkout.

- `WorkspaceEntry.eligible_hosts` records which nodes have a checkout (data,
  not coordination).
- The node currently serving publishes a **`ServeClaim`** with a short lease,
  renewed while alive — the control-plane doc's leased observed record,
  collapsed to user scale.
- **The local workspace lock is the ground truth** (the `.razel-cache/`
  `workspace.lock` single-writer contract from the grazel corpus): a node
  publishes a claim only while holding the local lock. The claim routes
  traffic; the lock prevents actual double-mutation even under a split-brain
  claim race.
- Takeover = lease lapse → another eligible node takes the local lock (if it
  can) and publishes its claim with a higher epoch. Conflicting live claims
  surface as MV data in the UI ("two nodes claim W — pick/investigate"), they
  are not resolved by consensus.

This is deliberately commodity: lease + fencing token + local lock. No Raft,
no lock service. What keeps it honest is that the *resource itself* (the
filesystem) carries the real mutual exclusion; the share only carries routing.

## 5. Multi-user access (designed now, built later)

"Workspaces the user has access to" = own entries (home share) ∪ **grants**
from other principals. A shared workspace carries its own membership records
signed under *its* owner's chain; an invite exchange lands a corresponding
grant in the invitee's home share. Revocation is forward-only and stated
honestly (GDL-009): a revoked peer stops receiving new ops and routes, but
replicated history is not clawed back. An org is the same mechanism nested —
a principal with its own genesis + directory share that user home shares
reference. **v1 scope stops at single-user/N-device;** this section only
proves the mechanism doesn't foreclose it.

## 6. Boundaries

- **razel stays grazel-unaware** (the deny rule). Everything here is
  glade/grazel-side; razel's only touchpoints remain the taut wire
  (razel-dev `contracts/comms-protocol.md` REQ-COMMS-010) and the
  workspace.lock convention.
- **gwz-core embeds as a library** in the grazel node; workspace identity in
  `WorkspaceEntry` derives from the gwz manifest, not ad-hoc paths.
- **taut-shape carries the home share**: directory records are a taut schema;
  the home share rides the `log` delivery shape; conformance via the oracle
  corpus pattern (Rust/TS/Python parity) — same discipline as glade-wire.

## 7. Phased plan

Foundational-first; phases are milestones; steps aim < 500 LOC. P1 and P2 can
proceed in parallel once P0.S3 pins the schema; P3 is independent of P2
internals. Execution plan gets a GLP number in root `plan-docs/` when the
build starts.

**P0 — Ratify the model (docs only).**
- S1: this doc's §1–§4 strawman → reviewed design; record the GDL-008/-013/
  -016/-017 user-scale outcomes in `DecisionLog.md`.
- S2: run the commissioned security analysis (the prompt exists); adopt its
  principal/attenuation shape into §2's record kinds. Its deliverable 4 *is*
  §3's ceremonies — reconcile.
- S3: pin the taut schema sketch (`glade_dir.taut.py`: record kinds, merge
  annotations, canonical CBOR) + the two fold rules (validity-before-fold,
  no-time-in-fold). Exit: schema sketch reviewed, DecisionLog updated.

**P1 — Identity & genesis (first code).**
- S1: genesis bundle create/load (root key, device certs) in the rust node;
  taut schema + golden vectors.
- S2: signed ops on home-share bindings only, using the M-LIMP retrofit seams
  (principal at `HELLO`, capability-ref envelope slots, GQ-9 hash chain as
  the integrity base). Allow-all preserved everywhere else.
- S3: TS client signing + verify parity (oracle vectors).
  Exit: a home share accepts ops iff signed under a certified device key.

**P2 — The home share (localhost, no iroh).**
- S1: directory records + folds (Rust + TS, corpus-gated).
- S2: gryth node mounts the home share by default; gwz-hosted workspaces
  registered as `WorkspaceEntry` (identity from gwz manifest).
- S3: gryth UI lists workspaces from the fold over the existing WS rails.
  Exit: fresh browser session sees the full workspace list from the node;
  list survives node restart; still true offline on a second session with a
  local replica.

**P3 — iroh carrier + rendezvous.**
- S1: iroh transport node↔node (already the queued post-LIMP item) — the
  same frames, new carrier (D9: carrier-first node logic pays off here).
- S2: publish/resolve via iroh discovery (pkarr/DNS, relays); `NodeHint`
  records as cache only.
- S3: invite-ticket flow — the add-a-device ceremony end-to-end.
  Exit: a second machine joins from one ticket, replicates the home share,
  and shows the same workspace list with the first machine offline.

**P4 — Serve-claims + grazel attach.**
- S1: `ServeClaim` lease records + projection-time expiry + epoch fencing.
- S2: local workspace-lock fencing wired to claim publish/withdraw.
- S3: grazel authority provider session serves workspace bindings
  (tree/status/build exchanges per SubstrateV1 §6); UI routes to the
  claim-holder.
  Exit: two machines hosting the same checkout hand over serving on lease
  lapse without double-mutation; the UI follows.

**P5 — Cross-user grants (deferred).**
Invite exchange between principals, membership records, forward-only
revocation. Explicitly after the single-user proof; §5 only guards the door.

## 7b. Session placement and roaming

Terminology, pinned: **personal node** = operator class (the operator is the
user, wherever the box sits); **home node** = a *role* (durable replica +
rendezvous — a personal server holds it today; a DC node can hold it too);
**entry node** = the session-host role (which node a UI session HELLOs to).
There is no "local node" — that was a topological accident.

Discovery has exactly two layers plus one bootstrap step, and they must not
merge:

1. **Session placement** (before glade): the UI finds an entry node —
   localhost when present, else a DC fleet at a well-known name (plain
   DNS/TLS web bootstrap).
2. **Service discovery** = a fold over shares: who serves X (ServeClaims,
   ServiceDefinitions), signed and offline-readable. The directory IS this
   layer.
3. **Node discovery** = iroh: node id → address (pkarr/DNS, relays, mDNS).
   Self-certifying — publication is signed by the node key itself, and the
   dial handshake authenticates the key, so poisoned hints degrade to DoS,
   never impersonation.

Merging 2 into 3 (or vice versa) recreates the coordination-service problem
§1 rejected: addresses churn at network speed and must not cost directory
writes; serving rights are policy and must not depend on network liveness.

The roaming ladder (composes §3 with `GladeAuthzModel` §7a/§7b): web
bootstrap → HELLO at the entry node (device cert, or operator-vouched per the
user's own authn-method policy) → directory from the operator-held home-share
replica (`replica.hold` granted at signup) → ServeClaim → iroh dial to the
personal node, relayed through NAT. Whether the entry node is a **warm
replica** (fast reads) or a **blind pipe** (every read rides home) is the
workspace's placement grant — the performance tier is the trust tier, chosen
by the owner as data.

Fault tolerance is the same rule pointed at durability: the "database" is the
share; a node's store (sqlite or otherwise) is an engine behind a replica.
Declare a minimum replica count in share policy and grant `replica.hold` to
enough operators; the substrate's ordinary sync is the replication. Authority
failover stays §4's takeover.

## 8. Open questions (not decided here)

| # | Question | Owner |
| --- | --- | --- |
| WD-1 | Root-key custody & recovery: lost-all-devices posture (paper backup? recovery keys? social recovery?) — determines how scary genesis is | Gianni (product) |
| WD-2 | Agent (MCP) principal shape and attenuation | security analysis (P0.S2) |
| WD-3 | Metadata exposure to relay-only peers (GDL-010) — what the home node sees when it is someone else's infra | security analysis |
| WD-4 | Is the home node bundled into the default gryth install (availability by default) or opt-in? | Gianni (product) |
| WD-5 | Workspace identity when the same repos are checked out twice on one node (two worktrees = two workspaces?) — gwz manifest identity vs path | P2.S2 |
| WD-6 | Entry-node fleet conventions: well-known names, selection (nearest? sticky?), and what the SPA caches | design (with product) |
| WD-7 | Min-replica declaration shape and who repairs under-replication (a controller-shaped record at user scale?) | design (ties AZ-8) |
| WD-8 | ~~Advertisement family + repair owner~~ **RULED 2026-07-05**: ReplicaHint + ServiceInstanceClaim adopted; ServiceDefinition dedup defaults to `per-node` (recompute — determinism makes duplication correct), expensive services opt into `global`; the under-replication repair loop is owned by the **home-node role**, and any replica MAY run it idempotently as the partition fallback. | ruled |
