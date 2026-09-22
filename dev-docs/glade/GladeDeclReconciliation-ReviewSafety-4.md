# GladeDeclReconciliation (revision 4) — SAFETY-AXIS REVIEW, ROUND 3 RE-VERDICT

**Review object:** `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclReconciliation.md`, **revision 4** (2257 lines), committed at glade-wz root `3b60234` and unchanged since — `git -C /Users/owebeeone/limbo/glade-wz log 3b60234..HEAD -- dev-docs/glade/GladeDeclReconciliation.md` is empty. Reviewed 2026-09-23.

**Baseline:** glade-wz root `3b60234`. The three commits since my round-3 tuple `c26211d` are documents only: `2bafc02` (the three re-verdicts, verbatim), `b1d3e04` (`-RemPlan-3.md`), `3b60234` (revision 4). The member lock is unchanged from my round-2/round-3 baselines, verified at start and again at end: `glade 559cb2c87e85` · `glade-decl d671f10c13e6` · `glade-decl-rs 21eefa1c3a53` · `glade-decl-ts 7e16e324630a` · `glade-decl-py 1b0f6d1f7886` · `taut 7a5f616c3a9f` · `glial 0dfe4b930063` · `grazel c66f029ad060` · `glade-gyld 65da8cb7e2b7` · `glade-gwz e53c87dddb8f` · `glade-chat 9238d21f6a36` · `grip-core 97ff6c26f12e` · `taut-shape 9a752094dbed` · `gryth-ui 9323818a39d2`. Root and every member clean (`glade-discover`'s working state remains out of scope). `AgentProcessRules.md` still has 0 commits since `ff431743cc4c`. One read-only command run: `corpus/build.py --check` in `glade-decl/` → green, exit 0, `git status --short` empty in `glade-decl` and `glade-decl-rs` afterwards. No writes, no builds, no test runs, no mutating git.

**Date:** 2026-09-23

**Axis:** Safety — what the text permits to go wrong. Independent, adversarial, read-only. Peer-blind: I read `-RemPlan-3.md` and my own `-ReviewSafety-3.md`, and no other axis's report of any round.

**Verdict: GO** — 0 × P0, 0 × P1, 0 × P2, **1 × P3** open. SAF-P2-12 is CLOSED on all five items, verified cell by cell; all three round-3 observations are CLOSED. The one new finding is a unit clause on a test specification, it does not block under the severity contract, and it is **not architectural** — I state that explicitly, because the round cap turns on it. This discharges my round-3 pre-commitment: "I pre-commit to GO on a revision that resolves SAF-P2-12 as specified."

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| **SAF-P2-12** — R9's durable-cost column counts only the spelling churn and omits the R2-forced `windowed` edits; §4.7 row 9 turns those numbers into a gate assertion | ACCEPT, all five items. (1) row (b1) → 2 under R2(a)/(c)/(d), 0 under R2(b), benefit kept; (2) row (b2) → 15 and 13; (3) row (a)'s "every R2 option" → "except (b)"; (4) §4.4 bullet 6 and §4.7(b) row 9 take the same conditional numbers; (5) the cost stated as a derivation with the census as input and one pair worked. Recommendation re-read; bullet 5's "under b1 no record moves" → "only the two `windowed` records move". Closure test: row 9's assertion list agrees cell by cell with R9's column for each (R2 × R9) pair; ≥1 pair worked census→number; {R2(a), b1} = 2 and {R2(a), b2} = 15 in both places | **All five items re-traced.** **(1)** `:926` reads "**2** under **R2(a)/(c)/(d)** … and **0** under **R2(b)**", marked *(corrected — SAF-P2-12)*, with the genuine benefit retained verbatim ("no stored byte changes for the 13"). **(2)** `:927` reads "**15** under **R2(a)/(c)/(d)** (13 normalised + 2 `windowed`) and **13** under **R2(b)**", marked. **(3)** `:925` reads "**every R2 option except (b)**", citing R2(b)'s own "0 meaning edits" cell, which I re-read at `:591`-equivalent in this revision. **(4)** §4.4 bullet 6 (`:1830-1841`) and §4.7(b) row 9 (`:2079`) both carry the conditional pairs. **(5)** The derivation at `:930-964` states the formula, grounds every step in code I re-verified at this tuple (`appdecl.rs:264` the diff, `:267` the append, `glade-node.rs:91` the persist, `:138` `toks[5]`→`retention`, `:37` the `crate::sysdata` import, `sysdata.rs:135` tag 6, `registry.rs:97` the dispatch), takes the census as input, and works {R2(a), R9(b1)} = 2 + 0 = **2** and {R2(a), R9(b2)} = 2 + 13 = **15**. **Cell-by-cell check, all four pairs:** (b1) column 2/0 ↔ row 9 2/0 ✓; (b2) column 15/13 ↔ row 9 15/13 ✓; (a)/(c) with respell, column 15/13 ↔ row 9 15/13 ✓. {R2(a), b1} = **2** in both; {R2(a), b2} = **15** in both. The recommendation (`:1040-1047`) is re-read honestly: the sub-recommendation stands, the spread is restated as "**2 against 15**, not 0 against 13", the 13-append criterion is unchanged, and the "costs nothing durable" gloss is withdrawn in terms. §4.4 bullet 5 (`:1800-1808`) now reads "Under **b1** only the two `windowed` records move", marked, and says those two are owed the same answer the 13 are owed under b2 | **CLOSED** (the column's *unit* is a separate, new matter — **SAF-P3-13**) |
| Observation 1 — §4.2's deletion bullet says "nine key names" where it designates ten | Rider CON-P3-10: "nine" → "ten", saying what it counts; extend §1's numeric assertion to cardinalities expanded through a group name | `:1365` reads "All **ten** key names this bullet designates", with the counting convention stated (`Retention` counted once as a key, not again as the message name it shares). I re-enumerated all ten against `corpus/decl.v0.json` at this tuple: every one present. §1 `:315-324` adds the strengthened rule — "a bare number naming the cardinality of a set its own sentence enumerates must equal that set's size, **expanded through every group name the sentence uses**" — and names the class the old rule missed. This generalises past the instance, which is more than the observation asked for | **CLOSED** |
| Observation 2 — §4.7(c) row 14's "row 16" reads ambiguously inside its own table | Rider: row 14's "row 16" reads "§2d row 16" | `:2092` reads "which is exactly **§2d row 16**'s divergence (SAF-P2-4) — `ZoneKind.private=1`, 'DIVERGENT BETWEEN BINDERS', not §4.7's own row 16 directly below", marked | **CLOSED** |
| Observation 3 — §4.4 step 4 does not repeat bullet 6's dual-maintenance rule for the header | Rider: step 4 repeats it | `:1569-1577` adds "**And the header is dual-maintained exactly as the tokens are**", names both `grazel-app.glade` homes, requires both headers in the same commit, and correctly classifies the consequence — "drift between two files that are required to be identical, not a boot failure — a both-headers node loads either — which is why it is a rule of this step and not a gate of it". That classification is right and matches what I found when I walked the interleaving in round 3 | **CLOSED** |

---

## Changed-range analysis

`git diff c26211d..3b60234 -- dev-docs/glade/GladeDeclReconciliation.md` → **302 insertions, 47 deletions**; 1373 → 2257 lines across four revisions, 2002 → 2257 here. I surveyed every hunk header and mapped each to `-RemPlan-3.md`:

| Hunk region | Disposition |
|---|---|
| `:7-88` revision-4 banner and closure map | the round's own record |
| `:315-324` §1's strengthened numeric assertion | rider CON-P3-10 |
| `:925-927` R9's three corrected cells; `:930-964` the derivation | SAF-P2-12 items 1–3, 5 |
| `:990-1012` R9's `dir.bindings` scope and the priced option **(s)** | rider SUR-P3-12 |
| `:1026-1047` the re-read recommendation | SAF-P2-12's recommendation clause |
| `:1365`, `:1410` §4.2's ten-count and the relabelling cross-reference to §4.1 item 1 | riders CON-P3-10 and the Consistency residual |
| `:1518-1527` the census command's working directory | Consistency residual |
| `:1569-1577` step 4's dual-maintenance rule | Safety observation 3 |
| `:1635-1680` the token-5 block, indexed per `BINDING_SHAPES` member | rider SUR-P3-11 |
| `:1799-1819`, `:1830-1841` §4.4 bullets 5 and 6 | SAF-P2-12 item 4, riders SUR-P3-12/13 |
| `:1898-1917` §4.4 bullet 12's grammar block | rider SUR-P3-10 |
| `:2065-2092` §4.7's new R8 row, row 9, row 14 | SAF-P2-12 item 4, Consistency residual, observation 2 |
| `:2212` the Appendix's gwz-dev re-pin | Consistency residual |

**Nothing falls outside the plan.** I also checked the four prohibitions under "What the revision may not do". *Recommendations:* I diffed every `**Recommendation:` line between `c26211d` and `3b60234` — **byte-identical**, so no ruling outcome or recommendation moved; R9's sub-recommendation was re-read and kept, which the plan permitted either way, and the re-read is argued from the corrected spread rather than asserted. *One file:* the commit touches only the object. *No gate weakened:* row 9's assertion is made stricter (conditional on R2 as well as R9, with an explicit instruction to read it off the derivation "never off memory"), and §4.7 gains a row rather than losing one. *Corrections marked, not deleted:* every one I checked carries its `*(corrected — …)*` mark with the superseded wording quoted — `:925`, `:926`, `:927`, `:1802`, `:1838`, `:2079`, `:1571`.

**On option (s).** R9 gains a priced symmetry option for `service`/`workspace` retraction (`:1003-1012`). The plan authorised it ("with symmetry offered as a new option under R9 if the owner wants it, priced there"). On my axis I checked two things: that it does not create a gap in the very column SAF-P2-12 was about — it does not, because (s) is offered in prose beside the table rather than as a fifth table row, and it adds retraction record kinds rather than first-boot appends; and that its cost is honestly stated — it is, including the part that matters most at a freeze: "**two more record kinds frozen at the same publish**, which is the part that is expensive to defer and cheap to decide now". It is marked "offered, not recommended: the recommendation above is unchanged". I verified its two supporting citations: `appdecl.rs:176` does refuse a duplicate workspace share, and `GladeGrazelAttachNotes.md:85-88` does publish `seed`'s remove half via revocation-wins. The accompanying statement that a deleted `service` line retracts nothing and "a **retired exchange stays routable**" is correct against `exchange.rs:62-78`, which scans every op and returns on first match.

**NEW root cause: one, and it is NOT ARCHITECTURAL.** I state the classification plainly because this is the third and last permitted round and the lane stops on an architectural finding. SAF-P3-13 is a missing scoping clause on one cell of one table: row 9 names a tree-wide integer where the test it specifies observes a per-registry share. It requires no ruling to change, no section to be rebuilt, no landing step to move, and no system property to differ. The instrument is correct, the derivation that feeds it is correct and worked, and the document already states the unit in the paragraph row 9 sends the reader to; what is missing is the last division. Nothing in revision 4 reopens an architectural question, and nothing in my four rounds on this object ever has.

---

## 0. Evidence base

Object: revision 4, read at the sections the diff touches plus R9, §4.2, §4.4 and §4.7 in full; the remainder read in full at revisions 2 and 3.

Re-verified at this tuple: `appdecl.rs:37, :138, :176, :264, :267` · `sysdata.rs:135` · `registry.rs:97` · `glade-node.rs:91` · `exchange.rs:62-78` · `GladeGrazelAttachNotes.md:85-88` · `corpus/decl.v0.json`'s ten designated deletion keys · `corpus/build.py --check` green.

**Per-file census, run at this tuple** (the evidence behind SAF-P3-13):

| file | bindings | `from-cursor` | `windowed` | `latest` |
|---|---|---|---|---|
| `grazel/apps/grazel-app.glade` | 7 | 4 | 1 | 2 |
| `glade/apps/grazel-app.glade` (byte-identical twin) | 7 | 4 | 1 | 2 |
| `grazel/apps/gyld-app.glade` | 7 | 2 | 0 | 5 |
| `glade-gyld/tests/fixtures/gyld-test-app.glade` | 6 | 2 | 0 | 4 |
| `glade-gwz/tests/fixtures/gwz-test-app.glade` | 1 | 1 | 0 | 0 |
| **total** | **28** | **13** | **2** | **13** |

Both `windowed` lines are the same authored surface: `binding term.log  log   share commons windowed` at line **25** of each twin. The document's 28/13/13/2 census and its statement that the 2 are one surface twice are exact.

---

## 1. Findings

### [SAF-P3-13] R9's durable-cost column is a tree-wide count, but §4.7 row 9 asserts it of a single `Registry` — so the number the row tells an executor to assert is not observable in any run of that test

**Root cause.** The derivation's unit is a binding line of the tree-wide census; row 9's unit is `Registered{appended}` from one registry. The document states the first unit and does not convert it into the second at the point where it is asserted.

**Location.** §3 R9's derivation, `:956-959`. §4.7(b) row 9, `:2079`.

**Reproduction.** The derivation says, verbatim: "The unit is a binding line of that census, i.e. the tree-wide cost of the migration: `grazel-app.glade`'s two homes are byte-identical and the two fixtures are test files, so any one store sees only its own loaded files' share of the number." Row 9 says: "register the **pre**-amendment parse into a `Registry`, then the **post**-amendment parse; assert … **The asserted `appended` is the chosen option's own stated durable cost and must equal it** … Under **R2(a)/(c)/(d)**: **2** under (b1), **15** under (b2)."

Against the per-file census above, under the recommended {R2(a), R9(b1)} a test registering `grazel-app.glade` observes **1** append — its single `term.log` line — where row 9 says assert 2. Under {R2(a), R9(b2)} the same test observes 4 + 1 = **5**, where row 9 says assert 15. Nor is the tree-wide number reachable by widening the test: registering all five files into one registry cannot produce 15, because the twin's seven records are byte-identical to the first copy's and register as `unchanged` — the maximum any single registry can observe under b2 is 4 + 2 + 2 + 1 + 1 = **10**.

**Impact.** Bounded, and materially smaller than the round-3 defect it descends from. The executor who follows row 9's own instruction — "read it off R9's derivation paragraph, never off memory" — reads the share sentence in the same paragraph as the number, so the information needed to scope the assertion is present at the point of use; the recommendation at `:1031-1032` independently names the two as "`term.log`'s `windowed` line in `grazel-app.glade`'s two byte-identical homes". What is missing is the instruction to divide. The concrete consequence is that a careless reading asserts 2, observes 1, and lands at the moment row 9's own parenthetical warns about — "the one place a red gate invites an executor to adjust the assertion to observed behaviour". That is a documentation defect with a concrete consequence, not a correctness defect in the cost model: the model is right, the derivation is right, and every integer in the column is right for the quantity it names.

**Required correction.** One clause on row 9: the asserted `appended` is **the share of the stated cost contributed by the files this test registers**, with one worked share beside the worked total — e.g. under {R2(a), R9(b1)}, tree-wide 2, and 1 for a test registering one copy of `grazel-app.glade`. Optionally add the twin's role in one clause, since it is the whole of the difference between 2 and 1. This does not need a further revision of this document: it can be carried into the amendment's own commit when row 9's test is written, and the derivation paragraph already contains everything the author needs.

**Closure/regression test.** Row 9's number and the derivation's number differ by a stated factor, and that factor is named — the test's file set. Checked by inspection: row 9 says which files it registers and what their share is.

---

## 2. Invariant analysis

**Attacks that failed** — revision 4 earns these:

- **The derivation is a genuine derivation, not a restatement.** It grounds every hop from `toks[5]` to the append in code I re-verified, names the two terms by which ruling fixes each, explains why the 13 `latest` lines are in neither term under any option, and works two pairs end to end. It also explains the ceiling ("which is why no cell exceeds **15**"), which is the kind of check that catches a future drift.
- **The numbers are now right and consistent in all four sites.** Column, row 9, bullet 5 and bullet 6 agree across all four (R2 × R9) pairs. I looked specifically for a fifth site carrying stale integers and found none.
- **The recommendation was re-read rather than re-asserted.** `:1040-1047` states the corrected spread, keeps the criterion on the unchanged 13-append gap, withdraws the "costs nothing durable" gloss in terms, and gives R2(b)'s spread too. That is the honest handling of a recommendation whose supporting number moved.
- **Option (s) is priced, scoped and explicitly not recommended**, and its freeze cost — two more durable record kinds at the same publish — is the fact a safety reviewer most wants stated before a one-shot freeze.
- **The three observations are closed beyond their letter.** The "nine"→"ten" fix came with a strengthened mechanical-pass rule that generalises the class; step 4's dual-maintenance clause correctly classifies itself as a rule rather than a gate, which is the distinction §4.7's (a)/(b)/(c) split exists to preserve.

**Everything I closed in earlier rounds remains closed.** I spot-re-traced the two with the largest blast radius: `appdecl.rs:89-95` is unchanged and §4.4's step 0 still precedes every header move, so the SAF-P2-9 interleaving has no reachable stuck state; and §4.2's `--compat` specification and superset language are untouched by this diff, so SAF-P2-10 stands closed.

---

## 3. Risks and next action

**Residual risk carried into the freeze, on this axis.** One: SAF-P3-13, above — row 9's assertion is in tree-wide units and its test observes a per-registry share. It costs one clause, it is self-revealing on first run, and the paragraph that resolves it is the one row 9 already points at.

**Single next action.** None blocking. When the amendment's row-9 test is written, scope the assertion to the files it registers and record the share beside the total.

**On the round cap and the verdict.** This was the third and last permitted remediation round, confined to non-architectural corrections. The one blocking finding it carried is closed on all five items, verified cell by cell with the closure test met exactly; the three observations are closed; nothing in the diff falls outside the plan; no ruling outcome or recommendation moved; and **no architectural root cause exists in revision 4**. Across four revisions this axis has gone P1/P1/P2×3 → P2×3/P3 → P2 → P3, with the defect surface narrowing each round from "an undefined operation on durable records" to "a missing division on one test's unit". That is a convergent process, and it has arrived. **GO.**
