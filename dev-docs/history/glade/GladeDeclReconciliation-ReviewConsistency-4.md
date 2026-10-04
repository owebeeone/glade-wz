Tuple identical at start and end. Writing the re-verdict.

# GladeDeclReconciliation (revision 4) — CONSISTENCY-AXIS REVIEW, ROUND 3 RE-VERDICT

**Review object:** `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclReconciliation.md`, **revision 4**, 2257 lines, sha1 `fe4ce0284a8c59ad5125450094fd176d7c534633`; committed at glade-wz root `3b60234` and byte-identical in the working tree. Status "assessment + amendment proposal, **DRAFT**, revision 4", dated 2026-09-23. Remediation round 3, confined by `-RemPlan-3.md` to non-architectural corrections. This review is restricted to the diff `c26211d..3b60234` and to my axis's sections and riders, as the plan's "Re-verdicts" clause directs.

**Baseline (verified identical at start and at end):** glade-wz root `3b60234` · glade `559cb2c87e85` · glade-decl `d671f10c13e6` · glade-decl-rs `21eefa1c3a53` · glade-decl-ts `7e16e324630a` · glade-decl-py `1b0f6d1f7886` · taut `7a5f616c3a9f` · glial `0dfe4b930063` · grazel `c66f029ad060` · glade-gyld `65da8cb7e2b7` · glade-gwz `e53c87dddb8f` · glade-chat `9238d21f6a36` · glade-discover `fd94a1f87bbc` · grip-core `97ff6c26f12e` · taut-shape `9a752094dbed` · gryth-ui `9323818a39d2` · gwz-dev `9c0008870ecd`. The member lock is unchanged from my round-2 and round-3 baselines, so every citation I verified in those rounds still resolves; `gwz-dev/dev-docs/AgentProcessRules.md` is byte-unchanged since `ff431743cc4c`. Every tree clean except `glade-discover`, out of scope by the tuple. Sources read with `git -C <repo> show <sha>:<path>`. One read-only script run: `corpus/build.py --check` in `glade-decl/` → `all 3 glade-decl artifacts in lockstep with the schema.`, exit 0, `git status --short` empty in both repositories afterwards.

**Date:** 2026-09-23
**Axis:** Consistency — the document against its controlling graph: internal contradictions between sections, agreement with every contract/design it cites at the cited line, satisfiability of its own test and evidence sections, unstated impacts. Independent, adversarial, read-only. Peer-blind: I opened `-RemPlan-3.md` and my own `-ReviewConsistency-3.md`, and no other axis's report of any round.

