# Glade Declaration Reconciliation — `glade-decl` vs the node, 2026-09-21

Status: assessment + amendment proposal, revision 4 — **accepted at glade-wz root
`3b60234` (glade `559cb2c`, glade-decl `d671f10`, glade-decl-rs `21eefa1`, the lock
of `a617c33`) after `-ReviewConsistency-4.md`, `-ReviewSafety-4.md` and
`-ReviewSurface-4.md` reported GO; this accepts the assessment and the amendment
shape only.** The amendment itself (§4) is not made and nothing is published by
this acceptance; the rulings R1–R11 were not part of it, and were recorded
afterwards in §3 (ruled 2026-09-23). Two P3s and one recorded residual
stand open as riders on the amendment's own commit, not on this document: SAF-P3-13 (§4.7 row 9's assertion
is stated in tree-wide units; a single registry sees its files' share), CON-P3-11
(R9's option (s) has no §4.7 row), and the Surface axis's standing residual (no
user-facing page is named for the grammar to move to). **No code, schema or corpus
changed by this document.** It exists to make the amendment a SINGLE edit.

**Review status: accepted, 2026-09-23.** Revision 4 was re-verdicted by the same
three reviewers at `3b60234`, filed verbatim at `00eac99`: Consistency **GO** (0 P2,
1 P3, all round-3 items CLOSED), Safety **GO** (0 P2, 1 P3, SAF-P2-12 CLOSED on all
five items), Surface **GO** (0 P2, 0 new P3, all four round-3 findings CLOSED). Every
report states that no finding of its axis in any round is architectural; the
three-round cap was not engaged. The paragraph below is the record as it stood
before those re-verdicts, kept as history. Revision 3 was re-reviewed on
three axes at glade-wz root `c26211d`, against the round-3 tuple, and the three
reports are filed verbatim beside this file at `2bafc02`:
`-ReviewConsistency-3.md` — **GO** (0 P2, 1 P3; all five round-2 findings CLOSED);
`-ReviewSurface-3.md` — **GO** (0 P2, 4 P3; all nine round-2 findings CLOSED);
`-ReviewSafety-3.md` — **NO-GO** (1 P2, 0 P3; all four round-2 findings CLOSED),
with a pre-commit to GO on a revision resolving **SAF-P2-12**. All three state in
terms that none of their findings, in any round, is an architectural root cause of
this object. This revision is that remediation, drafted under `-RemPlan-3.md`, which
confines it to the one blocking finding and the P3 riders the three reviewers asked
to have folded in. Remediation rounds 1 and 2 are spent; this is the **third round,
which the process permits only for non-architectural corrections** — and every
finding it carries is classified non-architectural by the reviewer who raised it,
in terms. **Those three round-3 verdicts stand as the last on record**: no
round-3 finding is closed by anything written here, nothing in this document is
accepted, and closure is the raising reviewer's to verify against the original
counterexample — this document only says where the correction is. Resuming means
Safety re-verdicting SAF-P2-12 on the revision-4 tuple, and Consistency and Surface
re-verdicting on the diff `c26211d..<revision 4>` restricted to their sections and
their riders, so that GO from all three axes stands on one revision.

### Revision 4 (2026-09-23) — what changed and where

Revision 3 was reviewed at glade-wz root `c26211d` and the round-3 member tuple,
which is unchanged from round 2 and is the Appendix's. `-RemPlan-3.md` governs this
revision: one blocking finding — three integers and a missing derivation in one
table — plus the accepted riders, and nothing else. **No ruling's outcome or
recommendation is changed.** R9's sub-recommendation was re-read against the
corrected spread, as the plan permits, and **kept**: the criterion still
discriminates, because b1 is still cheaper than b2 by the same 13 appends; what is
withdrawn is only the gloss that b1 costs nothing durable. Where a correction needed
a CHOICE it is laid out and not made — SUR-P3-12's symmetry is priced as an option
(s) under R9 and not recommended, and `crdt`'s retention is answered by saying what
determines it rather than by picking a value.

**Closure map, by round-3 finding ID.** Pointers are **headings plus the bullet or
row text** they land on, never bare ordinals (CON-P3-9's convention).

| Finding | Where the closure is |
| --- | --- |
| **SAF-P2-12** — R9's durable-cost column counts only the spelling churn and omits the R2-forced `windowed` edits, and §4.7 row 9 turns those numbers into a gate assertion | §3 R9 "**How the durable-cost column is computed (SAF-P2-12)**" — the derivation "**appends on the first boot = (lines whose token 5 changes in the FILE, which R2's answer fixes) + (lines whose STORED token the node rewrites though the file did not change, which R9's shape fixes)**", the census as its input, and the worked pair {R2(a), R9(b1)} → 2 + 0 = **2** · R9's option table, row **(a)**'s "the `windowed` lines change under **every R2 option except (b)**", row **(b1)**'s "**2** under R2(a)/(c)/(d) … **0** under R2(b)", row **(b2)**'s "**15** under R2(a)/(c)/(d) (13 normalised + 2 `windowed`) and **13** under R2(b)" · R9's recommendation, "*(Re-read against the corrected column — SAF-P2-12. **The sub-recommendation stands, and so does its criterion**)*" · §4.4 bullet 6's "which is a derivation over (R2's answer, R9's shape) and not the same number for each" · §4.4 bullet 5's "**Under b1 only the two `windowed` records move**" · §4.7(b) row 9's "that cost is conditional on R2 as well as on R9 — read it off R9's derivation paragraph" |
| **CON-P3-10** — §4.2's deletion enumeration says "nine key names" where it designates ten | §4.2 "**Compatibility**", the bullet "Options that delete a v0 vector outright" — "All **ten** key names this bullet designates … `Retention` counted once as a key and not again as the message name it shares with it" · §1 "**Revision 3 mechanical passes**", the added assertion "**Revision 4 extends that assertion (CON-P3-10)** … expanded through every group name the sentence uses". *(The Safety report's first observation — the same "nine" — is this finding, raised independently on the other axis; it closes here.)* |
| **SUR-P3-10** — the published grammar omits the `workspace` directive every app file uses | §4.4 bullet 12, "**The same edit completes that block, which omits a directive every app file uses (SUR-P3-10)**" — `workspace <share> <name>` added to `GladeGrazelAttachNotes.md:29-36` with the clause "**it is the line that makes a declared surface routable**", `appdecl.rs:173`'s own template and the 5-of-5 count as its grounds |
| **SUR-P3-11** — the retention rule has three rows and `BINDING_SHAPES` has four members | §4.4 "**Token 5 — the retention**", the bullet "**Which to write, per `BINDING_SHAPES` member**" — `value` → `latest`, `log` → `from_cursor`, `swmr` → `from_cursor`, and for `crdt` "**the shape does not determine the retention, and what does is how the surface is read**", with "the page must say in terms that `latest` does not mean 'the merged value'" and the (shape, retention) census showing **0** `crdt` lines |
| **SUR-P3-12** — R9 supplies a retract half for `binding` only | §3 R9 "**R9's options govern `dir.bindings` only (SUR-P3-12)**", with the four directives' remove halves and the priced option **(s)** ("offered, not recommended: the recommendation above is unchanged") · §4.4 bullet 5, "**What a deleted `service` or `workspace` line does today: nothing (SUR-P3-12)**" — "a **retired exchange stays routable**" |
| **SUR-P3-13** — the token-5 omission bullet calls a five-token line an arity refusal | §4.4 "**Token 5 — the retention**", the "**What omission means**" bullet — "a **four**-token line is an arity refusal", plus "**The counting convention throughout §4.4 is tokens *after* `binding`**" grounded on `appdecl.rs:111-114`'s `toks.len() != 6` |
| Safety observation — §4.7(c) row 14's "row 16" reads ambiguously inside its own table | §4.7(c) row 14, "which is exactly **§2d row 16**'s divergence (SAF-P2-4) — `ZoneKind.private=1` … not §4.7's own row 16 directly below" |
| Safety observation — step 4 does not repeat bullet 6's dual-maintenance rule | §4.4 "**Landing order**", step 4 — "**And the header is dual-maintained exactly as the tokens are** … both headers move in the same commit or the twins drift" |
| Consistency residual — §4.4's census command has no working directory | §4.4 "**Landing order**", the command note — "run **with `glade-wz` as the working directory**", with the parent-directory run returning six and naming `taut-dev/glade/apps/grazel-app.glade` · §4.4 bullet 6's closing `grep -rln` |
| Consistency residual — §4.2 asks a question §4.1 already answers | §4.2 "**Compatibility**", the rename clause — "**For the live candidates, §4.1 item 1 has already said it**… keeping their keys, so the relabelling is a comment change and not a rename" |
| Consistency residual — R8 is named in no §4.7 row | §4.7(b) row **21**, "grep `dev-docs/glade/GladeDeclSurface.md` for the record-kinds sentence R8's answer requires" · the (b) preamble, "**R8's *corpus* gate set is (a)**", correcting "R8 appears in no (b) row" in place |
| Consistency residual — the re-pinning discipline does not reach gwz-dev | Appendix "**The re-pinning discipline extends to the authority repository**" — `gwz-dev 9c0008870ecd`, and `git log ff431743cc4c..9c0008870ecd -- dev-docs/AgentProcessRules.md` → **0 commits** |
| Recorded, not a disposition — the page an author reads still does not exist | §4.4's note under the four-home rewrite, "The grammar block **should move to, or be mirrored in, a page presented as user-facing**". The Surface reviewer has recorded this in every round and `-RemPlan-3.md` declines to make it a disposition: it is the next increment's value, not this freeze's, and no page is named here |

Revision 3's, revision 2's and revision 1's false claims stay corrected **in place
and marked**, not deleted; revision 4 adds its own marks in the same form.

### Revision 3 (2026-09-22) — what changed and where

Revision 2 was reviewed at glade-wz root `1defe3b` and the round-2 member tuple; the
three reports are filed verbatim beside this file as `-ReviewConsistency-2.md`,
`-ReviewSafety-2.md` and `-ReviewSurface-2.md`, and the lane owner's `-RemPlan-2.md`
governs this revision. Every blocking finding and every accepted rider is implemented
below. Where a correction needed a CHOICE, this revision states the options and their
true costs and leaves the choice to §3's rulings — that is why R9(b) is now **two
shapes** rather than one, and why R2's sub-choice is written out with a third option
rather than choosing for the owner. **No ruling's outcome or recommendation is
changed** except where a disposition required it: R9(b)'s split into b1/b2 with their
true durable costs, R10(a)'s newly stated ordering cost, and R2's written-out
sub-choice.

**Closure map, by round-2 finding ID.** Each pointer is a **heading plus the bullet or
row text** it lands on, never a bare ordinal — ordinals renumber under edit, which is
how five of revision 2's pointers came to be wrong (CON-P3-9).

| Finding | Where the closure is |
| --- | --- |
| **CON-P2-6** — "every row of §4.7 is green" over conditional, red-by-design and non-gate rows | §4.7 "The checks that prove it", now three blocks: "**(a) Unconditional gates**", "**(b) Gates conditional on a ruling**" (each row naming its ruling and what green means per option), "**(c) Evidence, not gates**" (rows 14 and 16). §4.0 "**Publish ordering**" — the two bullets "every row of §4.7(a) is green on a settled tree" and "every row of §4.7(b) is green under the answers the owner actually recorded", plus "**Under R4(a), row 2 is replaced, not left red.**" |
| **CON-P2-7** — the sub-choice's third option has no landing step, and R2's imported an end condition that decides nothing about retention | §3 R2 "**Sub-choice, row 18b — validate token 5?**", written out in full with option (iii) "the token stays inert, and R2's own vocabulary is the only thing that could ever make it validatable", plus "**Cost, on a token-5 value.**" · §2e row 18b's *Ratified* cell, now "**no ratified entry requires it — and none governs retention at all**" · §4.4 bullet 2's third branch "**(iii) do not validate** — **this bullet does not land for that token.**" · §4.7(b) row 7, "Under **(iii)** the token is not validated … the row **does not apply to that token**" |
| **SAF-P2-9** — the landing order moves the header before a node can parse it | §4.4 "**Landing order, and why**" — "**The token and the header have opposite safe orders**" and step "**0. A both-headers node first.**", with the headers moving last at step 4 · §3 R10's App-files cell "**the token edits and the header bump are NOT the same commit**" and its new "**Ordering cost**" column ("one extra landing step" against (a); "**none**" against (b)) · §4.4 "**Every intermediate commit is a gate, not only the last one.**" · §4.7(b) row 17 |
| **SAF-P2-10** — `--compat` cannot be green under the recommended R7(b) | §4.2 "**Compatibility**" — the two enumerations "Options that move a surviving vector's bytes" and "Options that delete a v0 vector outright, moving no surviving byte", the three-case superset statement including "v1 is a byte superset **of a stated subset of v0**", and the numbered `--compat` specification (assert over `n ∈ v0 ∩ v1`; fail on an undeclared `n ∈ v0 \ v1`; fail on a declared name still present) · §4.7(b) row 2 · §4.0 "**Publish ordering**" |
| **SAF-P2-11** — R9(b) normalises inside `parse()`, so its title and its cost are false | §3 R9's option table rows "**(b1) The file keeps its spellings; the node maps them where a CONSUMER reads the record**" and "**(b2) … `parse()` normalises on the way into `sysdata::BindingDecl`**", under the new "**Durable cost on the first boot**" column · R9's recommendation, "**within (b), b1 if the durable cost is what matters and b2 if a single stored vocabulary is**", with the marked correction of "removes the … question for 13 of the 15" · §4.4 bullet 5's "**Under R9(b) the page owes the fold and retract rules for fifteen surfaces, not two**" · §4.7(b) row 9, "**The asserted `appended` is the chosen option's own stated durable cost and must equal it**" *(the integers this row quoted — the column's "0 / 13 / 15" and row 9's "**0** under (b1), **13** under (b2)" — were wrong under the recommended R2(a) and are corrected at revision 4; the closure sites are unchanged — SAF-P2-12)* |
| **SUR-P2-1** — the mount sentence has a third home, the one the other two cite | §4.4 "**And the contradicting sentence must go, in all four of its homes**", naming `dev-docs/glade/GladeDeclSurface.md:29-30` beside the two `grazel-app.glade` copies and `GladeGrazelAttachNotes.md:49-51` · §4.7(a) row 20 |
| **SUR-P2-6** — token 5 is validated in the same bullet as token 4 and documented nowhere | §4.4 "**Token 5 — the retention (SUR-P2-6)**", the five statements from "What `latest` means" to "What omission means" · §4.4 bullet 15, "`dev-docs/glade/GladeDeclSurface.md` gains a `Retention` row" · §4.2 "**Step 6 — the front page's own gaps**", "including the `Retention` members, which no published page names at all" · §4.7(b) row 18 |
| **SUR-P2-7** — R9(a)'s retraction has no stated scope | §3 R9 "**(a)'s retraction scope, stated in the option (SUR-P2-7)**" — "per `(app, glade_id)` … a file that is **not loaded** on a given boot **retracts nothing**" — with the two-file counterexample on grazel's documented default boot · §4.4 bullet 5's "**A surface declared by an app file that is not loaded on this boot stays declared**" · §4.7(b) row 10, widened to the two-file case |
| **SUR-P2-8** — nothing updates the grammar the author copies from | §3 R11's option table, which now carries a "**Docs**" column naming `GladeGrazelAttachNotes.md:32` and the three in-file comments, plus the sentence on how an author learns the tail exists · §3 R10's Docs cell, `GladeGrazelAttachNotes.md:30` · §4.4 bullet 12, "**Every published grammar and every in-file grammar comment moves with the line**" · §4.4 bullet 4's "**Under R11(a) this is not enough**" · §4.7(b) row 19 |
| **CON-P3-7** — claims new in revision 2 that the mechanical pass did not catch | §4.4 "**Landing order**", "The **five** app files live in **four** repositories" · §2m A9, `exchange.rs:651-658` marked "**test code** … `grazel_attach_end_to_end`" and "lists every directive **it publishes** — and it omits `workspace`" · §3 R6's "*(Premise restated — CON-P3-7 …)*" against `README.md:42` · §4.2 "**Step 6**", the quote restored to "**frozen once shared (GQ-6), derived from package id + grip key**" · §1 "**Revision 3 mechanical passes**" item 1 |
| **CON-P3-8** — the evidence is pinned to the round-1 members | Appendix "**Revisions — re-pinned to the round-2 tuple**" · §4.4 step 1, `grazel/src/lib.rs:159` · §3 R11 "**Naming (SUR-P2-4)**", `glade-gyld/README.md:499` · §3 R9's option (c) cell, `glade-gyld/README.md:1147` · §4.7(a) row 15, 61 suites and `vite.config.ts:310-316` · §2m A10's census, `0 2 0 0 0` · §1 "**Revision 3 mechanical passes**" item 2 |
| **CON-P3-9** — five closure-map rows point at the wrong place | This block's closure map, whose pointers are headings plus row or bullet text · the Revision 2 block's map below, whose five wrong targets are corrected in place and marked *(CON-P3-9)* and whose pointers are restated in the same anchor form |
| **SAF-P3-12** — §4.1.8 reserves a retired message name taut cannot reserve | §4.1.8 item 8, "**And for a retired message name (R7(b)) taut has no facility either**", naming the module docstring and `OpenNotes.md` as the carrier, with `model.py:136-140` as the proof |
| **SUR-P3-6** — sixteen `.glade` files, five of this format | §4.4 bullet 14, "**Say which `.glade` dialect this amendment freezes**", with the sixteen/five/eleven counts and the A3 treatment or the extension question |
| **SUR-P3-7** — "six app-file copies"; there are five | CON-P3-7's first item: §4.4 "**Landing order**", and §4.4 bullet 6's closing `grep -rln` |
| **SUR-P3-8** — three citations that do not reproduce at the round-2 tuple | CON-P3-8's second and third items: §3 R11's naming note, §3 R9(c), §2m A10 |
| **SUR-P3-9** — the file-side spelling rule, and the refusal that names the other one | §3 R9 "**Which spellings a file may contain, under each shape (SUR-P3-9)**" · §4.4 bullet 7's two per-shape refusal messages · §4.4 bullet 13, "**The joining-character convention goes in the format's own grammar documentation**" |
| Residual — §4.0 should read "every *gate* row" | Folded into CON-P2-6: §4.0 "**Publish ordering**" and "**§4.7(c) is evidence, not a gate.**" |
| Residual — R2's sub-choice imports R1's zone-specific cost example | Folded into CON-P2-7: §3 R2 "**Cost, on a token-5 value.**" |
| Residual — `glade-decl-ts/README.md:21-22` still teaches npm | §2m A5's "**And its README still teaches npm**" · §4.8's first bullet, "Fix the two command lines when A4's collapse is made" |
| Residual — §3's preamble implies eleven answers, not thirteen | §3's opening, "**Eleven rulings — but thirteen answers**" |
| Residual — "the retract form" reads as something an author writes | §4.4 bullet 5's "*(Say "the retraction rule", not "the retract form" …)*" |

Revision 2's and revision 1's false claims stay corrected **in place and marked**, not
deleted; revision 3 adds its own marks in the same form.

### Revision 2 (2026-09-21) — what changed and where

Revision 1 was reviewed on three axes at glade-wz root `b132b7e6d365`; all three
returned NO-GO. The reports are filed verbatim beside this file —
`GladeDeclReconciliation-ReviewConsistency.md` (`CON-`),
`-ReviewSafety.md` (`SAF-`), `-ReviewSurface.md` (`SUR-`) — and the lane owner's
`-RemPlan.md` governs this revision. Every blocking finding and every accepted
rider is implemented below. Where a correction needed a CHOICE, this revision
states the options and their true costs and adds the choice to §3; it does not
make the choice. Closure map, by finding ID — **five targets corrected and every
pointer restated as a heading plus the bullet or row text it lands on
(CON-P3-9)**, because revision 2 wrote them as ordinals against an earlier
draft's numbering and §4.4, §4.5 and §4.7 were renumbered after:

| Finding | Where the closure is |
| --- | --- |
| SAF-P1-1 (taut `optional` is nullable-and-always-emitted) | §1 "**One encoding fact this document turns on**"; §2h row 34's "**Adding the field is not free**"; §3 R3's struck third reason and R4's rewritten option table; §4.1 item 2 (`Msg("BindingDecl", …)`); §4.2 "**Compatibility**" |
| SAF-P1-2 + SUR-P2-5 (a changed or deleted binding line) | §2m **A9**; §3 **R9**; §4.4 bullets 5 ("**What a changed or deleted line does, under R9**") and 6 ("**App-file data, under R9 and R2**"); §4.6's "*(SAF-P1-2)* The retention and zone tokens are **not** 'fields nothing reads'"; §4.7(b) rows 9 and 10 |
| CON-P2-1 (`glade/grip-share` missing from the survey) | §2 legend ("two different binders"); §2c rows 12–14; §2d rows 15, 16; §2h row 30; §3 R1's "**`glade/grip-share` + `glade/demo`**" column; §4.5 item 3, "**`glade/grip-share` + `glade/demo`**" *(corrected — CON-P3-9: revision 2 said §4.5.**4**, which is the gryth-ui step)*; Appendix |
| CON-P2-2 + SAF-P2-5 (regeneration writes to the wrong dirs; ungated copies) | §2m **A4**; §4.3, including "**The copy step must be gated, not remembered**"; §4.7(a) rows 1 and 11 |
| CON-P2-3 (`decl.v0` retarget list) | §4.2 "**Every reference that must move**" |
| CON-P2-4 (SETTLED rows pre-empting open rulings) | §2e rows 18a/18b; §2h rows 31a/31b; §2's header arithmetic and its recount command; §3 R1's "**Sub-choice, row 31b**" and R2's "**Sub-choice, row 18b**" |
| CON-P2-5 (`glade-decl/dev-docs/DeclSurface.md:27` vs GDL-041) | §2m **A12**; §3 R8's "**Which document is controlling**"; §4.2 "**Step 5 — `glade-decl/dev-docs/DeclSurface.md`**" |
| SAF-P2-3 (un-migrated app files become a boot failure) | §4.4 "**Landing order, and why**"; §4.7 rows **6, 7 and 8** *(corrected — CON-P3-9: revision 2 said rows 5–8, and row 5 is `pytest` from a built wheel, which closes CON-P2-3)* |
| SAF-P2-4 (`private` frozen as an unimplemented guarantee) | §2d row 16, now **DIVERGENT BETWEEN BINDERS**; §3 R1's "**Note on `private`**"; §4.1 item 7 (`ZoneKind`'s comment, `README.md:39-41`, OpenNote N9); §4.7(c) row 14 |
| SUR-P2-1 (`zone` enforced while undocumented) | §4.4 "**What must be written before validation is turned on**", the "**Token 4 — the zone**" statements and "**And the contradicting sentence must go, in all four of its homes**" (the fourth home is revision 3's — SUR-P2-1 stayed open) |
| SUR-P2-2 (`glade-app v0` naming two languages) | §2m **A10**; §3 **R10**; §4.4 bullet **3**, "**The header, under R10**" *(corrected — CON-P3-9: revision 2 said bullet 2, which is the token validation)* |
| SUR-P2-3 + SUR-P2-4 (`ttl`'s duration; the sixth token) | §2m **A11**; §3 **R11**; §4.4 bullet **4**, "**The sixth thing on the line, under R11 and R4**" *(corrected — CON-P3-9: revision 2 said bullets 3–4; bullet 3 is R10's header)* |
| CON-P3-1 | §1 "**The process authority this document actually rests on**" — the L1-07 claim was false and is corrected and marked there *(corrected — CON-P3-9: revision 2 pointed at §1 "Precedence applied, in order", which contains no L1-07 correction)* |
| CON-P3-2 | every `file:line` re-opened; the eight corrections are marked *(CON-P3-2)* at the point of use |
| CON-P3-3 | §2e row 18a ("**three** of them `value`-shaped"); §2m A5's two `find`s; §4.2's ten taut-shape corpora; §4.7 row 15 — each count carries its command |
| CON-P3-4 | §3 "**Which ruling decides what**" and the two precedence blocks under it |
| CON-P3-5 | §4.0 "**Where every SETTLED item lands, and what rides separately**" |
| CON-P3-6 | §2m **A4** |
| SAF-P3-6 / SAF-P3-7 / SAF-P3-8 | §4.1 item 8 ("**Any delete option also reserves**") / §3 R3's option (b) cell ("**the synth … vectors change regardless**") / §4.1 item 1's "**Also add an `edge/binding-*-atom` curated vector**" |
| SUR-P3-1 … SUR-P3-5 | §4.4 "**What must be written before validation is turned on**" / §4.4 bullet 7 ("**Removed and renamed tokens stay recognised-but-refused**") / §3 R9's option (b) and its hyphen reason / §4.4 bullet 8 (`GladeGrazelAttachNotes.md:98`) / §4.2 "**Step 6 — the front page's own gaps**" |
| Safety residual (publish ordering) | §4.0 "**Publish ordering**" |

Revision 1's false claims are corrected in place and marked, not deleted.

**One fact of the world changed under this revision, by owner ruling, and is
recorded rather than reviewed here.** Artefact **A2** — the red drift gate, which
revision 1 and all three reports carried as a standing precondition on the freeze
— is **closed**. The owner ruled "discard them and have `build.py` run rustfmt";
`glade-decl-rs`'s uncommitted rustfmt-only edits were discarded, `corpus/build.py`
now formats the generated Rust on both the write and the `--check` path, and the
committed Rust is formatted. New heads: `glade-decl d671f10` (was `bbce73d`),
`glade-decl-rs 21eefa1` (was `555a977`), glade-wz root `dc311ba`. **The contract
itself did not move**: `ir/glade_decl.ir.json` and `corpus/decl.v0.json` are
byte-identical across `bbce73d` and `d671f10`, so every schema, enum, field and
corpus statement in this document is unaffected; only `corpus/build.py` and
`glade-decl/README.md` line numbers moved, and every citation into them was
re-opened at `d671f10`. See A2, §4.3, §4.7 row 1 and §5.

Trigger: owner instruction "reconcile `glade-decl` with the node first", ahead of
`dev-docs/PackageExtractionPlan.md` step 1.1, which *decides and records* the
GDL-041 catalogue question for `glade-decl`/`glade-decl-ts` (`:155`); the publish
itself is step 1.2 (`:156`). *(Revision 1 said step 1.1 publishes; corrected —
CON-P3-1 residual.)* `glade-decl/corpus/decl.v0.json` is already declared the
FROZEN byte oracle (`glade-decl/README.md:52-58`), so the amendment must be made
ONCE: this document enumerates every divergence and splits it into what is
already decided and what the owner must rule.

Prior art this supersedes in scope, not in fact: `dev-docs/glial/GlialFitAssessment-2026-09-15.md`
§1 found four divergences. All four are confirmed below; thirty-odd more are not
in that document.

---

## 1. Scope and rule of precedence

**"Reconcile with the node" means**: make the typed declaration surface
(`glade-decl/ir/glade_decl.taut.py`) agree with (a) the records the node actually
stores (`glade/node/ir/sysdata.taut.py:78-84`), (b) the grammar the node actually
parses (`glade/node/src/appdecl.rs:15-22, 43-47, 107-140`), (c) the app files
actually loaded (`grazel/apps/*.glade`, `glade-gyld/tests/fixtures/*.glade`,
`glade-gwz/tests/fixtures/*.glade`), and (d) the vocabulary the two clients, the
two binders and glial actually admit — *before* any of it is frozen by a publish.

It does **not** mean changing the wire. The node's wire IR
(`taut/corpus/glade.ir.json`, rendered at `glade/wire-rs/src/generated.rs`) is a
different contract with a different, frozen `Shape` enum. Both enums are spelled
`Shape` and they are **not** the same enum:

| | declaration `Shape` (`glade-decl/ir/glade_decl.ir.json`) | wire `Shape` (`taut/corpus/glade.ir.json`) |
| --- | --- | --- |
| members | `value:0 log:1 message:2 stream:3 exchange:4 window:5 swmr:6 crdt:7` | `value:0 log:1 stream:2 swmr:3 crdt:4` |
| `stream` | `3` | `2` |
| carries | what a tap declares | what an `Op` is |

Nothing compares them numerically and nothing should start. The wire is out of
scope for this amendment unless a ruling below says otherwise.

### One encoding fact this document turns on

taut's `optional=True` means **nullable and always emitted**, not omittable:
`taut/src/taut/wire/codec.py:12` — "optional -> always emitted; None -> CBOR null
(deterministic; no omission)" — and `:88` writes the key unconditionally
(`out[f.tag] = None if fv is None else _to_wire(...)`). The frozen corpus shows
it: `edge/binding-account-commons` opens `a7` — a **7**-entry map, one entry per
declared field — and carries `04 f6`, tag 4 (`source`) unset, encoded as an
explicit CBOR null. `glade-decl-ts/src/api.ts:23` mirrors it in the type system
as a *required* `source: string | null`.

Consequence: **adding a field to a message rewrites every vector of that message
and of every message that embeds it.** There is no free additive field. Revision 1
assumed protobuf-style omission throughout §4.2, R3 and R4 and was wrong; each
place is corrected below and marked (SAF-P1-1).

### Precedence applied, in order

1. **A ratified decision-log entry outranks both contract and code.** Ratified
   and in force here: GDL-029 is **not** one (root `dev-docs/DecisionLog.md:47`,
   status `open`); GDL-032, GDL-035, GDL-037, GDL-038 are ratified 2026-07-07
   (`:50, :53, :55, :56`); GDL-041 is ratified 2026-08-28 (`:58`); **GDL-039 (the
   zones vocabulary) is NOT ratified** — `:59` reads "open (implemented, pending
   ratify)". That single fact decides the shape of §3: the shape catalogue has a
   ruling to apply, and the zones/domain model has a ruling to *make*.
