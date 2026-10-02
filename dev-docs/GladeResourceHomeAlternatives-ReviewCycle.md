# Glade resource home alternatives — review-cycle record

Date: 2026-10-03.

Status: **accepted at root `cb87e4af820d5ead2ee0461ad966589bed60007f`,
Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, and Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851` after the focused
[Consistency](GladeResourceHomeAlternatives-ReviewConsistency-2.md) and
[Safety](GladeResourceHomeAlternatives-ReviewSafety-2.md) reports returned GO;
this accepts the comparison packet's fitness for an owner decision only**.

The [alternatives packet](GladeResourceHomeAlternatives.md) remains DRAFT as
a mechanism decision. No H1/H2/H3 choice, canonical-contract amendment, wire/API
freeze, implementation, dependency, deployment or runtime change is ratified.

## 1. Directive and review boundary

The owner requested an alternatives document and a two-agent review cycle.
Dynamic client creation of resources/scopes and discovery is a hard requirement.
Configured per-resource homes (H0) remain in the comparison as a baseline but
are ineligible as the general solution. Dynamic creation, cooperative movement
and automatic failover are distinct choices.

Only `dev-docs/GladeResourceHomeAlternatives.md` was the review object.
Controlling sources are listed in its §2 and the verbatim reports. Uncommitted
Glade cold-join corrections, launcher/paired-lane work, discovery review work,
scratch files, other untracked files and generated GWZ markers were excluded.
Reviewers read immutable Git objects, not moving working-tree evidence.

## 2. Process and object provenance

Applied [review-loop SKILL.md](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
and its [canonical prompt template](/Users/owebeeone/.claude/skills/review-loop/references/review-prompt-template.md).
The GWZ program-specific checkpoint/process documents are absent in this
Glade root; packet §8 and this ledger bind the skill to this document task.

The lane owner drafted the loose decision brief. Two fresh-context agents,
`/root/home_consistency` and `/root/home_safety`, inherited the lane owner's
model and effort. They ran concurrently, read-only and peer-blind on the
Consistency and Safety axes. No user-facing interface was frozen; no third
Surface axis was required. Each reviewer verified the exact source pins and
object blob at the start and end, and returned a complete report. Reports
are filed verbatim.

The same agents performed a bounded focused verification after the wording
correction. There was no architecture, interface or guarantee change requiring
fresh reviewers. Neither saw the other's current-round report.

| Object checkpoint | Root SHA | Object Git blob | Document SHA-256 |
| --- | --- | --- | --- |
| Round 1 draft | `c79c73ca6afc371628727c0ca125eb55fd9fe41a` | `b23ce879fc2d9b7f9012bce7909b931b9c9dc63a` | `0fa13a84a2fd2942eca660adb32f5c1ccdc741cc0c7aa5f2097d585194ce27e4` |
| Accepted comparison, Round 2 | `cb87e4af820d5ead2ee0461ad966589bed60007f` | `a34acb46b27beb164ac67194fae1227905228033` | `45f635b9db055ccd010ec025b871e25616daa5cd4a7ec422079e8a06307790b8` |

Member pins were unchanged in both rounds: Glade
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`. Root controlling sources were
unchanged between object checkpoints. These are immutable review revisions,
not a requirement that unrelated live work stop.

| Process input | SHA-256 |
| --- | --- |
| review-loop skill | `bbe0c21347c8961429d4304f334741b45f170da5e64c0982f67d9d4976797d0f` |
| canonical prompt template | `c2f5eb15549609e1972ef41ba73a8f1a8707bf956b2ef82f3784ef98417c24a9` |
| [Round 1 Consistency prompt](resource-home-review/prompts/Round1-Consistency.txt) | `86dcd6e448a9f70fd41b605a97f3aa601e9cfccbb37524485dab3b4a11e94f65` |
| [Round 1 Safety prompt](resource-home-review/prompts/Round1-Safety.txt) | `a39ed4b84c9e051ef44e318f98003f3ff8f847220004d5606c2c4407161b0b7e` |
| [Round 2 Consistency prompt](resource-home-review/prompts/Round2-Consistency.txt) | `79b54545ed40351789d8939d357422b8add397707236d29248e4f2edacec6e05` |
| [Round 2 Safety prompt](resource-home-review/prompts/Round2-Safety.txt) | `820ad0dc2e119857d91e83f441c7d3185530f4d2c3951dca59ee75ec686909e9` |

Round 1 prompts were generated from the canonical template with the exact
tuple, local scope and axis bindings. Round 2 used those generated prompts
with the revised root pin, report filename and focused counterexample mandate.

