# GladeDeclReconciliation — Remediation plan, round 2

Date: 2026-09-22. Lane owner's plan. Object: `dev-docs/glade/GladeDeclReconciliation.md`
revision 2 at glade-wz root `1defe3b`, re-reviewed at the tuple recorded in
`-RemPlan.md` "Round 2 — tuple and tier" (root `9026507`, lock `a617c33`). Reports
filed verbatim at `574319d`: `-ReviewConsistency-2.md` (NO-GO, 2 P2, 3 P3; all eleven
round-1 findings CLOSED), `-ReviewSafety-2.md` (NO-GO, 3 P2, 1 P3; all eight round-1
findings CLOSED), `-ReviewSurface-2.md` (NO-GO, 4 P2 of which one carried, 6 P3; six of
ten round-1 findings CLOSED). All three reviewers pre-committed to GO on a revision
resolving their P2s as specified, and all three state in terms that none of their
findings is architectural.

Rules of this round, unchanged from round 1: ONE patch to the object, no series. Every
blocking finding has exactly one disposition and one closure test below. The drafter
closes nothing; the reviewer who raised a finding verifies its original counterexample
on the revision. The object is a DRAFT design: where a correction requires a CHOICE,
the revision states the options, their true costs and a recommendation, and adds the
choice to the owner's rulings; it does not make the choice.

**This is remediation round 2 of at most 2.** If any reviewer's re-verdict on revision 3
names a new ARCHITECTURAL root cause, the lane stops and the owner decides
redesign-or-accept; no fourth revision is drafted.

## Two corrections to the round-1 record

- `-RemPlan.md` "Facts the resuming lane owner needs" called CON-P2-4's landing as an
  R1/R2 sub-choice a drafter departure. The Consistency reviewer ruled it is not: both
  the round-1 correction ("Move 'validate token 4' into R1's option set (or a new
  ruling)") and the RemPlan's own disposition ("moves under R1 … under R2") name that
  form first. CON-P2-4 is CLOSED; what the sub-choice form costs is a new finding,
  CON-P2-7, on its landing.
- The Safety reviewer ruled the R4 departure proper (the RemPlan's own rule converts
  "choose" into "lay out") and SAF-P1-1's closure test met under every R4 option. The
  gap it found beside it is a new finding, SAF-P2-10, on §4.2's compatibility model.

## Blind convergence (the strongest signal this process has)

| Defect | Axes that found it independently |
|---|---|
| `:1060` says "six app-file copies"; every other count in the object and a `find` say five | **all three**: SUR-P3-7, CON-P3-7, and the Safety report's second "fact" |
| Citations into members that moved between rounds land on unrelated text: `glade-gyld/README.md:315,934`, `grazel/src/lib.rs:150`, §4.7 row 15's count and globs, A10's census output | **all three**: SUR-P3-8, CON-P3-8, and the Safety report's first "fact" — tuple drift, not drafter error; every axis says the substance holds |
| §4.0's "every row of §4.7 is green" is stated over rows that are conditional, red by design, not gates, or manual | CON-P2-6 (P2) and the Safety report's first residual risk; SAF-P2-10 makes row 2 red under the recommended answers for an undeclared reason |
| The recommended R9 options each carry an unstated consequence: (a) has no retraction scope, so a normal two-file boot retracts seven live surfaces; (b) normalises inside `parse()`, which produces the durable record, so its title is false and its durable cost is 15 records, not 2 | SUR-P2-7 (from the pages and app files) and SAF-P2-11 (from `appdecl.rs:37,138,264`) — different root causes on the same ruling |
| Token 5's validation is switched on in §4.4 bullet 2 with less care than token 4's: no documentation wave (SUR-P2-6); R2's sub-choice by reference to R1's, missing its third option in §4 and gated on GDL-039, which does not decide retention (CON-P2-7) | SUR-P2-6 and CON-P2-7 |

## Blocking findings

