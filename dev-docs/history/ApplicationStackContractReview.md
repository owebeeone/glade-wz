# Application Stack Contract Review

Status: architecture review and proposed contract direction

Evidence snapshot: 2026-09-05

Scope: Garns, Grip, Glial, Glade, Taut, suppliers, generated declarations, and the current Grazel demonstration

## Executive conclusion

The application stack can have one coherent declarative contract, but it should not have one universal record or one symmetric supplier interface.

The smallest coherent model is a single authoritative record pipeline with three linked levels:

1. A **surface definition** states stable application semantics: identity, payload and command schemas, exact Taut shape engine and profile, supported interactions, and recovery capabilities.
2. A **binding instance** places that definition in a Glade domain and zone, selects its source-authority class and policies, and supplies instance parameters. This is the durable control-plane fact from which a client mount is resolved.
3. A **provider advertisement** is a signed, leased runtime claim that a principal at a route can currently perform specified roles for that binding under a particular authority term. It is discoverability evidence, not authority by itself.

These records MUST flow through the same Glade registry and validation path. A `.glade` file may author or generate record templates, and Glial may expose a convenient typed mount manifest, but neither may become a second authority for the same semantic fact.

Suppliers need a common lifecycle and capability envelope, plus two distinct ports:

- a **source port**, which understands the authoritative system, source revisions, commands, and source-specific recovery; and
- a **replica port**, which stores and replays Glade/Taut operations, checkpoints, and retained cursor ranges without acquiring source authority.

This separation matters most for Garns. A Garns world may be the authoritative relational source for a question or command, while Glade distributes a shaped projection. A Glade replica must not thereby become a Garns write authority. Conversely, when Garns captures an external source, Garns is a derived source unless authority is explicitly transferred.

The current codebase contains strong reusable mechanisms—Taut shape engines and conformance, Glial instance assembly, Glade attributed operations and routing, and the glade-discover authorization projection—but it does not yet implement the proposed cross-layer contract. In particular, current provider attachment is not grant- and epoch-fenced, current supplier sessions trust a presented principal, no standard source/replica port exists, Garns has no Glade adapter or transactional publication contract, and the file and terminal suppliers remain specifications rather than implementations.

## 1. Vocabulary and ownership

The following vocabulary keeps definition, placement, execution, and storage separate.

| Term | Meaning | Semantic owner |
|---|---|---|
| Surface definition | Stable, versioned application-facing behavior: schemas, interactions, shape engine/profile, and recovery capabilities | `glade-decl`, referring to Taut contracts |
| Shape engine | The merge and observation algebra, such as value, log, SWMR, or CRDT | Taut |
| Shape profile | A constrained application use of an engine, such as snapshot/delta over SWMR or text editing over CRDT | Taut catalog; application selects it |
| Application view | An application-facing interpretation with an explicit base engine | Application contract / Grip-facing generated API |
| Binding instance | A definition placed in a concrete domain/zone with policy, authority-class, retention, and parameter references | Glade registry/control plane |
| Source authority | The system permitted to decide canonical source changes and command outcomes | Supplier/source contract and Glade authority assignment |
| Replica | A durable or transient holder of attributed shape operations | Glade |
| Source port | Adapter from a source system to typed snapshots, changes, and commands | Supplier implementation against a standard contract |
| Replica port | Adapter for append, restore, checkpoint, compaction, and replay of Glade/Taut operations | Glade store implementation |
| Provider advertisement | Signed, leased claim that a route can currently serve named roles and exact contract versions | Glade discovery/control plane |
| Supplier host | Process/node lifecycle that registers providers; not automatically the source authority | Supplier kit / Glade node |
| Garns scope | Relational access path encoded in qualified links and `_scope`; not a Glade domain | Garns |
| Glade domain | Application ownership and sharing boundary that maps to a share identity | Glade |
| Glade zone | Security/replication partition that maps to keying and grant policy | Glade |
| Ledger entry | Garns semantic transaction audit | Garns |
| Operation journal | Attributed distributed operation history | Glade |
| Shape cursor | Consumer recovery position within retained shape history | Shape-specific Glade/Taut contract |

This allocation follows the already-ratified direction that Glial is the client kernel, Glade remains application-agnostic, and Taut-aware assembly uses thin taps rather than direct tap-to-Glade coupling (`dev-docs/DecisionLog.md:53-55`). It also preserves the Garns rule that scope is part of qualified link identity rather than a column bolted onto every relation (`../garns-wz/garns/ARCHITECTURE.md:155-198`).

