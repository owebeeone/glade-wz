# Glade Persistence Review — sources, shapes, and what the design corpus actually says (2026-09-05)

**What this is.** A cross-cutting review of persistence across Glade's data sources, commissioned
because the operator wants to design persistence and suspects the existing design corpus will not
survive scrutiny. The trigger observation: Glade/gryth consume several *source classes* — filesystem,
terminals/running commands, SQLite (gryth config), and Glade itself (peers, auth, discovery) — and
only some are databases. Each has a different authority model, a different durability need, and a
different taut-shape/fold. The suspicion is that prior designs treated them uniformly, or left them
undesigned. Separately, node discovery is believed to be a black hole.

**Method.** Adversarial: the job was to *refute* the corpus, not confirm it — and to refute in both
directions, so claims that held up are recorded too (§9). Read: the root `dev-docs/` anchors, the
`glade/dev-docs/` substrate/adapter set, `dev-docs/glade/*` (the design corpus proper),
`plan-docs/plans/GLP-0006-*`, and the real code — `glade/node/src/*.rs` (6,134 LOC),
`glade/wire-rs/`, `glade/client-ts/`, `glade/demo/src/`, `glial/src/`, `glade-gwz/src/`, `grazel/`,
`glade-discover/`, `taut-shape/`. Ran: `git log/show/status/diff/ls-files`, `rg`, `ls`, `wc`, `find`,
one `python3` script that counted length-prefixed records in the live demo journal, and one
`curl -o /dev/null` liveness probe of the running demo. **No builds, no tests, no writes, no `gwz`
mutating verbs.** `cargo test` was *not* run; where a claim would need it, it is marked INFERENCE.
The running demo was read, never driven.

> **Methodology warning for every future reviewer.** This workspace is a gwz multi-repo: each
> top-level directory is a nested git repo, and **`rg` from the workspace root silently skips their
> contents**. Reproduced: `rg -l "SwmrWriterConflict" .` → 0 hits; `rg -uu -l "SwmrWriterConflict" .`
> → `glade/node/src/{peer,store,session}.rs`. Every negative search below was re-run with `-uu`.
> At least one earlier "not referenced anywhere" conclusion in this session was wrong before
> correction. Treat any bare-`rg` negative in the corpus as unproven.

Convention, per house style: **REPRODUCED FACT** = read at the cited `path:line` or produced by a
command shown here. **INFERENCE** = my reading of what those facts imply. Paths are relative to
`/Users/owebeeone/limbo/glade-wz`; anything outside it is fully qualified. **Where a doc and the code
disagree, the code is the fact and the doc is the claim.**

---

## 1. Thesis

**Glade has one persistence design — an unbounded, never-reclaimed, per-`(share, glade_id, key,
origin)` append-only op journal fully replayed into RAM at boot — and it is applied identically to
every source class regardless of authority or durability need.** The operator's suspicion is correct
and understated: the corpus does not merely treat the sources uniformly, it *never separately
designed three of the four*. Terminal/command output is persisted forever, one durable op per line,
with the write result discarded (`glade-gwz/src/supplier.rs:244-247`). The filesystem is not a source
at all — `ws.tree`/`ws.files` are declared surfaces with no producer, no `read_dir`, and no watcher
anywhere in the tree. Peer auth is not persisted because it does not exist (`verify_peer` returns
`Ok` unconditionally, `glade/node/src/peer.rs:99-101`). And node discovery is disabled in code
(`presets::Minimal`, localhost-only, `glade/node/src/iroh_carrier.rs:31-38`) while a complete,
211-test discovery kernel sits in `glade-discover/` with **zero consumers**.

The single load-bearing mechanism that *is* real — the substrate, zones, SWMR and CRDT folds — works,
demonstrably: a two-user demo is running right now and its journal is on disk. What is missing is not
the fold. It is **every policy that decides what a fold keeps, for how long, and on whose authority.**
`retention` is parsed off every binding declaration and read by *nothing*
(`glade/node/src/appdecl.rs:138` → `sysdata.rs:125` → no consumer). GAP-10 is not a "retention tail";
it is the absence of the entire retention layer, and the growth is superlinear, not linear (§4.1).

---

## 2. Source classes and their authority model

The four classes in the brief, plus two discovered. "Enters at" is the first line of code where the
data crosses into Glade.

| # | Source class | Enters the system at | Shape / engine | Fold | Persisted | Derived on restart | Authority | REAL STATE |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| a | **Filesystem (files, trees, watches)** | **nowhere** | `ws.tree`=`value`, `ws.files`=`swmr` (declared only) | — | — | — | undefined | **NO PRODUCER EXISTS** |
| b | **Terminals / running commands** | `glade-gwz/src/supplier.rs:231-238` | `log` (per-line durable op) | log append | every line, forever | full replay | the supplier process | **BUILT, UNBOUNDED, UNACKED** |
| c | **SQLite (gryth/grazel config)** | `grazel/src/main.rs:55` | n/a (rusqlite, outside Glade) | none | one row | migration ladder | **nothing reads it** | **INERT** |
| d | **Glade itself — peers / auth / discovery** | `glade/node/src/peer.rs:99`, `iroh_carrier.rs:31` | n/a | n/a | node key only | recomputed | **no authentication exists** | **STUBBED / DISABLED** |
| e | *(discovered)* **Directory / serve claims** | `glade/node/src/claims.rs:236-260` | registry `Record::Serve` | LWW-by-epoch, lease at read time | one op **every 10 s per share**, forever | full replay + re-verify | the claiming node | **BUILT, PATHOLOGICAL GROWTH** |
| f | *(discovered)* **Browser/app collaborative state** | `glade/demo/src/taps.ts:38-73` | `value`, `log`, `swmr`, `crdt`+`text_crdt` | value/log native; SWMR/CRDT delegated | node journal + client IndexedDB | full replay both ends | per-tab random origin | **BUILT AND WORKING** |

### (a) Filesystem — REPRODUCED FACT: there is no filesystem source

`ws.tree`/`ws.files` are declared at `glade/apps/grazel-app.glade:22-23` (byte-identical in
`grazel/apps/`). A `-uu` search for `ws.tree` finds it in exactly three places: those declarations,
a parse test (`glade/node/src/appdecl.rs:286,304`), and `#[cfg(test)]` E2E fixtures
(`glade/node/src/mesh.rs:596,690,709-734`; `exchange.rs:399,510-516,641-692`). **No production
producer.** Confirming negatives, all `-uu`: no `notify`/`inotify`/`fsevent` in any `Cargo.toml`; no
`walkdir`; no `read_dir` outside `glade/node/src/store.rs:102,113` (which reads the *journal*). The
only `fs::read` on an app path is `grazel/src/lib.rs:302` — the **HTTP static-file handler** for the
UI bundle (`read_static`, `:296-304`), not a Glade surface.

The demo's file surface is one anonymous blob per document share: `glade/demo/src/files.ts:6`
*"every snapshot/delta body is one complete UTF-8 image"*, window fixed at `{from: 0, length: 4096}`
(`:14`). The UI says so — `glade/demo/src/WorkspacePanel.tsx:96` renders the literal
`ws.files · demo.txt (path routing pending)`. No path, no tree, no delta, no backfill, no watch, no
at-rest file.

**INFERENCE.** Class (a) is not "designed uniformly with the others" — it is *entirely undesigned in
code*, against a 301-line specification (`dev-docs/glade/suppliers/glade-files.md`) with no
implementation. The single largest gap between corpus and reality.

### (b) Terminals / running commands — REPRODUCED FACT: persisted, one durable op per line

The brief asks this most directly. **Yes, there is persistence today: the append-only journal, at
line granularity.** `glade-gwz/src/supplier.rs:231-238` loops `while let Some((stream, line)) =
rx.recv().await` and calls `append_output(...)` per line; `:244-247` is

```rust
async fn append_output(client: &GladeClient, config: &GwzConfig, run_id: &str, rec: &GwzOutputRecord) {
    let _ = client.append(&config.share, &config.output_id, "log", rec.to_bytes(), Some(run_id.as_bytes())).await;
}
```

Three findings:

1. **One durable, chain-hashed, replicated op per line of stdout/stderr**, keyed by `run_id`, so each
   run is its own chain — and each chain lives in the journal forever (§4.1).