**Verdict: GO** — 0 P0, 0 P1, 0 P2, 1 P3. **My round-3 GO is re-affirmed on revision 4.** CON-P3-10 and all four of my residuals are CLOSED, each re-traced on the new text. One new finding, CON-P3-11, is a coverage gap in a newly-offered, explicitly-not-recommended option; it is P3 and does not block. **No new architectural root cause** — stated in terms below, because this is the third round and the lane stops on that classification.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| **CON-P3-10** — §4.2's deletion bullet says "nine key names" where it designates ten | "nine" → "ten", saying what it counts (designated keys, `Retention` counted once as a key), or drop the count; extend §1's numeric assertion to cardinalities expanded through a group name | Re-traced both halves. §4.2's bullet now reads "All **ten** key names this bullet designates — the seven spelled out, plus the other three of the four `Retention` vectors (`Retention`, `edge/retention-latest`, `edge/retention-cursor`; the fourth, `edge/retention-ttl`, is already among the seven)", with the correction marked and both mis-readings named: the count is of designated keys, and "the count of backticked **names** actually written out in the bullet is eight — the seven keys plus `Retention` — which is the other reading, and it is not nine either". That is exactly the arithmetic I derived in round 3 (10 designated, 8 spelled), reproduced here against `decl.v0.json`: the seven spelled keys and all four `Retention`-typed vectors (`Retention`, `edge/retention-cursor`, `edge/retention-latest`, `edge/retention-ttl`) are present, union 10. §1 now carries the stronger assertion in the form I specified — "a bare number naming the cardinality of a set its own sentence enumerates must equal that set's size, **expanded through every group name the sentence uses**" — names the four group phrases it must expand ("all four `Retention` vectors", "the same 11", "the three in-file comments", "all 14 bindings"), says where it was run, and explains why the ten-line form missed the defect. | **CLOSED** |
| **Residual 1** — §4.4's census command has no stated working directory | Name it (inside `glade-wz`; a run from the workzones' parent also matches a third checkout and returns six) | §4.4's landing-order note now reads "run **with `glade-wz` as the working directory**", with an added parenthetical stating that a run from `/Users/owebeeone/limbo` "also matches `taut-dev/glade/apps/grazel-app.glade`, a third checkout outside this tuple, and returns **six** — the very number this correction exists to remove", and that `gryth-wz` contributes none. I re-ran all three scopings: `glade-wz` → the five enumerated paths; the parent → six, the sixth being exactly `taut-dev/glade/apps/grazel-app.glade`; `gryth-wz` → none. §4.4 bullet 6's closing `grep -rln` carries the same directory clause and cross-references the note. | **CLOSED** |
| **Residual 2** — §4.2 asks a relabelling question §4.1 item 1 already answers | Cross-reference §4.1 item 1 (keys survive, comments change) | §4.2's rename clause now adds "**For the live candidates, §4.1 item 1 has already said it**" and quotes it: keep the two vectors "as the relabelled recognition cases R3(a)'s table promises — **keeping their keys**, so the relabelling is a comment change and not a rename" — then draws the consequence, "so under R3(a) the deletion list stays empty on their account, which is what §4.7 row 2 and the recommended-set invocation below already assume", and reframes the surviving sentence as the general rule for future relabellings. I checked the quotation against §4.1 item 1 at `:1205-1209`: verbatim. I also checked that §4.1 item 1 is untouched by this diff and already carried those words at `c26211d:1082-1086` — so the fix is a cross-reference and not a retrofit, which is the right repair. | **CLOSED** |
| **Residual 3** — R8 is named in no §4.7 row | Add a §4.7(b) row on R8 for the `GladeDeclSurface.md` amendment landing | §4.7(b) row **21** added: "grep `dev-docs/glade/GladeDeclSurface.md` for the record-kinds sentence R8's answer requires, and `glade-decl/dev-docs/DeclSurface.md` for the mirror banner", with green defined under both R8 options and red defined as "a ratified document still asserts a file form the contract does not match". The (b) preamble is corrected **in place and marked**, per the plan's prohibition: "**R8's *corpus* gate set is (a)**" replaces "R8 appears in no (b) row", the old reason is kept with a note that it "now carries only the weight it can bear", and the closing sentence becomes "Every ruling, R1–R11, is named in at least one row below" — which I verified by enumerating the rulings named across rows 2, 7, 9, 10, 13, 17, 18, 19 and 21: R1–R11, all eleven. The row's grounds check out: `GladeDeclSurface.md:113-114` is GDL-037's three-record-kind file form, and §4.2 step 5's drift check is the mirror mechanism it cites. | **CLOSED** |
| **Residual 4** — the re-pinning discipline does not reach gwz-dev | Extend it (`9c0008870ecd`; `AgentProcessRules.md` byte-unchanged since `ff431743cc4c`) | The Appendix gains "**The re-pinning discipline extends to the authority repository**", recording gwz-dev at `9c0008870ecd` (was `7b3f1bc723d6` at round 2) and the command `git -C gwz-dev log ff431743cc4c..9c0008870ecd -- dev-docs/AgentProcessRules.md` → **0 commits**. I ran it: 0. The text then generalises the discipline — "Every later round re-runs that one command rather than re-reading the clauses: a moved head with an empty log over the cited path is not staleness, and a non-empty one is" — which is the durable form of the fix rather than a one-off pin. | **CLOSED** |

---

## Changed-range analysis

`git -C /Users/owebeeone/limbo/glade-wz diff c26211d..3b60234 -- dev-docs/glade/GladeDeclReconciliation.md` → **302 insertions, 47 deletions**, 2257 lines, in **24 hunks**. The root moved by three commits, all documents: `2bafc02` filed the three round-3 reports, `b1d3e04` the plan, `3b60234` revision 4. No source file moved; the member lock is untouched.

**Every hunk traced.** I mapped all 24 to a disposition or an accepted rider, and none falls outside `-RemPlan-3.md`:

| Hunk (old lines) | What it does | Authority |
|---|---|---|
| `-3`, `-7,13` | revision number; review-status block; the new revision-4 closure map | round convention |
| `-45` | the revision-3 map's SAF-P2-11 row annotated that its integers were corrected at revision 4, closure sites unchanged | SAF-P2-12 (and the plan's "mark, don't delete" rule) |
| `-271,0` | §1's extended numeric assertion | **CON-P3-10** (mine) |
| `-871,3` · `-875,0` · `-916,2` · `-924,0` | R9's option-table cells (a)/(b1)/(b2); the new derivation paragraph; the recommendation made R2-conditional; the re-read note | SAF-P2-12 items 1–5 |
| `-899,0` | R9's "governs `dir.bindings` only" + priced option (s) | SUR-P3-12 |
| `-1242,2` | §4.2 "nine" → "ten" | **CON-P3-10** (mine) |
| `-1278` | §4.2's rename clause cross-reference to §4.1 item 1 | **Residual 2** (mine) |
| `-1378` · `-1380` · `-1638` | the census working directory, in the landing order and in bullet 6 | **Residual 1** (mine) |
| `-1423` | step 4's dual-maintenance rule for the header | Safety observation |
| `-1481,6` | Token 5's "which to write, per `BINDING_SHAPES` member" and the omission bullet | SUR-P3-11, SUR-P3-13 |
| `-1606,5` | bullet 5's b1/b2 correction **and** the deleted-`service`/`workspace` paragraph | SAF-P2-12, SUR-P3-12 |
| `-1625,2` | bullet 6's durable-cost sentence | SAF-P2-12 item 4 |
| `-1680,0` | bullet 12's `workspace` addition to the published grammar | SUR-P3-10 |
| `-1805` · `-1827,4` · `-1841,0` | §4.7's "row 21 is new at revision 4"; the (b) preamble's R8 correction; row 21 | **Residual 3** (mine) |
| `-1836` | row 9's corrected, R2-conditional numbers | SAF-P2-12 item 4 |
| `-1848` | row 14's "§2d row 16" disambiguation | Safety observation |
| `-1967,0` | the Appendix's gwz-dev re-pin | **Residual 4** (mine) |

Nothing outside. No ruling's outcome or recommendation changed: R9's sub-recommendation was re-read as the plan permits and **kept**, with the reasoning stated ("b1 is still the cheaper shape by the same **13** appends"; what is withdrawn is only the gloss that b1 costs nothing durable). Option (s) is an addition the plan expressly authorised ("with symmetry offered as a new option under R9 if the owner wants it, priced there") and the text says "It is offered, not recommended: the recommendation above is unchanged."

**Standing checks re-run on the new text.**

- **§2 recount, by the document's own command:** `20 agrees, 1 DIVERGENT, 6 SETTLED, R1 6 R2 6 R3 2 R4 1 R5 2 R6 5 R7 3 R8 1 = 26`; `grep -cE` → **53**, matching the header at `:330`. No row moved class.
- **§4.7 classification with row 21 and the R8 preamble:** (a) = 1, 3, 4, 5, 6, 8, 11, 12, 15, 20; (b) = 2, 7, 9, 10, 13, 17, 18, 19, **21**; (c) = 14, 16 — **21 rows, 1–21, each in exactly one block, no gap, no duplicate**. Grepping the (a) block for "fails by design", "not a gate" and "manual" returns **0**. Every ruling R1–R11 is named in at least one (b) row.
- **The extended numeric assertion, applied to every count the diff touches.** R9's column against §4.7 row 9 and §4.4 bullets 5 and 6, cell by cell: {R2(a)/(c)/(d) × b1} = 2 / 2 / 2; × b2 = 15 / 15 / 15; × (a)or(c)-with-respelling = 15 / 15 / 15; {R2(b) × b1} = 0 / 0 / 0; × b2 = 13 / 13 / 13; × (a)or(c) = 13 / 13 / 13. **All three sites agree in all six cells.** The derivation reproduces them from the census: {R2(a), b1} = 2 + 0 = 2 (the worked pair the text gives); {R2(a), b2} = 2 + 13 = 15; {R2(b), b1} = 0 + 0 = 0; {R2(b), b2} = 0 + 13 = 13; with a file-side respelling the first term becomes 15 or 13 and the second 0 — so no cell exceeds 15, as the text asserts. The recommendation's spreads are arithmetically right (15 − 2 = 13; 13 − 0 = 13), and the file-edit sentence ("15 file edits to 2 under R2(a)/(c)/(d); from 13 to 0 under R2(b)") follows from R2's own App-files cells, which I re-read: (a)/(c)/(d) all re-type `windowed`, (b) reads "0 meaning edits". §4.2's two compatibility bullets: the byte-moving bullet's "11 of 26 — the 9 `BindingDecl` and the 2 `AdvertisementRecord`", "the same 11" and "the 3 surviving `Retention` vectors" all reproduce against the corpus; the deletion bullet is the corrected ten. §4.7 row 2 is untouched by this diff and was verified in round 3.
- **The census cross-tabulation the drafter added** — I re-ran it independently over the five app files: **13 `value`/`latest`, 11 `log`/`from-cursor`, 2 `log`/`windowed`, 2 `swmr`/`from-cursor`, and no `crdt` line of any retention**, over **28** binding lines, in exactly **four** (shape, retention) pairs. 13+11+2+2 = 28. Every derived claim holds: all 13 `value` lines say `latest`; 11 of the 13 `log` lines say `from-cursor` and the other 2 are the `windowed` pair; both `swmr` lines say `from-cursor`; `crdt` has no precedent at all.
- **Citation pass over every `file:line` the diff adds or changes.** All new: `sysdata.rs:135` = `(6, Cbor::Text(self.retention.clone())),` — tag 6, retention, exactly as the derivation needs; `registry.rs:97` = `Record::Binding(r) => r.to_cbor(),`; `appdecl.rs:173` = the `workspace <share> <name>` template; `appdecl.rs:176` = the duplicate-share refusal; `grazel/apps/grazel-app.glade:47` = `# workspace <share> <name>` and `:47-53` carries "Required for `(ws-razel, gwz.ops)` to route to the composed glade-gwz supplier" verbatim; `grazel/apps/gyld-app.glade:64` = the same comment; `GladeGrazelAttachNotes.md:85-88` = "`grants_for` applies revocation-wins at `(principal, share)` regardless of order" verbatim. Re-confirmed as still correct: `appdecl.rs:37`, `:111-114`, `:138`, `:168`, `:264`, `:267`, `glade-node.rs:91`, `grazel-app.glade:23`, `grazel/README.md:73-79`, `GladeDeclSurface.md:113-114`. Two new counted claims verified: **5 of the 5** app files carry exactly one `workspace` line (`grep -c '^workspace '` → 1 for each), and the in-file `# workspace <share> <name>` comment is present in exactly the three files named (`grazel-app.glade:47`, its twin, `gyld-app.glade:64`) and absent from both fixtures. The gwz-dev log command returns 0.