Two existing names need qualification. The `AdvertisementRecord` in `glade-decl` currently enumerates Tap/grok packages (`glade-decl/ir/glade_decl.taut.py:108-111`); it should be called a **tap advertisement** or equivalent and MUST NOT double as the provider advertisement above. Likewise, the current Glial `Surface` combines declaration fields with concrete share and key fields (`glial/src/manifest.ts:28-55`). It is a useful client projection, but not the canonical definition schema.

## 2. Proposed declarative contract

### 2.1 Surface definition

A surface definition is stable and versioned. In pseudotype form:

```text
SurfaceDefinition {
  surface_id
  definition_version
  payload_schema_ref
  shape: {
    engine_id
    engine_version
    profile_id?
    profile_version?
    application_view_id?
    explicit_base_engine?
  }
  interactions[]
  command_schemas[]
  parameter_schema_ref?
  recovery_capabilities
}
```

`interactions` are typed and directional: observe, append, propose replacement, invoke command, acknowledge checkpoint, and so on. They are not inferred from a vague “read/write” flag. Shape and profile versions MUST match exactly and unsupported combinations MUST fail before mutation, consistent with the adopted catalog and Glial dispatch rules (`dev-docs/TautShapeCatalogAdoption.md:18-46`, `glial/src/shapes.ts:71-103`).

The definition does not contain a concrete domain, share key, physical path, database connection, current provider, retention window, or grant. Those facts change at a different rate and have different authority.

### 2.2 Binding instance

The durable control-plane record is an instance of a definition:

```text
BindingInstance {
  binding_id
  definition_ref
  definition_version
  domain_ref
  zone_ref
  canonical_instance_key
  parameters
  source_binding_ref
  source_authority_class
  policy_refs[]
  retention_policy_ref
  placement_policy_ref?
  lifecycle_policy_ref?
}
```

The binding maps to a concrete Glade share and partition through the domain/zone rules; those wire identities need not be copied into every authoring surface. GDL-039 has working code for domain-to-share and zone-to-key mapping, but remains marked “open (implemented, pending ratify)” and still has unresolved anchor and axis questions (`dev-docs/DecisionLog.md:59`, `glade/dev-docs/GladeZones.md:80-126`). Contract publication therefore depends on an owner ruling rather than assuming the current encoding is final.

`.glade` files remain valuable as authored application packages. They should compile to the same registry records and ACL seed records described by GDL-037, after which only the Glade fold is runtime authority (`dev-docs/glade/GladeDeclSurface.md:111-149`). A Glial `Fill` is then a client-side resolution of a binding, not a separate source of binding truth.

### 2.3 Provider advertisement

Provider availability is volatile and must be independently leased:

```text
ProviderAdvertisement {
  advertisement_id
  binding_ref
  provider_principal
  node_ref
  route_ref
  roles[]
  contract_versions[]
  capabilities[]
  authority_term?
  lease_epoch
  expires_at
  freshness: {
    source_revision?
    checkpoint_ref?
    retained_cursor_floor?
    observed_heads?
  }
  proof_refs[]
  signature
}
```

Roles MUST be explicit: source authority, replica reader, command provider, or supplier host. A replica may advertise snapshot and replay capabilities without being permitted to execute commands. An advertisement MUST NOT grant authority. Acceptance requires, at minimum:

1. the binding and exact contract versions exist;
2. the provider principal is cryptographically bound to the connection;
3. grants cover the exact binding, roles, and operations;
4. a source-authority or command role matches the current assignment and authority term;
5. the shape/capability set is valid;
6. the lease epoch is newer than any replaced claim and is not an unauthorized takeover; and
7. freshness claims are internally valid and do not promise history below the retained cursor floor.

Withdrawal or expiry removes the route but does not delete durable data. Secrets, filesystem paths, DSNs, process handles, and private policy evidence MUST remain in supplier-private state; advertisements carry opaque references and proofs only.

This is the direct consequence of GLP-0006 B1-B5: exact attach grants, fail-closed decoding, transport-authenticated principal context, identity-bound grants, and signed governance records (`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md:185-292`; ratification recorded in `plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:625-647`). The glade-discover projection already demonstrates grant-, revocation-, principal-, execution-scope-, and expiry-aware claim filtering (`glade-discover/crates/glade-discover-core/src/projection.rs:21-131`). The current Glade node does not yet enforce the same contract: `attach_provider` replaces a provider map entry without grant or authority-term validation (`glade/node/src/exchange.rs:81-91`), and its `ServeClaim` lacks a grant reference (`glade/node/src/sysdata.rs:50-72`).

