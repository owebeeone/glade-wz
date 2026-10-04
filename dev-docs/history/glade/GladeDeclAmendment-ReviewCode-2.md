# GladeDeclAmendment — CODE-AXIS RE-VERDICT (round 2)

**Review object:** the glade-decl contract v1 amendment after its round-1 remediation, `dev-docs/glade/GladeDeclAmendment-RemPlan.md` (root `8056056`, dispositions confirmed "all recommended" at `0e4d882`). I checked each of my round-1 findings against my own counterexamples, then attacked the remediation diff. Status: remediated and locked, not published.

**Baseline** (read with `rev-parse --short=12 HEAD`; sources read with `git show` / `git diff` at these SHAs; every in-scope tree clean):

| Repository | SHA | Remediation range reviewed |
| --- | --- | --- |
| glade-wz root | `73c2bf7e9a72` at start, `690f2f6807f9` at end (see note) | `cd6a498..73c2bf7`: `GladeDeclSurface.md` rows (`47d3568`); plan Step 2.6 Done line, Steps 4.3 and 4.6 |
| glade | `1b9ac3fd5bd6` | `1b9ac3f` only. `831eded` and `97d6afc` are out of scope. |
| glade-decl | `3d109177a222` | `784840a..3d10917` |
| glade-decl-rs | `d3be799db4f3` | `9507c12..d3be799` (README) |
| glade-decl-ts | `85ec18da1cb7` | `37d9b7d..85ec18d` (README) |
| glade-decl-py | `ee2f9602b2c1` | `433215a..ee2f960` (README) |
| grazel | `e1a4078d211f` | `b604a06..e1a4078` (README) |
| glade-gyld / glade-gwz / glial / grip-core | `c9ef7a66b6e4` / `fc0bb990ef09` / `4c6e8561c174` / `97ff6c26f12e` | unchanged |
| gryth-wz · glade-decl-ts · glial · gryth-ui | `3c64e7a59064` · `37d9b7d8a680` · `4c6e8561c174` · `9323818a39d2` | unchanged |

