# Glial appearance settings — the plan for theme, scale and font scale per user and app

Plan, 2026-09-24, for the owner. It answers the ask of 2026-09-24: the Gyld desk's
appearance follows the user within an app, the same in every tab and session of that
user, across reloads and restarts. Two rulings of that day bound it: the principal
comes via `gyld-ui.py`, and only appearance moves; the window and pane layout stays in
the interim `localStorage` blob.

Nothing here is built. The code was read in the working trees on 2026-09-24 with no git
command, so no revision is named. Other agents were editing `glade/node/` and three
documents in `glade/dev-docs/`, so line numbers there may move. gryth-wz was read, never
written.

**Paths** are from the glade-wz root, except those under `gryth-wz/`, the sibling
workspace; `gryth-ui/` below is short for `gryth-wz/gryth-ui/`. **Short names:** "CW n.m"
is a step of `glade/dev-docs/GladeClientWritesPlan.md`; "X n.m" and "W n" are a step and
a rule of `glade/dev-docs/GladeCrossNodeWritesPlan.md`; "slice 4.3" is Step 4.3 of
`dev-docs/GladeFirstSlicePlan.md`; "S1–S6" is the order in
`dev-docs/glial/GlialFitAssessment-2026-09-15.md:145-154`; "the blob" is the interim desk
document (`gryth-ui/packages/desktop/src/layoutStorageTap.ts`, `layoutDocument.ts`).

## Summary

- **Where it lives.** One `value` on the desk's own node: share `ws-razel`, glade id
  `gyld.appearance`, zone `private`, key `self:<principal>`, retention `latest`, declared
  in `grazel/apps/gyld-app.glade`. Not `account:<principal>`: that share is served today,
  but nobody claims it, so it would fork across nodes and need grants no shipped seed can
  name. The desk's node claims `ws-razel`, so it crosses nodes once the cross-node plan's
  X3 lands, and reads under slice 4.3 with the grant the desk needs anyway (§1).
- **What it holds.** Theme, UI zoom and font scale, and (recommended) the wallpaper and
  its theme toggle: the desk's five appearance grips. Their consumers do not change; only
  their producer does.
- **The principal.** `gyld-ui.py start --principal NAME` (default `owner`, recorded with
  the instance); grazel serves it in `/bootstrap.json`; the page resolves it before it
  composes. `?principal=` still overrides it, and the op origin stays per tab (§2).
- **Client work.** The instance is connected from its mount, so no outbox is needed. Of
  S1–S6 only S2 is used, narrowed to this one instance. Before the node answers, the page
  shows the last value this browser held for this user (§3).
- **Four phases, twelve steps, about 2,650 lines** with tests. Phases 1 and 2 deliver the
  ask; Phase 3 adds first paint and offline durability; Phase 4 waits on CW 3.3 and slice
  4.3 (§6). **Nine questions** (§5); rule 1 to 4 before Phase 1 starts.
- **Found on the way:** the principal is captured as modules load, so it must be known
  before the page composes; the blob, left seeding appearance, would write each browser's
  stale value to the node at every boot; the desk never notices a dropped socket; and
  settings follow the node's data directory, so dev and built mode keep two sets.

## 1. Where the appearance surface lives

### The share

What the desk's node does with a share nobody claims:

- grazel runs the node booted (`grazel/src/lib.rs:65-72`, `:240-255`), and a booted node
  enables its mesh (`glade/node/src/bin/glade-node.rs:198-208`), so `route_subscribe`
  takes its mesh branch (`glade/node/src/mesh.rs:121-142`):
  - a live claim held by this node: `Local` (`:131`); by a linked peer: `Forward`
    (`:132-135`); by an unlinked one: `Absent` (`:136`);
  - no live claim, but the directory knows the share (a workspace entry or any claim,
    `:533-549`): `Absent` (`:139`), answered `Heads{streams: []}` then `UnknownShare`
    (`glade/node/src/server.rs:237-246`);
  - a share the directory never heard of: `Local` (`:140`).
- A write consults no route. The `Ops` arm appends any op that is not on `home`
  (`server.rs:280-337`). W1, ruled but not built, makes writes follow the read route, and
  an unknown share stays `Local` under it.

So `account:<user>` is **not refused**: the desk's node serves it today as a plain local
share. Its costs lie elsewhere.

| | `ws-razel`, key `self:<p>` (recommended) | `account:<p>`, key `self:<p>` |
| --- | --- | --- |
| On the desk's node | served: both app files declare `workspace ws-razel razel` (`grazel/apps/grazel-app.glade:59`, `gyld-app.glade:72`), and the loading node claims it (`glade/docs/AppFileFormat.md:190-193`) | served, as an unknown share (`mesh.rs:140`) |
| On a second node | routes `Forward` to the holder; its writes cross once X3 lands (option A, ruled) | "unclaimed shares cross nothing" (`GladeCrossNodeWritesPlan.md:86-93`): each node keeps its own copy until a host claims `account:<p>` with a `workspace` line or `workspace.create` (`glade/node/src/exchange.rs:147-193`) |
| With slice 4.3's switch on | the read asks `read.subscribe` on `ws-razel`, which `seed owner ws-razel read.*` grants (`gyld-app.glade:63`): the grant every Gyld surface needs anyway | needs a grant on `account:<p>` for each principal; no shipped seed can name one, and 4.3 has the node warn on a seed whose share no loaded `workspace` declares (`GladeFirstSlicePlan.md:794-800`) |
| What it means | per user, per app, per workspace | per user, per app, in every workspace: the home `glade/dev-docs/GladeZones.md:39` gives app settings |

Recommend `ws-razel` now. The semantic cost is nil while the desk has one workspace, and
a later move to an account share is one constant plus a one-time copy, the same shape as
§3's migration. Question 1.

### The address