Route resolution MUST preserve GDL-032's three folds in order. Session placement first selects where the client's session may run. Service/source discovery then selects an eligible advertised provider for the binding and role. Node discovery finally resolves that provider's node route. Provider advertisements may refer to node identities, but MUST NOT carry node reachability as if it were service authority, and node reachability MUST NOT imply that a node serves a binding. A physical store is private implementation state and does not participate in any of these discovery folds.

## 3. Standard supplier interfaces

Taps and suppliers should share conventions, but not an identical semantic interface. Taps adapt application behavior into Glial. Suppliers cross a trust and authority boundary and need explicit source and persistence roles.

```text
SourcePort<S> {
  describeCapabilities()
  openSnapshot(context, request) -> Snapshot<S>
  subscribe(context, cursor?) -> ChangeStream<S>
  execute(context, command, idempotency_key, base_revision?) -> Outcome
  sourceCheckpoint(context) -> SourceCheckpoint?
}

ReplicaPort<S> {
  describeCapabilities()
  append(attributed_ops) -> durable_heads
  restore(range_or_checkpoint) -> SnapshotAndTail<S>
  retainedRange() -> cursor_floor, heads
  checkpoint(validated_state, covered_heads) -> checkpoint_ref
  compact(policy, acknowledged_checkpoints)
  delete(tombstone_or_instance)
}
```

The common envelope covers lifecycle, exact contract negotiation, cancellation, health, observation, and structured errors. Each port is then specialized by the canonical Taut shape types. The Glade server remains payload-agnostic; adapters at the Glial/supplier boundary perform shape interpretation. This matches the existing SWMR boundary, where Glade persists and routes operations while Glial maps them to `SwmrNode` (`glade/dev-docs/GladeSwmrAdapter.md:9-17`).

Source ports MAY author canonical changes only when their authenticated context has the corresponding authority assignment. Replica ports MUST never gain source-write authority merely because they can restore or append attributed operations. “Bidirectional” is therefore not a general adapter property. A source port may support commands, a replica may support ingest and replay, and a deployment may deliberately expose only one direction.

Shape-specific minimum semantics are:

| Engine/profile | Source-facing operations | Replica/recovery requirement |
|---|---|---|
| value | read latest; authority-controlled replace | latest durable value and version |
| atom/live value | read/replace while live | no durable replay unless paired with another engine |
| log | append, tail, optionally seal | ordered cursor, retained floor, explicit expiry |
| stream | open, push, end; reconnect starts or declares a generation | no durable replay unless a log sidecar is declared |
| SWMR | snapshot, delta, reset under one writer term | generation, base revision, checkpoint plus tail |
| CRDT | causal operation, dependency/bootstrap, checkpoint acknowledgement | stable replica identity, causal checkpoint before pruning |
| exchange | invoke/reply with idempotency and authority context | separately correlated outcomes; not a Taut stream |
| window view | view contract over an explicit base | recovery inherited from the base plus window rules |

Retention is not a shape. It is a binding policy constrained by the shape recovery contract, as already established by the shape catalog (`dev-docs/TautShapeCatalogAdoption.md:28-29`).

## 4. Declaration composition: Garns, Glade, and generated APIs

No language should generate another language's complete declaration and become its shadow owner. Composition should use explicit, versioned references.

An application-authored **mapping artifact** should associate:

- a Garns world, question, command, result schema, and contract version;
- one or more Glade surface definitions;
- a shape mapping for snapshot and changes;
- parameter and scope bindings; and
- publication/idempotency semantics.

Tooling may generate Glade record templates, TypeScript/Rust bindings, Grip-facing APIs, and conformance fixtures from that mapping. Generated output MUST carry source contract IDs and MUST fail if the referenced Garns or Taut contract version is unknown. It must not copy policy decisions into an independently editable manifest.

The ownership split is:

| Form | Contains | MUST NOT become |
|---|---|---|
| Authored source | Garns models/questions/commands, Taut/app surface selection, mapping intent, Glade policy and package intent | A runtime route or current authority claim |
| Generated artifact | Language bindings, exact contract IDs, record templates, encoders/decoders, and conformance fixtures | An independently editable semantic owner or a grant |
| Runtime record/state | Binding instances, grants/revocations, authority assignments/terms, provider advertisements and leases, operation heads, cursors, and checkpoints | Source-language schema or supplier-private configuration |

