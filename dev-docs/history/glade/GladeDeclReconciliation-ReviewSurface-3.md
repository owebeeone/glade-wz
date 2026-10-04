# GladeDeclReconciliation (revision 3) — SURFACE-AXIS REVIEW, ROUND 2 RE-VERDICT

**Review object:** `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclReconciliation.md`, **revision 3**, committed at glade-wz root `c26211d` and unchanged since (`git log c26211d..HEAD -- dev-docs/glade/GladeDeclReconciliation.md` empty). 2002 lines. DRAFT design, unimplemented. The working-tree file is the object; the root working tree is clean.

**Baseline:** glade-wz root `c26211da0473` · glade `559cb2c87e85` · glade-decl `d671f10c13e6` · glade-decl-rs `21eefa1c3a53` · glade-decl-ts `7e16e324630a` · glade-decl-py `1b0f6d1f7886` · taut `7a5f616c3a9f` · glial `0dfe4b930063` · grazel `c66f029ad060` · glade-gyld `65da8cb7e2b7` · glade-gwz `e53c87dddb8f` · glade-chat `9238d21f6a36` · glade-discover `fd94a1f87bbc` · grip-core `97ff6c26f12e` · taut-shape `9a752094dbed` · gryth-ui `9323818a39d2`. **Every member SHA is identical to my round-2 baseline**, and all were verified identical at start and at end of this review. Every tree clean except glade-discover's three out-of-scope modified dev-docs. The three root commits since my round-2 tuple `9026507` are documents only (`574319d` the round-2 reports, `61bb479` the remediation plan, `c26211d` revision 3). `gwz-dev` moved `7b3f1bc723d6` → `9c0008870ecd` (one commit, "Record the accepted SSH setup-timeout plan"); `dev-docs/AgentProcessRules.md` has **no** commit since `ff431743cc4c`, so the L1 citations still hold.

Sources read at this tuple: the object's Revision 3 block and closure map (`:21-66`), §2m A9–A12 (`:453-456`), §3 R8–R11 (`:817-1005`), §4.2 step 6 (`:1210-1222`), §4.4 in full (`:1366-1731`), §4.6 (`:1770-1795`), §4.7 in full including the (a)/(b)/(c) headings and rows 17–20 (`:1797-1849`); `-RemPlan-2.md`; my own round-2 report. The five app files; the eight user-facing pages (`glade-decl/README.md`, `grazel/README.md`, `glade-gyld/README.md`, `glade-gwz/README.md`, `glade/README.md`, `glade/docs/README.md`, `glade/dev-docs/GladeGrazelAttachNotes.md`, `dev-docs/glade/GladeDeclSurface.md`). The one permitted grep of `appdecl.rs`'s diagnostic strings. **I read no Rust, TypeScript or Python source, no other axis's report, and no design or plan document beyond the above.** Read-only throughout: no writes, no mutating git, no build or test run.

**Date:** 2026-09-22

**Axis:** SURFACE — the `<app>.glade` file format as the person typing and editing it meets it: cold-read naming, the legal values of each token, lifecycle pairs, stated defaults, and the first-day walkthrough from the published pages alone. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: GO** — 0 P0, 0 P1, **0 P2**, 4 P3 (all new, none blocking). **All nine round-2 findings of this axis are CLOSED**, including the four P2s I pre-committed on and the two P3s that were carried open from round 1. My round-2 pre-commit — *"I pre-commit to GO on a revision that resolves SUR-P2-1, SUR-P2-6, SUR-P2-7 and SUR-P2-8 as specified"* — is honoured: each is resolved as specified, each closure test re-run, and I hold to it. **No new ARCHITECTURAL root cause.** The four P3s below are one-sentence-to-one-row text fixes; none of them needs a revision 4 to be safe, and I name in §3 the two I would fold into the freeze commit.

---

## Prior-finding closure table