**Tuple note.** The root moved from `73c2bf7` to `690f2f6` during the review. `git diff --name-only 73c2bf7 690f2f6` lists three gwz bookkeeping files: `gwz.lock.yml` (glade-discover's out-of-scope entry, `48bc104` → `9caac2e`), `markers/conf-integrity.yml`, and that commit's marker `01a0ce91-….yaml`. This matches the coordinator's corrected note. Every in-scope member SHA was identical at start (2026-09-23 23:50:59 AEST) and at end (2026-09-24 00:02:58 AEST). By the end, the root also held one untracked file, `dev-docs/glade/GladeDeclAmendment-ReviewSurface-2.md`, another axis's report being filed. I did not open it. I treat the tuple as held.

**Date:** 2026-09-23 / 2026-09-24

**Axis:** Code — architecture, interfaces, call graphs and compatibility reality. Independent, adversarial, read-only. The other axes' round-2 reports were not read. Filed verbatim by the lane owner.

**Verdict: GO.** All seven round-1 findings are closed (COD-P2-1, COD-P3-1..6). One new finding: 0 P0 · 0 P1 · 0 P2 · 1 P3 (COD-P3-7, non-blocking). No finding is architectural.

---

## 0. Evidence base

Commands run. All were read-only; builds went only to scratch targets, and every node binary ran with `GLADE_HOME` and `HOME` pointed at a scratch directory.

| Where | Command | Result |
| --- | --- | --- |
| glade-decl | `build.py --check` | exit 0. 7 artifacts in lockstep; mirror is banner plus byte-identical; `CONTRACT_VERSION = 7d18cd3…` in all 3 renderings. |
| glade-decl | `build.py --compat --deleted AdvertisementRecord,edge/advert` | green. 24 shared vectors byte-identical; exactly the two declared vectors departed. |
| glade-decl | `build.py --compat` (no deletion list) | exit 1 |
| glade-decl | `python3 -m unittest corpus/test_build.py` and `python3 corpus/test_build.py` | both report "Ran 25 tests … OK" |
| glade/node | `cargo test -p glade-node` | 121 passed, exit 0: lib 112, `binding_census` 5, `one_file_per_app` 3, `shipped_app_files` 1 |
| glade/contracts | `cargo test --locked --offline -p glade-binding-api --all-features` | 5 passed, including `rejects_atom_resolved_as_value` (should panic), plus the `compile_fail` doctest |
| glade-decl-ts | `pnpm test` | 27 passed |
| revised node binary | two files naming app `x`; a control of two apps over three starts; one file passed twice | see COD-P2-1 in §1 |

Read-only inspection:
- inode and link counts of glade-decl-ts sources and of the consumers' installed copies;
- a grep of the grazel, glade-gyld, glade-gwz and glade trees for assertions on any node message this patch changed (none found);
- COD-P3-4's grep;
- the TypeScript output list tautc writes (`taut/src/taut/gen/scaffold.py:33-38`, plus `api.ts`).

**Not run:**
- **`sh glade/node/check.sh`.** Mid-review, the data volume reached 100% with 172 MiB free. My attempt to rebuild the pre-remediation node from a `git archive` of `dddf8b8` failed with `No space left on device` while compiling iroh. I deleted all my scratch builds (about 2.9 GB). The volume then had 1.7–2.9 GiB free and was falling as other processes wrote. A full check.sh run (node workspace plus clippy) needs about as much space as that, so I did not start it. Its node-tests component overlaps the `cargo test -p glade-node` run above; its contracts component was covered only for binding-api.
- **grazel, glade-gyld, glade-gwz suites:** not run, by the standing rule.

## 1. Closure of round-1 findings

| ID | RemPlan disposition | Status | Evidence |
| --- | --- | --- | --- |
| **COD-P2-1** | ACCEPT, refusal variant (= STA-P2-1; SUR-P3-1 case 2) | **CLOSED** | See the notes below this table. |
| **COD-P3-1** | ACCEPT, State's variant (= STA-P2-2) | **CLOSED** | See the notes below this table. |
| **COD-P3-2** | ACCEPT (= SUR-P3-2) | **CLOSED**; the missing half is filed as COD-P3-7 | **Record fixed:** the plan's Step 2.6 Done line (`GladeFirstSlicePlan.md:446`) now matches the code: the warnings ship in the first release above `0.0.0`, and the release after it flips.<br>**Mechanism added:** a version bump that has not decided the flip now fails a test.<br>- `V1_WARNING_RELEASE` (`appdecl.rs:141`);<br>- `a_node_release_decides_the_flip` (`:1507-1519`): `flip_decided("0.1.0", None, false)` is `Err`, and past the release `!refuse` is `Err`.<br>**Texts:** `v1` warnings end with `V1_REFUSED_LATER` (`:144`, `:328`).<br>**What is still missing:** the check does not catch a premature flip. That is COD-P3-7. |
| **COD-P3-3** | ACCEPT (= STA-P3-3) | **CLOSED** | **Mechanism:** `build.py --ts-from` passes tautc's `*.ts` through `write_artifact` (`ts_copies`; `main`'s `--ts-from` arm). It sits in the mode group with `--check` and `--compat`, and it errors before any write if the rendering is absent or the source holds no `.ts` file.<br>**Procedure:** the regeneration block in `glade-decl-ts/README.md` no longer uses `cp`.<br>**Test:** `TsFromTest` passes; the installed copy's bytes are unchanged and the source's link count is 1.<br>**Real tree:** `glade-decl-ts/README.md` was replaced by rename (inode 397653465, link count 1). glial's and grip-core's installed READMEs keep inode 397373421 with 37d9b7d's bytes (md5 match).<br>**Correction to my round-1 closure test:** I accept disclosure 4. What holds is "installed bytes and inode unchanged, source link count 1", not "link counts unchanged".<br>**Old behaviour:** `cp` writes through a hard link (my round-1 scratch demonstration), and the old `build.py` has no `--ts-from`. |
| **COD-P3-4** | ACCEPT (= SUR-P3-3) | **CLOSED** | **Texts corrected:**<br>- `glade-decl/README.md:55-60` and `:136`;<br>- OpenNotes N3;<br>- root `GladeDeclSurface.md:30, :32`;<br>- the mirror, which `--check` holds byte-identical.<br>**Grep:** the stale phrase now appears only in N10 and in the schema comment (`ir/glade_decl.taut.py:122-123`).<br>**The schema comment** is left alone on purpose: editing it would re-pin three renderings. N10 dates it and schedules the fix with the next contract change, which was my round-1 remedy's second option. |
| **COD-P3-5** | ACCEPT | **CLOSED** | BI-002's list (`contracts/binding-api/src/lib.rs:116-125`) now names all eight non-`value` members. `rejects_atom_resolved_as_value` (`tests/public_contract.rs:78`) panics as required. Adding `Log` (disclosure 1) matches the fixture, which permits only `value`. With the old list, which lacked `Atom`, the wrong resolver would pass. |
| **COD-P3-6** | ACCEPT | **CLOSED** | Both invocations report 25 tests, and the `__main__` block is now the file's last. At `784840a:189-193`, `unittest.main()` preceded `WriteReplacesTest`, so the script path ran 20 of 23. |