Generated `.glade` templates are therefore acceptable when they are reproducible projections with provenance. Once accepted into the Glade registry, the resulting records—not the generated file—are the runtime facts.

The boundary is intentionally asymmetric:

- When a Garns-managed database is authoritative, Garns commits relational state and its semantic ledger; the supplier publishes a shaped Glade projection.
- Under `external_captured`, an external database remains authoritative. Garns captures changes and can compute derived results, but capture does not silently transfer source authority (`../garns-wz/garns/ARCHITECTURE.md:357-382`).
- A Garns projection of Glade data is a recoverable materialization or cache unless an explicit authority-transfer record says otherwise.
- Garns does not own Glade domains, zones, grants, routing, or provider leases. Glade does not reinterpret Garns link identity, `_scope`, transaction semantics, or result schemas.

Current Garns contracts do not yet expose the portable typed IR, diagnostic, result, transaction, live, and capture contracts needed by this mapping. Only the storage-binding, manifest, and index contracts are versioned today (`../garns-wz/garns/contracts/README.md:1-21`), while `garns-rust` currently recognizes artifact IDs and refuses unknown versions rather than running a Garns backend (`../garns-wz/garns-rust/README.md:3-25`). That refusal behavior is the correct compatibility posture and should be retained.

### Transaction-to-publication seam

For a Garns command, source commit and Glade publication cannot be made one distributed transaction. The source transaction MUST instead commit both the relational change and a durable outbox entry containing a stable publication ID, source revision, contract version, and canonical payload hash. A publisher retries until Glade durably accepts that ID. Glade MUST deduplicate an identical publication ID and MUST reject reuse with different content.

If the process crashes before publication, the outbox retries. If it crashes after Glade append but before acknowledgement, it repeats the same ID without duplicating the semantic change. Commands similarly carry an idempotency key, authenticated authority term, and optional base revision. The Garns ledger, Glade operation journal, and consumer cursor remain distinct records serving different recovery questions.

## 5. Dimensions and data ownership

The five dimensions in the prompt—shape, domain, source, authority, and retention—are necessary but insufficient. A valid binding also needs:

- principal and policy references;
- exact schema and contract versions;
- instance lifecycle and generation;
- authority term and provider freshness; and
- parameter/interest identity, including any explicit Garns scope mapping.

Some combinations are invalid or require an explicit sidecar:

- a stream cannot promise durable replay without a declared log or checkpoint source;
- a replica with command transport is not thereby a write authority;
- an externally captured projection cannot mutate the external source by default;
- SWMR cannot have concurrent source-authority terms;
- a window cannot omit its base engine;
- a CRDT cannot prune unacknowledged causal history;
- authoritative history cannot silently degrade to an LRU cache; and
- an offline command to a remote authority can be accepted only as pending intent, not reported as a completed effect.

The stack should classify state explicitly:

| Class | Owner and recovery rule |
|---|---|
| Authoritative source data | Restored by the source system; changed only through source authority |
| Authoritative Glade state | Restored from attributed ops/checkpoints according to shape and retention |
| Replica state | Rebuilt from an authority or retained journal; never silently promoted |
| Recoverable projection | Recomputed from a named source revision and mapping version |
| Supplier-private durable state | Credentials, process metadata, durable outbox, source checkpoints; never advertised as application data |
| Cache | Freely discardable and explicitly non-authoritative |
| Live process state | Usually non-recoverable; survival or external process-manager attachment must be declared |

Deletion needs the same precision. Durable shared facts require tombstones or an authority-defined deletion event. Instance teardown, cache eviction, retention expiry, and source deletion are not synonyms. This exposes a current client risk: Glial reference-counted unmount calls `store.drop` on the last mount (`glial/src/binder.ts:63-79`), so the persistence contract must say whether this is cache disposal or durable instance deletion.

## 6. Three end-to-end journeys

### 6.1 Garns live question plus command

An application selects a `snapshot_delta` surface definition over SWMR and a separate exchange command. A binding maps application parameters to a canonical instance key and maps authenticated Glade principal/domain context to an explicit Garns `_scope`; the two namespaces are related by the binding, not treated as equal.

