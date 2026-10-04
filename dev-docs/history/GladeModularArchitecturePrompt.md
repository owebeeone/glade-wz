# Assignment: independently buildable Glade modules across the application stack

Date: 2026-09-07.

## Objective

Produce a comprehensive, implementation-ready **architecture proposal** for
decomposing Glade into small component libraries with explicit interfaces,
minimal dependencies and independently runnable fixture-based contract tests.
Consider the entire **application → Grip → Glial → Glade → source/store** stack,
including **Garns**, suppliers, Taut declarations and canonical Taut shape engines.
Do not limit the architecture to discovery, persistence, or the existing Rust
contract crates.

The owner wants the same development property as the recent **GWZ local-clone
feature**: a module can be developed and tested against an interface and fixtures
without needing the rest of the system to be running or implemented. Find and
inspect that feature's actual interface/test separation before drawing lessons.
Do not invent its implementation or assume it is in this repository. If its
artifacts cannot be located, record that evidence gap and apply the stated
development objective without claiming the example was verified.

The central question is:

> What is the smallest coherent collection of modules, contracts and adapters
> that lets us build Glade incrementally, test each component independently, and
> let applications participate without implementing the whole distributed stack
> or a separate adapter for every shape × storage backend × runtime combination?

Deliver a buildable decomposition, not merely conceptual boxes, a list of traits,
an audit of current code, or a recommendation to design the architecture later.

## Workspace, authority and scope

- Primary workspace: `/Users/owebeeone/limbo/glade-wz`.
- Garns workspace: `/Users/owebeeone/limbo/garns-wz`. Verify its actual members and
  supported interfaces; do not conflate current Garns with older generated Grazel
  stores or assume Rust/TypeScript APIs exist because documents propose them.
- Read `AGENTS.md`, `AGENTS_GWZ.md` and applicable member instructions before work.
  Follow `dev-docs/LibraryBoundaryAndTestingPolicy.md` and distinguish adopted
  constraints from the proposed decomposition in `GladePackageArchitecture.md`.
- Use `gwz ls`, `gwz status` and `gwz forall <member> -- rg ...` for member discovery,
  status and searches. A search at the root does not establish absence across GWZ
  member repositories. State the scope of negative findings.
- This assignment authorizes architecture documents, evidence gathering and safe
  isolated validation—not production implementation, interface rewrites, repository
  restructuring, commits, deployment or changes to the demo or its stores.
- Write only the deliverables below. Preserve dirty work and inspect existing
  outputs before writing. If an output belongs to another run, use a consistent
  revision suffix across the new set and fix its internal links.
- Existing code is evidence, not a requirement to preserve accidental boundaries.
  Be willing to replace/deprecate it. Migration MUST nevertheless leave the demo
  working: propose new parallel crates and explicit cutover gates before deletion.
- Distinguish ratified requirements, owner constraints, proposed decisions,
  implementation facts and inference. Do not silently ratify open decisions.
  Cite source paths/lines and relevant revisions; verify historical review claims.

## Required starting evidence

Read the following, then inspect the source and tests that establish their claims.
Resolve moved paths through the workspace rather than treating stale links as proof
that a component is absent.

1. `dev-docs/DecisionLog.md`, `GladeArchitectureDiscussion.md`,
   `GladeProblemInventory.md`, `GladePackageArchitecture.md`, and
   `LibraryBoundaryAndTestingPolicy.md`.
2. `dev-docs/ApplicationStackContractReview.md`, its `-G6` counterpart, both decision
   worksheets, and `GladePersistenceReview.md`. These are prior analyses to challenge,
   not sources of new authority.
3. `dev-docs/glade/GladeDeclSurface.md`, `GladeSupplierModel.md`,
   `GladeSystemDataSeam.md`, `GladeAuthzModel.md`, `GladeWorkspaceDirectory.md`,
   `GladeInstanceLifecycle.md`, and relevant accepted GLP-0006 rulings.
4. `dev-docs/TautShapeCatalogAdoption.md`, canonical shape contracts/corpora,
   `taut-shape` stream/SWMR/snapshot-delta/CRDT decisions, and Glade's exact shape
   adapter documents. Recognition of a shape is not implemented runtime capability.
5. Grip Tap/declaration boundaries; Glial binding, assembly, stores, subscriptions,
   offline behavior and application adapters; `glade-decl` language renderings;
   Glade node/client/wire code; actual supplier code; current Garns contracts.
6. `glade/contracts/` and `glade/dev-docs/ApplicationContractDraft.md`; the discovery
   contract crates and `glade-discover/dev-docs/DraftHostContracts.md` and
   `RegistryContractDraft.md`; both architecture policies and fast-check scripts.