2. **The append result is discarded** (`let _ =`). Module comment `:175-176`: *"Best-effort — an
   append failure (link drop mid-run) is dropped."* No acknowledgement, no retry, no outbox: a link
   drop silently truncates the record with no marker.
3. Declared retention for the terminal surface is the free-form `windowed`
   (`glade/apps/grazel-app.glade:25`), which `glade/dev-docs/GladeShapeDispatch.md:38` itself grades
   *"Ambiguous legacy token; owner decision required"* — and which no code reads.

There is **no PTY**: `-uu` for `portable_pty|openpty|PtyPair` across `glade`, `glade-gwz`, `grazel`,
`glial` returns one hit, the string literal `"pty"` in an echo-provider fixture
(`glade/node/src/echo.rs:87`). `glade-terminal` is a supplier spec with **no repo**. The `stream`
engine — the canonical taut shape for exactly this — has a released contract (`stream.oracle/v1`,
`glial/src/shapes.ts:39-42`) and a corpus (`taut-shape/corpus/scripts_stream/`) but is **rejected by
the node** (`glade/node/src/appdecl.rs:44`).

**INFERENCE.** Terminal output rides the durability mechanism designed for collaborative document
state. Every property that makes that mechanism correct for a shared document — per-op hash chaining,
equivocation proofs, permanent retention, full replay — is a liability for a high-volume ephemeral
byte stream.

### (c) SQLite (gryth/grazel configuration) — REPRODUCED FACT: the store is inert, and it is not Garns

Fully treated in §6. Summary: `grazel/src/main.rs:55` opens the store into `let _application_store`
— the Rust idiom for a deliberately unused binding — writes one idempotent row duplicating an
authoritative `.glade` declaration, and never reads it. **Grazel configuration lives in argv**
(`Config::parse`, `grazel/src/lib.rs:116-168`) before and after the adoption commit.

### (d) Glade itself — peers, auth, discovery — REPRODUCED FACT: nothing to persist, because nothing exists

- **Auth.** `glade/node/src/peer.rs:99-101` — `verify_peer(node_id, _sig)` returns `Ok(...)`
  unconditionally; the signature argument is `_`-prefixed and unread. Comment `:96-98`: *"STUBBED:
  structure is real … but the check always accepts."* `:143-145`:
  `pub fn verify_origin_sig(_op: &Op) -> bool { true }`.
- **Identity.** The only persisted secret is `node.key` — 32 random bytes, mode-0600 enforced
  (`glade/node/src/sysdir.rs:190-236`; `check_key_perms` refuses group-readable);
  `node_id = sha256(key)` (`peer.rs:68-71`). No key-rotation path exists.
- **Discovery.** `glade/node/src/iroh_carrier.rs:31-38` — `Endpoint::builder(presets::Minimal)` …
  `.bind_addr((Ipv4Addr::LOCALHOST, 0))`, documented `:4-5` as *"relay + discovery disabled — direct
  dial by socket address only"*; echoed in `glade/node/Cargo.toml:19-21`. Peers are reached only by a
  hand-pasted `--peer <endpoint-id>@<ip:port>` (`glade/node/src/bin/glade-node.rs:43-47`).

**INFERENCE.** The brief asks what peer/auth/discovery state is persisted vs recomputed. The question
is premature: there is one persisted node key and *no auth state at all*. When authentication lands it
introduces a genuinely new persistence class (keys, sessions, revocations, replay windows) that no
current doc sizes.

### (e) Directory / serve claims — the growth pathology

`glade/node/src/claims.rs:40-42`: `LEASE_TTL_MS = 30_000`, `RENEW_EVERY_MS = 10_000`.
`glade/node/src/claims.rs:236-260`, `renew_leases`: every 10 s, per served share, append a fresh
`Record::Serve` op **and** `let _ = dir.persist()` — which is `BlobStore::save`
(`glade/node/src/registry.rs:155-163`), a **whole-state CBOR rewrite** of `records.json` via
tmp+rename. The op is then also fanned out to the store and every peer (`claims.rs:263-275`).

This is 6 ops/minute/share → ~8,640/day → ~3.2M/year, each one triggering a full rewrite of a file
that grows with the op count. See §4.1 for what that costs.

### (f) Browser/app collaborative state — the part that works

`glade/demo/src/manifest.ts:26-66` declares six typed surfaces: `status` (`value`, account/commons),
`selection` (`value`, doc/private), `notes` (`value`, doc/commons), `collaborativeNotes` (`crdt` +
`text_crdt` profile, doc/commons), `activity` (`log`, doc/commons), `files` (`swmr`, doc/commons).
`glade/demo/src/taps.ts:31-73` mounts each through the glial binder. This is live: the demo is
running (`curl` → `vite:200`; `glade-nod` PID 45158 LISTEN on 127.0.0.1:9099) and its journal is on
disk (§4.1). Zones hold: private is keyed `self:{self}` (`glade/demo/src/manifest.ts:76`) and never
leaves the writer's key.

**Identity is per-tab and random.** `glade/demo/src/glial.ts:67-77` (`stableOrigin`) reads
`sessionStorage["glade-origin"]` or mints `Math.random().toString(36).slice(2, 8)`; `:79` sets
`user = params.get("user") ?? origin`. A deliberate ruling
(`glial/dev-docs/DecisionLog.md:160-163`, Gianni 2026-07-11: per-tab origins are the product intent).
Its persistence consequence was not costed — §4.2.

---

## 3. Shapes and folds in practice

The catalogue (GDL-041, ratified 2026-08-28, `dev-docs/DecisionLog.md:60`) names engines `value`,
`atom`, `log`, `stream`, `swmr`, `crdt` and profiles `snapshot_delta`, `text_crdt`. What each layer
actually implements:

| Layer | Accepts / implements | Evidence |
| --- | --- | --- |
| Glade wire enum | `Value`, `Log`, `Stream`, `Swmr`, `Crdt` — **no `atom`** | `glade/wire-rs/src/generated.rs:113-119` |
| `.glade` app declaration | `value`, `log`, `swmr` only | `glade/node/src/appdecl.rs:44` |
| `.glade` recognized-but-refused | `message`, `stream`, `exchange`, `window` | `glade/node/src/appdecl.rs:43,119-122` |
| Node store contract check | `Swmr` (payload decoded), `Crdt` (multi-writer OK), shape-mixing refused | `glade/node/src/store.rs:159-188` |
| Glial delivery shapes | `atom`, `crdt`, `value`, `log`, `stream`, `swmr`, `text_crdt` | `glial/src/shapes.ts:8` |
| Glial **fold** shapes | `value`, `log` only | `glial/src/shapes.ts:9` |
| Glial **mount** shapes | `value`, `log`, `swmr`, `crdt` | `glial/src/shapes.ts:10` |

### 3.1 CONTRADICTION: `crdt` is not a declarable binding shape

`glade/dev-docs/GladeShapeDispatch.md:30` — the document titled *"exact capability record"*, dated
2026-08-29 — states:

> | Binding declaration | `value`, `log`, `swmr`, `crdt` | Accepted and registered |

The code refutes this. `glade/node/src/appdecl.rs:43-44`:

```rust
const KNOWN_SHAPES: [&str; 7] = ["value", "log", "message", "stream", "exchange", "window", "swmr"];
const BINDING_SHAPES: [&str; 3] = ["value", "log", "swmr"];
```

`rg -n "crdt" glade/node/src/appdecl.rs` returns **nothing**. A `.glade` file containing
`binding x crdt share commons latest` fails at `appdecl.rs:116-118` with *"unknown shape `crdt`"* —
it does not even reach the "unsupported binding shape" branch. **The doc is the claim; the code is
the fact.** CRDT works only on the client/mount path (`glial/src/shapes.ts:10`), which never passes
through `.glade` declaration. Grade: `GladeShapeDispatch.md` **STALE on one row**.

### 3.2 The catalogue is adopted at the *recognition* layer and aspirational at the *delivery* layer

`dev-docs/TautShapeCatalogAdoption.md:34-46` is careful about this and largely honest: *"Catalogue
recognition does not grant runtime capability."* It correctly says local folds stay `value`/`log`.
It correctly flags that terminal live I/O *"MUST NOT be relabelled as the Taut `stream` engine
without a versioned adapter contract"* (`:45-46`). Two of its eight `GSC-*` requirements are
currently unmet by construction:

- `GSC-06` (*"Every application view MUST name an explicit base delivery shape and recovery policy"*)
  cites GLP-0006 P3 and `glade-files.md` as evidence — a spec with no implementation (§2a).
- `GSC-08` (*"Retention MUST remain separate from shape and view identity. Ambiguous policies MUST be
  resolved explicitly, not inferred"*) is satisfied only in the trivial sense that retention is
  separate *because nothing consumes it* (§4.1).

### 3.3 Retention is declared data that no code reads — REPRODUCED FACT

The `.glade` binding grammar is `binding <glade_id> <shape> <authority> <zone> <retention>`
(`glade/node/src/appdecl.rs:18`). The token is parsed at `appdecl.rs:138` into
`BindingDecl.retention: String` (`glade/node/src/sysdata.rs:125`), CBOR-encoded at field 6
(`sysdata.rs:135`), decoded at `sysdata.rs:145`. A `-uu` search for `retention` across
`glade/node/src/*.rs` and `glade/wire-rs/src/*.rs` returns **exactly those five lines**. It is
carried across the wire, folded into the registry, replicated to peers — and never enforced.

The four tokens in use (`latest`, `from-cursor`, `windowed`) are therefore documentation. On the
client side, `glial/dev-docs/DecisionLog.md:196-199` states the same thing from the other end:
*"Enforcing `decl.retention` (TTL/quota) is deferred: the engine is keyed by instanceKey and never
sees the decl."*

---

## 4. What persists, what is derived, what is lost

### 4.1 Node side — one journal, no reclamation, superlinear cost

**REPRODUCED FACT — the storage model.** `glade/node/src/store.rs:85-91` — `Store { root: PathBuf,
logs: BTreeMap<ChainId, Vec<Op>>, proofs: Vec<EquivProof> }` where
`ChainId = (share, glade_id, key, origin)` (`:80-82`). **The entire op history of every chain is held
in RAM.** `Store::open` (`:98-125`) walks `<root>/<hex share>/<hex origin>.log`, decodes every
length-prefixed CBOR record, and pushes all of them into `logs`. `append_to_log` (`:314-322`) only
ever appends.

**REPRODUCED FACT — nothing is ever reclaimed.**
`rg -n "remove_file|truncate|compact|prune|retain|delete|evict" glade/node/src/{store,registry}.rs`
returns two hits, both comments: `store.rs:271` (*"below retained range — treat as seen"*, describing
a retained range that does not exist) and `:332` (*"truncated tail"*, crash recovery). No compaction,
no checkpoint, no snapshot-and-truncate, no TTL, no size cap.

**REPRODUCED FACT — per-append cost is O(chains).** `validate_surface_contract` (`store.rs:159-188`)
iterates **every chain in the store** on **every append** to check shape-mixing and the SWMR
single-writer rule. With the per-tab origin scheme (§4.2), chain count grows with every browser tab
ever opened, so append cost grows with historical usage.

**REPRODUCED FACT — the registry is worse.** `BlobStore::save` (`glade/node/src/registry.rs:155-163`)
serializes the whole `SystemSnapshot` (every registry op ever) and rewrites `records.json` via
tmp+rename; `claims.rs:257` calls it on **every 10-second lease renewal**, so total bytes written
after _n_ renewals is O(n²). `Registry::from_snapshot` (`registry.rs:236-259`) re-verifies every op's
chain continuity at boot, so boot time is O(n) in total lifetime writes.

**LIVE EVIDENCE.** The demo store at `glade/node/target/demo-store`, counted by re-implementing the
4-byte-LE framing from `store.rs:325-336`: **159 ops across 17 chains, 10 distinct origins, 5 shares
(`account:gg`, `account:zahx0z`, `chat`, `doc:1`, `doc:codex-crdt-cursor`), 80 KB.** Largest chain
`doc:1/zahx0z` = 31 ops. File mtimes span 2026-07-12 → 2026-09-05, so ops from tabs closed weeks ago
are still replayed into RAM on every node start. **Nothing has ever been reclaimed.** (The store sits
under `glade/node/target/` — `glade/demo/run_demo.py:35` — so a `cargo clean` destroys it; fine for a
demo, worth naming so it is not mistaken for a durability posture.)

### 4.2 Client side — GC-4 is built and wired, and keyed to an ephemeral identity

**Correction to a plausible-but-wrong reading.** GC-4 (browser store engine) is *not* unbuilt and
*not* unwired. `glial/src/store_idb.ts:56` defines `IndexedDbStoreEngine`; it is exported
(`glial/src/index.ts:19`), unit-tested (`glial/test/store_idb.test.ts`), and **used in production by
the demo** — `glade/demo/src/glial.ts:130`:
`new GlialBinder(await IndexedDbStoreEngine.open(\`glial:${origin}\`), origin)`. The binder's *default*
is memory (`glial/src/binder.ts:35`) and every `grip-share` test uses memory, but the shipped demo
uses IndexedDB.

**The finding is the key, not the engine.** The database name is `glial:${origin}` where `origin` is
the per-tab `Math.random()` value in `sessionStorage` (`glade/demo/src/glial.ts:67-77`). Therefore:

- A **reload** keeps `sessionStorage`, so the tab resumes its own chain — this works, and is the GAP-9
  fix landing correctly.
- A **new tab** mints a new origin → a new IndexedDB database.
- **Closing the tab** loses `sessionStorage`, so that database becomes **unreachable and unreclaimed**.
  `drop` explicitly retains (`glial/src/store_idb.ts:95-99`: *"Persisted ops are RETAINED"*), and
  `purge` (`:102-113`) is the only deletion primitive — called by nothing outside tests.

**INFERENCE.** The client's persistence layer is durable but its *addressing key is ephemeral*.
Storage outlives the only handle that can reach it. This is a per-viewer disk leak with no bound,
and it is the mirror image of the node-side problem: both sides retain everything, neither has an
owner for eviction.

### 4.3 Per-source table: restart / reconnect / epoch change / offline

| Source | Restart (node) | Restart (client) | Reconnect | Epoch change | Offline write |
| --- | --- | --- | --- | --- | --- |
| Terminal `gwz.output` | full replay of every run ever | replays from IDB for that tab | run's chain resumes by `run_id` key | n/a (no epoch on `log`) | **lost silently** (`supplier.rs:246`, `let _ =`) |
| Filesystem | n/a | n/a | n/a | n/a | n/a |
| Grazel SQLite | migration ladder runs | n/a | n/a | n/a | n/a — nothing reads it |
| Peers / auth | node key reloaded; **no peer state at all** | n/a | re-dial by hand-pasted `--peer` | n/a | n/a |
| Serve claims | full replay + re-verify of every renewal op | n/a | lease evaluated at reader clock (`registry.rs:196-199`) | epoch = fold max + 1 (`claims.rs:156-158`) | claim lapses after 30 s |
| `value`/`log` surfaces | full replay | IDB replay, prev-hash re-validated | heads/gap resume per `(origin, zone)` | n/a | GAP-11: **never ships** |
| `swmr` file window | full replay | IDB replay | `SwmrNode` reassembles one generation | `reset` action → new epoch (`glial/src/swmr.ts:101,120-122`) | GAP-11 |
| `crdt` / `text_crdt` | full replay | IDB replay | causal frontier in `Op.refs` | n/a | GAP-11 |

**GAP-11 (offline outbox) is the one loss-of-data item that is honestly recorded.**
`glial/dev-docs/DecisionLog.md:165-176`: *"nothing re-ships those ops when `attachGlade` later lights
connectivity … Until then an offline-first write is local-only durable, never replicated."* That is
accurate and unfixed. It is a **persistence item**, and it is the only one in the corpus stated
plainly enough to act on.

---

## 5. Discovery

**REPRODUCED FACT — node discovery is disabled in the running system, and `glade-discover` is
connected to nothing.**

Two independent investigations (one after correcting the `rg`-nested-repo trap) reached the same
conclusion by positive-search evidence.

**In `glade-node`:** `glade/node/Cargo.toml` dependencies are, in full — `glade-wire = { path =
"../wire-rs" }`, `sha2`, `tokio`, `iroh`. No discovery crate. `glade/node/src/iroh_carrier.rs:31-38`
binds `presets::Minimal` to `Ipv4Addr::LOCALHOST` with relay and discovery off. Peers are reached
only by the manual `--peer <id>@<ip:port>` flag (`glade/node/src/bin/glade-node.rs:43-47`).
`glade/node/src/mesh.rs:621-630` contains `s_discovery_golden_path_end_to_end()` — so the *s-discovery
trace* is implemented and E2E-tested, **by the node's own hand-rolled claim/route code**
(`claims.rs`, `registry.rs`, `router.rs`, `mesh.rs`), not by the kernel.

**In `glade-discover`:** one commit, `65fc18b`, 2026-07-18 — **49 days ago and the only commit in the
repo**. Four crates, 9,029 src LOC, 211 `#[test]`, `unsafe_code = "forbid"`, deps `sha2`/`serde` only;
no `tokio`, no `iroh`, no I/O — deliberate purity. `rg -uu` for `glade-discover|glade_discover` across
`glade/` with **no exclusions** returns exit 1; the reverse search (`glade-node|glade-wire` inside
`glade-discover/`) is likewise empty. Separate cargo worlds: `glade/Cargo.toml` does not exist (three
standalone manifests, `edition 2021`); `glade-discover` is its own 4-member workspace, `edition 2024`.

**The named "adapter" is a hole, not a wire.**
`glade-discover/crates/glade-discover-node-adapter/src/transport.rs:6-10` defines
`trait Transport { fn send(&mut self, to: &NodeId, message: &WireMsg) -> …; }` with **no implementor
anywhere**. `NodeId`/`WireMsg` come from `glade-discover-protocol`, not `glade-wire`, so integration
also needs a bridge between two independent wire vocabularies.

**The uncommitted work is cosmetic.** `git -C glade-discover status --short` → 3 modified docs + 2
untracked docs; the diff replaces *"initial repository commit pending user direction"* with
*"baseline commit `65fc18b`, tag `glade/discover-impl-stage1`"*. **Zero source files modified.**
(Left untouched, per the brief.)

**The F5 code review found 13 issues; NONE was dispositioned.**
`glade-discover/dev-docs/GladeDiscoverCode-ReviewF5.md` (untracked, written ~40 min after the commit)
grades CQ-01…CQ-13. Every finding was re-verified against the source and every one is still present.
The two HIGHs are **both persistence items**:

| ID | Sev | Finding | Verified still present |
| --- | --- | --- | --- |
| CQ-01 | **HIGH** | `PersistedState.mine` grows unbounded across renewals; nothing removes entries | `rg "mine\.(remove\|retain)"` over all crates → **0 hits**; only `insert` at `core/src/claims.rs:261` |
| CQ-02 | **HIGH** | O(claims × records) full re-decode/re-scan per event | `core/src/model.rs:454` → `projection.rs:66,96` → `authority.rs:81-90` |
| CQ-03 | MED | completed/stale sync rounds never evicted | `core/src/sync.rs:316-327`; `rg "sync\.remove"` → 0 |
| CQ-06 | MED | whole-snapshot-per-event commit | `node-adapter/src/driver.rs:559` |

The remediation plan `glade-discover/dev-docs/GladeDiscoverCodeReviewPlan.md:3` reads
**`Status: ready for execution`** and `:191-197` states CQ-01/02/03/06 *"MUST NOT close as
documentation-only, benchmark-only, or deferred work."* A clean source tree 7 weeks later means none
were executed.

Note that CQ-01, CQ-03 and CQ-06 are **the same three defects as the shipping node's**: unbounded
claim state, no eviction of finished work, whole-snapshot-per-event writes. The kernel independently
reproduced the node's persistence pathology.

**Requirement trace.** The brief expects MET/UNMET; there is no such vocabulary
(`rg -i "\b(unmet|partial|not met)\b" …/RequirementTrace.md` → 0 hits). It asserts blanket completion
(`:3-7`) and lists evidence paths; all ~30 named paths and all 24 primary scenario JSONs exist on
disk. **The honest criticism is scope, not truth: no row traces to a production consumer.** "Green"
means `cargo test` inside a workspace nobody imports.

**GDL-032.** Ratified 2026-07-07 (`dev-docs/DecisionLog.md:50`): layers are session placement (web
bootstrap) → service discovery (folds over shares) → node discovery (iroh), **never merged**;
`dev-docs/glade/GladeWorkspaceDirectory.md:230-246` warns that merging 2 into 3 *"recreates the
coordination-service problem"*. `glade-discover` sits cleanly in layer 2 (`rg -i iroh` across the
repo → **0 hits**), so it does not blur the layering. But **GDL-032 is never cited in the repo**
(`rg "GDL-032"` → 0 hits), and `glade-node` already occupies layer 2 with a different implementation.
**The live GDL-032 risk is duplication, not merging.**