| Part | Value | Why |
| --- | --- | --- |
| glade id | `gyld.appearance`, that is `<entry>.appearance` | the entry name already keeps the two desks' blobs apart (`layoutStorageTap.ts:51-55`, `gryth-ui/entries/gyld/desk.ts:40`); the full desktop's would be `desktop.appearance` |
| shape | `value` | last writer wins over the whole appearance; the fold is glial's own (`glial/src/folds/value.ts:59-77`) |
| zone and key | `private`, `self:<principal>` | the node's convention (`glade/node/src/router.rs:129-137`). glial does not build it: "the mount's zone fill never reaches the wire" (`AppFileFormat.md:210-224`), so the route carries the key, built in one function |
| retention | `latest` | declarative: nothing enforces it (`AppFileFormat.md:282`) |
| fill | `{domain: 'gyld', zone: 'private', key: 'self:<p>'}` | enters only the instance key (`glial/src/instance.ts:40-42`) |

### The app file

Add `binding gyld.appearance value share private latest` to `grazel/apps/gyld-app.glade`,
beside the other Gyld surfaces (`:46-52`). The node does not require it: neither the
subscribe arm nor the `Ops` arm looks a glade id up (`server.rs:220-337`; the one lookup
is for exchanges, `:228-235`). Declare it anyway, as every Gyld surface is "pre-declared
here so they exist node-side" (`gyld-app.glade:22-24`). A node reads its app files when
it starts (`AppFileFormat.md:302`), so the line takes effect at the desk's next restart.

Privacy stays routing-only until Step 4.3. A private key keeps other subscribers out
(`router.rs:129-137`), but any session may subscribe to or write any key: a node test has
a session with no principal writing into `self:p` (`server.rs:514-548`). Since the
hardening, only loopback pages, and clients that send no `Origin`, reach the socket
(`glade/node/src/ws.rs:114-130`, `:152-168`).

## 2. The principal via gyld-ui

### What exists

- **The page.** A tab's principal is `?principal=` or `?user=`, else a per-tab
  `sessionStorage` id that is also the op origin (`gryth-ui/packages/glade/src/runtime.ts:38-55`,
  `bootstrap-util.ts:27-30`).
- **gyld-ui** starts grazel, then, in dev mode, `pnpm run dev:gyld --port <ui>
  --strictPort` (`gryth-ui/gyld-ui.py:1487-1505`, `:1579`); in built mode grazel serves
  `dist-gyld` itself (`:131-135`). It prints the URL as its last line (`:1262`,
  `:159-162`) and opens nothing: no browser module is imported (`:38-55`). Its recorded
  options live per instance in `~/.gyld-ui/instances/<port>/gyld-ui.json` (`:84`,
  `:225-233`, `:241-265`), and `restart` replays them (`:1761-1801`).
- **`/bootstrap.json`** is grazel's, not gyld-ui's (`grazel/src/main.rs:157-158`,
  `:334-337`), proxied by Vite in dev mode (`gryth-ui/vite.config.ts:285-298`). It
  carries `{node_ws, mode, name}` (`grazel/src/lib.rs:379-388`); `name` is the node's
  instance name (`gyld-ui.py:71-72`), not a user.

### How the principal reaches the page

Recommend the served bootstrap, with the URL as the override:

1. `gyld-ui.py start --principal NAME` records `principal` in `gyld-ui.json` and passes
   `--principal NAME` to grazel.
2. grazel adds `"principal": NAME` to `/bootstrap.json`. That body is the GDL-032
   placement seam, and it is where slice 4.3's note would put the principal and, later, a
   per-start token (`glade/dev-docs/GladeNodeAssembly.md:1114-1118`, `:1404-1405`).
3. The page takes `?principal=` or `?user=`, else the bootstrap's principal, else the
   per-tab id.

The channels not chosen:

- **The URL alone** (`?principal=` in the printed URL) needs no grazel change, but a tab
  opened by hand at `http://localhost:5173/` would be a stranger with default settings.
- **A Vite `define`** works in dev mode only. Built mode would bake the name in at `pnpm
  build:gyld`, which gyld-ui runs only when `dist-gyld` is missing or `--build` is given
  (`gyld-ui.py:1465-1480`).

**It must be known before the page composes.** Two sites capture the principal when
their module loads or registers: `runtime.ts:53-55`, a `const` at import, and gyld's
operations handle (`gryth-ui/packages/plugins/gyld/src/live.ts:252-254`, `:287`). The
Hello (`runtime.ts:163`) comes late enough. A principal that arrived with `startGlade`'s
fetch (`runtime.ts:141-149`) would miss the first two. So each entry first awaits the
bootstrap, bounded by a timeout, then imports its composition, and `startGlade` reuses
the answer.

**A tab opened by hand** gets the same principal, because it fetches the same
`/bootstrap.json`. Only a page with no grazel behind it (`pnpm dev:gyld` alone, where the
proxy errors, `vite.config.ts:291-294`) or behind an older grazel falls back to the
per-tab id. Such a page keeps today's behaviour exactly, atoms and the blob, and its
settings window says so.

### The name

Recommend a `--principal` flag, default `owner`: validated as `[A-Za-z0-9._-]{1,63}`, so
never the 64 hex digits that name a node (`GladeFirstSlicePlan.md:810`); recorded per
instance; kept by `restart`; printed and checked by `status`. `owner` is the one principal
the shipped seeds grant (`gyld-app.glade:63`, `:66`; `grazel-app.glade:50-51`), and slice
4.3 ruled that "`owner` is the owner" (`GladeFirstSlicePlan.md:810`), so the desk keeps
its reads when 4.3's websocket switch goes on. `$USER` (`owebeeone` on this machine)
reads better in an attribution but holds no grant: with the switch on, every Gyld
subscribe would be refused (`GladeNodeAssembly.md:1225`, `:1279-1283`). Question 2.

