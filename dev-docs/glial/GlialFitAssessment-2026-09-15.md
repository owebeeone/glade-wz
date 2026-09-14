# Glial Fit Assessment — does the client kernel still fit the glade demo? (2026-09-15)

**Question, as asked.** "The design has moved a long way since last time glial was worked on;
does it fit where the demo glade work is at?"

**Answer, in one line.** It fits where it touches the running demo, and the drift is real but
almost entirely *beneath* and *beside* glial rather than across its seam: the September design
movement (GDL-042..049) is Rust host-side and contract-first, the wire has not changed since
2026-07-12, and the two things that would actually bite the next step — persistence and the
offline outbox — are glial's own recorded gaps, not new design.

**Scope and method.** Read first: root `dev-docs/DecisionLog.md`, `dev-docs/glial/GlialClientRuntime.md`,
`dev-docs/glade/GladeDeclSurface.md`, `glial/README.md`, `glial/dev-docs/DecisionLog.md` (GAP-1..14),
`dev-docs/glade/GladeWorkspaceDirectory.md` §7b, `glade/dev-docs/GladeZones.md`, `grazel/README.md`,
`glade-gyld/README.md`. Then code only where a document left a question open. No builds, no tests,
no writes outside this file. Glial's last substantive commits: `git -C glial log` — `0dfe4b9`
2026-08-29 (text CRDT), `da06989` 2026-08-29 (SWMR), `03578db` 2026-08-26, `ab9922c` 2026-08-22
(Taut catalogue); the binder/session/supplier kernel itself is 2026-07-10..12.

---

## 1. The seam — `glade-decl` vs the node's op and record shapes — **DRIFTED**

The wire half fits. `glade/wire-rs/src/generated.rs:189` still carries
`Op{share, glade_id, key, origin, seq, prev, lamport, refs, shape, payload}`, and the GQ-9 hybrid
causal refs (`GladeSubstrateV1.md` §2/GQ-9) are already modelled client-side: `glial/src/store.ts`
gives `StoredOp.refs?: StoredHead[]` — "Required for CRDT ops; optional on legacy folds."

The *declaration* half has drifted three ways.

- **Catalogue.** `glade-decl/ir/glade_decl.taut.py` still enumerates
  `Shape{value, log, message, stream, exchange, window, swmr, crdt}` — it kept `message` and `window`
  and never gained `atom`, against GDL-041 (root `DecisionLog.md`, ratified 2026-08-28). Glial's own
  `src/shapes.ts` *did* adopt GDL-041 exactly. Glial is ahead of the contract module it imports.
- **`domain` does not exist node-side.** `glade/node/ir/sysdata.taut.py:78` declares
  `BindingDecl{app, glade_id, shape, authority, zone, retention}` — all `STR`, no `domain` — and
  `glade/node/src/appdecl.rs:18` parses `binding <glade_id> <shape> <authority> <zone> <retention>`.
  `glade-decl`'s `BindingDecl` has `domain: DomainAnchor` as field 5. The app files the demo actually
  loads (`grazel/apps/grazel-app.glade`, `grazel/apps/gyld-app.glade`) therefore cannot express it.
  Vocabulary also diverges in the free-string fields: those files declare `retention windowed` and
  `from-cursor`, neither of which is in `glade-decl`'s `RetentionPolicy{latest, from_cursor, ttl}`.
- **Authorable shapes.** `appdecl.rs:43-44`: `KNOWN_SHAPES` omits `crdt` and `atom`;
  `BINDING_SHAPES = ["value","log","swmr"]`. A `crdt` surface glial can mount cannot be declared in
  an app file at all.

**The fill model (GAP-2, GAP-7) is now vestigial.** `Fill.domain` enters only the instance key
(`glial/src/instance.ts:41`, `gladeId\0domain\0zone\0key`). The wire `share` and zone `key` come from
the app-supplied `Route` on `SessionDestination`, not from the fill. Live proof:
`gryth-ui/packages/plugins/gwz/src/live.ts` declares `domain: 'document'` in its manifest, fills
`{domain: 'gwz', key: {param: GWZ_RUN_ID}}`, and hard-codes `share: 'ws-razel'` in the route;
`gryth-ui/packages/plugins/gyld/src/ops/surfaces.ts` does the same with `GYLD_DOMAIN = 'gyld'`;
chat fills `{domain: g.id, zone: 'commons'}`. So the zones mapping `GladeZones.md` states —
domain→`share`, zone→`key` — is performed by hand in each plugin, three times, and the decl's anchor
carries no weight. This is not a glial defect alone: `GladeZones.md` §Open still lists "Domain
anchoring" and "whether `domain`/`zone` replace `share`/`key` as the wire field names" as unresolved.

