# Process globals: the ratchet, and the plan to remove them

Plan, 2026-09-25, for the owner. The ask: "no globals", the rule gwz-core already
enforces (`gwz-dev/gwz-core`, commits `9e674476` and `730e7119`, "Add process-global
state ratchet check"), instituted in the Rust packages of this workspace, with a plan to
remove every global that can go.

## Summary

- **The rule.** Production code keeps no process-global mutable state. That covers:
  - statics with interior mutability, `static mut` and thread-local slots;
  - environment, working-directory and home-directory reads at the point of use;
  - process-wide hooks (panic hook, global logger or subscriber, C signal handlers);
  - child processes that inherit the live environment.

  State travels in configuration and context instead. A program reads its arguments
  and environment once, at its entry point, and passes them down. It spawns a child
  with `env_clear()` plus an explicit environment.
- **The ratchet (Phase 1, landed with this plan).** Each repository carries:
  - gwz-core's checker, `scripts/checks/check_process_globals.py`, vendored byte for
    byte from `730e7119`;
  - its own `scripts/checks/process_globals_allowlist.json`.

  glade's gate runs it as a ninth component, `process-globals`, and each other
  repository's `cargo test` runs it through a small test. The check fails in three
  cases: something new, a listed entry that is gone, and a count that changed. So the
  lists only shrink.
- **The inventory.** 25 items in six repositories: 18 debt and 7 permanent.
  glade-decl-rs has none, and glade's wire crate, contracts and client have none.
- **What stays.** Arguments read at a program's entry point, the node's
  composition-root switch, and one temp-name counter.
- **The plan.** Phases 2 to 4 pay the 18 debts in 9 steps, each small, and
  repository by repository, so separate agents can take them at once.

## 1. What the checker sees

The checker is a lexical scan, not a compiler pass. It strips comments and literals.
It follows `mod` and `#[path]` from the crate roots an allowlist names, and it reads
every platform branch without compiling any of them. It also scans, as production,
every other `.rs` file under a root's directory that no `mod` line reaches, and prints
a note that the file was not reached from a crate root. This is its fallback.

| Kind | What |
| --- | --- |
| `static` | a `static mut`, or a static whose type has interior mutability or lazy initialisation (`Atomic*`, `Mutex`, `RwLock`, `OnceLock`, `LazyLock`, `Cell`, …) |
| `thread_local` | a slot in `thread_local!` |
| `env` | `env::var`, `var_os`, `vars`, `set_var`, `remove_var`, `current_dir`, `set_current_dir`, `temp_dir`, `home_dir`, `args`; `dirs`/`home` crates' `home_dir`; libc's `getenv`, `setenv`, `chdir`, `umask`, `signal`, `sigaction` |
| `hook` | `panic::set_hook`, `take_hook`; `log::set_logger`, `set_max_level`; a tracing `set_global_default` |
| `process` | `Command::new`, keyed by its program literal where there is one |
| `libgit2` | `git2::opts` setters, `git2::transport::register` (no crate here uses libgit2) |

**Exempt:** code compiled only under `cfg(test)`, including a test-only
`cfg_if!` branch or `mod`. Each allowlist also names test-only features; glade names
`conformance`, the contracts' probe feature. Test directories (`tests/`) are outside
the crate roots and never scanned.

**One difference from gwz-core: programs are roots too.** A root is a file the walk
starts from. It is either a library (`src/lib.rs`) or a program (`src/main.rs`,
`src/bin/<name>.rs`). gwz-core names only libraries, which is all it has: its `src/bin`
is empty and every crate has a library. Here each allowlist also names the programs.

The fallback would still find most program files with library-only roots.
glade-node's, grazel's, glade-gwz's and glade-gyld's programs sit under their
library's `src/`. On 2026-09-26 each allowlist was rerun with library-only roots, and
those four repositories gave exactly the same items. Naming the programs still does
three things:

- **No note on every run.** With library-only roots, the checker reports each of those
  four programs on every run as "not reached from a crate root". With programs as
  roots, no repository prints a note.
- **Test-only module files stay exempt.** The fallback cannot see a `#[cfg(test)]` on
  the `mod` line that declares a file, so it would count that file as production. None
  of those four programs declares a module file today. taut-shape-tool declares 18, and
  one of them is test-only (`#[cfg(test)] mod snapshot_delta_json;`). The walk from its
  root exempts that one.
- **taut-shape-tool is checked at all.** It is a program with no library, so no library
  root points at its directory. With library-only roots, its 19 files go unscanned and
  its one allowlist entry goes stale.

The cost is that each program's read of its command-line arguments becomes an item,
recorded as *permanent* (question 2).

## 2. The inventory

As of this plan's Phase 1 commits.

| Repository | Where | Kind | Name | Disposition | Step |
| --- | --- | --- | --- | --- | --- |
| glade | `node/src/assembly.rs` | static | `REAL_PROVIDERS` (`AtomicUsize`) | debt | 4.1 |
| glade | `node/src/sysdir.rs` | env | `env::var` ×2 (`GLADE_HOME`, `HOME`) | debt | 2.1 |
| glade | `node/src/lifecycle.rs` | env | `env::temp_dir` | debt | 2.1 |
| glade | `node/src/bin/glade-node.rs` | env | `env::temp_dir` | debt | 2.1 |
| glade | `node/src/bin/glade-node.rs` | env | `env::args` ×2 | permanent | — |
| glade | `node/src/bin/glade-node.rs` | env | `env::var_os` (`GLADE_NODE_ASSEMBLED`) | permanent | — |
| grazel | `src/main.rs` | env | `libc::signal` ×2 | debt | 4.2 |
| grazel | `src/main.rs` | static | `NODE_PID`, `GWZ_PID`, `GYLD_PID` (`AtomicI32`) | debt | 4.2 |
| grazel | `src/main.rs` | process | `Command::new` ×2 (the node, the suppliers) | debt | 3.3 |
| grazel | `src/main.rs` | env | `env::args` | permanent | — |
| glade-gwz | `src/exec.rs` | process | `Command::new` (gwz) | debt | 3.2 |
| glade-gwz | `src/supplier.rs` | process | `Command::new` (a streamed gwz verb) | debt | 3.2 |
| glade-gwz | `src/bin/glade-gwz.rs` | env | `env::args` | permanent | — |
| glade-gyld | `src/agent.rs` | env | `env::var` (agent overrides) | debt | 2.2 |
| glade-gyld | `src/github.rs` | env | `env::var` (token variables) | debt | 2.2 |
| glade-gyld | `src/model.rs` | env | `env::var` (key variables) | debt | 2.2 |
| glade-gyld | `src/supplier.rs` | env | `env::var` (key variables) | debt | 2.2 |
| glade-gyld | `src/exec.rs` | process | `Command::new` (Python) | debt | 3.1 |
| glade-gyld | `src/github.rs` | process | `Command::new("gh")` | debt | 3.1 |
| glade-gyld | `src/github.rs` | static | `HELD` (`OnceLock<Token>`) | debt | 4.3 |
| glade-gyld | `src/bundle.rs` | static | `NEXT` (`AtomicU64`, temp names) | permanent | — |
| glade-gyld | `src/bin/glade-gyld.rs` | env | `env::args` | permanent | — |
| taut-shape-rs | `crates/taut-shape-tool/src/main.rs` | env | `env::args` | permanent | — |

Each allowlist carries the reason for each entry.

## 3. Phases and steps

**Rules for every step:**
- A failing test first.
- Braced control-flow bodies, and conditional compilation only inside a braced
  boundary.
- One commit per repository through gwz, then the root lock, with no attribution
  trailer.
- The step deletes the allowlist entries it pays. The checker then fails if one is
  left behind, and the step is done when the checker passes with the entries gone.
- A step that changes a binary the owner's desk runs (glade-node, grazel, glade-gwz,
  glade-gyld) replays the desk's start on a stand-in before it lands.

