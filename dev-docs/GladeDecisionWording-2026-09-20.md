# Glade decision wording, 2026-09-20: for you to correct

Every box in the decision graph now leads with the decision as a question. This document
puts the old text and the new text side by side so you can strike, reword or reject any of
it. It is written FOR CORRECTION, not as a record of something settled.

**What changed and what did not.** Only docstrings moved. No status, relation, source,
matrix row, class name or alternative list was touched, so the graph has exactly the shape
it had. A docstring is inside what a capture pins, so the base declaration is now lineage
`glade-decision-graph` revision `v2`; revision `v1` is what the v1 to v8 bundles hold and
they stay exactly as they were. The rebuilt bundle is `artifacts/decision-streams-v9` and
`GladeDecisionIndex.md` is regenerated from it.

**The shape each docstring now takes.**

- A question: first a real question in everyday words ending in a question mark, then a
  blank line and one to three short sentences saying what is at stake and what it holds up.
- An alternative: one or two sentences saying what taking it means in practice and what it
  costs.
- A trigger: what has to happen in the world, then one sentence on why the gated question
  is not worth answering before it.

**The marks.** Wherever a sentence says something no cited passage supports, it is flagged
below as **WROTE BEYOND THE SOURCES** with the reason. Strike those first: they are where I
guessed. Everything else can be checked against the source lines quoted with each entry.

**How to correct it.** Edit `gyld/examples/glade-decisions.gyld.py` in the gyld workzone and
rerun the index host; the boxes, the index and the bundle all read from that one file.

19 sentences across the 24 questions, 45 alternatives and 5 triggers are marked as written beyond the sources. Everything else is quoted or paraphrased from the lines listed under each entry.

## The nine that named a topic instead of asking a question

These are the ones the generated index flagged `names-a-subject`: the old sentence named the subject area and never said what was being decided.

### `async_witness` - matrix row `R27`

*declared as `AsyncWitness`, status `Open`.*

- **OLD**: The Shaku composition witness DI-E01..E04 with a real async port.
- **NEW question**: Does the chosen wiring actually survive a real async port, start-up and cleanup?
- **NEW stakes**: Shaku builds synchronously, is not a lifecycle supervisor, and needs a reviewed bridge for runtime-polymorphic injection, so the DI-E01 to E04 witness is what says whether it holds once sdax-rs and a real async port are in play. E04 is the sharp one: the real port must stay usable without framework imports in the pure crates.

**Alternatives.**

- `shaku_confirmed`
  - OLD: The witness passes; Shaku stays confined to assembly.
  - NEW: The witness passes and Shaku stays where the ruling put it, at assembly only. Nothing else has to move.
- `selection_reopened`
  - OLD: The witness fails; injector selection reopens.
  - NEW: The witness fails and the injector choice goes back on the table. Everything already wired through assembly is reconsidered with it.

**Stands on.**

- GladeBuyBuildMatrix.md, `G. Runtime, composition and engineering`, line 112 (R27): Shaku fits "with issues (synchronous construction; ... not a lifecycle supervisor; `ManagedResource` is not dyn-compatible, so runtime-polymorphic injection needs a reviewed bridge)", and "the still-open async-port/startup/cleanup witness (DI-E01..E04)".
- arch1/DependencyInjectionEvaluation.md, `Proposed acceptance for the next witness, not completed work`, line 136 (DI-E04): "The selected real async Glade port remains usable without framework imports in its contract/pure libraries".
- DecisionLog.md, `Notes`, lines 88-94 (GDL-048): Shaku selects the technology, "not a production Cargo installation, public-trait change, completed async bridge or automatic lifecycle guarantee".

> **WROTE BEYOND THE SOURCES** - `selection_reopened`: "Everything already wired through assembly is reconsidered with it". Nothing says what a failed witness costs; this is my reading of assembly-only use.

### `blind_relay_tier` - matrix row `R14`

*declared as `BlindRelayTier`, status `Open`.*

- **OLD**: Per-share payload encryption semantics for untrusted hops.
- **NEW question**: If a hop is not trusted with the content, what may it still do with it?
- **NEW stakes**: A blind relay carries per-share ciphertext it cannot read and forfeits node-side compute. What is open is whether it stores and forwards or is only a pipe, and what survives of resume and of heads when it cannot see inside.

**Stands on.**

- glade/GladeAuthzModel.md, `9. Open questions`, line 339 (AZ-10): "Blind-relay tier semantics: store-and-forward ciphertext vs pure pipe; what survives of resume/heads".
- glade/GladeAuthzModel.md, `7a. Operators, placement, and node trust`, lines 265-268: the blind relay/E2E tier is "owner-keyed payload encryption; nodes route and store ciphertext; node-side compute is forfeited".
- GladeBuyBuildMatrix.md, `C. Identity, permission and isolation`, line 79 (R14): "DECIDED vocabulary, OPEN axes".

### `bulk_transfer` - matrix row `Q5`

*declared as `BulkTransfer`, status `Lean`.*

- **OLD**: Verified, resumable bulk content transfer.
- **NEW question**: How does something too big for a record get moved, and is that worth building yet?
- **NEW stakes**: Verified, resumable, content-addressed transfer is genuinely missing from Glade and would be wanted by file suppliers, snapshot bootstrap and large exchange results. Whatever carries it must sit behind the same admission and budget rules as the ordinary carrier, not become a second delivery path around them.

**Alternatives.**

- `iroh_blobs_port`
  - OLD: iroh-blobs behind a new bulk-transfer port with an admission wrapper.
  - NEW: Adopt iroh-blobs behind a new bulk-transfer port. Authorisation is only the provider's event handler, so Glade must map each requested hash to its owning share and check per request; the crate's own docs still say it is not production quality.
- `own_chunked_streams`
  - OLD: Own chunked framing over QUIC streams; no content addressing.
  - NEW: Frame our own chunks over QUIC streams. Nothing 0.x is taken on, and there is no content addressing, so verification and resumption would have to be built.
- `defer_bulk`
  - OLD: No bulk path until a supplier needs one.
  - NEW: Build no bulk path at all until a supplier actually declares one. It costs nothing now, and the first supplier that needs one waits for it.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 152 (Q5): "`iroh-blobs` behind a new port, own chunked streaming, or defer until a supplier needs it?"
- IrohGladeMapping.md, `4.3 Shape C - protocol crates as the replicated substrate`, lines 202-214: content-addressed, verified, resumable transfer "is genuinely absent from Glade and needed by file suppliers, snapshot/checkpoint bootstrap ..., large exchange results and invite bundles"; "a concrete I/O adapter behind a contract with the same admission and budget semantics as the carrier, not a second delivery path that bypasses `Delivery`"; "Any peer may fetch any hash the store holds unless the handler rejects"; "0.x, 'not production quality' per its own docs".