**ggg-viz `s-discovery`.** Authored, registered, and the *default* scenario
(`ggg-viz/src/scenario/discovery.ts:14-16` — 30 steps, phases A–E; `index.ts:162`
`DEFAULT_SCENARIO_ID = DISCOVERY.id`; asserted at `suite.test.ts:80`). Gate statuses inside the trace:
3 × `designed`, 3 × `stub-allow-all`, **0 × `enforced`**. `ggg-viz` HEAD 2026-07-18, tree clean. The
trace cites the layering doc (`discovery.ts:3`); `glade-discover` does not.

---

## 6. The Garns boundary

### 6.1 What grazel actually adopted — REPRODUCED FACT: not the ratified Garns

`grazel` commit `a07af54` (2026-09-02) "Adopt Garns-generated P8 application store". Line 1 of the
generated artifact, `grazel/generated/p8_grazel_gryth_application_state/src/lib.rs:1`, reads
`//! GENERATED by garns v2 from operation_plans.json; do not edit.`

`garns v2` is the prototype at `/Users/owebeeone/limbo/datascad/garns-v2`, whose **first commit is
2026-09-01 — one day before the grazel commit**; its generator is `garns/inhabitant.py` and its
declaration a Python eDSL (`garns/corpus/p8.py:29`).

The ratified Garns at `/Users/owebeeone/limbo/garns-wz/garns` is a **different system**: version
`0.1.0a1` (`pyproject.toml:7`), initialized from ratified v9-5 on **2026-09-04 — two days after the
grazel commit**. `ls src/garns/` → 23 modules and **no `inhabitant.py`**; CLI is
`resolve`/`generate`/`ddl`/`execute` only; source language is `.garns` text over a frozen Lark LALR
grammar, not Python; Rust output is *"a parity instrument … not a bindings library, no crate"*
(`/Users/owebeeone/limbo/garns-wz/garns/LIMITATIONS.md:51`) whereas grazel holds a real `rusqlite`
crate with CAS and a migration ladder; storage bindings **must** be a hand-authored JSON document
with nothing guessed (`LIMITATIONS.md:48`) whereas the vendored crate derives names.

**There is no `.garns` source in the workspace.** The crate is checked in (`git ls-files generated/`
→ two files); there is no `build.rs`; regeneration is a manual ritual (`grazel/README.md:105-111`)
enforced by nothing.

### 6.2 How deep it goes — REPRODUCED FACT: structurally live, semantically inert

`grazel/src/main.rs:55` is `let _application_store = match open_application_store(...)` — the
`_` prefix is the deliberately-unused idiom, and `grep -n "_application_store" grazel/src/main.rs`
returns exactly two lines (the `use` and this binding). It is never read, queried, written, or moved.
Runtime behavior in full (`grazel/src/lib.rs:256-262`): open the SQLite file, run the migration
ladder, call `workspace_create_if_absent("ws-razel", "razel", now)`. That row duplicates the already
authoritative `grazel/apps/grazel-app.glade:53` (`workspace ws-razel razel`), which the node loads as
the runtime record (`glade/apps/grazel-app.glade:3-7`: *"The fold stays the only runtime authority"*).

Reach: of 9 generated functions, **7 have zero non-test callers**; of 10 tables, **6 have no generated
operation** — `workspace_doc_meta`, `environ_desktop`, `terminal_session`, `terminal_output_cache`,
`file_tree_entry`, `file_blob_cache`. Note what those six model: terminals and file trees — exactly
source classes (a) and (b), declared in a store nothing reads.

**Nothing crosses the glade wire.** `bootstrap_json` (`grazel/src/lib.rs:267-274`) emits
`{node_ws, mode, name}` from `Config` with zero store reads; no declared surface is backed by a Garns
table. The P8 declaration marks two queries `glade_visible = True`
(`/Users/owebeeone/limbo/datascad/garns-v2/garns/corpus/p8.py:302,365`) and **that flag is consumed by
nothing** — the ratified Garns's own "declared, validated, unconsumed" failure mode
(`/Users/owebeeone/limbo/garns-wz/garns/LIMITATIONS.md:52`).

