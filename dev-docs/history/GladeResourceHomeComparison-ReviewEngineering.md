# Glade resource home selection comparison — ENGINEERING-AXIS REVIEW

**Review object:** [GladeResourceHomeComparisonBrief.md](/Volumes/projects/limbo/glade-wz/dev-docs/GladeResourceHomeComparisonBrief.md:1) and [GladeResourceHomeAlternatives.md](/Volumes/projects/limbo/glade-wz/dev-docs/GladeResourceHomeAlternatives.md:1) at root `313b3c85dd623943251a0a14df1de47a0a8bf666`. DRAFT comparison mandate; owner selection pending.

**Baseline:** Root `313b3c85dd623943251a0a14df1de47a0a8bf666`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Sources read through `git show <exact-pin>:<path>`.

**Date:** 2026-10-03.

**Axis:** Safety bound to Correctness/engineering decision comparison. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0/P1/P2/P3 findings. The mandate supports a conditional owner decision; this verdict neither selects a mechanism nor approves implementation.

**Recommendation:** Design H1 next under the confirmed baseline. Require durable creation serialization, exact outcome recovery, and enforceable admission. Preserve a reviewed path toward generations and transfer; do not freeze permanent immobility into resource identity.

---

## 0. Evidence base

The generated `Round1-Engineering.txt` was read completely. Its SHA256 matched:

`f8cdd351e5bb12bd8f3d26cbcaca1501356a97caa44f094004a3e47287a59964`.

Read `AGENTS_GWZ.md`, the review-loop `SKILL.md`, and its canonical prompt template. No peer or earlier review reports were inspected. No files, runtime state, dependencies, or Git state were changed; no tests or builds were run.

Pinned evidence inspected:

| Repository | Sources and relevant locations |
| --- | --- |
| Root | Complete comparison brief, lines 1–118; complete alternatives packet, lines 1–262 |
| Root | `GladeAuthzModel.md` §3a–§4, §6–§7a and enforcement rulings; `DecisionLog.md` GDL-031/032/034/036/037/038/042/043 |
| Root | `GladeWorkspaceDirectory.md` §3–§4 and WD-8; `GladeDiscoveryModel.md` §0–§3 and owner-rooted serve-grant admission |
| Root | `GladeBuyBuildMatrix.md` D-06/R7/R9/R16; `LibraryBoundaryAndTestingPolicy.md` classifications, LBT-001–012 and verification rules; `GladePackageArchitecture.md` boundaries, ephemeral registration and staging |
| Glade | `GladeSubstrateV1.md` §6, especially R2; `GladeCrossNodeWritesPlan.md` alternatives, W1–W8, implementation notes and §7 limits |
| Glade | `node/src/registry.rs` claim ranking; `claims.rs` creation/epoch minting; `mesh/route.rs` routing and claim fold; `mesh/serve.rs` forwarded acceptance; `accept.rs` admission and placement |
| Glade-discover | Complete `dev-docs/RegistryContractDraft.md`, lines 1–164 |

**Start and end checks:** All five `git rev-parse` results matched at both boundaries:

| Object | Start = end |
| --- | --- |
| Root pin | `313b3c85dd623943251a0a14df1de47a0a8bf666` |
| Glade pin | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Discovery pin | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| Brief blob | `163fa8e93f93f247af026cf2ada750f4b84145fe` |
| Alternatives blob | `a34acb46b27beb164ac67194fae1227905228033` |

Live HEAD stability was not a requirement. The immutable evidence tuple remained available and unchanged.

## 2. Invariant analysis

**Verified source facts:** Creation roots authority; hosting and voting do not confer grant-issuing rights. Authorization is checked locally at serving/enforcement boundaries, with forward-only revocation as evidence propagates. Discovery returns partial local knowledge. Registry acceptance receipts promise local acceptance, not replication or resource ownership.

Current node routing ranks eligible live claims by epoch and node ID, using each reader’s clock. Creation increments the maximum locally observed epoch. These mechanisms do not establish exclusive acquisition across partitioned machines. The packet correctly identifies that limitation rather than claiming the current implementation already supplies H1–H3.

Substrate R2 promises process-crash survival, without fsync; forwarded success means the holder and forwarding node hold the operation. It does not promise permanent-machine-loss durability or automatic repair.

**Deductions from the candidate obligations:**

