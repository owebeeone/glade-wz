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

Q2 now adds an injected std-only persistence contract and real disk implementation.
See [the persistence contract](../../dev-docs/GladeRaftPersistenceContract.md).
The journal retains complete entries/HardState/binding; replay reconstructs committed
outcomes, payloads, movement fences, policy and tombstones. It refuses torn/invalid
history, holds a file lock and calls sync_all before publication. This is the
named APFS/process-crash experiment; power-loss and production quorum guarantees
remain open. Whole valid old-file rollback needs an independently trusted floor.

Keep the test tiers explicit (manifest and PROTOC above):

```sh
# Small contract/model loop, no Raft build or disk I/O.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-durability-api
# Existing memory driver/application loop; real-disk LightReady is ignored here.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib --test qualification
# Real disk/host recovery tiers.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-disk
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test recovery
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib q2_real -- --ignored
# External process-kill tier: --no-run reports the executable path to supply.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test process_crash --no-run
python3 proofs/raft-adoption/process-crash.py --self-test
# Set q2_worker_path to the exact executable Cargo just reported.
python3 proofs/raft-adoption/process-crash.py --worker "$q2_worker_path"
```

The default workspace run is broad qualification, not the pure edit loop. It
intentionally ignores the disk LightReady/terminal-term units and process worker: both MUST be
executed through their separate commands for Q2 acceptance. The runner kills only
its owned workers and uses disposable paths plus an explicit minimal environment.
It compares complete externally retained original receipts after fresh-process
lookup/exact retry at the acknowledged and durable-before-apply cuts. Measurements
and exact reviewed source belong in the evidence/ledger, not inferred budgets.


Q3 is now a **DRAFT contract-review package**, not an implementation. See
[configuration/snapshot contract](../../dev-docs/GladeRaftConfigurationSnapshotContract.md)
and [compilation/RED evidence](../../dev-docs/GladeRaftQ3ContractEvidence.md).
The new API's dyn consumer is GREEN; the spec scaffold intentionally returns
`NotQualified`, so all 15 `configuration_snapshot` behaviors are RED. No fake
model is presented as a working membership or snapshot adapter. The default
whole-workspace test command therefore intentionally fails Q3 during this gate;
no `default-members` or hidden exclusions are used. Select existing Q2 packages
explicitly when checking their unaffected regression tier.

```sh
# Carrier-free contract compiler witness.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api
# Compiles successfully, then deliberately fails behaviorally at NotQualified.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot
# Existing accepted packages, with compatible PROTOC supplied as above.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-api -p glade-raft-durability-api -p glade-raft-disk -p glade-raft-adoption-proof
```

The expanded architecture inventory and source roots are **proposals for this
Q3 review**; dependency allowlists/roles of existing packages and the empty
process-global exception list are unchanged. Physical V2 journal, actual
ConfChangeV2/RawNode snapshot lifecycle and SIGKILL matrix remain implementation
exit witnesses. Q2 format is never silently upgraded or described as dynamic.
