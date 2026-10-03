# Glade Raft Q3 contract compilation and RED evidence

Date: 2026-10-03. Status: **DRAFT remediation 1 checkpoint, renewed contract review pending; no implementation acceptance**. Controlling scope is [GladeRaftConfigurationSnapshotContract.md](GladeRaftConfigurationSnapshotContract.md) and [the qualification plan](GladeRaftQualificationPlan.md). The lane owner binds the exact committed tuple and independent review reports in [the review ledger](GladeRaftQualification-ReviewCycle.md).

## Initial executed object and limits — root fa1ff8b9fd0be53932300730ff925d0e41c76b1a

The following initial commands/counts/measurements are historical. Initial Safety review found three P2 defects; passing compiler checks and intentionally RED stubs did not prove the fixtures satisfiable. Current remediation evidence follows in the final section.

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


## Remediation 1 — bounded contract/specification correction

The [merged plan](GladeRaftQ3Contract-RemPlan-1.md) accepts Safety P2-1/2/3 for correction; closure is for independent renewed review, not the drafter to declare. One patch corrects the Create fixture, protects current live homes at actual joint exit, and amends the draft replay result to typed application/configuration/noop variants. No Q3 Raft/storage/snapshot codec algorithm is built, and no existing Application/Q2 implementation source changes.

**P2-1 observed RED before correction:** new `create_fixture_has_zero_preconditions_and_complete_selected_home` failed exact whole-command equality, actual generation/home 1/1 versus required 0/0. The new existing-Application integration target also failed: `corrected_q3_create_is_accepted_by_the_existing_application` returned `Rejected(StaleGeneration)` instead of the complete independently expected Accepted resource. Both commands compiled and exited 101. The paired nonzero generation/home negative regression passed and retained `StaleGeneration`/`WrongHome`.

After only the fixture correction, the exact-command API regression passed, and the unchanged real Application target passed **both** corrected-create and nonzero-precondition cases. The API compiler consumer now exhaustively handles every typed replay variant. Shared success setup uses `submit_accepted`/`accepted_create` to assert complete request and Accepted Resource; mutation, permission, retirement and movement success fixtures likewise assert their complete intended state, rather than treating any Receipt as success.

**P2-2 compiling RED additions:** three joint-exit cases permit Create home 3 while it is an outgoing voter; direct placement is restarted while joint, and another case queues placement between leave admission and actual application. Each requires retained `Refused(HomeInUse)`, unchanged joint configuration/version and exact original terminal retry after restart. Qualified movement to incoming home 2 or retirement resolves the blocker; old key stays refused, stale expected version rejects, and a NEW correct-version exit key succeeds. These are intentionally RED specifications, not executed actual membership evidence. LeaveJoint's deterministic current-home recheck cannot be replaced by admission-only refusal or local availability.

**P2-3 compiling RED addition:** preserve accepted and refused original complete ConfigReceipts, original application receipt and actual committed election noop Entry before checkpoint/install/restart, then require typed exact-index replay of each complete original. Changed envelopes for each kind return ConflictingReplay; absent/unapplied indexes return Missing, never Noop. The enum exhaustively compiles in a dyn consumer. This changes the draft Q3 interface and requires fresh dual contract reviewers; it does not amend Q1a's public application apply/replay semantics.

The new dev-only `glade-raft-adoption-proof -> glade-raft-q3-api` edge is explicitly **proposed for review**, solely for `proof/tests/q3_fixture_compatibility.rs`. Manifest and architecture inventory declare it; no normal edge, contract-to-implementation edge, classification relaxation, third-party dependency or process-global exception was added. The proof package's existing concrete implementations are untouched. The compatibility test validates numerical fixture compatibility, not Q3 algorithm/snapshot correctness.

Exact targeted commands (compatible PROTOC remains required for the proof package):

```sh
# Observed RED before correcting create(), now GREEN alongside dyn consumer.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api --test public_contract create_fixture_has_zero_preconditions_and_complete_selected_home
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api
# Observed real-Application RED before correction; now 2 PASS including negatives.
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test q3_fixture_compatibility
# Successful compilation, then 19 deliberate NotQualified behavioral failures.
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api -p glade-raft-q3-spec --no-run
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot
```

Current results: API **2 PASS** (compiler and exact-fixture regression); actual existing Application compatibility **2 PASS**; Q3 scaffold **19 RED**, all NotQualified, none ignored. The previous 15 cases remain, with complete Accepted setup checks and typed application replay. The four new cases are:

- `joint_exit_refuses_current_outgoing_home_until_retirement`
- `joint_exit_refuses_current_outgoing_home_until_qualified_movement`
- `joint_exit_rechecks_placement_queued_after_admission`
- `snapshot_original_accepted_refused_configuration_and_noop_replay_is_typed`

The initial table's `create: NotQualified` failure label is now `accepted command: NotQualified` because setups pass through the stricter accepted-state helper. All 19 remain genuinely behavioral RED, not compiler failures or tests inverted to expect NotQualified. Whole-workspace tests remain deliberately RED during this contract gate; no default-members/hidden skips changed.

Affected package-selected command from the initial section now passes **51** selected tests: the existing **49** plus **2** new compatibility cases. The separate `q2_real` tier remains **2 PASS**. These two extra compatibility cases do not expand Q2's accepted profile. Structural architecture/dependency/trait gate, formatting/token-aware boundary gate and all-target Clippy with warnings denied PASS. Expanded owned-source scan reports **21 files, zero allowlisted entries**. Q2 physical process-kill evidence remains at its previously accepted implementation tuple; no process-kill matrix was newly qualified by this contract patch.

Same machine/toolchain and measurement method as above, now on the corrected source; one sample each, temporary cold target removed, no latency promise:

| Remediation tier | Wall seconds | Result |
| --- | ---: | --- |
| Cold combined API/spec no-run | 1.001 | Compiles |
| Warm combined no-run | 0.095 | Compiles |
| Warm API tests/doctests | 0.909 | 2 PASS |
| Warm behavioral spec command | 0.397 | 19 RED, exit 101 |
| Prebuilt API integration execution | 0.006 | 2 PASS |
| Prebuilt behavioral integration execution | 0.004 | 19 RED, exit 101 |

Local full-precision logs are `/tmp/glade-q3-remediation-1-20261003`; committed commands/results here remain authoritative if transient files vanish. New cohesive joint-exit/replay modules preserve focused boundaries; no existing implementation declaration/attribute or conditional scope was relocated.

The lane owner MUST settle one source tuple and obtain renewed fresh peer-blind Consistency/Safety review plus originating Safety counterexample verification under the review-loop process. P2 closure is pending those verdicts. All actual V2 lifecycle/carrier/disk/fault/semantic/crash exit witnesses remain required later; this patch cannot close Q3 implementation or production gates.
