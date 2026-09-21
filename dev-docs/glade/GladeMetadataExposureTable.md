# Glade Metadata Exposure — what an ungranted observer learns

Status: **DRAFT for the owner's review**, 2026-09-21, written against glade
commit `4467696`. **Nothing here is a ruling.** It is evidence assembled so a
ruling can be made.

Update 2026-09-21: §7's four candidates are now `metadata_exposure`'s
alternatives and `relay_posture` is an ordinary question, in gyld `d544824`.

Purpose: produce the table `IrohGladeMapping.md` §5.2 names as the deliverable
GDL-010 needs — "exactly which of `TopicId`, share id, origin ids and head
hashes a non-granted node observes", per scope model — and answer
`GladeWorkspaceDirectory.md` WD-3, "what the home node sees when it is someone
else's infra". The configuration is the one the owner's eight rulings of
2026-09-21 fixed: `scope_model = node_trust`, `dissemination = sync_round_only`,
`transport_key_binding = binding_record`, `proof_family = taut_grants`,
`identity_adapters = none_in_v1`, `key_custody = recovery_keys`, iroh 1.2 floor.

The question this feeds, `metadata_exposure` (matrix R8), says content does not
flow without a grant either way. **That premise is not true of the code today.**
Section 6 is where that lands; the table carries the row so the reader is not
misled by its absence.

---

## 1. Observer classes

Seven, each one sentence. The first three sit outside the node's trust
boundary; the last four are inside it.

| # | Class | Definition |
| --- | --- | --- |
| **O1** | Path observer | Anything that can see the packets between two nodes — today only a privileged process capturing loopback, because the endpoint binds `127.0.0.1` and nothing else. |
| **O2** | Relay operator | Whoever runs an iroh relay that forwards a connection when hole-punching fails; not in the picture today, because relays are off. |
| **O3** | Address-lookup service | A pkarr publisher or resolver, a DNS origin, a DHT participant or an mDNS listener that maps an endpoint id to an address; not in the picture today, because lookup is off and peers are named on the command line. |
| **O4** | Accepted peer, no grant for X | A node that completed the node↔node HELLO and holds a live link, but whose operator holds no grant for share X. |
| **O5** | Home node as someone else's infrastructure | The durable-replica-plus-rendezvous role (WD §3), held by a box whose operator is not the share owner — a special case of O4 with uptime and a disk. |
| **O6** | Client session, no grant for X | A websocket session on a node, bound to a principal that holds no grant for X. |
| **O7** | Local process | **A class the code adds that the sources do not name:** any process on the node's machine that can open the websocket port, because that carrier is a plaintext HTTP/1.1 upgrade with no authentication of any kind. O7 is O6 without even a claimed principal. |

O5 is worth separating from O4 because WD-3 asks about it by name, and because
the home node is the one role that is *designed* to hold a full home-share
replica for someone else (WD §7b lines 248-255: `replica.hold` is granted to the
operator at signup).

---

## 2. The three marks, and what each cell says

Every cell carries one of three evidence marks. This discipline is the point of
the exercise: a cell filled from general knowledge about QUIC or peer-to-peer
systems, with no citation, is worse than an empty cell, because it cannot be
checked.

| Mark | Meaning |
| --- | --- |
| **`C-n`** | **VERIFIED IN CODE.** A file and line in this workspace. Listed in §3.3. |
| **`D-n`** | **FROM THE DOCS.** A document and section. Listed in §3.4. Nothing here was run or read in the code. |
| **`U-n`** | **UNKNOWN.** Neither settled. Listed in §8 with the cheapest test that would settle it. |

And each cell says *when*:

| Value | Meaning |
| --- | --- |
| **TODAY** | Visible as the node runs now: `presets::Minimal`, relay and address lookup off, localhost dial only. |
| **ROUTE** | Not visible now; visible at the first real route, when a relay, an address-lookup service, or a peer link crossing a network appears. |
| **no** | Not visible to this observer in either case. |
| **n/a** | The row does not exist under the rulings. |

---

## 3. The table (scope model N — node trust, as ruled)

### 3.1 Observers outside the node

Today there is no network path at all: the endpoint binds `127.0.0.1:0` with
`presets::Minimal`, which sets only a crypto provider — no relay, no address
lookup `[C1][C2][D3]`. So every O1 cell that says TODAY means "a privileged
process capturing loopback on the node's own machine", and O2 and O3 do not
exist yet `[D8]`.

