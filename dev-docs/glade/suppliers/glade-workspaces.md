# glade-workspaces — workspace hosting, directory, selection, creation (supplier spec)

Status: full spec v1 (2026-07-12) — expands the `SupplierOutlines.md` entry;
hosting/creation/operations requirements stated by Gianni 2026-07-12.
**GLP-0006 RATIFIED rulings landed** (`RulingWorksheet.md` §II/§VI): E-ws-1,
E-ws-2, C-gwz-7, C-gwz-8, H-R2, H-R3, H-C3, H-C4, H-C5 — cited inline below.
Common supplier contract: `dev-docs/glade/GladeSupplierModel.md`. Substrate
pieces already built: `dir.workspaces` fold + C2 claim routing (R3),
`workspace.create` target-routed ceremony + live claim minting (GLP-0006
P0.S2 — records only; disk materialization was left as the external seam
this spec now fills).

## 1. The hosting model (the gwz model, stated)

1. **A peer (or local+peer) hosts any number of workspaces.** A hosted
   workspace IS (most likely) a **gwz workspace** on that peer's disk: a
   root with **0–N member repos**. Glade's `WorkspaceEntry`/`ServeClaim`
   records are the directory view of it; the gwz tree is the substance.
2. **A stable workspace/share ID is the SOLE routing + authorization
   identity (E-ws-1).** The node resolves **ID→root**; the app-owned data
   seam (the peer's composition / grazel config-data dir) maps that stable
   ID — never a display name — to its real gwz root. The mapping never
   rides a request and is never derivable from one; a request that ASSERTS
   a root mismatching the ID→root resolution MUST **fail closed**. **Display
   names NEVER route** — display-only, duplicates tolerated (H-C3). The
   grip-side selector is classified SEPARATELY from the authority (§3): the
   authority trusts the addressed stable ID, not a grip-local value. This
   collapses the three sources of truth (grip-local selection / addressed
   share / terminal `workspace-ref`) to one — the addressed stable ID.
3. **Role state is DISTRIBUTED/REPLICATED, not node-local authority
   (H-R2).** The supplier is client taps (directory reads, selection) +
   host ceremonies (create, claim); `WorkspaceEntry`/`ServeClaim` replicate
   over the home share — no single node owns the role.
4. A workspace with zero members is legal (freshly created, not yet
   populated) and must render honestly in directories and UIs.

## 2. Requirements

### 2.1 Directory + selection (the outline's core, unchanged)
- List workspaces from the LOCAL replica (`dir.workspaces` fold) with
  claim/liveness state and eligible hosts, cross-peer via home-share sync.
- **Selection**: a client-visible selected-workspace surface that the tool
  suppliers (gwz ops, files, terminal, razel) key off. Selection is
  per-client-context (grip-side), but the *available* set is this
  supplier's data.
- Member repos of the selected workspace are enumerable (name, path within
  the workspace, remote, pinned state — the gwz manifest/lock view served
  as a surface).

### 2.2 Creation — workspaces and member repos
- **Workspace creation is a DURABLE state machine (C-gwz-8):**
  `intent → materialized → registered → claimed → complete`. Recovery is
  **forward-only + idempotent** (no orphan, no half-create); the **HOME
  share logs are authoritative**. glade-workspaces owns the ceremony + the
  records; the **gwz family does the disk leg** (§2.3, C-gwz-7). Authorship
  follows **H-R3**: the client submits the create INTENT; the host authority
  validates, performs the materialization, then appends the canonical
  result records preserving the requester (B3) context — not a
  client-appended effect record.
- **Create a workspace, brand new**: the `workspace.create` ceremony
  (records, target-routed) invokes the gwz `create`/`init` materializer leg
  (empty workspace, 0 members) on the target peer. **Disk gates records** —
  `materialized` commits before `registered`; a disk failure yields
  `failed` + no records (failure as data).
- **Create a workspace cloned from an existing one**: the ceremony invokes
  the gwz `clone-workspace` materializer leg from the source (another peer's
  hosted workspace or a remote). **Clone makes the new peer an ELIGIBLE +
  WARM provider; it does NOT seize the active claim (H-C5)** — eligibility
  registers into the `eligible_hosts` OR-set (E-ws-2) and the existing lease
  model governs any later claim handoff. **The clone manifest/lock data
  travels WITH the glade records (H-C4)**; bulk git objects still pull via
  the source's gwz/git remotes.
- **Member repos**: create brand-new members or add/clone existing repos
  are **typed gwz members** (`glade-gwz-repo` / `glade-gwz-clone-member`,
  §2.3, C-gwz-7) — the retired `ws.ops` façade; their results are data and
  the directory surfaces reflect the new member set on completion.
- **Member-repo relationships (dependency graph between members): DEFERRED
  until the razel package lands** (Gianni, 2026-07-12). The surface slot is
  reserved (a `ws.relations` surface fed by razel's model); nothing is
  built or speculated before razel provides it.

### 2.3 Operations — the full gwz verb set
- **All gwz operations are supported**: status, diff, ls, push, pull,
  capture/snapshot, branch, repo verbs, etc. — executed by the **glade-gwz
  supplier FAMILY** (one supplier per request type — ruled 2026-07-12)
  against the SELECTED workspace (this supplier provides the selection +
  **ID→root resolution**, E-ws-1; the gwz family provides execution). This
  SUPERSEDES glade-gwz's stage-1 read-only posture as the requirement
  statement: the read-only allow-list was a bring-up guard, not the product
  surface.
- **`ws.ops` façade RETIRED → typed gwz members (C-gwz-7).**
  Existing-workspace repo-member operations (create/add/clone) stay PUBLIC
  gwz — typed `glade-gwz-repo` / `glade-gwz-clone-member` exchanges, NOT a
  workspaces-owned `ws.ops` multiplexer. Ownership splits at **workspace
  lifecycle** (glade-workspaces owns `workspace.create` + the durable
  ceremony, §2.2) **vs repo operation** (public gwz members). glade-workspaces
  provides SELECTION + **ID→root resolution** (E-ws-1); the gwz family
  executes; workspaces consumes the member result into the directory
  surfaces.
- **Gating, not omission**: mutating operations (push, pull, capture, repo …)
  are in scope and are gated — stage-2 = ORDINARY surface grants on each
  supplier's exchange (the s-verbs taxonomy mapped onto surfaces; AZ-1 path
  scoping where applicable). Interim stage-1 posture for live systems:
  mutating suppliers opt-in per composition (attach them or don't), read
  suppliers on by default; the built allow-list remains the bring-up
  artifact's guard until the family reshape lands.
- **`forall` is NOT an exchange verb — it routes through glade-terminal.**
  Arbitrary per-member command execution is terminal-session semantics
  (owner-bound pty/channel, streamed output, stage-2 `shell.exec` gating),
  not a request/response ceremony. The gwz panel/plugin hands a composed
  `gwz forall … -c '<cmd>'` line to a terminal session in the selected
  workspace; glade-gwz itself never executes arbitrary commands. (This
  resolves the P1.S2 exclusion permanently: `forall` was excluded from the
  exchange allow-list as the sharpest surface — its home is the terminal.)

## 3. Surfaces (sketch — detail at build time)

| glade id | shape | content |
| --- | --- | --- |
| `dir.workspaces` | log (home) | WorkspaceEntry records (built); `eligible_hosts` is an **OBSERVED-REMOVE SET** (OR-set) — concurrent add/remove + stale replay converge, explicit removal supported (E-ws-2; LWW lost concurrent clones) |
| `ws.members` | value/log per workspace | member-repo enumeration (manifest/lock view) |
| `ws.selection` | client-context surface (grip-side) | the grip-side selector that **ADDRESSES** a workspace by its **stable ID** (E-ws-1); classified SEPARATELY from the authority — the authority trusts the addressed ID, never this grip-local value; ID→root is resolved node-side |
| `workspace.create` | exchange (built) | create ceremony — the durable state machine (§2.2, C-gwz-8); invokes the gwz materializer leg (C-gwz-7) |
| `ws.relations` | reserved | member dependency graph — DEFERRED (razel) |

**`ws.ops` RETIRED (C-gwz-7):** member-repo create/add/clone are typed gwz
members (`glade-gwz-repo` / `glade-gwz-clone-member`), not a workspaces-owned
exchange; workspaces consumes their result (§2.2/§2.3).

## 4. Split of responsibilities (who does what)

- **glade-workspaces**: directory + role state (distributed/replicated,
  H-R2), selection (**stable-ID addressing**, E-ws-1), the **workspace
  lifecycle ceremony** (`workspace.create` durable state machine + records,
  §2.2, C-gwz-8), the **ID→root resolution** contract (the map is
  app-owned, E-ws-1), member enumeration.
- **glade-gwz-\* family**: operation EXECUTION against the selected
  workspace (one supplier per request type, full set per §2.3, gated as
  surfaces) **PLUS the create/init/clone disk materializer leg** the
  ceremony invokes, and the typed repo-member create/add/clone members
  (C-gwz-7).
- **glade-terminal**: `forall` and any other arbitrary exec, as sessions.
- **razel package (external, WIP)**: member-repo relationships — deferred.

## 5. Traces to author before building

- **s-ws-host** — a peer hosting two workspaces; directory lists both with
  liveness; selection drives a tool supplier's target.
- **s-ws-select-by-id** — selection addresses a workspace by its **stable
  ID**; the node resolves ID→root and targets the tool supplier; a request
  that ASSERTS a root mismatching the ID→root resolution **fails closed**;
  a duplicate display name does not ambiguate (E-ws-1, H-C3). NEW.
- **s-ws-create-durable** — the create state machine
  `intent → materialized → registered → claimed → complete`: records commit
  ONLY on disk success; the client submits intent, the host appends the
  canonical result preserving B3 context (C-gwz-8, H-R3). Supersedes the old
  commit-or-fail trace.
  - arm **s-ws-create-recover** — a crash after `materialized`, and between
    the `registered`/`claimed` appends, both replay **forward-only** to
    `complete` (no orphan, no half-create).
- **s-ws-clone-orset** — two peers clone the same source concurrently; each
  registers eligibility into the `eligible_hosts` **OR-set**; the directory
  converges to BOTH eligible + warm hosts — neither seizes the active claim
  (H-C5, E-ws-2); the clone manifest/lock rides the glade records (H-C4).
  LWW would have lost one.
- Member-repo add/create ride the typed gwz members (§2.3); results appear
  in `ws.members` on completion.

## 6. Dependencies + user-testable-when

- Depends on: base glade only (spine). glade-users for attribution of
  ceremonies (creator identity on records).
- **User-testable when**: I see my real hosted gwz workspaces listed with
  live state; I select one and gwz/files/terminal visibly operate on it;
  I create a new workspace (empty) and clone an existing one from another
  peer; I add a member repo and it appears in the member list.

## 7. Open questions (Gianni)

- **RESOLVED (E-ws-1 + H-C3):** duplicate display names ARE tolerated — no
  unique name-claim; routing is by **stable ID**, and the UI disambiguates
  duplicates with an ID suffix. Display names never route (§1, §3).
- **RESOLVED (H-C4 + H-C5):** clone-from-existing — the **manifest/lock
  data travels WITH the glade records** (H-C4); bulk git objects still pull
  via the source peer's gwz/git remotes (the machinery exists). Clone makes
  the peer **eligible + warm, does NOT seize** (H-C5); eligibility lands in
  the `eligible_hosts` OR-set (E-ws-2).
- **RESOLVED (E-ws-2):** concurrent clones converge — `eligible_hosts` is
  an OR-set with explicit removal, not LWW (§3; s-ws-clone-orset).
- Interim mutating-verb opt-in granularity (per verb vs one "allow
  mutations" switch) until stage-2 grants land. *(Still open.)*