| ID | Disposition | Closure test (verified by the raising reviewer) |
|---|---|---|
| CON-P2-6 | ACCEPT. §4.7 is split into (a) unconditional gates, (b) rows conditional on a ruling, each naming the ruling and what green means under each option, and (c) evidence rows that are not gates (14, 16). §4.0's publish sentence is restated over that split: every (a) row green, every (b) row green under the answers actually chosen, (c) is evidence. Under R4(a), row 2 is replaced by the taut-shape-idiom compatibility artefact §4.2 already names, not left red. | Every §4.7 row is classified (a), (b) or (c); no (a) row's text contains "fails by design", "not a gate" or "manual"; for each option of each ruling a reader can name the rows that must be green. |
| CON-P2-7 | ACCEPT. R2's sub-choice (row 18b) is written out in full, not by reference to R1's, with a retention-appropriate third option — "(iii) Do not validate; the token stays inert, and R2's vocabulary is the only thing that could ever make it validatable" — and its own cost example on a token-5 value. "GDL-039 not ratified" is struck from row 18b's *Ratified* cell, leaving "no ratified entry requires it". §4.4 bullet 2 gains a third branch stating that under (iii) the bullet does not land; §4.7 row 7 is conditioned on the sub-choices in the shape rows 9 and 10 use for R9. | For each of the three options of each sub-choice, §4.4 names what lands (including "nothing") and §4.7 names which rows apply; `grep -n 'GDL-039'` over the object returns no hit inside a row or paragraph whose subject is token 5, `RetentionPolicy` or `Retention`. |
| SAF-P2-9 | ACCEPT. §4.4's landing order gains a step, ahead of the app-file wave, in which a node that accepts both `glade-app v0` and `glade-app v1` headers is deployed; R10(a)'s App-files cell says the header bump is NOT part of the app-files-first commit; the step-1 justification distinguishes the token (an old node accepts a new one, `appdecl.rs:137-138`) from the header (an old node refuses a new one, `appdecl.rs:89-95`); R10's table carries the extra step as a cost of (a) and notes that (b) has no ordering hazard. | §4.4 names the both-headers step before any file header moves; `cargo test -p grazel` and `cargo test -p glade-node` are named green at every intermediate commit of the wave; a `glade-node` unit test is named asserting the pre-amendment `parse()` refuses a `v1` header with the `:91` diagnostic. |
| SAF-P2-10 | ACCEPT. §4.2's superset condition list names every ruling whose options change the corpus, including the deleting ones: the claim holds under R4(b)/(c) with R3(a), R2(a), R5(b), R7(a) and R6(a)/(b); under R7(b), R6(c), R3(b) or R2(c), v1 is a superset of a stated subset of v0, and the text says which vectors leave. `--compat` is specified with a declared deletion list: it asserts `v1[n].cbor == v0[n].cbor` over `n ∈ v0 ∩ v1` and requires every `n ∈ v0 \ v1` to be named in the invocation, so a deletion is a checked decision and undeclared absence is a failure like byte drift. §4.7 row 2 and §4.0 say the same thing (this is the (b) classification CON-P2-6 requires for row 2). | The specified gate run over a v1 built under the recommended answers is green with `AdvertisementRecord` and `edge/advert` declared, and red without the declaration; no affirmative superset sentence survives without the gate named as its proof. |
| SAF-P2-11 | ACCEPT. R9(b) is restated truthfully as two shapes with true costs, the choice left to R9: (b1) the node maps spellings where a CONSUMER reads the record, so the stored token keeps the file's spelling and no stored byte changes — cost: the stored vocabulary and the contract vocabulary differ for the life of the store, and every reader maps; (b2) `parse()` normalises on the way into `sysdata::BindingDecl` — cost: 13 stored records change bytes on the first boot, so (b2) needs (a)'s or (c)'s fold exactly as they do. The recommendation clause at `:726-727` and §4.4 bullet 5 at `:1147` are corrected to the recommended shape's true count, and the benefit (b) genuinely buys — 13 fewer FILE edits and the hyphen convention kept — is the claim. | §4.7 row 9's test, instantiated for the recommended (b) shape, asserts the number of appends the text claims (0 under b1, 13 under b2); R9(b)'s stated durable cost equals that number. |
| SUR-P2-1 | ACCEPT. `dev-docs/glade/GladeDeclSurface.md:29-30` joins §4.4's rewrite list for the mount sentence, carrying the answer §4.4 `:1087-1091` already gives: the authored zone is the author's to choose, and on the path that honours it the mount does not override it. | Grep of the full page set for "fills domain/zone/key", "zone… fill" and "maps them at bind time": every surviving occurrence says the author writes the zone and what a mount does with it. |
| SUR-P2-6 | ACCEPT. §4.4's "What must be written before validation is turned on" block is extended to token 5, symmetrically with token 4: what `latest`, `from_cursor` and `ttl` mean; which to write for an append log and which for a snapshot value; under R11(a), how a duration is expressed. `GladeDeclSurface.md` gains a `Retention` row beside `Shape`, `Authority` and `Domain`/`Zone`; `glade-decl/README.md` gains the members table SUR-P3-1's round-1 disposition called for — both as named §4 steps. | Grep of the page set for each of `latest`, `from_cursor`, `ttl` returns a hit with a one-line gloss in the retention sense; a reader of the pages alone can say which retention to write for a terminal-scrollback log and which for a settings value. |
| SUR-P2-7 | ACCEPT. R9(a) states its retraction scope in the option itself: retraction is computed per `(app, glade_id)` against the declarations previously registered for the `app` named in the file being registered; a file not loaded on a boot retracts nothing; a declaration by a different app file is never in scope. §4.4 bullet 5 says a surface declared by an unloaded app file stays declared. §4.7 row 10 is widened to the two-file case, and the option is priced against that rule. | Three `glade-node` tests are named: register `grazel-app.glade` then `gyld-app.glade` into one registry and assert all 14 live; register `grazel-app.glade` alone on the next boot and assert gyld's 7 untouched; delete one line from `gyld-app.glade`, reload both, assert exactly that one surface retracted. |
| SUR-P2-8 | ACCEPT. R11 gains a Docs column like R1, R9 and R10. R10 and R11 name the artefacts that change with the line: `GladeGrazelAttachNotes.md:30` (header, under R10) and `:32` (tail, under R11); the in-file grammar comment at `grazel/apps/grazel-app.glade:19`, `glade/apps/grazel-app.glade:19` and `grazel/apps/gyld-app.glade:17`; and one sentence on how an author learns the tail exists, since `:113`'s template is unreachable for a valid five-token line once the arity check is a minimum. | Every published grammar and every in-file grammar comment shows the line the parser accepts, header and tail included; an author who has read only those can write `ttl=10m` and `shape-profile=text_crdt` at the first attempt. |

