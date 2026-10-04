# Glade resource home alternatives — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeResourceHomeAlternatives.md` at root commit `c79c73ca6afc371628727c0ca125eb55fd9fe41a`; DRAFT decision packet dated 2026-10-03.

**Baseline:** Root `c79c73ca6afc371628727c0ca125eb55fd9fe41a`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Product sources were read through `git show <exact pin>:<path>`, with numbered lines.

**Date:** 2026-10-03.

**Axis:** Consistency against the controlling document graph, internal guarantees, source claims and future acceptance scenarios. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Start verification:** All three revision-pin checks matched. The root object blob was `b23ce879fc2d9b7f9012bce7909b931b9c9dc63a`. The complete canonical prompt’s SHA-256 matched `86dcd6e448a9f70fd41b605a97f3aa601e9cfccbb37524485dab3b4a11e94f65`.

**Verdict: GO** — zero P0, P1 or P2 findings; one bounded P3 source-precision finding. GO means fitness for an owner decision, not implementation acceptance, architecture ratification or permission to change existing contracts.

---

## 0. Evidence base

The following evidence was inspected:

| Repository/source | Sections or numbered lines |
| --- | --- |
| Root `GladeResourceHomeAlternatives.md` | Entire object, lines 1–261 |
| Root `glade/GladeAuthzModel.md` | §3a, lines 76–123; also authorization enforcement, policy placement, operator trust and authorship at lines 21–37, 152–224, 244–283 and 370–410 |
| Root `DecisionLog.md` | GDL-031/032/034/036/037/038/042/043, lines 49–66; adjacent draft-contract status at lines 67–69 |
| Root `glade/GladeWorkspaceDirectory.md` | Entire document, particularly §4, lines 115–139; discovery layering, lines 222–261; WD-8, line 274 |
| Root `glade/GladeDiscoveryModel.md` | §0, lines 15–34; clock/fold separation, lines 130–142; §3/§4, lines 144–184; advertisement and test obligations, lines 222–295 |
| Root `GladeBuyBuildMatrix.md` | Entire worksheet, particularly D-06/D-07/D-09/D-10/D-12, lines 34–40; R7/R9/R16, lines 67/69/86 |
| Root `LibraryBoundaryAndTestingPolicy.md` | Entire document, lines 1–108 |
| Root `GladePackageArchitecture.md` | Entire document, lines 1–160 |
| Glade `dev-docs/GladeSubstrateV1.md` | Core-model amendments, lines 22–82; session outcomes and receipts, lines 280–449; cross-node write guarantees and contradictions, lines 514–689 |
| Glade `dev-docs/GladeCrossNodeWritesPlan.md` | Current-status header; options and W1–W8; owner rulings; completion records, including equal-epoch correction at lines 610–616; explicit limitations at lines 647–667 |
| Glade `node/src/mesh.rs` | Module ownership and routing exports, lines 1–99 |
| Glade `node/src/mesh/route.rs` | Routing, lines 27–68; live-claim fold, lines 389–431 |
| Glade `node/src/registry.rs` | Query contract, lines 337–346; ranking/fold, lines 754–775; equal-epoch regression test, lines 1071–1113 |
| Glade-discover `dev-docs/RegistryContractDraft.md` | Entire document, lines 1–164 |
| Process authority | `/Users/owebeeone/.claude/skills/review-loop/SKILL.md` and its canonical `references/review-prompt-template.md`; workspace `AGENTS_GWZ.md` |
| External primary precedent | Burrows’s Chubby paper, §2.2/§2.4/§2.6 |

The Chubby precedent supports the packet’s distinction between coordination and resource-side sequencer checking. It supplies an analogy, not a Glade proof. [Burrows, OSDI 2006](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/).

Only inspection commands were run. No files were written, no tests or builds ran, and no current-round peer report was inspected. A mistyped member SHA in one inspection command failed; the corrected pinned inspection supplied the evidence cited above.

## 1. Findings

### [P3-1] Record convergence alone does not guarantee one live routing answer

**Location:** Packet line 47: “Equal-epoch claim ranking converges on one routing answer once records meet.” Related shorthand appears at lines 194–195.

**Violated invariant:** An evidence summary must preserve the source’s query inputs. Both current folds filter claims by the reader’s `now_ms` before applying deterministic ranking. Their agreement requires the same eligible live-claim set, not merely the same replicated records. See Glade `node/src/mesh/route.rs:389–407` and `node/src/registry.rs:769–775`.

**Reproduction:** Both nodes possess identical claims for S:

1. A has the lower node ID, epoch 1 and expiry 10,000.
2. B has the higher node ID, epoch 1 and expiry 20,000.
3. A evaluates at `now_ms = 9,000` and chooses A.
4. B evaluates at `now_ms = 11,000` and chooses B.

The records have met, yet the routing answers differ. The equal-epoch regression test uses a common evaluation time of zero; it proves consistent ranking, not agreement under differing expiry evaluations.

**Impact:** The evidence row overstates what the existing correction establishes. A later reader could incorrectly expect exchanging records alone to eliminate differing holder answers. This remains P3 because the packet expressly rejects treating claim ranking as partition-safe acquisition or fencing, and RH-05/RH-06 independently require the missing enforcement and clock assumptions.

**Required correction:** Qualify the claim: equal-epoch ranking agrees for the same live eligible records at a common evaluation time; lease filtering remains reader-relative and may produce different routing answers.

**Closure/regression test:** Re-trace the example above against the corrected wording. The summary must permit A/B divergence while still stating that identical eligible sets rank identically. This document review requires no implementation change.