### Phase 1: the ratchet in every repository

Milestone: no process-global state lands unseen.

**Step 1.1: glade.**
- The vendored checker and glade's allowlist, whose roots are the node's library and
  binary, the wire crate, the ten contracts and client-rs.
- glade's gate gains the `process-globals` component, so `check.sh` runs nine
  components.
- client-rs gains `tests/process_globals.rs`, since client-rs changes are gated by its
  own `cargo test`.
- **Done, 2026-09-25,** glade `74636d0`:
  - The gate passes 9 of 9, and `process-globals` reports "49 files, 6 allowlisted items (4 debt, 2
    permanent); nothing new". client-rs's test passes.
  - Seen failing first: the allowlist without `REAL_PROVIDERS` fails with
    `NEW static REAL_PROVIDERS (AtomicUsize) at node/src/assembly.rs:203`, and one with a
    made-up entry fails with `STALE`.

**Step 1.2: grazel, glade-gwz, glade-gyld, taut-shape-rs, glade-decl-rs.**
- The vendored checker, an allowlist per repository, and `tests/process_globals.rs`.
- In taut-shape-rs the test sits in `taut-shape-tool`, which is not published. The
  published `taut-shape` crate carries no test that reaches outside its package.
- **Done, 2026-09-25:** grazel `8150b65`, glade-gwz `4bd274e`, glade-gyld `e9fcf6f`, taut-shape-rs
  `a8603b2`, glade-decl-rs `283e01b`.
  - Each repository's `process_globals` test passes against its allowlist: grazel 6 items,
    glade-gwz 3, glade-gyld 9, taut-shape-rs 1, glade-decl-rs none.
  - Seen failing first in each repository: an allowlist missing its first entry fails with `NEW`,
    and one with a made-up entry fails with `STALE`.
  - A static injected into a copy of glade-decl-rs fails with `NEW static INJECTED`.
  - Through `cargo test`, glade-decl-rs's test fails on a stale entry and passes once it is
    removed.