### What changes for the per-tab participant (ruling of 2026-07-11)

The ruling has two halves (`glial/dev-docs/DecisionLog.md:158-163`; `runtime.ts:38-42`):
each tab is a distinct **participant**, and each tab has its own **origin**, because two
tabs sharing one origin fork a chain. On a gyld-ui desk:

| | Today | After |
| --- | --- | --- |
| Hello principal (`runtime.ts:163`, bound per session, `server.rs:210-217`) | one per tab | one per desk. The node binds every tab to it and mints one `dir.principals` record (`glade/node/src/claims.rs:223-229`), not one per tab (`GladeNodeAssembly.md:1263-1264`) |
| Chat (full desktop only) | a line is "mine" only in the tab that sent it | every tab's lines are "mine" (`gryth-ui/packages/plugins/chat/src/Chat.tsx:42`); a second participant needs `?principal=` |
| gwz and gyld requests (`plugins/gwz/src/live.ts:88-90`; `plugins/gyld/src/ops/ops.ts:280`, `:325`) | stamped per tab | one name; an ask reply says "attributed to owner" (`plugins/gyld/src/ask/reply.ts:552`) |
| Op origin (`runtime.ts:43-53`, `:59`) | per tab | **unchanged**, still per tab |
| The decide window's principal (`plugins/gyld/src/decide/DecideWindow.tsx:39-45`) | a field the reader fills | unchanged |

Question 4 asks to supersede the participant half on gyld-ui desks only. glade/demo and
`?principal=` keep it.

## 3. The client work

### S1–S6, trimmed to appearance

| | Needed? | Why |
| --- | --- | --- |
| S1, refresh the vendored IR | no | its `Shape` enum lacks `swmr` and `crdt` (`gryth-ui/packages/glade/src/glade.ir.json:121-126`), not `value` |
| S2, IndexedDB | narrowed, Phase 3 | an engine of its own for this one instance, not for the runtime's shared binder: that binder also holds every run's output log, and the IndexedDB engine keeps rows past the last unmount (`glial/src/store_idb.ts:96-100`) and preloads every row at open (`:67-85`). It buys first paint, a reloaded tab that resumes its chain before it writes, and an offline write that survives a reload |
| S3, a local-only mount | no, the opposite | a mount with no destination mints with `mintLocal` (`glial/src/instance.ts:147`, `:195-201`), and GAP-11 never ships those ops (`DecisionLog.md:165-175`). This instance gets its destination at mount, so every write is session-minted and wire-shaped from the first |
| S4, enforce `latest` | no | writes are clicks: tens on a day of fiddling, usually none. Node and client keep every op (`AppFileFormat.md:282`; `DecisionLog.md:197-199`); revisit past a few thousand ops in the zone |
| S5, the outbox | no | follows from S3's answer; §4 says what a disconnected write does |
| S6, a glial-owned mapping | no | one surface. Its key is built in one function, as chat, gwz and gyld build theirs (`GlialFitAssessment-2026-09-15.md:46-55`); S6 keeps its own schedule |

GC-2 backpressure is not needed either: a value conflates (`DecisionLog.md:37-42`).

### One value, five grips

- **The document** is `{v: 1, theme, zoom, fontScale, wallpaper, wallpaperThemed}` in
  glial's default JSON codec (`glial/src/grip/index.ts:66-69`). Decoding validates as the
  blob's reader does (`layoutDocument.ts:323-343`) and clamps to the settings window's
  ranges, zoom 0.7–1.5 and font scale 5–15 (`plugins/settings/src/Settings.tsx:33-41`). An
  unknown field is kept and written back, so an older tab cannot drop a newer field.
- **The producers.** One glial tap mounts the instance; a projection tap publishes the
  five value grips and five handle grips the desk already declares
  (`gryth-ui/packages/desktop/src/grips.desktop.ts:113-142`). The desk
  (`Desktop.tsx:117-121`) and the settings window (`Settings.tsx:17-26`) do not change
  (`gryth-ui/dev-docs/CodingRules.md:64`).
- **The handles** are `AtomTapHandle`-shaped (`gryth-wz/grip-core/src/core/atom_tap.ts:30-39`):
  `set` and `update` read the document, change one field and write the whole. glial's
  controller has no `update` (`glial/src/grip/index.ts:77-93`).
- **Last writer wins over the whole document.** Two tabs changing different fields within
  one round trip lose one change, which one person does not do. The desk's rule for its
  windows applies: split "when replication needs per-window LWW — not before"
  (`grips.desktop.ts:16-17`).
- **The wallpaper field** writes on every keystroke today (`Settings.tsx:74-80`). Its
  handle shows the text at once and writes after 400 ms of quiet, on the desk's injectable
  clock (`layoutStorageTap.ts:14-16`).
- **Wallpaper and its toggle join** (recommended): they are appearance in the grips file
  and the blob (`grips.desktop.ts:134-142`, `layoutDocument.ts:60-66`). Question 5.

### Migration

- At registration, before the desk's persistence can rewrite the blob, the blob's
  `appearance` (`layoutDocument.ts:60-66`) is copied to a per-user key,
  `gryth.appearance.last.v1.<entry>.<principal>`, unless that key exists. The key then
  mirrors every value the zone takes, and is the placeholder below.
- Once the zone's replay is complete, an empty zone gets that value, once, and a marker
  stops it running again. Two browsers migrating at once both write; the last wins.
- **"Complete" needs care.** `subscribe()` resolves at the ack, before the replay's ops
  (`glade/client-ts/src/client.ts:113-114`, `:155-162`); CW 3.4 moves it after the replay.
  Until then a second subscribe of the zone is the barrier: its ack is queued behind the
  first replay, and a session's frames arrive in order (R4,
  `glade/dev-docs/GladeSubstrateV1.md:361-386`). Question 6.