- Two creators need one canonical binding transaction or an explicit conflict/unknown outcome. Two local journals with independent CAS checks are insufficient. Independent roots need no global registry, but naming within an existing scope needs its identified serialization authority.
- A delayed old-home effect must meet fencing at the actual store or effect sink. Checking a directory winner before queuing the effect leaves a race. A filesystem lock protects its physical checkout, not another machine’s independent store.
- For H2, crashing after irreversible fencing cannot justify restoring old admission. Successor unreadiness then means unavailable service. Before fencing, recovery may restore old admission only after proving the transition state and applicable permission.
- A delayed reply for an operation accepted before the cut may settle the original intent; it is not automatically invalid. An operation arriving after fencing must be rejected, and a lost reply must not trigger a fresh effect.
- Ownership consensus cannot recover absent settings history. H3 must refuse activation when its required data cut is missing.
- A revoked voter’s removal must follow safe membership transition. Simply excluding it from local counts can manufacture incompatible majorities.

Attempts to refute the packet through those interleavings failed: RH-02/05–08/11–12 and H2/H3 explicitly require these boundaries or declare unavailable/unknown outcomes. Their downstream protocols remain unproven, which the mandate acknowledges.

Immediate global revocation during partition is also unproven. Current local-fold reevaluation cannot reveal an unseen revocation. A selected design must declare freshness limits or introduce an explicitly reviewed barrier; H3 does not solve this merely by electing a home.

## 3. Scenario comparison

These are **conditional design outcomes**, not observed implementation behavior. “Authoritative” requires the candidate’s named admission and authorization prerequisites. Pending edits and cached reads provide no authoritative acceptance or completeness guarantee.

| Scenario | H1: stable home | H2: cooperative transfer | H3: consensus placement |
| --- | --- | --- | --- |
| **DC-01** Singleton creation; lost reply | Independent root: authoritative local creation with durable identity/retry. Existing-scope naming: unavailable if its authority is absent. | Same initial creation as H1. | Authoritative through reachable group or deliberately authorized singleton. Otherwise unavailable; retries recover the committed identity. |
| **DC-02** Invited join; empty view | Discover existing identity, fetch settings, write through home. Partial empty view means unknown/unavailable, never defaults. | Same; a transition may make admission temporarily unavailable. | Same; placement receipt does not certify complete data. Successor readiness remains necessary. |
| **DC-03** Canonical creation race | One serialized binding or explicit conflict/unknown; advertisements cannot arbitrate creation. | Same creation obligation. | Group serializes canonical entry and retry; naming/alias policy still required. |
| **DC-04** DP-2 non-home/home loss | Non-home loss can leave home authoritative. Home loss: pending edits, cached reads, authoritative service unavailable. New existing-scope creation depends separately on naming authority. | Same absent a safely completable transfer; home loss cannot be bypassed administratively. | Two voters: either loss removes majority. New placement unavailable. Existing service depends on validation branch; bounded lease grace is possible, not presumed. |
| **DC-05** Partition; writes both sides | Home side may remain authoritative under declared permission policy; other side pending/cached/unavailable. No second home. | Same; transfer blocked without required participants/enforcement. | Majority side may serve after fencing/readiness. Minority pending/cached/unavailable. Two-voter split has no majority. |
| **DC-06** Permanent home loss; missing acknowledged op | Explicit loss and unavailable recovery; no empty replacement under the original identity. | Loss; cooperative handoff unsupported without necessary old-home participation. | Loss remains loss. Ownership quorum cannot restore the missing op; unavailable until an explicit recovery policy permits progress. |
| **DC-07** Paused old home resumes | Relocation unsupported; stale discovery does not create another admission path. | Successor authoritative only after cut/readiness/fence; old effects rejected at enforcement. | Same exclusion obligation; election alone is insufficient. |
| **DC-08** Planned move; boundary crashes | Unsupported. Fork/import creates another identity. | Recover original transfer into old/new/blocked/unknown states; committed fence never rolls back. | Serialized transition plus independent data-cut/effect recovery; consensus receipt alone is insufficient. |
| **DC-09** One → two → three nodes | Authorized hosts/replicas may join; no voter transition. Migration to movable homes needs a separate gate. | Same; transfer protocol activation must exclude old admission. | Authorized reconfiguration and recovery required. Singleton cannot be cloned into independent groups; rollback cannot erase committed membership/generations. |
| **DC-10** Revocation, retirement, replay, legacy | Current applicable permission; persistent retry/retirement evidence; name reuse gets explicit identity rules. Legacy overlap blocks activation. | Same, including durable fences surviving restart and rollback. | Same plus voter removal/reconfiguration. Quorum is neither grant authority nor a retirement tombstone. |

## 4. Deployment comparison

