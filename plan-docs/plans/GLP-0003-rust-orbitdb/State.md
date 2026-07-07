# State

Plan: `GLP-0003`
Status: deferred (long-term; superseded near-term by an off-the-shelf Go path)
Owner: maintainer (sole owner across all workstreams)
Reached: `P03b` (entry/clock/conflict/append/join/traverse/refs + storage seam),
15 conformance/property tests green against `@orbitdb/core@4.0.0`, all pushed;
submodule pinned. Paused before P03c/h (crash contract) and P04+.

## Deferral rationale

Rust is judged the correct *long-term* substrate (in-process Python via PyO3 to
kill the Node/Python sidecar fd-hack, determinism, simulator-first, clean
seams), but building the full Rust transport/replication stack now is more
energy than warranted when a production-worthy Go path is available off the
shelf. Near-term GripLab sharing moves on Go (go-orbit-db + boxo/libp2p) and/or
JS. **Revisit trigger:** when the Python↔engine boundary friction (the sidecar
hack) becomes painful enough, or the deterministic/simulator value is wanted,
`rust-orbitdb-py` earns its keep — resume from `P03b`.

The completed spike stands as a documented feasibility result: Rust can match
OrbitDB byte-for-byte (entry encode/CID/secp256k1 sign+verify, clock/conflict,
append/join/traverse/refs), and the requirements it surfaced are
language-independent (see the share-semantics work feeding the Go path).

## Summary

This plan defines the work to create `rust-orbitdb` as a first-party Rust
submodule. The design is simulator-first, keeps libp2p behind adapter and
in-memory-libp2p-simulator boundaries, includes multi-writer document CRDT
semantics, and includes security/audit artifacts as release gates.

## Blockers

- Create the `git@github.com:owebeeone/rust-orbitdb.git` remote with an empty
  Rust workspace commit before root submodule addition (`P00b`).

## Next Actions

1. Plan and all decisions accepted (`P00a` repo-contract satisfied).
2. Create the `git@github.com:owebeeone/rust-orbitdb.git` remote.
3. Add the root submodule and pin the initial Rust workspace commit (`P00b`).
4. Roll `P00c`-`P00j` bootstrap gates, then enter `P01`.
5. Start simulator architecture review before implementation of `P06`.

## Owned Paths

- `plan-docs/plans/GLP-0003-rust-orbitdb/`
- Future: `rust-orbitdb/`
- Future: root `.gitmodules` entry for `rust-orbitdb`

## Paths To Avoid Unless Assigned

- Existing submodules except through explicitly assigned adapter integration
  tasks.
- `third-party/` vendored sources, except read-only fixture generation and
  inspection.
- Downstream product contracts; generic application scenarios MUST remain
  product-agnostic in this plan.
