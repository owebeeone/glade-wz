# GladeDeclReconciliation (revision 2) — SURFACE-AXIS REVIEW, ROUND 2

**Review object:** `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclReconciliation.md`, **revision 2**, committed at glade-wz root `1defe3b` and unchanged since (`git log 1defe3b..HEAD -- dev-docs/glade/GladeDeclReconciliation.md` empty, verified at start and end). 1372 lines. DRAFT design, unimplemented. The working-tree file is the object; the root working tree is clean.

**Baseline:** glade-wz root `90265076425e` · glade `559cb2c87e85` · glade-decl `d671f10c13e6` · glade-decl-rs `21eefa1c3a53` · glade-decl-ts `7e16e324630a` · glade-decl-py `1b0f6d1f7886` · taut `7a5f616c3a9f` · glial `0dfe4b930063` · grazel `c66f029ad060` · glade-gyld `65da8cb7e2b7` · glade-gwz `e53c87dddb8f` · glade-chat `9238d21f6a36` · glade-discover `fd94a1f87bbc` · grip-core `97ff6c26f12e` · taut-shape `9a752094dbed` · gryth-ui `9323818a39d2` · gwz-dev `7b3f1bc723d6`. **Verified identical at start and at end of this review.** Every tree clean except glade-discover's five out-of-scope dev-docs files. Round-1 baseline for the diff: root `b132b7e`.

Sources read: the object's §2m (`:356-372`), §3 R8–R11 (`:669-796`), §4.4 (`:1053-1188`), §4.6 (`:1227-1252`), §4.7 rows 5–10 (`:1262-1267`), and the Revision 2 closure map (`:24-46`); my axis's round-1 report and `-RemPlan.md`; the five app files (`glade/apps/grazel-app.glade` verified byte-identical to `grazel/apps/grazel-app.glade` by `diff` rather than opened separately); `glade-decl/README.md`, `grazel/README.md`, `glade-gwz/README.md`, `glade/README.md`, `glade/docs/README.md`, `glade/dev-docs/GladeGrazelAttachNotes.md` and `dev-docs/glade/GladeDeclSurface.md` in full; `glade-gyld/README.md` by targeted grep plus three line windows. One permitted grep over `glade/node/src/appdecl.rs` for quoted diagnostics. The read-only drift gate run once. **I read no Rust, TypeScript or Python source, and no design or plan document beyond the above.** Two scope departures are disclosed in §0.

**Date:** 2026-09-22

**Axis:** SURFACE — the `<app>.glade` file format as the person typing and editing it meets it: cold-read naming, the legal values of each token, lifecycle pairs, stated defaults, and the first-day walkthrough from the published pages alone. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: NO-GO** — 0 P0, 0 P1, **4 P2** (1 carried open, 3 new), **6 P3** (2 carried open, 4 new). Revision 2 is a large and real improvement: three of my five round-1 blockers are genuinely closed, the `crdt` dead end is gone, and R9(b) alone removes 13 of the 15 mandated edits. Every remaining blocker is bounded and text-fixable inside the object. *I pre-commit to GO on a revision that resolves SUR-P2-1, SUR-P2-6, SUR-P2-7 and SUR-P2-8 as specified below.*

---

## Prior-finding closure table