## Non-blocking findings, all accepted as riders on the same patch

`CON-P3-7` correct `:1060` to five; mark `exchange.rs:651-658` as the end-to-end test it
is or cite the production fan-out path; narrow "lists every directive" (it omits
`workspace`) and cite `appdecl.rs`'s match arms for "no retract form"; restate R6's
premise against `README.md:42`; fix the reversed quote at `:957`. `CON-P3-8` re-pin the
Appendix to the round-2 member SHAs and re-open the five stale proofs — `grazel/src/lib.rs:150`
→ `:159`, `glade-gyld/README.md:315` → `:499` and `:934` → `:1147`, §4.7 row 15 (61 suites;
`vite.config.ts:309-310`), A10's census (`0 2 0 0 0` with the two `async-witness` false
positives named, or restated as a count of release-note documents) — and state each count
with the SHA it was taken at. `CON-P3-9` re-derive the closure map's five wrong pointers
(CON-P2-1 → §4.5.3; SUR-P2-2 → §4.4 bullet 3; SUR-P2-3+4 → bullet 4; SAF-P2-3 → rows 6–8;
CON-P3-1 → "The process authority…") and write pointers as heading plus bullet text, not
ordinals; add the round-2 IDs to the map. `SAF-P3-12` §4.1.8: a retired message name is
recorded in `glade_decl.taut.py`'s module docstring and `OpenNotes.md`, naming the
retiring version and forbidding reuse with a different shape — taut has no schema-level
`reserved` (`taut/src/taut/ir/model.py:136-140`). `SUR-P3-6` one sentence saying which
`.glade` dialect the amendment freezes, and a §4 step giving the eleven
`dev-docs/examples/*.glade` files the A3 treatment (a banner naming their language) or
recording the extension question for the owner. `SUR-P3-7` is CON-P3-7's first item.
`SUR-P3-8` is CON-P3-8's second and third items. `SUR-P3-9` R9(b) states whether
`from_cursor` is legal on the file side under each shape; §4.4 bullet 7's example message
names a spelling the file format accepts; a §4.4 bullet requires the joining-character
convention in the format's own grammar documentation. From the residuals: §4.0 reads
"every gate row" (folded into CON-P2-6); R2's sub-choice carries a token-5 cost example
(folded into CON-P2-7); `glade-decl-ts/README.md:21-22` still teaches `npm`, noted beside
A4/A5 with the standing rule (pnpm for commands, no unasked lockfile migration); §3's
preamble says two rulings carry a sub-choice, so thirteen answers are expected.

## What the revision may not do

Change any ruling's outcome or recommendation except where a disposition above says
so (R9(b)'s two shapes; R10(a)'s ordering cost; R2's written-out sub-choice). Change any
file other than the object. Weaken a gate to make it green. Delete a corrected false
claim rather than marking it, as revision 2's convention has it.

## Round accounting

Remediation round 2 of at most 2. Re-verdict by the same three round-2 reviewers,
context intact, against the revision-3 tuple, filed as `-Review<Axis>-3.md`, each with a
closure table over every round-2 ID of its axis and a changed-range analysis over
`1defe3b..<revision 3>`.