> **WROTE BEYOND THE SOURCES** - `own_chunked_streams`: "so verification and resumption would have to be built". §4.3 names content addressing as what iroh-blobs gives; the consequence of not having it is mine.

> **WROTE BEYOND THE SOURCES** - `defer_bulk`: "the first supplier that needs one waits for it". My reading of what deferring costs.

### `key_custody` - matrix row `Q11`

*declared as `KeyCustody`, status `Open`.*

- **OLD**: Root, device, node and operator key custody and recovery posture.
- **NEW question**: Who holds the keys, and what happens when someone loses every device?
- **NEW stakes**: This covers the root key, each device, each node and the operator's own root at organisation scale. It decides how frightening first setup has to be, and no library choice below it means anything until it is answered: proof_family and transport_key_binding are both waiting on it.

**Alternatives.**

- `owner_held_only`
  - OLD: Keys live only on the owner's devices; loss is loss.
  - NEW: Nothing is minted beyond the owner's own devices, so first setup stays simple and there is no extra secret to look after. Lose every device and the identity is gone.
- `recovery_keys`
  - OLD: Offline recovery material minted at genesis.
  - NEW: Extra recovery material is minted at first setup and kept offline, on paper or in a safe. Loss becomes survivable, and that material is now itself something to protect.
- `social_recovery`
  - OLD: Threshold recovery across trusted principals.
  - NEW: Recovery takes several trusted principals acting together, so no single loss is fatal. It is the most to build, and it puts recovery partly in other people's hands.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 158 (Q11): "Key custody and recovery posture (WD-1, AZ-7, SEC-55-D5)"; the lean column reads "Product decision first; no library choice is meaningful before it".
- glade/GladeWorkspaceDirectory.md, `8. Open questions (not decided here)`, line 267 (WD-1): "Root-key custody & recovery: lost-all-devices posture (paper backup? recovery keys? social recovery?) - determines how scary genesis is".
- glade/GladeAuthzModel.md, `9. Open questions`, line 336 (AZ-7): "Operator/org root custody and rotation (the DC fleet's chain root) - mirrors WD-1 at org scale".
- glade/glade_sec_model-55/GladeGrythSecurityModelAnalysis.md, `Decision Log Items To Carry Forward`, line 704 (SEC-55-D5): "Define node/provider key storage and rotation requirements."
- The declaration's own `Requires` edges: `proof_family` and `transport_key_binding` both require `key_custody`.

> **WROTE BEYOND THE SOURCES** - `owner_held_only`: "so first setup stays simple and there is no extra secret to look after". The simplicity is my inference from there being nothing extra to mint; WD-1 only asks the question.

> **WROTE BEYOND THE SOURCES** - `recovery_keys`: "on paper or in a safe". WD-1 names a paper backup; "or in a safe" is mine.

> **WROTE BEYOND THE SOURCES** - `social_recovery`: "It is the most to build, and it puts recovery partly in other people's hands". No cited passage compares the cost of the three, and the second clause is my reading of what threshold recovery means.

### `proof_family` - matrix row `Q6`

*declared as `ProofFamily`, status `Lean`.*

- **OLD**: Capability presentation proof encoding.
- **NEW question**: What does a capability look like on the wire when a caller presents one?
- **NEW stakes**: Permission is already settled as records and one pure check; this is the encoding the presented proof carries. A foreign family brings its runtime and its wire format with it, and the Rust, TypeScript and Python corpora all have to agree on the bytes. identity_adapters waits on the answer.

**Alternatives.**

- `taut_grants`
  - OLD: Custom signed taut grants with Rust/TS/Python corpus parity.
  - NEW: Keep the encoding our own: a chain of signed taut grants where each link is signed by the parent principal and may only narrow scope, with one corpus proving the three languages agree. Nothing foreign enters, and outside interoperability is unbuilt.
- `biscuit`
  - OLD: Biscuit tokens; brings a Datalog runtime and a foreign encoding.
  - NEW: Adopt Biscuit tokens, an existing attenuable-token family with the shape Glade already wants. It brings a Datalog runtime and an encoding we do not control.
- `ucan`
  - OLD: UCAN; JWT-shaped encoding.
  - NEW: Adopt UCAN, whose proofs are JWT-shaped. The delegation semantics are close to what the grant chain already does, and it puts JWT inside Glade, which AZ §2 kept out.
- `macaroons`
  - OLD: Macaroons; caveat chains with a foreign encoding.
  - NEW: Adopt macaroons, where narrowing a grant is a chain of caveats. The model fits attenuation well, and the encoding is again one Glade does not own.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 153 (Q6): "Capability proof family: custom signed taut grants, Biscuit, UCAN, macaroons, CWT or hybrid (SEC-55-D1)?"; the lean column reads "Custom taut grants first; foreign families only if an interoperability need appears".
- glade/glade_sec_model-55/GladeGrythSecurityModelAnalysis.md, `Decision Log Items To Carry Forward`, line 700 (SEC-55-D1): the same list of families.
- glade/GladeAuthzModel.md, `2. Not JWT - and what survives of it`, lines 48-51 (AZ §2): what survives is "the signed-statement-of-claims primitive, upgraded to a capability chain (UCAN/biscuit-shaped semantics, our own taut encoding): each link is signed by the parent principal, names a subject key, and may only narrow scope".
- glade/GladeAuthzModel.md, `6. The check function - one pure function, everywhere`, line 204: byte-identical verdicts across Rust/TS/Python.
- The declaration's own `Requires` edge: `identity_adapters` requires `proof_family`.

> **WROTE BEYOND THE SOURCES** - `macaroons`: "The model fits attenuation well". AZ §2 names UCAN/biscuit-shaped semantics, not macaroons; the fit is mine.

### `registry_growth` - matrix row `Q12`

*declared as `RegistryGrowth`, status `Lean`.*

- **OLD**: Sharded registries, referrals and indexes toward the growth target.
- **NEW question**: When the registry outgrows one shard, what splits it, and does anything need ordering?
- **NEW stakes**: The growth target is a design target of roughly a million machines, not a demonstrated result, and it means sharded registries, referrals between them and bounded indexes over them. The real question is which parts, if any, ever need ordered coordination, because that is the one thing the fold cannot do.

**Alternatives.**

- `trusted_mapping_only`
  - OLD: An explicit trusted namespace-to-shard mapping; nothing dynamic.
  - NEW: Write down which namespace lives on which shard and trust that mapping. Nothing is dynamic and nothing has to agree, and every change to the layout is made by hand.
