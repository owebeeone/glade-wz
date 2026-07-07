# Razel Architecture Review — where it diverges from Bazel, and why (2026-06-23)

**What this is.** A comparative architecture + code review of `glial-dev/razel` against real Bazel
(`bazel-dev/bazel`), commissioned because razel keeps accreting bugs that deviate from its stated goal
of Bazel input-fidelity: it is **slower than Bazel loading large trees (TensorFlow)**, gives **poor
build feedback**, and **fails to compile targets Bazel builds fine**. The brief: find the *fundamental*
architectural gaps — the places where an unauthorized design shortcut was taken and is now being
maintained with band-aids where a structural fix (rewrite) is the honest cost.

**Method.** Seven subsystem pairings were deep-read on both sides (razel Rust ↔ Bazel Java), each
claimed gap was then **adversarially re-verified** against the actual code (the verifier's job was to
*refute*, not confirm), and a completeness pass looked for missing comparisons. Every claim below cites
the file:line that was read on **both** sides. Refuted/over-stated claims are kept in §7 so the picture
is calibrated, not inflated. The taut-vs-protobuf substitution was checked and is **clean** (§7) — it is
not a source of any symptom, so it is not litigated further.

Authoritative razel context this reconciles against: `razel/dev-docs/RazelDevStatus.md` (the AD1–AD9
decision table + the "designed vs built" §4) and `razel/dev-docs/ArchFundamentals.md` (the F1–F25
fundamentals). Where razel's own docs already admit a gap, this review says so and grades it.

---

## 1. Thesis (the one root cause)

**razel's live path took one shortcut, and every reported symptom descends from it.** The live loader
(`crates/razel-loading`, ~27k LOC) does **not** run Bazel's rulesets or use Bazel's toolchain model.
Instead it:

1. **Reimplements each ruleset natively in Rust** (`native_cc.rs`, `rust_rules.rs`, `py/sh/js_rules.rs`
   + baked `cc_defs.bzl`/`java_defs.bzl` shims) and papers over the unimplemented host API surface with
   a magic **`Absorb`** value that swallows any access; and
2. **Runs loading + analysis as a one-shot batch into a throwaway `Session`**, with no persistent,
   per-node demand graph. The daemon's only loading-reuse knob is a **single digest over every BUILD
   file in the whole tree**.

Bazel does the opposite on both axes: it **evaluates the real fetched `.bzl`** inside a **persistent,
fine-grained Skyframe graph** (one cached node per `.bzl` / package / glob / configured-target, with
early cutoff), and **binds tools by constraint-based toolchain resolution**.

Consequences, one-to-one:

| Reported symptom | Architectural cause | §  |
|---|---|---|
| **Slow loading of TF; "re-evaluates Starlark far more often than needed"** | No incremental loading/analysis graph. Any BUILD edit (or a different requested target) discards the entire analysis and re-evaluates the closure from a cold `Session`; even no-op builds re-read+rehash every BUILD file in the tree. | **§3** |
| **"Has not used Bazel's toolchain mechanisms"** | There is no toolchain resolution to adopt. Tool selection is a host constant; `resolve_toolchain` is `match name { "cc" => macos_core_config() }`. The only extension seam is *more host rows*, which is why agents band-aid instead of adopting the model. | **§4** |
| **"Fails to compile where Bazel compiles fine"** | Native rules reproduce only the hand-coded subset of each ruleset; anything outside it (a feature, a provider field, a `cc_common` call) hits `Absorb` and **silently yields empty/garbage inputs** rather than failing loudly. | **§5** |
| **Poor feedback** | Progress is an execute-phase side-channel; the loader/analyzer have **no event surface**, so the slow TF load+analysis window emits nothing. | **§6** |

**Verdict in one line:** the **loading/analysis spine** and the **tool-binding layer** need structural
rewrites toward the designed substrate; the **action-execution engine, the wire seam, and within-session
caching are sound and should be kept** (§7). The current failure mode is a *band-aid treadmill* (more
host rows, more `Absorb` fields, coarser digests) that entrenches the wrong shape with each fix.

---

## 2. The shape, side by side

