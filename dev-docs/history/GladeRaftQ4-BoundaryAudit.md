The production integration boundary is broader than `accept_ops`: protected acceptance must replace or fence every route that can alter the protected application history, while preserving canonical Glade addressing, operation bytes, declaration, authenticated requester, policy frontier and recoverable outcomes. The current node has useful signing, storage, transport and assembly seams, but no production Raft profile, signed group genesis, complete application recovery contract or legacy-writer fence.

This was read-only. No source/report writes, Git commands, builds or tests were performed. References below are to **live filesystem sources**, not an independently verified checkout of the supplied tuple: root `55b6ee30bb18fa58c80805be2d29a44b404077f6`, glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Live cold-join sources are identified explicitly below; a pinned/live diff was not supplied.

### Controlling contracts and allocation

- `dev-docs/GladeBuildEntry.md`, opening and “Recommended first working slice”: architecture is declared in Gyld before implementation; new/recomposed ports need compiling consumer/conformance specifications and adversarial review. Existing demo/uncommitted work is preserved; adapters begin alongside it.
- `dev-docs/LibraryBoundaryAndTestingPolicy.md` §§1–4: reviewed classifications, meaningful implementation contracts, minimal dependencies, deterministic pure tests, actual adapter conformance, measured fast commands, affected-consumer checks and explicit exceptions. A proof harness does not become a production dependency merely by reclassification.
- `dev-docs/GladePackageArchitecture.md` §§2–6: node is the composition root; existing discovery host ports are not presumed extracted; pure libraries retain deterministic boundaries; routing does not grant source authority; durable exact signed-operation acceptance precedes discovery gossip.
- `dev-docs/GladeRaftAdoptionContract.md:12–25`: M3-D orders actual protected commands and complete authoritative data/outcomes. Canonical scope includes share and declared binding/zone, including authenticated private-principal derivation. Equal labels do not identify equal independent roots.
- `dev-docs/GladeRaftAdoptionContract.md:42–53`, RA-001–012: signed bound genesis; separate leadership/home/generation; complete data-bearing acceptance; principal-scoped exact retry; ordered eligibility and cuts; creation-rooted policy/evidence; projection write exclusion; external-effect exclusion; retirement; authorized membership; coherent recovery; complete legacy-writer exclusion and preserved baseline fence.
- `dev-docs/GladeRaftAdoptionContract.md:83–96`: Binding owns canonical scope/declaration; Admission/Policy own identity/evidence validation; Directory remains routing; Records is proposed host for M3-D history/outcomes/recovery; Supplier/Application retains source/effect fencing; Delivery consumes application frontiers; NodeAssembly selects providers.
- External `gyld/examples/glade-architecture.gyld.py:161–181`: AdmissionPort and RecordHost/ProfilePort are **proposed** boundaries; serialized state ownership and profile-defined canonical identity/atomic commit unit already belong to the architecture.
- Same Gyld declaration `:339–362`: Records owns accepted history, discovery state and durable handoff through injected ports. Its current “Source commits stay outside” at `:343` needs the scoped M3-D qualification, not wholesale reinterpretation.
- Same declaration `:365–403`, `:504–510`, `:558–608`: Delivery, Invocation, SupplierHost, ApplicationSupplier and NodeAssembly allocations. Provider attachment epoch alone does not fence physical source writes.
- `dev-docs/GladeRaftQ3ImplementationAllocation.md` §§1–2: existing Q3 API/session/disk allocation is expressly qualification work. Scope `7`, group `70`, profile `2`, principals `[1,2]` are trusted numeric fixtures, not cryptographic production evidence. The accepted learner clarification permits private incomplete nonserving catch-up only after validating a committed bound join.

### Canonical identity, scopes and the actual settings case

| Fact | Exact source |
|---|---|
| Domain is the replicated world; zone is the converging partition; surface is the declared typed binding. | `glade/dev-docs/GladeZones.md:25–45` |
| Wire address is `(share, glade_id, key)`; chain is `(share, glade_id, key, origin)`; journal is per `(share, origin)`. | `GladeZones.md:80–96`; `glade/node/src/store.rs:3–11,134–140` |
| Op includes address, origin, seq, predecessor hash, Lamport time, causal refs, shape and complete opaque payload. | `taut/ir/glade.taut.py:88–106` |
| Exact canonical op hash is the existing operation correlation; it is not a durable requester transaction ID. | `glade/node/src/chain.rs:1–13`; `glade/node/src/session.rs:84–94` |
| Stable binding identities are declared/runtime-neutral; key schemas are canonical and versioned. | `glade/dev-docs/GladeSubstrateV1.md:24–32,126–131,140–168` |
| Binding request preserves principal, exact declaration/version, domain instance and canonical parameter bytes. Ingress must establish principal. Bound result is descriptive and requires operation reauthorization. | `glade/contracts/binding-api/src/lib.rs:5–45` |
| Private `self` must derive from authenticated B3 principal, rejecting substituted literal identities on subscribe, append, replay and forwarding. | `dev-docs/glade/GladeAuthzModel.md:169–190,345` |
| Account owner access is by identity; nonowner access remains grant-gated. Membership gates private zones within another resource. | `GladeAuthzModel.md:184–190,345–346` |

