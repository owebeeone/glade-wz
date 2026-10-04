# GladeDeclAmendment — STATE-AXIS REVIEW

**Review object:** the glade-decl contract v1 amendment as landed. This is plan Steps 2.1–2.8 of `dev-docs/GladeFirstSlicePlan.md` (Phase 2), at glade-wz root `cd6a498bd2c1`. Status: landed, not published; this is Step 2.9's acceptance review. Read 2026-09-23.

**Baseline:**
- glade-wz root `cd6a498bd2c1`
- glade-decl `784840acfb05`
- glade-decl-rs `9507c1233c0a`
- glade-decl-ts `37d9b7d8a680`
- glade-decl-py `433215acc4dc`
- glade `dddf8b89cad7`
- grazel `b604a06970fe`
- glade-gyld `c9ef7a66b6e4`
- glade-gwz `fc0bb990ef09`
- glial `4c6e8561c174`
- grip-core `97ff6c26f12e`
- gryth-wz root `3c64e7a59064`, gryth-ui `9323818a39d2`, gryth-wz/glade-decl-ts `37d9b7d8a680`, gryth-wz/glial `4c6e8561c174`
- Authority: gwz-dev `918627c`. `git log ff431743cc4c..HEAD -- dev-docs/AgentProcessRules.md` returns 0 commits.

How the baseline was checked:
- Every SHA was read with `git rev-parse --short=12 HEAD` at the start and at the end of the review. They were identical both times.
- All 19 glade-wz lock entries equal their member HEADs. The gryth-wz lock names the fast-forwarded glade-decl-ts and glial.
- Sources were read from the working trees, and the per-member ranges with `git log/show/diff`. Every in-scope path was clean at the start and at the end.
- During the review, 10 uncommitted paths appeared under `glade/contracts`: the carrier, clock, grant and signer api crates, the contracts workspace manifest and lock, `architecture-policy.json`, `check.sh`, `test-selection.sh` and `arch002-fixture.sh`. This is the parallel lane's Step 3.1 work, which the prompt names out of scope. No commit moved and no in-scope path changed, so the tuple held.

**Date:** 2026-09-23

**Axis:** State — durable-state semantics and adversity: the first boot on an existing store, crash points between writes, the binding fold as a state machine, fail-closed direction, and the migration's reach outside the node. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: NO-GO** — 0 P0, 0 P1, 2 P2, 3 P3. I pre-commit to GO on a revision that resolves STA-P2-1 and STA-P2-2 as specified.

---

## 0. Evidence base

**Documents read**
- Controlling DRAFT: `GladeDeclReconciliation.md` revision 4 — the §3 RULED lines and both notes, §4.0–§4.8, the Appendix.
- The plan's Phase 2, including every Done line.
- `GladeDeclSurface.md` as amended.
- `AgentProcessRules.md` L1-07..L1-20.
- `DecisionLog.md` GDL-036..038.
- `LibraryBoundaryAndTestingPolicy.md` LBT-004 and LBT-007..011.
- `PackageExtractionPlan.md` steps 1.1–1.6.
- Every in-range diff that touches state: `node/src/{appdecl.rs, appdecl/tail.rs, registry.rs, sysdata.rs, exchange.rs, bin/glade-node.rs}`, `node/ir/sysdata.taut.py`, `node/tests/{binding_census,shipped_app_files}.rs`, `docs/AppFileFormat.md`, `dev-docs/GladeGrazelAttachNotes.md`, the five app files, glade-decl's `corpus/build.py` and `test_build.py`, the renderings' READMEs, and glial's `binder.ts`.
- `store.rs`, `sysdir.rs`, `claims.rs`, `mesh.rs` and `server.rs` were read unchanged, for context.

