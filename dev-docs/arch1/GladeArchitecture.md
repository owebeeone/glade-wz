# Glade target architecture — candidate 1, revision 3

Date: 2026-09-09. Status: **assistant recommendation for review; not an adopted
protocol, implemented package tree or completed executable architecture.**

This answers the owner's correction: a problem inventory and implementation
sequence are not an architecture. The deliverable here makes component ownership,
dependency direction, runtime boundaries and verification obligations explicit.
Implementation sequencing follows those choices, not the reverse.

Start with the actual [Gyld architecture declaration](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py),
especially its `GladeArchitecture` composition. This document is supporting rationale.
[Components and interfaces](Components.md) expands the allocation; [runtime and
assurance](RuntimeAndAssurance.md) specifies recovery and the contract-first gate.
The [generated graph](/Users/owebeeone/limbo/gyld-wz/gyld/artifacts/glade-architecture-v3/full-graph.svg)
retains typed relationship labels; it is not evidence of implementation conformance.
All package names below are proposed except where marked existing.

Revision 2 adds [explicit injection and cleanup relationships](InjectionGraphRefinement.md)
and distinguishes the pure DirectoryRules provider from the live Directory facade.
Following the [Shaku/Dill evaluation](DependencyInjectionEvaluation.md), the owner
selected **Shaku at NodeAssembly only**, recorded in revision 3. No domain-contract
dependency, production Cargo change or completed async integration is implied.
GWZ's context was a recovery mechanism, not the desired starting point.
For understanding rather than auditing every edge, start with the
[find / call / follow reading map](ReadingMap.md).

## 1. Architectural decision

**A modular, record-driven node, assembled from independently testable libraries;
not a universal kernel library, and not a required network of microservices.**

One node process is the initial deployment unit. Its domain libraries have narrow
contracts and no concrete transport/database dependency. The composition root
chooses providers and execution scopes. Different nodes can host different roles
without changing application identity or making deployments into protocol types.

The whole stack retains these ownership boundaries:

| Boundary | Owns | Does not own |
|---|---|---|
| Application / Grip | UI/context, application declaration choices, user-visible intent | Peer protocols, replica persistence, shape engines |
| Glial | Per-binding-instance lifecycle, local-first persistence, shape assembly, fan-out to taps; optional Glade connectivity | Source effects, node routing, Grip context/matcher internals |
| Glade client/session boundary | Versioned wire sessions and translation into typed operations/events | Application storage or consumer UI assembly |
| Glade node libraries | Admission, directory, replicated records, delivery, invocation and record-driven supplier lifecycle | Application-specific transactions or source truth |
| Supplier / application source | Source semantics, resource fencing, command deduplication/outcome, commit/publication repair | The directory's grant authority or transport implementation |
| Garns or another source backend | Application-owned storage/transaction machinery behind its supplier | Mandatory storage for every Glade application; database-level Glade replication |

`glade-decl` remains the generated, runtime-free shared vocabulary. Taut owns exact
shape/profile semantics and corpora. Suppliers attach through the wire/client
contract; loopback is a composition optimization with the same authorization and
epoch checks. A tap remains the consumer counterpart, not a supplier.

## 2. Open the node into explicit components

| Component | Architectural responsibility | Independent boundary |
|---|---|---|
| **Binding** | One canonical interpretation of declaration + parameters + domain/scope + supported capability | Pure resolution rules, existing `BindingResolver` draft facade |
| **Admission** | One reusable boundary pipeline for wire, persisted and replayed input; authenticated call context; operation-specific permission checks | Validated values and admitted requests with explicit proof scope; new contract work |
| **Trust policy** | Evaluate supplied identity/grant/revocation evidence for an exact action and source scope | Existing `TrustPolicy`, `Signer`/`Verifier` seams; policy and cryptography stay separate |
| **Directory** | Claims, expiry, provider selection, metadata projections, namespace-to-shard routing policy | `RegistryWriter`/`RegistryReader`, `ShardLocator`; existing discovery state machine is a reuse candidate |
| **Records** | Coordinate canonical acceptance, idempotent recovery, durable handoff and peer reconciliation under a named protocol profile | `AcceptanceJournal`, `DurableOperationStore`, `ReplicaSync`; profile bridge is not yet defined |
| **Delivery** | Interest aggregation, profile selection, subscriptions, explicit gaps and bounded fan-out | `Subscriber`/`Subscription` draft for its supported profile; shape-specific extensions require review |
| **Invocation** | Directed calls, correlation, provider routing and honest unknown outcomes | `Invoker`; source idempotency is not inferred from correlation |
| **Supplier host** | Authenticated attachment/handoff, provider epochs, record-driven demand instances and zero-interest teardown | New common attachment/instance contracts; effects execute in the supplier |
| **Runtime services** | Owned work, shutdown/drain, scheduling and resource budgets | `ManagedResource` plus small ownership/budget/clock ports; no domain policy |
| **Sessions** | Client/peer protocol sessions, reconnect and dispatch to the appropriate domain facade | Transport-independent session logic plus separate WebSocket/Iroh adapters |
| **Node assembly** | Configuration, bootstrap composition and constructing/injecting the components above | Executable/composition root, not the place to implement all their behavior |

This is a proposed module allocation, not one crate per captured function. The
component register names the prospective compilation units and explains the few
internally grouped mechanisms. Contracts remain smaller than implementations.

## 3. Five consequential design choices

**A1 — No second policy authority.** Signed runtime records and their valid folds
remain authoritative. Declaration files seed records; they are not a competing
ACL database. Policy evaluation is pure over supplied evidence. Obtaining evidence
is a host activity, not a hidden network call from inside the policy evaluator.

