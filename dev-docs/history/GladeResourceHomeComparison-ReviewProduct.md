# Glade resource home selection comparison — PRODUCT-AXIS REVIEW

**Review object:** `dev-docs/GladeResourceHomeComparisonBrief.md` and `dev-docs/GladeResourceHomeAlternatives.md` at root `313b3c85dd623943251a0a14df1de47a0a8bf666`. DRAFT comparison mandate and decision packet, dated 2026-10-03; mechanism selection remains pending.

**Baseline:** Root `313b3c85dd623943251a0a14df1de47a0a8bf666`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Repository evidence was read using `git show <exact pin>:<path>`.

**Date:** 2026-10-03.

**Axis:** Consistency bound to Product/availability decision comparison. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0–P3 findings. The mandate supports a conditional owner decision; this does not ratify a candidate or establish implementation readiness.

**Recommendation:** Design H1 next under the confirmed requirements. Prefer H2 if cooperative movement becomes mandatory; prefer H3 only with an explicit failover requirement and an adequate ownership-and-data deployment.

---

## 0. Evidence base

Read the complete comparison brief, lines 1–118, and alternatives packet, lines 1–262. Also read workspace `AGENTS_GWZ.md`, the review-loop skill and its canonical prompt template. No earlier or current peer report was inspected.

Controlling evidence checked:

- Root authorization model §3a, §4, §6 and §7a: creation-rooted authority, local enforcement, replicated policy and operator-authorized placement.
- Root workspace directory §4, lines 115–139, and WD-8, line 274: physical-copy locks enforce exclusivity; claims route; home-node repair is a role.
- Root discovery model §0 and §3, lines 15–34 and 144–154; buy/build matrix D-06, R7, R9 and R16: local discovery folds and current coordination exclusions.
- Root decision log GDL-031/032/034/036/037/038/042/043, lines 49–66; library policy and package architecture: authority and layering constraints, honest receipts, isolated deterministic verification and proposed boundaries.
- Glade substrate §6, particularly R1/R2, lines 295–352; cross-node writes plan §2–§4 and §7, including W1–W8 and the owner ruling, lines 114–276 and 647–667.
- Glade `node/src/claims.rs`, introductory lifecycle and lease constants; `registry.rs:754–775`; `mesh/route.rs:27–68,391–431`; `session.rs:18–129`.
- Discovery registry contract, lines 1–164, particularly acceptance/recovery, partial resolution, authorization and placement limitations.

Inspection only; no tests, builds, writes or runtime operations occurred. No external precedent was needed.

The generated prompt SHA256 matched:

`9663eba065dd48ba02ce44e97b20cd6c662f15dafb42a0785680bc557ec2c33a`

Exact `git rev-parse` checks at both start and end returned:

| Object | Start and end result |
|---|---|
| Root pin | `313b3c85dd623943251a0a14df1de47a0a8bf666` |
| Glade pin | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Discovery pin | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| Brief blob at root pin | `163fa8e93f93f247af026cf2ada750f4b84145fe` |
| Alternatives blob at root pin | `a34acb46b27beb164ac67194fae1227905228033` |

These are immutable-object checks, not assertions that live HEADs stayed stationary.

## 2. Invariant analysis

### Verified facts and failed attacks

The packet preserves the distinction between authority ownership, serving home and discovery possession. Neither placement nor voting grants administrative ownership. This agrees with GDL-031/034 and authorization §3a/§7a.

The advertised deterministic claim ranking is correctly treated as routing evidence. `registry.rs:769–775` filters leases at the reader’s supplied time; `mesh/route.rs:394–407` does likewise. Different knowledge or clock evaluations can yield different holders. The packet does not mistake convergence for acquisition or fencing.

`Ok` does not establish permanent-loss durability: substrate R2 promises process-crash survival without fsync. W4 adds storage at both forwarding and holding nodes for a forwarded write, not a general replicated-data commitment. The permanent-loss scenario therefore cannot be solved merely by electing the surviving node.

The apparent conflict with authorization §1’s offline-write direction is already acknowledged by the ruled cross-node plan, lines 231–236: authoritative node placement follows the holder, while the client continues local append. The comparison preserves that distinction rather than promising disconnected authoritative admission.

The packet explicitly reserves later amendments to D-06/R7/R9/R16 for H3. Keeping discovery projected locally does not remove the need to review a new ownership service and its enforcement consequences.

The future HF tests remain satisfiable acceptance obligations, not claimed implementation evidence. No public command family or option is introduced here; there is no newly frozen interface shape to reject.

