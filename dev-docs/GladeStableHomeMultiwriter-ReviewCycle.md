# H1 design and multiwriter evaluation — review-cycle record

Date: 2026-10-03.

Status: **accepted at root `f9f020b800ba33cabb0c036ee213b15f6e1b33d7`,
Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, and Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851` after
[Consistency](GladeStableHomeMultiwriter-ReviewConsistency-2.md) and
[Safety](GladeStableHomeMultiwriter-ReviewSafety-2.md) returned GO;
this accepts the semantic design/evaluation scope for the next contract/proof
tranche only**.

The owner authorized proceeding with the recommended design and suggested
multiwriter evaluation. [GDL-050](DecisionLog.md) records that direction.
The [H1 design](GladeStableHomeDesign.md) and
[multiwriter evaluation](GladeMultiwriterSettingsEvaluation.md) remain DRAFT
with respect to executable contracts, wire/API ratification and deployment.
No implementation, dependency, migration, push, merge or runtime restart was made.

## 1. Outcome and next tranche

H1 uses share-level placement: one fixed home serializes creation, canonical
naming/retries and admission, while replicated records advertise availability.
New independent roots choose eligible homes dynamically. Existing-scope creation
may wait for its home; absence of local knowledge does not create another root.
Identity metadata has a proposed OS/power-loss durable atomic transaction,
separate from the present weaker application-data `Ok`. Retirement, conditional
default initialization, actual effect enforcement and migration exclusion are
explicit design obligations. No automatic takeover is promised.

Multiwriter is evaluated only for the five appearance preferences. Whole-document
LWW is the evidenced existing-shape baseline; per-field LWW is the conditional
preferred profile for independent fields when same-field concurrent loss is
acceptable. A conflict-exposing register is the alternative when that loss is
unacceptable. Wallpaper/toggle coupling requires an atomic group/compound
register if their schema demands it. Neither new preference profile is claimed
supported by the current text CRDT adapter or reserved `message` enum.

The complementary boundary is initial identity versus later admission. A future
multiwriter appearance profile still needs one canonical binding/bootstrap and
explicit activation; it cannot mint duplicate roots, borrow W4's two-node
receipt, or apply preference convergence to grants, SWMR or external effects.
Layout remains local under the retained historical owner ruling.

The next contract/proof tranche MUST:

1. Record the actual allocation in the Gyld declaration and supply compiling
   consumer/conformance witnesses before implementation, per BuildEntry.
2. Define H1's atomic ledger, outcome lookup, declaration/admission and lifecycle
   contracts; settle exact taut/authz/version profiles and canonical amendments.
3. Write RED-first SH-001..010 creation, crash, ordering, identity, permission,
   capacity and migration traces; concrete storage/locking/crypto adapters then
   prove their actual guarantees and affected-consumer integration.
4. Build the bounded whole-value appearance baseline and compare the proposed
   per-field/MV profiles through MW-001..012 deterministic cases. Settle scope,
   atomic field groups, conflict/reset UX, disconnected validity/receipt policy,
   storage guarantee, bounds and rejoin floors before activation.

An optional owner preference question was submitted for deterministic same-field
winner versus visible conflicts. No reply was recorded at this landing;
the evaluation's recommendation remains explicitly conditional. No silence is
interpreted as selection. Field independence and the remaining security/storage
choices likewise remain gates.

## 2. Independent review and verdict merge

Applied [review-loop SKILL.md](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
and its [canonical template](/Users/owebeeone/.claude/skills/review-loop/references/review-prompt-template.md).
The main lane owner drafted H1 and the direction note; a bounded drafter
`/root/multiwriter_evaluation_draft` wrote only the evaluation, without Git or
runtime mutations. Its pinned evidence was inspected by the lane owner.

Fresh agents `/root/stable_mw_consistency` and `/root/stable_mw_safety` inherited
the lane-owner model/effort and attacked the same committed packet concurrently,
read-only and peer-blind. They verified three revision pins and three reviewed
blobs at start and end. Reports are filed verbatim. No public surface was
frozen; exactly two document axes were used.

| Round | Report | Verdict | Findings/disposition |
| --- | --- | --- | --- |
| 1 | [Consistency](GladeStableHomeMultiwriter-ReviewConsistency.md) | GO | Zero P0/P1/P2; one P3 source-pointer defect |
| 1 | [Safety](GladeStableHomeMultiwriter-ReviewSafety.md) | GO | Zero P0–P3 |
| 2, focused citation check | [Consistency](GladeStableHomeMultiwriter-ReviewConsistency-2.md) | GO | P3-1 closed; zero remaining findings |
| 2, focused citation check | [Safety](GladeStableHomeMultiwriter-ReviewSafety-2.md) | GO | GO retained; zero new findings |

Consistency P3-1: H1 §8 attributed takeover to DiscoveryModel §3 rather than
§7's epoch-fence/takeover test. One table-row correction now cites both §3's
routing/home roles and §7's takeover expectation, requiring an H1 rejection
counterpart while preserving the legacy scenario for unactivated shares.
The originating reviewer re-traced the corrected pointer; Safety independently
traced the attempted higher-epoch takeover against the unchanged admission gate.
Closed. This was optional P3 cleanup, not a blocking remediation package.

One full dual round and one focused correction round completed. There were
no NO-GO verdicts, no blocking findings, no new architectural root causes and
no open P3s. There was no blind convergence on a defect: only Consistency found
P3-1. Both axes independently found H1 and the appearance-only evaluation
compatible. Supplied Round 2 counterexamples are closure evidence, not blind
defect discovery.

## 3. Exact inputs and provenance

| Input | Round 1 | Accepted final review |
| --- | --- | --- |
| Root revision | `d6f994e1e4d3c4458be9b577336fe849ae628e2c` | `f9f020b800ba33cabb0c036ee213b15f6e1b33d7` |
| H1 Git blob | `e1cdb3660c1d7c993880d19120b829c4ab5af407` | `8c0dc56cf1f2b6b5e674c6206a7cdcfa6c101c09` |
| H1 SHA-256 | `a28b506de2355b0c3a915c46ff70094a01e8686c0983428252e0d4b5146245c4` | `c7c770f740b1abe9419408ce63bd9baa7aedddcd09777614aad2e24f4e32689e` |
| Multiwriter Git blob | `83784016d3e418ddf4fab4a087aa1ccc9067adb4` | unchanged |
| Multiwriter SHA-256 | `57b43a1130c2f09c7bbdc83cbd38a8d94c0cea1c1c28b7bf0c6f9d5416d1738c` | unchanged |
| DecisionLog Git blob | `c8ce426b3aa125457f28f778ac212424a5c03873` | unchanged |
| DecisionLog SHA-256 | `a091cef443c727531fd7d77b08ae5b3fb1fc8b4b9211492429365c2df7023ec5` | unchanged |

Both rounds use Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b` and
Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Root controlling
sources were unchanged between review checkpoints. External Gyld capture/model
were qualified allocation context, not ratified sources or changed objects.
Uncommitted CJ/launcher/discovery/scratch work and generated GWZ metadata were
explicitly excluded from review and landing scope.