**Doc vs code.** `grazel/generated/.../Cargo.toml:26-27` says *"This is the live application-state
store."* It is opened into `_` and never read. Code wins.

### 6.3 What the ratified Garns can and cannot hold

From `/Users/owebeeone/limbo/garns-wz/garns/LIMITATIONS.md` (35-row Inventory) and `README.md`:

| Capability | Ratified Garns |
| --- | --- |
| Storage bindings | explicit JSON, nothing guessed; 17 `STORAGE_*` refusal codes (`:48`) |
| `query` vs live `question` | questions must state `live bounded N`; footprints → `(carrier, field)` routing keys; `listener_scans == 0` by construction |
| Scope model | scope is a **resolved link path, not a column**; partition `scope:<identity>` or `global` (`ARCHITECTURE.md:155-241`) |
| Typed ledger | `Engine.transaction(writer, transaction_id)`; typed pre-effect validation; listeners fire after COMMIT |
| `external_captured` | declared changelog + 3 triggers per relation; coverage refused **twice** — bind-time `WRITER_CAPTURE_INCOMPLETE`, load-time `PRAGMA table_info` |
| Schema evolution | 18 event kinds (additive/deprecating/breaking); column/table-level migration only (`:38`) |
| **Runtime languages** | Python only. Rust output is a parity instrument (`:51`); **TypeScript is a deliberate refusal** (`:41`) |
| **Browser** | zero mentions of browser/wasm in 13 docs; structurally impossible (CPython + stdlib `sqlite3`) |
| **Concurrency** | *"Do not share one `Store` across threads or processes"* (`:160-161`); one connection, no pooling, no `SQLITE_BUSY` retry (`:63`) |
| **Streams / logs / CRDTs / blobs / p2p** | none. Blobs only as an `opaque`→`BLOB` type mapping; *"not a service host"* (`:49`) |
| **Subscriptions / indexes / auth** | subscriptions **cannot be cancelled**, unbounded retained state (`:60`); no `CREATE INDEX` anywhere (`:59`); capabilities caller-asserted, not authenticated (`:61`) |
| Glade adapter | `ARCHITECTURE.md:174-180`: *"No Rust runtime and no 'Glade' adapter exist in this repository."* `INTEGRATION.md:353-359`: *"PROPOSED — NOT IMPLEMENTED."* |

**INFERENCE — where the boundary falls, mechanically.** Garns is Python, single-writer,
single-process, SQLite-local, with no browser runtime and no replication. Glade is a p2p replicated
op-log across a Rust node and a TypeScript browser client. These do not overlap on the replicated
path *at all*. The state that could plausibly be a Garns world is the state that is (i) local to one
process, (ii) queried relationally, (iii) not replicated, and (iv) reachable from Python or Rust —
i.e. **authority-side supplier state**: the at-rest file index, terminal scrollback at rest,
workspace/desktop configuration. The state that must stay a taut fold is anything that crosses the
wire, converges between writers, or is read in the browser.

The `external_captured` model is the interesting piece the corpus has no counterpart for: it is a
principled answer to *"a source of truth I do not own writes into my store"* — which is precisely the
authority model of source classes (a) and (b). But it requires SQL triggers on the writer's own
tables, which a filesystem and a PTY do not have.

### 6.4 The decisions this forces (naming only)

Four, carried into §10 items 8–9: (1) may a supplier own a private relational store at all, or must
all supplier state be a declared surface; (2) if yes, which Garns — the ratified Python-only one, or a
Rust generator that does not exist; (3) are a `.glade` declaration and a Garns world declaration two
declarations of one thing, and if so which generates which; (4) are the six unread P8 tables a design
(a modelled gryth desktop) or debris.

---

## 7. Doc currency ledger

Dates from `git log -1 --date=short -- <path>`. Grades are mine, defended by the cited evidence.

| Doc | Last touched | Grade | Evidence |
| --- | --- | --- | --- |
| `dev-docs/GladeProgramStatus.md` | 2026-08-29 | **STALE — five wrong rows** | Self-declares "LIVING … Last update 2026-08-29" (`:4`). (1) `:31` lists F1 open — built (`claims.rs:1-4,124-131`, which names itself "the audit's F1"). (2) `:31` lists F2 as a pending "deferral/ruling" — built (`exchange.rs:145-152`). (3) `:22,32,118-120` call P4 CRDT "next" — built the same day the page was last edited (`GladeCrdtAdapter.md:3`, glade `71509c1`, 22:38). (4) `:49-51` lists GC-1…4 as a live design queue — GC-1 and GC-3 are ruled and built. (5) Repo-reconciliation §Actions 1 says the zones work is UNCOMMITTED; every member tree is clean (`git status --short` = 0 for all 10). Only F4 on its closure list is genuinely still open. |
| `dev-docs/DecisionLog.md` | 2026-08-29 | **STALE on one row** | GDL-039 (`:63`) states *"grip-core ShareDecl +domain/zone and glade client-ts changes are UNCOMMITTED — commit adjudication pending"*. `grip-core` HEAD 2026-07-10, tree clean. GDL-031…038, 040, 041 accurate. |
| `dev-docs/GladeE2EStage1Audit.md` | 2026-07-11 | **CURRENT as a dated record** | It is an audit snapshot, not a living page. Its F1/F2/F4 findings are still the best statement of what was open; F1 has since been built, F4 has not. |
| `dev-docs/StackMap.md` | 2026-07-12 | **STALE** | 8 weeks old; predates the SWMR/CRDT slices and the catalogue adoption. |
| `dev-docs/TautShapeCatalogAdoption.md` | 2026-08-29 | **CURRENT (mostly)** | Its capability boundary matches the code, including the honest "recognition ≠ capability" line (`:34`). `GSC-06`/`GSC-08` cite evidence that does not exist yet (§3.2). |
| `dev-docs/glade/GladeSystemDataSeam.md` | 2026-07-07 | **CURRENT in substance** | GDL-036's blob-now/SQLite-later seam is exactly what `glade/node/src/registry.rs:135-183` implements, including the `MemStore` conformance twin (`:167-181`). |
| `dev-docs/glade/GladeDeclSurface.md` | 2026-08-29 | **CURRENT** | GDL-037/038's "declarations are runtime records" holds: `glade/node/src/appdecl.rs` loads, diffs, and appends. |
| `dev-docs/glade/GladeWorkspaceDirectory.md` | 2026-07-07 | **ASPIRATIONAL on §7b** | The three-layer split is ratified (GDL-032) but layer 3 is switched off in code (`iroh_carrier.rs:31-38`) and layer 2 has two rival implementations. |
| `dev-docs/glial/GlialClientRuntime.md` | 2026-08-29 | **CURRENT** | "Persistence first" is real: `glade/demo/src/glial.ts:130` boots the binder on IndexedDB. |
| `glade/dev-docs/GladeSubstrateV1.md` | 2026-08-29 | **STALE — F4 still open** | §11's "Explicitly NOT in M-LIMP" list (`:322-324`) still names *"iroh multi-node, grazel authority session"* as not built; `:362-364` still says *"IndexedDB client destination deferred"*. All three are built (`glade/node/src/iroh_carrier.rs`, `exchange.rs:590-692`, `glial/src/store_idb.ts`). F4 was raised 2026-07-11; `git show 71509c1 -- …GladeSubstrateV1.md` touched §3/§4/GQ-3 only. Still true in that list: **MV folds** and **security enforcement**. |
| `glade/dev-docs/GladeShapeDispatch.md` | 2026-08-29 | **STALE on one row** | Claims `crdt` is an accepted binding declaration (`:30`); `glade/node/src/appdecl.rs:43-44` excludes it entirely (§3.1). The `term.log` retention follow-up (`:47-54`) is accurate and unactioned. |
| `glade/dev-docs/GladeSwmrAdapter.md` | 2026-08-29 | **CURRENT** | The adapter it specifies is `glade/wire-rs/src/swmr.rs` and is enforced pre-mutation at `store.rs:161-163`. |
| `glade/dev-docs/GladeCrdtAdapter.md` | 2026-08-29 | **CURRENT** | The `crdt`/`text_crdt` path is live in the demo. |
| `glade/dev-docs/GladePeerSyncNotes.md` | 2026-07-10 | **CURRENT with a stubbed core** | Sync mechanics match; §6's crypto is `verify_peer`→`Ok(_)` (`peer.rs:99-101`). |
| `glade/dev-docs/GladeTerminalSliceProposal.md` | **2026-06-13** | **SUPERSEDED** | Its whole premise is *"carried over libp2p"* (`:12-17`) and a `TerminalPty` live channel. `rg -uu` for `libp2p` across all `Cargo.toml` → **0 hits** (iroh replaced it); no PTY exists anywhere. Self-labelled *"proposal for pressure test, not yet a contract"*. |
| `dev-docs/glade/suppliers/glade-files.md` | (spec set) | **ASPIRATIONAL + MISLEADING** | 301 lines specifying `ws.files` blobs, viewport, backfill, `BlobRef {hash: BLAKE3}`; `ls glade-files` → No such file or directory. Worse, `:298-301` declares **"GAP-10 retention — RESOLVED (F-GAP10)"** with concrete numbers (1 GiB/ws, 4 GiB/node, LRU after 7d). Its own source is `RulingWorksheet.md` §IV, headed *"Files and Storage — **v1 recommendations**"* (`:380`), and `:179-181` states a `RECOMMENDATION` *"remains pending until accepted"*. `Plan.md:65` still lists the P3-gate as blocking. **An unaccepted recommendation is being cited downstream as a closed ruling on the single most important open persistence item.** |
| `dev-docs/glade/suppliers/glade-terminal.md` | (spec set) | **ASPIRATIONAL** | `ls glade-terminal` → No such file or directory. Of 14 supplier specs, only `glade-gwz` and `glade-chat` have code. |
| `dev-docs/EstateVision.md` + 2 reviews | 2026-08-30 (untracked) | **DRAFT, NO-GO ×2** | Both axis reviews returned **NO-GO** (`EstateVision-ReviewViability.md:8`, `EstateVision-ReviewConsistency.md:8`). Untracked, unrevised 6 days later. Not a constraint on persistence. |
| `glade-discover/dev-docs/*` | 2026-07-18 | **MISLEADING** | "P0–P7 complete" is true of an isolated artifact; the plan's own integration goal (`GladeDiscoverPlan.md:22`) is unmet and unflagged. `Decisions.md:188-190` ("Open decisions: None") is falsified by `GladeDiscoverCodeReviewPlan.md:29-82`, written the same day. |
| `grazel/README.md:93-99` + `generated/.../Cargo.toml:26-27` | 2026-09-02 | **WRONG** | *"This is the live application-state store"* — it is opened into `_` and never read (`grazel/src/main.rs:55`). |