7. Available Rust async/lifecycle research and API proposals, including
   `dev-docs/Rust Async Lifecycle Orchestration.md` if present. Research availability
   MUST NOT block unrelated interface design. Neither sdax nor channels nor macros
   nor a particular executor is mandated.
8. The actual GWZ local-clone implementation, its task/plan if available, interface
   boundaries, fixture tests and integration gates. Cite what is being reused as a
   development pattern, not copied as domain behavior.

## Non-negotiable design requirements

### A. Whole-stack ownership, small application burden

Map responsibilities across applications, Grip, Glial, Glade, Taut/taut-shape,
Garns, suppliers and physical storage. Identify who owns declarations, domain
mapping, source truth, operation identity, authority, materialized state, delivery,
retention, persistence, resource lifetime and error presentation.

Explicitly distinguish these application participation modes:

1. A consumer uses generated/typed bindings with built-in Glial/Glade behavior.
2. A supplier uses Glade-managed replica persistence and supplies only its domain
   data/commands or canonical shape-specific behavior.
3. An application retains its own database/files/external service and implements
   a narrow source adapter, with explicit publication and recovery semantics.
4. A deployment/application supplies a custom persistence backend implementing
   only the capabilities it advertises.
5. An advanced application supplies a new payload/shape profile and its conformance
   evidence, without rewriting transport, authentication or generic lifecycle.

For each mode, show the **minimum code the application author writes**, what is
generated, what is configured, what is supplied by libraries, and which tests the
author runs. Include concrete Rust sketches and explain the equivalent TS/Python
boundary where relevant. Do not mandate that every application implement all roles.

Garns MUST be considered as an authoritative relational source, a source of live
query results, and potentially a store/projection participant. Evaluate each role
separately. Keep Garns transaction/ledger/query identity distinct from Glade origin
chains, delivery cursors, domain/zone identity and physical database identity.
Explain how a non-Garns application implements the same source-side contract.

### B. Enumerate before multiplying adapters

Produce separate, linked matrices:

1. **Shape/profile → source obligations → delivery/recovery semantics → required
   persistence capabilities.** Cover `value`, `atom`, `log`, `stream`, `swmr`,
   `crdt`, and the `snapshot_delta`/`text_crdt` profiles. Keep `unary`/`exchange`,
   terminal channels and `window` projections correctly classified; do not invent
   extra delivery engines for them.
2. **Persistence capability → backend family.** Consider volatile memory, local
   checkpoint files, segmented journals, transactional databases such as SQLite,
   browser IndexedDB, immutable/blob storage, and application-provided backends.
   Separate checkpoint/journal/blob/atomic-commit mechanisms from physical products.
3. **Application/source family → reusable adapter family.** Include application
   state, Garns live queries/commands, external databases, filesystem content,
   collaborative documents, terminal processes and discovery/control-plane records.

Every meaningful cell MUST say supported now, proposed, conditional, unsupported,
or not applicable; identify evidence and required tests. State which combinations
must exist initially and which are optional/future. Do not imply all databases
provide the same transactions, durability, indexing, eviction or cursor guarantees.

Show how N shape implementations and M backend implementations compose rather
than requiring N×M handwritten adapters. Identify the residual combinations that
really need special integration code. Implementation reuse MUST NOT be confused
with proof that all supported combinations work. Give an inventory/count by family
and identify which counts are commitments versus illustrative candidates.

Preserve canonical semantics: stream is disposable/no durable replay; SWMR repair
differs from snapshot-delta external refresh; CRDT bootstrap/causal history and text
tombstones cannot be compacted by an arbitrary generic retention policy. Separate
payload lifetime from authority, deduplication, sequence and fencing history.

### C. Concrete component decomposition

Enumerate every Glade responsibility and assign it to a named module or an explicit
external owner. At minimum assess:

- Declaration/version/capability negotiation and canonical parameter mapping.
- Binding resolution and application-facing facades.
- Shape-specific delivery bridges, projection/assembly and source adapters.
- Invocation, authenticated provider context, attachment, epochs and handoff.
- Subscriptions, live channels, backpressure and demand/interest accounting.
- Operation validation, canonical identity, ingestion, replica synchronization,
  bootstrap/checkpointing and conflict/equivocation handling.
- Acceptance/deduplication journals, snapshots, blob references, transaction
  coordination, outboxes, publication acknowledgements and crash recovery.
- Retention, quotas, eviction, pinning and deletion semantics.
- Identity, signing, trust/evidence and authorization, including revocation.
- Registry publication/renewal/resolution, shard location/referrals and the separate
  node-discovery/transport layer; locality and partition behavior.
