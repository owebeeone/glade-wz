# Raft adoption qualification fixture

Standalone experiment, outside Glade production dependency paths. Start with
[the plan](../../dev-docs/GladeRaftQualificationPlan.md) and
[the contract](../../dev-docs/GladeRaftAdoptionContract.md).

Initial Q1a profile: known fixed group, numeric trusted identity/policy fixtures,
complete small payloads, memory-retained logs, explicit messages and manual
campaigns. Receipts are fixture outcomes, never Glade durable receipts. No disk,
automatic election, real cryptography, real network or independent-machine claim.

The raft-proto build requires a **3.x protoc** (its build helper rejects newer
major versions). This arm64 machine uses protobuf-build 0.14.1's bundled macOS
x86-64 `libprotoc 3.9.0`, verified runnable through the machine's translation
support. Supply that executable explicitly; an absent tool or modern v32.1
compiler produced build failures, not behavioral RED. No global installation or
build-helper patch is part of the proof.

From the Glade workspace root, with the compatible compiler supplied:

```sh
export PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml
proofs/raft-adoption/check.sh
cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --all-targets -- -D warnings
```

The local gate explicitly adopts the existing architecture and process-global
checkers with its own inventory/empty allowlist. Token-aware source checking
rejects all conditional attributes in this fixture; rustfmt parses braced Rust
control flow. Disabled branches are inspected without compiling each platform.
Dependency source compliance is separately audited: raft-rs's timeout RNG remains
an open production gate. No upstream or legacy cfg migration is claimed.

Results: [qualification evidence](../../dev-docs/GladeRaftQualificationEvidence.md)
and [review ledger](../../dev-docs/GladeRaftQualification-ReviewCycle.md).
Movement readiness is a private trusted fixture envelope retained in the log;
first-time public apply without it refuses movement. Exact replay recovers retained results. Production ports/certificates remain
unqualified. The public resource observer is trusted test inspection, not a client
read endpoint; local reply/outcome accessors enforce fixture disclosure permission.