- **The blob stops carrying appearance in the step that binds the surface.** Left on, its
  restore seeds through the handle (`layoutStorageTap.ts:159-172`, `:216-220`): a write to
  the node at every boot, with that browser's stale value. Its version stays 1: a document
  of another version is ignored whole (`layoutDocument.ts:23-25`), losing every stored
  layout, and a version-1 document without `appearance` already reads (`:323`).

### Before the node answers

The projection shows, in order: the instance's value (from Phase 3, preloaded from
IndexedDB before first paint), else the per-user key above, else the defaults. An
existing user sees their own theme at first paint, and the replay's value replaces it when
it lands. If the node never answers, the page keeps what it shows, and `startGlade` sets
the status to `offline` (`runtime.ts:170-172`).

## 4. The failure paths

**A write while disconnected.** The instance always has its destination, so the op is
minted by the session and appended to the instance (`glial/src/session.ts:100-104`;
`instance.ts:147-150`). GAP-11 does not arise. What the frame meets depends on the socket:

- **Not yet created:** `send` does nothing (`client.ts:234-236`), and `startGlade`
  re-ships the session's ops once it connects (`runtime.ts:167-168`). The write lands.
- **Connecting:** the browser's `WebSocket.send` throws, out of `bus.publish`
  (`runtime.ts:66-68`) and before the instance appends (`instance.ts:147-150`). The tab
  shows no change, and the op ships later in the dump. Step 1.3 makes the bus swallow it.
- **Closed after a drop:** the browser discards the frame, and nothing notices. The
  runtime registers no drop listener (`runtime.ts:156-174`, though the client offers one,
  `client.ts:89-93`, `:209-214`), the status stays `live`, and nothing reconnects. After
  Phase 2 the write is lost at the next reload. After Phase 3 it is in IndexedDB,
  rehydrated into the next session (`glial/src/session.ts:78-98`) and shipped by that
  boot's dump. Reconnecting is not in this plan (§8).

**A refused write.** The node answers every op (R1, built; `server.rs:280-337`), but
client-ts has no `Error` branch (`client.ts:97-136`), and glial keeps and folds the op
(`GladeClientWritesPlan.md:813-816`): the tab shows a value nobody else has. The likely
causes are an equivocation, from a reloaded tab writing seq 0 before its replay arrives
(`glade/client-ts/src/session.ts:34-43`), a shape conflict, and, after X4.1, a missing
`write.*` grant. Phase 3's chain resume removes the first. CW 3.3 puts refusals on the
console (`GladeClientWritesPlan.md:637-642`); Steps 4.1 and 4.2 take the op out of the
fold and say "not saved". Between Phase 3 and Step 4.1, a refused op persists in that
browser's IndexedDB, wins its fold and is refused again at each boot (question 7).

**A second node.** With `ws-razel`, a desk on node B subscribes `Forward` to the claim
holder A (`mesh.rs:132-135`) and reads A's value. Its writes stay on B today
(`GladeCrossNodeWritesPlan.md:59-63`), so the two disagree for good; after X3 they go
through A, and `UnknownShare` means "kept, sent again" (W5, built in CW 3.3). No step
here. Already true: two gyld-ui instances are two unlinked nodes, since grazel passes no
`--peer` (`grazel/src/lib.rs:240-255`); and the data directory follows `--port`
(`gyld-ui.py:1394-1398`), so dev on 5173 and built on 8080 are two stores, and two
browser origins. Each keeps its own settings.

**Grants, once slice 4.3's switch is on.** Reads ask `read.subscribe` on `ws-razel`,
granted to `owner` (`gyld-app.glade:63`). 4.3 checks no write
(`GladeClientWritesPlan.md:821-822`); X4.1 checks `write.append` at the client's node
behind the same switch, and no shipped seed grants a write verb
(`GladeCrossNodeWritesPlan.md:254-261`, ruled at `:267-273`). A share grant is also
share-wide: anyone granted `read.*` on `ws-razel` could read `self:owner`. Recommend the
own-zone rule, Step 4.3: under the switch, `self:P` is open to sessions bound to P with no
grant, and refused to all others — GladeZones' "`private` needs no grant" and "only that
self's sessions ever produce or subscribe to that key" (`GladeZones.md:52-56`, `:74`),
enforced. An agent writing a user's appearance, as environ delegation allows
(`gryth-wz/dev-docs/GrythVision.md:21-24`), would then need a grant naming the zone, and
none exists. Question 8.

## 5. Questions for the owner

1. **The share.** Recommend `ws-razel` with key `self:<principal>`: claimed, crosses
   nodes with X3, and reads under the desk's own grant. The alternative,
   `account:<principal>`, is the design's home for settings but needs a claim and
   per-principal grants before it roams or survives 4.3's switch (§1).
2. **The principal's name.** Recommend `--principal`, default `owner`, recorded per
   instance: the one principal the seeds grant. `$USER` reads better, but the desk goes
   dark when 4.3's switch goes on, unless a seed names it (§2).
3. **The channel.** Recommend grazel's `/bootstrap.json`, with `?principal=` as the
   override, resolved before the page composes. The alternatives are the URL alone, which
   leaves a hand-opened tab a stranger, and a Vite `define`, which is dev only (§2).
4. **The participant ruling of 2026-07-11.** Recommend superseding its participant half
   on gyld-ui desks, one principal per desk, and keeping its origin half, one origin per
   tab. glade/demo and `?principal=` keep a second participant (§2).
5. **What the value holds.** Recommend all five appearance grips in one value, last
   writer wins. The alternatives are the three named, or one value per setting (§3).
6. **The migration's barrier.** Recommend the ordered second subscribe until CW 3.4, then
   `subscribe()` itself. The alternative is to wait for CW 3.4 before Step 2.4 (§3).
