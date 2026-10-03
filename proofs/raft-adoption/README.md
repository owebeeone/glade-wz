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


Q3a/Q3b private membership and snapshot implementation is **accepted at root
`468fa7725bcd7cf025b4889c37f697f2bfc3d2d2` after Code/State GO/GO**. See the
[implementation evidence](../../dev-docs/GladeRaftQ3ImplementationEvidence.md) and
[exact tuple/review ledger](../../dev-docs/GladeRaftQualification-ReviewCycle.md).
Actual V2 stores and RawNodes implement the reviewed injected contracts; all
nineteen original configuration/snapshot consumer cases are GREEN and remain
selected. The default workspace run passes 121 tests and explicitly ignores
only two Q2 tier cases plus the two externally supervised process workers.
The [contract evidence](../../dev-docs/GladeRaftQ3ContractEvidence.md) preserves
historical compiling RED scaffold results; refusing scaffold types are not the
current concrete test providers. Q2 format is never silently upgraded.

```sh
# Carrier-free contract compiler/fixture witnesses.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api
# Actual injected V2/RawNode consumers; all nineteen pass.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot
# Owning membership, recovery, snapshot, semantic and fault cases.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib q3
# Existing Application/shared fixture compatibility.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test q3_fixture_compatibility
# Q3 actual SIGKILL: use the exact executable Cargo reports.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test q3_process_crash --no-run
python3 proofs/raft-adoption/process-crash-q3.py --self-test
python3 proofs/raft-adoption/process-crash-q3.py --worker "$q3_worker_path"
```

Supply the compatible PROTOC above for carrier commands and set `q3_worker_path`
to the Cargo-reported executable. The runner performs ACK, joint-before-apply
and snapshot-before-apply SIGKILL cuts and compares fresh recovery with complete
parent-held APP/CONFIG/ENTRY originals. The worker alone is not a durability test.

Accepted allocations, normal/dev edges and the empty process-global exception
list remain reviewed and enforced by `check.sh`. The profile retains full
original application history and append-only physical journals, a 16 MiB refusal
boundary and rollback detection requiring an independent trusted floor. Manual
campaigns and process-crash evidence do not certify automatic elections, power
loss, independent physical failure domains or production authority/transport.
Q4 Glade integration, legacy/effect exclusion, production selection and
activation remain open.
