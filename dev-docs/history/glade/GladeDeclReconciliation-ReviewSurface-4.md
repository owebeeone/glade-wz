# GladeDeclReconciliation (revision 4) — SURFACE-AXIS REVIEW, ROUND 3 RE-VERDICT

**Review object:** `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclReconciliation.md`, **revision 4**, committed at glade-wz root `3b60234` and unchanged since (`git log 3b60234..HEAD -- dev-docs/glade/GladeDeclReconciliation.md` empty). 2257 lines. DRAFT design, unimplemented. Root working tree clean. This is a re-verdict **restricted to the diff `c26211d..3b60234` within this axis's permitted sections**, to re-affirm or withdraw my revision-3 GO so that GO from all three axes stands on one revision.

**Baseline:** glade-wz root `3b60234c6938` · glade `559cb2c87e85` · glade-decl `d671f10c13e6` · glade-decl-rs `21eefa1c3a53` · glade-decl-ts `7e16e324630a` · glade-decl-py `1b0f6d1f7886` · taut `7a5f616c3a9f` · glial `0dfe4b930063` · grazel `c66f029ad060` · glade-gyld `65da8cb7e2b7` · glade-gwz `e53c87dddb8f` · glade-chat `9238d21f6a36` · glade-discover `fd94a1f87bbc` · grip-core `97ff6c26f12e` · taut-shape `9a752094dbed` · gryth-ui `9323818a39d2`. **The member lock is unchanged from my round-2 and round-3 baselines**, verified identical at start and at end. The three root commits since `c26211d` are documents only (`2bafc02` the three round-3 reports, `b1d3e04` the round-3 plan, `3b60234` revision 4). `gwz-dev/dev-docs/AgentProcessRules.md` re-verified byte-unchanged since `ff431743cc4c`.

Sources read at this tuple: the object's "Revision 4 (2026-09-23)" block and closure map (`:28-62`); §2m; §3 R8–R11 (`:871-1128`), including R9's new "How the durable-cost column is computed", corrected cost cells and "R9's options govern `dir.bindings` only"; §4.2 step 6; §4.4 in full (`:1506-1969`); §4.6; §4.7 rows 5–10, 17–21 and the (a)/(b)/(c) headings. `-RemPlan-3.md`; my own `-ReviewSurface-3.md`. The five app files and the eight user-facing pages; the one permitted grep of `appdecl.rs`'s diagnostic strings. **I opened no other axis's report of any round**, no code, and no design or plan document beyond the above. Read-only throughout.

**Date:** 2026-09-23

**Axis:** SURFACE — the `<app>.glade` file format as the person typing and editing it meets it. Independent, adversarial, read-only. Filed verbatim by the lane owner.

