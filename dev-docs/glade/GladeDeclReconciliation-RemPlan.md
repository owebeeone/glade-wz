# GladeDeclReconciliation — Remediation plan, round 1

Date: 2026-09-21. Lane owner's plan. Object: `dev-docs/glade/GladeDeclReconciliation.md`
reviewed at glade-wz root `b132b7e6d365`; reports filed verbatim at `879de50`:
`-ReviewConsistency.md` (NO-GO, 5 P2, 6 P3), `-ReviewSafety.md` (NO-GO, 2 P1, 3 P2, 3 P3),
`-ReviewSurface.md` (NO-GO, 5 P2, 5 P3). All three reviewers pre-committed to GO on a
revision resolving their blocking findings as specified.

Rules of this round: ONE patch to the object, no series. Every blocking finding has
exactly one disposition and one closure test below. The drafter closes nothing; the
reviewer who raised a finding verifies its original counterexample on the revision.
Finding IDs are prefixed by axis: `CON-`, `SAF-`, `SUR-`.

The object is a DRAFT design. Where a reviewer's correction requires a CHOICE, the
revision states the options, their true costs and a recommendation, and adds the
choice to the owner's rulings. It does not make the choice. A deferral covers the
outcome only; every option offered must be safe and fully laid out.

## Blind convergence (recorded because it is the strongest signal this process has)