| Profile | H1 | H2 | H3 |
| --- | --- | --- | --- |
| **DP-1: one node** | Natural independent genesis; existing unavailable scope stays unavailable. | Same; no movement target yet. | Needs reachable existing group or authorized singleton. Singleton offers no voter-loss tolerance. |
| **DP-2: two processes, one machine** | Process redundancy can help discovery; shared machine/storage failure remains. | Shared-resource handoff possible only with actual exclusion and data readiness. | Two-voter majority needs both; two processes do not establish independent failure domains. |
| **DP-2: two machines** | Home may survive non-home failure; home failure stops its service. | Cooperative move possible while participants remain reachable; no unilateral takeover. | No majority after either loss. Neither singleton reset nor peer discovery repairs membership safely. |
| **DP-3: two data nodes + separate witness** | Extra replica does not grant takeover. | More possible transfer targets, subject to complete cut. | Can retain ownership majority after one failure. Witness cannot supply missing data. Requiring both data nodes before success can protect prior acknowledgements, but loses that write guarantee when either is absent unless policy/membership changes. |
| **DP-3: three data-bearing voters** | More copies, still stable home. | Cooperative movement remains possible. | Strongest failover direction **if** durable data commitment and successor catch-up are specified. Three voters alone do not make application writes a replicated log. |

H3’s lease, quorum-per-acceptance and shared-sink branches have different availability and timing proofs. A valid lease may permit existing service during coordination loss; no lease duration is established here. Quorum-per-acceptance stops without quorum. A sink may continue validating existing authority under its own contract.

## 5. Ranked requirement profiles

Eligibility below means a viable **next design direction**, subject to the packet’s proof gates.

| Requirement profile | Ordinal ranking | Reason |
| --- | --- | --- |
| **Confirmed baseline:** dynamic creation/discovery; two-node default; failover and moves optional | **H1 > H2 > H3** | H1 addresses the mandatory gap with fewer transition states. H2 adds recovery work for an optional feature. H3 adds membership/coordination obligations without two-voter failure tolerance. All conditionally eligible. |
| **Planned identity-preserving movement mandatory; cooperative outage acceptable** | **H2 > H3; H1 ineligible** | H2 directly supplies the required operation. H3 adds automatic coordination capabilities without a corresponding requirement. |
| **Automatic failover after one machine loss, preserving all acknowledged data, mandatory** | **H3 only conditionally eligible** | Requires independent voter quorum, adequate durable data replicas and enforceable fencing. H1/H2 are ineligible. Under unchanged DP-2, none satisfies this profile. |

If exclusive authoritative acceptance on **both partition sides** becomes mandatory, none of H1–H3 qualifies. That requirement needs changed semantics.

For a narrowly declared mergeable settings class, an additional replicated multiwriter alternative deserves investigation: signed operations, predeclared schema/shape, deterministic merge/conflict policy, authorization-validity filtering and honest revocation limits. It cannot inherit current two-way replication as a proof; the cross-node plan documents permanent divergence when replicas accept operations another rejects. This alternative does not generalize to physical checkout mutations or external effects.

## 6. Risks and next action

**Proposed policy:** Begin with share-level placement, matching current routing. Keep settings zones inside that unit; introduce smaller explicit units only for a demonstrated isolation or availability need.

The next action is one H1 contract/design tranche covering canonical naming, durable identity/home binding, outcome lookup, permission checks, admission, retirement and migration eligibility. Keep discovery as projected records. Classify transition logic as a deterministic pure machine, storage/fencing services behind meaningful contracts, and orchestration at integration boundaries.

Verification must exercise HF-01–10, including overlapping creation, cancellation, boundary crashes, delayed effects, clock uncertainty, missing data, membership changes and legacy overlap. Concrete adapters require actual crash tests and affected-consumer checks alongside the adopting architecture gate; volatile draft fixtures are insufficient.

I challenge the packet’s “H1 then H2” staging hypothesis in one respect: immutable binding must describe **H1’s current behavior**, not an irreversible identity or wire invariant. Preserve identity separately from host identity and record the future migration gate now. If planned moves are already expected in the first supported deployment, design H2 directly and stage its implementation.

The smallest owner questions are:

1. Are existing-scope creations allowed to wait for their naming authority?
2. Are planned moves required in the first supported release?
3. Must acknowledged writes survive permanent machine loss, and are three independent domains acceptable?
4. Which effect sinks can enforce generations, and what disconnected authorization freshness is acceptable?

Answers requiring planned movement flip the choice to H2. Answers requiring automatic loss recovery flip it to H3 only with the deployment/data commitment. Creation-authority availability, real storage latency, transfer volume and implementation effort remain unmeasured; no numerical cost or performance advantage is claimed.
