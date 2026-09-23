# GladeDeclAmendment — SURFACE-AXIS REVIEW

**Review object:** the glade-decl contract v1 amendment as landed: plan Steps 2.1–2.8 of `glade-wz/dev-docs/GladeFirstSlicePlan.md` Phase 2, at glade-wz root `cd6a498bd2c1`. Status: landed. The publish (the second half of Step 2.9) is waiting on this review. Reviewed 2026-09-23.
**Baseline:** read at the start and again at the end; nothing moved.
- glade-wz root `cd6a498bd2c1`
- glade-decl `784840acfb05`, glade-decl-rs `9507c1233c0a`, glade-decl-ts `37d9b7d8a680`, glade-decl-py `433215acc4dc`
- glade `dddf8b89cad7`, grazel `b604a06970fe`, glade-gyld `c9ef7a66b6e4`, glade-gwz `fc0bb990ef09`, glial `4c6e8561c174`, grip-core `97ff6c26f12e`
- gryth-wz root `3c64e7a59064`, gryth-ui `9323818a39d2`, gryth-wz/glade-decl-ts `37d9b7d8a680`, gryth-wz/glial `4c6e8561c174`

How the sources were read:
- Every page and app file read, and the two node source files the grep reads, showed no working-tree change against their member's HEAD (`git status --porcelain` on those paths, at start and at end).
- Pages were read with Read, cat, sed and grep.
- The node's messages were read only through the one permitted grep.
- `git log` and `git diff` were run only on the permitted pages, to tell the amendment's text from older text. Commit subjects of the object's ranges were read with the `log --oneline` commands the brief lists.
- `cmp` was run on the two grazel-app.glade copies.

**Date:** 2026-09-23
**Axis:** Surface — the interface as an author meets it: the user-facing pages, the app files and their in-file grammar, and the node's message texts. No code and no design or plan document was read. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: GO** — 0 P0, 0 P1, 0 P2, 8 P3. One P3 (SUR-P3-5) is classified ARCHITECTURAL. No finding needs a compatibility break to fix after release. Each one is closed by page text, a message text, a warning, or (SUR-P3-5) a recorded design decision.

---

## 0. Evidence base

**Read in full**, with line counts at the tuple:
- `glade/docs/AppFileFormat.md` (299 lines). This is the user-facing format page, new in 7bd5f9d and edited through dddf8b8.
- `glade/docs/README.md` (10) and `glade/README.md` (39).
- `glade/dev-docs/GladeGrazelAttachNotes.md`, its format section only (L25-81). That section holds the published grammar block (L31-39) and the tail, validation and record paragraphs.
- `dev-docs/glade/GladeDeclSurface.md` (161), plus the amendment's diff to it (112d30a).
- `glade-decl/README.md` (241), plus its diff `d671f10..784840a`.
- The three rendering READMEs (42, 36 and 39 lines), plus the regeneration commits' diffs to them.
- `grazel/README.md` (155) and `glade-gwz/README.md` (76).
- `glade-gyld/README.md` L1-30 and L1150-1192. These are its only app-file passages; grep found them.
- The five app files:
  - `grazel/apps/grazel-app.glade` (57), byte-identical to `glade/apps/grazel-app.glade` (checked with `cmp`)
  - `grazel/apps/gyld-app.glade` (72)
  - `glade-gyld/tests/fixtures/gyld-test-app.glade` (30)
  - `glade-gwz/tests/fixtures/gwz-test-app.glade` (21)
  - The amendment's diffs to the three authored files.
- The node's message texts, through the one permitted grep over `glade/node/src/appdecl.rs` and `appdecl/tail.rs`. That gave about 130 distinct strings: production templates, plus test inputs and expectations.

**Not read, by mandate:**
- Any code.
- `GladeDeclReconciliation.md`, `GladeFirstSlicePlan.md`, `DecisionLog.md`, `PackageExtractionPlan.md`, `LibraryBoundaryAndTestingPolicy.md` and `AgentProcessRules.md`.
- `dev-docs/examples/` and `GladeHandoff-260710.md`. These are part of the object but not on this axis's source list.
- The rest of the attach notes, and the glial and grip-share pages.

What that costs this review:
- The rulings R1–R11 are known here only as the pages cite them.
- Rider CON-P3-11's option (s) cannot be identified.