**Verdict: GO — re-affirmed on revision 4.** 0 P0, 0 P1, 0 P2, **0 new P3**. **All four round-3 findings CLOSED**, each by re-tracing its counterexample against the pages as the amendment would leave them. Every hunk in my sections traces to a disposition or an accepted rider; none falls outside. **No NEW root cause of any kind, and no NEW ARCHITECTURAL root cause** — I state again, for the record and for the cap, that **none of my findings in any of the four rounds is an architectural root cause of this object.** The authoring walkthrough is down to **one** guess, and that one is the standing residual `-RemPlan-3.md` deliberately declines to make a disposition.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| **SUR-P3-10** — the published grammar omits the `workspace` directive every app file uses | Rider: §4.4 bullet 12's edit of `GladeGrazelAttachNotes.md:29-36` adds `workspace <share> <name>` with one clause on what it does | §4.4 bullet 12 gains "**The same edit completes that block, which omits a directive every app file uses (SUR-P3-10)**", instructing the addition **with the exact clause I asked for**: "it is the line that makes a declared surface routable". Grounds re-verified by me: `parse()` prints its own template at `appdecl.rs:173` (`` `workspace <share> <name>` ``, seen in my permitted diagnostic grep); `grep -c '^workspace '` → **1** in each of the five files; the three in-file comments do carry `# workspace <share> <name>` (`grazel-app.glade:47`, its twin, `gyld-app.glade:64`), so the object's "a copier is safe and only the published page misleads" is exactly right. **My closure test re-run:** after this edit the block publishes the header plus all five directives `parse()` accepts — `app`, `binding`, `service`, `seed`, `workspace` — and a file written from it alone routes | **CLOSED** |
| **SUR-P3-11** — the retention rule had three rows against a four-member `BINDING_SHAPES` | Rider: index the rule per `BINDING_SHAPES` member as bullet 4's omission rule is; for `crdt`, either state a value or state that the shape does not determine the retention and what does; the answer must not be contradicted by `latest`'s gloss | §4.4's "**Which to write, per `BINDING_SHAPES` member**" is re-indexed by shape: `value` → `latest`, `log` → `from_cursor`, `swmr` → `from_cursor`, and for `crdt` **the second permitted limb** — "the shape does not determine the retention, and what does is how the surface is read: `from_cursor` if a subscriber resumes a history, `ttl` if its entries expire". That is the honest limb, and the object says why the first was unavailable: **zero `crdt` lines exist**, so the corpus cannot supply a precedent. My "must not be contradicted by `latest`'s gloss" clause is answered head-on — "the page must say in terms that `latest` does not mean 'the merged value'". Every census figure re-verified by me (below) | **CLOSED** |
| **SUR-P3-12** — R9 supplies a retract half for `binding` only | Rider: one sentence in R9 scoping it to `dir.bindings`; one in §4.4 bullet 5 on what a deleted `service` or `workspace` line does today; symmetry offered as an option, priced | §3 R9 gains "**R9's options govern `dir.bindings` only (SUR-P3-12)**", which walks all four adding directives' remove halves — `binding` answered by (a) with row 10's three tests; `seed` already answered and published (`GladeGrazelAttachNotes.md:85-88`, which I re-read and which does state revocation-wins at `(principal, share)`); `service` and `workspace` "**not answered, and not answered by this amendment either**" — and adopts my own framing that this is pre-existing and becomes worth stating only because `binding` is about to acquire the first remove half. §4.4 bullet 5 carries "**What a deleted `service` or `workspace` line does today: nothing**", with "a **retired exchange stays routable**". Symmetry is priced as option **(s)** — two more record kinds, the same scope rule, two folds, `--legacy-codec`, ~200 LOC — and explicitly "offered, not recommended: the recommendation above is unchanged", which is the draft-stage rule honoured. **My closure test re-run:** for each of the four adding directives a reader of the page can now say what deleting the line does | **CLOSED** |
| **SUR-P3-13** — the omission bullet called a five-token line an arity refusal | Rider: "a four-token line", or one stated counting convention used throughout §4.4 | The bullet now reads "a **four**-token line is an arity refusal", and the object took **both** limbs: it also states "**The counting convention throughout §4.4 is tokens *after* `binding`**", grounded on `appdecl.rs:111-114` testing `toks.len() != 6` — "six whitespace tokens including the directive, five after it". The correction is marked in place and lists the five other sites where a five-token line is the legal one; I re-verified four of them (R11's question, R11(c), bullet 4, bullet 12) and did not open the fifth (§5, outside my sections). Consistent with §2m A11's "six whitespace tokens (`binding` + 5)", which is unchanged. **My closure test re-run:** no sentence in §4.4 describes the legal binding line as an arity refusal | **CLOSED** |

---

## Changed-range analysis

`git diff --stat c26211d..3b60234` → **+302 / −47**, 2002 → 2257 lines, across 23 hunks. Every hunk traced:

| Hunk (old → new) | Section | Traced to |
|---|---|---|
| `-3 +3` | doc status line | revision 3 → revision 4. Bookkeeping. |
| `-7,13 +7,56` | Review-status + **Revision 4 block and closure map** | The plan's required revision block. **Within.** |
| `-45 +88` | Revision-3 block, SAF-P2-11 closure row | Word-diff shows the stale durable-cost triple `(0 / 13 / 15)` removed and the row re-quoted — SAF-P2-12's marked-in-place correction of an older block. **Within** (another axis's finding; traced, not opined on). |
| `-271,0 +315,11` | §1 "Revision 3 mechanical passes" | CON-P3-10's second item. **Outside my permitted sections** — not opened, not opined on. |
| `-871,3 +925,3` · `-875,0 +930,36` · `-916,2 +1030,3` · `-924,0 +1040,8` | **§3 R9** — cost cells, "How the durable-cost column is computed", recommendation re-read | SAF-P2-12 items 1–3 and 5. **Within.** Another axis's blocking finding; I traced the edits and did not adjudicate the integers, except to confirm they are consistent with the census I re-ran (13 `from-cursor`, 2 `windowed`, 28 lines) and that R9's recommendation is *kept and re-argued*, not silently changed — which is what `-RemPlan-3.md` permits. |
| `-899,0 +990,24` | **§3 R9** — "R9's options govern `dir.bindings` only" + option (s) | **SUR-P3-12.** Within. |
| `-1242,2 +1365,11` · `-1278 +1410,9` | §4.2 **Compatibility** | CON-P3-10 and a Consistency residual. **Outside my permitted sections** (only §4.2 **step 6** is mine) — not opened. **§4.2 step 6 itself is untouched**, verified. |
| `-1378 +1518` · `-1380 +1520,7` | §4.4 landing order — census working directory | Consistency residual. **Within.** I reproduced it: from `glade-wz` the command returns **five**; from `/Users/owebeeone/limbo` it returns **six**, the sixth being `taut-dev/glade/apps/grazel-app.glade`, a third checkout outside this tuple. The note is exactly right. |
| `-1423 +1569,9` | §4.4 **step 4** — header dual-maintenance | Safety observation. **Within.** Correct on my axis's facts: the twins are byte-identical and both headers sit at `:16`. |
| `-1481,6 +1635,45` | §4.4 **Token 5** — per-`BINDING_SHAPES` rule + omission bullet | **SUR-P3-11 + SUR-P3-13.** Within. |
| `-1606,5 +1799,21` · `-1625,2 +1834,8` · `-1638 +1853,3` | §4.4 bullets 5 and 6 — b1's two `windowed` records, deleted `service`/`workspace`, the corrected cost derivation | SAF-P2-12 item 4 + **SUR-P3-12**. Within. |
| `-1680,0 +1898,20` | §4.4 bullet 12 — the `workspace` directive | **SUR-P3-10.** Within. |
| `-1805 +2042,2` · `-1827,4 +2065,9` · `-1836 +2079` · `-1841,0 +2085` | §4.7 — (b) preamble corrected for R8, new **row 21** | Consistency residual. **Within.** |
| `-1848 +2092` | §4.7(c) row 14 — "§2d row 16" disambiguation | Safety observation. **Within.** |
| `-1967,0 +2212,11` | Appendix — gwz-dev re-pinning | Consistency residual. **Outside my sections** — though I independently re-verified its factual claim (`AgentProcessRules.md` unchanged since `ff431743cc4c`). |