On subscribe, Glade authenticates the caller, checks read grants, resolves a live source-authority advertisement, and requests a snapshot from the Garns source port. The port runs the typed question and returns a source revision plus snapshot. Subsequent committed Garns `Batch(instance, base_seq, seq, revision, changes)` events become SWMR deltas only after the Garns transaction commits (`../garns-wz/garns/ARCHITECTURE.md:200-270`). Glade persists attributed operations, replicas replay them, and Glial hydrates local state before exposing the live view.

A command travels over exchange with authenticated provider-call context, idempotency key, authority term, and optional base revision. The source port validates the command and Garns scope, commits state, ledger, and outbox together, then publishes the resulting canonical delta. The command outcome identifies the committed Garns revision and publication ID.

Failure behavior is observable:

- a crash after commit replays the outbox and does not double-apply either command or delta;
- a client reconnect resumes from a retained cursor or receives a checkpoint/reset when its cursor is too old;
- an authoritative external change produces a new committed batch, not an untracked UI mutation;
- source turnover increments the authority term, invalidating old command providers;
- revocation blocks new reads and commands at the relevant replica/authority boundary, subject to documented offline limits; and
- offline commands remain pending and may later conflict or be rejected—they are never optimistic proof of a completed remote effect.

### 6.2 Filesystem edit and save

Use three distinct surfaces: a saved-file SWMR snapshot, a text-CRDT editing document, and a `files.write` exchange command with compare-and-swap base revision. An optional editing marker is a separate lifecycle surface. Cursor and selection identity live in an account/private or tab-private domain and never become file source authority.

Opening a file reads a source revision and full saved image from the filesystem source port. Editing occurs against the CRDT binding; local operations can survive a transient disconnect if their retention/checkpoint policy allows it. Saving invokes `files.write` with the expected file revision and canonical bytes. Only the filesystem authority writes the file. A successful write publishes a new saved-file generation/revision and advances the edit base.

An external file change creates a new saved revision. If there are unsaved edits, the application must expose conflict/rebase/reset behavior rather than overwriting either side. Provider turnover fences the old writer term. Revocation cuts future reads, writes, and presence according to policy; already-present plaintext cannot be magically recalled from an offline device. On supplier restart, saved file state is reread from the filesystem, editing state is restored only from retained CRDT history/checkpoints, and supplier-private path mapping stays outside advertisements.

The existing 4 KiB full-image SWMR demo is a useful transport proof, not this contract: its documented limits include missing path binding, backfill/blob strategy, and write acknowledgement (`glade/dev-docs/GladeSwmrAdapter.md:54-64`). GLP-0006 records owner ratification of the file and retention worksheet rows (`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:658-662`), but the plan gate still lists blob strategy as open. That documentation conflict must be canonicalized; it is not a license to silently choose an implementation.

### 6.3 Interactive terminal session

Use a `term.open` exchange command, a live `term.pty` channel, and a retained `term.scrollback` log. The process host advertises supplier-host and command-provider roles. A local-only first stage may omit network advertisement, but it still uses the same definition and binding semantics.

Input is fenced by a driver authority term. Output is identified by process generation plus monotonically increasing byte offset; the live channel optimizes latency while the log provides recoverable history. A reconnect resumes scrollback from a retained cursor and attaches to the current live generation when possible.

Process recovery is not implied by log recovery. If the supplier crashes, the PTY normally dies while retained scrollback remains. Survival requires an explicitly declared external process manager or spool/reattach capability. Host turnover cannot migrate an ordinary PTY; it may transfer only future routing or input-driver authority. Revocation immediately removes input authority and future replay access, while previously delivered bytes remain subject to ordinary offline limitations.

The semantic contract must declare whether a disconnected terminal survives indefinitely, survives for a grace period, or closes. The terminal specification explicitly leaves stronger reconnect behavior in a later stage (`dev-docs/glade/suppliers/glade-terminal.md:60-138`). That is a binding lifecycle decision, not an incidental transport timeout.

## 7. Private account data and control-plane examples

An account-private settings value demonstrates why domain, zone, source, and replica are independent. The binding uses an account domain anchored to the owner identity and a private zone. The settings source authority may be a user-controlled replicated value, while several devices hold replicas. A per-tab cursor or draft can use a more specific private instance key. None of those device or tab replicas may execute source commands merely because they possess decrypted state.