**New root causes.** One, a coverage gap: CON-P3-11.

**Architectural classification.** **No new ARCHITECTURAL root cause.** I state this explicitly because this is the third round and an architectural finding here stops the lane. CON-P3-11 is a missing row in a table — one option of one ruling has no stated gate. It changes no ruling, no recommendation, no landing step and no invariant of the system the document describes; the remedy is one §4.7 cell. Everything else in `c26211d..3b60234` is a correction of integers, a citation, a working directory or a cross-reference. The object's own architecture — the eleven rulings, the §4 landing plan, the (a)/(b)/(c) gate model — is unchanged and was not challenged by anything I found.

---

## 0. Evidence base

**Object:** the full diff `c26211d..3b60234` (24 hunks, read in full), plus §1's Method, §2's header, §3 R9 and R2, §4.0, §4.1 item 1, §4.2's compatibility block, §4.4's landing order and bullets 5, 6, 12 and Token 5, §4.7 in full, and the Appendix, read at revision 4 in the working tree (proved identical to `3b60234`).

**Round inputs:** `-RemPlan-3.md` (71 lines, in full) and my own `-ReviewConsistency-3.md`. No other axis's report of any round.

**Sources re-opened at the tuple:** `glade/node/src/{appdecl.rs, sysdata.rs, registry.rs}`, `node/src/bin/glade-node.rs`, `glade/dev-docs/GladeGrazelAttachNotes.md`, `dev-docs/glade/GladeDeclSurface.md`, all five `.glade` files, `grazel/README.md`, `glade-decl/corpus/decl.v0.json`, `gwz-dev/dev-docs/AgentProcessRules.md` (by log).

**Commands (read-only):** tuple `rev-parse` and `status --short` for sixteen repositories at start and end, identical; `git diff --stat` and `-U0` hunk enumeration for the object and the root; the object's own §2 recount; the (a)/(b)/(c) row extraction and the forbidden-phrase grep; the app-file census and the (shape, retention) cross-tabulation; `grep -c '^workspace '` per file; the three scopings of the `.glade` census grep; a Python inventory of `decl.v0.json`; `git log ff431743cc4c..9c0008870ecd -- dev-docs/AgentProcessRules.md`; `shasum`; and `corpus/build.py --check` in `glade-decl/`, which wrote nothing.

---

## 1. Findings

### [CON-P3-11] R9 gains a fifth option, (s), that no §4.7 row covers — against the property CON-P2-6 established and §4.0 depends on