| Process input | SHA-256 |
| --- | --- |
| review-loop skill | `bbe0c21347c8961429d4304f334741b45f170da5e64c0982f67d9d4976797d0f` |
| canonical template | `c2f5eb15549609e1972ef41ba73a8f1a8707bf956b2ef82f3784ef98417c24a9` |
| [Round 1 Consistency prompt](stable-home-multiwriter-review/prompts/Round1-Consistency.txt) | `4d4ad87e88f24af32cab3bc3e1ca330184d7a62bdf49d3f5217865c26cd6d224` |
| [Round 1 Safety prompt](stable-home-multiwriter-review/prompts/Round1-Safety.txt) | `80724bab8344d8b53e710dd72aa7beb72adbbed4a1d818361391b6ac615224c2` |
| [Round 2 Consistency prompt](stable-home-multiwriter-review/prompts/Round2-Consistency.txt) | `385ae16b08ed3cc2cbceda9d4f0c9751c293af0b4fa8baa40c2ea824589ec1f2` |
| [Round 2 Safety prompt](stable-home-multiwriter-review/prompts/Round2-Safety.txt) | `bdd4c1eb1d957f24f38d6af62dbc6b78469e43f9fbe3d09a6355d8d169abca36` |

Prompts were generated from the canonical template with exact tuple, source
graph and axis binding. Round 2 changed the root/H1 pin and supplied the
bounded citation mandate; same reviewers retained context because no architecture,
interface, scope or guarantee changed. Glade has no GWZ-program-specific
checkpoint/process files; H1 §9 and this ledger bind the skill locally.

## 4. Verification and limits

The lane owner checked local Markdown targets, whitespace, prompt hashes and
exact final document bytes against the reviewed blobs. Reviewers performed
source traces only. The primary register and logical-clock sources support
merge/order distinctions, not Glade correctness; direct CRDT PDF retrieval
timed out for the Consistency reviewer, whose report qualifies that limit.
No full-paper metadata-collection verification is asserted by this ledger.

No implementation tests or builds ran for this document task. The SH/MW
scenarios are future RED-first obligations, not executed test evidence.
Real metadata durability, origin signing, authority propagation, profile
convergence, bounded retention, effect enforcement and migration remain
unproven until their named contract and adapter gates pass.

Scoped local GWZ commits settle the drafts, correct the citation and file
the evidence. No source code, dependency, test selection, process-global
allowlist or legacy contract bytes were changed. Existing unrelated work is
preserved. The direction note records the owner instruction; it does not
silently replace the canonical clauses listed for later amendment.
