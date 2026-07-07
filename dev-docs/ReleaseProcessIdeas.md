# Release Process Ideas

Status: draft
Date: 2026-06-05

## Short answer

Yes, the problem is real: `glial-dev` is no longer one build. It is a
polyrepo checkout with first-party Git submodules, npm packages, Rust
workspaces, Python packages, generated wasm packages, demos, and third-party
reference trees. The isolation is valuable, but the root checkout now needs a
package graph and release orchestrator.

The practical direction is not "one magic release script". It should be:

1. Keep submodules as independently owned repos.
2. Add a root package inventory that makes dependency edges explicit.
3. Use ecosystem-native tools where they solve real problems:
   - pnpm workspaces plus Changesets for first-party npm packages.
   - Cargo workspaces plus release-plz or cargo-release for Rust crates.
   - uv/maturin/cibuildwheel for Python and PyO3 wheels.
   - GitHub Actions matrix jobs for cross-platform validation.
4. Add a root `packages_maker` that coordinates those tools across submodules,
   enforces graph order, and owns checkpoint/release policy.

No single off-the-shelf tool cleanly solves "polyrepo submodules plus npm,
Cargo, Python, wasm, cross-platform tests, checkpoint tags, and dependency
chain publishing". Nx is the closest general orchestrator. pnpm plus Changesets
is the cleanest npm release foundation. A custom `packages_maker` is still
justified, but it should become a small orchestrator over explicit metadata,
not a pile of ad hoc version edits.

## Current repo facts

The parent repo has first-party submodules for:

| Path | Role observed |
| --- | --- |
| `grip-core` | npm package `@owebeeone/grip-core`, version `0.2.1` |
| `grip-react` | npm package `@owebeeone/grip-react`, version `0.2.0`, depends on `grip-core` |
| `grip-vue` | npm package `@owebeeone/grip-vue`, version `0.2.0`, depends on `grip-core` |
| `grip-lab` | private npm app, Rust workspace, Python services, wasm package |
| `grip-react-demo` | private npm app, depends on `grip-react` |
| `grip-vue-demo` | private npm app, depends on `grip-core` and `grip-vue` |
| `rust-orbitdb` | Rust workspace, currently bootstrap-stage and unpublished |
| `vm_manager` | Python package using `pyproject.toml` and `uv.lock` |

The first-party npm dependency style is inconsistent:

| Package | First-party dependencies |
| --- | --- |
| `grip-react` | `@owebeeone/grip-core: file:../grip-core` |
| `grip-vue` | `@owebeeone/grip-core: ^0.2.0` in dependencies and peers |
| `grip-lab` | `@owebeeone/grip-core: file:../grip-core`, `@owebeeone/grip-react: file:../grip-react`, `griplab-core-wasm: file:rust/griplab-core-wasm/pkg` |
| `grip-react-demo` | `@owebeeone/grip-react: file:../grip-react` |
| `grip-vue-demo` | `@owebeeone/grip-core: ^0.2.0`, `@owebeeone/grip-vue: ^0.2.0` |

There are current hygiene gaps that matter for release automation:

- `package-lock.json` files contain local symlink/link entries for first-party
  packages. A release flow MUST verify publish artifacts from a clean pack, not
  from the developer `node_modules` tree.
- `grip-core/package.json` says `0.2.1`, but `grip-core/package-lock.json`
  still records `0.1.0` at the lockfile root. That is a release smell.
- `grip-vue-demo/package-lock.json` records `@owebeeone/grip-core` as `0.2.0`
  while the local package is `0.2.1`.
- `grip-lab/README.md` says it installs `@owebeeone/grip-react` from npm, but
  the manifest currently uses local `file:` links.
- No root npm workspace file, Nx config, Turbo config, Rush config, or GitHub
  workflow is present in the root checkout.

## Existing `release_maker`

Existing script:

`/Users/owebeeone/limbo/anchorscad-dev/dev-setup/src/release_maker/release_maker.py`

The checked-out version at that path is a useful prototype, but it is scoped to
Python `pyproject.toml` files. It currently:

- finds `pyproject.toml` files under source directories;
- bumps `[project].version`;
- optionally fetches Git tags;
- writes TOML files;
- optionally commits, pushes, tags, and pushes tags.

It does not yet do the hard parts needed here:

- discover npm/Cargo/Python package graphs;
- topologically order builds and releases;
- update dependent version ranges;
- convert local npm `file:` references to publishable semver;
- produce changelogs;
- dry-run artifact contents with `npm pack --dry-run`, `pnpm pack`,
  `cargo package --dry-run`, or `uv build`;
- smoke-test packed artifacts from a temp install;
- coordinate multiple submodule Git repos transactionally;
- record root submodule pointer checkpoints after child repos are released;
- run OS matrix validation;
- retry/poll npm registry availability before releasing downstream dependents.

Keep the semver bumping code idea, but treat the current script as a spike, not
as the base architecture for the new tool.

Note: there may have been, or may still be elsewhere, a newer `release_maker`
variant that handles npm. I only found the Python/TOML variant in the current
`/Users/owebeeone/limbo` checkouts. If an npm-capable version turns up, it
should be treated as input to the new design rather than assumed to be the
canonical implementation.

## Dependency graph

The immediate first-party release graph is:

```text
@owebeeone/grip-core
  -> @owebeeone/grip-react
       -> grip-lab
       -> grip-react-demo
  -> @owebeeone/grip-vue
       -> grip-vue-demo
  -> grip-lab
  -> grip-vue-demo

grip-lab/rust/griplab-core
  -> grip-lab/rust/griplab-core-py
  -> grip-lab/rust/griplab-core-wasm
       -> grip-lab npm app through file:rust/griplab-core-wasm/pkg

rust-orbitdb/crates/rust-orbitdb-core
  -> currently separate; no observed dependency from first-party npm packages
```

For npm publication, the rule should be:

1. Publish leaf dependencies first, e.g. `grip-core`.
2. Wait until the registry can resolve the exact version.
3. Update dependents to the released semver range.
4. Build, pack, smoke-test, publish dependents.
5. Repeat up the chain.

Do not rely on deleting local `node_modules` symlinks as the release mechanism.
The reliable test is a clean temp install of the packed artifact against the
registry versions that a user will actually consume.

## Tooling options

### pnpm workspaces plus Changesets

This is the best npm-specific foundation.

pnpm workspaces give a root-level package graph. The `workspace:` protocol
forces local resolution during development and avoids accidentally pulling an
unintended registry package. On `pnpm pack` or `pnpm publish`, pnpm rewrites
`workspace:` dependency specs into normal semver ranges, which directly solves
the "local refs before npm publish" problem.

Changesets is built for multi-package versioning and changelogs. It records
release intent as changeset files, combines bump types, updates package
versions, updates internal dependents, and can publish updated packages.

Good fit:

- first-party npm packages;
- independent package versions;
- changelog generation;
- dependency range updates;
- dry-runable release PRs.

Limits:

- npm-first; it will not manage Cargo/Python release semantics by itself;
- assumes a coherent workspace checkout;
- submodules can live inside the workspace, but each submodule also needs a
  standalone story if it is developed outside `glial-dev`.

Recommended npm policy:

- Add root `pnpm-workspace.yaml` covering first-party packages.
- Prefer `workspace:^` or `workspace:*` for local first-party npm deps during
  development.
- Use `pnpm pack --dry-run` or `pnpm publish --dry-run` as a required gate.
- Use Changesets for npm package version/changelog updates unless Nx Release is
  selected as the broader orchestrator.

Primary docs:

- pnpm Workspace: https://pnpm.io/workspaces
- pnpm Catalogs: https://pnpm.io/catalogs
- pnpm Publish: https://pnpm.io/cli/publish
- Changesets: https://github.com/changesets/changesets

### Nx

Nx is the strongest general-purpose candidate if we want to avoid writing too
much of `packages_maker` ourselves.

Nx can infer or define projects, build a project graph, run tasks in dependency
order, cache results, run only affected tasks, and manage releases with
independent projects or release groups. Its task pipeline syntax covers the
core need: `build` for a project can depend on `^build` for upstream projects.

Good fit:

- root orchestrator for build/test/lint across many packages;
- affected-only CI;
- dependency-ordered task execution;
- release groups and independent versions;
- enough structure for npm, Rust, and Python tasks if configured explicitly.

Limits:

- more configuration and conceptual weight than pnpm plus Changesets;
- release features are strongest when projects live in a monorepo-shaped graph;
- submodule repos still need clean/push/tag handling.

