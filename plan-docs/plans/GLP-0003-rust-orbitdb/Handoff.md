# Handoff

Plan: `GLP-0003`
Status: deferred (long-term; near-term GripLab sharing moves on an off-the-shelf
Go/JS path — see State.md deferral rationale)

## Current State

Implemented and pushed through `P03b`: rust-orbitdb-core (entry encode/CID/
secp256k1 sign+verify, clock/conflict ordering, Log append/join/traverse/refs/
values) + an EntryStore storage seam, with 15 conformance/property tests green
against `@orbitdb/core@4.0.0`. Submodule pinned in the root repo. Paused before
P03c/h (crash/atomicity contract — a define-it decision, no JS oracle) and P04+.

Deferred because Rust is the right *long-term* substrate but the full transport/
replication build is more energy than warranted now; a production-worthy Go path
is available off the shelf. Resume from `P03b` when the revisit trigger fires
(Python↔engine sidecar friction, or determinism/simulator value wanted).

## Next Owner Should

1. Create the `git@github.com:owebeeone/rust-orbitdb.git` remote with an empty
   Rust workspace commit.
2. Add and pin the root submodule (`P00b`); keep root changes to `.gitmodules`
   and the submodule pointer.
3. Roll the remaining `P00` bootstrap gates (`P00c`-`P00j`) in the submodule.
4. Resolve the residual questions below as their owning checkpoints are reached.

## Residual Questions

- Should the first Rust workspace publish crates publicly, stay private, or use
  unpublished workspace crates only?
- Should submodule CI be independent GitHub Actions, root-triggered, or both?
- Are the named hot-path complexity bounds in `P00h` (D0015) complete, and is
  any one of them at risk of a superlinear implementation?
- Which formal/model-checking tool should be selected first for invariant work?
- What is the minimum useful scale for the in-memory libp2p semantics simulator?
- Should D0014's custom get-by-CID request-response transport remain the first
  adapter path, or should an external Rust IPFS/bitswap stack be selected before
  P10?