| Row | O1 path observer | O2 relay operator | O3 address lookup |
| --- | --- | --- | --- |
| iroh endpoint id | ROUTE `[C1][C2][D2]` | ROUTE `[D1][D2]` | ROUTE `[D5]` |
| Glade node id (`sha256(node.key)`) | no `[C3][C5]` — it rides inside the encrypted stream | no `[D1]` | no `[C16][D5]` |
| principal ids | no `[D1]` | no `[D1]` | no `[D5]` |
| workspace ids | no `[D1]` | no `[D1]` | no `[D5]` |
| share ids | no `[D1]` | no `[D1]` | no `[D5]` |
| glade ids (`gyld.streams`, `dir.grants`) | no `[D1]` | no `[D1]` | no `[D5]` |
| record keys (conversation ids, stream names) | no `[D1]` | no `[D1]` | no `[D5]` |
| origin ids (`glade-gyld:ws-razel:gyld.ops`) | no `[D1]` | no `[D1]` | no `[D5]` |
| head hashes | no `[D1]` | no `[D1]` | no `[D5]` |
| lamport / seq values (activity volume) | ROUTE `[U2]` | ROUTE `[D1]` — "how much" | no `[D5]` |
| record sizes and timing | ROUTE `[U2]` | ROUTE `[D1]` | no `[D5]` |
| serve / provider claims, directory records | no `[D1]` | no `[D1]` | no `[D5]` |
| declaration-envelope fields | no `[D1]` | no `[D1]` | no `[D5]` |
| ALPN string `glade/node/1` | ROUTE `[U1]` | ROUTE `[U1]` | no `[D5]` |
| gossip `TopicId` | n/a `[C24]` | n/a `[C24]` | n/a `[C24]` |
| **op payloads (content)** | no `[D1]` | no `[D1]` | no `[D5]` |

Two things to read off this half of the table. First, iroh's transport
encryption does its job: a relay is told the endpoint ids, the timing, the
volume and both IP addresses, and nothing else `[D1]`. Second, the endpoint id
*is* the long-term public key and therefore a stable identity `[D2]` — turning on
address lookup publishes it, and publishing is signed by the endpoint key
itself `[D5]`, so the record is self-authenticating to anyone who holds it.

### 3.2 Observers inside the trust boundary

These are the cells that matter, because node trust puts every one of them
inside. "For home" below means the home share, which is the directory.

| Row | O4 accepted peer, no grant for X | O5 home node as someone else's infra | O6 client session, no grant for X | O7 local process |
| --- | --- | --- | --- | --- |
| iroh endpoint id | TODAY `[C1][C16]` | TODAY `[C1]` | no `[C13]` | no `[C13]` |
| Glade node id | TODAY `[C3]` | TODAY `[C3]` | TODAY `[C12]` via `dir.nodes` | TODAY `[C12][C13]` |
| principal ids | TODAY `[C5][C7][C9]` via `dir.principals`, `dir.grants` | TODAY `[C9][D17][D19]` | TODAY `[C12][C26]` | TODAY `[C12][C13]` |
| workspace ids | TODAY `[C5][C7][C9]` via `dir.workspaces` | TODAY `[C9][D17]` | TODAY `[C12]` | TODAY `[C12][C13]` |
| share ids | TODAY `[C5][C7][C9]` via `dir.workspaces`, `dir.claims`, `dir.grants` | TODAY `[C9][D17]` | TODAY `[C12]` | TODAY `[C12][C13]` |
| glade ids | TODAY `[C8][C9][C21]` via `dir.bindings`, `dir.services` | TODAY `[C21][D17]` | TODAY `[C12][C26]` | TODAY `[C12][C13]` |
| record keys | TODAY for X `[C10][C15]` — subscribe X and the keys come with it | TODAY `[C10][C15]` | TODAY `[C12][C15]` | TODAY `[C12][C15]` |
| origin ids | TODAY `[C6][C14]` — home at connect, X on subscribe | TODAY `[C6][C14]` | TODAY `[C12][C14]` | TODAY `[C12][C14]` |
| head hashes | TODAY for home via the head vector `[C5][C6]`; for X the ack carries origins and seqs only, `hash: None` `[C10]`, but every delivered op carries its predecessor's hash in `prev` `[C6a]` | TODAY for home `[C5][C6]`, and via `prev` on anything it stores `[C6a]` | TODAY via `Op.prev` on delivered ops `[C6a]`; the ack itself sets `hash: None` `[C12]` | TODAY via `Op.prev` `[C6a]` |
| lamport / seq values | TODAY `[C6][C10]` | TODAY `[C6]` | TODAY `[C12]` | TODAY `[C12]` |
| record sizes and timing | TODAY `[C7][C10]` | TODAY `[C7]` | TODAY `[C12]` | TODAY `[C12]` |
| serve / provider claims | TODAY `[C9]` via `dir.claims` — node, share, lease, epoch | TODAY `[C9][D17]` | TODAY `[C12]` | TODAY `[C12]` |
| declaration-envelope fields | TODAY `[C9][C21]` — app, glade id, shape, authority, zone, retention | TODAY `[C21]` | TODAY `[C26]` | TODAY `[C12]` |
| ALPN string | TODAY `[C1]` — it dialled with it | TODAY `[C1]` | no `[C13]` | no `[C13]` |
| gossip `TopicId` | n/a `[C24]` | n/a `[C24]` | n/a `[C24]` | n/a `[C24]` |
| **op payloads (content)** | **TODAY `[C10][C11][C20]`** — see §6.1 | **TODAY `[C10][C11]`** | **TODAY `[C12]`** | **TODAY `[C12]`** |

**Why `TopicId` is `n/a`.** The gossip row is empty by ruling, not by accident.
`dissemination = sync_round_only` takes no gossip overlay, and the node has no
gossip dependency to take one with: its whole manifest is `glade-wire`, `sha2`,
`tokio` and `iroh` `[C24]`. With no overlay there is no shared topic, so the
specific leak §5.2 warns about under model N — "shared gossip topics leak that
share ids changed and their heads to nodes not granted that share" `[D20]` —
does not exist. Everything an ungranted peer learns today, it learns from the
directory it is handed at connect, not from a topic.