```
                         BAZEL (live)                              RAZEL (live = razel-loading)
  rule logic        real fetched .bzl, evaluated in           native Rust reimpls (cc/rust/py/sh/js)
                    Starlark (BzlLoadFunction)                 + .bzl shims; Absorb swallows the rest
  tool binding      toolchain resolution over platform/        resolve_toolchain("cc") → one baked
                    constraint (ToolchainResolutionFunction)   macOS host fixture; rules carry host flags
  loading/analysis  persistent Skyframe graph: one node per    one-shot batch into a fresh Session;
                    .bzl/package/glob/target, early cutoff,    no cross-build node graph; daemon reuse
                    per-node invalidation across builds        gated by a single whole-tree BUILD digest
  config            (Target × BuildConfiguration) is the key;  single Session-global GlobalFlags;
                    transitions, exec/target split             analysis keyed by Label only (host==target)
  execution         SpawnStrategy/SpawnRunner contract;        one hardcoded local subprocess; no worker
                    sandbox/worker/remote/dynamic; workers     protocol; sandbox dir is the only "warm"
  feedback          EventBus → UiEventHandler; loading,        per-action callback installed only after
                    analysis, execution all post progress      analysis; loader/analyzer emit nothing
  wire              protobuf at boundaries (BEP/remote/query)  taut/CBOR at the RPC boundary — same seam ✅
  exec engine       Skyframe node versions + early cutoff      razel-engine: real Skyframe-lite, early
                                                               cutoff, tested ✅ (but actions only)
```

The two green ✅ rows are the parts that are architecturally *right*. Everything above them is where the
shortcut lives.

---

## 3. FINDING 1 — No incremental loading/analysis graph *(the re-eval pessimism)* — **CRITICAL / rewrite**

This is the headline performance defect and it is confirmed at the literal mechanism level.

### 3.1 What razel does

- **Every analysis runs on a fresh, throwaway `Session`.** `analyze_workspace_resolved`
  (`crates/razel-loading/src/rules/pkg.rs:211`) constructs `Session::new(...)` (`pkg.rs:220`) on every
  call. *Every* loading cache — `bzl_cache`, `ast_cache`, `glob_cache`, `fold_cache`, the analyzed-target
  `results` map — is a field on that `Session` (`crates/razel-loading/src/state/session.rs:38-148`), so it
  is **born and discarded per analysis pass**. The session doc-comment is explicit: "Built fresh per
  `analyze_*` call (so there is no reset to forget)." There is **no on-disk persisted loading/analysis
  cache** (no `serde`/`Serialize` on `Session`/`LoadedTarget`) — nothing survives a process restart.
- **The CLI cold path re-analyzes from scratch every invocation** (`crates/razel-build/src/drive.rs:78`
  → `analyze_workspace_resolved` → fresh `Session`).
- **The daemon is warm only for the *action* engine, not for loading.** `WorkspaceActor`
  (`crates/razel-daemon/src/actor.rs:179-197`) holds a warm `IncrementalBuilder` and a single post-analysis
  `Analysis { digest, token, targets, build_name }` (`actor.rs:168-174`) — **it does not hold a `Session`**.
  On any reuse miss it calls `analyze_workspace_resolved` again (`actor.rs:482`), i.e. a fully **cold**
  Starlark re-eval.
- **Reuse is gated by a single digest over the entire source tree.** `compute_analysis_digest`
  (`actor.rs:553-582`) folds `MODULE.bazel`/`.bazelrc` plus the **full content of every `BUILD`/`BUILD.bazel`
  in the workspace** (`collect_build_files`, `actor.rs:639-664`) into **one** `Digest`. `ensure_analysis`
  (`actor.rs:458-484`) reuses the cached analysis only if `a.digest == digest && a.token == token`.
  Therefore:
  - **Any BUILD edit anywhere** — even in a package outside the built target's closure — flips the
    global digest and forces a full cold re-analysis.
  - **Requesting a different target** (different `token`) discards the analysis and re-evaluates.
  - The digest is recomputed by **reading every BUILD file in the tree on every build**, including
    no-op rebuilds — O(repo) file I/O per invocation before anything else happens.
- **Any non-leaf change wholesale-invalidates.** `apply_pending_changes` (`actor.rs:526-545`) re-digests a
  known build-graph *leaf* in place (`sync_file`), but any new/deleted source, glob change, or BUILD/MODULE
  edit sets `self.analysis = None` — all-or-nothing.
- **Secondary amplifier: the ~10 native ruleset modules are rebuilt per package.** `eval_build_src_in`
  calls `ruleset_modules()` + `build_globals()` (`crates/razel-loading/src/rules/eval.rs:27-28`) **once per
  package**, and neither is memoized — each call re-`AstModule::parse`s + evaluates + freezes
  `RUST_RULES_BZL`, `cc_defs.bzl`, `java_defs.bzl`, skylib/python/shell/js (`rules/load.rs:292-339`,
  `shims.rs`, `rust_rules.rs:646`). The session caches `.bzl` loads and ASTs but has **no ruleset/globals
  cache** — at TF's package count that is thousands of redundant parse/eval/freeze cycles.

