# Glade buy/build decision matrix

Date: 2026-09-12. Status: **decision worksheet for the owner; consolidates recorded
decisions and evaluated options, decides nothing new.** Each row names a requirement
Glade must meet, what has already been decided about it, which existing tools or
libraries could carry it, whether they fit, and what stays to be built. Rows marked
OPEN are the buy/build questions still to answer; §4 lists them as decisions.

Vocabulary: **buy** = adopt an existing tool, library or hosted service; **build** =
Glade-owned code or contract; **DECIDED** = ratified or ruled in the decision log or
a frozen design; **LEAN** = an assistant recommendation on recorded evidence; **OPEN**
= no ruling yet. Option fit: **fits**, **fits with issues** (issues named), **does not
fit** (reason named), **not evaluated**.

Sources: [GladeRequirements.md](glade/GladeRequirements.md) (GLA), [GladeProblemInventory.md](GladeProblemInventory.md)
(GPI), the captured concerns in the [Gyld frame](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-problem-space.gyld.py),
[DecisionLog.md](DecisionLog.md) (GDL), [GladeSubstrateV1.md](../glade/dev-docs/GladeSubstrateV1.md) (GQ),
[GladeAuthzModel.md](glade/GladeAuthzModel.md) (AZ), [GladeDiscoveryDesign.md](glade/GladeDiscoveryDesign.md) (INV-D),
[arch1](arch1/GladeArchitecture.md), the [DI evaluation](arch1/DependencyInjectionEvaluation.md),
the [async lifecycle research](Rust%20Async%20Lifecycle%20Orchestration.md), the
[iroh review](../glade/dev-docs/IrohReview.md) and [mapping](IrohGladeMapping.md),
the [persistence review](GladePersistenceReview.md), the [security analysis](../glade/dev-docs/glade_sec_model-55/GladeGrythSecurityModelAnalysis.md)
and the [consolidated P2P research](GLResearchConsolidatedFindings.md).

## 1. Decisions already made (the fixed points)

| # | Decision | Where | What it fixes for buy/build |
|---|---|---|---|
| D-01 | Substrate core in Rust; browser client is a full TypeScript session with folds, no wasm P2P | GQ-4, GQ-8 | Rust crates for the node; TS packages for the client; Rust/TS/Python parity by corpus, not by shared binaries |
| D-02 | Node-to-node transport is **iroh**; browser to node is a WebSocket carrying the same frames | GQ-2 (2026-06-13) | libp2p was the proving ground (GLP-0001) and is not the keeper; iroh wasm/bindings not needed |
| D-03 | Schema, wire and canonical encoding are **taut** (canonical CBOR, generated Rust/TS/Python codecs, golden corpus) | GDL-020, GDL-021 direction; GQ-9; D10 | No JSON schema, protobuf, JOSE or irpc on the wire |
| D-04 | Delivery shapes are the **Taut shape catalogue** (`value`, `atom`, `log`, `stream`, `swmr`, `crdt`; `text_crdt` a profile) | GDL-041 (2026-08-28) | Shape engines are owned; third-party CRDTs enter only as a profile payload engine |
| D-05 | Causal refs are the GQ-9 hybrid: `(origin, seq)` ids, per-origin `prev` hash chain, version-vector sync, signed checkpoints later | GQ-9 | Rules out last-writer-wins stores as the record substrate |
| D-06 | Discovery is a **fold over signed records**, not a service; no Paxos/Raft cell; leases fenced by local locks | WD-8, Discovery Model §0/§5, GDL-037 | Consensus libraries are not on the critical path |
| D-07 | Three discovery layers never merge: session placement → service discovery (fold) → node discovery (iroh) | GDL-032 (ratified) | iroh is layer 3 only |
| D-08 | Grants are data in the share (`CapabilityGrant`/`CapabilityRevocation`, revocation wins); one pure `check()`; JWT never travels inside Glade; OIDC only as edge authentication input | AZ §2–§6, B4/B5 rulings | Authorization is built; identity providers are pluggable at the edge |
| D-09 | Node trust is the operator relation; placement is a granted verb (`replica.hold`, `session.host`) | GDL-031 | Relay or store operators gain no authority by hosting |
| D-10 | System data seam: `RegistryApi` queries-over-fold and `StoreApi` whole-state blob now, **SQLite as a store engine later, never the replication mechanism** | GDL-036 | Database choice cannot change the protocol |
| D-11 | Glial owns client assembly and local-first persistence (IndexedDB engine built and wired) | GDL-035, GC-4 | Browser storage is decided; Glade optional for local-only |
| D-12 | Base glade is app-agnostic; `.glade` declaration files compile to records; management is ordinary bindings | GDL-037, GDL-038 | No admin plane product to buy |
| D-13 | Library boundaries, interface/implementation separation, TDD, fast isolated tests, architecture gates | GDL-042, LibraryBoundaryAndTestingPolicy | Every buy sits behind a contract with a conformance suite |
| D-14 | **Dependency injection: Shaku**, at NodeAssembly only; domain contracts never import it | GDL-048 (2026-09-09); candidate 1 revision 3 | Dill evaluated and not adopted (§3, R27) |
| D-15 | Tokio-based node host retained; pure and contract crates do not import Tokio; no actor framework, macro system or lifecycle framework required by domain APIs | arch1 A5; GAD-10 | Lifecycle orchestration lives at the runtime host; owner direction GDL-049 names sdax-rs (R28) |
| D-16 | Gyld is the modelling and comparison tool, not a source of requirements | GladeBuildEntry | No architecture product to buy |

