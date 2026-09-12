# Application Stack Contract Decisions

Status: owner decision worksheet

Evidence snapshot: 2026-09-05

Related review: `dev-docs/ApplicationStackContractReview.md`

## How to use this worksheet

These are semantic owner decisions needed before a stable application-stack contract is published. They are deliberately not an implementation backlog. Each decision should be answered with normative language, recorded in the owning decision log, and referenced by contract tests. The identifiers here are stable cross-references for review findings.

The first six decisions are on the contract-design critical path. Work on engine conformance, test fixtures, durable outbox experiments, and adapters behind unstable interfaces can proceed independently, but their schemas must not be declared stable until the relevant owner decisions land.

Existing constraints are not reopened here: GDL-031–038 and GDL-041 are ratified; GDL-040 is ruled; GLP-0006 records ratification of its approximately forty worksheet rows. GDL-039 is explicitly still pending ratification (`dev-docs/DecisionLog.md:49-59`).

## ASCR-D01 — Factor the contract into definition, binding, and advertisement

**Status:** proposed; critical path

**Owner:** Glade architecture / `glade-decl` contract owner

**Depends on:** GDL-035, GDL-037, GDL-038, GDL-041

**Findings:** ASCR-F01, ASCR-F04

**Exact question**

MUST the declarative stack contract distinguish a stable `SurfaceDefinition`, a durable `BindingInstance`, and a volatile leased `ProviderAdvertisement`, with explicit references between them?

**Why it matters**

The current `BindingDecl` contains identity, shape, authority, domain, zone, and retention, while Glial's `Surface` adds concrete share/key fields and provider state lives elsewhere. Adding source, schema, route, grant, and freshness fields to the same object would combine facts that have different writers and lifetimes. It would also let generated manifests become a competing control plane.

**Alternatives**

1. Extend one flat `BindingDecl` with all definition, placement, source, provider, and policy fields.
2. Use the three linked record levels proposed by the review.
3. Keep separate per-layer configurations and reconcile them operationally.

**Recommendation and tradeoff**

Choose alternative 2. It adds explicit joins and version references, but gives each fact one authority and permits provider turnover without redefining an application surface. Alternative 1 is easier to serialize initially but bakes in conflated authority. Alternative 3 preserves current flexibility at the cost of permanent drift.

**Affected contracts**

`glade-decl` IR and generated bindings; `.glade` loader; RegistryApi record kinds; Glial manifest/fill resolution; provider discovery records; migration/version rules.

**Validation scenario**

Change a provider route and authority term while keeping a binding and surface definition unchanged; change a binding's domain while keeping the definition unchanged; reject an advertisement that refers to an unknown definition version.

## ASCR-D02 — Separate source authority from replica persistence

**Status:** proposed; critical path

**Owner:** Glade supplier and store contract owners

**Depends on:** ASCR-D01, GDL-036, GDL-040, GDL-041

**Findings:** ASCR-F02, ASCR-F07

**Exact question**

MUST suppliers implement distinct typed `SourcePort` and `ReplicaPort` roles, even when one process hosts both, and MUST replica capability be insufficient to confer source-write or command authority?

**Why it matters**

Source systems know source revisions, transactions, commands, and authoritative external changes. Replica stores know attributed operations, checkpoints, compaction, retained cursor floors, and replay. A symmetric bidirectional interface makes it too easy to treat restored replica state as writable source truth.

**Alternatives**

1. One universal `get/put/subscribe` supplier interface.
2. Separate source and replica ports under a common lifecycle/capability envelope.
3. Define no standard ports and keep every supplier bespoke.

**Recommendation and tradeoff**

Choose alternative 2. It requires more explicit adapter code but makes authority reviewable and enables common conformance tests. A single process MAY implement both interfaces only when it advertises and is authorized for both roles.

**Affected contracts**

Supplier kit; Glade `StoreApi`; Glial hydration/persistence boundary; shape adapters; provider capabilities; Garns, file, terminal, and other source adapters.

**Validation scenario**

Restore a complete replica and prove it can serve authorized history but cannot execute a source command. Then run the same process with an exact source-authority assignment and show that command capability appears only for that fenced term.

## ASCR-D03 — Define provider advertisement validation and role vocabulary

**Status:** proposed; critical path

**Owner:** Glade security/discovery owner

**Depends on:** ASCR-D01, ASCR-D02, GDL-031–034, GDL-038, GLP-0006 B1-B5

**Findings:** ASCR-F03, ASCR-F08

**Exact question**

What exact signed fields and validations MUST make a provider advertisement eligible for routing, and which roles are standard: source authority, replica reader, command provider, and supplier host?