**What the HELLO carries.** `NodeHello` and `NodeWelcome` carry exactly three
fields: the 32-byte claimed `node_id`, a protocol integer, and an optional
signature `[C3]`. No share, no workspace, no capability. The signature is a
domain-separated digest, and `verify_peer` accepts unconditionally `[C4]`.

**What a sync round offers a peer, and filtered by what.** Two different
answers, and the difference matters:

- The **live path** (`mesh.rs`, what a running node does) is filtered — to the
  **home share and nothing else**. `pull_home` announces only home heads and
  `serve_home` serves only home ops `[C5][C7]`. This is a *scope* filter, not an
  authorization filter: it is there because the directory replicates everywhere
  by design `[D18]`, not because anyone checked a grant.
- The **library path** (`peer.rs::serve_sync`) offers **every zone the store
  holds**, with no filter at all `[C7a]`.

**Where the per-share filter point is, and whether it exists.** The docs name a
drop-in filter point, and the code names it too, in a comment: "ACL
zone-filtering (a peer withholding an entire private-zone chain) is a drop-in
here: filter `store.zones()`" `[C7a]`. **It is a comment. No filter exists.**
`grants_for` is implemented and correct, and its only callers in the whole
repository are tests `[C11]`. Nothing on the peer path, the client path or the
exchange path consults a grant before serving.

### 3.3 Evidence key — VERIFIED IN CODE

| Tag | Where | What it establishes |
| --- | --- | --- |
| `C1` | `glade/node/src/iroh_carrier.rs:24,32-40` | One ALPN, `glade/node/1`, for all shares; `Endpoint::builder(presets::Minimal)` bound to `(Ipv4Addr::LOCALHOST, 0)`. |
| `C2` | `~/.cargo/registry/src/index.crates.io-*/iroh-1.2.0/src/endpoint/presets.rs:59-79` and `src/endpoint.rs:192` | `Minimal::apply` sets only the rustls crypto provider; the builder starts "with no address lookup services, and `RelayMode::Disabled`". |
| `C3` | `glade/node/src/peer.rs:110,128`; `glade/wire-rs/src/generated.rs:285-330` | `NodeHello`/`NodeWelcome` = `{node_id, protocol, sig}`. Nothing else is on the handshake. |
| `C4` | `glade/node/src/peer.rs:76-81,99-101` | `verify_peer` returns `Ok` for any 32-byte id; `stub_sig` is a digest, not a signature. |
| `C5` | `glade/node/src/mesh.rs:405-410` | `pull_home` sends `store.all_heads()` filtered to `share == HOME`. |
| `C6` | `glade/node/src/store.rs:215-230`; `glade/wire-rs/src/generated.rs:140-188` | `all_heads` emits `StreamHeads{share, glade_id, key, heads}`, each `Head{origin, seq, hash: Some(op_hash)}`. |
| `C6a` | `glade/wire-rs/src/generated.rs:189-232`; `glade/node/src/chain.rs:11` | Every `Op` carries `prev: Option<Vec<u8>>`, the hash of its predecessor in the chain. Anyone who receives ops receives chain hashes, whether or not the head vector carried any. |
| `C7` | `glade/node/src/mesh.rs:371-397` | `serve_home` ships every home-zone op the peer lacks; the only test is `share != HOME { continue }`. |
| `C7a` | `glade/node/src/peer.rs:172-174,175-203` | `serve_sync` iterates `store.zones()` unfiltered; the named ACL filter point is a comment. |
| `C8` | `glade/node/src/registry.rs:37,42-52` | Home-share glade ids: `dir.nodes`, `dir.workspaces`, `dir.claims`, `dir.grants`, `dir.revocations`, `dir.bindings`, `dir.services`, `dir.principals`. |
| `C9` | `glade/node/src/sysdata.rs:6-10,26-31,49-55,75-80,98-102,118-126,150-155,173-176` | Record shapes, all plaintext strings: `NodeRecord{node_id, operator}`, `WorkspaceEntry{workspace, name, eligible_hosts}`, `ServeClaim{node, share, lease_expiry_ms, epoch}`, `CapabilityGrant{principal, share, verbs}`, `CapabilityRevocation{principal, share}`, `BindingDecl{app, glade_id, shape, authority, zone, retention}`, `ServiceDefinition{app, name, glade_id}`, `PrincipalRecord{principal}`. |
| `C10` | `glade/node/src/mesh.rs:255-308` | `serve_peer_subscribe` registers any peer's `Subscribe` as an ordinary subscriber, acks with the zone's heads (`hash: None`), ships the gap, then feeds it live. No grant check. |
| `C11` | `glade/node/src/registry.rs:205,366-381` + repository grep | `grants_for(principal, share)` exists and folds revocation-wins; every caller is a test. |
| `C12` | `glade/node/src/server.rs:171-178,181-240` | A websocket `Subscribe` is answered from the local replica with no authorization; a `Hello` binds a principal on the client's word, "nothing enforced"; the ack sets `hash: None`. |
| `C13` | `glade/node/src/ws.rs:93-94,113`; `glade/node/src/bin/glade-node.rs:138-140` | The client carrier is a plaintext HTTP/1.1 websocket upgrade, no TLS, no auth, bound to `127.0.0.1`. |
| `C14` | `glade-gyld/src/supplier.rs:212` | `format!("glade-gyld:{}:{}", config.share, config.glade_id)` — the literal `glade-gyld:ws-razel:gyld.ops`. |
| `C15` | `glade-chat/src/manifest.ts:31-33,74` | `groupKey(id)` is the UTF-8 of the conversation id; the record key is the name. |
| `C16` | `glade/node/src/bin/glade-node.rs:41-46` | Peers arrive as `--peer <endpoint-id-hex>@<ip:port>` on the command line. There is no lookup path. |
| `C17` | repository grep over `glade/` for `EndpointHooks`, `accept_hook`, `allowlist` | No match. No accept-time policy exists. |
| `C18` | repository grep over `glade/` for `NodeBinding`, `iroh_node_signature`, `endpoint_key` | No match. The binding record the ruling names is not built. |
| `C19` | `glade/node/src/mesh.rs:233-249`; `glade/node/src/claims.rs:266-275` | `push_home` pushes self-minted directory ops to every live link, unconditionally. |
| `C20` | `glade/node/src/exchange.rs:232-263` | `serve_peer_exchange` answers a peer's forwarded exchange with no grant check. |
| `C21` | `glade/node/src/appdecl.rs:1-27,43-47` | An `<app>.glade` file registers `BindingDecl` + `ServiceDefinition` records and compiles its ACL seeds to `CapabilityGrant` records, all into the home share. |
| `C22` | `glade/node/Cargo.toml:24` | `iroh = "1.2"` — the version-pin ruling is in the manifest. |
| `C23` | `glade/node/src/mesh.rs:334-346` | `run_forward` sends `Subscribe{share, glade_id, key, from}` carrying our heads to the claim holder, so the holder learns what we are interested in and how far along we are. |
| `C24` | `glade/node/Cargo.toml:15-24` | Dependencies are `glade-wire`, `sha2`, `tokio`, `iroh`. No `iroh-gossip`, no `iroh-docs`, no `iroh-blobs`. |
| `C25` | `glade/node/src/mesh.rs:214-222` | Inbound `Ops` on any peer stream are ingested when `op.share == HOME` — a peer can write directory records into our replica. |
| `C26` | `glade/node/src/mesh.rs:520-584`; `glade/node/src/exchange.rs:599,655-687` | Existing tests demonstrate the leak end to end: A holds B's `WorkspaceEntry` and `ServeClaim` after one connect; a client reads `dir.bindings` and `dir.grants` as ordinary records. |
| `C27` | `glade/node/src/store.rs:311,340` | On disk, share and origin are hex-encoded into path components — reversible, but not plaintext to a naive file indexer. |