> **Honest scoping (verifier corrections):** Within one invocation the loader *is* demand-scoped — it
> loads only the requested target's transitive closure on demand (`pkg.rs:224` + the alias/dep walk), not
> the whole repo, and `bzl_cache` correctly gives one eval per `.bzl` *per Session*. The defect is that the
> **reuse/invalidation key is whole-tree** and **nothing persists across builds**, so the warm daemon
> behaves essentially cold for the loading workload. Also: a true no-op rebuild *does* reuse — it is not
> literally re-evaluated on *every* invocation, but it *is* re-evaluated on every BUILD edit and every
> target switch, plus the O(repo) digest scan every time.

### 3.2 What Bazel does

Bazel models loading/analysis as a **persistent, fine-grained demand graph** in the server's resident
`MemoizingEvaluator`:

- Each `.bzl` is compiled once, keyed for change-pruning by file digest (`BzlCompileFunction.java`), and
  loaded once as a memoized `BzlLoadValue` whose **direct + transitive deps are recorded** so it can be
  shared across loaders and reused across builds (`CachedBzlLoadData.java:37-52` — *read first-hand*; the
  doc: "split into other CachedBzlLoadData objects so they can be shared by other requesting bzls",
  interned to exactly one value per key). Packages are `PackageValue` nodes (`PackageFunction.java`);
  globs are `GlobValue` nodes (`GlobFunction.java`).
- On a file change, `InvalidatingNodeVisitor` marks only the changed node dirty and walks **only its
  transitive reverse-deps** (`src/main/java/com/google/devtools/build/skyframe/InvalidatingNodeVisitor.java:544,563-564`);
  **early cutoff** (`IncrementalInMemoryNodeEntry.java:169-189`: an unchanged recomputed value keeps its
  old version and stops propagating) means an edit to one BUILD re-evaluates **that package plus its
  rdeps**, not the tree.

### 3.3 Why this is a rewrite, not a patch

razel already owns the right *primitive* — `razel-engine` is a genuine Skyframe-lite with two-version
nodes and early cutoff (§7.1) — but **nothing populates it with loading nodes**: the entire Starlark
load+analysis runs *above* the engine and returns a `Vec<AnalyzedTarget>` that is only then lowered into
action nodes (`crates/razel-build/src/incremental.rs:144-241`). Closing the gap means **modeling `.bzl` /
package / glob / target as demand nodes keyed individually** (a Skyframe-equivalent loading store with
rdep invalidation, early cutoff, and cross-build persistence in the daemon). That is a structural change
to how loading is keyed and stored — Fundamentals **F5 (incrementality)** and **F6 (early cutoff)** are
unmet on the live path, and AD7 ("one demand-driven engine") is admittedly Partial. Bolting per-node
invalidation onto a coarse whole-tree digest is not achievable as a band-aid on `compute_analysis_digest`.

The uncached-ruleset-modules amplifier (§3.1, last bullet) is, by contrast, **fixable without a rewrite**
— memoize the rulesets/globals once per Session like `bzl_cache` (key on `session.global.cc_toolchain`,
constant within a session). Worth doing early because it inflates every cold load and masks the real work.

> **Latent correctness bug adjacent to the perf gap:** `compute_analysis_digest` hashes only BUILD/MODULE
> *contents*, never the source-tree listing. A **new source file that a `glob()` would pick up does not
> change the digest**, so the warm daemon will not re-analyze for it on the watcher-off path — `actor.rs:552`
> self-admits this residual glob gap. Bazel's globs depend on `DirectoryListingValue`/`FileValue` nodes, so a
> source add re-globs exactly one package. This is a correctness gap, not just speed.

---

## 4. FINDING 2 — No toolchain / platform resolution *(the toolchain gap)* — **CRITICAL / rewrite**

The user's observation that razel "has not used Bazel's toolchain mechanisms" is exactly right, and the
reason it keeps being deferred is structural: **there is no resolution mechanism to adopt — the three
pillars of Bazel's model are all dead no-ops or host constants.**

### 4.1 What razel does

- **Registration is dropped.** `register_toolchains` / `register_execution_platforms`
  (`crates/razel-loading/src/fetch.rs:154-164`) are literal no-ops returning `NoneType`. `toolchain(...)` /
  `toolchain_type(...)` (`dialect.rs:432-461`) only `record_target` an empty target — no constraints
  captured. The registry Bazel's resolver iterates is never populated.
- **Resolution does not exist.** `ctx.toolchains[type]` (`ctxv.rs:118-133`) linear-scans a **single global
  table** built once by `toolchain_rows()` (`toolchains.rs:29-179`) — the *same* rows for every target,
  regardless of the rule's declared `toolchains=`, exec platform, or target platform. The rows are seeded
  from the **physical host** (`host_tools(sess)`, `host_triple()`, `std::env::consts::ARCH`, literal
  `"clang"`, `"macosx"`) and are largely **`Absorb` stand-ins** ("any field/method resolves"). The header
  comment admits it: *"Real `rule(toolchains=)` resolution is L3"* and the field list "grows probe-step by
  probe-step." `config_common.toolchain_type` is also a no-op (`engine.rs:287-292`). *(Read first-hand.)*
