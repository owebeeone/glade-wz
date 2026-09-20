# Package Extraction Plan

Date: 2026-09-20. Owner: root engineering / Gianni.

Status: **plan, for correction.** Nothing here is a ruling. No extraction, publish, member
retirement or document move has been performed; this document was written read-only against the
three workzones as they stand on 2026-09-20.

Scope: the three gwz workzones under `/Users/owebeeone/limbo` — `glade-wz` (19 members),
`gryth-wz` (`gryth-ui` plus duplicate checkouts of `grip-core`, `grip-react`, `glade-decl-ts`,
`glial`), and `gyld-wz` (`gyld`). `wyred-wz` is out of scope and stays a sibling; the
`link:../../wyred-wz/wyred-ui/packages/plugin-wyred` edge in `gryth-ui/package.json` is left
exactly as it is.

Related and unexecuted: [DocStructurePlan.md](DocStructurePlan.md) already proposes that the root
repository stop owning every subsystem document; [ReleaseProcessIdeas.md](ReleaseProcessIdeas.md)
(2026-06-05) already argued for a package graph and a release orchestrator;
[GladePackageArchitecture.md](GladePackageArchitecture.md) and
[LibraryBoundaryAndTestingPolicy.md](LibraryBoundaryAndTestingPolicy.md) hold the boundary policy
this plan assumes. Section 2's facts extend
[GlialFitAssessment-2026-09-15.md](glial/GlialFitAssessment-2026-09-15.md) and
`gryth-wz/dev-docs/GrythReadiness-2026-09-13.md`.

---

## 1. Position

The long-term shape is **packages, not a bigger workspace.** A workspace is scaffolding for the
period while the seams between repositories are still moving. Eventually `gryth-ui` is an
npm-installable components harness and most of the other repositories are dependencies rather than
checkouts. So consolidation should be **cheap and temporary** — `glade-wz` as the single home,
mainly to end the four duplicate checkouts — and the real work is **extraction**: TypeScript leaves
first, each package leaving the workspace when its contract has stopped moving, until what remains
is what is actively co-developed plus a small demo app repository that depends on versions.

**The trigger for extracting a package is contract stability, not tidiness.** Documents live in the
repository they describe, because root-level workzone documents are a symptom of work with no home.
**The measure of progress is that the workspace shrinks.**

**The counterweight: publishing freezes a contract, and the estate has already broken that promise
twice.** `@owebeeone/grip-core@0.2.1` was published 2025-11-25. Nineteen commits have landed in
`/Users/owebeeone/limbo/glade-wz/grip-core` since, thirteen of them after
`322390b "Bump version to v0.2.1"`, including public-surface changes
(`1daa6e9 "Roll-build step 1/2 keyed contexts"` and `"export keyed context types"`,
`03a56ec "removeParent re-resolves consumers"`, `1cf6ab3 "ShareDecl gains domain/zone"`,
`8965577 "share decl types ride glade-decl"`). The version is still `0.2.1`. The same is true of
`@owebeeone/grip-react@0.2.0`: ten commits since publish, including `c46e14f "Add new hooks"`.
The registry copies and the workspace copies are different contracts wearing the same number.

That is the real hazard, and it says where the stability gate belongs. Publishing does not freeze a
contract by itself — it freezes the *number*. The gate has to bite **before** the publish: a
candidate leaves the workspace when its public surface has stopped moving *and* a version bump is
part of the act of changing it. Where that is not yet true, the honest move is to keep the member in
the workspace and say so, not to publish and then quietly drift.

---

## 2. Where things stand

One row per candidate. "Published" means present on the registry named. Stability evidence is commit
counts in the window 2026-07-26 → 2026-09-20 (8 weeks), with public-surface counts measured against
each repository's declared entry points.