### 3.4 Evidence key — FROM THE DOCS

Read the status lines first. `GladeAuthzModel.md` is a "working draft — design
direction, not yet a contract" except for the ratified rulings B4 (§4a), B5
(§3b) and D3/D5/H-R3 (§11); **§7 and §7a are not ratified.** The security
analysis "makes no implementation proposal beyond the V1 seams".
`GladeWorkspaceDirectory.md` is likewise a working draft.

| Tag | Where | What it says |
| --- | --- | --- |
| `D1` | `glade/dev-docs/IrohReview.md` §3.2, lines 114-115 | Relay traffic is end-to-end encrypted; the relay sees "only metadata (which ids talk, when, how much, source/destination IPs)". |
| `D2` | `IrohReview.md` §3.1, lines 91-92 | The public key *is* the address, so observing an address observes a stable long-term identity. |
| `D3` | `IrohReview.md` §3.4, lines 135-136 | `presets::Minimal` means "no relays, no address lookup", marked "This is what glade uses." |
| `D4` | `IrohReview.md` §3.6, lines 171-175 | If hole-punching fails iroh falls back to the relay automatically; each endpoint keeps a home relay. Relay exposure is not opt-in per connection. |
| `D5` | `IrohReview.md` §3.7 lines 184-193, §10 line 562 | Published address records are signed by the endpoint key; n0 hosts `dns.iroh.link`; mDNS and DHT are separate crates that cannot be on unless added. |
| `D6` | `IrohReview.md` §3.3, lines 118-124 | iroh has no ACL, capability or permission system for peers; `EndpointHooks` can only observe or reject, at `after_handshake`. |
| `D7` | `IrohReview.md` §5.2, lines 336-338 | A gossip `TopicId` is 32 bytes and the guidance is to derive it "by hashing a meaningful string". |
| `D8` | `IrohReview.md` §11, lines 572-578 | Glade today runs `Minimal`, direct localhost dial, ALPN `glade/node/1`, hand-driven accept; no `Router`. |
| `D9` | `IrohReview.md` §7, lines 466-467 | n0 advises against public relays for sensitive data, because connection metadata is visible to the relay operator. |
| `D10` | `GladeGrythSecurityModelAnalysis.md` §Metadata Exposure, lines 504-531 | The only existing metadata table: `Relay-only` / `Store-only` / `Router` / `Authorized follower`, against one column, "Legitimately sees". Its store-only row assumes "workspace/share **opaque** ids". Closes with "GDL-010 should remain open until the exact redaction/blinding rules are chosen." |
| `D11` | same, lines 518-520 | The design "SHOULD treat human-readable workspace names, repo paths, binding names, operation names, and principal display names as sensitive metadata". |
| `D12` | `GladeAuthzModel.md` §7, lines 230-231 | "Node↔node replication is governed by node trust (device certs), not user grants." |
| `D13` | same §7, lines 233-237 | "A compromised node leaks what it legitimately replicated. Enforcement guards sessions, not stolen disks." Mitigation order: replicate only to nodes with at least one granted user, then per-share payload encryption. |
| `D14` | same §7a, lines 251-252 | A node is an acceptable home for a share's plaintext "iff the share's policy accepts the node's operator". |
| `D15` | same §7a, lines 265-267 | Three tiers: operator-accepted plaintext, blind relay / E2E, and confidential compute — the third "named, not built". |
| `D16` | same §7a, lines 280-283 | An untrusted node in the path "can withhold (DoS) or observe what it stores — never forge". Operator trust governs confidentiality and placement only. |
| `D17` | `GladeWorkspaceDirectory.md` §2, lines 54-60 | The home share's record kinds include `PrincipalDecl`, `CapabilityGrant`/`CapabilityRevocation`, `WorkspaceEntry` (workspace id, human name, eligible hosts), `ServeClaim`, `NodeHint`. |
| `D18` | same §3, lines 94-96 | "Every device is a destination of the home share; the list is readable offline before any network." |
| `D19` | same §3, lines 108-113 | The home node is "availability, not authority": durable replica plus relay plus ticket target, holding no special keys. |
| `D20` | `IrohGladeMapping.md` §5.2, lines 250-254 | The three models T / N / C, their iroh support, Glade cost and trade-off — the source this table is built against. |
| `D21` | same §7.2, lines 389-395 | `NodeTransportBinding{node, endpoint_id, valid_from, sig}`. "Until it exists, the only honest binding is the CLI `--peer <endpoint-id>@<ip:port>` flag." |
| `D22` | same §4.4, lines 220-221 | Relay options (self-hosted, Iroh Services Pro, dedicated) and address-lookup options (self-hosted `iroh-dns-server`, n0 community DNS, mDNS, mainline DHT). |
| `D23` | `GladeAuthzModel.md` §10, lines 357-359, 366-368 | INV-4 gates serving `OPS` to **client/service sessions** in stage-2 traces; INV-5 gates a node *carrying* replica state. Neither covers node↔node links or heads. |
| `D24` | same §9, line 339 (AZ-10) and line 333 (AZ-4) | AZ-10: blind-relay semantics and "what survives of resume/heads" are open. AZ-4: "Directory (home share) visibility to guest principals — what does a grantee of one workspace see of the list?" is open. |
| `D25` | same §4a, lines 169-182 (ratified B4) | Grants gate commons-zone joins; private zones need no grant; the per-zone contiguous chain "is what makes private zones filterable from what a peer receives without breaking chain verification". |
| `D26` | `GladeGrythSecurityModelAnalysis.md` §V1 Seam Audit, lines 580-591 | `HELLO` MUST carry a node/transport key binding slot; `SUBSCRIBE`, `APPEND`, `HEADS`, `EXCHANGE`, `CHANNEL` MUST carry `principal_id`, `capability_ref`, `policy_ref` slots; the "Metadata boundary" seam is ranked High. |
| `D27` | same §What Is Not Defensible, line 666 | Not defended: "privacy of timing/size/route metadata unless GDL-010 mitigations are applied". |
| `D28` | same §Principal Model, lines 203-204, and §Zero-Infra Default Design, lines 622, 635 | The iroh node id MUST NOT be an authorization identity; it MAY be evidence in a `NodeBinding`, which is named in three places and never given a field list. |