- `delegated_referrals`
  - OLD: Authenticated referrals and local preference over the same fold.
  - NEW: Let a shard hand a caller on with an authenticated referral, with local preference, over the same fold. The layout can change without a central edit, and no device has to hold every entry; there is still no ordering guarantee across shards.
- `ordered_reconfiguration`
  - OLD: Consensus-backed shard split, move and membership.
  - NEW: Back shard splits, moves and membership with consensus. It is the only option that makes reconfiguration ordered, and the only one that brings a consensus library into a system that deliberately kept one off the read path.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 159 (Q12): "Sharded registry and index growth: which parts, if any, ever need ordered coordination, and which library would carry it?"; the lean column reads "Trusted mapping only now; reopen with a failure model".
- GladeArchitectureDiscussion.md, `4. GAD-04 - Owner's discovery and scaling proposal`, line 60: "as a design target, a backend of roughly one million machines", and line 67: "The scale statement is a target, not a demonstrated capacity result."
- GladeArchitectureDiscussion.md, `5. GAD-05 - Reviewer refinements, not yet adopted`, line 81: "Cache bounded referrals and relevant indexes", to avoid "requiring every device to store every registry entry".
- The `discovery_fold` anchor of this declaration: no consensus cell on the read path.

> **WROTE BEYOND THE SOURCES** - `registry_growth`: "because that is the one thing the fold cannot do". This leans on the `discovery_fold` anchor rather than on GAD-04 or GAD-05.

> **WROTE BEYOND THE SOURCES** - `delegated_referrals`: "there is still no ordering guarantee across shards". GAD-05 proposes referrals and bounded indexes without saying this.

### `relay_posture` - matrix row `Q1`

*declared as `RelayPosture`, status `Lean`.*

- **OLD**: Relay and DNS hosting for the first real-route slice.
- **NEW question**: Whose relays and whose DNS does the first real route run on?
- **NEW stakes**: A real route needs somewhere to fall back to when a direct connection cannot be made, and somewhere to look addresses up. n0's free tier says in its own terms that it is not suitable for production and that a relay operator sees connection metadata, while the discovery model still assumes a public iroh relay; that wording has to move too.

**Alternatives.**

- `self_hosted`
  - OLD: Run iroh-relay and iroh-dns-server; free; authenticated relays available.
  - NEW: Run n0's own iroh-relay and iroh-dns-server yourself; the binaries are the ones they run, and it is free beyond the hosting. Authenticated relays come with it, through an allowlist or a shared password, so a leaked relay address alone is useless.
- `iroh_services_pro`
  - OLD: Paid shared relays at 19 USD per month.
  - NEW: Pay 19 USD a month for authenticated shared relays with a rate limit, an egress allowance and support tickets. Nothing has to be run, and connection metadata sits with someone else.
- `community_dev_only`
  - OLD: n0 free relays for development only; not for production.
  - NEW: Use n0's free public relays for development only. They cost nothing, are rate limited and carry no uptime guarantee, and n0 advise against them for sensitive data.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 148 (Q1): "Relay and DNS posture for the first real-route slice: self-host, Iroh Services Pro, or n0 community for development only?"; the lean column reads "Self-host both binaries; revise the discovery model's 'public iroh relay' wording".
- glade/dev-docs/IrohReview.md, `7. Hosted services`, lines 461-488: free community tier "not suitable for production", n0 "advises against public relays for sensitive data because connection metadata is visible to the relay operator"; Pro at "$19/month"; authenticated relays admit "only endpoints presenting a signed capability token bound to their key; a leaked relay URL alone is useless"; "Self-hosting everything (relay + DNS server) is supported and free; the binaries are the ones n0 runs."
- glade/GladeDiscoveryModel.md, `5. Topology - locality-aware rendezvous`, lines 198-199: "v1 = flat (small p2p mesh + the public iroh relay)".

### `simulator_tooling` - matrix row `Q13`

*declared as `SimulatorTooling`, status `Lean`.*

- **OLD**: Deterministic runtime simulators and dependency-policy tools for adapter tiers.
- **NEW question**: Do we bring in outside simulators and dependency-policy tools, or stay on our own?
- **NEW stakes**: The kernel oracle, glade-discover-sim and the taut corpora are already ours and decided, and the outside tools were never evaluated and would be complementary at most. The matrix says not to look before the first real route passes, so what is really being weighed is the cost of looking early.

**Alternatives.**

- `own_sim_only`
  - OLD: glade-discover-sim and the taut corpora only.
  - NEW: Stay with glade-discover-sim and the taut corpora. Nothing is added, and the adapter tiers keep testing against what is already there.
- `deterministic_runtime_sim`
  - OLD: madsim or turmoil for adapter-tier journeys.
  - NEW: Bring in madsim or turmoil to run adapter-tier journeys under virtual time and fault schedules. Neither replaces the transport-free kernel oracle, so it is an addition to keep working rather than a replacement for anything.
- `dependency_policy_tools`
  - OLD: cargo-deny and related checks beside the own gate.
  - NEW: Run cargo-deny and its relatives beside the own architecture gate. They cover policy the local gate does not, and the matrix judges them complementary at most.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 160 (Q13): "Deterministic simulators for adapter tiers (madsim/turmoil) and dependency-policy tooling (cargo-deny and friends): evaluate or not?"; the lean column reads "Not before the first real-route slice".
- GladeBuyBuildMatrix.md, `G. Runtime, composition and engineering`, line 116 (R31): "DECIDED (own)"; "madsim / turmoil ...: not evaluated; would not replace the transport-free kernel oracle".
- GladeBuyBuildMatrix.md, `G. Runtime, composition and engineering`, line 117 (R32): "cargo-deny / cargo-udeps / cargo-modules: not evaluated; complementary at most."

### `traffic_budget_numbers` - matrix row `R5`

*declared as `TrafficBudgetNumbers`, status `Open`.*

- **OLD**: Control, interactive and bulk budgets as quantities; per-lane streams or connections.
- **NEW question**: How much traffic is each lane allowed, and does a lane get a stream or a connection?
- **NEW stakes**: Control, interactive and bulk traffic already have separate lanes, but the quantities are open and a hostile peer is what makes them matter: bulk data or hostile advertisements can starve lease renewal and terminal input. QUIC gives per-stream flow control and no priority across streams, so the numbers have to come from Glade.

**Stands on.**

- GladeBuyBuildMatrix.md, `A. Connectivity and bootstrap`, line 60 (R5): "Traffic budgets: control, interactive and bulk lanes under pressure; hostile peers"; "OPEN quantities"; "QUIC per-stream flow control (iroh): fits with issues (no cross-stream priority)"; open items include "fan-out bounds, budget numbers".
- GladeProblemInventory.md, `A. Connectivity and bootstrap`, line 36 (GPI-03): "Bulk data or hostile advertisements starve lease renewal and terminal input."
- glade/GladeRequirements.md, `Observability and Cost Discipline`, line 109 (GLA-072): bounds for "projection size, replay cost, and upstream subscription cardinality".