- **Tool binding is hard-wired to host.** `resolve_toolchain` (`toolchains.rs:16-24`) is
  `match name { "cc" => razel_cc_toolchain::macos_core_config(), other => Err }` — a **string switch with
  no platform/constraint argument in scope**. `macos_core_config()` parses a single compiled-in fixture
  (`crates/razel-cc-toolchain/src/lib.rs:226`, `fixtures/cc_macos_core.bzl`) carrying
  `-mmacosx-version-min`, `/usr/bin/libtool`, etc. — a one-time capture of *this* machine. The rust rules
  call host `rustc()` directly with `--target=host_triple()` (`rust_rules.rs:90-105`) and **consult zero
  toolchain rows** (the rust row in `toolchains.rs` is read only by the `.bzl` host-stub shims).

### 4.2 What Bazel does

`SingleToolchainResolutionFunction.resolveConstraints`
(`src/main/java/com/google/devtools/build/lib/skyframe/toolchains/SingleToolchainResolutionFunction.java:148-234`)
takes the requested toolchain **type**, the **target platform**, and the available **execution platforms**;
filters registered `DeclaredToolchainInfo`s to that type; matches `targetConstraints` against the target
platform and `execConstraints` against each exec platform; and emits a `(execPlatform → resolvedToolchainLabel)`
map. `DeclaredToolchainInfo.java:43-50` is exactly `{toolchainType, execConstraints, targetConstraints,
targetSettings, resolvedToolchainLabel}` — the record `register_toolchains` feeds. **The resolved tool is a
function of (type × targetPlatform × execPlatform).** For cc, the *command line* is then a declarative
feature-config expansion from the **resolved** `cc_toolchain`'s `CcToolchainConfigInfo`
(`CcCommon.java:167-256`, `CcToolchainFeatures`), not a hand-rolled argv.

### 4.3 Why this is a rewrite, and why band-aids keep winning

In Bazel the selection *key* (platform/constraints) flows into tool selection; in razel that key is **absent
from the entire tool-selection path**. There is therefore **no seam** to add a second toolchain or a
cross-compile target without building the resolution function and constraint model that don't exist. The
*only* extension seams present are (a) widening the host stand-in rows and (b) adding a fixture string to
`resolve_toolchain` — **neither introduces the (type × platform) key**, so every immediate failure is
cheapest to fix by adding one more host row or one more `Absorb` field. That is the band-aid treadmill: it
makes a specific target pass while entrenching the host-constant shape. A genuine fix requires (1) real
registration (collect `DeclaredToolchainInfo` from `toolchain()`/`register_toolchains`), (2) a
constraint-keyed resolution pass, and (3) routing `ctx.toolchains[t]`, the cc config lookup, **and** the
rust tool/flag emission through the *resolved* toolchain. **Its prerequisite is Finding 3** — the rules must
stop being native host-coded before there is anything for a resolver to feed. This is the AD8
(Target×Config / cross-compile) work, currently Designed-only.

> **Parity-gate blind spot worth flagging:** the rust path filters `--sysroot` / bare `-L` (the toolchain
> std/sysroot search) / `--remap-path-prefix` out of **both sides** of the parity diff
> (`crates/razel-parity/src/lib.rs:239-262`; `rust_rules.rs:84-87`). So the gate is structurally **blind to
> exactly the toolchain-dependent portion** of the command line — the divergence is invisible to the test,
> not absent from the build. Host==target hides it today; any cross-compile or non-default sysroot would
> mis-invoke `rustc`. (Dependency `-Ldependency=` *is* emitted; only the toolchain `-L`/sysroot are filtered.)

---

## 5. FINDING 3 — Native rule reimplementation + `Absorb` *(the meta-shortcut everything descends from)* — **CRITICAL / rewrite**

### 5.1 The shortcut

The designed path (AD4 "rule packs = declarations", AD5 "Bazel is an adapter, run real `.bzl`") is parked;
the live loader **reimplements each ruleset natively**. `ruleset_modules()` (`rules/load.rs:292-339`) maps
`@rules_cc`/`@rules_java`/`@rules_rust`/`@rules_python`/`@rules_shell`/`@aspect_rules_js` to synthetic native
modules; when a `load()` path matches a ruleset prefix the loader returns the native module and **never
executes the upstream `.bzl`** (`load.rs:232-233`). `native_cc_library` (`native_cc.rs:18-42`) takes a fixed
attr list (srcs/hdrs/deps/copts/defines/includes + a discarded kwargs sink) and emits a fixed action shape.
The whole native rule surface is ~1,400 LOC across five files — *N hand-maintained rulesets chasing
upstream*. razel's own `RazelDevStatus.md:62-63` admits AD4=D/parked ("live rules are native Rust
reimplementations") and AD5=P ("standard rulesets are native reimplementations + host-repo stubs, not the
fetched `.bzl`").

