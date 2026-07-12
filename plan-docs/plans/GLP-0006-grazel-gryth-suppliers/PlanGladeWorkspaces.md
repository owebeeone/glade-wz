# GLP-0006 — glade-workspaces build plan (+ the glade-gwz verb-set expansion)

Status: proposed (2026-07-12). Sub-plan of GLP-0006 (`Plan.md`); house style
per `Plan.md` / `SupplierRequirements.md` (trace → build → demo tab → live
verify; aspirational <500 hand-written LOC per step; repo-owned; parallel where
marked). Plans the supplier specified in
`dev-docs/glade/suppliers/glade-workspaces.md`; common contract
`dev-docs/glade/GladeSupplierModel.md`.

## Goal

Turn `glade-workspaces` from an outline into a live supplier: a **real
directory of hosted gwz workspaces** with liveness, a **client-context
selection** every tool supplier keys off, **member enumeration** from the gwz
manifest/lock, and **creation** — brand-new (empty) and clone-from-existing —
where the glade records and the on-disk gwz tree **commit-or-fail together on
the hosting peer**. In the same program it **expands `glade-gwz`** from its
stage-1 read-only allow-list to the **full gwz verb set, config-gated**, because
§2.3 of the spec makes the full set the product requirement. Landing gate is the
spec's user-testable-when line (§6), not green tests: *I see my real hosted
workspaces, pick one, it drives the tools, I create one and clone one across two
peers, and a member I add appears.*

North-star tie-in: the workspace directory + selection is the declarative
**operating-context** seam — consumers (gwz, files, terminal, razel) stay thin
projections that read one selected-workspace value; the host-mapping complexity
lives in the app-owned producer (grazel), never in a request.

## What exists to build on (the audited substrate for this supplier)

Verified in-tree 2026-07-12. This plan is additive over these:

- **The directory folds (built, node-served).** `dir.workspaces` +
  `dir.claims` are ordinary HOME-share folds
  (`glade/node/src/registry.rs`): `who_serves(share, now)` picks the live
  holder at the **reader's clock** (read-time expiry, highest live epoch,
  `registry.rs:342`); `replicas_of(share)` returns `eligible_hosts` (LWW,
  `registry.rs:351`). Records: `WorkspaceEntry{workspace, name, eligible_hosts}`,
  `ServeClaim{node, share, lease_expiry_ms, epoch}` (`sysdata.rs`). **A client
  reads the directory by subscribing these folds — no new supplier is needed for
  the read half.**
- **Live minting + renewal (built, F1).** `glade/node/src/claims.rs`:
  `serve_workspace_on` mints the `WorkspaceEntry` (diffed) + first `ServeClaim`
  (epoch = fold-max + 1, fencing), joins a **node-lifetime renewal loop**
  (`renew_leases`, cadence `RENEW_EVERY_MS`). `note_principal` is the P0.S7
  attribution precedent.
- **The create ceremony (built, records-only, node-reserved).**
  `workspace.create` is a **reserved built-in exchange id** the node intercepts
  *before* provider routing (`exchange.rs:98` → `handle_create`,
  `exchange.rs:155`): target == self → `claims::create_workspace` (mint under our
  origin); target == a linked peer → forward the frame 1:1; else fail-as-data.
  `claims::create_workspace` (`claims.rs:181`) mints the **glade records only** —
  its own doc-comment says *"gwz-core MATERIALIZATION (repos on disk) is
  deliberately EXTERNAL: grazel hooks it AROUND this ceremony."* That external
  hook is what §2.2 now asks us to make commit-or-fail; it does not exist yet.