A control-plane example uses ordinary Glade records for `BindingInstance`, `Grant`/`Revocation`, `AuthorityAssignment`, `ProviderAdvertisement`/claim, and retained heads. That follows GDL-038's decision that management surfaces are ordinary bindings (`dev-docs/DecisionLog.md:56`). Physical store identity, credentials, and process handles remain private to the node, consistent with the discovery rule that physical store identity is not a protocol fact (`dev-docs/glade/GladeWorkspaceDirectory.md:244-258`).

## 8. Alternatives

### Alternative A: one enlarged `BindingDecl` and one symmetric supplier API

This is superficially simple and resembles the current direction of adding more fields to `BindingDecl`. It reduces the number of named schemas. It also combines facts with incompatible lifetimes: application semantics, domain placement, grants, current route, source authority, retention, and physical storage. A symmetric `get/put/subscribe` supplier interface would make replicas look like sources and encourage authority laundering. It also makes generated manifests a likely second control plane. This alternative should be rejected.

### Alternative B: linked definition, binding, and advertisement records with split ports

This review recommends this option. It has more explicit joins, but each record has one owner and lifecycle. It permits static validation before runtime, dynamic placement and turnover without redefining the app, and a standard supplier kit without erasing source/replica asymmetry. It is compatible with app-agnostic Glade records, Taut exact shape dispatch, and Garns refusal of unknown contracts.

### Alternative C: generate Glade declarations directly from Garns source

This minimizes authoring for Garns-backed applications, but makes Garns the accidental owner of Glade domains, policy, routing, and supplier lifecycle. It also poorly serves non-Garns sources. Garns SHOULD generate bindings and conformance material from an explicit mapping artifact, not generate and own the entire Glade control plane.

## 9. Compatibility with accepted decisions

| Decision | Compatibility / required follow-up |
|---|---|
| GDL-031 | Compatible: roles, authenticated principals, and authority terms preserve node trust and session-host distinctions. |
| GDL-032 | Compatible: binding placement, service/provider discovery, and node discovery remain separate folds. |
| GDL-033 | Compatible: authentication-method policy remains replicated user data referenced by bindings/policies. |
| GDL-034 | Compatible: creation and administrative ancestry validate creation of definitions/bindings, not provider self-assertion. |
| GDL-035 | Compatible: Glial remains the Taut-aware client assembly kernel; taps stay thin. |
| GDL-036 | Compatible: registry records use `RegistryApi`; physical persistence uses `StoreApi`; SQLite never becomes replication. |
| GDL-037 | Compatible: `.glade` is an authoring package that compiles to ordinary records and ACL seeds. |
| GDL-038 | Compatible: the new control-plane facts are ordinary bindings/records, not a privileged side channel. |
| GDL-039 | Needs owner ratification/refinement before wire publication: current domain/zone mapping is implemented but not ratified. |
| GDL-040 | Compatible: supplier vocabulary is retained, with source/replica roles made explicit. |
| GDL-041 | Compatible and dependent: exact engine/profile/view rules are the shape portion of `SurfaceDefinition`. |

The GLP-0006 decision record says all approximately forty worksheet rows, including B1-B5, file, retention, terminal, and people rulings, were ratified (`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:625-692`). Some subsidiary plan and worksheet headings still say “recommendation” or list the same subject as open. Owner decisions take precedence, but those documents need status repair before contract authors can reliably distinguish an open choice from unimplemented accepted behavior. The earlier `dev-docs/GladePersistenceReview.md` is useful implementation evidence, but its classification of some file/retention rows as unaccepted is superseded by that later explicit ratification record.

There is also a direct implementation/document mismatch: the shape dispatch document lists CRDT as accepted, while the node application declaration loader currently accepts only value, log, and SWMR bindings (`glade/node/src/appdecl.rs:39-47`). Contract versioning and conformance tests must expose that as unsupported rather than permitting declarations that fail later.

## 10. Demo disposition

| Disposition | Current material |
|---|---|
| Reusable | Taut value/log/SWMR/CRDT engines and exact conformance; Glial binder/instance assembly and operation deduplication; attributed Glade operations; glade-discover's pure, grant-aware claim projection |
| Adaptable | `glade-decl` and typed manifests after splitting definition from binding; supplier-kit session/lifecycle envelope; `.glade` loader; node routing/claims after grant, signature, term, and freshness validation; Glial store interface after recovery/retention expansion |
| Experimental only | 4 KiB full-image file transport; single `gwz.ops` exchange and allowlist; stage-one chat share; `Hello`-presented principal; last-writer-wins provider attachment; legacy generated Grazel application store |
| Absent | Standard source and replica ports; signed provider-ad contract; integrated B1-B5 enforcement in the Glade node; Garns mapping/outbox contracts and adapter; file and terminal supplier implementations; retention/compaction enforcement; file CAS save; terminal reconnect/process recovery implementation |

