I've read all required files plus the supporting docs, and verified the key claims against the vendored OrbitDB source (`third-party/orbitdb/src/`) and `rust-libp2p`. The LOC arithmetic checks out (oplog spine 1,181 + sync 317 = 1,498). Critique below.

---

## 1. Findings by severity

### Critical

**F1 — The sync catch-up protocol in the plan does not exist in OrbitDB.**
P05c (`sync-catchup`) and ROR-006 specify "missing-entry request/response and bounded stream." OrbitDB does no such thing. `sync.js` does exactly two things: subscribe to a gossipsub topic per log address and, on peer-subscribe, dial a *one-shot* direct stream that ships current **heads only** (`sync.js:112-113, 191-260, 291-294`). Missing transitive entries are fetched **by CID through the storage tier**, which in production is IPFS/Helia (`oplog-store.js:5` "Options: Memory, LRU, IPFS"; `log.js:212-363` traverses `next`/`refs` via `get(hash)`). There is no application-level request/response.

Consequences:
- ROR-006's "same sync tests pass on direct sim, libp2p sim, and real libp2p" is testing a protocol the real adapter won't speak the same way.
- **D0004 cannot validate the catch-up layer** — JS OrbitDB has no request/response catch-up to compare traces against (P07b). D0004's real reach is the oplog/entry/heads layer, not sync catch-up.
- Inputs (Plan 66-76) and the substrate trait (Plan 232-243) omit any **content-addressed block fetch (get-by-CID)** concept — the actual replication mechanism. The `store` crate models memory/crash/persistence (Plan 175) but not the content-addressed block-exchange tier OrbitDB relies on.

This is either a deliberate (better) redesign or an oversight; either way it is an unrecorded protocol delta. ROR-004 says deltas go in `Decisions.md` — this one isn't there.

**F2 — "Multi-writer document CRDT" conflates OrbitDB's LWW document store with a true CRDT.**
ROR-012/D-none/Risk line 10 treat documents as a CRDT to be "solved." OrbitDB's `documents` store is **not** a field-merge CRDT — it is a keyed last-write-wins store reduced from the oplog: `put`/`del` are just oplog operations keyed by `indexBy`, reduced in traversal order (`databases/documents.js:38-60`). `database.js`/`documents.js` are **not in Inputs** (Plan 66-72). So:
- ROR-012's "JS comparison fixtures where applicable" is undefined — there's no JS CRDT to fixture against; the JS behavior is just oplog-order LWW.
- The convergence invariant (all replicas converge regardless of delivery order) is the core CRDT property and is **never stated as an acceptance gate**. P04b tests "concurrent set/delete/update" but not order-independent convergence as such; P04d defers it to the unbuilt sim.
- The single highest-risk semantic has its entire model deferred to P04a ("model documented with failing tests") with no spec in the plan. For a HIGH/HIGH item that's the weakest part of the plan.

**F3 — Difficulty is materially understated (calibration).**
Plan 22-26: "about 1,498 raw LOC… small in source size." True for the spine, but the hard, brittle surface lives in OrbitDB's **dependencies**, which the LOC count excludes: dag-cbor canonical encoding, CID/multihash/multibase (`entry.js:2-5`), the identity double-signature scheme + keystore (`identity.js:26-38` — `signatures.id` and `signatures.publicKey+id`), secp256k1 sign-input reconstruction byte-for-byte, the IPFS block storage tier (`storage/{composed,ipfs-block,level}.js`), and the encryption path (`entry.js:69` `encryptPayloadFn`). Inputs omit `identities/` and `access-controllers/` entirely, yet ROR-003 requires signature verification to match JS fixtures — which is impossible without porting the identity model. Per CLAUDE.md's "calibrate difficulty honestly," this plan errs by *under*-stating: it sounds like a 1.5k-LOC port; it's a 1.5k-LOC spine sitting on a multi-thousand-LOC encoding/crypto/content-addressing iceberg.

### High

**F4 — ROR-017 formal/model-checking artifacts have no producing checkpoint.**
ROR-017 acceptance: "formal/model-checking artifacts exist before release." SecurityAuditPlan lists TLA+/Kani/Prusti as candidates (line 56). But no phase or checkpoint produces them — `security-bootstrap` (P00d) is "skeleton/checklist/placeholders" only. A release-blocking requirement with no checkpoint = no test and no owner. (Workstreams "Security/audit" output lists "proof-oriented invariant checks" but no Checkpoints row schedules it.)