Recommendation:

- Use Nx if `packages_maker` should become mostly configuration plus a thin
  wrapper.
- Otherwise, copy the Nx mental model: explicit projects, explicit targets,
  `dependsOn`, cacheable outputs, affected selection, release groups.

Primary docs:

- Nx Run Tasks: https://nx.dev/docs/features/run-tasks
- Nx Affected: https://nx.dev/docs/features/ci-features/affected
- Nx Independent Releases: https://nx.dev/docs/guides/nx-release/release-projects-independently
- Nx Manage Releases: https://nx.dev/docs/features/manage-releases

### Turborepo

Turborepo is lighter than Nx and very good for JS/TS task graph execution and
caching. It builds a package graph from the package manager and a task graph
from `turbo.json`. A `build` task can depend on upstream `^build`.

Good fit:

- simple JS/TS build/test orchestration;
- caching;
- easier adoption than Nx for npm-only packages.

Limits:

- not a release manager by itself;
- less useful for Cargo/Python/submodule release policy;
- would still need Changesets plus a custom outer wrapper.

Primary docs:

- Turborepo Package and Task Graphs:
  https://turborepo.dev/docs/core-concepts/package-and-task-graph
- Turborepo Config:
  https://turborepo.dev/docs/reference/configuration

### Rush

Rush is a mature, strict JS monorepo management system. It supports package
governance, version policies, incremental builds, publishing, and pack mode.

Good fit:

- large npm-oriented repos with strict centralized dependency policy;
- lockstep or individual version policies;
- npm package publishing discipline.

Limits:

- heavy for this current repo shape;
- less attractive while first-party repos intentionally remain separate
  submodules;
- not the best fit for Python/Rust orchestration.

Primary docs:

- Rush Getting Started: https://rushjs.io/pages/intro/get_started/
- Rush Publishing: https://rushjs.io/pages/maintainer/publishing/

### Bazel

Bazel is a serious candidate, not a crazy option. It directly addresses several
things this repo is starting to need:

- module-local `BUILD.bazel` files with explicit targets and dependencies;
- stable labels such as `//grip-core:pkg`;
- graph query tools;
- dependency-ordered builds and tests;
- cross-language build/test orchestration;
- local and remote caching;
- a real path toward reproducible CI.

If Bazel is adopted, avoid inventing `BUILD.wsk`. Use real `BUILD.bazel` files
and make `wsk` a Bazel rules/macro library plus release wrapper. In that world,
`packages_maker` becomes thinner: it can query Bazel for the graph, ask Bazel to
build/test/package targets, and then handle the non-Bazel release actions
that Bazel does not own well: semver decisions, changelogs, registry publish,
polling registry availability, Git tags, and root submodule pointer commits.

The strongest argument for Bazel is Starlark. A small set of `wsk` rules/macros
can expand a module declaration into all expected targets:

```python
wsk_npm_package(
    name = "grip-react",
    package = "@owebeeone/grip-react",
    deps = ["//grip-core:pkg"],
)
```

That single rule can generate conventional targets such as:

```text
//grip-react:build
//grip-react:test
//grip-react:pack
//grip-react:publish_plan
//grip-react:pkg
```

This keeps dependency information inside the module while still giving the root
checkout a queryable graph.

Recommended Bazel shape:

```text
glial-dev/
  MODULE.bazel
  .bazelrc
  wsk/                  # reusable Bazel rules/macros and release helpers
  grip-core/BUILD.bazel
  grip-react/BUILD.bazel
  grip-vue/BUILD.bazel
  grip-lab/BUILD.bazel
```

For separate submodule repos, there are two plausible models:

1. **Root-owned Bazel workspace:** the `glial-dev` root has one
   `MODULE.bazel`; submodules are packages under that root; labels look like
   `//grip-core:pkg`. This is easiest for orchestration.
2. **Each submodule is a Bazel module:** every first-party repo has its own
   `MODULE.bazel`; the root uses local overrides for the checked-out submodule
   paths; labels look more like `@grip_core//:pkg`. This preserves standalone
   repo identity better, but adds more module management.

Start with the root-owned workspace unless standalone Bazel builds in every
submodule are an immediate requirement.

Risks:

- Bazel is adoption, not just configuration. The repo will need `MODULE.bazel`,
  `BUILD.bazel`, `.bazelrc`, rule dependencies, and contributor conventions.