## The other fifteen questions

### `iroh_transport` - no matrix row

*declared as `IrohTransport`, status `Decided`.*

- **OLD**: Node-to-node transport is iroh; browsers use a WebSocket to a node.
- **NEW question**: How do two nodes reach each other, and how does a browser get in?
- **NEW stakes**: Settled: node-to-node traffic runs over iroh, and a browser talks to a node over a WebSocket carrying the same frames. Browser-to-browser peer-to-peer was tried and dropped. Everything below about relays, addresses and node keys assumes this.

**Stands on.**

- glade/GladeSubstrateV1.md, `10. Open decisions (gate the build)`, line 283 (GQ-2): "Transport: iroh, node-to-node only. Browser-node is a websocket carrying the same frames. wasm/browser p2p is dead".

> **WROTE BEYOND THE SOURCES** - `iroh_transport`: "Everything below about relays, addresses and node keys assumes this". This reads the declaration's own `Requires` edges rather than GQ-2.

### `discovery_fold` - no matrix row

*declared as `DiscoveryFold`, status `Decided`.*

- **OLD**: Discovery is a fold over signed records; no consensus cell on the read path.
- **NEW question**: How does a node work out what exists and who is holding it?
- **NEW stakes**: Settled: by folding signed records, with no consensus step anywhere on the read path. The fold is the only runtime authority, so a read stays local and replayable and nothing has to agree before a question can be answered.

**Stands on.**

- DecisionLog.md, `Open Decisions`, line 55 (GDL-037): "fold stays the only runtime authority"; ratified 2026-07-07.
- glade/GladeWorkspaceDirectory.md, `8. Open questions (not decided here)`, line 274 (WD-8): RULED 2026-07-05.

> **WROTE BEYOND THE SOURCES** - `discovery_fold`: "so a read stays local and replayable and nothing has to agree before a question can be answered". GDL-037 says the fold is the only runtime authority; the consequence is mine.

### `grants_as_data` - no matrix row

*declared as `GrantsAsData`, status `Decided`.*

- **OLD**: Grants and revocations are share records; one pure check(); no JWT inside Glade.
- **NEW question**: How is permission written down, and what decides whether a request is allowed?
- **NEW stakes**: Settled: a grant and a revocation are ordinary share records, and one pure check over the grant fold gives the same verdict in Rust, TypeScript and Python. No JWT is carried inside Glade. Permission is data that replicates and replays like the rest.

**Stands on.**

- glade/GladeAuthzModel.md, `2. Not JWT - and what survives of it`, lines 39-51: JWT rejected; the capability chain in "our own taut encoding" survives.
- glade/GladeAuthzModel.md, `6. The check function - one pure function, everywhere`, line 204: one pure `check(principal_chain, grant_fold, resource, verb)`, "no I/O, byte-identical verdicts across Rust/TS/Python".

### `store_seam` - no matrix row

*declared as `StoreSeam`, status `Decided`.*

- **OLD**: RegistryApi over the fold; StoreApi whole-state blob now, SQLite an engine later.
- **NEW question**: What sits between Glade and the disk, and what is that allowed to be?
- **NEW stakes**: Settled: RegistryApi answers queries over the fold and appends records; StoreApi holds a whole-state taut blob now, with a real engine possible behind it later. A store engine is never the replication mechanism, which stays in the ops.

**Stands on.**

- DecisionLog.md, `Open Decisions`, line 54 (GDL-036): "RegistryApi (queries-over-fold + record appends ...) and StoreApi (whole-state taut-message blob now -> SQLite engine later; SQLite is a store engine, never the replication mechanism - replication stays ops)"; ratified 2026-07-07.

### `shape_catalogue` - no matrix row

*declared as `ShapeCatalogue`, status `Decided`.*

- **OLD**: Delivery shapes are the Taut catalogue; text_crdt is a profile over crdt.
- **NEW question**: Which delivery shapes does Glade recognise, and what does recognising one mean?
- **NEW stakes**: Settled: the canonical Taut catalogue, with value, atom, log, stream, swmr and crdt as engines and text_crdt a profile over crdt. Recognising a shape does not mean it runs, and an unsupported path must fail before it changes anything.

**Stands on.**

- DecisionLog.md, `Open Decisions`, line 58 (GDL-041): engines are "value, atom, log, stream, swmr, and crdt; snapshot_delta and text_crdt are profiles"; "Recognition MUST NOT imply runtime support, and unsupported paths MUST fail closed before mutation." Ratified 2026-08-28.

### `shaku_injector` - no matrix row

*declared as `ShakuInjector`, status `Decided`.*

- **OLD**: Shaku for dependency injection, at NodeAssembly only.
- **NEW question**: What wires the node's parts together, and where is it allowed to appear?
- **NEW stakes**: Settled: Shaku, used at NodeAssembly and nowhere else, with a regression check that domain components and contracts do not depend on it. Whether it survives a real async port is a separate open question, which async_witness asks.

**Stands on.**

- DecisionLog.md, `Notes`, lines 88-94 (GDL-048): Shaku, "assembly-only library dependency, with a regression witness that domain components/contracts do not depend on it".
- The declaration's own `Requires` edge: `async_witness` requires `shaku_injector`.

### `lifecycle_composition` - matrix row `Q10`

*declared as `LifecycleComposition`, status `Directed`.*

- **OLD**: How async ownership, drain and reverse-order cleanup are composed.
- **NEW question**: What owns a running task: who admits it, who drains it, who cleans up after it?
- **NEW stakes**: The node needs owned work, admission, drain, cancellation and shutdown reports that tell the truth, with a parent resource still owned while a child's cleanup is pending. You directed sdax-rs on 2026-09-12; what is left is the witness and the dependency posture, since sdax-rs is a Git dependency until it is published.

**Alternatives.**

- `sdax_rs`
  - OLD: Owner direction 2026-09-12: sdax-rs (sdax, sdax-tokio, sdax-testkit) at the runtime host.
  - NEW: Your own declarative async lifecycle library at the runtime host, beside Shaku, with sdax, sdax-tokio and sdax-testkit. Stages 0 to 3 have landed with conformance and adversarial review, and it is consumed from Git until it reaches crates.io.
- `tokio_primitives`
  - OLD: JoinSet waves, CancellationToken, TaskTracker and a LIFO teardown stack in-house; fallback.
  - NEW: Compose Tokio's own pieces in-house: JoinSet waves, a cancellation token, a task tracker and a last-in-first-out teardown stack. It was the research lean and is now the fallback; no new dependency, and the ordering rules stay ours to hold.