---

## 4. Per scope model

§5.2 asks for the table per model. The owner ruled **N**. T and C are given
here as contrast, from the docs, so the cost of the choice is visible. Neither
is built; both paragraphs are `[D20]` unless marked.

**T — topology-scoped.** A node may connect on the protocol carrying share X
only if it is authorised for X, rejected at accept time by remote key and ALPN.
Under T the §3.2 table mostly empties: an unauthorised node never completes a
connection on X's protocol, so it never sees X's share id, glade ids, record
keys, origin ids or heads, and GDL-010 is answered by construction for node
peers. Two things do not empty. The ALPN string is *per share* under T, so the
row that today says "one opaque protocol name" becomes "a share name on the
wire" — which, with `[U1]` unresolved, may be the worst row in the whole table
rather than the best. And the directory still has to replicate somewhere, so
the home-share rows are unchanged unless T is applied to `home` too. Iroh
supplies the mechanism (`EndpointHooks` reject by key and ALPN, `[D6]`); Glade
supplies a per-share ALPN registry and an accept-time view of the placement
fold, neither of which exists `[C17]`. Cost: one QUIC connection per peer per
share, and churn on every grant change.

**C — crypto-scoped.** Each share's payload is encrypted, so a node without the
keys routes and stores ciphertext. C is the only model that bounds a node that
is trusted but should not be `[D15]`, and it is the only one that survives the
home node being someone else's hardware. It changes almost none of the §3.2
table: share ids, glade ids, record keys, origin ids, seqs and head hashes are
envelope fields, not payload, so an ungranted node still sees all of them. What
C removes is exactly one row — op payloads — and what it costs is key
distribution, key rotation, and every node-side fold, window and service on that
tier. AZ-10 leaves "what survives of resume/heads" undecided `[D24]`, which
means C's own version of this table cannot be finished yet either.

