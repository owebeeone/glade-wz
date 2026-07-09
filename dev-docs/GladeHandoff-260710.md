# Glade Handoff — 2026-07-10

Status: handoff note for the next agent picking up glade work. Written after the
glade-wz workspace went live (2026-07-07) and the three build lanes each cleared
step 1 (2026-07-08). Reflects state at the commits pinned below.

Read alongside — these are the authoritative, living docs (this note points at
them, it does not replace them):

- `dev-docs/GladeProgramStatus.md` — the one living stage-tracker (stages,
  decision queue, the three lanes). **Read this first.**
- `dev-docs/DecisionLog.md` — GDL decisions; GDL-031…038 ratified 2026-07-07.
- `dev-docs/StackMap.md` — layer map.
- `dev-docs/glade/*` and `dev-docs/glial/*` — the design corpus.
- `ggg-viz/comments.json` — Gianni's state-anchored review queue on the atlas.

---

## 1. Where the work moved: glial-dev → glade-wz

**glade-wz (`~/limbo/glade-wz`) is now the home of all glade/glial/ggg work.**
`~/limbo/glial-dev` is a **FROZEN fallback** — do not develop glade there. Its
copies of the design docs are stale by definition.

Why the move: glial-dev was a single-repo checkout that accreted the glade node,
grip runtime, taut, the design corpus, and ggg-viz all in one tree with
submodule stubs that were never populated. It could not express "ten
independently-versioned repos that must move together." glade-wz replaces that
with a **gwz multi-repo workspace** (see §2).

What physically happened during the migration (2026-07-07):

- The design corpus (`dev-docs/`), `plan-docs/`, and `docs/` were committed to
  the **workspace root** repo (`owebeeone/glade-wz`), which is the gwz
  *container*, not a member.
- The five pre-existing repos — `glade`, `grip-core`, `grip-react`, `taut`,
  `taut-shape` — were `git clone`d into the workspace and registered as members
  via `gwz repo add`.
- The five new repos — `glade-decl` (+`-ts`/`-rs`/`-py`) and `ggg-viz` — were
  created on GitHub (`scratch/make-repos.sh`), populated, and pinned.
- A tenth member, `glial`, was added later as the extraction target (§5).
- ggg-viz's grip dependencies were re-pointed from `file:../../glial-dev/*` to
  workspace-local `file:../grip-core` / `file:../grip-react`.

### Push authentication gotcha (you WILL hit this)

The SSH key on this machine authenticates to GitHub as **`gripd`**, which has
**no push access** to `owebeeone/*`. SSH *reads* (clone/fetch of public repos)
work fine; SSH *pushes* fail with `Permission … denied to gripd`. `gh` is logged
in as `owebeeone` with a repo-scoped token. So push over HTTPS with gh's
credential helper, leaving the ssh remotes as Gianni configured them:

```sh
git -c credential.helper='!gh auth git-credential' \
  push https://github.com/owebeeone/<repo>.git main:main
```

Do **not** "fix" his SSH/git config — the mismatch may be intentional.

---

## 2. gwz: the multi-repo workspace mechanism

`gwz` (`~/.cargo/bin/gwz`) manages the workspace. **Members are NOT git
submodules** and the workspace is NOT reproduced with `git clone`. Key facts:

- **Clone the workspace with `gwz clone`, never `git clone`.** A plain
  `git clone` of `owebeeone/glade-wz` gives you the container only (manifest +
  design corpus) — a skeleton with no members.
- The manifest + lock live under the tracked `gwz.conf/` directory in the root
  repo: `gwz.yml` (members, paths, remotes, desired branch) and `gwz.lock.yml`
  (each member pinned to an exact commit).
- Each member is an ordinary independent git repo in its own subdir. You commit
  and push members individually (with the HTTPS trick above).
- After changing member revisions, **`gwz capture`** records the live worktree
  HEADs into the lock; then commit + push the root so the pins travel.

Useful verbs (all run from the workspace root): `gwz ls`, `gwz status`,
`gwz capture`, `gwz repo add <path>`, `gwz repo create <path>`, `gwz repo sync
<path>`, `gwz forall … -- <cmd>`, `gwz pull`, `gwz push`. `gwz help <verb>` for
details.

### The ten members (HEADs at handoff)

