# Glade resource home alternatives — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeResourceHomeAlternatives.md` at root commit `c79c73ca6afc371628727c0ca125eb55fd9fe41a`; DRAFT decision packet dated 2026-10-03. No alternative selected or implementation authorized.

**Baseline:** Root `c79c73ca6afc371628727c0ca125eb55fd9fe41a`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Repository evidence was read using `git show <exact SHA>:<path>`, with numbered output for citations.

**Date:** 2026-10-03.

**Axis:** Safety — degraded paths, irreversible transitions, disclosure, stuck states, and blast radius. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Start verification:** All three revision pins resolved exactly. The review-object blob resolved to `b23ce879fc2d9b7f9012bce7909b931b9c9dc63a`. The complete generated prompt’s SHA-256 matched `a39ed4b84c9e051ef44e318f98003f3ff8f847220004d5606c2c4407161b0b7e`.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. The packet is fit for an owner decision. This verdict does not ratify an alternative, establish distributed correctness, approve implementation, or amend existing contracts.

---

## 0. Evidence base

The complete Safety prompt was read before reviewing. Workspace instructions in `AGENTS_GWZ.md`, the review-loop skill, and its canonical prompt template were also read.

The substantive evidence examined was:

| Immutable source | Sections or numbered lines examined |
| --- | --- |
| Root: `dev-docs/GladeResourceHomeAlternatives.md` | Entire document, lines 1–261; HF-01 through HF-10 reread at lines 197–215 |
| Root: `dev-docs/glade/GladeAuthzModel.md` | Lines 1–147, particularly §3a and §3b; revocation lines 162–167; zones/verbs lines 169–204; placement/privacy lines 260–280; relevant recorded rulings lines 333–361 |
| Root: `dev-docs/DecisionLog.md` | Usage rule and GDL-031/032/034/036/037/038/042/043; relevant entries at lines 49–66 |
| Root: `dev-docs/glade/GladeWorkspaceDirectory.md` | Entire document, particularly §4 lines 115–139, home-node role lines 108–113, and WD-8 line 274 |
| Root: `dev-docs/glade/GladeDiscoveryModel.md` | Entire document, particularly §0, §3, authorization, specificity and failure scenarios |
| Root: `dev-docs/GladeBuyBuildMatrix.md` | D-06 line 34; R7 line 67; R9 line 69; R16 line 86, with surrounding context |
| Root: `dev-docs/LibraryBoundaryAndTestingPolicy.md` | Entire document, lines 1–108 |
| Root: `dev-docs/GladePackageArchitecture.md` | Entire document, lines 1–160, particularly §5 |
| Glade: `dev-docs/GladeSubstrateV1.md` | Core model and §6; focused rereads of R1–R8 lines 291–494 and W1–W6 lines 495–613 |
| Glade: `dev-docs/GladeCrossNodeWritesPlan.md` | Lines 1–300 and 610–667; holder-directed writes, equal-epoch ranking, and absence of repair |
| Glade-discover: `dev-docs/RegistryContractDraft.md` | Entire document, lines 1–164 |

The optional primary Chubby precedent was checked. Its replicated coordination structure and receiving-server sequencer checks support the packet’s separation of acquisition from effect fencing; they do not establish a Glade implementation. [Burrows, §2.2 and §2.4](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/)

Commands were limited to reading instructions/prompt, hashing the prompt, immutable `git show` inspection, text filtering, revision/blob resolution, and the permitted primary-paper read. No files were written; no Git mutations, tests, builds, runtime operations, or current-round reviewer reports were accessed. Claims/routing code was not required to decide this document’s fitness, and no code acceptance claim is made.

## 2. Invariant analysis

### Dynamic creation does not become permission to invent replacement identities

HF-01 and HF-02 were attacked with this sequence: two clients choose different hosts for the same existing-scope name; one loses its creation reply; a third client sees an empty local directory and attempts to initialize defaults.

The packet rejects all three shortcuts. RH-02 requires stable identity and canonical alias treatment; RH-04 rejects absence inferred from a partial view; RH-08 preserves retry identity and unknown outcomes. H1 explicitly assigns existing-scope naming/retry serialization to an authorized authority and requires durable binding plus resource enforcement (lines 102–124). Independently created roots sharing a human label remain distinct or conflict; they do not merge implicitly.