2. **Where no ratified decision exists, what the node implements and the app
   files already use is EVIDENCE of intent, not a decision.** `grazel-app.glade`
   has said `retention windowed` since P1.S3 and nothing has ever read it *as a
   policy*; that is a fact about the file, not a ratification of a retention
   vocabulary. (It is read as *bytes* — see row 18b and R9.)
3. **The contract as written is a claim to be tested, not the truth.** Every
   element below was checked against a consumer. An element with no producer and
   no consumer is reported as unexercised, not as "already agreed".
4. **`dev-docs/OpenNotes.md` (N1–N6) is the authoring record, not authority.** It
   documents calls the spec left open; several are exactly the calls this
   document asks the owner to confirm or reverse.

### The process authority this document actually rests on

*(Corrected — CON-P3-1. Revision 1 cited L1-07 for "publishing freezes the
contract". L1-07 forbids exactly that equation.)* Each clause is quoted verbatim
beside its number, from `/Users/owebeeone/limbo/gwz-dev/dev-docs/AgentProcessRules.md`
at `ff431743cc4c`:

- **L1-09** (`:267-277`) is the trigger: "Re-freeze after any change to a wire
  shape, durable state, transition vocabulary, mutation owner, public behavior,
  compatibility rule, platform trust boundary, shared interface, visibility
  boundary, or package ownership." This amendment changes a shared interface, a
  compatibility rule and — via §4.4 — durable state. That is what makes it
  one-shot, not the publish.
- **`glade-decl/README.md:52-58`** is the local rule: "`corpus/decl.v0.json` is
  the FROZEN oracle … Every rendering conforms iff its codec reproduces these
  bytes (parity == correctness)." A byte oracle is what makes a missed element
  expensive.
- **L1-07** (`:240-253`) is a *constraint on this document's wording*, not its
  authority: "Use 'normative' only for authoritative text within a declared
  scope; 'frozen' only when implementers may no longer choose its meaning …
  Never use any of these words as a synonym for implemented or released." Under
  it, "published" never implies "frozen" here; where this document says frozen it
  means the oracle's bytes and the enum numbers, and says so.
- **L1-08** (`:255-265`) applies only once something *is* frozen: its trigger is
  "a frozen contract [that] is unimplementable, unsafe, ambiguous, or
  contradicted by evidence". R7 therefore rests on L1-09, not on L1-08.
- **L1-17** (`:370-386`) and **L1-18** (`:387-421`) govern §5.

### Method

Files read and cited by line, at the revisions in the Appendix. No builds, no
writes outside this file. One read-only command was run: `python3
corpus/build.py --check` in `glade-decl/` — red at revision 1 (A2), green at
revision 2 after the owner's ruling, and re-run green at revision 3
(`all 3 glade-decl artifacts in lockstep with the schema.`, exit 0, `git status
--short` empty in `glade-decl` and `glade-decl-rs` afterwards).

**Revision 2 mechanical passes** (CON-P3-2, CON-P3-3). Every `<file>:<line>` in
this document was re-opened at the pinned revision with `git -C <repo> show
<rev>:<path>` and compared against the claim it carries; the eight corrections
CON-P3-2 lists are marked *(CON-P3-2)* at the point of use, and three further
misses found by the same pass are marked *(rev-2 pass)*. Every total was re-run;
each now carries the command that produced it. `glade-decl-rs` was read with
`git -C glade-decl-rs show 555a97746fc6:<path>` because its working tree then
carried uncommitted rustfmt-only edits to `src/api.rs` and `src/vectors.rs`;
those are now committed formatted at `21eefa1` (A2), and no citation into either
file survives in this document. `corpus/build.py` and `glade-decl/README.md` were
re-opened at `d671f10` after A2's fix grew both.

**Revision 3 mechanical passes** (CON-P3-7, CON-P3-8). Revision 2's pass checked
that a cited line *existed* at the pinned revision. Two gaps in that pass were
found, and both are closed here.

1. *The line can exist and the claim about it still be false.* The pass now
   prints the cited line **and its enclosing item** — function, `mod`, heading —
   and both are compared against the claim. Five places where that changes the
   answer are corrected and marked *(CON-P3-7)*: §4.4's landing order, which said
   "six" app-file copies where there are five; `exchange.rs:651-658` (inside
   `mod tests`, in `grazel_attach_end_to_end`); `GladeGrazelAttachNotes.md:30-35`
   ("every directive" omits `workspace`); R6's premise about `README.md:47` (its
   subsection heading at `:42` does qualify it); and §4.2 step 6's reversed quote
   of `README.md:28-29`.
2. *The pins themselves go stale.* Four members moved between review rounds, and
   revision 2 had already built the re-pinning habit under A2 but applied it only
   to the two the owner's A2 ruling moved. The Appendix is re-pinned to the
   round-2 tuple, the five stale proofs are re-opened there, and **every count
   whose target can move now carries both its command and the revision it was run
   at** — so a later reader can tell staleness from error without re-deriving the
   claim. Marked *(CON-P3-8)* or *(SUR-P3-8)* at the point of use.

A separate assertion belongs to the same pass and is run here: **no bare number
in this document contradicts an enumeration within ten lines of it.** That is the
check §4.4's "six app-file copies" failed, three lines above an enumeration of
five.

**Revision 4 extends that assertion (CON-P3-10)**, because as worded it did not
reach the defect it was written for. The stronger form: **a bare number naming
the cardinality of a set its own sentence enumerates must equal that set's size,
expanded through every group name the sentence uses** — "all four `Retention`
vectors", "the same 11", "the three in-file comments", "all 14 bindings". It is
run here over §4.2's two compatibility bullets, §4.7 row 2 and R9's durable-cost
column, which is the class the ten-line form missed: §4.2's "nine key names" sat
on the *same bullet* as an enumeration designating ten, because one of its
members arrives through a group name and one key shares its spelling with a
message.

---

## 2. The divergence table

**53 line items audited: 20 agree, 1 DIVERGENT BETWEEN BINDERS, 6 SETTLED, 26
need a RULING.** *(Revision 1 said "51 elements: 21 agree, 7 SETTLED, 23 RULING".
That count was arithmetically exact for revision 1's classification; the
classification was wrong in three places — CON-P2-4 split rows 18 and 31, and
CON-P2-1 + SAF-P2-4 moved row 16 out of "agrees".)*

Recount command, over this file:

```sh
D=dev-docs/glade/GladeDeclReconciliation.md
C='\*\*(agrees|DIVERGENT BETWEEN BINDERS|SETTLED|RULING \(R[0-9]+\))\*\* \|$'
grep -oE "$C" $D | sort | uniq -c   # 20 agrees, 1 DIVERGENT, 6 SETTLED,
grep -cE "$C" $D                    # R1 6 R2 6 R3 2 R4 1 R5 2 R6 5 R7 3 R8 1 = 26; total 53
```

(The pattern matches only §2a–2l's class cells: every §2m class cell carries
trailing prose, so none of them ends `** |`.)

Elements with no divergence are listed so this is a complete audit, not a
complaint list. "node" = `sysdata.taut.py` + `appdecl.rs`; "files" = the five
live `.glade` files; "clients" = `glade/client-ts/src/shapes.ts` +
`glade/client-rs/src/session.rs`; "**binders**" = `glial/src/binder.ts` **and**
`glade/grip-share/src/manifest.ts` with its manifest data at
`glade/demo/src/manifest.ts` — two different binders, which is why row 16 is
DIVERGENT (CON-P2-1, SAF-P2-4); "glial" = `glial/src/{shapes,manifest,instance,events}.ts`;
"UI" = `gryth-wz/gryth-ui`.

### 2a. `Shape` — 8 members + 1 absent

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `Shape.value=0` | present | `KNOWN_SHAPES` + `BINDING_SHAPES` (`appdecl.rs:43-44`); 13 of the 28 binding lines in the five live app files | wire `value:0`; `requireOpShape` ✓ (`client-ts/src/shapes.ts:27`); `requireMountShapeAdapter` ✓ (`glial/src/shapes.ts:83`); 6 UI surfaces | GDL-041 engine | — | — | **agrees** |
| 2 | `Shape.log=1` | present | authorable; 13 binding lines | wire `log:1`; both clients ✓; glial ✓; 3 UI surfaces + chat's, via glade-chat | GDL-041 engine | — | — | **agrees** |
| 3 | `Shape.message=2` | present | recognized (`appdecl.rs:43`) but REFUSED as a binding (`:119-124`; test `:324-330`) | absent from wire; `requireShapeAdapter` throws (`glial/src/shapes.ts:72-75`) | GDL-041: "`message` is unsupported" | contract carries a shape every decision and every runtime rejects | delete the member, or retain it reserved-and-unbindable | **RULING (R3)** |
| 4 | `Shape.stream=3` | present | recognized, refused as a binding (`:119`) | wire `stream:2`; `requireOpShape` ✗; `requireMountShapeAdapter` ✗; `requireShapeAdapter` ✓ (`glial/src/shapes.ts:39-43`) | GDL-041 engine; "recognition MUST NOT imply runtime support" | none in principle — recognized engine with no durable adapter | keep; restate recognition≠support in the schema comment. **Why `stream` is not an R3 candidate**: GDL-041 ratifies it an *engine* and glial ships a delivery adapter for it (`shapes.ts:39-43`), so the name has a live meaning; `message`/`window` have none anywhere (CON residual) | **SETTLED** |
| 5 | `Shape.exchange=4` | present | refused as a binding with "exchange uses `service`" (`appdecl.rs:121`); authored by the `service` line | not a wire `Shape` (it is `FrameType.exchange_req/res`); `requireExchangeShape` (`glial/src/shapes.ts:101-103`); `client-rs/src/supplier.rs:156`; `M.gwzOps.shape === "exchange"` (glial GAP-14, `glial/dev-docs/DecisionLog.md:263`) | GDL-041: "separate correlated service path" | — | — | **agrees** |
| 6 | `Shape.window=5` | present | recognized, refused as a binding; test `:324-330` | absent from wire; not in glial's `ADAPTERS` (`shapes.ts:18-52`) | GDL-041: "an application projection over an explicit base shape"; `GlialClientRuntime.md:26-27` "not an engine" | same as row 3 | delete, or retain reserved | **RULING (R3)** |
| 7 | `Shape.swmr=6` | present (added `99a04e0`) | authorable (`appdecl.rs:44`, test `:350-356`); 2 binding lines — `grazel-app.glade:23` `ws.files swmr` | wire `swmr:3`; both clients ✓; glial mount ✓; node `store.rs:160-180` | GDL-041 engine | — | — | **agrees** |
| 8 | `Shape.crdt=7` | present (added `bbce73d`) | **NOT in `KNOWN_SHAPES`** (`appdecl.rs:43` lists 7 names, no `crdt`) → `binding x crdt …` fails "unknown shape" | wire `crdt:4`; `requireOpShape` ✓; `requireMountShapeAdapter` ✓; `store.rs:171` | GDL-041 engine | a shape every runtime carries cannot be declared in an app file | add `crdt` to `KNOWN_SHAPES` and `BINDING_SHAPES` | **SETTLED** |
| 9 | `Shape.atom` *(absent)* | **missing** | absent | `glial/src/shapes.ts:19-23` `atom.oracle/v1`; `gryth-ui/packages/glade/src/runtime.ts:15` `createAtomValueTap` | GDL-041 engine | contract omits a ratified engine its own exported IR's catalogue lists (`glade_decl.ir.json` `shapes`: `unary value atom log stream swmr snapshot_delta crdt text_crdt`) | add `atom=8` — additive in the enum's numbers, and **no existing vector's bytes move** (`synth.py:32-34` picks `members[tag % len(members)]`; `2 % 8` and `2 % 9` both select `message`). It adds no vector either, which is SAF-P3-8's hole: §4.1.1 must add one | **SETTLED** |

Note on row 9: the *same file* that carries the offending `Shape` enum already
carries the GDL-041 catalogue verbatim in its `shapes` block. The contract
contradicts itself inside one artefact.

### 2b. `Authority` — 2 members

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | `Authority.share=0` | present | `AUTHORITIES` (`appdecl.rs:47`); all 28 binding lines | glial default `"share"` (`glial/src/manifest.ts:70`); all 9 UI surfaces (defaulted) | DeclSurface §Contents (`dev-docs/glade/GladeDeclSurface.md:28`) | — | — | **agrees** |
| 11 | `Authority.external=1` | present | accepted by the parser; **zero uses**; the grammar has no token to name the source | never set by any consumer | DeclSurface §Contents: "`share` \| `external(source)`" | declarable but unusable: an `external` binding cannot name its source node-side | keep + extend the grammar, or drop the pair from v1 | **RULING (R5)** |

### 2c. `DomainAnchor` — 3 members

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 12 | `DomainAnchor.account=0` | present | **no `domain` field exists node-side** (`sysdata.taut.py:75-84`); **no token** in the grammar (`appdecl.rs:18`) | **produced and consumed**: `glade/demo/src/manifest.ts:29-30` declares `app:status` with `domain: "account"`, and `grip-share`'s `manifestScope` (`glade/grip-share/src/manifest.ts:63-67`) resolves it through `manifest.domains["account"].share = "account:{self}"` (`demo/src/manifest.ts:71`) to the wire `share` | GDL-039 **not ratified**; `GladeZones.md:122` "account/universal domain's exact shape" open | the anchor exists node-side only in the contract — but it is live on the demo's client path | see R1 | **RULING (R1)** |
| 13 | `DomainAnchor.document=1` | present | same — absent | glial default (`glial/src/manifest.ts:72`); `demo/src/manifest.ts:70` maps it to `doc:{doc}`; all 8 non-default UI surfaces say `'document'` and then hard-code an unrelated `share` | as above | live through one binder (grip-share), vestigial through the other (glial) | see R1 | **RULING (R1)** |
| 14 | `DomainAnchor.deployment=2` | present | absent | **no consumer anywhere** in either workzone — not in `demo/src/manifest.ts:69-72`'s `domains` table either | `GladeZones.md:42` lists a deployment row | speculative; unexercised at freeze | see R1; R7 does **not** decide it (see §3 precedence) | **RULING (R1)** |