**F7 — Head-announcement delivery semantics are unspecified, risking a too-reliable simulator.**
OrbitDB announces heads over gossipsub: lossy, best-effort, mesh-routed, no delivery guarantee (`sync.js:18-29, 260`). The substrate trait (Plan 232-243) lists "head announcements" with no delivery contract. If the direct simulator models announce as reliable/ordered/dedup'd, it is *more reliable than reality* and gives false confidence — exactly Risk line 11, but the plan never pins the contract that would prevent it. The trait must state: lossy? deduplicated? ordered per-peer? at-most-once?

### Medium

**F5 — "Same simulator N=2→million by parameter" is partly aspirational.**
ROR-008/D0003/Plan 60-62 insist it's "a scale parameter, not a separate mode," but `compressed-state` (P08b) introduces counters + sampled-exact sessions — a genuinely different state representation and execution path. Compression *is* a fork. P08a's stop condition "Stop if high-scale path forks from small-scale path" is therefore unfalsifiable as written. Define "same" precisely: shared scenario DSL + shared invariant checks, with a divergent state representation, plus an explicit equivalence check (a compressed run must reproduce the invariants of a sampled exact session at the same seed).

**F6 — No memory/time budget for the million-scale gate.**
ROR-008 acceptance references "an agreed memory/time budget"; Handoff residual Q3 confirms it's undefined. P08c is untestable until a number exists.