**Testing the hypothesis in both directions.** The operator's hypothesis is that many docs will not
survive scrutiny. It half-holds. Of 20 significant docs graded: **6 CURRENT, 5 STALE (4 of them on
one row each), 1 SUPERSEDED, 3 ASPIRATIONAL, 2 MISLEADING, 1 WRONG, 2 draft/NO-GO**. The pattern is
not rot — it is **omission**: the docs that describe *built* mechanisms are largely accurate, and the
docs that describe *unbuilt* mechanisms are written in the same voice, so a reader cannot tell them
apart. That indistinguishability, not staleness, is the corpus defect.

---

## 8. Open items reclassified

| Item | Persistence? | What it actually means | Real state |
| --- | --- | --- | --- |
| **GAP-10 retention** | **YES — the central one** | Named as a "retention tail"; is in fact the total absence of a retention layer on **both** sides | **UNDESIGNED.** `retention` parsed at `appdecl.rs:138`, read by nothing. No compaction/TTL/cap in `store.rs` or `registry.rs`. Client `drop` retains (`store_idb.ts:95-99`); `purge` uncalled outside tests; `glial/dev-docs/DecisionLog.md:196-199` defers TTL/quota explicitly |
| **GAP-11 offline outbox** | **YES** | Locally-minted ops never re-ship on a later attach | **OPEN, honestly recorded** (`glial/dev-docs/DecisionLog.md:165-176`). *"local-only durable, never replicated."* Needs an unsent flag + re-mint under the session chain |
| **GC-1 event schema home** | Partly — it fixes the *at-rest record shape* | Where the change-event/delta envelope schema lives | **RULED + BUILT** 2026-07-07 (`dev-docs/glial/GlialClientRuntime.md:83`): generic `ChangeEvent` shell in glade-decl (`glade-decl/ir/glade_decl.taut.py:121,125`), per-shape deltas opaque. Residual GAP-5 interim payload encoding |
| **GC-2 conflation/backpressure** | **YES, indirectly** | No coalescing of rapid writes | **STUBBED, undesigned** (`glial/dev-docs/DecisionLog.md:37-42`: *"Not attempted in v0"*). Every keystroke on a `value` surface is a durable op — a *retention amplifier*, so a persistence item in effect |
| **GC-3 binder migration** | No | Per-binding cutover to the glial binder | **DONE, verifiable by deletion** — `glade/grip-share/package.json:6`: *"The binder … was deleted; bindings are glial mounts."* `glade/grip-share/src/` retains only `decl.ts`+`manifest.ts` |
| **GC-4 browser store engine** | Yes | IndexedDB behind the sync seam | **BUILT AND WIRED** (`glial/src/store_idb.ts:56`, used at `glade/demo/src/glial.ts:130`). The residual is the ephemeral DB key (§4.2), not the engine |
| **F1 live WorkspaceEntry/ServeClaim minting** | Yes — it is the growth source | No production path minted directory records | **BUILT** (`glade/node/src/claims.rs`, GLP-0006 P0.S2). Status page still lists it open (`GladeProgramStatus.md:31`). Closing it *created* the 10-second renewal-op pathology (§4.1) |
| **F2 s-create target routing** | No | Creation aimed at a named target with no claim yet | **BUILT** — `glade/node/src/exchange.rs:145-152` routes `workspace.create` by a target named in the payload; ceremony at `claims.rs:174-192`; E2E at `exchange.rs:451-460`. Status page still lists it as a pending "deferral/ruling" |
| **F4 SubstrateV1 §11 sweep** | No (doc hygiene) | The "not built" list is stale | **STILL OPEN 8 weeks on** — `GladeSubstrateV1.md:322-324` still lists built things; `:362-364` still says *"IndexedDB client destination deferred"*. `git show 71509c1 -- …GladeSubstrateV1.md` touched §3/§4/GQ-3 only, never §11 |
| **`--legacy-codec`** | **YES — a migration deadline** | Fail-open generated codec; dies at taut v0.10 | **LIVE, and narrower than the status page implies.** Exactly one file carries the banner: `glade/node/src/sysdata.rs:1`. `glade/wire-rs/src/generated.rs:1` does **not** — the wire types are already fail-closed. taut is at 0.9.1 (`taut/pyproject.toml:35`), so the deadline has not landed. Scope is one regen of the sysdata types |
| **GLP-0006 P3 blob strategy** | **YES — an explicitly OPEN gate** | iroh-blobs vs content-addressed store; *not* ops-in-chains | **UNRULED.** `Plan.md:65` lists P3-gate as blocking P3.S2. `Risks.md:15-17`: *"A 2GB file must never become chain ops."* The `RulingWorksheet` D6 answer sits under the heading *"IV. Files and Storage — **v1 recommendations**"* (`:380`), and `:179-181` says a `RECOMMENDATION` *"remains pending until accepted"* |
| **GLP-0006 glade-files** | **YES — the biggest undesigned surface** | Path-addressed viewport/backfill, blob refs, at-rest files | **SPEC ONLY, 301 lines, no code.** Zero `glade-files` in any `Cargo.toml`/`package.json`; not a gwz member. `plan-docs/ActiveWork.md:7` lists it as a repo that does not exist. Demo shows `(path routing pending)` in the UI (`WorkspacePanel.tsx:96`) |
| **GLP-0006 P4 CRDT/text_crdt** | Partly — CRDT history growth | Glade transport + binder integration for CRDT | **BUILT 2026-08-29.** `glade/dev-docs/GladeCrdtAdapter.md:3`: *"Status: **Built 2026-08-29** — GLP-0006 P4.S1/P4.S2 vertical slice"*, `GCA-01`…`GCA-09` each citing a test. Four docs still say pending/next (`Checkpoints.md:13`, `State.md:22`, `GladeProgramStatus.md:22,32,118-120`). **Genuine remainder is a persistence item nobody tracks**: causal compaction / checkpoint acknowledgement (`GladeCrdtAdapter.md:50-56`) — CRDT ops carry a causal frontier in `Op.refs` (`GCA-02`) and accumulate in the same unbounded journal, and this is *not* filed under GAP-10 |
| **WD-1 root custody** | **NO — corrected** | Root-key custody & recovery (paper backup / recovery keys / social recovery) | Product call, unmade (`dev-docs/glade/GladeWorkspaceDirectory.md:267`). **Gates Phase 5 enforcement/root semantics ONLY** — `Plan.md:64`, `Risks.md:3-6`, `PlanGladeUsers.md:291-293` (*"the ONLY phase behind the wall"*). Not on the persistence critical path |
| **AZ-1 path-scoped grants v1** | No (enforcement, not storage) | Path/key-scoped read grants in v1, or a later caveat type | `GladeAuthzModel.md:330`. Phase 5. `glade-files.md:294-297` keeps *structure* now, enforcement stage-2 |
| **AZ-2 multi-user local nodes day-one** | No (but adjacent) | Several identities on one local node | `GladeAuthzModel.md:331`. Phase 5. Adjacent to §4.2's ephemeral store key, but does not gate it |
| **AZ-3 OIDC v1** | No (downstream) | OIDC/AD bridge in v1, or single-user + manual grants first | `GladeAuthzModel.md:332`. Phase 5. Would eventually introduce the first real auth state to persist (§2d) |

