# Iroh ↔ Glade requirement mapping — what the ecosystem covers, how to lean on it, what it leaves

Date: 2026-09-12. Status: **analysis for owner decision; not a ruling, not an
adopted dependency plan, no code changed.** It takes the iroh facts from
[IrohReview.md](../glade/dev-docs/IrohReview.md) (verified 2026-09-12) as given
and does not re-verify them; every claim about Glade cites the design document or
decision it comes from. Where the review is silent, the gap is marked *unverified*
rather than assumed either way.

Question answered: which of Glade's requirements the iroh ecosystem already meets,
the distinct ways iroh can be mapped onto Glade, what each mapping does to the
*scope* aspects that are still undecided, and the enumerated requirements that no
mapping addresses.

## 1. Summary

- **Iroh answers exactly one of Glade's three discovery layers.** GDL-032 fixes
  session placement → service discovery → node discovery as separate layers. Iroh
  is the third layer only: node id → dialable, mutually authenticated route (raw
  public-key TLS, hole punching, relay fallback, multipath, pkarr/DNS/mDNS/DHT
  address lookup, tickets). It contributes nothing to service discovery (the
  fold over `ServeClaim`s), nothing to authorization, and nothing to record or
  scope semantics. This matches the owner's own framing in GAD-05 ("Iroh provides
  endpoint connectivity and address discovery; the proposed namespace/shard
  registry sits above it") and the review's capability matrix.
- **Three mapping shapes exist; only one is a substrate change.** (A) iroh as the
  authenticated carrier under Glade's own taut protocol (today's design, hardened
  with relays, address lookup, hooks, tickets); (B) A plus `iroh-gossip` as the
  dissemination overlay for directory/home-share announcements; (C) `iroh-docs`
  or `iroh-blobs` as the replicated record substrate. C is a poor fit for records
  (keyed last-writer-wins, namespace-secret write capability, no revocation,
  time inside the merge rule) and should be rejected for that role; `iroh-blobs`
  remains a candidate for *bulk payload transfer only*, behind a port the current
  architecture does not yet name.
- **Scope is Glade's, in every meaning of the word.** Glade uses "scope" for at
  least five different things (§5). Iroh has one construct that touches any of
  them: connection-level acceptance by peer key and ALPN. The undecided question
  is not "which iroh feature implements scope" but "which of three enforcement
  models bounds *dissemination* scope": topology (who may connect on which
  protocol), node trust (any accepted node may hold what its operator is granted;
  policy at serve), or cryptography (per-share payload encryption). Iroh supports
  the first two mechanically and the third not at all beyond transport
  encryption. §5.2 lays out the decision.
- **Of the 27 captured concerns, iroh fully addresses none, materially assists
  four, and touches nine others only at the transport edge; fourteen it does not
  address at all** (§6). Of the 24 responsibilities, iroh is the lead mechanism
  for one (`peer_connectivity`) and a partner for four more. The "how do we
  leverage iroh" answer is therefore narrow and concrete (§7): a hardened carrier
  adapter, a transport-key binding record, self-hosted relay/DNS, tickets for the
  invite ceremony, optional gossip for directory push, and a gated blobs port.
- **Two facts in the review change existing Glade assumptions.** The discovery
  model's v1 topology ("flat mesh + the public iroh relay") meets n0's
  "not suitable for production" terms for free relays, so v1 needs a self-hosted
  `iroh-relay` or the paid tier. And the lock resolves iroh 1.0.2 while 1.1.0
  fixed a deserialization panic on crafted `EndpointAddr` values, which matters the
  moment addresses come from tickets or directory records rather than a CLI flag.

## 2. Inputs and the requirement spine

Glade's requirements are recorded at three levels, all used here:

| Level | Source | Used as |
|---|---|---|
| Kernel requirements `GLA-001..082` | [GladeRequirements.md](glade/GladeRequirements.md) | §6.2 group-by-group coverage |
| Problem inventory `GPI-01..18` | [GladeProblemInventory.md](GladeProblemInventory.md) | §6.1 rows, failure scenarios |
| Captured frame: 27 concerns, 24 responsibilities, 6 journeys | [glade-problem-space.gyld.py](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-problem-space.gyld.py) and the [architecture candidate](arch1/GladeArchitecture.md) (candidate 1, revision 3) | §6.1 spine; §7 attaches iroh to the components that own each responsibility |
| Ratified constraints | GDL-031..038, GDL-040..041 in [DecisionLog.md](DecisionLog.md); GQ-2/GQ-9 in [GladeSubstrateV1.md](../glade/dev-docs/GladeSubstrateV1.md); INV-D0..D6 in [GladeDiscoveryDesign.md](glade/GladeDiscoveryDesign.md) | Fit tests: a mapping that violates one is rejected, not "adapted" |

Iroh facts come from the review's §3–§10 and §11. The review's own limits carry
over: the `EndpointHooks` semantics, relay terms and crate statuses are as
documented on 2026-09-12; numeric transport defaults were not verified; the
`iroh-blobs` "not production quality" sentence may be stale boilerplate.

Current usage, for reference (review §11, [GladePeerSyncNotes.md](../glade/dev-docs/GladePeerSyncNotes.md),
[GladeDirectoryNotes.md](../glade/dev-docs/GladeDirectoryNotes.md)): `presets::Minimal`,
relays and address lookup disabled, localhost dial by `(EndpointId, ip:port)`,
ALPN `glade/node/1`, hand-driven `accept` without a `Router`, one bidirectional
stream per link for HELLO plus a stream per forwarded zone or sync pull, and a
Glade node id (`sha256(node.key)`) that is deliberately *not* the iroh key.

## 3. What iroh provides, by Glade layer

The capability matrix from the review, re-sorted into Glade's own boundaries.