**N — node-trust-scoped, as ruled.** §3.2 is N's table. The corpus predicted N's
leak as "share ids changed and their heads" via shared gossip topics `[D20]`.
With `sync_round_only` there are no topics, so the predicted leak vector is
gone — but the *actual* leak is larger than predicted and arrives by a different
road: the directory itself. An accepted peer is handed the whole home share at
connect, which is every workspace id, every share id, every principal, every
capability grant and every serve claim, in plaintext `[C5][C7][C9]`. That is by
design `[D17][D18]`, and AZ-4 records it as an open question `[D24]`.

---

## 5. What the rulings already removed

A short list of rows that would otherwise be in the table, and are not.

1. **No gossip topics.** `dissemination = sync_round_only` means no overlay, no
   `TopicId`, no topic membership to observe, and no 32-byte id derived from a
   guessable string `[C24][D7]`. The specific model-N leak §5.2 names is closed
   by this ruling, not by anything in the node.
2. **No second 0.x wire.** With no `iroh-gossip`, `iroh-docs` or `iroh-blobs`
   dependency there is no second ALPN, no second swarm and no second
   observability surface `[C24]`.
3. **The endpoint key stops doubling as identity — half done.**
   `transport_key_binding = binding_record` is already reflected in the carrier:
   `PeerEndpoint::bind_with` takes an explicit Glade identity and keeps the iroh
   key transport-only `[C1]`. The record that would *prove* the pairing does not
   exist `[C18]`, so today the binding is the CLI flag, exactly as §7.2
   predicted `[D21]`.
4. **No foreign identity source.** `identity_adapters = none_in_v1` means no
   OIDC token, Kerberos ticket or SPIFFE SVID on any wire, and no external
   issuer that learns who logged in where.
5. **No third-party proof verifier.** `proof_family = taut_grants` keeps the
   encoding Glade's own, so no outside tool sees a grant — though note that
   grants are plaintext CBOR records in a share that replicates everywhere
   `[C9]`, so "no outside verifier" is not the same as "not readable".
6. **The crafted-address panic is out of reach.** The iroh 1.2 floor is in the
   manifest `[C22]`, which matters the moment addresses are parsed from tickets
   or directory records rather than typed on a command line.

---

## 6. The uncomfortable rows

Most serious first. Each one says what closing it would take, and what that
costs.

### 6.1 Content flows to any accepted peer that asks for it

The question's premise — content does not flow without a grant — does not hold.
`serve_peer_subscribe` takes any `Subscribe` frame from any linked peer,
registers it as an ordinary subscriber of that zone, acks with the zone's heads,
ships the gap and then feeds it live until the peer closes `[C10]`. There is no
authorization step anywhere on that path. The same is true of forwarded
exchanges `[C20]` and of every websocket client `[C12]`. `grants_for` is
implemented and never called outside tests `[C11]`. The existing end-to-end
tests demonstrate the behaviour without naming it as a leak `[C26]`.

This is not a contradiction of the *design*: AZ §7 says plainly that node↔node
replication is governed by node trust, not user grants `[D12]`, and INV-4 is
scoped to client and service sessions `[D23]`. But the client-session half of
the design — the serve hop, the "cached ≠ allowed" point — is also absent, and
that half *is* what the ruling's own wording leans on ("the serve hop enforces
per-session grants").

*To close:* implement the filter at the point the code already names
`[C7a]` — one `grants_for` fold query in `serve_peer_subscribe` and in the
websocket subscribe path. *Cost:* small. The fold, the query and the
revocation-wins semantics all exist. The real work is deciding what happens
when the grant fold is stale or the clock is uncertain, and AZM §6 already
requires re-evaluation when the fold changes, which means cutting live streams.

### 6.2 The whole grant table reaches every accepted peer

`dir.grants`, `dir.revocations` and `dir.principals` are streams of the home
share `[C8]`, holding `CapabilityGrant{principal, share, verbs}` in plaintext
`[C9]`. The home share is what `pull_home`/`serve_home` exchange at connect,
unfiltered `[C5][C7]`, and `push_home` pushes new records to every live link
`[C19]`. So a node that holds no grant for anything still learns every
principal's name, every share id, and every verb anyone holds on any share.

This is designed `[D17][D18]` — the directory is an ordinary share and every
device is a destination — and it is recorded as an open question, AZ-4: "what
does a grantee of one workspace see of the list?" `[D24]`.

*To close:* either move the policy streams into a zone that only policy-granted
nodes receive, or encrypt them. The ratified B4 zone refinement is the hook:
per-zone contiguous chains are explicitly "what makes private zones filterable
from what a peer receives without breaking chain verification" `[D25]`.
*Cost:* high. It breaks the property that the directory is one ordinary share
(GDL-038), and it needs a boot path that works for a node holding the shares but
not the policy.