**Why it matters**

The current node can replace an exchange provider entry without exact grant or authority-term validation, while the ratified supplier model requires authenticated principal context, exact attach grants, and epoch fencing. Discovery must not become an authority-escalation path.

**Alternatives**

1. Treat possession of a route/session as sufficient provider authority.
2. Validate a signed, leased advertisement against binding, exact grant, authenticated principal, assignment/term, shape/capability, freshness, and nonreplacement rules.
3. Permit unsigned local advertisements and add validation only for remote routes.

**Recommendation and tradeoff**

Choose alternative 2 for all routes. Local deployment may use a local trust root, but not a weaker semantic record. This costs validation and lease management while eliminating a local/remote contract fork. The advertisement grants no authority; it is accepted evidence under existing authority records.

**Affected contracts**

Serve/service claims; exchange attachment; glade-discover projection; session handshake; provider call context; route selection; lease expiry; takeover and withdrawal; audit events.

**Validation scenario**

Reject advertisements with a wrong grant, caller principal, binding, shape version, expired lease, or old epoch before the routing fold changes. Accept a newer authorized term and prove the old provider cannot execute after turnover.

## ASCR-D04 — Ratify domain/zone mapping and define Garns scope mapping

**Status:** proposed; critical path; extends open GDL-039

**Owner:** Glade domain/identity owner with Garns contract owner concurrence

**Depends on:** GDL-031, GDL-033, GDL-034, GDL-039

**Findings:** ASCR-F05

**Exact question**

MUST a binding map `domain -> share identity` and `zone -> key/replication partition` as currently implemented, and how MUST an authenticated Glade principal/domain/parameter tuple map to a Garns qualified link path and `_scope` without equating the two namespaces?

**Why it matters**

The domain/zone wire mapping is implemented but not ratified. Garns scope is relational access identity, while a Glade domain is an application sharing boundary. Treating one as the other would either leak data across Garns scope or multiply Glade domains for internal relational details.

**Alternatives**

1. Ratify the current domain/share and zone/key mapping and require an explicit per-binding Garns scope mapping.
2. Make Garns scope itself the Glade domain.
3. Leave mappings supplier-defined and opaque.

**Recommendation and tradeoff**

Choose alternative 1, after resolving GDL-039's remaining account/domain anchor and axis questions. The explicit mapping is more verbose but is reviewable and portable. It MUST be deterministic, versioned, and evaluated after transport authentication and Glade authorization.

**Affected contracts**

`BindingInstance`; domain and zone records; key selection; parameter canonicalization; Garns mapping artifact; authorization test fixtures; account-private and document-private bindings.

**Validation scenario**

Two authenticated principals query the same surface and parameters but map to different Garns scopes; each receives only its authorized result while sharing the same surface definition. A zone/key change does not alter Garns relational identity.

## ASCR-D05 — Adopt reference-based Garns/Glade declaration composition

**Status:** proposed; critical path

**Owner:** Garns and Glade contract owners jointly

**Depends on:** ASCR-D01, ASCR-D04, GDL-035, GDL-037, GDL-041

**Findings:** ASCR-F04, ASCR-F05, ASCR-F06

**Exact question**

MUST Garns and Glade compose through a versioned mapping artifact that references Garns world/question/command/result contracts and Glade surface definitions, rather than either language generating and owning the other's declaration?

**Why it matters**

Garns owns relational identity, transactions, live batches, capture, and result schemas. Glade owns domains, grants, binding records, routing, replication, and provider lifecycle. Wholesale code generation in either direction duplicates semantic ownership and makes non-Garns sources second-class.

**Alternatives**

1. Generate the complete `.glade` declaration from Garns.
2. Generate Garns queries/adapters from a Glade surface declaration.
3. Use a small mapping artifact with exact references; generate only duplicate boilerplate and conformance material.
4. Hand-maintain both configurations.

**Recommendation and tradeoff**

Choose alternative 3. It preserves one owner per fact and supports fail-closed version negotiation. It requires Garns to version the currently missing portable IR/result/transaction/live contracts before a stable adapter can be published.

**Affected contracts**

Garns contracts; `garns-rust` artifact IDs; `glade-decl`; generator manifests; mapping schema; app package tooling; result/change encoders; conformance suites.

**Validation scenario**

Compile one Garns question and command mapping into TypeScript and Rust adapter bindings. Both refer to identical Garns and Glade contract IDs, reject an unknown version, and produce byte-equivalent canonical mapping metadata without duplicating domain or grant policy.

