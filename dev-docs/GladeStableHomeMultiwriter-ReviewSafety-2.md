# Glade stable-home design and multiwriter evaluation — SAFETY-AXIS REVIEW

**Review object:** Focused citation amendment in DRAFT `dev-docs/GladeStableHomeDesign.md` §8, combined with unchanged `GladeMultiwriterSettingsEvaluation.md` and GDL-050, at root `f9f020b800ba33cabb0c036ee213b15f6e1b33d7`.

**Baseline:** Previous root `d6f994e1e4d3c4458be9b577336fe849ae628e2c`; revised root `f9f020b800ba33cabb0c036ee213b15f6e1b33d7`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Sources read through exact-pin `git show`; reviewed-document changes inspected with the exact revision diff.

**Date:** 2026-10-03.

**Axis:** Safety — verify the citation correction preserves admission, takeover and activation boundaries. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero new P0–P3 findings. Previous Safety GO remains applicable to the unchanged semantic packet. This accepts semantic design/evaluation scope for subsequent contract/proof work only.

---

## Prior-finding closure table

| Prior item | Verification | Status |
| --- | --- | --- |
| Safety Round 1 | No prior findings; independently checked the amendment for unsafe scope or guarantee changes. | GO retained |
| Consistency P3-1, identified in the authorized Round 2 prompt | Revised pointer correctly separates DiscoveryModel §3 routing/home roles from §7’s takeover scenario; adds distinct H1 rejection coverage. | Correction independently verified; formal closure belongs to its originating reviewer |

## Changed-range analysis

The exact diff changes only H1’s amendment-table row at line 271. Multiwriter and DecisionLog produce no diff and retain their original reviewed blobs. No architecture, interface, protocol boundary or runtime behavior changed.

The row preserves the existing prohibition on lease-based binding movement and the physical-copy lock requirement. It corrects source attribution and adds an explicit future test obligation.

## 0. Evidence base

I read the complete generated `Round2-Safety.txt`. Its SHA-256 matched:

`bdd4c1eb1d957f24f38d6af62dbc6b78469e43f9fbe3d09a6355d8d169abca36`.

I inspected H1 lines 154–190 and 250–279, and DiscoveryModel §3 lines 144–154 and §7 lines 256–278. The cited passages also matched their contents at the previous root.

Start and end checks returned identical exact results:

| Object | Verified value |
| --- | --- |
| Root revision | `f9f020b800ba33cabb0c036ee213b15f6e1b33d7` |
| Glade revision | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Discovery revision | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| H1 blob | `8c0dc56cf1f2b6b5e674c6206a7cdcfa6c101c09` |
| Multiwriter blob | `83784016d3e418ddf4fab4a087aa1ccc9067adb4` |
| DecisionLog blob | `c8ce426b3aa125457f28f778ac212424a5c03873` |

No files, Git state or runtime state were modified. No tests/builds were run, and no current-round peer report was accessed.

## 2. Invariant analysis

DiscoveryModel §3 describes local replicated routing, unreachable-host status and the home-node role. Its §7 lines 268–269 specifically requires higher-epoch takeover and stale-holder rejection. The revised citation now locates both accurately.

I retraced the dangerous interpretation: home A becomes unreachable; B publishes a higher epoch; a router selects B; B attempts authoritative admission for A’s activated H1 share. H1 lines 167–180 still requires the committed binding at the actual admission boundary and prohibits independent non-home acknowledgement. Line 271 now expressly requires an H1 test rejecting this takeover. Directory precedence cannot manufacture another acceptance path.

For an unactivated legacy share, the same revised row retains the historical §7 scenario. This preserves the legacy contract rather than silently deleting its coverage. H1 lines 252–265 continues to prohibit simultaneous legacy/H1 admission and requires migration exclusion and rollback fencing. Thus retaining legacy takeover coverage does not authorize legacy takeover on an activated H1 address.

## 3. Risks and next action

The citation fix improves traceability without weakening the previously reviewed guarantees. Actual profile discrimination, binding enforcement and migration exclusion remain downstream proof obligations.

Proceed to the scoped contract/proof tranche with separate legacy takeover and activated-H1 takeover-rejection scenarios. This GO does not ratify wire mappings, executable interfaces, production adapters or deployment.