- Resource lifecycle, cancellation, cleanup ordering, scheduling, clocks and retries.
- Composition roots, configuration, diagnostics, observability and compatibility.

This is a responsibility checklist, NOT an instruction to create one crate per
bullet. Group tightly coupled invariants; split replaceable responsibilities and
compilation dependencies. Avoid both a universal common crate and gratuitous
one-method packages. Pure state machines and data/codec libraries need justified
classifications and direct tests—not meaningless marker traits.

Give separate diagrams for compile-time dependencies and runtime flows. Mark
contract, implementation, pure, protocol/data, integration and harness/tool roles.
Explain concrete dependency exceptions and who owns their retirement. Show where
the same modules run in a browser, application process, local node and server.
Do not confuse deployment colocation with compile-time coupling.

### D. A module contract card for every proposed component

Assign stable IDs (`GMA-M001`, etc.). Each card MUST contain:

1. Responsibility, owning layer and explicit non-responsibilities.
2. Proposed package/language and architectural classification with rationale.
3. Public interfaces: representative required Rust trait methods, associated
   types, explicit event/effect boundary or versioned data contract, as appropriate.
4. Inputs, outputs, errors, authority context, identity and versioning rules.
5. State ownership, atomicity, durability and the exact meaning of success.
6. Async behavior, cancellation before/after effects, uncertain outcomes, cleanup
   ownership, retries and resource bounds. Explain Send/dyn/runtime choices where
   they change usability; do not let executor types leak into general contracts.
7. Allowed dependencies and forbidden edges, including dev/build/optional deps.
8. Fixture construction: explicit injected clock, identities, storage/transport
   outcomes and fault controls; no hidden running node or real sleeps.
9. Canonical success/failure/edge cases and at least one rejecting mutant proving
   the assertions detect a broken implementation. Tests have stable IDs.
10. Real-adapter obligations the fixtures cannot prove, such as fsync, transactions,
    browser eviction, cryptographic evidence and overlapping execution.
11. First implementation, alternative implementation/substitution proof, fast test
    command, affected-consumer checks and proposed performance budget.
12. Current-code mapping: retain, wrap, replace, deprecate or new; prerequisites,
    unresolved decisions and the safe migration/cutover gate.

Interface sketches must be sufficiently concrete that implementation teams can
write failing conformance tests without rediscovering the architecture. Explain
generic substitutions and provide at least one representative implementor sketch;
do not hide every hard semantic behind opaque bytes or an unconstrained type.

### E. Audit the existing drafts rather than freezing them by accident

Treat existing `glade/contracts` and discovery APIs as provisional inputs. Produce
a retain/refine/split/replace disposition for every existing draft interface.

Specifically test whether the snapshot-first, snapshot/delta/gap subscription
surface can faithfully express atom replacement, log EOF/expiry, stream live-only
join/drop, SWMR in-band repair and CRDT causal/bootstrap delivery. If not, propose
the smallest appropriate specialization or separation. Do not force canonical
shape semantics into a convenient universal enum.

Assess whether snapshot CAS, operation journals, replica ingest, durable accepted
intent and application transaction publication require different seams. Memory
fixtures and fire-and-forget browser writes MUST NOT masquerade as durable stores.
Document compatibility implications; do not edit those interfaces in this task.

### F. Independent TDD and system assurance

Design three explicit test layers: module conformance, adjacent-boundary composition,
and a small set of full-stack journeys. Describe a reusable deterministic fault
harness with scheduled failures before/after commits, lost replies, duplicate and
reordered delivery, restart, cancellation, deadline, revocation and cleanup faults.

Define generic invariants and shape-specific oracles. Reuse canonical corpora;
parameterize adapter suites with real native cursors, signed operations and schemas.
Do not couple reusable tests to one toy byte encoding or concrete implementation.
Preserve every discovered failure as a small reproducible regression.

Specify dependency-aware local/CI selection, bounded local schedules, wider CI
matrix coverage and separate stress/fuzz/crash suites. Every supported matrix cell
needs an explicit verification owner and gate; sampling alone is not conformance.
Distinguish execution time, warm incremental time and cold-build time. Label
unmeasured budgets as targets, never achieved guarantees.

Describe enforcement for unknown libraries, missing meaningful required traits,
forbidden dependency edges, missing conformance targets and affected-consumer
checks. Explain what fails locally, what fails CI and what plain `cargo build`
cannot enforce. Source parsing is not semantic review. Do not relax policies to
make a design pass or claim existing gates cover repositories they do not cover.

### G. End-to-end proofs and incremental construction