**F8 — Several checkpoints exceed the reasoning budget and should be pre-split.**
The 500-LOC target measures diff size, not cognitive load (Checkpoints 9-13 admits it's "not a hard metric"). dag-cbor compatibility requires holding the IPLD spec in context regardless of line count. Candidates that will blow the budget: `dag-cbor-cid` (P02b), `direct-sim-kernel` (P06a — event loop + seed replay + lifecycle), `substrate-trait` (P05a), `js-orbitdb-runner` (P07a — cross-language plugin), `doc-crdt-conflicts` (P04b). See §3.

**F9 — Phase ordering inverts a dependency.**
P04 (document CRDT, incl. P04d convergence) precedes P05 (sync) and P06 (simulator), but convergence is a multi-replica property requiring the sim — P04d's own split condition admits it ("Split if sim dependencies are not ready"). Either make P04 model-only and add a convergence gate after P06, or reorder.

**F10 — C10k/runtime study is scope bloat for a plan that declares it out of scope.**
ROR-016/D0007 say the runtime work is *downstream handoff only*, yet `host-c10k-contract` (P11c) is a checkpoint here producing a full benchmark contract. That belongs in the downstream host plan; keep only the handoff note (P11b) in GLP-0003. Separately, the study analyzes raw TCP C10k while the actual downstream is a **libp2p swarm** (gossipsub/Kademlia, QUIC vs TCP) with its own scaling profile — the analysis is disconnected from the real workload. And NetworkingRuntimeStudy:58 leans on "64 bytes of task metadata" as if it were per-connection cost; that's per-task metadata only — real cost is buffers + protocol state, which the same doc then concedes. Don't anchor on the 64-byte figure.

### Low

- **F11** — Node runner pulls OrbitDB JS + js-libp2p into CI: a large test-harness supply-chain surface. SecurityAuditPlan covers it only via "npm audit" (line 53). Add lockfile pinning + provenance.
- **F12** — Fixtures are the oracle but their corpus integrity (hash/provenance pinning) isn't an audit artifact. Add it to SecurityAuditPlan §"Audit Artifacts."
- **F13** — Entry `v:2` versioning (`entry.js:79`) is never mentioned; state whether v1 entries are in scope.

### Categories with no issue
- **Accidental downstream product naming in rust-orbitdb scope:** none. Grep across the plan for griplab/glade/glial/grok/grip-share returns nothing; scenarios are consistently "terminal-shaped, product-agnostic" (Plan 318, Sim 124-138). Clean.
- **Conflict zero-compare:** correctly handled — OrbitDB throws on a 0 comparison (`conflict-resolution.js:68-77`) and P02d's stop condition captures it.

---

## 2. Open questions that block implementation

1. **(F1)** Is entry catch-up modeled as content-addressed get-by-CID (matching OrbitDB's IPFS-backed storage), or as a new request/response protocol? If the latter, record the delta and accept that JS trace comparison (P07b) can't cover it.
2. **(F1)** Where does content-addressed block exchange live — a substrate concept, a store-tier trait, or both? It currently lives nowhere.
3. **(F2)** What *is* the document model — oplog-reduced LWW (OrbitDB-faithful) or a genuine field-merge CRDT (a new design)? The conflict rule (Lamport clock + identity tiebreak vs per-field)? This blocks all of P04.
4. **(F7)** What is the delivery-semantics contract for head announcements (lossy/ordered/dedup)?
5. **(F6)** What memory/time budget defines the accepted million-scale gate?
6. **(F4)** Which invariants get formal/bounded verification, with which tool, in which checkpoint, owned by whom?
7. Identity/access-controller scope: which identity provider(s) and access-controller behaviors must be fixture-compatible for ROR-003/ROR-004? (Inputs currently exclude them.)

## 3. Checkpoints that should be split further

- `dag-cbor-cid` (P02b) → `dag-cbor-encode` / `cid-multihash-multibase` (already flagged "split if needed"; make it mandatory).
- `direct-sim-kernel` (P06a) → `sim-event-loop+seed-replay` / `node-lifecycle`.
- `substrate-trait` (P05a) → `substrate-types+errors` / `fake-transport+delivery-contract` (forces F7 to be decided here).
- `js-orbitdb-runner` (P07a) → `node-plugin-load+instantiate` / `scenario-drive+trace-emit`.
- `doc-crdt-conflicts` (P04b) → split per operation family (the "split if needed" note should be promoted), *after* the model spec (F2) exists.
- `signature-verify` (P02c) implicitly needs an `identity-model` checkpoint added before it (F3).

## 4. Requirements lacking clear tests

- **ROR-017** (formal/model-checking) — no checkpoint produces the artifacts (F4).
- **ROR-008** — acceptance cites a budget that doesn't exist; "same path" gate is unfalsifiable (F5/F6).
- **ROR-012** — "JS comparison fixtures where applicable" undefined; convergence invariant never gated (F2).
- **ROR-006** — catch-up half of the acceptance tests a protocol absent from the JS reference (F1).
- **ROR-003** — signature fixtures require an identity model that isn't in Inputs or any checkpoint (F3).

## 5. Decisions needing stronger wording or addition

- **D0004 — narrow it.** It legitimately tests the *oplog/entry/heads/conflict* harness against JS. It cannot test the catch-up protocol or the document CRDT (no JS equivalent). State the in-scope conformance surface explicitly, or D0004 oversells.
- **New decision (sync/replication model)** — record whether replication is content-addressed block fetch (OrbitDB-faithful) or custom request/response, and why. Currently undecided and unrecorded (F1).
- **New decision (document semantics)** — LWW-by-clock vs true CRDT, with the operation set and conflict rule (F2).
- **New decision (head-announcement delivery contract)** (F7).
- **D0007 — strengthen scope boundary.** Move the C10k *benchmark contract* (P11c) out of GLP-0003 into the downstream plan; keep only the handoff (F10). As written, D0007 declares the work out of scope while a checkpoint does the work.
- **D0008** — fine as is; no change.

## 6. Suggested edits (patch guidance)

- **Plan 22-26:** replace "small in source size" framing. Add a sentence: the 1,498-LOC spine excludes dag-cbor/CID/multiformats, identity+keystore signing, IPFS block storage, and encryption, which OrbitDB imports and this port must re-implement or bind — the real surface is several times larger.
- **Plan 66-76 (Inputs):** add `databases/database.js`, `databases/documents.js`, `identities/identity.js`, `identities/providers/*`, `access-controllers/*`, and `storage/{composed,ipfs-block,memory,lru}.js`. Note the IPLD/multiformats and crypto dependency surface explicitly.
- **Plan 232-243 (Transport boundary):** add two concepts — (a) content-addressed entry fetch (get-by-CID) and (b) a stated delivery contract for head announcements (lossy/best-effort to mirror gossipsub).
- **Plan 309 / Checkpoints 64-67 (P04):** prepend an explicit document-model decision and a delivery-order-independent **convergence invariant** as a named gate; make P04 model-only, move convergence to a post-P06 checkpoint.
- **Checkpoints 70 (P05c):** rename/rescope from "missing-entry request/response" to match the chosen replication model (F1); if request/response is kept, add a delta note that it diverges from OrbitDB.
- **Checkpoints 79-82 (P08):** add a concrete memory/time budget to P08c and an explicit "compressed run reproduces sampled-exact invariants at the same seed" equivalence test to P08b; rewrite P08a's unfalsifiable stop condition.
- **Add a Checkpoints row** under P00 or a new sub-phase for formal-verification scoping + first artifact (closes ROR-017).
- **SecurityAuditPlan 73-85:** add fixture-corpus integrity (hash/provenance pinning) and Node-runner lockfile/provenance to the artifact list.
- **NetworkingRuntimeStudy:** add a note that the downstream workload is a libp2p swarm, not raw TCP, and stop anchoring per-connection cost on the 64-byte task-metadata figure.