## ASCR-D06 — Specify transactional publication and idempotency

**Status:** proposed; critical path

**Owner:** Garns transaction owner and Glade operation-ingest owner jointly

**Depends on:** ASCR-D02, ASCR-D03, ASCR-D05

**Findings:** ASCR-F06, ASCR-F10

**Exact question**

MUST an authoritative Garns transaction commit a durable publication outbox entry atomically with source state and ledger, and MUST Glade deduplicate a stable publication ID while rejecting content-changing reuse?

**Why it matters**

There is no atomic transaction across a source database and Glade replicas. Publishing after commit without a durable intent loses changes on crash; republishing without idempotency duplicates effects. Garns ledger sequence, Glade operation identity, and shape cursor cannot safely substitute for one another.

**Alternatives**

1. Best-effort publish after transaction commit.
2. Distributed two-phase commit between Garns and Glade.
3. Transactional outbox plus idempotent, fenced Glade publication.

**Recommendation and tradeoff**

Choose alternative 3. It is eventually published rather than globally atomic, but it has a simple recoverable failure model. The publication record should carry stable ID, source revision, contract version, canonical payload hash, and authority term. Commands should carry an idempotency key and optional base revision.

**Affected contracts**

Garns transaction/live contracts; supplier-private store; source port; Glade append acknowledgement and deduplication; command exchange; audit/diagnostics; retry policy.

**Validation scenario**

Inject crashes before Glade append, after append but before acknowledgement, and after acknowledgement. In every case the source command takes effect once, the outbox eventually clears, one canonical delta is visible, and publication ID reuse with different bytes is rejected.

## ASCR-D07 — Place retention, checkpoint, and deletion authority

**Status:** proposed; needed before durable non-demo bindings

**Owner:** Glade persistence/shape contract owner

**Depends on:** ASCR-D01, ASCR-D02, GDL-036, GDL-041, GLP-0006 F-GAP10 and file rulings

**Findings:** ASCR-F07, ASCR-F09

**Exact question**

Which record owns retention policy, which shape-specific acknowledgements permit compaction, and how are cache drop, instance teardown, retention expiry, and authoritative deletion distinguished?

**Why it matters**

Retention is separate from shape, but it is constrained by recovery semantics. Logs need a retained cursor floor, SWMR needs a checkpoint/base, and CRDT pruning needs causal acknowledgement. Current code has persistence seams but no complete retention consumer, and Glial unmount can drop stored instance state.

**Alternatives**

1. Let each store compact opportunistically.
2. Put retention in the surface definition.
3. Reference retention from the binding and require shape-specific checkpoint/ack evidence before destructive compaction.

**Recommendation and tradeoff**

Choose alternative 3. Stable recovery capabilities remain in the definition; deployment policy remains in the binding; current retained ranges and checkpoints remain runtime state. This adds state and acknowledgement protocols but avoids lying about replay. First reconcile the GLP-0006 final ratification record with subsidiary plan text that still calls blob/retention choices open.

**Affected contracts**

Binding retention reference; shape recovery contracts; StoreApi; ReplicaPort; checkpoint and cursor records; Glial store/drop behavior; deletion/tombstone records; file/terminal histories.

**Validation scenario**

Attempt log, SWMR, and CRDT compaction without the required evidence and observe fail-closed behavior. After valid checkpoints/acknowledgements, reconnect an old client and receive either correct replay or an explicit typed reset—not silent truncation.

## ASCR-D08 — Define offline effect and revocation semantics

**Status:** proposed; needed before writable remote applications

**Owner:** Glade authorization and client-kernel owners

**Depends on:** ASCR-D02, ASCR-D03, ASCR-D07, GLP-0006 B4-B5

**Findings:** ASCR-F07, ASCR-F08

**Exact question**

When disconnected from a remote authority, which interactions MAY be durably queued as intent, which MUST fail immediately, and what result language distinguishes pending, committed, conflicted, rejected, and revoked outcomes?

**Why it matters**

Local-first replica persistence is not proof of a remote effect. Reporting an offline command or file save as successful before authority execution breaks both user expectations and authorization. Revocation also cannot recall bytes already delivered to an offline device.

**Alternatives**

1. Optimistically report local append as success for every shape.
2. Reject every interaction while offline.
3. Allow only explicitly declared queueable intents and expose their lifecycle; never report the authority effect as committed early.

**Recommendation and tradeoff**

Choose alternative 3. Value/CRDT interactions may be locally meaningful under their authority model; exchange commands and external source writes remain pending until authority acknowledgement. The UI contract must expose conflicts and rejection. Document the unavoidable offline revocation limit.