The settings reproduction must not be reduced to an arbitrary numeric resource:

- `gryth-wz/dev-docs/GrythGripScopes.md:14–17,40–51`: appearance follows a user across every session/browser/node; a durable desk session has shared geometry; page instance is ephemeral.
- Same document `:119–157`: scope specification is `(principal, entry, session)`; tab origin identifies the **op chain only**, not user/session scope. Appearance is `<entry>.appearance`; desk is `<entry>.desk`.
- Same document `:232`: current appearance is a complete `{v, theme, zoom, fontScale, wallpaper, wallpaperThemed}` value, `gyld.appearance` on `ws-razel`, private key `self:<principal>`, stored in IndexedDB.
- Same document `:127–130`: current principal comes from URL/bootstrap/tab fallback. This is implementation context, not authenticated-principal proof.
- `glade/demo/src/manifest.ts:26–57,65–92`: typed account/document bindings, identity/session templates and a deliberately labelled URL-minted **stub grant**.
- `glade/grip-share/src/manifest.ts:25–46,58–67`: manifest distinguishes identity from session parameters, but implementation combines supplied maps and substitutes templates. It is not the authenticated node-side B4 resolver.

A production identity must bind these actual address/declaration/parameter bytes to the creation root/incarnation and group mapping. Neither `ws-razel`, a node’s local directory path, a display name nor Raft term establishes that binding.

### Real signing, current authentication and authority gaps

**Existing real cryptography**

- `glade/node/src/signing.rs:23–36,46–88`: genuine Ed25519, distinct purpose domains, `verify_strict`, public key as node ID.
- `glade/node/src/peer.rs:111–184`: peer HELLO signs protocol/role/node/endpoints/TLS exporter; verifies possession for this transport connection.
- `peer.rs:187–197`: optional Door checks node-to-endpoint binding against authenticated records/configuration.
- `glade/node/src/envelope.rs:1–8,70–90,213–243`: home records carry canonical SignedRecord envelopes; signature covers op chain position and full record; structural/canonical kind, origin signature and checkpoints verify.
- `glade/node/src/registry.rs:503–521`: sealed registry ingests through this verification path.
- App ops are expressly unsigned in `glade/node/src/peer.rs:446–453` and `glade/dev-docs/GladeCrossNodeWritesPlan.md:663–665`.

**Current dev authentication is insufficient for RA-004/006**

- `glade/node/src/server.rs:321–347`: client Hello principal is taken on the client’s word; unknown principal becomes a minimal record; only node-shaped principal strings are excluded.
- `server.rs:64–79,95–108,119–127`: session principal table is connection-local; client-grant enforcement defaults off; enabling it checks the supplied principal.
- `glade/node/src/accept.rs:219–241`: forwarding checks authenticated **forwarding node** `write.append`; local checked client checks its named principal. End requester does not reach the holder.
- `glade/node/src/mesh/serve.rs:153–172`: forwarded writes receive `Source::Forward(zone, node)`, not authenticated original requester/evidence.
- `glade/node/src/server.rs:349–409`: subscribe uses caller-supplied key bytes; share grant check does not derive authenticated `self`.
- `glade/node/ir/sysdata.taut.py:124–131`: PrincipalRecord is deliberately stage-1 identity data, not enroll/attenuate/revoke lifecycle.
- `glade/node/src/envelope.rs:213–243`: valid origin signature proves the node signed the home record; it does not validate resource-root issuance ancestry.
- `glade/node/src/registry.rs:475–494,632–637`: grant policy folds held grants/revocations; current schema lacks the full root/parent/caveat chain required by Authz.
- `glade/node/src/grants.rs:60–91,105–159`: pure union/revocation-wins policy and current generation-sensitive view are useful components, but they are not the complete creation-rooted governance frontier.

