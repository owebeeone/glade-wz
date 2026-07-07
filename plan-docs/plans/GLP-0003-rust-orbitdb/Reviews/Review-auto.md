# Review: Auto

Status: received and patched into plan docs

## Findings

### Critical

- No document CRDT input baseline: `Plan.md` listed oplog JS sources, but
  `ROR-012` and `P04` had no vendored document sources, CRDT choice, or fixture
  checkpoint.
- `P04d` vs `P06` ordering: `doc-crdt-sim-convergence` needed direct sim before
  the direct simulator checkpoint existed.
- D0004 pressure arrived late: harness falsification mainly appeared at `P07c`,
  allowing simulator divergence until late.
- Million-scale compression was not linked to small-N exact invariants.

### High

- Substrate API was libp2p-shaped while only banning libp2p types.
- Document CRDT gates were weaker than oplog gates.
- Security/proof artifacts were not phased deeply enough into roll-build.
- Million-scale budget was undefined.
- Trace compare rules for JS vs Rust were missing.
- Several checkpoints were still multi-domain despite the 500 LOC guidance.
- No dedicated scenario DSL / testkit checkpoint existed before sim and JS work.

### Medium

- Product-shaped scenario vocabulary should be more generic.
- "Provider bindings" was unexplained.
- C10k study is docs-only while completion wording implied implementation.
- `--all-features` CI needs care alongside I/O boundary audits.
- `ROR-002` and `D0002` crate lists should match.
- Iterator/storage semantics should be defined before late `P03e`/`P03f`.

## Requested Edits

- Add document source baseline and CRDT authority decision.
- Fix checkpoint ordering and split broad checkpoints.
- Add a versioned scenario DSL/testkit early.
- Add early JS OrbitDB smoke and normative trace comparison.
- Add exact-to-compressed invariant bridge for million-scale simulation.
- Neutralize substrate trait vocabulary.
- Phase security gates through the roll-build.
- Add negative downstream-name tests for generic application scenarios.