> Nuance: the loader *does* prefer a real vendored `.bzl` over the shim when one is materialized
> (`load.rs:208-217`), so the adapter seam is *understood* — but the live system does not fetch the standard
> rulesets, so the prefix-shim path is what runs. The seam exists in concept and is parked.

### 5.2 `Absorb` — the scar tissue, and the silent-failure mode behind symptom 3

Where a native rule doesn't cover a ruleset API (`cc_common`, `apple_common`, `java_common`, `ctx.fragments`,
runfiles), the loader substitutes **`Absorb`** (`engine.rs:301-379`): a Starlark value whose
`get_attr`/`invoke`/`at`/`add`/`slice` all return another `Absorb`, with `to_bool=false`, `length=0`,
empty iteration, and `equals(None)=true`. Packages "load" while the real rule computation is **silently
dropped**.

razel's own defense (the `engine.rs:296` comment) claims absorption "surfaces at analysis time … fails with
a clear `<host-absorbed>` in the traceback." **The adversarial check refutes that for the dominant path:**
razel's action factory is *untyped*. An `Absorb` used as a file path falls through to its Display string
`"<host-absorbed>"` (`values.rs:636-643`); as an input/output list it flattens to one bogus
`"<host-absorbed>"` path or, as a depset member, **vanishes entirely** (empty iteration) — and no guard
anywhere in `razel-build`/`razel-engine`/`razel-exec` rejects that path (grep: zero hits). So the real
behavior is **"trade a loud failure for a silent wrong/empty result"**: actions get missing inputs (→
compile fails) or wrong inputs (→ compiles with the wrong thing). This is the proximate mechanism behind
**symptom 3** *and* its dangerous inverse. Bazel has no value that absorbs unimplemented API — a missing
`cc_common`/`java_common` call is a hard `StarlarkEvalException`, never a silent empty
(`rules/cpp/CcModule.java` is the full typed API over real `Artifact` types).

### 5.3 Why it cannot converge

A native reimpl can only reproduce the subset of each ruleset's behavior its authors hand-coded. The moment
a real BUILD relies on logic outside that subset — a feature flag, a transition, a provider field, a macro
path — razel diverges, and the divergence is *silent* (§5.2). This is rewrite-forcing by **shape**, not by
bug: *N* hand-maintained native rulesets chasing upstream cannot converge on Bazel parity and cannot scale to
the open ruleset ecosystem (Fundamentals **F15/F16**, engine-closed extensibility, are unmet). The only shape
that closes it is the designed one — run the real `.bzl` through an effect-capturing `ctx` that records facts.
**Findings 1 and 2 are both blocked on this:** there is no point building toolchain resolution for rules that
bypass it, and the loading graph should cache the *real* evaluated `.bzl`, not the shims.

---

## 6. FINDING 4 — Feedback covers only the execute phase — **HIGH / missing-feature**

`build_label` (`actor.rs:390`) calls `ensure_analysis` **first** — running the cold Starlark load + analysis
synchronously through `analyze_workspace_resolved`, **which takes no progress channel** — and only *after* it
returns installs the per-action progress sink and sends the first frame (`T\x1f{action_count}`, `actor.rs:398`).
The sink itself (`incremental.rs:60-65`) is a per-*action* callback. The daemon frame loop (`rpc.rs:286-356`)
only knows two phases: `execute` and `log`. **There is no loading/analysis phase event anywhere**, so during
the entire TF load+analysis window — the part that dominates wall time — the socket is silent and the CLI
shows a static "Building …" line.

Bazel feeds one `UiEventHandler` from an `EventBus` that **loading, analysis, and execution all post to**:
`loadingStarted` subscribes and starts a live update thread (`UiEventHandler.java:601-609`), and
`PackageProgressReceiver` drives a live "Loading: N packages loaded" counter (`UiStateTracker.java:428-430`)
*before* any action runs.

**Fix is feature-work, not a rewrite:** the streaming plumbing (channel, frame loop, phase-tag field) already
exists; thread a progress channel into `analyze_workspace_resolved` and emit loading/analysis phase frames
before the action total. Tied to **symptom 2** only — silence during a slow phase is not the *cause* of the
slowness (that is Finding 1).