| Glade layer / owner (arch1) | Iroh provides | Guarantee limits that matter to Glade |
|---|---|---|
| **Iroh carrier** (I/O adapter, `peer_connectivity`) | Dial by public key; mutual authentication (TLS 1.3 raw public keys, `remote_id()` proven); ordered bi/uni streams, unordered receive, datagrams; 0-RTT; multipath with automatic direct↔relay migration; keep-alive/idle timeouts | Ed25519 only; no per-stream priority scheduling; relay traffic E2E-encrypted but relays see ids, timing, sizes |
| **Sessions** (`client_sessions`, reconnect) | Path events and `online()` watchers; connection close semantics; ALPN-multiplexed protocols via `Router`, version negotiation by offering several ALPNs | Reconnect at the transport level only; session resume, heads, correlation are Glade's |
| **Node discovery** (GDL-032 layer 3) | `AddressLookup` services: DNS/pkarr (n0-hosted or self-hosted `iroh-dns-server`), mDNS (separate 0.x crate), mainline DHT (separate 0.x crate), static memory; `UserData` payload in published records; `AddrFilter` | Resolves an *endpoint key* to addresses. It cannot answer "who serves share X"; publication is signed by the endpoint key, so it is node-asserted, never owner-granted |
| **Bootstrap** (`bootstrap`, GPI-01) | `iroh-tickets`: string-encoded `EndpointAddr` plus an application payload; the documented out-of-band bootstrap | The ticket carries addresses, not trust; Glade's invite grant is the payload |
| **Admission** (accept edge only) | `EndpointHooks::before_connect` / `after_handshake` → accept or reject by remote key and ALPN before any application frame; `ProtocolHandler::accept` sees `remote_id()` and `alpn()` | Observe-or-reject only; no ACL, capability or policy model anywhere in the ecosystem |
| **Relay / deployment** | Self-hosted `iroh-relay` (Let's Encrypt or own certs, WebSocket for browsers, per-connection rate limiting, authenticated relays by signed capability token bound to the endpoint key, allowlist or shared password) | Free public relays: rate limited, no SLA, "not suitable for production", metadata visible to n0 |
| **Dissemination overlay** (candidate) | `iroh-gossip`: HyParView membership plus PlumTree broadcast per 32-byte `TopicId`; `broadcast` / `broadcast_neighbors`; `NeighborUp/Down`, `Lagged` | Best effort; no persistence, ordering, delivery or membership guarantee; oldest messages dropped under backpressure; 4 KiB default message size; `delivered_from` is the relaying neighbour, not the author; 0.x |
| **Bulk transfer** (candidate) | `iroh-blobs`: BLAKE3-verified streaming, chunk-range requests, resumption, multi-provider downloader, tags/GC, provider event handler with `Throttle`/`AbortReason` | Any peer may request any hash the store holds unless the handler rejects it; "not production quality" per its own docs; 0.x |
| **Replicated KV** (candidate) | `iroh-docs`: `(namespace, author, key)` → hash+length+timestamp, range-based set reconciliation, per-namespace write secret / read-only capability, `DocTicket`; `iroh-smol-kv`: per-writer signed KV over one gossip topic | Keyed last-writer-wins, not an append log or text CRDT; capability is per namespace, not per key or principal; future timestamps accepted up to 10 min; least active protocol crate |
| **RPC framing** (candidate) | `irpc`: typed request/response and streaming over memory, QUIC or iroh | Rust-only, 0.x; Glade's wire is a taut corpus with Rust/TS/Python parity |
| **Observability** | `iroh-metrics`, `qlog`, `tracing`, net report (unstable feature), `iroh-doctor` | Transport facts only; `node.status` semantics are Glade's |
| **Browser** | wasm build, relay-only over WebSocket; official bindings expose the endpoint surface only | GQ-2 already keeps browsers on a WebSocket to a node; nothing here changes that ruling |

What the ecosystem does **not** provide, in the review's own words: authorization
or ACLs, reliable broadcast, store-and-forward messaging, a text CRDT, a hosted
tracker, and any protocol crate at 1.0 other than `iroh-ping`.

## 4. The mapping shapes

Each shape is judged against the ratified constraints that a mapping must not
violate: the three-layer discovery separation (GDL-032), the record fold as the
only runtime authority (GDL-037), the pure discovery kernel with no transport in
its boundary (Discovery Model §2), persist-before-gossip (INV-D5), transport
independence at the semantic level (GLA-082), and browsers as WebSocket clients
of a node (GQ-2).

### 4.1 Shape A — iroh as the authenticated carrier (current direction, hardened)

Iroh carries Glade's existing taut frames between nodes and nothing else. Every
Glade semantic stays in Glade. What "hardened" adds over today's adapter:

| Addition | Iroh construct | What it buys | Constraint check |
|---|---|---|---|
| Real routes beyond localhost | `RelayMode::Custom(RelayMap)` with a self-hosted `iroh-relay`, or Iroh Services Pro; `portmapper` default feature | NAT traversal with relay fallback and multipath migration: the transport half of `Reconnection` (GPI-02) | Free public relays are ruled out for production by n0's terms; the Discovery Model's "public iroh relay" needs revising |
| Node address lookup | `PkarrPublisher`/`PkarrResolver` + `DnsAddressLookup` against a self-hosted `iroh-dns-server` (or n0's), `iroh-mdns-address-lookup` on LANs | Cold-start ladder step 4 (re-rendezvous from keys) and step 3 (ticket dial); replaces `NodeHint` records, which the directory design already calls "optimization only — iroh owns truth" | Requires the directory to know each node's *iroh* key (§7.2); today it records only `sha256(node.key)` |
| Accept-time peer policy | `EndpointHooks` reject by `remote_id()`/`alpn()` before the first frame; `Router` per ALPN | Cheapest bound on GPI-03 hostile-peer load and the first check of `verify_identity`; not authorization | Must remain a pre-filter: HELLO and every frame still pass Admission (A4 in arch1: fresh observations validate) |
| Protocol versioning | Offer `glade/node/2` and `glade/node/1` ALPNs on one endpoint | Version negotiation without a Glade-level handshake change | ALPN names are carrier detail, kept out of the wire IR |
| Invite ceremony | A Glade `Ticket` type on `iroh-tickets` carrying addresses + home share id + the signed device-cert grant | Directory §3 step 3 exactly; QR/link out of band | The `EndpointAddr` inside is untrusted input: needs iroh ≥ 1.1.0 |
| Lanes | Separate QUIC streams (or connections) for control/sync, interactive and bulk; datagrams for lossy presence if ever wanted | Substrate §6's one-lane-per-binding rule; QUIC gives per-stream flow control | QUIC does not prioritise streams: Glade's `OutQueue` scheduling stays in Glade (GPI-03) |
| Version bump | `iroh = "1"` already admits 1.2.0; lock it | Three security fixes in 1.1.0 | 1.x wire compatibility promise covers all bindings |

Fit: every constraint holds by construction. The discovery kernel keeps its
"taut messages + a clock" boundary; the carrier implements the draft
`Transport::send` (`LocalAcceptance` only) plus the new duplex/session port that
[Components.md](arch1/Components.md) already lists as the gap. Cost: Glade keeps
building its own dissemination (today: a pull each way at connect, a best-effort
home-share push, transitive gossip deferred). Shape A is the floor every other
shape assumes.

### 4.2 Shape B — A plus `iroh-gossip` as the dissemination overlay

Use gossip topics to *announce* directory and home-share changes to all
interested trusted nodes; keep Glade's sync round (`SyncStart`/`SyncOps`/`SyncEnd`)
as the repair path and the only path that moves canonical bytes a peer is missing.

- **Topic per share** (`TopicId = hash(share id)`), or per share class (home
  shares vs application shares). The directory design's gossip fan-out bound
  (≤ 8 per node) becomes HyParView's active view (about five neighbours per
  topic) and PlumTree's tree.
- **Payload**: either the exact accepted canonical `SignedOp` bytes (a `DirOp`;
  records are ≤ 16 KiB, so the 4 KiB default must be raised) or only per-origin
  heads/digests, with the peer pulling the gap. Heads-only keeps announcements
  small and turns gossip into a change notification; INV-D5 (gossip only after
  persist) is satisfied either way because the announcement is emitted after the
  host's durable commit, exactly as today's `Gossip` effect.
- **What gossip's weak guarantees cost**: nothing semantic. Absence of author
  binding is irrelevant (records are B5-signed and chain-verified per origin);
  absence of ordering or delivery is absorbed by the monotone fold and the sync
  round (INV-D6 already assumes loss, reorder and duplication); `Lagged` maps to
  "schedule a sync round".
- **What it costs in trust**: HyParView membership is open to any endpoint that
  knows the `TopicId` and is accepted on the gossip ALPN. The review records no
  per-topic admission hook (*unverified*: check `iroh-gossip` for one before
  relying on this). Per-share dissemination scope therefore has to be enforced
  either by which peers are accepted at all (§5.2, model T) or by running one
  `Gossip` instance per share with its own ALPN (`Gossip::builder().alpn(..)`) and
  rejecting unauthorised peers per ALPN in `EndpointHooks`. The second option
  multiplies QUIC connections (one per peer per ALPN) and swarms; the first
  accepts that a node trusted for share A learns *that* share B changed (ids and
  heads, not payloads) — a GDL-010 metadata question, not a confidentiality
  breach of content.
- **Status risk**: `iroh-gossip` is 0.x and outside the 1.0 wire promise, but it
  is the most recently pushed protocol repo and Delta Chat runs it on hundreds of
  thousands of devices. Pin it; keep it behind the carrier port so the pull-based
  path still works without it.

Fit: the kernel stays topology-blind (Discovery Model §5: locality later arrives
as records and a smarter environment, "zero kernel change"); gossip is exactly
the "smarter environment". GDL-032 holds because the overlay carries Glade
records, not a second registry. This shape is the natural implementation of the
deferred "transitive gossip" item in the directory notes, and of the flat-mesh
v1 topology.

### 4.3 Shape C — protocol crates as the replicated substrate

**`iroh-docs` for records: rejected on semantics.**

| Glade record model (Substrate §2, Discovery Design §4) | `iroh-docs` | Consequence |
|---|---|---|
| Per-origin append log, `(origin, seq)` ids, `prev`-hash chain, equivocation provable | `(namespace, author, key)` → latest record; last-writer-wins on author timestamp; no chain | Loses replay, equivocation evidence and per-origin sequencing; GQ-9 cannot be expressed |
| Fold selected by shape (`log`, `value`, `swmr`, `crdt`…); time never enters the fold | One merge rule (LWW by timestamp), future-dated entries accepted up to 10 min | Violates the no-time-in-fold rule; one shape only |
| Owner-rooted, per-principal, attenuable, revocable grants; revocation wins | Write capability = possession of the namespace secret; read = namespace public key; per namespace only | A leaked secret is an unrevocable, unattributable writer; no attenuation, no per-verb or per-key scope (AZ-1 stays impossible) |
| Retention and compaction are contract decisions (R2-11 deferred, chain retained) | `delete_prefix` tombstones, GC via blobs tags | Different deletion semantics; no signed checkpoint concept |
| Cross-language parity (Rust/TS/Python corpus) | Rust crate, excluded from official bindings | Browser folds (GQ-4/GQ-8) impossible |

`iroh-smol-kv` shares the LWW-over-gossip shape with per-writer signatures; it is
closer to a presence/hint store than to Glade's records and adds nothing the
Glade fold does not already do. Neither crate helps with scope: a namespace is
the only boundary, and it is coarser than `(share, glade_id, key)` and finer
than nothing useful.

**`iroh-blobs` for bulk payloads: viable, gated.** Content-addressed, verified,
resumable transfer is genuinely absent from Glade and needed by file suppliers,
snapshot/checkpoint bootstrap (AZ-12 signed checkpoints; the deferred R2-11
compaction), large exchange results and invite bundles. Conditions:

- **Authorization is the provider event handler only.** Any peer may fetch any
  hash the store holds unless the handler rejects. Glade must map hash → owning
  share → `check()` against the requesting `remote_id()`'s node/principal binding,
  per request, re-evaluated when grants change. Store scoping (one `FsStore` per
  share, or tags per share) is a mitigation, not enforcement.
- **A new port.** The candidate architecture has no bulk-transfer port; it would
  be a concrete I/O adapter behind a contract with the same admission and
  budget semantics as the carrier, not a second delivery path that bypasses
  `Delivery`.
- **Status.** 0.x, "not production quality" per its own docs (possibly stale),
  memory-store only in wasm (irrelevant under GQ-2). Adopt only when a supplier
  slice needs it, with the 1.x pin and a real-I/O conformance tier.

### 4.4 Infrastructure and edge choices (orthogonal to A/B/C)

| Choice | Options | Note |
|---|---|---|
| Relays | Self-host `iroh-relay` (free, same binary n0 runs) · Iroh Services Pro ($19/mo, authenticated shared relays) · Dedicated | Authenticated relays admit only endpoints holding a signed token bound to their key: a *deployment-scope* admission, not a Glade grant. It can carry the operator relation of GDL-031 (only nodes of accepted operators get relay tokens) but must not become a second policy authority |
| Address lookup | Self-host `iroh-dns-server` · n0 community DNS (no SLA) · mDNS · mainline DHT | Records are signed by the node's endpoint key; poisoned hints degrade to DoS, never impersonation (Directory §7b) |
| Browser edge | Keep GQ-2 (WebSocket to a node) | Iroh's relay-only wasm mode and endpoint-only bindings offer no reason to reopen it |
| RPC | Keep taut frames | `irpc` is Rust-only 0.x; the wire corpus is the cross-language contract |

## 5. The scope aspects

"Scope" names five different things in the Glade corpus. Iroh's only relevant
construct in all five is acceptance by `(remote key, ALPN)`.

| # | Glade meaning | Defined in | Iroh construct | Fit |
|---|---|---|---|---|
| S1 | **Address / zone tuple** `(share, glade_id, key)`: domain, zone, surface; the replicated unit, the chain axis, the routing key | [GladeZones.md](../glade/dev-docs/GladeZones.md), Substrate §4, Discovery Design §4 (`StreamId`) | None. Nearest: one QUIC stream per tuple (already how forwarded interests work); ALPN per *protocol*, not per share | Glade-only. QUIC streams are cheap enough to keep the one-stream-per-zone mapping |
| S2 | **Dissemination / placement scope**: which nodes may receive or hold a share's records; `replica.hold` / `session.host` grants; INV-5; GDL-010 metadata exposure | [GladeAuthzModel.md](glade/GladeAuthzModel.md) §7/§7a, [GladeWorkspaceDirectory.md](glade/GladeWorkspaceDirectory.md) §7b, Problem Inventory §4 "dissemination scope" | `EndpointHooks` / handler rejection by key and ALPN; gossip topic membership; relay tokens | Partial: mechanical enforcement of *which peers connect*, nothing about *which shares they may hold*. This is the undecided item; §5.2 |
| S3 | **Authority scope**: `CapabilityGrant.scope` (`Execution{def_ref,compute_key}`, `Takeover{slot,supersedes}`), verbs, attenuation chains, source scope of a policy decision | Discovery Design §4, Authz §3/§6, `TrustPolicy` draft | None. `iroh-docs` capabilities are per namespace and non-attenuable; relay tokens are per endpoint | Glade-only, by design (A1: no second policy authority) |
| S4 | **Route/query scope**: the `Slot` `(share, glade_id?, key?)`, most-specific-match, `LocateRequest{namespace, key}` | Discovery Model §6, `ShardLocator` draft | None. Address lookup resolves keys to addresses, not slots to nodes | Glade-only |
| S5 | **Injection / execution scope**: node, account, binding, session, operation scopes; `NodeScope`, `WorkScopes` | [RuntimeAndAssurance.md](arch1/RuntimeAndAssurance.md), GDL-048 | Not applicable | The carrier is one provider inside `NodeScope` |

### 5.1 Consequence: scope is never delegated to iroh

Every shape in §4 leaves S1, S3, S4 and S5 untouched. Shape C would have
imposed a sixth, alien scope (the docs namespace) that matches none of the five;
that is the strongest single reason to reject it for records. The remaining
question is S2.

### 5.2 The undecided S2 decision: how dissemination scope is bounded

Three enforcement models are consistent with the corpus. They are not exclusive;
the decision is which one the first live slice relies on.

| Model | Rule | Iroh support | Glade cost | Trade-off |
|---|---|---|---|---|
| **T — topology-scoped** | A node may connect on the protocol carrying share X only if it is authorised for X | `EndpointHooks` reject per `(key, ALPN)`; one gossip instance/ALPN per share (or per placement class) | Per-share ALPN registry at runtime (`custom-router` pattern); the accept hook needs the current placement fold | One QUIC connection per peer per ALPN; churn on grant change; announcements never reach unauthorised nodes, so GDL-010 is answered by construction for node peers |
| **N — node-trust-scoped** (the corpus's current lean) | Any accepted node may receive what its *operator* is granted (`replica.hold`); the serve/fan-out hop enforces per-session grants; INV-5 checks a node carries `replica <share>` only when its operator holds it | Accept by key against the known-node set; everything else at Glade's serve seam | One accept policy; per-share checks in `mesh` before serving or forwarding a zone (the drop-in filter point the peer-sync notes named) | Shared gossip topics leak that share ids changed and their heads to nodes not granted that share; content does not flow without a grant. Simplest to build; matches AZ §7 "node↔node replication is governed by node trust" |
| **C — crypto-scoped** | Per-share payload encryption; nodes without keys route and store ciphertext (AZ-10 blind-relay tier) | Only transport E2E (irrelevant here: the concern is trusted-but-unauthorised *nodes*, not the wire) | Key distribution, rotation, and the forfeit of node-side folds/windows/services on that tier | The only model that bounds a compromised or over-trusted node; explicitly "named, not built" in AZ §7a |

Recommendation for the first slice (build entry: one namespace, fixed
authorised peers): **N**, because the fixed-peer configuration already is a
node-trust set, it needs no new ALPN machinery, and its leak is bounded to ids
and heads. **T** is the hardening step if Shape B is adopted and GDL-010 rules
that share ids and heads must not reach nodes outside a share's placement set.
**C** stays a tier decision (AZ-10). Whichever is chosen, the metadata table in
the security analysis (relay-only vs store-only peers) should list, per model,
exactly which of `TopicId`, share id, origin ids and head hashes a non-granted
node observes; that table is the deliverable GDL-010 needs, and iroh cannot
produce it.

Two things every model needs that do not exist yet:

1. **A transport-key binding record.** The directory identifies nodes by
   `sha256(node.key)`; iroh authenticates the endpoint key. Nothing replicated
   binds the two. Address lookup (Shape A), gossip peer identity (Shape B), blobs
   provider authorisation (Shape C) and accept-time policy (all models) all need
   "endpoint key K is transport for Glade node N", signed by N's chain. The
   discovery design already has `NodePrincipalBinding` in `KernelConfig`; the
   security analysis already names `NodeBinding` with an `iroh_node_signature`
   proof kind. This is a record kind and a fold rule, not an iroh feature.
2. **The accept-time view.** `EndpointHooks` run before any Glade frame; they
   need a read-only, revision-identified view of the current node/placement fold
   (A1/A4: evidence supplied, never fetched from inside the evaluator). That is
   an `Admission` consumer of a `Directory` projection, wired through
   `NodeScope`, and it must fail closed while the clock is `Uncertain`.

## 6. Coverage and the unaddressed requirements

Classes: **P** iroh provides the mechanism; **A** iroh supplies a building block
and Glade owns the semantics; **E** iroh touches only the transport edge of the
concern; **—** not addressed by anything in the ecosystem.

### 6.1 By captured concern (the 27 rows of the Gyld frame)

| Concern (sources) | Class | What iroh gives | What remains Glade's, or missing |
|---|---|---|---|
| FirstContact (GPI-01, GDL-008/013) | A | Tickets (address + payload), address lookup for re-rendezvous | Genesis ceremony, trust anchors, invitation grant, "invitations stay unavailable when every peer sleeps" (a home-node availability role, not a transport fact) |
| Reconnection (GPI-02, GDL-032, GLA-001/002/040/043) | A | Relay fallback, multipath migration, path/online watchers, keep-alive | Session resume, heads exchange, identity continuity of shares, "connectivity confers no authority" |
| TrafficBudgets (GPI-03, GLA-072) | E | Per-stream QUIC flow control; relay rate limiting (off by default) | Control/interactive/bulk priority, decode budgets, fan-out bounds, untrusted peer urgency; QUIC does not schedule across streams |
| DefinitionKnowledge (GPI-04, GDL-007/015/019/020/021) | — | | Versioned definitions, bindings, provenance, compatibility |
| DiscoveryKnowledge (GPI-05, GDL-032/037/043) | E | Node id → route only | Definition, grant, advertisement, coverage and route as separate facts; the fold; leases; `Matched/NoClaim` |
| MetadataVisibility (GPI-06, GDL-010) | E | Transport E2E encryption; relay sees ids/timing/sizes | Which nodes receive which records (§5.2); reconciliation; retention of revocations |
| IdentityIntegrity (GPI-07, GDL-033, B3/B5) | A | Proven transport key per connection (`remote_id()`) | Principal/device/session identity, signed ops, the transport-key binding record (§5.2) |
| PolicyFreshness (GPI-08, GDL-009/031/034, GLA-050/051/053) | — | | Grants, revocation, freshness at use, offline rules |
| ScopeIsolation (GPI-09, GDL-001/004/030/039, GLA-003/004/052) | — | | Canonical scope mapping, zone keys, no aliasing (§5, S1) |
| SourceAuthority (GPI-10, GDL-005, GLA-020..024) | — | | Epochs, fencing at the resource, takeover |
| CommandOutcome (GPI-11, GDL-006, GLA-035..038) | E | A reliable stream while both peers are online; `OutcomeUnknown` is the natural transport result of a dropped connection | Intent/attempt/correlation planes, Unknown outcomes, idempotency identity |
| CommitPublication (GPI-12) | — | | Source commit vs publication repair |
| AcceptanceMeaning (GPI-13, GDL-036/043/045) | — | (`Transport::send` returns local acceptance only, by Glade's own contract) | Durable-local, replicated and source-committed receipts |
| RecoveryProfiles (GPI-14, GDL-003/026/028/041, GLA-030..034/041) | — | | Shape-specific replay/repair/refresh, generations |
| Retention (GPI-15, GDL-012) | — | (blobs GC/tags are the wrong unit) | Chains, revocation evidence, ceilings, checkpoints |
| SourceReplicaSeparation (GPI-16, GLA-005/060..062, GDL-027) | — | | Source vs replica adapters |
| IndependentLifetimes (GPI-17, GDL-002/040, GLA-010..014/042) | — | | Interest, supplier, service and projection lifetimes |
| ConsumerMeaning (GPI-18, GDL-035, GLA-033) | — | | Glial assembly; browsers are not iroh peers (GQ-2) |
| FastFeedback (GDL-042, LBT) | — | Localhost QUIC already usable in adapter tests | Pure kernel/sim tiers must stay iroh-free (they do) |
| ScopedInjection (LBT-003/004/008/009) | — | | `NodeScope` provider sharing; the carrier is one injected provider |
| PreservedValidation (B2) | — | (TLS authenticates the peer; it validates no Glade bytes) | Validate every observation; reuse structural proof only |
| CommonSemanticBoundary (B1/B3, GDL-041) | — | | One binding/scope normalisation per seam |
| DeclarativeConfiguration (GDL-019/020/021/037) | E | Endpoint presets, relay maps and lookup services are configuration inputs | `NodeConfiguration` profile; no ambient n0 defaults (`presets::N0`) in a record-driven node |
| InspectableManagement (GDL-016/017/022/023/038, GLA-070/071) | E | Metrics, path events, `iroh-doctor` diagnostics | `node.status` as ordinary bindings; redaction; ownership/term/generation visibility |
| EcosystemConstraints (GDL-041, GLA-080..082) | A | A 1.x wire-stability and support policy; Ed25519-only keys; MSRV 1.91; protocol crates 0.x | Pin versions and licences; keep iroh behind the carrier port (GLA-082) |
| DemoPreservation (GDL-044/045/046/024/025/029) | — | | Build alongside the localhost demo |
| FutureGrowth (GDL-011/014/018, GAD-01/04/05) | E | Relays and lookup services scale independently of Glade; n0 hosts multi-region relays | Sharded registries, indexes, referrals, placement: entirely above iroh (GAD-05); the mainline DHT and `iroh-dht-experiment` do not index application records |

Count: P 0 · A 4 · E 9 · — 14.

### 6.2 By kernel requirement group (`GLA-*`)

| Group | Coverage | Unaddressed by iroh |
|---|---|---|
| Identity and container (001–005) | — | Share identity is a Glade record identity; an `EndpointId` is per node key and changes on rekey, so it can never be the share's public identity (GLA-002) |
| Interest model (010–014) | — | All |
| Ownership and fencing (020–024) | — | All; relay or registry leadership never fences a resource (GAD-05, Components table) |
| Projection contract (030–034) | — | All |
| Exchange semantics (035–038) | E | Planes, attempt identity, observation vs publication |
| Failure, resync, migration (040–044) | A (transport reconnect) | Rejoin from snapshot plus incremental state, registration reuse, identity preservation, generation rollover |
| Delegated capability model (050–053) | — | All; the only first-party capability model (`iroh-docs`) is per-namespace and non-attenuable |
| Source taxonomy (060–062) | — | All |
| Observability and cost (070–072) | E | Glade-state visibility; bounds on projection size, replay cost, subscription cardinality |
| Boundary (080–082) | A | GLA-082 is satisfied only while iroh stays behind the carrier port; adopting gossip or blobs adds two more adapters behind the same rule |

### 6.3 By responsibility (the 24 rows, with their arch1 owner)

| Responsibility → owner | Iroh role |
|---|---|
| `peer_connectivity` → Iroh carrier | **Lead mechanism**: dial, accept, relays, multipath, address lookup |
| `bootstrap` → Node assembly | Partner: tickets, address lookup for re-rendezvous |
| `client_sessions` → Sessions | Partner for peer sessions only (path events, close); WebSocket sessions untouched |
| `verify_identity` → Admission | Partner: proven transport key at accept; needs the binding record |
| `schedule_traffic` → Runtime | Partner at the edge: per-stream flow control, relay rate limits; no priority |
| `reconcile_metadata` → Records | Optional partner (Shape B): gossip as change notification |
| `locate_providers` → Directory | None beyond the node-discovery layer; `ShardLocator` and indexes are Glade's |
| `compose_runtime` → Node assembly | Configuration input: presets, relay map, lookup services, key custody |
| `expose_management` → System suppliers | Data source: metrics and path state into `node.status` |
| `verify_integration` → Conformance + harness | Real-I/O tier: localhost QUIC today; a relay container for relay-path journeys |
| The other 14 (`resolve_definitions`, `enforce_policy`, `canonicalize_scope`, `fence_source`, `invoke_source`, `repair_publication`, `retain_replica`, `resume_data`, `deliver_interest`, `assemble_consumer`, `attach_suppliers`, `validate_ingress`, `own_lifecycle`, `author_declarations`) | None |

### 6.4 Where existing Glade documents now disagree with the review

| Document | Statement | Review fact | Consequence |
|---|---|---|---|
| Discovery Model §5 | "v1 = flat (small p2p mesh + the public iroh relay)" | Free relays: rate limited, no SLA, "not suitable for production", metadata visible to n0 | v1 needs a self-hosted relay or the Pro tier; the *topology* ruling is unaffected |
| Directory §3 ladder step 4 | "resolve the user's published node set via iroh discovery (pkarr under known node keys)" | Works only for *endpoint* keys; Glade's known node keys are `node.key`, not the iroh key | Needs the transport-key binding record (§5.2) or one key for both roles |
| Peer-sync notes §1 | "`node_id = sha256(node key)`; the node key is the iroh endpoint's ed25519 public-key bytes" | Directory notes ambiguity 2 later made the HELLO identity derive from `node.key` with the iroh key transport-only | The two notes describe different bindings; the record above settles it either way |
| P2P-first topology | Written in libp2p vocabulary (`PeerId`, pubsub, delegated routing); GDL-011 still asks which libp2p mechanisms | GQ-2 selected iroh node-to-node | GDL-011 should be re-asked in iroh terms (Shape A vs B) or closed |
| Research findings (mid-2026) | iroh pre-1.0, 1.0 slipped | 1.0.0 shipped 2026-06-15; 1.2.0 current | Stale; browser and `iroh-docs` conclusions remain right |
| Carrier (`node/Cargo.lock`) | iroh 1.0.2 | 1.1.0 fixed a crafted-`EndpointAddr` panic | Bump before any address is parsed from a ticket or record |

## 7. Leveraging iroh: concrete attachments to the architecture

Nothing here is a phased plan; it names what each component takes from iroh so
the build entry's first slice ("a configured provider registers; another
participant discovers it … then a fixed-peer Iroh route") can be specified
without re-deriving it.

### 7.1 Iroh carrier adapter (`peer_connectivity`)

- Implements the draft `Transport::send` (local acceptance only, TR-001..004)
  and the duplex/session port Components.md lists as N, with explicit size,
  cancellation and ownership semantics.
- Endpoint from `presets::Minimal` plus explicit configuration (never `N0`):
  ALPNs, `RelayMode::Custom` from `NodeConfiguration`, lookup services from
  configuration, `EndpointHooks` bound to the accept-time view (§5.2), a
  `Router` so protocols are added and removed at runtime rather than a
  hand-driven accept loop.
- Stream discipline unchanged: HELLO on the first stream; then one stream per
  `(share, glade_id, key)` interest or sync round; separate lanes for bulk.
  Datagrams and 0-RTT are not used for anything that carries an effect.
- Version: 1.2.0; feature flags minimal (`metrics` on natively, no `unstable-*`).

### 7.2 Transport-key binding (Admission + Directory)

One record kind: `NodeTransportBinding{node, endpoint_id, valid_from, sig}` signed
under the node's chain, folded set-union with the usual revocation-wins rule, and
read by: `EndpointHooks` (accept only known bindings when policy requires),
HELLO verification (the presented `node_id` must be bound to the connection's
`remote_id()`), address lookup (publish and resolve the *bound* endpoint key),
and any future blobs/gossip peer check. Until it exists, the only honest binding
is the CLI `--peer <endpoint-id>@<ip:port>` flag.

### 7.3 Bootstrap and node discovery (`bootstrap`, `locate_providers` partner)

- `GladeInviteTicket: iroh_tickets::Ticket` = endpoint addresses + home share id
  + the signed device-cert grant; parsing is untrusted input through Admission.
- Address lookup: self-hosted `iroh-dns-server` as the private pkarr/DNS origin
  (or n0's for development only), mDNS on LANs. `NodeHint` records are retired
  in favour of it once bindings exist; the kernel never sees addresses (Discovery
  Model §2).

### 7.4 Directory and Records (`reconcile_metadata`, Shape B if adopted)

- Gossip topic per share; announcements are heads or exact accepted bytes,
  emitted only from the host's post-commit `Gossip` effect; repair stays the
  Glade sync round with `SyncEnd` and retries.
- Scope model from §5.2 decided first; per-share ALPN instances only if model T
  is chosen.

### 7.5 Bulk transfer (new port; `deliver_interest` / suppliers)

A `BulkTransferPort` implemented by `iroh-blobs`, admitted per request through
`check()` on the requester's bound node, budgeted by Runtime, and used only where
a supplier declares a bulk profile. Not a second delivery fold.

### 7.6 Deployment and operations (`compose_runtime`, `expose_management`)

- Relay and DNS: self-host both binaries (Docker images exist) for the first
  real-route slice; authenticated relays admit only nodes of accepted operators
  (a deployment control, mirrored from, never replacing, `replica.hold`).
- Metrics and path events feed `node.status` through ordinary bindings
  (GDL-038); nothing in Glade reads iroh types outside the adapter.

### 7.7 Assurance

Pure kernel and simulator tiers stay iroh-free (INV-D0 needs it). Adapter
conformance runs against real localhost QUIC (exists) and, once relays are
configured, against a local `iroh-relay` container for the relay-path and
migration journeys. The `JoinUnderLoad` journey (a device joins during a bulk
transfer while a provider disappears and a grant revokes) is the first place
iroh's multipath and relay fallback are exercised against Glade's budgets.

## 8. Decisions for the owner

| # | Decision | Options | Analysis lean |
|---|---|---|---|
| I-1 | Mapping shape for the first real-route slice | A · B · C | **A now**; B as the dissemination overlay once the scope model is chosen; C rejected for records, blobs gated |
| I-2 | Dissemination scope enforcement (S2) | T · N · C | **N for the fixed-peer slice**; T if GDL-010 forbids id/head exposure to non-granted nodes; C as the AZ-10 tier |
| I-3 | Relay and DNS posture | self-host · Iroh Services Pro/Dedicated · n0 community (dev only) | **Self-host** for the slice; revise Discovery Model §5's "public iroh relay" |
| I-4 | Transport-key binding record | new record kind under the node chain · one key serves both roles | **Record kind**; keeps the ratified identity split and lets keys rotate independently |
| I-5 | Version policy | bump to 1.2.0 now; pin 0.x protocol crates individually | **Bump now**; adopt no 0.x crate without a port and a pin |
| I-6 | `iroh-gossip` adoption gate | before/after the fixed-peer slice | After: the slice's "fixed-peer Iroh route" needs only Shape A |
| I-7 | `iroh-blobs` adoption gate | on first bulk supplier · never · now | On first bulk supplier, behind the new port |
| I-8 | GDL-011 wording | re-ask in iroh terms · close as superseded by GQ-2 | Re-ask as I-1/I-2 |

## 9. Evidence and limits

Read for this analysis: the iroh review; `GladeRequirements.md`;
`GladeProblemInventory.md`; `GladeArchitectureDiscussion.md`; `DecisionLog.md`;
`GladeBuildEntry.md`; `arch1/{GladeArchitecture,Components,RuntimeAndAssurance,ReadingMap}.md`;
`glade/{GladeDiscoveryModel,GladeDiscoveryDesign,GladeWorkspaceDirectory,GladeAuthzModel,GladeP2PFirstTopology,GladeBootstrapModel,GladeDeclSurface}.md`;
`glade/dev-docs/{GladeZones,GladeDirectoryNotes,GladePeerSyncNotes,GladeSubstrateV1,IrohReview}.md`;
the `GLResearchConsolidatedFindings.md` iroh sections; the captured Gyld frame;
the draft `transport-api`, `placement-api` and the trait inventory of the other
draft contract crates; the public items of `node/src/iroh_carrier.rs` and
`node/src/mesh.rs`. No code was run or changed, no crate was fetched, and no
iroh behaviour was tested. Statements about iroh are the review's, as of
2026-09-12; two points it leaves unverified are flagged above (per-topic gossip
admission; the blobs production-quality note). Nothing here is ratified, and no
decision-log row was added.

## 10. Evaluator run and requirement relaxations

Added 2026-09-12 after the owner asked to apply Gyld's evaluator to this report.
The run and the method position are recorded on the Gyld side in
[IrohIntegrationEvaluation.md](/Users/owebeeone/limbo/gyld-wz/dev-docs/case-studies/glade/IrohIntegrationEvaluation.md);
its outputs are in `gyld/artifacts/iroh-integration-v1/`. This section keeps the
Glade-facing consequences: what the gates said, and which Glade requirements a
complete component would need relaxed, changed or merely corrected.

### 10.1 What the evaluator said

The four proposals were expressed as saved operations over candidate 1 revision 3
under a mutation frame, with documentary gates fed by the review. No proposal is
eligible and there is no winner, because no runtime witness exists for any of
them. The documentary gates separated the proposals anyway:

| Proposal | Documented outcome |
|---|---|
| Shape A, carrier | The current lock **fails** the untrusted-address-fix gate (1.0.2 against the 1.1.0 fix); the hardened carrier passes it and the production-relay-terms gate; only 1.x crates, so the stability gate holds |
| Shape B, gossip | **Fails** the 1.x wire-stability gate by policy (0.x crate); admissible only under a pin decision that keeps the obligation visible and ungated |
| Shape C, blobs | Same policy failure; the same pin decision, behind a new bulk-transfer port |
| Shape C, docs as record substrate | **Fails** three documented semantic gates (revocable capability, time-free fold, chain evidence) while every obligation stays retained; a relaxed frame that removes five obligations clears the failures and still grants nothing |

### 10.2 Relaxation candidates, by component

Three kinds of change would admit already-complete components. Only the third
kind is a true relaxation; the first two are corrections and policy decisions.

| Component | Collides with | Change that admits it | Kind | What is lost | Lean |
|---|---|---|---|---|---|
| iroh core as carrier | Nothing ratified. Directory §3 step 4 and Peer-sync notes assume the node key is usable for iroh lookup, while Directory notes made the iroh key transport-only | Add the transport-key binding record (§5.2); alternatively **relax the identity split** and let the iroh `EndpointId` be the node id | correction, or relaxation of the P2P-first identity split | With the relaxation: no independent rotation of transport and node keys; Ed25519 fixed for the node principal; one compromise takes both roles | Keep the split, add the record |
| iroh version 1.0.2 in the lock | The documented 1.1.0 security fix | Bump to 1.2.0 | correction | Nothing | Do it |
| Public relays | Discovery Model §5 "public iroh relay" versus n0's production terms | Self-host `iroh-relay` (or paid tier); accept that a relay operator sees ids, timing and sizes, as the security analysis's relay-only row already grants | correction | Zero-infrastructure default weakens: someone runs a relay or pays for one | Self-host for the slice |
| `iroh-gossip` overlay | EcosystemConstraints: "pin capability/version assumptions" against a 0.x crate; Discovery Design `MAX_RECORD` 16 KiB against a 4 KiB default; GDL-010 metadata exposure under model N | Pin the crate behind the carrier port with its own conformance tier; raise the message size; decide the S2 scope model | policy decision | Wire changes at any 0.x minor are Glade's to absorb; under model N, non-granted trusted nodes learn share ids and heads | Acceptable after I-2 |
| `iroh-blobs` bulk transfer | Same 0.x policy; arch1 A4 "fresh observations validate at their boundary" against a provider-side event handler as the only authorization point; no bulk port in the architecture | Pin; accept the provider handler as the admission seam for blob requests (hash → share → `check()` per peer); add the port | policy decision plus one new contract | A second admission seam to keep conformant; "not production quality" caveat carried | Only when a supplier needs bulk transfer |
| `iroh-docs` as records | GQ-9 per-origin chains (Substrate §2, "MUST carry from day one"); no-time-in-fold (Directory §2, Discovery Design §2); owner-rooted revocable grants (Authz §3/§4, GDL-009/GDL-034); Rust/TS/Python fold parity (GQ-4/GQ-8); retention as a contract decision (R2-11) | Drop all five | relaxation of ratified and frozen rulings | Replay and equivocation evidence, revocation, browser folds, deterministic time-free convergence, and the ability to attenuate a writer | Reject; the list is the proof |
| `iroh-smol-kv` | Same last-writer-wins shape | Would only carry ephemeral presence-class data, which Substrate §3 already leaves unreplicated (`stream`) | none needed | Nothing gained | Not needed |
| pkarr/DNS address lookup | Needs the endpoint key recorded somewhere replicated | The binding record above | correction | Nothing | Do it with the record |
| `iroh-tickets` for invites | Nothing; the ticket payload is untrusted input for Admission | None | none | Nothing | Adopt |
| `EndpointHooks` as admission | A4 and "checked once upstream is not trusted everywhere" | None if hooks stay a pre-filter; **relaxing** A4 to trust the hook decision would remove per-frame admission | none, or a relaxation to reject | Under the relaxation: HELLO and frame validation lose their independent check | Pre-filter only |
| 0-RTT connects | CommandOutcome / idempotency rules | Use only for effect-free frames | none | Nothing | Do not use for effects |
| Browser bindings / wasm | GQ-2 | None | none | Nothing | Unchanged |
| GAD-04 "one Iroh-based substrate" with sharded registries and indexes | Iroh has no application-record registry, DHT or index | Read GAD-04's substrate as the carrier only; the registry, shards and indexes are Glade's (GAD-05) | correction of an expectation | Nothing in Glade; the ambition stays, unhelped | Record it |

### 10.3 The rule this yields

A complete component earns its way in through corrections and pin decisions,
which the evaluator can show as gate outcomes and futures; it never earns its way
in through semantic surrender, which the evaluator can only display as dropped
obligations for the owner to refuse or accept. For this ecosystem that puts the
core, relays, DNS and tickets on the "adopt with corrections" side, gossip and
blobs on the "pin behind a port when needed" side, and docs on the "reject" side.