| Track | Candidate → package | Published? | How consumers reach it today | What blocks extraction | Stability (evidence) |
|---|---|---|---|---|---|
| TS | `taut-shape-ts` → `@owebeeone/taut-shape` | **npm 0.9.2** (0.9.0/0.9.1/0.9.2, all 2026-08-26) | **by version**: `glial/package.json` `"@owebeeone/taut-shape": "^0.9.1"` | nothing | **Settled.** 7 commits, 1 touched `src/`; HEAD *is* the release commit `137f843`; 0 commits since publish. Builds on `prepack`, ships `dist` + `.d.ts` + a `bin`. **The reference shape.** |
| Py | `taut` → PyPI `taut-proto` | **PyPI 0.9.1** (8 releases from 0.1.0) | **by path**: `glade-decl/corpus/build.py:39` and `glade-decl-py/tests/conftest.py:16` both `sys.path.insert(0, WORKSPACE/"taut"/"src")` | nothing in the package; consumers still bootstrap from the sibling. HEAD is 3 commits past `v0.9.1`, so a tag is due | 6 commits, 2 surface, last 2026-08-29. Release path exists: `.github/workflows/publish.yml`, PyPI Trusted Publishing (OIDC), 3.10–3.13 matrix, `setuptools-scm` from `v*` tags. **The reference shape for Python.** |
| Rust | `taut-shape-rs` → crates.io `taut-shape` | **crates.io 0.9.2** | no consumer in these three workzones (the TS port is what `glial` uses) | local workspace is at `0.9.1` (`tag v0.9.1`), **one release behind** `origin/main` `df13036 tag v0.9.2`. `crates/taut-shape-tool/Cargo.toml:14 publish = false`; its `path` dep carries no companion `version =` | 5 commits, 1 surface (2026-08-22). Publish workflow on GitHub release, `CARGO_REGISTRY_TOKEN`, and a `workflow_dispatch` input described as "Existing immutable release tag to verify" |
| TS | `grip-core` → `@owebeeone/grip-core` | **npm 0.2.1**, 2025-11-25 — **but drifted** | `file:../grip-core` (grip-react, ggg-viz, grip-react-demo, glial devDep); `workspace:*` + `overrides` in gryth-ui | the published 0.2.1 declares **no dependencies**; the working tree's 0.2.1 declares `"@owebeeone/glade-decl": "file:../glade-decl-ts"` on an **unpublished** package. It is type-only (`src/core/share_decl.ts:17 import type`) and survives only in `dist/index.d.ts` — so the runtime is clean but the **published types are unresolvable**. Must republish at a new version, and cannot until `@owebeeone/glade-decl` is installable | 0 commits in the window (dormant since 2026-07-10) **but 13 public-surface commits since the version was last bumped**. Dormant, not settled |
| TS | `grip-react` → `@owebeeone/grip-react` | **npm 0.2.0**, 2025-11-25 (`next: 0.1.0`) — **drifted** | `file:../grip-react`; `workspace:*` + override in gryth-ui; peer-declared by every `@grythjs/*` package | published manifest says `"@owebeeone/grip-core": "^0.2.1"`; the tree says `file:../grip-core` — the publish rewrote it by hand. 10 commits since publish at an unchanged version. Also carries an upstream gap: `GripContext.addParent` does not notify the resolver, which `gryth-ui` works around at `packages/desktop/src/tabContexts.ts:98` (`home.getGrok().resolver.addParent(home, source)`); the symmetric fix already landed for `removeParent` in grip-core `03a56ec` | 0 commits in the window (dormant since 2026-07-11) |
| TS | `glade-decl-ts` → `@owebeeone/glade-decl` | **absent** (`0.0.0`) | `file:../glade-decl-ts` (grip-core, glial, glade-chat); `link:` (glade/demo, glade/grip-share); `workspace:*` + override in gryth-ui | `0.0.0`; source-only (`main`/`types`/`exports` all `./src/index.ts`); no build script; no LICENSE file. **And its catalogue is unreconciled**: GlialFitAssessment §1 — `glade-decl/ir/glade_decl.taut.py` still enumerates `message`/`window` and never gained `atom`, against GDL-041 | 2 commits in the window (2026-08-29), 4 in its whole history. **Quiet, not settled** — it is a generated port of `glade-decl`, and `glade-decl` is where the drift is. Contract hash pinned at `99a04e0b960d03cbe92c0ec17321761eda860845` |
| — | `glade-decl` (the contract repo) | n/a — **no build manifest at all** | `corpus/build.py:36` writes `../glade-decl-rs/src/vectors.rs`; `:39` imports taut from `../taut/src` | not a package; the renderer needs its siblings checked out at fixed relative positions to run | 2 commits, both surface (`ir/`, `corpus/`), 2026-08-29 |
| Rust | `glade-decl-rs` → crate `glade-decl` | **absent** (`0.0.0`) | `glade/contracts/binding-api/Cargo.toml:12` `glade-decl = { path = "../../../glade-decl-rs" }` — a three-level escape out of `glade` | `Cargo.toml:8 publish = false`; `0.0.0`; `license = "MIT"` with no LICENSE file on disk | 2 commits, both surface, 2026-08-29 |
| Py | `glade-decl-py` → dist `glade-decl` | **absent** (`0.0.0`) | tests bootstrap taut by `sys.path` (above) | `0.0.0`; no LICENSE file; no `[project.scripts]`; declares **no dependencies** although it needs taut at test time; no readme/authors/urls/classifiers | 2 commits, both surface, 2026-08-29 |
| TS | `glial` → `@owebeeone/glial-runtime` | **absent** (`0.0.0`, `private: true`) | `file:`/`link:` from glade-chat, glade/demo, glade/grip-share; `workspace:*` + override in gryth-ui, consumed through **three subpath source exports** (`/grip`, `/manifest`, `/supplier`) | `private: true`; `0.0.0`; source-only exports; `tsconfig.json` sets `noEmit: true`, so nothing emits today; no LICENSE. Relative imports are extensionless, so an emit needs no specifier rewriting | 5 commits, 3 surface, last 2026-08-29. Its own GAP ledger records persistence (GAP-10/11) and the offline outbox as open — GlialFitAssessment §5. **Not yet** |
| TS | `glade/client-ts` → `@glade/client-ts` | **absent** (`0.0.0`, `private: true`) | **deep source paths**: `gryth-ui/packages/glade/src/runtime.ts` imports `@glade/client-ts/src/session.ts`, `/src/client.ts`, `/src/taut/schema.ts` | `private: true`; no `main`, no `exports`, no `files`, no `license`; only script is `test`; the `@glade` npm scope is not owned by anyone we can see. Deep `src/*.ts` imports bypass any exports map that is added | `glade` 6 commits, 4 surface, last 2026-09-13 ("draft application-side contract workspace"). The *wire* has not changed since 2026-07-12 (GlialFitAssessment §2) |
| TS | `glade-chat` → `@owebeeone/glade-chat` | **absent** (`0.0.0`, `private: true`) | `file:../../../../glade-wz/glade-chat` from `gryth-ui/packages/glade` and `packages/plugins/chat` — a four-level cross-workzone hop | `private: true`; `0.0.0`; source-only exports; no LICENSE | **Dormant** — 0 commits in the window, head 2026-07-12. Its only consumer of the glial supplier kit is its own tests; the chat plugin runs stage-1 with the supplier out of the message path |
| TS | `@grythjs/plugin-api` | **absent** (`0.1.0`, `private: true`) | `workspace:*` | `private: true`; `exports` is `./src/index.ts`; no build; no license | **2 commits**, both in `src/registry.ts`; `src/index.ts`, `src/grips.ts`, `src/runtime.ts` each 0. 281 lines total. **The most settled thing in gryth-ui** |
| TS | `@grythjs/desktop` | **absent** (`0.1.0`, `private: true`) | `workspace:*` | as above, plus `src/index.ts:4 import './desktop.css'` (a stylesheet that must ship), plus the grip-react workaround at `src/tabContexts.ts:98` | 10 commits |
| TS | `@grythjs/glade` | **absent** (`0.1.0`, `private: true`) | `workspace:*` | as above, plus `file:` cross-workzone deps on `glade-chat` and `@glade/client-ts`, plus the **vendored wire IR**: `src/glade.ir.json` is 21 396 bytes against `taut/corpus/glade.ir.json`'s 25 906 and is missing the `swmr` and `crdt` shapes — `src/runtime.ts:31` calls it "VENDORED … refresh if the glade wire protocol ever changes (it is frozen today)"; it changed on 2026-08-22/29 and the copy did not follow | 2 commits. Blocked by its dependencies, not by its own churn |
| TS | `@grythjs/plugin-{code,terminals,vm,workspace}` | **absent** (`0.1.0`, `private: true`) | `workspace:*` | `private: true`, source-only, one `.css` each | **0 commits each.** Dormant |
| TS | `@grythjs/plugin-{chat,gwz,settings}` | **absent** (`0.1.0`, `private: true`) | `workspace:*` | as above; chat also carries the cross-workzone `file:` dep on `glade-chat` | 1 commit each |
| TS | `@grythjs/plugin-gyld` | **absent** (`0.1.0`, `private: true`) | `workspace:*` | as above, plus `@viz-js/viz` pinned at `3.30.0`, plus three subpath exports | **77 commits — 79% of gryth-ui's whole churn**, 201 tracked files. **Not stable. Stays** |
| — | `gryth-ui` (the app) | private, `0.0.0` | — | it is the consumer, not a candidate | 97 commits, last 2026-09-20. On branch `glp-0006-p1s4-gryth-panels`, **not `main`** |
| Rust | `glade/wire-rs` → `glade-wire` | **absent** (`0.0.0`) | `path = "../wire-rs"` from `glade-node` and `glade-client` | `0.0.0`; **no `license` field at all**; no LICENSE file; path deps on it carry no `version =`. There is no Cargo workspace root in `glade/` — the three crates are unlinked | see `glade` above |
| Rust | `glade/client-rs` → `glade-client` | **absent** (`0.0.0`) | `path` deps from `glade-gwz`, `glade-gyld`, and `grazel` (dev) | `0.0.0`; no `license`; its tests spawn the `glade-node` **binary**, not a crate | see `glade` above |
| Rust | `glade/node` → `glade-node` | **absent** (`0.0.0`) | as a built binary at `../glade/node/target/debug/glade-node` | `0.0.0`; no `license`; `[[bin]]` with no release; deps on `iroh 1`, `tokio 1` | see `glade` above |
| Rust | `grazel` | **absent** (`0.0.0`) | built binary; `gyld-ui.py` resolves it at `<glade-wz>/grazel/target/debug/grazel` | **hard-blocked**: its required dep `garns-p8-store` is `generated/p8_grazel_gryth_application_state/Cargo.toml:5 publish = false`. Defaults at `src/lib.rs:151-152` point into `../glade/node/target/debug/glade-node` and `../glade-gwz/target/debug/glade-gwz` — a consumer must not merely check out two siblings but `cargo build` them in debug. No `license` | 5 commits, **all 5 public surface** — each declares a new surface (`gyld.ask` 09-16, `gyld.file` 09-14, `ws.files` SWMR 08-29). Moving |
| Rust | `glade-gwz` | **absent** (`0.0.0`) | built binary | path deps on `glade-client` + `glade-wire`; `0.0.0`; no `license`. Shells out to `gwz` on `$PATH` (`src/supplier.rs:68`) — the least path-coupled of the suppliers | **Dormant** — 0 commits, head 2026-07-12 |
| Rust | `glade-gyld` | **absent** (`0.0.0`) | built binary; `grazel` composes its argv | path deps; `0.0.0`; no `license`. Requires a Gyld **checkout**: `src/bundle.rs:74` `<gyld-root>/scripts/<name>`, `:78-84` `PYTHONPATH=<gyld-root>/src:<gyld-root>`, `:162` seeds overlays from `<gyld-root>/examples`; `src/supplier.rs:58 DEFAULT_PYTHON = "/opt/homebrew/bin/python3.13"` — a hard-coded Homebrew absolute path | **22 commits = its entire history; 18 on the public surface; `src/supplier.rs` alone 16.** The repo is 7 days old. **Not stable** |
| Py | `gyld` → PyPI `gyld` | **absent** (`0.1.0`) | as a **checkout** via `glade-gyld --gyld-root` (required, no default); `gyld-ui.py:920` defaults it to `../../gyld-wz/gyld`; `gryth-ui/vite.config.ts:61,72` mount `../../gyld-wz/gyld/artifacts/...` | `[tool.setuptools.packages.find] where = ["src"]` ships only `src/gyld/**`, so **none of the 15 hosts in `scripts/` is packaged** — a wheel satisfies `import gyld` but not `<gyld-root>/scripts/<host>.py`. No `license`/`readme`/`authors`/`urls`/`classifiers`, no LICENSE file. `scripts/emit_decision_streams.py:136 DEFAULT_SOURCES_ROOT = "../../glade-wz"` names the other workzone's directory, resolved from `__file__`'s grandparent, and is asserted in `tests/test_architecture.py:2073` | **Split.** The library is frozen: every file under `src/gyld/` has exactly 1 commit (the extraction). `scripts/` took 25 of the 27 commits; `artifacts/` 16. **The importable package is settled; the hosts are not** |
| Rust | `glade-discover` (11 crates) | **absent** (`0.0.0`) | intra-workspace paths only — no cross-repo escapes | `Cargo.toml:22 publish = false` gates all 11 via `publish.workspace = true`; `0.0.0`; no LICENSE; no `[[bin]]` anywhere | 2 commits, 1 surface. Draft contracts. **Out of scope for now** |
| — | `ggg-viz`, `grip-react-demo`, `glade/demo`, `glade/grip-share` | private apps/fixtures | `file:`/`link:` | not candidates — they are consumers | `ggg-viz` 0 commits (head 2026-07-18); `glade/grip-share` documents a binder that was deleted 2026-07-10 |