7. **The local store.** Recommend Phase 3 after Phase 2, accepting that until Step 4.1 a
   refused write persists in that browser. The alternative is Phase 3 after 4.1 (§4).
8. **The own-zone rule.** Recommend Step 4.3: under 4.3's switch, `self:P` for P's
   sessions only, with no grant needed, and delegation later (§4).
9. **The full desktop.** Recommend later: it keeps its appearance in its blob. Now would
   add `desktop.appearance`, a line in both copies of `grazel-app.glade`, and one call in
   its composition.

**Ruled, owner, 2026-09-25 ("all recommended"):** 1 the surface lives on `ws-razel`; 2 the principal defaults to
`owner`; 3 it reaches the page through grazel's `/bootstrap.json`, with `?principal=` as the override; 4 on gyld-ui
desks the 2026-07-11 ruling's "each tab is a participant" half is superseded and its "each tab has its own origin"
half stands; 5 one value holds all five appearance settings, the last writer winning; 6 the migration subscribes
twice until client-writes 3.4 lands; 7 Phase 3 goes ahead now; 8 the node keeps each user's private zone to that
user; 9 the full desktop is wired later.

**Ruled, owner, 2026-09-27 ("all recommended"), who lands the gryth-wz steps:** agents build them in a separate
copy, the gwz local lane `appearance` (`/Users/owebeeone/limbo/gryth-wz-appearance`, a `gwz local clone` of gryth-wz
made that day at its locked commits, with gryth-ui on `glp-0006-p1s4-gryth-panels`). The lane owner verifies each step
and commits it there. The owner's gryth-wz, which his live desk runs from, is never written; his desk changes only
when he merges the lane, for example once per phase: `gwz --root ~/limbo/gryth-wz --target @all merge --remote
appearance`.

## 6. Phases and steps

**Rules for every step.**

- A failing test first (glade-wz `AGENTS.md`, rule 0). Braced control-flow bodies in the
  code a step writes or changes.
- glade-wz steps land through gwz, the member first and then the root lock, with no
  attribution trailer. gryth-wz steps are built and committed in the `appearance` lane, and reach the owner's
  gryth-wz only when he merges it (ruled 2026-09-27).
- In gryth-ui, no React state and timers only in taps (`gryth-ui/dev-docs/CodingRules.md:6-25`),
  and no enums or string tags for a concept (`gryth-ui/AGENTS.md:45-49`): the identity is
  an object.
- No step touches the running desk. Each live check is the owner's, after
  `python3 gyld-ui.py stop` and then `start` (`GladeFirstSlicePlan.md:832`).
- **Reaching the desk.** A grazel change needs `cargo build` in `grazel/`, since gyld-ui
  runs `grazel/target/debug/grazel` (`gyld-ui.py:966-967`). A glial change reaches
  gryth-ui only once the owner fast-forwards the `gryth-wz/glial` member it resolves
  (`gryth-ui/pnpm-workspace.yaml:10`, `:25-29`). A client-ts change needs `pnpm install`
  in gryth-ui (`GladeClientWritesPlan.md:129-137`).

**Gates, named once.**

- **ui:** in gryth-ui, `pnpm lint`, `pnpm test`, `pnpm build` and `pnpm build:gyld`; both
  targets, since the render and the entries are shared (`gryth-ui/AGENTS.md:115-116`).
- **py:** in gryth-ui, `pnpm test:py`, then `uvx ruff@0.13.0 check` and
  `uvx ruff@0.13.0 format --check` on `gyld-ui.py` and `scripts/gyld_ui_test.py`. Python
  3.10, stdlib only.
- **grazel:** `cargo test --offline --locked --manifest-path grazel/Cargo.toml`; its
  integration tests start the node.
- **glial:** `pnpm --dir glial test` and `pnpm --dir glial typecheck`.
- **node:** `glade/node/check.sh`, and the grazel, glade-gwz and glade-gyld suites at
  baseline.

### Phase 1 — The desk knows its user

Milestone: every tab of a gyld-ui desk presents one principal, the one gyld-ui names, and
the surface is declared.

**Step 1.1 — grazel serves the principal**

- **Repository:** glade-wz member `grazel`.
- **Files:** `grazel/src/lib.rs` (`USAGE`, `Config`, `Config::parse`, `:75-221`;
  `bootstrap_json`, `:379-388`), `grazel/src/main.rs:157-158`,
  `grazel/tests/integration.rs`, the run line in `grazel/README.md`.
- **Change:** an optional `--principal <NAME>`, validated as gyld-ui validates it.
  `/bootstrap.json` gains `"principal"` when one is given and is byte-identical when not.
- **Tests, red first:** the parse accepts `owner` and refuses 64 hex digits, `a b` and an
  empty value; `bootstrap_json` with a principal; the shape test (`lib.rs:754-759`)
  unchanged; `grazel_both_mode_serves_bootstrap_static_and_node`
  (`tests/integration.rs:95-137`), run with `--principal it-user`, asserts the field.
- **Proves:** grazel names the principal it was given. **Does not prove:** that a page
  uses it.
- **Gate:** grazel. **Done when:** the gate is green, and the body carries the principal
  given and no field otherwise.
- **Size:** ~50 production, ~70 test. **Depends on:** nothing.
- **Done, 2026-09-25,** grazel `5fc2598`: `--principal` is optional with no default in
  grazel, and holds names to 1 to 63 of `A-Z a-z 0-9 . _ -`, each refusal naming its rule.
  `/bootstrap.json` carries `"principal"` only when given and is byte-identical
  otherwise. grazel 29 + 3 (was 26 + 3), clippy at its baseline of one warning, in the
  generated crate. No CORS header: the page's fetch stays same-origin, through gryth-ui's
  Vite proxy in dev (`gryth-ui/vite.config.ts:295-298`).