- **glade-gwz (built, single-root, read-only).** `glade-gwz/src/exec.rs`:
  `ALLOWED_VERBS = ["status","ls","diff"]` (a `const`), `DENIED_ARGS` refuses
  scope/force levers, `argv()` prepends `--root <config root>` first. The
  supplier serves **one** `--root` for **one** `--share`
  (`glade-gwz/src/bin/glade-gwz.rs`); envelope `GwzRequest{verb,args,cwd?,
  stream?,principal?}` (`envelope.rs`), `cwd` parsed-but-ignored (root stays the
  app's).
- **grazel (built, one hardcoded workspace).** `grazel/src/lib.rs`: data layout
  `<data>/{sys,files,config}`; `files_dir() = <data>/files`;
  `gwz_supplier_argv()` hardcodes `--root <data>/files --share ws-razel
  --principal grazel`. **grazel spawns the node as a subprocess** and the gwz
  supplier as a child (Decisions P0.S6.6, P1.S3) — there is **no name→path map
  and no in-process node embedding today.** `grazel-app.glade` declares one
  `workspace ws-razel razel` line (dual-maintained in `grazel/apps/` and
  `glade/apps/`; the glade copy is a node-test fixture asserted by record count).
- **The demo chassis (built).** `glade/demo/src/tabs.tsx` — `TABS` registry +
  `CURRENT_TAB` grip atom (client-context, no wire) is the exact pattern for
  `ws.selection`. The **Gwz tab "picks its share" via a hardcoded constant**
  `GWZ_SHARE = "ws-razel"` (`glade/demo/src/gwz.ts:22`); every exchange call
  passes it. `WorkspacePanel` shows the atom-tap write pattern
  (`SELECTION`/`SELECTION_TAP`, `grips.ts`).
- **The gwz manifest/lock (the ws.members source).** `gwz.conf/gwz.yml`:
  `members[{id, path, type, source_id, active, desired.branch, remotes[{name,
  url, fetch, push}]}]`. `gwz.conf/gwz.lock.yml`: per member `{path, commit,
  branch, detached, dirty, materialized}`. **`ws.members` = the join** (manifest
  = declared name/path/remote/desired; lock = resolved commit/dirty/materialized).
- **glade-terminal does NOT exist.** The `forall` handoff is a documented seam
  this plan notes but does not build (see "The forall → terminal seam").

## The two attachment modes of glade-workspaces (frames the whole plan)

The spec (§4) lists glade-workspaces as one supplier owning directory +
selection + creation + materialization + enumeration. Against the built
substrate that surface splits by **attachment mode**, and the split is the
plan's organizing principle:

1. **Read / selection (client-side, wire-free).** The directory is already a
   node-served fold; selection is `CURRENT_TAB`-style client context. These need
   **client taps + a grip atom**, not a served supplier. Fully honors P00-a by
   needing nothing new node-side.
2. **Served ceremonies (the glade-workspaces *supplier* proper).** `ws.members`
   (host-co-located read of the gwz manifest/lock), the **materializer**, and the
   member ceremonies are wire-attached authority surfaces on the **hosting
   peer's** composition — a new member repo `glade-workspaces`, wire-attached per
   `GladeSupplierModel.md` §2 where it can be.

The one leg that resists a clean wire-attachment is **creation+materialization**
— its own section next.

## The materialization seam (the surfaced design decision)

**The constraint (§2.2):** *"the ceremony and the materialization commit-or-fail
together on the target; failure is data."* The target may be a remote peer
(clone-onboarding, §2.2/§5 s-ws-clone).

**Why it cannot be a plain wire-attached supplier** — the honest tension between
this leg and `GladeSupplierModel.md` §2:

- `workspace.create` is a **node-reserved built-in** that pre-empts provider
  routing (`exchange.rs:98`) — a wire supplier never receives it.
- The record mint is **node-internal and node-lifetime**: epoch fencing reads the
  served replica, and the `ServeClaim` must be **renewed by the node that hosts**
  (`claims.rs` renewal loop). A transient wire session cannot own a renewing
  claim — a one-shot appended claim lapses in one TTL.
- The **name→path mapping is app-owned** (§1.2, grazel config) and **must never
  ride the request** — so the node cannot resolve the path itself, and the
  requester cannot send it.
- grazel today **spawns the node as a subprocess**, so there is no in-process
  callback from node into app.

Three ways to close it; the plan **surfaces all three and recommends (A)**, but
this is a **ruling for Gianni** (see feedback):

- **(A) Recommended — node-issued `ws.materialize` exchange, before the mint.**
  At the target, `create_workspace` issues an **internal exchange** to whatever
  provider is attached for materialization at that node (grazel's
  glade-workspaces materializer, wire-attached to the target). The materializer
  resolves name→path from grazel config, runs `gwz init` / `gwz clone`
  (commit-or-fail on disk), answers ok/fail **as data**. On ok the node mints the
  records; on fail it returns the existing `WorkspaceCreateRes{created:false}` +
  error and mints nothing. This **uniquely preserves both contracts**: the disk
  op stays a wire-attached supplier (P00-a), the claim stays node-internal, it
  works for remote targets (the materializer is co-located with the target), and
  the path never rides the client request. Cost: a small **new node capability**
  — a node-*originated* exchange to a local provider (today the node only relays
  client exchanges).
- **(B) In-process materializer hook (design-intent alternative).** grazel
  **embeds** the node as a library (the `both` = "one process" intent,
  `Plan.md` §Architecture; loopback per `GladeSupplierModel.md` §2);
  `create_workspace` awaits a registered `Fn` before minting. Simpler seam, but
  flips grazel from subprocess-spawn to embedded-lib — a real hosting-side
  re-architecture. Keep as the path if node-originated exchange is unwanted.
- **(C) grazel orchestrates disk-then-records from outside (rejected for the
  general case).** grazel resolves the path, `gwz init`s, then (only on success)
  sends `workspace.create` to its node as an ordinary wire client. Works for
  **self-target only** — grazel-A cannot touch peer-B's disk, so it cannot serve
  the clone-onboarding target case. Usable as a stage-0 shim for the local demo,
  not the contract.

**Ordering under all three: disk commits first, records second, fail-closed.**
"Commit-or-fail together" resolves to *disk gates records* — a disk failure
yields no records (clean fail-as-data via the built `created:false` path). The
reverse (records mint, disk gone) is near-impossible on a healthy chain
(diff-idempotent append); the materializer still carries a compensating teardown
of the just-created empty root for completeness. This ordering is a stated
design constraint of P2, not an implementation detail.

## Surfaces this plan builds (extends spec §3)

| glade id | shape | served by | content | phase |
| --- | --- | --- | --- | --- |
| `dir.workspaces` | log (HOME) | node (built) | `WorkspaceEntry` directory | P0 (consume) |
| `dir.claims` | log (HOME) | node (built) | `ServeClaim` liveness | P0 (consume) |
| `ws.selection` | **grip atom** (client) | — (grip-side, no wire) | selected workspace share | P0 |
| `ws.members` | value/log | glade-workspaces @ host | gwz manifest/lock view | P1 |
| `workspace.create` | exchange (node built-in) | node + materializer | ceremony + materialization leg | P2 |
| `ws.materialize` | exchange | glade-workspaces @ host | node-issued disk commit-or-fail | P2 |
| `ws.ops` | exchange | glade-gwz / glade-workspaces | member create/add/clone | P2/P3 |
| `gwz.ops` | exchange (built) | glade-gwz | **full verb set, gated** | P3 |
| `ws.relations` | reserved | — | member dep graph (razel) — DEFERRED | P4 (empty) |

## Phases

Foundational-first: real directory + selection (P0) → multi-workspace hosting so
selection drives real targets (P1) → creation + materialization (P2) → full verb
set + clone/replica onboarding (P3) → razel reserved slot (P4). Each phase exits
on a slice of the §6 user-testable-when line, stated as its gate.

### P0 — Directory read + selection seam (milestone: the directory is real and drives a client-context selection)

Traces `s-ws-host` / `s-ws-create` / `s-ws-clone` are being authored in a
parallel wave **now**; P0 **consumes and reconciles** them, it does not author.

- **S0.1 — Consume + reconcile the traces (ggg-viz; read-only for us).** Adopt
  `s-ws-host` / `s-ws-create` / `s-ws-clone` as the executable spec; reconcile
  drift against the built create ceremony (`claims.rs` / `exchange.rs`) and this
  plan's seams — notably the already-flagged stale `s-app-register` shape
  (Decisions P1.S3.3). Output: a reconciliation note + any trace-fix requests to
  the authoring wave. No node/supplier code. *~0 LOC (doc/coordination).*
- **S0.2 — Directory read tap + typed handles.** Client taps over the
  `dir.workspaces` + `dir.claims` HOME folds → a workspaces view: `{share, name,
  eligible_hosts, live_holder}` with `live_holder` computed **at the reader's
  clock** (mirror `who_serves`). Typed manifest handles (P0.S5 compile wall).
  Repo: **glial** (tap) + **glade-decl-ts** (handles) if generalized, else demo.
  Trace: `s-ws-host` directory leg. *~200 LOC.*
- **S0.3 — `ws.selection` client-context grip + de-hardcode the Gwz tab.** A
  `CURRENT_TAB`-style grip atom (`WS_SELECTION` + `_TAP`, no wire) holding the
  selected workspace share. Refactor `gwz.ts` so the exchange calls read the
  selection grip **instead of** the `GWZ_SHARE = "ws-razel"` constant — the
  concrete change that makes "selection drives the tools" true. Repo: **glade
  (demo)** (+ glial if the atom pattern is promoted for reuse). *~150 LOC.*

**P0 exit gate (user-testable slice):** the demo lists the real directory with
live/absent state that flips when the host stops renewing; the selected
workspace is a first-class grip value the Gwz tab reads (still one workspace —
P1 makes it *several*).

### P1 — Multi-workspace hosting (milestone: N real hosted workspaces; selection drives gwz against the right one; members enumerate)

The de-hardcoding phase: one host serves several workspaces, and the selection
from P0 selects among them for real.

- **S1.1 — grazel workspace map (name/share → real gwz root).** A config file in
  the app-owned `<data>/config` slot mapping each hosted workspace share to its
  real gwz root (the §1.2 data seam; never a request). Replaces the single
  `files_dir()`/`ws-razel` hardcode. Load + validate + tests. Repo: **grazel**.
  *~250 LOC.*
- **S1.2 — glade-gwz share → root resolution.** The supplier resolves the
  **request's target share** → the configured root for *that* share (a map passed
  at attach: repeated `--map share=root`, or a config path), replacing the single
  `--root`. One supplier answering several shares; `argv()`/`DENIED_ARGS`
  unchanged (root still app-authoritative, still root-first). Repo: **glade-gwz**.
  *~200 LOC.*