**Affected contracts**

Interaction definitions; Glial controller result types; local outbox; exchange outcomes; source port; revocation handling; file save and Garns command UX.

**Validation scenario**

Queue a permitted command intent offline, revoke the principal before reconnect, and prove it becomes rejected rather than committed. Attempt a non-queueable command offline and receive a typed refusal without source or replica mutation.

## ASCR-D09 — Make live-process survival a declared lifecycle profile

**Status:** proposed; needed before terminal contract stabilization

**Owner:** terminal supplier and Glade lifecycle owners

**Depends on:** ASCR-D01, ASCR-D03, ASCR-D07, GLP-0006 terminal rulings

**Findings:** ASCR-F07, ASCR-F09

**Exact question**

MUST every live-process binding declare one of `disconnect_closes`, `grace_period`, or `externally_managed_reattach`, and MUST process survival remain independent from retained output-log recovery?

**Why it matters**

Terminal output can be durably replayed while the PTY is irrecoverably gone. Treating scrollback recovery, client reconnect, provider turnover, and process migration as one capability produces false promises and unsafe input routing.

**Alternatives**

1. Define one global terminal timeout and assume process survival within it.
2. Leave behavior supplier-specific and undocumented.
3. Declare a lifecycle profile per binding; require additional proof/capability for external process reattachment.

**Recommendation and tradeoff**

Choose alternative 3. It complicates the binding slightly but makes user-visible behavior testable. Ordinary host turnover MUST NOT claim PTY migration. Input-driver handoff is separately fenced by an authority term.

**Affected contracts**

Binding lifecycle reference; terminal definition; provider capabilities/freshness; process-private state; output generation/offset; input driver epoch; UI status.

**Validation scenario**

For each profile, disconnect and restart client and supplier independently. Verify the declared outcome, retained scrollback behavior, process generation, and refusal of input from an old driver term.

## ASCR-D10 — Publish a migration and support-status rule

**Status:** proposed; needed before declaring version 1 stable

**Owner:** Glade declaration/runtime owners

**Depends on:** ASCR-D01–D07

**Findings:** ASCR-F09

**Exact question**

How MUST existing flat `BindingDecl`, Glial `Surface`, tap advertisements, legacy serve claims, and unsupported shape declarations migrate to the new versioned records without being mistaken for fully secured provider contracts?

**Why it matters**

Current documents and implementations disagree about CRDT declaration support, GLP-0006 subsidiary documents lag the final ratification record, and the existing `AdvertisementRecord` has a different meaning. Silent reinterpretation would create both wire-compatibility and security bugs.

**Alternatives**

1. Reinterpret existing records in place.
2. Introduce new contract IDs/versions, explicit migration, and a support matrix; reject ambiguous legacy records at secured boundaries.
3. Maintain both models indefinitely with best-effort conversion.

**Recommendation and tradeoff**

Choose alternative 2. Provide a one-way, auditable migration for declarations whose meaning is complete. Treat legacy provider attachment and hello-presented identity as development-only until replaced; do not auto-upgrade them into trusted claims. Exact unsupported shapes fail before mutation.

**Affected contracts**

Contract IDs; record decoders; `.glade` loader; generated TypeScript/Rust/Python APIs; Glial manifests; node and discovery claims; fixtures; documentation status headers.

**Validation scenario**

Load each supported legacy declaration into a migration fixture and obtain a deterministic new definition/binding pair. Reject ambiguous provider identity, role, or retention data with a diagnostic. Confirm a CRDT declaration is either explicitly supported end-to-end or refused at load time.

## Recommended decision order

1. **ASCR-D01** establishes record ownership and version boundaries.
2. **ASCR-D02** establishes the source/replica authority seam.
3. **ASCR-D03** makes runtime advertisements safe enough to route.
4. **ASCR-D04** closes GDL-039 and the Garns scope boundary.
5. **ASCR-D05** fixes declaration composition and generation ownership.
6. **ASCR-D06** fixes source-commit/publication recovery.
7. **ASCR-D07** fixes truthful durability, compaction, and deletion.
8. **ASCR-D08** defines offline user-visible effect semantics.
9. **ASCR-D09** closes the remaining live terminal lifecycle choice.
10. **ASCR-D10** publishes migration and support status after the target contract is settled.

The first review gate should decide ASCR-D01 through ASCR-D06 together. They form one dependency chain: a provider cannot be authorized without a binding; a source role cannot be advertised safely without distinct source and replica semantics; a Garns mapping cannot be stable without domain/scope rules; and an authoritative command cannot be correct without a recoverable publication seam.
