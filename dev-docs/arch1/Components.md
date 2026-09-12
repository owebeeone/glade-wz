# Component and interface register

Status: proposed architecture detail, 2026-09-09. Start with
[GladeArchitecture.md](GladeArchitecture.md). Names are proposed compilation units,
not assertions that crates were created. E = existing draft contract, N = new or
extended contract work required. An E does not mean a production implementation.

Revision 2: [injection refinement](InjectionGraphRefinement.md) separates the
independently constructible `DirectoryRules` profile from Directory's live facade.
The `glade-directory` row below is a logical grouping, not permission to construct
the facade from Records' profile dependency or a requirement for a new rules crate.

## Compilation boundaries

All implementations depend on their own contract and only the narrow contracts
they consume. Pure profile/kernel dependencies are explicit exceptions to
"contracts only": reuse is permitted for pure logic, never as a route to I/O.
The node assembly is the only ordinary caller that selects concrete implementations.
Conformance helpers are development-only, and do not import the node.

| Proposed package / role | Owns | Provides / consumes | Excludes |
|---|---|---|---|
| `glade-binding` / pure rules | Canonical declaration, parameter and scope interpretation over supplied catalogue/context | E `BindingResolver` facade; generated declaration/key contract. Async fetching is outside the pure resolver | Grip internals, database/network lookup, creating authority |
| `glade-policy` / pure rules | Scoped policy decisions over supplied, identified evidence | E `TrustPolicy`; N explicit policy-frontier/evidence contract | Directory implementation, evidence fetching, ambient clock, signing keys |
| `glade-admission` / integration library | Ordered decode/profile validation, signature verification, call context and permission composition | N typed admission facade; consumes binding, verification, policy and clock contracts | Physical network/storage, a separate ACL fold, command-specific copied validators |
| `glade-directory` / pure profile plus facade logic | Discovery state machine/profile, claims, expiry, projection and shard-referral rules | E registry and placement APIs; N record-profile/command bridge; current `glade-discover-core` is an explicit pure reuse candidate | Source-write authority, concrete transport/storage, a new generic consensus engine |
| `glade-records` / integration library | Profile-scoped acceptance/retry/recovery, durable handoff and synchronization orchestration | E `AcceptanceJournal`, `DurableOperationStore`, `ReplicaSync`; N profile host and derived-state/outbox commit contract | Shape algorithms, source transactions, a universal wire operation/cursor |
| `glade-delivery` / integration library | Interest accounting, subscription state, profile dispatch, bounded delivery | E subscription draft where compatible; N exact profile adapter/capability contract; consumes replica, admission and budget ports | Universal snapshot assumption, application folds, database implementations |
| `glade-invocation` / integration library | Correlation, dispatch, local waiting and explicit Unknown outcomes | E `Invoker`; N provider-route/epoch view; consumes admission, provider and transport/session ports | Automatic retry of possibly executed effects, source deduplication database |
| `glade-supplier-host` / integration library | Attachment table, epochs, common attach/handoff policy, demand-instance ownership | N attachment/instance host contracts; E `ManagedResource`; consumes admission, directory facade and runtime ports | Application commands, Garns/file schemas, supplier dependencies on node internals |
| `glade-runtime` / integration library with pure policies | Work/resource scopes, supervision and budget arbitration | E `ManagedResource`; N scope owner, clock and budget ports | Discovery/shape/authz/application policy, concrete Iroh/storage dependencies |
| `glade-session` / integration library | Client/peer negotiation, bounded framing orchestration, session ownership and dispatch | N session/duplex-carrier contract; consumes wire codec, admission and selected domain facades | A second implementation of binding or policy semantics |
| `glade-node` / composition executable | Validated configuration, bootstrap construction and selecting/injecting implementations | Constructs the above and concrete adapters; loads ordinary system declarations | Domain algorithms, a public global service locator, library consumers depending on this executable |

The `glade-runtime` ownership and budget policies start as separate modules within
one host library because both account for owned work. If their dependency/test
budgets diverge, split them behind their already separate ports. This is a named
grouping choice, not permission for runtime to absorb domain behavior.

`glade-directory` and `glade-records` MUST NOT both own independent copies of the
authoritative discovery state. The proposed profile host owns one live state
instance and durable revision per profile scope; the directory supplies transitions
and views through its profile interface. Its public facade delegates commands to
that host. The current discovery transaction cannot be dismantled merely to match
these names. AR-04 must prove the bridge; otherwise retain the existing core/driver
inside a bounded directory implementation and record the unperformed extraction.

## Contracts and dependency direction

Existing contract crates remain separate; there is no new mega `glade-api` crate.
The following are **allowed target categories**, not yet exact Cargo allowlists:

| Importer | Allowed lower-level dependencies |
|---|---|
| Pure kernels/profiles | Their minimal contract/data packages; explicitly named pure libraries; canonical shape engines where applicable |
| Orchestration libraries | Their consumed/provider contracts; explicitly named pure kernels/codecs; no concrete I/O adapter or peer orchestration implementation |
| Concrete I/O adapters | Their contract/data packages and the selected vendor/OS/runtime dependency |
| Composition executable | Concrete orchestrators, pure implementations, adapters and contracts needed for explicit construction |
| Consumer/supplier SDK | Versioned declaration and relevant session/client contracts; never the node executable or private database/transport types |