The supplier model already specifies authenticated provider call context and exact attach grants (`dev-docs/glade/GladeSupplierModel.md:180-247`), but the supplier kit currently says any session may append in stage one and its hello path does not establish the ratified trust contract (`glial/src/supplier/index.ts:1-25`, `glial/src/supplier/index.ts:405-415`). That gap is architectural, not merely polish.

## 11. Smallest useful proof

The smallest proof that exercises the proposed contract is a scoped Garns live question plus one command, projected as SWMR snapshot/delta and exchange. It is more valuable than another in-memory shape demo because it crosses every contested boundary without requiring file blobs or PTY recovery.

The proof should contain exactly one versioned `SurfaceDefinition`, one `BindingInstance`, one authenticated source-authority advertisement, one read-only replica advertisement, a Garns source port, and a replica port. Its observable acceptance criteria are:

1. An authorized subscriber receives an initial scoped snapshot and a committed transaction delta; a differently scoped principal does not.
2. A command commits Garns state, ledger, and outbox intent; an injected crash before acknowledgement causes retry with the same publication ID and no double effect.
3. A replica can restore and serve permitted snapshot/tail data but cannot execute the command.
4. An advertisement with no exact grant, an expired lease, an old authority term, or an unsupported shape version is rejected before route mutation.
5. Client disconnect and local restart recover from checkpoint/tail or receive an explicit reset when below the retained floor.
6. Revocation stops subsequent authorized reads/commands at the correct boundary, with the documented offline limitation.
7. The Garns ledger revision, Glade op identity, and shape cursor are all observable and demonstrably non-interchangeable.

Passing this proof would establish the stack contract. It would not claim file conflict handling, CRDT compaction, terminal survival, arbitrary provider migration, or every retention policy.

## 12. Findings and uncertainties

- **ASCR-F01 — Contract factoring:** one coherent contract requires linked definition, binding, and advertisement records; the current flat declaration surfaces conflate those lifetimes.
- **ASCR-F02 — Authority-safe interfaces:** source and replica ports require distinct semantics even if they share lifecycle machinery.
- **ASCR-F03 — Discovery is not authority:** a provider advertisement is acceptable only after exact grant, principal, assignment, term, shape, lease, and freshness validation.
- **ASCR-F04 — Shape ownership:** Taut owns merge/observation semantics; application views and supplier transports must name an exact base rather than invent a parallel shape vocabulary.
- **ASCR-F05 — Namespace mapping:** Glade domain/zone, Garns scope, parameter identity, and principal policy are related but independent axes. GDL-039 must be ratified or refined.
- **ASCR-F06 — Garns publication seam:** a durable outbox and idempotent Glade publication are required; neither Garns nor Glade currently exposes the complete contract.
- **ASCR-F07 — Recovery classes:** authoritative data, replicas, projections, private supplier state, caches, and live processes need distinct deletion and recovery rules.
- **ASCR-F08 — Security gap:** the ratified provider-security model is stronger than the current node and supplier-kit implementation.
- **ASCR-F09 — Status drift:** GLP-0006's final decision record ratifies items that subsidiary documents still describe as open or recommended; CRDT declaration support is also inconsistent across docs and code.
- **ASCR-F10 — Proof target:** a Garns scoped live question plus command is the smallest vertical slice that can validate definition composition, authority, publication, replay, and revocation together.

The decisions required before contract design are recorded in `dev-docs/ApplicationStackContractDecisions.md`. Implementation can continue on isolated conformance fixtures and non-semantic plumbing, but publishing a stable cross-layer schema before the first six decisions are resolved would freeze the current ambiguities into compatibility obligations.

## Inspection and testing limits

This was a read-only architecture review of the checked-out workspaces and their recorded heads. No implementation, tests, services, or migrations were changed or run. Existing dirty documentation files in the root and `glade-discover` member were treated as user work and were not modified. Assertions about absent behavior are based on repository searches and inspected call paths, not on exhaustive runtime fault injection. The current demo was classified from its source and existing review evidence; no live demo process was started.
