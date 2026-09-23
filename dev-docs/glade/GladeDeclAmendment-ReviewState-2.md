# GladeDeclAmendment — STATE-AXIS RE-VERDICT (round 2)

**Review object:** the glade-decl contract v1 amendment (plan Steps 2.1–2.8) as revised by round 1's single remediation. The remediation is planned in `dev-docs/glade/GladeDeclAmendment-RemPlan.md` (root `8056056`), and the owner confirmed its dispositions ("all recommended") at `0e4d882`. This review is round 2 of 2, re-verdicting my round-1 report (filed at root `87ce46f` as `GladeDeclAmendment-ReviewState.md`). Nothing is published yet.

**Baseline:**
- Read at the start: root `73c2bf7e9a72`; glade `1b9ac3fd5bd6`; glade-decl `3d109177a222`; glade-decl-rs `d3be799db4f3`, glade-decl-ts `85ec18da1cb7`, glade-decl-py `ee2f9602b2c1`; grazel `e1a4078d211f`; glade-gyld `c9ef7a66b6e4`; glade-gwz `fc0bb990ef09`; glial `4c6e8561c174`; grip-core `97ff6c26f12e`; gryth-wz `3c64e7a59064`, gryth-wz/glade-decl-ts `37d9b7d8a680`, gryth-wz/glial `4c6e8561c174`. Every in-scope tree was clean.
- Read at the end: every member SHA was unchanged, and all 19 glade-wz lock entries still equal their member HEADs.
- The root moved during the review, to `690f2f6807f9` ("Lock glade-discover at its braced conformance modules"). `git diff --name-only 73c2bf7 690f2f6` lists three bookkeeping files: `gwz.conf/gwz.lock.yml` (only glade-discover changes, `48bc104` → `9caac2e`, which is out of scope), `gwz.conf/markers/conf-integrity.yml`, and the per-commit marker `gwz.conf/markers/01a0ce91-….yaml`. That marker records every in-scope member at its tuple SHA. This matches the coordinator's corrected note. The in-scope tuple held.
- At the end the root also carries one untracked report output, `dev-docs/glade/GladeDeclAmendment-ReviewSurface-2.md`. It belongs to another axis and I did not open it.
- glade-discover shows 5 uncommitted paths; it is out of scope.
- Authority: `git -C gwz-dev log ff431743cc4c..HEAD -- dev-docs/AgentProcessRules.md` returns 0 commits.
- Sources were read from the working trees and through `git show 1b9ac3f` and the ranges the coordinator named. Out of scope and not reviewed: glade `831eded` and `97d6afc`, and glade-discover.

**Date:** 2026-09-24

**Axis:** State — durable-state semantics and adversity: whether each round-1 finding is closed on the revised tuple, and attacks on the remediation's new durable paths. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: GO** — round-1 findings: 5 of 5 CLOSED. New: 0 P0, 0 P1, 0 P2, 1 P3 (STA-P3-4, not architectural, non-blocking under L1-19). This honours my round-1 pre-commit: STA-P2-1 is resolved by one of the two variants I specified (the refusal), and STA-P2-2 by exactly my variant.

---

## 0. Evidence base

**Gates run**
1. `CARGO_TARGET_DIR=<scratch>/review-target cargo test -p glade-node --offline` in `glade/node`: 121 tests pass, 0 fail (lib 112; `binding_census` 5; `one_file_per_app` 3; `shipped_app_files` 1). The one binary test runs the node with `GLADE_HOME` and `HOME` pointing into a temp dir.
2. `PYTHONDONTWRITEBYTECODE=1 /opt/homebrew/bin/python3 corpus/build.py --check` in glade-decl: exit 0, with all 7 artifacts in lockstep. The mirror is byte-identical and `CONTRACT_VERSION` is `7d18cd3` in all three renderings.

**Probes** — all under `<scratch>/state-probes/`, built only into the scratch target, with every store in a fresh scratch directory.
- **The round-1 probes, rebuilt against the tuple's crate:** `compat`, `upgrade`, `census`, `killretract`, `sameapp`, `rename`, `multiorigin`. `old-node/` is still glade `559cb2c`, taken with `git show`. In the output, `compat`'s "new (dddf8b8)" label is stale: that arm is the tuple's crate.
- **New probes:**
  - `v0raw`: a v0 file with `=` tokens, registered first by 559cb2c and then by the tuple on the same `BlobStore`.
  - `mergeflapped`: a store flapped by a split app, then loaded through `load_all` from a merged file.
  - `inspect`: reads the `records.json` and the served `cache/store` that a real node run leaves behind, and calls `exchange::declared_exchange`.