### Phase 2: the environment read once, at the entry point

Milestone: no library reads the environment.

**Step 2.1: glade-node's instance root and store directory.**
- **The change:** `sysdir::glade_home()` stops reading `GLADE_HOME` and `HOME`. Both
  composition roots read them once and pass the instance root into `boot`.
- **The legacy form:** its temp-directory default store (`glade-node.rs` and
  `lifecycle.rs`) comes from the settings the root builds, where the root reads
  `temp_dir` once (question 3).
- **The tests** set the root explicitly rather than through the environment. Many
  already give both `GLADE_HOME` and `HOME`.
- **Pays:** 4 entries (1 moves to the entry point as permanent, or goes, per
  question 3).
- **Size:** ~120 production, ~100 test.

**Step 2.2: glade-gyld's environment snapshot.**
- **The change:** `main` captures the variables the supplier reads, the agent
  overrides and the token and key variables, into an `Environment` passed through
  `GyldConfig`. `AgentOverrides::from_env` becomes `from_vars` over the snapshot, as
  its tests already call it. `github::discover`, `model::discover_key` and the
  supplier's key check read the snapshot.
- **Pays:** 4 entries.
- **Size:** ~150 production, ~120 test.

### Phase 3: child processes with an explicit environment

Milestone: no child inherits the live environment. Each child gets
`env_clear()` plus an environment its parent captured at start: the variables it
needs, or the whole start-up environment made explicit. Behaviour is unchanged except
that a variable set after start no longer leaks.

**Step 3.1: glade-gyld's Python and `gh`.**
- `exec` and `github`'s `gh auth token` spawn with `env_clear()` plus the snapshot
  from 2.2.
- **Tests:** a child that prints its environment sees only the snapshot, and a
  variable set after start is not seen.
- **Pays:** 2.
- **Size:** ~60 production, ~100 test.
- **Depends on:** 2.2.

**Step 3.2: glade-gwz's gwz runs.**
- `main` captures the environment once. `exec` and the streamed verb in `supplier`
  spawn gwz with `env_clear()` plus it.
- gwz needs `PATH`, `HOME` and, for SSH remotes, `SSH_AUTH_SOCK`. The snapshot keeps
  them.
- **Pays:** 2.
- **Size:** ~80 production, ~100 test.

**Step 3.3: grazel's node and suppliers.**
- The node and the composed suppliers spawn with `env_clear()` plus grazel's
  start-up environment, with `GLADE_HOME` set as today.
- The integration tests' full-stack start is the check, plus the desk replay.
- **Pays:** 2.
- **Size:** ~60 production, ~80 test.

### Phase 4: statics and hooks

Milestone: every allowlist holds only permanent entries.

**Step 4.1: glade-node's `REAL_PROVIDERS`.**
- The counter exists only so `tests/assembly_registration.rs` can see which real
  providers an assembly built.
- Give the test an observer of its own, for example a recording component in the
  test composition, or an assembly that reports what it constructed. Production then
  carries no counter.
- **Pays:** 1.
- **Size:** ~50 production, ~60 test.

