# Glade Raft Q3 contract compilation and RED evidence

Date: 2026-10-03. Status: **DRAFT contract-review checkpoint, not implementation acceptance**. Controlling scope is [GladeRaftConfigurationSnapshotContract.md](GladeRaftConfigurationSnapshotContract.md) and [the qualification plan](GladeRaftQualificationPlan.md). The lane owner binds the exact committed tuple and independent review reports in [the review ledger](GladeRaftQualification-ReviewCycle.md).

## Executed object and limits

New `q3-api` and `q3-spec` are std-only workspace packages. The former depends only on existing application/durable-entry contracts; the latter depends only on q3-api. `UnqualifiedStore` and `UnqualifiedSession` implement the compiler-required dyn-compatible traits but return `Error::NotQualified` for every operation. No membership, V2 storage, snapshot codec, real Raft configuration/snapshot driver or process-crash implementation has been built. An intentionally refusing provider is not a contract-faithful working implementation; its role is explicit behavioral RED.

One API compiler consumer passed. All **15 behavioral specifications compiled successfully and then failed at NotQualified**, exit 101. Assertions after the first refusal are compiling specifications only, not executed safety evidence. These cases are reusable in the real implementations; storage's arbitrary byte fixtures are only structural roundtrip tests and cannot establish application semantic replay or physical durability. The contract's implementation exit matrix additionally requires actual V2 lifecycle/faults/rollback/bounds, valid-format cross-map corruption, complete prefix/suffix/carrier behavior and externally retained original receipt/Entry process-crash oracles. Those remain unimplemented and unqualified.

No `default-members`, hidden skips or expectation-inversion was added: a default whole-workspace Cargo run now intentionally fails on Q3 during this contract-review checkpoint. Normal selected Q2 regression targets remain passing. Q3 acceptance requires GREEN against actual providers; changing the test assertions to expect NotQualified would defeat this gate.

## Exact commands and results

From workspace root:

```sh
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api -p glade-raft-q3-spec --no-run
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot
```

Compile command: PASS, no import/type/build error. API target: **1 PASS** (`dyn_consumers_compile_without_carrier_or_disk_types`). Spec target: **0 PASS, 15 RED**, all NotQualified. Tests are unignored, deterministic and carrier/filesystem-free.

| RED test | Initial failure |
| --- | --- |
| `v2_checkpoint_exact_roundtrip_and_immutable_same_cut` | `explicit V2 genesis: NotQualified` |
| `v2_suffix_replacement_bounds_and_commit_cut` | `complete prefix: NotQualified` |
| `v2_historical_same_term_vote_survives_removed_voter` | `vote 3 at term 2: NotQualified` |
| `membership_authorized_namespace_and_exact_intent_retry` | `known configured scope: NotQualified` |
| `learner_unavailable_cannot_promote_or_supply_quorum` | `known configured scope: NotQualified` |
| `joint_outgoing_majority_alone_cannot_accept` | `known configured scope: NotQualified` |
| `joint_incoming_majority_alone_cannot_accept` | `known configured scope: NotQualified` |
| `joint_restart_and_lost_configuration_reply_are_recoverable` | `known configured scope: NotQualified` |
| `removing_current_resource_home_refuses_without_rehoming` | `create: NotQualified` |
| `snapshot_original_receipts_commands_and_exact_index_replay_survive` | `create: NotQualified` |
| `snapshot_policy_retirement_and_name_reservation_survive` | `create: NotQualified` |
| `snapshot_valid_foreign_binding_corrupt_state_and_capacity_refuse` | `create: NotQualified` |
| `snapshot_movement_preserves_complete_readiness_and_generation_fence` | `create: NotQualified` |
| `configuration_eligibility_lost_after_admission_retains_exact_refusal` | `create: NotQualified` |
| `compacted_snapshot_restores_learner_before_joint_promotion` | `create: NotQualified` |