### 6.3 "Accepted peer" is not a check

`verify_peer` accepts any 32-byte id with any signature `[C4]`, and no
accept-time policy exists `[C17]`. There is no known-node set, no allowlist, no
`EndpointHooks`. The identity a peer claims on HELLO is unbound to the iroh key
that authenticated the QUIC connection, and the record that would bind them is
not built `[C18][D21]`. The security analysis is explicit that the HELLO frame
MUST carry a node/transport binding slot `[D26]`; the frame has three fields and
no slot `[C3]`.

Today the blast radius is one machine, because the endpoint binds loopback
`[C1]`. At the first real route it is whoever can reach the port.

*To close:* the binding record plus a real signature check plus an
`EndpointHooks` reject on unknown keys. *Cost:* moderate and well understood —
one record kind, one fold rule, one hook. §7.2 already specifies the record
`[D21]`, and iroh already supplies the hook at the right moment `[D6]`. The open
part is what the hook does while the clock is uncertain.

### 6.4 The ids are legible, and the security analysis says they should not be

The existing metadata table assumes "workspace/share **opaque** ids" and
"opaque route/topic ids" `[D10]`, and says human-readable workspace names,
binding names and principal display names should be treated as sensitive
metadata `[D11]`. The code ships the opposite everywhere: `ws-razel` as a share
id, `dir.grants` and `gyld.streams` as glade ids, `glade-gyld:ws-razel:gyld.ops`
as an origin id built by string interpolation `[C14]`, and conversation ids as
raw UTF-8 record keys `[C15]`. A node that learns only *the ids* of a share it
has no grant for learns what that share is for.

*To close:* opaque ids on transport paths, with the legible names in a mapping
only granted peers hold `[D10]`. *Cost:* high, and partly non-technical. The ids
are the debugging surface, the log surface and the fold keys, and every trace in
the corpus is written in terms of them. It is a wire change and a readability
loss.

### 6.5 The client port is unauthenticated

The websocket carrier is a plaintext HTTP/1.1 upgrade with no TLS and no
authentication `[C13]`; a `Hello` binds whatever principal the client names, and
the code says so: "identity as data, nothing enforced" `[C12]`. Any process on
the machine reads any share the node holds. Loopback binding bounds this today
`[C13]`, but the entry-node role is by definition a session host for sessions
arriving over a network `[D19]`, and AZM §7b's key-signed vs operator-vouched
session distinction has no counterpart in the code.

*To close:* the same serve-hop check as 6.1, plus a real session identity.
*Cost:* the check is small; the session identity is the device-certificate work
that `key_custody = recovery_keys` opens and `identity_adapters = none_in_v1`
says will be done by hand.

### 6.6 The relay and DNS question is gated behind a branch that was not taken