**Step 1.2 — gyld-ui names, records and passes the principal**

- **Repository:** gryth-wz (`gryth-ui`).
- **Files:** `gryth-ui/gyld-ui.py`: `InstanceState` gains `principal = "owner"`
  (`:241-265`), a defaulted field so that older state files still load (`:270-284`);
  `Bootstrap` (`:538-570`); the bootstrap check in `status_checks` (`:1089-1116`); the
  grazel argv (`:1487-1505`); `restart_command` (`:1779-1798`); the parsers
  (`:1809-1912`). Also `scripts/gyld_ui_test.py`.
- **Change:** `start --principal NAME` (default `owner`) and `restart --principal`
  (default: the recorded one), validated and passed to grazel. `status` reports the
  principal and fails its bootstrap line when the body names another principal or none,
  as an older grazel binary would.
- **Tests, red first:** the parse and the validation; the state round trip, with and
  without the field; the argv carries it; `restart` keeps it; the status check against
  fake bootstrap bodies.
- **Proves:** an instance has one recorded principal and checks that grazel serves it.
  **Does not prove:** the page.
- **Gate:** py. **Done when:** the gate is green, and `start` against a grazel with 1.1
  prints the principal and passes `status`.
- **Size:** ~60 production, ~110 test. **Depends on:** 1.1's binary before it reaches the
  desk, since grazel refuses a flag it does not know (`grazel/src/lib.rs:197`). Its tests
  need nothing.

**Step 1.3 — The page resolves its identity before it composes**

- **Repository:** gryth-wz.
- **Files:**
  - a new `packages/glade/src/identity.ts`, DOM-free: a `DeskIdentity` object (principal,
    origin, and whether it roams), resolved once from the URL, a bounded fetch of
    `/bootstrap.json`, and the per-tab id moved from `runtime.ts:43-51`;
  - `bootstrap-util.ts`: `BootstrapJson.principal`, and `pickPrincipal` takes the
    bootstrap;
  - `runtime.ts`: the identity and the cached bootstrap (`:53-60`, `:141-149`), a bus
    that never throws (`:66-68`), and a `GLADE_IDENTITY` grip;
  - `packages/glade/package.json`: an `./identity` export;
  - the two entries become loaders: `entries/gyld/main.tsx` and `src/bootstrap.tsx` await
    the identity (1.5 s at most), then import a new `compose.tsx` holding today's bodies
    (`App.tsx` exists, so not `app.tsx`);
  - `src/boot.test.ts`, whose entry-shape test (`:38-47`) reads the compositions and
    checks the loaders; `bootstrap-util.test.ts`;
  - the per-tab comments (`runtime.ts:38-42`, `plugins/gyld/src/live.ts:26-35`).
- **Tests, red first:** the URL beats the bootstrap, which beats the per-tab id; a
  bootstrap without the field, a failed fetch and a hung one (fake clock) fall back to the
  per-tab id and do not roam; two tabs with one principal keep two origins; a send that
  throws does not escape the bus.
- **Proves:** every capture site sees the final principal. **Does not prove:** the Hello
  against a live node.
- **Gate:** ui. **Done when:** the gate is green on both targets.
- **Size:** ~110 production, ~140 test. **Depends on:** nothing; the tests fake the fetch.

**Step 1.4 — Declare the surface**

- **Repository:** glade-wz member `grazel`.
- **Files:** `grazel/apps/gyld-app.glade`: one `binding` line beside `:46-52`, with a
  comment that the client builds the key `self:<principal>`, since glial does not.
- **Tests:** `grazel_both_mode_composes_gyld_supplier_and_loads_a_second_app_file`
  (`grazel/tests/integration.rs:425-487`) loads the file, and a line the node refused
  would stop its start (`AppFileFormat.md:86-88`). Seen red with a misspelt shape first.
- **Proves:** the node accepts the declaration. **Does not prove:** anything at run time;
  the node needs no declaration to serve the zone (§1).
- **Gate:** grazel. **Done when:** the gate is green.
- **Size:** ~10 lines. **Depends on:** nothing. It shares its repository with 1.1: one
  agent at a time, or a worktree.
- **Correction, 2026-09-25:** this step touches the glade checkout too. The node's
  census test counts `gyld-app.glade`'s binding lines (`glade/node/tests/binding_census.rs`:
  gyld's 7 bindings, 14 live over grazel's two files, and the registration counts built on
  them), so the new line moves those counts by one. The step updates that test in the
  same change, runs the node gate as well as grazel's, and lands after the node lane's
  step in flight (4.2b) leaves the glade checkout: a glade commit and a grazel commit,
  locked together at the root.
- **Done, 2026-09-25,** grazel `c9f9c7f` and glade `63a5799`: `binding gyld.appearance value
  share private latest` beside the Gyld surfaces, with the comment on `self:<principal>`.
  Seen red first: with the shape misspelt `valeu`, the node refused line 59 and grazel's
  composition test failed; with the old counts, all five census tests failed. grazel
  29 + 3; node gate 8/8, 256 tests on both paths. The desk's node registers the line
  at its next restart.

### Phase 2 — Appearance follows the user

Milestone: on a gyld-ui desk, theme, UI zoom, font scale and wallpaper are one value in
the user's private zone on `ws-razel`. Every tab converges, a reload or a node restart
keeps the value, the blob keeps only the layout, and a user's old appearance moves across
once.

**Step 2.1 — The appearance surface, as data**

- **Repository:** gryth-wz.
- **Files:** a new `packages/plugins/settings/src/appearance.ts`, pure: the manifest per
  entry, `selfKey`, the document codec, and the five field handles over a controller;
  `appearance.test.ts`.