- **S1.3 — `ws.members` surface (the manifest/lock view).** Serve the gwz
  manifest+lock join for the selected workspace: per member `{name, path, remote,
  desired_branch, commit, dirty, materialized}`. Read from the same app-owned
  files root (glade-workspaces becomes a host-co-located reader) **or** via a
  glade-gwz `manifest`/`members` read-verb it relays — pick per the feedback
  underdetermination (recommend: a glade-gwz read-verb, reusing its disk access;
  glade-workspaces relays). Trace: `s-ws-host` member leg. Repo: **glade-workspaces**
  (new) + possibly **glade-gwz**. *~300 LOC.*
- **S1.4 — grazel serves N workspaces + declares them.** grazel attaches the gwz
  supplier for each mapped workspace and the loading node mints a
  `WorkspaceEntry` + `ServeClaim` per hosted workspace; `grazel-app.glade` gains
  the N `workspace` lines (or a workspace-registry form). Repo: **grazel** +
  **glade** (app-file + node-test record counts — dual-maintained fixture, per
  P1.S3.1/2). *~200 LOC.*
- **Demo: a Workspaces tab.** Lists the directory (S0.2) with live state; picking
  a row sets `ws.selection` (S0.3); the Gwz tab now operates on the picked
  workspace; a members panel renders `ws.members`. Repo: **glade (demo)**.