These are unresolved design obligations, but they are explicitly identified as eligibility gates at lines 192–195. The packet does not present a claim fold or empty registry response as their solution.

### Routing convergence does not establish exclusive admission

HF-03 was attacked by partitioning two hosts, allowing each to see only its own claim, then healing the partition after both have accepted writes.

The pinned cross-node plan records equal-epoch claim ranking at lines 610–615. That establishes consistent ranking after records meet; it does not serialize acquisition during isolation. The packet correctly treats that distinction as load-bearing at lines 47, 75–76 and 192–195.

H1 requires one durable creation binding and an enforcement point. H2 requires a recoverable transfer and fence. H3 requires serialized acquisition and enforceable exclusion of the old generation. No candidate is permitted to replace those obligations with the eventual directory winner.

### A new placement record cannot fence delayed effects

HF-04 was attacked by pausing the old holder after it receives an operation, activating a successor, then resuming the old holder and delivering its delayed request.

RH-05 and RH-06 put validation at authoritative acceptance/effect boundaries. H2’s outline requires an exact cut, successor verification, irrevocable old-generation fencing, and subsequent activation (lines 128–134). Its separate-store qualification rejects independently incremented counters and blocks transfer when required participation or enforcement authority is unavailable (lines 136–146).

H3 likewise makes failover conditional on a real exclusion mechanism, naming leases, quorum-validated acceptance and a shared authoritative sink as branches requiring later specification (lines 157–162). An authorization receipt or router cache alone is insufficient.

The outline is not a proof of a transaction protocol, and the packet says so. Consequently, an irreversible fence followed by a crash remains a required recovery-design case, rather than an implicitly authorized best-effort handoff.

### Ownership quorum cannot certify missing acknowledged history

HF-05 was attacked with three available ownership voters and a successor replica missing an operation already acknowledged by the previous home.

RH-07 expressly distinguishes ownership quorum from data replication, requires a verifiable cut and successor readiness, and forbids successful empty recreation after permanent loss. H3 repeats that voters cannot certify absent data (lines 164–170). The comparison separately lists acknowledged-data guarantees at line 188.

This is consistent with the limited existing acknowledgement promises: Substrate §6 R2 names process-crash survival without fsync, while W4 describes the forwarding and holder stores without promising delivery or a third replica. The cross-node plan explicitly supplies no repair for previously split history or holder loss (lines 657–660).

The packet therefore avoids importing stronger durability into the word `Ok`. It also exposes changed acknowledgement/storage guarantees as an owner question at lines 225–227.

### Crash, cancellation and lost replies cannot roll back committed transitions

HF-06 was attacked at every H2 boundary, particularly loss of the reply after fencing and a restart that retries the original transfer.

RH-08 requires recovery of exact outcomes and persistent generations/retry identities. HF-06 requires old/new/blocked/unknown states without rollback of committed fencing. H2 declares the recovery grammar a later design obligation; H1 answers transfer as unsupported.

That treatment matches the registry draft’s exact-retry and unknown-outcome rules, including the prohibition on interpreting cancellation as rollback and on treating corrupt storage as a fresh scope (RegistryContractDraft lines 21–55). The packet does not claim those independent registry ports already implement a distributed handoff.

### Permission freshness and clock assumptions are exposed

HF-07 was attacked by revoking a candidate or voter while it is partitioned, restarting it with retained proofs, rolling back its clock, and delaying renewal.

RH-03 prevents hosting or voting from creating grant authority. RH-06 requires permission/generation checks at enforcement boundaries and explicit clock/validation assumptions. HF-07 explicitly requires the limits of revocation freshness under partition.

This does not prove immediate global revocation. The canonical authorization model describes forward-only revocation as records replicate, and the registry draft disclaims an atomic distributed revocation barrier. The packet remains decision-fit because it requires the selected design to state those limits and permits explicitly narrower declared guarantees under lines 64–67. A downstream design cannot honestly satisfy that gate merely by validating the signature on a cached proof.

Governance M-of-N approval also remains distinct from crash-fault consensus; selecting a voter does not make it an owner or administrator.

### Membership and data availability remain separate

HF-08 was attacked with one, two and three voters; an isolated old membership; delayed reconfiguration messages; and a newly discovered group claiming the same existing scope.

The packet states that a two-voter majority requires both and identifies the ordinary three-voter configuration for tolerating one voter failure. It requires authorized membership/bootstrap/reconfiguration and rejects independently discovered overlapping groups for an existing identity (lines 164–170).