## 2. The matrix

Requirement IDs cite the kernel requirements (GLA), the problem inventory (GPI) and
the captured concern names. "Build" names the Glade component from candidate 1
revision 3 that owns the residue.

### A. Connectivity and bootstrap

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R1 | Peer transport identity and mutual authentication (GPI-07 transport leg; IdentityIntegrity; P2P-first identity split) | DECIDED (iroh) | **iroh core 1.x**: fits (raw-public-key TLS, proven `remote_id()`, Ed25519 only). Bump from 1.0.2 to 1.2.0 for the 1.1.0 security fixes. | Transport-key ↔ node-principal binding record; HELLO verification against `remote_id()`; Ed25519-only is accepted |
| R2 | Reconnection across NAT, relay fallback, path migration (GPI-02; GLA-040/043; Reconnection) | DECIDED mechanism, OPEN hosting | **iroh relays + multipath**: fits. **n0 free public relays**: does not fit production (rate limited, no SLA, metadata visible). **Self-hosted `iroh-relay`**: fits (free, same binary, authenticated relays). **Iroh Services Pro/Dedicated**: fits at 19 or 199 USD/month. | Session resume, heads, correlation; relay operations or subscription |
| R3 | Node discovery: node key → address (GDL-032 layer 3; Directory §3 ladder step 4) | DECIDED layer, OPEN provider | **iroh-dns + self-hosted `iroh-dns-server`**: fits. **n0 DNS at dns.iroh.link**: fits for development only (no SLA). **iroh-mdns-address-lookup** (0.x): fits for LAN. **iroh-mainline-address-lookup** (0.x DHT): fits with issues (serverless but 0.x). | Publish only the bound endpoint key; retire `NodeHint` records; kernel never sees addresses |
| R4 | First contact and invite ceremony (GPI-01; GLA-001/002; FirstContact; GDL-008/013) | LEAN | **iroh-tickets** (1.x): fits for the out-of-band string (addresses + payload). Nothing exists for trust: genesis, device certs and grants are Glade's. | Genesis bundle, device-cert grant payload, Admission of the ticket as untrusted input, home-node availability role |
| R5 | Traffic budgets: control, interactive and bulk lanes under pressure; hostile peers (GPI-03; GLA-072; TrafficBudgets) | OPEN quantities | **QUIC per-stream flow control** (iroh): fits with issues (no cross-stream priority). **Relay rate limiting** (`iroh-relay`, off by default): fits as a backstop. **EndpointHooks**: fits as a peer pre-filter. | Priority scheduler (the unit-tested `OutQueue` is not wired), decode budgets, fan-out bounds, budget numbers |
| R6 | Browser participation (GQ-2; ConsumerMeaning) | DECIDED | **WebSocket to a node**: fits (built). **iroh wasm** (relay-only) and **official bindings** (endpoint-only): do not fit the need and are unnecessary under GQ-2. | Nothing new |