- Bazel is large and JVM-based. That means heavier installs, more cache/output
  state, more moving parts, and a higher "why is this here?" cost for small
  repos. Use Bazel only if the graph/reproducibility/caching payoff is worth
  the tool weight.
- JS/TS, Rust, Python, PyO3, wasm, and Vite all have Bazel rules, but each adds
  integration work. The first pass should wrap existing package-manager
  commands before replacing them with fully hermetic rules.
- Bazel builds artifacts; it is not by itself an npm/PyPI/crates.io release
  manager. Release orchestration still needs `packages_maker`, Changesets/Nx,
  release-plz, maturin/cibuildwheel, or explicit publish steps.
- Native and Python wheel cross-platform release remains a CI matrix problem.
  Bazel helps with reproducible commands, but it does not remove the need to
  test on macOS/Linux/Windows.

Practical adoption path:

1. Add `MODULE.bazel`, `.bazelrc`, and Bazelisk pinning.
2. Add `BUILD.bazel` files for `grip-core`, `grip-react`, `grip-vue`, and
   one app/demo.
3. Initially define Bazel targets that call existing `npm`, `cargo`, and `uv`
   checks through small scripts.
4. Add `bazel query`/`bazel test` as root CI gates.
5. Move one ecosystem at a time toward native Bazel rules:
   rules_js/rules_ts for JS/TS, rules_rust for Rust, rules_python for Python.
6. Keep registry publishing outside Bazel until packaging is boring.

Primary docs:

- Bazel Bzlmod:
  https://bazel.build/external/overview
- Bazel Build Concepts:
  https://bazel.build/concepts/build-files
- rules_js:
  https://github.com/aspect-build/rules_js
- rules_rust:
  https://bazelbuild.github.io/rules_rust/
- rules_python:
  https://github.com/bazel-contrib/rules_python

### Cargo, release-plz, and Rust checks

Cargo already provides workspace mechanics: shared lockfile, shared target dir,
workspace metadata, workspace dependencies, and `cargo check --workspace`.

For Rust release automation, look at release-plz first. It supports workspace
configuration, package-specific release settings, changelog updates, git tags,
GitHub/Gitea/GitLab releases, publish controls, and optional semver checks.

For test execution, `cargo nextest run` is worth adopting for Rust workspaces
because it is faster and CI-friendly.

Good fit:

- `rust-orbitdb`;
- `grip-lab/rust`;
- future Rust crates;
- semver/API checks before publishing public Rust crates.

Primary docs:

- Cargo Workspaces:
  https://doc.rust-lang.org/cargo/reference/workspaces.html
- Cargo Publish:
  https://doc.rust-lang.org/cargo/commands/cargo-publish.html
- release-plz Configuration:
  https://release-plz.ieni.dev/docs/config
- cargo-release:
  https://github.com/crate-ci/cargo-release
- cargo-nextest Running Tests:
  https://nexte.st/docs/running/

### Python, uv, maturin, and cibuildwheel

For pure Python packages, `uv` is a good modern project/build/publish front-end.
For PyO3 packages, keep maturin as the build backend. For cross-platform binary
wheels, cibuildwheel is the normal CI-level tool; it builds manylinux,
musllinux, macOS, and Windows wheels and can run tests against the installed
wheel.

Good fit:

- `vm_manager`;
- `grip-lab/services/*`;
- `grip-lab/rust/griplab-core-py`;
- any future PyO3 or extension packages.

Primary docs:

- uv packaging guide:
  https://docs.astral.sh/uv/guides/package/
- cibuildwheel:
  https://cibuildwheel.pypa.io/en/stable/

### GitHub Actions matrix

Cross-platform testing should be CI-native, not simulated locally. GitHub
Actions matrix jobs can expand a single job definition across OS and language
versions, e.g. Ubuntu/macOS/Windows, Node 20/22, Python 3.11/3.12, and stable
Rust.

Primary docs:

- GitHub Actions matrix jobs:
  https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/run-job-variations

## Recommended architecture: `packages_maker`

`packages_maker` should be a root-level orchestrator with explicit metadata,
but the metadata should live beside the module it describes. Do not make one
large root manifest the package graph. Discovery is useful, but the release
contract should be declared locally.

There are two viable descriptor choices:

1. If the project adopts Bazel now, use real `BUILD.bazel` files and a root
   `MODULE.bazel`. This is cleaner than inventing a parallel syntax.
2. If the project defers Bazel adoption, use `BUILD.wsk` as a staging format
   with Bazel-like labels and rule syntax, so migration to `BUILD.bazel` stays
   tractable.

Avoid plain `BUILD` unless we want to reserve that name for Bazel-compatible
semantics. The examples below use `BUILD.wsk` as the staging form; under real
Bazel they should become `BUILD.bazel` files that load `wsk` macros/rules.

Example descriptors:

```python
# grip-core/BUILD.wsk

npm_package(
    name = "pkg",
    package = "@owebeeone/grip-core",
    publish = True,
    tags = ["library"],
)
```

```python
# grip-react/BUILD.wsk

npm_package(
    name = "pkg",
    package = "@owebeeone/grip-react",
    deps = ["//grip-core:pkg"],
    publish = True,
    tags = ["library", "react"],
)
```

```python
# grip-lab/BUILD.wsk

npm_app(
    name = "app",
    deps = [
        "//grip-core:pkg",
        "//grip-react:pkg",
        "//grip-lab/rust/griplab-core-wasm:pkg",
    ],
    publish = False,
    tags = ["app"],
)
```

```python
# grip-lab/rust/griplab-core/BUILD.wsk

cargo_package(
    name = "crate",
    publish = False,
    tags = ["rust"],
)
```

```python
# grip-lab/rust/griplab-core-wasm/BUILD.wsk

wasm_package(
    name = "pkg",
    deps = ["//grip-lab/rust/griplab-core:crate"],
    publish = False,
    tags = ["rust", "wasm", "npm"],
)
```

This gives the repo a Bazel-like label system:

```text
//grip-core:pkg
//grip-react:pkg
//grip-lab:app
//grip-lab/rust/griplab-core:crate
```

The descriptor is not a replacement for `package.json`, `Cargo.toml`, or
`pyproject.toml`. Those files remain the ecosystem source of truth. `BUILD.wsk`
or `BUILD.bazel` declares the orchestration target: package identity, release
policy, public targets, local cross-module edges, and command overrides when
default commands are not enough.

The tool should also discover and report edges from manifests:

- npm: `dependencies`, `peerDependencies`, `devDependencies`, local `file:`,
  `workspace:`, and first-party package scopes;
- Cargo: workspace members, `path =`, package names, `publish = false`;
- Python: `[project]` metadata, dependencies, build backend, local paths where
  present;
- Git: submodule URL, current commit, branch, dirty status, pushed status.

If declared edges and discovered edges disagree, `packages_maker doctor` should
fail. That is the main value of keeping descriptors local: the module owns its
declared edges, while the root orchestrator checks them against real manifests.

## Proposed commands

```text
packages_maker list
packages_maker graph
packages_maker doctor
packages_maker plan --changed origin/main..HEAD
packages_maker check [--all|--affected|--package ID]
packages_maker build [--all|--affected|--package ID]
packages_maker pack --package ID
packages_maker release-plan --bump patch|minor|major --package ID...
packages_maker release-prepare --plan release-plan.toml
packages_maker release-publish --plan release-plan.toml
packages_maker checkpoint --name NAME
```

`doctor` should validate:

- root submodules initialized and at expected paths;
- each first-party package has clean or explicitly allowed dirty status;
- npm lockfile package versions match package manifests;
- no publishable npm package contains `file:` dependencies;
- no publishable npm artifact includes local-only build output accidentally;
- Cargo packages marked `publish = false` are not scheduled for registry
  publication;
- Python package metadata is present and build backends are installed;
- required commands exist on the current platform;
- dependency graph has no cycles.

`graph` should produce both text and machine-readable output:

```text
packages_maker graph --format mermaid
packages_maker graph --format json
```

`check` should run type/lint/test gates in dependency order:

- npm library: install, lint if present, test if present, build,
  `npm pack --dry-run` or `pnpm pack --dry-run`;
- npm app/demo: install, lint if present, test if present, build;
- Cargo workspace/package: `cargo fmt --check`, `cargo clippy`, `cargo test` or
  `cargo nextest run`;
- Python package: `uv sync --frozen`, `uv run pytest`, `uv build`;
- PyO3 package: maturin build/check plus Python import smoke test;
- wasm package: wasm build plus npm package smoke test.