**Two facts that frame everything above.**

1. **The four duplicate checkouts are byte-identical to their `glade-wz` twins.** `grip-core` `97ff6c2`,
   `grip-react` `c13b8a7`, `glade-decl-ts` `7e16e32`, `glial` `0dfe4b9` — same HEAD, same branch,
   clean in both workzones. Removing them risks no content. The two-copies-of-glial hazard that
   `GrythReadiness-2026-09-13.md` found has since been adjudicated by the `overrides` block in
   `gryth-ui/pnpm-workspace.yaml`, which also has to declare `../grip-core`, `../grip-react`,
   `../glial` and `../glade-decl-ts` as workspace packages *outside the pnpm root* to make it work.
2. **The cost of the `file:`/`link:` seams is already written down, in YAML.**
   `gryth-wz/gryth-ui/.github/workflows/taut-shape-consumer.yml` checks out **eight** repositories
   and symlinks them into a reconstructed `glade-wz`/`wyred-wz`/`workspace` tree before it can run
   `pnpm install --frozen-lockfile`. Publishing the leaves deletes most of that file. That is the
   clearest available measure of what extraction buys.

---

## 3. The plan

Phases are milestones: each one is a coherent, shippable increment and each leaves the tree green.
Steps are one goal each, budgeted to an **aspirational < 500 LOC** (a target, not a limit). Steps
within a phase are written so that different agents can take different ones; cross-step coupling is
called out where it exists.

**The standard exit check, referenced below as `PACKED`.** Unless a step says otherwise:

1. Build the artefact and pack it — `pnpm pack`, `cargo package`, or `python -m build`.
2. Install the **tarball** (not the sibling source) into a scratch consumer created outside all
   three workzones, and import the package's public entry points from it.
3. Then, in `gryth-ui`, with the sibling source removed from resolution, `pnpm install`,
   `pnpm test`, `pnpm lint`, `pnpm build` **and** `pnpm build:gyld` — the Gyld target is checked
   separately by standing rule in `gryth-ui/AGENTS.md` — plus `pnpm test:py` where `gyld-ui.py`
   is touched. The Gyld target must build and its tests pass **against the packed artefact**.

Where a step cannot yet satisfy step 3 because a dependency is still a sibling, it says so and the
check degrades to steps 1–2 plus a named follow-up.

### Phase 0 — the cheap consolidation (foundational; gates everything else)

