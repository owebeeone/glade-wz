# Glade System-Data Seam — defer the storage design, not the shape

Status: working draft — records the GDL-036 direction (Gianni, 2026-07-06)

Purpose: get the full end-to-end glade system working SHORT TERM without
paying the system-data (and gryth-data) storage design price now. The model
is already locked and does not change here: the registry (node→resource map,
claims, hints, grants) is a **share** — ops, folds, no separate ACL database
(`GladeAuthzModel.md` §4, `GladeWorkspaceDirectory.md`). What this document
defers is two *implementations*, each behind a seam that makes the later swap
an impl detail. Seams-at-inception: the wall ships now, around a stub.

## The two seams

| Seam | Question it hides | Interim impl | Later impl |
| --- | --- | --- | --- |
| **RegistryApi** | where answers come from | in-memory fold loaded from a snapshot blob; appends applied in memory | real home-share folds (WD P2), op-granular sync |
| **StoreApi** | how a node persists | the WHOLE system state as **one taut message** on disk, rewritten on change | SQLite engine (indexes for hot queries); later, whatever earns it |

The browser twin: glial's client store (GC-4, IndexedDB) is the SAME StoreApi
shape in TS — one seam, two runtimes.

## The non-negotiable shape (this is what makes the swap cheap)

1. **Reads are queries-over-fold**, never `getConfig()`: `whoServes(ws)`,
   `replicasOf(share)`, `grantsFor(principal, share)`, `nodesOf(operator)`.
   A fold-backed impl must slot in without any caller changing.
2. **Writes are record APPENDS**, never `setConfig(blob)`: `append(ServeClaim{…})`,
   `append(CapabilityRevocation{…})` — each carrying **origin attribution**
   even in blob-land, so migration to per-origin logs is mechanical.
3. **Records are taut-defined and versioned from day one** (they already are:
   the WD §2 record kinds) with the AZ-11 rule: unparseable policy records
   fail CLOSED.
4. The snapshot wrapper is substrate vocabulary, not a hack:
   `SystemSnapshot{records, heads}` — *a snapshot is a cached fold + heads*
   (SubstrateV1 §2). Shipping the whole blob to a connecting peer is
   degenerate sync, and op-granular sync replaces it later behind the same
   RegistryApi.

Anti-pattern, named: a `get/set-config-object` API. It couples every caller
to blob shape and turns the migration into a rewrite. If the interim API
cannot be re-implemented over folds without touching callers, it is wrong.

## SQLite's place, pinned

Node-local **store engine** — indexes and hot-query support behind StoreApi —
**never the replication mechanism**. Replication is ops on shares (heads,
gaps, verify-as-ingest — the s-sync trace); no database-level sync ever.
This keeps "the DC database = per-(share, origin) logs" true at every scale.

## Migration ladder (each rung invisible to consumers)

1. blob file (one taut message) — NOW
2. SQLite behind StoreApi — when query volume or size hurts
3. op-granular home-share sync replacing blob-ship behind RegistryApi — WD P2
4. (gryth app data follows the same pattern into app shares)

**The test of the seam is the atlas**: no rung may change any ggg-viz trace.
The 25 traces are the regression net for this deferral — if a swap becomes
visible in a trace, the seam failed.

## On-disk layout (ruled — Gianni, 2026-07-06)

Three launch **profiles** of the one node binary, each a default instance
name, overridable with `--name` (two peers on a dev box = `--name glade-peer2`;
`GLADE_HOME` overrides `$HOME/.glade` for tests):

| Profile | Default name | Typical roles |
| --- | --- | --- |
| localhost session host | `glade-local` | entry node for the local UI |
| workspace host | `glade-peer` | claim-holder, grazel/gwz embedded |
| DC / fleet | `glade-server` | entry + durable home-node roles |

Names are DEPLOYMENT profiles, not protocol types — the protocol still knows
only roles + operators (WD §7b); no trace ever sees a profile name.

Per instance, a DIRECTORY (refined 2026-07-06 from the flat-file sketch — the
grazel `~/.grazel/scopes/<scope>/` idiom): `$HOME/.glade/sys/<name>/`

| File | Trust class (§Classification) | Ships? |
| --- | --- | --- |
| `node.key` | 1 — secrets (mode 0600) | NEVER |
| `records.json` | 2 — signed replicated records (`SystemSnapshot`) | yes — this IS the degenerate-sync artifact |
| `local.json` | 3 — node-private assertions (node-self-signed) | never |
| `cache/` | 4 — derived (rebuildable; later the SQLite home) | never |
| `instance.lock` | — single-writer lock (workspace.lock precedent) | never |