### B. Shared knowledge and discovery

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R7 | Service discovery: who serves `(share, glade_id?, key?)`, leases, epochs, takeover, owner-rooted serve grants, absence as data (GPI-05; DiscoveryKnowledge; INV-D0..D6) | DECIDED (build) | **iroh address lookup**: does not fit (keys, not slots). **iroh-docs / iroh-smol-kv**: do not fit (last-writer-wins, no owner-rooted grants). **Consul/etcd/ZooKeeper-class registries**: do not fit D-06 (live service on the read path; central authority). | `glade-discover` kernel v3.1 (frozen), simulator and scenarios (built, RED-first) |
| R8 | Dissemination and anti-entropy of records between trusted nodes (GPI-06; MetadataVisibility; INV-D5/D6) | LEAN: build now, gossip later | **Glade sync round** (SyncStart/SyncOps/SyncEnd): fits (built, one pull each way plus best-effort home push). **iroh-gossip** 0.101: fits with issues (0.x outside the wire promise; best effort; 4 KiB default message; no author binding, harmless; no documented per-topic admission, so scope must be enforced at accept or per-ALPN instance). **libp2p pubsub**: does not fit D-02. | Transitive propagation, standing home-share interest, the dissemination-scope model (mapping §5.2) |
| R9 | Sharded registries, referrals, bounded indexes toward the million-node target (GAD-04/05; FutureGrowth; GDL-014/018) | OPEN (deferred by design) | No iroh component provides a record registry, index or DHT for application records (`iroh-dht-experiment` is an experiment). **Raft/Paxos libraries** (for example `openraft`): fit with issues only for shard reconfiguration if ordered coordination is ever required; not for advertisements (D-06); crash-fault consensus is no defence against malicious members. | `ShardLocator` with a trusted mapping first; delegation proof schema; reconfiguration protocol later |
| R10 | Versioned definitions, bindings, declaration packages and cross-language parity (GPI-04; DefinitionKnowledge; GDL-007/015/019/020/021) | DECIDED direction (taut, `glade-decl`) | **taut**: fits (own). **Protobuf/JSON Schema/Cap'n Proto**: do not fit D-03. | `glade-decl` leaf split into four repos; GDL-019/020/021 exact formats |

### C. Identity, permission and isolation

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R11 | Signatures, hashing and canonical bytes for ops, grants and checkpoints (B5; PreservedValidation; GQ-9) | DECIDED primitives | **ed25519-dalek, blake3, rustls, sha2** (already in the graph via iroh and the node): fit. **Web Crypto** in the browser: fits with issues (async-only, so TS uses a pure sha256 for op hashes). | Signed-op envelope field 11, verifier seams (`Signer`/`Verifier` drafts), key storage |
| R12 | Capability grants: owner-rooted, attenuable, revocable, offline-checkable, cross-language (GLA-050..053; PolicyFreshness; AZ §2/§3; GDL-009/034) | DECIDED model, OPEN proof family (SEC-55-D1) | **JWT/JOSE**: does not fit inside Glade (no offline attenuation, wrong verification topology, central revocation lists). **Biscuit, UCAN, macaroons, CWT**: fit with issues as proof encodings (foreign encodings versus taut parity; Biscuit brings a Datalog runtime; UCAN is JWT-shaped). **iroh-docs namespace capabilities**: do not fit (per namespace, non-attenuable, no revocation). | Taut-encoded grant/revocation records, pure `check()` with golden vectors, proof slots (`capability_ref`) |
| R13 | Authentication methods and enterprise identity facts (GDL-033; IdentityIntegrity; security analysis §trust providers) | DECIDED policy ownership, OPEN adapters (AZ-3) | **OIDC/OAuth2 libraries**, **Kerberos/AD/LDAP clients**, **SPIFFE/SPIRE**: fit as edge adapters behind the trust-provider interface; additive. **iroh node id as human identity**: does not fit (transport only). | Device certs under the user root, `NodeBinding`/`SessionBinding` records, authn-method policy as user data |
| R14 | Scope and isolation: domain/zone/surface tuple, identity-bound private zones, canonical keys, no aliasing (GPI-09; ScopeIsolation; GDL-039; AZ-16/17) | DECIDED vocabulary, OPEN axes | No external component expresses it; iroh offers only accept-by-key-and-ALPN. | `canonicalize_scope` in Binding; zone-key resolver; remaining axis vocabulary (GDL-039 open items) |
| R15 | Key custody, rotation and recovery for roots, devices, nodes and operators (WD-1, AZ-7, SEC-55-D5) | OPEN | **OS keychains, hardware tokens, passkeys/WebAuthn**: not evaluated. | Custody posture is a product decision before any adapter |

