# Review-48-2

Second pass over the amended `GLP-0003` plan. I re-read every file and re-verified
claims against `third-party/orbitdb/src/` and `third-party/rust-libp2p/`. This
review covers (A) resolution status of the 13 findings in `Review-48.md`, then
(B) new or residual issues the revision surfaced, (C) remaining blocking
questions. No general praise; only what still needs work.

---

## A. Resolution of prior findings

All 13 prior findings were addressed. I verified each against the amended text,
not just the changelog.

| # | Prior finding | Status | Where resolved (verified) |
| --- | --- | --- | --- |
| F1 | Sync catch-up protocol doesn't exist in OrbitDB | **Resolved** | `D0011`, `ROR-019`, transport boundary (Plan 280), `content-addressed-fetch` (P05d), Risks row "Sync implementation invents a protocol". |
| F2 | "Document CRDT" conflated with LWW store | **Resolved** | `ROR-012` rewritten, `D0009`, "Document Baseline" (Plan 118-139), split `doc-compat-model`/`doc-crdt-model` (P04a/b), Risks row. |
| F3 | Difficulty understated by LOC framing | **Resolved** | Plan 22-29 reframed, `D0013`, `ROR-020`, `identity-model` (P02d), Inputs expanded (Plan 76-87). |
| F4 | Formal verification had no producing checkpoint | **Resolved** | `formal-verification-scope` (P00g); SecurityAuditPlan "Roll-Build Security Gates". |
| F5 | "Same sim N=2→million" was a fork | **Resolved** | `compressed-exact-bridge` (P08b), "Exact-To-Compressed Bridge" (SimArch 165-187), `ROR-008` rewritten. |
| F6 | No million-scale budget | **Resolved** | `scale-budget-contract` (P00h), with explicit "blocking P08" escape. |
| F7 | Head-announcement delivery semantics unspecified | **Resolved** | `D0012`, transport boundary (Plan 286-290), `fake-transport-delivery-contract` (P05b). |
| F8 | Oversized checkpoints | **Resolved** | All five split: dag-cbor/CID (P02b/c), sim kernel (P06a/b), substrate (P05a/b), JS runner (P07a-c), doc conflicts (P04c/d). |
| F9 | P04 convergence inverted dependency on sim | **Resolved** | `doc-crdt-exact-replica` (P04f) "without direct simulator dependency"; sim convergence moved to `direct-sim-doc-convergence` (P06f). |
| F10 | C10k scope bloat / raw-TCP vs libp2p | **Resolved** | `host-c10k-contract` removed; libp2p-host profile + lower-bound caveat (NetworkingRuntimeStudy 10-13, 111); 64-byte caveat (63-65). |
| F11 | Node-runner supply chain | **Resolved** | SecurityAuditPlan 87-89, 109; `node-plugin-load` (P07a) supply-chain stop condition. |
| F12 | Fixture corpus integrity | **Resolved** | "fixture corpus provenance manifest with content hashes" (SecurityAuditPlan 89), fuzz target 71. |
| F13 | Entry `v:2` versioning | **Resolved** | `ROR-003` now names entry-version compat and historical-version deltas. |

Also resolved beyond the original list: `D0004` now states its JS comparison
surface explicitly (closes the "D0004 oversells" point), `D0007` bans all async
runtimes and requires injected clocks/timers, and `D0010` bans libp2p *vocabulary*
(not just types) in semantic APIs — which is the stronger form of the hidden-coupling
concern.

---

## B. New / residual findings

The fix for F1 (adopt OrbitDB's real model: head announcement + content-addressed
block fetch) is correct, but it surfaced an implementation gap that the old
invented-request/response framing hid.

### B1 — High: the real libp2p adapter has no block-exchange protocol, and the plan assigns it to no crate.

`content-addressed-fetch` (P05d) and `real-libp2p-content-fetch` (P10c) require
"missing transitive entries resolved by get-by-CID/storage fetch ... against real
rust-libp2p." But:

- `rust-libp2p` ships **no bitswap / block-exchange protocol** — verified:
  `third-party/rust-libp2p/protocols/` contains autonat, dcutr, floodsub,
  gossipsub, identify, kad, mdns, perf, ping, relay, rendezvous, request-response,
  stream, upnp. No bitswap.
- OrbitDB's `IPFSBlockStorage` does not implement block exchange either — it
  delegates to a Helia/IPFS instance (`storage/ipfs-block.js:29` requires
  `params.ipfs`); bitswap lives in Helia, outside libp2p.

So for the real adapter, get-by-CID over the wire must be **implemented**, by one of:
(a) a custom get-by-CID over libp2p `request-response`, or
(b) a Rust bitswap/IPFS stack (e.g. iroh/beetle) as a new dependency.

Neither is scoped. `D0011` pushes fetch "through the storage/substrate boundary,"
but `rust-orbitdb-store` is I/O-free and the substrate is neutral — so no crate
owns the networked block fetch. The crate table (Plan 212-225) gives
`rust-orbitdb-libp2p` "streams/request-response/pubsub/rendezvous" but never a
block-exchange responsibility.

Two consequences worth a decision:
- **Option (a) re-introduces a request/response protocol** — exactly what `D0011`
  says "MUST NOT be treated as JS OrbitDB-compatible." The plan needs to state that
  compatibility is asserted at the **block/CID layer**, and the wire block-exchange
  protocol (bitswap vs custom RR vs Helia-equivalent) is an implementation choice
  *not* required to byte-match JS's bitswap. Without that statement, P10c's
  "compatibility" claim collides with D0011's exclusion.