## npm release chain

The npm release path should be deterministic and artifact-based:

1. Compute release closure from selected packages.
2. Topologically sort by package dependencies.
3. For each publishable npm package:
   - ensure manifest version is the target version;
   - ensure first-party dependencies point at released target semver ranges;
   - build from a clean install;
   - run tests;
   - run `npm pack --dry-run` or `pnpm pack --dry-run`;
   - create the `.tgz`;
   - install that `.tgz` in a temp project with registry-resolved deps;
   - smoke-test import/types;
   - publish with provenance from CI when available;
   - poll `npm view <name>@<version>` until it resolves;
   - continue to dependents.
4. Tag the package repo.
5. Commit the root submodule pointer update.

For local development, prefer pnpm `workspace:` dependencies if the submodule
repos can tolerate being developed through the root workspace. pnpm will rewrite
`workspace:` references for pack/publish. If standalone submodule checkouts must
remain installable without the root workspace, then keep publishable semver in
`package.json` and let `packages_maker dev-link` create local links as a
development overlay. Do not leave `file:` refs in publishable manifests.

## Checkpoint model

The existing `plan-docs/AGENTS.md` roll-build discipline is already a good
checkpoint model. `packages_maker` should integrate with it rather than invent
another one.

Suggested checkpoint names:

```text
checkpoint/<plan>/<phase>/<name>
release/npm/<package>/<version>
release/rust/<crate>/<version>
release/python/<package>/<version>
root/submodules/<date-or-plan>
```

A checkpoint MUST record:

- package ids;
- exact versions;
- submodule commit SHAs;
- command plan;
- commands actually run;
- artifact names and checksums;
- tags created;
- registry URLs;
- rollback notes.

Release should be two-stage:

1. `release-prepare`: update versions, update dependents, generate changelogs,
   build/test/pack, and produce a release plan artifact.
2. `release-publish`: consume that exact plan and publish in graph order.

That split matters because npm, PyPI, and crates.io versions are not reusable
once published. The final publish command should do no improvisation.

## Cross-platform CI shape

Start with one root workflow that checks the whole graph:

```yaml
name: packages-check

on:
  pull_request:
  push:
    branches: [main]

jobs:
  plan:
    runs-on: ubuntu-latest
    outputs:
      matrix: ${{ steps.plan.outputs.matrix }}
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      - id: plan
        run: packages_maker plan --format github-matrix >> "$GITHUB_OUTPUT"

  check:
    needs: plan
    strategy:
      fail-fast: false
      matrix: ${{ fromJSON(needs.plan.outputs.matrix) }}
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      - run: packages_maker check --package "${{ matrix.package }}"
```

Initial matrix:

```text
os: ubuntu-latest, macos-latest, windows-latest
node: 20, 22
python: 3.11, 3.12
rust: stable
```

Keep expensive matrix cells targeted:

- run all npm library checks on all three OSes;
- run app builds on Ubuntu plus scheduled full matrix;
- run PyO3/cibuildwheel only on release branches or tags;
- run wasm checks on Ubuntu first, then add macOS/Windows when stable;
- run `rust-orbitdb` full workspace checks on Ubuntu per PR and broader matrix
  on schedule/release.

## Implementation plan for `packages_maker`

Use TDD. The first implementation should be boring and testable.

Phase 1: inventory and graph

- Discover and parse module-local `BUILD.bazel` or `BUILD.wsk` files.
- Parse local manifests.
- Detect first-party npm/Cargo/Python edges.
- Topologically sort.
- Detect cycles.
- Test success, failure, and edge cases.

Phase 2: doctor

- Check submodule presence and dirty status.
- Check npm lockfile root version mismatch.
- Check publishable packages for forbidden local refs.
- Check command availability.
- Test dirty/missing/bad-manifest cases.

Phase 3: task runner

- Execute `check`, `build`, and `pack` targets in graph order.
- Support `--affected` from Git diff.
- Record command output paths and status.
- Test ordering, failure stop, resume, and no-op packages.

Phase 4: release prepare

- Generate release plan.
- Update versions and dependent ranges.
- Create changelog entries or delegate to Changesets/Nx/release-plz.
- Build/pack/smoke-test artifacts.
- Test exact manifest rewrites and rollback on failure.

