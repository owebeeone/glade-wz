# GladeDeclReconciliation — Remediation plan, round 3

Date: 2026-09-23. Lane owner's plan. Object: `dev-docs/glade/GladeDeclReconciliation.md`
revision 3 at glade-wz root `c26211d`, re-verdicted by the three round-2 reviewers
(contexts intact) at that tuple. Reports filed verbatim at `2bafc02`:
`-ReviewConsistency-3.md` (**GO**, 0 P2, 1 P3; all five round-2 findings CLOSED),
`-ReviewSurface-3.md` (**GO**, 0 P2, 4 P3; all nine round-2 findings CLOSED),
`-ReviewSafety-3.md` (**NO-GO**, 1 P2, 0 P3; all four round-2 findings CLOSED; pre-commit
to GO on a revision resolving SAF-P2-12). Every report states in terms that none of its
findings, in any round, is an architectural root cause of the object.

**Round accounting.** Remediation rounds 1 and 2 are spent. The process permits a third
round confined to non-architectural corrections; any architectural root cause found in
it triggers the cap and stops the lane. This round is that third round. It contains one
blocking finding — three integers and a missing derivation in one table — and the P3
riders the three reviewers asked to have folded in. Nothing else.

Rules unchanged: ONE patch to the object, no series; the drafter closes nothing; the
raising reviewer verifies; where a correction needs a CHOICE the revision lays it out and
does not choose.

## Blocking finding

| ID | Disposition | Closure test (verified by the raising reviewer) |
|---|---|---|
| SAF-P2-12 | ACCEPT, all five items as the report specifies. (1) R9 row **(b1)**'s durable-cost cell becomes the R2-conditional floor: **2** under R2(a)/(c)/(d) — the two `windowed` lines, which change for R2's reason, not R9's — and **0** under R2(b); the genuine benefit stays stated (no stored byte changes for the 13; one stored spelling). (2) Row **(b2)** becomes **15** under R2(a)/(c)/(d) (13 normalised + 2 `windowed`) and **13** under R2(b). (3) Row **(a)**'s "under every R2 option" becomes "under every R2 option except (b), which keeps `windowed`". (4) §4.4 bullet 6's closing sentence and §4.7(b) row 9's assertion list take the corrected numbers, stated the same conditional way. (5) The cost is stated as a derivation, not as integers: *appends on the first boot = (lines whose token 5 changes in the file, which R2's answer fixes) + (lines whose stored token the node rewrites, which R9's shape fixes)*, with the census (13 `from-cursor`, 2 `windowed`, 28 lines, at the SHA it was taken) as its input, and one pair worked from census to number. R9's recommendation clause ("b1 if the durable cost is what matters") is re-read against the corrected spread and kept or re-argued; §4.4 bullet 5's "under b1 no record moves" becomes "under b1 only the two `windowed` records move", so the page owes those two surfaces the same answer. | §4.7(b) row 9's assertion list, read against R9's column, agrees cell by cell for each (R2 answer × R9 shape) pair; at least one pair is worked in the text from the census to the number; under {R2(a), R9(b1)} both read **2**, under {R2(a), R9(b2)} both read **15**. |

## Riders, all accepted on the same patch

- `CON-P3-10` §4.2's deletion bullet: "nine" → "ten", saying what it counts (designated
  keys, `Retention` counted once as a key), or drop the count; extend §1's numeric
  assertion to cardinalities expanded through a group name.
- `SUR-P3-10` §4.4 bullet 12's edit of `GladeGrazelAttachNotes.md:29-36` adds
  `workspace <share> <name>` to the published grammar block, with one clause on what it
  does (the line that makes a declared surface routable).
- `SUR-P3-11` §4.4 "Token 5 — the retention", the "which to write" bullet, indexed per
  `BINDING_SHAPES` member as bullet 4's omission rule is: state the retention for `swmr`
  and for `crdt`, or state in one clause that the shape does not determine the retention
  and what does; the answer for `crdt` must not be contradicted by `latest`'s gloss.
- `SUR-P3-12` one sentence in R9 that its options govern `dir.bindings` only, and one in
  §4.4 bullet 5 on what a deleted `service` or `workspace` line does today (nothing
  retracts; a retired exchange stays routable), with symmetry offered as a new option
  under R9 if the owner wants it, priced there.
- `SUR-P3-13` "a four-token line is an arity refusal" in the token-5 omission bullet, or
  one stated counting convention used throughout §4.4.
- From the Safety report's observations: §4.7(c) row 14's "row 16" reads "§2d row 16";
  §4.4 step 4 repeats bullet 6's dual-maintenance rule for the header move.
- From the Consistency report's residuals: §4.4's census command names its working
  directory (inside `glade-wz`; a run from the workzones' parent also matches a third
  checkout and returns six); §4.2's relabelling question cross-references §4.1 item 1,
  which already answers it (keys survive, comments change); a §4.7(b) row on **R8** for
  the amendment of `GladeDeclSurface.md` landing; the Appendix's re-pinning discipline
  extends to gwz-dev (`9c0008870ecd`; `AgentProcessRules.md` byte-unchanged since
  `ff431743cc4c`).
- Not a rider, recorded: the Surface reviewer's standing residual that the page an author
  reads still does not exist — the grammar "should move to, or be mirrored in, a page
  presented as user-facing" with no page named. It has been recorded in every round and
  is not made a disposition here; it is the next increment's value, not this freeze's.

## What the revision may not do

Change any ruling's outcome or recommendation except R9's sub-recommendation if the
corrected spread no longer supports its wording. Change any file other than the object.
Weaken a gate to make it green. Delete a corrected false claim rather than marking it.

## Re-verdicts

Safety re-verdicts SAF-P2-12 on the revision-4 tuple with a closure table. Consistency
and Surface re-verdict on the diff `c26211d..<revision 4>` restricted to their sections
and their riders, so that GO from all three axes stands on the same revision. Filed as
`-Review<Axis>-4.md`.

## Status: accepted, 2026-09-23

Revision 4 (`3b60234`) received GO from all three axes, filed verbatim at `00eac99`:
Consistency (1 P3: CON-P3-11, R9's option (s) has no §4.7 row), Safety (1 P3:
SAF-P3-13, row 9's assertion is in tree-wide units), Surface (no new finding). No
reviewer classified any finding in any round as architectural, so the cap was never
engaged; three remediation rounds were used, the third confined to non-architectural
corrections as the process permits. The object is **accepted at `3b60234` after the
three `-Review<Axis>-4.md` reports reported GO; this accepts the assessment and the
amendment shape only.** The three open P3s and the Surface residual ride the
amendment's own commit (slice plan Phase 2), not another revision of this document.
Round accounting closes: remediation round 1 (`b132b7e` → `1defe3b`), round 2
(`1defe3b` → `c26211d`), round 3 (`c26211d` → `3b60234`). Across the four review
rounds 53 numbered findings were raised (29 on revision 1, 16 on revision 2, 6 on
revision 3, 2 on revision 4); 51 are closed by their raising reviewers and 2 stand
open at P3 (CON-P3-11, SAF-P3-13), beside the un-numbered Surface residual.
