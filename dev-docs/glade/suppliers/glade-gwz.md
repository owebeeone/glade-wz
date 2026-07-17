# glade-gwz-* — the gwz supplier FAMILY, one supplier per request type (spec)

Status: full spec **v3 (2026-07-12) — RATIFIED**. v2 (the corrective respec) had
the right shape but the WRONG grain (21) and no security-substrate consumption.
This rewrite lands GLP-0006 `RulingWorksheet.md` §II (C-gwz-1..8, E-ws-1) + the
consumed security substrate (§I B1–B5) + cross-cutting `RootRelativePath` (§IV
D14): the DERIVED **25-interface** grain, the four-way tag split on disjoint
closed DTO enums, path-free DTOs on the shared path type, the closing REPLICATED
result record, commons-log visibility (no grant-keyed zone), authenticated
non-replaceable provider attach, and a delivered requester context.

Protocol source = **gwz-core v0.9.2** (on-disk `gwz-dev/gwz-core`, `Cargo.toml
version = "0.9.2"`, the release commit) **on taut v0.8.1** — RULED (`Decisions.md`
2026-07-12, "second review (SR56) verified", pin line). Ratifying source =
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md` §I/§II/§IV.
Common contract: `GladeSupplierModel.md` (§2 exchange = provider-attach; §5 data
seam; §6 failure as data). Cross-spec: glade-workspaces §1.2/§2.2/§2.3 (ID→path
map + selection + `workspace.create`), glade-users §1 (fingerprint attribution),
glade-terminal §5 (`forall` is terminal's), glade-diff §9 (cross-surface diff is
glade-diff's; **working-tree diff stays a gwz member**). Reshapes the P1.S2
bring-up (`glade-gwz/src/` — one `gwz.ops` exchange + a `{verb,args}` allow-list).

## 1. The family + the full inventory (24 methods — SR56-01, verified)

gwz-core's request types "are different enough that they need a different UI and
interface — best served as separate glade suppliers" (Gianni). Each `role="in"`
method contributes to its OWN glade supplier grain, not a verb multiplexed
through one exchange. Glade-native reasons (`Decisions.md`): **gating** =
ordinary per-surface grants, not a "granted ActionKinds" sub-vocabulary inside one
exchange (a GDL-038 parallel-plane smell); **legibility** — the `.glade` file
enumerates every surface; **seams** — a read-only deployment never attaches the
mutating/egress members, so the wall is STRUCTURAL (and, per B1, authenticated).

Pinned v0.9.2 (`gwz-dev/gwz-core/protocol/gwz.taut.py`) has **24** `role="in"`
methods (verified). The 3 `role="out"` methods (`events.subscribe` shape=`log`:101,
`operation.result`:105, `diff.output` shape=`log`:122) are the response side,
mirrored in §4. **Per-method audit, classified by what the request FIELDS can DO
— never by the label:**

| # | method (line) | what it does | capability class |
| --- | --- | --- | --- |
| 1 | `status` (49) | read git + lock status | **read** (→ `WorkspaceGitStatus`) |
| 2 | `ls` (53) | list members | **read** |
| 3 | `list_snapshots` (61) | list snapshots | **read** |
| 4 | `diff` (113) | plan working-tree diff (index/tree/worktree) | **read** (→ manifest + `diff.output` byte log) |
| 5 | `repo_sync` (29) | refresh member metadata from local git | **local-mutate** (manifest) |
| 6 | `add_existing_repo` (21) | register an existing local repo | **local-mutate** — carries `repository_path` (983; §3 → `RootRelativePath`/`RepoImportHandle`) |
| 7 | `create_repo` (25) | init a new local member repo | **local-mutate** (materialize: new member) |
| 8 | `detach_repo_member` (37) | soft-remove one active member | **local-mutate** (manifest active flag) |
| 9 | `attach_repo_member` (41) | reactivate a historical member | **local-mutate** |
| 10 | `materialize` (45) | move members to lock/head/snapshot/tag/commit | **local-mutate** (checkout; no network) |
| 11 | `pull_snapshot` (85) | materialize to a NAMED snapshot | **local-mutate** (checkout; **no network despite "pull"**) |
| 12 | `capture` (69) | capture live state → lock | **local-mutate** (lock; no worktree) |
| 13 | `snapshot` (57) | capture state → named snapshot | **local-mutate** (snapshot artifact) |
| 14 | `commit` (73) | commit staged/tracked changes, members+root | **local-mutate** (git commit) |
| 15 | `stage` (77) | multi-repo `git add` | **local-mutate** — carries `cwd` (1134; §3 → ws-relative) |
| 16 | `stash` (93) | git stash push/list/apply/pop/drop | **op-enum SPANS read│local** (`StashOp` 164; no remote) |
| 17 | `branch` (97) | git branch list/create/delete/merge | **op-enum SPANS read│local** (`BranchOp` 195; no remote) |
| 18 | `pull_head` (81) | fetch + fast-forward to upstream | **egress** (network fetch) |
| 19 | `push` (89) | push members to remotes | **egress** (network write) |
| 20 | `tag` (65) | git tags create/list/**fetch/push**/delete | **op-enum SPANS read│local│egress-read│egress-write** (`TagOp` 156 + `remote`/`all` 1102) |
| 21 | `create_workspace` (9) | create an empty workspace | **create-materialize** (new root 957; no selection) |
| 22 | `init_from_sources` (13) | workspace from source URLs | **create-materialize + egress**, indivisible (clones `sources[url]`) |
| 23 | `clone_workspace` (17) | clone a root repo + materialize members | **create-materialize + egress**, indivisible (2nd-peer replica) |
| 24 | `clone_repo_member` (33) | clone + register one member from a URL | **egress + materialize**, indivisible (adds to a SELECTED ws) |

**The label-vs-fields catch (SR56-03):** `tag` is labelled "versioning" but its
`TagOp` includes `fetch`/`push` with `remote`/`all` — remote ops inside a local
label; `pull_snapshot` is labelled "pull" but touches no network; `stash`/`branch`
carry a read `list` op inside a mutate verb. Classification is per-op, never
per-name — this drives the §2 grain function.

**`forall` is NOT here — verified unchanged in v0.9.2.** It is no `role="in"`
method: only an `ActionKind` enum value (`forall=15`, 144) and CLI-local
`ExecRequest`/`ExecResponse` messages the file explicitly marks "gwz-core MUST NOT
handle" (1062–1089). So `forall` stays glade-terminal's (owner-self-exec on the
owner's own pty, glade-terminal §5); this family never executes arbitrary
commands.

## 2. The partition — the ratified 25-interface grain (C-gwz-1, C-gwz-2)

**Governing rule (RulingWorksheet §II):** a merge is legal ONLY when operations
share the **same UI contract AND the same capability class**. A difference on
either axis requires a separate interface. Do not split merely because
implementations differ; do not merge read and mutation authority merely because
one command groups them.

**The grain is a MECHANICAL function of the IR, not a hand-count.** Capability
class ∈ {`read`, `local-mutate`, `egress-read`, `egress-write`}. Applied to the 24
methods:

1. **One surface per `(method, capability-class)`.**
2. A method whose DTO carries a **discriminated op-enum spanning classes**
   projects to **one surface per class present**; each surface receives a
   **disjoint, closed sub-enum** (C-gwz-2). `tag`→4, `stash`→2, `branch`→2.
3. A method that is a **single indivisible op with multiple effects** (no separable
   op-enum) stays **one surface**, classed at its **most-privileged effect**:
   `clone_repo_member`→egress, `init_from_sources`/`clone_workspace`→create+egress.
4. Two methods **merge** iff they share **both** UI contract **and** capability
   class — the four accepted twin-merges.

**Derivation.** 21 single-class/indivisible methods → 21 surfaces; 3 op-enum
methods split by class → `tag` 4 + `stash` 2 + `branch` 2 = 8; subtotal 29; −4
twin-merges = **25**. Equivalently: 24 − 4 twin-merges + (tag 1→4) + (stash 1→2) +
(branch 1→2) = 25. **The generator recomputes this count from the protocol IR and
FAILS THE BUILD on drift** (C-gwz-1) — 25 is the derived result, never a literal.
**Do not accept 21/22/23:** each leaves at least one read/mutation capability
exception (a list-only UI holding mutation authority).

| member | fronts (method{ops}) | interface / UI | class |
| --- | --- | --- | --- |
| `glade-gwz-status` | `status` | status dashboard (branches, ahead/behind, file-changes, lock-match) | read |
| `glade-gwz-ls` | `ls` | member list | read |
| `glade-gwz-list-snapshots` | `list_snapshots` | snapshot list | read |
| `glade-gwz-diff` | `diff` | working-tree diff manifest + patch viewer | read |
| `glade-gwz-tag-list` | `tag`{list} | tag list panel | read |
| `glade-gwz-stash-list` | `stash`{list} | stash list panel | read |
| `glade-gwz-branch-list` | `branch`{list} | branch list panel | read |
| `glade-gwz-repo` | `add_existing_repo`, `create_repo` | add-member (existing local │ new empty) | local-mutate |
| `glade-gwz-repo-active` | `detach_repo_member`, `attach_repo_member` | deactivate │ reactivate member | local-mutate |
| `glade-gwz-repo-sync` | `repo_sync` | refresh member metadata | local-mutate |
| `glade-gwz-materialize` | `materialize`, `pull_snapshot` | move worktree to target (lock│head│snapshot│tag│commit) | local-mutate |
| `glade-gwz-capture` | `capture`, `snapshot` | save state → lock │ snapshot:name | local-mutate |
| `glade-gwz-commit` | `commit` | commit (message; -a) | local-mutate |
| `glade-gwz-stage` | `stage` | stage pathspecs (multi-repo add) | local-mutate |
| `glade-gwz-tag-mutate` | `tag`{create, delete} | local tag create/delete | local-mutate |
| `glade-gwz-stash-mutate` | `stash`{push, apply, pop, drop} | stash mutate | local-mutate |
| `glade-gwz-branch-mutate` | `branch`{create, delete, merge} | branch mutate | local-mutate |
| `glade-gwz-pull` | `pull_head` | pull (fetch + fast-forward) | **egress** |
| `glade-gwz-push` | `push` | push (remote / refspec override) | **egress** |
| `glade-gwz-tag-fetch` | `tag`{fetch} | remote tag fetch | **egress-read** |
| `glade-gwz-tag-push` | `tag`{push} | remote tag push | **egress-write** |
| `glade-gwz-clone-member` | `clone_repo_member` | clone a member from a URL into the selected ws | **egress** |
| `glade-gwz-create` | `create_workspace` | new empty-workspace materializer (internal leg, §5) | create |
| `glade-gwz-init` | `init_from_sources` | init-from-sources materializer (internal leg, §5) | create + **egress** |
| `glade-gwz-clone-workspace` | `clone_workspace` | clone a whole workspace (internal leg, §5) | create + **egress** |

**The four twin-merges** (same UI contract AND class): `repo` (both add a member,
local); `repo-active` (both flip the active flag); `materialize` (+`pull_snapshot`
— both local checkout-to-target, so `pull_snapshot` groups by CAPABILITY with
`materialize`, **not** `pull_head`); `capture` (+`snapshot` — both freeze state).

**The four-way `tag` split (C-gwz-2).** `TagOp` = {create, list, fetch, push,
delete} spans all four classes, so `tag` projects to **`tag-list`** (read),
**`tag-mutate`** (create/delete), **`tag-fetch`** (egress-read), **`tag-push`**
(egress-write). v2's two-surface local/remote split preserved only two classes —
it let a `tag-fetch` (remote READ) grant confer `tag-push` (remote WRITE). The
split is enforced by **disjoint, generated, closed DTO op-enums per surface**, NOT
a per-op runtime check: a `tag-list` provider cannot receive a `push` op because
`push` is not in its wire type. `stash`→`stash-list`/`stash-mutate` and
`branch`→`branch-list`/`branch-mutate` are the same least-privilege projection —
keeping them whole would let a list-only UI hold mutation authority (the ruling
makes no such exception).

## 3. Path-free DTOs on the shared path type (C-gwz-3, D14)

The canonical requests CARRY host paths (`CreateWorkspaceRequest.workspace_root`:957,
`AddExistingRepoRequest.repository_path`:983, `StageRequest.cwd`:1134,
`CloneWorkspaceRequest.target`:978, `RequestMeta.workspace.root` via
`WorkspaceRef`:499). **The wire payload is a glade-owned, path-free DTO per member**,
projected SERVER-SIDE into the canonical `Request`:

- The supplier FILLS `workspace_root` / `WorkspaceRef.root` / `cwd` /
  `repository_path` / clone `target` from the **ID→root resolution** (the stable
  workspace/share ID is the sole authority input, E-ws-1; glade-workspaces §1.2
  owns the map) or, for the creation arm, from the **app-allocated new root** (§5).
  The DTO NEVER carries an absolute host path — the bring-up's `DENIED_ARGS`
  scope-guard becomes STRUCTURAL: there is no `--root` to refuse.
- **Every filesystem-facing field uses the shared `RootRelativePath` type (D14):**
  one normalize-and-safe-open utility that normalizes separators, **rejects
  absolute paths and parent traversal**, constrains symlink resolution to the
  selected root, and avoids check/use (TOCTOU) races. Path-free or ws-relative
  fields ride the wire: `snapshot_id`, the closed `TagOp`/`StashOp`/`BranchOp`
  sub-enums, `MaterializeTarget`, `Selection.member_ids`, `SourceUrl.url`, commit
  `message`, and ws-relative pathspecs (`StageRequest.pathspecs`, `member_path`,
  `DiffRequest.workspace_cwd`:1329 — resolved to `cwd`-relative server-side).

**The hard field is `add_existing_repo.repository_path`** — importing a repo that
may sit OUTSIDE the workspace tree. **Ruling (C-gwz-3):** the ONLY allowed forms
are (a) a **selected-root-relative path** (a repo already under the selected root,
as `RootRelativePath`) OR (b) a **server-issued, scoped, EXPIRING `RepoImportHandle`**.
An **arbitrary caller-supplied host path is FORBIDDEN** — AZ-1 alone cannot
sanctify it (a path-scoped grant does not make an unbounded host path safe).
Out-of-tree imports beyond a scoped handle are DEFERRED to a future contract (§14).

## 4. The shared kit — exchange + events + the closing result record (C-gwz-4, C-gwz-5, B2)

Every member instantiates ONE kit — the wire-attached authority session
(GladeSupplierModel §2), parameterised by its method. The kit mirrors the
**canonical trio**:

1. **Exchange: path-free DTO in → the METHOD-SPECIFIC typed response wrapper.**
   Not a bare `ResponseEnvelope`: `StatusResponse` adds `workspace_git_status`
   (1214), `LsResponse` adds `members[]`, `ListSnapshotsResponse` `snapshots[]`,
   `TagResponse` `tags[]`, `StashResponse` `bundles[]`, `BranchResponse` `repos[]`,
   `DiffManifestResponse` `files[]`/`summary`/`targets`/`output`. Short ops answer
   inline; long ops answer `{operation_id, aggregate_status: accepted}` first.
   `corr` preserved 1:1. **Decode is fail-closed (B2):** a malformed/oversized DTO,
   a wrong member, or a gwz-core failure returns a well-formed wrapper
   (`failed│partial│rejected`, typed `GwzError`) — never a panic, never a partial
   fold. Failure is DATA (`ExchangeRes.ok` stays `true`), never a hang.

2. **The events stream on a run-keyed log.** Each op's `OperationEvent` stream
   (`operation_started · member_started · member_progress` with
   `GitTransferProgress` · `member_finished · operation_finished`, 917–936) appends
   to the member's log keyed by `operation_id` — the `events.subscribe` shape=`log`
   (101); from-cursor backfill converges a late subscriber.

3. **The final `OperationResult` is the CLOSING replicated log record (C-gwz-4).**
   `OperationEvent` carries no final result (939–950 is a distinct message); the
   authoritative result is a **closing, replicated `OperationResult` TERMINAL
   record appended to the run-keyed events log** (`started`/`finished_at_ms`,
   `members[]`, `errors[]`, `aggregate_status`). Because it lives on the replicated
   log, it **survives provider restart** — the "accepted pull never yields final
   member/errors" gap is closed durably. A `operation.result(operation_id)` query
   (105) MAY exist as a **derived convenience VIEW** that projects the closing
   record, but is **NOT the authority**. This retracts v2's paired-result-surface-
   as-authority and restores v1's "closes the log", now with the durability
   guarantee made explicit.

**`glade-gwz-diff` has an extra out-log**: besides events, the `diff.output` byte
log (`DiffOutputRecord`, shape=`log`:122) keyed by `DiffManifestResponse.output.log_id`
— the shape_log cursor/EOF/close contract.

**Events-log visibility (C-gwz-5) — RATIFIED, not a grant-keyed zone.** All events
and result logs are **commons**, keyed by the **workspace share** (the zone key is
the share, never a grant id). Read authorization is a CHECK at read time, not a
separate zone: **local (read/local-mutate) events require MEMBERSHIP** in the
workspace share; **egress events, results, and `diff.output` require an EXACT
surface grant** on that member. v2's "grant-keyed zone" (a per-grant private log)
is retracted — **grant identifiers MUST NOT become zone keys** — so watching
`push`/`pull`/`tag-push` progress (remote URLs, errors) needs the egress surface
grant, enforced as a commons read check.

## 5. The creation arm — internal materializer leg + durable state machine (C-gwz-7, C-gwz-8, E-ws-1)

**Ownership splits at workspace-lifecycle vs repo-operation (C-gwz-7).**
`glade-workspaces` owns the **public `workspace.create`** ceremony. The three
create-materialize members (`create`, `init`, `clone-workspace`) are the
**INTERNAL materializer LEG** that ceremony's state machine invokes — NOT public
`gwz.create` exchanges racing it. Existing-workspace repo members
(`add_existing_repo`, `create_repo`, `clone_repo_member`) **stay PUBLIC gwz** — a
user adds/creates/clones a member on a workspace that already exists. **The
`ws.ops` façade is RETIRED.**

**Addressing is by stable ID (E-ws-1).** For the 22 existing-workspace members the
selection IS the **stable workspace/share ID** in the binding route — the sole
routing AND authorization identity. The node **resolves ID→root** server-side
(§3); a DTO that also asserts a root and MISMATCHES the resolved root **fails**;
**display names never route or authorize**. The DTO carries no selection token —
the addressed ID is the selection.

**Creation members cannot use selection — nothing exists to select** (F5-10):

- **Root = APP-ALLOCATED by the data seam**, never the request: the ID→path map
  GAINS an entry for the new workspace; `workspace_root`/`target` come from that
  allocation.
- **Events + the closing result ride the HOME share** (where `workspace.create`
  ceremonies live, `s-ws-create`) until the new workspace's share exists.
- **Durable create state machine (C-gwz-8):** `intent → materialized → registered
  → claimed → complete`. `intent` durably records the ceremony; `materialized` =
  disk root+members exist; `registered` = the `WorkspaceEntry` record commits;
  `claimed` = the `ServeClaim` commits; `complete` closes the ceremony.
  **Recovery is FORWARD-ONLY and idempotent** — a crash at any state replays
  forward (re-materialize is a no-op if disk exists; re-append is idempotent on
  record identity), never a compensating rollback, no orphan, no half-create. A
  disk failure before `registered` → `aggregate_status: failed` + **no** records
  (typed `GwzError`). **The HOME-share logs are authoritative** for this recovery.
  Governance records (`WorkspaceEntry`/`ServeClaim`) are signed ops (B5).

`init`/`clone-workspace` supply the populated/replica variants (how a 2nd peer
becomes an eligible host). Trace = glade-workspaces `s-ws-create` extended with
this leg (§11).

## 6. Requester context — consumed from the substrate (B3)

Attribution + stage-2 gating need a **NODE-AUTHENTICATED `ProviderCallContext`
delivered to the provider** — never a caller-payload principal. The built
`supplier.rs:146-147` trusts a caller-supplied `req.principal`: **the retired
anti-pattern** — forgeable; Eve sets `principal` and the provider cannot tell her
from the owner. **This family CONSUMES the B3 substrate seam, it does not build
it:** the node constructs a `ProviderCallContext` from the authenticated
Hello/session (requester principal, certified device, grant evidence, session
assurance, correlation, forwarding provenance) and delivers it **beside, never
inside, the request DTO**. The DTO carries **NO principal**; a DTO principal field
MUST NOT override the context. The supplier fills `OperationAttribution.actor`
(525–532) from the context principal (glade-users §1 fingerprint). Any effectful
follow-on uses the same authenticated principal unless an explicit delegation is
verified. Forwarders preserve the context; local and forwarded paths use the same
check.

## 7. Surfaces (the whole family)

Declared per GladeSupplierModel §3; the per-member PATTERN (N = 25) not 25×rows.
Of the 25, **22 are public exchanges** (existing-workspace members) and **3 are
internal create-arm legs** (§5).

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `gwz.<member>` (public exchange; the 22 existing-workspace members) | exchange | directed; addressed to the workspace SHARE by its **stable ID** (E-ws-1); node resolves ID→root, mismatch fails | path-free DTO (`RootRelativePath` fields) → the member's method-specific typed response wrapper (§4); long ops answer `{operation_id, accepted}`; fail-closed decode (B2), failure as data |
| `gwz.<member>.events` (one **log**/member, keyed `operation_id`; carries the CLOSING `OperationResult`) | log | **commons** (workspace share); grant ids are NOT zone keys. Read: local members by MEMBERSHIP, **egress events/results by EXACT surface grant** (C-gwz-5) | the run-keyed `OperationEvent` stream + the closing replicated `OperationResult` terminal record (§4, survives restart); a derived `.result(operation_id)` view MAY project it but is NOT the authority |
| create-arm legs (`create`/`init`/`clone-workspace`) | (internal) | HOME share (the `workspace.create` ceremony log) | invoked by glade-workspaces' `workspace.create` state machine (C-gwz-7); events + closing result ride the HOME-share create log until the new workspace's share exists (§5) |
| `gwz.diff.output` (`glade-gwz-diff` ONLY, keyed `log_id`) | log | commons (workspace share); membership read (local working-tree bytes) | `DiffOutputRecord` byte stream; `log_id` from `DiffManifestResponse.output`; shape_log cursor/EOF/close contract |

The root — the stable ID + the app-side ID→path map — is glade-workspaces' input,
consumed not owned (§3, §5). Working-tree diff is `glade-gwz-diff`'s;
**cross-surface** diff is glade-diff's (glade-diff §9).

## 8. Gating + stage split — security is STAGE-1 (B1–B5)

The model makes attribution a **stage-1 must**: "stage-1 allow-all" is neither
safe nor honestly exercisable without the security substrate, so B1–B5 are
STAGE-1, not deferred. Gating itself = ordinary per-surface grants, one per member
(no allow-list, no ActionKind vocabulary): `gwz.<member>` each take a
`CapabilityGrant` (`seed owner grazel gwz.*` in `grazel-app.glade` wildcard-covers
them); the per-member exchange IS the grant unit; the four-way `tag` split (§2)
keeps `tag-list`/`tag-mutate`/`tag-fetch`/`tag-push` as separate grant surfaces, so
the egress wall holds at grant time too.

- **Stage-1 (the family as data, ON the security substrate):** the §4 kit + §7
  surfaces, built on **authenticated, non-replaceable provider attach (B1)**,
  the **delivered `ProviderCallContext` (B3, §6)**, and **fail-closed decode (B2,
  §4)**. **The composition wall now rests on B1, not just "declare-or-not":** a
  provider MUST authenticate and present an exact `provider.attach(share_id,
  glade_id)` grant; an existing attachment is **never silently replaced**;
  handoff/detach advance a monotonic provider epoch and every call binds
  {principal, surface, epoch, glade}; stale epochs fail closed. So "compose only
  what you trust" is a HARD wall — attach the read members by default; a
  mutating/egress member is present iff its composition attaches it (an unattached
  member has no provider → `ExchangeRes{ok:false}` absence-as-data), AND no racing
  peer can hijack an attached surface. Attribution is real and honest from
  stage-1 (the actor is the authenticated principal, not a payload field).
- **Stage-2 (the grant CHECKS decide):** per-member surface grants gate; AZ-1
  path-scoping on `RootRelativePath`/`RepoImportHandle` fields; creation
  commit-or-fail enforced; egress members gated by their own grants + real
  credentials (`OperationAttribution.credential_ref` is a handle, not a secret);
  egress events/results read-gated by the surface grant (C-gwz-5). The identity
  these checks consume is already real from stage-1.

## 9. Packaging + dispatch (C-gwz-6)

A supplier is the unit of **declaration / grant / composition** — not necessarily
a repo or a process. **v1 (C-gwz-6): one `glade-gwz` repo** hosting the 25 members
over one shared kit crate (GDL-040 repo-says-what), **one process attaching N
authority sessions** (one provider per member's exchange + its events log with the
closing result record), and **gwz-core in-process** as a library. **A hard process
boundary around the egress members remains an OPTIONAL deployment profile**, not
the v1 default. The uniform kit (§4) plus the mechanical grain (§2) make the
family **derivable from the protocol IR**: generated DTOs, `.glade` declarations,
manifests, provider registration, and test expectations MUST all arise from the
**same IR** (the packet gate), and the IR-derived interface COUNT gates the build
(§2). N repos / N processes is the heavier alternative behind the same data seam.

## 10. Version discipline + migration from the P1.S2 bring-up

**Pinned, additive family.** Depends on **gwz-core v0.9.2 on taut v0.8.1**, PINNED
— `RequestMeta`/`ResponseMeta.schema_version` name it; a mismatch is failure as
data (`GwzErrorCode.schema_unsupported`). **A new request type in vNext = a NEW
member, additive** — existing members untouched. **Bugfix path** (RULED): gwz-core
MAY temporarily join glade-wz as a member and be modified, but changes push/pull
back to gwz-dev (kept working there) and likely cut a new release — never a
fork-in-place; glade-gwz re-pins.

**Migration (the reshape):**

- **`gwz.ops` → RETIRED**, replaced by 25 per-member surfaces (22 public
  exchanges + 3 internal legs). **The `ws.ops` façade is RETIRED too** (C-gwz-7).
  **The allow-list DIES**: `exec.rs::ALLOWED_VERBS`/`DENIED_ARGS` + the CLI
  passthrough are gone; the `{verb,args}` argv → the path-free DTO / typed wrapper
  (§3, §4). The scope-guard is subsumed by ID→root (§3, §5); the
  read/mutate/egress wall is authenticated composition (§8, B1); stage-2 is
  per-member grants.
- **`gwz.output` → per-member `gwz.<member>.events`** (each carrying its closing
  `OperationResult` record, §4) — NOT a separate `.result` exchange (v2's design,
  retracted by C-gwz-4).
- **`grazel-app.glade` grows** to {25 members' services + per-member events
  bindings + `gwz.diff.output`}. It is a **dual-maintained node-test fixture with
  record-count assertions** (`glade/node/{appdecl,exchange}.rs`; P1.S3 note #1/#2)
  — the reshape MUST update the counts (now IR-derived — the count-drift gate, §2),
  the asserted id set, and BOTH byte-identical copies (`glade/apps/` +
  `grazel/apps/`).
- **grazel composition** (P1.S3's `--gwz-supplier-bin` child) generalises: grazel
  attaches the members it trusts (read-only = the seven read members), one binary /
  N sessions per §9; non-fatal spawn preserved.

## 11. Traces to author before building (atlas leads)

- **s-gwz-status** — a path-free status DTO → `StatusResponse` + `WorkspaceGitStatus`;
  the dashboard renders. Proves the typed wrapper (not a bare envelope) +
  selection-as-stable-ID (E-ws-1) + no path on the wire + the `ProviderCallContext`
  actor present from stage-1 (B3).
- **s-gwz-dto-project** — the path-free DTO → server-side projection into the
  canonical `Request`: host-path fields filled from **ID→root resolution**, all
  filesystem fields normalized through `RootRelativePath` (D14); a DTO carrying an
  absolute host path, a `..` traversal, or a root mismatching the ID is refused as
  data (C-gwz-3).
- **s-gwz-stream** — a long op (`pull_head`) streams `OperationEvent`s on the
  run-keyed log; a late subscriber backfills; the **closing `OperationResult`
  record** yields the final `members[]`/`errors[]` and **survives a provider
  restart** (C-gwz-4); a derived `.result` view projects the same record.
- **s-gwz-create** — `glade-gwz-create` materializes an empty workspace as the
  `workspace.create` state machine's **internal host-local leg** (C-gwz-7): app-
  allocated root, events on the HOME share, the `intent→materialized→registered→
  claimed→complete` machine (C-gwz-8) commits records ONLY after disk success; a
  disk failure → `failed` → no records. THE seam shared with `s-ws-create`.
  - arm **s-gwz-create-recover** — a crash at EACH state (after `materialized`,
    between `registered` and `claimed`, etc.) replays **forward-only and
    idempotently** to `complete` (no orphan, no rollback); the HOME log is the
    recovery authority.
  - arm **s-gwz-init** — `init_from_sources` clones N sources (egress, indivisible),
    one unreachable under `PartialBehavior` → `aggregate_status:partial`.
- **s-gwz-tag-egress** — the **4-way** op-enum wall (C-gwz-2): `tag-list`{list} /
  `tag-mutate`{create,delete} / `tag-fetch`{fetch} / `tag-push`{push} on **disjoint,
  closed generated DTO enums**. A read+local composition attaches `tag-list` (+
  `tag-mutate`) only → `tag-fetch`/`tag-push` have no provider → remote tag
  fetch/push = absence-as-data; and a `tag-list` provider **structurally cannot
  receive** a `push` op (it is not in its wire type — no runtime check).
- **s-gwz-compose-readonly** — a composition attaching ONLY the seven read members;
  the mutating/egress panels are structurally absent (no provider → absence as
  data). The allow-list's heir.
- **s-gwz-requester-ctx** (B3) — attribution from the node-authenticated
  `ProviderCallContext`, delivered BESIDE the DTO; a DTO with a forged `principal`
  field is ignored (there is no principal field to honor); local and forwarded
  paths agree.
- **s-gwz-provider-hijack-negative** (B1) — a peer attempting to attach as the
  provider for a live `gwz.push` (or any member) surface is REJECTED: attach
  without a grant, a foreign-share attach, an undeclared provider class, and a
  silent-replacement attempt all fail closed; a stale-epoch call after an
  authorized handoff fails. Proves the composition wall is B1, not "declare-or-not".
- **s-gwz-push** (stage-2) — `push` denied at the SURFACE by a missing `gwz.push`
  grant (not an allow-list) → failure as data; and a member WITHOUT the grant
  cannot read the egress events/result log (commons + surface-grant check, C-gwz-5).

## 12. Dependencies + user-testable-when

- **Depends on:** glade-workspaces (the stable ID→path map = the root, and the
  `workspace.create` state machine the creation arm legs into), glade-users
  (attribution — the fingerprint principal, §6), gwz-core **v0.9.2** release, base
  glade (exchange + log + the kit). **Consumes but does not build** the B1
  attach-policy and B3 requester-context seams. Otherwise a RESHAPE (contrast
  glade-terminal / glade-diff).
- **User-testable when:** each member's panel exercises ITS flow on one of MY real
  selected workspaces — the status dashboard renders live; capture writes a named
  snapshot; pull streams per-member progress and yields a durable final result;
  create materializes a new empty workspace (records + disk together, forward-only
  recoverable); local tag list/create works while remote tag fetch/push is absent
  in a read+local composition; and a composition WITHOUT the mutating members
  visibly lacks their panels.
- **This family "working" is the G-razel gate** (RulingWorksheet §VII): no `razel.*`
  interface is normative until the generated GWZ inventory, provider composition,
  per-surface authorization, operation-result closure, and one real local + one
  forwarded integration path all pass.

## 13. Review dispositions (finding → ratified ruling)

- **SR56-01** (stale 12-method inventory) → regenerated: 24 methods + per-method
  capability audit (§1).
- **SR56-02 / C-gwz-3 / D14** ("canonical Request" ≠ path-free; no shared path
  algo) → glade-owned path-free DTO, server-side projection, every filesystem
  field on the shared `RootRelativePath` type; `repository_path` = ws-relative
  under-root OR scoped expiring `RepoImportHandle`; arbitrary host paths FORBIDDEN
  (§3).
- **SR56-03 / C-gwz-1 / C-gwz-2** (merges cross the wall; grain) → the mechanical
  25-interface grain, IR-derived and count-gated (§2); `tag` split **four ways** on
  disjoint closed enums; `stash`/`branch` split read/mutate; `pull_snapshot`
  regrouped with `materialize`.
- **SR56-04 / B3** (forgeable `req.principal`) → node-authenticated
  `ProviderCallContext` delivered beside the DTO; the DTO carries no principal; the
  seam is substrate this family consumes (§6).
- **SR56-14 / C-gwz-4 / C-gwz-5** (bare envelope / result / visibility) →
  method-specific wrappers + events stream + the **closing replicated
  `OperationResult` record** (survives restart; a `.result` view is derived, not
  authority); events logs are **commons** with membership/surface-grant read checks
  — no grant-keyed zone (§4).
- **SR56-15 / C-gwz-8** (create = uncomposed ceremonies) → the durable
  `intent→materialized→registered→claimed→complete` state machine with forward-only
  idempotent recovery; HOME logs authoritative (§5).
- **SR56-16 / E-ws-1** (selection has no path to the authority) → the **stable
  workspace/share ID** in the binding route is the sole routing+authz identity;
  node resolves ID→root, mismatch fails, display names never route (§5).
- **F5-10 / C-gwz-7** (create-family breaks the kit; public ceremony collision) →
  the creation arm is the **internal materializer LEG** of glade-workspaces'
  public `workspace.create`; existing-workspace repo members stay public; the
  `ws.ops` façade is retired (§5).
- **F5-11 / C-gwz-5** (events visibility outruns the grant) → commons logs, local
  reads by membership, egress events/results/output by exact surface grant; grant
  ids are not zone keys (§4).
- **SR56-2-03 / B1** (provider-attach LWW hijack) → authenticated, non-replaceable,
  epoch-guarded attach; the composition wall rests on B1 (§8).
- **SR56-2-02 / B2** (panic-on-decode DoS) → fail-closed typed decode; failure as
  data, no partial fold (§4).

## 14. Residual opens (post-ratification)

- **Events-log packaging granularity:** one `gwz.<member>.events` log per member vs
  one shared `gwz.events` keyed by `(member, operation_id)`. The VISIBILITY model
  is ruled (commons + membership/surface-grant, C-gwz-5); the log COUNT is an
  implementation packaging choice.
- **Out-of-tree repo import:** the `RepoImportHandle` issuance protocol (who mints
  it, its scope + expiry shape) and the DEFERRAL of arbitrary-host adds to a future
  scoped-handle / AZ-1 path-grant contract — C-gwz-3 forbids arbitrary host paths
  in v1.
- **Generation depth:** full IR codegen of the family vs IR-checked hand-authoring.
  Either way the packet gate holds — DTOs, `.glade` declarations, manifests,
  registration, and tests MUST share one IR, and the interface count is drift-gated
  against it (§2, §9).
