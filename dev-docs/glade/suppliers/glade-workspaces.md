# glade-workspaces — workspace hosting, directory, selection, creation (supplier spec)

Status: full spec v1 (2026-07-12) — expands the `SupplierOutlines.md` entry;
hosting/creation/operations requirements stated by Gianni 2026-07-12.
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
2. **Name → path mapping is app-owned** (the data seam): the peer's
   composition (grazel config/data dir) maps each hosted workspace name to
   its real gwz root. The mapping never rides a request and is never
   derivable from one.
3. A workspace with zero members is legal (freshly created, not yet
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
- **Create a workspace, brand new**: the built `workspace.create` ceremony
  (records, target-routed) PLUS disk materialization on the target peer —
  `gwz init` shape (empty workspace, 0 members). The ceremony and the
  materialization commit-or-fail together on the target; failure is data.
- **Create a workspace cloned from an existing one**: materialize via
  `gwz clone` semantics from the source (typically another peer's hosted
  workspace or a remote) — this is also HOW a second peer becomes an
  eligible host/replica of an existing workspace (clone → register
  eligibility → claim per the existing lease model).
- **Member repos**: create brand-new members (`gwz repo create` shape) or
  add/clone existing repos into the workspace (`gwz repo add` / clone from
  a remote). Exposed as ceremonies (exchange verbs) with results as data;
  the directory surfaces reflect the new member set on completion.
- **Member-repo relationships (dependency graph between members): DEFERRED
  until the razel package lands** (Gianni, 2026-07-12). The surface slot is
  reserved (a `ws.relations` surface fed by razel's model); nothing is
  built or speculated before razel provides it.

### 2.3 Operations — the full gwz verb set
- **All gwz operations are supported**: status, diff, ls, push, pull,
  capture/snapshot, branch, repo verbs, etc. — executed by the glade-gwz
  supplier against the SELECTED workspace (this supplier provides the
  selection + root mapping; glade-gwz provides execution). This SUPERSEDES
  glade-gwz's stage-1 read-only posture as the requirement statement:
  the read-only allow-list was a bring-up guard, not the product surface.
- **Gating, not omission**: mutating verbs (push, pull, capture, repo …)
  are in scope and are gated — stage-2 per-verb grants (the s-verbs model;
  AZ-1 path scoping where applicable). Interim stage-1 posture for live
  systems: mutating verbs opt-in per composition (config), read verbs on by
  default; the allow-list mechanism already built is the enforcement point
  until grants replace it.
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
| `dir.workspaces` | log (home) | WorkspaceEntry records (built) |
| `ws.members` | value/log per workspace | member-repo enumeration (manifest/lock view) |
| `ws.selection` | client-context surface | the selected workspace (grip-side value; documented here because tool suppliers key on it) |
| `workspace.create` | exchange (built) | create ceremony — gains the materialization leg (§2.2) |
| `ws.ops` | exchange | member-repo create/add/clone ceremonies |
| `ws.relations` | reserved | member dependency graph — DEFERRED (razel) |

## 4. Split of responsibilities (who does what)

- **glade-workspaces**: directory, selection data, workspace + member
  CREATION/materialization, the name→path mapping contract (mapping itself
  is app-owned), member enumeration.
- **glade-gwz**: verb EXECUTION against the selected workspace (full verb
  set per §2.3, gated).
- **glade-terminal**: `forall` and any other arbitrary exec, as sessions.
- **razel package (external, WIP)**: member-repo relationships — deferred.

## 5. Traces to author before building

- **s-ws-host** — a peer hosting two workspaces; directory lists both with
  liveness; selection drives a tool supplier's target.
- **s-ws-create** — extends the existing create trace with the
  materialization leg (records + disk commit-or-fail together; failure as
  data), both self and target-routed.
- **s-ws-clone** — clone-from-existing: second peer materializes, registers
  eligibility, claims; the directory converges to two eligible hosts.
- Member-repo add/create can ride s-ws-create variants.

## 6. Dependencies + user-testable-when

- Depends on: base glade only (spine). glade-users for attribution of
  ceremonies (creator identity on records).
- **User-testable when**: I see my real hosted gwz workspaces listed with
  live state; I select one and gwz/files/terminal visibly operate on it;
  I create a new workspace (empty) and clone an existing one from another
  peer; I add a member repo and it appears in the member list.

## 7. Open questions (Gianni)

- Does workspace creation REQUIRE a name-claim in the directory to be
  unique per deployment, or are duplicate display names tolerated
  (fingerprint-suffixed) like principals?
- Clone-from-existing across peers: pull via the source peer's gwz remotes
  (git-level) vs glade-carried transfer — v1 lean is gwz/git remotes (the
  machinery exists), glade carries only the records.
- Interim mutating-verb opt-in granularity (per verb vs one "allow
  mutations" switch) until stage-2 grants land.
