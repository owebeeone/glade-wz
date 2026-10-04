# Glade stable-home design and multiwriter evaluation — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT H1 design, DRAFT multiwriter evaluation, and GDL-050 at revised root `f9f020b800ba33cabb0c036ee213b15f6e1b33d7`. Focused Round 2 closure of P3-1.

**Baseline:** Root `f9f020b800ba33cabb0c036ee213b15f6e1b33d7`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Evidence read using exact-pin `git show`, numbered source excerpts, and the document diff from Round 1.

**Date:** 2026-10-03.

**Axis:** Amendment-target accuracy and consistency of the corrected legacy/H1 distinction. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — P3-1 closed; zero remaining findings on this axis. Acceptance remains limited to semantic design/evaluation for the next contract/proof tranche. No executable interface, implementation, wire profile, migration or activation is ratified.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P3-1 | Separate DiscoveryModel §3 routing/home-role prose from §7’s epoch-fence/takeover scenario; require an H1 rejection counterpart and preserve legacy coverage. | H1 line 271 now names both sections and explicitly requires the two distinct test outcomes. Re-traced against DiscoveryModel lines 144–154 and 268–269. | Closed |

## Changed-range analysis

The exact diff from `d6f994e1e4d3c4458be9b577336fe849ae628e2c` to the revised root changes only H1’s amendment-table row at line 271 within the three reviewed documents.

The row replaces the incorrect “DiscoveryModel §3 takeover” association with separate references to §3 routing/home roles and §7’s epoch-fence/takeover scenario. It adds the required H1 rejection test while retaining the legacy scenario for unactivated shares.

This repairs traceability without changing the stable-home guarantee, admission boundary, migration conditions, public interface or protocol. Multiwriter and DecisionLog blobs remain identical. No new architectural root cause was found.

## 0. Evidence base

Read the complete Round 2 generated prompt and verified SHA-256:

`385ae16b08ed3cc2cbceda9d4f0c9751c293af0b4fa8baa40c2ea824589ec1f2`.

Inspected the exact three-document diff, revised H1 lines 250–304, and revised-root DiscoveryModel §§3/7 source passages. The unchanged source graph retains the Round 1 evidence basis.

Start and end checks returned identical values:

| Object | Verified at both boundaries |
| --- | --- |
| Root revision | `f9f020b800ba33cabb0c036ee213b15f6e1b33d7` |
| Glade revision | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Glade-discover revision | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| H1 blob | `8c0dc56cf1f2b6b5e674c6206a7cdcfa6c101c09` |
| Multiwriter blob | `83784016d3e418ddf4fab4a087aa1ccc9067adb4` |
| DecisionLog whole blob | `c8ce426b3aa125457f28f778ac212424a5c03873` |

No files were modified, tests/builds/runtime operations performed, or current-round peer report accessed.

## 2. Invariant analysis

The original counterexample no longer reproduces: following the amendment row now reaches the actual takeover requirement, rather than a routing recap containing no takeover clause.

DiscoveryModel §3 describes identity, local directory routing and home-node roles. Section 7 requires a higher-epoch claim to win and the stale holder’s serve to bounce. The corrected row assigns each passage its proper reconciliation obligation.

The scenario distinction is explicit and coherent. An activated H1 share must reject epoch-based takeover; an unactivated legacy share retains the existing takeover scenario. This agrees with adjacent H1 lines 252–255, which prohibit simultaneous legacy and H1 admission for one address, and line 269, which preserves W1–W8 for legacy shares. The amendment therefore neither silently deletes legacy coverage nor authorizes a second H1 home.

## 3. Risks and next action

The correction supplies a future test obligation, not executed test evidence. Exact wire/schema amendments, consumer witnesses and adapter proofs remain subsequent gates.

Proceed to the scoped contract/proof tranche with P3-1 recorded closed and the corrected legacy/H1 scenario pair retained in its amendment traceability.