**COD-P2-1 — detail**
- **Live refusal.** The revised binary was run as `--profile local --name t --app a1 --app a2 0`, both files declaring `app x`. It exited 1. Stderr read `Error: Custom { kind: InvalidData, error: ".../a2.glade: app \`x\` is already declared by .../a1.glade (an app is declared by one file)" }`. The temporary home stayed empty.
- **Control.** `x.glade` and `y.glade` (two different apps), started three times on one store: `+3`, then `+0` / `+0`, then `+0` / `+0`, three records in all.
- **Code.**
  - `load_all` (`appdecl.rs:511-527`) runs at `glade-node.rs:81`, before `boot()` at `:82` and before any `register`/`save` (`:95-106`). This is L1-14's preflight.
  - A parse failure in any `--app` file now also stops the node before it writes. At `dddf8b8` it came after `boot()` and after the earlier files were saved.
- **Tests.** `tests/one_file_per_app.rs`, 3 passed. The binary test fails on the old behaviour: a node that does not refuse runs past the 20 s deadline and is killed.
- **Old behaviour.** `dddf8b8:glade-node.rs:89-101` loaded and registered each file with no cross-file check. My round-1 trace gives +3 and then +4 per boot. State reproduced it live on the old tuple, per the RemPlan (5 → 29 records over five boots). My own live re-run of the old binary was aborted by the disk.
- **Texts** match (`appdecl.rs:43-55`, `register`'s doc, `AppFileFormat.md:42-47, :76-79, :291-294`, attach notes). A later start whose file names the same app replaces that app's set, and the page says so.

**COD-P3-1 — detail**
- **My counterexample now loads.** `glade-app v0` with `binding g value share commons ttl=10m` parses Ok, stores `ttl=10m` raw, and warns on its line (tests `appdecl.rs:1376`, `tail.rs:294`).
- **`v1` twins** are refused whatever `V1_TOKEN_CHECKS_REFUSE` says (guard `tail.rs:60`; test `appdecl.rs:1399`). This is the owner-confirmed variant.
- **The texts agree:** `appdecl.rs:36-41`, `tail.rs:15-23`, `AppFileFormat.md:71-75` and `:134-138`.
- **No lines newly accepted.** Seven-token lines that were refused at `559cb2c` are still refused.
- **Old behaviour:** `dddf8b8:tail.rs:54-60` refused the token under both headers, and its test asserted that under `v0`.

## 2. New findings

### [COD-P3-7] The flip check catches a late flip but not an early one

**Location:** `glade/node/src/appdecl.rs:1482-1494` (`flip_decided`), held to the build at `:1507-1519` (`a_node_release_decides_the_flip`). The constants are at `:135` and `:141`.

**Invariant violated:** 31b (ii) and 18b (ii) as the code states them (`appdecl.rs:118-134`): the first release above `0.0.0` ships the `v1` warnings, and only the release after it refuses. My round-1 remedy asked for a test that fails if the constant is still `false` once the version is past that release, **or `true` before it**. Only the first half was built.

**Reproduction** (static; each step is a one-line edit):
1. Set `V1_TOKEN_CHECKS_REFUSE = true` while the version is `0.0.0`. `flip_decided("0.0.0", None, true)` returns `Ok` (`:1483-1485`).
2. Every token test reads the constant and holds on both sides of it (`v1_reports`, `:1151`; `a_v1_file_is_warned_this_release_and_refused_from_the_next`, `:1284`). So `cargo test -p glade-node` stays green.
3. Cut the first release: version `0.1.0`, `V1_WARNING_RELEASE = Some("0.1.0")`, constant still `true`. `numbered("0.1.0") > numbered("0.1.0")` is false (`:1489`), so the check passes again.
4. That first release refuses `v1` files for zone and retention tokens, so no release ever carries the warnings.

**Impact:** the warning window both rulings require can be lost with every gate green. This is the same outcome COD-P3-2 raised, reached by the other direction. It is bounded: it takes a mistaken edit, and the corrected Done line no longer tells anyone to make one.

**Remedy:** in `flip_decided`, return `Err` when `refuse` is `true` either at `0.0.0` or at a version at or below the warning release.

**Closure test:**
- `flip_decided("0.0.0", None, true)` is `Err`;
- `flip_decided("0.1.0", Some("0.1.0"), true)` is `Err`;
- `flip_decided("0.2.0", Some("0.1.0"), true)` stays `Ok`.

**ARCHITECTURAL:** no.

## 3. Attacks on the remediation that failed

1. **`load_all` placement.** It is the only production call before `register`, and nothing between them can make the preloaded declarations stale. Step 3.2's assembly will keep the preflight or trip the binary test, which spawns the real binary.
2. **One file passed twice** (disclosure 2). `--app x.glade --app x.glade` now exits 1, and the message names the file twice. At the `register` level, loading the same declaration twice still appends 0 (`registering_twice_appends_nothing`). No shipped caller names one app twice:
   - grazel's `node_argv` (`src/lib.rs:239-253`) passes `grazel` and `gyld`;
   - each fixture repository passes one file;
   - `gyld-ui.py` uses grazel's defaults.
3. **Contract untouched.** Pins are still `7d18cd3`, `--compat` is green, and the rendering commits change READMEs only. No stored record changed: `BindingRetraction` is unchanged, and the `sysdata.taut.py` comment edit changes no generated line.
4. **Changed node texts** (header error, `crdt` parenthetical, `v1` warning suffix, `external` warning): no consumer or fixture test asserts on them. The census still holds at 15 / 7, and every shipped file loads with no warning; none uses `external`.
5. **`--ts-from`.**
   - tautc writes no `index.ts` or `corpus.test.ts`, so the hand-written pin and gate cannot be clobbered.
   - `TsFromTest` redirects every path `main()` writes, and its write wrapper asserts the path is inside the scratch tree before writing.
6. **Multi-origin.** The fold's claims are now scoped to one registry (`registry.rs:472-506`, `sysdata.taut.py:98-106`, attach notes). The two-origin test pins today's outcome, and plan Step 4.6 records the open question. "One clock per registry" holds even after a node-id change, because `next_binding_lamport` spans every origin in the registry.
7. **Observations for the Surface axis** (text only, not code defects):
   - `AppFileFormat.md`'s zone and retention "Checking" paragraphs (`:211-221`, `:271-283`) state the `v1` rule without the key=value exception. The exception is stated at `:71-75` and `:134-138`.
   - "Retiring an app … withdraw every surface" (`:320-324`) withdraws binding surfaces only. The app's `service` exchange stays routable, which is SUR-P3-5's recorded deferral and the page's own "Other lines".

## 4. Risks and next action

- **Disk (operational, urgent).**
  - The data volume filled to 100% during this review (172 MiB free). After I deleted all my scratch builds, 1.7 GiB was free at the end and falling.
  - The session scratchpad still holds about 12 GB that is not mine; I did not open it.
  - The owner's live instance writes `records.json` through a temporary file and a rename. With the disk full that write fails.
  - Reclaim space before any further builds or runs of the gate.
- **Gate not run by me.** `glade/node/check.sh` (see §0); the implementer reports it 8/8.
- **Rebuilt node binary** (disclosure 9). `glade/node/target/debug/glade-node` is the binary the owner's grazel restarts from, and it now includes this patch.
  - The owner's two files name two apps, so the new refusal does not affect them.
  - The first restart appends the census share documented in round 1, or more if the store holds duplicates or lines deleted before the upgrade.
- **Recorded elsewhere, unchanged:**
  - gryth-wz/glade-decl-ts is one README-only commit behind;
  - the v0 consumers (glade-chat and others) are recorded for PackageExtractionPlan step 1.6;
  - CON-P2-2's durable gate for `api.ts`;
  - Step 4.6's multi-origin order.
- **Next action.** The code axis does not block the publish. COD-P3-7 is one extra condition and three asserts; it can ride this patch or the node's next change.