**P1 exit gate (§6 line 1):** *two* real hosted gwz workspaces are listed with
live state; I select one and the Gwz tab runs `status`/`ls`/`diff` against
**that** workspace's real tree (the wrong-target P1 gap is closed); its members
enumerate.

### P2 — Creation + materialization (milestone: create an empty workspace, records + disk commit-or-fail; add a member)

Implements the surfaced seam. Sequence S2.1 → S2.2 (S2.1 defines the seam S2.2
fills); S2.3 parallels once the seam is chosen.

- **S2.1 — The materialization seam (the crux).** Implement decision (A): the
  target's `create_workspace` issues an internal `ws.materialize` exchange to the
  attached materializer **before** minting, mints only on ok, returns
  `created:false` + error on fail (fail-closed). Node side: the reserved-id
  handler + the node-originated-exchange capability. Provider side: the
  `ws.materialize` surface stub in **glade-workspaces**. Trace: `s-ws-create`
  materialization leg (records + disk commit-or-fail together; failure as data),
  self **and** target-routed. Repo: **glade (node)** + **glade-workspaces**.
  *~350 LOC. Gate: this step alone decides the seam — if Gianni rules (B)/(C),
  re-scope before building.*
- **S2.2 — grazel materializer (`gwz init`, empty workspace).** The materializer
  body: resolve name→path from the S1.1 map, `gwz init` an empty (0-member) root,
  commit-or-fail, register the new mapping; compensating teardown on the
  records-fail path. A zero-member workspace must render honestly (§1.3). Repo:
  **grazel** (+ the glade-workspaces provider it backs). *~250 LOC.*
- **S2.3 — `ws.ops` member ceremonies + first mutating gwz verbs.** Create a
  brand-new member (`gwz repo create` shape) and add/clone an existing repo
  (`gwz repo add` / clone from a remote) as exchange ceremonies, results as data;
  `ws.members` reflects the new set on completion. This lights the **first
  mutating gwz verbs** under the P3 gating mechanism (built here, generalized in
  P3). Trace: `s-ws-create` member-add variant. Repo: **glade-gwz** (verbs) +
  **glade-workspaces** (ceremony surface). *~350 LOC.*