| Defect | Axes that found it independently |
|---|---|
| A changed binding line has no defined effect on a declaration already registered: `register` diffs on payload bytes, `dir.bindings` has no fold and no retract, so the amendment's own rewrite appends contradictory duplicates | SAF-P1-2 (traced `appdecl.rs:264`) and SUR-P2-5 (could not tell whether a migration took effect, reading no code) |
| Node-side validation of zone and retention is neither settled nor safe as drafted | CON-P2-4 (pre-empts open rulings, leans on unratified GDL-039), SUR-P2-1 (enforced while undocumented and contradicted by the file's own comment), SAF-P2-3 (un-migrated files become a boot failure behind a green gate) |
| The rendering lockstep is ungated | CON-P2-2 (the §4.3 commands write to the wrong directories and the TypeScript gate stays green over stale `api.ts`) and SAF-P2-5 (the ts and py gates read local corpus copies that `build.py --check` never compares) |
| `BindingDecl.profile` as drafted is wrong three ways | SAF-P1-1 (bytes), SUR-P2-3 (spends the last positional slot), SUR-P2-4 (no legal values, no omission semantics, no enforcement point) |
| Un-migrated app files fail with no way to tell old from new | SUR-P2-2 (unchanged `glade-app v0` header) and SAF-P2-3 (landing order) |

## Contested between axes, adjudicated

`private` and the `self:<id>` key. SAF-P2-4 states that no production code produces the
key and that `private` would be frozen as an unimplemented guarantee. CON-P2-1 shows
that `glade/grip-share/src/manifest.ts` (`manifestScope`) with `glade/demo/src/manifest.ts`
does produce `self:{self}`, and that row 16's "nothing client-side produces that key"
is false. Both are right about different binders: the grip-share binder honours
`private` on the demo path; glial, which gryth-ui mounts through, passes `Fill.zone`
into a local instance key only, so a surface declared `private` there converges in
the commons partition. Disposition: accept both. Row 16 is restated as DIVERGENT
BETWEEN BINDERS and leaves the "agrees" class; the caveat SAF-P2-4 requires is
written scoped to glial; the consumer survey and R1's costs take in grip-share.

## Blocking findings

| ID | Disposition | Closure test (verified by the raising reviewer) |
|---|---|---|
| SAF-P1-1 | ACCEPT. State in §1 and §4.2 that taut `optional` is nullable and ALWAYS emitted (`codec.py:12,88`; `edge/binding-account-commons` opens `a7` and carries `04 f6`). Rewrite R4: (a) a field on `BindingDecl` changes 11 of 26 vectors and v1 is then not a byte superset; (b) a separate message keyed by glade id, leaving `BindingDecl` untouched; (c) defer past the freeze. Strike R3's third reason and every "strict superset" claim unless R4 is not (a). | §4.2 specifies a `build.py --compat` mode asserting `v1[n].cbor == v0[n].cbor` for every `n` in v0, and no superset claim survives anywhere in the text unless that gate is stated as its proof. |
| SAF-P1-2 | ACCEPT, merged with SUR-P2-5 into a new ruling R9 (supersede and retract for `dir.bindings`). The revision states what an existing data directory does on the first boot after the amendment, under every option: (a) consumers and `register` read a fold by `glade_id`, a changed binding supersedes, an absent one is retracted by an explicit record, (b) no stored token is ever rewritten, the file format keeps its spellings and the node maps them to the contract vocabulary at the boundary, (c) duplicates accepted with a stated fold rule. §4.6 is corrected: the tokens ARE read, by the byte diff. §4.4 attributes the durable reach to the token edit itself, under every R2 option, not only to R4. Recommendation: (b) for spellings, which cuts the rewrite from 15 lines to the 2 `windowed` lines, plus (a) for those. | §4.7 gains a `glade-node` test that registers the PRE-amendment parse then the POST-amendment parse and asserts the intended `Registered{appended, unchanged}` and resulting `dir.bindings`; the text names the rule that test encodes. |
| SUR-P2-5 | ACCEPT, merged into R9 above. The format documentation gains the rule for a changed line and for a deleted line, stated before any line is migrated. | A reader of the format page can predict which declaration is live after a token change, and whether a surface is gone after a line is deleted. |
| CON-P2-4 | ACCEPT. Split rows 18 and 31. SETTLED keeps only what a ratified entry or a unanimous implemented fact decides (`crdt` declarable; the enum member's identifier spelling). "Validate token 4" moves under R1, "validate token 5" under R2, each with the sub-choice hard error or warn for one release. Delete "that part is settled" at lines 270-272. Restate the §2 arithmetic. | Every SETTLED row satisfies: a ratified DecisionLog entry decides it, or it changes no input the node accepts today. The header totals match a recount. |
| SUR-P2-1 | ACCEPT. The revision requires, in the same wave as any zone validation: the meaning of `commons` and of `private`, the rule for choosing, the behaviour when absent, whether a mount overrides an authored zone, and the rewrite of "the mount fills domain/zone/key" in both homes of `grazel-app.glade` and at `GladeGrazelAttachNotes.md:49`. | No page tells an author the mount fills a token the parser requires them to type; the format page answers which zone to write for a per-principal surface. |
| SAF-P2-3 | ACCEPT. §4.4 states the landing order and why: app files first, in all four repositories, node validation last, because an old node accepts any spelling and a new node refuses the old one. §4.7 names `cargo test -p grazel` and all four repositories that hold app files. The hard-error or warn choice is the sub-choice recorded under CON-P2-4. | §4.7 lists the grazel integration suite and a `glade-node` unit test asserting the line-numbered diagnostic for `from-cursor`, `windowed` and an unknown zone. |
| SUR-P2-2 | ACCEPT as new ruling R10: (a) the validated grammar is `glade-app v1`, `v0` files still load with a deprecation warning naming the replacement; (b) `v0` is redefined in place, stated explicitly, with a migration note. Recommendation (a). | A file headed `glade-app v0` with `from-cursor` either loads with a warning naming the replacement or fails naming both the replacement and the version in which it changed. |
| SUR-P2-3 | ACCEPT as new ruling R11, with SUR-P2-4: (a) a keyword tail after the five positional tokens carries `ttl`'s duration and the shape profile, leaving the grammar open; (b) `ttl` is held reserved and unauthorable until it has a parameter slot. The contract's `Retention` record versus the file's bare `RetentionPolicy` token is named as a third declaration-only gap. Recommendation (a). | An app file can express a TTL of 10 minutes and one of 1 hour as different records, and adding a further option changes the meaning of no existing line. |
| SUR-P2-4 | ACCEPT, under R11 and R4: legal values of the profile, what omission means for EACH shape in `BINDING_SHAPES`, enforcement at parse with a line number, the arity diagnostic at `appdecl.rs:113` updated, and a name that does not collide with the node's `--profile` and the supplier's compatibility profile. | A five-token `crdt` line either yields a working surface or is refused at parse naming the missing token and its legal values. |
| SAF-P2-4 | ACCEPT, as adjudicated above. `ZoneKind.private` gets the R5(b) treatment, scoped: honoured by the grip-share binder, NOT by glial mounts today; the schema comment, the README and a new OpenNote say so. Row 16 leaves "agrees". | The README caveat sits beside the published enum, and the revision records as a pre-freeze item a glial test that a `zone: "private"` mount produces a `self:` wire key or throws. |
| CON-P2-1 | ACCEPT. Add `glade/grip-share/src/manifest.ts` and `glade/demo/src/manifest.ts` to the consumer column and the appendix; restate rows 12, 13, 15, 16, 30; add a demo and grip-share column to R1's options, with (c) and (d) carrying the cost of breaking `manifestScope`, reconciled against §4.6. R1(b) counts the mapping that already exists. | R1's option table has a cost in every repository each option touches, and §4.6's running-demo clause is not contradicted by any option's stated cost. |
| CON-P2-2 | ACCEPT, merged with SAF-P2-5. §4.3 is replaced by the rendering READMEs' form: generate to a scratch directory, copy each language's files into its `src/`, then the ir and corpus copies for ts and py, then `build.py`. §4.7 says the TypeScript corpus gate does not exercise `api.ts` and gains a row that greps for `atom` in all three generated APIs. | No `typescript/`, `rust/` or `python/` directory appears in a rendering after regeneration, and the `atom` grep passes in all three. |
| SAF-P2-5 | ACCEPT. `build.py --check`'s artefact list is extended to the four rendering copies; A4 is restated as five documented forms of which three are correct (CON-P3-6); `CONTRACT_VERSION` is made checkable or is no longer called a pin. | `build.py --check` reports STALE when any one rendering copy diverges. |
| CON-P2-3 | ACCEPT. §4.2's four-item list is replaced by the full enumerated set of `decl.v0` references (20 hits), split into "must change for a gate to pass" (first `glade-decl-py/pyproject.toml:20`) and "must change for the text to be true". | After the amendment `git grep 'decl\.v0'` over the four contract repositories and root `dev-docs` is empty, and the Python gate passes from a built wheel. |
| CON-P2-5 | ACCEPT. An audit row records that `glade-decl/dev-docs/DeclSurface.md:27,30,32` is pre-GDL-041; §4 gains a step for it; R8 states that the root `dev-docs/glade/GladeDeclSurface.md` is controlling and the repository copy becomes a mirror with a banner and a drift check. | The two documents diff empty, or the mirror carries its banner and `build.py --check` fails on drift. |

## Non-blocking findings, all accepted as riders on the same patch

`CON-P3-1` re-ground §1 on L1-09 and the corpus oracle, quote every process clause
beside its number. `CON-P3-2` correct the eight file:line citations and run a
mechanical pass over every citation. `CON-P3-3` fix the three miscounts with the
command beside each. `CON-P3-4` give each ruling an exclusive row list and state the
precedence between rulings and SETTLED rows; fix R4's reference from R3 to row 8.
`CON-P3-5` give rows 33, A3 and A7 a §4 step or say why they ride separately.
`CON-P3-6` restate A4. `SAF-P3-6` any delete option also reserves the tag and name
and sets `next_id`. `SAF-P3-7` R3(b) changes the synth vectors regardless, because
synth selects by member index. `SAF-P3-8` add an `atom` curated vector and a
`build.py` assertion that every `Shape` member has one. `SUR-P3-1` a members table
with a gloss per enum value on the contract's front page, and the grammar on a page
presented as user-facing. `SUR-P3-2` removed and renamed tokens stay
recognised-but-refused with a diagnostic naming the replacement, in the shape of
`appdecl.rs:121`. `SUR-P3-3` the joining-character convention, folded into R9's
recommendation to keep the file format's hyphen. `SUR-P3-4` correct
`GladeGrazelAttachNotes.md:98` and give `atom`, `message`, `window` a redirect or a
reserved note. `SUR-P3-5` say on the front page whether an app-file glade id is
authored or derived. Also, from the Safety report's residual risks: one sentence in
§4 that nothing is published until every row of §4.7 is green on a settled tree.

## Standing precondition, not a finding against the object

`glade-decl/corpus/build.py --check` is red today because `glade-decl-rs` carries
uncommitted rustfmt-only edits to `src/api.rs` and `src/vectors.rs` (reproduced by
the Consistency reviewer). L1-17 requires a settled tree before an ACCEPTANCE review.
This draft-stage review named it out of scope. It is the owner's working-tree state
and the owner's call to commit or discard; the freeze cannot start while it is red.

## Round accounting

Round 1 of at most 2. No reviewer classified any finding as an architectural root
cause of the OBJECT (the document); the architectural facts found (no fold for
`dir.bindings`, glial not producing the private key) are properties of the system the
document describes, which the revision must state truthfully rather than fix.
Re-verdict by the same three reviewers, context intact, against the revised tuple.

## Status: held by the owner, 2026-09-21

The single patch was drafted as revision 2 of the object and is committed with this
plan. The round-1 re-verdicts were NOT dispatched: the owner held the loop ("hold off
on the reviews for this one"). No finding is closed; revision 1's three NO-GO
verdicts are the last on record; round accounting stays at round 1 of at most 2, with
the re-verdict outstanding.

Facts the resuming lane owner needs:
- Tuple for the re-verdict: glade-wz root at this commit; glade-decl `d671f10` and
  glade-decl-rs `21eefa1` (both moved by the owner's A2 ruling: the rustfmt edits were
  discarded and `build.py` now formats the Rust it generates, so the drift gate is
  green); every other repository as in the round-1 tuple.
- The drafter departed from two corrections as specified and said so: SAF-P1-1 asked
  the revision to CHOOSE a profile treatment, and as a draft it lays out R4(a)/(b)/(c)
  with a recommendation instead; CON-P2-4's zone validation became an R1 sub-choice
  rather than a twelfth ruling. The raising reviewers must rule on both.
- The drafter reported three facts no reviewer had: 60 vitest files, not 58; a fourth
  key convention in `glade/grip-share/src/manifest.ts:67`; and five app files in four
  repositories, not five.
- The reviewers ran on a tier one below the session's at the owner's standing
  instruction to conserve quota. An interface freeze calls for the strongest tier, so
  the resuming owner should decide the tier before dispatching.