Goal: one workspace, four fewer checkouts, no behaviour change. Fully reversible.

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 0.1 | all three roots | `gwz snapshot pre-consolidation-2026-09-20` in each of `glade-wz`, `gryth-wz`, `gyld-wz`, then `gwz capture`. There are **no snapshots today** (`gwz snapshot --list` → "no snapshots") | 0 | the reversibility every later step assumes | `gwz snapshot --list` names it in all three; `gwz status` clean |
| 0.2 | `glade-wz` root | Move the `gyld` and `gryth-ui` checkouts to `glade-wz/gyld` and `glade-wz/gryth-ui` (members live at workspace-relative paths), then `gwz repo add <path>` each — "Add an existing local git repository … it does not clone a new copy." Structural change only, via the CLI: `AGENTS_GWZ.md` forbids hand edits to `gwz.conf/`, and there is no rename or move verb | 0 (config) | one workspace | `gwz status` lists 21 members; both members clean and on their existing branches |
| 0.3 | `gryth-wz` root | `gwz repo detach` `grip-core`, `grip-react`, `glade-decl-ts`, `glial`. The checkouts stay on disk per `gwz repo detach` semantics; delete them only after 0.5 is green | 0 (config) | **the duplicates go** | `gwz --root gryth-wz status` lists only `gryth-ui`; the four HEADs verified identical to their `glade-wz` twins first |
| 0.4 | `gryth-ui` | Retarget resolution: `pnpm-workspace.yaml`'s four out-of-root entries and the `overrides` block now point at same-workspace siblings; the `file:../../../../glade-wz/...` deps in `packages/glade/package.json` and `packages/plugins/chat/package.json` shorten by two levels. Keep the comment block explaining the singleton requirement — it is still true | ~40 | 0.5 | `pnpm install`; `pnpm why @owebeeone/glial-runtime` shows exactly one copy; `pnpm test`, `pnpm build`, `pnpm build:gyld` |
| 0.5 | `gryth-ui`, `gyld` | Fix the cross-workzone default paths: `gryth-ui/vite.config.ts:61,72` (`GYLD_BUNDLE_DEFAULT`, `GYLD_EVALUATOR_DEFAULT`), `gyld-ui.py:916,920` (the `glade-wz` and `gyld-wz/gyld` roots), `gyld/scripts/emit_decision_streams.py:136` (`DEFAULT_SOURCES_ROOT`) and the assertion at `gyld/tests/test_architecture.py:2073`. Each becomes a one-hop sibling path. Every one of these is already overridable by env or flag, so the change is to the documented default only | ~60 | a working `pnpm dev:gyld` and `gyld-ui.py start` in the merged layout | `python3 -B scripts/test_fast.py --all` in `gyld`; `pnpm test:py` and `pnpm build:gyld` in `gryth-ui`; `gyld-ui.py start` reaches its `published builds/… (N streams)` line, then `status` exits 0 |
| 0.6 | `glade-wz` root | Update `AGENTS.md` and `.claude/launch.json` for the 21-member layout; record the consolidation in `dev-docs/DecisionLog.md` as a GDL entry with the snapshot name | ~30 | later agents finding the right root | `gwz status`; the decision entry cites 0.1's snapshot |

Not in Phase 0, deliberately: the `@wyredjs/plugin-wyred` link, `gryth-ui`'s branch
(`glp-0006-p1s4-gryth-panels` — a separate call), and anything about publishing.

### Phase 1 — the TypeScript leaves, published with built types

The order is **forced by a fact, not by preference**: `grip-core`'s published `dist/index.d.ts`
already references `@owebeeone/glade-decl`, so `glade-decl-ts` publishes first or `grip-core`
cannot publish honestly at all.

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 1.1 | `glade-decl`, `glade-decl-ts` | Decide and record the GDL-041 catalogue question: either reconcile `ir/glade_decl.taut.py` (`message`/`window` out, `atom` in) and regenerate the three ports, or publish `0.1.0` with the divergence written into the README and the DecisionLog. **This is the stability gate, not a formality** — GlialFitAssessment §1 found glial already ahead of the contract module it imports | ~120 | 1.2 and the whole Rust `glade-decl` leg | `pnpm test` in `glade-decl-ts`; the contract hash in `src/index.ts` and `glade-decl-rs/src/lib.rs` still agree |
| 1.2 | `glade-decl-ts` | Make it a real package: `tsconfig.build.json` emitting `dist` + `.d.ts`, `exports` map onto `dist`, `prepack`, a version, `LICENSE`, `publishConfig.access: public`, and a `publish.yml` copied from `taut-shape-ts`'s | ~90 | 1.3, 1.4, Phase 2 | `PACKED`, degraded to steps 1–2 (gryth-ui still resolves siblings until 1.5) |
| 1.3 | `grip-core` | Republish at a **new minor** — `0.3.0`, because the surface changed under an unchanged `0.2.1`. Replace `file:../glade-decl-ts` with a caret range on 1.2's version. Add a `publish.yml` | ~40 | 1.4, and every `@grythjs` package's peer | `PACKED` steps 1–2; a scratch consumer type-checks `dist/index.d.ts` **without** `skipLibCheck` |
| 1.4 | `grip-react` | Republish at `0.3.0` against `@owebeeone/grip-core@^0.3.0` by version. Independent of 1.5 | ~30 | 1.5, Phase 3 | `PACKED` steps 1–2 |
| 1.5 | `grip-core` | Land the upstream fix the workaround points at: `GripContext.addParent` notifies the resolver, symmetric with `removeParent` (`03a56ec`). Ship in 1.3's release or a `0.3.1` | ~80 | deleting `gryth-ui/packages/desktop/src/tabContexts.ts:98` | a grip-core test that resolves a consumer after a parent is added post-resolution; then the two lines come out of `tabContexts.ts` and `pnpm test` + `pnpm build:gyld` stay green |
| 1.6 | `gryth-ui`, `ggg-viz`, `grip-react-demo`, `glade/demo`, `glade/grip-share`, `glade-chat` | **The extraction itself**: replace `file:`/`link:`/`workspace:*`/`overrides` on grip-core, grip-react and glade-decl with caret ranges. Keep `resolve.dedupe` in `vite.config.ts` — one grip registry is still required | ~60 | Phase 7 retirement of three members | full `PACKED`. Additionally: `taut-shape-consumer.yml` loses its `grip-core`, `grip-react` and `glade-decl-ts` checkout steps and still passes |
| 1.7 | `taut-shape-rs`, `taut` | Housekeeping, independent of everything else: fast-forward the `taut-shape-rs` checkout to `origin/main` (`v0.9.2`, matching crates.io) and cut the due `taut` release for the 3 commits past `v0.9.1` | ~10 | nothing; removes two stale-version traps | `cargo test` in `taut-shape-rs`; the `taut` publish workflow runs on the GitHub release |

### Phase 2 — `glial-runtime` and the Glade client published; the interim seam ends