| Member (path) | Repo | HEAD | Role |
| --- | --- | --- | --- |
| `glade` | `owebeeone/glade` | `2dc3545` | rust node + wire + client-ts + grip-share binder + demo (Lane R) |
| `grip-core` | `owebeeone/grip-core` | `8965577` | TS reactive runtime; now imports glade-decl types |
| `grip-react` | `owebeeone/grip-react` | `c46e14f` | React bindings for grip |
| `taut` | `owebeeone/taut` | `83da9f5` | the codegen tool (`tautc`) + runtime; see §3 |
| `taut-shape` | `owebeeone/taut-shape` | `2376f2f` | message-shape contracts + oracle corpora (§4) |
| `glade-decl` | `owebeeone/glade-decl` | `ccdae14` | the declaration-surface CONTRACT (schema + IR + corpus) |
| `glade-decl-ts` | `owebeeone/glade-decl-ts` | `4467e31` | generated TS rendering, corpus-gated |
| `glade-decl-rs` | `owebeeone/glade-decl-rs` | `42cca3c` | generated Rust rendering, corpus-gated |
| `glade-decl-py` | `owebeeone/glade-decl-py` | `37480ba` | generated Python rendering, corpus-gated |
| `ggg-viz` | `owebeeone/ggg-viz` | `2d33428` | the trace atlas / visualizer (§5) |
| `glial` | `owebeeone/glial-runtime` | `d35bd48` | client-kernel — seed only, fresh build (§6) |

Note the `glial` name mismatch: the member path is `glial` but the repo is
**`owebeeone/glial-runtime`** — because bare `owebeeone/glial` is glial-dev's
own historical remote and was left untouched.

**Not yet members** (exist on GitHub, populated, but not cloned into the
workspace): `taut-shape-ts`, `taut-shape-rs`, `taut-shape-py`. These are needed
for taut-shape's cross-language interop matrix (§4) — add them with
`gwz repo add` when that work starts.

### Fresh-clone bootstrap gotcha

`grip-core` and `grip-react` ship **`dist/`-only package entry points** and do
not commit `dist/`. A freshly-cloned workspace cannot resolve
`@owebeeone/grip-core` / `@owebeeone/grip-react` until each is built:

```sh
cd grip-core && npm install && npm run build   # tsup
cd grip-react && npm install && npm run build   # tsup
```

Until then, ggg-viz's vite dev server errors with "Failed to resolve entry for
package". This also silently masked as ggg-viz's "pre-existing build errors" —
they were just the missing grip-react dist. A root bootstrap recipe
(`gwz forall`) is an open nicety, not yet written.

---

## 3. tautc: the codegen tool and the regen discipline

**taut** is Gianni's codegen tool. A `.taut.py` schema file describes messages
(structs), enums, and int/bytes/string fields in one language-neutral place;
`tautc` renders it to native types + a CBOR codec in Rust, TypeScript, and
Python. This is the "declarative-first, boilerplate-is-a-smell" principle made
concrete: **the wire shape is declared once; every language's types are
generated, never hand-written.**

### Invocations (verified working at handoff)

From a repo that holds a schema, with taut as a sibling member:

```sh
# glade-decl renderings (run from glade-decl/, taut at ../taut):
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py -o <out> -l rust       --api-only --with-runtime
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py -o <out> -l typescript --api-only --with-runtime
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py -o <out> -l python     --api-only

# glade node's system-data types (run from taut/):
PYTHONPATH=src python3 -m taut.cli gen ../glade/node/ir/sysdata.taut.py -o <out> -l rust --api-only
```

Output lands under `<out>/<lang>/` (e.g. `<out>/rust/api.rs`). `--api-only`
emits messages only (no service/RPC scaffolding); `--with-runtime` bundles the
CBOR runtime helpers. **Never hand-edit generated files.**

### The golden-corpus discipline (this is the whole game)

Each rendering is gated against a **frozen golden corpus** owned by the
contract: representative message instances encoded to canonical CBOR by taut's
Python reference codec. Each language's test decodes + re-encodes the corpus and
asserts **byte-identical** output. Because Rust and TS reproduce Python's bytes
through *independent* codecs, this is a genuine cross-language conformance proof,
not a tautology.

- glade-decl: `corpus/build.py` exports the IR, writes the golden
  (`corpus/decl.v0.json`), and regenerates `glade-decl-rs/src/vectors.rs`.
  `python3 corpus/build.py --check` is the drift gate (3 artifacts lockstep).
- glade-decl-{rs,py,ts}: `cargo test` / `pytest` / `npm test` each run the
  corpus byte-parity gate.
- taut-shape: `python3 ir/regen.py --check` (IR lockstep for all shapes) and
  `python3 corpus/value_gen.py --check` (oracle lockstep).

