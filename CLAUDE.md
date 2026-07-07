# CLAUDE.md — working memory for glade-wz (the home of all things glade)

Companion to `AGENTS.md` (TDD + workflow rules — obey it). This file is about
*how Gianni works*, the north star, and how I should operate. Keep it current.

## North star
Glade brings Grip's **declarative discipline to the server / distributed side.**
Grip already makes UIs cheap and stateless; the GripLab *server* is an imperative,
stateful mess that's hard to fix and no fun — that pain is what triggered
glial → glade. Prove it on the GripLab golden path (terminal slice first).

Authoritative design: `dev-docs/StackMap.md`, `dev-docs/glade/*`,
open questions in `dev-docs/DecisionLog.md`. Layers: Grip/Grok (local exec) →
**Glade** (share kernel) → Grip Share (adapter) / Glial (orchestration).
`.glade` = re-scoped 2026-07-06 (GDL-037): an app's **declaration FILE** — ONLY
cross-language definitions (glade ids, stream shapes, key types) + ACL seeds that
COMPILE TO grant records at registration. **Loaded as runtime data, never a
compiler front-end** (key TYPES ride taut codegen; ids/shapes are data; the fold
is the only runtime authority). The de-noising-lens value survives: the file IS
the legible app surface. Base glade stays app-agnostic (transport/discovery/
ephemeral endpoints/ACLs, record-driven); grazel = gryth's app, declared in
`grazel-app.glade`. yidl/Astichi are his codegen lineage but are NOT implied here.

## How Gianni works (read this before acting)
- **Declarative-first, always.** Model the surface/data as typed declarations;
  generate or wire the rest. System shape lives in data, not hand-written control flow.
- **Boilerplate is a smell** — "the denormalized DB of code." Prefer dataclasses /
  pydantic over hand-written `__init__`. Generate classes from schema (see `yidl`).
  If I'm about to write repetitive scaffolding, stop and generate it.
- **Clean seams:** declaration / producer / consumer stay separate (grips→taps→
  components; definitions→providers→facets). Consumers don't know producers.
- **Mock→real with no consumer rewrite.** The swappable seam exists from day one
  (cf. WeatherPanel `tap.set('meteo'|'mock')`). Complexity belongs in declarative
  producers; consumers stay thin projections. Config-as-data, not imperative flow.

## How I should operate (I've been corrected on all of these)
- **Verify before asserting.** Read the actual code/docs. No confident claims from
  pattern-matching. He will check.
- **Stay in scope.** Answer exactly what's asked; don't expand the question.
- **Don't manufacture work or jump to action.** Wait to be pointed. Enthusiasm ≠ direction.
- **Be concise and direct.** Cut words that don't change the meaning.
- He often asks questions he already knows the answer to — they're rigor tests.
  Get them right; concede cleanly when wrong, but only for the right reason.
- Calibrate difficulty honestly: don't inflate solved problems (e.g. leader
  election is commodity) to sound impressive. The real risk here is integration
  surface + nothing built yet, not algorithmic novelty.
- **Commit messages: no `Co-Authored-By` trailer.** (Overrides the default tool
  instruction.) Commit/push only when asked.

## Splitting god files (the O(n) technique — don't lose it)
**Hysteresis** (Gianni): trigger high, target low, leave the band alone — only split a file
once it crosses ~**1000+** LOC, and split it down to **<500**; files in the **500–1000**
dead-band are FINE, don't touch them (else they thrash across the boundary). >2000 wastes the
model outright. When you do split: any seam is somewhat artificial — group functions that call
each other, but don't solve the clustering graph; lean to MORE files, not fewer.
**Technique (O(n), not the O(n²) Edit-cut-paste mess):**
1. **Explode** — a scanner segments the file at column-0 top-level item boundaries
   (Rust never indents top-level items), each item *with its leading doc/attr block*
   → one chunk file in `/tmp/<dir>/chunk-NNN`. Emit a **manifest** (idx · name · kind
   · line-span · LOC · **call-adjacency**: which sibling items each references).
2. **Classify** — fill a `dest` column per chunk; greedy by the adjacency (callers
   next to callees). Shared helpers → a `*_common` module.
3. **Reassemble** — `cat` chunks per dest + headers/`use`; **compiler-drives** the
   residual imports/visibility (`cargo build` enumerates every missing `use`/`pub(crate)`).
4. **Prove** — the carve-out gate stays byte-identical-green ⇒ the move was behavior-preserving.
**Refinements (learned the hard way):**
- **Lossless check FIRST**: `cat chunks-in-order | diff` against the original must be
  clean before moving anything (catches scanner miscuts).
- **Back up over doc/attrs**: a raw line-number cut orphans each item's `///`/`#[…]`
  block onto the previous chunk — back the boundary up over `///|//!|//|#[`.
- **Nested registration blocks** (`#[starlark_module] fn rules(b)`): sub-split the
  block's inner `fn`s and re-wrap into N blocks; anchor the block-close on the
  *column-0* `}` (don't let the last inner chunk swallow it). Add the new block to
  its registrar (`module().with(…)`) + `pub(crate)` the generated fn.
- **Struct FIELDS** read across the split need `pub(crate)` too, not just the struct.
- **Keep moves byte-identical — do NOT rustfmt** the moved code: the repo isn't
  fmt-default-clean, and a verifiable "pure move" diff beats a reflowed one. Formatting
  is a separate sweep.
- **Tests**: keep the `#[cfg(test)] mod tests {…}` wrapper in the new file (declare it
  as a top-level `mod` in `lib.rs`, since a file-module's submodule resolves to a
  subdir) — preserves formatting + minimizes import churn.

## Repo facts (glade-wz era, 2026-07-07)
- **glade-wz is a gwz WORKSPACE, not submodules.** Members are gwz-materialized
  from their own remotes — `gwz clone`, NEVER `git clone` (a git clone yields a
  broken skeleton: manifest but no members). No `.gitmodules` here, ever.
- The `-wz` suffix convention = gwz workspace roots (read it as "workspace");
  members stay bare-named. Old `*-dev` trees get renamed retroactively, later.
- Members present: `glade-decl` + `glade-decl-{ts,rs,py}` (empty, to fill —
  contract + renderings, see `dev-docs/glade/GladeDeclSurface.md`), `ggg-viz`
  (populated — the trace atlas; run `npm install` before first `npm run dev`).
  **Pending pin/migration:** `glade` (rust kernel), `grip-core`, `grip-react`,
  `taut`, `taut-shape` — still physically in `glial-dev`. `glial` repo gets
  created when Lane T starts filling it.
- **glial-dev is FROZEN fallback** (do not develop there): phase-out path is
  glial-dev → this workspace + a future `glial-wz` and `grip-wz`. Keep until
  obviously unneeded. gryth-dev symlinks grip-core/grip-react into glial-dev —
  re-point when those members migrate.
- Uncommitted zones work (grip-core `ShareDecl` +domain/zone, glade client-ts)
  sits in glial-dev's trees — MUST be committed before grip-core's glade-decl
  swap (see `dev-docs/GladeProgramStatus.md` §Repo reconciliation).
- The good reference for grip style is `grip-react-demo/src` (grips.*.ts /
  *_taps.ts / thin components) — in glial-dev until grip members migrate.