The last case requires a freshly regenerated snapshot whose configuration includes learner 4, a later ordinary committed suffix, observed durable snapshot cut, verified complete applied catch-up, promotion and joint restart with original exact retries. It cannot pass by assigning counters or rewriting an old snapshot's ConfState. The refusal case logs an old readiness envelope with a queued earlier application command; the applied predecessor-cut mismatch MUST produce the same retained refusal on every replica, not depend on local availability.

Existing accepted regression commands, with the pinned compatible protoc supplied (no global tool installation):

```sh
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-api -p glade-raft-durability-api -p glade-raft-disk -p glade-raft-adoption-proof
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib q2_real -- --ignored
proofs/raft-adoption/check.sh
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --all-targets -- -D warnings
```

Selected accepted packages: **49 PASS**, two real-disk unit cases and process worker explicitly ignored as designed. Separate `q2_real` command: **2 PASS**, preserving the supported LightReady lifecycle and reserved-term publication fence. Architecture/declared dependencies/trait/conformance gate: PASS. Expanded owned-source scan: **19 files, zero allowlisted entries**; token-aware explicit-boundary scan and rustfmt: PASS. All-target Clippy with warnings denied: PASS. The checks include the newly proposed roles/edges; they do not approve those roles semantically or certify third-party source/global behavior.

Existing Q2 actual process-kill evidence remains accepted at its prior pinned implementation tuple; it was not rerun or extended into Q3 by this source-only contract task. No Q2 implementation file changed. No member/production interface, dependency, canonical format, process-global exception or profile limit was loosened.

## Measured compilation and execution

Machine: Apple M3 Pro arm64; macOS 26.6.2 build 25G83; Rust/Cargo 1.96.0. Measured with Python monotonic `perf_counter`, one sample per named command. Cold uses a fresh disposable CARGO_TARGET_DIR with existing local registry and warm OS caches; it is fresh Cargo artifacts, not a fresh machine. Temporary cold artifacts were removed. Warm commands include Cargo startup and selected compilation/linking/doctest overhead. Prebuilt execution uses only the named integration-test executable, without Cargo/build/doctest overhead. These numbers are observations, not a latency guarantee or a production budget.

| Tier | Wall seconds | Result |
| --- | ---: | --- |
| Cold combined q3-api/q3-spec `--no-run` | 0.902 | Compiles |
| Warm combined `--no-run` | 0.033 | Compiles |
| Warm q3-api build/test/doctest command | 0.949 | 1 PASS |
| Warm q3-spec behavioral command | 0.416 | 15 RED, exit 101 |
| Prebuilt API consumer integration execution | 0.004 | 1 PASS |
| Prebuilt spec integration execution | 0.004 | 15 RED, exit 101 |

Prebuilt executable paths observed in this checkpoint are `target/debug/deps/public_contract-4811b915d13d7696` and `target/debug/deps/configuration_snapshot-6a85f3021358f95c`; reruns MUST use Cargo's newly reported executable paths rather than assume hashes survive source/toolchain changes. Local transient logs and full-precision metrics were captured under `/tmp/glade-q3-contract-20261003`; this committed document preserves the commands/results if temporary logs disappear.

The large NEW conformance draft was split by storage/membership/snapshot responsibility before settlement using the already installed rust-split. `explode` manifest-order byte reconstruction was asserted identical; generated candidate grouping was inspected, then whole function items/comments were moved with bounded module imports, followed by a separate formatting pass. The compiler consumer stayed GREEN and behavioral refusing cases stayed RED. Current helper modules are 49/160/233/198 lines; no old source or conditional attribute was moved. This is draft specification structure, not a tested implementation refactor or broader cfg migration.

The remaining action is independent peer-blind Consistency/Safety review of the same settled package/doc tuple, bounded remediation as needed, then implementation only after both contract gates accept. Code/State and every actual matrix witness remain required for Q3 acceptance. Production activation, power loss, independent domains, crypto/bootstrap, automatic-election randomness and legacy/external-effect exclusion remain separate Q4/profile gates.