### When taut is re-pushed (it was, on 2026-07-08 → `83da9f5`, "int64 support
all langs") — the drill:

1. `cd taut && git pull --ff-only`.
2. Re-run every `--check` gate above. **If a gate stays green, wire bytes did
   not change** — the contract still holds.
3. Regenerate every rendering into a scratch dir and `cmp` against the committed
   generated files to see what the new tautc actually changed.
4. **If only generated code changed and the corpus gate is still green, adopt
   the regen verbatim** (copy generated files in, commit) — no hand edits.
5. Re-run downstream consumers (grip-core tests + `tsc`, ggg-viz tests).
6. `gwz capture`, commit + push root to re-pin taut.

The `83da9f5` bump did exactly this: Rust/Python/sysdata output was
**byte-identical** (Rust already `i64`, Python ints arbitrary-precision); only
**TypeScript drifted** — i64 fields (`ttl_ms`, `seq`, `base_seq`) became
`bigint` instead of `number`, plus typed `DecodeError`/`EncodeError` with I64
range enforcement. Adopted verbatim (`glade-decl-ts@4467e31`); corpus 25/25
unchanged; grip-core (220 + clean tsc) and ggg-viz (219) unaffected because the
grip-core swap imports only the enum types, none of the bigint fields.

**If a future taut push turns a corpus gate RED** (wire bytes changed): do NOT
silently regenerate the golden. Bring the drift to Gianni — a changed wire shape
is a design event, not a codegen refresh.

---

## 4. taut-shape: managing message shapes

`taut-shape` owns the **delivery-shape contracts** — the reusable CRDT/stream
semantics that glade's `Shape` enum (`value`, `log`, `message`, `stream`,
`exchange`, `window`, …) *names* but does not define. Each shape is a
`.taut.py` wire schema **plus** a golden oracle corpus that pins its fold
semantics with a Python reference implementation.

At handoff, two shapes are built:

- `shape_log` (pre-existing).
- `shape_value` — Lane C step 1, landed 2026-07-08 (`2376f2f`): an LWW register.
  `ir/shape_value.taut.py` (schema) + `corpus/value.v0.json` (11-vector oracle,
  generated from `taut.crdt.glade_fold.fold_value` so the fold stays
  authoritative). `ir/regen.py` gates both shapes' IR; `corpus/value_gen.py`
  gates the oracle.

The split of concerns to keep straight:

- **taut** = the codegen tool + generic CBOR runtime + the fold reference
  implementations.