**Location.** Object `:1004` (§3 R9, "**R9's options govern `dir.bindings` only (SUR-P3-12)**", the priced option (s)) and `:1817` (§4.4 bullet 5's "If R9 takes its (s) option the page says the opposite"), against §4.7(b) rows 9 and 10 and §4.0's publish ordering.

**Violated invariant.** §4.7's own (b) contract, which I made blocking at CON-P2-6 and the plan accepted: "Each row names the ruling it turns on and what green means under **each of that ruling's options**", and §4.0's "every row of §4.7(b) is green **under the answers the owner actually recorded**". An answer the owner can legally record must be findable in the rows.

**Reproduction.** Revision 4 adds, inside R9: "*If the owner wants the symmetry instead, that is a further option under R9 — call it (s)*" — a `ServiceRetraction` and a `WorkspaceRetraction` record kind beside (a)'s `BindingRetraction`, each with the same scope rule, two folds, `sysdata.rs` regenerated, "roughly (a)'s ~200 LOC again, and **two more record kinds frozen at the same publish**". Grepping every §4.7 row for `(s)` returns **zero**: row 9 enumerates green under (b1), (b2) and (a)/(c); row 10 under (a), (c) and (b); neither names (s). The only two mentions of (s) in the whole document are the two lines above.

**Impact.** Bounded, and smaller than CON-P2-6's was: (s) is (a) plus two record kinds, so rows 9 and 10 would sensibly be read as (a)'s, nothing becomes red-for-ever and §4.0 still resolves. What is missing is a gate for the part (s) adds — and the option's own text calls that part "two more record kinds frozen at the same publish, which is the part that is expensive to defer". So the one element of (s) that is irreversible at the freeze is the one element no row proves. An owner who records (s) is also given no instruction by §4.0, which sends them to rows that do not mention their answer.

**Required correction.** Either (i) state in R9 that (s) is (a) plus two record kinds and that rows 9 and 10 apply to it exactly as to (a), adding one clause to row 10 for the two new retraction kinds ("under (s), the same three tests for a deleted `service` line and a deleted `workspace` line"); or (ii) say in R9 that (s) is offered for decision but not costed as a gated option in this amendment, and that taking it re-opens §4.7. Option (i) is the smaller edit and matches how rows 9 and 10 already treat R9's other options.

**Closure test.** `grep -n` every option label R9 offers — (a), (b1), (b2), (c), (s) — and confirm each appears in at least one §4.7(b) row's "what green means" cell; and, for (s), that the cell names a test for each durable record kind the option freezes.

---

## 2. Invariant analysis

Attacks that failed, briefly. **The durable-cost correction is internally consistent everywhere it lands** — R9's column, §4.7 row 9, §4.4 bullets 5 and 6 and R9's recommendation agree in all six (R2 answer × R9 shape) cells, and the derivation reproduces each from a census I re-ran myself; the code path it rests on (`appdecl.rs:138` → `sysdata.rs:135` tag 6 → `registry.rs:97` → `:264`/`:267` → `glade-node.rs:91`) is correct line by line. **The §4.7 model survived the addition of row 21**: 21 rows, each classified once, no forbidden phrase in (a), all eleven rulings named. **The §2 arithmetic is untouched** at 53 = 20/1/6/26. **The census cross-tabulation is exact** and its four derived rules are each true of the data. **Both new grammar claims hold**: all five app files carry exactly one `workspace` line, and the in-file comment is a 3-of-5 property as stated. **The quotation §4.2 makes of §4.1 item 1 is verbatim**, and §4.1 was not edited to make it so. **Every correction is marked in place, none deleted**, as the plan requires — I checked the six marked corrections in this diff and each preserves the superseded claim.

---

## 3. Risks and next action

**Residual risks below the finding bar**

- §4.4 bullet 5's sentence "If R9 takes its (s) option the page says the opposite for `service` and `workspace`, and this sentence moves with it" is the documentation half of (s); the gate half is CON-P3-11. Fixing the row would naturally pick up this sentence as its evidence.
- R9's option table still has four rows while R9 now offers five answers, (s) living only in prose below it. Consistent, since (s) is priced in the paragraph, but a reader scanning the table will not see it. A one-line pointer under the table would close the gap that CON-P3-11 gates.
- The standing item the plan records but declines to disposition — the grammar has no user-facing page to move to, and none is named — remains true at revision 4 and is unchanged by this diff. I note it only so the archive shows it was seen again and deliberately left.

**Single next action.** Take CON-P3-11 as a one-cell rider: give §4.7(b) row 10 a clause for R9(s), or state in R9 that (s) is offered for decision and not gated here. Nothing else on this axis blocks. **My GO on revision 3 is re-affirmed on revision 4**: CON-P3-10 and all four residuals are closed against their original counterexamples, every hunk traces to a disposition or an accepted rider, every standing check reproduces, every new citation lands, and no finding of mine in any round — this one included — is an architectural root cause of this object.