**Unchanged in my sections, verified by grep over the diff:** §2m (A9–A12 rows untouched), §4.6 (body untouched), §4.2 step 6 (untouched), §4.7 row 19 and row 20 (untouched) — so the three gates that carry SUR-P2-1, SUR-P2-6 and SUR-P2-8 stand exactly as I closed them on revision 3.

**Nothing falls outside `-RemPlan-3.md`'s dispositions and riders.** The plan's "What the revision may not do" holds on my sections: no ruling outcome changed; R9's sub-recommendation is re-read against the corrected spread and *kept with its criterion re-argued*, which the plan expressly permits; no gate weakened; every corrected false claim marked in place rather than deleted; no file but the object touched.

**NEW root causes: NONE. NEW ARCHITECTURAL root causes: NONE.** I looked specifically for one, knowing this third round is permitted only for non-architectural corrections and that finding one here stops the lane. Every hunk in my sections is a sentence, a table cell, a number or a directive line added to a document; not one implies a mechanism, a design change or a different shape for any ruling. The only genuinely new *content* on my axis — R9's option (s) — is offered and explicitly not recommended, priced rather than assumed, which is the correct draft-stage treatment of a choice.

### Authoring walkthrough, re-done

**A — author a file with one binding of each allowed shape, including the `workspace` line.** Find the format: still no user-facing page named — **Guess #1**, the standing residual. Header: `:30` moves under bullet 12, gated by row 19 — resolved. `value` → `latest`, `log` → `from_cursor`, `swmr` → `from_cursor`: all three now stated per shape with their census grounding — resolved. `crdt`: shape published, `shape-profile=text_crdt` mandatory and refused at parse if omitted, tail syntax in the grammar, retention answered by what determines it plus the explicit anti-`latest` instruction — **resolved** (was Guess #3 last round). `workspace ws-razel razel`: now in the published grammar with the routability clause — **resolved** (was Guess #4).

**B — change, delete, migrate.** Change a line: R9 plus the derivation — resolved. Delete a `binding` line: R9(a), scoped, three tests — resolved. Delete a `service` or `workspace` line: now answered in bullet 5 — **resolved** (was a guess). Migrate: R10(a), bullet 7's per-shape messages, step 0/step 4 with the dual-maintenance rule now stated for the header too — resolved. Tell it migrated: the `v1` header — resolved. Verify offline: still only by booting a node; unchanged across four rounds, not worsened, and recorded rather than filed, as in every round.

**Guess count: 1.** Round 1: five guesses and two dead ends. Round 2: six. Round 3: four. Revision 4: **one**, and it is the one `-RemPlan-3.md` consciously left standing.

---

## 0. Evidence base

Everything re-verified at this tuple; the member lock has not moved since round 2, so my standing corpus facts hold unchanged: five files carry `glade-app v0`, 28 binding lines, 13 `from-cursor` / 13 `latest` / 2 `windowed`, 26 `commons` / 2 `private`, 28/28 `share`, 14 hand-typed ids, zero underscores in any authored token, three of five files carrying a grammar comment, none of five showing the header.