**Commands run** (none changes a repository)
1. `CARGO_TARGET_DIR=<scratch>/review-target cargo test -p glade-node --offline` in `glade/node`: 104 unit tests, `binding_census` 5 and `shipped_app_files` 1 all pass; 0 ignored.
2. `PYTHONDONTWRITEBYTECODE=1 /opt/homebrew/bin/python3 corpus/build.py --check` in glade-decl: exit 0. Output: "all 7 glade-decl artifacts in lockstep"; mirror banner present and byte-identical; `CONTRACT_VERSION` is `7d18cd3` in all three renderings.
3. Probes, written and run only under `<scratch>/state-probes/`, built into the scratch target. Every store was a fresh directory under `state-probes/stores/`.
   - No `node.key` was created or read. The class-2 path — `BlobStore::load` → `Registry::from_snapshot` → `appdecl::load` → `register` → `save`, per file, in `glade-node.rs:89-101`'s order — was driven with a fixed origin. No node or grazel process was started.
   - `old-node/`: the glade node crate at `559cb2c`, rebuilt file by file with `git show 559cb2c:node/src/…` (package renamed `glade-node-old`, bin target dropped). This runs the pre-amendment parser and register exactly as shipped, alongside the amended crate at the tuple.
   - `probe/src/bin/`:
     - `compat`: v0 parse, old against new.
     - `upgrade`: first boot, kill points, rollback, and an old node meeting v1 files.
     - `census`: §4.7 row 9 through the real base code.
     - `killretract`: kill points with retractions.
     - `sameapp`, `rename`, `multiorigin`.
   - `flipped-node/`: the dddf8b8 crate with `V1_TOKEN_CHECKS_REFUSE = true`; `cargo test --lib appdecl` gives 41 passed.
   - `hardlink/`: `cp` onto a hard-linked file, compared with build.py's replace.
4. Metadata only:
   - `stat -f %l` over `glade-decl-ts/src/*`;
   - `find glade-wz gryth-wz -inum …` for their hard-linked copies;
   - the mtime of `glade/node/target/debug/glade-node` (Sep 23 22:30, the dddf8b8 build);
   - both workspaces' `gwz.lock.yml` files;
   - a scan of every `.glade` file in glade-wz, gryth-wz and taut-dev for a binding whose zone or retention contains `=` (none found).

Not run, as instructed: the grazel, glade-gyld and glade-gwz suites, and any node or grazel process. Nothing under `~/.gyld-ui`, `~/.claude` or any `*.key` file was opened.

## 1. Findings

### [STA-P2-1] Two `--app` files naming the same `app` never converge: every boot retracts and re-declares each other's bindings

- **Location:**
  - `glade/node/src/appdecl.rs:488`: `BindingFold::over(&ops).declared_by(&decl.app)`. The diff basis is every live declaration of the app *name*.
  - `appdecl.rs:498-504`: one `BindingRetraction` for each of those declarations the file does not carry.
  - `glade/node/src/bin/glade-node.rs:89-101`: one `register` + `save` per `--app` *file*, with no check that app names are distinct across files.
- **Invariant violated:**
  - Registration is idempotent by diff. The module doc says "re-loading the file appends nothing" (`appdecl.rs:42-44`); `AppFileFormat.md:260-262` says "loading an unchanged file registers nothing new".
  - R9(a)'s scope, as ruled (§3 R9, "(a)'s retraction scope") and as restated at `GladeGrazelAttachNotes.md:108-109`, `AppFileFormat.md:273-274` and `sysdata.taut.py:96-99`: "a declaration made by a **different** app file is **never** in scope".
  - The ruling's "therefore" holds only if each app lives in one file. Neither the parser, the node nor the page requires that. The page's `app` rule is "exactly once" per file, and `--app` "may repeat".