- **`binrun/run.py`:** runs the tuple's `glade-node`, rebuilt at `review-target/debug/glade-node`, with `GLADE_HOME` and `HOME` set to fresh directories under `binrun/`.
  - A start that reaches `listening` is killed there; every registration and save happens before that line.
  - A kill leaves `instance.lock` behind, and sysdir.rs says a human clears it. The script removes it before the next start of the same home.
  - Ports: the node was given port `0` and binds loopback only. Nothing touched ports 5173, 8080 or 9099, `~/.glade` or `~/.gyld-ui`.
- **`flipped-node/`:** glade `1b9ac3f` with `V1_TOKEN_CHECKS_REFUSE = true`. 48 appdecl tests pass, including the six closure tests (§1).
- **`tsfrom/`:** `git archive` copies of glade-decl `3d10917` and glade-decl-ts `85ec18d`, with taut reached by symlink and `PYTHONDONTWRITEBYTECODE=1`. The README's two documented commands were run there. The taut, glade-decl and glade-decl-ts repositories stayed clean.

**Metadata**
- `stat -f %l` over the renderings' sources and the gryth-wz duplicate, and `find -inum` for the hard-linked copies.
- The default node binary, `glade/node/target/debug/glade-node` (23:45), contains the revised strings and not the old `crdt` parenthetical. So the binary that grazel and `gyld-ui.py` start is the revised node.
- `glade/node/Cargo.lock`: mtime unchanged (Sep 21).
- The scratch `review-target` was found recreated at 23:59, by something outside this review. I rebuilt the node binary into it at 00:02 for the last binary check; no repository was affected.

**Not run:** the grazel, glade-gyld and glade-gwz suites, and `test_build.py` inside the repository. The rules allow only `build.py --check` there, so the `--ts-from` behaviour was reproduced in the scratch copy instead.

## 1. Closure of round-1 findings

| ID | RemPlan disposition | Status | Evidence on the revised tuple |
|---|---|---|---|
| **STA-P2-1** — two `--app` files naming one app flap every boot | ACCEPT, the refusal variant. `appdecl::load_all` (`appdecl.rs:511`) runs before `boot()` (`glade-node.rs:81` then `:82`) and refuses when two files name one app, naming the app and both paths. The page, notes and docs were updated. | **CLOSED** | See the evidence list after this table. |
| **STA-P2-2** — a v0 file with `=` in its zone or retention is refused at boot | ACCEPT, State's variant. The slot guard applies to `glade-app v1` files only (`tail.rs:60`). In a v0 file the token is stored raw, with a line-numbered warning. | **CLOSED** | See the evidence list after this table. |
| **STA-P3-1** — multi-origin order and scope claimed beyond one registry | The claims are scoped to one registry. The question is recorded open at plan Step 4.6. A two-origin test pins today's outcome. | **CLOSED** | See the evidence list after this table. |
| **STA-P3-2** — a renamed `app` orphans its declarations; the page described neither the rename nor any retirement | The page and notes state the per-`(app, glade_id)` fold, the rename behaviour and the retirement file. Tests cover the rename-then-delete sequence and the retirement. | **CLOSED for binding declarations** | See the evidence list after this table. The new text's reach beyond bindings is **STA-P3-4**. |
| **STA-P3-3** — the documented TS regeneration writes through pnpm hard links | `build.py --ts-from <dir>` copies through `write_artifact`. The README carries that one command and a sentence on why. | **CLOSED** | See the evidence list after this table. |

**STA-P2-1 evidence**
- **Real binary, fresh home:**
  - `--app a1 --app a2` exits 1 with ``…/a2.glade: app `x` is already declared by …/a1.glade (an app is declared by one file)``, and `GLADE_HOME` is empty.
  - The reversed order behaves the same.
  - The same path given twice is also refused (lane-owner disclosure 1).