### D. Authority and effects

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R16 | Exclusive source authority, epochs and resource-enforced fencing (GPI-10; GLA-020..024; SourceAuthority) | DECIDED shape | **Filesystem locks** (`workspace.lock`) as the ground truth: fit. **Distributed lock services**: do not fit D-06. | Supplier-host epochs, takeover grants, per-resource fence in the supplier |
| R17 | Directed invocation with correlation, honest Unknown outcomes and attempt identity (GPI-11; GLA-035..038; CommandOutcome) | DECIDED contract draft | **irpc** (0.x, Rust-only): does not fit the cross-language wire. **QUIC streams** (iroh): fit as the carrier of taut EXCHANGE frames. | `Invoker` draft, supplier-side idempotency, outcome lookup |
| R18 | Commit and publication recovery without repeating effects (GPI-12; CommitPublication) | OPEN per source | **Garns**: fits with issues as an application source backend (single-process, SQLite-local, no browser, no replication; not mandatory Glade storage). | Supplier outbox/publication intent; per-source uncertainty policy |

### E. Durability and recovery

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R19 | Durable local acceptance: journal, operation store, retry identity, restart-safe watermarks (GPI-13; AcceptanceMeaning; GDL-036/043/045) | DECIDED contracts, OPEN engine | **Whole-state taut blob on disk** (current `BlobStore`): fits for now. **SQLite via rusqlite**: fits later behind `StoreApi` (D-10). **redb** (pure Rust, used by iroh-blobs/docs): fits with issues (not evaluated for Glade; embedded-DB-in-async friction noted by n0). **sled**: not evaluated. **IndexedDB** (browser): decided in Glial. | `AcceptanceJournal`, `DurableOperationStore`, `SnapshotStore` drafts and their crash tests |
| R20 | Replica synchronisation: heads, gaps, chain verification, equivocation proofs (GQ-9; ReconcileMetadata; SyncContract) | DECIDED (build) | **iroh-docs range-based set reconciliation**: does not fit (D-05: no chains, LWW). **Automerge/Yjs sync protocols**: do not fit the envelope. | Heads/gap protocol (built for peers), checkpoints (AZ-12 ruled: origin-signed, replicated head) |
| R21 | Shape engines and folds, including collaborative text (GDL-041; RecoveryProfiles; GPI-14) | DECIDED (own catalogue) | **taut-shape** engines `crdt.oracle/v1` and `text_crdt.profile/v1`: built. **yrs/Yjs (+ pycrdt)**: fit with issues as an alternative text engine (binary compatibility across Rust/TS/Python documented as intent; 69 KB browser bundle); research called it the "safe hybrid default" but the own engine shipped first. **Loro** 1.x: fits with issues (Fugue, strong performance, JS+Rust; no Python parity). **Automerge** 3.x: fits with issues (1.7 MB browser bundle). | Profile conformance rows per engine; adapter matrix gates (AR-07) |
| R22 | Retention, compaction, checkpoints and storage ceilings (GPI-15; Retention; R2-11 deferred) | OPEN | No external component sets the policy. **iroh-blobs tags/GC**: do not fit as the record retention unit. | Byte ceilings (`MAX_RETAINED_BYTES`), signed checkpoints, revocation-evidence preservation |
| R23 | Bulk content transfer: verified, resumable, range-addressable (file suppliers, snapshot bootstrap, large exchange results) | OPEN | **iroh-blobs** 0.103: fits with issues (0.x; docs still say not production quality; provider event handler is the only authorization point, so Glade must map hash → share → `check()` per request; new port needed). **Own chunked framing over QUIC streams**: fits with issues (no content addressing or multi-provider resume). | `BulkTransferPort` contract, admission wrapper, budget integration |

### F. Source and consumer semantics

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R24 | Local-first client assembly, persistence receipts and incremental change events (GPI-18; ConsumerMeaning; GDL-035) | DECIDED (Glial) | **IndexedDB** (built): fits with issues (silent write failure in the current engine; the durability receipt is the gap). **OPFS / SQLite-wasm**: not evaluated. | Honest receipts (GAP-11), reassembler layer, interest regions |
| R25 | Consumer UI binding without Grip internals leaking (GLA-080; Grip/Glial boundary) | DECIDED (own) | **Grip** taps and **grip-share** binder: own. React demo. | `glade-decl` types-only import wall |
| R26 | Interest aggregation, subscriptions, explicit gaps, bounded fan-out (GLA-010..014; IndependentLifetimes; DeliverInterest) | DECIDED contract draft | Nothing external; **iroh-gossip** is not a subscription model. | `Subscriber`/`Subscription` profile review; delivery buffers and cursors |