RH-05 still applies across membership transitions. Crash-fault consensus is explicitly distinguished from malicious authorized voters at lines 84–88. Data readiness remains an additional condition. The packet neither offers arbitrary forced recovery nor promises that a newly assembled majority can replace unavailable history.

### Legacy acceptance cannot run beside the new path

HF-09 was attacked by activating a fenced home while an older node continues using the legacy local/unclaimed-share path.

The pinned substrate W1 preserves that legacy local behavior at lines 528–530. The packet does not claim it is already fenced. RH-11 prohibits an overlapping legacy authoritative path, and HF-09 explicitly requires activation to block overlap and state rollback/recovery preconditions.

This is a consequential migration gate, but migration is not authorized by this document. The packet identifies the dangerous coexistence rather than silently assuming all nodes understand the new generation.

### Retirement is distinct from disappearance

HF-10 was attacked by retiring a resource, reusing its label, deleting local files, and replaying old advertisements and create retries.

RH-12 requires identity/epoch/tombstone treatment and forbids equating local file loss with retirement. HF-10 pairs retirement with reuse and delayed replay; RH-02 and RH-08 preserve identity and retry semantics.

Thus creation has an explicit retirement counterpart at the decision level. No lifecycle omission, misleading command placement, or unspecified user-facing option default was found: this packet freezes no command/API surface, and leaves its later shape subject to review.

### Disclosure and scale do not gain authority through discovery

The metadata attack was an unauthorized precise claim or referral causing a client to traverse or reveal private scope records.

RH-03 preserves scope authority; RH-10 requires bounded referral, verification, metadata retention and retries, plus disclosure policy. HF-02 includes unauthorized peers. Canonical discovery also filters authority before specificity ranking, and the registry draft binds namespace/node/shard/epoch evidence while bounding referral work.

The packet does not specify concrete capacities or proof profiles. Those are downstream choices explicitly exposed as obligations, with no claim that present adapters meet them. No decisive choice between H1/H2/H3 is concealed by that deferral.

## 3. Risks and next action

The principal residual risk is treating this document-level GO as evidence that its eligibility gates have already been met. They have not. In particular, immutable creation binding, resource-specific fencing, recoverable transfer, revocation freshness, membership recovery and data-commit policy need their own reviewed designs and deterministic closure traces before implementation acceptance.

The packet makes H1/H2’s potential stuck states visible: home loss can stop authoritative writes indefinitely; H2 cannot necessarily recover a permanently lost home; an explicit fork is not restoration of the original identity. These are declared availability limits, not concealed safety defects. H3 can also remain unavailable when ownership quorum, trustworthy authority validation, or successor data is absent.

**Independent conditional recommendation:** Choose H1 for the first selected design if the owner accepts a stable home, authoritative write unavailability during its loss, and explicit fork/import rather than automatic restoration after permanent loss. Its decisive benefit is avoiding a transition protocol before dynamic identity, naming and creation binding are sound. This recommendation is conditional on proving those initial binding/enforcement obligations; existing claim ranking is insufficient.

Add H2 only for a concrete requirement for planned cooperative movement and only after its complete crash/retry/fencing protocol is reviewed. Do not use an H1-to-H2 staging label as approval of an unspecified handoff.

Choose H3 at the initial design stage if retaining the same resource identity through uncooperative home failure is a required service guarantee. That choice must include a data-durability policy and enforceable stale-home exclusion; deploying three ownership voters alone does not supply either. Its decisive cost is the additional quorum, membership, freshness and recovery dependency.

The owner decision should record:

- Whether permanent-home-loss recovery and automatic failover are required.
- The placement unit and canonical identity/naming model.
- The acknowledgement guarantee and accepted failure domains.
- The actual fencing boundary for settings, working copies and external suppliers.
- The permission-freshness, fault and deployment assumptions, followed by exact canonical amendments in the selected design.

**Next action:** Obtain and record the owner’s conditional mechanism selection and these guarantee choices; then draft the selected design for its own contract/design review. This report authorizes no implementation or architecture ratification.

**End verification:** Repeated `git rev-parse` checks returned root `c79c73ca6afc371628727c0ca125eb55fd9fe41a`, Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, and review-object blob `b23ce879fc2d9b7f9012bce7909b931b9c9dc63a`, identical to start verification. The immutable review tuple and object did not change.
