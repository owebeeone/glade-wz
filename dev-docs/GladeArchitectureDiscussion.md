# Glade — Architecture Discussion Record

Date: 2026-09-05. Status: **working discussion record**.

This records the owner/assistant conversation following the two application-stack reviews and [problem inventory](/Users/owebeeone/limbo/glade-wz/dev-docs/GladeProblemInventory.md). It distinguishes agreed direction, the owner's proposed architecture, reviewer suggestions, and unanswered questions. Canonical decision status is recorded separately in [DecisionLog.md](DecisionLog.md), including GDL-042/GDL-043 for the later package and registry discussion. Discussion alone does not assert implementation.

## 1. Discussion status

| ID | Subject | Status |
|---|---|---|
| GAD-01 | Independent users and devices, explicitly granted trust, intermittent connectivity | **AGREED — operating environment** |
| GAD-02 | Trust as a separate problem developed alongside core Glade | **AGREED — work separation** |
| GAD-03 | Automatic development grants with genuine signing and verification | **AGREED — initial approach; exact library contract unselected** |
| GAD-04 | One Iroh-based substrate, dynamically sharded registries, local access, indexes for remote shards | **OWNER PROPOSAL — recorded for architectural evaluation** |
| GAD-05 | Replica groups, delegated referrals, bounded indexes, and limited use of consensus | **REVIEWER REFINEMENTS — awaiting discussion** |
| GAD-06 | What successful registration guarantees when replicas are unreachable | **DISCUSSED — renewable local advertisements proposed; exact receipt contract open** |
| GAD-07 | Minimal dependencies, meaningful library interfaces, TDD and fast isolated tests | **OWNER REQUIREMENTS — recorded in the reusable policy** |
| GAD-08 | Package graph, contract extraction, composition root and staged integration | **REVIEWER PROPOSAL — exact decomposition unselected** |
| GAD-09 | Reusable policy, AGENTS reference and automated architecture checks | **OWNER REQUEST — initial discovery-local gate added; wider adoption open** |

Agreement here records the conversation. It is not a claim that every detail has been promoted into a stable specification. The owner has not yet accepted GAD-05 as a complete design.

## 2. GAD-01 — Operating environment

The owner answered yes to an environment where multiple independent users share domains across multiple devices. Devices can disconnect for extended periods. Hosted nodes and relays may improve availability. Their infrastructure role alone confers no authority over a domain.

Trust is granted for particular responsibilities: carrying traffic, holding replicas, serving reads, publishing particular data, and executing source commands. Trust is directional and scoped; permission to publish one category does not imply permission to publish another or mutate its source.

This resolves the broad operating-scope question in the inventory. It leaves first-release deployment coverage, scale measurements, and detailed trust policy open.

## 3. GAD-02/GAD-03 — Separate trust work, with a usable development boundary

The owner clarified the intended sequencing: **“It's not so much defer as it is — break it off as a separate problem — work on it in the bg but get the core glade infra working.”** The assistant accepted that distinction.

The agreed approach is a separate trust library/contract whose policy can develop alongside core infrastructure. Early development can use an explicitly installed development trust root and automatic issuance of scoped grants to known development identities, while exercising real signatures and verification. The discussion did not select a package name, implementation language, token encoding, or complete delegation protocol.

The owner's JWT analogy identified two questions: who may publish which kinds of data, and whether a received object is trustworthy. The discussion decomposed these into:

1. **Origin and integrity:** which key signed this statement, and were the signed bytes altered?
2. **Authority:** does the receiver accept a grant/delegation permitting this signer to make this statement or perform this operation?
3. **Data validity:** do schema, shape, causal order, source revision, and application checks accept this particular object?
4. **Current applicability:** are the grant, provider term, revocation knowledge, and information freshness adequate for the attempted use?

A signature alone does not answer all four. The low-level ability to sign bytes is separate from authorization to obtain a grant under an issuer that peers trust. The development shortcut concerns grant issuance policy; it does not mean that the verifier accepts arbitrary issuers, identities, scopes, or expired claims.

Three proposed library responsibilities were identified: signing/verification, authority evaluation from supplied evidence, and grant issuance/renewal/revocation. An evaluator outcome of `allow | deny(reason) | need_evidence(description)` was suggested. This interface is still a proposal, including the way Glade obtains missing evidence.

Expiry also has separate meanings. Permission for a new operation may expire; an already accepted historical operation does not automatically disappear with that grant; a provider availability claim has its own lifetime. Renewal, historical validation, retroactive consequences of revocation, key rotation/recovery, and disconnected policy freshness remain part of the trust investigation. JWT was an analogy, not a serialization decision.

Existing B1–B5 requirements still constrain the interface: scoped attachment, authenticated context, identity-bound zones, validated decoding, and signed governance. [Security ruling record](/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:630)

**Execution status:** no separate trust task or library implementation has been launched in this conversation. The parallel workstream is agreed as an approach; it should not be described as already running.

## 4. GAD-04 — Owner's discovery and scaling proposal

