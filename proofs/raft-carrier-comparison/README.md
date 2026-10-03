# Private Q4-B0 compiler / behavioral RED scaffold

**DRAFT only.** All election operations in both named provider crates return
`NotQualified`. No actual carrier, runtime, source adaptation, election, disk,
production dependency or authority is present. See
[the allocation](../../dev-docs/GladeRaftB0Allocation.md) for proposed signatures,
upstream engineering feature/timing profile and still-open proof obligations.

The API is std-only. The spec normally depends only on API; its current std-only
refusing providers are dev dependencies. Their future engine replacement would
contaminate package unit loops through those dev edges: a reviewed runner/test
allocation must preserve the common consumers while moving real composition out
before installing engines. Source, future scheduling and queued-RPC modules here
are pure controlled test scaffolding, not claimed implementation providers.

From the adopting workspace root, compiler/fixture/oracle witnesses (19 passing):

```sh
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-api --test public_contract
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-raft-rs -p glade-carrier-openraft --test compiler_contract
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-spec --test constructor_contract --test fixture_plumbing --test oracle --test rpc_reply
```

Ordinary behavioral RED command (20 failing assertions; no ignored/filtered cases):

```sh
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-spec --test b0_election
```

Structural/source/global/format and lint commands (passing, separately from RED):

```sh
proofs/raft-carrier-comparison/check.sh
cargo clippy --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-api -p glade-carrier-spec -p glade-carrier-raft-rs -p glade-carrier-openraft --all-targets -- -D warnings
```

[Measurements](evidence/measurements.json) record exact commands, exit statuses,
machine/toolchain, execution-only, warm build-plus-RED and fresh-target build.
[Behavior assertions](evidence/behavior-red.log),
[structural results](evidence/structural.log), [lint](evidence/clippy.log),
[API](evidence/api-witness.log), [provider compiler](evidence/provider-witness.log)
and [fixture/compiler/oracle](evidence/spec-witness.log) are filed separately.
The correlated-reply fixture supplement has its own observed [RED](evidence/rpc-red.log)
and [GREEN](evidence/rpc-green.log) evidence; it implements only controlled fixture mechanics.
`measure.py` replays this checkpoint with the RED failure recorded as failure;
recognizing its expected status does not make behavior pass or qualify a carrier.

Future real adapters MUST reuse the same exported scenario functions. The refusing
command remains explicit and RED; a separate reviewed composition runs real
adapters without changing these expected invariants or selecting fewer scenarios.
