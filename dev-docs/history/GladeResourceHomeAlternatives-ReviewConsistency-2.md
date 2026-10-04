# Glade resource home alternatives — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeResourceHomeAlternatives.md` at root commit `cb87e4af820d5ead2ee0461ad966589bed60007f`; DRAFT decision packet dated 2026-10-03. Focused amendment verification of Consistency P3-1.

**Baseline:** Root `cb87e4af820d5ead2ee0461ad966589bed60007f`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Document and implementation evidence were read through `git show <exact pin>:<path>`; the object diff was read between the exact prior and revised root commits.

**Date:** 2026-10-03.

**Axis:** Consistency of the routing-summary amendment with both pinned fold implementations and the original counterexample. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Start verification:** All three immutable revision pins matched. The revised object blob was `a34acb46b27beb164ac67194fae1227905228033`. The complete Round 2 canonical prompt’s SHA-256 matched `79b54545ed40351789d8939d357422b8add397707236d29248e4f2edacec6e05`.

**Verdict: GO** — P3-1 is closed. No new findings; zero open P0, P1, P2 or P3 findings on this axis. This accepts the packet’s fitness for an owner decision, not an alternative, implementation or architecture amendment.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P3-1 — Record convergence alone does not guarantee one live routing answer | Qualify the evidence row and comparison shorthand to distinguish deterministic ranking from reader-relative lease filtering | Revised line 47 expressly permits different routing answers from identical records; lines 194–196 repeat that qualification. Both pinned folds produce A at time 9,000 and B at time 11,000 for the original example | **Closed** |

## Changed-range analysis

The object diff from `c79c73ca6afc371628727c0ca125eb55fd9fe41a` to `cb87e4af820d5ead2ee0461ad966589bed60007f` contains exactly two substantive changes:

- Line 47 replaces the unconditional convergence summary with agreement for the same eligible live claims at a common evaluation time, and explicitly identifies reader-relative lease filtering.
- Lines 194–196 replace “eventually convergent claim fold” with a deterministic fold over the same eligible live records and explicitly permit differing expiry evaluations to produce differing answers.

Both changes address P3-1. Neither changes a candidate mechanism, requirement, acceptance boundary, migration rule or owner question. No new architectural root cause was identified.

## 0. Evidence base

Read and inspected:

- Complete `Round2-Consistency.txt`, with the matching SHA-256 above.
- Exact object diff between the prior and revised root pins.
- Revised packet, lines 1–262, particularly line 47, RH-05/RH-06 at lines 75–76, and lines 192–196.
- Glade `node/src/mesh/route.rs`, lines 389–407, at the unchanged Glade pin.
- Glade `node/src/registry.rs`, lines 754–776, at the unchanged Glade pin.
- Start and end `git rev-parse` checks for all three revisions and the revised object blob.

Round 1’s controlling-source analysis remains the baseline for unchanged text. The bounded diff did not warrant rereading the wider source graph.

No files were written, no tests or builds ran, and no current-round peer report was inspected.

## 2. Invariant analysis

The original counterexample remains valid in the pinned code:

| Evaluation | Eligible live claims | Result |
| --- | --- | --- |
| `now_ms = 9,000` | A: lower ID, epoch 1, expiry 10,000; B: higher ID, epoch 1, expiry 20,000 | A |
| `now_ms = 11,000` | B only; A has expired | B |

`mesh::who_serves` applies `lease_expiry_ms > now_ms` before ranking. `RegistryApi::who_serves` applies the same filter and shared `rank_claims`. The revised wording now accurately allows this divergence despite identical replicated records.

For identical eligible live sets, both implementations rank by higher epoch and then lower node ID. The amendment preserves that deterministic agreement without claiming that record exchange synchronizes expiry evaluations.

The surrounding safety distinction remains intact: routing is a baseline, not an exclusive-ownership protocol. RH-05/RH-06 still require actual serialization, fencing, generation validation and explicit clock assumptions. The correction therefore closes the misleading evidence claim without weakening any candidate obligation.

## 3. Risks and next action

The correction does not resolve the downstream creation, transfer, fencing, membership or data-preservation designs, and does not claim to. Those remain required gates after owner selection.

**Independent conditional recommendation:** The Round 1 recommendation remains unchanged: choose H1 if fixed-home write unavailability is acceptable; choose H2 directly if planned identity-preserving moves are required; choose H3 if automatic failover is required and the owner accepts quorum deployment plus a sufficient data-preservation policy. The decisive tradeoff remains outage availability versus coordination, fencing and durability cost.

The owner still must decide placement granularity, naming authority, tolerated home outage, acknowledgement failure domain, enforcement boundaries and trust assumptions.

**Next action:** File this closure report and complete the lane-owner verdict merge at the revised tuple before seeking the owner’s conditional selection. No implementation is authorized by this GO.

**End verification:** Repeated all three revision-pin checks and the revised object-blob check. Results remained:

- Root: `cb87e4af820d5ead2ee0461ad966589bed60007f`
- Glade: `90dc1a60981185fa26ae5bfafbbb5377c12a413b`
- Glade-discover: `52ea2d118f45d9e7c3d9a789310dd5d669958851`
- Review-object blob: `a34acb46b27beb164ac67194fae1227905228033`

The immutable tuple and object blob did not change.