The owner proposed Iroh/P2P as common infrastructure for both client equipment and data-centre equipment. The namespace can be sliced into registry shards, with new registries created as growth requires. Most advertisements/registrations should go to a particular node, preferably nearby. Requests concerning another shard locate its responsible node through distributed indexes, minimizing repeated search.

The proposal includes these intentions:

- **One substrate across deployment sizes:** the same underlying system should support a small peer network and, as a design target, a backend of roughly one million machines.
- **Dynamic namespace sharding:** distribute responsibility and allow it to grow instead of maintaining one registry for everything.
- **Locality:** optimize normal registration and lookup for local or nearby service.
- **Distributed routing knowledge:** nodes retain some index information that identifies the relevant remote shard.
- **Authenticated registries:** an attacker must not gain namespace authority merely by publishing a registry or signing its own claims.
- **Possible elected responsibility:** investigate the leader-election/coordination pattern used in large distributed systems, including whether it should operate in tiers.

The scale statement is a target, not a demonstrated capacity result. “One substrate” means shared protocols/runtime capabilities, not a single process, a single leader, or a million-member voting group.

The proposal does not yet answer the earlier question about open discovery of previously unknown domains or providers. Sharding locates responsibility within a namespace; how a participant first learns and trusts a namespace remains a separate issue.

Related inventory entries: GPI-01–06, GPI-08–10, and GPI-13. The inventory's original categories remain useful, but namespace partitioning, index topology, placement, and shard reconfiguration now need explicit architectural treatment.

## 5. GAD-05 — Reviewer refinements, not yet adopted

The assistant recognized the proposal as a partitioned, replicated registry with local access and indexes routing to responsible shards. The following refinements were suggested:

| Refinement | Reason / boundary |
|---|---|
| A shard is served by a replica group, with a preferred node or leader where appropriate | Preserves state and permits recovery when one serving node disappears. Replica count and write protocol remain open. |
| Separate `resource identity → shard → current replica endpoints` | Allows relocation and splitting without redefining application identity. Physical locality and logical partitioning are different choices. |
| Cache bounded referrals and relevant indexes | Avoids requiring every device to store every registry entry or receive every global membership update. Exact hierarchy, coverage, and bounds remain open. |
| Authenticate namespace delegation and shard-group membership | A signature from an arbitrary registry is insufficient. The receiver needs a trusted authority's delegation for the relevant namespace and role. |
| Apply ordered coordination to decisions that actually need it | Exclusive shard ownership and shard reconfiguration may need consensus; independently signed, mergeable advertisements need not all use the same protocol. |
| Keep source fencing separate from registry leadership | Registry takeover cannot itself prevent an obsolete provider from mutating a filesystem or database. |

Illustrative topology from the discussion:

```text
Trusted namespace anchor
          |
   delegated shard index
      /           \
 shard group A   shard group B
      ^
 local client/cache
   (consult index when its mapping is missing or stale)
```

Anchors and indexes are logical roles; this sketch does not select one unreplicated root machine. Small deployments may colocate roles that larger deployments separate. The permitted loss of availability under different configurations must be explicit.