### Scenario comparison

The following are **protocol deductions from the candidate definitions**, conditional on their later serialization, authorization, recovery and fencing proofs. “Cached” means previously retained data with stated freshness/completeness limits. “Pending” means client-local edits awaiting authoritative acceptance.

| Scenario | H1: stable home | H2: cooperative transfer | H3: consensus-backed selection |
|---|---|---|---|
| **DC-01: first creation and lost reply** | New authorized independent root: authoritative creation with durable identity/binding and exact retry. Existing-scope naming unavailable when its authority is unavailable. | Same creation obligations as H1. | Authoritative creation through an existing authorized group or deliberately authorized singleton. Otherwise unavailable; discovering peers is insufficient bootstrap. |
| **DC-02: invited cold join** | Invite identifies the existing scope; verified discovery reaches its home. Writes authoritative there. Missing placement/history is unavailable, not empty defaults. | Same; resolve the current transferred generation. | Same onboarding, plus current group authority. Ownership receipt alone supplies no settings history. |
| **DC-03: concurrent same-resource creation** | Scope naming authority serializes one binding or reports conflict/unknown; aliases cannot create competing homes. | Same. | Group serializes binding, but canonical namespace/alias policy is still required. |
| **DC-04: DP-2 node loss** | Non-home lost: home’s authoritative service continues if required policy evidence remains valid. Home lost: authoritative service unavailable; cached reads/pending edits possible. New independent roots remain possible. | Same outage behavior; losing home cooperation blocks transfer. | Two voters lose quorum after either loss. New placement unavailable. Existing admission lasts only for a separately proven lease/grace policy; otherwise unavailable. No two-voter failover. |
| **DC-05: partition, writes both sides** | Reachable authorized home admits authoritative writes; other side pending/cached. No replacement identity from an empty view. | Same until a cooperative, safely fenced transition completes. | Quorum side may progress only with ready data and fencing; minority pending/cached. A two-voter split has no quorum. |
| **DC-06: permanent home loss, missing acknowledged op** | Loss/unavailable. Explicit fork/import is a new identity, not restoration. | Loss/unavailable; administrator intent cannot replace absent data or old-home participation. | Missing op remains loss/unavailable unless covered by an independent replicated-data guarantee. Consensus cannot certify its recovery. |
| **DC-07: old home resumes** | Transfer unsupported; stale advertisements cannot alter immutable binding. | Successor authoritative only after old generation is irrevocably fenced; delayed effects rejected at their actual sink. | Same rejection obligation after quorum transition; directory winner alone is insufficient. |
| **DC-08: planned move, crashes/lost replies** | Unsupported. | Authoritative successor after verified cut/readiness/fence; interruptions recover old/new/blocked/unknown outcomes without replaying effects. | Serialized transition plus identical data/effect obligations; quorum adds no automatic transfer transaction. |
| **DC-09: grow one→two→three nodes** | Add authorized replicas/discovery peers; existing home stays bound. | Same, with optional explicit cooperative movement. | Authorized membership transition required; singleton expansion and rollback are separate protocols. Independently formed groups cannot usurp a scope. |
| **DC-10: revoke, retire, replay, legacy activation** | Current applicable permission, durable retirement/identity rules and fenced activation required. Offline revocation freshness remains bounded. | Same, additionally preserving generation/fence history. | Same for hosts and voters; majority agreement does not confer grants or solve mixed-version admission. |

For all candidates, reconnect needs explicit pending, accepted, denied and conflicted states. An authoritative holder orders admission; it does not automatically provide a satisfactory semantic conflict policy for concurrent preference changes.

### Deployment comparison

| Profile | H1 | H2 | H3 |
|---|---|---|---|
| **DP-1: one node alone** | Useful independent-root creation and authoritative local service. Known unavailable scope cannot be recreated. | Same; transfer unavailable without a successor. | Useful only through accessible existing authority or expressly authorized singleton genesis. Singleton has no failure tolerance. |
| **DP-2: two development processes** | Dynamic placement useful; home-dependent availability. Shared machine/storage loss can remove both. | Same, plus cooperative movement between stores. Two processes do not establish independent storage survival. | Two-voter majority requires both. Shared-machine deployment adds no machine-loss tolerance. |
| **DP-2: two separate machines** | Non-home failure need not stop home service; home failure stops authoritative writes. | Same; planned maintenance can move beforehand, unplanned lost-home transfer remains blocked. | Neither single surviving voter can acquire authority. A bounded lease may postpone cessation, but must forbid overlapping successors. |
| **DP-3: three independent voters** | Extra replicas improve retained-read/data options, not automatic takeover. | Extra replicas support prepared moves, not absent-home recovery. | Conditional single-failure progress. Assume application history stored durably on a data majority across these domains before durable acknowledgement, with successor catch-up. This is proposed policy, not today’s `Ok`. Three placement voters plus one sole data copy do not provide it. |