**Correction to a widespread framing.** WD-1/AZ-1/2/3 are repeatedly cited across the corpus as if
they gated the program. They gate `PlanGladeUsers.md` Phase 5 and nothing else — `Plan.md:64`
(*"P2 stage-2 enforcement/root semantics only … Phases 0–4 MAY proceed"*). This over-claim was
already filed as a finding by an independent reviewer: `dev-docs/EstateVision-ReviewConsistency.md:57`
notes the rationale *"tells the operator every program gate is blocked on WD-1/AZ-1..3 when, by the
document's own tests, three of five are not."* **The persistence critical path is GAP-10, GAP-11 and
the P3 blob gate — none of which sit behind WD-1/AZ.**

---

## 9. Claims I could not verify / refuted

**Held up under attack (recorded so the picture stays calibrated):**

1. **The substrate/zone/SWMR/CRDT model demonstrably runs.** Not merely claimed — the demo is up
   (`curl` → 200 on :5175; `glade-nod` PID 45158 on :9099) and its journal holds 159 ops across 17
   chains and 5 shares, including a `doc:codex-crdt-cursor` share with CRDT ops. Privacy-by-key holds
   structurally: private surfaces are keyed `self:{self}` (`manifest.ts:76`); chains never mix.
2. **GDL-036's storage seam is honest and well-built.** `StoreApi` has two real implementations,
   `BlobStore` and `MemStore` (`registry.rs:135-181`), so *"SQLite is a store engine, never the
   replication mechanism"* is structurally enforced, not merely asserted.
3. **The chain/equivocation model is real.** `op_hash` reproduces taut's Python oracle byte-for-byte
   (`glade/node/src/chain.rs:44-50`, asserted against a literal digest); equivocation proofs are
   persisted, not discarded (`store.rs:190-200`).
4. **Fail-closed shape validation happens before mutation.** `validate_surface_contract` runs first in
   `append` (`store.rs:137`), so a malformed SWMR payload or a second writer is refused before any
   journal write. The doc claims this; the code does it.
5. **`glade-discover` respects GDL-032's layering.** `rg -i "iroh"` across the repo → 0 hits; the
   crate is layer-2-pure. The claim survives; only its *relevance* is refuted (nothing consumes it).
6. **The gwz supplier's security posture is real.** `ALLOWED_VERBS = ["status","ls","diff"]`
   (`glade-gwz/src/exec.rs:31`), `--root` forced by the supplier (`:66-71`), scope-redirect flags
   denied (`:35-46`).

**Refuted:**

1. *"grazel adopted Garns"* — it vendored output from a 6-commit prototype (`garns v2`) begun one day
   earlier, and the ratified Garns was initialized two days **after** (§6.1).
2. *"This is the live application-state store"* (`grazel/generated/.../Cargo.toml:26-27`) — inert.
3. *`GladeShapeDispatch.md:30`* — `crdt` is not an accepted binding declaration (§3.1).
4. *`GladeProgramStatus.md` repo-reconciliation Action 1* — the zones work is committed; all trees clean.
5. *`glade-discover/dev-docs/Decisions.md:188-190`, "Open decisions: None"* — four CR-D decisions were
   open the same day.
6. *`GladeTerminalSliceProposal.md`'s libp2p premise* — no libp2p dependency exists anywhere.
7. *`GladeProgramStatus.md:31`'s three closure items (F1, F2, F4)* — F1 and F2 are built; only F4 stands.
8. *`glade-files.md:298-301`, "GAP-10 retention — RESOLVED"* — its source is an unaccepted
   recommendation (`RulingWorksheet.md:179-181,380`) and the gate it claims to close is still listed
   as blocking (`Plan.md:65`).
9. *`Plan.md:194`, GAP-11 "may already be fixed … verify before P1"* — it was not, and P1 shipped
   without the verification. `rg -in 'outbox|unsent|re-?mint'` across `glial/src`,
   `glade/client-ts/src`, `glade/client-rs/src` returns one hit, and it is a comment
   (`glade/client-rs/src/client.rs:244-245`).
10. *`plan-docs/ActiveWork.md:7`* lists `glade-files` among the plan's repos; no such repo exists.

**Could not verify (would need a build or a run, deliberately not done):**

- Whether `glade-discover` compiles today. Circumstantially strong (committed `Cargo.lock`, deps
  `sha2`/`serde` only, CI config present, no source changed since the gated commit) but **not run**.
- Whether any workspace test suite is currently green. Every "N tests green" figure here is quoted
  from a doc, not reproduced.
- The live zone-isolation behavior of the running demo. Inferred from the on-disk chain layout and the
  manifest, not from driving the UI (the brief forbids driving it).
- Real-world growth rates for a long-running booted node. §4.1's O(n²) claim is read off
  `claims.rs:257` + `registry.rs:155-163`, not measured — no long-lived booted instance exists
  (`~/.glade` is absent on this machine; the demo runs the legacy non-booted form,
  `glade/node/src/bin/glade-node.rs:5-11`).

---

## 10. Decisions this review forces

Decisions only. No architecture is proposed and none should be read in.

1. **Does a retention policy bind at the declaration, the fold, or the store?** Options: (a) the
   `.glade` `retention` token becomes executable and the node enforces it; (b) retention is a store-
   engine property invisible to declarations; (c) each supplier owns its own reclamation.
   *Evidence:* the token exists and is inert (`appdecl.rs:138` → no consumer); the client seam has the
   same hole from the other side (`glial/dev-docs/DecisionLog.md:196-199`). A concrete candidate policy
   already exists as an **unaccepted recommendation** (F-GAP10: `max_bytes`/`max_age`/pressure-priority
   /pin per retained class, 1 GiB/ws + 4 GiB/node, LRU after 7d, active windows and referenced blobs
   pinned, eviction never rewriting authoritative history —
   `dev-docs/glade/suppliers/glade-files.md:298-301`). **The first sub-decision is whether to accept it
   as a ruling**, because it is already being cited as one. Nothing can be built until this is chosen.

2. **Is an op-log with permanent retention the right substrate for high-volume ephemeral streams
   (terminal output), or does terminal output need a different durability class?** Options: (a) keep
   `log`, add retention; (b) adopt the canonical `stream` engine, which already has a contract and
   corpus but is refused by the node (`appdecl.rs:44`); (c) terminal output never becomes a durable
   op and lives only in a channel.
   *Evidence:* one durable chained op per output line (`glade-gwz/src/supplier.rs:233`), the
   ambiguous `windowed` token (`grazel-app.glade:25`), and `GladeShapeDispatch.md:47-54` already
   asking the owner to decide.

3. **Do terminal writes need acknowledgement?** Today the append result is discarded and a mid-run
   link drop truncates silently (`glade-gwz/src/supplier.rs:246`). Options: (a) accept best-effort and
   mark truncation in-band; (b) acknowledge and retry; (c) an outbox shared with GAP-11.