`relay_posture` ("Whose relays and whose DNS does the first real route run
on?") is recorded in `GladeDecisionIndex.md` as branch-induced, live *only if*
`topology` is taken. The owner took `node_trust`. So the one question that
governs the entire ROUTE column of §3.1 is currently not live on the graph — yet
turning relays on is not optional for a real route, and iroh falls back to a
relay automatically when hole-punching fails `[D4]`. The choice of whose relay
and whose DNS `[D22]` is a metadata decision under every scope model, not just
under T.

*To close:* re-gate `relay_posture` so it hangs off `metadata_exposure` or off
the first-real-route slice rather than off `topology`. *Cost:* a graph edit,
which is the owner's to make. **This document does not touch the graph.**

---

## 7. Candidate alternatives for `metadata_exposure`

Four, written the way the graph writes alternatives. They form a ladder: each
rung includes the one above it, so choosing is choosing how far to climb, and
they are mutually exclusive as a ruling.

**`legible_by_design`** — Accept that any node the operator accepts sees the
whole directory in plaintext: every workspace id, share id, principal name and
capability grant, plus the heads of every share it asks for. Nothing is built,
debugging stays readable, and the answer to "what does a relay-only peer see" is
"everything except payload". The cost is that the security analysis's
"treat human-readable names as sensitive metadata" is refused rather than
deferred, and a node later found to be someone else's has already read the list.

**`granted_shares_only`** — Filter what a peer is offered by the grants its
operator holds, at the one drop-in point the sync path already names, so a node
without a grant for X never receives X's ops, keys, origins or heads. It makes
the question's own premise true and costs one fold query on the serve path. The
directory still replicates whole, so share ids, principal names and the grant
table itself stay visible to every accepted node, and a stale fold needs a
defined fail direction.

**`policy_zone_split`** — Keep the directory replicating everywhere but move the
policy streams into a zone that only nodes holding a policy grant receive, using
the per-zone chains that already make a zone filterable without breaking chain
verification. An ungranted node then learns that shares exist but not who may
touch them. It costs a second replication rule, a boot path that works when a
node has the shares but not the policy, and it gives up the property that the
directory is one ordinary share.

**`opaque_wire_ids`** — Replace share, glade and origin ids on the wire with
opaque derived ids, keeping the legible names in a mapping only granted peers
hold. It is the only option that closes the name leak against a relay and an
address-lookup service as well as against a peer, and it is the only one that
matches the table the security analysis already drafted. It costs a wire change,
the readability of every log, trace and test in the corpus, and a mapping that
has to be kept correct forever.

---

## 8. Open unknowns, each with the cheapest experiment

| Tag | Unknown | Cheapest test |
| --- | --- | --- |
| `U1` | Is the ALPN string `glade/node/1` readable by an on-path observer, or by a relay? `IrohReview.md` never says — ALPN appears only as a dial parameter and dispatch key, and there is no statement about the QUIC/TLS handshake's visibility. Do not fill this cell from general knowledge. | Run the existing `iroh_carrier::tests::dial_and_hello_over_iroh` under `tcpdump -i lo0 -w /tmp/c.pcap` and grep the capture for the literal `glade/node/1`. One command, no new code. This also settles the T-model's worst row before T is ever costed. |
| `U2` | Do packet sizes and timing on the direct path let an observer count ops or estimate record sizes? `IrohReview.md` scopes "how much" to the relay's view only and says nothing about the direct path, padding or traffic analysis. | Capture `iroh_carrier::tests::sync_over_iroh` twice with a known op count (the test appends 4; make a copy that appends 400) and compare datagram-length histograms. |
| `U3` | What does a pkarr relay, DNS origin or DHT participant learn from *queries* — who looks up whom, enumerability, TTL? The review describes only the publish side. | Run `iroh-dns-server` locally, point one node at it with the `N0DisableRelay` preset instead of `Minimal`, do one dial, and read the server's request log. |
| `U4` | Does a QUIC connection id stay stable across a path migration, linking one peer's sessions across networks? The review notes only that `noq` supports custom and zero-length connection ids, not what iroh uses. | The `U1` capture, plus a forced rebind: compare connection ids before and after. |
| `U5` | If `gossip_overlay` is ever taken, would a `TopicId` derived by "hashing a meaningful string" be recoverable by dictionary from a share id? | Not applicable under `sync_round_only`. Costs nothing to leave open until the dissemination ruling is revisited. |
| `U6` | What is the fail direction when the grant fold is stale or the clock is uncertain at a serve hop? No code exists to inspect and no document rules it; AZM §6 requires re-evaluation when the fold changes but does not say what happens under uncertainty. | Not an experiment — a design choice to record when 6.1 is closed. The security analysis's policy classes (low-risk read may allow with a stale cursor; trust/policy update must fail closed) are the obvious starting shape. |
| `U7` | Will the entry-node role put the websocket carrier on a network, and under what authentication? AZM §7b distinguishes key-signed from operator-vouched sessions and requires the difference to be policy-visible; nothing in the code does. | Read the gryth entry-node plan before 6.5 is costed. A document question, not a measurement. |

---

## 9. Where the code and the documents disagree

Recorded plainly, because the table is only as good as this list.

1. **"Content does not flow without a grant" is asserted by the question and by
   §5.2's model-N row, and the code does not implement it** `[C10][C11][C20]`.
   The documents are internally consistent about *why* (node trust, INV-4 scoped
   to sessions), but the session-side check they rely on is also absent.
2. **The metadata table in the security analysis assumes opaque ids; the code
   uses descriptive ones** `[D10][D11]` vs `[C9][C14][C15]`. Its `Store-only`
   row reads "workspace/share **opaque** ids"; the wire carries `ws-razel` and
   `glade-gyld:ws-razel:gyld.ops`.
3. **The `HELLO` frame is required to carry a node/transport key binding slot
   and does not** `[D26]` vs `[C3]`. Likewise `HEADS`, `SUBSCRIBE` and
   `EXCHANGE` are required to carry `principal_id` / `capability_ref` /
   `policy_ref` slots; `Heads{streams}` and `Subscribe{share, glade_id, key,
   from}` have none.
4. **The verb taxonomies do not agree, and the code implements neither.**
   AZM §5 has three wire verbs (`read.subscribe`, `read.window`,
   `write.append`); the security analysis adds `route`, `store`, `resume` and
   `decrypt`, with `resume` defined as "Serve heads and gaps". The node's
   `CapabilityGrant.verbs` is an unvalidated `Vec<String>` `[C9]`, and nothing
   reads it `[C11]`. There is consequently no document that states a node must
   hold a grant before receiving *heads* — AZ-10 leaves it open `[D24]`.
5. **`WorkspaceEntry` is specified with a gwz manifest identity and does not
   have one** `[D17]` vs `[C9]`. Minor, but it means the directory cannot yet
   distinguish two checkouts of the same repo, which is WD-5's problem.
6. **`presets::Minimal` does what the comments say it does.** Not a
   disagreement — recorded because it was the one claim the whole "TODAY"
   column rests on, and it is now verified at the dependency, not inferred from
   a comment `[C2]`.

---

## 10. What this document does not settle

It does not rule. It does not edit the decision graph, and the alternatives in
§7 are candidates for the owner to accept, reject or rewrite. It does not
address what a *stolen disk* reveals, which AZ §7 already answers honestly
("enforcement guards sessions, not stolen disks"). It does not cost the
remediation in §6 beyond naming the shape of the work. And it leaves GDL-010
open, as the security analysis asks — but GDL-010 now has the table it was
waiting for.