- `graceful_shutdown_crate`
  - OLD: tokio-graceful-shutdown as the subsystem supervisor; untyped signalling.
  - NEW: Hand supervision to tokio-graceful-shutdown. It supervises subsystems and nothing more, and its signalling is untyped, so what is being shut down is not in the types.
- `dag_runner`
  - OLD: dagx or dagrs as a DAG task runner; no symmetrical teardown.
  - NEW: Run the work as a task graph with dagx or dagrs. Both fit as runners, and neither gives symmetrical teardown, so reverse-order cleanup would still have to be built.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 157 (Q10): "directed to sdax-rs (GDL-049); remaining question is the witness and the dependency posture (Git now, crates.io after publication)".
- GladeBuyBuildMatrix.md, `G. Runtime, composition and engineering`, line 113 (R28): "owned work, admission, drain, reverse-order cleanup, cancellation, truthful shutdown reports"; Tokio primitives "was the research lean, now the fallback"; tokio-graceful-shutdown "untyped signalling; supervisor only"; dagx/dagrs "DAG runners, no symmetrical teardown".
- DecisionLog.md, `Notes`, lines 73-86 (GDL-049): sdax, sdax-tokio, sdax-testkit; "stages 0-3 landed with conformance, Monte Carlo and adversarial review"; "Glade can consume it as a Git dependency now, from crates.io only after publication".
- arch1/RuntimeAndAssurance.md, `Architecture acceptance`, line 126 (AR-08): "Parent resource remains owned while child cleanup is pending; partial shutdown retries without double-release."
- arch1/GladeArchitecture.md, `3. Five consequential design choices`, lines 102-107 (A5): "pure rules and contract crates do not import Tokio".

> **WROTE BEYOND THE SOURCES** - `graceful_shutdown_crate`: "so what is being shut down is not in the types". R28 says "untyped signalling"; the gloss is mine.

### `scope_model` - matrix row `I-2`

*declared as `ScopeModel`, status `Lean`.*

- **OLD**: How dissemination scope is bounded: topology, node trust or cryptography.
- **NEW question**: What stops a record reaching a node that should not see it: wiring, trust, encryption?
- **NEW stakes**: Three enforcement models fit the corpus and the decision is which one the first live slice relies on. The choice sets what a node with no grant can still observe, and each branch opens a different follow-up question.

**Alternatives.**

- `node_trust`
  - OLD: Any accepted node may hold what its operator is granted; policy at serve.
  - NEW: A node receives whatever its operator has been granted, and the serve hop enforces the per-session grants. Simplest to build and needs no new machinery; a shared topic lets a node learn that a share it holds no grant for changed, and what its heads are.
- `topology`
  - OLD: A node connects on a share's protocol only if authorised for that share.
  - NEW: A node may only connect on the protocol carrying a share it is authorised for, rejected at accept time by key and ALPN. An announcement then never reaches an unauthorised node, at one connection per peer per share and churn when grants change.
- `cryptography`
  - OLD: Per-share payload encryption; unauthorised nodes route ciphertext.
  - NEW: Each share's payload is encrypted, so a node without the keys routes and stores ciphertext it cannot read. It is the only model that bounds an over-trusted node, and it costs key distribution and rotation and gives up node-side folds on that tier.

**Stands on.**

- IrohGladeMapping.md, `8. Decisions for the owner`, line 442 (I-2): "Dissemination scope enforcement (S2) | T · N · C | N for the fixed-peer slice".
- IrohGladeMapping.md, `5.2 The undecided S2 decision: how dissemination scope is bounded`, lines 245-281: "Three enforcement models are consistent with the corpus. They are not exclusive; the decision is which one the first live slice relies on." The table gives each model's rule, cost and trade-off, including the node-trust leak of "share ids ... and their heads", topology's "One QUIC connection per peer per ALPN; churn on grant change", and crypto's "Key distribution, rotation, and the forfeit of node-side folds".
- DecisionLog.md, `Open Decisions`, line 28 (GDL-010): metadata exposure is open.
- glade/GladeAuthzModel.md, `10. ggg-viz mapping`, line 366 (INV-5): a node may carry replica state "only when its `hold <share>` entry names its operator".

### `version_pin` - matrix row `Q2`

*declared as `VersionPin`, status `Lean`.*

- **OLD**: Bump iroh from the locked 1.0.2 to 1.2.0 and pin 0.x crates individually.
- **NEW question**: Do we move iroh off the locked 1.0.2 to 1.2.0 now, and pin each 0.x crate by hand?
- **NEW stakes**: The lockfile resolves 1.0.2, and 1.1.0 fixed a crash on a crafted address, which matters the moment addresses are read from untrusted input such as tickets or directory records. The 1.x line promises wire compatibility; the protocol crates are 0.x and outside that promise, so they need pins of their own.

**Alternatives.**

- `bump_to_current`
  - OLD: Take 1.2.0 now; the 1.1.0 fixes cover untrusted EndpointAddr input.
  - NEW: Move to 1.2.0 now, picking up the 1.1.0 fixes for crafted address input, the relay CPU pin and NAT misrouting. The manifest already admits it, so this is the lockfile.
- `stay_on_lock`
  - OLD: Keep 1.0.2 until the first real-route slice.
  - NEW: Stay on 1.0.2 until the first real route is built. Nothing moves today, and the address-parsing fix is not in place when untrusted addresses first arrive.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 149 (Q2): "Bump iroh to 1.2.0 now and pin protocol crates individually?"; the lean column reads "Yes; nothing is parsed from untrusted addresses safely on 1.0.2".
- glade/dev-docs/IrohReview.md, `1. Summary`, lines 18-43: "1.0 commits to wire compatibility across all 1.x minors"; "Protocol crates are 0.x and outside the 1.0 promise".
- glade/dev-docs/IrohReview.md, `11. Observations for glade`, lines 570-598: "1.1.0 (2026-09-01) fixed a deserialization panic on crafted `EndpointAddr` `CustomAddr` variants, a relay batch-message CPU pin, and NAT-probe misrouting"; "the `EndpointAddr` one matters as soon as addresses are parsed from untrusted input (tickets, directory records)"; "`iroh = \"1\"` already admits 1.2.0".

### `transport_key_binding` - matrix row `Q3`

*declared as `TransportKeyBinding`, status `Lean`.*

- **OLD**: Bind the iroh endpoint key to the Glade node principal.
- **NEW question**: How do we say that this iroh endpoint key belongs to that Glade node?
- **NEW stakes**: The directory names a node by a hash of its node key and iroh authenticates the endpoint key, and nothing replicated joins the two. Address lookup, gossip peer identity and every accept-time policy need that join, and it is a record kind and a fold rule rather than anything iroh provides.