- **Demo:** create-workspace + add-member controls in the Workspaces tab; a
  failed create (bad path/permission) surfaces as data.

**P2 exit gate (§6 lines "create … empty" + "add a member"):** I create a new
empty workspace and it appears in the directory (0 members, honestly); a disk
failure comes back as data with no orphan records; I add a member repo and it
shows up in `ws.members`.

### P3 — Full gwz verb set (gated) + clone-as-replica onboarding (milestone: full verbs gated; a second peer becomes an eligible host by cloning)

- **S3.1 — glade-gwz verb-set expansion (config-gated).** Turn `ALLOWED_VERBS`
  from a `const` into a **policy**: read verbs on by default; mutating verbs
  (`push`, `pull`, `capture`/`snapshot`, `branch`, `tag`, `repo`, `stash`,
  `add`, `commit`, …) **opt-in per composition** (§2.3 interim). `forall` is
  **explicitly not added** — its home is glade-terminal. Document the stage-2
  seam: per-verb **grants** replace the config list (the s-verbs taxonomy;
  `gwz.*` seeds already ride `grazel-app.glade`), at the same enforcement point.
  Repo: **glade-gwz**. *~250 LOC.*
- **S3.2 — grazel verb-gating config.** The per-composition opt-in surface
  (recommend a **per-verb allow list** in `<data>/config` over a single "allow
  mutations" switch — feedback Q3), threaded into the supplier attach. Repo:
  **grazel**. *~150 LOC.*
- **S3.3 — Clone ceremony = replica onboarding (`s-ws-clone`).** A second peer
  materializes an existing workspace via **`gwz clone` (git/gwz remotes carry
  repo content; the gwz manifest + the glade records travel via glade)**, then
  **registers eligibility** (adds itself to `eligible_hosts`) and **claims** per
  the built lease model — so the directory converges to **two eligible hosts**
  for the same share. This reuses the P2 materializer (clone body) + the create
  ceremony's target routing to peer B. Transport lean per §7-Q2: gwz/git remotes
  for content, glade for records. Repo: **glade-workspaces** + **grazel**
  (clone materializer) + **glade** (eligibility registration if node-side).
  *~350 LOC.*
- **Demo:** the verb picker expands (showing the gated set + a denied mutating
  verb when the composition opts out); a "clone onto peer B" flow; the directory
  shows the share with two hosts.

**P3 exit gate (§6 line "clone … from another peer"):** with two peers running, I
clone an existing hosted workspace onto the second; it registers as a second
eligible host and claims; I run the full (gated) gwz verb set against the
replica, and a mutating verb the composition disabled is refused as data.

### P4 — Reserved: razel `ws.relations` (milestone: the slot exists, empty)

- **S4.1 — `ws.relations` reserved surface slot (EXPLICITLY EMPTY).** Reserve the
  glade id in the surface table and document the seam — member dependency graph
  **fed by razel's model** — and **build nothing** (§2.2: "nothing is built or
  speculated before razel provides it"). This step exists to mark the reservation
  and exits immediately; it depends on the external razel package and floats.
  *~0 LOC.*

## The forall → terminal seam (documented, not built)

`forall` is **not** an exchange verb and **not** in the expanded glade-gwz set
(§2.3). Arbitrary per-member exec is terminal-session semantics (owner-bound
pty, streamed output, stage-2 `shell.exec` gating). When **glade-terminal**
lands (GLP-0006 P3, separate plan), the workspaces/gwz UI composes a
`gwz forall … -c '<cmd>'` line and hands it to a terminal session opened in the
**selected** workspace (this supplier provides the selection + root; terminal
runs it). glade-workspaces builds **none** of this — it is a noted downstream
consumer of `ws.selection`. glade-terminal does not exist today.

## Parallelism + repo-ownership map

Single-writer-per-repo per wave (GLP-0006 discipline). Note the **glade repo
holds both the node and the demo** (`glade/node`, `glade/demo`), so glade-repo
steps serialize within a wave.

- **P0:** S0.1 (ggg-viz, read-only) ‖ S0.2 (glial/decl-ts) ‖ S0.3 (glade/demo).
  S0.2 and S0.3 both may touch the demo — sequence within glade.
- **P1:** S1.1 (grazel) ‖ S1.2 (glade-gwz) ‖ S1.3 (glade-workspaces) first;
  then S1.4 (grazel + glade) + the demo tab (glade). glade-repo work (S1.4 node
  app-file + demo tab) serializes.
- **P2:** S2.1 (glade node + glade-workspaces) **leads** (defines the seam);
  S2.2 (grazel) follows; S2.3 (glade-gwz + glade-workspaces) parallels S2.2 once
  the seam is chosen.
- **P3:** S3.1 (glade-gwz) ‖ S3.2 (grazel) then S3.3 (glade-workspaces + grazel +
  glade). **Needs two peers** in the live gate.
- **P4:** S4.1 doc-only, anytime.

New repo introduced: **`glade-workspaces`** (member per P00-b naming — repos say
what; README first line "a glade supplier: …"). Existing repos touched: glade
(node + demo + apps), glade-gwz, grazel, glial, glade-decl-ts, ggg-viz
(read-only).

## Discipline (inherits GLP-0006 `Plan.md` §Discipline)

Trace leads build; the demo is the live gate against the user-testable-when line,
not green tests; suppliers never import node internals (the create/materialize
leg is the one node-co-located ceremony, and it is explicitly node-side, not a
supplier reaching in); commit-per-step with all repo gates green; no attribution
trailers; pnpm never npm; tests never touch the real `~/.glade` (pin
`GLADE_HOME`/`HOME`, per the P0.S6/P1.S2 harness precedent); the dual-maintained
`grazel-app.glade` copies stay byte-identical + the node-test record counts move
with them (P1.S3.1/2); corpus/trace gates red = a design event.

## Open underdeterminations (feedback — collected for a ruling)

1. **Materialization seam ownership (the big one).** §2.2 wants create + disk to
   "commit-or-fail together on the target," but `workspace.create` is a
   node-reserved built-in, claim-mint is node-internal + node-lifetime, the path
   is app-owned + must not ride the request, and grazel spawns the node as a
   subprocess. Recommendation **(A)**: the node issues an internal
   `ws.materialize` exchange to a wire-attached materializer at the target before
   minting (fail-closed) — the only option preserving both the wire-attachment
   contract and node-internal claim ownership for **remote** targets, at the cost
   of a new node-originated-exchange capability. **(B)** (embed the node in-process
   for a callback) and **(C)** (grazel orchestrates; self-target only) are the
   alternatives. **Needs a ruling before P2.S1.**
2. **`ws.selection` is grip-only or also a declared surface?** §2.1 says
   "per-client-context (grip-side)" yet §3 lists it in the Surfaces table with a
   glade id. Plan assumes **pure client grip** (like `CURRENT_TAB`, no wire) for
   v1; a declared surface (selection syncs across a user's devices) is a deferred
   option. Confirm.
3. **Who serves `ws.members`, and via what disk access?** The manifest/lock live
   on the **host's** app-owned files. Does glade-workspaces read them directly
   (becoming a second app-owned-storage consumer alongside glade-gwz), or relay a
   glade-gwz read-verb? Plan leans **relay a glade-gwz `manifest` read-verb**
   (one disk-access owner). Confirm the ownership split.
4. **Name uniqueness (§7-Q1).** `WorkspaceEntry` has a unique **share** (the
   routing key — necessarily unique) plus a display **name**. Plan assumes
   **share-id unique, display-name duplicates tolerated** (fingerprint-style, like
   principals). Confirm, or require a name-claim.
5. **Clone transport granularity (§7-Q2).** Plan takes the v1 lean — git/gwz
   remotes for repo **content**, glade for **records**. Underspecified: how peer B
   gets the source **gwz manifest** itself (the `gwz.yml`) — carried by glade with
   the records, or fetched over a gwz remote? These can differ; recommend
   **manifest travels with the glade records** so B can plan the clone before any
   git fetch.
6. **Interim mutating-verb gating granularity (§7-Q3).** Plan recommends a
   **per-verb allow list** in grazel config over a single "allow mutations"
   switch — finer, and it lines up 1:1 with the stage-2 per-verb grants that
   replace it. Confirm.
7. **Single-root supplier assumption is now false.** The spec's multi-workspace
   hosting model requires glade-gwz + grazel to move from one `--root`/one
   `--share` to a **share→root map** (S1.1/S1.2). Not a spec contradiction, but a
   build-surface the spec assumes and the built code doesn't yet have — flagged so
   it isn't mistaken for scope creep.