A related-but-secondary gap: razel has **no spawn-strategy abstraction** — `run_action` (`incremental.rs:351-368`)
always builds one local `Action` and calls one `Sandbox::run` (`Command::new` + `.output()`); there is no
`SpawnRunner`/`SpawnStrategy` contract (Bazel: `SpawnStrategyResolver.java:36-70` over many runners) and **no
persistent-worker protocol** (Bazel runs `JavaBuilder`/`scalac`/proto compilers as warm JVM workers via
`worker/SingleplexWorker.java` + `WorkRequestHandler`). For a JVM/Java-heavy tree the missing worker protocol
is a large constant-factor execution cost. Both are **missing-feature/medium** and sanctioned by current scope
(AD8 designed-only; RBE/workers are named gaps), not rewrite-forcing — but they are why the feedback and
execution models are thin.

---

## 7. What is actually fine *(calibration — do not rewrite these)*

The adversarial pass refuted several plausible gaps. Keeping them honest:

- **The action-execution engine is a correct Skyframe-lite.** `razel-engine` carries two-version nodes
  (`verified_at`/`changed_at`, `lib.rs:80-81`), backdates on validation, advances `changed_at` only on real
  value change, and has **genuine early cutoff** — with tests asserting the firewall and
  `incremental == from-scratch` (`lib.rs:575-615`). This matches Bazel's `IncrementalInMemoryNodeEntry`
  bit-for-bit. AD7 is genuinely realized **for actions**. The gap (Finding 1) is only that this model is not
  extended *up* into loading — which is *additive*, not a teardown.
- **The taut/CBOR ⇄ protobuf substitution is clean.** A full-tree grep shows `razel_wire` appears only at the
  RPC/transport boundary (`razel-daemon`, CLI); the loader (`razel-loading`) is codec-free, and `AnalyzedTarget`
  / the DDS fact store are plain in-memory types. Encoding is deterministic (sorted map keys, `cbor.rs:126`)
  and identity is `blake3`, independent of the codec. This is exactly where Bazel puts protobuf
  (`ConfiguredTargetValue` is a plain SkyValue; proto lives in BEP/remote/serialization). **No symptom
  originates here.**
- **Per-action output buffering is parity-correct.** razel buffers a spawn's stdout/stderr via `.output()`
  and surfaces it after the action (`sandbox.rs:226-258`) — but **Bazel does the same**: local build actions
  redirect to `FileOutErr` *files* and are surfaced post-hoc as "INFO: From …"
  (`LocalSpawnRunner.java:371-372,409-412`). Live streaming of build-action stdout is not something Bazel does
  either. Not a gap.
- **The local action cache is fine for local scope.** A flat content-addressed dir keyed by action content
  key gives the zero-exec local rebuild (`razel-exec/src/lib.rs:51-90,215-235`). The missing AC/CAS split +
  remote seam only matters once RBE is in scope — same gap as the spawn-strategy finding, not a separate defect.
- **"No cross-process persistence" is *not* a Bazel deviation.** Bazel also loses its Skyframe graph on server
  restart, and `bazel --batch` re-evaluates cold exactly like razel's cold path. The real gap is **within-server
  warm-loading granularity** (Finding 1), not cross-process persistence.
- **Compilation-mode flags *do* flow on the live cc path.** The claim that "`-c opt` gets the same argv as
  fastbuild" is **false** for the live native cc path: `args.rs:54-62` maps opt→`-O2 -DNDEBUG`,
  dbg→`-O0 -g`, and `native_cc.rs` threads global+target `copts`/`defines`. The real residual is that **no
  PIC / sanitizer / coverage / LTO feature selection** exists on *either* cc path — a breadth/missing-feature
  consequence of AD4/AD5 parking, not a "wrong argv" bug.

One doc-accuracy note: `RazelDevStatus.md:80-82` says `razel-dds::DdsRead::fold_depset` is "unused outside its
own tests" — **stale**: the DDS fold *is* the live provider fold (`crates/razel-loading/src/dds.rs` ←
`deps.rs:186-187`). The *rest* of the typed spine (`razel-rulepack`'s rule-pack engine, AD3 producer-purity
wall, AD9 merge-class engine) is genuinely stranded with zero live callers — deferrable debt, not
rewrite-forcing. Its only real harm is creating a false impression that the designed substrate exists on the
live path; it does not — **`razel-loading` is the de-facto architecture.**

---

## 8. FINDING 5 — Configuration is a single-instance host==target singleton — **HIGH / missing-feature (bounded, deferrable)**

This is the one shortcut that is **not poisoned** — it is a clean, documented deferral (AD8). razel has no
`BuildConfiguration` object; configuration is one `GlobalFlags` set once on the `Session` (`session.rs:57-58`),
analyzed targets are keyed by **Label only** (`state/types.rs:24-36`; results overwrite by label), and
`select()` resolves against that one config and the **physical host** (`flags.rs:167-208`, `host.rs:336-345`).
Transitions / exec-target split are no-op stubs (`dialect.rs:247-251,290-296`). Bazel keys configured targets
by `(Label × BuildConfigurationKey)` (`ConfiguredTargetKey.java:79-85,147`) and resolves `select()` against
the bound `BuildConfigurationValue` (`ConfiguredAttributeMapper.java:85-92`).