*(Rows 12 and 14 restated — CON-P2-1. Revision 1 said "the anchor exists only in
the contract", which is false on the demo path.)*

### 2d. `ZoneKind` — 2 members

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 15 | `ZoneKind.commons=0` | present | stored as a free `STR` (`sysdata.taut.py:83`), **never validated** (`appdecl.rs:137` takes `toks[4]` raw); 26 of the 28 binding lines say `commons` | glial default (`glial/src/manifest.ts:73`); grip-share maps it to the empty wire key (`demo/src/manifest.ts:74` `commons: { key: "" }`); `chat/src/live.ts:30` is the only UI *fill* that passes `zone` | GDL-039 not ratified, but the word is used verbatim everywhere | vocabulary agrees; on the glial path the value never becomes the wire `key`, on the grip-share path it does | keep | **agrees** |
| 16 | `ZoneKind.private=1` | present | `grazel-app.glade:24` `ws.diff log share private from-cursor`; unvalidated | **the two binders disagree.** grip-share: `manifestScope` (`glade/grip-share/src/manifest.ts:64-67`) reads `decl.zone`, looks it up in `manifest.zones`, and the demo supplies `private: { key: "self:{self}" }` (`demo/src/manifest.ts:75`), filled from the grant's identity — so it **does** produce `self:<user>`, and `stubGrant` allows it (`:88`). glial: `Fill.zone` (`instance.ts:36`) is a free string that feeds `instanceKey` (`:40-41`), a **local** store key, and never the wire key — so a surface declared `private` and mounted through glial converges in the commons partition. gryth-ui mounts through glial | as above | a declared `private` surface is keyed private by one binder and not by the other; the node itself mints no `self:` prefix — it routes on opaque `key` bytes | **keep the member; caveat it.** §4.1.7 puts the scoped warning in the schema comment, the README and a new OpenNote. Reclassifying to a ruling is the cleaner fix and is available to the owner; the caveat alone closes the safety defect | **DIVERGENT BETWEEN BINDERS** |

*(Row 16 wholly restated — CON-P2-1 + SAF-P2-4, adjudicated in the remediation
plan as "both reviewers right about different binders". Revision 1 said "nothing
client-side produces that key" and classed the row **agrees**; both were wrong.
It also cited `router.rs:133` and `store.rs:409` for "node keys private zones
`self:<id>`" — **both lines are inside `#[cfg(test)]` fixtures**
(`router.rs:128-137` `keys_isolate_subscribers`, `store.rs:405-421`
`keys_are_independent_chains`), and `git grep 'self:' -- node/src` returns only
test code and `server.rs:382-404`, also a test. The node routes on opaque key
bytes and mints no prefix. The claim was inherited from
`GlialFitAssessment-2026-09-15.md:118-119`, which carries the same two citations
and words the claim more carefully — CON-P3-2.)*

### 2e. `RetentionPolicy` — 3 members + 1 absent

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 17 | `RetentionPolicy.latest=0` | present | free `STR`, unvalidated; 13 binding lines | glial `DEFAULT_RETENTION` (`glial/src/manifest.ts:64`); nothing enforces it (`glial` GAP-10, GC-4 `GlialClientRuntime.md:86` *(CON-P3-2: was `:87`)*) | none | vocabulary agrees; wholly unenforced | keep | **agrees** |
| 18a | spelling: `from-cursor` vs `from_cursor` | `from_cursor=1` | **13 binding lines across all five files spell it `from-cursor`** (`grazel-app.glade:23,24,32,33`; `gyld-app.glade:47,48`; both fixtures); `appdecl.rs:352` spells it `from_cursor` — both parse, because nothing validates | `glade/demo/src/manifest.ts:30,35,40,46,51,56` uses `from_cursor` on six surfaces — **three** of them `value`-shaped (`:29`, `:34`, `:39`), the rest `crdt` (`:45`), `log` (`:50`), `swmr` (`:55`) *(CON-P3-3: revision 1 said five)* | none | two spellings for one policy, coexisting only because the field is unchecked | a taut enum member must be an identifier, so the **contract's** member stays `from_cursor`. Whether the **file** must respell is R9's (b) sub-choice, not a settled fact — see SUR-P3-3 | **SETTLED** |
| 18b | enforcement: validate token 5 node-side | — | `appdecl.rs:137-138` stores `toks[5]` raw | — | **no ratified entry requires it** — and none governs retention at all: the log's only retention-bearing entries are GDL-006 (`:24`, exchange observation and retention) and GDL-012 (`:30`, instance retention), both **open** and neither about this token *(struck — CON-P2-7. Revision 2 opened this cell with the zones entry's ratification status, which is R1's end condition and decides nothing here; only the second clause was ever true, and rows 17, 19 and 20 correctly say "none".)* | validation is a new normative act on a vocabulary R2 has not fixed; under R2(d) there is no vocabulary to validate at all | see R2, whose sub-choice is written out in full there rather than by reference to R1's (CON-P2-7) | **RULING (R2)** |
| 19 | `RetentionPolicy.ttl=2` | present | **no app file uses it**; the grammar has no slot for its duration (see A11) | no consumer; wire `ErrorCode.Retention=5` exists and is never produced | none | unexercised at freeze, and unusable-as-specified: `binding cache.entries value share commons ttl` parses and the duration is unsayable | see R2 for whether it survives; R11 for whether the line can carry a duration | **RULING (R2)** |
| 20 | `windowed` *(absent)* | **missing** | `grazel-app.glade:25` `binding term.log log share commons windowed` — 2 lines (the file is dual-maintained), loaded by the running demo | no consumer | none. GDL-041 says a window is a projection over a base shape, i.e. not retention | the app file names a retention policy the contract does not have | see R2 | **RULING (R2)** |

*(Row 18 split — CON-P2-4. Revision 1 classed the spelling and the enforcement
together as SETTLED; the enforcement half is decided by no ratified entry and is
contradicted by R2's own option (d).)*

### 2f. `ChangeKind` — 2 members

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 21 | `ChangeKind.refresh=0` | present | n/a (client-side vocabulary) | `glial/src/events.ts:57,70`, and the `kind` parameter at `:99` | GC-1 ruled 2026-07-07 (`GlialClientRuntime.md:83`) | — | — | **agrees** |
| 22 | `ChangeKind.delta=1` | present | n/a | `glial/src/events.ts:84`, and the `kind` parameter at `:123` | as above | — | — | **agrees** |

### 2g. `GladeId`, `Retention` — 3 fields

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 23 | `GladeId.id: STR` | a message wrapping one `STR` | a bare `STR` on the record (`sysdata.taut.py:80`) and on the wire `Op` | `decl.glade_id.id` everywhere (`glial/src/instance.ts:101`; `gryth-ui` 11 sites); `grip-core/src/core/share_decl.ts:28` uses a flat `gladeId: string` | — | wrapper-vs-bare, deliberate (`OpenNotes.md` N6) ; every consumer unwraps at the boundary | keep | **agrees** |
| 24 | `Retention.policy` | `Ref(RetentionPolicy)` | a free `STR` (`sysdata.taut.py:84`) — there is no `Retention` message node-side | glial always supplies the default; **no UI call site ever writes it** | none | typed message vs free string; no reader on either side. The app-file grammar can express a `RetentionPolicy` and cannot express a `Retention` — a **third** declaration-only gap beside `external` and `source` (SUR-P2-3) | see R2; the grammar half is R11 | **RULING (R2)** |
| 25 | `Retention.ttl_ms` | optional `INT` | absent | no consumer | `OpenNotes.md` N3 records the modelling call | unexercised; its stated coupling to `policy == ttl` is documented, not enforceable in taut (`OpenNotes.md:28-30`) | see R2; R11 decides whether a file can ever state it | **RULING (R2)** |

### 2h. `BindingDecl` — 7 fields + 2 absent

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 26 | `.glade_id` | `Ref(GladeId)` | `STR`, token 1; duplicate ids refused (`appdecl.rs:195-201`, GQ-6) | `glial/src/binder.ts:54`; UI `DECLS` lookup keyed off `glade_id.id` (`gyld/src/live.ts:238-244`) | GQ-6 pinning | as row 23. Whether an app-file id is authored or derived is unstated on the contract's front page (SUR-P3-5); §4.2 step 6 fixes that text, not the field | keep | **agrees** |
| 27 | `.shape` | `Ref(Shape)` | `STR`, token 2, validated against two lists | gated by `requireMountShapeAdapter` (`glial/src/binder.ts:50`) | GDL-041 | the member set diverges (rows 3–9); the field does not | keep | **agrees** |
| 28 | `.authority` | `Ref(Authority)` | `STR`, token 3, validated (`appdecl.rs:125-130`) | defaulted, never chosen | DeclSurface | none at field level | keep | **agrees** |
| 29 | `.source` optional `STR` | present; "set iff `authority == external`" | **absent** from the record and from the grammar | `glial/src/manifest.ts:71` sets `null`; no consumer ever sets it | DeclSurface §Contents | the other half of row 11. Note both copies of DeclSurface list `BindingDecl` as six fields and omit `source` (`GladeDeclSurface.md:30`) although the contract has it at field 4 — R5's amendment sentence must fix that too (CON residual) | see R5 | **RULING (R5)** |
| 30 | `.domain` | `Ref(DomainAnchor)`, field 5 | **absent since the first commit** (`glade b9b1126`) — deliberate: `sysdata.taut.py:75-76` "app-static — no share/key here; the ServeClaim selects the node and the mount fills domain/zone/key" *(CON-P3-2: revision 1 cited `:73`, a bare `#`; the em-dash wording it quoted is `grazel/apps/grazel-app.glade:20-21`)* | **used as an anchor by one binder**: `grip-share`'s `manifestScope` reads `decl.domain` and selects a share template through it (`grip-share/src/manifest.ts:63,65,67`), the mapping `GladeZones.md:107-117` records as implemented 2026-07-12 and `grip-core/src/core/share_decl.ts:33-34` documents. Vestigial in glial: `Fill.domain` (`instance.ts:34-41`) is a *concrete string*, enters only the instance key. Four different meanings in the UI: `'gwz'`, `'gyld'` (`gyld/src/ops/surfaces.ts:64`), a chat group id (`chat/src/live.ts:30`), `'gryth-local'` (`taps.ts:34`) | GDL-039 **not ratified**; `GladeZones.md:123` "Domain anchoring" open | the contract's field 5 has no node counterpart and no app-file syntax — but it is NOT unconsumed | see R1 | **RULING (R1)** |
| 31a | `.zone` type | `Ref(ZoneKind)`, field 6 | `STR`, token 4 | glial `Fill.zone` is a separate free string; grip-share reads `decl.zone` as the key selector | GDL-039 not ratified | contract accepts two things, node accepts anything. `OpenNotes.md:16-23` (N2) records that a ratified axis vocabulary turns `zone` from an enum into a message — "a breaking change" — so the type is not settled | see R1 | **RULING (R1)** |
| 31b | `.zone` enforcement: validate token 4 node-side | — | `appdecl.rs:137` stores `toks[4]` raw; `binding g value share frobnicate latest` parses today | — | **GDL-039 not ratified** | validating against `{commons, private}` is a new normative act on an unratified vocabulary, and it converts an inert token into a boot-blocking one (SAF-P2-3) | see R1, with the hard-error-or-warn sub-choice; and it cannot land before §4.4's documentation wave (SUR-P2-1) | **RULING (R1)** |
| 32 | `.retention` | `Ref(Retention)`, field 7 | `STR`, token 5 | never read by any fold, store or engine in either language — **but read as bytes** by `appdecl.rs:264`'s registration diff (see A9) | none | message vs string; vocabulary mismatch (rows 17–20); zero semantic readers | see R2 | **RULING (R2)** |
| 33 | `app` *(absent from contract)* | — | `sysdata.taut.py:79` field 1 — the node's record carries the owning app | `appdecl.rs:133` sets it from the `app` line | GDL-037 | the node's record is structurally `AdvertisementRecord`-minus-`grip_key`, not `BindingDecl` | document the mapping `sysdata.BindingDecl.app ≡ AdvertisementRecord.package` **in the node's schema comment**, not in `glade-decl` — so the resolution survives R7(b) removing `AdvertisementRecord` *(CON-P3-4: revision 1's resolution was void under its own recommended set)*. Lands at §4.4 bullet 9 | **SETTLED** |
| 34 | `profile` *(absent everywhere)* | — | absent | **glial REQUIRES it at mount**: `binder.ts:51-53` throws unless `config.crdtProfile === "text_crdt"` for a `crdt` decl | GDL-041 ratifies profiles (`snapshot_delta`, `text_crdt`) as a distinct axis; `glade_decl.ir.json`'s `shapes` block gives each a `class: profile` and a `core` | a `BindingDecl{shape: crdt}` is unmountable without out-of-band config the contract cannot carry. **Adding the field is not free**: by §1's encoding fact it rewrites 11 of the 26 oracle vectors (SAF-P1-1) | see R4; the file-grammar half is R11, the legal-value table is §4.4 bullet 4 | **RULING (R4)** |

*(Row 31 split — CON-P2-4; row 34 re-costed — SAF-P1-1.)*

### 2i. `AdvertisementRecord` — 3 fields

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 35 | `.binding` | `Ref(BindingDecl)` | no producer | no consumer | cites GDL-029, which root `DecisionLog.md:47` lists **open** | the contract encodes an unratified decision's record format | see R7 | **RULING (R7)** |
| 36 | `.package` | `STR` | — | — | as above | grok enumeration does not exist in either workzone | see R7 | **RULING (R7)** |
| 37 | `.grip_key` | `STR` | — | — | as above | — | see R7 | **RULING (R7)** |

### 2j. `OriginMeta`, `ChangeEvent` — 8 fields

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 38 | `OriginMeta.origin` | `STR` | wire `Op.origin` | `glial/src/events.ts:59` *(CON-P3-2: was `:60`, the `payload` line)* | GC-1; `OpenNotes.md` N5 | — | — | **agrees** |
| 39 | `OriginMeta.seq` | `INT` | wire `Op.seq` | `events.ts:59` *(CON-P3-2)* | as above | — | — | **agrees** |
| 40 | `ChangeEvent.glade_id` | `Ref(GladeId)` | n/a | `events.ts:55` *(CON-P3-2: was `:56`, the `shape` line)*, and `:68, :82, :97, :121` | GC-1 | — | — | **agrees** |
| 41 | `ChangeEvent.shape` | `Ref(Shape)` | n/a | set to the delivery shape; a `text_crdt` profile still emits `"crdt"` (`events.ts:122`) | GDL-041 | — | — | **agrees** |
| 42 | `ChangeEvent.kind` | `Ref(ChangeKind)` | n/a | `events.ts:57,70,84`, and the pass-through `kind` at `:99,:123` | GC-1 | — | — | **agrees** |
| 43 | `ChangeEvent.base_seq` | optional `INT`; schema comment: "the refresh baseline a **delta** applies against" | n/a | glial populates it on **refresh** too — `events.ts:58` (value), `:71` (`records.length`), `:100` (swmr assembly seq) — and `null` on a crdt refresh (`:124`) | GC-1 | the documented meaning is narrower than the implemented one | amend the comment to "the position this event is anchored at"; **comment-only, so no byte change** (a comment is not encoded) | **SETTLED** |
| 44 | `ChangeEvent.origin_meta` | optional `Ref(OriginMeta)` | n/a | set only when origin+seq are known (`events.ts:59`) *(CON-P3-2)* | GC-1; N5 | — | — | **agrees** |
| 45 | `ChangeEvent.payload` | `BYTES`, opaque | n/a | interim glial encoding, explicitly temporary (`glial` GAP-5) | GC-1 split | none — the shell is what is frozen | keep | **agrees** |

### 2k. `GladeIdManifest` — 3 fields

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 46 | `.package_id` | `STR` | no producer; every glade id in every app file is a literal | `glial` GAP-14: "GQ-6 NOT implemented (seam left)" (`glial/dev-docs/DecisionLog.md:274`) | GQ-6 named in DeclSurface; the algorithm is explicitly deferred (`OpenNotes.md` N4, `:43-47`) | the record format is frozen before the function that fills it exists | see R6 | **RULING (R6)** |
| 47 | `.grip_key` | `STR` | — | — | as above | — | see R6 | **RULING (R6)** |
| 48 | `.glade_id` | `Ref(GladeId)` | — | — | as above | — | see R6 | **RULING (R6)** |

### 2l. The two documented interfaces + one absent message

| # | Element | Contract | Node / files | Clients / binders / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 49 | `canonical_key(param_shape_ir, params) -> BYTES` | a documented signature (`README.md:47`, `OpenNotes.md` N4) | keys are raw bytes on the wire; the node mints no prefix (row 16) | **no implementation anywhere.** Four ad-hoc conventions in use: `groupKey(id)=utf8(id)` (`glade-chat/src/manifest.ts:31-33`), `utf8(runId)` (`gryth-ui .../gwz/src/live.ts:48` *(CON-P3-2: was `:47`)*), `utf8(<fill key>)` (`gryth-ui .../gyld/src/live.ts:77`, also `:184`, `:257` — the ask surface's key is the conversation id, set at `gyld/src/ops/surfaces.ts:112`; `utf8` appears nowhere in `surfaces.ts`) *(CON-P3-2)*, and grip-share's `utf8(fill(keyTmpl, vars))` (`grip-share/src/manifest.ts:67`) *(rev-2 pass: a fourth, and the only data-driven one)* | DeclSurface §Contents lists the interface | a named contract interface with no implementation and four incompatible substitutes | see R6 | **RULING (R6)** |
| 50 | `derive_glade_id(package_id, grip_key) -> GladeId` | a documented signature; golden vectors deferred (N4) | glade ids are literals in every app file | `glial` GAP-14 says the seam is left open on purpose | GQ-6 | all three languages must compute it identically; none computes it | see R6 | **RULING (R6)** |
| 51 | `ServiceDefinition` / ACL seed / workspace entry *(absent from contract)* | — | `sysdata.taut.py:89-92` `ServiceDefinition`; `:60-63` `CapabilityGrant`; `:45-48` `WorkspaceEntry`; grammar lines `service`, `seed`, `workspace` (`appdecl.rs:141-179`) | `client-rs/src/supplier.rs:156`; glial `requireExchangeShape` | **GDL-037 ratified**: the file form of this surface is "its `BindingDecl`s, `ServiceDefinition`s, and **ACL seeds**" (`dev-docs/glade/GladeDeclSurface.md:113-114`) | the contract covers one of the three record kinds a ratified decision assigns to it | see R8 | **RULING (R8)** |

### 2m. Artefact-level divergences (outside the element table, inside the amendment)

| Id | Finding | Evidence | Class |
| --- | --- | --- | --- |
| A1 | All three renderings pin `CONTRACT_VERSION = "99a04e0b…"`, now **two** commits behind `glade-decl` HEAD `d671f10` — `bbce73d` added `crdt=7` and `d671f10` is A2's fix. All three nevertheless render `crdt`. The pin understates the contract it renders, and nothing checks it: it is a string constant no gate reads (SAF-P2-5). | `glade-decl-ts/src/index.ts:18`, `glade-decl-rs/src/lib.rs:38`, `glade-decl-py/src/glade_decl/__init__.py:21`; `git -C glade-decl log --oneline -4` | **SETTLED** — re-pin at the amendment commit, and either make it checkable or stop calling it a pin (§4.3) |
| A2 | ~~The drift gate is red today.~~ **RESOLVED by the owner, 2026-09-21, between review round 1 and this revision.** Revision 1 reported `python3 corpus/build.py --check` exiting nonzero with `STALE: glade-decl-rs/src/vectors.rs`, caused by uncommitted rustfmt-only edits to `glade-decl-rs/src/{api,vectors}.rs` (284 / 43 lines) against a generator that emits unformatted text — a gate that could not distinguish formatting from content. The owner ruled **"discard them and have `build.py` run rustfmt"**: the working-tree edits were discarded; `corpus/build.py` now formats the generated Rust through `rustfmt` on the write path *and* before `--check` compares (`_rustfmt` at `:179-201`, `_vectors_text` at `:204-206`), taking the edition from the crate's `Cargo.toml` and honouring a crate `rustfmt.toml`, and failing loudly if `rustfmt` is absent rather than comparing unformatted against formatted; `src/api.rs` and `src/vectors.rs` are committed formatted; and both rendering procedures gained a `cargo fmt` step after the copy-in (`glade-decl/README.md:90-98`, `glade-decl-rs/README.md:28-40`). | `glade-decl d671f10` ("Format the generated Rust before writing and before the gate"), `glade-decl-rs 21eefa1` ("Commit the generated sources formatted"), glade-wz root `dc311ba`. Re-verified here: `git -C glade-decl-rs status --short` empty; `/opt/homebrew/bin/python3 corpus/build.py --check` → `all 3 glade-decl artifacts in lockstep with the schema.`, exit 0. `ir/glade_decl.ir.json` (`1e7d4d16…`) and `corpus/decl.v0.json` (`f669da0b…`) are **byte-unchanged** from `bbce73d`, so every schema and corpus citation in this document still holds | **CLOSED** — no longer a precondition on the freeze. Two consequences remain live: `--check` still compares only **3** artefacts, not the four rendering copies (SAF-P2-5, §4.3), and `CONTRACT_VERSION` is still unchecked (A1) |
| A3 | A superseded SKETCH of the same schema lives inside the node repo: `glade/decl/glade_decl.taut.py` (`Shape { value, log, message, stream, exchange, window }`, `domain: STRING`, `zone: STRING`, `retention: STRING` with the vocabulary "ttl / latest / from-cursor") plus `glade/decl/README.md` marked "Status: SKELETON". This is the origin of the node's free-string fields and of the hyphen spelling in row 18a. | `glade/decl/glade_decl.taut.py:13,28-30`; `glade/decl/README.md:19-20` | **SETTLED** — archive or banner it as superseded by `glade-decl/`; it is a second answer to a frozen question. Lands at §4.4 bullet 10 |
| A4 | The rendering procedure is documented in **five** places, **three** of them correct and complete: `glade-decl/README.md:78-88` (three `-l` invocations, `-o <out>`, no copy step — though `:90-98` now adds a Rust copy-and-`cargo fmt` block, A2) and `glade-decl/ir/glade_decl.taut.py:31-33` (`-l rust,typescript,python --api-only [--with-runtime]`, `-o <out>`, no copy step) are the two incomplete contract-side forms; `glade-decl-rs/README.md:28-34`, `glade-decl-ts/README.md:29-34` and `glade-decl-py/README.md:32-37` each generate to `/tmp/g`, copy `/tmp/g/<lang>/*` into `src/`, and — for ts and py — copy the ir + corpus (`-ts/README.md:33`, `-py/README.md:36`); rs runs `build.py` for its generated `src/vectors.rs` (`-rs/README.md:32`) and then `cargo fmt` for `api.rs` (`:33`). There are **four** ir/corpus copies in **two** renderings, all four byte-identical to the contract's (`glade_decl.ir.json` `1e7d4d16…`, `decl.v0.json` `f669da0b…`); `glade-decl-rs/src/` holds none. | as cited; `shasum glade-decl/ir/glade_decl.ir.json glade-decl-{ts/src,py/src/glade_decl}/glade_decl.ir.json` etc.; `ls glade-decl-rs/src/` | **SETTLED** — collapse onto the **rendering READMEs'** shape, including the copies, since the copies are what the gates read. *(Restated — CON-P3-6. Revision 1 said "twice", "neither documents the copy step", "all three copies", and would have deleted the three correct forms; that error is what produced revision 1's §4.3 — CON-P2-2.)* |
| A5 | `glade-decl-ts` carries an npm `package-lock.json` (45 687 B, lockfileVersion 3) and no `pnpm-lock.yaml`. It is **one of two** such TypeScript members, not the only one: `glade-wz/ggg-viz` is the other. `grip-core` and `grip-react-demo` carry both; `glial`, `grip-react`, `taut-shape-ts`, `glade-chat` and `gryth-ui` are pnpm-only. **And its README still teaches npm**: `glade-decl-ts/README.md:21-22` is `npm install` / `npm test`, while §4.7 row 3 and §4.8 both prescribe `pnpm` and §4.3 adopts that README's own shape as canonical *(added — CON residual)*. | `find . /Users/owebeeone/limbo/gryth-wz/gryth-ui -name package-lock.json -not -path '*/node_modules/*'` → 4; `find … -maxdepth 2 -name pnpm-lock.yaml …` → 7, both at the Appendix tuple. *(CON-P3-3: revision 1 said "only".)* | note only — see §4.8, tooling, and A4's collapse |
| A6 | GDL-038 (ratified) says base glade ships `glade-sys.glade`. **No such file exists** in either workzone. | `find /Users/owebeeone/limbo/{glade-wz,gryth-wz} -name glade-sys.glade` → empty, at the Appendix tuple | **out of scope** — adjacent to R8; it rides GDL-038's own amendment, not this one (§4.0) |
| A7 | `glade/client-rs/src/session.rs:19-30` — `shape_of` accepts `"crdt"` (`:24`) but its error text (`:27`) lists "supported: value, log, swmr". | as cited | **SETTLED** — message fix; lands at §4.4 bullet 11 |
| A8 | `gryth-ui/packages/glade/src/glade.ir.json` is the **node's WIRE IR** vendored by hand (`runtime.ts:31-32`), not the declaration IR — a different contract, a same-named different `Shape` enum. It is separately stale: `{value:0, log:1, stream:2}` against the source's `{value:0, log:1, stream:2, swmr:3, crdt:4}`. | agent-verified 2026-09-21; `PackageExtractionPlan.md` step 2.6 owns the fix; `GlialFitAssessment` S1 (`:149`) owns the drift check | **out of scope** — named in §4.6 as must-not-change |
| A9 | **A changed binding line has no defined effect on an already-registered declaration, and a deleted one has none at all.** `register` diffs on `(glade_id, payload bytes)` (`appdecl.rs:264`), so any token edit appends a **second** `BindingDecl` beside the first. Nothing folds `dir.bindings`: `RegistryApi` (`registry.rs:191-211`) exposes `append/who_serves/replicas_of/grants_for/nodes_of/snapshot` and no binding query; the only production reader is `exchange.rs:62-78`'s `declared_exchange`, an `any()` over every op. The duplicates are ordinary home-share records, so they fan out to every peer and to every client that subscribes to `dir.bindings` — demonstrated end to end by `exchange.rs:651-658`, which is **test code**: `mod tests` opens at `:266` and the function is `grazel_attach_end_to_end` (`:599`), a `#[tokio::test]` that boots a real node, connects a WebSocket client and reads the `BindingDecl` records back as Ops. It is evidence of production behaviour rather than a production call site, and it is marked as such here for the same reason row 16's `#[cfg(test)]` fixtures are *(CON-P3-7: revision 2 cited it unmarked, alone among this document's test citations, while supporting its newest ruling)*. Removing a line appends nothing and retracts nothing; **the grammar has no retract form**, which is true on the code: `parse()`'s directive arms are exactly `app` (`:98`), `binding` (`:107`), `service` (`:141`), `seed` (`:155`), `workspace` (`:168`) and a catch-all refusal (`:180`), and none of them retracts. The published grammar is weaker evidence than the object claimed: `GladeGrazelAttachNotes.md:30-35` lists every directive **it publishes** — and it omits `workspace`, which `appdecl.rs:21` documents and `:168-179` parses *(CON-P3-7: revision 2 said it "lists every directive")*. | as cited; `appdecl.rs:404-413` `registering_twice_appends_nothing` registers the **same** decl twice into a **fresh** `Registry` and cannot see this | **RULING (R9)** — the amendment's own headline action (rewriting a token on up to 15 lines) is the first mass exercise of an undefined operation (SAF-P1-2 + SUR-P2-5) |
| A10 | **`glade-app v0` would name two incompatible languages.** The header is parsed (`appdecl.rs:89-95`, `toks != ["glade-app", "v0"]`) and never advanced. After token validation, a third-party file that boots today stops booting, with the same header, so neither the user nor the parser can tell a pre- from a post-amendment file and no diagnostic can name the version a spelling changed in. There is no CHANGELOG, migration or release note in any of the **four** repositories that hold app files, nor in `glade-decl`. | `grazel/README.md:73-79` documents `--app` repeated, i.e. third-party app files are the intended shape. Census command, run at grazel `c66f029` · glade `559cb2c` · glade-gyld `65da8cb` · glade-gwz `e53c87d` · glade-decl `d671f10`: `for r in grazel glade glade-gyld glade-gwz glade-decl; do git -C $r ls-files \| grep -icE 'changelog\|migrat\|upgrad\|releas'; done` → **`0 2 0 0 0`**. The two are `glade/dev-docs/async-witness/real/tests/peer_release.rs` and `release_order.rs` — Rust tests about lock-release ordering, matched on the filename and **not** release notes, so the claim holds and its proof needed re-stating rather than the claim *(re-pinned — CON-P3-8 / SUR-P3-8: revision 2 published "`0` five times" at glade `960c9b0`; the async-witness workspace landed at `559cb2c`)*. Read as a count of release-note documents it is still `0` in all five. *(rev-2 pass: the Surface report said "five repos"; the five `.glade` **files** live in four repositories)* | **RULING (R10)** (SUR-P2-2) |
| A11 | **The binding line's last positional slot is contested.** `appdecl.rs:111-114` requires exactly six whitespace tokens (`binding` + 5). Two things want position 6: `ttl`'s duration (row 19/25) and, under R4(a), the shape profile (row 34). Whichever takes it, the other can never be added without a compatibility break, because position 7 would make position 6 mandatory. The format has no `key=value` form. | as cited; `GladeGrazelAttachNotes.md:32` is the published 5-token grammar | **RULING (R11)** (SUR-P2-3 + SUR-P2-4) |
| A12 | **The repo being frozen ships a design document that contradicts the ratified catalogue.** `diff dev-docs/glade/GladeDeclSurface.md glade-decl/dev-docs/DeclSurface.md` → three differing lines. `:27` — root: "canonical engines are `value`, `atom`, `log`, `stream`, `swmr`, `crdt`; … Registry recognition does not grant runtime support. … `message` is unsupported; `window` is a view over an explicit base shape." glade-decl copy: "the delivery-shape enum (`value`, `log`, `message`, `stream`, `exchange`, `window`, `swmr`, `crdt`) — names taut-shape engines, owns none of them; `text_crdt` is a profile over `crdt`" — pre-GDL-041. `:30` lacks the mount-instance clause; the root's `:32` Supplier row (GDL-040) is absent entirely (149 lines vs 148). | as cited | **SETTLED** by §1 rule 1 — a ratified entry outranks the copy, so the root document is controlling. **R8 decides only** whether `GladeDeclSurface.md` additionally gains a record-kinds amendment sentence. Lands at §4.2 step 5 (CON-P2-5) |

---

## 3. The rulings needed

Eleven rulings — but **thirteen answers**: R1 and R2 each carry a sub-choice
(rows 31b and 18b), and each sub-choice is a separate act with its own three
options. Each of the thirteen is answerable in a line. *(Stated explicitly —
CON residual: revision 2 said "Eleven. Each is answerable in a line", and a
reader counting answers came up two short.)*

**RULED (Gianni 2026-09-23): every recommendation, and b2 within R9(b).** The
thirteen answers: R1 (a), with (b) as a separate step · 31b (ii) · R2 (a) ·
18b (ii) · R3 (a) · R4 (b) · R5 (b) · R6 (a) · R7 (b) · R8 (b) · R9 (b2) + (a),
(s) not taken · R10 (a) · R11 (a). Each ruling's own line below says what it
fixes; §4.0 says how §4's conditional steps and §4.7(b)'s gates resolve under
these answers.

**Not reached by these answers** (found while recording them; neither is one of
the thirteen), **and ruled the same day**:

- *Which spelling of `from_cursor` an app file may write.* Under R9(b2) both
  `from-cursor` and `from_cursor` parse and normalise, and R9's "Which spellings
  a file may contain" hands the question of whether the hyphen is nevertheless
  the file's only legal spelling to 18b. But 18b's options say whether token 5 is
  validated, not in which spelling, so (ii) does not answer it. It matters
  because §4.4's token-5 block and §4.7 row 18 gloss `from_cursor`, the
  contract's spelling, while bullet 7's message under b2 names `from-cursor`.
  Due before plan Step 2.3, whose format page glosses the token. No app file
  contains `from_cursor` today (§4.4 bullet 6's census, re-run 2026-09-23 at the
  same four revisions: 13 `from-cursor`, 13 `latest`, 2 `windowed`; the tree's
  only `from_cursor` tokens are in two `dev-docs/examples/*.glade` files, the
  other dialect §4.4 bullet 14 banners), so either answer edits no line.
  **RULED (Gianni 2026-09-23): the hyphen only.** An app file writes
  `from-cursor`; `parse()` normalises it to `from_cursor` for the stored record
  (R9(b2)). A file that writes `from_cursor` is recognised and warned, naming
  `from-cursor`, for one release, and refused after (18b (ii)) — the rule R9
  already states for b1's file side. Pages an author reads gloss `from-cursor`;
  the contract's `from_cursor` is the stored and published spelling.
- *Who owns `canonical_key`.* R6(a)'s hardening gives it a named owner, and the
  ruling names none. Due at plan Step 2.1, where `OpenNotes.md` N4 carries the
  name. **RULED (Gianni 2026-09-23): Gianni.**

### Which ruling decides what (CON-P3-4)

Each row below is decided by **exactly one** ruling. Where revision 1 pointed a
row at two, the second pointer is removed here.

| Ruling | The rows it decides, exclusively |
| --- | --- |
| R1 | 12, 13, 14, 30, 31a, 31b |
| R2 | 18b, 19, 20, 24, 25, 32 |
| R3 | 3, 6 |
| R4 | 34 |
| R5 | 11, 29 |
| R6 | 46, 47, 48, 49, 50 |
| R7 | 35, 36, 37 — **`AdvertisementRecord` only** |
| R8 | 51 |
| R9 | A9 |
| R10 | A10 |
| R11 | A11 |

**Precedence between rulings.**

- **R7 does not decide `DomainAnchor.deployment` or `RetentionPolicy.ttl`.**
  Revision 1's R7 question text claimed both, which made R1(d)+R7(a) and
  R2(c)+R7(a) legal contradictory pairs. `deployment` follows **R1**; `ttl`
  follows **R2**. R7's own answer applies to `AdvertisementRecord` alone.
- **R11 is subordinate to R2 and R4.** It decides the *form* of any sixth thing
  on the line, not whether there is one. If R2 drops `ttl` and R4 returns (b) or
  (c), R11's option (c) is free and (a) is work for nothing.
- **R9 constrains R2's landing, not its answer.** Whatever retention vocabulary
  R2 returns, R9 decides what the *token edit* does to stored records. R9(b) can
  reduce the number of lines R2's answer forces anyone to edit; it cannot change
  which words are legal.
- **R10 constrains R1's and R2's landing.** Any validation the owner turns on
  under R1(31b) or R2(18b) is what shrinks the language, so R10's answer decides
  whether that shrinkage is announced.
- **R3 and R4 are independent of each other**, and both are independent of R1/R2.

**Precedence between rulings and SETTLED rows.** A SETTLED row is one that (a) a
ratified DecisionLog entry decides, or (b) changes no input the node accepts
today. No ruling's option may void a SETTLED resolution; where revision 1's did,
the row was restated (row 33) or split (rows 18, 31). Concretely:

- Rows 4, 8, 9 and A12 are settled by GDL-041 and survive every combination of
  R1–R11.
- Row 18a is settled by taut's identifier rule for enum members and constrains
  only the **contract's** spelling; the **file's** spelling is R9(b)'s sub-choice.
- Row 33 is settled and now lands node-side, so R7(b) cannot void it.
- Row 43 is comment-only and cannot conflict with anything.
- Row 16 is DIVERGENT, not SETTLED: it is a true statement about two binders,
  and the caveat it demands (§4.1.7) rides every option of R1.

### R1 — Does `BindingDecl.domain` (and `DomainAnchor`) survive, change, or go?

**RULED (Gianni 2026-09-23): (a) now, (b) as a separate step; row 31b (ii).**
`BindingDecl.domain` and `DomainAnchor` stay as they are, documented as a
declaration-time hint the binder resolves (grip-share already does; glial gains
the mapping later, GlialFitAssessment S6). The zone token is validated against
`{commons, private}` as a warning for one release and a hard error after,
landing by §4.4's order. No vector's bytes move.

**The question.** The contract says a surface declares which replicated world it
anchors to. The node has never had the field and the app-file grammar has no
token for it — but one of the two binders *does* resolve the wire `share` through
the anchor. Keep the anchor, replace it with the concrete thing, or delete it?
And, separately (row 31b): does the node start validating the zone token?

**Why it is a ruling and not settled.** GDL-039 — the zones vocabulary that
introduced domain/zone — is `open (implemented, pending ratify)` (root
`DecisionLog.md:59`), and `GladeZones.md:119-126` still lists "Domain anchoring"
and "whether `domain`/`zone` replace `share`/`key` as the wire field names" as
open. No ratified text decides this. Validation (31b) is a *new normative act* on
that unratified vocabulary, which is the R7 defect applied to GDL-039.

**Options.** *(A `glade/demo` + `glade/grip-share` column added — CON-P2-1.
Revision 1 costed (c) and (d) as glial-only, which is false and contradicted
§4.6's "running demo" clause.)*

| | Contract | Node | glial | **`glade/grip-share` + `glade/demo`** | App files |
| --- | --- | --- | --- | --- | --- |
| **(a) Keep as-is**, document it as declaration-only | no edit | no edit | no edit; `Fill.domain` stays unrelated | no edit — `manifestScope` keeps working exactly as today | no edit |
| **(b) Keep + make it mean something**: glial gains the `(DomainAnchor, ZoneKind, principal) → (share, key)` mapping | no edit | no edit | GlialFitAssessment S6 (`:154`), ~150 LOC; retires 4 hand-rolled mappings (rows 30, 49) | **the fifth mapping already exists here** and is the data-driven one: `manifestScope` + the demo's `domains`/`zones` tables. S6 should lift its shape rather than invent a sixth | no edit |
| **(c) Replace the anchor with a concrete `domain: STR`** | breaking field-type change; **all 11 `BindingDecl`/`AdvertisementRecord` vectors change** (§1 encoding fact) | grammar gains a token; `sysdata.BindingDecl` gains a field → durable-record change | `Fill.domain` becomes redundant | `manifest.domains` keying breaks: `demo/src/manifest.ts:69-72` is keyed by the anchor **member names** (`document`/`account`), a rekey `GladeZones.md:110-117` records as deliberate. A concrete string would have to re-introduce the retired `ANCHOR` translation table | every binding line gains a token |
| **(d) Delete `domain` + `DomainAnchor`** | removes field 5 and an enum; **all 11 vectors change**; tag 5 and the name must be reserved (§4.1.8) | node already agrees | `Surface.domain` and `SurfaceSpec.domain` go; `glial/src/manifest.ts:72` default goes | **breaks the running demo's share resolution**: `manifestScope` reads `decl.domain` (`grip-share/src/manifest.ts:63`) and `grip-core/src/core/share_decl.ts:35` types it. This is what §4.6 forbids | no edit |

**Recommendation: (a) now, (b) as a separate step.** Reasons: (d) changes every
`BindingDecl` vector's bytes **and** breaks the demo path §4.6 protects — a cost
revision 1 did not state; it also deletes a vocabulary a ratification is still
pending on. (c) contradicts the node's own stated design (`sysdata.taut.py:75-76`)
and GDL-037's app-static rule, and undoes a deliberate 2026-07-12 rekey. (a)
costs one comment and freezes nothing new; (b) is the work that makes the field
earn its place, it is client-side, and the shape to copy already exists in
`grip-share`. Say so in the schema: *the anchor is a declaration-time hint; the
binder resolves the concrete world; grip-share already does, glial does not yet.*
If the owner instead wants the field gone, say so now — after the publish it
costs a major version.

**Sub-choice, row 31b — validate token 4?** (i) Validate against
`{commons, private}` as a **hard error**, consistent with the shape and authority
checks. (ii) Validate as a **warning for one release**, hard error after. (iii)
Do not validate until GDL-039 ratifies. Costs: (i) turns an inert token into a
boot refusal across four repositories (SAF-P2-3), so it cannot land before the
app files and before §4.4's documentation wave (SUR-P2-1); (ii) needs a warning
channel `parse()` does not have (see R10(a)); (iii) costs nothing and leaves
`binding g value share frobnicate latest` parsing. **Recommendation: (ii)**, with
the landing order of §4.4 — it is the only one that is never worse than today.

This sub-choice is R1's alone. R2's row-18b sub-choice is **written out
separately** there, and it is not "the same three options": (iii) here can name
GDL-039 as its end condition, because GDL-039 *is* the zones vocabulary that
introduced this token; the corresponding option under R2 has no entry of any kind
to wait for, which is why it is stated in full there rather than imported
(CON-P2-7).

**Note on `private` (SAF-P2-4).** Whatever R1 returns, row 16's caveat lands:
`private` is honoured by the grip-share binder and **not** by glial mounts, and
gryth-ui mounts through glial. Validating the token (31b) *elevates* `private`
from an unread string to a checked vocabulary, which makes the caveat more
necessary, not less.

### R2 — What is the retention vocabulary, and is it enforced?

**RULED (Gianni 2026-09-23): (a); row 18b (ii).** v1 keeps `{latest,
from_cursor, ttl}` with `ttl_ms` and drops `windowed`, which becomes
recognised-but-refused; `term.log`'s `windowed` line, in both homes of
`grazel-app.glade`, becomes `from-cursor` (the file's spelling under R9(b2)),
with the window kept app-side. Token 5 is validated as a warning for one release
and a hard error after. The contract edit is comment-only; no vector's bytes
move.

**The question.** The contract says `{latest, from_cursor, ttl}` + `ttl_ms`. The
app files say `latest`, `from-cursor`, `windowed`. Nothing reads any of it as a
policy. What is the v1 set, and is it a promise or a comment?

**Options.**

| | Contract | Node | glial | App files |
| --- | --- | --- | --- | --- |
| **(a) Keep the three; drop `windowed`** | comment only — no byte change | validate token 5 (see sub-choice); `windowed` becomes recognised-but-refused (SUR-P3-2) | none | `term.log windowed` → the ruled replacement (2 lines, dual-maintained). The 13 `from-cursor` lines move only if R9 says the *file* must respell |
| **(b) Keep three + add `windowed`** | additive `windowed=3`: an enum member, so **no existing vector changes** (`2 % 9` still selects `message` for `BindingDecl.shape`; `Retention.policy` is tag 1 over 4 members → `1 % 4 = 1`, still `from_cursor`, so `edge/retention-*` are stable) | as (a) | must then define what a window retains | 0 meaning edits |
| **(c) Keep `{latest, from_cursor}`, drop `ttl`/`ttl_ms`** | removes a member and a field. Of the 4 `Retention`-typed vectors, 3 shrink `a2 …` → `a1 …` and `edge/retention-ttl` is deleted; all 11 `BindingDecl`/`AdvertisementRecord` vectors change too, because each embeds a `Retention` at tag 7. `Retention`'s tag 2 and the names `ttl`/`ttl_ms` must be reserved (§4.1.8) | as (a) | none | as (a) |
| **(d) Demote `retention` to a free `STR`, matching the node** | `Retention` message deleted (its 4 vectors go); `BindingDecl.retention` becomes `STR`, so tag 7 goes from a nested map to a text string in **all 11** `BindingDecl`/`AdvertisementRecord` vectors | already free; **nothing to validate** | `DEFAULT_RETENTION` becomes `"latest"` | as (a) |

**Recommendation: (a), with the enforcement as a sub-choice, not a given.**
`windowed` is not a retention policy — GDL-041 rules a window an application
projection over a base shape, so `term.log` wants `from_cursor` with an app-side
window, and keeping `windowed` would encode a category error in a frozen enum.
`ttl` stays because it is cheap, byte-free in the enum, and GC-4
(`GlialClientRuntime.md:86`) names retention enforcement as live future work —
but see R11: as the grammar stands, `ttl` is authorable and its duration is
unsayable, so "keep `ttl`" and "give the line a keyword tail" are one decision in
two parts. (d) is the honest "it means nothing" answer but throws away the only
typed vocabulary either side has, and it is not cheap: it rewrites every
`BindingDecl` vector.

**Sub-choice, row 18b — validate token 5?** *(Written out in full — CON-P2-7.
Revision 2 stated it as "Same three options as R1's 31b, same recommendation",
which imported R1's end condition and R1's zone-specific cost example onto a
token neither governs.)* The set the node would validate against is whichever
set (a)–(d) above returns.

- **(i) Validate as a hard error**, consistent with the shape and authority
  checks at `appdecl.rs:116-130`. Cost: it turns an inert token into a boot
  refusal across four repositories (SAF-P2-3), so it cannot land before the app
  files and before §4.4's documentation wave for token 5 (SUR-P2-6) — today no
  published page glosses a single retention value.
- **(ii) Validate as a warning for one release**, hard error after. Cost: a
  warning channel `parse()` does not have — it returns `Result<AppDecl, String>`
  — so it needs R10(a)'s API change or an `eprintln!` in `load`.
- **(iii) Do not validate; the token stays inert, and R2's own vocabulary is the
  only thing that could ever make it validatable.** There is no external end
  condition to wait for, and that is the substantive difference from R1's (iii):
  R1's zone token has a pending zones ratification over it, which R1's own (iii)
  names, whereas **no decision-log entry governs retention at all**. The log's
  only retention-bearing entries are GDL-006 (`DecisionLog.md:24`, exchange
  observation and retention) and GDL-012 (`:30`, instance retention); both are
  open, and neither is about a declaration's retention token. So (iii) ends when
  R2 fixes the vocabulary and the owner returns to this sub-choice — not when
  some other entry ratifies. An end condition borrowed from R1 would one day read
  as satisfied by an event that decides nothing here, and token 5 would then be
  validated against a vocabulary no ratified entry decides: exactly the act
  CON-P2-4 split row 18 to prevent.

**Cost, on a token-5 value.** Under (iii),
`binding cache.entries value share commons hourly` keeps parsing today and keeps
parsing after the amendment — and `"hourly"`, a policy no enum has, is written
into a durable record (`appdecl.rs:138` → `sysdata.BindingDecl.retention`) and
read as **bytes** by `register`'s diff (A9). That is (iii)'s real price: not a
wrong answer, but a token that no v1 vocabulary constrains and that any later
validation turns into a boot refusal on a file already in the field. Under (i)
the same line is a line-numbered refusal at the first boot after the node
lands; under (ii) it is a warning for one release and then that refusal.

**Recommendation: (ii)**, with the landing order of §4.4 — the same reason as
R1's 31b: it is the only one of the three that is never worse than today.

*(Revision 1 ended this section with "Whatever is chosen, the spelling
normalizes to `from_cursor` and the node starts validating the token — that part
is settled (row 18/31)". **Deleted** — it is false under this section's own
option (d), and it asserted a normative act on a vocabulary no ratified entry
fixes. CON-P2-4.)*

### R3 — `message`, `window`: delete the members, or retain them reserved?

**RULED (Gianni 2026-09-23): (a).** `message` and `window` stay as reserved,
recognised-but-unbindable members with their numbers; `atom=8` is added with its
curated vector. No existing vector's bytes move.

**The question.** GDL-041 says `message` is unsupported and `window` is not an
engine. Every runtime refuses both. Does the enum lose the members, or keep them
as reserved-and-unbindable names?

**Options.**

| | Contract | Corpus | Node | glial | UI |
| --- | --- | --- | --- | --- | --- |
| **(a) Retain reserved**, comment says recognition ≠ support; add `atom=8` | comment + one additive member | **every existing vector's bytes unchanged** (verified: enums encode as their integer wire value, and `synth.py:32-34`'s `members[2 % 8]` and `members[2 % 9]` both select `message`); two vectors get relabelled as recognition cases; §4.1.1 adds an `atom` vector | no edit (`appdecl.rs:39-42` already implements exactly this posture) | no edit | no edit |
| **(b) Delete both members** | two members go; the surviving numbers for `stream..crdt` must be held (taut allows gaps — `validate.py:77-80` checks only for duplicate wire values); the retired numbers want recording in the enum comment, because taut has no enum `reserved` (§4.1.8) | `edge/binding-message-private` and `edge/binding-deployment-window` are removed → the corpus content changes even where bytes do not. **And the synth `BindingDecl`/`ChangeEvent` vectors change regardless, because synth selects by member *index*, not by wire value**: `Shape` drops to 6 members, `2 % 6 = 2` now selects `stream` (held wire value 3), so tag 2 goes `02` → `03` (SAF-P3-7) | `KNOWN_SHAPES` loses two | no edit | no edit |

**Recommendation: (a).** Two reasons. GDL-041's own words are "Recognition MUST
NOT imply runtime support" — an argument for keeping the name, not deleting it.
And the node already implements (a) verbatim and says why: "Legacy
wire/declaration names remain recognizable so diagnostics can be precise and
numeric wire values remain reserved" (`appdecl.rs:39-42`).

*(Revision 1 gave a third reason — "(a) makes the v1 corpus a strict superset of
v0's bytes, which makes the compatibility proof in §4 free". **Struck** —
SAF-P1-1. Superset-ness is decided by R4, not by R3: under R4(a) no option of R3
yields a superset, and under R4(b)/(c) the two R3 options differ exactly as the
table above says: (a) moves no byte, while (b) moves the synth `BindingDecl` and
`ChangeEvent` vectors — the enum's **wire values** are stable under both, but
synth selects by member index. §4.2's compatibility enumeration files (b) under
the byte-moving options for that reason.)*

Note `PackageExtractionPlan.md:155` phrases the option as "`message`/`window`
out, `atom` in" — that is a plan's shorthand, not a ratification, and this is the
ruling that settles it either way.

### R4 — Does `BindingDecl` carry the shape profile?

**RULED (Gianni 2026-09-23): (b).** The profile is a separate message keyed by
glade id, `ShapeProfileDecl{glade_id, profile}` (§4.1 item 2). `BindingDecl` is
untouched, so no existing vector changes and the corpus gains one synth vector.
An app file writes the profile as R11(a)'s `shape-profile=` key.

**The question.** GDL-041 ratifies `snapshot_delta` and `text_crdt` as profiles
over `swmr`/`crdt`. glial *requires* the profile at mount and throws without it
(`binder.ts:51-53`). The contract cannot express it, so a `BindingDecl{shape:
crdt}` is not independently mountable.

*(Wholly rewritten — SAF-P1-1. Revision 1 said "(a) … additive, one field, absent
on every existing vector so v0 bytes are unaffected". That is false: taut
`optional` is nullable-and-always-emitted, so the field is written as an explicit
null on every vector.)*

**Options.**

| | Contract bytes | Corpus | Node | glial | App files |
| --- | --- | --- | --- | --- | --- |
| **(a) Add optional `profile: STR` to `BindingDecl` (field 8)** | **breaking for 11 of the 26 vectors**: every `BindingDecl` map goes `a7 …` → `a8 … 08 f6`, and both `AdvertisementRecord` vectors embed one (`edge/advert` opens `a3 01 a7 01 …`). The synth vector gets `08 62 7338` | the 9 `BindingDecl` + 2 `AdvertisementRecord` vectors are regenerated; v1 is then **not** a byte superset of v0 | `sysdata.BindingDecl` gains field 7 → `to_cbor` (`sysdata.rs:128-137`) emits a 7-entry map where every stored record is a 6-entry map → see R9 | reads `decl.profile` ahead of `config.crdtProfile`, keeping the config path for back-compat | gain a sixth thing on the line → R11 |
| **(b) A separate message keyed by glade id** — e.g. `ShapeProfileDecl{glade_id, profile}` | **zero existing vectors change**; a new message adds a new synth vector only (`synth.py:51-53` is one per message) | one new vector | the node can carry it as a second record kind, or not carry it at all | reads a second declaration, or a lookup table | a second directive line, or none |
| **(c) Defer past the freeze; profile stays mount config** | no edit | no edit | no edit | keeps throwing without out-of-band config | no edit; `crdt` stays unauthorable-in-practice even after row 8 lands |

**Count command**, run at glade-decl `d671f10` (`corpus/decl.v0.json` = `f669da0b…`): `python3 -c "import json;d=json.load(open('glade-decl/corpus/decl.v0.json'));print(sum(1 for v in d.values() if v['message'] in ('BindingDecl','AdvertisementRecord')),'of',len(d))"` → `11 of 26`.

**Recommendation: (b).** A declaration that cannot be mounted from its own
contents is not a declaration, so (c) is the wrong end state — but (a) buys that
property at the cost of the only compatibility claim the freeze has, and it is
the *most* byte-invasive option in this document, more so than R2(c) or R5(c).
(b) gets the same end state additively: the profile is a fact *about* a glade id,
which is exactly what a separate record keyed by glade id says, and it leaves
`BindingDecl` — the one message four repositories embed — untouched. Its cost is
one more thing for a consumer to look up, and a second directive on the app-file
line (R11 places it either way).

If the owner prefers (a) anyway, that is a defensible call — the field is where a
reader expects it — but then §4.2's compatibility claim must be *deleted*, not
weakened, and `build.py --compat` (§4.2) will fail by design on 11 vectors. Say
which explicitly; do not let the claim survive the choice.

**Whatever is chosen, the profile needs its rules before the freeze** (SUR-P2-4);
they are written in §4.4 bullet 4.

### R5 — Do `Authority.external` and `BindingDecl.source` survive v1?

**RULED (Gianni 2026-09-23): (b), with (a) when a bridged source appears.**
`Authority.external` and `BindingDecl.source` stay, recorded as "declared, not
yet authorable"; `dev-docs/glade/GladeDeclSurface.md:30`'s field list gains
`source`.

**The question.** `external(source)` is in the ratified DeclSurface §Contents, is
accepted by the node's parser, and has zero uses. The app-file grammar has no
token to name the source, so an `external` binding can never be complete.

**Options.** (a) Keep both and add an optional `source <name>` token to the
grammar (~15 LOC in `appdecl.rs`, one `STR` field on `sysdata.BindingDecl` — a
durable-record change, so R9 applies; and it wants R11's keyword form, not a
seventh position). (b) Keep both, declaration-only, and document that the node
cannot author `external` yet. (c) Drop `external` and `source` from v1 — removes
an enum member and **rewrites all 11 `BindingDecl`/`AdvertisementRecord`
vectors**, and tag 4 + the name `source` must be reserved (§4.1.8).

**Recommendation: (b), with (a) when a bridged source actually appears.**
DeclSurface §Contents is ratified text naming `external(source)`, so (c) needs
its own amendment to that document and buys nothing at freeze time. (a) is real
work for a feature with no caller. (b) costs one sentence and changes no bytes.
Record it explicitly as "declared, not yet authorable" so it is not mistaken for
working.

**Also fix the ratified text.** Both copies of DeclSurface list `BindingDecl` as
`(glade id, shape, authority, domain, zone, retention)` — six fields, omitting
`source`, which the contract carries at field 4 and `glade-decl/README.md:31-32`
lists as `source?`. R5's amendment sentence must correct
`dev-docs/glade/GladeDeclSurface.md:30` (CON residual; lands with A12 at §4.2
step 5).

### R6 — Must `canonical_key` and `derive_glade_id` be fixed before the publish?

**RULED (Gianni 2026-09-23): (a), with the hardening.** Both publish labelled
DEFERRED; `README.md:47` gains the v1 status line `:48-50` already carries;
`OpenNotes.md` N4 names the version they are deferred past; `canonical_key` gets
a named owner, whom this ruling does not name; ruled separately the same day:
Gianni (§3's note above).

**The question.** Both are documented signatures with no implementation in any
language and no golden vectors (`OpenNotes.md` N4 defers them). `GladeIdManifest`
is frozen as the *record* of a derivation nothing performs, and **four**
incompatible key conventions are live (row 49).

**Options.** (a) Publish with both explicitly labelled DEFERRED in `README.md`
and the DecisionLog; `GladeIdManifest` ships as an unexercised forward
declaration. (b) Implement `derive_glade_id` + its golden vectors before the
freeze (the hash is unchosen; all three languages must agree — realistically a
package of its own). (c) Publish and *remove* `GladeIdManifest` until the
function exists — additive to re-add, but it rewrites nothing else (messages
carry no ordinal on the wire), so the only cost is reserving the retired name.

**Recommendation: (a), with one hardening.** Blocking the first leaf of the
extraction on an unchosen hash would stall the whole TypeScript track
(`PackageExtractionPlan.md:150`). The hardening is narrower than revision 1
stated: `README.md:48-50` **already** says `derive_glade_id`'s "algorithm + its
golden derivation vectors are deferred to the implementation step (N4)". *(Premise
restated — CON-P3-7. Revision 2 said it is `canonical_key` at `README.md:47`
"that reads as part of the contract **with nothing qualifying it**". Something
does qualify it: `:42`, the heading of the subsection `:47` sits in, reads
"### Interfaces (signatures, documented — not encoded; see `dev-docs/OpenNotes.md`)",
and `:44-45` adds "taut models data, so these two live as documented signatures,
not schema constructs". What `:47` lacks is **not** a statement that it is
unencoded — it has that — but a statement of its v1 **status**: unlike `:48-50`
beside it, `:47` does not say the function is unimplemented and unoracled, so a
reader takes "documented signature" to mean "documented and implemented
somewhere". Revision 1's wider claim about `README.md:42-50` was already narrowed
once, and this is the second narrowing.)* Amend `:47` to say it is **not
implemented and not oracled in v1** — the status line `:48-50` already carries
and `:47` does not — add the same line to `OpenNotes.md` N4 with the version it
is deferred past, and give `canonical_key` a named owner: four live conventions
are already diverging (row 49), and freezing a contract that claims a canonical
key while four exist is exactly the kind of claim L1-07 tells this document not
to make.

### R7 — Which unexercised elements ship in the frozen v1?

**RULED (Gianni 2026-09-23): (b).** `AdvertisementRecord` is held out of v1, its
name reserved, until GDL-029 ratifies. Its two vectors leave the corpus and are
declared to the gate: `build.py --compat --deleted AdvertisementRecord,edge/advert`
(§4.7 row 2).

**The question.** `AdvertisementRecord` (3 fields) cites GDL-029, which the root
DecisionLog still lists **open**. Freezing a record format for an open decision
is the failure mode L1-09 exists to catch. *(Re-grounded — CON-P3-1: revision 1
cited L1-08, whose trigger is a contract that is already frozen.)*

**Scope note (CON-P3-4).** This ruling decides `AdvertisementRecord` **only**.
`DomainAnchor.deployment` follows R1; `RetentionPolicy.ttl` follows R2.

**Options.** (a) Ship it, with its status written into `README.md` and its
citation corrected from "(GDL-029)" to "(GDL-029, open)". (b) Hold
`AdvertisementRecord` out of v1 and add it when GDL-029 ratifies — genuinely
additive: a message carries no ordinal on the wire (`codec.py` encodes a message
as a bare CBOR map keyed by field tag) and `schema.messages` is a name-keyed
dict, so removing it changes no other message's bytes. It removes its own two
vectors (`AdvertisementRecord`, `edge/advert`) from the corpus, and the retired
*name* wants reserving. (c) Ratify GDL-029 first.

**Recommendation: (b).** A three-field record for an enumeration mechanism that
exists nowhere in either workzone is the cheapest thing in this document to
defer, and re-adding a message later is strictly additive. Either way the
citation must be corrected — a contract that cites an open decision as if
ratified is a precedence defect, not a typo.

### R8 — Does the contract cover the whole `.glade` file form, or only bindings?

**RULED (Gianni 2026-09-23): (b), amending the ratified text.** `glade-decl`
stays the tap/binding vocabulary; `ServiceDefinition` and the ACL seeds stay node
`sysdata`. `dev-docs/glade/GladeDeclSurface.md` gains the amendment sentence
saying so, and the repository copy becomes a banner-marked mirror (§4.7 row 21).

**The question.** GDL-037 (ratified) assigns three record kinds to this surface's
file form — `BindingDecl`s, `ServiceDefinition`s and ACL seeds
(`dev-docs/glade/GladeDeclSurface.md:113-114`). The contract has only the first.
The node has all three plus `WorkspaceEntry`.

**Which document is controlling (CON-P2-5).** `dev-docs/glade/GladeDeclSurface.md`
is controlling; `glade-decl/dev-docs/DeclSurface.md` is a stale copy of it (A12).
Whatever R8 returns, the amendment sentence goes in the **root** document, and
the repository copy becomes a banner-marked mirror with a drift check (§4.2
step 5). This is settled by §1 rule 1, not by R8.

**Options.** (a) Grow the contract with `ServiceDefinition` and an ACL-seed
record, rendered in all three languages — makes the ratified file form typed
end-to-end; additive in bytes (new messages), so it costs two new synth vectors
and no existing one. (b) Keep `glade-decl` as the tap/binding vocabulary only;
the other record kinds stay node `sysdata`; amend `GladeDeclSurface.md` to say
so.

**Recommendation: (b), amending the ratified text.** `glade-decl`'s stated
exclusion is "No runtime, no wire, no folds, no sessions, no persistence"
(`glade-decl/dev-docs/DeclSurface.md:36-38`, identical in the root copy); a
`CapabilityGrant` is an ACL fold input and a `ServiceDefinition` names a routed
provider — both need the node to mean anything, so both fail the module's own
admission test. But GDL-037 says otherwise, so this needs the owner's word and an
explicit amendment sentence in `GladeDeclSurface.md`, not a silent omission.
Related and separate: GDL-038's `glade-sys.glade` does not exist (A6).

### R9 — What does a changed or deleted binding line do to an already-registered declaration?

**RULED (Gianni 2026-09-23): (b2) + (a); (s) not taken.** App files keep their
spellings and `parse()` normalises `from-cursor` to `from_cursor` on the way into
`sysdata::BindingDecl`, so the store and the contract speak one vocabulary.
`dir.bindings` is folded by `glade_id`, newest wins, with a `BindingRetraction`
record scoped per `(app, glade_id)`; under b2 that fold is required, not
optional. Durable cost on the first boot, by the derivation below under R2(a):
**2 + 13 = 15** appends over the census's 28 lines (a single store sees its own
files' share — SAF-P3-13). Deleting a `service` or `workspace` line still
retracts nothing, and the page says so.

*(New — SAF-P1-2 merged with SUR-P2-5.)*

**The question.** `register` skips a record only when `(glade_id, payload bytes)`
already exist (`appdecl.rs:264`). Any token edit changes the bytes, so the
amended file appends a **second** `BindingDecl` for the same glade id. Nothing
folds `dir.bindings`, and deleting a line retracts nothing. On the first boot
after this amendment, what does an existing `~/.glade/sys/<name>/records.json`
contain, and which declaration is live?

**Why it is a ruling and not settled.** Because there is no rule to report. The
node has no binding query (`registry.rs:191-211`), its one production reader is
an `any()` (`exchange.rs:62-78`), and the published account of registration
(`GladeGrazelAttachNotes.md:56-61`) states the diff rule and stops. A user cannot
determine from any page whether their migrated file took effect.

**Options.**

*(Option (b) restated as two shapes with their true costs, and (a) given its
retraction scope — SAF-P2-11 and SUR-P2-7. The choice between b1 and b2 is
R9's; this document states both and recommends, it does not choose.)*

| | Node | glade-decl | App files | Clients | **Durable cost on the first boot** |
| --- | --- | --- | --- | --- | --- |
| **(a) Fold by `glade_id`, newest wins; retract explicitly** | a `bindings_of()` fold beside `grants_for` in `registry.rs`; `exchange.rs:62-78` folds instead of `any()`; a new `BindingRetraction` record kind in `sysdata.taut.py` and a `Record::Retract` arm; `register` diffs the parsed file against the fold **within the scope stated below** to know what to retract. Durable-record addition → `sysdata.rs` regenerates with `--legacy-codec`. ~200 LOC, scope rule included | none (the node's record, not the contract's) | none of its own; whatever R2's answer forces | a client subscribing to `dir.bindings` (an integration test drives exactly this today — `exchange.rs:651-658`, inside `mod tests` at `:266`) must fold the same way; today none does | Under **R2(a)/(c)/(d)**: **15** if the file side respells (13 `from-cursor` + 2 `windowed`), **2** if it does not. Under **R2(b)**: **13** and **0**, because (b) keeps `windowed` *(corrected — SAF-P2-12: revision 3 said the `windowed` lines "change under every R2 option, so they are the floor"; they are the floor under **every R2 option except (b)**, whose own App-files cell reads "0 meaning edits")*. Plus a retraction record for every line R2's answer removes. The fold then makes the superseded records unreachable, which is what (a) buys and neither (b) shape does |
| **(b1) The file keeps its spellings; the node maps them where a CONSUMER reads the record** | `parse()` stores `toks[5]` raw, as today; the mapping `from-cursor` → `from_cursor` happens in the fold/read path a consumer goes through, not on the way in (~10 LOC there, plus SUR-P3-2's diagnostics) | none | **0 edits for the 13 `from-cursor` lines**; 2 edits for `windowed`, which is a meaning change, not a spelling one | every reader maps, for the life of the store | **2** under **R2(a)/(c)/(d)** — the two `windowed` lines, which change for **R2's** reason and not R9's — and **0** under **R2(b)** *(corrected — SAF-P2-12: revision 3 read "**0.** No stored byte changes", which is true of the 13 and false of the file as a whole)*. The genuine benefit is unchanged and is real: **no stored byte changes for the 13**, and the store keeps one spelling of them. Cost instead: the **stored** vocabulary and the **contract** vocabulary differ permanently, and every consumer that reads a stored `BindingDecl` must know both |
| **(b2) The file keeps its spellings; `parse()` normalises on the way into `sysdata::BindingDecl`** | `parse()` normalises in the binding arm (~10 LOC, plus SUR-P3-2's diagnostics) — but `appdecl.rs:37` imports `BindingDecl` from `crate::sysdata`, so the value normalised at `:138` **is** the durable record's byte | none | as (b1) | none — the store already speaks the contract's spelling | **15** under **R2(a)/(c)/(d)** (13 normalised + 2 `windowed`) and **13** under **R2(b)** *(corrected — SAF-P2-12: revision 3 read "**13 appends**", counting only the records b2 itself rewrites)*. `register`'s diff is on `(glade_id, payload bytes)` (`appdecl.rs:264`); every stored record still carries `"from-cursor"`, so none of the 13 matches and all are appended (`:267`) and persisted (`glade-node.rs:91`) — and the `windowed` lines take that same path for R2's reason. **(b2) therefore needs (a)'s or (c)'s fold exactly as much as they do** |
| **(c) Duplicates accepted, with a stated fold rule** | `exchange.rs:62-78` must fold rather than `any()` — today a stale `exchange` record keeps a retired surface routable. The rule ("highest `(lamport, origin)` for a `glade_id` wins", matching the `value` fold at `glade-gyld/README.md:1147` *(re-pinned — CON-P3-8 / SUR-P3-8: `:934` at glade-gyld `024d2a8`, now `:1147` at `65da8cb`)*) goes in `GladeGrazelAttachNotes.md:56-61` and in the format page. ~40 LOC | none | none | as (a) | as (a) or (b2), whichever the spelling answer produces — (c) does not reduce the appends, it only makes them adjudicable |

**How the durable-cost column is computed (SAF-P2-12).** *(Stated as a
derivation rather than as hand-written integers — SAF-P2-12. Revision 3's column
counted only the records R9's own spelling mechanism rewrites, so (b1) read
**0** and (b2) read **13** while the two `windowed` lines, which R2 moves
through the same code path, went uncounted.)* On the first boot `register`
appends one record per binding line whose stored payload differs from the record
already there — `appdecl.rs:264` diffs `(glade_id, payload)`, `:267` appends,
`glade-node.rs:91` persists — and a binding line's payload differs exactly when
its retention byte does, because `toks[5]` becomes
`sysdata::BindingDecl.retention` at `appdecl.rs:138` (`:37` imports that type
from `crate::sysdata`), `sysdata.rs:135` encodes it at tag 6 and `registry.rs:97`
dispatches `Record::Binding` to that encoder. So **appends on the first boot =
(lines whose token 5 changes in the FILE, which R2's answer fixes) + (lines whose
STORED token the node rewrites though the file did not change, which R9's shape
fixes)**. The input is §4.4 bullet 6's census, run at grazel `c66f029`, glade
`559cb2c`, glade-gyld `65da8cb`, glade-gwz `e53c87d`: **28** binding lines —
**13** `from-cursor`, **13** `latest`, **2** `windowed`. *First term*: the 2
`windowed` lines change under R2(a), (c) and (d), whose App-files cells all read
"as (a)", and not under R2(b), whose cell reads "0 meaning edits"; the 13
`from-cursor` lines change only where the **file** respells, which is R9(a) or
(c) taken with a file-side respelling and neither (b) shape. *Second term*: only
**b2** rewrites a stored token whose line did not change, because it normalises
on the way into the durable record, and it rewrites exactly those 13; b1 maps in
the read path and rewrites none, and (a)/(c) store what the file says. The 13
`latest` lines are in neither term under any option — `latest` stays writable
under every R2 answer, including (d), where nothing is validated at all — which
is why no cell exceeds **15**. The unit is a binding line of that
census, i.e. the tree-wide cost of the migration: `grazel-app.glade`'s two homes
are byte-identical and the two fixtures are test files, so any one store sees
only its own loaded files' share of the number. **Worked, for the recommended
pair {R2(a), R9(b1)}**: R2(a) drops `windowed`, so its 2 lines are re-typed →
first term **2**; b1 stores `toks[5]` raw, so no unchanged line's stored byte
moves → second term **0**; **2 + 0 = 2** appends, because neither re-typed
payload matches the record registered under `windowed`. The same walk on
{R2(a), R9(b2)} is that same 2 plus b2's 13 = **15**.

**(a)'s retraction scope, stated in the option (SUR-P2-7).** Retraction is
computed **per `(app, glade_id)`**, against the declarations previously
registered for the `app` named in the file being registered. Therefore: a file
that is **not loaded** on a given boot **retracts nothing**, and a declaration
made by a **different** app file is **never** in scope. The scoping key already
exists — `sysdata.BindingDecl` carries `app` as field 1 (`sysdata.taut.py:79`),
set from the `app` line at `appdecl.rs:133`, which is row 33's mapping.

Without that sentence the option is unsafe as written, and the counterexample is
grazel's documented default boot. `glade-node` accepts `--app` repeatedly and
grazel passes two files when its gyld leg is on (`grazel/README.md:73-79`):
`grazel-app.glade` registers its **7** bindings, then `gyld-app.glade` is parsed
and diffed against the fold — under an unscoped diff, grazel's 7 are in the fold
and absent from this file, so `ws.tree`, `ws.files`, `ws.diff`, `term.log`,
`gwz.output`, `chat.msgs` and `chat.groups` are retracted on a boot where nobody
edited anything. The same hole reaches the flag: the gyld leg is **default off**
(`grazel/README.md:41-42`), and "everything this registrant declared" as a scope
would retract gyld's 7 surfaces whenever the supplier is switched off — against
the files' own stated purpose, "Pre-declared here so they exist node-side
whether or not a supplier is running" (`grazel/apps/gyld-app.glade:19-20`) and
the same of `chat.msgs`/`chat.groups` at `grazel/README.md:15-17`. Declare-ahead,
provider-optional is the documented design; absence-implies-retraction is its
negation. §4.7 row 10 tests all three cases.

**R9's options govern `dir.bindings` only (SUR-P3-12).** Every option, cost and
test above is about the `binding` directive, and no answer here changes what any
other directive does. The format has four directives that add durable state, and
after this amendment their remove halves stand as: `binding` — answered by (a),
scoped, with §4.7 row 10's three tests; `seed` — answered already and published,
since `grants_for` applies revocation-wins at `(principal, share)` regardless of
order, so a runtime revocation beats a re-registered seed
(`GladeGrazelAttachNotes.md:85-88`); `service` and `workspace` — **not answered,
and not answered by this amendment either**: deleting either line retracts
nothing, which is A9's finding about every directive arm of `parse()`. This is a
pre-existing property of the format and not something the amendment introduces;
it becomes worth stating now only because `binding` is about to acquire the
first remove half in the grammar, which makes the asymmetry visible.
*If the owner wants the symmetry instead, that is a further option under R9 —
call it (s) — and it is priced here rather than assumed*: a `ServiceRetraction`
and a `WorkspaceRetraction` record kind beside (a)'s `BindingRetraction`, each
with the same per-`(app, …)` scope rule, a fold for `exchange.rs:62-78`'s
`declared_exchange` and one for the workspace entries whose shares
`appdecl.rs:176` already refuses to duplicate, and `sysdata.rs` regenerated with
`--legacy-codec` for two more durable record kinds — roughly (a)'s ~200 LOC
again, and **two more record kinds frozen at the same publish**, which is the
part that is expensive to defer and cheap to decide now. It is offered, not
recommended: the recommendation above is unchanged.

**Which spellings a file may contain, under each shape (SUR-P3-9).** Under
**(b1)** the stored token is whatever the file wrote, so the file side must be
told which spellings are legal — and the answer that keeps one spelling in the
store's history is: the file writes `from-cursor`, and `from_cursor` is
recognised-but-refused with a diagnostic naming the hyphen. Under **(b2)** both
spellings parse and normalise to `from_cursor`, so a file containing either is
accepted and stores one value; whether the hyphen is nevertheless the only
*legal* spelling is R2's 18b sub-choice, not R9's *(18b's options do not carry
it; ruled separately on 2026-09-23 — the hyphen only, §3)*. Under **(a)** and **(c)** with
a file-side respelling, `from_cursor` is the file's spelling and `from-cursor`
becomes the recognised-but-refused one. Whichever shape is taken, §4.4 bullet 7's
refusal message must name a spelling the file format actually accepts.

**Recommendation: (b) for the spellings, plus (a) for what remains — and, within
(b), b1 if the durable cost is what matters and b2 if a single stored vocabulary
is.** (b) in either shape is not an alternative to (a): it is a reduction of (a)'s
blast radius on the **file** side, and that reduction is real and large — it cuts
the mandated rewrite from **15 file edits to 2** under R2(a)/(c)/(d) (from 13 to
0 under R2(b)), the two being `term.log`'s `windowed` line in `grazel-app.glade`'s
two byte-identical homes, and keeps the format's hyphen convention
(SUR-P3-3: there is not one underscore in any authored token of any of the five
files, so `from_cursor` would be a convention of one). *(Corrected — SAF-P2-11.
Revision 2 claimed (b) also "removes the 'did my migration take effect?' question
for 13 of the 15". That is true of **b1** and false of **b2**: b2 normalises
inside `parse()`, which builds `sysdata::BindingDecl`, so those 13 records are
appended and the question is live for exactly them. The benefit (b) genuinely
buys is 13 fewer FILE edits and the hyphen, not 13 fewer durable records.)*
*(Re-read against the corrected column — SAF-P2-12. **The sub-recommendation
stands, and so does its criterion**: under the recommended R2(a) the spread is
**2 against 15**, not 0 against 13, so b1 is still the cheaper shape by the same
**13** appends, and the two it does cost are R2's `windowed` edits, which every
shape and every option pays. What the correction removes is the gloss that b1
costs nothing durable — "if the durable cost is what matters" is a statement
about the 13, not about a floor of zero, and under R2(b) the spread is 0 against
13.)*

(a) is then the only option that answers **deletion**, which (c) leaves
permanently unanswered, and the only one under which a later retention-honouring
store (GC-4) can ask "what is `term.log`'s retention?" and get one answer. Under
**b2 the fold in (a) is not optional but required**, because b2 appends 13
records that nothing else adjudicates. If the owner takes (c), the fold rule must
land in the **same commit** as the first token edit, not after — a duplicate
written before the rule exists is a record nothing can adjudicate.

**Whatever is chosen, it must be written in a page a user reads before any file
is migrated**, because the migration is the first mass exercise of it
(SUR-P2-5). §4.4 bullet 5 names the pages.

### R10 — Does the app-file version header advance to `glade-app v1`?

**RULED (Gianni 2026-09-23): (a).** `glade-app v1` names the validated grammar;
`v0` files still load, with a warning. A node accepting both headers lands first
(§4.4 step 0) and the headers move last (step 4), both twins of
`grazel-app.glade` in one commit.

*(New — SUR-P2-2.)*

**The question.** Validation shrinks the set of accepted programs while the
header stays `glade-app v0` (`appdecl.rs:90`). `v0` then names both the language
that accepts `from-cursor`/`windowed` and the language that refuses them, so no
diagnostic can say which language a file is written in, and no third party gets
notice before the node fails to start.

**Options.**

| | Node | App files | Third-party files | Docs | **Ordering cost** |
| --- | --- | --- | --- | --- | --- |
| **(a) `v1` is the validated grammar; `v0` still loads, warned** | `appdecl.rs:89-95` gains a version variable and the binding arm branches on it (~25 LOC) — **plus a warning channel `parse()` does not have**: it returns `Result<AppDecl, String>`, so a non-fatal message needs either a `Vec<String>` on `AppDecl` or an `eprintln!` in `load`, which touches `glade-node.rs:88` and grazel's integration path | **the token edits and the header bump are NOT the same commit** *(corrected — SAF-P2-9; revision 2 said "each file's header moves to `v1` in the same commit as its token edits")*. The tokens move in §4.4's step 1; each file's header moves **after** a both-headers node is deployed, in §4.4's step 4 | keep working, with a message naming the replacement and the version it changed in | a migration note, which these repos do not have yet — **and** the header token in the one published grammar block, `GladeGrazelAttachNotes.md:30`, which still reads `glade-app v0`. Without that edit the format's own specification instructs every new author to write the deprecated header, i.e. to generate a deprecation warning (SUR-P2-8). No in-file comment shows the header in any of the five files — the header is only ever the data line itself (`grazel-app.glade:16`, `gyld-app.glade:14`) — so `:30` is the **only** place an author can read it | **one extra landing step.** A node that accepts both `glade-app v0` and `glade-app v1` must be deployed **before any file's header moves**, because the header is an exact-equality check at `appdecl.rs:89-95` and an old node **refuses** a new header. §4.4 step 0 |
| **(b) Redefine `v0` in place, stated explicitly** | 0 beyond the validation itself | 0 beyond the token edits | **break at boot**, with no version to key a message on | the draft must say so in terms, plus the same migration note; `GladeGrazelAttachNotes.md:30` keeps `v0` and gains the sentence saying it now means the shrunken language | **none.** No header moves, so no node ever meets a header it cannot parse. This is (b)'s one genuine advantage over (a) |

**Recommendation: (a).** The mechanism is already in the file and already parsed;
using it costs about 25 lines in one function and converts an unannounced
breaking change into an announced one. Not using it burns the mechanism
permanently: after the freeze `v0` means both languages forever, and the next
shrinkage has the same problem with no way out. The real costs to weigh are two:
the warning channel, a small API change to `parse()`/`load()` that three call
sites see; and the extra landing step in the column above, which is not optional
— §4.4 step 0 exists because of it. Note the interaction with R9(b): if the file
keeps `from-cursor`, the only shrinkage is `windowed`, and (a)'s deprecation path
has one token to carry rather than two — but the header hazard is unchanged,
because it is the header and not the token that an old node refuses.

### R11 — Does the binding line get a keyword tail?

**RULED (Gianni 2026-09-23): (a).** The binding line gains a `key=value` tail
after its five positional tokens — `ttl=<duration>` and
`shape-profile=<profile>` — and an unknown key is refused by name with a line
number. It follows from R2(a) keeping `ttl`. The published grammar and the three
in-file grammar comments show the tail (§4.7 row 19).

*(New — SUR-P2-3 + SUR-P2-4; subordinate to R2 and R4.)*

**The question.** The line is five positional tokens. Two things want position 6:
`ttl`'s duration and the shape profile. Whichever takes it, the other can only go
at position 7, which would make position 6 mandatory — which an "optional
trailing token" forbids. Does the grammar gain a keyword tail instead?

**Options.**

*(A **Docs** column added — SUR-P2-8. Revision 2's table had none, and both
R11(a) and R10(a) change the line an author copies from without costing a line
for the two places that show it.)*

| | Node | glade-decl | App files | **Docs** | What it leaves open |
| --- | --- | --- | --- | --- | --- |
| **(a) A keyword tail after the five positional tokens** — `binding <id> <shape> <authority> <zone> <retention> [k=v …]`, e.g. `ttl=10m`, `shape-profile=text_crdt`; an unknown key is refused by name with a line number | `appdecl.rs:111-140` splits the tail on `=` (~35 LOC); the arity check at `:111-114` becomes a minimum and `:113`'s template gains the tail; each key that must persist adds a `sysdata.BindingDecl` field → durable-record change, `--legacy-codec`, and R9 applies | none beyond what R2/R4 already decide | **0 edits** — no existing line has a tail | **the published grammar at `GladeGrazelAttachNotes.md:32`** and **the three in-file grammar comments** that carry the same template — `grazel/apps/grazel-app.glade:19`, its byte-identical twin `glade/apps/grazel-app.glade:19`, and `grazel/apps/gyld-app.glade:17` (the two fixtures carry no grammar comment at all, so the in-file grammar is a 3-of-5 property). **Plus one sentence on how an author learns the tail exists**, because (a) makes `:113`'s template unreachable for them: once the arity check is a minimum, a legal five-token line never trips arity, so the corrected template never prints for the author who wrote a valid line and now wants a TTL, and `:121`-style refusals fire only on a wrong key, which presupposes knowing keys exist | everything: a later option is a new key, not a new position, and it changes the meaning of no existing line |
| **(b) Hold `ttl` reserved and unauthorable until it has a slot**; `profile` takes position 6 | one entry in the refusal table; `:113`'s template gains the optional token | `RetentionPolicy.ttl` becomes a **fourth** declaration-only gap beside `external`, `source` and `Retention`'s parameters | 0 | the same four artefacts, carrying a sixth **positional** token instead of a tail: `GladeGrazelAttachNotes.md:32` and the three in-file comments. Here `:113` does still reach the author, because a five-token `crdt` line is an arity refusal | **nothing** — position 6 is spent, so a duration can only ever go at position 7, which makes the profile mandatory whenever a duration is present. One-way door |
| **(c) Neither: the line stays exactly five tokens** | 0 | 0 | 0 | 0 — the published grammar and the three in-file comments already show exactly this line | position 6, still free — but only reachable if R2 drops `ttl` **and** R4 returns (b) or (c) |

**Recommendation: (a) if either R2 keeps `ttl` or R4 returns (a); (c) otherwise.**
(a) is the single highest-leverage pre-freeze change available: it costs one
paragraph here and about 35 lines in one function, and it is the only option that
leaves the format extensible after the freeze. (b) is cheap today and
unrecoverable tomorrow. (c) is free but only legal under a specific pair of
answers to R2 and R4, which is why R11 cannot be decided before them.

**Naming (SUR-P2-4).** If the tail carries the profile, do not spell the key
`profile`: `grazel/README.md:102-106` documents the node's `--profile local|peer`
boot profile and `glade-gyld/README.md:499` documents an LLM "compatibility
profile" *(re-pinned — CON-P3-8 / SUR-P3-8: `:315` at glade-gyld `024d2a8`, where
the section heading then was; the README grew 981 → 1192 lines and the heading is
`:499` at `65da8cb`)*, so the word already means two other things in the same
product. Spell the file key `shape-profile=` (hyphen, per R9(b)/SUR-P3-3). The
contract's field name stays `profile`, because GDL-041 calls them profiles.

---

## 4. The amendment, as it would be made once the rulings are in

One schema edit, one corpus version, three regenerations, the node, four
consumers — in this order. Everything below assumes the recommended answers;
substitute freely, subject to §4.1.8.

### 4.0 Where every SETTLED item lands, and what rides separately (CON-P3-5)

| Item | §4 step |
| --- | --- |
| Row 4 (`stream` stays, comment) | §4.1.1 |
| Row 8 (`crdt` declarable) | §4.4 bullet 1 |
| Row 9 (`atom=8`) | §4.1.1, with its corpus vector |
| Row 18a (contract spells `from_cursor`) | already true in the contract; the *file* side is §4.4 bullet 6, under R9 |
| Row 33 (`app ≡ package` mapping) | §4.4 bullet 9 — node-side, so R7(b) cannot void it |
| Row 43 (`base_seq` comment) | §4.1.4 |
| A1 (`CONTRACT_VERSION`) | §4.3 |
| A2 (the drift gate) | **already closed** at `glade-decl d671f10` / `glade-decl-rs 21eefa1` — not a step of this amendment; §4.3 and §4.7 row 1 record what it now guarantees |
| A3 (`glade/decl/*` sketch) | §4.4 bullet 10 — and §4.4 bullet 14 applies the same treatment to the eleven `dev-docs/examples/*.glade` files of the other dialect (SUR-P3-6) |
| A4 (one rendering procedure) | §4.3 |
| A5 (lockfile) | §4.8 |
| A7 (`session.rs` error text) | §4.4 bullet 11 |
| A12 (DeclSurface drift) | §4.2 step 5 |

**Rides separately, and why.** **A6** (`glade-sys.glade` does not exist despite
ratified GDL-038) belongs to GDL-038's own amendment: this document changes no
GDL-038 text and creating the file would be a design decision about base glade's
own app surface, not a reconciliation of `glade-decl`. **A8**
(`gryth-ui/packages/glade/src/glade.ir.json`) is the wire IR and is owned by
`PackageExtractionPlan.md` step 2.6 and `GlialFitAssessment` S1 (`:149`);
touching it here would smear two unrelated contracts into one commit (§4.6).

**Publish ordering.** *(Safety residual; restated over §4.7's split —
CON-P2-6.)* Nothing is published — to npm, crates.io or PyPI — until

- **every row of §4.7(a) is green on a settled tree**, and
- **every row of §4.7(b) is green under the answers the owner actually
  recorded** — each (b) row names its ruling and says what green means under
  each of that ruling's options, so "which rows count" is read off the recorded
  answers rather than decided at the publish boundary. A (b) row whose condition
  does not arise under those answers is not a gate and nothing waits on it.

**§4.7(c) is evidence, not a gate.** Rows 14 and 16 are recorded pre-freeze
items — a glial test that does not exist yet, and a manual demo observation —
and no publish waits on either.

*(Revision 2 read "every row of §4.7 is green on a settled tree" over a table in
which row 2 fails by design under R4(a), row 7 presupposes a validation R1's and
R2's sub-choice (iii) decline to turn on, row 14 says in its own cell that it is
not a gate, and row 16 has no green/red state. Under R4(a) the precondition was
unsatisfiable for ever; under row 14 it was self-contradictory on its face.
Corrected — CON-P2-6.)*

**Under the answers recorded in §3 (2026-09-23).** The condition of every
§4.7(b) row arises, so all nine join (a) and the publish waits on each; nothing
in §4 is left to branch on.

| Where | Resolves to |
| --- | --- |
| §4.7 row 2 | the "deletions only" case: green only as `build.py --compat --deleted AdvertisementRecord,edge/advert` (R7(b)); R1(a), R2(a), R3(a), R4(b), R5(b) and R6(a) move no byte |
| §4.7 row 7 | both tokens, as warnings for one release (31b (ii), 18b (ii)): an unknown zone; `windowed`, naming `from-cursor`; and `from_cursor` written in a file, also naming `from-cursor` (hyphen only, §3). No diagnostic on `from-cursor` itself, because under R9(b2) it is the file's spelling |
| §4.7 row 9 | `appended` = **15** under {R2(a), R9(b2)}, in the census's tree-wide units (SAF-P3-13 rides the amendment) |
| §4.7 row 10 | R9(a)'s three scoped-retraction tests |
| §4.7 row 13 | R1(a): the demo resolves with no edit |
| §4.7 row 17 | R10(a): the pre-amendment `parse()` refuses a `v1` header |
| §4.7 row 18 | R2(a), in the file's spelling (hyphen only, §3): `latest`, `from-cursor` and `ttl` glossed; the page may add that the contract spells the second `from_cursor` |
| §4.7 row 19 | R10(a) and R11(a): `v1` at `GladeGrazelAttachNotes.md:30`; the tail at `:32` and in the three in-file comments |
| §4.7 row 21 | R8(b): the amendment sentence in the root `GladeDeclSurface.md`, and the mirror banner |
| §4.1 | item 2 adds `ShapeProfileDecl`; item 5 removes `AdvertisementRecord`, its name reserved (item 8) |
| §4.4 | steps 0–4 all land (R10(a)); bullet 2 lands branch (ii) for both tokens; bullet 5 owes the fold and retraction rules for 15 surfaces; bullet 6 is 0 edits for the 13 `from-cursor` lines and 2 for `windowed`; bullet 7's message names `from-cursor`, and a second message covers `from_cursor` written in a file; the token-5 documentation block glosses the file's `from-cursor` |
| §4.5 | glial looks the profile up (R4(b)) rather than reading a field |
| Does not arise | "Under R4(a), row 2 is replaced" below; R9(c)'s same-commit fold rule; R10(b)'s redefinition of `v0`; R9's option (s), so the row CON-P3-11 found missing would gate nothing under these answers (the finding stays open until the Consistency reviewer closes it) |

**Under R4(a), row 2 is replaced, not left red.** The substitute is the
compatibility artefact §4.2 already names in the taut-shape idiom
(`taut-shape/release/compatibility.v1.json`): it must exist and enumerate
exactly which of the 26 vectors changed, and the superset claim is deleted
rather than weakened. The same substitution applies to every other byte-moving
option (R1(c)/(d), R2(c)/(d), R3(b), R5(c)) — see §4.7 row 2.

`PackageExtractionPlan.md:155`'s step-1.1 exit check covers only `pnpm test` in
`glade-decl-ts` plus a `-ts`/`-rs` hash comparison, and step 1.2 publishes; that
is not sufficient here, because the node half of this reconciliation (§4.4) has
its own gates and they are in a different repository.

### 4.1 Schema edits — `glade-decl/ir/glade_decl.taut.py`

1. `Enum("Shape", …)` — add `atom=8`. Keep `message=2` and `window=5` with their
   numbers (R3a). Rewrite the comment above it to state recognition ≠ support,
   to name GDL-041 as the catalogue's owner, **and to say the same about `atom`
   itself**: `atom` enters `KNOWN_SHAPES` and not `BINDING_SHAPES` (§4.4), and
   glial's `requireMountShapeAdapter` rejects it (`shapes.ts:83`) while
   `requireShapeAdapter` accepts it (`:72`) — so a consumer can legally emit
   `BindingDecl{shape: atom}` that no node accepts and no client mounts. **Also
   add an `edge/binding-*-atom` curated vector to `corpus/build.py`'s
   `curated_values()`** (`:84-150` is hand-authored; `synth_values` is one per
   *message*, never per enum member), and keep `edge/binding-message-private` and
   `edge/binding-deployment-window` as the relabelled recognition cases R3(a)'s
   table promises — **keeping their keys**, so the relabelling is a comment
   change and not a rename: a renamed key is a deletion plus an addition under
   §4.2's `--compat` gate and would have to be declared. Without the `atom`
   vector, `README.md:55-58`'s "every shape" coverage claim is false at the
   moment of freeze (SAF-P3-8). *(rows 3, 4, 6, 9)*
2. `Msg("BindingDecl", …)` — **under R4(a)** add `F("profile", 8, STR,
   optional=True)`, and delete every superset claim (§4.2). **Under R4(b)** add a
   new `Msg("ShapeProfileDecl", F("glade_id", 1, Ref("GladeId")), F("profile", 2,
   STR))` and leave `BindingDecl` untouched. Under R4(c), no schema edit.
   *(row 34)*
3. `RetentionPolicy` — unchanged under R2(a); rewrite the comment to say the
   policy is declarative and unenforced pending GC-4, and that `ttl`'s duration
   is expressible in the record (`Retention.ttl_ms`) but not, today, in an app
   file (R11). *(rows 17–20, 24, 25)*
4. `ChangeEvent.base_seq` comment — "the position this event is anchored at
   (the baseline a delta applies against; the current position on a refresh)".
   Comment-only; no byte change. *(row 43)*
5. `Msg("AdvertisementRecord", …)` — removed under R7(b) with its name reserved
   (see 8), or its comment corrected to "(GDL-029, open)". *(rows 35–37)*
6. Module docstring + `dev-docs/OpenNotes.md` N3/N4 — record the deferrals
   (R6a) and the new notes N7 (profile), N8 (domain is declaration-only) and
   N9 (`private` — see 7).
7. **`ZoneKind`'s comment, `README.md:39-41`, and new OpenNote N9** (SAF-P2-4).
   The schema comment today asserts a mechanism as fact: "`private` (keyed to a
   self, `self:<id>` at runtime)". Scope it: *`private` is honoured by the
   grip-share binder (`glade/grip-share/src/manifest.ts:64-67` with the manifest's
   `zones` table); glial mounts do **not** produce the key today — `Fill.zone`
   feeds a local instance key only (`glial/src/instance.ts:36,40-41`) — so a
   surface declared `private` and mounted through glial converges in the commons
   partition. Do not rely on `private` for confidentiality until the glial
   mapping (GlialFitAssessment S6) lands.* The same sentence goes beside the
   published enum in `README.md`. *(row 16)*
8. **Any delete option also reserves** (SAF-P3-6). Taut has the facility and
   `glade_decl.taut.py` declares none of it: `Msg(..., reserved=(), next_id=None)`
   (`taut/src/taut/ir/dsl.py:151,176-179`), enforced only when declared
   (`taut/src/taut/ir/validate.py:45-58,70-75`), and `taut/docs/Reference.md:95-100`
   is explicit — "When you remove a field, reserve its tag and name so they can
   never be reused (reuse with a different type silently corrupts the wire)".
   So: R1(d) → `reserved=[5, "domain"]` on `BindingDecl`; R5(c) → `reserved=[4,
   "source"]`; R2(c) → `reserved=[2, "ttl_ms"]` on `Retention`. Set `next_id`
   above every used and reserved tag on any message that is edited. For a
   retired **enum** member (R3(b)) taut has no `reserved`, so record the retired
   number in the enum comment — which is what R3(b)'s "hold the numbering"
   actually requires.

   **And for a retired message name (R7(b)) taut has no facility either**
   *(corrected — SAF-P3-12; revision 2 said "R7(b) → reserve the retired message
   **name**", which cannot be executed as written)*. `reserved` is a per-message
   field list: `dsl.py:176-179` splits it into `reserved_tags` and
   `reserved_names`, `model.py:69-71` holds both on `MessageDef` as "retired
   tags" and "retired **field** names", and `validate.py:55-58` enforces them
   against that message's own `f.tag` and `f.name`. `model.py:136-140` shows
   `Schema` carrying `enums`, `messages`, `services` and `extensions` and **no
   reservation list of any kind**, so there is nowhere to record that a *message*
   name is retired and nothing that would reject a later schema re-declaring it.
   Treat it exactly as the enum case beside it: record the retired message name
   in `glade_decl.taut.py`'s **module docstring** and in
   `glade-decl/dev-docs/OpenNotes.md`, naming the version it was retired in and
   forbidding reuse with a different shape. That is the carrier — not
   `Msg(reserved=…)`. It matters because the node's `--legacy-codec` decoder is
   fail-open (`sysdata.rs:138-147`), so a v2 that re-added `AdvertisementRecord`
   with different fields would decode v1 bytes into the wrong shape **quietly**,
   which is the hazard SAF-P3-6 named. A build-time check would need a taut
   feature that does not exist and must not be invented here.

Field numbers 1–7 of `BindingDecl` and every existing enum number stay exactly
where they are under every recommended option. *(Revision 1 added "which is what
keeps §4.2 cheap" — struck, SAF-P1-1: holding the numbers keeps the *enum* values
stable, it does not keep a message's map size stable when a field is added.)*

### 4.2 The corpus — `decl.v1.json` **replaces** `decl.v0.json`

**What the repository's own rule is.** `glade-decl/README.md:52-58` names
`corpus/decl.v0.json` "the FROZEN oracle" and describes exactly one of them.
`corpus/build.py:43` hard-codes a single `GOLDEN` path; `--check` compares
exactly `[IR_JSON, GOLDEN, RS_VECTORS]` (`:219-231`). Each rendering holds one
byte-identical copy at one fixed name and its gate reads that name:
`glade-decl-ts/src/decl.v0.json` (`src/corpus.test.ts:18`),
`glade-decl-py/src/glade_decl/decl.v0.json` (`tests/test_corpus.py:27`), and
`glade-decl-rs/src/vectors.rs` (generated table). The sibling precedent agrees:
`taut-shape/corpus/` has **ten** corpora (`ls taut-shape/corpus/*.json | wc -l`
→ 10, at taut-shape `9a75209`) *(CON-P3-3: revision 1 said nine)* and never two
versions of the same one — the suffix is *that contract's* version, one live file
each.

**Conclusion: the README implies replacement, not coexistence.** Add
`corpus/decl.v1.json` and delete `corpus/decl.v0.json` in the same commit. v0
stays recoverable at commit `bbce73d`, which is its historical oracle. Two live
corpora would require `build.py` to hold two schema versions (it loads exactly
one, `:41, :59-63`) and would leave the renderings' gates ambiguous about which
file is authoritative — the opposite of what the oracle is for.

**Every reference that must move (CON-P2-3).** Revision 1 named four; there are
**20**, plus the two data files themselves. Enumerating command, run at
glade-decl `d671f10` · glade-decl-ts `7e16e32` · glade-decl-rs `21eefa1` ·
glade-decl-py `1b0f6d1` and glade-wz root `61bb479`:

```sh
for r in glade-decl glade-decl-ts glade-decl-rs glade-decl-py; do \
  git -C $r grep -n 'decl\.v0' HEAD -- . ; done; \
git grep -n 'decl\.v0' -- dev-docs ':!dev-docs/glade/GladeDeclReconciliation*'
```

*Must change or a gate cannot pass:*

1. `glade-decl-py/pyproject.toml:20` — `"src/glade_decl/decl.v0.json" = "glade_decl/decl.v0.json"`. `tests/test_corpus.py:23` reads the oracle through `importlib.resources.files("glade_decl")`; rename without this line and the corpus is not in the distribution, so §4.7's `pytest` row cannot pass from a wheel.
2. `glade-decl/corpus/build.py:43` — `GOLDEN`.
3. `glade-decl-ts/src/corpus.test.ts:18` and `glade-decl-py/tests/test_corpus.py:27` — the two gate reads.
4. The two data files: `glade-decl-ts/src/decl.v0.json`, `glade-decl-py/src/glade_decl/decl.v0.json` (renamed by the copy step, §4.3).

*Must change for the text to be true:*

5. `glade-decl/README.md:54,61`; `glade-decl/corpus/build.py:4,22`; `glade-decl/ir/glade_decl.taut.py:28`; `glade-decl/dev-docs/DeclSurface.md:58`; `glade-decl/dev-docs/OpenNotes.md:45`.
6. `glade-decl-ts/README.md:16,33`; `glade-decl-py/README.md:15,36` — including the two `cp … corpus/decl.v0.json …` commands, i.e. the copy step itself.
7. `glade-decl-ts/src/corpus.test.ts:3`; `glade-decl-py/src/glade_decl/__init__.py:11`; `glade-decl-py/tests/test_corpus.py:3`.
8. Root `dev-docs/glade/GladeDeclSurface.md:59` and `dev-docs/GladeHandoff-260710.md:163`.

**Step 5 — `glade-decl/dev-docs/DeclSurface.md` (A12, CON-P2-5).** The root
`dev-docs/glade/GladeDeclSurface.md` is controlling. Either make the copies diff
empty, or give the repository copy a banner — "mirror of
`dev-docs/glade/GladeDeclSurface.md` @ `<sha>`; do not edit here" — and add a
drift check to `corpus/build.py --check`. At minimum `:27` (pre-GDL-041 Shape
gloss), `:30` (no mount-instance clause; and `source` missing from the field
list — R5), `:32` (the GDL-040 Supplier row is absent) and `:58` (the corpus
filename) move. R8's answer adds the record-kinds sentence to the **root** copy.

**Step 6 — the front page's own gaps (SUR-P3-1, SUR-P3-5).** `glade-decl/README.md`
names six enums (`:24`) and not one member of any of them. Grow it a members
table per enum with a one-line gloss each — including which values an app file
may actually write (`BINDING_SHAPES` is three of eight today) — including the
`Retention` members, which no published page names at all (SUR-P2-6; §4.4's
token-5 block). And state at `:28-30` whether an app-file glade id is **authored
or derived**: the front page says a `GladeId` is "**frozen once shared (GQ-6),
derived from package id + grip key**" (`:28-29`, verbatim) *(quote corrected —
CON-P3-7: revision 2 elided it to "derived from package id + grip key … frozen
once shared", which implies a forward order the source does not have)* while 14
ids were typed by hand into the five app files and `derive_glade_id` is deferred
(`:48-50`). If authored, give the syntax rule (charset, the dot's meaning,
length) and say what `derive_glade_id` is then for.

**Compatibility.** *(Rewritten — SAF-P1-1; the condition list and the gate
corrected for vectors v1 **deletes** — SAF-P2-10.)* A v0 vector can leave v1 in
two different ways, and revision 2's model accounted for only one of them.
Enumerate both, over the 26 vectors of `corpus/decl.v0.json` (`f669da0b…`) at
glade-decl `d671f10`:

- **Options that move a surviving vector's bytes.** `R4(a)` (11 of 26 — the 9
  `BindingDecl` and the 2 `AdvertisementRecord`), `R1(c)` and `R1(d)` (the same
  11), `R5(c)` (the same 11), `R2(c)` (the 3 surviving `Retention` vectors shrink
  `a2 …` → `a1 …`, **and** all 11, each embedding a `Retention` at tag 7),
  `R2(d)` (all 11, tag 7 from a nested map to a text string), `R3(b)` (the synth
  `BindingDecl` and `ChangeEvent` vectors, because synth selects by member
  *index* — `2 % 6` now picks `stream`, so tag 2 goes `02` → `03`).
- **Options that delete a v0 vector outright, moving no surviving byte.**
  `R7(b)` removes `AdvertisementRecord` and `edge/advert`; `R6(c)` removes
  `GladeIdManifest` and `edge/manifest`; `R3(b)` additionally removes
  `edge/binding-message-private` and `edge/binding-deployment-window`; `R2(c)`
  additionally removes `edge/retention-ttl`; `R2(d)` additionally removes all
  four `Retention` vectors. All **ten** key names this bullet designates —
  the seven spelled out, plus the other three of the four `Retention` vectors
  (`Retention`, `edge/retention-latest`, `edge/retention-cursor`; the fourth,
  `edge/retention-ttl`, is already among the seven) — verified present in v0 at
  this tuple. *(Corrected — CON-P3-10: revision 3 said "nine", and neither
  reading of the bullet is nine. The count is of **designated corpus keys**,
  expanded through the group name "all four `Retention` vectors", with
  `Retention` counted once as a key and not again as the message name it shares
  with it; the count of backticked **names** actually written out in the bullet
  is eight — the seven keys plus `Retention` — which is the other reading, and
  it is not nine either.)*

So:

- Under **R4(b)** or **R4(c)**, with **R1(a)/(b), R2(a)/(b), R3(a), R5(a)/(b),
  R6(a)/(b) and R7(a)**: no enum number moves, no message gains or loses a
  field, no vector is deleted, and every v0 vector re-encodes identically. v1
  **is** a byte superset of v0.
- Under that same set **but with R7(b) or R6(c)**: every surviving vector
  re-encodes identically and v1 is a byte superset **of a stated subset of v0** —
  v0 minus `{AdvertisementRecord, edge/advert}` under R7(b), minus
  `{GladeIdManifest, edge/manifest}` under R6(c). The sentence must name the
  subset; "superset of v0" is then false. *(This is the case revision 2 missed
  entirely, and **R7(b) is this document's recommendation** — SAF-P2-10.)*
- Under **any byte-moving option** — R4(a), R1(c)/(d), R2(c)/(d), R3(b), R5(c) —
  v1 is **not** a superset in either sense, and no text anywhere may say it is.

**The claim must be proved, not asserted.** Add a `--compat` mode to
`corpus/build.py`, reading v0 from `git show bbce73d:corpus/decl.v0.json`. It
takes an explicit, enumerated deletion list on the invocation — e.g.
`python3 corpus/build.py --compat --deleted AdvertisementRecord,edge/advert` —
and it:

1. asserts `decl.v1.json[n].cbor == decl.v0.json[n].cbor` for every
   `n ∈ v0 ∩ v1`;
2. **fails if any `n ∈ v0 \ v1` is not named in the invocation** — an undeclared
   absence is a failure exactly like byte drift, so a deletion is a checked
   decision and never a silent pass;
3. **fails if any declared name is in fact still present in v1**, so the list
   cannot rot into a blanket waiver as the corpus changes.

A **renamed** vector key is a deletion plus an addition under rule 2, so any
relabelling that changes a key — R3(a)'s two recognition cases are the live
candidates — must appear in the deletion list or the gate is red. Say which of
the two a relabelling is: if the key survives and only the comment changes, the
gate never sees it. **For the live candidates, §4.1 item 1 has already said it**
— keep `edge/binding-message-private` and `edge/binding-deployment-window` "as
the relabelled recognition cases R3(a)'s table promises — **keeping their
keys**, so the relabelling is a comment change and not a rename" — so under
R3(a) the deletion list stays empty on their account, which is what §4.7 row 2
and the recommended-set invocation below already assume. What is stated here is
the general rule, for any future relabelling, not an open question about these
two *(cross-reference added — Consistency residual: revision 3 left it reading
as a question this document had already decided in §4.1)*.

It is a ~40-line addition. **No "strict superset" sentence survives anywhere in
this document, in `README.md` or in the DecisionLog unless that gate is stated as
its proof and is green** — and, where a deletion list is non-empty, unless the
sentence names the subset. Under the recommended answer set
{R1(a), R2(a), R3(a), R4(b), R5(b), R6(a), R7(b)} the gate is green **with**
`AdvertisementRecord,edge/advert` declared and **red without it**, which is the
behaviour that makes it worth running.

If the owner takes R4(a) — or any other byte-moving option — delete the claim
rather than weakening it, and expect `--compat` to fail by design on the vectors
named above; a separate artefact in the taut-shape idiom
(`taut-shape/release/compatibility.v1.json`) is then the right place to record
*what* changed, not that nothing did. §4.7 row 2 and §4.0's publish ordering
state the same three cases in the same words.

### 4.3 Regenerate the three renderings

By the **rendering READMEs'** procedure, which is the complete and correct one
(A4, CON-P2-2): generate to a scratch directory, copy each language's files into
its `src/`, then the ir + corpus copies for ts and py, then `build.py`.

*(Revision 1's block passed `-o ../glade-decl-rs` and so on. `taut/src/taut/gen/scaffold.py:639`
is `d = out_dir / lang`, so that writes `glade-decl-rs/rust/api.rs`,
`glade-decl-ts/typescript/api.ts` and `glade-decl-py/python/api.py` — three new
untracked directories — and leaves every `src/api.*` at v0. The Rust case then
fails loudly (`build.py` regenerates `vectors.rs` from the new schema against a v0
`api.rs`). **The TypeScript case fails silently**: `src/corpus.test.ts:9-20`
drives `codec.ts`/`schema.ts` from the copied IR and never imports `api.ts`, and
`src/index.ts:15` re-exports it as `export type *`, erased at build — so
`pnpm test` is green, `CONTRACT_VERSION` is re-pinned, and
`@owebeeone/glade-decl` publishes v1-labelled types that are the pre-amendment
ones. CON-P2-2.)*

```sh
# from glade-decl/
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o /tmp/g -l rust       --api-only --with-runtime   # -> /tmp/g/rust/{api,cbor,ext}.rs
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o /tmp/g -l typescript --api-only --with-runtime   # -> /tmp/g/typescript/*.ts
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o /tmp/g -l python     --api-only                  # -> /tmp/g/python/api.py

cp /tmp/g/rust/*.rs       ../glade-decl-rs/src/
cp /tmp/g/typescript/*.ts ../glade-decl-ts/src/
cp /tmp/g/python/api.py   ../glade-decl-py/src/glade_decl/api.py

python3 corpus/build.py          # rewrites ir/glade_decl.ir.json, corpus/decl.v1.json,
                                 # and ../glade-decl-rs/src/vectors.rs — which build.py
                                 # passes through rustfmt itself, on write and on
                                 # --check alike (A2, glade-decl d671f10)

(cd ../glade-decl-rs && cargo fmt)   # api.rs/cbor.rs/ext.rs: tautc does not format its
                                     # own output, and the committed Rust is formatted

cp ir/glade_decl.ir.json corpus/decl.v1.json ../glade-decl-ts/src/
cp ir/glade_decl.ir.json corpus/decl.v1.json ../glade-decl-py/src/glade_decl/
git -C ../glade-decl-ts rm src/decl.v0.json   # and the -py twin (§4.2)
```

`rustfmt` is now a **requirement** of the drift gate whenever `glade-decl-rs` is
present: `build.py` takes the crate's edition from its `Cargo.toml`
(`_rust_edition`, `:169-176`), honours a crate `rustfmt.toml`, and exits with a
clear error if `rustfmt` is not on `PATH` (`_rustfmt`, `:179-201`) rather than
comparing unformatted output against formatted files. `cargo fmt --check` must be
clean in `glade-decl-rs` after the copy-in, because `build.py` formats only
`vectors.rs` — `api.rs` comes from the taut compiler.

Then, in one commit per rendering: re-pin `CONTRACT_VERSION` to the amendment
commit in all three (A1). *(Revision 1 also carried "settle A2 … A2 must be green
before anything is published". A2 is now closed at `glade-decl d671f10` /
`glade-decl-rs 21eefa1`; the sentence is kept as history, not as a task.)*

**The copy step must be gated, not remembered (SAF-P2-5).** `build.py`'s
`artifacts` list is still exactly three paths (`:219-221`, compared at
`:223-231`; A2's fix changed what is compared for `vectors.rs`, not what is in
the list) and names nothing under `glade-decl-ts/` or `glade-decl-py/`. Meanwhile both those
gates read **local** copies (`corpus.test.ts:16-19`; `test_corpus.py:23-27`), so
a rendering left at the old commit is locally self-consistent and green while
`build.py --check` is also green. Extend `artifacts` to the four rendering copies
(`glade-decl-ts/src/{glade_decl.ir.json,decl.v1.json}`,
`glade-decl-py/src/glade_decl/{glade_decl.ir.json,decl.v1.json}`) — all four are
byte-identical today, so this is a list extension, not a redesign. And make
`CONTRACT_VERSION` checkable (assert it equals the commit `build.py` last ran at)
or stop calling it a pin: it is an unread string constant in all three renderings
(A1).

### 4.4 The node and the app files

Not a consumer — the other half of the reconciliation.

**Landing order, and why (SAF-P2-3; corrected for the header — SAF-P2-9).**
`appdecl::load` propagates a parse error with `?` out of `main`
(`glade-node.rs:49,88`; `appdecl.rs:204-211` wraps it as
`io::ErrorKind::InvalidData`), so one bad token is a process exit before the
listener binds. The **five** app files live in **four** repositories and cannot
land atomically. *(Corrected — CON-P3-7 / SUR-P3-7: revision 2 said "six
app-file copies", contradicting its own enumeration one line below, A10, §4.4
bullet 6, R9's prose and §2's legend ("the five live `.glade` files"). Command,
run **with `glade-wz` as the working directory** at the tuple in the Appendix:
`grep -rln 'glade-app v0' --include='*.glade' .` → the five paths enumerated in
step 1.)* *(Working directory named — Consistency residual: revision 3 said "run
over both workzones", which a single `.` cannot span. Inside `glade-wz` it
returns the five; run from the workzones' common parent
`/Users/owebeeone/limbo` it also matches `taut-dev/glade/apps/grazel-app.glade`,
a third checkout outside this tuple, and returns **six** — the very number this
correction exists to remove. `gryth-wz` contributes no `.glade` file either
way.)*

**The token and the header have opposite safe orders**, and this is what
revision 2's step 1 got wrong by writing its justification about the token only:

- An **old** node **accepts** a new *token*. `appdecl.rs:137-138` stores
  `toks[4]` and `toks[5]` raw, with no validation, so a pre-amendment node loads
  a file that already says `from_cursor`. Tokens are therefore safe to move
  first.
- An **old** node **refuses** a new *header*. `appdecl.rs:89-95` is an exact
  equality — `if toks != ["glade-app", "v0"] { return Err(…) }` at `:90-91` —
  checked seventeen lines before the binding arm opens at `:107`, so it fires
  before any token is looked at. A file headed `glade-app v1` is a process exit
  on every node that has not been upgraded. Headers are therefore safe to move
  **last**, and only after a node that accepts both headers is deployed.

Therefore, under **R10(a)**:

0. **Step 0 — a both-headers node first.** Deploy a node whose `parse()` accepts
   `glade-app v0` **and** `glade-app v1` — bullet 3 below, the `appdecl.rs:89-95`
   branch and the warning channel — with **no** new token validation. This node
   is strictly more permissive than today's: it accepts everything today's node
   accepts and one thing more, so deploying it breaks nothing.
1. **Step 1 — then the app files' tokens, in all four repositories** — `grazel`
   (`apps/grazel-app.glade`, `apps/gyld-app.glade`), `glade`
   (`apps/grazel-app.glade`, the demo's byte-identical copy), `glade-gyld`
   (`tests/fixtures/gyld-test-app.glade`) and `glade-gwz`
   (`tests/fixtures/gwz-test-app.glade`). **Headers stay `v0` in this commit.**
   The reverse order — node validation before the files — is the only one with a
   stuck state on the *token*, and it is invisible to `cargo test -p glade-node`,
   which reads `CARGO_MANIFEST_DIR/../apps/grazel-app.glade` (`appdecl.rs:281`)
   — the `glade` copy — and would be green while the node refuses to boot on the
   files grazel actually ships (`grazel/src/lib.rs:159`, `let mut app =
   PathBuf::from("apps/grazel-app.glade")`) *(re-pinned — CON-P3-8: `:150` at
   grazel `924cb3c`; the config parsing moved into an `impl Config` block at
   `c66f029` and `:150` is now a doc comment)*.
2. **Step 2 — then the node's vocabulary** (bullets 1, 4, 7, 9, 10, 11 below).
3. **Step 3 — then validation** (bullet 2), under R1's 31b and R2's 18b
   sub-choices — and not at all for a token whose sub-choice is (iii).
4. **Step 4 — last, each file's header moves to `v1`.** A `v1` header is the file's own
   assertion that it is written in the validated language, which is true only
   once its tokens are migrated (step 1) and the validating node is deployed
   (step 3). Step 0 is what makes this step safe; steps 1–3 are what make it
   true. **And the header is dual-maintained exactly as the tokens are** — bullet
   6's rule, repeated here because this step moves a different line of the same
   twinned file *(carried over — Safety observation: revision 3 stated the rule
   for the tokens at step 1 and not for the headers at step 4)*:
   `grazel/apps/grazel-app.glade` and `glade/apps/grazel-app.glade` are
   byte-identical, so **both headers move in the same commit** or the twins
   drift. The consequence of missing it here is drift between two files that are
   required to be identical, not a boot failure — a both-headers node loads
   either — which is why it is a rule of this step and not a gate of it.

**Under R10(b)** there is no step 0 and no step 4: no header moves, so no node
ever meets a header it cannot parse. That is the ordering cost R10's table
carries against (a).

**Every intermediate commit is a gate, not only the last one.**
`cargo test -p glade-node` and `cargo test -p grazel` (§4.7 rows 6 and 8) must be
green after each of steps 0–4 — both suites boot a node on a file this wave
edits, so a red between two steps is the fleet down, not a deferred failure.
§4.7 row 17 is the regression that pins step 0: the **pre**-amendment `parse()`
refuses a `glade-app v1` header with `:91`'s diagnostic, which is why the
both-headers node cannot come after the headers move.

**What must be written before validation is turned on (SUR-P2-1, SUR-P2-6,
SUR-P3-1).** In the same wave, the format's documentation must state the
following — **for token 4 and token 5 alike**. *(Extended to token 5 —
SUR-P2-6. Revision 2 wrote five statements about the zone and none about
retention, while bullet 2 below switches **both** tokens from ignored to
enforced in one sentence. Token 5 is the one the amendment migrates, on the
only 15 lines that change, and its wrong choice is **silent**: a legal token
produces no diagnostic, so an author who reaches for `latest` on an append log
gets a last-writer-wins value and no error.)*

**Token 4 — the zone.**

- **What `commons` means**: everyone in the domain; the wire `key` is empty
  (`demo/src/manifest.ts:74`).
- **What `private` means**: keyed to the principal — with row 16's caveat, because
  only one of the two binders produces the key.
- **The rule for choosing**, per value, in a page a user reads.
- **The behaviour when the token is absent**: there is none — `appdecl.rs:111-114`
  requires exactly five tokens after `binding`, so a four-token line is an arity
  refusal, not a default. Say that; the corpus teaches "type `commons`" (26 of
  28 lines) and teaches nothing else.
- **Whether a mount overrides an authored zone**: on the grip-share path it does
  not — `manifestScope` reads `decl.zone ?? spec?.zone ?? ""`
  (`grip-share/src/manifest.ts:64`), so the declared zone wins and the manifest's
  surface spec is only a fallback. On the glial path the question does not arise,
  because `Fill.zone` never reaches the wire.

**Token 5 — the retention (SUR-P2-6).** Symmetrically, and for whichever set R2
returns:

- **What `latest` means**: the surface keeps one value, last write wins. This is
  the hazardous default to reach for: on a `log`-shaped surface it silently
  converts an append log into a last-writer-wins value, and no diagnostic fires,
  because `latest` is a legal token.
- **What `from_cursor` means**: the surface keeps its history and a subscriber
  resumes from a position rather than from the head. This is what
  terminal-scrollback, output-stream and message-log surfaces want — which is
  why 13 of the 28 authored lines say it.
- **What `ttl` means**, if R2 keeps it: the surface's records expire after a
  duration — and, **as the grammar stands, the duration is unsayable**, so a
  bare `ttl` parses and says nothing. Under **R11(a)** the duration is expressed
  as a tail key, `ttl=10m` / `ttl=1h`, spelled per R11's naming note; under
  R11(b)/(c) `ttl` is unauthorable or absent, and the page must say so rather
  than leave a member of a published enum unexplained.
- **Which to write, per `BINDING_SHAPES` member** — indexed by the shape token
  the author has just typed, exactly as bullet 4's omission rule is *(re-indexed
  — SUR-P3-11: revision 3 wrote this rule by surface kind, which reached three
  kinds against a four-member vocabulary, so `swmr` and `crdt` mapped to none of
  them)*. After this amendment `BINDING_SHAPES` is `value`, `log`, `swmr`,
  `crdt` (bullet 1):
  - `value` → **`latest`**. A settings or status value read at the head; all 13
    `value` lines in the census say it.
  - `log` → **`from_cursor`**. An append log or an output stream, whose
    subscriber resumes from a position rather than from the head; 11 of the 13
    `log` lines say it and the other 2 are the `windowed` pair this amendment
    migrates.
  - `swmr` → **`from_cursor`**. A workspace file tree is resumed, not
    last-write-wins; both `swmr` lines in the census say so
    (`grazel/apps/grazel-app.glade:23` and its byte-identical twin — one
    authored line, dual-maintained).
  - `crdt` → **the shape does not determine the retention, and what does is how
    the surface is read**: `from_cursor` if a subscriber resumes a history,
    `ttl` if its entries expire. **What must not happen is the reflex**: `latest`
    is glossed above as "the surface keeps one value, last write wins", which is
    the negation of the merge semantics `crdt` exists for, so the page must say
    in terms that `latest` does not mean "the merged value" — otherwise the only
    non-log member of the set reads as the natural default for the member this
    amendment adds. This is the one answer the corpus cannot supply: **no app
    file declares a `crdt` binding today**. Census by (shape, retention) at the
    Appendix tuple, from bullet 6's command with `$3` beside `$6`: the 28 lines
    fall into four (shape, retention) pairs — 13 `value`/`latest`, 11
    `log`/`from-cursor`, 2 `log`/`windowed`, 2 `swmr`/`from-cursor` — and **no**
    `crdt` line of any retention. The member this amendment adds, whose profile
    key bullet 4 makes mandatory at parse, is the one with no retention
    precedent at all.
- And across all four, a **cache whose entries should expire** → `ttl` with its
  duration, or nothing at all if R11 leaves the duration unsayable.
- **What omission means**: as for token 4, there is no default — a **four**-token
  line is an arity refusal, not a defaulted retention *(corrected — SUR-P3-13:
  revision 3 said "a five-token line", which is the **legal** line in five other
  places in this document — R11's question ("The line is five positional
  tokens"), R11(c) ("the line stays exactly five tokens"), bullet 4 ("a legal
  five-token line never trips it"), bullet 12 ("someone who wrote a valid
  five-token line") and §5's "is a five-token line people type by hand" — so the
  sentence told an author that the line they are supposed to write is refused)*.
  **The counting convention throughout §4.4 is tokens *after* `binding`**, as
  token 4's bullet above uses it: `appdecl.rs:111-114` tests `toks.len() != 6` —
  six whitespace tokens including the directive, five after it — so omitting the
  retention leaves four after `binding` and is refused there.

Today none of this exists anywhere. Over the eight published pages, `latest`,
`from-cursor`, `from_cursor`, `windowed` and `ttl` score **zero** hits in the
retention sense (`latest`'s ten hits are all `latest.json` or "the latest
build"), `dev-docs/glade/GladeDeclSurface.md` has no `Retention` row beside its
`Shape`, `Authority` and `Domain`/`Zone` rows, and `glade-decl/README.md:24-25`
names `RetentionPolicy` and `Retention` and no member of either. The two page
edits that fix it are named §4 steps: **§4.4 bullet 15** (the `Retention` row in
`GladeDeclSurface.md`) and **§4.2 step 6** (the members table on
`glade-decl/README.md`). §4.7 row 18 is the gate.

**And the contradicting sentence must go, in all four of its homes
(SUR-P2-1).** `grazel/apps/grazel-app.glade:19-21` (byte-identical at
`glade/apps/grazel-app.glade`) tells the author to type a `<zone>` on line 19
and, on line 21, that "the mount fills domain/zone/key (GladeDeclSurface.md)".
`glade/dev-docs/GladeGrazelAttachNotes.md:49-51` says both halves in one
sentence pair, with the clause itself at `:50` and the same parenthetical
citation. **And the page both of them cite is
`dev-docs/glade/GladeDeclSurface.md`, where `:29` reads "The binder's scope maps
them at bind time" and `:30` "each **mount** creates a binding *instance*
`(decl, domain/zone/key fill)`"** — so an author who does what the citation
invites lands on the authoritative statement of the thing the other two were
about to stop saying. *(Added — SUR-P2-1. Revision 2's rewrite list named the
two derivative homes and omitted the source; rewriting only those two silences
the pages that would have warned the author and leaves the authority
untouched.)*

Rewrite it in **both** homes of `grazel-app.glade` (`:21`), in
`GladeGrazelAttachNotes.md:49-51` (the clause is `:50`), and in
`dev-docs/glade/GladeDeclSurface.md:29-30`, carrying in each the answer this
section already gives: the authored zone is the author's to choose, the mount
does not override it on the path that honours it, and on the path that does not,
the fill never reaches the wire. `GladeDeclSurface.md` is the one document R8 and
§4.2 step 5 already open for amendment, so the edit costs nothing extra. Today
the contradiction is inert; after validation it is a line-numbered boot refusal
that no page resolves. §4.7 row 20 is the grep.

Note also that `glade/docs/` — which `glade/README.md:23` advertises as "Public
support contracts and user-facing documentation" — contains exactly one 8-line
placeholder README, which additionally carries the standing rule "Do not move
internal architecture drafts here until they are intended to become public
promises." The only specification of the hand-edited format is
`GladeGrazelAttachNotes.md`, which `glade/README.md:24` classifies as "Internal
engineering design" and which is titled after a different subject. The grammar
block should move to, or be mirrored in, a page presented as user-facing — and
that page, not this document, is where the conventions in bullets 13 and 14
belong.

Then:

- **1.** `glade/node/src/appdecl.rs:43-44` — `KNOWN_SHAPES` gains `crdt` and
  `atom`; `BINDING_SHAPES` gains `crdt` (row 8). Keep the comment at `:39-42`; it
  is already the correct posture. Note `atom` lands in the recognised-but-
  unauthorable tier, where — unlike `exchange`, which redirects to `service` at
  `:121` — there is nothing to redirect to; `:121`'s message then prints only the
  implemented list for `atom`, `message` and `window`. Say so, or give them a
  reserved note (SUR-P3-4).
- **2.** `appdecl.rs:107-140` — validation, **per token and per sub-choice; the
  two tokens are independent and each has three answers, so this bullet has
  three branches, not two** *(third branch added — CON-P2-7: revision 2 wrote
  "hard error or warn-for-one-release is the owner's sub-choice", which lands
  two of the three options and leaves an owner who answers (iii) with no step)*:
  - **(i) hard error** — validate token 4 against `{commons, private}` (row 31b,
    R1 sub-choice) and/or token 5 against the set R2 returns (row 18b, R2
    sub-choice), with the same line-numbered diagnostics as the shape and
    authority checks at `:116-130`.
  - **(ii) warn for one release** — the same check, reported through the
    non-fatal channel R10(a) adds, hard error at the next release.
  - **(iii) do not validate** — **this bullet does not land for that token.**
    No code changes in `appdecl.rs` for it; the token keeps being stored raw at
    `:137` / `:138`; §4.7 row 7 does not apply to it; and §4.4's documentation
    block above is still owed, because the meaning of the token is what the
    author needs whether or not the parser checks the spelling.

  The two sub-choices are answered separately: (i) for the zone and (iii) for
  retention is a legal pair, and so is its converse. Both interact with R10,
  because the shrinkage is what R10 announces.
- **3.** The header, under **R10** — **landing at step 0, ahead of the app-file
  wave, not after it** (SAF-P2-9). Under (a), `appdecl.rs:89-95` branches on
  `v0` vs `v1` — accepting **both** — and `parse()` gains a way to return a
  non-fatal message, which touches `glade-node.rs:88` and grazel's integration
  path; under (b), `v0` is redefined in place and the change is stated in terms,
  with a migration note — the first such document in these repos — and there is
  no step 0 and no step 4.
- **4.** The sixth thing on the line, under **R11** and **R4**. If it exists, it
  ships with its rules (SUR-P2-4), which are:
  - **Legal values**: exactly the GDL-041 profiles, which
    `glade_decl.ir.json`'s `shapes` block already carries with `class: profile`
    and a `core` — `snapshot_delta` (core `swmr`) and `text_crdt` (core `crdt`).
  - **What omission means, per `BINDING_SHAPES` member**: `value`, `log` — no
    profile exists, so a profile key is **refused at parse**; `swmr` — omission
    is legal and means the bare engine (glial mounts bare `swmr` today,
    `shapes.ts:83`); `crdt` — omission is **refused at parse**, because glial
    throws at mount without it (`binder.ts:51-53`), and a file that validates
    into an unmountable surface is the defect.
  - **Enforcement at parse, with a line number**, as every other binding check
    is; tightening this after the freeze is a compatibility break, and the
    spelling is by then in durable records.
  - **The arity diagnostic at `appdecl.rs:113`** updated: it prints the
    five-token template today, so it does not reveal that a sixth thing exists.
    **Under R11(a) this is not enough**, because the arity check at `:111-114`
    becomes a minimum: a legal five-token line never trips it, so the corrected
    template never prints for the author who wrote a valid line and now wants a
    TTL. The tail must therefore also reach the **published** grammar and the
    in-file comments — bullet 12 (SUR-P2-8).
  - **A name that does not collide** — see R11's naming note.
- **5.** **What a changed or deleted line does, under R9**, written *before* any
  file is migrated, in `glade/dev-docs/GladeGrazelAttachNotes.md:56-61` (which
  states the diff rule and stops) and in whatever page §4.4's documentation wave
  makes user-facing. Under R9(a) that is the **retraction rule** plus the fold;
  under (c) it is the fold rule plus an explicit "a deleted line retracts
  nothing". *(Say "the retraction rule", not "the retract form": deletion is
  expressed by a line's **absence** from the file, not by a directive an author
  writes — SUR residual.)*
  - **Under R9(b) the page owes the fold and retract rules for **fifteen**
    surfaces, not two** *(corrected — SAF-P2-11; revision 2 said "under (b) it is
    both, for two lines instead of fifteen")*. (b) reduces the **file** edits to
    two; it does not reduce the surfaces a reader has to be able to reason about.
    Under **b2** it does not reduce the durable churn either: the 13 normalised
    records are appended on the first boot, and under R2(a)/(c)/(d) the 2
    `windowed` records with them, so all **15** are surfaces for which "did my
    migration take effect?" is live and the page must answer it. Under **b1**
    only the two `windowed` records move *(corrected — SAF-P2-12: revision 3 said
    "under **b1** no record moves", which counted R9's spelling churn and not
    R2's edits)*, so those two are owed under b1 exactly the answer the 13 are
    owed under b2 — and they are `term.log` in `grazel-app.glade`'s two
    byte-identical homes, the most visible surface in the tree. The page must
    also say, under b1, that the store holds the file's spelling and the
    contract's spelling is produced at read time.
  - **What a deleted `service` or `workspace` line does today: nothing
    (SUR-P3-12).** R9's answer is scoped to `dir.bindings`, so the page must say
    the other half rather than leave it to be inferred from a ruling's silence.
    Deleting a `service` line retracts nothing and a **retired exchange stays
    routable** — R9(c)'s own cell says exactly that of the unfolded reader.
    Deleting a `workspace` line retracts nothing either, although the node does
    model the entry (`appdecl.rs:176` refuses a duplicate share). `seed` is the
    one directive whose remove half is already published and already works
    (`GladeGrazelAttachNotes.md:85-88`). If R9 takes its (s) option the page
    says the opposite for `service` and `workspace`, and this sentence moves
    with it.
  - **A surface declared by an app file that is not loaded on this boot stays
    declared (SUR-P2-7).** Retraction under R9(a) is scoped per `(app,
    glade_id)`; `glade-node` accepts `--app` repeatedly and grazel passes two
    files when its gyld leg is on, so an unscoped rule would retract grazel's
    seven surfaces the moment `gyld-app.glade` is registered, and gyld's seven
    the moment the leg is switched off. Both files say in terms that their
    surfaces are pre-declared "whether or not a supplier is running"
    (`grazel/apps/gyld-app.glade:19-20`; `grazel/README.md:15-17`). The page must
    say this, because it is the property an author relies on when they declare a
    surface a supplier will serve later.
- **6.** App-file data, under **R9** and **R2**. Under R9(b1) or (b2): **0 edits
  for the 13 `from-cursor` lines**, 2 for `windowed`. Under R9(a)/(c) with a
  file-side respelling: `from-cursor` → `from_cursor` on 13 lines across the 5
  files, plus `windowed` → the R2 replacement on 2. The **durable** cost of each
  is R9's "Durable cost on the first boot" column, which is a derivation over
  (R2's answer, R9's shape) and not the same number for each: under
  **R2(a)/(c)/(d)**, **2** under b1, **15** under b2 and **15** under (a)/(c)
  with a file-side respelling; under **R2(b)**, **0**, **13** and **13**
  *(corrected — SAF-P2-12: revision 3 said "0 under b1, 13 under b2, 15 under
  (a)/(c)", which left the two `windowed` lines — this bullet's own second
  number, and R2's edit rather than R9's — out of the first two)*. Read the
  number off R9's derivation, not off this sentence. `grazel-app.glade` is
  dual-maintained byte-identical in two homes (`grazel/apps/` and `glade/apps/`,
  see its header note at `:10-13` and `grazel/README.md:7-8`) — edit both or the
  node tests fail. Census command, run at grazel `c66f029`, glade `559cb2c`,
  glade-gyld `65da8cb`, glade-gwz `e53c87d`:
  ```sh
  cat grazel/apps/{grazel,gyld}-app.glade glade/apps/grazel-app.glade \
      glade-gyld/tests/fixtures/*.glade glade-gwz/tests/fixtures/*.glade \
    | grep '^binding ' | awk '{print $6}' | sort | uniq -c
  #   13 from-cursor   13 latest   2 windowed        (28 binding lines total)
  ```
  The same five paths are the whole of the migration: `grep -rln 'glade-app v0'
  --include='*.glade' .`, run with `glade-wz` as the working directory, returns
  exactly them (SUR-P3-7) — see the landing order's note on why the directory is
  part of the command.
- **7.** **Removed and renamed tokens stay recognised-but-refused** (SUR-P3-2),
  with a diagnostic naming the replacement, in the exact shape of `:121`'s
  "exchange uses `service`". **The replacement the message names must be a
  spelling the file format accepts** *(SUR-P3-9; revision 2's example message
  named `from_cursor` — the underscore — while its own recommended R9(b) keeps
  the file's hyphen, so an author following the parser's instruction would have
  written a token the format does not use)*:
  - under **R9(b1)** or **(b2)** with the file keeping the hyphen:
    ``unknown retention `windowed` (removed; use `from-cursor`)``;
  - under **R9(a)** or **(c)** with a file-side respelling:
    ``unknown retention `windowed` (removed; use `from_cursor`)``, and the same
    message for `from-cursor` itself, naming `from_cursor`.

  Without it, a reader migrating `term.log` — terminal scrollback — meets
  `["latest","from_cursor","ttl"]`, reaches for `latest` (which silently converts
  an append log into a last-writer-wins value) or `ttl` (which parses and leaves
  the duration unsayable), and the migration is a one-shot event. The diagnostic
  covers the **removed** token; only the documentation block above covers the
  author who types a legal token and means something else (SUR-P2-6).
- **8.** **`GladeGrazelAttachNotes.md:98`** (SUR-P3-4) says a provider may attach
  to a glade id declared by "a `dir.services` record, **or a `dir.bindings`
  record with shape `exchange`**". A reader following that writes `binding
  gwz.ops exchange share commons latest` and meets `:121`'s refusal. Fix the
  sentence to say the `dir.bindings` form is not authorable and `service` is the
  authored form. (The `dir.bindings` *fold* at `exchange.rs:70-77` does honour
  such a record if one exists; nothing can author one.)
- **9.** Document `sysdata.BindingDecl.app ≡ AdvertisementRecord.package`
  **in `glade/node/ir/sysdata.taut.py`'s comment at `:74-77`** (row 33), so the
  statement lives with the record it describes and survives R7(b).
- **10.** `glade/decl/*` (A3) — banner it as superseded by `glade-decl/`, or
  archive it. It is the origin of the node's free-string fields and of the hyphen
  spelling, and it is a second answer to a question this amendment freezes.
- **11.** `glade/client-rs/src/session.rs:27` (A7) — the error text lists
  "supported: value, log, swmr" while `:24` accepts `"crdt"`.
- **12.** **Every published grammar and every in-file grammar comment moves with
  the line** (SUR-P2-8). The format has exactly two kinds of place an author
  copies from, and the amendment changes the line in both R10(a) and R11(a)
  without either ruling costing a line for them:
  - `glade/dev-docs/GladeGrazelAttachNotes.md:29-36`, the one published grammar
    block: `:30` is the header token (moves under **R10(a)**) and `:32` is
    ``binding <glade_id> <shape> <authority> <zone> <retention>`` (gains the
    tail under **R11(a)**, or the sixth positional token under **R11(b)**).
  - **The same edit completes that block, which omits a directive every app file
    uses (SUR-P3-10).** `:29-36` publishes `glade-app v0`, `app`, `binding`,
    `service`, `seed` and the comment rule, and no `workspace` — the omission A9
    already records. `parse()` accepts one (`appdecl.rs:168`, printing its own
    template ``workspace <share> <name>`` at `:173`), and **5 of the 5** app
    files carry exactly one (`grep -c '^workspace ' <file>` → 1 for each of the
    five paths in step 1, at the Appendix tuple). Add
    ``workspace <share> <name>`` to the block **with the one clause that says
    what it does: it is the line that makes a declared surface routable.**
    Without it, a file written from the published grammar alone parses,
    registers and does not route — which the files themselves state and the page
    does not (`grazel/apps/grazel-app.glade:47-53`: "Required for
    `(ws-razel, gwz.ops)` to route to the composed glade-gwz supplier"). The
    exposed author is the third-party one `grazel/README.md:73-79` describes,
    working from the specification rather than from a neighbour's file: the
    three files that carry an in-file comment already show
    `# workspace <share> <name>` (`grazel/apps/grazel-app.glade:47`, its
    byte-identical twin, and `grazel/apps/gyld-app.glade:64`), so a copier is
    safe and only the published page misleads. Additive, no compatibility
    consequence, and it is this bullet's own wave that opens the block.
  - The in-file comment, present in **3 of the 5** files —
    `grazel/apps/grazel-app.glade:19`, its byte-identical twin
    `glade/apps/grazel-app.glade:19`, and `grazel/apps/gyld-app.glade:17`. The
    two fixtures carry no grammar comment and no token names at all, so the
    in-file grammar is a 3-of-5 property, not a 5-of-5 one; say so, or give the
    two fixtures the comment.
  - **And one sentence on how an author learns the tail exists**, because under
    R11(a) `:113`'s template is unreachable for them (bullet 4). The honest
    answer is that the tail must be in the grammar they read, since neither the
    arity check nor a key refusal will teach it to someone who wrote a valid
    five-token line. §4.7 row 19 is the gate.
- **13.** **The joining-character convention goes in the format's own grammar
  documentation**, not only in this design document (SUR-P3-9, SUR-P3-3). There
  is not one underscore in any authored token of any of the five files, so the
  convention is the hyphen; state it once, beside the grammar block, so the next
  multi-word token is not a second coin flip after the freeze. Today the
  convention exists only in R9's prose and R11's naming note — both inside this
  file, which no author reads.
- **14.** **Say which `.glade` dialect this amendment freezes** (SUR-P3-6). One
  file extension names two mutually unintelligible declaration languages:
  `find . -name '*.glade'` over `glade-wz` returns **sixteen** files, of which
  `grep -rln 'glade-app v0' --include='*.glade' .` matches exactly **five** —
  the app files. The other **eleven**, all under root `dev-docs/examples/`, are
  brace-nested with `//` and `/* */` comments, quoted string ids and
  `package … version "0.1.0" { … }` / `application … { }` blocks composing a
  package graph. The amendment must say, in one sentence, that what it freezes
  is the line-oriented `glade-app` app-declaration dialect and not that one.
  Then either give the eleven the **A3 treatment** — a banner naming them a
  different, unimplemented language — or record the extension question for the
  owner. Bounded, because the header check refuses the wrong dialect at line 1
  with a line number (`appdecl.rs:90-91`) and `--app` takes explicit paths with
  no globbing; but this is the review that fixes what the extension means, and
  `dev-docs/glade/GladeDeclSurface.md:123` makes a promise the other language
  visibly does not keep — "`.glade` is data; it never becomes a compiler
  front-end".
- **15.** **`dev-docs/glade/GladeDeclSurface.md` gains a `Retention` row**
  beside its `Shape` (`:27`), `Authority` (`:28`) and `Domain`/`Zone` (`:29`)
  rows, glossing each member of the set R2 returns (SUR-P2-6). This is the
  companion to §4.2 step 6's members table on `glade-decl/README.md`, and it
  lands in the same edit as A12 and the zone-sentence rewrite, since §4.2 step 5
  already opens that document. Today `retention` appears in it only inside
  `BindingDecl`'s tuple at `:30`, and `GladeGrazelAttachNotes.md:145`
  explicitly defers the question ("retention/timeout policy per declaration is a
  decl-surface question") — which is the deferral this amendment ends.
- **Gate:** `cargo test -p glade-node` (`appdecl.rs`'s **7** `#[test]` functions —
  `grep -c '#\[test\]' glade/node/src/appdecl.rs` → 7 at glade `559cb2c` —
  including the 7-binding assertion at `:296` and the 11-record one at `:408`)
  **and** `cargo test -p grazel` (`grazel/tests/integration.rs:109,280,454` boot
  a node on `apps/grazel-app.glade`; nothing in §4.7 reached that repository in
  revision 1). Both must be green **after every step of the landing order**, not
  only after the last (SAF-P2-9).

### 4.5 Consumers, in order

1. **`glial`** (first — it is the typed consumer of the changed names).
   `src/shapes.ts` gains nothing (it is already GDL-041-correct). `src/manifest.ts`
   — `SurfaceSpec`/`Surface` gain optional `profile` under R4(a), or a lookup
   under R4(b); `toSurface` defaults it; `DEFAULT_RETENTION` unchanged.
   `src/binder.ts:50-53` reads the declared profile ahead of `config.crdtProfile`,
   keeping the config path for back-compat. Gate: `vitest run` + `tsc --noEmit`
   (GAP-14's `@ts-expect-error` wall, `glial/dev-docs/DecisionLog.md:256-278`).
2. **`grip-core` types.** `src/core/share_decl.ts:17` is the single import line;
   nothing in it breaks under an additive change. It must still be republished at
   `0.3.0` (`PackageExtractionPlan.md:157`) because its published
   `dist/index.d.ts` references an unpublished `@owebeeone/glade-decl`. If R1(d)
   removed `DomainAnchor`, `share_decl.ts:35` would break; if R5(c) removed
   `Authority`, `:32` would. *(CON-P3-2: revision 1 said "`share_decl.ts:35,39`";
   `:39` is `zone?: ZoneKind`, which neither ruling touches.)*
3. **`glade/grip-share` + `glade/demo`** *(new — CON-P2-1; revision 1 omitted
   this repository entirely)*. `manifestScope` (`grip-share/src/manifest.ts:58-70`)
   resolves the wire `share` from `decl.domain` and the wire `key` from
   `decl.zone`, through the demo's `WORKSPACE_MANIFEST.domains`/`.zones`
   (`demo/src/manifest.ts:65-78`). Under the recommended R1(a) and R5(b): no
   edit. Under R1(c) or R1(d): this is the repository that breaks, and it is the
   running demo §4.6 protects. Gate: the demo resolves `{domain:"account",
   zone:"private"}` to `share="account:<user>"`, `key=utf8("self:<user>")`.
4. **`gryth-wz/gryth-ui`** (last, and probably a no-op). It names **zero**
   glade-decl types; its whole contact is untyped object literals passed to
   `defineManifest` at four sites (`packages/plugins/gwz/src/live.ts:32`,
   `packages/plugins/gyld/src/ops/surfaces.ts:28`, `src/taps.ts:12`, and
   `glade-chat/src/manifest.ts:77` on its behalf). All nine of its surfaces get
   `authority`, `source` and `retention` from glial's defaults — none chosen.
   Sequencing note: gryth-ui resolves the contract through a **duplicate
   checkout**, `gryth-wz/glade-decl-ts` (`pnpm-workspace.yaml:11` reaches outside
   the repo root; `overrides` at `:25-29` forces the singleton). Both checkouts
   are at `7e16e32` and clean today. Either land Phase 0.3 of the extraction plan
   first, or update both checkouts in the same pass — otherwise the singleton
   override silently pins the old contract.

### 4.6 What must NOT change

- **The node's wire IR.** `taut/corpus/glade.ir.json`, `glade/wire-rs/src/generated.rs`,
  `glade/client-ts/src/taut/`. The wire `Shape{value:0, log:1, stream:2, swmr:3,
  crdt:4}` numbering is frozen and shares only a name with the declaration enum
  (§1). `atom` does **not** enter the wire in this amendment.
- **`gryth-ui/packages/glade/src/glade.ir.json`** — that is the wire IR vendored
  by hand, not the declaration IR, and it is separately stale (A8). Its fix is
  `PackageExtractionPlan.md` step 2.6. Touching it here would smear two
  unrelated contracts into one commit.
- **`glade/node/src/sysdata.rs`** unless R1/R2/R4/R5/R9/R11 add a record field; it
  is generated with `--legacy-codec` by its own procedure
  (`glade/node/ir/sysdata.taut.py:19-25`) and changing it changes durable records.
- **The running demo's behaviour** — which is now a *constraint on the rulings*,
  not a description of them. Two corrections to revision 1:
  - *(CON-P2-1)* R1(c) and R1(d) **would** change it: they break
    `manifestScope`'s domain branch and `ShareDecl.domain`. That cost is now in
    R1's option table, and it is the strongest argument against (d).
  - *(SAF-P1-2)* The retention and zone tokens are **not** "fields nothing
    reads". They are read as **bytes**, by `register`'s diff at `appdecl.rs:264`.
    Revision 1 said the app-file edits touch "fields nothing reads"; that is
    false, and it is what hid A9. Any token edit changes a record's bytes and
    therefore appends a second declaration on the next boot, under **every**
    option of R2 and independently of R4. §4.4 bullet 3 of revision 1 attributed
    the durable reach to R4(a) alone; the token normalisation reaches stored data
    on its own.

### 4.7 The checks that prove it

*(Split into (a) gates, (b) gates conditional on a ruling, and (c) evidence —
CON-P2-6. Revision 2 stated one flat table under a publish precondition reading
"every row of §4.7 is green", while row 2 fails by design under R4(a), row 7
presupposes a validation the sub-choices may decline to turn on, row 14 declares
in its own cell that it is "not a gate", and row 16 is a manual observation with
no green/red state. The row numbers are unchanged, so every pointer into revision
2's table still lands; rows 17–20 are new at revision 3, and row 21 — R8's
amendment of ratified text — at revision 4.)*

**(a) Unconditional gates.** Green on a settled tree under every combination of
answers to R1–R11 — and, for rows 6 and 8, green at **every intermediate commit**
of §4.4's landing wave, not only at its end (SAF-P2-9).

| # | Check | Where | Proves |
| --- | --- | --- | --- |
| 1 | `python3 corpus/build.py --check` | `glade-decl/` | IR + corpus + rs vectors + **the four rendering copies** in lockstep. Green today (A2 closed at `d671f10`/`21eefa1`; the Rust vectors are rustfmt'd on both paths, so the gate compares like with like) — but until §4.3's artefact-list extension lands it proves only **three** of the seven paths (SAF-P2-5). `rustfmt` must be on `PATH`: `build.py` exits with a clear error when it is absent (`_rustfmt`, `:179-201`) rather than comparing unformatted output against formatted files, so a missing tool is a red gate and never a false green |
| 3 | `pnpm test` (`vitest run`) | `glade-decl-ts/` | independent TS codec reproduces every v1 byte. **Does not exercise `api.ts`** (`corpus.test.ts:9-20` never imports it; `index.ts:15` erases it at build) — see row 11 |
| 4 | `cargo test` | `glade-decl-rs/` | independent Rust codec reproduces every v1 byte |
| 5 | `pytest`, from a built wheel | `glade-decl-py/` | reference codec round-trips **and** the renamed corpus is in the distribution (`pyproject.toml:20`) |
| 6 | `cargo test -p glade-node` | `glade/` | `appdecl.rs`'s **7** `#[test]` functions — the 7-binding assertion at `:296`, the 11-record one at `:408` — still pass with `crdt` and `atom` in `KNOWN_SHAPES` (rows 8 and 9, both SETTLED). Count at glade `559cb2c`: `grep -c '#\[test\]' glade/node/src/appdecl.rs` → 7. The zone/retention assertions are row 7's, not this row's |
| 8 | `cargo test -p grazel` | `grazel/` | the node boots on the app files grazel actually ships (`tests/integration.rs:109,280,454`) — the repository revision 1's table never reached (SAF-P2-3) |
| 11 | `git status --porcelain` shows no `typescript/`, `rust/` or `python/` path, and `grep -q '"atom"'` in all three generated APIs | the three renderings | the regeneration landed in `src/`, not in a new directory, and the new member is actually in the shipped types (CON-P2-2) |
| 12 | `vitest run` + `tsc --noEmit` | `glial/` | `Surface extends BindingDecl` still holds; the compile wall still errors on an undefined key |
| 15 | `pnpm test` + `pnpm build:gyld` | `gryth-ui/` | **61** vitest suites, incl. `packages/plugins/chat/src/groups.test.ts:23-40` (the only decl-field assertions in the repo). Count command, run at gryth-ui `9323818`: `git -C ../gryth-wz/gryth-ui ls-files \| grep -cE '\.test\.tsx?$'` → **61**, and all 61 match the include globs at `vite.config.ts:310-316` (the `test:` block opens at `:309`) *(re-pinned — CON-P3-8 / SUR-P3-8. Revision 2 said 60 against `vite.config.ts:287-293` at gryth-ui `3af64c2`; eight `gyld` suites landed in between and the vitest block moved, so `:287-293` is now a dev-server proxy comment.)* |
| 20 | grep the page set for "fills domain/zone/key", "zone … fill" and "maps them at bind time" | the published pages | every surviving occurrence says the author writes the zone and what a mount does with it — no page tells an author the mount fills a token the parser requires them to type (SUR-P2-1). Unconditional, because the sentence is already wrong on the grip-share path today: `manifestScope` reads `decl.zone ?? spec?.zone ?? ""`, so the declared zone wins. The four homes are `grazel/apps/grazel-app.glade:21`, its byte-identical twin at `glade/apps/grazel-app.glade:21`, `GladeGrazelAttachNotes.md:50`, and `dev-docs/glade/GladeDeclSurface.md:29-30` — the page the other three cite by name |

**(b) Gates conditional on a ruling.** Each row names the ruling it turns on and
what green means under each of that ruling's options. Once the owner records the
answers, every (b) row whose condition arises joins (a); a row whose condition
does not arise is not a gate and no publish waits on it. **R8's *corpus* gate set
is (a)**, and that is deliberate: both of its options are additive (new messages,
so new synth vectors and no existing one moves — §4.2). *(Corrected —
Consistency residual: revision 3 said "**R8 appears in no (b) row**" and gave
that additive reason, which is right about the corpus and leaves R8's edit to
**ratified text** — the amendment sentence in `dev-docs/glade/GladeDeclSurface.md`
— covered by no row at all. Row 21 is that gate; the reason revision 3 gave now
carries only the weight it can bear.)* Every ruling, R1–R11, is named in at
least one row below.

| # | Check | Ruling | What green means, per option |
| --- | --- | --- | --- |
| 2 | `python3 corpus/build.py --compat` (`glade-decl/`) | **R4**, and every ruling with an option that deletes or moves a vector: **R1, R2, R3, R5, R6, R7** | **No byte moves and nothing is deleted** — R4(b)/(c) with R1(a)/(b), R2(a)/(b), R3(a), R5(a)/(b), R6(a)/(b), R7(a): green with an **empty** deletion list. **Deletions only** — add R7(b) (drops `AdvertisementRecord`, `edge/advert`) or R6(c) (drops `GladeIdManifest`, `edge/manifest`): green **only** when the invocation declares exactly the departing vectors. Under the recommended set that is `--compat --deleted AdvertisementRecord,edge/advert`; **without the declaration the gate is red**, which is the point (SAF-P2-10). **Any byte move** — R4(a), R1(c)/(d), R2(c)/(d), R3(b), R5(c): this row **does not apply**. It is replaced by the taut-shape-idiom compatibility artefact §4.2 names (`taut-shape/release/compatibility.v1.json`), which must exist and enumerate exactly which vectors changed; the superset claim is then **deleted**, not weakened (§4.2) |
| 7 | new `glade-node` unit test (`glade/`) | **R1**'s sub-choice (row 31b) and **R2**'s (row 18b) | Under **(i)** or **(ii)** of a sub-choice, the **line-numbered diagnostic** exists for that token and names the replacement (SUR-P3-2): an unknown zone under R1(31b); `windowed`, and `from-cursor` if the file side respells, under R2(18b). Under **(iii)** the token is not validated, no diagnostic is produced, and the row **does not apply to that token** — asserting one would be asserting a refusal the owner declined. Both sub-choices (iii) ⇒ the row is not a gate at all (CON-P2-7) |
| 9 | new `glade-node` unit test (`glade/`) | **R9** | register the **pre**-amendment parse into a `Registry`, then the **post**-amendment parse; assert the intended `Registered{appended, unchanged}` and the resulting `dir.bindings`. **The asserted `appended` is the chosen option's own stated durable cost and must equal it**, and that cost is conditional on R2 as well as on R9 — read it off R9's derivation paragraph, never off memory. Under **R2(a)/(c)/(d)**: **2** under (b1), **15** under (b2), **15** under (a) or (c) taken with a file-side respelling. Under **R2(b)**: **0**, **13** and **13** *(corrected — SAF-P2-12: revision 3 asserted 0 / 13 / 15 unconditionally; its first two are R2(b)'s numbers, so under the document's own recommended R2(a) this test was specified red on its first run — the one place a red gate invites an executor to adjust the assertion to observed behaviour)*. `registering_twice_appends_nothing` (`appdecl.rs:404-413`) cannot see this, because it registers the same decl twice into a fresh registry (SAF-P1-2, SAF-P2-11) |
| 10 | new `glade-node` unit tests (`glade/`) | **R9** | Under **(a)**, three tests, because retraction is scoped per `(app, glade_id)` (SUR-P2-7): register `grazel-app.glade` then `gyld-app.glade` into **one** registry and assert all **14** bindings live and none retracted; register `grazel-app.glade` alone on the next boot and assert gyld's **7** untouched; delete one line from `gyld-app.glade`, reload both, and assert exactly that one surface retracted. Under **(c)**: assert the documented fold picks the newer record **and** that a deleted line retracts nothing, so the page and the code say the same thing. Under **(b)** alone: there is no retraction to test, and no answer to deletion either |
| 13 | the demo resolves `{domain:"account", zone:"private"}` (`glade/grip-share` + `glade/demo`) | **R1** | Under **(a)** or **(b)**: `manifestScope` still produces `share="account:<user>"`, `key=utf8("self:<user>")` with **no edit** — the clause §4.6 protects (CON-P2-1). Under **(c)** or **(d)**: this is the repository that breaks; green then means the demo resolves the same surface through whatever replaces the anchor, and R1's option table prices that work |
| 17 | new `glade-node` unit test (`glade/`) | **R10** | Under **(a)**: the **pre**-amendment `parse()` refuses a `glade-app v1` header with `appdecl.rs:91`'s diagnostic — it does today, and that is exactly why a both-headers node must be deployed before any file's header moves. This test is the regression that pins §4.4's step 0 (SAF-P2-9). Under **(b)**: no header moves, so there is no ordering hazard and no test |
| 18 | grep the published page set for each member of the retention set v1 keeps | **R2** | every value an app file may write has a one-line gloss **in the retention sense** on a page an author reads (SUR-P2-6). Under **(a)**: `latest`, `from_cursor`, `ttl`. Under **(b)**: those three plus `windowed`, with what a window retains. Under **(c)**: `latest`, `from_cursor`. Under **(d)**: there is no vocabulary to gloss, so what must be written is the free-string convention and the fact that nothing enforces it. Baseline, over the eight published pages at this tuple: `latest`, `from-cursor`, `from_cursor`, `windowed` and `ttl` score **zero** hits in the retention sense |
| 19 | every published grammar and every in-file grammar comment shows the line the parser accepts | **R10** and **R11** | Under **R10(a)** the header token at `GladeGrazelAttachNotes.md:30` moves to `v1`; under **R10(b)** it does not, and the page says in terms that `v0` was redefined. Under **R11(a)** the tail appears at `GladeGrazelAttachNotes.md:32` and in the three in-file comments (`grazel/apps/grazel-app.glade:19`, `glade/apps/grazel-app.glade:19`, `grazel/apps/gyld-app.glade:17`); under **R11(b)** or **(c)** the line keeps its five tokens and only `:113`'s template moves. Green means an author who has read only those and never provoked a parse error writes `ttl=10m` and `shape-profile=text_crdt` correctly at the first attempt (SUR-P2-8) |
| 21 | grep `dev-docs/glade/GladeDeclSurface.md` for the record-kinds sentence R8's answer requires, and `glade-decl/dev-docs/DeclSurface.md` for the mirror banner | **R8** | The controlling document is the **root** one (CON-P2-5), and either answer obliges it to stop saying what it says today against GDL-037's three record kinds (`GladeDeclSurface.md:113-114`). Under **(b)** — the recommendation — green means the root document carries the explicit amendment sentence that `glade-decl` is the tap/binding vocabulary only and that `ServiceDefinition` and the ACL-seed records stay node `sysdata`, and that the repository copy is a banner-marked mirror with §4.2 step 5's drift check. Under **(a)** green means the two record kinds are in the contract and rendered in all three languages — row 1's `--check` then covers the bytes — and the same page says so rather than the pre-amendment claim. Red under either answer means a ratified document still asserts a file form the contract does not match *(added — Consistency residual: R8(b) amends ratified text and no row covered that landing; row 20 greps only for the mount sentence)* |

**(c) Evidence, not gates.** Recorded so the freeze is made with them known.
Neither has a green/red state and no publish waits on either.

| # | Observation | Where | What it records |
| --- | --- | --- | --- |
| 14 | a glial test: mounting `zone: "private"` either produces a `self:`-prefixed wire key or throws | `glial/` | a **pre-freeze item**: today it silently does neither, which is exactly **§2d row 16**'s divergence (SAF-P2-4) — `ZoneKind.private=1`, "DIVERGENT BETWEEN BINDERS", not §4.7's own row 16 directly below *(disambiguated — Safety observation: the two tables both have a row 16 and this one sits above the other)*. Written down so the gap is a known one at the freeze, not a discovery after it |
| 16 | manual: `gyld-ui.py start` reaches its published-builds line | — | the running demo untouched. A human observation of `/Users/owebeeone/limbo/gryth-wz/gryth-ui/gyld-ui.py`, with no exit code to read |

### 4.8 Tooling facts found

- `glade-decl-ts` carries `package-lock.json` only, and it is one of two such
  members, not the only one (A5). The owner's standing rule is pnpm for
  *commands* (`pnpm test`, `pnpm install`) and **no unasked lockfile migration** —
  so run pnpm, leave the npm lockfile alone, and raise the migration as its own
  question. `PackageExtractionPlan.md` step 1.2 is where it naturally belongs.
  One consequence lands inside this amendment rather than there: `glade-decl-ts/README.md:21-22`
  still documents `npm install` / `npm test`, and §4.3 adopts that README's
  procedure as canonical while §4.7 row 3 and this section prescribe `pnpm`. Fix
  the two command lines when A4's collapse is made — that is a *command* change,
  not a lockfile migration, so the standing rule permits it.
- `glade-decl` itself has no build manifest and is not a package
  (`PackageExtractionPlan.md:72`); `corpus/build.py` needs its siblings at fixed
  relative positions (`:44-46` for `glade-decl-rs`, `:50` for `taut`). Step 4.7
  of that plan owns it. Since A2 it also needs `rustfmt` on `PATH` whenever
  `glade-decl-rs` is present — a new tool dependency of the drift gate, worth
  naming wherever CI for this repo is eventually described.
- `glade-decl-py` bootstraps taut by `sys.path` at `tests/conftest.py:16`; per
  the workspace memo, use `/opt/homebrew/bin/python3` (the asdf 3.10 default
  lacks `tomllib`).

---

## 5. Should this go through the adversarial review loop before the freeze?

**Yes — the full loop, three axes, before any publish.** Rounds 1 and 2 have both
been run; this document is round 2's remediation, and **remediation round 2 is
the last one the loop allows**: if any re-verdict on revision 3 names a new
ARCHITECTURAL root cause, the lane stops and the owner decides redesign-or-accept
rather than a fourth revision. No reviewer has so classified any finding in either
round — all three round-2 reports state in terms that none of theirs is
architectural, and the architectural facts in play (no fold for `dir.bindings`;
glial not producing the private key) remain properties of the system this document
describes, stated truthfully rather than fixed here. The object is precisely the one
`AgentProcessRules.md` L1-09 names: a change to a shared interface and a
compatibility rule — and, via §4.4, to durable state — at a release boundary.
L1-18 (`:387-393`) makes two independent review axes mandatory there, and its
2026-09-18 amendment (operator decision D7, `:406-420`) adds a third,
**Surface**, mandatory at "every interface freeze — any package that fixes a
command family, a flag set, a settings block, **a file format people edit**, or
an API". `<app>.glade` is a hand-edited file format and this amendment changes
its accepted vocabulary, so the Surface axis is not optional here. L1-17
(`:372-375`) requires the tree "clean except for explicitly authorized report
outputs". Round 1 flagged A2 (the red drift gate) and `glade-decl-rs`'s
uncommitted rustfmt diff as a standing precondition, out of scope by the lane
owner's instruction. **That precondition is now met**: the owner ruled on it
between round 1 and this revision, `glade-decl-rs` is clean at `21eefa1`, and
`corpus/build.py --check` exits 0 at `glade-decl d671f10` (A2). Round 2 has a
settled tree to review.

The economic argument is the same one that produced this document: publishing
freezes the contract, the corpus is a byte oracle, and the amendment is
one-shot. A missed element does not cost a patch — it costs a second corpus
version, three regenerations, a republish of `@owebeeone/glade-decl` and
`@owebeeone/grip-core`, and a second freeze. Round 1 is the evidence: three
reviewers reading this table independently against the code found a false
byte-compatibility claim that the whole §4.2 argument rested on (SAF-P1-1), a
first-boot duplication in durable records that no gate could see (SAF-P1-2), a
consumer repository the survey had missed (CON-P2-1), a regeneration block that
would have published pre-amendment types silently (CON-P2-2), and the fact that
`binding <glade_id> <shape> <authority> <zone> <retention>` is a five-token line
people type by hand with no page naming a single legal value of three of those
tokens (SUR-P3-1). None of that is reachable from the schema alone.

Round 2 is the second piece of evidence, and it is about the **remediation**
rather than the original: every one of its nine blocking findings lives inside
text revision 2 added, and three of them are the same shape — a correction made
for the thing it was asked about and not for the thing beside it. The zone token
was documented and the retention token was not (SUR-P2-6); the two derivative
homes of a false sentence were rewritten and the page they both cite was not
(SUR-P2-1); the app-file wave was ordered correctly for the token and not for the
header the same commit moved (SAF-P2-9); the option recommended to avoid a
durable rewrite was placed where the durable record is built (SAF-P2-11); the
gate that proves compatibility was written for vectors that change and not for
vectors that leave (SAF-P2-10). A one-shot amendment is exactly where that class
of near-miss is expensive, and none of the five is reachable without reading the
code and the pages the amendment does not cite.

---

## Appendix — files read

**Revisions — re-pinned to the round-2 tuple (CON-P3-8 / SUR-P3-8).** Revision 2
pinned the members it was drafted against; four of them moved between rounds, and
five of its published proofs stopped reproducing at the tuple it is read at. The
pins below are the tuple recorded in `-RemPlan.md`'s "Round 2 — tuple and tier"
block and verified directly (`git -C <member> rev-parse --short=12 HEAD`, every
tree clean), and every citation into a moved member was re-opened at its new SHA.

glade-wz root `61bb479dadf4` (documents; the round-1 reports were read at
`879de50a455e` and the round-2 reports at `574319d`, unchanged since) ·
**`glade 559cb2c87e85`** (was `960c9b0fa038`: 25 commits, the async-witness
workspace under `dev-docs/`, `node/src/iroh_carrier.rs`, an iroh floor. No
`node/src` path this document cites changed — `appdecl.rs` is byte-identical
across the two — but A10's repository-wide census does, and is restated) ·
`glade-decl d671f10c13e6` (A2's fix; the audit was performed at `bbce73d67146`
and `ir/glade_decl.ir.json` + `corpus/decl.v0.json` are byte-identical across the
two, so every schema and corpus citation holds — only `corpus/build.py` and
`README.md` line numbers moved, and those were re-opened at `d671f10`) ·
`glade-decl-ts 7e16e324630a` · `glade-decl-rs 21eefa1c3a53` (clean; the audit was
performed at `555a97746fc6` with `git show`, its tree then being dirty — A2) ·
`glade-decl-py 1b0f6d1f7886` · `glial 0dfe4b930063` · `grip-core 97ff6c26f12e` ·
**`grazel c66f029ad060`** (was `924cb3c4bab9`: one commit, `src/lib.rs` only,
+83/−8; the default app path moved `:150` → `:159` into an `impl Config` block —
§4.4 step 1 re-pinned; `apps/*.glade` and `tests/integration.rs` unchanged) ·
**`glade-gyld 65da8cb7e2b7`** (was `024d2a8ae061`: `README.md` 981 → 1192 lines;
`:315` → `:499` for the compatibility profile (R11's naming note) and `:934` →
`:1147` for the `(lamport, origin)` value fold (R9(c)); `tests/fixtures/gyld-test-app.glade`
unchanged) · `glade-gwz e53c87dddb8f` · `glade-chat 9238d21f6a36` ·
`taut 7a5f616c3a9f` · `taut-shape 9a752094dbed` (the ten-corpora count, §4.2) ·
**`/Users/owebeeone/limbo/gryth-wz/gryth-ui 9323818a39d2`** (was `3af64c2bab74`:
eight new `gyld` suites, so §4.7 row 15's count is 61 and its include globs are
`vite.config.ts:310-316`, not 60 and `:287-293`); for authority only,
`/Users/owebeeone/limbo/gwz-dev ff431743cc4c` (`dev-docs/AgentProcessRules.md`
byte-unchanged at gwz-dev HEAD `7b3f1bc723d6`).

**The re-pinning discipline extends to the authority repository.** *(Added —
Consistency residual: `gwz-dev` is not a member of the lock, this document cites
it by path, and it moved between rounds while the pinned-revision list did
not.)* `gwz-dev` is at **`9c0008870ecd`** at the round-3 tuple (it was
`7b3f1bc723d6` at round 2), and
`git -C gwz-dev log ff431743cc4c..9c0008870ecd -- dev-docs/AgentProcessRules.md`
returns **0 commits**, so L1-07, L1-08, L1-09, L1-17 and L1-18 are quoted from
`ff431743cc4c` and are byte-identical at that head. Every later round re-runs
that one command rather than re-reading the clauses: a moved head with an empty
log over the cited path is not staleness, and a non-empty one is.

**Every count in this document states the command that produced it and the
revision it was run at**, so a later reader can tell staleness from error: §4.7
row 15 (61, gryth-ui `9323818`), A10's census (`0 2 0 0 0`, the five repositories
above), §4.4 bullet 6's app-file census and the five-file `grep -rln`, §4.2's 20
`decl.v0` references and its ten taut-shape corpora, R4's `11 of 26`, A5's two
`find`s, A6's `find`, and `grep -c '#\[test\]'` → 7 at glade `559cb2c`. A count
whose target can move and which carries no revision is a defect of this document,
not of the reader.

Contract: `glade-decl/{README.md, ir/glade_decl.taut.py, ir/glade_decl.ir.json,
corpus/build.py, corpus/decl.v0.json, dev-docs/DeclSurface.md,
dev-docs/OpenNotes.md}`. Renderings: `glade-decl-ts/{package.json, README.md,
src/{index,api,corpus.test}.ts}`, `glade-decl-rs/{README.md, src/{lib,api,vectors}.rs}`,
`glade-decl-py/{README.md, pyproject.toml, src/glade_decl/{__init__,api}.py,
tests/{conftest,test_corpus}.py}`.
Node: `glade/node/{ir/sysdata.taut.py, src/{appdecl,registry,store,router,sysdata,exchange}.rs,
src/bin/glade-node.rs}`, `glade/{client-ts/src/shapes.ts,
client-rs/src/{session,supplier}.rs, wire-rs/src/generated.rs, decl/*,
demo/src/manifest.ts, **grip-share/src/manifest.ts**, docs/README.md, README.md,
dev-docs/GladeGrazelAttachNotes.md}`, `taut/corpus/glade.ir.json`.
taut itself (new in revision 2, for §1's encoding fact and §4.1.8):
`taut/src/taut/{wire/codec.py, corpus/synth.py, ir/{dsl,validate}.py,
gen/scaffold.py}`, `taut/docs/Reference.md`.
App files: `grazel/apps/{grazel,gyld}-app.glade`, `glade/apps/grazel-app.glade`,
`glade-{gwz,gyld}/tests/fixtures/*.glade`; and the five repos' READMEs, for the
Surface axis. Consumers:
`glial/src/{shapes,manifest,instance,events,binder}.ts`,
`glial/dev-docs/DecisionLog.md` (GAP-2, 5, 7, 10, 14), `grip-core/src/core/share_decl.ts`,
`glade-chat/src/manifest.ts`, `gryth-wz/gryth-ui/**` (agent-verified 2026-09-21).
Decisions: root `dev-docs/DecisionLog.md`, `dev-docs/glade/GladeDeclSurface.md`,
`glade/dev-docs/GladeZones.md`, `dev-docs/glial/{GlialClientRuntime.md,
GlialFitAssessment-2026-09-15.md}`, `dev-docs/PackageExtractionPlan.md`,
`gwz-dev/dev-docs/AgentProcessRules.md` (L1-07..09, L1-17..19).
Round-1 reports: `dev-docs/glade/GladeDeclReconciliation-Review{Consistency,Safety,Surface}.md`
and `-RemPlan.md`.