- **Option (b) is a large unlisted dependency** with its own audit/SBOM surface.

Suggested edits:
- Add a crate-table responsibility: `rust-orbitdb-libp2p` owns "content-addressed
  block fetch wire protocol (bitswap-equivalent or custom RR over request-response)."
- Add a decision (D0014) recording the wire block-exchange choice and stating that
  JS compatibility is at the CID/block layer, not the wire protocol.
- `D0011` para 2: clarify that a get-by-CID request/response used *only as the
  block-fetch transport* is not the banned "application-level catch-up" — the ban
  targets head/entry catch-up semantics, not block retrieval.

### B2 — Medium: convergence claim lacks an explicit data-availability assumption.

Plan 137-139 and the CRDT gate assert replicas "converge ... regardless of delivery
order, provided the modeled delivery assumptions are eventually satisfied." With the
now-explicit content-addressed model, convergence also requires that **every
referenced block stays fetchable from at least one reachable peer**. The plan added
a typed `missing block` error (Plan 284) and a "missing content-addressed block
fetch" bridge invariant (SimArch 182) — but the convergence *assumption set* never
names block/data availability. As written, a `missing block` (last holder departs
before others fetch) silently falsifies the convergence claim while every stated
assumption still holds.

Suggested edit: add to the Document Baseline convergence statement and to
SecurityAuditPlan "Proof-Oriented Correctness" an explicit modeled assumption:
"every entry/block referenced by a delivered head remains retrievable from some
reachable peer or storage tier within the modeled fetch window." The simulator
should have a fault that violates this and asserts the system reports a typed
unavailability rather than diverging silently.

### B3 — Medium: `ROR-002`'s per-crate `cargo tree` libp2p-absence gate has no owning checkpoint.

`ROR-002` acceptance is "`cargo tree` for those crates contains no libp2p." The
revision strengthened *type/vocabulary* leak detection (P05a stop condition, D0010)
but no roll-build checkpoint runs the **dependency-tree** audit. `io-boundary-audit`
(P11a) covers I/O, not the libp2p dependency graph. Note also the new verification
block runs `cargo clippy --all-features` (Plan 388), which pulls libp2p into the
whole-workspace graph — the ROR-002 check must therefore be explicitly per-crate
(`cargo tree -p <crate>` / `cargo tree -e no-dev -i libp2p`), not against the
all-features workspace.

Suggested edit: add a `dependency-boundary-audit` checkpoint (docs + small script)
near P05a or P11a that runs the per-crate `cargo tree` / `cargo deny` ban for the
eight semantic crates, and a `cargo deny` `[bans]` rule for libp2p in those crates.

### B4 — Low: `D0009` mixes observed-remove and total-order-LWW vocabularies.

`D0009` proposes "an observed-remove map ... with deterministic ordering from the
accepted oplog conflict rules." OR-map semantics (causal add/remove tags) and
total-order LWW-by-clock (OrbitDB's `conflict-resolution.js` total order, which
*throws* on a zero compare) are different conflict models — an OR-map's remove
needs to observe specific adds, whereas LWW-by-clock just takes the max under the
total order. This is appropriately deferred to `doc-crdt-model` (P04b) so it does
not block, but the decision text should not imply both at once. Suggested edit:
in `D0009`, state which one is the proposed default (the OrbitDB-faithful path is
total-order LWW registers + tombstones; OR-semantics is the *extension*) and let
P04b confirm.

### Categories with no new issue
- Inputs: all newly listed paths exist (verified `orbitdb.js`, `database.js`,
  `manifest-store.js`, `documents.js`, `identities/*`, `key-store.js`,
  `access-controllers/*`, `storage/*`, both document test files). Only identity
  provider is `publickey.js`, which usefully bounds the `identity-model` (P02d)
  scope.
- Checkpoint sizing: the splits are sound; no remaining checkpoint obviously
  exceeds the reasoning budget. P00 now carries 9 sub-checkpoints (a–i) — front-
  loaded but each is small and independently gated; not an issue.
- Product naming: still clean; `generic-session-negative-names` (P13a) now makes it
  an executable gate.

---

## C. Open questions still blocking implementation

1. **(B1)** What implements over-the-wire block fetch in the real adapter —
   custom get-by-CID over libp2p `request-response`, or an external Rust IPFS/bitswap
   dependency? This blocks P10c and the crate boundary for `rust-orbitdb-libp2p`.
2. **(B1)** Is OrbitDB "compatibility" asserted at the block/CID layer only, with the
   wire block-exchange protocol explicitly out of the compatibility claim? Needed to
   reconcile P10c with D0011.
3. **(B2)** Is block/data availability a named modeled assumption of the convergence
   proof, and what fault exercises its violation?
4. **(B4)** Which document conflict model is the rust-orbitdb default — total-order
   LWW registers (OrbitDB-faithful) or observed-remove — with the other as the
   recorded extension?

These are narrower than the first round. B1 is the one that should be settled before
P05d/P10 are scheduled; the rest can be resolved inside their existing model
checkpoints (P04b, P00g/threat-model) as long as the decisions are recorded.