For the host==target product this gives **correct results** and `select()` on `compilation_mode`/`define` is
genuinely implemented and tested. The debt is that the config dimension is **absent from the key**, woven
through three caching layers (`bzl_cache`, package cache, `results`) — so adding a second config later is a
mechanical-but-pervasive key change, *and the Target×Config axis already exists in the parked spine*
(`razel-dds` has `TargetKey { instance, label }`, with every live caller hardwiring `InstanceId::SINGLE`).

Two in-scope cautions:
- The genuine current risk is **not** "host instead of target" (that only bites cross-compile, which razel
  doesn't attempt) but the **silent conservative non-match**: unmodeled `flag_values` and forced-false
  `local_config_cuda/rocm/sycl/tensorrt` resolve to `false` with no error (`flags.rs:151-159`, `host.rs:320-328`).
  TF/XLA `select()` heavily on these — a wrong arm or a "no matching condition" can contribute to **symptom 3**.
  This is a modeling/coverage gap, fixable independently of the config rewrite.
- The exec/target split is **unrepresentable** with a Label-only key (a split transition produces two
  configured targets for one label), so it is genuinely downstream of Finding 1's keying change.

Verdict: **deferrable, not a poisoned shortcut** — but it is load-bearing for any cross-compile story and
shares the keying rewrite with Finding 1.

---

## 9. Additional gaps from the completeness pass *(breadth — confidence flagged)*

These were surfaced but not all deep-verified; listed so they are not lost. Each names the file:line to
examine to close it.

| # | Gap | Status | Examine |
|---|---|---|---|
| A | **No persistent-worker protocol** — bare one-shot subprocess per action; large constant-factor cost on JVM/Java-heavy trees | **Verified absent** | razel `crates/razel-exec/src/lib.rs:175` (`Command::new`) vs bazel `worker/SingleplexWorker.java` + `WorkRequestHandler` |
| B | **Glob → `FileValue`/`DirectoryListing` Skyframe edges** — the *positive* reason a source-add re-globs one package in Bazel; razel's digest doesn't fingerprint source-file additions (correctness, §3.3 note) | **Verified (the razel half)** | razel `state/session.rs:122-145` + `actor.rs:563-575` vs bazel `GlobFunction.java` |
| C | **Bzlmod / `MODULE.bazel` module-graph resolution** — razel reads `MODULE.bazel` only as a digest input and drops `register_*`/`repository_rule`; if the module dep graph isn't resolved, external `@repo//…` labels Bazel resolves will fail to load (candidate **symptom 3**) | **Needs verification** | razel `fetch.rs:154-164` + `actor.rs:555` vs bazel bzlmod resolution |
| D | **Aspect propagation** — `aspect(...)` is registered (`dialect.rs:185-214`) but it is unverified whether aspects *propagate along attribute edges* or are merely recorded; if they don't propagate, aspect-provider-based rulesets (TF gen/proto layers) silently under-compute (candidate **symptom 3**) | **Needs verification** | razel `dialect.rs:185-214` vs bazel `skyframe/LoadAspectsFunction.java` |
| E | **`--keep_going` / partial-failure recovery** — if the monolithic Session analysis aborts wholesale on one bad package, that is both a feedback gap and a symptom-3 amplifier (one unsupported construct kills the whole load) | **Needs verification** | razel `state/session.rs` analysis-error path vs bazel keep-going semantics |

---

## 10. Verdict & recommended path

### 10.1 Rewrite vs band-aid, by subsystem

| Subsystem | Verdict | Why |
|---|---|---|
| Native rule reimplementation + `Absorb` (Finding 3) | **Rewrite** | Wrong shape; cannot converge on parity or scale (F15/F16). The root the others hang off. |
| Loading/analysis incrementality (Finding 1) | **Rewrite** | No demand-node graph; whole-tree digest. F5/F6 unmet. The headline slowness. |
| Toolchain/platform resolution (Finding 2) | **Rewrite** | The (type × platform) selection key is absent; only host-row/fixture seams exist. F13 unmet. |
| Per-package uncached ruleset modules (§3.1) | **Fixable** | Pure memoization on the Session. Do early. |
| Loading/analysis progress events (Finding 4) | **Fixable** | Plumbing exists; thread a channel + phase frames. |
| Config / Target×Config (Finding 5) | **Deferrable** | Correct for host==target; mechanical key change later (spine already config-keyed). |
| Spawn strategy / workers / remote (Findings 4, 9A) | **Additive** | Sanctioned scope; drop a `SpawnRunner` trait in front of the existing executor. |
| Action engine, wire seam, within-session cache, action cache (§7) | **Keep** | Correct and parity-shaped. |

### 10.2 The brick wall, named

The recurring-bug pattern is real and it is **architectural debt cycling as bug-fixes**: each compile
failure is cheapest to fix by adding one more host toolchain row, one more `Absorb` field, or one more
fixture string; each "stale build" by coarsening the digest. Every such fix **moves further from Bazel's
shape**, not toward it, because the seams that would let you converge (real `.bzl` evaluation, constraint
resolution, per-node invalidation) **do not exist to extend** — so the path of least resistance always
entrenches the shortcut. That is why parity stalls and why agents "resist" the toolchain model: there is
nothing to adopt *until* the rules stop being native.

### 10.3 Ordered path back (foundational-first)

1. **De-nativize the rules onto an effect-capturing `ctx` adapter that runs the real fetched `.bzl`**
   (realize AD5). This is the keystone — it deletes the `Absorb`/native-reimpl class of silent failures
   (symptom 3) and creates the resolved-toolchain consumer that Finding 2 needs. Start with one ruleset
   (cc) end-to-end against a vendored `rules_cc`, golden-gated.
2. **Model loading/analysis as persistent demand nodes** keyed per `.bzl` / package / glob / target,
   reusing the `razel-engine` primitive, with rdep invalidation + early cutoff and warm-daemon persistence
   (realize AD7 for loading). Closes symptom 1. Fix the glob/source-add digest correctness gap (§9B) here.
3. **Real toolchain resolution over constraints** (realize the AD8 resolution): collect
   `DeclaredToolchainInfo` from `register_toolchains`/`toolchain()`, resolve by (type × target × exec
   platform), and route `ctx.toolchains` + cc config + rust tool/flag emission through the resolved context.
   Closes symptom 4; un-blinds the parity gate (§4.3).
4. **Thread config into the analysis key** (Target×Config) — mechanical once steps 1–2 land; the spine is
   already config-keyed.
5. **Loading/analysis progress events** (symptom 2) — independent, can land any time.

Steps 1–3 are the genuine rewrites; they are also the only ones that make the symptom list go away rather
than relocate. Steps 4–5 and the §3.1 memoization are incremental and parallel-friendly.

---

### Appendix — primary evidence anchors

*razel* — `crates/razel-daemon/src/actor.rs:179-197,458-484,526-545,553-582,639-664` (warm action graph,
whole-tree analysis digest); `crates/razel-loading/src/rules/pkg.rs:211-220` (fresh Session per analysis);
`.../state/session.rs:38-148` (all caches per-Session); `.../rules/load.rs:204-285,292-339` (label-keyed
bzl_cache, native ruleset map); `.../rules/eval.rs:27-28` (per-package ruleset rebuild);
`.../toolchains.rs:16-24,29-179` (string-switch resolution, host stand-in rows); `.../engine.rs:301-379`
(`Absorb`); `.../native_cc.rs:18-42` + `.../rust_rules.rs:84-105` (native rules, host-pinned);
`.../fetch.rs:154-164` (dead `register_*`); `crates/razel-cc-toolchain/src/lib.rs:226` (baked fixture);
`crates/razel-engine/src/lib.rs:80-81,575-615` (sound action engine); `crates/razel-parity/src/lib.rs:239-262`
(parity diff filters toolchain flags).

*bazel* — `skyframe/CachedBzlLoadData.java:37-52`, `BzlCompileFunction.java`, `PackageFunction.java`,
`GlobFunction.java`, `SequencedSkyframeExecutor.java` + `skyframe/InvalidatingNodeVisitor.java:544,563-564` +
`IncrementalInMemoryNodeEntry.java:169-189` (persistent fine-grained loading graph, early cutoff);
`skyframe/toolchains/SingleToolchainResolutionFunction.java:148-234` + `analysis/platform/DeclaredToolchainInfo.java:43-50`
(constraint-based toolchain resolution); `rules/cpp/CcCommon.java:167-256` + `CcToolchainProvider.java`
(feature-config command line); `skyframe/BzlLoadFunction.java:1396-1457` (executes real `.bzl`);
`exec/SpawnStrategyResolver.java:36-70` + `worker/SingleplexWorker.java` (spawn strategies, workers);
`runtime/UiEventHandler.java:601-609` + `UiStateTracker.java:428-430` (loading-phase progress).

*Method note:* findings produced by 7 parallel subsystem deep-reads, each gap adversarially re-verified
against source on both sides (verifiers tasked to refute), plus a completeness critic. The two headline
findings (§3, §4) and the action-engine/wire calibration (§7) were additionally confirmed first-hand by the
reviewer against the cited files.