**Alternatives.**

- `binding_record`
  - OLD: A NodeTransportBinding record under the node chain; keys rotate independently.
  - NEW: A signed record under the node's own chain says which endpoint key is its transport. The two keys stay separate and rotate independently, and it is a new record kind with a fold rule to write.
- `one_key_both_roles`
  - OLD: The iroh EndpointId is the node id; the identity split is relaxed.
  - NEW: Use the iroh endpoint id as the Glade node id and drop the split. Nothing new has to be replicated, and transport key and node identity can no longer rotate or be reasoned about apart.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 150 (Q3): "Transport-key binding: a new record kind under the node chain, or one key for both transport and node identity?"; the lean column reads "Record kind; keep the identity split".
- IrohGladeMapping.md, `5.2`, lines 338-345: "The directory identifies nodes by `sha256(node.key)`; iroh authenticates the endpoint key. Nothing replicated binds the two." Address lookup, gossip peer identity, blobs provider authorisation and accept-time policy all need it; "This is a record kind and a fold rule, not an iroh feature."
- glade/dev-docs/GladeDirectoryNotes.md, `Ambiguities and smallest-reasonable resolutions`, lines 65-71 (ambiguity 2): one identity, two renderings; "the iroh key stays transport-only".

> **WROTE BEYOND THE SOURCES** - `one_key_both_roles`: "transport key and node identity can no longer rotate or be reasoned about apart". The directory notes keep the iroh key transport-only; what collapsing the two costs is my inference.

### `dissemination` - matrix row `Q4`

*declared as `Dissemination`, status `Lean`.*

- **OLD**: How records reach trusted nodes beyond the connect-time pull.
- **NEW question**: Once a record is written, how does it reach the other trusted nodes?
- **NEW stakes**: The connect-time sync round already moves what a peer is missing; the question is whether anything announces a change between rounds. An overlay makes a change visible sooner and adds a 0.x dependency outside iroh's wire promise, so it waits on the scope model that says who may hear the announcement at all.

**Alternatives.**

- `sync_round_only`
  - OLD: The Glade sync round and best-effort push; no overlay.
  - NEW: Keep only what exists: one pull each way plus a best-effort push to the home node. Nothing new is taken on, and a change is noticed at the next round rather than when it happens.
- `gossip_overlay`
  - OLD: iroh-gossip topics per share as change notification; pinned 0.x.
  - NEW: Announce changes on an iroh-gossip topic per share, keeping the sync round as the repair path and the only thing that moves canonical bytes. Gossip is 0.x and outside the 1.0 wire promise, so it must be pinned and kept behind the carrier port.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 151 (Q4): "keep the Glade sync round only, or adopt `iroh-gossip` behind the carrier port once the scope model ... is chosen?"
- GladeBuyBuildMatrix.md, `B. Shared knowledge and discovery`, line 68 (R8): the sync round is "built, one pull each way plus best-effort home push".
- IrohGladeMapping.md, `4.2 Shape B`, lines 136-175: gossip announces, "keep Glade's sync round ... as the repair path and the only path that moves canonical bytes"; "`iroh-gossip` is 0.x and outside the 1.0 wire promise ... Pin it; keep it behind the carrier port".
- glade/GladeDiscoveryDesign.md, `10. FROZEN - invariants`, lines 392-395 (INV-D5, INV-D6): gossip only after persist; convergence under loss, reorder and duplication.

> **WROTE BEYOND THE SOURCES** - `sync_round_only`: "a change is noticed at the next round rather than when it happens". R8 describes the round; the latency framing is mine.

### `identity_adapters` - matrix row `Q7`

*declared as `IdentityAdapters`, status `Lean`.*

- **OLD**: Which enterprise identity adapters, if any, ship in v1.
- **NEW question**: Does v1 talk to an existing company directory, or only to grants made by hand?
- **NEW stakes**: The trust-provider interface exists either way, and it answers only "is this assertion real?"; deciding what a principal may do stays with the capability model. The question is whether any adapter ships in v1, because each one is a foreign identity source to keep working. It waits on what a capability proof looks like.

**Alternatives.**

- `none_in_v1`
  - OLD: Device certs and manual grants only; keep the trust-provider interface.
  - NEW: Ship with device certificates and grants made by hand, keeping the trust-provider interface declared and unimplemented. Nothing foreign has to be kept working, and anyone with an existing directory wires it up themselves.
- `oidc`
  - OLD: OIDC/OAuth2 as edge authentication input.
  - NEW: Accept an OIDC or OAuth2 token as an authentication input at the edge. It is the proof kind most organisations already issue, and it makes an outside issuer part of who gets in.
- `kerberos_ldap`
  - OLD: Kerberos, AD or LDAP facts through an adapter.
  - NEW: Read facts from Kerberos, Active Directory or LDAP through an adapter. It reaches the older corporate estate, and it is another adapter kept current with someone else's schema.
- `spiffe`
  - OLD: SPIFFE/SPIRE workload identities for nodes and providers.
  - NEW: Take workload identities from SPIFFE or SPIRE for nodes and providers. It fits a fleet that already issues them, and it assumes that fleet exists.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 154 (Q7): "Which enterprise identity adapters, if any, in v1 (AZ-3)?"; the lean column reads "None in v1; keep the trust-provider interface".
- glade/GladeAuthzModel.md, `9. Open questions`, line 332 (AZ-3): "OIDC/AD bridge in v1, or single-user + manual grants first? (the trust-plug interface exists either way)".
- glade/glade_sec_model-55/GladeGrythSecurityModelAnalysis.md, `Trust And Capability Plug Interfaces`, lines 244-245: "Trust providers produce facts. Capability providers/evaluators produce decisions." The `TrustProofKind` list at lines 255-258 carries `oidc_jwt`, `kerberos_ap_req`, `ldap_bind` and `spiffe_svid`.

> **WROTE BEYOND THE SOURCES** - `identity_adapters`: "each one is a foreign identity source to keep working". AZ-3 asks the question without costing an adapter.

> **WROTE BEYOND THE SOURCES** - `oidc`: "It is the proof kind most organisations already issue". SEC-55 lists `oidc_jwt` as one proof kind among seven; "most" is mine.

> **WROTE BEYOND THE SOURCES** - `kerberos_ldap`: "It reaches the older corporate estate". SEC-55 lists `kerberos_ap_req` and `ldap_bind`; calling that estate older is mine.

> **WROTE BEYOND THE SOURCES** - `spiffe`: "It fits a fleet that already issues them, and it assumes that fleet exists". SEC-55 lists `spiffe_svid`; the fit and the assumption are mine.

### `store_engine` - matrix row `Q8`

*declared as `StoreEngine`, status `Lean`.*