Canonical authority remains `GladeAuthzModel.md:76–110` (creation root, ancestry and declared governance), `:152–167` (policy travels with data), `:245–270` (operator acceptance/replica custody), `:291–305` (key-signed versus operator-vouched authentication strength). Eligibility to host/vote cannot replace requester verbs.

### Actual writers and store paths the exclusion fence must cover

| Path | Current implementation and integration consequence |
|---|---|
| Local client/provider `Ops` | `server.rs` Ops dispatch enters `accept_ops`; `accept.rs:82–136` performs home/stream/grant/zone checks and decides placement. `accept.rs:144–186` directly appends `Store`, then queues fan-out/status. Protected profile needs ordered application here, not a Raft preflight followed by this append. |
| Unclaimed and mesh-less fallback | `mesh/route.rs:40–67`: no mesh always Local; home Local; unknown share Local; known no-live-claim absent. `accept.rs:195–216` adopts this route. A protected identity must remain fenced even when discovery is missing or disabled. |
| Forwarded write admission at holder | `mesh/serve.rs:94–106,153–172` calls the same acceptance path with node identity and zone. Must preserve authenticated original requester/retry/evidence in the new profile. |
| Forward reply landing at B | `mesh/route.rs:309–316`: holder `Ok` triggers direct `accept::place`, and B relays success only if its local placement succeeds. Must become verified committed projection/outcome landing, not another authoritative admission. |
| Live forwarded replica data | `mesh/route.rs:268–275` calls `ingest_and_fanout` on matching zone; `mesh/home.rs:328–350` directly calls `Store::append`. This function is shared by application forwarding and home publication, so fencing only client acceptance leaves this write route. |
| Home peer gossip/pull | `mesh/serve.rs:42–65` accepts only HOME pushed Ops through Round; `mesh/home.rs:52–86,89–149` pulls HOME and validates known origins before shared ingest/fan-out. Profile-governance evidence may depend on this, but receiving gossip cannot confer protected write authority or mutate its ordered state independently. |
| Generic public peer-sync API | `peer.rs:407–443,454–498` exposes serve/pull sync; pull directly appends each op. Current production mesh uses home-specific pull; generic sync references are in carrier tests. Nevertheless it is a callable Store writer and must be excluded from protected authoritative admission. |
| Registry seed | `server.rs:130–143`: boot snapshot ops directly append into the served Store. Must not seed protected accepted history from an unverified partial legacy snapshot. |
| Assembly’s record provider | `assembly/ports.rs:24–42`; `assembly/providers.rs:169–206`: Records append/register/ingest stages Registry and saves it, separately from application Store. New profile must not overload home-only directory methods as protected application command semantics. |
| Declaration registration/retraction | `appdecl.rs:757–838`; Records register above; live startup loads/registers apps under node origin. Governance/declaration changes affecting protected eligibility need the defined ordered frontier, with existing unrelated declarations remaining intact. |
| Workspace create/configured claim | `claims.rs:298–375`; `exchange.rs:287–335`: WorkspaceEntry + ServeClaim mint under directory node origin through local durable Registry acceptance. This remains a directory ceremony, not RA-001 signed group genesis or RA-002 ordered protected Create. |
| Principals, renewals, checkpoint publication | `claims.rs:384–404,464–515`; `checkpoint.rs:213–224`: ordinary signed home records, then fan-out/gossip. Lease renewal/checkpoint must not advance protected generation or erase required retry/governance evidence. |
| Boot/recovery identity | `sysdir.rs:304–365`: creates cache/lock, creates missing keys, loads snapshot, and absent self NodeRecord causes first-boot presence/home claim. This is node bootstrap, not authority to create/reset an existing protected scope. |
| External provider/source writes | `exchange.rs:96–105,193–250` attaches provider by declared exchange and routes calls; supplier writes can reenter ordinary Ops. ApplicationSupplier/source mutations, filesystem/build/Git/shell effects require separate exclusion or M3-E sink qualification. An attachment/session epoch is insufficient. |

Lower-level Store is independently writable: `store.rs:185–268` opens/replays; `:291–334` appends; `:696–701` uses `write_all` without fsync. Non-home open loads decoded ops directly at `:230–234`. Its per-chain classifier and shape constraints do not enforce protected root/group/generation/principal/policy.

Registry uses a separate transaction path: `registry.rs:728–750` stages, saves snapshot, then replaces fold; `records_file.rs:141–178` compares revision under lock, writes and syncs temp, renames and syncs directory, with explicit OutcomeUnknown after rename. That is local snapshot durability, not coherent Raft hard-state/log/application publication or quorum acceptance.