- **Tests, red first:** the round trip; clamping; a malformed field falls back to its
  default, an unreadable document to all defaults; an unknown field survives a write;
  `update` reads the latest value; a burst of wallpaper keystrokes is one write after
  400 ms (fake clock).
- **Proves:** the value's shape and the handles' meaning. **Does not prove:** any wiring.
- **Gate:** ui. **Done when:** the gate is green.
- **Size:** ~170 production, ~210 test. **Depends on:** nothing.

**Step 2.2 — The blob can leave appearance out**

- **Repository:** gryth-wz.
- **Files:** `packages/desktop/src/taps.desktop.ts` (`DesktopSetup.persistAppearance`,
  default true, `:282-303`; `startPersistence`, `:340-358`); `layoutStorageTap.ts`
  (`deskPorts` without the five appearance slots, `:184-194`, `:209-221`);
  `layoutDocument.ts` (appearance optional in `DeskState` and left out of the fold when
  absent, `:32-44`, `:69-88`; a `readLegacyAppearance` export); `index.ts`; their tests.
- **Tests, red first:** with `persistAppearance: false`, a stored appearance is not
  seeded and a write carries none; a version-1 document without `appearance` round-trips;
  the legacy reader returns a stored appearance; every existing test passes unchanged.
- **Proves:** the blob keeps the layout and leaves appearance alone when told to.
  **Does not prove:** that anything else holds appearance.
- **Gate:** ui. **Done when:** the gate is green, and the default leaves the full
  desktop's behaviour as it is.
- **Size:** ~60 production, ~90 test. **Depends on:** nothing.

**Step 2.3 — Bind the settings to the surface**

- **Repository:** gryth-wz.
- **Files:**
  - a new `packages/plugins/settings/src/surfaceTaps.ts`, DOM-free: registers the glial
    tap and the projection tap from injected parts (binder, destination factory,
    identity, entry, placeholder);
  - a new `live.ts`, the only settings file that imports `@grythjs/glade`, as gyld's
    `live.ts` is (`plugins/gyld/src/live.ts:26-35`): the `SessionDestination` on
    `ws-razel` with key `self:<principal>`, and the boot subscription with that key
    (`runtime.ts:127-135`);
  - `index.ts` stops registering producers at import (`:22`); each composition registers
    one set;
  - `package.json`: a `./live` export, and `@grythjs/glade` and `@owebeeone/glial-runtime`
    as `workspace:*`, as `plugins/gyld/package.json` has them;
  - `Settings.tsx`: the Desk note (`:81-86`) says appearance follows the user, names the
    user, and says the layout stays in this browser. "Reset layout" no longer resets
    appearance;
  - `entries/gyld/desk.ts` gains `gyldDesk(identity)`, whose `persistAppearance` is false
    when the identity roams; `entries/gyld/compose.tsx` registers the surface and boots
    that desk (`src/boot.test.ts` accepts the call); `src/compose.tsx` registers today's
    atoms explicitly.
- **Tests, red first:** two binders over an in-process bus with two client-ts sessions,
  glial's own harness (`glial/test/session.test.ts:1-60`), stand for two tabs: a theme set
  in one shows in the other, and an `update` in one sees the other's last write. An
  identity that does not roam registers today's atom taps. `desk.test.ts`: the roaming
  desk leaves appearance out of the blob, the other keeps it.
- **Proves:** the grips the desk reads carry the zone's value, across tabs. **Does not
  prove:** the node path end to end (the live check below), the migration, or first
  paint.
- **Gate:** ui, after `pnpm install` for the new workspace edges; no external package.
- **Done when:** the gate is green; and on the owner's desk, after `stop` and `start`, a
  theme picked in one tab shows in a second tab opened by hand, and survives a reload and
  a `gyld-ui.py restart`.
- **Size:** ~170 production, ~220 test. **Depends on:** 1.3, 2.1, 2.2; 1.1 and 1.2 for
  the live check. 1.4 is not needed at run time.

**Step 2.4 — Carry the old appearance across once**

- **Repository:** gryth-wz.
- **Files:** a new `packages/plugins/settings/src/migrate.ts`, pure: the per-user key,
  the placeholder and the once-only seed; `surfaceTaps.ts` (the placeholder until the
  replay); a new `packages/glade/src/replay.ts`, `whenReplayed(zone)` over an injected
  client, the second-subscribe barrier until CW 3.4; `live.ts`.
- **Tests, red first,** with fake storage and a fake zone: an empty zone gets one write
  of the old value; a zone that is not empty gets none; a second boot writes nothing; with
  no old value, nothing; the projection shows the placeholder until the replay, then the
  zone's value; `whenReplayed` resolves only after the first replay's ops were delivered
  (a fake client that delivers ack, gap, ack, in order).
- **Proves:** the once-only rule and the barrier's order. **Does not prove:** the barrier
  against a real node.
- **Gate:** ui. **Done when:** the gate is green; and a desk with a stored theme moves to
  the surface without its colours changing.
- **Size:** ~90 production, ~150 test. **Depends on:** 2.3. CW 3.4 retires the barrier.

### Phase 3 — Right before the node answers

Milestone: first paint shows what this browser last held for this user; a reloaded tab
resumes its chain before it writes; and a write made while the node is away survives a
reload and lands at the next boot.

**Step 3.1 — A local store for the appearance instance**

- **Repository:** gryth-wz.
- **Files:** a new `packages/plugins/settings/src/store.ts`, which opens
  `IndexedDbStoreEngine` (`glial/src/store_idb.ts:67-85`) as a database of its own, with
  memory on failure or after 1 s; `live.ts`, with its own `GlialBinder` over that engine,
  as `gryth-ui/src/taps.ts:27` keeps one for the workspace name, while the runtime's
  shared binder stays memory (`runtime.ts:85-88`); the entries' loaders, which open the
  store beside the identity; tests.