- **taut-shape** = the per-shape *contracts + oracles* (the "what does a `value`
  converge to" spec), language-neutral, no engine of its own.
- **glade** = names shapes and routes them (`share`/`glade_id`/`key`); owns the
  node, wire carrier, and the live folds.
- **glade-decl** = the thin leaf declaring `Shape` (and GladeId/Domain/Zone/
  BindingDecl/…) so grip-core and glial share one vocabulary (§ below).

**Open**: taut-shape P1's S3 interop matrix (live A-vs-B across
`taut-shape-{ts,rs,py}`) is **blocked** — those three repos aren't workspace
members yet. The Python-reference corpus already proves conformance-against-spec;
the matrix adds cross-language live checks once the sibling repos are added. P2
(merging glade's fold oracles into taut-shape corpora) is the next Lane C step.

### glade-decl — the shared leaf (how the shapes reach the runtimes)

`glade-decl` is the razel `*-api`-crate idiom: a **leaf module both grip-core
and glial import without importing each other.** Contract (schema + IR + golden
corpus, zero language code) lives in `glade-decl`; renderings are committed
generated types in `glade-decl-{ts,rs,py}`. As of `8965577`, **grip-core's
`ShareDecl` field types come from `@owebeeone/glade-decl-ts`** (one seam module,
`src/core/share_decl.ts`, is the only importer) — a compile wall: out-of-contract
drift is a hard `TS2322`. Runtime behavior unchanged (types-only, erases at
build). glial (§6) will import the same leaf; that closes the seam.

---

## 5. ggg-viz: the trace atlas / visualizer (the spec leads the build)

`ggg-viz` is an interactive **protocol-scenario explorer** built with grip-react
(**no React state hooks** — a hard rule, gated by
`scripts/no-react-state.test.mjs`). It is not decoration: **the atlas LEADS the
build.** Each protocol behavior is authored as a *typed trace* (actors, steps,
frames, invariants) BEFORE the code that implements it — the trace is the
executable spec, and the code must match its observable behavior.

- Traces live in `src/scenario/*.ts` with per-trace integrity tests; generic
  suite tests + invariants run over all of them. At handoff: **28 traces, 5
  invariants, 219 tests** (`npm test` = no-react-state gate + vitest).
- The stage-1 / stage-2 split of the traces **IS the build phasing**: build
  everything end-to-end at stage-1 posture (allow-all, seams present), then
  switch stage-2 (security/enforcement) on. Both postures are specified by the
  same traces.
- Run it: `preview_start` config `ggg-viz-wz` (port 5178) in glial-dev's
  `.claude/launch.json`, or `npm --prefix ggg-viz run dev`. Verified rendering
  clean against grip HEAD on 2026-07-08.

Three traces were authored 2026-07-08 as the specs for the lanes' step 1
(`253518e`): **s-boot** (system-data seam — the spec Lane R built against),
**s-app-register** (`.glade` loaded as data), **s-zones** (commons-by-grant vs
private-by-key).

### The comment loop (do this at session start)

Gianni leaves **state-anchored review comments** in `ggg-viz/comments.json`. At
the start of a glade/ggg session, read it. To address one, set its `status` +
`reply` in the file. At handoff all 7 comments are `addressed` (awaiting his
re-read), so the queue is effectively clear.

---

## 6. Current build state + what's next

GDL-031…038 **ratified 2026-07-07**. The glade-dev extraction was **decided
YES** — the glial client-runtime gets its own home: repo
`owebeeone/glial-runtime`, member path `glial`, currently a **seed commit only**
(charter README, no code).

All three lanes cleared **step 1** on 2026-07-08 (verified + pushed):

- **Lane R** (rust node) — system-data seam: `SystemSnapshot` taut msg,
  `RegistryApi`/`StoreApi` traits, `~/.glade/sys/<name>/` layout + load-
  validation ladder, blob≡fold conformance test (`glade@7394ce5`, fix
  `2dc3545`). **Next: R2** — iroh carrier + node↔node HELLO + heads/gap sync.
- **Lane T** (TS/glial) — glade-decl contract + 3 renderings + grip-core swap
  (compile wall). **Next: T2** — glial binder v0 in the `glial` member
  (persistence-first store-only path → mount→session), retiring direct
  tap→glade coupling. s-stack-* traces are the spec.
- **Lane C** (contracts) — `shape_value` P1. **Next: C P2** — merge glade fold
  oracles into taut-shape corpora (and add the `taut-shape-{ts,rs,py}` members
  for the S3 matrix).

E2E stage-1 definition-of-done (from GladeProgramStatus): browser (glial binder)
↔ glade-local ↔ glade-peer over iroh; workspace list from the registry; one
grazel binding live; one gwz exchange round-trips; all snapshot files under
`~/.glade/sys/`; every behavior matching its trace.

### Awaiting Gianni (do not proceed without his call)

Two INV-4 questions surfaced authoring **s-zones** — recorded in the decision
queue in `GladeProgramStatus.md`:

1. Private-zone serves satisfy INV-4 via the participant's **doc-membership**
   grant (no zone-specific grant ever exists; asserted mechanically — no grant
   key contains `self:`). Confirm this is the intended reading.
2. Account-domain serves were modeled as **owner self-grants**. If account
   domains are meant to be self-authoritative *without* a grant record, INV-4
   needs a `home`-style carve-out.

Plus the standing product calls (WD-1 root custody "the big one", AZ-1/2/3,
WD-4/6, AZ-7/13) — see the decision queue.

### Operating rules that bit us (carry them forward)

- **The atlas is the regression net.** No code change may alter an existing
  trace's observable behavior. Lane R's first commit made node boot touch the
  real `~/.glade` unconditionally, which broke grip-share's legacy
  `glade-node <port> <dir>` spawns on the singleton instance-lock — caught only
  because the gate was actually re-run. Fixed by making sysdir boot opt-in
  (`--profile`/`--name`); legacy positional serve form never touches `~/.glade`.
  **Tests must never write to the real `~/.glade`** — use temp dirs / `GLADE_HOME`.
- **Verify before asserting; re-run the actual gate.** A subagent reporting
  green is not the gate being green.
- **TDD, byte-identical moves, no rustfmt on pure moves, no `Co-Authored-By`
  trailers**, commit/push only when asked (obey `AGENTS.md` + the glial-dev
  `CLAUDE.md`, which carry the full rules).