## 2. Transport — **FITS**, with one stale artefact

`git -C glade log -- client-ts wire-rs node/src/session.rs node/src/frame.rs` shows no
protocol-affecting commit after 2026-07-12 (`b21f56d` supplier seam hooks, `4c5241a`
`Hello.principal`); the August commits are the shape catalogue and SWMR/CRDT slices.
`glial/src/session.ts` (`SessionDestination`, `feedSession`, two-way `hydrate`) still matches
`glade/client-ts`.

Discovery layer 1 sits exactly where GDL-032 and `GladeWorkspaceDirectory.md` §7b put it, and
correctly *outside* glial: `gryth-ui/packages/glade/src/runtime.ts` fetches `/bootstrap.json`,
`bootstrap-util.ts` picks `node_ws`, `grazel/src/main.rs:334` serves it. Per-tab principal matches
GAP-9's residual ruling (`runtime.ts` `sessionStorage` origin + `pickPrincipal`).

**The one broken artefact.** `gryth-ui/packages/glade/src/glade.ir.json` is a hand-vendored copy of
`glade-wz/taut/corpus/glade.ir.json` and its wire `Shape` enum is stale — source
`{value:0, log:1, stream:2, swmr:3, crdt:4}`, vendored `{value:0, log:1, stream:2}`. gryth-ui can
neither encode nor decode an `swmr` or `crdt` op today. It does not bite because every live mount is
`value`/`log`, and the file's own header says "refresh if the glade wire protocol ever changes (it is
frozen today)" — it changed on 2026-08-22/29 and the copy did not follow.

## 3. Taut — **FITS**

`glial/package.json` pins `@owebeeone/taut-shape: ^0.9.1`; `taut-shape-ts` is at 0.9.2 — in range.
`glial/src/shapes.ts` is the GDL-041 catalogue verbatim (`value`, `atom`, `log`, `stream`, `swmr`,
`crdt` plus the `text_crdt` profile; `exchange` split off to `requireExchangeShape`; fail-closed
`UnsupportedShapeError`), and `requireMountShapeAdapter` admits `value|log|swmr|crdt` — the same set
`glade/client-ts/src/shapes.ts:requireOpShape` admits. The Rust suppliers publish `value`/`log` only.
Where the catalogue is unreconciled it is in `glade-decl` and `appdecl.rs` (item 1), not in glial.

## 4. Supplier kit — **DRIFTED** (contract still matches; nothing live uses it)

`glial/src/supplier/index.ts` (`serveExchange` = the attach ceremony; `serveShare` = op-publishing
with a `set`/`append` controller; reattach-on-drop over an injectable clock — GAP-12/GAP-13) is
mirrored one-to-one on the Rust side: `glade/client-rs/src/supplier.rs`
(`Supplier::serve_exchange`, `serve_share`, `ShareController`, the same shape gating), and
`glade/client-rs/src/lib.rs:11-12` says so in as many words. The contract holds.

What has drifted is usage. The only consumer of the TS kit is `glade-wz/glade-chat/src/supplier.ts`
and its tests; nothing in gryth-ui imports it, and the chat plugin explicitly runs stage-1 with the
supplier *out* of the message path (`gryth-ui/packages/plugins/chat/src/live.ts:6`). The two
suppliers grazel actually spawns are Rust (`glade-gwz`, `glade-gyld` — `grazel/README.md`).
GAP-13's caveat list ("inbound `ExchangeReq` decoded but dropped; no `respondExchange`; `onOps` is a
single settable field") is stale: all three landed in `glade` `b21f56d`, 2026-07-12.

## 5. Persistence — **DRIFTED** (regressed in the consumer; the layout gap is GAP-11)

- **GAP-10 is built but not lit.** `glial/src/store_idb.ts` exists; the live app runs
  `new GlialBinder(new MemoryStoreEngine(), origin)` (`gryth-ui/packages/glade/src/runtime.ts`,
  in-file note: IndexedDB "a later upgrade"). The July demo *did* have it (`glade` `f63652f`,
  2026-07-11). So GAP-9 reload-resume is half-live: `feedSession` and `hydrate` are wired, but with a
  memory engine there is nothing persisted to hydrate from.
- **GAP-11 is the load-bearing gap for the owner's intent.** "Local-only first, node-bound later" is
  precisely the path GAP-11 says loses data: a write with no destination is minted by `mintLocal`
  with no wire address and under a chain scheme that is not the session's, and nothing re-ships it
  when `attachGlade` later lights connectivity. Binding the layout to a node after local use would
  silently drop every local-only edit.
