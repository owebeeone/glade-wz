# Glade resource home alternatives — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeResourceHomeAlternatives.md` at root commit `cb87e4af820d5ead2ee0461ad966589bed60007f`; DRAFT decision packet dated 2026-10-03. Round 2 focused amendment verification.

**Baseline:** Root `cb87e4af820d5ead2ee0461ad966589bed60007f`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Sources were inspected with `git show <exact SHA>:<path>`. The object diff was inspected against prior root `c79c73ca6afc371628727c0ca125eb55fd9fe41a`.

**Date:** 2026-10-03.

**Axis:** Safety — verify that the routing-summary amendment accurately permits reader-relative divergence without weakening authority, fencing or recovery requirements. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its current report. Filed verbatim by the lane owner.

**Start verification:** All three revision pins resolved exactly. The revised object blob was `a34acb46b27beb164ac67194fae1227905228033`. The complete Round 2 prompt’s SHA-256 matched `820ad0dc2e119857d91e83f441c7d3185530f4d2c3951dca59ee75ec686909e9`.

**Verdict: GO** — no prior Safety findings and no new P0, P1, P2 or P3 findings. The revised packet remains fit for an owner decision. This is a focused amendment verdict, not implementation acceptance or architecture ratification.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on revised object | Status |
| --- | --- | --- | --- |
| None — Safety Round 1 | Prior Safety verdict was GO with no findings | Checked the exact amendment and its effect on the prior Safety invariants | No Safety finding requires closure; GO retained |

The routing counterexample supplied by the Round 2 mandate was independently retraced below. No current-round report from the other reviewer was accessed.

## Changed-range analysis

The complete object diff contains two changes:

- Line 47 replaces the unconditional summary that routing answers converge once records meet with agreement for the same eligible live claims at a common evaluation time. It expressly permits different answers from identical replicated records when readers evaluate expiry differently.
- Lines 194–196 replace the “eventually convergent claim fold” shorthand with deterministic ranking over the same eligible live records and repeat the expiry qualification.

Neither change alters candidate mechanisms, owner requirements, authoritative acceptance rules, handoff ordering, acknowledgement guarantees, membership assumptions, migration gates, or HF-01 through HF-10. No architectural root cause or expansion of implementation authority was introduced.

## 0. Evidence base

The complete generated `Round2-Safety.txt` prompt was read and its hash verified before review. The review-loop skill and canonical template were retained from Round 1; the focused mandate did not require rereading the wider source graph.

The following immutable evidence was inspected:

| Source | Evidence examined |
| --- | --- |
| Root object at `cb87e4af820d5ead2ee0461ad966589bed60007f` | Exact diff from `c79c73ca6afc371628727c0ca125eb55fd9fe41a`; numbered revised document, particularly line 47, RH-05/06 at lines 75–76, and comparison/scenarios at lines 178–216 |
| Glade `node/src/registry.rs` at its pin | Reader-clock contract at lines 342–346; shared comparator at lines 754–762; routing implementation at lines 769–776; existing read-time-expiry and common-ranking test source at lines 1063–1106 |
| Glade `node/src/mesh.rs` at its pin | Route module and `who_serves` re-export at lines 47–52 |
| Glade `node/src/mesh/route.rs` at its pin | Reader-clock routing description at lines 27–28; store-backed `who_serves` implementation at lines 389–408 |
| Root `dev-docs/glade/GladeWorkspaceDirectory.md` at revised pin | Time-free fold/read-time expiry distinction at lines 62–69 |
| Root `dev-docs/glade/GladeDiscoveryModel.md` at revised pin | Time-free fold and time-dependent projection distinction at lines 130–138 |

Inspection used read-only hashing, `git rev-parse`, `git diff`, `git show`, and text filtering. No tests or builds were run, no files were written, and no Git or runtime mutations occurred.

## 2. Invariant analysis

### Identical records can produce different routing answers

Assume valid claims for the same share:

- A: lower node ID, epoch 1, expiry 10000.
- B: higher node ID, epoch 1, expiry 20000.

Both pinned implementations first require `lease_expiry_ms > now_ms`. They then rank by higher epoch and, at equal epoch, lower node ID.

| Evaluation time | Eligible live claims | Registry answer | Store-backed mesh answer |
| --- | --- | --- | --- |
| 9000 | A and B | A | A |
| 10000 | B | B | B |
| 11000 | B | B | B |
| 20000 | None | None | None |

Thus two readers with identical replicated records but evaluation times 9000 and 11000 choose A and B respectively. The revised line 47 and lines 194–196 explicitly allow that divergence. They no longer claim that record convergence alone guarantees identical routing answers.

This trace was established directly from the pinned predicates and comparator; it was not presented as an executed test.

### Identical eligible live sets still rank identically

`registry::rank_claims` compares epoch and then reverses the node-ID comparison so the lower ID wins an equal epoch. The registry uses that comparator through `max_by`; the mesh replaces its current best only when the same comparator returns greater.

For the same eligible live claims, both therefore select the same holder independently of traversal order. Different reader times can change eligibility without changing the ranking rule. The amended text preserves that distinction and agrees with the canonical separation between time-free replicated state and read-time expiry projection.

### Routing disagreement does not authorize dual authoritative admission

The amendment acknowledges routing divergence; it does not make that divergence safe acquisition or automatic takeover. Line 47 still says ranking is neither partition-safe acquisition nor data repair. Lines 192–196 retain each candidate’s serialization, fencing and data-safety eligibility gates.

The adverse sequence remains: one reader routes to A before its expiry, another routes to B after it, and both attempt writes. RH-05 requires exclusive authoritative acceptance paths; RH-06 requires permission and generation validation at the actual enforcement boundary. HF-03 and HF-04 continue to require partition/stale-claim and delayed-old-effect analysis.

The document therefore does not permit a later design to justify simultaneous authoritative acceptance by pointing to these two valid local routing answers. It also does not repair or approve the existing implementation through a source-summary correction.

### Other Safety obligations remain intact

The exact diff leaves the previously reviewed protections unchanged: concurrent creation and alias binding, partial-empty-view handling, acknowledged-history preservation, exact retry and crash recovery, permission freshness, membership transitions, legacy activation, bounded disclosure, and retirement/replay handling.

There was no changed guarantee warranting a new review of those mechanisms in this focused round.

## 3. Risks and next action

Reader-relative routing divergence remains an input to the selected design’s authority and fencing proof. The amendment correctly exposes that limitation; it supplies no new guarantee that legacy routing implementations already prevent competing acceptance.

**Independent conditional recommendation:** Retain H1 as the initial choice if the owner accepts stable-home dependence and authoritative write unavailability during home loss, subject to proving durable creation binding and actual enforcement. Add H2 only for a concrete cooperative-movement requirement after its complete transfer/recovery protocol is reviewed. Choose H3 initially if automatic recovery under uncooperative home failure is required, with separate data durability and stale-home exclusion guarantees.

The decisive owner choices remain failover expectations, placement/identity/naming, acknowledgement failure domains, resource-specific fencing, and trust/freshness/deployment assumptions. The routing-summary correction does not change those tradeoffs.

**Next action:** Record the owner’s mechanism and guarantee decisions, then review the selected design before implementation. No further Safety remediation is required for this amendment.

**End verification:** Repeated checks returned root `cb87e4af820d5ead2ee0461ad966589bed60007f`, Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, and object blob `a34acb46b27beb164ac67194fae1227905228033`, identical to start verification. The immutable tuple and reviewed object did not change.