- **Tests, red first,** with an injected `StoreEngine`, since gryth-ui's tests run in
  node without IndexedDB (the engine is covered by `glial/test/store_idb.test.ts`): a
  second page over the same engine shows the stored value before any node op; its first
  write takes its origin's next seq, not 0; a write made with no socket is in the store
  and in the boot dump; a failed open falls back to memory and the per-user key.
- **Proves:** persistence first, for this instance. **Does not prove:** quota or
  eviction.
- **Gate:** ui. **Done when:** the gate is green; and with the node stopped, the desk
  paints the user's theme and keeps a change across a reload, and the change reaches the
  node at the first page load after the node is back.
- **Size:** ~90 production, ~160 test. **Depends on:** 2.3; 2.4 first is better.

### Phase 4 — Hardening that waits on other plans

Milestone: a refused appearance write is taken back and shown, and the private zone is
private under grants.

**Step 4.1 — glial takes back a refused own op**

- **Repository:** glade-wz member `glial`.
- **Files:** `glial/src/session.ts` (`SessionDestination` keeps the ops it sent by hash
  and takes the client's refusals); `instance.ts` (a refused own op leaves the store and
  the fold runs again, for the value and log shapes); `store.ts` and `store_idb.ts` (an
  optional remove of one op); tests.
- **Tests, red first:** a refused own op leaves the value fold and IndexedDB
  (fake-indexeddb is already a dev dependency, `glial/package.json`); a refusal naming
  another origin's op is ignored; a `Retention` answer removes nothing, since it is ruled
  settled (`GladeClientWritesPlan.md:303-305`).
- **Proves:** glial folds only what the node holds, for its own writes. **Does not
  prove:** SWMR or CRDT, which are not on this path.
- **Gate:** glial. **Done when:** the gate is green.
- **Size:** ~140 production, ~170 test. **Depends on:** CW 3.3, whose client reports
  refusals.

**Step 4.2 — The desk shows a refused appearance write**

- **Repository:** gryth-wz.
- **Files:** `runtime.ts` (the client's refusals reach glial's destinations);
  `surfaceTaps.ts` (a "not saved" grip); `Settings.tsx` (one line).
- **Tests, red first:** a refused write returns the projection to the zone's value and
  sets "not saved"; the next accepted write clears it.
- **Proves:** the reader is told. **Does not prove:** anything for other surfaces.
- **Gate:** ui, once the owner has fast-forwarded `gryth-wz/glial` and run `pnpm install`.
  **Done when:** the gate is green.
- **Size:** ~70 production, ~90 test. **Depends on:** 4.1 and CW 3.3.

**Step 4.3 — The node keeps `self:P` to P**

- **Repository:** glade-wz member `glade`, the node.
- **Files:** `glade/node/src/server.rs`, the subscribe and `Ops` arms (`:220-337`) at
  slice 4.3's check, and their tests.
- **Change:** under slice 4.3's websocket switch, a subscribe or an op on a key `self:Q`
  from a session not bound to Q is refused `Unauthorized`, in R6's form for a subscribe
  and R1's for an op; a session bound to P reads and writes `self:P` with no share grant.
  With the switch off, nothing changes.
- **Tests, red first:** a session bound to `bob` is refused `self:alice` with the switch
  on and served it with the switch off; `bob` is served `self:bob` with no grant; a session
  with no principal is refused every `self:` key with the switch on.
- **Proves:** private by key, enforced on one node. **Does not prove:** the forwarded
  path (W2), where X4.1 checks the holder.
- **Gate:** node. **Done when:** the gate is green.
- **Size:** ~50 production, ~170 test. **Depends on:** slice 4.3 part 2 (the switch), and
  question 8. One agent at a time in the glade checkout (`GladeFirstSlicePlan.md:934`).

## 7. Order and parallelism

```
 1.1 grazel ──── 1.2 gyld-ui (reaches the desk after 1.1)
 1.4 declare     (grazel too: after 1.1, or in a worktree)
 1.3 identity ─────────────────┐
 2.1 surface ──────────────────┼── 2.3 bind ── 2.4 migrate
 2.2 blob ─────────────────────┘        └───── 3.1 local store

 CW 3.3 ── 4.1 glial ── 4.2 desk         slice 4.3 part 2 ── 4.3 node
```

- **At once:** 1.1 in grazel; 1.2, 1.3, 2.1 and 2.2 in gryth-ui, which share no file, so
  agents in their own worktrees can take them together; 1.4 after 1.1. The critical path
  is 1.3, 2.3, 2.4.
- **Only 1.4 touches the glade checkout before 4.3,** to move the node's census counts
  (corrected 2026-09-25). It takes its turn after the node lane's step in flight; no
  other step contends with the node lane. Phases 1 to 3 depend on no other plan.
- **Phase 4 waits** on CW 3.3 and on slice 4.3's switch. X3 carries the zone across nodes
  with no step here.

## 8. What this plan does not do

- **The layout.** Windows, panes and the sidebar stay in the blob (ruling of
  2026-09-24).
- **Reconnect.** After a dropped socket the desk says `live` and stays silent until a
  reload, for every surface (`runtime.ts:156-174`).
- **The full desktop,** which keeps its appearance in its blob until question 9.
- **An account share,** or any claim or grant for one.
- **Retention.** Nothing trims the zone; S1, S4, S5 and S6 are not taken up.
- **Cross-node.** No step: X3 carries a claimed `value` zone.
- **Delegation.** No agent may write a user's appearance.
- **Per-device values.** A zoom set on a laptop applies on a large monitor too.
- **A reset-appearance control.** "Reset layout" stops resetting appearance, and nothing
  replaces that.
- **The rest.** The decide window's principal field, presence, glade/demo, the wire, the
  node's routing and the shipped seeds are untouched.