Depends on Phase 1. This is the phase `gryth-ui/tsconfig.app.json` is waiting for: its comment names
`noUnusedLocals`, `noUnusedParameters` and `erasableSyntaxOnly` as relaxed "program-wide" because
the app deep-checks leniently-configured cross-workspace **source**, and says "Restore when the
glade packages ship built `.d.ts` (publish-or-member call)."

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 2.1 | `glade/client-ts` | Give it an identity: a scoped name in a scope we own (`@glade` is not ours), an `exports` map with the subpaths gryth-ui actually needs (`session`, `client`, `taut/schema`), a build, a version, a `LICENSE`, and `private` removed | ~110 | 2.3 | `PACKED` steps 1–2 |
| 2.2 | `gryth-ui` | Replace the three deep `@glade/client-ts/src/*.ts` imports in `packages/glade/src/runtime.ts` with the 2.1 exports. Pure import rewrite | ~20 | 2.4 | `pnpm build:gyld`, `pnpm test` |
| 2.3 | `glial` | Publish `@owebeeone/glial-runtime`: drop `private`, emit from a `tsconfig.build.json` (today's `tsconfig.json` is `noEmit`), four build entry points to match the four `exports` entries (`.`, `./grip`, `./supplier`, `./manifest`), a version, `LICENSE`, `prepack`, `publish.yml`. Relative imports are extensionless, so no specifier rewriting is needed. Keep `@owebeeone/grip-core` as a **peer** — the singleton requirement is real | ~140 | 2.4, 2.5, Phase 3 | `PACKED` steps 1–2, plus `vitest run` and the existing `taut-shape-consumer.yml` value-shape job |
| 2.4 | `glade-chat` | Publish `@owebeeone/glade-chat` the same way. Dormant since 2026-07-12, so this is mechanical | ~70 | 2.5 | `PACKED` steps 1–2 |
| 2.5 | `gryth-ui` | **Close the seam**: the three `file:` deps become caret ranges; restore `noUnusedLocals`, `noUnusedParameters`, `erasableSyntaxOnly` in `tsconfig.app.json` and delete the explanatory comment | ~40 | Phase 3, and retiring three more members | full `PACKED`, with the three flags on |
| 2.6 | `taut` or `glade-decl-ts`, then `gryth-ui` | **Stop vendoring the wire IR.** Decide which package ships `taut/corpus/glade.ir.json` (it is the frozen wire schema; `glade-decl` is the natural home), then delete `gryth-ui/packages/glade/src/glade.ir.json` and import it. The vendored copy is 21 396 bytes against 25 906 and cannot encode or decode an `swmr` or `crdt` op — GlialFitAssessment §2 calls it "the one broken artefact" | ~60 | correct `swmr`/`crdt` wire handling in gryth-ui | a test that asserts the shipped IR's `Shape` enum contains `swmr` and `crdt`; `pnpm build:gyld` |

### Phase 3 — the `@grythjs` packages as libraries

Depends on Phases 1–2 for the peer versions. Ordered by the stability evidence, most settled first.

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 3.1 | `gryth-ui` | A shared library build recipe for one package, proved on `@grythjs/plugin-api` (2 commits in 8 weeks, 281 lines): `exports` onto `dist`, `.d.ts`, `peerDependencies` on `@owebeeone/grip-react` and `react` kept as-is, `private` removed, version, `LICENSE`, `publishConfig` | ~120 | every later step in this phase | `PACKED` steps 1–2; a scratch consumer with only the peers installed |
| 3.2 | `gryth-ui` | Settle the **stylesheet** question once and apply it to the nine `.css` files. A library cannot rely on a bundler for `import './x.css'`; the package ships the file and either keeps the side-effect import with `sideEffects` declared, or exports `./styles.css` for the app to import | ~80 | 3.3–3.5 | the packed tarball contains the `.css`; a scratch Vite app renders the component styled |
| 3.3 | `gryth-ui` | Publish the four dormant plugins — `code`, `terminals`, `vm`, `workspace` (0 commits each in 8 weeks). Four independent, parallel steps if wanted | ~60 each | Phase 6 | `PACKED` per package |
| 3.4 | `gryth-ui` | Publish `@grythjs/desktop` (10 commits) — after 1.5 removes the resolver workaround, so the package ships no upstream patch | ~90 | Phase 6 | `PACKED` |
| 3.5 | `gryth-ui` | Publish `@grythjs/glade` (2 commits), and `plugin-chat`/`plugin-gwz`/`plugin-settings` (1 each). Requires Phase 2 complete | ~120 | Phase 6 | `PACKED` |
| — | — | **`@grythjs/plugin-gyld` is explicitly not in this phase.** 77 commits in 8 weeks, 201 files. It stays in the workspace until its own surface stops moving | — | — | — |

### Phase 4 — the Rust crates (independent of Phases 1–3 after Phase 0)

Bottom-up, because every path dep must become a version. Note that none of the current `path` deps
carries a companion `version =`, so `cargo publish` refuses them all as written.

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 4.1 | all Rust members | The metadata floor: a `license` field and a `LICENSE` file for `glade-wire`, `glade-node`, `glade-client`, `grazel`, `glade-gwz`, `glade-gyld`, `taut-shape-rs`, `glade-decl-rs`. Today only `taut/LICENSE` exists anywhere in the three workzones | ~40 | every later Rust step | `cargo package --list` per crate shows the LICENSE |
| 4.2 | `glade-decl-rs` | Remove `publish = false`, set a real version, publish. Update `glade/contracts/binding-api` to `{ version = "…", path = "…" }` | ~30 | 4.3 | `cargo package`; `cargo build` in a scratch crate depending on it by version |
| 4.3 | `glade` | Add a Cargo workspace root over `wire-rs`, `client-rs`, `node` (there is none today), give the three real versions, publish `glade-wire` then `glade-client` | ~120 | 4.4, 4.5 | `cargo package` each; a scratch crate builds against `glade-client` by version |
| 4.4 | `glade` | Release `glade-node` as a **binary**, not only a crate: a tagged GitHub release with built artefacts | ~60 | 4.6 | the release asset runs `--version` on a clean machine path |
| 4.5 | `glade-gwz`, `glade-gyld` | Path deps → versions; release both as tagged binaries. `glade-gwz` is dormant and goes first; `glade-gyld` waits, because 18 of its 22 commits are public surface and `src/supplier.rs` alone took 16 | ~80 | 4.6, Phase 5 | `cargo package`; `gyld-ui.py start` green with the released binaries on `PATH` |
| 4.6 | `grazel` | Two blockers, in order: (a) `generated/p8_grazel_gryth_application_state` is `publish = false` and is a **required** dependency — either it is published or grazel vendors the generated module; (b) `src/lib.rs:151-152` defaults point into two sibling `target/debug/` trees — resolve a binary by `PATH`, then env, then the sibling path, in that order | ~180 | Phase 6 | `cargo package`; `gyld-ui.py start` with no sibling `target/debug/` present |
| 4.7 | `glade-decl` | Decide the generator's own home: `corpus/build.py` writes into `../glade-decl-rs/src` and imports taut from `../taut/src`. Either it depends on `taut-proto` by version and takes an output path, or `glade-decl` is declared a workspace-only member for good | ~90 | Phase 7 retirement of `glade-decl-*` | `corpus/build.py` runs from outside the workspace with explicit `--out` paths |

### Phase 5 — Gyld as a Python package, hosts as entry points (independent of Phases 1–4)

The goal is precise: **the supplier stops needing a checkout.** `taut`'s `pyproject.toml` +
`publish.yml` is the reference.

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 5.1 | `gyld` | The metadata floor: `license`, `license-files`, `LICENSE`, `readme`, `authors`, `[project.urls]`, `classifiers`. Settle `requires-python`: the manifest says `>=3.11`, the consumer hard-codes 3.13, and taut/glade-decl-py say `>=3.10` | ~40 | 5.2 | `python -m build`; `twine check dist/*` |
| 5.2 | `gyld` | **Package the hosts.** `where = ["src"]` ships only `src/gyld/**`, so none of the 15 scripts in `scripts/` is in the wheel. Move each host's body under `src/gyld/hosts/` and declare `[project.scripts]` entry points beside the existing `gyld = "gyld.composition:main"`, leaving thin shims in `scripts/` for the current callers | ~300 | 5.3, 5.4 | `pipx run --spec dist/*.whl gyld-emit-decision-streams --help`; `python3 -B scripts/test_fast.py --all` and `check_architecture.py` still green |
| 5.3 | `gyld` | Fix `DEFAULT_SOURCES_ROOT = "../../glade-wz"` (`scripts/emit_decision_streams.py:136`). Resolved from `__file__`'s grandparent it is meaningless inside site-packages. Make the default explicit-or-absent and keep the "record a missing root as data" behaviour that `:125-127` relies on; update `tests/test_architecture.py:2073` and `tests/io/test_architecture.py` | ~120 | 5.4 | the fast suite; a run from an installed wheel with no workzone anywhere records the absent root as data rather than failing |
| 5.4 | `glade-gyld` | `--gyld-root` becomes optional: the supplier invokes the installed entry points, and only falls back to `<gyld-root>/scripts` when a root is given. `src/bundle.rs:78-84`'s `PYTHONPATH` and `:162`'s `examples` seeding both need a story — examples ship as package data or move to the bundle root. Also replace `DEFAULT_PYTHON = "/opt/homebrew/bin/python3.13"` with interpreter discovery | ~250 | Phase 6 | the `glade-gyld` integration test passes with **no** Gyld checkout on disk; `gyld-ui.py start` green against a `pip install`ed gyld |
| 5.5 | `glade-decl-py` | Publish it, declaring `taut-proto` as a real dependency instead of the `sys.path` bootstrap at `tests/conftest.py:16` | ~50 | Phase 7 | `python -m build`; a scratch venv imports `glade_decl` with taut resolved by version |

### Phase 6 — the demo becomes a small app repository

Depends on Phase 3 and Phase 5 (and Phase 4.6 for grazel). What is left of `gryth-ui` once the
libraries have gone: the Gyld entry (`entries/gyld/`), `src/` (7 commits in 8 weeks), the two Vite
configs, `gyld-ui.py` + `scripts/gyld_ui_test.py`, `AGENTS.md`, `README.md` and the runbook.

| Step | Repo | Goal | ~LOC | Unblocks | Exit check |
|---|---|---|---|---|---|
| 6.1 | `gryth-ui` | Split the repository in intent before splitting it on disk: decide whether the extracted `@grythjs/*` packages keep living in `gryth-ui/packages/` as a published monorepo, or move to a `grythjs` repository of their own leaving `gryth-ui` as the app. Record the call | ~40 (a doc) | 6.2 | the decision is in `dev-docs/DecisionLog.md` |
| 6.2 | `gryth-ui` | Every first-party dependency becomes a version range; `pnpm-workspace.yaml` keeps only what is still co-developed (`plugin-gyld` at minimum) | ~50 | Phase 7 | full `PACKED`; `taut-shape-consumer.yml` shrinks to one checkout |
| 6.3 | `gryth-ui` | The runbook, `gyld-ui.py` and the agent configuration move with the app and name released binaries and installed entry points rather than sibling `target/debug` trees and checkouts | ~120 | — | `gyld-ui.py start` from a fresh clone of only this repository, with published packages and released binaries, reaches `status` exit 0 |

### Phase 7 — members retire from the workspace

Runs **continuously**, not at the end: each member retires the moment its consumers are pinned. One
`gwz repo detach` per member, one entry in the DecisionLog per retirement, recording the pinned
version. Expected order, following the phases above: `taut-shape-ts`, `taut-shape-rs`, `taut`
(already pinned or pinnable today) → `grip-core`, `grip-react`, `glade-decl-ts` (after 1.6) →
`glial`, `glade-chat`, `glade/client-ts` (after 2.5) → `glade-decl-rs`, `glade-decl-py`,
`glade-wire`/`client-rs`/`node` (after Phase 4) → `glade-gwz` (after 4.5) → `ggg-viz`,
`grip-react-demo` (dormant demos, retirable at any point after 1.6). Expected to remain:
`glade`, `glade-discover`, `grazel`, `glade-gyld`, `gyld`'s hosts and `gryth-ui` — the things
actually being co-developed.

**Exit check for the phase as a whole, and the measure the owner named: the member count in
`glade-wz/gwz.conf/gwz.yml` goes down.** It is 19 today, 21 after Phase 0.

### What can run in parallel

- **Phase 0 gates everything.** Nothing else starts before 0.5 is green.
- **Three independent tracks after Phase 0**: TypeScript (1 → 2 → 3), Rust (4), Python (5).
  They share no files. Different agents, different days.
- Within Phase 1: 1.1 → 1.2 → {1.3, 1.4} → 1.6. 1.5 and 1.7 are independent of all of it.
- Within Phase 3: 3.1 → 3.2 → {3.3 ×4, 3.4, 3.5} — six parallel steps once the recipe exists.
- Within Phase 4: 4.1 first, then 4.2 → 4.3 → {4.4, 4.5} → 4.6. 4.7 is independent.
- Within Phase 5: 5.1 → 5.2 → 5.3 → 5.4. 5.5 is independent of all of them.
- Phase 6 needs 3, 4.6 and 5. Phase 7 is incremental throughout.

---

## 4. Release mechanics

**Versioning while contracts move.** Everything here is 0.x and stays 0.x until its consumers stop
forcing changes in it. Under 0.x, **a breaking change bumps the minor** (`0.3.0` → `0.4.0`) and
nothing else may; a patch is additive or a fix. "Breaking" means anything a consumer can observe:
a removed or renamed export, a narrowed type, a changed default, a moved `exports` subpath, a new
required peer, or a changed wire or file format. The estate has two live counter-examples —
`@owebeeone/grip-core@0.2.1` and `@owebeeone/grip-react@0.2.0` both moved under a fixed number — so
the rule that matters is the negative one: **a public-surface commit that does not bump the version
is a defect.** A syntax-aware check over each package's declared entry points would catch it; that
is the same class of cheap source check the global rules already ask for.

**Released tags are immutable and are never moved or re-pointed.** This is an owner rule and it is
already encoded in practice: `taut-shape-rs/.github/workflows/publish.yml` takes a
`workflow_dispatch` input described as "Existing immutable release tag to verify". A bad release is
superseded by a higher version, never by moving a tag. Yanking a crate or deprecating an npm version
is allowed; rewriting one is not.

**Who publishes, and from where.** CI, on a published GitHub release — the pattern already working
in three repositories:

| Repo | Workflow | Trigger | Credential |
|---|---|---|---|
| `taut` | `.github/workflows/publish.yml` | `release: published` | PyPI **Trusted Publishing** (OIDC, `id-token: write`, `pypi` environment) — no stored token |
| `taut-shape-ts` | `.github/workflows/publish.yml` | `release` + `workflow_dispatch` | `npm publish --access public` |
| `taut-shape-rs` | `.github/workflows/publish.yml` | `release` + `workflow_dispatch` | `cargo publish -p taut-shape --locked`, `secrets.CARGO_REGISTRY_TOKEN` |

Every new package copies one of these three rather than inventing a fourth. Hand publishing is the
exception and needs saying out loud, because there is no local credential today: `pnpm whoami`
returns **401 Unauthorized** on this machine. Consumer verification also already has a pattern —
`taut-shape-consumer.yml`, present in `glial` and `gryth-ui` — which should be re-pointed at packed
artefacts as each package lands, and which is the natural home for `PACKED` step 3.

**Registry scopes, and the open question of who owns them.**

| Scope / name | State on the registry | Note |
|---|---|---|
| `@owebeeone` (npm) | **In use** — 5 published packages: `taut-shape` 0.9.2, `grip-core` 0.2.1, `grip-react` 0.2.0, `grip-vue` 0.2.0, `click-reel` 0.2.1. Publisher recorded as `owebeeone` | The safe default for anything new |
| `@grythjs` (npm) | **Nothing published.** Ownership **not verified** — an unowned scope and a scope owned by someone else are indistinguishable from outside, and the org endpoints need auth | Must be settled before Phase 3 |
| `@wyredjs` (npm) | **Nothing published.** Same uncertainty | Out of scope, but the same question |
| `@glade` (npm) | **Nothing published.** Used today by two private packages (`@glade/client-ts`, `@glade/grip-share`) | Phase 2.1 needs a real scope; `@owebeeone` is the cheap answer |
| crates.io | `taut-shape` 0.9.2 taken and ours. `glade-decl`, `glade-wire`, `glade-node`, `glade-client`, `grazel`, `glade-gwz`, `glade-gyld` all **absent** — unscoped, so first-come | Reserve the names early in Phase 4, or rename |
| PyPI | `taut-proto` 0.9.1 taken and ours. `gyld`, `glade-decl` **absent** — `glade-decl` is a very generic name on a flat namespace | Check `gyld` and `glade-decl` are still free at Phase 5.1 |

---

## 5. Where each root-level document goes

Grouped by family; counts are files. The principle is the owner's: a document lives in the
repository it describes. Two constraints cut across it, and both are recorded as open questions
below. First, `gyld-wz/AGENTS.md` and `gyld-wz/dev-docs/delivery/RepositorySplit.md` record an
**owner ruling, ON-070**, that Gyld's `dev-docs/` deliberately stays in the workzone while code
lives in the `gyld` member — so the Gyld rows below contradict a standing ruling unless it is
revisited. Second, `gyld` has **no `dev-docs/` directory at all** today, so the Gyld moves create
one.

| Document(s) | Now at | Goes to | Why |
|---|---|---|---|
| **The UI specification** — `GyldGrythPlugins.md` | `gyld-wz/dev-docs/ui/` | `gryth-ui/dev-docs/` | Specifies the plugin package, the store tap, the surfaces and the artefact contract — all of it code in `gryth-ui/packages/plugins/gyld/` |
| **The UI simplification report** — `GyldUiSimplification.md` | `gyld-wz/dev-docs/ui/` | `gryth-ui/dev-docs/` | Diagnoses `packages/plugins/gyld/` and `packages/desktop/src/foundations.ts` |
| `MultiDimensionalGraphViewing.md`, `GyldUiArchitectureEvaluationPrompt.md` | `gyld-wz/dev-docs/ui/` | `gryth-ui/dev-docs/` | Renderer choice and the prompt for the run below; both name gryth-ui files. `gryth-ui/dev-docs/` already holds `GraphRendererFeatures.md` and `GraphRendererRequirements.md` |
| **The ask-agent design** — `GyldAskAgent.md` | `gyld-wz/dev-docs/ui/` | `glade-gyld/dev-docs/` | The agent is **a new verb on the supplier**, not a UI feature; `glade-gyld`'s `src/agent.rs` and its `--agent-*` configuration are the subject. Cross-reference from `gryth-ui/dev-docs/` for the gesture |
| **The UI case study** — `case-studies/ui/GyldUiArchitectureEvaluation.md` | `gyld-wz/dev-docs/` | `gryth-ui/dev-docs/` | A run record whose **subject** is gryth-ui's architecture; the evaluator hosts stay in `gyld` |
| **The Glade case studies** — `case-studies/glade/` (5 md + 1 declaration) | `gyld-wz/dev-docs/` | `glade/dev-docs/` (or this workzone's `dev-docs/glade/`) | They describe Glade's problem space and the Iroh evaluation. Their companion, `IrohGladeMapping.md`, is already here |
| **Gyld's own case study and examples** — `case-studies/gyld/`, `dev-docs/examples/` (13 files) | `gyld-wz/dev-docs/` | `gyld/dev-docs/` | Gyld's self-model and the declaration-language specimens; `examples/*.gyld.py` sit beside `gyld/examples/` |
| **External-subject case studies** — `case-studies/gwz/`, `case-studies/sdax/`, `integrations/GarnsWorkspaceRequirements.md` | `gyld-wz/dev-docs/` | `gwz-dev`, `sdax-wz`, `garns-wz` respectively | None of the three subjects is a member of any of these workzones |
| **The whole Gyld design corpus** — 31 top-level md + `delivery/` (~45 files incl. two run trees) | `gyld-wz/dev-docs/` | `gyld/dev-docs/` — **or stays, per ON-070** | Charter, decision log, owner notes, product description, acceptance, evaluation spec, architecture, delivery plans and run records all describe `gyld`. The ruling says otherwise; see open questions |
| **The runbook** — `GrythGyldDemoRunbook.md` | `gryth-wz/dev-docs/` | `gryth-ui/dev-docs/`, then with the app in Phase 6 | It explains `gyld-ui.py`, which lives in `gryth-ui` |
| **The readiness notes** — `GrythReadiness-2026-09-13.md` | `gryth-wz/dev-docs/` | `gryth-ui/dev-docs/` | Self-declared scope: "assesses `gryth-wz` only" |
| `GrythVision.md`, `GrythDemoProposal.md`, `GripLabGripAndTapInventory.md` | `gryth-wz/dev-docs/` | `gryth-ui/dev-docs/` | The vision and tap inventory `gryth-ui/AGENTS.md` already tells agents to read as `../dev-docs/`; that pointer becomes local |
| `GrythGladeSeamAssessment.md` | `gryth-wz/dev-docs/` | `gryth-ui/dev-docs/` | Two-sided, but it is the gryth-side verdict on the seam; cross-reference from `dev-docs/glade/` |
| `gryth-wz/dev-docs/DecisionLog.md`, `StackMap.md` | `gryth-wz/dev-docs/` | **delete, do not move** | Stale near-duplicates of this workzone's originals (63 lines against 109; different digests) and their content is glade/glial. Leave a one-line pointer |
| Glial design set — `dev-docs/glial/` (7 md), `docs/glial-specs/` (14 md), `docs/System Design_Overview- Glial.md`, `docs/DOC_PLAN.md`, `docs/DOC_ISSUES.md`, `GlialGlossary.md`, `GlialRequirements.md`, `GlialTopology.md`, `requirements/GlialFrankenappSilverPath.md` | `glade-wz/` | `glial/dev-docs/` | All describe the client kernel. The 14 `GLIAL-DOC-0NN_*` specs are the superseded centralized design and should move **marked superseded**, not silently |
| Glade design corpus — `dev-docs/glade/` (~30 md incl. `suppliers/`), `arch1/` (6 md), `GladeKernel`-family, `GladeBuyBuildMatrix.md`, `GladeDecisionGraph.md`, `IrohGladeMapping.md`, `GladePersistenceReview.md`, `GladeE2EStage1Audit.md`, `Phase1Libp2pTest.md`, the Rust-async trio, `GLResearchConsolidatedFindings.md`, `research/` | `glade-wz/dev-docs/` | `glade/dev-docs/` | `glade/dev-docs/` already holds 19 documents in this convention (`GladeSubstrateV1.md`, `GladeZones.md`, `IrohReview.md`) |
| `dev-docs/glade/GladeDeclSurface.md`, `dev-docs/examples/` (`.glade` files + keyword maps) | `glade-wz/dev-docs/` | `glade-decl/dev-docs/` | The declaration surface and its examples; `glade-decl/dev-docs/` already has `DeclSurface.md` and `OpenNotes.md` |
| `dev-docs/glade/GladeDiscovery{Model,Design}.md` + the two `-Review56` files | `glade-wz/dev-docs/` | `glade-discover/dev-docs/` | That repo already keeps its own copy of `GladeDiscoveryDesign.md`; one canonical copy, not two |
| `dev-docs/grip-share/` (2 md) | `glade-wz/dev-docs/` | `grip-core/dev-docs/`, marked **superseded** | The binder they describe was deleted 2026-07-10 |
| `TautShapeCatalogAdoption.md`, `TautShapeCatalogDecision.md` | `glade-wz/dev-docs/` | `taut-shape/dev-docs/` | The catalogue's own repo, which already has 21 documents in this convention. The pointer stub can just die |
| `AtlasGladeReview-56s.md`, `AtlasGladeReviewPrompt.md` | `glade-wz/dev-docs/` | `ggg-viz/dev-docs/` (new) | Their findings cite `ggg-viz/src/scenario/files.ts` |
| `RazelArchReview.md` | `glade-wz/dev-docs/` | `razel-dev` (external) | Its subject is the razel build tool, not the `grazel` member |
| `EstateVision.md` + its two reviews | `glade-wz/dev-docs/` | an estate-level home, **not** this workzone | It spans glymik and bizscad as well; it is only here because it had nowhere else |
| **Stays in `glade-wz/dev-docs/`** — `DecisionLog.md`, `StackMap.md`, `DocStructurePlan.md`, `GladeProgramStatus.md`, `LibraryBoundaryAndTestingPolicy.md`, `GladePackageArchitecture.md`, `ReleaseProcessIdeas.md`, this document, the four `ApplicationStackContract*` files + prompt, `GLDevPlan.md`, `GladeHandoff-260710.md`, `requirements/RapidDevEnvironment.md`, `requirements/HarshRealityTriage.md`, `requirements/GladeDeveloperGoldenPath.md`, `docs/WORKFLOW_PROMPTS.md`, `plan-docs/` in full (74 files) | — | — | The GDL ledger, the program tracker, the cross-stack reviews, the policies, the GLP plan instances and the workzone's own process. These are genuinely about the workspace or span three or more members — which is exactly the test `DocStructurePlan.md` proposes |

Convention for the moves: keep each target repository's existing filename grammar. `glade-wz` and
`gryth-ui` use PascalCase with reviewer/model tags (`-G6`, `-F5`, `-Review56`) and occasional dates;
`gyld-wz` uses PascalCase with date, version and review-chain suffixes and structured subtrees
(`case-studies/<subject>/`, `delivery/runs/<run-id>/<lane>/`). Nothing is renamed by the move
itself.

---

## 6. Open questions for the owner

1. Does ON-070 (Gyld's `dev-docs/` stays in the workzone) survive consolidation, or do Gyld's ~90
   documents move into `gyld/dev-docs/` like everyone else's?
2. `@owebeeone/grip-core@0.2.1` and `@owebeeone/grip-react@0.2.0` on npm are not the code in the
   tree at those numbers — republish as `0.3.0` and leave the stale versions standing, or deprecate
   them on the registry as well?
3. Do we own the `@grythjs` scope? Nothing is published under it and ownership cannot be checked
   without a login; Phase 3 needs an answer.
4. `@glade/client-ts` needs a real scope. `@owebeeone/glade-client` is the cheap answer — or do we
   register `@glade`?
5. Does `glade-decl-ts` publish with the GDL-041 catalogue divergence documented, or does
   reconciling `glade-decl/ir/glade_decl.taut.py` block the whole TypeScript track?
6. Do the `@grythjs/*` packages stay a published monorepo inside `gryth-ui`, or move to their own
   `grythjs` repository leaving `gryth-ui` as the app?
7. `gryth-ui` has been on `glp-0006-p1s4-gryth-panels` since 2026-07-12 with 97 commits on it —
   does it land on `main` before Phase 0, or does the branch come across as it is?
8. Does `grazel` publish at all? It needs its generated `garns-p8-store` dependency published or
   vendored, and its `../*/target/debug/` defaults replaced — or is grazel simply declared a
   workspace-only tool for good?