- **OLD**: When the whole-state blob gives way to an engine, and which.
- **NEW question**: When does the whole-state blob stop being good enough, and what takes over?
- **NEW stakes**: The seam is already fixed: StoreApi holds a blob now, a real engine may sit behind it later, and an engine is never the replication mechanism. What is open is the measured point at which the blob hurts, and which engine is worth an evaluation then.

**Alternatives.**

- `whole_state_blob`
  - OLD: Keep the taut blob until size or query volume hurts.
  - NEW: Keep writing the whole state as one taut blob. Nothing changes and nothing is added, until size or query volume makes rewriting it hurt.
- `sqlite_engine`
  - OLD: SQLite behind StoreApi; never the replication mechanism.
  - NEW: Put SQLite behind StoreApi when the blob stops paying. It is the successor the seam ruling already names, and it stays a store engine: replication stays in the ops.
- `redb_engine`
  - OLD: redb behind StoreApi; not evaluated for Glade.
  - NEW: Put redb behind the same seam instead: pure Rust, and already used by iroh's own crates. It has not been evaluated for Glade, and n0 note friction running an embedded database inside async, so taking it means doing that evaluation first.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 155 (Q8): "when does SQLite replace the whole-state blob, and is redb worth an evaluation?"; the lean column reads "Blob until query volume or size hurts; evaluate redb only with a measured need".
- GladeBuyBuildMatrix.md, `E. Persistence`, line 94 (R19): "redb (pure Rust, used by iroh-blobs/docs): fits with issues (not evaluated for Glade; embedded-DB-in-async friction noted by n0)".
- DecisionLog.md, `Open Decisions`, line 54 (GDL-036): the seam itself.
- GladePersistenceReview.md, `9. Claims I could not verify / refuted`, lines 578-580: the seam "is honest and well-built"; §4.1 lines 272-276 record the blob rewrite cost the gate is about.

### `text_engine` - matrix row `Q9`

*declared as `TextEngine`, status `Lean`.*

- **OLD**: Whether a second text CRDT engine joins the own profile.
- **NEW question**: Does collaborative text stay on our own engine, or does a second one come in beside it?
- **NEW stakes**: text_crdt is already a profile over the own crdt engine and it ships. A second engine could only enter as another profile with conformance rows of its own, and it brings its own wire format and its own browser bundle with it.

**Alternatives.**

- `own_text_profile`
  - OLD: text_crdt.profile/v1 over the own crdt engine; shipped.
  - NEW: Keep text_crdt.profile/v1 over the own crdt engine, which is already shipped. No new engine and no new wire format, and whatever it cannot do stays undone.
- `yrs`
  - OLD: yrs/Yjs family; documented cross-language wire intent; small browser bundle.
  - NEW: Admit the yrs/Yjs family as a second profile engine. Its binary compatibility with Yjs is documented and its browser bundle is the smallest of the three; the Python leg through pycrdt is stated as intent rather than guaranteed.
- `loro`
  - OLD: Loro 1.x; Fugue; no Python parity.
  - NEW: Admit Loro 1.x, which orders text with Fugue and benchmarks well on large replays. There is no Python parity, so the three-language corpus could not be kept whole.
- `automerge`
  - OLD: Automerge 3.x; large browser bundle.
  - NEW: Admit Automerge 3.x, a Rust core reached from JavaScript through Wasm. Its browser bundle is by far the largest of the three, which every browser fold would carry.

**Stands on.**

- GladeBuyBuildMatrix.md, `4. The buy/build questions to answer`, line 156 (Q9): "keep the own `text_crdt` profile, or admit yrs/Yjs or Loro as a second profile engine?"; the lean column reads "Keep the own profile; admit a second engine only through a profile with its own conformance rows".
- DecisionLog.md, `Open Decisions`, line 58 (GDL-041): `text_crdt` is a profile over crdt.
- GLResearchConsolidatedFindings.md, `CRDT / local-first data layer`, lines 120-144: "yrs (Rust) documents binary-protocol compatibility with Yjs (JS)", while the pycrdt leg is "'aims to maintain' = best-effort intent, not a guarantee"; bundle weights "Yjs 69 KB / ywasm 678 KB / Automerge 1.74 MB"; "Loro is a genuine text/sequence CRDT (Fugue) ... B4 benchmark replays 259,778 ops"; "Automerge: current (v3.x, 2026), Rust core via FFI to JS+Wasm+C".

> **WROTE BEYOND THE SOURCES** - `loro`: "There is no Python parity". This is the v1 declaration's own claim, kept; GLResearch says only that pycrdt is a yrs binding, which does not by itself establish Loro's gap.

### `metadata_exposure` - matrix row `R8`

*declared as `MetadataExposure`, status `Open`.*

- **OLD**: Which ids, heads and topic identifiers a non-granted trusted node or relay may observe.
- **NEW question**: What can a node or relay learn about a share it holds no grant for?
- **NEW stakes**: Which envelope fields, workspace ids, share ids, provider claims and topic identifiers may be visible to a relay-only peer is an open ruling, and the security analysis wants a table saying exactly what each scope model leaks. Content does not flow without a grant either way; this is about what is learned from the outside.

**Stands on.**

- GladeBuyBuildMatrix.md, `B. Shared knowledge and discovery`, line 68 (R8): the dissemination-scope model is an open item.
- DecisionLog.md, `Open Decisions`, line 28 (GDL-010): "Which declaration-envelope fields, workspace ids, share ids, provider claims, and transport topics are allowed to be visible to relay-only peers?"
- glade/GladeWorkspaceDirectory.md, `8. Open questions`, line 269 (WD-3): "Metadata exposure to relay-only peers (GDL-010)".
- IrohGladeMapping.md, `5.2`, lines 238-242: the security analysis "should list, per model, exactly which of `TopicId`, share id, origin ids and head hashes a non-granted node observes"; "content does not flow without a grant".

## The five triggers

A trigger is an event, not a question, so it still opens with the event.

### `first_bulk_supplier`

- **OLD**: A supplier declares a bulk profile (files, snapshots, large results).
- **NEW event**: A supplier declares a bulk profile: files, snapshots or large results.
- **NEW why it gates**: Until one does, a bulk path would be built for nobody, and the admission wrapper it needs would have no real request to be shaped against.

**Stands on.**

- GladeBuyBuildMatrix.md, line 152 (Q5): "Defer; adopt blobs behind a port on the first bulk supplier".
- IrohGladeMapping.md, `4.3`, lines 202-214: "Adopt only when a supplier slice needs it".

### `store_pressure`

- **OLD**: Measured size or query volume the whole-state blob cannot carry.
- **NEW event**: A measured size or query volume the whole-state blob cannot carry.
- **NEW why it gates**: The measurement is what turns the store-engine question from speculation into work, and it is also what says which engine would have to be evaluated.

