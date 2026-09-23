# Glade first-slice profile — what the fixed-peer route relies on from discovery

Status: **profile, 2026-09-23**, plan Step 2.10 ([GladeFirstSlicePlan.md](../GladeFirstSlicePlan.md)
`:507-540`), the rest of build entry step 1 ([GladeBuildEntry.md](../GladeBuildEntry.md) `:47-51`).
It pins; it does not cut over. glade-discover `fd94a1f` says "no production cutover is
authorised by these tranches", and GDL-044/045 record the seven contracts as drafts.
Nothing here changes the wire IR or weakens the frozen discovery contract
([GladeDiscoveryDesign.md](GladeDiscoveryDesign.md), v3.1; cited below as *frozen*).

Read at glade-wz `8056056`, glade `831eded` (every cited `node/` and `wire-rs/` file unchanged
since `d4db2fc`; Step 3.1's `contracts/clock-api` and `contracts/signer-api` new there),
glade-discover `fd94a1f`, taut `d98ff68` (`corpus/glade_hashes.json`); rulings stream
`rulings@1`, notebook `decisions/glade-decisions-rulings.gyld.py` at `9d19ee6`.

How to read a statement: **Decided by** names the ruling, ratified entry or frozen section
that decides it. *Frozen* is the v3.1 semantic freeze the build entry forbids weakening
(`:50-51`). The build entry, the plan and `arch1/*` are proposals, so they direct but do not
decide. **Not decided** says where nothing decides, and stops there. **Pinned by** names
the test. New tests are in `glade-discover/crates/glade-discover-node-adapter/tests/slice_profile.rs`
(§7), each run against a deliberately wrong fixture that it must reject. Tests that already
pinned a statement are cited, not copied (`arch1/RuntimeAndAssurance.md:105`). Drafts decide
nothing; §6 compares the code with them.

## 1. Canonical records

**SP-R1 — The node's home-share kinds, as they stand.** Each is a bare taut CBOR map
(integer keys ascending, no version or kind wrapper) carried as the payload of a glade-wire
`Op` on share `home`, one stream per kind, shape `Log` (`registry.rs:345`). They are decoded
with the fail-open legacy codec (`sysdata.rs:1`). Policy streams fail closed on load
(`registry.rs:92`).

| Kind (`sysdata.rs`) | Stream | Fields |
| --- | --- | --- |
| `NodeRecord` `:7` | `dir.nodes` | 1 node_id, 2 operator |
| `WorkspaceEntry` `:27` | `dir.workspaces` | 1 workspace, 2 name, 3 [eligible_hosts] |
| `ServeClaim` `:50` | `dir.claims` | 1 node, 2 share, 3 lease_expiry_ms (int), 4 epoch (int) |
| `CapabilityGrant` `:76` | `dir.grants` | 1 principal, 2 share, 3 [verbs] (text) |
| `CapabilityRevocation` `:99` | `dir.revocations` | 1 principal, 2 share |
| `BindingDecl` `:119` | `dir.bindings` | 1 app, 2 glade_id, 3 shape, 4 authority, 5 zone, 6 retention |
| `BindingRetraction` `:151` | `dir.binding-retractions` | 1 app, 2 glade_id |
| `ServiceDefinition` `:171` | `dir.services` | 1 app, 2 name, 3 glade_id |
| `PrincipalRecord` `:194` | `dir.principals` | 1 principal |

Streams: `registry.rs:43-56`. `WorkspaceCreateReq/Res` and `SystemSnapshot` are not
home-share records. **Decided by:** GDL-036, ratified 2026-07-07: records are origin-attributed
appends, and reads are queries over the fold. `BindingRetraction` is R9(a), ruled 2026-09-23
(`GladeDeclReconciliation.md:537`, `:994`) and landed at glade `967fcdf`. **Not decided:**
the field layouts are as built; no ruling fixes them (`GladeWorkspaceDirectory.md` is "not
yet a contract"). **Pinned by:** the node's own tests, outside this workspace. SP-R4's
fixtures transcribe these bytes.

**SP-R2 — `NodeTransportBinding{node, endpoint_id, valid_from, sig}`, to be added by
Step 4.2.** **Decided by:** `transport_key_binding = binding_record` (2026-09-21): "a signed
binding record in the node's own chain names its iroh endpoint key". **Not decided:** the
four fields, the set-union fold with revocation winning, and the readers (accept hooks,
HELLO, address lookup) are `IrohGladeMapping.md:389-394`'s, adopted by the plan (§1 table),
not by a ruling. Nothing decides the taut field numbers and types, its stream, the bytes
`sig` covers, or the shape of its revocation. No discovery code reads it. **Pinned by:**
nothing yet; 4.2's tests.

**SP-R3 — The registry records the discovery core writes.** Exactly two kinds: a
`ServeClaim` for a workspace slot and a `ServiceInstanceClaim` for a binding slot on share
`svc`. The kernel emits them as `Append` drafts, and the adapter finalizes them: a `Mint`
identity becomes the allocated `RecordId` (`append.rs:91-119`). The core reads grants and
revocations; it never writes them. The payload is directory record v1: canonical CBOR
`{1: 1, 2: kind, 3: body}`, where kind is `ServeClaim=0 | ServiceInstanceClaim=1 |
CapabilityGrant=2 | CapabilityRevocation=3`. The `ServeClaim` body is `{1 node, 2 share,
3 claim_id, 4 grant_ref, 5 lease_expiry_ms, 6 epoch}`. **Decided by:** frozen §4 `:197-229`
and §8 `:336-349`. **Pinned by:** `sp_r3_p2_registration_signs_the_unsigned_bytes_of_a_v1_serve_claim`,
with the wrong fixture `sp_r3_rejects_a_glade_shaped_serve_claim`; existing
`p6_append::allocation_finalizes_the_minted_identity_before_signing`.

**SP-R4 — The two families share names, not encodings or semantics, and no translation
between them exists.**

| | Node (`sysdata.rs`, `registry.rs`) | Discovery v1 (frozen §4) |
| --- | --- | --- |
| `ServeClaim` | no id and no grant reference. `who_serves`: live while `lease > now`; highest epoch wins; no grant consulted (`:379-386`) | `claim_id` and `grant_ref`. Live while `effwall + SKEW < lease ≤ effwall + SKEW + MAX_LEASE`; needs an owner grant and a configured binding; winner by `(epoch, claim_id)` |
| `CapabilityGrant` | no id, issuer or scope; verbs as text. `grants_for` folds every grant op, whoever appended it (`:403-425`) | id, issuer, scope; verbs `serve=0, execute=1, takeover=2`. The issuer must be the share's configured owner root and must sign it |
| `CapabilityRevocation` | `(principal, share)`: clears every grant for the pair, including later ones | `{1 revokes: GrantId}`: one grant, revoked by the owner |
| Envelope | bare map, legacy fail-open decode | versioned map, fail-closed decode |

**Not decided:** any mapping between the families. Frozen §4's table is the v3 *successor*
of three sysdata kinds ("delta from `sysdata.rs`"), not a translation, and no ruling cuts
over. Any adapter that feeds node records to the discovery kernel, or the reverse, is
blocked on that definition. Where discovery records would ride in Glade's share space is
also not decided. They cannot share a `dir.*` stream, because the node decodes every payload
there as its own kind with the legacy codec. **Pinned by:**
`sp_r4_glade_home_share_kinds_are_not_discovery_records` (all nine kinds are quarantined as
`Malformed`), with the wrong fixture `sp_r4_rejects_a_discovery_grant_offered_as_a_glade_kind`.

## 2. Proof profile

**SP-P1 — Grants stay Glade's own taut/CBOR records: no outside proof family and no
presented proof.** For discovery, authority is folded, not presented. The kernel checks the
grant records it retains against configured owner roots and node bindings (frozen §4
`:235-261`). A route carries an authenticated `PrincipalCtx`, not a proof (frozen §1
`:32`, `:92-95`). No grant verb is added: discovery's verbs are `serve | execute | takeover`
(`:230-233`). **Decided by:** `proof_family = taut_grants` (2026-09-21);
`identity_adapters = none_in_v1` (grants made by hand); frozen §4. **Not decided:** the chain
the ruling describes ("each link signed by its parent and only ever narrowing scope").
Neither family has a parent link or attenuation, and the grant schema is "pinned at build
time" (`GladeAuthzModel.md:59`, a working draft). The slice's owner-root principal is also
not decided; the kernel takes it from `KernelConfig.workspace_owner_roots`. **Pinned by:**
SP-R3 and SP-R4's tests (the encodings); existing
`p5_authority_mutations::workspace_authority_conjuncts_are_independently_necessary`.

**SP-P2 — The signed bytes, exactly.**

- The `Signer` is handed `unsigned_canonical_bytes(op)` (`codec.rs:59`, `:1156-1187`) as they
  are, not a digest. These bytes are a canonical CBOR map of 10 fields: `{1 share: tstr,
  2 glade_id: tstr, 3 key: bstr (≤ 4 KiB), 4 origin: tstr, 5 seq: uint, 6 prev: bstr .size 32
  / null, 7 lamport: uint, 8 refs: [* {1 origin: tstr, 2 seq: uint, 3 hash: bstr .size 32 /
  null}], 9 shape: uint (0 value, 1 log, 2 stream), 10 payload: bstr}`. Heads are
  shortest-form, lengths definite, keys ascending.
- `op_hash` is SHA-256 of those bytes. `prev` and head hashes are op_hashes.
- The signed op is a map of 11: the same ten fields byte for byte, plus `11 signature: bstr`
  (any length, no algorithm tag), ≤ 16 KiB in total (`codec.rs:85-95`).
- For every op discovery can represent, those ten fields are byte-identical to glade-wire's
  `Op::to_cbor()`. Discovery's `op_hash` reproduces taut's op-hash oracle, which the node's
  `chain.rs:11` also reproduces.

**Decided by:** frozen §4 `:180-184`: "the unsigned envelope preserves the current taut `Op`
fields 1–10; required signature field 11 is the B5 signature over canonical fields 1–10".
Also B5, ratified 2026-07-12 (`RulingWorksheet.md:272-293`): security-sensitive ops are
signed, carry their strict predecessor, and are verified before persistence or fold.
`signature-api` SG-001 is a draft with the same bytes. **Pinned by:**
`sp_p2_unsigned_bytes_are_taut_op_fields_1_to_10` (4 corpus vectors), with the wrong fixture
`sp_p2_rejects_a_wrong_shape_mapping`; and `sp_r3_p2_…`, with the wrong fixture
`sp_p2_rejects_a_signer_handed_the_op_hash`.

**SP-P3 — Encoding incompatibilities that are explicit blockers, not fixtures** (build entry
`:65-66`).

- (a) The glade-wire `Op` has no field 11 (`generated.rs:189-200`). Where a node op's
  signature rides on the node's wire is not decided, and adding it is a wire-IR change
  (plan `:38`). This blocks the 4.1 adapter's per-op origin signature
  (`verify_origin_sig`, `peer.rs:143`).
- (b) The shapes `Swmr=3` and `Crdt=4` (`generated.rs:114-137`) have no discovery shape. This
  blocks any adapter that reuses `SignedOp` for the node's app-data ops. Directory records
  are `Log`, so they are unaffected.
- (c) Discovery requires 32-byte `prev` and ref hashes and non-negative `seq`/`lamport`. The
  node's types (`Option<Vec<u8>>`, `i64`) do not enforce this, though every value the node
  produces today satisfies it.
- (d) `NodeHello.sig` (`generated.rs:285-289`) is not over an `Op`. Its signed bytes are not
  decided; today they are a stub digest, `sha256("glade/peer/hello" ‖ node.key)`
  (`peer.rs:76-81`), which 4.1 names.

Not blockers: signature length (a 64-byte signature fits field 11), and principal text
(discovery's `Principal`/`NodeId` are free text, so the node's hex id fits). **Pinned by:**
`sp_p3_glade_ops_outside_the_envelope_are_explicit_blockers` for (a), (b) and the fit, with
the wrong fixture `sp_p3_rejects_a_fixture_that_calls_swmr_representable`.

**SP-P4 — Algorithm and key: not decided.** No ruling or ratified entry selects the signature
algorithm. GDL-007 asks for the "signature scheme" and is open (`DecisionLog.md:25`). B5
names none. `proof_family` fixes the encoding family, not the algorithm. Ed25519 is named
by the plan (4.1) and by `GladeAuthzModel.md:58`, a working draft. How a verifier maps an
origin to a verifying key is also not decided: the node's id is `hex(sha256(node.key))`
(`sysdir.rs:213-216`), from which no public key can be derived, and the plan leaves `NodeId`
open (`:907`). Which key signs for a node, `node.key` or a device key certified under an
account root (E-users-1, `RulingWorksheet.md:489`), is not decided for the slice either;
Step 3.1's `SignerPort` assumes the node key. Nor is the per-purpose domain encoding
decided. `SignerPort` requires one ("the purpose is part of what is signed") and leaves
"algorithm, domain encoding and key resolution" to the implementation, noting that "the
proof profile names them" (glade `contracts/signer-api/src/lib.rs:50-54`). This profile can
name none of them, because nothing decides them. The discovery `Signer` signs the unsigned
bytes under a scheme it also leaves to the implementation (SG-001). A node's `OriginOp`
signature is a discovery field-11 signature only if both sides apply the same `OriginOp`
domain. Until that encoding is named, it is an explicit blocker to the single ed25519
adapter that is meant to "implement both and run both suites" (`signer-api/src/lib.rs:11`;
plan 4.1). What *is* decided: a verifier's `Valid{signer}` must equal the envelope origin, or
the kernel records `BadSignature` (frozen §4 `:243`; `ingest.rs:69-73`). **Pinned by:**
`p3b_ingest_authority::every_derived_authority_equality_is_independently_necessary`.

**SP-P5 — Verification outcomes.** The kernel's `VerificationResult` is
`Valid{signer} | BadSignature` (`model.rs:195-198`). The draft `Verifier` adds
`Err(Unavailable)`, which SG-003 forbids mapping to `Valid` or `Invalid`. **Not decided:**
how a host reports "verifier unavailable" to the kernel. Feeding `BadSignature` quarantines a
possibly valid op; withholding it leaves the op to the next round. Consistent with B5
either way: the adapter verifies its own op before persisting it (`append.rs:114-118`;
`p6_append::freshly_signed_bytes_are_verified_before_the_durable_acceptance_write`).

## 3. Clock inputs

**SP-C1 — One clock feeds the restore and every step.** The kernel reads no clock (frozen §1
`:23-24`, INV-D0). Each step takes `StepCtx{mono, wall}`, sampled together (`:29`). In a
composition, the restore (`restore_with_recovery`, `NodeDriver::from_snapshot`,
`clock::restore`) and every step (`handle`, `schedule_wall`) MUST sample the one
`ClockPort`. **Decided by:** frozen §1–§2 (time is an explicit input), and the
`async_witness` ruling (2026-09-21: "every real provider in a test composition is overridden
or lazy"). **Not decided by a ruling:** that the one override reaches every consumer. The
plan directs it (Step 2.10), taking it from `arch1/InjectionGraphRefinement.md:24-25` ("the
clock binding MUST be substituted once for all named consumers in a test-node composition")
and AR-03 (`RuntimeAndAssurance.md:121`: "a second uncoordinated fake fails a wiring test").
Both are proposals. **Pinned by:**
`sp_c1_c2_one_clock_and_the_watermark_committed_with_each_step`, with the wrong fixture
`sp_c1_rejects_a_second_clock_at_restore`. A restore fed by a clock 10 s ahead turns the
registration made on the composition's clock into an expired, refused one.

**SP-C2 — The watermark is committed with each state delta, and a restart behind it fails
closed.** The forward-only watermark commits atomically with the delta (frozen §1 `:101-110`;
the driver's `durable_watermark`, `driver.rs:831`). A restore whose wall is behind the
watermark is `Uncertain{floor}` until `wall ≥ floor + CLOCK_RESYNC_MS`. An unreadable
watermark stays `Uncertain` until a trusted `ClockReseed`. While `Uncertain`, nothing is
published or renewed and projection is empty (`:120-128`, INV-D1). So a route answers
`NoClaim` for a retained, live claim, and only `State::clock()` tells that from absence: the
kernel's `RouteAns` has no clock-uncertain answer (§6 row 5). **Decided by:** frozen §1–§2.
**Not decided:** the source of a trusted `ClockReseed`, and what a node reports on its first
boot, when no watermark exists yet. The tests seed the first sample as readable; that is a
fixture choice, not a decision. **Pinned by:** `sp_c1_c2_…`, with the wrong fixture
`sp_c2_rejects_a_store_that_keeps_its_first_watermark`; existing
`p3a_clock::readable_uncertainty_recovers_only_at_the_checked_threshold`,
`p6_clock::unreadable_or_backward_watermark_restores_uncertain_and_blocks_append`, and
`p6_driver::durable_state_and_watermark_commit_before_dependent_effects`.

Constants: the frozen defaults are `SKEW_MARGIN` 5 000, `MAX_LEASE` 3 600 000 and
`CLOCK_RESYNC_MS` 30 000 ms, configured per node (`:129-131`). The node's lease is 30 000 ms,
renewed every 10 000 ms (`claims.rs:40`, `:43`), which is inside those bounds. Its own
expiry rule is different (SP-R4).

**SP-C3 — The one `ClockPort` must yield a monotonic instant together with the wall time.**
**Decided by:** frozen §1 `:29` and §2 `:116-119`: `WallMs` is cross-node and used in records;
`MonoInstant` is local and used only for scheduling. **Not decided:** which port supplies the
monotonic instant. Step 3.1's `ClockPort` (glade `contracts/clock-api/src/lib.rs:54-56`,
adopted from the witness's) is `now_ms()` only, and says "Wall time is not monotonic". A
discovery consumer built on it alone has no `mono` to put in `StepCtx`; that is an explicit
mismatch for 3.2. The node also reads the wall clock directly to stamp leases
(`claims.rs:163`, `:250`, `sysdir::now_ms`), which 3.2 replaces. **Pinned by:** no test here;
the port lives in glade's contracts workspace.

## 4. Local-acceptance guarantee

**SP-L1 — Durable local acceptance precedes any sync effect.**

1. The adapter verifies and persists the exact signed bytes before it returns `OpAccepted`
   (`append.rs:109-119`).
2. The kernel's only sync effect for a registration is emitted on that `OpAccepted`, and it
   carries exactly the persisted bytes (frozen §8 `:336-349`, INV-D5).
3. The driver commits each step's state delta, watermark and outbox before executing any
   effect, and a failed commit executes none (frozen §1 `:101-110`; `driver.rs:559-592`).

A receipt means local durable acceptance only, not replication or discoverability
(`RegistryContractDraft.md:34-36`; transport-api `LocalAcceptance`). The sync effect is the
kernel's `Gossip` effect, an addressed send (frozen `:38`) carrying the post-acceptance
`DirOp` push and the `SyncStart`/`SyncOps`/`SyncEnd` round (frozen §5). Under
`dissemination = sync_round_only` there is no gossip overlay; the effect's name predates the
ruling. **Decided by:** frozen §1, §8, §10 (INV-D5). GDL-043 (`DecisionLog.md:66`, a
proposal: "persistence-before-acceptance/gossip remains mandatory") and build entry `:57-58`
restate it. **Pinned by:** `sp_l1_sync_effect_follows_durable_local_acceptance`
for 1–2, with the wrong fixture `sp_l1_rejects_a_host_that_accepts_without_persisting`.
Existing tests pin 3: `p6_vertical_integration::advertise_to_exact_gossip_commits_each_durable_boundary_first`
and `p6_driver::failed_commit_leaves_state_unchanged_and_executes_no_effect`. The fixture is
volatile. Physical durability is 4.4's, and a fixture must not pretend to be it (build entry
`:55-56`).

## 5. Trust and namespace

**SP-T1 — `scope_model = node_trust`, as far as discovery is concerned.** The kernel's trust
inputs are its configuration (the peer set, node↔principal bindings with their plane, and
owner roots; `model.rs:34-48`) plus records the host has verified. That configuration is the
build entry's "explicit development trust configuration" (`:40-41`).

- The directory round answers `SyncStart` only from configured peers (`sync.rs:31-33`).
- A pushed `DirOp` is judged on its record and the configuration, not on who delivered it
  (`model.rs:364-375`). So until signatures are real (4.1), a node's protection against records
  pushed by anyone who can reach it is the host's verifier and who may connect. Until 4.2's
  accept-time binding check lands, the relay ruling notes that endpoint ids "are the only
  lock on the door".
- A claim routes only if signer = envelope origin = `grant.principal`; the grant is issued by
  the share's configured owner root, carries `serve` and has no scope; and the principal is
  bound to `claim.node` (frozen §4 `:235-247`).

**Decided by:** the `scope_model` ruling (2026-09-21), frozen §4, and the `relay_posture`
ruling (2026-09-22). **Pinned by** existing tests:
`p3d_append_sync::sync_start_from_an_unconfigured_peer_is_inert`;
`p5_authority_mutations::workspace_authority_conjuncts_are_independently_necessary` (eleven
conjuncts, each necessary); `walking_skeleton::forged_claim_cannot_produce_a_match`. No test
pins the absence of a check on the delivering peer; that point is read from the code.

**SP-T2 — `identity_adapters = none_in_v1`.** Principals come only from configuration and
from the `PrincipalCtx` supplied by the trusted ingress. Discovery never reads
`authenticated_context` and consumes no identity token: routing is authz-blind (frozen §7
`:317`). No identity-adapter crate can enter a discovery crate without an
architecture-policy change (ARCH-002). The ruling's "trust-provider interface" answers "is
this assertion real?" (`glade-decisions.gyld.py:420-427`, gyld). It is not the draft
`TrustPolicy`, which authorizes. Neither is implemented for the slice. **Decided by:** the
`identity_adapters` ruling (2026-09-21). **Pinned by:** `sh scripts/check-architecture.sh`;
`public_contract::route_event_and_effect_are_direct_and_authz_blind` (core).

**SP-T3 — `metadata_exposure = granted_shares_only`, as far as discovery is concerned.**

- A route answer (a `NodeId`) leaves the trusted node only with the consumer policy's
  evidence for that principal and that query. With none, the answer is `Denied` and discovery
  is not consulted (INV-D4; `ingress.rs:184-195`). A replay re-evaluates the policy, so a
  revocation reaches a cached answer (`:220-228`).
- Discovery's records are directory records, exempt by the ruling: "the directory stays
  exempt, since it is how grants arrive". They replicate whole to configured peers, and no
  share content is involved.
- The serve-hop grant check on the node's four paths is 4.3's, outside discovery.

**Decided by:** the `metadata_exposure` ruling (2026-09-22) and frozen §7. **Pinned by**
existing tests: `p6_ingress::unauthorized_request_stops_before_discovery_and_service_without_leaks`
and `p6_ingress::source_revocation_on_replay_suppresses_cached_node_until_authority_returns`.

**SP-N1 — One namespace, one fixed mapping.** Slots match exactly on
`(share, glade_id?, key?)` (`routing.rs:6-14`). A share with no configured owner root never
routes, and a grant copied to another principal never routes (AR-05,
`RuntimeAndAssurance.md:123`). The slice has no referral or placement: the peers are
`KernelConfig.peers`, and `ShardLocator` is off the path (the relay ruling: "no address-lookup
service and peers named directly"). **Decided by:** frozen §4 (a claim needs a grant from
the share's configured owner root, and the grant's principal must be the claim's origin), and
the `relay_posture` ruling (peers named directly). The single namespace is the build entry's
slice definition (`:40-41`), not a ruling. **Not decided:** the namespace-authority proof
schema (GDL-045, `DecisionLog.md:68`; placement-api PL-002). **Pinned by** existing tests:
`p5_authority_mutations::workspace_authority_conjuncts_are_independently_necessary` ("grant
signer must be configured owner", "grant subject must match claim origin");
`routing::exact_slot_matching_prevents_cross_scope_shadowing`.

## 6. Divergences: `glade-discover-core` and `-node-adapter` against the seven draft contracts

Across all seven, every draft is asynchronous (`&self`, `Send + Sync`, lazy
`impl Future + Send`), while every adapter host trait is synchronous (`&mut self`, a generic
`Error`). No crate depends on both an adapter trait and a draft contract (the dependency
lists in `architecture-policy.json`). So no test can run a draft's conformance suite against an
adapter provider without a new dependency edge, which is a policy change this step does not
make. Two different traits are named `Transport`. This step records the divergences; it
refactors neither side.

| # | Draft contract | Core / node-adapter today | The slice follows |
| --- | --- | --- | --- |
| 1 | `Transport::send` → `LocalAcceptance` or `Unavailable \| Rejected \| OutcomeUnknown` (TR-001..004) | `transport::Transport::send(&mut, to, msg)` → `Result<(), E>` (`transport.rs:6-10`); `GossipSend{Sent, Failed}` cannot tell a known rejection from an unknown outcome (TR-002). The driver keeps a failed effect in the durable outbox and retries it, the safe reading of `OutcomeUnknown` | The adapter trait in the discovery composition, unchanged. TR-001..003 are the requirements the real carrier is measured against (plan `:514-524`; `IrohGladeMapping.md:374`). Not decided: which trait the carrier implements |
| 2 | `Signer{principal, sign}`, `Verifier::verify` → `Valid{signer} \| Invalid`, or `Err(Unavailable)` | `AppendHost::{signer, sign, verify → bool}` (`append.rs:32-47`); verify names no signer, and the adapter asserts `host.signer()` (`:172-178`). The kernel's `VerificationResult` has no `Unavailable` | The signed bytes are the same on both sides (SP-P2). The plan's "reused where the types allow" (`:583-584`) did not hold: 3.1 made `SignerPort` local, because the draft traits take a `SignedOp` and are not dyn-safe (plan `:594`). The `Unavailable` mapping and the domain encoding are not decided (SP-P4, SP-P5) |
| 3 | `DurableOperationStore::{load(hash), persist(op)}` (OS-001..007) | No hash-keyed store. `AppendHost::{accepted, persist_accepted}` is keyed by `(slot, generation, intent)`; `DurableCommit` stores whole snapshots under a revision CAS (`driver.rs:210-231`) | `DurableCommit`: the draft says it "alone cannot satisfy the complete v3.1 append/restart transaction boundary" (`operation-store-api/src/lib.rs:1-3`). 4.4 also runs the store suite on the real adapter |
| 4 | `AcceptanceJournal`: an atomic commit of key, draft, exact op, revision, cursor, watermark and handoff; the chain is validated before commit and leaves no residue (AC-007); `RequestKey` names the requester | The adapter allocates, signs, verifies and persists (`append.rs:91-119`); the kernel checks chain position only at `OpAccepted`. A bad allocation leaves persisted bytes that every retry replays while the intent stays pending. `AppendKey` has no requester | The adapter (frozen §8). The journal is a draft (GDL-045: "no production cutover") |
| 5 | `RegistryWriter::{publish, renew}` → a receipt; `RegistryReader::resolve(principal, query, limit)` → `Resolution{claims, truncated, observed_at_ms}`, or `Denied \| Unavailable \| ClockUncertain` | `Advertise` → `Append` → `OpAccepted`; `Route` → `Reply{Matched{node} \| NoClaim}` (`model.rs:259-291`): one winner, no bound or truncation flag, `NoClaim` while the clock is uncertain (`projection.rs:26-28`), denial in the ingress | The kernel (frozen). Not decided: whether 3.4's "partial lookup" must report clock uncertainty |
| 6 | `TrustPolicy::evaluate` → `Permit(Authorization)` or `Deny`, or `PolicyError` (TP-001..005) | `ConsumerPolicy::authorize` → `Option<AuthorizationEvidence>` (`ingress.rs:45-54`); `None` covers both deny and cannot-decide (both fail closed). Publish authority is configuration (`authority.rs:10-63`), with no policy call | `ConsumerPolicy` plus configuration. `TrustPolicy` is off the path |
| 7 | `ShardLocator::locate` → shard, authority, mapping epoch, candidates (PL-001..004) | None: the kernel is topology-blind (frozen §9 `:385`); the peers are `KernelConfig.peers` | Fixed configured peers (SP-N1). The proof schema is open (GDL-045) |

## 7. Tests and the fast loop

Command, from `glade-discover`: `cargo test --locked -p glade-discover-node-adapter --test slice_profile`.
It runs 14 tests: 6 statement tests and 8 wrong-fixture tests (`should_panic` on the
statement's exact message). Deterministic: no clock read, network, executor or node
(LBT-008).

| Test | Pins | Wrong fixture it must reject |
| --- | --- | --- |
| `sp_r3_p2_registration_signs_the_unsigned_bytes_of_a_v1_serve_claim` | SP-R3, SP-P2 through `append::handle_append` | a node-shaped `ServeClaim` payload (`sp_r3_rejects_…`); a signer handed `op_hash` (`sp_p2_rejects_a_signer_handed_the_op_hash`) |
| `sp_p2_unsigned_bytes_are_taut_op_fields_1_to_10` | SP-P2 against taut's 4 oracle vectors | `value` mapped to `Log` (`sp_p2_rejects_a_wrong_shape_mapping`) |
| `sp_p3_glade_ops_outside_the_envelope_are_explicit_blockers` | SP-P3 (a), (b) and the 64-byte fit | a fixture calling `Swmr` representable |
| `sp_r4_glade_home_share_kinds_are_not_discovery_records` | SP-R4, all nine kinds | a discovery grant offered as a node kind |
| `sp_c1_c2_one_clock_and_the_watermark_committed_with_each_step` | SP-C1, SP-C2 | a second clock at restore; a store that keeps its first watermark |
| `sp_l1_sync_effect_follows_durable_local_acceptance` | SP-L1 steps 1–2 | a host that acknowledges without persisting |

What the tests do not prove: cryptography (the signer echoes its input), durability (the
host is volatile), and anything about iroh or the node. Measured on an Apple M3 Pro (load
average ≈ 5), Rust 1.96: test bodies 0.00 s at harness precision; the warm command
0.03–0.04 s wall; after touching the test file, rebuild plus run 0.79–0.96 s wall. It also
passes on MSRV 1.85.0. These are observations, not budgets.

## 8. Not decided, collected

1. Which family carries the slice's registration. The plan's steps name the node's
   (3.4 "`claims.rs` leases", 4.3 `grants_for`, 4.6 "lease expiry from `claims.rs`"); no
   ruling routes it through the discovery kernel (SP-R4).
2. Any translation between the families, and where discovery records ride in Glade's
   share space (SP-R4).
3. `NodeTransportBinding`'s fields and fold (adopted from `IrohGladeMapping.md`, not ruled),
   its taut layout, stream, signed bytes and revocation shape (SP-R2).
4. The grant chain (parent link, narrowing) and the slice's owner-root principal (SP-P1).
5. Where a node op's signature rides on the node's wire, and what HELLO signs (SP-P3).
6. The signature algorithm, origin → key resolution, the key that signs for a node, and
   the per-purpose domain encoding that `SignerPort` defers to this profile (SP-P4).
7. How a host reports "verifier unavailable" to the kernel (SP-P5).
8. That one clock override reaches every consumer. The plan directs it; no ruling decides it
   (SP-C1).
9. The source of a trusted `ClockReseed`, and the first-boot watermark (SP-C2).
10. Which port supplies the monotonic instant; Step 3.1's `ClockPort` is wall-only (SP-C3).
11. Whether "partial lookup" must report clock uncertainty (§6 row 5), and which
    `Transport` the carrier implements (§6 row 1).
12. The namespace-authority proof schema, and the single namespace itself, which is the build
    entry's slice definition, not a ruling (SP-N1).
