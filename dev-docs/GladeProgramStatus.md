# Glade Program Status — the one page that tracks the moving parts

Status: LIVING document — update whenever anything changes stage.
Last update: 2026-07-10.

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
| Workspace directory / home share / discovery | **Ratified** (GDL-032, 2026-07-07) | `glade/GladeWorkspaceDirectory.md` + s-discovery… | build B3 (Lane R step 3) |
| Authz (grants, check(), verbs, ownership) | **Ratified** (GDL-031/033/034, 2026-07-07) | `glade/GladeAuthzModel.md` + stage-2 traces | build after E2E stage-1 |
| Operators / placement / roaming | **Ratified** (GDL-031, 2026-07-07) | AZ §7a/§7b + s-roam/s-tenant/s-local-guest | build with stage-2 |
| Sync / checkpoints / migration / service ads | **Ruled** (AZ-12, dedup, repair owner) | s-sync / s-svc-shared / s-migrate | build with B2 |
| Glial client runtime (persistence-first, assembly, rich events) | **v0 Built** (2026-07-10: glial 4270235+d94c49c — mount/unmount instances (idiom-agnostic), store-only path, value+log folds ORACLE-GATED (11/11 + 6 assembly vectors), injectable session seam; s-stack-multi trace 0938190) | `glial/GlialClientRuntime.md` + s-stack-* + `glial/dev-docs/DecisionLog.md` (GAP-1..5) | **T3 BUILT 2026-07-10**: grip adapter (glial a31587f, kernel stays grip-free) + per-binding cutover DONE (glade e2679d6..3dfc2e6 — GripShareBinder DELETED, decl/manifest plumbing remains, coverage 12→17 with per-binding wire-byte interop proofs). GAP-9 CLOSED 2026-07-11 (glial 6d05757+bb53122: two-way attachGlade hydration, semantic echo guard, IndexedDbStoreEngine; glade f63652f+88ba2b2: demo adopts natively, per-tab identity RULED, offline-from-boot regression — grip-share 18). Recorded open: GAP-11 offline outbox (mintLocal ops never ship on later attach — last silent-replication hole), GAP-10 tail (per-tab IDB stores never evicted; retention TTL unenforced), GC-2 conflation, dump-trim cosmetic |
| glade-decl seam | **Built** (2026-07-08: contract ccdae14 + ts/rs/py renderings + grip-core swap 8965577, corpus-gated ×3 langs) | `glade/GladeDeclSurface.md`, `glade-decl/` (canonical; `glade/decl/` = superseded seed) | Lane T step 2: glial binder v0 (home: `glial` member) |
| `<app>.glade` / `glade-sys.glade` (app/substrate split, mgmt surface) | **Built** (R4 2026-07-11: glade b9b1126+39bd59a — grazel-app.glade loaded as DATA, seeds→CapabilityGrants under registrant chain, diff-idempotent, revocation-wins survives re-load; exchange routes to AUTHORITY via C2, failure = ExchangeRes data; node 52) | `glade/apps/grazel-app.glade` + `glade/dev-docs/GladeGrazelAttachNotes.md` | **ALL E2E STAGE-1 BUILDERS DONE → DoD audit next**; FLAG: sysdata regen needed --legacy-codec (taut ≥v0.8 fail-closed codec vs frozen wire runtime; flag dies at taut v0.10 — migration follow-up) |
| System-data seam + `~/.glade/sys` layout + data classes | **Built** (2026-07-08: glade 7394ce5+2dc3545) | `glade/GladeSystemDataSeam.md` + notes | — |
| Peer sync (iroh carrier, HELLO, per-(origin, zone) heads/gap, verify-as-ingest, equiv proofs) | **Built** (2026-07-10: glade 4789177+02e4522, taut c0758b7 NodeHello/Welcome; store chains were ALREADY per-(origin, zone); crypto stubbed-structure-real) | s-sync (reframed) + `glade/dev-docs/GladePeerSyncNotes.md` | **R3 BUILT 2026-07-10** (glade 6667360+4d190a2: accept loop, dir.workspaces as ordinary binding, C2 claim routing, absence-as-Error; node 45, E2E×5 stable). R4 BUILT 2026-07-11 → **E2E STAGE-1 AUDITED 2026-07-11: MET** (live composed run + 11-trace conformance sweep; `dev-docs/GladeE2EStage1Audit.md`). Closure items: F1 live WorkspaceEntry/ServeClaim minting (small) · F2 s-create target-routing deferral/ruling · F4 SubstrateV1 §11 stale-list sweep. Then STAGE-2 switch-on |
| taut-shape consolidation | **P1+P2 Built** (P2 2026-07-10: 389c867 — glade's 12 M-LIMP fold vectors re-homed as fold.v0.json + conflict tripwire, NO conflicts; glade oracle test repointed 6295061; S3 value-matrix assessed ~700-900 LOC, deferred) | `taut-shape/dev-docs/TautShapeGladeConsolidation.md` | S3 value matrix (de-log-hardwire CLIs) when called |
| Trace atlas (ggg-viz) | Built, leading | 29 traces · 5 invariants · 228 tests · comment loop (s-boot/s-app-register/s-zones landed 253518e) | s-stack-multi landed 0938190 (228 tests); queue: s-discovery already authored — next new traces ride Lane R3 |
| Dynamic grip-context sharing (headless AI) | Deferred by design | GDL-037 note; GDL-004/030 | after E2E |
| grazel + gryth suppliers (GLP-0006) | **ACTIVE — P0 Built 2026-07-12** (P00 ruled: wire-attach, glade-wz homes, F2 BUILT; F1+F2 live in glade 8d6f0cd..d872838; supplier kit + typed manifest glial 4dfbd23; rust client-rs; grazel skeleton 56d9a32 → repo grazel-node; supplier model doc + GDL-040) | `plan-docs/plans/GLP-0006-grazel-gryth-suppliers/` + `dev-docs/glade/GladeSupplierModel.md` | P1 S1-S3 BUILT 2026-07-12 (glade-chat + glade-gwz + grazel composition; demo Chat/Gwz tabs live; atlas 241). S4: gryth-ui wiring (gryth-wz at ~/limbo/gryth-wz; grip-core pin-skew check first) |
| glade-dev repo extraction | **Decided: YES** (2026-07-07) | glial-runtime home = new repo `glial-runtime`, member path `glial` (old `owebeeone/glial` = glial-dev's remote, untouched) | create member + seed |

## Decision queue (Gianni)

- ~~Ratify flips~~: GDL-031…038 **ratified 2026-07-07** (DecisionLog flipped).
- **Product calls**: AZ-1 (path-scoped grants v1?) · AZ-2 (multi-user local
  nodes day-one?) · AZ-3 (OIDC v1?) · WD-1 (root custody — the big one) ·
  WD-4 (guest directory visibility) · WD-6 (entry fleet conventions) ·
  AZ-7/13 (org custody; quorum/handover).
- ~~s-zones INV-4 questions~~ **RULED 2026-07-10 (AZ-16/AZ-17)**: (a) private
  rides the MEMBERSHIP grant — revoking membership cuts commons AND private;
  (b) account domains are OWNER-exempt by identity (owner-scoped carve-out,
  non-owners stay gated). INV-4 + s-zones updated; suite 222 green.
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
4. ~~s-sync framing note~~ — done 2026-07-10: trace reframed to
   per-`(origin, zone)` chains (heads, ranges, fork slots + filterability note).
5. ~~Decide glial-runtime home~~ — decided 2026-07-07: extraction YES; new repo `glial-runtime`, member path `glial`.

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
   tap→glade coupling. Mounts create binding INSTANCES `(decl, domain/zone/
   key fill)` — several per decl, refcounted; the seam is mount/unmount and
   is grip-idiom-agnostic (never references MatchingContext) — clarified
   2026-07-10, GlialClientRuntime §Boundaries.

**Lane C (contracts)** — parallel:
1. taut-shape P1: `shape_value` contract + corpus.
2. P2: glade fold oracles merge into taut-shape corpora.

**Atlas stays ahead**: s-boot + s-app-register authored before Lane R
steps 1/4 build them.

E2E stage-1 definition of done: browser (glial binder) ↔ glade-local ↔
glade-peer over iroh; workspace list from the registry; one grazel binding
live; one gwz exchange round-trips; all snapshot files under `~/.glade/sys/`;
every behavior matching its trace.
