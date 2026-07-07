# Security Audit Plan

Plan: `GLP-0003`

## Goal

Data security is release-critical. The project MUST be auditable and SHOULD
pursue proof-oriented correctness for scoped invariants.

This plan does not claim whole-system proof. It defines auditable artifacts and
specific invariants that can be tested, fuzzed, model-checked, or formally
verified where practical.

## Threat Model

The first security checkpoint MUST define:

- trusted and untrusted boundaries
- local process assumptions
- remote peer assumptions
- identity and authorization model
- key generation, storage, rotation, and compromise assumptions
- storage corruption assumptions
- block/data availability assumptions for content-addressed fetch
- replay, duplicate, reorder, delay, and partition assumptions
- malicious payload and malformed CID/block assumptions
- dependency and supply-chain assumptions
- fixture corpus provenance and hash assumptions
- Node.js runner package and lockfile assumptions

## Security Invariants

Initial invariant candidates:

- canonical entry bytes are stable across supported platforms
- entry CID matches canonical bytes
- tampered payload, key, identity, signature, or CID is rejected
- malformed dag-cbor is rejected without panic
- conflict ordering is deterministic and never returns an invalid zero compare
- append-only history cannot be rewritten through join
- missing dependencies are not silently accepted
- storage crash recovery either repairs by replay or reports a typed error
- document CRDT convergence holds under modeled delivery and data-availability
  assumptions
- missing or unreachable referenced blocks are reported as typed
  unavailability, not silent divergence
- unauthorized operations are refused before state mutation
- simulator faults cannot hide invalid state transitions
- named hot paths (append/join/traverse/heads/conflict-order/head-exchange/
  doc-index/sim-loop/compressed-state) stay sub-quadratic, and live state stays
  O(active or sampled), not O(total inactive sessions) (see D0015)

## Testing And Analysis Layers

| Layer | Purpose | Tools / shape |
| --- | --- | --- |
| Unit/regression tests | Known edge cases and fixed failures | `cargo test`, fixture corpus |
| Property tests | Randomized state-space pressure | `proptest`, deterministic seeds |
| Fuzzing | Malformed byte input and parser robustness | `cargo fuzz` for decode, CID, signature, frames |
| Model tests | State-machine invariants | direct simulator, small exhaustive schedules |
| Concurrency tests | Await-boundary and async interleavings | deterministic scheduler/testkit; consider `loom` for small shared-state units |
| Dependency audit | Known vulnerable dependencies | `cargo audit`, `cargo deny`, npm audit for Node runner |
| Supply-chain review | License, source, and update policy | SBOM, lockfiles, pinned toolchain |
| Unsafe review | Safety boundary review | `#![forbid(unsafe_code)]` by default; isolated unsafe if approved |
| Formal/bounded verification | Specific invariants where feasible | TLA+/PlusCal for sync model; Kani/Prusti/Creusot candidates for small pure functions |

## Fuzz Targets

Minimum fuzz targets:

- entry decode
- dag-cbor canonicalization
- CID parse/encode
- signature verify input reconstruction
- sync frame decode
- simulator scenario parser
- document CRDT operation decode
- fixture corpus loader and provenance manifest
- Node.js scenario trace parser

Fuzz failures MUST become regression tests.

## Audit Artifacts

Before release, the repo SHOULD contain:

- `SECURITY.md`
- threat model document
- invariant list
- fuzz target list and latest run notes
- dependency audit output or CI gate
- SBOM or dependency inventory
- pinned Cargo lockfile and toolchain provenance
- pinned Node.js package manager version and lockfile for the JS runner
- npm dependency audit output or CI gate for the JS runner
- fixture corpus provenance manifest with content hashes
- vendored source inventory for identity, access-control, storage, and
  encryption compatibility surfaces
- unsafe-code policy
- key/signature handling review notes
- model-checking or formal-methods notes for accepted invariants
- release-blocking security checklist

## Roll-Build Security Gates

Security MUST be phased through the roll-build, not left as a final review:

- `P00d`: threat model skeleton
- `P00e`: invariant list
- `P00f`: audit/fuzz/SBOM placeholders
- `P00g`: first formal or bounded-verification target selected
- `P03i`: signature/key/canonicalization review
- `P04`: document CRDT invariants added to the invariant list
- `P06`: simulator fault model reviewed against invariant list
- `P08`: compressed simulation proves named invariant bridge
- `P07`: Node runner lockfile, provenance, and trace integrity reviewed
- before release: all high/critical audit findings resolved or explicitly
  accepted

## Proof-Oriented Correctness

Accepted proof scope:

- pure encoding/decoding invariants
- conflict ordering invariants
- append/join state transition invariants
- storage recovery state machine
- document CRDT convergence under modeled delivery and data-availability
  assumptions
- sync delivery/idempotence under modeled delivery and data-availability
  assumptions
- data availability for entries/blocks referenced by delivered heads, where the
  modeled assumption is that each referenced block remains retrievable from at
  least one reachable peer or storage tier within the modeled fetch window

Rejected proof scope unless a later plan narrows it:

- whole system across arbitrary OS, runtime, network, and dependency behavior
- cryptographic primitive correctness beyond using reviewed libraries correctly
- real-world network liveness under unbounded adversarial behavior
- convergence when all holders of a referenced block disappear before fetch and
  no durable storage tier retains it

## Release Gate

A release MUST NOT proceed while any of these are unresolved:

- known critical/high vulnerability in direct dependencies without mitigation
- failing fuzz regression
- failing signature/canonicalization fixture
- unreviewed unsafe code in core/store/sync/document/sim
- unaudited key-handling change
- undocumented protocol/schema change
- simulator or conformance failure that affects data integrity