**Limits of the message evidence.** The grep prints only strings that contain its keywords. Three things are not visible:
- the prefix the node puts on a refusal (the page says it names the file);
- the refusal for a file with no `app` line;
- the registration and retraction log lines (grazel's README quotes `1 unchanged`).

A message that contains none of the keywords would be missed.

## 1. Findings

### [SUR-P3-1] The page never says how the glade-id fold and the app-scoped retraction interact across app names and files

**Location**
- `glade/docs/AppFileFormat.md`:
  - L67-68 and L73-74: uniqueness is stated per file only.
  - L264-278: binding records are "folded by glade id and the newest is the live one"; a deleted line "retracts its surface, for its own app only"; "A surface declared by another app file is never touched"; "A file that is not loaded retracts nothing".
- The attach notes, L63-64: the refusal of a duplicate glade id is justified by GQ-6 ("frozen-once-shared"), which is a global rule, but it is enforced per file.
- Node texts `` line {line}: duplicate glade id `{id}` `` and `` line {n}: duplicate `app` declaration ``. Both carry a line number, so both are per-file. No cross-file check shows up in the grep output.

**Invariant.** Every line an author edits, the `app` line included, has a stated effect. The uniqueness rules the fold and the retraction depend on (which names must be unique, and where) are stated.

**Reproduction.** Worked out from the page's rules, not from code.
1. **Rename an app.** A file with `app notes` declares `notes.x`. Change the line to `app notes2` and restart. Later, delete `notes.x` from the file and restart.
   - The page says the deletion retracts only the `notes2` declaration. No loaded file names `notes`, so the old declaration is never retracted.
   - One reading: the old declaration becomes live again, so the surface stays declared, with its pre-rename content, after its line was deleted.
   - The other reading: the fold is purely newest-by-glade-id. But then case 3 below breaks the promise that another app's surface is "never touched".
   - The "Changing or deleting a line" section mentions neither renaming the `app` line nor retiring an app.
2. **Two files with one app name.** `a.glade` and `b.glade` both say `app notes`, with different bindings, loaded with `--app a.glade --app b.glade`.
   - The node "registers each file in turn" (`grazel/README.md` L73-74).
   - Retraction is keyed to "the app that file names", and records carry the app name, not the file (page L67-68).
   - Read literally, each file then retracts the other's bindings at every start.
   - Nothing on the page forbids this setup, and no message in the grep output refuses it.
3. **One glade id in two apps.** `app a` declares `shared.x` as `value`; `app b` declares it as `log`.
   - Both files load cleanly.
   - Which declaration is live depends on registration history ("the newest"), not on anything in the files.
   - Deleting a's line then either leaves b's older declaration live, or retracts the surface for b as well.

**Impact.**
- An author cannot predict whether a surface is declared after renaming an app, after splitting an app across two files, or when two apps pick the same id.
- Glade ids are authored and unconstrained (`glade-decl/README.md` L36-44), so two apps choosing the same id is a matter of time.
- All three cases are silent: nothing warns.
- If the node behaves as the page reads, case 2 retracts declarations at every start.

**Remedy.** State three things on the page:
- (a) An app name may be declared by one file per node. Alternatively, files naming one app are merged before the difference is taken.
- (b) Which declaration is live when two apps declare one id and one of them retracts it.
- (c) How to rename or retire an app. For example: empty the file's declarations under the old name, start once, then rename or drop the file.

Back (a) and (b) with a node refusal or warning that names both files or both apps.

**Closure test.** Three node integration tests, plus a check that the page states the three rules:
- Two files with one app name, two boots: both files' surfaces are declared, or the node refuses with a message naming both files.
- Two apps declaring one id: the stated precedence holds, and a warning names both apps.
- Rename, then delete the line: the surface ends up as the page says.

**Classification: not ARCHITECTURAL** on this axis's evidence. The page says retraction is scoped by app, so the design appears to intend declarations of one id by several apps; the gap is stating the precedence and the preconditions. Reclassify it as ARCHITECTURAL if R9 (b2+a) does not define which declaration is live after a scoped retraction.

### [SUR-P3-2] The v0 → v1 transition isn't tied to any named release, and the warnings that carry it don't say they will become refusals

**Location**
- `AppFileFormat.md`:
  - L64-66: a v0 file "still loads".
  - L188-196 and L246-256: "In this release the report is a warning … From the next release it refuses the file", and "A `glade-app v0` file is never refused for its zone/retention".
  - L229-230: "A bare `ttl` is still accepted".
- Node texts:
  - `` unknown zone `{zone}` (one of {ZONES:?}) ``
  - `` unknown retention `{retention}` (one of {RETENTIONS:?}) ``
  - `` unknown retention `windowed` (removed; use `from-cursor`) ``
  - `` retention `from_cursor` is the contract's spelling (use `from-cursor` in a file) ``
  - `` empty file: expected `glade-app v0` or `glade-app v1` header ``
  - `` line {line}: expected `glade-app v0` or `glade-app v1` header, got `{found}` ``

**Invariant.** A file format people edit states its deprecation schedule in terms a third party can act on. Every warning that will become a refusal says so where the operator sees it.

**Reproduction.**
- A third party with a `glade-app v1` file containing `binding x log share commons windowed` starts today's node and gets `` x.glade: warning: line N: unknown retention `windowed` (removed; use `from-cursor`) ``. The node starts. Nothing in that line says it will become fatal.
- The page says "the next release" but names neither this release nor that one. The operator cannot tell whether the node they are about to install is "the next release".
- A third party with a v0 file reads "still loads", which suggests v0 support will end. Next to it they read "never refused for its zone", which suggests it won't.
- They also cannot tell whether a v0 file may use the tail or `crdt`. The page says only that v0 loads "as it always did".
- A bare `ttl` is "still accepted". No warning for it appears among the messages, and no future for it is stated.
- An author whose file has no header gets an error that offers `glade-app v0` first, as an equal choice.

**Impact.**
- The rulings chose one release of warnings as the migration mechanism, and it is invisible where it counts. An operator who does not read the page upgrades and gets a node that refuses to start, with no earlier notice in the warnings.
- A third party cannot choose between staying on v0 and migrating.
- The page's wording goes stale the day the next release ships.

**Remedy.**
- On the page, name the release that turns each warning into a refusal, or state a version policy ("the first node release after X").
- Add that notice to the v1 zone and retention warning texts, for example "…; refused from glade-node <ver>".
- State whether v0 is supported indefinitely, and which v1 features (the tail, `crdt`) a v0 file may use.
- State whether bare `ttl` is permanent, and what it means. If it is being retired, warn now.
- Make the header error say "write `glade-app v1`", with v0 accepted only for old files.

**Closure test.**
- Message tests: each v1 zone and retention warning carries the refusal notice, and the header error names `glade-app v1` as the header to write.
- A page check: "next release" is replaced by a named release, and v0's lifetime and feature set and bare `ttl`'s status are stated.

**Classification: not ARCHITECTURAL.**

### [SUR-P3-3] What the binding tail does is described three incompatible ways, and a `crdt` author is never told how the profile reaches glial

**Location**
- `AppFileFormat.md` L131-133: "The node checks the tail and does not record it yet … nothing downstream learns a duration or a profile from the node today."
- `glade-decl/README.md`:
  - L55-56: "An app file writes it as the binding line's `shape-profile=<profile>` tail key".
  - L132: "the duration cannot be stated until the node parses R11(a)'s `ttl=<duration>` tail key".
- `GladeDeclSurface.md` L30 and L32 carry the same two sentences. This is the page the format page's L296-298 sends readers to for the vocabulary.
- Node text: `` line 3: shape `crdt` needs `shape-profile=text_crdt` (glial cannot mount a crdt surface without its profile) ``.

**Invariant.** Pages and messages describe the behaviour that actually landed, and a required token's effect is described the same way everywhere.

**Reproduction.**
1. An author writing a collaborative document enters `binding notes.body crdt share commons from-cursor`. The node refuses it and says glial needs the profile.
2. They add `shape-profile=text_crdt`. The node accepts the line.
3. The contract front page says the file has now written a `ShapeProfileDecl`. The user page says nothing downstream learns it.

- The contract front page and `GladeDeclSurface.md` also say a duration "cannot be stated until the node parses" `ttl=`. The node now does parse it, and refuses a malformed one.
- No page says where a glial `crdt` mount gets its profile today. The glial commit subject in this object says the declared profile is read "ahead of config.crdtProfile" — a client-side setting that no author page mentions.

**Impact.**
- Glial and grip developers read the contract pages. Those pages describe a pipeline — the file writes a ShapeProfileDecl, glial looks it up — whose middle link does not exist in this object.
- A client developer who trusts them and relies on the declared profile gets a crdt mount with no declared profile.
- A first-day author declares a `crdt` surface and cannot find the next step.
- The refusal's stated reason supports the same wrong reading.

**Remedy.** Either:
- make the node register `ShapeProfileDecl(glade_id, profile)` for every binding that carries a profile. It is a separate record kind, so no `BindingDecl` byte moves and the plan's constraint holds; or
- correct `glade-decl/README.md` L55-56 and L132, `GladeDeclSurface.md` L30 and L32, and the refusal's parenthetical to match what landed, and say on the page what a `crdt` surface needs today on the client side.

**Closure test.** Either:
- a node test that a `crdt … shape-profile=text_crdt` line yields a `ShapeProfileDecl` record that glial's lookup returns; or
- a doc check that no page claims an app file's tail reaches a record, plus a page statement of where a crdt mount gets its profile.

**Classification: not ARCHITECTURAL.**

### [SUR-P3-4] `seed` has a declare half but no documented retract half, and grants from deleted seed lines carry into enforcement

**Location.** `AppFileFormat.md`:
- L142-148: "The node records grants but does not enforce them yet".
- L289-290: "Deleting a `seed` line does not withdraw the grant it made; revoking the grant does".

**Invariant.** Declare and retract are documented together for each directive.

**Reproduction.**
1. An author adds `seed guest ws-notes notes.*` to try something, then deletes the line.
2. The page says the grant stays until it is revoked.
3. No page — the format page, grazel's, or glade's — says how to revoke: which command, tool or record to use, or who may do it.

**Impact.**
- On the day enforcement lands, every grant a seed ever made becomes an enforced permission. That includes grants from lines the author deleted.
- The author has no documented way to withdraw them before then.

**Remedy.**
- Next to `seed` on the page, state the revocation procedure: the tool or record, and who may write it.
- State that grants from deleted seed lines stay live until revoked.
- Optionally, have the node list at start the grants whose seed line no longer appears in any loaded file.

**Closure test.**
- A page check that `seed` and revocation are documented together.
- A node test that the documented revocation, once recorded, wins over a re-loaded seed.

**Classification: not ARCHITECTURAL.** Revocation exists in the design; what is missing is the author's route to it.

### [SUR-P3-5] `service` and `workspace` have no retract at all, and the page doesn't say what deleting a `workspace` line does to the node's claim

**Location.** `AppFileFormat.md`:
- L150-155: a `workspace` line makes the node claim the share "while it runs".
- L285-289: "Deleting a `service` or a `workspace` line retracts nothing: a retired exchange stays declared and so stays routable, and the workspace's registered entry stays".

**Invariant.** Every declare has a retract, or the page says there is none on purpose and what to do instead.

**Reproduction.**
- Retire an exchange by deleting `service notes notes.ops`. It stays declared and routable with no provider behind it, and the page gives no step that undeclares it.
- Delete `workspace ws-notes notes`. The page says the entry stays, but not whether this node still claims `ws-notes` (the claim is renewed "while it runs"). The author cannot tell whether the share still routes to this node.
- Editing either line (a new display name, a new exchange id) amounts to a delete plus an add, so old entries and exchanges pile up.

**Impact.**
- Retired exchanges stay routable forever, and clients reach a node that has no provider for them.
- After a `workspace` line is deleted, which node hosts the share cannot be predicted.

**Remedy.** Record a decision on retiring services and workspaces. The options are a mechanism (deletion retracts, as it does for bindings, or an explicit operation), or "none in v1" with the reason. Document the decision on the page, together with what happens to the node's claim after deletion.

**Closure test.**
- Page text for both directives' retract halves.
- If a mechanism lands, a node test that a retired exchange is unroutable and that the node stops claiming a removed share.

**Classification: ARCHITECTURAL.** The missing operation is in the design, not the page: the object's scoped retraction covers `binding` lines only.

### [SUR-P3-6] The page doesn't define what `service` and `seed` operands refer to, and the six examples disagree on them

**Location**
- `AppFileFormat.md`:
  - L135-148: `service <name> …` is "answered by the service `<name>`"; `seed <principal> <share> <verb…>` has verbs "such as `read.*`", with no vocabulary.
  - L20: a principal is "such as `owner`".
  - L35-37: the page's own example.
- `grazel/apps/grazel-app.glade` L43 and L48-49 (and its twin in `glade/apps/`).
- `grazel/apps/gyld-app.glade` L57 and L62-66.
- `glade-gyld/tests/fixtures/gyld-test-app.glade` L12 and L27.
- `glade-gwz/tests/fixtures/gwz-test-app.glade` L12 and L18.

**Invariant.** Every token has a stated meaning, and the shipped examples the page points to agree with the page.

**Reproduction.**
- **`service <name>`** has three different meanings across the six examples:
  - the app name in the page (`notes`) and in grazel-app (`grazel`);
  - the supplier binary in gyld-app (`glade-gyld`, whose app is `gyld`);
  - neither in the fixtures (`gyld` in app `gyld-test`; `gwz` in app `gwz-test`).
- **The seed's `<share>`** has two conventions:
  - The workspace share, in the page (`ws-notes`) and in gyld-app, whose comment says "The share named here is the workspace share these surfaces live on".
  - A share named after the app or supplier: `grazel` in grazel-app.glade, and `gyld` and `gwz` in the fixtures. grazel-app.glade is the file the page calls "a longer example" (L44-45). No file declares any of these three as a workspace.
- **Verbs** mix an action pattern (`read.*`) with patterns named after surfaces (`gyld.*`, which "covers every surface above"). No page lists the verbs or says what `read.*` matches.
- **Principals:** no page says which principals exist or whether `owner` is special.

**Impact.**
- An author copying grazel-app.glade seeds grants on a share named after the app. By gyld-app's own comment, that is not where the surfaces live.
- Seeds become permanent grant records (SUR-P3-4), so those grants land in the store now and are wrong the day enforcement starts.
- Whether `<name>` plays any part in routing (the page states none) is left to guesswork.

**Remedy.**
- Define on the page what `service <name>` refers to (or say it is a label with no effect on routing), what a seed's share must be, and the verb vocabulary (or link to it) and principal naming.
- Make the shipped files follow those definitions: correct or explain `seed owner grazel …` in both grazel-app.glade copies.

**Closure test.**
- A page check for the four definitions.
- A lint (a node warning) for a seed whose share no loaded `workspace` line declares, with the shipped files passing it cleanly.

**Classification: not ARCHITECTURAL.**

### [SUR-P3-7] `external` is accepted with no warning, although the contract calls it not yet authorable and requires a source

**Location**
- `AppFileFormat.md` L96: "`external` is accepted, but the file cannot name the source yet".
- `glade-decl/README.md` L47-51 ("`source` is set iff `authority == external`, and both are declared, not yet authorable") and L105 ("parses, but no token names the source").
- `GladeDeclSurface.md` L31.
- No message about `external` appears in the grep output beyond `` unknown authority `{}` (one of {AUTHORITIES:?}) ``, and the page describes no warning for it.

**Invariant.** A declaration the contract calls not yet authorable is either refused or warned as incomplete, and its future is stated.

**Reproduction.**
- `binding feed.cache value external commons latest` loads silently.
- It registers a binding whose authority is `external` with no source — a record the contract's "iff" says cannot exist.
- Neither page says what that declaration means today (nothing caches anything), or what happens to such lines once a source token arrives.

**Impact.**
- Files written now take on a silent debt.
- When a source becomes authorable, one of two things must happen, and the choice is made after release: these lines start failing, which is a break, or the contract's invariant is loosened.

**Remedy.**
- Warn on `external` through the existing v1 warning channel, for example "external has no source until …; declares nothing a node acts on".
- State on the page what an `external` binding without a source means now, and what will happen to it.

**Closure test.**
- A message test for the `external` warning.
- A page check that its future is stated.

**Classification: not ARCHITECTURAL.**

### [SUR-P3-8] The package front pages don't tell their readers about the v1 amendment

**Location**
- `glade-decl-rs/README.md` L3-8.
- `glade-decl-ts/README.md` L3-7 and L14-23.
- `glade-decl-py/README.md` L3-6 and L13-26.
- `glade/README.md` L17 and L27-36.
- `grazel/README.md` L21-42 and L71-79.

**Invariant.** A consumer's front page says:
- how to install the package;
- what the current version contains;
- what was removed and why;
- what is declared but not yet authorable.

The format's main consumer points to the format page.

**Reproduction.**
- **Rendering READMEs.**
  - The amendment's regeneration commits replaced `AdvertisementRecord` with `ShapeProfileDecl` in each type list. None notes that `AdvertisementRecord` was removed (R7(b): held out until GDL-029). A consumer whose code names it finds it gone with no explanation.
  - None says how a consumer depends on the package. The TS README never gives its package name, and the Rust README never says "v1".
  - None says that `source`/`external` and the domain anchor are declared but not yet authorable.
- **`glade/README.md`** still says the repo "currently contains scaffolding only", with a Phase 1 focus, and does not point to `docs/AppFileFormat.md`.
- **`grazel/README.md`.** Its `--app` and `--gyld-app` flags are how most app files reach a node. Yet:
  - It does not point to the format page.
  - It does not mention the move to the v1 header.
  - It does not say where the node's app-file warnings appear while grazel supervises the node. It says only that grazel prints "the tail of the node's stderr" when the node exits, which leaves open whether a running node's warnings are shown at all.

**Impact.**
- Consumers of the renderings meet a breaking removal with no notice.
- A grazel operator with a custom `--app` file may never see the one release of warnings (see SUR-P3-2).

**Remedy.**
- Add a short "contract v1" block to each rendering README: the version, how to depend on the package, the removed `AdvertisementRecord` and why, and the declared-but-not-authorable members. A link to the contract README's section would also do.
- Update `glade/README.md`'s status line and link the docs page.
- In `grazel/README.md`, link the format page and say where node warnings go.

**Closure test.**
- A doc check (grep) for each of these items on each page.
- If grazel buffers the node's stderr, a grazel test that an app-file warning reaches grazel's own output.

**Classification: not ARCHITECTURAL.**

## 2. Riders

**SAF-P3-13 (units of the appended count) — closed from the author's side; its test half is outside this axis.**
- The only author-facing statement is `AppFileFormat.md` L279-283, and it counts per node: "on a node whose records were written before it did this, each binding that says `from-cursor` is registered once more on the first start".
- No page quotes a tree-wide number. grazel's README quotes the node's per-store count (`1 unchanged`).
- Whether Step 2.5's census test names its units is code, and not visible here.

**CON-P3-11 (R9 option (s) has no §4.7 row) — nothing on the surface points to it; this axis cannot close it.**
- The pages and messages describe exactly four behaviours:
  - bindings folded by glade id;
  - app-scoped retraction of deleted binding lines;
  - no retraction for files that are not loaded;
  - no retraction for service, workspace and seed lines.
- Nothing implies a fifth mode.
- This axis cannot identify (s), which lives in the design document, so it cannot confirm "nothing implements it".
- One pointer for the owner: SUR-P3-1's unhandled cases (renaming or retiring an app, two files naming one app) are where an author runs past the edge of the retraction design. If (s) was the option that covered them, re-check "nothing now needs the row" against that finding.

**Surface residual (the grammar had no user-facing page) — closed.**
- `glade/docs/AppFileFormat.md` exists and is listed in `glade/docs/README.md`.
- The grammar comment in all three authored app files points to it: `grazel-app.glade` L19-25 (in both copies) and `gyld-app.glade` L17-21. The attach notes point to it too (L27-29).
- It covers every directive, the tail, zone, retention, spelling, and edits.
- It agrees with the node's message texts at every point checked (§3) except one: the reason the crdt refusal gives (SUR-P3-3).
- The test fixtures carry no grammar comment, on purpose (attach notes L50-54).
- The page's remaining gaps are filed as SUR-P3-1 to SUR-P3-7; how hard it is to find from the front pages is filed as SUR-P3-8.

## 3. Invariant analysis

### Walkthrough

The app: a setting, an append log, a `crdt` document and an expiring cache, written from the page alone:

```text
glade-app v1
app pantry
binding pantry.settings value share commons latest
binding pantry.events   log   share commons from-cursor
binding pantry.doc      crdt  share commons from-cursor shape-profile=text_crdt
binding pantry.cache    value share commons ttl ttl=15m
seed owner ws-pantry read.*,pantry.*      # guessed (SUR-P3-6)
workspace ws-pantry pantry                 # needed for routing (L150-155)
```

It is started with `glade-node --profile local --app pantry.glade` (L42-43).

For each likely mistake, what the node would say (from the message texts), and whether the page agrees:

| Mistake | What the node says (from the texts) | Page agrees? |
| --- | --- | --- |
| Token missing, no tail | Refused, and it prints only the template: `` line N: `binding <glade_id> <shape> <authority> <zone> <retention> [ttl=<duration>] [shape-profile=<profile>]` ``. It never says "expected 5". | Yes, L177-182 |
| Token missing, with tail | Refused: `` `ttl=15m` is a key=value entry where <retention> goes (the tail follows all five tokens) ``. It names where the tail entry landed, not which token is missing. | Yes, L122-123 |
| `windowed` (v1 file) | Warning `` unknown retention `windowed` (removed; use `from-cursor`) ``; stored as `windowed`; the node starts. | Yes, L246-252, but no refusal notice (SUR-P3-2) |
| `from_cursor` (v1 file) | Warning `` retention `from_cursor` is the contract's spelling (use `from-cursor` in a file) ``; stored as `from_cursor`. | Yes |
| Unknown zone (v1 file) | Warning `` unknown zone `shared` (one of ["commons", "private"]) ``; stored as written. | Yes, L188-192 |
| `crdt` without a profile | Refused: `` shape `crdt` needs `shape-profile=text_crdt` (glial cannot mount …) `` | The refusal, yes. Its reason contradicts L131-133 (SUR-P3-3). |
| Bad duration (`1h30m`, `0s`, `10`) | Refused: `` bad duration `…` (`ttl=` takes a whole number above zero and one unit …) ``. A value that is too large gets `` duration `…` is out of range ``, without the range. | Yes, L108. The page doesn't state the range either (minor). |
| `ttl=` on a `latest` line | Refused: `` `ttl=` needs the retention `ttl` (this line's is `latest`) `` | Yes |
| `glade-app v0` header | Warning `` line N: header `glade-app v0` names the old language; write `glade-app v1` ``, plus v0-form warnings (`` … is not in `glade-app v1` … ``). | Yes, L64-66, L192-196, L252-256 |
| Wrong profile, or wrong key spelling | `` profile `snapshot_delta` is over `swmr`, not `crdt` ``; `` unknown binding key `shape_profile` (binding keys: `ttl`, `shape-profile`) ``; `` repeated binding key `ttl` `` | Yes |
| A shape that cannot be bound | `` unsupported binding shape `exchange` (…; exchange uses `service`) ``; `` … (recognised and reserved, not bindable; …) `` | Yes |
| No header, or a misspelt one | `` expected `glade-app v0` or `glade-app v1` header, got `…` ``, listing v0 first (SUR-P3-2). | Partly |

### Edits

- **Changing a binding line:** stated (L266-269).
- **Changing only a tail value:** the record does not change, because the tail is not recorded (L131-133), so nothing new is registered. The page's "a changed line replaces the surface's declaration" does not say this, but it can be worked out.
- **Deleting a binding line:** stated (L270-274).
- **Changing the `app` line:** not covered (SUR-P3-1).
- **Deleting a service, workspace or seed line:** stated as "retracts nothing" (SUR-P3-4 and SUR-P3-5).
- **Removing the file from `--app`:** stated (L275-278), with grazel's gyld leg as the worked case.

### Where I had to guess, beyond the findings

- Whether `--app` is ignored when there is no `--profile`. L42 says the node loads the files "when it starts with a profile".
- What paths are relative to.
- That a workspace display name must be a single token. Only L69 implies it.
- What "the plain engine" is (L109).
- What `GDL-041` means (L84). It is a design id on a public page.
- Whether the hyphen rule (L80-84) binds names the author chooses (shares, app names, principals) or only keywords.
  - The node enforces it only on keywords, and the glade-id row says "any single token".
  - Nothing follows from it either way, so it is not filed.
- That a binding names no share at all. This is said only in the app-file comment ("no share/key here — the ServeClaim selects the node") and in the attach notes, not on the page.

### Attacks that failed (the invariant holds)

- **Refusals show what was expected.** Every token's refusal carries the template or the list of valid values. The one exception is `` unknown declaration `…` ``, which does not list the four directives. That is harmless: the page's grammar block is one line away.
- **Every warning names its fix:** header, zone, retention, `windowed` and `from_cursor`, in both the v1 and v0 forms.
- **Every tail key's absence has a stated meaning.**
  - `ttl=` missing on a `ttl` line "names no duration"; its status is in SUR-P3-2.
  - `shape-profile` missing is refused on `crdt`, means "the plain engine" on `swmr`, and is simply absent on `value` and `log`.
  - Because the page states what `swmr` without a profile means, a later default cannot silently change it.
- **No binding token has a default, and a missing token never slides silently into the next slot.** Every such shift either puts a `key=value` entry in a positional slot or leaves too few tokens, and both are refused. The message texts confirm both paths.
- **Names read cold.**
  - `shape-profile=` and `ttl=` say what they carry.
  - The units are listed, and "90m, not 1h30m" shows that `m` means minutes.
  - `from-cursor` in a file versus `from_cursor` in the contract and the record is explained on the page (L237-240), in the contract README (L125-126) and in both warnings.
  - The underscores in profile values are stated as the one exception.
  - `glade-app v1` versus `v0` is explained, apart from its schedule (SUR-P3-2).
- **Declared-but-not-authorable items are named**, apart from SUR-P3-3 and SUR-P3-7:
  - on the contract front page: source and external, the domain anchor, `canonical_key`, `derive_glade_id`, `AdvertisementRecord`;
  - on the format page: `external`.
- **The rulings' two notes are carried through.** The file writes the hyphen only (L237-240). `canonical_key`'s owner is Gianni (glade-decl README L152-154, GladeDeclSurface L35).
- **The plan's constraint holds on the page.** "No new field on `sysdata.BindingDecl`" matches the page's "the registered record holds the five tokens only".
- **The shipped files are clean under v1.** Every zone is `commons` or `private` and every retention is `latest` or `from-cursor`. The two grazel-app.glade copies are byte-identical, as grazel's README and the attach notes say.
- **The `private` caveat is stated.** Glial mounts converge `private` surfaces in commons; both the format page and the contract front page say so.
  - The shipped `ws.diff … private` line predates the object.
  - The page's example uses `private` for a cursor. Its caveat is headed "for confidentiality" but also states the practical consequence: the surface lands in commons, "where everyone shares it".

### Noted and not filed

- The glade-decl-ts README says `pnpm install` against an npm lockfile. That is the deferred §4.8 question.
- `atom` is called "reserved" in the node text and on the page, but "a GDL-041 engine … recognised, not bindable" in the contract README. This is wording only.
- `GladeDeclSurface.md`'s Status line still reads "working draft", while the contract README calls the page ratified and amended.
- The page says the template is "the one message that shows the tail". The unknown-key messages also list the keys.

## 4. Risks and next action

**Risk (not a finding): the comment rule may collide with a future source token.**
- v1 ships two rules together: `#` starts a comment anywhere (L61-63), and a `key=value` value is a single token.
- The declared-but-not-authorable `source` is likely to be a URL, and a URL may contain `#` or need spaces.
- Admitting such values later by changing the comment rule would change how existing files parse.
- Decide the source token's syntax before more comes to rely on the comment rule.

**Risk: SUR-P3-1 case 2 is predicted from the page, not observed.** If the node really takes the difference file by file under one app name, that is a correctness defect well above P3. The lane owner should run the closure test and confirm before the publish.

**Next action.**
- **Publish:** GO.
- **Before the first outside author or consumer meets the pages:** close SUR-P3-2, SUR-P3-3 and SUR-P3-8. They are text and message fixes.
- **Before a second app is loaded beside grazel's:** settle SUR-P3-1's rules and SUR-P3-5's decision.
- **At publish time:** re-verify the tuple.
