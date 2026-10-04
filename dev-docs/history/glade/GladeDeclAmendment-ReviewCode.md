# GladeDeclAmendment — CODE-AXIS REVIEW

**Review object:** the glade-decl contract v1 amendment as landed. This is plan Steps 2.1–2.8 of `dev-docs/GladeFirstSlicePlan.md` (Phase 2), at glade-wz root `cd6a498bd2c1`. Status: landed and locked, not published (Step 2.9's publish waits on this review). Reviewed 2026-09-23.

**Baseline:**

| Repository | SHA |
| --- | --- |
| glade-wz root | `cd6a498bd2c1` |
| glade-decl | `784840acfb05` |
| glade-decl-rs | `9507c1233c0a` |
| glade-decl-ts | `37d9b7d8a680` |
| glade-decl-py | `433215acc4dc` |
| glade | `dddf8b89cad7` |
| grazel | `b604a06970fe` |
| glade-gyld | `c9ef7a66b6e4` |
| glade-gwz | `fc0bb990ef09` |
| glial | `4c6e8561c174` |
| grip-core | `97ff6c26f12e` |
| gryth-wz | `3c64e7a59064` |
| gryth-wz/gryth-ui | `9323818a39d2` |
| gryth-wz/glade-decl-ts | `37d9b7d8a680` |
| gryth-wz/glial | `4c6e8561c174` |

- **Authority only:** gwz-dev `918627cb88ee`. `git log ff431743cc4c..HEAD -- dev-docs/AgentProcessRules.md` is empty, so L1-07..09 and L1-17..19 are unchanged.
- **Tuple checks:** every SHA was read with `rev-parse --short=12 HEAD` at the start and again at the end, and they were identical.
- **How sources were read:** with `git show <sha>:<path>` and with `git diff` over the stated ranges. Gates and app files were read in clean working trees equal to their HEADs.
- **Parallel-lane edits in glade:** during the review, glade's working tree picked up a parallel lane's uncommitted Step 3.1 edits under `contracts/`. Their timestamps start at 22:48; my node test run had finished at 22:38. They are out of scope, glade's HEAD did not move, and nothing in this report was read from them.

**Date:** 2026-09-23

**Axis:** Code — architecture, interfaces, call graphs and compatibility reality, checked against:

- the controlling DRAFT, `GladeDeclReconciliation.md` rev 4: §3's RULED lines and §4.0–§4.8;
- plan Phase 2.

Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: NO-GO** — 0 P0 · 0 P1 · 1 P2 · 6 P3. No finding is architectural. I pre-commit to GO on a revision that resolves COD-P2-1 as specified.

---

## 0. Evidence base

Commands run. All are read-only, and builds went only into the scratch target.

| Where | Command | Result |
| --- | --- | --- |
| glade-decl | `build.py --check` | exit 0. "all 7 glade-decl artifacts in lockstep". The mirror is banner plus byte-identical. `CONTRACT_VERSION = 7d18cd3c…` in all 3 renderings. |
| glade-decl | `build.py --compat --deleted AdvertisementRecord,edge/advert` | green. 26 v1 against 26 v0 vectors: 24 shared, every one byte-identical. Exactly the two declared vectors departed. Added: `ShapeProfileDecl` and `edge/binding-account-atom`. |
| glade-decl | `build.py --compat` | exit 1, with two "undeclared absence" failures. This is row 2 red without the declaration, as specified. |
| glade-decl | `python3 -m unittest corpus/test_build.py` | 23 tests OK |
| glade/node | `cargo test -p glade-node` | lib: 104 passed. `tests/binding_census.rs`: 5 passed. `tests/shipped_app_files.rs`: 1 passed. Exit 0. |
| glade-decl-rs | `cargo test` | `corpus_byte_parity` ok |
| glade-decl-ts | `pnpm test` | 27 passed |
| glial | `pnpm exec vitest run` and `pnpm exec tsc --noEmit` | 15 files, 99 tests passed. tsc exit 0, against an installed `@owebeeone/glade-decl` whose `index.ts` pins `7d18cd3`. |

`git status --short` was clean in every tree after each run, apart from glade's out-of-scope `contracts/` edits.

**Not run, by instruction.** Their recorded results are the plan's Done lines; I read their sources instead.

- the grazel, glade-gyld and glade-gwz suites;
- the `glade/contracts` workspace (binding-api was read at `dddf8b8` with `git show`);
- pytest from the wheel;
- gryth-ui;
- grip-share's `node:test` file.

**Other read-only inspection:**

- **§4.2 `decl.v0` enumeration:** it finds only `V0_PATH` in `build.py` and the handoff note.
- **§4.4 census, re-run at the tuple:** 15 `from-cursor`, 13 `latest`, 0 `windowed`, 0 tails. All five paths are headed `glade-app v1`, and the twins are byte-identical.
- **grazel's stderr handling** (`src/main.rs:89-90`, `:121-135`): it drains and forwards the node's stderr.
- **Hard links:** `stat` inode and link counts of `glade-decl-ts/src/*` and of the consumers' installed copies.
- **History:** `sysdata.taut.py`'s `BindingDecl` shape is unchanged since `b9b1126`.
- **`cp` behaviour:** a scratch-only demonstration that `cp` onto an existing hard-linked file rewrites the shared inode. No repository was touched.

## 1. Findings

### [COD-P2-1] Two `--app` files that name the same app retract each other's bindings on every boot

**Location:**
- `glade/node/src/appdecl.rs:479-504`: `register` diffs against `BindingFold::declared_by(&decl.app)` (`registry.rs:556`), which is keyed by the app **name**.
- `glade/node/src/bin/glade-node.rs:89-101`: registers each `--app` file in turn, and nothing checks that an app name comes from only one file.

**Invariant violated:**
- R9(a)'s ruled scope states its own consequence (`GladeDeclReconciliation.md:1068-1074`, repeated at `glade/dev-docs/GladeGrazelAttachNotes.md:107-109`): "a declaration made by a **different app file** is never in scope".
- The module's headline guarantee: "re-loading the file appends nothing" (`appdecl.rs:42-44`).
- The public page (`glade/docs/AppFileFormat.md:260-262, 270-278`):
  - "loading an unchanged file registers nothing new";
  - "A surface declared by another app file is never touched";
  - "A surface stays declared on a boot that leaves its file out".

The retraction key is the app name. Nothing in the format, the page or the loader ties an app name to a single file. The per-file guarantee the ruling derives therefore rests on an invariant that is neither stated nor enforced.

**Reproduction:** `a1.glade` holds `glade-app v1`, `app x`, `binding x.one value share commons latest`. `a2.glade` is the same with `x.two`. Run `glade-node --profile local --app a1.glade --app a2.glade` repeatedly, with no edits.

- **Boot 1:** a1 appends `x.one`. a2 finds `x.one` live for app `x`, so it appends `x.two` and a retraction of `x.one`. That is 3 appends.
- **Boot 2, and every boot after:**
  - a1 finds `x.one` retracted and `x.two` live, so it re-declares `x.one` and retracts `x.two`;
  - a2 then re-declares `x.two` and retracts `x.one`.
  - That is 4 appends per boot, forever.
- **After every boot, `bindings_of()` is `[x.two]`.** `x.one` is declared by a file loaded on that boot, yet it is not live.
- **Traced through:** `BindingFold::over` (stamp `(lamport, origin, retraction, seq)`) and the one-clock rule in `next_binding_lamport` (`registry.rs:358-365`).
- **Left-out files:** a file left out on a later boot, while another file naming its app is loaded, loses its surfaces the same way. This brings back the SUR-P2-7 counterexample (the gyld leg switched off) whenever two files share an app name.
- **Before this object:** at glade `559cb2c`, the same two files headed `v0` registered 2 records on boot 1 and 0 on every boot after, with both surfaces declared.

**Impact:**
- **Silent:** no warning and no error.
- **Durable growth:** records accumulate without bound in `records.json` and in `dir.bindings` / `dir.binding-retractions`, which replicate to every peer through the home share.
- **Missing surface:** a loaded surface is absent from every fold (`bindings_of`, `declared_exchange`).
- **Untested:** the row-9 and row-10 tests in `node/tests/binding_census.rs` use distinct app names only, so nothing sees this.
- **After the publish:** fixing it would mean tightening a published file format.

**Remedy:**
- Make "one file per app" a rule of the format.
- Enforce it where it can be detected: refuse a boot whose `--app` files name one app twice, and name both paths in the diagnostic. This can live in `glade-node.rs`, or in a `register_all` that sees every file of the boot.
- State the rule on the format page and in the module doc: the `app` line names the app's whole binding set, so a different file naming the same app on a later boot replaces that set.
- The alternative is to register an app's files as one union. That restores idempotence, but a left-out file's surfaces are still retracted, so it needs the same sentence.

**Closure test:**
- A node test over `a1` and `a2` as above: the load is refused with a diagnostic naming both files. Under the union option instead, the second boot appends 0 and both surfaces are live.
- A regression: a single file loaded twice still appends 0.

**ARCHITECTURAL:** no. The ruled per-`(app, glade_id)` scope stands. The missing precondition is enforced at the load boundary.

### [COD-P3-1] A zone or retention token containing `=` is refused outright, in `v0` and `v1` files alike

**Location:** `glade/node/src/appdecl/tail.rs:54-60`, the slot check. It is called at `appdecl.rs:280`, ahead of the header-branched check at `appdecl.rs:285-297`.

**Invariant violated:**
- R10(a): "`v0` files still load, with a warning".
- 31b (ii) and 18b (ii): "a warning for one release, a hard error after".
- The module doc: "A `v0` file loads as it always did … it is never refused for them" (`appdecl.rs:36-39`).
- The page: "A `glade-app v0` file is never refused for its zone" and "… for its retention" (`AppFileFormat.md:192-196, 252-256`).

The same page (`:118-123`) and the tail's own doc say such a token is refused. So the texts disagree with each other as well as with the ruling.

**Reproduction:** a file of `glade-app v0`, `app x`, `binding g value share commons ttl=10m`.
- **At glade `559cb2c`:** this is a legal six-token line, the retention is stored raw, and the node boots.
- **At `dddf8b8`:**
  - `parse` returns: line 3: `ttl=10m` is a key=value entry where <retention> goes (the tail follows all five tokens);
  - `load` maps that to `InvalidData`, and `glade-node` exits before binding its listener.
- **Pinned by a test:** `a_tail_entry_where_a_token_goes_is_refused` (`tail.rs:253-267`) asserts exactly this refusal under a `v0` header.
- **`v1` files:** they get the same immediate refusal, with no warning release.

**Impact:** bounded. No shipped file has `=` in token 4 or 5, and the refusal is loud and line-numbered. But a third-party `v0` file that booted yesterday now stops the node. That is the SAF-P2-3 boot stop the landing order exists to avoid, and three texts promise it cannot happen.

**Remedy:** choose one.
- **(a)** Route a `=`-bearing zone or retention through the header branch as a token `v1` does not accept. A `v0` file gets a warning that also explains the tail. A `v1` file gets a warning until `V1_TOKEN_CHECKS_REFUSE` is flipped, then the refusal.
- **(b)** Keep the refusal as an owner-ruled exception. Then correct `appdecl.rs:36-39`, `AppFileFormat.md:192-196, 252-256` and the plan's Step 2.6 Done line.

**Closure test:**
- Under (a): the line above loads with a warning in a `v0` file. In a `v1` file, it warns this release and is refused after the flip, asserted through `v1_reports`.
- Under (b): the three texts name the exception, and a test pins it.

**ARCHITECTURAL:** no.

### [COD-P3-2] The warn-then-refuse flip has no owner, trigger or check, and the lane's record puts it one release early

**Location:**
- `appdecl.rs:111-125`: `V1_TOKEN_CHECKS_REFUSE`, a `const` edited by hand.
- `dev-docs/GladeFirstSlicePlan.md:446`: the Step 2.6 Done line.
- `AppFileFormat.md:190-191, 250-252`: "this release" and "the next release".

**Invariant violated:** 31b (ii) and 18b (ii) say exactly one release of warnings, then refusal. That is a lifecycle pair, and its second half exists only as prose.

**Reproduction:**
- **The code** (`appdecl.rs:119-124`) says the warnings ship in the first release that carries this code, meaning the first version above `0.0.0`, and the release after that sets the constant to `true`.
- **The Done line** says the constant "flips it to a refusal at the next node version above `0.0.0`". That is the first release.
- **What exists to trigger it:** nothing.
  - `node/Cargo.toml` is at `0.0.0` and no tag names a node release;
  - nothing reads `CARGO_PKG_VERSION`;
  - no OpenNote, plan step or checklist owns the flip;
  - the page's "this release" and "next release" point at nothing.

**Impact:** there are two ways this goes wrong.
- An executor following the lane's record flips before the first release, so no release ever carries the warnings.
- Or nobody flips, and `v1` validation stays advisory indefinitely.

Either way, the lifecycle the rulings require is not the one delivered.

**Remedy:**
- State the trigger once, and correct the Done line.
- Record the flip as an owned step, in an OpenNote or a release checklist.
- Preferably make it mechanical: add a `FIRST_WARNING_RELEASE` constant and a test that fails if the constant is still `false` once `CARGO_PKG_VERSION` is past that release, or `true` before it.

**Closure test:** that test, or the recorded step together with the corrected Done line.

**ARCHITECTURAL:** no.

### [COD-P3-3] The documented TypeScript regeneration still writes through pnpm's hard links; 784840a fixed only `build.py`'s own writes

**Location:** `glade-decl-ts/README.md:32-33`.
- Line 32: `cp /tmp/g/typescript/*.ts ../glade-decl-ts/src/`.
- Line 33: `cp ir/glade_decl.ir.json corpus/decl.v1.json ../glade-decl-ts/src/`. This one is now redundant, because `build.py` writes both files.

**Invariant violated:** 784840a's own rule (`build.py:436-461`): a regeneration must leave consumers' installed copies as installed.

**Reproduction:**
- **Link count 4:** at the tuple, `glade-decl-ts/src/api.ts`, `index.ts` and `glade_decl.ir.json` each have link count 4. They share inodes with the `@owebeeone/glade-decl` installed in glial and in grip-core (for example, `api.ts` is inode 397369444 in all three places).
- **Link count 8:** `cbor.ts`, `codec.ts` and `schema.ts` each have link count 8. They share inodes 73996343, 73996344 and 73996346 with glade-chat's install, which is still the v0 contract (`CONTRACT_VERSION 99a04e0…`, `decl.v0.json`).
- **`cp` semantics:** `cp` onto an existing file truncates it and rewrites that inode, as the scratch demonstration showed.
- **Line 33** recreates, word for word, the `glade_decl.ir.json` incident that 784840a records.

**Impact:** latent until the next regeneration, then silent.
- glial's and grip-core's installed contract changes without a reinstall.
- glade-chat becomes a mixed install: v0 types with new runtime files.
- Consumer gates such as row 12 then read content that no install produced.

Reinstalling recovers it, as it did last time.

**Remedy:**
- Make the procedure replace files rather than overwrite them. Any of these works:
  - `rm -f` each target before copying;
  - copy into a temporary file and `mv` it into place;
  - have `build.py` copy tautc's output using `write_artifact`.
- Drop line 33.

**Closure test:** after the documented regeneration, the consumers' installed inodes and link counts are unchanged. A script, or a `build.py` mode, can assert this.

**ARCHITECTURAL:** no.

### [COD-P3-4] The contract-side texts still say an app file cannot state a TTL duration

**Location:**
- `glade-decl/README.md:132`;
- `glade-decl/dev-docs/OpenNotes.md:39-42` (N3);
- `glade-decl/ir/glade_decl.taut.py:120-124`;
- the root `dev-docs/glade/GladeDeclSurface.md:30` (the `Retention` row), and its mirror `glade-decl/dev-docs/DeclSurface.md:32`.

**Invariant violated:** code and documents must agree at the settled tuple.

**Reproduction:**
- **The five texts** each say the duration cannot be stated until the node parses R11(a)'s `ttl=<duration>` tail key.
- **The node:** since Step 2.5 (`967fcdf`) it parses and validates `ttl=10m` (`tail.rs:74-81`, `:104-125`).
- **The format page** teaches the key (`AppFileFormat.md:108`, `:226-230`) and says the node does not record it yet (`:131-133`).
- **Row 18's set:** the ratified root page is one of row 18's published pages.

**Impact:** the controlling declaration page and the contract README tell an author something that both the node and the format page contradict. The texts were accurate at `7d18cd3`, and nothing in Step 2.5 carried them forward.

**Remedy:**
- **Wording:** replace the claim with "stated with the tail key `ttl=<duration>`, which the node validates and does not yet record".
- **Where:** the root row, `README.md:132` and N3. Then re-copy the mirror, which `--check` enforces.
- **The schema comment** is a contract path. Either fix it in a comment-only commit and re-pin the three renderings, or leave it and date the claim.

**Closure test:** a grep of the five locations, and `build.py --check` green (the mirror, plus the pins if the schema comment moved).

**ARCHITECTURAL:** no.

### [COD-P3-5] binding-api's BI-002 does not probe `Shape::Atom`, the member v1 added

**Location:** `glade/contracts/binding-api/src/lib.rs:116` at `dddf8b8`: `for shape in [Shape::Message, Shape::Window, Shape::Stream, Shape::Crdt]`.

**Invariant violated:**
- The trait's rule: "Unknown/unsupported shapes MUST NOT fall back to value" (`lib.rs:37`).
- LBT-011: a public-contract change must test its affected consumers.

**Reproduction:** take a resolver whose shape check is `matches!(shape, Shape::Value | Shape::Atom)`, and which resolves both as the value fixture. It passes BI-001, BI-002 and BI-003. `atom` is described as "a single-writer latest value", which makes it exactly the member a naive resolver folds into `value`. `Exchange` and `Swmr` are also missing from the list, but that gap predates this object.

**Impact:** binding-api is the one crate that depends on a rendering, and it cannot detect the fallback its own contract forbids for the member this amendment introduced.

**Remedy:**
- Add `Shape::Atom` to the rejection list.
- The fixture permits only `value`, so add the other non-value members too.
- The edit lands in `glade/contracts`, where a parallel lane holds uncommitted work, so coordinate it with that lane.

**Closure test:** a deliberately wrong resolver that accepts `Atom` fails `bi_002_fail_closed`.

**ARCHITECTURAL:** no.

### [COD-P3-6] `WriteReplacesTest` is defined after `unittest.main()`, so the script entry point skips the hard-link regression

**Location:** `glade-decl/corpus/test_build.py:189-222` at `784840a`.

**Invariant violated:** a test file's own entry points should run the whole suite.

**Reproduction:**
- **The script path:** `python3 corpus/test_build.py` reaches `unittest.main()` at line 190. That runs the 20 tests defined so far and calls `sys.exit` before `WriteReplacesTest` exists.
- **What it skips:** the three write-by-replace tests, which are the regression for the incident 784840a fixes. The run reports green without them.
- **The documented path:** `python3 -m unittest corpus/test_build.py` imports the whole module and runs all 23.

**Impact:** small but concrete. One of the two ways to run the file silently drops the guard for the hazard in COD-P3-3.

**Remedy:** move the `if __name__ == "__main__":` block to the end of the file.

**Closure test:** both invocations report 23 tests.

**ARCHITECTURAL:** no.

## 2. Riders

- **SAF-P3-13: closed from the code's side.**
  - `node/tests/binding_census.rs:145-178` names its units explicitly: one registry per census file, and "the 15 is their SUM … No single store appends 15".
  - `:180-201` asserts one store's share, 7, for grazel's two-file boot.
  - Both tests pass.
- **CON-P3-11: closed from the code's side; formal closure belongs to the Consistency axis.**
  - Nothing implements R9(s): no `ServiceRetraction` or `WorkspaceRetraction` exists (`git grep` at `dddf8b8`).
  - `service`, `seed` and `workspace` keep the pre-R9 diff (`appdecl.rs:506-533`, the same comparison as at `559cb2c`).
  - The format page and the attach notes both say that deleting a `service` or `workspace` line retracts nothing.
  - Under the recorded answers, no §4.7 row is needed.
- **Surface residual: closed from the code's side; formal closure belongs to the Surface axis.**
  - `glade/docs/AppFileFormat.md` exists, and is linked from `glade/docs/README.md` and `GladeGrazelAttachNotes.md`.
  - It carries the grammar the parser accepts: the header, the tail, `workspace`, a gloss for each token, and R9's rules.
  - COD-P2-1 and COD-P3-1 falsify some of its sentences (listed in those findings). Their remedies include those edits.

## 3. Invariant analysis

These attacks failed, and are part of the result.

1. **Contract bytes (§4.1, §4.2).**
   - `atom=8` is appended, and `message` and `window` keep their numbers.
   - `ShapeProfileDecl{glade_id: 1 Ref(GladeId), profile: 2 STR}` matches §4.1 item 2 exactly.
   - `BindingDecl` is untouched.
   - `AdvertisementRecord` is removed, and its retired name is recorded in the docstring and in OpenNotes.
   - `--compat` proves the 24 shared vectors byte-identical, so synth's selection by member index did not move (`2 % 9` still selects `message`).
2. **The `--compat` specification.**
   - Rules 1–3 are implemented, plus a fourth: a declared name must be a v0 vector.
   - v0 is read from `bbce73d`, and v1 is compared as built rather than as committed.
   - The gate is red without the declaration and green with it.
3. **`--check`.**
   - It covers 7 artifacts, including the four rendering copies (SAF-P2-5).
   - It fails on mirror drift, and on a missing or ambiguous pin.
   - The expected pin is the last commit that touched the contract paths, `7d18cd3`; `784840a` touched only `build.py` and `test_build.py`.
   - An uncommitted contract change fails even a matching pin.
4. **Renderings.**
   - All three are generated into `src/` only; no `rust/`, `typescript/` or `python/` directory exists in any tree.
   - All three are pinned to `7d18cd3`, include `atom`, and no longer include `AdvertisementRecord`.
   - The wheel force-includes `decl.v1.json`.
   - `api.ts` carries the v1 types, so CON-P2-2's silent failure does not occur at this tuple, and glial type-checks against them.
5. **Nothing the rulings forbid.**
   - `sysdata.BindingDecl` is unchanged; the only addition is the `BindingRetraction` record kind.
   - The tail reaches no record (`the_tail_reaches_no_record`).
   - The wire IR and the gryth-ui vendored IR are untouched.
6. **Existing stores.**
   - An upgrade only appends; no existing record is rewritten.
   - On a fresh store, the first registration is byte-identical to the pre-amendment envelope (lamport = seq on one origin).
   - The first boot appends exactly the census share: 15 summed over the files, 7 for a grazel-plus-gyld store.
   - The first boot also corrects a historical duplicate or a stale winner, including one from a pre-amendment second origin. `register` re-appends with a newer lamport whenever the newest record differs from the file.
   - `BindingDecl`'s shape has been stable since `b9b1126`, so no record of an older shape can reach the fold's fail-open decoder.
7. **Lamport.**
   - Neither registry ingest nor `store.rs` validates lamport.
   - The single clock keeps each chain monotonic.
   - Every other record kind keeps lamport = seq, and the tests for this pass.
8. **Retained readers and older writers.**
   - A pre-2.3 node refuses a `v1` header (row 17, documented).
   - A downgraded node ingests `dir.binding-retractions`, and finds its raw `from-cursor` records unchanged.
   - No client folds `dir.bindings`: a grep over glial, client-ts, client-rs, grazel, glade-gyld, glade-gwz and gryth-ui finds none. The attach notes say a future client must fold both streams.
9. **Header and validation.**
   - Both headers load, and any other header is refused with its line number.
   - Every validation test holds on both sides of `V1_TOKEN_CHECKS_REFUSE`, and `from-cursor` is never reported.
   - The only previously accepted `v0` line that is now refused is the one in COD-P3-1.
10. **Error paths.**
    - Production code gains no new `unwrap`, `expect` or unguarded index.
    - The new decode sites use the same fail-open legacy codec as the pre-existing `declared_exchange` decode.
    - The warning channel cannot fill a pipe: grazel drains and forwards the node's stderr.
11. **Consumers.**
    - glial reads the profile lookup before any store opens, and never for a non-`crdt` mount.
    - grip-core's import is types-only.
    - grip-core, grip-share and client-ts contain no exhaustive `never` switch over `Shape`.
    - binding-api's `2beb57f` move is content-identical (the module body diffs empty), and its `#[cfg]` now sits on a braced module.
12. **Hygiene.**
    - New TypeScript control flow is braced.
    - No commit in the ranges carries an attribution trailer.

## 4. Risks and next action

- **Multi-node (Phase 4; recorded, not a finding).** The object's texts define "newest" as the highest `(lamport, origin)`, which is literally true, but Phase 4 has to decide what "newest" means across nodes.
  - The registry holds only this node's records, while the served fold holds every peer's.
  - So a node's binding clock never sees a peer's records, and "newest" across nodes is an arbitrary lamport comparison.
  - A retraction from node B can outrank node A's live declaration in every served fold. A's registry still counts that declaration as live, so A never re-appends it.
  - Retraction is scoped per app, not per origin, so two nodes loading different versions of one app's file retract each other.
  - A pre-amendment peer's lamport is simply its seq.
- **glade-chat.** It is a lock member and depends on `@owebeeone/glade-decl`, but it is not among the consumers listed in §4.5 or Step 2.8. It still holds the v0 install whose runtime files share inodes with `glade-decl-ts/src`. Its own suite has not been run under v1.
- **CON-P2-2 durability.** `api.ts` has no durable gate; row 11 was a one-time check.
- **The owner's first boot** will append more than the model figure of 7 if the store holds historical duplicates, or lines deleted before the upgrade. Those deleted lines are retracted on that boot, as the rule says.
- **Next action.**
  - Remediate COD-P2-1: refuse at the load boundary, and add the format rule.
  - Then re-review COD-P2-1's counterexample on the corrected tree.
  - Batch the P3s: COD-P3-1 and COD-P3-4 edit the same pages as COD-P2-1's text change, and COD-P3-3 and COD-P3-6 belong in one pass over the regeneration procedure.