There MUST be no compile-time cycle. Binding and policy do not import the directory
to fetch data. The host supplies identified catalogue/evidence views through narrow
ports or explicit values. Directory, invocation and delivery do not instantiate
one another. Runtime collaboration crosses supplied contracts. Runtime callbacks
are not implicitly safe just because the compile graph is acyclic; ownership and
reentrancy are specified in [the runtime contract](RuntimeAndAssurance.md).

## Concrete I/O adapters

| Adapter boundary | Implementation choice / responsibility | Contract gap to settle |
|---|---|---|
| Iroh carrier | Existing Iroh dependency direction retained; endpoint/connect/accept/send/receive/close behind a carrier port | E discovery `Transport::send` is not a complete endpoint/receive lifecycle. N duplex/session port with size, cancellation and ownership semantics |
| WebSocket carrier | Browser/client transport; same admission and scope rules after framing | N carrier/session contract; not protocol equivalence with arbitrary peer frames |
| Signature/key adapter | Real cryptography/key resolution behind E Signer/Verifier; explicit development root | Key storage, identity binding and trust freshness profile; no accept-anything verifier |
| Node storage adapter | Physical storage implements atomic/durable capabilities required by the selected profile | Three E ports (journal, operation store, snapshot store) are not interchangeable; derived-state/outbox atomicity needs explicit mapping |
| Browser storage adapter | Glial's local persistence and quota/failure reporting | Browser receipt/recovery semantics; no false node-fsync guarantee |

Do not select a new database implementation through this architecture document.
Garns integration is permitted where its exact contract fits, but neither this
architecture nor Gyld's Garns handoff establishes that fit. Store selection must
not alter the replication protocol, and database types MUST NOT leak through
domain contracts. Pin dependency versions, license suitability and required
capabilities before adopting the concrete provider.

## Shape, persistence and application composition

The adapter contract should describe three independently owned things:

1. **Shape/profile**: accepted operations, cursor domain, recovery, retention and
   assembly semantics, plus the persistence capabilities those semantics require.
2. **Storage capability**: atomic batch/snapshot replacement, replay/read access,
   limits, conflict semantics and the supported durability/failure model.
3. **Supplier mapping**: source truth, scope mapping, effects/fencing and how a
   committed source result becomes published data.

A binding validates the combination before activation. A profile may require
atomic accepted-operation retention and causal history; a snapshot-only store
cannot satisfy that merely because both can hold bytes. `snapshot_delta` is not
ordinary SWMR repair, `text_crdt` is not an ordered terminal channel, and `exchange`
does not enter a delivery fold. Reuse canonical engines; keep profile-specific
conformance rows. This defines the factoring, not a completed capability schema.

## Responsibility allocation

Each of the 24 responsibilities in the Gyld problem capture has a lead owner.
"Partners" name explicit cooperation; they are not duplicate ownership or proof.

| Captured responsibility | Lead component | Partners / boundary qualification |
|---|---|---|
| `bootstrap` | Node assembly | Session, key/trust adapters; initial trust must not depend on a working directory |
| `client_sessions` | Sessions | WebSocket adapter, admission, runtime |
| `peer_connectivity` | Iroh carrier | Sessions, runtime; a usable route grants no source authority |
| `schedule_traffic` | Runtime | Per-session/carrier/delivery budgets and explicit backpressure |
| `resolve_definitions` | Binding | Supplied versioned catalogue; host fetches missing data |
| `locate_providers` | Directory | Records profile host, admission, configured or authenticated shard locator |
| `reconcile_metadata` | Records | Directory profile defines permitted metadata transitions/visibility |
| `verify_identity` | Admission | Verifier/key-resolution adapter; immutable origin evidence |
| `enforce_policy` | Admission | Pure policy evaluator; effect boundary reevaluates as required |
| `canonicalize_scope` | Binding | Application-owned mapping and common cross-language vectors |
| `fence_source` | Supplier / source adapter | Supplier host owns attachment epoch; actual resource must reject obsolete writers |
| `invoke_source` | Invocation | Supplier host routes; supplier owns execution/outcome recovery |
| `repair_publication` | Supplier / source adapter | Records publishes with stable identity; source transaction/outbox remains source-owned |
| `retain_replica` | Records | Physical storage adapter supplies declared commit/durability capability |
| `resume_data` | Delivery | Canonical shape adapter, replica read port and Glial consumer assembly |
| `deliver_interest` | Delivery | Glial contributes per-instance interest; supplier host owns instance effects |
| `assemble_consumer` | Glial | Shape engines, browser storage and thin Grip taps |
| `attach_suppliers` | Supplier host | Admission plus runtime scopes; wire-attached SDK, not node imports |
| `validate_ingress` | Admission | Canonical wire/record/profile validators; raw disk input is another observation |
| `own_lifecycle` | Runtime | Each component retains its owned work and cleanup progress |
| `compose_runtime` | Node assembly | Explicit providers/configuration and narrowed dependency views |
| `author_declarations` | Declaration toolchain / application | Existing generated `glade-decl` boundary; runtime files append records, not a second policy language |
| `expose_management` | System suppliers | Ordinary bindings/grants; projection/diagnostics from owning components |
| `verify_integration` | Contract conformance + integration harness | Independent package tests, wiring witnesses and targeted real-I/O journeys |

Requirements stay attached to these responsibilities, not moved into a conveniently
smaller library's scope. This is a proposed allocation of the existing problem
model; the original unallocated Gyld snapshot remains unchanged. The
[Gyld candidate](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py)
encodes the composition and allocations by explicit inheritance/overrides. It is
a separately identified candidate, not a claim of replayed mutation history.