- **Existing instance:** a refused start leaves the whole instance tree unchanged (names, mtimes, sizes), and `records.json`'s hash is unchanged.
- **Control:** two different apps through the binary over three starts append +3, +0, +0. The live bindings are `x/x.one`, `x/x.two` and `y/y.one`.
- **Tests:** `one_file_per_app.rs` passes 3 of 3.
- **Flapped store:** `mergeflapped` takes the store my round-1 probe flapped (29 records), merges the files, and loads them through `load_all`: +2, then +0. All three surfaces are live and none is retracted.
- **Library path:** `sameapp`'s library loop, which calls `register` per file and bypasses `load_all`, still flaps (+5, then +6 four times). This is expected, because `register` is unchanged. The guard sits at the node's only production caller (`glade-node.rs:81` and `:101`; a grep finds no other caller). `register`'s doc (`appdecl.rs:549-554`) states the precondition. See §4.

**STA-P2-2 evidence**
- **`compat` probe:** `ttl=10m`, `a=b` and `window=100` in a v0 file now parse Ok, are stored raw, and are warned with, for example, ``line 3: `ttl=10m` is a key=value entry where <retention> goes; it belongs in the tail, after all five tokens (`glade-app v1` refuses it)``.
- **Binary:** the v0 file boots, and `inspect` shows `cache retention=ttl=10m` and `slot zone=a=b`. Its v1 twin is refused with the unchanged message, and nothing is written.
- **`v0raw` byte parity:** 559cb2c registers the file, then the tuple registers it on the same store. The two `=` lines append +0, meaning the same bytes as 559cb2c stored. Only the `from-cursor` line appends, which is b2's designed first-boot migration.
- **With the flip on:** `a_v0_file_keeps_a_key_value_zone_or_retention_and_is_warned`, `a_v1_file_is_refused_for_a_key_value_zone_or_retention` and both tail tests pass.
- **Whole parse path:** every six-token v0 binding line that 559cb2c accepted is accepted at `1b9ac3f`.
  - The known shapes and the bindable shapes only grew; `crdt` was already refused at 559cb2c.
  - A v0 file runs no slot guard, and `fits_shape` with no tail refuses only `crdt`.
  - The zone and retention checks only warn in a v0 file.
  - The only new refusal of a v0 configuration is start-level (two files naming one app), which is the ruled STA-P2-1 variant.

**STA-P3-1 evidence**
- **Scoped claims:**
  - `registry.rs:472-479` (Stamp: "across them it is an order and not a newest");
  - `registry.rs:496-506` (BindingFold: the rules hold "within one registry");
  - `sysdata.taut.py:96-106`;
  - the attach notes `:128-141`;
  - plan Step 4.6 (`GladeFirstSlicePlan.md:826`, "Open before two nodes load one app").
- **Test:** `across_two_origins_one_nodes_retraction_outranks_the_others_later_declaration` (`registry.rs:804`) asserts lamport 8 against 2. Folded together, `g` is down; in B's own registry, `g` is live. My `multiorigin` probe, which folds through a real served `Store`, gives the same result, and B's reboot appends +0.
- **Claims left unscoped still hold:** `exchange.rs:61-64` describes the fold's output, and `next_binding_lamport` speaks about "here", meaning one registry.

**STA-P3-2 evidence**
- **`rename` probe:** unchanged outcome. The renamed app keeps the old name's `n.old`, and a later delete brings back `notes/n.list value`. This is now the outcome the page documents (`AppFileFormat.md:297-319`).
- **Tests:** `renaming_the_app_line_leaves_the_old_names_declarations_live` and `a_file_naming_an_app_with_no_binding_lines_retires_it` pass.
- **Binary:** the retirement retracts the binding (+1), and loading it again appends +0 (per the tests).

**STA-P3-3 evidence**
- **Scratch run:**
  - Every `src/` file was hard-linked into a "consumer" directory, and the README's `tautc gen` then `build.py --ts-from` were run.
  - The consumer's bytes and inodes are unchanged.
  - Each generated file now holds the new bytes with link count 1.
  - The hand-written `corpus.test.ts` and `index.ts` were not written.
- **Repository:**
  - `glade-decl-ts/README.md` no longer contains a `cp` onto `src/`.
  - The Python and Rust READMEs keep `cp` steps, but no source file in either rendering has more than one link.
  - `glade-decl-ts/README.md` itself now has link count 1 (disclosure 5).
