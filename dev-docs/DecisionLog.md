# G* Decision Log

Status: working draft

Purpose: record unresolved architecture decisions for the current authoritative
G* design work.

## Usage Rule

- Open decisions stay here until resolved.
- When resolved, record the chosen outcome and rationale here.
- Supporting detail MAY live in architecture documents, but the decision status
  MUST be visible here.

## Open Decisions

| ID | Topic | Current Question | Status |
| --- | --- | --- | --- |
| `GDL-001` | Share container | Is a `Share Scope` always exactly one `GraphSpace`, or can one share intentionally span multiple graph spaces? | open |
| `GDL-002` | Interest aggregation | How are contributions from multiple sessions combined into one effective `Interest Spec` for a mutable source? | open |
| `GDL-003` | Progressive delivery envelope | What is the canonical shared-state shape for progressive source delivery, including generation boundaries, partial values, loading, stale, and error state? | open |
| `GDL-004` | Delegated references | How are agent-visible references represented, narrowed, and revoked without exposing full private session state? | open |
| `GDL-005` | Primary ownership control | What exact control-state structure selects and transfers primary ownership for upstream registration and side-effectful taps? | open |
| `GDL-006` | Exchange observation and retention | How are response observation confirmation, attempt retention, and diagnostic garbage collection represented in the Glade exchange model? | open |
| `GDL-007` | Declaration validation | What canonical declaration envelope, signature scheme, policy reference, and capability proof format make declarations independently verifiable across JS/TS and Python peers? | open |
| `GDL-008` | Genesis authority | Who can create, update, rotate, or revoke genesis bundles for an environment or workspace in p2p-first Glade? | open |
| `GDL-009` | Capability revocation | What guarantees are required when revoking delegated or workspace capabilities, especially for offline peers and cached local replicas? | open |
| `GDL-010` | Metadata exposure | Which declaration-envelope fields, workspace ids, share ids, provider claims, and transport topics are allowed to be visible to relay-only peers? | open |
| `GDL-011` | P2P v1 topology | Which libp2p mechanisms are required for v1 declaration replication, live channels, provider discovery, and relay-only participation? | open |
| `GDL-012` | Instance retention | What retention defaults apply to ephemeral exchange instances, terminal live-channel logs, diagnostics, observations, and abandoned leases? | open |
| `GDL-013` | Bootstrap and self-provisioning | What exact bootstrap kernel, genesis format, system declaration versioning, and mandatory-feature negotiation are required before Glade can provision higher Glade behavior safely? | open |
| `GDL-014` | Provider placement and affinity | How are provider groups, provider instance leases, session affinity, work assignment, failover, and provider-local state migration declared and resolved? | open |
| `GDL-015` | Application definition sharing | How are application definition collections stored, versioned, imported, trusted, and shared across genesis or system declaration spaces? | open |
| `GDL-016` | Provisioning authority | Which principals may define services, deploy services, publish routes, assign sessions, and grant provider capabilities in a p2p-first system? | open |
| `GDL-017` | Application manager coordination | Can multiple application managers coordinate one environment, and if so what ownership, election, or partition behavior keeps provisioning decisions consistent? | open |
| `GDL-018` | Scale modes and hot paths | Which runtime views, queues, indexes, and sharding boundaries are allowed at each scale mode while preserving declaration semantics? | open |
| `GDL-019` | Declaration package format | What exact source layout, canonical signed representation, generated binding strategy, and package dependency model should declaration packages use? | open |
| `GDL-020` | Definition schema language | Which schema language, canonical runtime form, and code-generation pipeline should define Glade declarations across Python and TypeScript? | open |
| `GDL-021` | Declaration DSL | What grammar, canonical IR, import model, comment/hash behavior, and generated binding conventions should the Glade declaration DSL use? | open |
| `GDL-022` | Distributed control plane | What controller partitioning, owner-term, lease, admission, placement, quota, health, route-index, and split-brain rules make the cluster-control problem safe as a distributed Glade application? | open |
| `GDL-023` | Control-plane console | What projection schema, metadata redaction policy, operator-intent model, and UI/service boundary make provisioning visible without making the console a hidden source of truth? | open |
| `GDL-024` | Developer golden path | What exact GripLab vertical slice proves mock tap to generated Glade endpoint to provider-backed tap to console visibility without UI rewrite? | open |
| `GDL-025` | Frankenapp silver path | What minimum connector, identity, consent, audit, and cross-app link model proves application composition without blocking the GripLab golden path? | open |
| `GDL-026` | Source handles and view binding | What canonical handle shape binds a source instance to its commands, live channels, logs, materialized views, route keys, replay cursors, provider instance, and owner term? | open |
| `GDL-027` | Legacy-to-Glade adapters | How do legacy snapshot streams, request methods, and service subscriptions declare whether they are canonical Glade flows or compatibility adapters into target flow semantics? | open |
| `GDL-028` | Terminal output cursor and ordering | Which cursor forms are allowed for terminal output logs across provider-sequenced, byte-offset, content-id, and causal-head substrates, and how are they exposed to generated taps? | open |
| `GDL-029` | Grip share advertisement format | What exact canonical record kind, DSL syntax, source-map metadata, and generated binding behavior should represent Grip Share advertisements? | open |
| `GDL-030` | Shared versus session-local inputs | How should an advertised input Grip distinguish personal session state from shared collaboration focus without forcing UI rewrites? | open |
| `GDL-031` | Node trust and placement | Is node trust the operator relation (node keys chain to operator principals; `replica.hold`/`session.host` as placement verbs in share policy)? Proposed in `glade/GladeAuthzModel.md` §7a; demonstrated by ggg-viz s-tenant / s-local-guest / INV-5. | **ratified 2026-07-07** |
| `GDL-032` | Discovery layering and session placement | Are the layers fixed as: session placement (web bootstrap) → service discovery (folds over shares) → node discovery (iroh), never merged? Proposed in `glade/GladeWorkspaceDirectory.md` §7b; demonstrated by ggg-viz s-roam. | **ratified 2026-07-07** |
| `GDL-033` | Authn-method policy ownership | Is the set of acceptable authn methods (and the attenuation of operator-vouched sessions) the USER's replicated data rather than operator config? Proposed in `glade/GladeAuthzModel.md` §7b. | **ratified 2026-07-07** |
| `GDL-034` | Ownership and administration | Creation-mints-the-root (no revocable record), ancestry-based admin revocation (no sibling removal; quorum opt-in), and the governance/access split (`admin.revoke` cooperative over user grants, hierarchical over admin chains). Proposed in `glade/GladeAuthzModel.md` §3a; demonstrated by ggg-viz s-admin. | **ratified 2026-07-07** (spelled out 2026-07-05) |
| `GDL-035` | Glial client runtime | Is glial the client-side kernel: local persistence FIRST (glade optional, configured-in), taut-shape-aware assembly inside glial, rich incremental change events (consumer chooses delta vs whole-refresh against live UI state), taps as thin declared conduits with no direct tap→glade coupling, and `glade-decl` as the shared leaf module? Stated by Gianni 2026-07-05; proposed in `glial/GlialClientRuntime.md` + `glade/GladeDeclSurface.md`; stack traces s-stack-*. | **ratified 2026-07-07** |
| `GDL-036` | System-data storage seam | Defer registry/gryth-data storage design behind two seams: RegistryApi (queries-over-fold + record appends with origin attribution — NEVER get/set-config-object) and StoreApi (whole-state taut-message blob now → SQLite engine later; SQLite is a store engine, never the replication mechanism — replication stays ops). Stated by Gianni 2026-07-06; pinned in `glade/GladeSystemDataSeam.md`; the atlas is the regression net (no rung may change a trace). | **ratified 2026-07-07** |
| `GDL-037` | App/substrate split + `.glade` re-scope | Base glade is app-agnostic (transport, endpoint discovery, ephemeral endpoint management, ACL enforcement, `~/.glade/sys` persistence — all runtime-record-driven); applications (grazel = gryth's) declare endpoint productions in `<app>.glade` — data-only cross-language declarations (glade ids, shapes, key types) + ACL seeds that COMPILE TO grant records (fold stays the only runtime authority). Dynamic grip-context-graph sharing deferred as a glial-grip mechanism (rides BindingDecl-as-record; enables headless AI clients; GDL-004/GDL-030). Stated by Gianni 2026-07-06; pinned in `glade/GladeDeclSurface.md`. | **ratified 2026-07-07** |
| `GDL-038` | Management surface = ordinary bindings | Glade-specific management endpoints (ACLs, users, node CRUD) are NOT a privileged plane: reads = subscriptions to system shares, writes = the same record-appends, effects = admin/lifecycle verbs — all through glade→glial→grip, declared in base glade's own `glade-sys.glade`. Management UIs are ordinary grip apps holding grants (GDL-023 collapsed to user scale). Confirmed with Gianni 2026-07-06; pinned in `glade/GladeDeclSurface.md`. | **ratified 2026-07-07** |
| `GDL-039` | Zones vocabulary (domain/zone/surface) | Adopt `glade/dev-docs/GladeZones.md` (implemented+verified 2026-06-14, rediscovered during repo reconciliation 2026-07-06): domain→`share`, zone→`key` (commons \| private(self) + future axes), surface→`glade_id`; grants gate commons joins, privacy is a key (AZ §4a); D8 refined — each zone is its own chain. `BindingDecl` carries domain/zone (`GladeDeclSurface.md`). Open sub-items (from its §Open): axis vocabulary, account-domain shape, domain anchoring, wire field naming. Code status: grip-core ShareDecl +domain/zone and glade client-ts changes are UNCOMMITTED — commit adjudication pending (Gianni). | open (implemented, pending ratify) |

## Notes

- These decisions are derived from the current stack documents in this folder.
- No decision is considered closed until it is reflected in the future detailed
  architecture.