### Live dirty cold-join correction and limits

These references describe **live source**, including the existing work to preserve:

- `claims.rs:273–278,307–345,538–556`: configured join includes expired remote claims and follows a known remote owner without minting an entry/claim; self restart advances existing epoch; this is expressly not a quorum election or proof of application completeness.
- `GladeCrossNodeWritesPlan.md:669–730`: the settings regression involved both app declarations naming `ws-razel`; startup previously claimed max+1 without acquiring existing application data.
- Same document `:706–710`: no repair of divergent replicas, completeness from nonempty cache, transfer/election, or safety against mutually unknown first boots; follower ack remains local replica frontier.
- Same document `:721–724`: Gyld launcher is an affected consumer and accepts explicit following readiness, rather than requiring every node to claim `ws-razel`.
- `lifecycle.rs:290–326,685` and `bin/glade-node.rs:482`: both composition roots participate in workspace startup; new production activation cannot modify one root and leave the other permissive.
- `claims.rs:730–759` and `node/tests/cross_node_writes.rs`: existing cold-join regression coverage is valuable routing evidence, not RA-012 baseline completeness evidence.

### Receipts, unknown outcomes and wire compatibility

- Existing `ErrorCode` values and frame tags are frozen/additive: `taut/ir/glade.taut.py:49–69,194–201`.
- Existing Op has **no** group/root/incarnation/generation/durable request ID/authenticated requester/evidence frontier/terminal outcome fields: `glade.taut.py:96–106`.
- Client status is `Error{share,glade_id,corr=canonical op hash,code,message}`: `session.rs:84–94`.
- Local Ok currently means appended/byte-identical held and fan-out queued: `accept.rs:139–186`; append lacks fsync as above.
- Forward Ok additionally depends on B holding A’s accepted op: `mesh/route.rs:309–316`. It is not a durable data-bearing Raft/application receipt.
- Subscribe ack names local per-origin heads/hash and observed cut: `session.rs:47–53`; `server.rs:384–419`; peer ack/gap has the same queued cut at `mesh/serve.rs:87–140`. It is not group-wide committed/applied completeness or a linearizable barrier.
- TS local Session builds/stores/folds pending ops before server acceptance: `client-ts/src/session.ts:34–60,82–107`. Client carries binder-made ops via `client-ts/src/client.ts:258–285`.
- TS refusal drops owned session chain tail, while binder-owned session is only notified; UnknownShare keeps/resends: `client.ts:288–303,415–438`. Rust mirrors refusal/unplaced reporting/resumption in `client-rs/src/client.rs:226–272`.
- Directed exchanges have connection-local reminted correlations and forget pending calls at end: `exchange.rs:108–178,182–190,241–283`. Timeout/provider departure is currently `ok:false`; those bytes cannot silently acquire RA-004 “known not committed” semantics.
- Existing draft `Invoker` already requires Unknown after possible dispatch and preserved authenticated context: `contracts/invocation-api/src/lib.rs:39–54`.
- Existing draft `ReplicaSync` guarantees bounded atomic **local** ingestion; cursor is local/profile scoped, no source authority or global frontier: `contracts/sync-api/src/lib.rs:31–53`.
- Existing SnapshotStore explicitly excludes request dedup, machine-loss or peer guarantees: `contracts/persistence-api/src/lib.rs:26–48`.

Therefore Q4 needs a separately versioned command/outcome/recovery mapping with explicit profile negotiation. It must retain canonical original Op/declaration/payload bytes and distinguish pending local intent, durable ordered acceptance, projection receipt, read frontier and Unknown. Existing Ok/hash/Heads cannot be silently reinterpreted.

### Minimal additive boundary and first useful slice — inference from the facts above

A reviewable minimal allocation is:

1. A small production command/history contract and protocol representation using actual canonical Glade values. It owns meaningful submit/recover/read-frontier operations and complete typed outcomes, plus narrow injected durable publication/message boundaries. No node, Shaku, Iroh, filesystem or proof-harness types cross it.
2. Deterministic protected application rules, classified pure, owning names/resources/incarnations/generations/homes, complete payload/history, retry outcomes, governance frontier and retirement. Explicit evidence/time are inputs; no live PolicyView/discovery/clock calls during apply.
3. A production Records host/provider implementing that contract with the selected qualified Raft carrier and injected store/message providers. Qualification code may be reused only through a reviewed extraction/translation with production conformance; importing `adoption-proof` or numeric proof APIs does not establish this boundary.
4. Node boundary translation/admission, actual authentication/signing and store/transport adapters assembled alongside the existing NodeAssembly. Shaku remains assembly-only (`assembly.rs:287–309`).

