# Glade Program Status — the one page that tracks the moving parts

Status: LIVING document — update whenever anything changes stage.
Last update: 2026-07-07.

> **HOME: `~/limbo/glade-wz`** (gwz workspace — `gwz clone`, never `git
> clone`) as of 2026-07-07. Docs corpus + ggg-viz live here; glial-dev is the
> FROZEN fallback (its copy of this file is stale by definition). Workspace
> LIVE 2026-07-07: all 10 members registered + lock-pinned (`glade`,
> `grip-core`, `grip-react`, `taut`, `taut-shape` cloned in; glade-decl-ts/rs/py
> still empty shells), root + members pushed to github/owebeeone, ggg-viz grip
> deps re-pointed to `../grip-core`/`../grip-react` (183 tests green).
> `glial-wz` and `grip-wz` follow.

Stage model: **Design → Proposed (doc + atlas traces) → Ruled (Gianni decided
specifics) → Ratified (GDL flipped, frozen-unless-thawed) → Built (code, gated)**.

## Where everything stands

| Area | Stage | Artifact | Next step |
| --- | --- | --- | --- |
| Substrate core (op model, frames, folds lww+log, WS carrier) | **Built** (M-LIMP, oracles frozen) | `glade/dev-docs/GladeSubstrateV1.md` | rebase shapes onto taut-shape (P2) |
| Workspace directory / home share / discovery | Proposed | `glade/GladeWorkspaceDirectory.md` + s-discovery… | ratify GDL-032; build B3 |
| Authz (grants, check(), verbs, ownership) | Proposed (+3 rulings) | `glade/GladeAuthzModel.md` + stage-2 traces | ratify GDL-031/033/034; build after E2E stage-1 |
| Operators / placement / roaming | Proposed | AZ §7a/§7b + s-roam/s-tenant/s-local-guest | ratify GDL-031 |
| Sync / checkpoints / migration / service ads | **Ruled** (AZ-12, dedup, repair owner) | s-sync / s-svc-shared / s-migrate | build with B2 |
| Glial client runtime (persistence-first, assembly, rich events) | Proposed | `glial/GlialClientRuntime.md` + s-stack-* | build Lane T |
| glade-decl seam | Proposed + skeleton | `glade/GladeDeclSurface.md`, `glade/decl/` | **Lane T step 1** |
| `<app>.glade` / `glade-sys.glade` (app/substrate split, mgmt surface) | Proposed | GDL-037/038, decl doc | atlas: s-app-register |
| System-data seam + `~/.glade/sys` layout + data classes | **Ruled** | `glade/GladeSystemDataSeam.md` | **Lane R step 1**; atlas: s-boot |
| taut-shape consolidation | Planned | `taut-shape/dev-docs/TautShapeGladeConsolidation.md` | Lane C step 1 (shape_value) |
| Trace atlas (ggg-viz) | Built, leading | 25 traces · 5 invariants · comment loop | s-boot + s-app-register |
| Dynamic grip-context sharing (headless AI) | Deferred by design | GDL-037 note; GDL-004/030 | after E2E |
| glade-dev repo extraction | **Undecided fork** | — | Gianni: yes/no/when |

## Decision queue (Gianni)

- **Ratify flips**: GDL-031…038 all sit at *open (proposed)*.
- **Product calls**: AZ-1 (path-scoped grants v1?) · AZ-2 (multi-user local
  nodes day-one?) · AZ-3 (OIDC v1?) · WD-1 (root custody — the big one) ·
  WD-4 (guest directory visibility) · WD-6 (entry fleet conventions) ·
  AZ-7/13 (org custody; quorum/handover).
- **Design-owned, non-blocking**: AZ-14/15 (cosign mechanics, agent
  enrollment), GC-1…4 (event schema home, backpressure, binder migration,
  browser store engine).

## Repo reconciliation (audited 2026-07-06)

Findings:

- **grip\* has NO fork**: gryth-dev's `grip-core`/`grip-react`/`grip-react-demo`/
  `grip-lab` are SYMLINKS into glial-dev (since Jun 10). One working copy,
  two paths — document, don't reconcile.
- **The real drift = the parked ZONES iteration** (`GladeZones.md`,
  implemented+verified 2026-06-14, GDL-039): grip-core `ShareDecl`
  +domain/zone and glade `client-ts/{client,session,store}.ts` + demo changes
  sit UNCOMMITTED since ~Jun 14. Design corpus reconciled 2026-07-06
  (glade-decl, AZ §4a, GDL-039); code commit adjudication is Gianni's.
- **glade checkouts**: taut-dev/glade is a clean strict ancestor of
  glial-dev/glade — fast-forward on request. glial-dev/glade is the working
  checkout (per the doc-home rule) and also carries the untracked `decl/`
  skeleton.
- **grip-lab**: frozen Jun 4 (iroh-debug era) — SUPERSEDED by the glade node;
  keep archived, lessons already harvested (SubstrateV1 §9, GDL-035).
- **grip-vue(+demo)**: dormant since 2025-11 — parked, out of scope.
- **glial-core/py/react/server**: empty submodule stubs. The GDL-035
  glial-runtime is a FRESH build (Lane T); its home rides the glade-dev
  extraction fork.

Actions (owner):

1. Commit the zones work — grip-core + glade dirty files (Gianni: adjudicate
   & commit; load-bearing for Lane T's grip-core swap, which must not land on
   a dirty tree).
2. Fast-forward taut-dev/glade or retire it to read-only (Gianni: git call).
3. Atlas: add `s-zones` (commons vs private keying — privacy WITHOUT a grant,
   plus a commons join gated by one) to the queue with s-boot/s-app-register.
4. s-sync framing note: chains are per-`(origin, zone)` after D8-refined
   (update trace notes when touched next).
5. Decide glial-runtime home (with the extraction fork).

## The build: three parallel lanes to E2E (stage-1 posture first)

The atlas's stage split IS the build phasing: **build stage 1 end-to-end
(allow-all, seams present), then switch stage 2 on** — the traces are the
spec for both.

**Lane R (rust node)** — critical path:
1. `SystemSnapshot` taut msg + RegistryApi/StoreApi traits + `~/.glade/sys/<name>/`
   layout + profiles/CLI (`glade-local|peer|server`, `--name`) + load-validation
   ladder. Conformance test: blob impl ≡ future fold impl.
2. iroh carrier + node↔node HELLO + heads/gap sync (s-sync minus checkpoints).
3. Directory minimal: WorkspaceEntry + ServeClaim in the registry; C2 claim
   routing; serve `dir.workspaces` (the s-discovery golden path, E2E).
4. grazel attach: `grazel-app.glade` loaded → authority session serves one
   real binding (ws.tree or terminal log) + one gwz exchange.

**Lane T (TS / glial)** — parallel:
1. glade-decl schema → generated rs/ts → grip-core swaps inline types
   (compile wall proves the seam).
2. Glial binder v0: persistence-first store-only path (s-stack-local
   behavior), then mount→session (s-stack-connect); retires direct
   tap→glade coupling.

**Lane C (contracts)** — parallel:
1. taut-shape P1: `shape_value` contract + corpus.
2. P2: glade fold oracles merge into taut-shape corpora.

**Atlas stays ahead**: s-boot + s-app-register authored before Lane R
steps 1/4 build them.

E2E stage-1 definition of done: browser (glial binder) ↔ glade-local ↔
glade-peer over iroh; workspace list from the registry; one grazel binding
live; one gwz exchange round-trips; all snapshot files under `~/.glade/sys/`;
every behavior matching its trace.