Rules: writes are tmp+rename (crash-atomic); JSON is the at-rest rendering
only — signatures/hashes over records are computed on canonical CBOR, never
the JSON text; nothing above StoreApi knows files exist.

## Data classification — the four trust classes

Two axes, not one: *who may read it* (confidentiality) and *who vouches for
it* (authenticity). "secure vs control" collapses these; four classes keep
them straight.

**Class 1 — node secrets** (`node.key`): the ed25519 secret; later, session
keys and E2E-tier share keys. Possession IS the credential. Never shipped,
never in any snapshot. The asymmetry worth knowing: key **theft** =
impersonation (blast radius: that node's grants, until its chain is revoked);
key **replacement** = identity loss only — the NodeId changes, nothing in the
directory matches, the mesh rejects. A tamperer who overwrites the key gains
nothing and announces themselves.

**Class 2 — signed replicated records** (`records.json`): WorkspaceEntry,
ServeClaim, Grant/Revocation, PrincipalDecl, NodeRecord, checkpoints,
ReplicaHints — everything that is (or will be) ops in shares.
**Self-authenticating at rest**: origin signatures + GQ-9 chains verify on
load exactly as on the wire. A malicious edit of the JSON is
indistinguishable from a tampered sync chunk — rejected, quarantined,
healed from any replica. File tamper here is self-DoS, never forgery.
Confidentiality follows placement (`replica.hold`).

**Class 3 — node-private assertions** (`local.json`): things only this node
asserts and nobody countersigns — the authority overlay (§6 AuthzModel),
suspect marks (s-sync), trust-plug configuration, repair-loop state, resume
vectors. Never shipped (they are private judgments). **Node-self-signed**:
signed with `node.key` on write, checked on load — detects corruption and
non-root tamper, honestly does NOT stop local root (hardware trumps records,
§7a). Two saving properties: (a) the overlay only ever NARROWS, so overlay
tamper cannot exceed granted rights; (b) every class-3 item MUST declare a
**fail-closed default** — a failed self-signature discards to the MOST
restrictive value, never to "off".

**Class 4 — derived caches** (`cache/`): cached folds, indexes, reassembly
state, NodeHints. No external authenticity needed — rebuildable from class 2.
Two rules make them safe: (a) **`rm -rf cache/` is always correct** — any
impl that breaks this rule is wrong; (b) cached folds carry the heads + hash
they were folded at, self-signed like class 3 — on doubt, refold. Blast
radius of cache tamper: lies to LOCAL consumers only — anything served to a
peer ships the class-2 signed ops, which the peer re-verifies (every hop
verifies), so cache tamper cannot propagate.

## Validation at load — the disk is another untrusted carrier

Boot = sync from a carrier named "the disk", in class order:

1. `node.key`: permissions check (refuse group/world-readable, the ssh
   discipline); derive NodeId; it MUST match our own NodeRecord in class 2.
2. `records.json`: **verify-as-ingest** (the s-sync Y2 path, verbatim):
   chains, signatures, seq monotonicity. Failures follow Y3 — reject the
   record and its suffix, quarantine the bytes as evidence, heal from peers;
   policy records additionally fail CLOSED (AZ-11).
3. `local.json`: node-self-signature; on mismatch, discard each item to its
   declared fail-closed default and record evidence.
4. `cache/`: hash-check or discard-and-refold; never trusted, never load-
   bearing.

One consequence to enjoy: there is no separate "storage security audit" to
maintain — the load path IS the sync path, so every hardening of s-sync
hardens boot for free.

## Threats, honestly

| Threat | Answer |
| --- | --- |
| Local root / the operator | Out of scope by design (§7a): hardware trumps records. They can change class 3/4 (their node's own behavior, bounded by grants) and self-DoS class 2. |
| Non-root local process | File permissions + class-3/4 self-signatures + class-2 chains. |
| Bit rot / crash corruption | Same machinery: chains and self-signatures catch it; tmp+rename bounds it; heal from replicas. |
| Stolen disk | Confidentiality only as good as OS disk encryption; `node.key` exposure = revoke the node's chain (one op). E2E-tier share keys are the later, stronger story (AZ-10). |
| Malicious snapshot FROM a peer | Class 2 self-authenticates; classes 1/3/4 are never accepted from the wire at all. |

## Ties

WD-7/AZ-8 (min-replica policy — still deferred, now explicitly behind
RegistryApi), GC-4 (client store twin), AZ-11 (versioning/fail-closed),
`seams-enforced-at-inception` (the working rule this instantiates).