## 3. Verdict merge and finding disposition

| Round | Axis/report | Verdict | P0/P1/P2 | P3 |
| --- | --- | --- | --- | --- |
| 1 | [Consistency](GladeResourceHomeAlternatives-ReviewConsistency.md) | GO | 0/0/0 | 1 |
| 1 | [Safety](GladeResourceHomeAlternatives-ReviewSafety.md) | GO | 0/0/0 | 0 |
| 2, focused amendment | [Consistency](GladeResourceHomeAlternatives-ReviewConsistency-2.md) | GO | 0/0/0 | 0 open; P3-1 closed |
| 2, focused amendment | [Safety](GladeResourceHomeAlternatives-ReviewSafety-2.md) | GO | 0/0/0 | 0 |

There were no blocking findings or NO-GO verdicts. P3-1 was optional bounded
documentation cleanup; it did not become a separate remediation package.
It was nevertheless corrected and independently rechecked on both axes so
the final bytes have two GO verdicts at one tuple. There was one focused
correction round, zero new architectural root causes and no open P3s.

| Finding | Disposition and closure |
| --- | --- |
| Consistency P3-1: record convergence alone does not guarantee one live routing answer | Corrected §2's evidence row and §5's shorthand in one patch. Equal-epoch ranking agrees for identical eligible live sets, but reader-relative expiry may change eligibility. The originating reviewer verified the original A/B counterexample on the corrected document and pinned folds; Safety independently retraced it. Closed by the Round 2 reports. |

The counterexample is A (lower ID, epoch 1, expiry 10000) and B (higher ID,
epoch 1, expiry 20000), with identical records on both readers. At time 9000
A wins; at 11000 only B is live. The revised text permits that divergence
without treating either routing answer as partition-safe ownership acquisition.
This was a source trace, not an executed implementation test.

There was no independent blind convergence on a defect: only Consistency
raised P3-1 in Round 1. Both reviewers independently agreed on the central
decision distinction: routing, exclusive authority and acknowledged-data
readiness require separate guarantees. Round 2's shared counterexample was
explicitly supplied in the mandate, so its agreement is closure evidence,
not discovery-phase blind convergence.

## 4. Independent recommendations and remaining decision

Both reviewers condition H1 on accepting write unavailability while the home
is unreachable. Both condition H3 on a requirement for automatic recovery of
the same identity after uncooperative home failure, with quorum deployment,
stale-home exclusion and a separate data-preservation guarantee.

Their staging advice differs slightly. Safety prefers establishing H1's
creation/binding/enforcement first and adding H2 only for a concrete movement
need. Consistency recommends choosing H2 directly if planned identity-preserving
moves are already required, to avoid baking immutability into an H1 design
and then migrating it. Neither treats dynamic creation alone as a reason to
adopt H3 or treats H1/H2 as free of serialization obligations.

The owner decision MUST record:

1. Whether an unreachable or permanently lost home may stop authoritative
   writes, or automatic failover preserving identity is required now.
2. Placement granularity and canonical identity/naming/alias authority.
3. The acknowledgement guarantee and tolerated process, machine, storage and
   network failures; ownership quorum alone does not preserve acknowledged data.
4. The actual fencing boundary for settings, physical working copies and
   external supplier effects.
5. Permission-freshness, voter/operator trust, membership and deployment
   assumptions, including the intended one/two/three-node behavior.

After selection, its design MUST identify exact amendments to controlling
clauses and undergo its own contract/design review. The downstream inventory
includes the discovery matrix, WorkspaceDirectory §4, DiscoveryModel's
epoch/takeover prose, Authz's local-decision constraints, and the management/data
seams in GDL-036/037/038. Creation closure tests SHOULD explicitly crash between
durable naming, host creation and advertisement and lose/cancel creation replies.
These are carried-forward design obligations, not accepted implementations.

## 5. Verification and landing scope

The lane owner checked local Markdown link targets, prompt hashes, document
hash/blob equality with the accepted checkpoint and whitespace via
`git diff --check`. Reviewers traced the shared adversarial scenarios in the
packet; exact source ranges and inspection commands are in their reports.
No implementation tests, builds or runtime checks were run for this document
change. No source, API, dependency or test-selection change was made.

Scoped local GWZ commits settle the draft, qualify the source-summary wording
and file the evidence. Generated GWZ commit markers are tool-managed metadata.
No push, merge or runtime restart is part of this task. Existing unrelated
working-tree changes remain outside the landing scope.
