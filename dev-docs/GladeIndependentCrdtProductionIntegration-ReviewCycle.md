# Independent CRDT production integration — IC-3 review and delivery ledger

Date: 2026-10-04. Status: **owner authorized IC-3A, IC-3B and IC-3C; initial semantic dual NO-GO; remediation 1 in progress, no integration accepted**.

The owner directed “Proceed IC-3ABC” after the deterministic IC-2 kernel was
accepted and its source/checkpoint history pushed. This authorizes design,
review and implementation of real scoped evidence, process-restart persistence
and automatic two-node application exchange under accepted contracts. It does
not authorize desk activation, existing-store migration or stronger storage
receipts. A known authenticated resource and text profile bound the first real
witness; they do not bound the general multiwriter capability.

Baseline: workspace `8c8332ad94ba42b43144830b6c3809c45d0b7bbd`, Glade
`37dff286ce1eb9690204d4a7d14c940333396a30`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`. IC-2 reviewed tuple, closures and
limitations remain in [its ledger](GladeIndependentCrdtAdmission-ReviewCycle.md)
and [archived component record](history/GladeIndependentCrdtAdmissionKernelImplementation.md).
Moving documentation did not amend any contract; historical references remain
unmodified as the owner requested. New references MUST name the actual location.

## Recorded gates and ownership

| Checkpoint | Recorded tier | Acceptance requirement |
| --- | --- | --- |
| IC-3A semantic production-adapter design/amendment | Fresh dual Consistency/Safety | Exact changed clauses, implementable evidence/storage/reconstruction/exchange contracts, all blocking findings closed on one committed tuple. |
| IC-3A typed boundaries/allocation and behavioral RED | Fresh dual Code/State | Meaningful interfaces, justified roles/dependencies and source-qualified Gyld allocation; actual compiling behavioral RED and retained consumer controls. No implementation before acceptance. |
| IC-3B authenticated physical host/recovery component | Fresh dual Code/State | Real signatures, disk atomicity/coupled outcomes, process kill/reopen and authoritative continuation proof; scoped gates and all original consumers. |
| IC-3C automatic duplex real-node aggregate | Fresh dual Code/State | Real separate processes authenticate, edit while partitioned, exchange without client ferrying, converge valid operation sets/text, survive restart/retry and reject wrong scopes/profiles; focused and aggregate evidence. |
| A user-facing interface/CLI/file-format freeze, if introduced | Additional Surface | Cold help/docs lifecycle/defaults inspection; IC-4 browser/compatibility and activation remain separate. |

Review-loop skill and canonical template control. One source/design drafter owns
the packet; lane owner owns this ledger, git, prompt generation, verbatim report
filing and verdict merge. Reviewers are fresh, independent, read-only and peer
blind each round, and verify exact HEADs at start/end. Strongest tiers inherit the
parent session; no cross-family selection. Completed prompts/reports/remediation
evidence SHOULD be filed under dev-docs/history for this cleanup convention.

Each new object starts at zero remediation rounds. No earlier stopped contract
object or IC-2 count is reset. At most two merged remediation rounds; the skill's
exception permits a third confined nonarchitectural correction, and any
architectural root in that third correction stops. A reviewer-classified third
new architectural root requires STOP and owner redesign-or-accept decision.
Originating reviewers alone close findings; writer GREEN is evidence, not closure.
Material boundary changes require fresh full axes.

## Immediate action

Source-ground the IC-3A design in current admission/storage API and actual
node signing, persistence, ownership, carrier and lifecycle seams. Explicitly
resolve retained observation and authoritative full-cut reconstruction before
claiming a sticky completeness marker can clear. Choose bounded injectable
development trust/time/storage/transport inputs; do not invent production keys,
quotas or live configuration. Tests precede implementation. Normal independent
package loops MUST remain fast; no role, dependency, selector, budget or
process-global allowance is loosened to make checks pass.

## IC-3A semantic packet settled for initial review

One source drafter completed the
[design](GladeIndependentCrdtProductionIntegrationDesign.md) (450 lines; SHA256
`e96d4ffc0bf8a8a213d4bb0d58d9f5e595604e80ca4ff71925dd4df5904eafc0`) and
[plan](GladeIndependentCrdtProductionIntegrationPlan.md) (195 lines; SHA256
`3cd2255bbc0e124bb7ffae77db41bba9b26abadfa977fe29585d40a355b96f1c`).
New links resolve and whitespace passes; source, archives, old references,
manifests and classifications remain unchanged. This is design/source inspection
evidence only, no crypto/disk/network execution or compiling typed gate.

Packet selects the bounded genuine direct-root authorization profile, complete
versioned per-instance images plus independently trusted floor handshake, same
logical-owner process reattachment under physical lifetime ownership, complete
retained callback/outcome/custody state, actual production node routing/lifecycle
and real Iroh symmetric full-history exchange. A narrow combined cut can complete
only durably retained adapter obligations; the accepted kernel sticky flags have
no clearing event and remain untouched. Those choices are DRAFT, not self-approved.

The first fresh dual Consistency/Safety review MUST verify exact source-qualified
contracts and implementation feasibility on a committed five-repository tuple.
Prompts are generated from the canonical skill template after commitment; exact
root SHA appears there. Other pins remain the baseline above. During review only
current generated prompts and verbatim report outputs may be untracked. Zero
remediation rounds/architectural roots initially; neither implementation nor
interface creation is authorized by writer completion. No push or live changes.

## Initial IC-3A semantic review — dual NO-GO

Both read-only reviewers verified the root `4d641dd179e5b0bd94e84df9cb334d7a21dae371`
and all four unchanged source pins at start and end. Their complete testimony
is filed verbatim in history:

- [Consistency](history/GladeIndependentCrdtProductionIntegrationDesign-ReviewConsistency.md),
  SHA256 `ae3dc3e72e9cbc858982324f85e9e173649943e838a5a05e72c66eb061a60bdf`:
  NO-GO, three P2 findings.
- [Safety](history/GladeIndependentCrdtProductionIntegrationDesign-ReviewSafety.md),
  SHA256 `7dda23b87d73a36eac9db41f8aaed2bfc3c180a616a68ffcad58231db69b5f46`:
  NO-GO, one P2 finding.

Four IDs represent three distinct roots. Consistency P2-1 and Safety P2-1
independently converge on the same architectural root: history can be consumed
before a durable ingress discriminator exists, and failed observation plus
failed loss-marker writes allow false completeness after restart. Consistency
P2-2 is a second architectural root, incomplete authoritative declaration/schema
identity composition. Consistency P2-3 restores the required pre-remote-use
Rust/TS/Python canonical-vector gate and is nonarchitectural. There are two
unique architectural roots, not four; no third root is established.

[Remediation 1](history/GladeIndependentCrdtProductionIntegrationDesign-RemPlan-1.md)
maps every ID to a correction and closure witness. One drafter edits the design
and plan as one patch. Completed remediation count remains zero until that
patch settles; round 1 is authorized. The original reviewers MUST verify their
own counterexamples on the corrected tuple. Because durable consumption and
authenticated identity boundaries change materially, fresh full
Consistency/Safety review is also required on that same tuple. This remains
the same semantic object and preserves the two-root count and all historical
caps. A reviewer-classified third architectural root requires STOP before
another patch. No source/interface implementation is accepted or begun.

The aggregate IC-3C gate also requires fresh Consistency/Safety verification
of the semantic contract in addition to the recorded Code/State review, as
specified by the delivery plan; this adds evidence and does not replace an axis.