Chubby/Paxos and Raft were identified as relevant coordination examples, not selected dependencies. Ordinary crash-fault consensus does not establish trust in arbitrary malicious voters. A decision about registry participants and the failure model precedes choosing a protocol. [Chubby](https://research.google/pubs/the-chubby-lock-service-for-loosely-coupled-distributed-systems/), [Raft](https://raft.github.io/raft.pdf)

Iroh provides endpoint connectivity and address discovery; the proposed namespace/shard registry sits above it. This preserves GDL-032's separation of placement, service discovery, and node discovery. Registry facts should remain compatible with the record/fold authority of GDL-037. A separate authoritative configuration database or a change to replication semantics would require an explicit decision. [Iroh boundary](https://docs.iroh.computer/about/faq), [GDL-032](/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md:50), [GDL-037](/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md:55)

The million-machine target requires evidence that routing state, connections, update fan-out, and per-node work remain bounded under growth and churn. Hot shards, migration, stale referrals, and adversarial load are still evaluation cases, not solved by drawing a hierarchy.

## 6. GAD-06 — Ephemeral registry and unreachable replicas

The question was clarified to mean a publisher can reach registry A, but A cannot confirm replication to designated shard peers B/C. This differs from a publisher being unable to contact any registry, an unreachable remote shard, or an advertised provider failing to answer.

The owner identified the registry as ephemeral and asked for proven strategies. The assistant recommended renewable provider advertisements, local acceptance without a synchronous replication quorum, asynchronous replication, idempotent renewal/retry, and reconciliation after partitions. Discovery results are locally known candidates, not global completeness or guaranteed reachability. Forwarding MUST NOT renew a claim's expiry; unreachability is an observation rather than revocation. [Eureka precedent](https://github.com/Netflix/eureka/wiki/Understanding-Eureka-Peer-to-Peer-Communication), [SWIM failure-detection separation](https://www.cs.cornell.edu/projects/Quicksilver/public_pdfs/SWIM.pdf).

Authority/delegation records, source fencing and restart/sequence protection are not disposable merely because availability advertisements are ephemeral. A later source check clarified that existing discovery v3.1 requires durable local persistence before acceptance/gossip. The proposed local receipt does not relax that rule. Exact receipt types, retention and disconnected freshness remain open under GDL-043; no new wire contract has been ratified.

## 7. GAD-07/GAD-08/GAD-09 — Independent libraries and reusable enforcement

The owner asked to wrap the responsibilities in interfaces and stage working implementations, with interfaces subject to TDD. They then strengthened this: each library should have minimal dependencies and fast independent tests, without a massive test run for minor changes. These are owner requirements, not merely reviewer preferences.

The assistant proposed small contract packages, independent logic libraries and a node composition root, with separate I/O adapters. Rust traits were explained as interface equivalents; crates, unlike modules, provide a separate compilation boundary. The exact package names and extractions remain proposals, captured with the dependency diagram and stages in [GladePackageArchitecture.md](GladePackageArchitecture.md).

The owner requested a written record, an AGENTS reference, and build-failing checks for omitted traits/boundaries. They subsequently requested a reusable document for another agent. [LibraryBoundaryAndTestingPolicy.md](LibraryBoundaryAndTestingPolicy.md) is now the shared engineering policy; the Glade document references it rather than maintaining a duplicate policy.

The initial automated gate is scoped to `glade-discover`'s primary-workspace libraries. It checks classifications, declared dependency edges, named public required-method traits, explicit implementation declarations where applicable, and conformance-target presence. Compiler/behavioral tests and review are still needed; the gate cannot infer semantic intent or prevent an authorized writer from deleting policy. Existing pure/protocol/harness boundaries have named rationales; port extraction is not implemented. CI workflow changes are local files, not a claim of a completed hosted run or configured branch protection.

## 8. Where the discussion resumes

The next work is to settle the concrete package/API boundaries and useful first live integration without weakening current contracts. A separate trust task has still not been launched.

Additional unresolved items, to work through individually:

- What a registration stores and who owns that record; distinguish local receipt, durable acceptance, and discoverability.
- Namespace introduction, trust anchors, and whether open/public discovery is required.
- Logical shard boundaries versus placement/locality policy.
- Index hierarchy, cache freshness, referral validation, and stale-route repair.
- Shard split/merge/migration, authorized group membership, and required ordering.
- Development trust contract, separate trust-workstream scope, and its actual execution setup.
- Interface/data ownership, extraction compatibility, per-package feedback budgets, and adoption of the shared gate in other repositories.

The record is adequate to continue requirements discussion without losing the intent or attributing reviewer suggestions to the owner. It is not yet adequate to implement a stable registry/consensus contract or claim scalability. Those require the unanswered guarantees above and explicit validation scenarios.

## 9. GAD-10 — Interface-first tranche while lifecycle research proceeds

The owner clarified that SDAX is an example of orchestrated cleanup and useful
concurrency, not a mandated Python port. Rust lifecycle alternatives and declarative
API designs are being explored separately. Channels, actors, macros and a particular
orchestration engine are not selected by this discussion.

The owner then requested actual traits and canonical TDD tests now. The first
tranche adds three independent draft contract crates for transport, signatures and
canonical operation storage, leaving the existing adapter, kernel and demo intact.
No production implementations were added. The higher-level discovery receipts,
trust-policy evidence, shard/referral contracts and atomic accepted-intent/clock
transaction remain open rather than being guessed into an interface.

The tests were written first; the interface scaffold reached RED before contracts
were supplied. Independent adversarial review found and resolved a protocol-hash
versus signed-byte storage ambiguity and gaps in failure/lazy-I/O checks. Exact
requirements, review disposition, limits and fast commands are recorded in
[DraftHostContracts.md](../glade-discover/dev-docs/DraftHostContracts.md), tracked
under GDL-044. Draft test-fixture success is not production conformance or owner
ratification of the broader architecture.

## GAD-11 — Acceptance and discovery contracts before implementation

The owner authorized steps 1–3: draft atomic acceptance/recovery semantics,
publish/renew/resolve interfaces, and separate trust-policy and shard-location
interfaces, each backed by canonical tests. These are added alongside the existing
implementation; the demo is not migrated. Local durable receipts do not promise
replication, a globally complete registry, or resource reachability.

Independent adversarial review prompted complete-source authorization checks,
node/shard/epoch binding checks, trust expiry checks, restart-safe acknowledgement
tests and explicit per-operation authorization reevaluation. The reviewer found
no remaining blocker for the draft-only tranche. Storage crash testing, actual
concurrent execution, cryptographic namespace delegation and production adapter
approval remain separate obligations. Details and traceable requirements are in
[RegistryContractDraft.md](../glade-discover/dev-docs/RegistryContractDraft.md),
tracked under GDL-045. This does not select channels, macros or an sdax-like runtime.