## 2. Invariant analysis

**Authority ownership remains separate from serving placement.** Lines 16–23 and RH-03 do not let a host, replica or voter acquire grant-issuing authority. This agrees with creation-rooted ownership and ancestry-based administration in Authz §3a/GDL-034, and operator-authorized placement in GDL-031. Governance M-of-N approval is correctly distinguished from crash-fault consensus.

**The current routing and receipt baseline is not presented as a transfer protocol.** The packet correctly identifies share-level holder selection and zone-level addresses. Current `route_subscribe` accepts a share, while the served zone remains `(share, glade_id, key)`. The pinned substrate’s forwarded `Ok` means the holder and forwarding node hold the operation under their local, non-fsynced store guarantee; it promises neither quorum durability nor automatic repair. Packet RH-07 and question 3 preserve that distinction.

**Dynamic creation is not disguised manual placement.** H0 is explicitly disqualified by RH-01. H1 selects a host during authorized creation; H2 retains that creation mechanism; H3 creates entries dynamically within an authorized group. Fixed bootstrap or voter configuration is distinguished from per-resource mappings. Existing-scope naming availability is stated honestly rather than silently replaced with disconnected global-name creation.

**Identity and absence semantics withstand the obvious attack.** RH-02/RH-04, H1 and HF-01/HF-02 distinguish competing creation attempts from independently minted roots sharing a human label. A partial empty view cannot authorize replacing a known identity. RegistryContractDraft lines 59–80 independently support exact retry, partial local resolution and the distinction between candidate placement and leadership.

**H2 does not smuggle in unilateral takeover.** Lines 128–146 require a verified cut, successor readiness and actual exclusion of the old generation. Separate machine-local counters are rejected. An unavailable old home or enforcement authority blocks transfer rather than legitimizing a competing store. HF-04/HF-06 test the enforcement and recovery obligations.

**H3 does not equate ownership quorum with replicated data.** Lines 157–170 separately require quorum, successor data readiness and enforceable exclusion. Two voters do not acquire a one-failure availability promise. HF-05/HF-08 expose the distinction between acquiring placement authority and possessing acknowledged history.

**The packet does not silently supersede canonical clauses.** H3 explicitly reopens D-06/R7/R9/R16 for owner review. A separate placement-transition service with projected discovery is distinguished from replacing discovery lookup with a live registry service. R9’s conditional allowance for ordered shard reconfiguration is not presented as existing authorization for consensus-backed resource acquisition. Lines 259–261 reserve exact clause amendments and contract review for the selected design.

That downstream amendment inventory must also examine WorkspaceDirectory §4, DiscoveryModel’s epoch/takeover claims, Authz’s local-decision constraints, and the GDL-036/037/038 management/data seams. The alternatives packet’s general requirement to identify exact amended clauses covers those impacts; it has not claimed that naming the four matrix entries completes an amendment.

**The future verification approach is satisfiable.** Section 6 identifies scenarios rather than claiming implemented tests. Its deterministic contract path, real-adapter crash checks and affected-consumer checks agree with LBT-006–011 and PackageArchitecture §§5–7.

HF-01 through HF-10 collectively address creation races, cold discovery, partitions, delayed old operations, missing acknowledged data, interrupted handoff, revocation/clocks, quorum/reconfiguration, legacy activation and retirement. H1 can answer transfer scenarios as unsupported without pretending to supply relocation. No frozen command family or option surface exists here; retirement/recreation is nevertheless explicitly included in RH-12/HF-10.

## 3. Risks and next action

The hard serialization and enforcement work remains downstream for every eligible candidate. H1 is not automatically inexpensive merely because it avoids consensus: durable naming, creation recovery and resource enforcement still need a concrete design. H2 likewise requires a closed handoff recovery grammar. H3 requires an explicit membership/fault model and a separate data-safety policy.

When producing the selected contract, strengthen HF-01 with crashes between durable naming, host creation and advertisement, including cancelled or lost creation replies. RH-08 already requires that behavior; the implementation suite should make its creation-specific coverage unmistakable.

**Independent conditional recommendation:** Choose H1 if the owner accepts authoritative write unavailability when the fixed home is unreachable and has no near-term relocation requirement. Choose H2 directly if preserving identity during planned moves is required; staging an immutable H1 design first could create avoidable migration work. Choose H3 if automatic failover is required, provided the owner also accepts its quorum deployment and commits to a data guarantee sufficient for the declared failures.

The decisive tradeoff is availability during loss or partition of the old home versus the cost of coordination, fencing and data preservation. Dynamic creation alone does not justify H3, and avoiding coordination does not eliminate H1/H2’s creation-authority obligations.

The owner must decide the placement unit, canonical naming authority, tolerated home outage, acknowledgement failure domain, resource enforcement boundary and acceptable voter/operator trust assumptions. Those answers can select among H1/H2/H3 without selecting a wire format or dependency prematurely.

**Next action:** File this report, correct P3-1’s bounded wording, and combine the independent review verdicts before requesting the owner’s conditional selection. A GO on this axis authorizes no implementation or canonical-contract amendment.

**End verification:** Repeated all three `git rev-parse` revision checks and the root object-blob check. Results remained:

- Root: `c79c73ca6afc371628727c0ca125eb55fd9fe41a`
- Glade: `90dc1a60981185fa26ae5bfafbbb5377c12a413b`
- Glade-discover: `52ea2d118f45d9e7c3d9a789310dd5d669958851`
- Review-object blob: `b23ce879fc2d9b7f9012bce7909b931b9c9dc63a`

The immutable tuple and object blob did not change.