**The drafter's fact, put to me to judge — VERIFIED EXACTLY.** Cross-tabulating the census by `(shape, retention)`:

```
  13 value latest      11 log from-cursor      2 log windowed      2 swmr from-cursor      (28)
```

and **no `crdt` line of any retention** (`grep -c 'crdt'` → 0 in all five files). The two `swmr` lines are `grazel/apps/grazel-app.glade:23` and `glade/apps/grazel-app.glade:23`; the two `windowed` lines are `:25` in the same twinned pair. So each is **one authored surface counted twice through the dual-maintained twin**, exactly as reported — and this corrects my own round-3 wording, which called `grazel-app.glade:23` "the sole `swmr` line". The object states it correctly ("one authored line, dual-maintained"), and the arithmetic it draws from it — 13 `value` all `latest`; 11 of 13 `log` lines `from_cursor` with the other 2 the `windowed` pair; both `swmr` lines `from_cursor` — is right in every figure.

Also re-verified: the census working-directory claim (five from `glade-wz`, six from the parent, the sixth being `taut-dev/glade/apps/grazel-app.glade`); `appdecl.rs:173` printing `` `workspace <share> <name>` ``; `grep -c '^workspace '` → 1 in each of the five files; `# workspace <share> <name>` present in the three commented files.

---

## 1. Findings

**None.** No P0, P1, P2 or P3 is raised against revision 4 on this axis, and no round-3 finding remains open.

I attacked the new text on its own terms before concluding this. The two places I pressed hardest: whether the `crdt` retention answer merely defers the question — it does not, because it states the discriminant (how the surface is read), gives both branches, and requires the page to dispel the `latest` reflex that made the gap dangerous; and whether bullet 12's `workspace` addition needed a §4.7 gate of its own — row 19 is scoped to the header and the tail under R10/R11 and does not cover it. I record that as an observation rather than a finding: the addition is unconditional, lands as a named §4 step in the bullet whose own edit opens the block, and is one line whose absence is visible in the block itself. Filing it would be padding at the last gate of a round convened for corrections, and neither my closure test nor the rider asked for a gate.

---

## 2. Invariant analysis

- **Lifecycle pairs, all four adding directives.** `binding`: add, change and remove, with scope and three tests. `seed`: remove half published and working. `service` and `workspace`: remove half absent, now **stated as absent** in both R9 and the page-facing bullet, with symmetry priced as option (s). The pair is complete where it can be and honest where it cannot. **Held.**
- **Every value of every validated token has a stated meaning, and every shape a stated default.** Token 4 and token 5 blocks are symmetric; token 5 is now indexed by the shape the author has just typed, matching bullet 4's omission rule; the one member with no corpus precedent is named as such rather than papered over. **Held.**
- **The grammar the author copies from is complete and moves with the line.** The published block now carries the header, all five directives and — under R10(a)/R11(a) — the moved header token and the tail; the three in-file comments move with it; row 19 gates it. **Held.**
- **Counting conventions and counts.** One stated convention for §4.4, grounded on the parser's own arity test; every count carries the SHA it was taken at; the census now cross-tabulates and distinguishes authored surfaces from dual-maintained lines. I re-ran every figure on my axis and all agree. **Held.**
- **Draft discipline.** Three places in this revision could have resolved a question by choosing: `crdt`'s retention, SUR-P3-12's symmetry, and R9's sub-recommendation under a corrected spread. All three lay the choice out and leave it to the owner, and the one that was re-read was re-argued rather than quietly adjusted. **Attacked, held** — this is the behaviour the process exists to produce.

---

## 3. Risks and next action

Revision 4 does exactly what a third, non-architectural round should: four one-to-few-sentence corrections, each at the site the closure map names, each carrying its counterexample, each marked in place. It adds no mechanism, changes no outcome, and introduces no new claim I could falsify. My revision-3 GO is **re-affirmed**, so a GO from this axis now stands on revision 4.

The residual on this axis is unchanged and, I think, correctly placed: **the page an author actually reads still does not exist.** Every correction across four rounds is a requirement laid on an internal engineering note, a design document and a contract front page. `-RemPlan-3.md` declines to make that a disposition and records it as the next increment's value rather than this freeze's — which I accept. It is not a defect in this amendment; it is the shape of the work that should follow it, and I would rather see it done properly once than bolted onto a freeze commit.

**Next action for the lane owner:** nothing on this axis blocks the verdict merge or the freeze. When the amendment is made, the format's grammar block, the `Retention` row and the members table are the three edits that carry almost all of this axis's value — they are named §4 steps and gated by rows 18, 19 and 20, and they should not be allowed to slip behind the code. The rulings remain the owner's; nothing in this report asks for a different answer to any of them.