One row per round-1 finding of this axis. "Verified" means the original counterexample was re-traced against revision 2 and the round-2 tuple by this reviewer, not that the RemPlan claims a disposition.

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| **SUR-P2-1** — `zone` enforced while undocumented; the file's own comment says the mount fills it | ACCEPT: state meaning of `commons`/`private`, rule for choosing, behaviour when absent, mount-override; rewrite the contradicting sentence in both homes of `grazel-app.glade` and `GladeGrazelAttachNotes.md:49` | §4.4 `:1075-1098` delivers all five statements, and the mount-override answer is concrete (`manifestScope` reads `decl.zone ?? spec?.zone ?? ""`, so the declared zone wins). **But I re-ran the RemPlan's own closure test — "no page tells an author the mount fills a token the parser requires them to type" — across the full round-2 page set and it FAILS: `dev-docs/glade/GladeDeclSurface.md:29-30` says it too, and that is the document both rewritten homes cite by name.** Not in the rewrite list | **OPEN** |
| **SUR-P2-2** — `glade-app v0` names two incompatible languages | ACCEPT as R10: (a) `v1` validated, `v0` warned; (b) redefine `v0` explicitly + migration note; recommend (a) | §2m A10 (`:369`) states the defect; R10 (`:740-765`) gives both options with Node / App-files / Third-party / **Docs** columns, recommends (a), and prices the real cost honestly — the warning channel `parse()` does not have. Re-traced: a third-party `glade-app v0` file carrying `from-cursor` loads-with-warning under (a) or breaks-at-boot-as-stated under (b). A10's substantive claim (no CHANGELOG/migration/release note in the four app-file repos or `glade-decl`) re-verified true at the round-2 tuple | **CLOSED** |
| **SUR-P2-3** — `ttl` in the frozen set with no duration slot; position 6 spent by `profile` | ACCEPT as R11: (a) keyword tail; (b) hold `ttl` reserved; recommend (a). Name `Retention` as a third declaration-only gap | §2m A11 (`:370`) + R11 (`:767-796`). Closure test re-traced: under (a), `ttl=10m` and `ttl=1h` are different records and a further option is a new key, changing no existing line; under (b) `ttl` is unauthorable; under (c) `ttl` does not exist. The hole is closed under **every** option. R11(b) `:781` names the gap list explicitly ("a **fourth** declaration-only gap beside `external`, `source` and `Retention`'s parameters") | **CLOSED** (discoverability residue filed separately as SUR-P2-8) |
| **SUR-P2-4** — the sixth token without its rules | ACCEPT: legal values, per-shape omission semantics, parse enforcement, `:113` updated, non-colliding name | §4.4 bullet 4 (`:1125-1141`) delivers all five. Re-traced my exact counterexample `binding doc.body crdt share commons latest`: it is now **refused at parse** ("`crdt` — omission is **refused at parse**… a file that validates into an unmountable surface is the defect"). Legal values named (`snapshot_delta`, `text_crdt`) and independently confirmed published at `GladeDeclSurface.md:27`. R11's naming note (`:791-796`) spells the key `shape-profile=` and cites both collisions | **CLOSED** |
| **SUR-P2-5** — a write with no documented change or retract half | ACCEPT, merged into R9; the format documentation gains the rule for a changed line and a deleted line, before any migration | §2m A9 (`:368`) + R9 (`:698-738`) + §4.4 bullet 5 (`:1142-1147`) + §4.6 bullet 4 (`:1245-1252`) + §4.7 rows 9–10. Change half: three options, each with a stated behaviour; R9 `:736-738` requires it in a page a user reads **before** any file is migrated. Delete half: R9(a) supplies it and the prose is explicit that (c) "leaves [deletion] permanently unanswered"; the recommendation includes (a). §4.6 retracts revision 1's false "fields nothing reads" | **CLOSED** (option-scope residue filed as SUR-P2-7) |
| **SUR-P3-1** — no page names a legal value of `shape`, `zone` or `retention` | Rider: a members table with a gloss per enum value on the contract's front page, and the grammar on a page presented as user-facing | Vocabulary grep re-run over the round-2 page set (8 pages, +2 vs round 1, incl. the grown `glade-gyld/README.md`). **Shape, Authority and Zone are now discoverable** — `GladeDeclSurface.md:27,28,29` names every member. **Retention is named nowhere**: `GladeDeclSurface.md` has no `Retention` row at all; `ttl`/`from-cursor`/`from_cursor`/`windowed` score 0 hits across all 8 pages; all 10 `latest` hits are `latest.json`/"latest build". `glade-decl/README.md:24-25` is unchanged — enums named, members never listed — and §4.4 asks for no members table. The user-facing-page half is one unowned clause ("should move to, or be mirrored in") with no page named, no §4 step and no §4.7 row | **REOPENED-AS-SUR-P2-6** (validated-token half) / **OPEN** (remainder) |
| **SUR-P3-2** — refusals must name the replacement | Rider: recognised-but-refused with a diagnostic in the shape of `appdecl.rs:121` | §4.4 bullet 7 (`:1161-1168`) states it verbatim with the exact message ``unknown retention `windowed` (removed; use `from_cursor`)``, and §4.7 row 7 gates it for `from-cursor`, `windowed` and an unknown zone. Re-traced: `binding term.log log share commons windowed` now yields a message naming `from_cursor` | **CLOSED** (spelling residue filed as SUR-P3-9) |
| **SUR-P3-3** — `from_cursor` would be the format's only underscore | Rider: folded into R9's recommendation to keep the hyphen | Re-verified at the round-2 tuple: **zero underscores in any non-comment line of any of the five app files.** R9(b) `:720` keeps the hyphen, R9's prose `:727-728` states the reason, R11's naming note `:795` applies it to `shape-profile=` | **CLOSED** |
| **SUR-P3-4** — the format's only spec documents a line the format refuses | Rider: correct `GladeGrazelAttachNotes.md:98`; redirect or reserve `atom`/`message`/`window` | §4.4 bullet 8 (`:1169-1175`) corrects `:98` and explains the fold subtlety; §4.4 bullet 1 (`:1109-1115`) handles the recognised-but-unauthorable tier ("Say so, or give them a reserved note"). Re-read `GladeGrazelAttachNotes.md:96-98` — the wrong sentence is still there, as expected for a draft, and the object now requires its correction | **CLOSED** |
| **SUR-P3-5** — glade ids derived vs hand-typed | Rider: say on the front page whether an app-file glade id is authored or derived | Counterexample re-traced on the page: `glade-decl/README.md:28-30` ("derived from package id + grip key… frozen once shared") and `:48-50` ("deferred to the implementation step (N4)") are **unchanged** at the round-2 tuple, against 14 hand-typed ids. The object's claimed closure is **§4.2 step 6, which is outside this axis's permitted read list** — I did not open it and make no claim about it | **OPEN** (closure site not verifiable on this axis) |

---

## Changed-range analysis

`git -C /Users/owebeeone/limbo/glade-wz diff b132b7e..1defe3b -- dev-docs/glade/GladeDeclReconciliation.md` → **+1056 / −303**, 619 → 1372 lines. Restricted to the sections on this axis's read list:

| Section | Revision 1 | Revision 2 | Assessment |
|---|---|---|---|
| Closure map (`:13-46`) | absent | new: 20 rows mapping each round-1 finding to a site | Within disposition. Every SUR row points at a section that exists. |
| §2m (`:356-372`) | A1–A8 | A1–A8 restated + **A9, A10, A11, A12 new** | Within disposition. A9/A10/A11 are the three new SUR-derived rulings' evidence rows; A12 is CON-P2-5's. A2 flipped from "red today" to **CLOSED**, which I verified independently (below). |
| §3 R8 (`:669-696`) | 24 lines | 28 lines: gains the "Which document is controlling (CON-P2-5)" paragraph and a byte-cost for option (a) | Within disposition. |
| §3 **R9, R10, R11** (`:698-796`) | absent | 99 lines, three new rulings with option tables | Within disposition (SAF-P1-2+SUR-P2-5 → R9; SUR-P2-2 → R10; SUR-P2-3+SUR-P2-4 → R11). **R11's table is the only one of the four with no Docs column** — see SUR-P2-8. |
| §4.4 (`:1053-1188`) | 22 lines, 5 bullets | 136 lines: a new **Landing order** paragraph, a new **"What must be written before validation is turned on"** block, 11 bullets, a census command, a two-repo gate | The largest single change and the axis's main remediation. Within disposition, with three residues filed below. Revision 1's app-file arithmetic ("13 lines across 5 files") is corrected to a census that I re-ran and **confirmed exactly**: 28 binding lines, 13 `from-cursor`, 13 `latest`, 2 `windowed`. |
| §4.6 (`:1227-1252`) | 17 lines | 26 lines: the running-demo bullet becomes "a *constraint on the rulings*, not a description of them", and revision 1's "fields nothing reads" is explicitly retracted as false | Within disposition, and the retraction is the correct one. |
| §4.7 rows 5–10 (`:1262-1267`) | one `cargo test -p glade-node` row | six rows: py-from-a-wheel, `glade-node`, the SUR-P3-2 diagnostic test, `cargo test -p grazel`, and two new R9 register/retract tests | Within disposition. Row 10's retraction test is **single-file only** — see SUR-P2-7. |