### G. Runtime, composition and engineering

| # | Requirement | Status | Buy options and fit | Build residue |
|---|---|---|---|---|
| R27 | **Dependency injection**: narrow typed dependencies, explicit sharing scopes, one coordinated fake per test composition, no service locator (ScopedInjection; LBT-003/004/008/009; AR-03) | **DECIDED: Shaku** (GDL-048) | **Shaku**: fits with issues (synchronous construction; a service can outlive container drop via `Arc`; not a lifecycle supervisor; `ManagedResource` is not dyn-compatible, so runtime-polymorphic injection needs a reviewed bridge or generic composition). **Dill** 0.17: does not fit on present evidence (child catalog is not an override layer; parent singleton can capture a child dependency; transaction-cache first creation is not atomic; validation partial and opt-in). **Hand-rolled context bag (GWZ AppContext)**: does not fit (the pattern being escaped). | Assembly-only use; six binding recipes; the still-open async-port/startup/cleanup witness (DI-E01..E04) |
| R28 | Async lifecycle: owned work, admission, drain, reverse-order cleanup, cancellation, truthful shutdown reports (OwnLifecycle; RuntimeStrategy; AR-08; `ManagedResource`) | **OWNER DIRECTION: sdax-rs** (GDL-049, 2026-09-12) | **sdax-rs** (`sdax` std-only core, `sdax-tokio`, `sdax-testkit`; owner's library; stages 0–3 landed, conformance + Monte Carlo + adversarial review; private, unpublished): fits with issues (Git dependency until published; `sdax-tokio` pins Tokio; lifecycle conformance against `ManagedResource` and the Shaku bridge still to be witnessed). **Tokio primitives** composed in-house: fits; was the research lean, now the fallback. **tokio-graceful-shutdown** 0.20: fits with issues (untyped signalling; supervisor only). **dagx** 0.3, **dagrs** 0.8: fit with issues (DAG runners, no symmetrical teardown or type-erased). **moro**: does not fit (inactive research crate). **flawless/Temporal**: does not fit (durable execution engine, wasm compilation). **Actor frameworks** (actix, ractor): not selected (A5). | Plan authoring for the node's resources and services, the AR-08 witness on sdax-rs, the async bridge to Shaku-built components |
| R29 | Declarative configuration and record-driven behaviour (DeclarativeConfiguration; GDL-037; `NodeConfiguration`) | DECIDED direction | **taut** declarations and `.glade` files: own. **Generic config crates** (config/figment): not evaluated; would sit below the validated profile. | `NodeConfiguration` profile; iroh presets, relay map and lookup services as validated inputs, never `presets::N0` |
| R30 | Observability: inspect interest, owner, term, generation, rejoin, lag; redaction (GLA-070/071; InspectableManagement; GDL-038) | DECIDED surface (ordinary bindings) | **iroh-metrics, Prometheus export, tracing, qlog, iroh-doctor**: fit for transport facts. | `node.status` system share, redaction policy, bounded diagnostics |
| R31 | Deterministic simulation and conformance: scenarios as data, virtual time, fault schedules, corpus parity (FastFeedback; VerifyIntegration; GDL-042) | DECIDED (own) | **glade-discover-sim** and the taut corpora: own. **madsim / turmoil** (deterministic Tokio simulators): not evaluated; would not replace the transport-free kernel oracle. **Graphviz** for rendering models: fits (used by Gyld). | Adapter real-I/O tier, relay-path journeys, wiring suite |
| R32 | Architecture policy gates: classifications, dependency edges, named traits, conformance targets (GDL-042; LBT; AR-09) | DECIDED (own) | **`glade-discover` local gate** and Gyld's `check_architecture`: own. **cargo-deny / cargo-udeps / cargo-modules**: not evaluated; complementary at most. | Wider adoption across repos; measured budgets |
| R33 | Workspace and multi-repo tooling (GWZ discipline) | DECIDED (own) | **gwz, razel**: own. | Unchanged |

## 3. Notes on the decided injector

Shaku was selected on 2026-09-09 after a bounded, measured evaluation of Shaku and
Dill (23 runtime probes, four expected compile failures). What the selection means
and does not mean:

- It is a **composition-root technology**: `NodeAssembly` is the only component that
  depends on it (candidate 1 revision 3 carries a regression witness that domain
  contracts do not). No public async trait changes.
- It does **not** decide lifecycle. DI scope answers "which instance is reused and
  where"; it cannot await shutdown or drain work. Both frameworks let a service
  outlive container drop through `Arc`, so the lifecycle contract (`ManagedResource`,
  retained unfinished shutdown work, truthful reports) is separate and still open.
- The **next witness** (DI-E01..E04) must show one injected provider scope on every
  caller path, a rejected real-I/O bypass, one coordinated fake per test composition,
  and the selected real async port usable without framework imports in contracts.
  A failed witness reopens the selection. Under GDL-049 the lifecycle side of that
  witness runs on sdax-rs: Shaku builds the components, sdax-rs owns their
  acquisition, run and reverse-order release.
- **Dill** is not adopted because its default scoped composition let a parent
  singleton capture a child's dependency, treated child registration as ambiguity
  rather than override, created first-use transaction-scope instances non-atomically,
  and validated only part of those faults.

## 4. The buy/build questions to answer

| # | Question | Rows | Lean on recorded evidence |
|---|---|---|---|
| Q1 | Relay and DNS posture for the first real-route slice: self-host, Iroh Services Pro, or n0 community for development only? | R2, R3 | Self-host both binaries; revise the discovery model's "public iroh relay" wording |
| Q2 | Bump iroh to 1.2.0 now and pin protocol crates individually? | R1 | Yes; nothing is parsed from untrusted addresses safely on 1.0.2 |
| Q3 | Transport-key binding: a new record kind under the node chain, or one key for both transport and node identity? | R1, R3 | Record kind; keep the identity split |
| Q4 | Dissemination: keep the Glade sync round only, or adopt `iroh-gossip` behind the carrier port once the scope model (topology, node trust or cryptography) is chosen? | R8, R14 | Sync round for the fixed-peer slice; gossip after the scope decision, pinned |
| Q5 | Bulk transfer: `iroh-blobs` behind a new port, own chunked streaming, or defer until a supplier needs it? | R23 | Defer; adopt blobs behind a port on the first bulk supplier |
| Q6 | Capability proof family: custom signed taut grants, Biscuit, UCAN, macaroons, CWT or hybrid (SEC-55-D1)? | R12 | Custom taut grants first; foreign families only if an interoperability need appears |
| Q7 | Which enterprise identity adapters, if any, in v1 (AZ-3)? | R13 | None in v1; keep the trust-provider interface |
| Q8 | Store engine sequencing: when does SQLite replace the whole-state blob, and is redb worth an evaluation? | R19 | Blob until query volume or size hurts; evaluate redb only with a measured need |
| Q9 | Text CRDT engine: keep the own `text_crdt` profile, or admit yrs/Yjs or Loro as a second profile engine? | R21 | Keep the own profile; admit a second engine only through a profile with its own conformance rows |
| Q10 | Async lifecycle: **directed to sdax-rs (GDL-049)**; remaining question is the witness and the dependency posture (Git now, crates.io after publication) | R28 | Run the AR-08 and DI-E01..E04 witness on sdax-rs plus Shaku; keep Tokio primitives as the fallback if it fails |
| Q11 | Key custody and recovery posture (WD-1, AZ-7, SEC-55-D5) | R15 | Product decision first; no library choice is meaningful before it |
| Q12 | Sharded registry and index growth: which parts, if any, ever need ordered coordination, and which library would carry it? | R9 | Trusted mapping only now; reopen with a failure model |
| Q13 | Deterministic simulators for adapter tiers (madsim/turmoil) and dependency-policy tooling (cargo-deny and friends): evaluate or not? | R31, R32 | Not before the first real-route slice |

## 5. Limits

This worksheet records decisions and evaluations as written on 2026-09-12; it does
not re-verify crate versions, licences or behaviour, and "not evaluated" means
exactly that. Fit judgments for iroh components come from the review and mapping;
for DI and async frameworks from the two evaluations; for CRDT engines from the
mid-2026 research. No row here changes a decision-log entry.