Exact recomposed consumers are client/provider Ops ingress, forwarded protected command ingress, committed outcome recovery, verified projection ingestion, Delivery/replay, protected declaration/governance changes, group bootstrap/join/recovery, both startup roots, and the settings/Gyld launcher/client consumers when their profile is activated. Existing RecordHostPort is home-directory-specific (`assembly/ports.rs:22–42`); TransportPort push reports carrier uptake, explicitly not acknowledgment (`:44–54`). Neither is already the required command/quorum boundary.

First useful production-facing slice: one authenticated known scope/group, complete canonical Glade `value` binding with nonempty existing data, real signed genesis/mapping/governance, fixed authorized data-bearing voters, actual persistence, typed exact retry/outcome recovery and permitted read projection; then failover/restart/partition/cancellation witnesses using that real representation. Appearance/settings is a good affected-consumer witness because the previously empty-key smoke test missed existing data. This slice still needs Create/Mutate/BeginMove/Activate/Retire ordering and policy/retry/configuration retention; selecting one shape does not waive those requirements. External effect-bearing exchanges remain explicitly refused/excluded until their sink contract is separately qualified.

### Baseline fence and absent capabilities

RA-012 exclusion must cover both node roots, downgraded/restarted node binaries, legacy local fallback, provider/client Ops, holder admission, forward landing, live replica ingestion, generic peer sync, seed/import/replay, public direct Store access, relevant declaration/governance mutation, workspace create/claim startup, snapshot restoration/compaction and external supplier mutation paths. A runtime flag or new advertisement covers none of the retained rollback obligation by itself.

The baseline cut must establish complete protected application bytes/chains and materialized state, exact declarations/key/profile mapping, authenticated creation root/incarnation/group/configuration, policy/evidence frontier, historical outcomes, home/generation/retirement, pending/unknown work and a retained exclusion fence. It must compare actual legacy copies and refuse unknown/divergent history. Current per-origin heads, claim rank, known principal, nonempty cache, records.json revision and instance lock are insufficient proofs.

A source search across current node/contracts found no Raft production dependency or protected activation/genesis/request-ID representation. Specifically absent or unresolved:

- Signed scope-root genesis and durable root intent custody; authenticated known-scope/group mapping recovery and conflicting-mapping quarantine.
- Cryptographic end-requester admission and forwarding, canonical authenticated `self` resolution and full ancestry-governed policy evidence.
- Deterministically ordered governance frontier and honest revocation freshness.
- Coherent production durable quorum/application receipt profile and independent failure-domain guarantee.
- Durable request namespace/exact original terminal outcome recovery across generation/retirement/cancellation.
- Group/application frontier-aware Delivery and separately qualified linearizable read barrier.
- Persistent legacy exclusion/migration fence and baseline reconciliation.
- External source/sink generation+sequence enforcement and recoverable dedup outcomes.
- Production membership/bootstrap/recovery APIs; Q3 private learner fixture is not production genesis.
- Exact new package classifications/dependency adoption and measured production fast-test/affected-consumer selection.

### Canonical amendment locations

`GladeRaftAdoptionContract.md:61–76` already names the controlling amendment table. Q4 must resolve those locations and consumers, not amend the adoption document alone:

- `GladeBuyBuildMatrix.md` §1 D-06; §2.B R7/R9; §2.D R16; §4 Q12.
- `dev-docs/glade/GladeWorkspaceDirectory.md` §4 and WD-8.
- `dev-docs/glade/GladeDiscoveryModel.md` §§0/3/7.
- `glade/dev-docs/GladeSubstrateV1.md` §2; §6 W1/W2/W7 and R1/R2/R7/W3–W6; `GladeCrossNodeWritesPlan.md` §3 and its current §8 limits.
- `dev-docs/glade/GladeAuthzModel.md` §§1/3a/3b/4/4a/7a, preserving B4/AZ-16/AZ-17.
- Gyld Records’ source-commit exclusion, related RecordHost/Profile/Admission/Session/Delivery ports, and NodeAssembly allocation.
- Exact protocol/declaration sources and compatibility corpus once profile serialization is selected: `taut/ir/glade.taut.py`, actual declaration/key/payload representation and both clients.
- Local adopting architecture policies: `glade/node/architecture-policy.json`, nested `glade/contracts/architecture-policy.json`, and any new production workspace’s explicitly adopted gate. Existing `glade-discover` gate does not automatically cover these new packages.