- **Retention is not enforced.** GAP-10's closing paragraph: the engine is keyed by `instanceKey` and
  never sees the decl. `glial/src/folds/value.ts` folds an LWW register over the *whole* retained op
  list, and `IdbInstanceStore` retains rows past `drop`. A desktop layout appending an op per drag
  grows an unbounded IndexedDB log with no compaction. The layout is the first workload where
  `retention: latest` has to mean something.
- **The private zone key is hand-built.** The node routes on the `key` bytes and uses `self:<id>` for
  private zones (`glade/node/src/router.rs:133`, `glade/node/src/store.rs:409`). `Fill.zone` never
  becomes that key. A per-user layout surface is exactly a private zone, so this lands on the path.
- What the desktop document is, for sizing: `gryth-ui/packages/desktop/src/grips.desktop.ts` calls it
  "the serializable class-1 atom map that persists and roams", currently `createAtomValueTap`s with
  in-memory initials and no `localStorage` anywhere in the package. It also already rules the
  granularity: "Split to per-window atoms when replication needs per-window LWW — not before."

## 6. Discovery and grants — **DRIFTED** (absent; only the July placement seam is assumed)

Glial has no discovery vocabulary at all — no match for "discover" in `glial/src`, `glial/README.md`
or `glial/dev-docs`. Layer 1 lives in `runtime.ts` (correct). Layers 2 and 3 never reach the client:
every share and glade id is a literal in the plugins (`GYLD_SHARE = 'ws-razel'`,
`GWZ_SHARE = 'ws-razel'`), and nothing folds `ServeClaim`/`ServiceDefinition` to learn who serves
what — which is layer 2 as `GladeWorkspaceDirectory.md` §7b defines it. Grants-as-data exists
node-side (`glade/node/ir/sysdata.taut.py:60` `CapabilityGrant`; seeds compiled at registration in
`appdecl.rs`) and has no client counterpart: glial has no grant type and no way to distinguish a
denied write from a slow one.

The reassuring half: the entries dated after glial's last substantive commit — GDL-042..046
(2026-09-05), GDL-047/048 (09-09), GDL-049 (09-12) in root `DecisionLog.md` — are the library
boundary policy, seven draft host/registry contract crates in `glade-discover`, the
`glade/contracts` application-side workspace, Shaku DI and sdax-rs. All Rust host-side,
all explicitly "draft contracts, no production cutover". None of them changes the client seam.

---

## Recommended order for the layout-persistence step, smallest first

| # | Step | Why here |
| --- | --- | --- |
| S1 | Refresh `gryth-ui/packages/glade/src/glade.ir.json` from `glade-wz/taut/corpus/glade.ir.json` and add a drift check | One file, currently wrong, removes a silent decode failure before anything else is built on the session |
| S2 | Light `IndexedDbStoreEngine` in `runtime.ts` behind an async boot step, memory as the fallback when `open()` rejects | The demo already did this (`glade` `f63652f`); without it "persistence first" is not true in the app that matters |
| S3 | Mount the desktop document as a **local-only** `value` instance — manifest surface `domain: account`, `zone: private`, `retention: latest`, mounted with **no** `gladeFor` | The whole point of GDL-035 rule 1; one surface for the document, not per-window, per that file's own note |
| S4 | Honour `retention.policy == latest` in the store seam (truncate superseded ops at append) — GAP-10's deferred slice | Without it S3 grows an unbounded IDB log on a drag-heavy surface |
| S5 | Close GAP-11 (outbox): mark locally minted ops unsent, re-mint them through the session at attach | This is what makes "local-only first, node-bound later" a promise rather than a data-loss path. Do it **before** binding the layout to a node, not after |
| S6 | A glial-owned `(DomainAnchor, ZoneKind, principal) → (share, key)` mapping including the node's `self:<id>` convention | Retires the hand-rolled mapping in three plugins and makes item 1's fill model mean something again |

## What can be skipped

- **Reconciling `glade-decl`'s `Shape` enum with GDL-041, and `appdecl.rs`'s `KNOWN_SHAPES`/retention
  vocabulary.** Worth doing on its own schedule; a `value` surface is in every catalogue on both sides,
  so it does not block the layout.
- **Discovery layer 2 and grants-as-data.** A local-only layout needs neither, and a node-bound one
  still lands on a hard-coded share. Revisit when a second node or a second workspace exists.
- **The TS supplier kit.** Nothing in the layout path serves. Leave it as the Rust suppliers' mirror
  and delete GAP-13's stale caveat list when someone next touches it.
- **GC-2 / GAP-3 backpressure.** A layout write is a value write and the LWW register conflates
  naturally. Only revisit if the drag path writes per animation frame.
- **SWMR or CRDT for the layout.** The desktop document is an LWW map by its own design comment;
  reaching for a CRDT here buys nothing and costs the `crdtProfile` mount contract.