**A2 — Separate semantic profiles from mechanical acceptance.** Records coordinates
acceptance/recovery; each protocol profile defines canonical identity, validation,
state transition and what must commit together. Storage supplies physical atomicity
and durability. Do not force current discovery operations, application replica ops
and snapshots into one invented universal cursor/transaction. A shared profile host
is a candidate extraction that must first preserve the existing discovery
state/watermark/outbox transaction in a compiler/behavioral witness.

**A3 — Shape × storage × application is composition, not a cross-product of adapters.**
A shape adapter states its required persistence and recovery capabilities. A store
implements capabilities once. A supplier maps application semantics once. The
binder rejects unsupported combinations explicitly. Some combinations are genuinely
incompatible; a capability bit alone cannot prove temporal or durability conformance.

**A4 — Common validation, distinct validity lifetimes.** Decode/structural/integrity
results are reusable for the same immutable value and pinned profile. Fresh input
is validated anew. Current authorization, expiry and source fencing are checked
at their use boundaries. Reusing a structural proof never means caching permission
indefinitely. One semantic implementation per profile, with thin language/wire
adapters and the same canonical corpus—not copied policy logic in each verb.

**A5 — Async ownership at the host boundary.** Recommend retaining the existing
Tokio-based Rust node host initially; pure rules and contract crates do not import
Tokio. Standard futures/direct calls are the default interface. Bounded queues are
used only for independent producers/lifetimes, not between every library. Blocking
I/O gets an explicitly bounded executor boundary. No SDAX clone, actor framework or
macro system is required by the domain APIs.

## 4. Directory growth is an implementation axis

Preserve placement, service discovery and node connectivity as distinct layers.
Start `ShardLocator` with an explicit trusted mapping. The target boundary permits
authenticated referrals and local preference, followed by dynamic group membership
and split/move operations when their protocols are specified. Lookup returns
authorized candidates and coverage/freshness information, not global absence or
proof of reachability.

The directory's **data plane** uses the record profile's acceptance and replication
rules. Its **topology control** owns authorized namespace/group mappings. Ordered
coordination, if required for reconfiguration, belongs behind that latter boundary;
it does not make every advertisement a consensus write. Registry leadership never
confers source-write authority. These are logical concerns, not a privileged admin
API: GDL-038 management still uses ordinary bindings and grants.

The million-node ambition constrains bounded per-node indexes, referral work,
connections, churn and fan-out; it is not an achieved guarantee. The failure model
and reconfiguration algorithm remain open. This candidate does not select Raft or
claim crash-fault consensus is protection against malicious members.

## 5. Why this decomposition, rather than the nearest alternatives?

| Alternative | Tradeoff behind this recommendation |
|---|---|
| Keep all behavior in one node crate with internal traits | Easy initial linking, but compilation dependencies and test scope remain coupled. Separate replaceable/domain boundaries where independent feedback is valuable. |
| Deploy each responsibility as a service | Adds wire/version/lifecycle and operating obligations before distribution is needed. Library boundaries first; explicit remote contracts where suppliers/peers already require them. |
| Build a universal graph/plugin/event-bus kernel | Hides concrete ownership and invalid combinations behind registration. Explicit typed composition is more reviewable for this bounded system. |
| Separate adapter for every shape/backend/application | Repeats mechanics and increases conformance combinations. Capability-oriented composition, with exact profile gates, should reduce duplication without claiming all combinations work. |
| Split every discovery submodule immediately | Risks tearing apart its existing transactional/state-machine invariants. Keep the current core as a unit until an extraction witness proves equivalent behavior. |

No costs were measured and no Gyld score selected a winner. These are explicit
engineering judgments that can be challenged, not an optimization proof.

## 6. What makes this an implementable architecture?

The proposed allocation accounts for all 24 captured responsibilities; see
[the mapping](Components.md#responsibility-allocation). It assigns durable state,
authority, cleanup and failure outcomes—not just library names. It also identifies
where the existing thirteen draft contracts are insufficient.

Before an implementation workstream starts, its boundary MUST have compilable
consumer/conformance specifications, an independently runnable test target, allowed
dependencies and a reviewed fixture matrix. Existing draft tests are inputs, not
automatic approval of a new composition. [AR-01–10](RuntimeAndAssurance.md#architecture-acceptance)
are the proposed acceptance witnesses; they are **not implemented by this document**.

An adversarial review of this candidate and those executable contracts precedes
implementation. This task creates the design recommendation only; it does not
launch reviewers or implement skeletons. The [build entry](../GladeBuildEntry.md)
now puts this architecture gate before the registration/discovery milestone.

## 7. Sources and authority

This extends the [captured problem frame](/Users/owebeeone/limbo/gyld-wz/dev-docs/case-studies/glade/ProblemSpaceCapture.md),
not its claimed evidence maturity. Adopted GDL-031–038/040–042 remain constraints;
GDL-039 and the draft interface/profile decisions retain their open status.
The [decision log](../DecisionLog.md) remains the status authority.

Additional source reads for this candidate: [Glial client runtime](../glial/GlialClientRuntime.md),
[declaration surface](../glade/GladeDeclSurface.md), [system-data seam](../glade/GladeSystemDataSeam.md),
[application contract draft](../../glade/dev-docs/ApplicationContractDraft.md), and
[registry contract draft](../../glade-discover/dev-docs/RegistryContractDraft.md).
The existing discovery core exports and node-adapter driver state/commit declarations
were inspected; this was not a fresh implementation audit or test run.

The earlier discovery-focused [package proposal](../GladePackageArchitecture.md)
remains useful history and a record of adopted constraints. This whole-stack
candidate is the current proposal for review, not a silent ratification of its
new package names, type designs or profile extraction.