All deployments require root custody, an authenticated invite/bootstrap path and explicitly eligible operators. H3 additionally exposes membership administration, quorum monitoring and data-repair responsibilities. A third witness may reduce application-storage burden but cannot replace missing history.

### Ranked requirement profiles

These are eligible **design directions**, not proven implementations. H0 is ineligible throughout because manual resource mappings fail RH-01.

| Requirement profile | Ordinal ranking and eligibility |
|---|---|
| **Confirmed baseline:** dynamic creation/discovery; two-node default; pending outages acceptable; movement and failover optional | **1 H1, 2 H2, 3 H3.** All conditionally eligible. H1 delivers the required behavior with fewer recovery states and operational assumptions. H2’s movement and H3’s failover are unrequired benefits; two-voter H3 adds quorum dependence without single-failure takeover. |
| **Cooperative movement mandatory:** preserve identity through planned maintenance/rebalance; outages may wait | **1 H2, 2 H3; H1 ineligible.** H2 directly satisfies the added requirement. H3 adds membership/quorum machinery without a required availability benefit. |
| **Automatic single-machine-loss progress mandatory:** acknowledged settings survive; DP-3 and replicated-data commitment accepted | **1 H3; H1/H2 ineligible.** Only H3 supplies absent-home acquisition within the stated fault model. Its eligibility also requires data readiness and enforceable old-home exclusion. |
| **Automatic takeover mandatory, fixed two-voter deployment, no third authority** | **No eligible candidate.** H1/H2 lack takeover; H3 lacks surviving quorum. Lower effort cannot compensate for the missing requirement. |

### Scoped additional settings alternative

**Proposed product policy:** investigate multi-replica acceptance for a declared class of mergeable preferences, such as per-user appearance or layout, separately from exclusive resources.

Prerequisites include immutable shape/schema selection, stable signed operation identity, equivalent authorization evidence and validation at every accepting/folding node, exact retry, explicit durability receipts, and a documented merge rule. Concurrent updates must merge deterministically or expose conflicts; invalid/revoked operations must not silently become committed shared truth.

This is not the cross-node plan’s rejected option B transplanted unchanged: that option can acknowledge operations another node later refuses because policy, shape or writer admission differs.

Exclude grants, membership, physical copies and external effects. Effectful settings require a separate authority to apply changes. This alternative could outperform all three homes for offline preference editing, but it cannot satisfy the general exclusive-resource problem.

## 3. Risks and next action

The single next action is an **H1 creation/identity contract design**, conditional on the owner accepting home-dependent authoritative availability. Start with whole-share placement and zone-addressed settings; no per-key elections. Specify naming authority, lost-reply retry, unavailable-versus-absent discovery, retirement, trust bootstrap and activation before implementation.

Challenge the packet’s H1→H2 staging hypothesis: H2 is not an inevitable second stage. If movement is already essential, design H2 now; otherwise building transfer prematurely adds an interruption/recovery grammar without demonstrated user value. Conversely, making H1’s identity and receipts incompatible with future generations would create avoidable migration cost.

The smallest owner decisions are:

1. May existing settings remain pending while their home is unavailable, or must another machine accept them authoritatively?
2. Must acknowledged settings survive permanent machine loss, and will the owner fund additional independent data/authority domains?
3. Must planned moves preserve identity now?
4. Which settings are mergeable preferences, and which govern exclusive actions or policy?
5. Are distinct independent roots sharing a human label acceptable, with canonical naming serialized only inside an existing scope?

The decisive ranking flip is **required authoritative progress after unplanned home loss**. With DP-3, replicated-data acknowledgement and enforceable fencing, choose H3; with only cooperative moves, choose H2.

No data-volume, latency, implementation-duration or operational-cost measurements support quantitative claims. H1’s likely smaller burden follows from fewer mechanisms, not measured delivery time. The next design must expose deterministic HF/DC traces, then adapter crash and affected-consumer checks; current draft fixtures and loopback journeys do not establish those guarantees.