One row per round-2 finding of this axis. "Verified" means I re-ran or re-traced the **original counterexample** at the revision-3 tuple, not that the RemPlan or the object claims a disposition.

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| **SUR-P2-1** — the mount sentence has a third home, the one the other two cite | ACCEPT. `dev-docs/glade/GladeDeclSurface.md:29-30` joins §4.4's rewrite list, carrying the answer §4.4 already gives. Closure test: grep the page set for "fills domain/zone/key", "zone… fill", "maps them at bind time"; every survivor says the author writes the zone | **Re-ran the closure grep myself** over the eight pages **and** the five app files. It returns exactly four homes and no fifth: `grazel/apps/grazel-app.glade:21`, `glade/apps/grazel-app.glade:21`, `GladeGrazelAttachNotes.md:50`, `GladeDeclSurface.md:29` and `:30`. §4.4 "**And the contradicting sentence must go, in all four of its homes**" names all four, quotes `:29` and `:30` verbatim, and states why the source matters ("an author who does what the citation invites lands on the authoritative statement of the thing the other two were about to stop saying"). §4.7(a) row 20 is that exact grep, classified **unconditional** with the four homes enumerated, and its reasoning is independently sound — the sentence is already wrong on the grip-share path today | **CLOSED** |
| **SUR-P2-6** — token 5 validated in the same bullet as token 4 and documented nowhere | ACCEPT. §4.4's documentation block extended to token 5 symmetrically: what `latest`/`from_cursor`/`ttl` mean; which to write for an append log and which for a snapshot value; under R11(a) how a duration is expressed. `GladeDeclSurface.md` gains a `Retention` row; `glade-decl/README.md` gains the members table — both as named §4 steps | §4.4 now has parallel "**Token 4 — the zone**" and "**Token 5 — the retention (SUR-P2-6)**" blocks. The five token-5 statements are present, and the closure test's own language is met verbatim: *"an append log or an output stream → `from_cursor`; a settings or status value read at the head → `latest`"*. The two page edits are named §4 steps — **§4.4 bullet 15** (the `Retention` row, which I confirmed `GladeDeclSurface.md` lacks beside its `:27`/`:28`/`:29` rows) and **§4.2 step 6** (the members table, "including the `Retention` members, which no published page names at all"). §4.7(b) row 18 is the gate, enumerated per R2 option (a)/(b)/(c)/(d), with my own zero-hit baseline recorded in the row. The object also adopts the hazard I raised, in terms: `latest` "silently converts an append log into a last-writer-wins value, and no diagnostic fires, because `latest` is a legal token" | **CLOSED** (residue → SUR-P3-11) |
| **SUR-P2-7** — R9(a)'s retraction has no stated scope | ACCEPT. Scope stated in the option: per `(app, glade_id)`; a file not loaded retracts nothing; another app file's declaration is never in scope. §4.4 bullet 5 says so. §4.7 row 10 widened to the two-file case, with three named tests | R9 gains "**(a)'s retraction scope, stated in the option (SUR-P2-7)**", which states the rule, names the scoping key (`sysdata.BindingDecl` carries `app` as field 1), and reproduces **both** of my counterexamples — the two-file boot naming all seven retracted ids, and the default-off flag path — then states the design they negate, citing `gyld-app.glade:19-20` and `grazel/README.md:15-17`. §4.4 bullet 5 carries "**A surface declared by an app file that is not loaded on this boot stays declared**". §4.7(b) row 10 names exactly the three tests my closure test asked for. **I verified the drafter's arithmetic:** `grazel-app.glade` declares 7 bindings and `gyld-app.glade` declares 7, so row 10's "assert all **14** bindings live" is exact | **CLOSED** |
| **SUR-P2-8** — nothing updates the grammar the author copies from | ACCEPT. R11 gains a Docs column. R10 and R11 name the artefacts: `GladeGrazelAttachNotes.md:30` and `:32`, the three in-file comments, and one sentence on how an author learns the tail exists | R11's table now carries a **Docs** column populated for all three options, naming `:32` and the three in-file comments (`grazel/apps/grazel-app.glade:19`, its twin, `grazel/apps/gyld-app.glade:17`) and recording that the two fixtures carry no grammar comment — the 3-of-5 correction I made to my own round-1 report, adopted. R10's Docs cell names `:30` and states the consequence I raised: without it "the format's own specification instructs every new author to write the deprecated header". §4.4 bullet 12 is the landing step; §4.7(b) row 19 is the gate, and its green condition is my closure test verbatim. **I verified the drafter's fact:** `grep -c '^#.*glade-app'` is **0** in all five app files — no in-file comment shows the header anywhere; the header is only the data line (`grazel-app.glade:16`, `gyld-app.glade:14`, both exact; the fixtures at `:8`). So R10's claim that `:30` is the **only** place an author can read the header is correct, and it strengthens the closure | **CLOSED** |
| **SUR-P3-5** — glade ids derived vs hand-typed (carried open from round 1; closure site was off my round-2 read list) | Rider under SUR-P3-1/P3-5: §4.2 step 6 says on the front page whether an app-file glade id is authored or derived | **Now verifiable — §4.2 step 6 is on this round's read list, and I opened it.** It requires the front page to "state at `:28-30` whether an app-file glade id is **authored or derived**", quotes `glade-decl/README.md:28-29` verbatim (with the elision I did not catch corrected under CON-P3-7), notes the 14 hand-typed ids against a deferred `derive_glade_id` (`:48-50`), and requires "If authored, give the syntax rule (charset, the dot's meaning, length) and say what `derive_glade_id` is then for" — my round-1 required correction, word for word | **CLOSED** |
| **SUR-P3-6** — sixteen `.glade` files, five of this format | Rider: one sentence saying which dialect is frozen, and a §4 step giving the eleven examples the A3 treatment or recording the extension question | §4.4 bullet 14 states it, with my sixteen/five/eleven counts and both commands, describes the other dialect accurately, requires the one sentence, offers the A3 banner or the owner's extension question, carries my own boundedness reasoning, and cites `GladeDeclSurface.md:123` — which I re-verified reads "`.glade` is data; it never becomes a compiler front-end" | **CLOSED** |
| **SUR-P3-7** — "six app-file copies"; there are five | Rider (CON-P3-7's first item): correct to five | §4.4's landing order now reads "The **five** app files live in **four** repositories", with the correction marked and the `grep -rln` command published; step 1 enumerates the five paths by repository; bullet 6 closes with the same command. **I re-ran it: exactly five files carry `glade-app v0`** | **CLOSED** |
| **SUR-P3-8** — three citations that do not reproduce at the round-2 tuple | Rider (CON-P3-8): re-pin `glade-gyld/README.md:315` → `:499`, `:934` → `:1147`, restate A10's census | **Re-verified at this tuple:** `:499` is "### The compatibility profile"; `:1147` is "`value` last-writer-wins by `(lamport, origin)` — never by content". Both re-pinned in R11's naming note and R9(c) with the drift marked. A10's census is restated as `0 2 0 0 0` with the two `async-witness` false positives named by path — **I re-ran it and got `0 2 0 0 0`** — and the claim correctly held rather than weakened. The whole object is now pinned to the round-2/round-3 member SHAs, and §4.4's census and gate counts carry the SHA they were taken at | **CLOSED** |
| **SUR-P3-9** — the file-side spelling rule, and the refusal that names the other one | Rider: R9(b) states whether `from_cursor` is legal per shape; bullet 7's message names a spelling the format accepts; a bullet requires the joining convention in the format's own grammar documentation | R9 gains "**Which spellings a file may contain, under each shape (SUR-P3-9)**", answering for b1, b2, (a) and (c), and correctly routing the residual question to R2's 18b sub-choice rather than inventing an answer. §4.4 bullet 7 now carries **two** messages — hyphen under b1/b2, underscore under (a)/(c) — with the defect I raised marked in place. §4.4 bullet 13 requires the joining convention in the format's own grammar documentation, noting it exists today "only in R9's prose and R11's naming note — both inside this file, which no author reads" | **CLOSED** |

---

## Changed-range analysis

`git -C /Users/owebeeone/limbo/glade-wz diff 1defe3b..c26211d -- dev-docs/glade/GladeDeclReconciliation.md` → **+831 / −201**, 1372 → 2002 lines. Restricted to this axis's sections:

| Section | Revision 2 | Revision 3 | Assessment |
|---|---|---|---|
| Revision-3 block + closure map (`:21-66`) | absent | new: 22 rows, each pointer a **heading plus the bullet or row text** rather than an ordinal | Within disposition (CON-P3-9). I spot-checked every SUR row's pointer and all six land on the text they name. The anchor-not-ordinal convention is the right fix and made this review materially cheaper. |
| §2m A9–A12 (`:453-456`) | A9–A12 | A9 gains the test-code marking of `exchange.rs:651-658`, the enumeration of `parse()`'s five directive arms ("none of them retracts"), and the narrowing of "lists every directive"; A10 gains the re-pinned census | Within disposition (CON-P3-7, CON-P3-8). A9's new directive enumeration is what makes SUR-P3-12 below nameable. |
| §3 R8 (`:817-845`) | unchanged in substance | unchanged | — |
| §3 R9 (`:846-936`) | 3 options, one table | **4 options** (a, b1, b2, c), a new "**Durable cost on the first boot**" column (15/0/13/as-(a)-or-(b2)), the retraction-scope paragraph, the per-shape spelling paragraph, and a recommendation split within (b) | Within disposition (SAF-P2-11 + SUR-P2-7 + SUR-P3-9). The b1/b2 split is the drafter laying out a choice rather than making it, which the RemPlan's own rule requires. |
| §3 R10 (`:938-965`) | 5 columns | gains an "**Ordering cost**" column and the header-artefact Docs cell | Within disposition (SAF-P2-9 + SUR-P2-8). |
| §3 R11 (`:967-1002`) | 5 columns, **no Docs column** | gains the **Docs** column, populated for all three options | Within disposition (SUR-P2-8). |
| §4.2 step 6 (`:1210-1222`) | existed but off my read list | members table per enum incl. `Retention`; authored-or-derived; the corrected verbatim quote | Within disposition (SUR-P3-1/P3-5, CON-P3-7). |
| §4.4 (`:1366-1731`) | 136 lines, 11 bullets | **366 lines**: landing order with **step 0**, the opposite-safe-orders paragraph, "every intermediate commit is a gate", parallel **Token 4** / **Token 5** blocks, the four-home rewrite, bullet 2's **three** branches, and bullets 12–15 | The largest change and this axis's main remediation. Within disposition throughout. |
| §4.6 (`:1770-1795`) | 26 lines | **byte-unchanged** (verified: the diff contains no `§4.6` body line) | Within disposition — nothing was owed here. |
| §4.7 (`:1797-1849`) | one flat 16-row table | split into **(a) unconditional gates**, **(b) gates conditional on a ruling** with per-option green conditions, **(c) evidence, not gates**; rows 17–20 new; row numbers preserved so every existing pointer still lands | Within disposition (CON-P2-6). Rows 18, 19 and 20 are this axis's three new gates and each encodes my closure test rather than a restatement of it. |

**Changes outside `-RemPlan-2.md`'s dispositions and riders:** none found in my sections. Every edit I can attribute maps to a listed disposition, rider or residual, and the plan's "What the revision may not do" is respected on my sections — no ruling outcome or recommendation is changed except the three the plan authorises (R9(b)'s two shapes, R10(a)'s ordering cost, R2's written-out sub-choice), no gate is weakened to make it green, and every corrected false claim is marked in place rather than deleted.

**NEW root causes found in this round:** four, all P3, all filed below.

**NEW ARCHITECTURAL root causes: NONE.** I applied the bar deliberately, knowing this is remediation round 2 of at most 2 and that a third architectural root cause stops the lane. All four new findings are missing sentences or a missing table row in documents this amendment already opens: SUR-P3-10 adds one line to a grammar block bullet 12 is already editing; SUR-P3-11 adds two entries to a list §4.4 already writes; SUR-P3-12 adds one scope sentence to a ruling that already has one; SUR-P3-13 is a single wrong word. None implies a design change, a new mechanism, or a different shape for any ruling. **I state in terms that none of my findings, in any round, is an architectural root cause of this object.**

**First-day walkthrough, re-done: the guess count fell from six to four.** Round 1: five guesses and two dead ends. Round 2: six guesses, no dead ends. **Revision 3: four guesses, no dead ends** — and two of the four are SUR-P3-11 and SUR-P3-10 below, each one table row or one grammar line from resolution. The walkthrough itself is in §2.

---

## 0. Evidence base

Everything in my round-2 evidence base was re-verified at this tuple, since every member SHA is unchanged: five app files carrying `glade-app v0`, 28 binding lines, 13 `from-cursor` / 13 `latest` / 2 `windowed`, 26 `commons` / 2 `private`, 28/28 `share`, 14 distinct hand-typed ids, zero underscores in any authored token. `appdecl.rs`'s diagnostics are unchanged: `:113` prints the five-token template, `:121` carries the `exchange uses \`service\`` precedent, and there is still no zone or retention diagnostic.

**Three facts established for the first time in this round:**

1. **The published grammar block omits a directive every app file uses.** `GladeGrazelAttachNotes.md:29-36` lists `glade-app v0`, `app`, `binding`, `service`, `seed` and the comment rule. It does not list `workspace`. `grep -n -w 'workspace'` over that page returns **zero hits in the whole file**. All **5 of 5** app files carry exactly one `workspace` line. (Finding SUR-P3-10.)
2. **The object's own two-directive claim about retraction.** §2m A9 now enumerates `parse()`'s arms — `app` (`:98`), `binding` (`:107`), `service` (`:141`), `seed` (`:155`), `workspace` (`:168`), catch-all (`:180`) — and states "none of them retracts". R9 supplies a retract half for one of them. (Finding SUR-P3-12.)
3. **Both drafter-reported facts are correct.** No in-file comment shows the header in any of the five files (`grep -c '^#.*glade-app'` → 0 ×5), so under R10(a) `GladeGrazelAttachNotes.md:30` is the only place an author reads it. And `gyld-app.glade` declares exactly 7 bindings against `grazel-app.glade`'s 7, so §4.7(b) row 10's "all 14" is exact.

**Scope note, carried from round 2 and now discharged.** The eleven `dev-docs/examples/*.glade` files I opened headers of in round 2 are addressed by §4.4 bullet 14; I did not re-open them this round. `grazel/src/lib.rs:159` and `gryth-ui`'s vitest count are re-pinned citations into sources outside my read list; I did not open either and make no claim about them.

---

## 1. Findings

All four are P3. None blocks the verdict.

### [SUR-P3-10] The one published grammar omits the `workspace` directive that every app file uses, and bullet 12's wave opens that block without adding it

**Root cause.** The format's only published specification enumerates five directive forms and not the sixth; §4.4 bullet 12 edits two lines of that block for the header and the tail and does not complete it.

**Location.** `glade/dev-docs/GladeGrazelAttachNotes.md:29-36`, against object §4.4 bullet 12 (`:1673-1691`) and §4.7(b) row 19. Corroborated by the object's own §2m A9, which states that this block "lists every directive **it publishes** — and it omits `workspace`".

**Violated invariant.** The grammar an author copies from must contain every directive the format requires to produce a working file.

**What a user writes and meets.** The published block is:

```text
glade-app v0                                        # header, first decl line
app grazel                                          # exactly once, first
binding <glade_id> <shape> <authority> <zone> <retention>
service <name> <exchange-glade-id>
seed <principal> <share> <verb[,verb...]>
# comments + blank lines anywhere; `#` starts a comment
```

The word `workspace` appears nowhere on that page. An author who writes a file from it produces no `workspace` line. That file parses and registers cleanly — and the surfaces do not route. The app files say so themselves: `glade-gwz/tests/fixtures/gwz-test-app.glade:2-4` declares "the workspace share that makes them routable (audit F1: the loading node mints the WorkspaceEntry + ServeClaim, so `(ws-razel, gwz.ops)` routes Local)", and `grazel/apps/grazel-app.glade:47-53` says the entry is "Required for `(ws-razel, gwz.ops)` to route to the composed glade-gwz supplier". Every one of the five files carries the line; the specification never mentions it.

This is the same class as SUR-P3-4, which was accepted as a rider in round 1: the one specification of the format misdirects the reader it is written for. There, the page described a line the parser refuses; here it omits a line the product requires.

**Impact.** Bounded — the omission produces a file that loads, so nothing is corrupted, and the three app files that carry an in-file comment block do show `# workspace <share> <name>` at `grazel-app.glade:47` and `gyld-app.glade:64`, so a copier of an existing file is safe. The exposed party is exactly the third-party author `grazel/README.md:73-79` describes, working from the specification rather than from a neighbour, who gets a silently non-routing app. P3 rather than P2 because the remedy is one additive line in a page and needs no compatibility break.

**Required correction.** Add `workspace <share> <name>` to the published grammar block at `GladeGrazelAttachNotes.md:29-36`, in the same bullet-12 edit that already opens that block for `:30` and `:32`; and say in one clause what it does, since it is the line that makes a declared surface routable.

**Closure test.** Every directive `parse()` accepts appears in the published grammar block; an author who writes a file from that block alone produces one whose surfaces route.

---

### [SUR-P3-11] The new by-surface-kind retention rule has three rows and the authorable shape vocabulary has four members; `swmr` and `crdt` map to none of them

**Root cause.** §4.4's token-5 rule is indexed by what a surface *is* rather than by the shape token the author has just written, and it covers only the three kinds that correspond one-to-one with the three retention values.

**Location.** Object §4.4 "Token 5 — the retention", the "**Which to write, by surface kind**" bullet (`:1481-1484`), against §4.4 bullet 1 (`BINDING_SHAPES` gains `crdt`) and bullet 4's per-shape profile table (`:1576-1581`).

**Violated invariant.** A rule that tells an author which value to write must reach every shape the format lets them write.

**What a user writes and meets.** The rule reads: *"an append log or an output stream → `from_cursor`; a settings or status value read at the head → `latest`; a cache whose entries should expire → `ttl` with its duration, or nothing at all if R11 leaves the duration unsayable."* After this amendment `BINDING_SHAPES` is `value`, `log`, `swmr`, `crdt`. For `value` and `log` the mapping is immediate. For the other two it is not:

- `swmr` — is a workspace file tree "an append log or an output stream", or "a settings or status value read at the head"? The corpus answers `from-cursor` (`grazel-app.glade:23`, the sole `swmr` line), but the rule does not.
- `crdt` — the amendment's headline addition — is none of the three kinds. And the default a reader would reach for is actively wrong: `latest` is now glossed, correctly, as "the surface keeps one value, last write wins", which is the negation of the merge semantics a CRDT exists for. So the only non-log value in the set reads as wrong for `crdt`, and there is no fourth.

Note that bullet 4 does exactly the right thing one token to the left: it states "**What omission means, per `BINDING_SHAPES` member**" and enumerates all four. Token 5's rule does not get the same treatment.

**Impact.** Bounded, and much smaller than the round-2 finding it descends from — this is the last remaining guess in the authoring walkthrough, on two shapes rather than on the whole vocabulary. It matters because `crdt` is the member this amendment adds, bullet 4 makes its profile key mandatory at parse, and the retention beside it stays a guess whose wrong answer is silent. P3: the remedy is two rows, additive, and fixable after the freeze.

**Required correction.** Extend the "which to write" bullet to be indexed per `BINDING_SHAPES` member, as bullet 4 is: state the retention for `swmr` and for `crdt`, or state in one clause that the shape does not determine the retention and what does.

**Closure test.** For each member of `BINDING_SHAPES` a reader of the pages alone can name the retention to write and why; the answer for `crdt` is not contradicted by `latest`'s own published gloss.

---

### [SUR-P3-12] R9 supplies a retract half for `binding` only, while the object's own A9 establishes that none of the five directives retracts

**Root cause.** R9's question, options and tests are scoped to `dir.bindings`; the amendment does not say what a deleted `service` or `workspace` line does, in the revision that first makes a deletion rule exist at all.

**Location.** Object §3 R9 options (a)–(c) (`:871-874`) and the retraction-scope paragraph (`:876-898`); §4.4 bullet 5 (`:1593-1620`); §4.7(b) row 10 (`:1837`). Against §2m A9 (`:453`), which enumerates `parse()`'s arms `app`, `binding`, `service`, `seed`, `workspace` and the catch-all, and states "none of them retracts".

**Violated invariant.** Every add in the grammar has a remove, documented together with it.

**What a user writes and meets.** The format has four directives that add durable state. After this amendment:

- `binding` — R9(a) supplies a `BindingRetraction` record kind, a scope rule, and three tests. **Answered.**
- `seed` — answered pre-existing, and well: `GladeGrazelAttachNotes.md:85-88` states that `grants_for` applies revocation-wins at `(principal, share)` regardless of order, so a runtime revocation beats a re-registered seed. **Answered.**
- `service` — unanswered. An author deletes `service grazel gwz.ops` from `grazel-app.glade:39` and reboots. Nothing retracts, and the object's own R9(c) cell states the consequence in terms: "today a stale `exchange` record keeps a retired surface routable".
- `workspace` — unanswered. `appdecl.rs:176` refuses a duplicate workspace share, so the node models these entries; deleting the line is not addressed anywhere.

**Impact.** Bounded. This is a pre-existing property of the format, not something revision 3 introduced — I did not raise it in round 1 or round 2, and I record that: my brief scopes the lifecycle pair to "add a binding / remove a binding", which revision 3 answers completely. It becomes nameable now because revision 3 is the first text in which `binding` has a stated remove half, which makes the asymmetry with the other three visible, and because A9 now enumerates the arms. I file it as P3 and explicitly **not** as a blocker: what is needed before the freeze is a sentence of scope, not new machinery. Adding a `ServiceRetraction` record kind later is a durable-record change and therefore expensive — which is the argument for writing the scope down now, while the reader is in R9.

**Required correction.** One sentence in R9 stating that its options govern `dir.bindings` only, and one in §4.4 bullet 5 stating what a deleted `service` or `workspace` line does today (nothing retracts; a retired exchange stays routable) — so that the page a user reads says it, rather than leaving it to be inferred from a ruling's silence. If the owner wants symmetry instead, that is a new option under R9, priced there.

**Closure test.** For each of the four adding directives, a reader of the format page can say what deleting the line does. No directive's remove half is left to inference.

---

### [SUR-P3-13] The bullet that defines omission for token 5 says a five-token line is an arity refusal; everywhere else in the object a five-token line is the legal one

**Root cause.** A token count written with a different convention from the one the rest of the document — and the neighbouring token-4 bullet — uses.

**Location.** Object §4.4 "Token 5 — the retention", the "**What omission means**" bullet at `:1485-1486`.

**What a user meets.** The bullet reads, verbatim:

> **What omission means**: as for token 4, there is no default — a five-token line is an arity refusal, not a defaulted retention.

Against the object's own usage elsewhere:

- `:1454-1456`, the token-4 counterpart: "requires exactly five tokens after `binding`, so a **four-token line** is an arity refusal".
- `:971`: "The line is **five positional tokens**."
- `:986`, R11(c): "the line stays **exactly five tokens**".
- `:984` and `:1588`: "a **legal five-token line** never trips arity".
- `:1691`: "someone who wrote a **valid five-token line**".

So in five places a five-token line is the legal line, and in this one place it is the refused one. Read under the document's dominant convention, the sentence tells an author that the line they are supposed to write is an arity refusal; read under the token-4 bullet's local convention (tokens after `binding`), it says the same thing. Either way the sentence is wrong. The intended statement — omitting the retention leaves four tokens after `binding`, which `appdecl.rs:111-114` refuses — is correct and is what token 4's bullet says.

**Impact.** Small and entirely bounded, but it sits in the one paragraph that closes a P2 and defines the omission semantics of the token the amendment migrates, and it is the kind of thing that gets copied into the user-facing page §4.4 requires. P3.

**Required correction.** "a four-token line is an arity refusal", matching the token-4 bullet; or state both counts explicitly once ("five arguments after `binding`; six whitespace tokens including it") and use one convention throughout §4.4.

**Closure test.** Every token count in §4.4 uses one stated convention, and no sentence describes the legal binding line as an arity refusal.

---

## 2. Invariant analysis

### What I attacked and what held

- **Lifecycle pair: add a binding / remove a binding.** Complete. *Add* is `binding …`; *remove* is the line's absence, with R9(a)'s `BindingRetraction`, a stated `(app, glade_id)` scope, the "not loaded ⇒ retracts nothing" rule in §4.4 bullet 5, and three tests at row 10 asserting 14 live / 7 untouched / exactly one retracted. The phrasing residual I raised is fixed in place: bullet 5 now says "the **retraction rule**, not the retract form", with the reason. **Held.**
- **Lifecycle pair: migrate a file / tell it migrated.** Complete and now *ordered*. R10(a) gives the file its own assertion (`v1`); §4.4's step 0 / step 4 make the assertion safe, and the object states the asymmetry correctly — an old node **accepts** a new token (`appdecl.rs:137-138` stores raw) and **refuses** a new header (`:89-95` exact equality, checked before the binding arm opens). Row 17 pins it. **Held.**
- **Every option in R8–R11 has a stated default and a stated cost.** R8 → (b). R9 → (b)+(a), now with b1/b2 distinguished by a durable-cost column (0 / 13) and a recommendation that names the criterion for choosing between them. R10 → (a), with the extra landing step priced as its cost and (b)'s "no ordering hazard" named as its one genuine advantage. R11 → (a)-or-(c) conditional on R2 and R4, consistent with (c)'s stated precondition. **Held.**
- **Validation and documentation land together.** This was the through-line of both my NO-GO verdicts. §4.4 now has symmetric Token 4 and Token 5 blocks, bullet 2 has a branch for each of the three sub-choice answers including "(iii) this bullet does not land for that token" — and, correctly, notes that under (iii) "§4.4's documentation block above is still owed, because the meaning of the token is what the author needs whether or not the parser checks the spelling". That last clause is the habit actually broken. **Attacked, held.**
- **Refusals name their replacement in a spelling the format accepts.** Bullet 7 now branches per R9 shape. **Held.**
- **The grammar the author copies from moves with the line.** R11's Docs column, R10's Docs cell, bullet 12, row 19. **Held**, with SUR-P3-10 against that block's completeness — a different root cause from the one I raised.
- **Counts and citations.** The census, the five-files claim, the `0 2 0 0 0` census, `glade-gyld/README.md:499` and `:1147`, the 7+7=14 arithmetic, the 3-of-5 in-file-comment property, the no-header-comment fact: I re-ran or re-traced every one and all are correct at this tuple. Each count now carries the SHA it was taken at. **Held.**
- **Dual maintenance, token order, line-numbered diagnostics, the `atom` tier, `seed`/`service`/`workspace` line-style consistency.** Re-verified unchanged and still correct. **Held.**

### First-day walkthrough, against the pages as the amendment would leave them

Assuming the recommendations: R9 (b1 or b2) + (a), R10(a), R11(a).

**A — author a file with one binding of each allowed shape (`value`, `log`, `swmr`, `crdt`).**

1. *Find the format.* `glade/README.md:23` advertises `docs/` as user-facing; it holds an 8-line placeholder that forbids moving internal drafts in. The grammar lives in a page classified "Internal engineering design" and titled after a different subject. §4.4 still says the block "should move to, or be mirrored in, a page presented as user-facing", and revision 3 strengthens it ("that page, not this document, is where the conventions in bullets 13 and 14 belong") — but names no page, no §4 step and no §4.7 row. **Guess #1, carried from round 1.** Not filed: the RemPlan never made it a blocking disposition, and every round has recorded it.
2. *Header.* `GladeGrazelAttachNotes.md:30` moves to `v1` under bullet 12, gated by row 19. **Resolved** — this was round 2's Guess #2.
3. *`value` binding.* Id → §4.2 step 6 requires the authored-or-derived answer and the syntax rule. Authority, zone → published and glossed. Retention → "a settings or status value read at the head → `latest`". **Resolved.**
4. *`log` binding.* "an append log or an output stream → `from_cursor`". **Resolved** — this was round 2's hazardous guess, and it is the single most valuable line revision 3 adds.
5. *`swmr` binding.* Shape published; profile omission legal and glossed. Retention: no row. **Guess #2** (SUR-P3-11).
6. *`crdt` binding.* Shape published; `shape-profile=text_crdt` mandatory, refused at parse if omitted, values published; tail syntax now in the grammar and the three in-file comments. Retention: no row, and `latest`'s own gloss reads as its negation. **Guess #3** (SUR-P3-11).
7. *Make it routable.* My file needs `workspace ws-razel razel`; the published grammar never mentions the directive. **Guess #4 — I would not have written the line**, and the file would parse, register and not route (SUR-P3-10).

**Four guesses, down from six.** No dead ends, for the second round running.

**B — change a line, delete a line, migrate an old file.**

- *Change a line.* Answered under all four R9 shapes, with the durable cost per shape and row 9 asserting the number the chosen option claims. **Resolved.**
- *Delete a binding line.* Answered by R9(a), scoped, three tests. **Resolved.**
- *Delete a `service` or `workspace` line.* Unanswered (SUR-P3-12). **One guess.**
- *Migrate.* Under b1/b2: zero edits on 13 of 15 lines. `windowed` refused with a message naming a spelling the format accepts. Step 0 / step 4 make the header move safe, and every intermediate commit is a gate. **Resolved.**
- *Tell it migrated.* The `v1` header. **Resolved.**
- *Verify offline.* Still no `--check`, lint or dry-run for an app file; the only verification is booting a node. Unchanged across all three rounds and not worsened by this amendment. **Recorded, not filed** — the same call I made in round 2, held for consistency.

---

## 3. Risks and next action

Revision 3 closes every open finding on this axis, and it closes them the way the process is supposed to work: each correction is at the site the closure map names, each carries the counterexample that produced it, each has a §4 landing step and a §4.7 gate, and each correction the drafter made to my own work — the 3-of-5 in-file-comment property, the "five repos" slip, the elided front-page quote — is marked in place rather than absorbed. The two facts the drafter put to me for judgement are both correct and both strengthen the closures they support. My round-2 pre-commit is honoured and I hold to it: **GO**.

The residual risk on this axis is small and concentrated in one place: **the page an author actually reads still does not exist.** Every correction in this revision is a requirement laid on `GladeGrazelAttachNotes.md`, `GladeDeclSurface.md` and `glade-decl/README.md` — an internal engineering note titled after a different subject, a design document, and a contract front page. The amendment says the grammar "should move to, or be mirrored in, a page presented as user-facing" and stops there, while the only page presented as user-facing carries a standing rule against receiving it. That is Guess #1 in every walkthrough I have run, across three rounds. It is not a blocker and I do not file it again, but it is the thing that will still be true after this amendment lands, and it is where the next increment's value is.

Two of the four new P3s are worth folding into the freeze commit rather than carried:

- **SUR-P3-10** costs one line in a block §4.4 bullet 12 already opens, and without it the specification produces a non-routing app for exactly the third-party author R10 exists to protect.
- **SUR-P3-11** costs two rows in a list §4.4 already writes, and it is the last remaining authoring guess — on `crdt`, the member this amendment adds.

**SUR-P3-12** and **SUR-P3-13** are a scope sentence and a single word; they can ride any later edit.

**Next action for the lane owner.** Nothing on this axis blocks the verdict merge. If a revision 4 is drafted for another axis's sake, add the `workspace` directive to the published grammar block (SUR-P3-10) and the `swmr`/`crdt` rows to the retention rule (SUR-P3-11); otherwise record all four P3s as freeze-commit riders on the amendment itself. The rulings remain the owner's; nothing in this report asks for a different answer to any of them.