Phase 5: publish

- Publish only from a prepared plan.
- Poll registries.
- Tag child repos.
- Commit root submodule pointer updates.
- Test with fake registries or mocked publish clients before touching real
  registries.

## Tool ownership split

There is a separate but related ownership problem: the old `dev-setup` folders
mix generic root-repo tools with repo-specific tools. The generic pieces now
want their own lifecycle.

Observed examples across sibling roots:

| Tool | Likely ownership |
| --- | --- |
| `collect_dependencies` | generic root/submodule inventory helper |
| `vscode_configutator` | generic-ish editor/config bootstrap helper, maybe with repo-specific config |
| `release_maker` | generic release/versioning spike |
| `submodule_exec` | generic submodule command runner |
| `tag_rename.sh` | generic Git utility, if still needed |
| `run_glial_stack` | repo-specific Grip/Glial runtime helper |

Recommended layout, using `wsk` ("workspace kit") as the placeholder name for
the reusable submodule:

```text
glial-dev/
  wsk/                # generic reusable tooling submodule or pinned checkout
  dev-tools/          # glial-dev-specific/local tools; keep this name local
  MODULE.bazel        # if Bazel is adopted
  BUILD.wsk           # optional root defaults/discovery if Bazel is deferred
  .dev-tools.local/   # optional ignored personal/local overrides
```

`wsk` should be the reusable submodule. It SHOULD NOT contain Glial,
AnchorSCAD, Pyrolyze, or Grip-specific behavior except as examples/tests. It
MUST take root-specific behavior from config files, plugin entry points, or
commands supplied by the root repo.

`dev-tools` should remain the root-specific/local tooling bucket. For Glial, this is
where commands like `run_glial_stack`, phase-specific harnesses, local service
launchers, and Glial-only wrappers belong. It MAY import generic APIs from
`wsk`, but generic tools MUST NOT import from `dev-tools`.

`.dev-tools.local/` should be ignored and used only for machine-local scripts,
experimental probes, credentials-adjacent wrappers, or one-off developer
shortcuts. Nothing required for CI or release MAY live there.

The dependency direction should be:

```text
wsk       <- imported by dev-tools
wsk       <- configured by module-local BUILD.bazel/BUILD.wsk files / repo config
dev-tools <- may wrap wsk for glial-dev
```

This avoids the current ambiguity:

- generic tools get versioned, tested, and released once;
- root repos keep their own local knowledge without leaking it into shared
  tooling;
- the root repo can pin `wsk` as a submodule commit, giving reproducible
  release/checkpoint behavior;
- upgrades become explicit: update the `wsk` submodule pointer, run
  `packages_maker doctor`, and checkpoint the result.

`packages_maker` belongs in `wsk` if it is designed as a generic graph
orchestrator. `glial-dev/**/BUILD.bazel` or `glial-dev/**/BUILD.wsk`
descriptors and any Glial-specific check/build targets belong in `glial-dev`.
If the first implementation is developed inside `glial-dev` for speed, it
should still be structured so it can be extracted into `wsk` without rewriting
the command model.

## Recommendation

Adopt pnpm workspaces for the npm side first, because it directly addresses the
local-reference problem. Add Changesets unless Nx Release is selected.

In parallel, start `packages_maker` as a generic `wsk` Python or
TypeScript CLI that owns only inventory, graphing, doctor checks, command
planning, and checkpoint records. Glial-specific data should live in
module-local `glial-dev/**/BUILD.bazel` or `glial-dev/**/BUILD.wsk`
descriptors and optional `dev-tools` wrappers. Do not make it publish in v1.
Publishing should wait until graph, doctor, check, build, and pack are
boringly reliable.

Evaluate Nx after the inventory exists. If Nx can consume the declared package
graph cleanly, it may replace a large part of `packages_maker` task execution
and affected CI. If not, keep `packages_maker` as the root orchestrator and use
pnpm/Changesets/Cargo/uv/cibuildwheel as delegated tools.

The first concrete deliverable should be:

1. `BUILD.bazel` descriptors if Bazel is adopted, otherwise `BUILD.wsk`
   descriptors for the first-party modules
2. `packages_maker graph`
3. `packages_maker doctor`
4. a root CI workflow that runs `doctor` and package checks on Ubuntu

Only after that should release automation be added.