- **Residual, not a finding:** gryth-wz's duplicate checkout at `37d9b7d` (disclosure 6) still carries the old `cp` lines.
  - Its runtime files share inodes with gryth-wz's grip-core and grip-react installs (3 links).
  - The README's relative targets, run as documented from glade-decl, resolve to glade-wz's checkout, not this one.
  - It picks up the fix at the owner's fast-forward.

## 2. Findings (new)

### [STA-P3-4] The format page says retiring an app withdraws "every surface" it declared; it withdraws only binding declarations

- **Location:**
  - `glade/docs/AppFileFormat.md:320-324`, the new "**Retiring an app.**" bullet: "To withdraw every surface an app declared … each of the app's declarations is retracted … Retiring the old name this way is how to rename an app without leaving its declarations live."
  - The page's own vocabulary makes a service's exchange a surface: `:17-18` ("**surface**: one typed thing shared on a share, named by its glade id") and `:160-161` ("`service <name> <exchange-glade-id>` declares an exchange: a directed request/response surface").
  - The same page says the opposite at `:333-346` ("Other lines": no retraction for `service` or `workspace`, and a seed's grant stays).
  - The same wording appears in the test doc at `appdecl.rs:1461-1463` ("It retracts every declaration of that app"), although that test asserts only binding retractions.
  - Stated correctly elsewhere: `register`'s doc (`appdecl.rs:553`, "retracts every binding that app has live") and the attach notes (`:126`).
- **Invariant violated:**
  - The page misstates the durable state that a documented recovery procedure leaves, and contradicts itself.
  - R9's scope as ruled: `binding` only, with option (s) not taken.
- **Reproduction** (`binrun/run.py`, case `retire`; tuple binary; temp `GLADE_HOME`):
  1. Start 1, `--app notes.glade`: `app notes` / `binding n.list value share commons latest` / `service notes notes.ops` / `workspace ws-notes notes`. Result: +3.
  2. Start 2, `--app retire-notes.glade`: `glade-app v1` / `app notes`. Result: +1.
  3. `inspect`: live bindings `[]`, retracted `[(notes, n.list)]`, `dir.services` still holds `notes:notes:notes.ops`, and the served store's `declared_exchange("notes.ops")` is **true**.
- **Impact:**
  - An operator who retires a decommissioned app by the page's procedure leaves its exchange declared and routable. A stale provider can still attach and answer; with no provider attached, callers get `no authority provider attached` failure data instead of the undeclared echo path.
  - The app's `WorkspaceEntry` and seed grants also stay.
  - An operator who renames an app "without leaving its declarations live" by the same procedure leaves the old name's `ServiceDefinition` behind.
  - The impact is bounded: grants are not enforced yet, and an exchange needs an attached provider.
- **Required correction:**
  - Scope the bullet to bindings: "To withdraw every `binding` surface an app declared … each of the app's binding declarations is retracted."
  - Say in the same bullet that the app's `service` exchange stays declared and routable, and that its workspace entry and seed grants stay (point to "Other lines").
  - Word the rename sentence the same way, and correct the test doc at `appdecl.rs:1461-1463`.
  - A retract half for `service` is R9's option (s), which is the owner's open question at Step 4.6 and outside this fix.
- **Closure test:** a test that retires an app whose file declared a binding and a service. It asserts that the app's bindings have left the fold, and that `declared_exchange(<its exchange id>)` is still true, pinning the documented outcome. The page's sentence names both results.
- **Not architectural.** It is a text correction. The missing mechanism is SUR-P3-5's recorded architectural root cause, not a new one.
- My own round-1 remedy for STA-P3-2 ("an app is retired by loading once a file that names it and has no binding lines") did not scope the retirement either.

## 3. Attacks on the remediation (failed attacks are part of the result)

1. **The preflight's ordering against durable writes.**
   - `load_all` (`glade-node.rs:81`) runs before `boot` (`:82`). `boot` is the first durable act: it creates the cache directory, `instance.lock` and `node.key`, and saves the first-boot presence records.
   - Registration zips the preflight's parsed files (`:95`). No file is read twice, so no check-then-use race is possible.
   - A refused start writes nothing: a fresh home stays empty, and an existing instance is left unchanged.
   - The same ordering now stops a start for a later file's parse error, or for the flip's refusal, before any file registers. In round 1, earlier files had already registered and saved by then, so the new ordering produces strictly fewer partial states.
   - The `app … registered` line prints only after the save (`:103-104`), so its presence is evidence that the registration persisted.
   - File warnings print only once `boot` succeeds; that is a cosmetic point.
   - The legacy form never loads `--app` files. That form is used when there is no `--profile` and no `--name`, or when `Profile::parse` rejects the value. This is pre-existing and writes no system state.
2. **The v0 raw-token path.**
   - **In the fold:** an ordinary declaration, byte-identical to what 559cb2c stored (`v0raw` appends +0). A later edit appends and supersedes it normally. Moving the header to v1 without fixing the line is refused before any write. The fixed line (`ttl ttl=10m`) stores `ttl` and supersedes the raw one.
   - **For consumers:** nothing in production reads the zone or the retention. `declared_exchange` reads only the shape, and `bindings_of` has no production caller. A client fold would see a free string outside the contract's enums, which is the same class as `windowed` and `hourly` that v0 files already stored at 559cb2c and dddf8b8 under R10(a)/18b(ii). R9(b2)'s "one stored vocabulary" covers what b2 normalises, not v0's warned raw tokens.
3. **The retirement file.**
   - It is idempotent: loading it again appends +0.
   - If a crash comes before its save, nothing persists, and the missing `registered` line shows the retirement did not happen.
   - A retirement file and the app's real file in one start are refused by `load_all`, which fails closed.
   - Across nodes, its retractions carry no origin, so they take down every node's declaration of that app in a served store. This is the STA-P3-1 question, open at Step 4.6.
   - Its reach beyond bindings is STA-P3-4.
4. **The two-origin test.** Its assertions, name and "pinned, not endorsed" framing match what my `multiorigin` probe observes with a real served `Store`. No unscoped multi-origin claim remains in the node code, the notes or the page. The page describes a single node throughout.
5. **Crash, restart and rollback.**
   - Kill points converge byte-identically, exactly as in round 1 (`killretract`; `upgrade` B2 and B3).
   - The census still appends 15 (5+5+2+2+1) through the real 559cb2c code, with byte-identical prefixes. The owner's two-file store appends 7 (`upgrade` B1). Every shipped file loads with no warning.
   - Rollback to 559cb2c: the old node accepts the migrated store and appends nothing, and rolling forward appends nothing (B4). A pre-2.3 node refuses the v1 files at line 16 (B5).
   - Rollback to dddf8b8: it refuses a v0 file with `=` tokens, which fails closed.
   - No split-app configuration can exist under `1b9ac3f`. A store that dddf8b8 flapped converges once the files are merged (`mergeflapped`).
6. **Deliberate behaviour changes; fail-closed and not findings.**
   - Loading the same path twice is now refused, where before it appended +0 (disclosure 1).
   - A split-app start that loaded at 559cb2c, where nothing was ever retracted, is now refused with no warning release. This is the owner-ruled refusal variant.
   - A v0 file may now use the tail and `crdt` (`AppFileFormat.md:66-75`). A pre-2.5 node refuses such a file at the binding line, not at the header.

## 4. Risks and next action

- **The one-file-per-app guard lives in `load_all`, not in `register`.** Calling `register` per file for one app still flaps.
  - Plan Step 3.2 will turn `glade-node.rs`'s straight-line code into an assembled composition root; `load_all` must stay ahead of `boot` there.
  - `a_start_with_two_files_naming_one_app_is_refused_and_writes_nothing` guards the ordering as long as it spawns the assembled binary.
  - An `apps`-level registration API would carry the rule by construction.
- **Stale-binary gates (pre-existing, outside the object).** The glade-gyld and glade-gwz suites build the node only when it is missing (disclosure 7), so for a node change they can pass against a stale binary.
- **Open before Phase 4.** Multi-origin order and scope (STA-P3-1) and R9's option (s) (SUR-P3-5) are open at Step 4.6, and must be settled before two nodes load one app.
- **Next action:** fix STA-P3-4's text in the next page edit or the publish commit; it does not block. From the State axis the amendment can proceed to Step 2.9's publish once the other axes' round-2 verdicts are in.