Walk through at least these cases, naming module/interface/test IDs at every step:

- A private application value persisted locally, optionally replicated later.
- A Garns scoped live query and a command whose transaction commits before a
  publication reply is lost; reconcile without lost publication or duplicate effect.
- Filesystem read, SWMR changes, collaborative text edits and save against a source
  revision. Identify authoritative source versus replica/projection throughout.
- Terminal creation/input, ephemeral output, separately retained scrollback,
  disconnect and cleanup. Do not silently select an unresolved retention policy.
- Discovery registration/renewal, unavailable replica, authorized shard referral,
  stale authority and reconciliation after partition.

Include application/database failure, offline edits, restart, cursor expiry,
authority turnover and revoked access. Identify what survives, is rebuilt or is
intentionally lost. Publication-after-source-commit MUST explicitly address the
dual-write gap (outbox/change capture/reconciliation or justified alternative).

Order delivery in independently assignable work packages: contracts + failing
fixtures first; adversarial contract review second; implementation third; then
adjacent integration and demo cutover. Give dependencies, parallelizable work,
acceptance criteria and rollback/legacy retirement gates. The first useful slice
must not require all modules or the whole trust/lifecycle research program to finish.
Conversely, staged delivery must not silently weaken authentication or durability.

## Alternatives and adversarial review

Compare at least two credible decompositions, including application implementor
effort, dependency/build cost, semantic fidelity and recovery complexity. Recommend
one, with concrete counterexamples to the rejected arrangement—not generic praise.

After the design is drafted, obtain independent adversarial reviews with bounded
sub-agents: one for semantic/security/recovery correctness and one for dependency
isolation/application usability/testing. Reviewers must inspect the proposal and
relevant evidence; their role is not to endorse it. Record findings, severity,
disposition and residual uncertainty. Resolve architectural contradictions or
identify precise owner decisions; do not claim production correctness from review.

Proceed with explicitly labeled assumptions where safe. Ask only genuinely
blocking questions; do not hand every implementation choice back to the owner.
An unresolved decision must identify exactly which module/contract it blocks and
which work remains independent. Do not leave the whole design waiting on it.

## Deliverables

Create `/Users/owebeeone/limbo/glade-wz/dev-docs/arch1/` if needed and write all
five generated documents below in that subfolder. Keep links between those
documents relative to `arch1/`; adjust links to existing workspace evidence
accordingly. Leave this prompt in `dev-docs/`.

1. **`GladeModularArchitecture.md`** — executive explanation, whole-stack ownership,
   vocabulary, recommended decomposition, alternatives, compile/runtime diagrams,
   deployment placements and worked journeys. Keep the main explanation readable
   to an experienced developer who is not a Rust specialist.
2. **`GladeModuleContracts.md`** — complete module inventory and contract cards,
   dependency table, concrete interface sketches and current-draft dispositions.
3. **`GladeAdapterMatrix.md`** — the three linked matrices, capability definitions,
   supported/conditional/unsupported combinations, application participation modes
   and minimal implementor examples. Distinguish implemented evidence from targets.
4. **`GladeArchitectureDeliveryPlan.md`** — canonical fixture/test catalog, fault
   harness design, enforcement and fast-feedback strategy, phased work packages
   and demo-preserving integration/cutover gates. This is a proposal, not automatic
   registration or execution of a development plan.
5. **`GladeArchitectureReview.md`** — independent review findings/dispositions,
   remaining owner decisions, evidence/search/test limitations and readiness verdict.

Use stable module, interface, requirement and test IDs with cross-links so every
responsibility maps to a module, every module to a contract, every guarantee to
a test, and every supported adapter combination to an explicit gate.

## Completion criteria

The architecture is ready for owner review only when:

- All relevant stack responsibilities are assigned, with no hidden application or
  Garns coupling inside Glade and no unowned cross-boundary transaction/cleanup.
- Each proposed module can be implemented against documented fixtures while its
  peers are absent; real integration obligations are stated separately.
- Applications have concrete small entry points, not a requirement to implement
  all framework traits or all shape/storage combinations.
- Shapes, profiles, stores and source authority are not conflated; unsupported
  combinations fail explicitly and existing canonical behavior is preserved.
- Existing draft mismatches are dispositioned, not concealed by passing toy tests.
- Dependency/fast-test enforcement and the implementation order are actionable.
- Independent review findings and exact unresolved owner decisions are recorded.

Return links to the five documents, the recommended component structure in a few
sentences, the highest-impact owner decisions, and the first independently buildable
work packages. State what was inspected or executed. Do not begin implementation,
change canonical rulings, modify the demo or create additional user-owned tasks.