- **Reproduction** (probe `sameapp`; real `BlobStore`; `glade-node.rs`'s loop):
  - file1: `glade-app v1` / `app notes` / `binding notes.list value share commons latest` / `binding notes.edits log share commons from-cursor`.
  - file2: `glade-app v1` / `app notes` / `binding notes.body value share commons latest`.
  - Boot `--app file1 --app file2` five times with no edit.
  - Amended node: appends +5, +6, +6, +6, +6 (records.json goes from 5 to 29 records). After every boot the only live binding is `notes.body`: registering file2 retracts file1's two surfaces, and registering file1 on the next boot declares them again.
  - Pre-amendment node (559cb2c) on the same files: +3, then +0 four times.
- **Impact:**
  - A new non-convergent durable state that the pre-amendment semantics did not have.
  - 2 × (the app's binding count) records are appended per boot, for ever: to records.json, then seeded into the served store and replicated into every peer's home share. The log is append-only; nothing compacts it.
  - Every file except the last loses its surfaces from the fold on every boot.
  - `declared_exchange` decodes the whole binding family on every subscribe and exchange request (`exchange.rs:74-79`), so that path also slows with each boot.
  - The trigger is an ordinary authoring move the page allows: splitting an app over two files, or copying an app file as the template for another and forgetting its `app` line. No shipped configuration triggers it; grazel's two files name `grazel` and `gyld`.
- **Required correction:**
  - Either preflight every `--app` file before registering any (L1-14) and refuse to start, naming both paths, when two files name the same app;
  - or register once per app, diffing the union of that app's files.
  - State the rule on `AppFileFormat.md` and in the attach notes.
- **Closure test:** load two files naming one app on three boots (same order, then reversed).
  - Refusal variant: records.json is unchanged and the message names both files.
  - Union variant: boots 2 and 3 append 0, `BindingFold::retracted()` is empty, and all three surfaces are live.
  - Keep a one-app-per-file control that stays at +0 after the first boot.
- **Not architectural.** The ruled scope key (the app name) stands. The defect is local: the implementation diffs per file but scopes per app.

### [STA-P2-2] A `glade-app v0` file whose zone or retention contains `=` is now refused at boot, with no warning release

- **Location:**
  - `glade/node/src/appdecl/tail.rs:50-60`: the slot guard. A zone or retention token containing `=` is a parse error.
  - It is reached from `appdecl.rs:280` for every binding line, whatever `decl.version` is.
  - Introduced at `967fcdf` (Step 2.5), whose goal was "with nothing validated yet". Kept at `d4db2fc` (Step 2.6), whose comment justifies it by the very property it breaks.
- **Invariant violated:**
  - R10(a) as ruled: "`v0` files still load, with a warning". R10's option table says third-party files "keep working, with a message naming the replacement".
  - The object's own statements:
    - `appdecl.rs:36-39` ("A `v0` file loads as it always did … it is never refused for them");
    - `AppFileVersion::V0` (`appdecl.rs:175-178`);
    - `AppFileFormat.md:192-196` and `:252-256` ("A `glade-app v0` file is never refused for its zone / retention");
    - plan Step 2.6's Done line (`GladeFirstSlicePlan.md:446`, "`v0` files are warned, never refused").
  - The page contradicts itself: `:120-123` lists this refusal without exempting `v0`.
- **Reproduction** (probe `compat`):
  - File: `glade-app v0` / `app x` / `binding cache value share commons ttl=10m`.
  - At 559cb2c, `parse` returns Ok and stores the retention `"ttl=10m"`.
  - At dddf8b8, `parse` returns ``line 3: `ttl=10m` is a key=value entry where <retention> goes (the tail follows all five tokens)``.
  - The zone `a=b` and the retention `window=100` behave the same way.
  - `load` wraps the error as `InvalidData`, and `glade-node.rs:90` exits before the listener binds.
- **Impact:**
  - A v0 file that booted a node yesterday stops it on upgrade, with no release in which it was warned. This is the break R10(a) and 18b(ii) exist to prevent.
  - The likeliest such token is `ttl=<duration>`: what an author wanting a duration would have typed before the tail existed.
  - The failure is fail-closed and line-numbered; no record is invented.
  - No file in glade-wz, gryth-wz or taut-dev has such a token. The exposed population is third-party files, which are the ones R10(a) protects.
- **Required correction:**
  - Apply the slot guard only to `glade-app v1` files.
  - In a `v0` file, store the token as written, as before, and warn through the v0 channel like any other token v1 does not accept.
  - If the owner prefers to keep the refusal, the ruling's implementation statements, both "never refused" sentences on the page, the module doc and the Done line must name the exception, and the refusal still needs a warning release first.
- **Closure test:** a `v0` file with retention `ttl=10m`, and one with zone `a=b`:
  - both parse Ok;
  - both store the token raw;
  - both carry a line-numbered v0 warning.
  - Their `v1` twins are refused with today's message on both sides of `V1_TOKEN_CHECKS_REFUSE`.
- **Not architectural.**

### [STA-P3-1] The binding family's order and retraction scope work for one registry, but the object claims them for the replicated store

- **Location:**
  - `glade/node/src/registry.rs:358-365`: `next_binding_lamport` takes one past the maximum over `self.ops`, i.e. this registry only.
  - `:472-475`: "the documented `value` rule, highest `(lamport, origin)` wins".
  - `:497-498`: "The same fold serves the registry, `register`'s diff, and the served store".
  - `exchange.rs:61-80`: the fold over the served store.
  - `sysdata.taut.py:100-102`: "newest by (lamport, origin) winning, so a later declaration of the surface revives it".
  - `GladeGrazelAttachNotes.md:104-113`: "a declaration made by a different app file is never in scope", and "A client that folds `dir.bindings` itself must fold `dir.binding-retractions` with it the same way".
- **Invariant violated:** the stated revival and scope rules do not hold in the store the object says the fold serves.
  - The registry never takes in a peer's op. `Registry::ingest` (`registry.rs:279`) is private; the only writers are `from_snapshot` and this node's `append_returning`.
  - The served store holds every node's home-share records, through the connect-time sync and `ingest_and_fanout` (`mesh.rs:434`).
  - So the binding "lamport" is a per-node counter. It never advances past what the node has seen, unlike the client's value-fold clock (`glade/client-ts/src/session.ts:64`).
  - A retraction is keyed `(app, glade_id)` with no origin, so it takes down every node's declaration of that surface.
- **Reproduction** (probe `multiorigin`):
  - Registries A and B both register `app grazel` with `g` and `h`. A already holds six other binding records.
  - A re-registers without `g`, appending a retraction at lamport 8.
  - Folding A's ∪ B's ops through a real served `Store`: `g` is absent, while B's own registry fold shows it live.
  - B then edits `g`, after its served store already holds A's retraction. B's new declaration gets lamport 2.
  - The served-store fold still shows `g` retracted. B's reboot appends 0, because its registry says `g` is live and unchanged, so nothing ever re-asserts it.
- **Impact:**
  - Latent today. The served-store fold's only reader is `declared_exchange`, a file cannot author an `exchange` binding, and `bindings_of` has no production caller.
  - It becomes real with the first store in which two nodes load one app (Phase 4's route), and for any client that follows the attach notes and folds the family "the same way".
- **Required correction:** either
  - scope the three claims to a single registry, and record the multi-origin order and scope as an open Phase 4 question in the slice profile or OpenNotes. The options are a merged clock (the maximum over the served store's binding family at boot), putting the origin into the retraction's scope, or both;
  - or implement the merged clock now.
- **Closure test:** a two-origin fold test that pins whichever rule is chosen. For the merged clock: B's declaration, made after B has seen A's retraction, is live in the merged fold. For the scoped claims: today's multi-origin outcome, pinned under a name that does not call it revival.
- **Not architectural as remediated** (text plus an open note). The multi-origin clock itself is an architectural question that belongs to Phase 4.

### [STA-P3-2] Renaming an app's `app` line orphans its declarations; deleting a line later brings back the stale one, and the page describes neither

- **Location:**
  - `glade/node/src/registry.rs:540-552`: `live()` takes, per glade id, the newest *live* declaration across apps. When this app's declaration is retracted, another app's older declaration stands in.
  - `appdecl.rs:488`: the scope is only the app the file names now.
  - `AppFileFormat.md:266-274`: "folded by glade id and the newest is the live one"; "A deleted line retracts its surface".
  - Nothing says what renaming the `app` line does.
- **Invariant violated:** the page's two rules. Also a missing half of a lifecycle pair: loading a file creates an app's declarations, but no documented step retires them once no file names that app.
- **Reproduction** (probe `rename`, one registry):
  1. `app notes` declares `n.list value` and `n.old log`.
  2. The same file, renamed `app notes2`, drops `n.old` and makes `n.list` a `log`. Live: `notes2/n.list log`, `notes/n.old log`. `n.old` stays declared.
  3. `n.list` is deleted from `notes2`: +1 retraction. Live: `notes/n.list value`, `notes/n.old log`. The deleted surface is live again, with its pre-rename shape.
  4. A file `app notes` with no bindings: +2. Live: nothing. This is the only way out, and it is not documented.
- **Impact:**
  - The fold answers "declared, as a `value`" for a surface its author deleted.
  - Surfaces dropped from a renamed app stay declared for ever.
  - Any reader of the fold — GC-4's retention question, a client fold — gets that answer.
  - No production reader regresses, because before the amendment every deleted line stayed declared. The defect is that the page now promises otherwise.
- **Required correction:** on `AppFileFormat.md` and in the attach notes, say that:
  - the fold is per `(app, glade_id)`, and the newest live declaration across apps stands for a glade id;
  - renaming the `app` line leaves the old name's declarations live;
  - an app is retired by loading once a file that names it and has no binding lines.

  Alternatively, refuse a registration that would orphan an app.
- **Closure test:** a registry test of the rename-then-delete sequence asserting the documented outcome, and a test that the retirement file retracts every declaration of the old app.
- **Not architectural.**

### [STA-P3-3] The documented TypeScript regeneration still writes through pnpm's hard links; only build.py's writes were fixed

- **Location:**
  - `glade-decl-ts/README.md:25-34`, the "Regenerate" block:
    - `cp /tmp/g/typescript/*.ts ../glade-decl-ts/src/` (`:32`);
    - `cp ir/glade_decl.ir.json corpus/decl.v1.json ../glade-decl-ts/src/` (`:33`), a line this object edited at `37d9b7d`.
  - §4.3 of the reconciliation adopts these READMEs as the canonical regeneration procedure.
  - Compare `glade-decl/corpus/build.py:436` `write_artifact` (`784840a`, committed one minute after `37d9b7d`) and `corpus/test_build.py:193` `WriteReplacesTest`.
- **Invariant violated:** `784840a`'s own rule. A write into a rendering file in place "would also rewrite every consumer's installed copy"; the build "must replace a file, never write through it".
- **Reproduction:**
  - `stat -f %l glade-decl-ts/src/*` gives:
    - 4 links for `api.ts`, `index.ts`, `corpus.test.ts`, `decl.v1.json`, `glade_decl.ir.json`;
    - 8 links for `cbor.ts`, `codec.ts`, `ext.ts`, `schema.ts`, `taut_client.ts`, `package.json`.
  - `find … -inum` finds:
    - the runtime files as the same inodes inside the installs of grip-core, grip-react, glade/demo, glial, grip-react-demo and glade-chat;
    - `api.ts` inside grip-core, glade/demo and glial.
  - BSD `cp` onto an existing file writes the existing inode. In the scratch demo the hard-linked copy changes under `cp`, and is left alone by build.py's `os.replace`.
- **Impact:**
  - The next regeneration done by the README rewrites `@owebeeone/glade-decl` inside those consumers' `node_modules` with no install, no lockfile change and no git trace. This is the same event that changed seven installs at Step 2.1.
  - Three of those installs still hold contract v0: `grip-react`, `grip-react-demo` and `glade-chat` have `AdvertisementRecord`, `decl.v0.json` and `CONTRACT_VERSION` `99a04e0`, and share only their runtime files with the source. A write-through there makes a mixed-version install that no install step produced.
  - No bytes differ today, because the runtime files did not change between v0 and v1. The exposure is the next contract or taut change.
  - The gryth-wz checkout that the running dev server resolves has link count 1, so it is not exposed.
- **Required correction:** make the README's copy steps replace instead of writing through (for example `rm -f` each destination before `cp`, or copy to a temporary name and `mv`), or route the TypeScript copies through `write_artifact`. Say why in the README.
- **Closure test:** run the README's copy steps against a scratch `src/` that holds a hard-linked "installed" copy, and assert that the installed copy is unchanged and the source file's link count is 1. This extends `WriteReplacesTest` to the documented procedure.
- **Not architectural.**

## 2. Riders

- **SAF-P3-13 — CLOSED, from durable state's side.**
  - `binding_census.rs:147-156` names its units: one registry per file, 15 as their sum, and "no single store appends 15". `row9_the_owners_two_file_store_appends_7` asserts one store's share.
  - I checked this independently through the real pre-amendment code, not the test's simulation of it (probe `census`). One BlobStore per file, filled by 559cb2c's parse+register from the base files, then the amended code on the files at the tuple:
    - `grazel/apps/grazel-app.glade` +5 (6 unchanged);
    - `glade/apps/grazel-app.glade` +5 (6);
    - `grazel/apps/gyld-app.glade` +2 (9);
    - the glade-gyld fixture +2 (7);
    - the glade-gwz fixture +1 (3).
    - Total 15. Every store's existing records are a byte-identical prefix, and no file warns.
  - The owner's two-file store (probe `upgrade` B1): +5 then +2 = 7, byte-identical prefix, second boot +0.
  - The derivation assumes a store holding one registration of the base files. For grazel's two files that holds: their git history (grazel `56d9a32`..`c66f029`) has no deleted line and one shape change (`ws.files` log → swmr), which the fold already supersedes.
- **CON-P3-11 — CLOSED, from state's side.**
  - Nothing implements option (s): no `ServiceRetraction` or `WorkspaceRetraction` kind exists, and `Record::Retract` wraps only `BindingRetraction`.
  - `service`, `seed` and `workspace` keep the plain `(glade_id, payload)` diff (`appdecl.rs:506-533`).
  - `AppFileFormat.md` states that deleting a `service` or `workspace` line retracts nothing.
  - With (s) declined, no §4.7 row would gate anything. The formal closure belongs to the Consistency axis.
- **Surface residual — CLOSED.** `glade/docs/AppFileFormat.md` exists (299 lines). It is linked from `GladeGrazelAttachNotes.md:28` and `docs/README.md:7`, and carries the grammar, the tail, the meaning of each token, and the change and delete rules. STA-P2-1, STA-P2-2 and STA-P3-2 correct three of its statements.

## 3. Invariant analysis (attacks that failed)

1. **First boot on an existing store.**
   - Exactly the derivation's records append: 15 over the census, 7 for grazel + gyld.
   - Every existing record stays byte-identical in records.json.
   - The only meaning change is intended: the `from-cursor` and `windowed` records are superseded.
   - The served store's journal is append-only, and the seed only adds records.
   - `sysdata.BindingDecl` is unchanged; the diff adds only `BindingRetraction`, and the tail reaches no record.
   - Observation, not a finding: in a store whose history holds lines deleted or reverted under the old diff, the first boot also retracts the deleted lines and re-appends the reverted ones. Both converge to the file, which is R9(a)'s end state. The page's first-start paragraph mentions only the `from-cursor` re-registration.
2. **Crash points.**
   - Kill after `register`'s appends but before the save (probes B2, G1): records.json is unchanged. The next boot appends the same records, byte-identical to an uninterrupted run, and the third boot appends 0.
   - Kill between two apps' saves (B3, G2): the first app is persisted, and the next boot finishes the second, byte-identically.
   - No retraction is duplicated or lost, because `declared_by` excludes retracted keys.
   - `BlobStore::save` is tmp+rename, which is atomic against a process kill.
3. **Rollback.**
   - The old node on the migrated store with v0 files appends nothing, and the amended node appends nothing on return (B4).
   - A pre-2.3 node meeting the v1 files is refused at line 16 before any `register` (B5). This is fail-closed.
4. **The fold as a state machine inside one registry.**
   - Newest wins; a newest retraction takes a surface down; a later declaration revives it, because `register` re-declares at max+1.
   - The tie rule — a retraction outranks a declaration with the same `(lamport, origin)` — is reachable only across a rollback, where the old node writes lamport = seq. The next amended boot re-asserts the file.
   - The fold is order-independent (reversed op order and a reload give the same live set) and the lamport strictly increases across both streams. The unit tests and the probes agree.
   - A file left out of `--app` retracts nothing (row 10).
   - A surface declared by two apps stays stable across boots. For the page's wording of that case, see STA-P3-2.
5. **The `V1_TOKEN_CHECKS_REFUSE` flip.**
   - In the flipped copy all 41 appdecl tests pass, so the flip really is one line.
   - A v1 file's bad token refuses that file before any of its records are written.
   - Files earlier in the `--app` list stay registered. The loop has no preflight (L1-14), which is harmless while registration is per app and idempotent — except under STA-P2-1.
   - The refusal is announced on the page (`AppFileFormat.md:191`, `:252`), not in the warning line itself; that wording is for the Surface axis.
6. **`v0` files never refused.** Holds for every token except one containing `=` (STA-P2-2). The `crdt`, `atom` and extra-token cases were already refused by the old node.
7. **Header landing order.**
   - The binary at `glade/node/target/debug/glade-node` is the dddf8b8 build.
   - grazel's default `--node-bin` (`grazel/src/lib.rs:160`) and `gyld-ui.py` (which only checks that the binary exists) start whatever binary is at that path and never rebuild it. grazel's integration tests do rebuild it.
   - A stale pre-2.3 binary would refuse the v1 files at line 16, and grazel would exit with the node's stderr tail. That is fail-closed, and nothing is written to the store for that file.
8. **gwz locks.** Both workspaces' locks match every member HEAD. The gryth-wz fast-forward under the running Vite server changes no durable state: `BindingDecl` bytes and glial's `instanceKey` are unchanged.
9. **glade-decl.** `build.py --check` passes, and build.py's own writes replace their files.
10. **Pre-existing hazards the new stream inherits, not attributed to this object:**
    - `BlobStore::save` does not fsync the file or its directory.
    - `store.rs`'s `append_to_log` can leave a torn tail that later appends write past.
    - The legacy codec panics on a malformed payload, and HOME accepts unauthenticated `Ops` frames. A malformed op on `dir.binding-retractions` therefore breaks `declared_exchange` exactly as one on `dir.bindings` already could.

## 4. Risks and next action

- **Blocking:** STA-P2-1 and STA-P2-2. Both are bounded code-and-text fixes with the closure tests above; LBT-007 requires those tests to be written first.
- **Non-blocking:** STA-P3-1..3 can ride the same patch.
- **Outside this axis, noted for the owners:**
  - The grip-react, grip-react-demo and glade-chat installs still hold contract v0. Those repositories are not in §4.5's consumer list, and a reinstall would switch them silently; consumer parity belongs to the Code and Consistency axes and to PackageExtractionPlan step 1.6.
  - A declared `shape-profile` is validated and then dropped, so no consumer receives it. This is the recorded deferral from the plan's Phase 2 preamble.
- **Next action:** one remediation revision on STA-P2-1 and STA-P2-2 (and the P3s if cheap), then a State re-verdict on its diff.