4. **What is the authority model for the filesystem — at-rest truth, live fold, or both?** This is
   `RulingWorksheet.md`'s D13 restated. Options: (a) `ws.files` is at-rest-only plus a
   "being edited by X" marker; (b) files serves the live fold when a session exists.
   *Evidence:* no producer exists at all (§2a), so this is a green-field ruling, not a migration.

5. **Does the workspace directory grow forever, or do lease renewals get a different at-rest
   representation?** Options: (a) claims stay ops and the registry gets compaction/checkpointing;
   (b) leases become non-durable liveness state outside the fold; (c) renewal cadence is decoupled
   from op minting.
   *Evidence:* `claims.rs:236-260` + `registry.rs:155-163` — one op and one whole-state rewrite every
   10 s per share, with O(n²) cumulative bytes and O(n) boot.

6. **Do CRDT chains get causal compaction / checkpointing, and who acknowledges a checkpoint?**
   CRDT ops carry a causal frontier in `Op.refs` (`GladeCrdtAdapter.md` `GCA-02`) and accumulate in the
   same unbounded journal as everything else; `GladeCrdtAdapter.md:50-56` names compaction and
   checkpoint acknowledgement as the built slice's remainder. **This item is not currently filed under
   GAP-10 or anywhere on the status page**, so it is invisible to the retention decision above unless
   named here. Options: (a) fold it into the general retention policy; (b) CRDT gets its own
   checkpoint contract because a truncated causal history is not recoverable the way a truncated log is.

7. **Is the identity that keys client persistence allowed to be ephemeral?** Today the IndexedDB
   database is named `glial:${Math.random()}` (`glial.ts:70,130`), so closed tabs orphan their
   databases permanently. Options: (a) per-tab identity keeps a per-profile store; (b) persistence
   keys on a durable profile identity and per-tab origin becomes a chain-level concept only;
   (c) accept the leak and add an eviction sweep. Interacts with **AZ-2**.

8. **May an authority-side supplier own a private relational store, or must all supplier state be a
   declared surface?** *Evidence:* the P8 store is the first instance of this and it is currently
   inert (§6.2); six of its ten tables model terminals and file trees.

9. **If yes to 8 — which Garns, and does it generate the `.glade` declaration or vice versa?**
   *Evidence:* grazel holds `garns v2` output while the ratified Garns is Python-only, refuses
   TypeScript, has no Rust crate, no browser runtime, no concurrency model, and states plainly that
   no Glade adapter exists (`/Users/owebeeone/limbo/garns-wz/garns/ARCHITECTURE.md:174-180`,
   `INTEGRATION.md:353-359`).

10. **Does `glade-discover` get integrated, superseded, or retired?** Options: (a) integrate — needs a
   `Transport` implementor, a `glade-wire`↔`glade-discover-protocol` bridge, and CQ-01…CQ-13
   remediation; (b) declare `glade-node`'s own claim/route code the layer-2 implementation and retire
   the kernel; (c) leave both and accept the duplication.
   *Evidence:* zero call paths in either direction; 13 unremediated findings (2 HIGH, both persistence);
   one commit in 49 days; `glade-node` already passes the s-discovery E2E on its own code
   (`mesh.rs:621-630`). **Note that (c) is the status quo and is itself the GDL-032 risk.**

11. **Is node discovery in scope at all before auth exists?** Discovery is switched off
    (`iroh_carrier.rs:31-38`) and authentication is `Ok(_)` (`peer.rs:99-101`). Turning discovery on
    without authentication makes any reachable peer a trusted origin. Options: (a) auth first;
    (b) discovery first behind an allow-list; (c) both remain deferred and the localhost posture is
    written down as the supported one.

12. **Which of the fourteen supplier specs are commitments and which are sketches?**
    *Evidence:* 12 of 14 have no repo. They are written in the same normative voice as the two that
    are built, which is the corpus defect named in §7.

13. **What is the migration story when retention lands?** Existing journals contain ops written under
    "keep everything". Options: (a) reclaim retroactively; (b) grandfather existing chains;
    (c) treat the current stores as disposable. *Evidence:* the live demo store already holds ops from
    2026-07-12; `glade/node/target/demo-store` is inside `target/` and a `cargo clean` deletes it, so
    the *current* answer is (c) by accident rather than by decision.

---

## 11. Contradictions with ratified ground

Three. The first is the one that matters.

### 11.1 GDL-041 (ratified 2026-08-28) — `crdt` is documented as declarable and is not

GDL-041 adopts the catalogue and requires (via `GSC-01`/`GSC-04`) that dispatch go only to exact
implemented adapters and that unsupported paths fail closed. The **capability record that the ratified
decision points to** — `glade/dev-docs/GladeShapeDispatch.md:30` — states `crdt` is *"Accepted and
registered"* at binding declaration. `glade/node/src/appdecl.rs:43-44` excludes `crdt` from both
`KNOWN_SHAPES` and `BINDING_SHAPES`, so a `.glade` file declaring it is refused as an *unknown* shape.

The failure is **fail-closed**, so no unsafe path is open — this is a documentation defect, not a
runtime one. But the ratified decision's own evidence document misstates the built capability, which
means a reader cannot trust the capability table to say what is dispatchable.

### 11.2 GDL-041's `GSC-08` (ratified) — retention is separate from shape *because nothing enforces it*

`GSC-08` requires that *"Retention MUST remain separate from shape and view identity. Ambiguous
policies MUST be resolved explicitly, not inferred."* The requirement is nominally satisfied and
substantively void: the retention token reaches the registry and the wire and is read by no code
(§3.3). The `term.log` `windowed` token that `GladeShapeDispatch.md:47-54` says *"the terminal owner
must choose"* has been unresolved since the catalogue was ratified, while terminal output continues
to be written durably at line granularity. **A ratified MUST with no enforcement point is not a
constraint.**

Worse, the corpus has begun to route *around* the requirement rather than satisfy it.
`dev-docs/glade/suppliers/glade-files.md:298-301` declares **"GAP-10 retention — RESOLVED (F-GAP10)"**
and states specific bounds. Those bounds are an unaccepted `RECOMMENDATION` under
`RulingWorksheet.md:380` (*"v1 recommendations"*), which `:179-181` explicitly says *"remains pending
until accepted"* — while `Plan.md:65` still lists the P3-gate as blocking. This is the exact failure
mode `GSC-08` prohibits: an ambiguous policy being treated as resolved by inference downstream. It is
not a contradiction with ratified *ground* (the worksheet is not ratified), but it is a contradiction
with a ratified *requirement*, and it is why §7 grades that doc MISLEADING.

### 11.3 GDL-032 (ratified 2026-07-07) — the layering is preserved by duplication, not by design

GDL-032 fixes three layers, *never merged*: session placement → service discovery → node discovery.
The layers are not merged, so the letter holds. But:

- **Layer 3 (node discovery, iroh) is switched off in the shipping node** — `presets::Minimal`,
  localhost only, manual `--peer` (`glade/node/src/iroh_carrier.rs:31-38`,
  `glade/node/src/bin/glade-node.rs:43-47`).
- **Layer 2 has two independent implementations**: `glade/node/src/{claims,registry,router,mesh}.rs`
  (shipping, E2E-tested) and `glade-discover/crates/glade-discover-core` (9,029 LOC, 211 tests, zero
  consumers). Neither references the other; neither cites GDL-032 (`rg "GDL-032"` inside
  `glade-discover` → 0 hits).

**INFERENCE.** GDL-032 is not violated, but the ratified decision has produced a second, orphaned
layer-2 engine carrying two unremediated HIGH persistence defects that duplicate the shipping node's
own. Any persistence design that assumes one directory implementation must first rule which one it is.

### Not a contradiction, but flag it

GDL-035 (ratified) makes glial *"local persistence FIRST (glade optional)"*. This holds — the demo
binds the binder to IndexedDB at `glade/demo/src/glial.ts:130`. But GAP-11 means an offline write is
*"local-only durable, never replicated"* (`glial/dev-docs/DecisionLog.md:175-176`). "Persistence
first" is therefore currently realized as "persistence *only*" for anything written offline. That is
a known, recorded gap rather than a contradiction — but it is the gap most likely to lose a user's
work, and it should not be filed under design-owned/non-blocking.