**Changes outside the RemPlan's dispositions:** none found in my sections. Every edit I can attribute maps to an accepted disposition.

**NEW root causes found in this round:** four, all filed below (SUR-P2-6, SUR-P2-7, SUR-P2-8, plus the four P3s).

**NEW ARCHITECTURAL root causes: NONE.** I considered SUR-P2-7 (R9(a)'s retraction scope) and SUR-P2-8 (the published grammar block) carefully against that bar and reject the classification for both. Neither is a property of the system's structure that the object can only report; both are missing sentences in an option's specification, fixable by text inside the object with no change to any design. SUR-P2-6 is the same: the vocabulary exists and is frozen in `glade_decl.taut.py`; what is missing is a requirement to write it down. The architectural facts in play (no fold for `dir.bindings`; glial not producing the private key) were classified in round 1 as properties of the described system, and revision 2 now states them truthfully. **The two-round cap is not triggered by anything in this report.**

**Independently verified facts underlying the A2 ruling** (the ruling's outcome is the owner's; its facts are mine): `cd glade-decl && /opt/homebrew/bin/python3 corpus/build.py --check` → `all 3 glade-decl artifacts in lockstep with the schema.`, **exit 0**; `git status --short` **empty** in both `glade-decl` and `glade-decl-rs` after the run, so the `--check` path wrote nothing. The standing precondition is met.

---

## 0. Evidence base

**The authored corpus, recounted at the round-2 tuple.** Exactly **five** files carry the `glade-app v0` header (`grep -rln 'glade-app v0' --include='*.glade' .`), in four repositories, holding **28** binding lines:

| position | token | values authored, anywhere | distribution |
|---|---|---|---|
| 1 `<glade_id>` | hand-typed dotted word | `ws.tree` `ws.files` `ws.diff` `term.log` `gwz.output` `chat.msgs` `chat.groups` `gyld.streams` `gyld.stream` `gyld.decisions` `gyld.lens` `gyld.file` `gyld.output` `gyld.ask` | 14 distinct |
| 2 `<shape>` | bare word | `value` `swmr` `log` | 13 / 2 / 13 |
| 3 `<authority>` | bare word | `share` | 28 / 28 |
| 4 `<zone>` | bare word | `commons` `private` | 26 / 2 (both `private` are `ws.diff`, in the two byte-identical `grazel-app.glade` copies) |
| 5 `<retention>` | bare word | `latest` `from-cursor` `windowed` | 13 / 13 / 2 |

The object's §4.4 census command reproduces exactly. Revision 1 and my own round-1 report both had this arithmetic wrong (round 1 said "22 binding lines"); **revision 2 is right and round 1 was not**, and I record that correction against my own prior work.

**Vocabulary coverage, re-run over the round-2 page set** (8 pages: the six of round 1 plus `glade/docs/README.md` and `dev-docs/glade/GladeDeclSurface.md`; `glade-gyld/README.md` grew 981 → 1192 lines since round 1 and was re-swept):

| token | hits, correct sense | where |
|---|---|---|
| `value` `atom` `log` `stream` `swmr` `crdt` `snapshot_delta` `text_crdt` `message` `window` `exchange` | **yes** | `GladeDeclSurface.md:27`, one table cell |
| `share`, `external` | **yes** | `GladeDeclSurface.md:28` |
| `commons`, `private` | **yes** | `GladeDeclSurface.md:29` (`glade-gyld/README.md:802,889` are "private or loopback address" — unrelated) |
| `latest`, `from-cursor`, `from_cursor`, `windowed`, `ttl` | **zero, all eight pages** | `latest` scores 10 hits, every one `latest.json` or "the latest build" |
| `shape-profile` | **zero** | — |

`GladeDeclSurface.md` has **no `Retention` row** in its Contents table; `retention` appears only inside `BindingDecl`'s tuple at `:30`, in the grammar placeholder at `GladeGrazelAttachNotes.md:32`, as "rides as STRINGS" at `:51`, and at `:145` where it is explicitly deferred ("retention/timeout policy per declaration is a decl-surface question"). `glade-decl/README.md:24-25` names `RetentionPolicy` and `Retention` and no member of either.

**The round-1 caveat is now discharged.** `glade/docs/README.md` is 8 lines, titled "Glade Public Docs", "Status: placeholder", and contains no vocabulary. It also carries a standing rule that bears directly on §4.4's remedy: *"Do not move internal architecture drafts here until they are intended to become public promises."* Every round-1 finding survives its contents, as predicted.

**Node diagnostics (permitted grep, `grep -n 'line {' glade/node/src/appdecl.rs`).** Unchanged from round 1: `:113` still prints the five-token template `` `binding <glade_id> <shape> <authority> <zone> <retention>` ``; `:121` still carries the good precedent ``unsupported binding shape `{}` (implemented: …; exchange uses `service`)``; there is still no zone or retention diagnostic.

**A correction to my own round-1 report.** Round 1 recorded as "held" that *"All five files carry `# binding <glade_id> <shape> <authority> <zone> <retention>` immediately above their binding block."* **That is false.** Only three do — `grazel/apps/grazel-app.glade:19`, `glade/apps/grazel-app.glade:19` and `grazel/apps/gyld-app.glade:17`. `glade-gyld/tests/fixtures/gyld-test-app.glade` and `glade-gwz/tests/fixtures/gwz-test-app.glade` carry **no grammar comment and no token names at all** (verified by grep over their comment lines). The in-file grammar is a 3-of-5 property, not a 5-of-5 one.

**Two scope departures, disclosed.** (1) A `find . -name '*.glade'` returned sixteen files. To answer whether the other eleven are the same format I opened the first 8 lines of four of them under `dev-docs/examples/`. They are not on my permitted read list. Finding SUR-P3-6 does not depend on their contents beyond the fact — established independently by the permitted `grep -rln 'glade-app v0'` — that they are not `glade-app v0` files. (2) A `git grep -n 'dev-docs/examples' -- '*.md'` surfaced one line each of `PackageExtractionPlan.md` and `StackMap.md`. I did not open either document and cite neither as authority.

---

## 1. Findings

### [SUR-P2-1 — still open] The "the mount fills domain/zone/key" sentence has a third home, and it is the document the other two cite by name

**Root cause.** §4.4's rewrite list for the zone contradiction enumerates two homes of the sentence and omits the source both of them point at.

**Location.** Object §4.4 `:1092-1098`. Against `dev-docs/glade/GladeDeclSurface.md:29-30`; `grazel/apps/grazel-app.glade:21`; `glade/dev-docs/GladeGrazelAttachNotes.md:50`.

**Violated invariant.** The RemPlan's own closure test for SUR-P2-1: *"No page tells an author the mount fills a token the parser requires them to type."*

**What a user meets.** §4.4 `:1097-1098` says: *"Rewrite it in **both** homes of `grazel-app.glade` and in `GladeGrazelAttachNotes.md:49`."* Three files, one statement. But the statement in `grazel-app.glade:21` is *"the mount fills domain/zone/key (**GladeDeclSurface.md**)"* and the one at `GladeGrazelAttachNotes.md:50` is *"the mount fills domain/zone/key (**GladeDeclSurface**)"*. An author who does what a citation invites — opens the cited document — lands on `dev-docs/glade/GladeDeclSurface.md`, where `:30` reads *"each **mount** creates a binding *instance* `(decl, domain/zone/key fill)`"* and `:29` reads *"The binder's scope maps them at bind time."* After §4.4's specified rewrite the two derivative statements are gone and the authoritative one remains, still telling the author the mount supplies the zone the parser now forces them to type.

**Impact.** The correction does not survive one hop of the citation graph an author is explicitly invited to follow. It is worse than leaving all three, because the two pages that would have warned the author that this claim is contested are precisely the two being silenced. And the object has the answer already — §4.4 `:1087-1091` establishes that on the grip-share path the declared zone wins and on the glial path the question does not arise — so this is a routing failure, not a knowledge gap. `GladeDeclSurface.md` is also the one document R8 and §4.2 step 5 already open for amendment, so the edit costs nothing extra.

**Required correction.** Add `dev-docs/glade/GladeDeclSurface.md:29-30` to §4.4's rewrite list, carrying the same answer §4.4 `:1087-1091` already states: the authored zone is the author's to choose and the mount does not override it on the path that honours it.

**Closure test.** Grep the full page set for "fills domain/zone/key", "zone… fill" and "maps them at bind time"; every surviving occurrence states that the author writes the zone and says what a mount does with it. A reader who follows the citation in `grazel-app.glade` reaches a page that agrees with the parser.

---

### [SUR-P2-6] Token 5 is switched from ignored to enforced in the same bullet as token 4, and only token 4 gets its meanings written

**Root cause.** §4.4's "What must be written before validation is turned on" block covers the zone token exclusively; the retention token is validated by the same bullet with no documentation requirement anywhere in the object.

**Location.** Object §4.4 `:1075-1098` (the block) against §4.4 bullet 2 `:1116-1120` (the validation). Closure map `:35` and `:45` both point SUR-P2-1 and SUR-P3-1 at this one block.

**Violated invariant.** The invariant the owner accepted for token 4 in round 1, applied to token 5: *a token the author must write correctly or the node refuses to boot must be documented as the author's to choose, with a stated meaning per value.*

**What a user writes and meets.** §4.4 bullet 2 reads: *"validate token 4 against `{commons, private}` (row 31b, R1 sub-choice) **and token 5 against the R2 policy set** (row 18b, R2 sub-choice), with the same line-numbered diagnostics."* Two tokens, one sentence, one wave. The block immediately above it states what `commons` means, what `private` means, the rule for choosing, the behaviour when absent, and whether a mount overrides — five statements, all about token 4. There is no corresponding requirement for `latest`, `from_cursor` or `ttl`.

A first-day author writing a new log binding must pick one of three:

```
binding my.out log share commons ???
```

Across all eight permitted pages, `latest`, `from-cursor`, `from_cursor`, `windowed` and `ttl` have **zero** hits in the retention sense. `GladeDeclSurface.md` — which does gloss Shape, Authority and Zone — has no `Retention` row. The nearest published statement is `GladeGrazelAttachNotes.md:145`, which says the question is deferred.

The object knows exactly what this costs. §4.4 bullet 7 `:1165-1168` states that a reader meeting `["latest","from_cursor","ttl"]` *"reaches for `latest` (which silently converts an append log into a last-writer-wins value) or `ttl` (which parses and leaves the duration unsayable)"*. Bullet 7's remedy is a redirect on **removed** tokens — it fires for `windowed` and `from-cursor` and never for a new author, who types a legal token and gets no diagnostic at all.

**Impact.** This is the token the entire amendment migrates, on the only 15 lines that change. It becomes a validated, frozen, boot-refusing enum whose members mean nothing on any published page, and whose wrong choice is **silent** — no arity error, no unknown-token error, just a log surface that folds last-writer-wins. Zone, by comparison, has two values, one of which is authored twice in 28 lines. The amendment documents the cheap token and validates the expensive one. Once frozen, the spelling can be taught by a diagnostic; the meaning cannot, and it is the meaning that decides whether `term.log` keeps its history.

**Required correction.** Extend §4.4's "What must be written before validation is turned on" block to token 5, symmetrically with token 4: what `latest` means, what `from_cursor` means, what `ttl` means, which to write for an append log versus a snapshot value, and — under R11(a) — how the duration is expressed. Give `dev-docs/glade/GladeDeclSurface.md` a `Retention` row beside its `Shape`, `Authority` and `Domain`/`Zone` rows, and `glade-decl/README.md` the members table the RemPlan's SUR-P3-1 disposition already called for.

**Closure test.** Grepping the user-facing page set for each of `latest`, `from_cursor`, `ttl` returns a hit with a one-line gloss in the retention sense. A reader who has seen only the pages can state which retention to write for a terminal-scrollback log and which for a settings value, without copying a neighbour.

---

### [SUR-P2-7] R9(a)'s retraction rule has no stated scope, and the node's documented default boot loads two app files

**Root cause.** R9(a) specifies retraction as a diff of "the parsed file" against "the fold" and never says what subset of the fold is in scope; the two readings the wording permits differ by seven declared surfaces on the deployment `grazel/README.md` documents.

**Location.** Object §3 R9 option (a), `:719`; §4.4 bullet 5 `:1142-1147`; §4.7 row 10 `:1267`. Against `grazel/README.md:13-17`, `:41-42`, `:71-79`; `grazel/apps/grazel-app.glade:22-34`; `grazel/apps/gyld-app.glade:42-48,64-68`.

**Violated invariant.** The remove half of a lifecycle pair must state what it removes. An option offered in a draft must be safe as written — the deferral covers R9's outcome, not the completeness of its options.

**What a user meets.** R9(a) reads, in full on this point: *"`register` diffs the parsed file against the fold to know what to retract."* Nothing in R9, §4.4 bullet 5 or §4.7 row 10 scopes that diff.

The node accepts `--app` repeatedly. `grazel/README.md:73-79`: *"`glade-node` has always accepted `--app FILE.glade` **more than once** — it accumulates the flag and registers each file in turn."* When grazel's gyld leg is on it passes **two** files, and the two files overlap deliberately: both declare `workspace ws-razel razel`, and `gyld-app.glade:64-68` says so in terms (*"Registration is idempotent by diff, so loading both files registers this entry once"*), which `grazel/README.md:78-79` confirms (*"the node logs the second as `1 unchanged`"*).

Now apply R9(a) literally on that boot. `grazel-app.glade` registers its 7 bindings. `gyld-app.glade` is then parsed and diffed against the fold; grazel's 7 bindings are in the fold and absent from this file, so they are retracted. `ws.tree`, `ws.files`, `ws.diff`, `term.log`, `gwz.output`, `chat.msgs`, `chat.groups` disappear — during a normal boot, with no file edited by anyone.

The second reading is safe: scope the diff to the `app` named in the file being registered. The scoping key already exists — the object itself notes at §4.4 bullet 9 that `sysdata.BindingDecl` carries an `app` field. R9(a) simply does not say to use it, and the option is priced at "~200 LOC" with no mention of the question.

The flag path is a second instance of the same hole even under the safe reading. The gyld leg is **default off** (`grazel/README.md:41-42`: *"The gyld leg is **default off**: `--gyld-supplier-bin` is its switch"*), and switching it off is what stops `--gyld-app` being passed at all. If a later boot's retraction scope is "everything this registrant declared" rather than "everything this app declared", turning a supplier off retracts its seven surfaces. That collides head-on with the files' stated purpose: `gyld-app.glade:19-20` says the surfaces are *"Pre-declared here so they exist node-side whether or not a supplier is running"*, and `grazel/README.md:15-17` says the same of `chat.msgs`/`chat.groups`, whose supplier grazel never runs. Declare-ahead, provider-optional is the documented design; absence-implies-retraction is its negation.

**Impact.** The recommended combination is "(b) for the spellings, plus (a) for what remains", so R9(a) is the path the object steers toward, and it is the half of the lifecycle pair my axis exists to check. As drafted it can retract live surfaces on an unmodified boot. §4.7 row 10 would not catch it: it tests *"a deleted binding line retracts its surface"* — one file, one line, one registry.

**Required correction.** State R9(a)'s retraction scope in the option itself: retraction is computed per `(app, glade_id)` against the declarations previously registered for the `app` named in the file being registered, so a file that is not loaded on a given boot retracts nothing, and a declaration made by a different app file is never in scope. Say in §4.4 bullet 5 that a surface declared by an app file that is not loaded stays declared. Price the option against that rule.

**Closure test.** A `glade-node` test registers `grazel-app.glade` then `gyld-app.glade` into one registry and asserts all 7 of grazel's bindings and all 7 of gyld's are live and none retracted; a second test registers `grazel-app.glade` alone on the next boot and asserts gyld's 7 are untouched; a third deletes one line from `gyld-app.glade`, reloads both, and asserts exactly that one surface is retracted.

---

### [SUR-P2-8] R10(a) and R11(a) both change the line people type, and nothing in the amendment updates the grammar the author copies from

**Root cause.** The format's published grammar block and the in-file grammar comments are the author's only copy-sources, and no ruling column, §4.4 bullet or §4.7 row updates either when the grammar changes.

**Location.** Object §3 R10 `:752-755` (Docs column) and §3 R11 `:778-782` (option table, **no Docs column**); §4.4 bullets 3 and 4 `:1121-1141`. Against `glade/dev-docs/GladeGrazelAttachNotes.md:29-36`; `grazel/apps/gyld-app.glade:17`; `grazel/apps/grazel-app.glade:19` and its byte-identical twin; `appdecl.rs:113`.

**Violated invariant.** When a grammar changes, the published grammar changes with it. A token an author cannot see in any grammar and cannot provoke a diagnostic for does not exist to that author — a diagnosability defect under this review's P2 tier.

**What a user meets.** The format has exactly two places that show an author the shape of a binding line:

1. `GladeGrazelAttachNotes.md:29-36` — the published grammar block, opening `glade-app v0` (`:30`) and `binding <glade_id> <shape> <authority> <zone> <retention>` (`:32`).
2. The in-file comment, present in **3 of 5** app files: `grazel/apps/grazel-app.glade:19`, its twin, and `grazel/apps/gyld-app.glade:17`.

**Under R10(a)** the validated grammar becomes `glade-app v1`. R10's Docs column says only *"a migration note, which these repos do not have yet"*; §4.4 bullet 3 changes the parser. Neither updates `GladeGrazelAttachNotes.md:30`. The one published grammar then instructs every new author to write the deprecated header — which under R10(a) loads with a deprecation warning, so the format's own specification is a generator of warnings.

**Under R11(a)** the line becomes `binding <id> <shape> <authority> <zone> <retention> [k=v …]`. R11's option table has columns Node / glade-decl / App files / What it leaves open and **no Docs column at all** — alone among R8–R11. Its "App files: **0 edits** — no existing line has a tail" is true of the data and silent about the comments. §4.4 bullet 4 requires *"The arity diagnostic at `appdecl.rs:113` updated: it prints the five-token template today, so it does not reveal that a sixth thing exists"* — and that is the only update named.

The diagnostic remedy does not reach the author who needs it. R11(a) itself says *"the arity check at `:111-114` becomes a minimum"*. A legal five-token line therefore never trips arity, so `:113`'s newly-corrected template never prints for the author who wrote a valid line and now wants a TTL. `:121`-style refusals fire only on a wrong key, which presupposes knowing keys exist. The single mitigation SUR-P3-1 relied on — that after validation the vocabulary becomes discoverable by failing — is unavailable for the tail, because the tail's absence is not a failure.

So under the object's own recommendations the author's path to `ttl=10m` is: not in the published grammar, not in the in-file comment, not in any diagnostic they can provoke.

**Impact.** SUR-P2-3's accepted closure test — *"An app file can express a TTL of 10 minutes and one of 1 hour"* — is met by the parser and not by any author. The same holds for `shape-profile=text_crdt`, which §4.4 bullet 4 makes **mandatory** for every `crdt` binding: `crdt` is the amendment's headline addition, its profile key is required at parse, and the key's syntax appears in no grammar an author reads. Round 1 recorded the in-file grammar comment as *"the best thing about this surface"*; R11(a) makes it wrong in three files and R10(a) makes it wrong in three files, and neither ruling costs a line for it.

**Required correction.** Give R11 a **Docs** column, as R10, R9 and R1 have. In it, and in §4.4, name the artefacts that change with the grammar: `GladeGrazelAttachNotes.md:30` (the header token, under R10) and `:32` (the tail, under R11); the in-file comment at `grazel/apps/grazel-app.glade:19`, `glade/apps/grazel-app.glade:19` and `grazel/apps/gyld-app.glade:17`; and — since `:113`'s template is now unreachable for a valid line — one sentence stating how an author is expected to learn the tail exists.

**Closure test.** After the amendment, every published grammar and every in-file grammar comment shows the same line the parser accepts, including the header token and the tail. An author who has read only those and never triggered a parse error can write `ttl=10m` and `shape-profile=text_crdt` correctly at the first attempt.

---

### [SUR-P3-6] Sixteen `.glade` files exist in the workzone; five are this format and eleven are a different language

**Root cause.** One file extension names two mutually unintelligible declaration languages, and the amendment that freezes one of them does not say which `.glade` it froze.

**Location.** `grep -rln 'glade-app v0' --include='*.glade' .` → exactly the five app files. `find . -name '*.glade'` → sixteen. The other eleven are under root `dev-docs/examples/` (see the §0 scope disclosure). Against object §2m A3 `:362`, which applies this exact "second answer to a frozen question" treatment to `glade/decl/*` and not to these.

**Violated invariant.** A cold reader must be able to tell, from a file listing or an editor tab, which language a file is written in.

**What a user meets.** The frozen format is line-oriented, `#`-commented, five positional bare tokens, header `glade-app v0`. The eleven others are brace-nested with `//` and `/* */` comments, quoted string ids, and `package … version "0.1.0" { … }` / `application … { }` blocks composing a package graph. They share nothing but the extension. `glade/README.md:24` sends readers to `dev-docs/` for design, and eleven of the sixteen `.glade` files there are the other language, unbannered — where A3 requires exactly such a banner for the node's superseded schema sketch. The refusal an author gets for feeding one to `--app` names the header (`line 1: expected \`glade-app v0\` header, got …`) but not the existence of a second dialect, so they cannot tell a wrong path from a wrong language.

**Impact.** Bounded: the header check rejects the wrong dialect at line 1 with a line number, so nothing loads silently, and `--app` takes explicit paths with no globbing. But this is the review that freezes the extension's meaning, and `GladeDeclSurface.md:123` makes a promise the other language visibly does not keep — *"`.glade` is data; it never becomes a compiler front-end"*. P3 rather than P2 because a first-non-blank-line discriminator remains an additive fix after release; a reviewer weighting "what ships forever" more heavily would rank it P2, and I flag that reading.

**Required correction.** One sentence in the amendment stating which `.glade` dialect is frozen; and either a banner on the eleven example files naming them a different, unimplemented language, or a distinct extension for them — the A3 treatment, applied to the file format rather than the schema.

**Closure test.** Every `.glade` file in both workzones either parses as `glade-app v0` or carries a banner naming the language it is written in, and one page says which dialect the frozen format is.

---

### [SUR-P3-7] §4.4's landing order tells the implementer to migrate six app-file copies; there are five

**Root cause.** A count in the paragraph that specifies the migration's scope and order.

**Location.** Object §4.4 `:1060` — *"The six app-file copies live in **four** repositories and cannot land atomically."*

**What a user meets.** The enumeration two lines below gives five: *"`grazel` (two files), `glade` (the demo's copy of `grazel-app.glade`), `glade-gyld` and `glade-gwz` (one fixture each)"*. §4.4's own census command (`:1156-1158`) lists five paths; R9's prose at `:728` says "any of the five files"; the RemPlan's round-1 record of the drafter's own finding says five. `git ls-files | grep '\.glade$'` across all members returns five. The "six" is the only occurrence.

**Impact.** The landing order is the one instruction whose correctness is safety-relevant — the object establishes at `:1065-1070` that the reverse order has a stuck state invisible to `cargo test -p glade-node`. An implementer executing it is told to find six files and finds five, and must decide whether they missed one before proceeding to step 2.

**Required correction.** "five app files".

**Closure test.** Every file count in §4.4 matches the census command printed in the same section.

---

### [SUR-P3-8] Three evidence citations do not reproduce at the round-2 tuple, because two members moved after revision 2 was drafted

**Root cause.** The object cites line numbers and a command output in member repositories that advanced between the object's commit and the tuple this review is conducted at.

**Location.** Object §3 R11 naming note `:793` (`glade-gyld/README.md:315`); §3 R9 option (c) `:721` (`glade-gyld/README.md:934`); §2m A10 `:369` (the changelog census).

**What I verified.** The root lock at `1defe3b` — where revision 2 was committed — names glade-gyld `024d2a8e`; the lock at `a617c33`, which the RemPlan declares this round's tuple, names `65da8cb7`. The README grew 981 → 1192 lines in between. At `024d2a8e`, `:315` is exactly *"### The compatibility profile"* and `:934` is inside the value-fold passage: both citations were **correct when written**. At the round-2 tuple `:315` is prose about a `fragment` record and `:934` is prose about prompt injection; the intended targets are now `:499` and `:1147`. Separately, A10 publishes `for r in grazel glade glade-gyld glade-gwz glade-decl; … → 0 five times`; re-run at the round-2 tuple it returns `0 2 0 0 0`, the two being `glade/dev-docs/async-witness/real/tests/{peer_release,release_order}.rs`, Rust tests about lock-release ordering. **A10's substantive claim is still true** — no CHANGELOG, migration or release note exists in any of the four app-file repositories or `glade-decl` — but its published command no longer prints its published output.

**Impact.** Both R11's naming argument (why the key must not be spelled `profile`) and R9(c)'s fold rule (what the format page would have to say) rest on citations that now land a reader on unrelated text. Bounded, and the provenance is blameless; but the object will be read at this tuple and at later ones.

**Required correction.** Re-point `glade-gyld/README.md:315` → `:499` and `:934` → `:1147`; restate A10's census as a count of *release-note documents* rather than a filename grep, or record its output as `0 2 0 0 0` with the two false positives named.

**Closure test.** Every `file:line` in §3 R8–R11 and §2m resolves to the cited text at the tuple named in the object's header, and every published command reproduces its published output.

---

### [SUR-P3-9] Under the recommended R9(b) the file-side spelling rule is unstated, and the refusal message names the other spelling

**Root cause.** R9(b) defines the boundary mapping in one direction and never says which spellings a file may legally contain.

**Location.** Object §3 R9 option (b) `:720`; §4.4 bullet 7 `:1161-1164`; R9's prose `:727-728`; R11's naming note `:795`.

**What a user meets.** R9(b) says *"`parse()` normalises `from-cursor` → `from_cursor` on the way into `BindingDecl` … The file keeps the hyphen"*. It does not say whether a file containing `from_cursor` is accepted, rejected, or normalised to itself. Meanwhile §4.4 bullet 7 fixes the refusal text as ``unknown retention `windowed` (removed; use `from_cursor`)`` — the **underscore** — and adds *"and, if the file side respells, the same for `from-cursor`"*, making the hyphen conditional. So under the object's own recommendation, an author migrating `term.log` is told by the parser to type a spelling the recommendation has just decided the file does not use, and cannot tell from any page whether typing it is legal.

Downstream of the same gap: SUR-P3-3's correction asked that, whichever joining character is chosen, *"state the convention in the grammar doc so the next token is not a second coin flip."* The convention is stated — in R9's prose and R11's naming note, both inside this design document. No §4.4 bullet requires it in the format's own documentation, so the next token is still a coin flip for whoever adds one after the freeze.

**Impact.** Either the format permanently accepts two spellings for one value — the outcome SUR-P3-3 existed to prevent — or an author following the parser's own instruction writes an illegal token. Bounded, because a diagnostic catches the illegal case; but the ambiguity is being frozen.

**Required correction.** State in R9(b) whether `from_cursor` is legal on the file side. Make §4.4 bullet 7's example message consistent with the option chosen. Add a §4.4 bullet requiring the joining convention to be written in the format's grammar documentation, not only in this design document.

**Closure test.** One sentence in the format's grammar documentation states the joining character for multi-word tokens; the replacement named in every refusal message is a spelling the file format accepts.

---

## 2. Invariant analysis

### What I attacked and what held

- **Lifecycle pair: add a binding / remove a binding.** *Add* is `binding …`. *Remove* now exists — R9(a) supplies a `BindingRetraction` record kind and a `Record::Retract` arm, the recommendation takes (a), and R9's prose is honest that (c) leaves deletion *"permanently unanswered"*. The pair is present and named. **Held, with SUR-P2-7 against the scope.** Note that deletion is expressed by absence from the file rather than by a directive; that is a defensible declarative design, but §4.4 bullet 5's phrase *"the retract form"* reads as something the author writes. One word ("the retraction rule") would remove the ambiguity.
- **Lifecycle pair: migrate a file / tell it migrated.** R10(a) is exactly this half — the header becomes `v1`, the file says which language it is in, and a `v0` file loads with a warning naming the replacement. R10(b) is honest that there is then no way to tell. Both options priced, recommendation stated. **Held** (the grammar block's staleness is SUR-P2-8, a separate root cause).
- **Every option in R8–R11 has a stated default.** R8 → (b). R9 → (b)+(a), with the prose explaining that (b) is a blast-radius reduction of (a), not an alternative. R10 → (a). R11 → *"(a) if either R2 keeps `ttl` or R4 returns (a); (c) otherwise"* — a conditional default that names its dependency and is internally consistent with (c)'s stated precondition. **Held.**
- **Token order across the line.** Shape → authority → zone → retention, stable across all 28 lines in all five files, matching every grammar comment and `appdecl.rs:113`'s template. A user copying a neighbour cannot get the order wrong. **Held.**
- **Dual maintenance of `grazel-app.glade`.** Re-verified byte-identical by `diff`. Both copies carry the `DUAL-MAINTENANCE NOTE` at `:10-13`; `grazel/README.md:6-8` repeats it; §4.4 bullet 6 flags it for the migration and cites the tests that fail otherwise. **Held.**
- **`seed`, `service` and `workspace` line consistency.** All use the same bare-word positional style, the same dotted ids and the same hyphen convention (`ws-razel`, `glade-gyld`), and their comment blocks in `grazel-app.glade` and `gyld-app.glade` explain the `verb.*` pattern. `gyld-app.glade:60-61` even explains why a new surface needs no new seed. **Held.**
- **Line-numbered diagnostics.** Every message in `appdecl.rs` begins `line {n}:`. The amendment extends that posture to zone, retention, the header and the tail, and §4.7 row 7 gates it. **Held, and it is the right posture for a hand-edited file.**
- **`atom` entering the recognised-but-unauthorable tier.** §4.4 bullet 1 handles it explicitly, including the fact that `atom`, `message` and `window` have nothing to redirect to unlike `exchange`. **Held.**
- **The `crdt` dead end from round 1 is gone.** Legal profile values are named and published, per-shape omission semantics are stated, enforcement is at parse with a line number, and the key is renamed away from two live collisions. This is the clearest improvement in revision 2. **Attacked, held.**

### First-day walkthrough, from the pages alone

**Walkthrough A — author a file with one `value`, one `log` and one `crdt` binding**, assuming the amendment lands under its own recommendations.

1. *Find the format.* `glade/README.md:23` advertises `docs/` as *"Public support contracts and user-facing documentation"*; `glade/docs/` holds one 8-line placeholder which additionally forbids moving internal drafts in. The grammar lives in `GladeGrazelAttachNotes.md`, classified *"Internal engineering design"* at `glade/README.md:24` and titled after a different subject. §4.4 `:1100-1105` says the grammar *"should move to, or be mirrored in, a page presented as user-facing"* — no page named, no §4 step, no §4.7 row. **Guess #1, unchanged from round 1.**
2. *Header.* Under R10(a) I must write `glade-app v1`. The published grammar at `GladeGrazelAttachNotes.md:30` says `glade-app v0`. **Guess #2 — new in revision 2** (SUR-P2-8).
3. *The `value` binding.* Id: **Guess #3**, `glade-decl/README.md:28-30` unchanged (SUR-P3-5). Authority `share`: now published at `GladeDeclSurface.md:28`. **Resolved.** Zone `commons`: published at `:29`, and §4.4 requires a meaning. **Resolved.** Retention: **Guess #4** — no page names a retention value (SUR-P2-6).
4. *The `log` binding.* Shape `log` published. Retention: **Guess #5**, and it is the hazardous one — the object itself says picking `latest` silently converts an append log into a last-writer-wins value, and no diagnostic fires on a legal token.
5. *The `crdt` binding.* Shape now published at `GladeDeclSurface.md:27`. Profile values `snapshot_delta` / `text_crdt` published in the same cell. Omission refused at parse for `crdt`, so I will be told. **The round-1 dead end is gone.** Remaining: the tail's *syntax* appears in no grammar I can read (**Guess #6**, SUR-P2-8), and the retention for a crdt is still unnamed — and still actively misleading, since `GladeDeclSurface.md:29` glosses the zone as "who converges within it" while `glade-gyld/README.md:1147` says `value` folds *"last-writer-wins by `(lamport, origin)` — never by content"*, which is the opposite of merge.

Round 1: five guesses and one dead end. Revision 2: six guesses, no dead end, and four of the six reduce to two root causes (SUR-P2-6, SUR-P2-8).

**Walkthrough B — migrate a file that uses `from-cursor` and `windowed`.**

1. *Learn that I must.* Under R10(a) the node warns and names the replacement, and R10's Docs column requires the migration note these repos still lack (re-verified). **Resolved** — this was round 1's first dead end.
2. *`from-cursor`.* Under the recommended R9(b): **zero edits, on 13 of the 15 lines.** The single most valuable change in revision 2.
3. *`windowed` on `term.log`.* §4.4 bullet 7 names the replacement in the message. **Resolved** — round 1's wrong guess is gone. Residue: the message names `from_cursor` while R9(b) keeps the file's hyphen (SUR-P3-9).
4. *Both copies of `grazel-app.glade`.* Named in the file, in `grazel/README.md`, and in §4.4 bullet 6 and the landing order. **Held**, modulo the "six copies" count (SUR-P3-7).
5. *Reboot.* R9 now answers what a changed line does, under all three options, and §4.7 rows 9–10 gate it. **Resolved** — round 1's second dead end is gone.
6. *Verify offline.* Still no `--check`, `--validate`, lint or dry-run for an app file; the sole verification of the migration is booting a node, which §4.4 `:1057-1059` confirms is a process exit before the listener binds. Unchanged from round 1, which judged it non-blocking; nothing in revision 2 worsens it. **No finding** — recorded so a later auditor sees it was weighed.

---

## 3. Risks and next action

Revision 2 closes six of my ten round-1 findings on their own terms, and closes them properly rather than by assertion: R9, R10 and R11 are real rulings with real option tables, §4.4 grew from 22 lines to 136, and R9(b) alone turns a 15-line mandatory rewrite into a 2-line one. The `crdt` binding — unwritable at round 1 — is now writable. I want that on the record before the residue.

The residue has one shape. **Revision 1's habit was to validate tokens without documenting them. Revision 2 fixed that for the token it was asked about and not for the token beside it, and fixed the two documents it was pointed at and not the one they both cite.** Every open blocker is a scope gap in a correction that is otherwise right:

- **SUR-P2-6** applies the accepted SUR-P2-1 principle to token 5. Same wave, same bullet, same invariant — and token 5 is the one the amendment migrates, the one whose wrong choice is silent, and the one the object itself identifies as capable of converting an append log into a last-writer-wins value.
- **SUR-P2-1** needs one more path in the rewrite list: the document the other two cite by name, which §4.2 step 5 already opens.
- **SUR-P2-8** needs a Docs column on R11 and a named artefact list for R10 — the published grammar and three in-file comments. This is cheapest now because `:113`'s template stops being reachable the moment R11(a)'s arity check becomes a minimum.
- **SUR-P2-7** needs one sentence of scope on R9(a). The scoping key already exists on the record. Without it, the recommended option can retract seven live surfaces on a boot nobody edited.

None of these is architectural; none needs a design change; all four are text inside the object.

**Next action for the lane owner.** Revise the object to (1) extend §4.4's "What must be written before validation is turned on" to token 5, symmetrically with token 4, and give `GladeDeclSurface.md` a `Retention` row; (2) add `dev-docs/glade/GladeDeclSurface.md:29-30` to the zone-contradiction rewrite list; (3) give R9(a) a stated retraction scope — per `(app, glade_id)`, a file not loaded retracts nothing — and widen §4.7 row 10 to the two-file case; (4) give R11 a Docs column and name, in R10 and R11, the published grammar and the three in-file grammar comments that change with the line. The four P3s are one-line text fixes that can ride the same patch.

I pre-commit to GO on a revision that resolves **SUR-P2-1, SUR-P2-6, SUR-P2-7 and SUR-P2-8** as specified above.