**Step 4.2: grazel's shutdown.**
- Today a C signal handler, installed with `libc::signal` for SIGINT and SIGTERM,
  kills the children through three PID statics.
- A signal task (`tokio::signal`) in `main` that owns the `Child` handles replaces
  the handler and the statics.
- **Tests:** SIGTERM to grazel stops the node and both suppliers, and leaves no child
  behind; today's integration shutdown test is the red.
- **Pays:** 4 entries (the handler counted twice, and three statics).
- **Size:** ~100 production, ~80 test.

**Step 4.3: glade-gyld's cached token.**
- `github::discovered()` caches the token process-wide in `HELD`.
- The supplier discovers it once, from the snapshot (2.2) and `gh` (3.1), and holds
  it in its context.
- **Pays:** 1.
- **Size:** ~60 production, ~60 test.
- **Depends on:** 2.2, 3.1.

## 4. Order and parallelism

```
Phase 1 (landed)
  glade:       2.1 ── 4.1                 (the node lane: one agent in glade at a time)
  glade-gyld:  2.2 ── 3.1 ── 4.3
  glade-gwz:   3.2
  grazel:      3.3 ── 4.2                 (either order; 4.2 first if the desk prefers)
```

- **The repositories are independent.** glade-gyld, glade-gwz and grazel can each
  have an agent at once.
- **Within glade-gyld,** 2.2 comes first.
- **The two glade steps touch the node.** They share the node lane with the first
  slice's parked 4.1c (question 4).

## 5. What stays permanent

- **Arguments read at an entry point:** glade-node (×2), grazel, glade-gwz,
  glade-gyld and taut-shape-tool. Read once, passed down.
- **glade-node's `GLADE_NODE_ASSEMBLED`:** it chooses the composition root before
  anything else runs.
- **glade-gyld's `NEXT`:** a temp-name counter, only ever incremented, with no shared
  state.

## 6. What the ratchet does not see

- **Dependencies' own process state:** tokio's runtime, iroh's threads and metrics,
  rustls's crypto provider. They are not scanned.
- **Process-wide effects the checker has no pattern for:**
  - writes to stdout and stderr, and `process::exit`;
  - `tokio::signal`, which Step 4.2 introduces in grazel. That is a binary owning its
    process's signals, the same boundary as its arguments.
- **What a lexical scan cannot see:** an aliased import (`use std::env as e`), a
  global reached through a re-export or a macro from another crate.
- **Outside the scan:** glade-discover, which holds someone else's uncommitted work,
  and glade's `dev-docs` witnesses, which are not production code.
- **Vendored copies:** each repository holds a copy of gwz-core's checker. An update
  there is copied here deliberately, with the source commit recorded in each
  allowlist.
- **Python:** the checker needs Python 3.10 or later. The Mac and the Pi have it;
  dabeest's MinGW shell is unchecked.

## 7. Questions for the owner

1. **Programs as roots,** unlike gwz-core (§1). Recommend yes. Each program is then
   walked like a library: no run prints a "not reached" note, a program's test-only
   module files stay exempt, and taut-shape-tool, a program with no library, is
   checked at all. The other programs' files would be scanned either way, through the
   checker's fallback. No means matching gwz-core exactly: a note on every run for
   four programs, a program's test-only module files counted as production, and
   taut-shape-tool unchecked. (Corrected 2026-09-26: this question first gave the
   reason that grazel's globals would otherwise be missed. The fallback scans them.)
2. **A read at an entry point is permanent.** Recommend yes: it is the capture point
   the rule asks for.
3. **glade-node's legacy temp-directory store:** read `temp_dir` once at the entry
   point, which keeps today's behaviour, or drop the default and require a store
   directory. Recommend reading it once at the entry point.
4. **Where the node steps fall.** Recommend 2.1 and 4.1 in the node lane before
   parked 4.1c resumes: they are small, and 4.1c adds a boot path that would
   otherwise read the environment where it is used too. The supplier repositories'
   steps can start at once.
5. **Six vendored copies or one shared checker.** Recommend vendored copies for now,
   with the source commit recorded. A shared tool can follow if the checker starts
   to change often.
6. **Agents' instructions.** Recommend adding the rule to each repository's agent
   instructions (`AGENTS.md` or `CLAUDE.md`), so every brief carries it.