**Stands on.**

- GladeBuyBuildMatrix.md, line 155 (Q8): "Blob until query volume or size hurts; evaluate redb only with a measured need".

### `editing_need`

- **OLD**: An editing requirement the own text profile cannot meet.
- **NEW event**: An editing requirement the own text profile cannot meet.
- **NEW why it gates**: Without one, a second text engine is conformance work with nothing asking for it.

**Stands on.**

- GladeBuyBuildMatrix.md, line 156 (Q9): a second engine enters "only through a profile with its own conformance rows".

### `first_real_route_slice`

- **OLD**: The fixed-peer iroh route slice from GladeBuildEntry passes.
- **NEW event**: The fixed-peer iroh route slice from GladeBuildEntry passes.
- **NEW why it gates**: It is the first time a real route exists, so it is the first time there is something to measure, simulate or harden against.

**Stands on.**

- GladeBuyBuildMatrix.md, line 160 (Q13): "Not before the first real-route slice".
- GladeBuildEntry.md names the fixed-peer route slice this trigger is about.

### `failure_model`

- **OLD**: A written registry failure and membership model.
- **NEW event**: A written registry failure and membership model exists.
- **NEW why it gates**: Until it does, there is nothing to say which part of registry growth would ever need ordered coordination, so any answer would be a guess about a shape nobody has drawn.

**Stands on.**

- GladeBuyBuildMatrix.md, line 159 (Q12): "Trusted mapping only now; reopen with a failure model".

## What I could not stand on anything

Some cited tags resolve to no passage at all in the source index, and the wording above leans on the matrix row or on the declaration's own edges instead. They are `DependencyInjectionEvaluation` and `GladePersistenceReview` and `GLResearchConsolidatedFindings` (bare document names, which the index cannot resolve to a row or a heading), `AZ §2-§6`, `AZ §2`, `AZ §7a`, `GladeDiscoveryModel §5`, `GladeDirectoryNotes ambiguity 2` and `SEC-55 trust providers` (free-text locators), and `B4`, `B5`, `GAD-04`, `GAD-05`, `GPI-03`, `GLA-072`, `INV-5`, `INV-D5`, `INV-D6`, `AR-08` and `A5` (row ids no document in the index declares). I found every one of them by hand and quoted them above, so the wording stands on real text; the source index still reports them unresolved, which is a separate thing to fix if you want it fixed.

## Revision v3, 2026-09-21: `version_pin`

One correction, made after everything above was written. It is a correction of a PREMISE
rather than of prose, so the entry below reads the revision `v2` text as its OLD. Nothing
else on this page moved, and no other question was touched.

### `version_pin` - matrix row `Q2`

*declared as `VersionPin`, status `Lean`. Status, matrix row, sources, `Requires`, `Offers`
and the recorded lean are all untouched; only the three docstrings moved.*

- **OLD question**: Do we move iroh off the locked 1.0.2 to 1.2.0 now, and pin each 0.x crate by hand?
- **NEW question**: Do we require iroh 1.2 or later, and pin each 0.x crate exactly when we add one?
- **OLD stakes**: The lockfile resolves 1.0.2, and 1.1.0 fixed a crash on a crafted address, which matters the moment addresses are read from untrusted input such as tickets or directory records. The 1.x line promises wire compatibility; the protocol crates are 0.x and outside that promise, so they need pins of their own.
- **NEW stakes**: The node's lockfile is not tracked, so nothing in the repository pins iroh: a fresh clone resolves the newest 1.x, and an old checkout keeps whatever it last resolved (this one kept 1.0.2 until 2026-09-21). 1.1.0 fixed a crash on a crafted address, which matters the moment addresses are read from untrusted input such as tickets or directory records. The manifest is therefore the only place a minimum can be recorded. The 1.x line promises wire compatibility; the protocol crates are 0.x and outside that promise, so each needs an exact pin of its own when it is first added.

**Alternatives.**

- `bump_to_current`
  - OLD: Move to 1.2.0 now, picking up the 1.1.0 fixes for crafted address input, the relay CPU pin and NAT misrouting. The manifest already admits it, so this is the lockfile.
  - NEW: Require 1.2 or later in the manifest now, so no checkout can build a release that still has the crafted-address crash, the relay CPU pin or NAT misrouting. One tracked line; the node's tests pass on 1.2.0.
- `stay_on_lock`
  - OLD: Stay on 1.0.2 until the first real route is built. Nothing moves today, and the address-parsing fix is not in place when untrusted addresses first arrive.
  - NEW: Leave the requirement at any 1.x until the first real route is built. Nothing to maintain today, and nothing stops a stale checkout from building a release with the address-parsing crash when untrusted addresses first arrive.

**Why it changed.**

The old sentence called `node/Cargo.lock` the thing that pins iroh, and it is git-ignored. So the repository pinned nothing at all: the 1.0.2 this page, IrohReview §1 and IrohReview §11 all read as what Glade was on was one checkout's stale resolution, a fresh clone would have resolved the newest 1.x, and "move off the lock" was a choice nobody could have recorded anywhere a reader could see. The manifest is the only place a minimum can be written down, so that is what the question now asks about: whether we require 1.2 or later there, and whether each 0.x protocol crate gets an exact pin of its own as it is added. The owner raised that floor on 2026-09-21, `iroh = "1.2"` in `node/Cargo.toml` at glade `74ffeb0`, and the two alternatives are now leaving the floor at any 1.x or requiring 1.2 or later, which is what the recorded lean already pointed at.

**Stands on.**

- glade/node/.gitignore: `/Cargo.lock`, so the node's lockfile is not a tracked file and nothing committed to the repository resolves a version.
- glade `74ffeb0`, "Require iroh 1.2 or later in the node": `iroh = "1.2"` in `node/Cargo.toml`, built and tested against iroh 1.2.0, iroh-dns 1.3.0 and noq 1.3.0, with the node's 61 tests passing, including the dial, hello, sync, exchange and mesh tests that run over real iroh.
- glade/dev-docs/IrohReview.md, `11. Observations for glade`, the dated update under **Version gap**: the review's own record that the 1.0.2 above it was a stale resolution and not a pin. The review is verified against 2026-09-12 and is otherwise left as written.

**What it is published as.** The base declaration is lineage `glade-decision-graph` revision
`v3`, snapshot digest
`443ab91bb03c3b46bc2f065ec65f883a5929a2943d0afed9a47fecc8081a8475`. The rebuilt bundle is
`artifacts/decision-streams-v10` and `GladeDecisionIndex.md` is regenerated from it;
`artifacts/decision-streams-v9` holds revision `v2` and stays exactly as it was.
