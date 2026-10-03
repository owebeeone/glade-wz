# Q3 implementation evidence — 2026-10-03

Status: **implementation complete, pending independent Code/State review and owner acceptance**. This document records the implementation drafter's evidence; it is not a review verdict or production approval. The owner MUST pin the settled implementation source tuple and file the independent reviews in [GladeRaftQualification-ReviewCycle.md](GladeRaftQualification-ReviewCycle.md). No commit, push, desktop rebuild or production activation was performed by the drafter.

## Accepted scope and concrete providers

The normative sources are [GladeRaftConfigurationSnapshotContract.md](GladeRaftConfigurationSnapshotContract.md), [GladeRaftQ3ImplementationAllocation.md](GladeRaftQ3ImplementationAllocation.md), [GladeRaftPersistenceContract.md](GladeRaftPersistenceContract.md), and the existing adoption contract. Allocation GO/GO was `ccab267c6b23bfec7471923098944048a7959563`; the learner bootstrap clarification received GO/GO on `d4589feb02ad86b92f58686da35c8a216c370c44`. The final implementation source is the owner's subsequent frozen object, not either allocation object.

- `disk::v2::V2DiskStore` implements the reviewed dyn `CheckpointStore`. It uses an exclusive, locked, checksummed, bounded append-only full-image V2 journal; whole file and parent-directory sync; coherent checkpoint/configuration/HardState/applied/suffix publication; injected one-shot faults; explicit poison and reopen semantics. `V2StoreFactory` implements reviewed `StoreLifecycle` with injected owned roots and meaningful separate create/open operations.
- `proof::q3::Q3Session` implements the reviewed private `QualificationSession`, with actual `raft::RawNode` and custom `Q3Storage`. Original serialized Entries, complete application/configuration commands/intents/results, and full materialized application maps are retained in a private versioned checkpoint. Replay reconstructs and compares all evidence before serving. No current-counter assignment stands in for learner catch-up.
- The host verifies a retained accepted external join before creating or reopening private learner 4. Its original-genesis empty state is nonserving and cannot vote/pre-vote/campaign. Only actual restored/replayed committed history can establish its local join and complete applied cut. Missing reopen never creates a replacement.
- All nineteen original `q3-spec/tests/configuration_snapshot.rs` cases use concrete providers through `tests/common/mod.rs`, remain default-selected, and pass without ignores, skips, name changes or model substitution. Historical refusing types remain labelled scaffolds, outside these concrete tests.
- Roles, normal/dev dependency edges, meaningful public traits and Q1a/Q2 public interfaces retain their reviewed shapes. Inventory edits update descriptive reasons only. No third-party dependency, process-global exception, allowlist relaxation, production edge or global path selector was introduced. Application additions are private full-evidence/voter helpers; the public Q1a ordered replay contract remains preserved.

## TDD and pre-review discoveries

The initial compiling consumer checkpoint had **eight actual-store RED tests** and **nineteen concrete-consumer RED tests**, against refusing providers. The compiler failure preceding the new lifecycle declaration was a compiler check, not behavioral RED evidence. Additional actual behavior was tested before each corresponding correction:

| Named regression | Observed behavioral RED and correction |
| --- | --- |
| `seeded_join_cut_cannot_receive_same_cut_as_actual_snapshot_ready` | A receiver already at S had no incoming snapshot Ready at S. The carrier-only counterexample distinguishes seeded rejection/fast-forward from actual original-genesis receive; it motivated the separately reviewed bootstrap clarification. It establishes no application/authority/I/O claim alone. |
| `incomplete_original_genesis_learner_is_gated_before_votes_and_pre_votes` | RawNode persisted a vote despite receiver 4 being absent from voter ConfState. The host now suppresses Vote and PreVote before stepping it; restart is separately covered. |
| `pending_configuration_retry_keeps_unknown_and_rejects_changed_intent_bytes` | Pending exact configuration intent could be subjected to new eligibility checks. Exact admitted intent is located in retained unapplied suffix before eligibility; exact retry remains unknown and changed key bytes conflict. |
| `a_missing_promoted_voter_store_blocks_recovery_instead_of_recreating` | Recovery accepted a group missing promoted voter 4. Newest validated voter configuration now requires every incoming/outgoing voter store before constructing carriers. |
| `removing_leader_preserves_home_and_manual_restart_campaigns_an_eligible_voter` | Removed node 1 remained an upstream raw leader. Host serving excludes removed members; explicit manual restart campaign selects an eligible member. No automatic-election qualification is inferred. Original home 2/generation 1 are preserved. |
| `an_empty_private_four_requires_external_retained_join_authority_on_recovery` | An original-genesis empty 4 could be opened without a retained external join. Constructor and restart now revalidate the other member's accepted join before private carrier construction. |
| `checkpoint_history_terms_must_not_regress` | A valid-format history with noop term 2 then creation term 1 was accepted. Complete original history now rejects term regression. |
| `actual_snapshot_ready_rejects_valid_conflicting_committed_application_history` | A fully valid same-group foreign payload 99 snapshot could replace previously acknowledged payload 11 and emit an append response. Both pre-step host validation and Ready publication compare all common original committed Entries and typed results before install. |
| `a_new_checkpoint_cannot_regress_the_compacted_term` | Checkpoint term regression was accepted. Exact prior checkpoint and retained committed-entry bounds are now checked. |
| `an_older_applied_checkpoint_retains_a_later_term_committed_suffix` | An older applied checkpoint at S/term 1 with original later committed suffix term 2 was incorrectly rejected by comparison to latest commit term. Exact term at the checkpoint cut is now used when retained; hostile rewrites/regressions still refuse. This was an owner pre-review discovery, not a hidden review closure. |
| `adding_promoted_voter_as_learner_refuses_before_proposal_and_keeps_group_usable` | Fresh AddLearner4 while 4 is incoming/outgoing voter returned the wrong admission result and could create unsupported overlap after leave. It now refuses `InvalidImage` before proposal; private ordered bypass retains deterministic `InvalidChange`, never invokes `apply_conf_change`, and keeps the group usable. Existing learner duplicate semantics remain permitted. |
| `private_uncommitted_configuration_entries_reject_unsupported_carrier_grammar` | Unknown fields nested in ConfChangeSingle were accepted. Parser now validates nested unknown fields, bounded unique nodes, supported change kinds and exact intent-dependent transition grammar even for uncommitted private Entries. |
| `learner_can_restore_actual_snapshot_after_source_compacted_beyond_join` | Late catch-up tried to regress source checkpoint to its earlier join S and failed `InvalidImage`. Source regeneration uses at least the already persisted checkpoint cut, retaining original history; actual learner receives the later valid cut. |
| `explicit_joint_with_identical_voters_is_real_joint_then_can_leave` | Canonical unchanged target voters yielded no configuration receipt: empty V2 means leave-joint to raft-rs. Internal exact encoding emits an idempotent existing-voter AddNode with Explicit transition for this case. Typed retained intent/result and subsequent explicit exit are checked. No target-domain restriction was added. |
| `empty_explicit_carrier_entry_cannot_be_reopened_as_private_enter_joint` | Follow-up parser RED accepted an empty Explicit V2 as an uncommitted private enter intent. The bounded private grammar now refuses it before carrier construction. Legitimate same-target joint uses the nonempty idempotent encoding above. |

Additional tests that were green against already implemented behavior are coverage, not claimed implementation REDs. A few draft test/compiler corrections (wrong helper names, struct field names, and expecting a changed private Application reply to exist) were corrected without changing behavior and are not listed as feature REDs.

## Contract §7 exit matrix

All cases below pass against actual files/carriers unless specifically labelled parser/semantic or parent-oracle checks. `configuration_snapshot` denotes the nineteen original concrete consumer tests; owning host tests are under `proof/src/q3/`, store tests under `disk/`.

| Mandatory row | Exact owning witnesses and assertions |
| --- | --- |
| V2 lifecycle/compatibility | `explicit_create_sync_reopen_and_exact_binding`, `open_missing_never_creates`, `empty_old_format_and_wrong_instance_never_reset`, `held_lock_and_exclusive_create_prevent_overwrite`, `invalid_instance_is_refused_before_file_creation`, `real_q2_file_is_incompatible_and_preserved_byte_for_byte`; `join_authority_missing_reopen_and_incomplete_restart_never_reset_or_vote` checks unauthorized/wrong-group admission leaves node-4 absent, missing learner reopen remains missing, and incomplete restart revalidates external join. Foreign valid checkpoint binding is rejected by `snapshot_valid_foreign_binding_corrupt_state_and_capacity_refuse`. |
| Learner and dual quorum | Original `membership_authorized_namespace_and_exact_intent_retry`, `learner_unavailable_cannot_promote_or_supply_quorum`, both `joint_*_majority_alone_cannot_accept`, and `compacted_snapshot_restores_learner_before_joint_promotion`; owning actual vote/pre-vote and incomplete-restart gates; `ordered_stale_promotion_refusal_agrees_at_every_replica_and_demotion_is_invalid`; `direct_and_queued_home_exit_refusals_are_retained_at_every_actual_replica` executes all direct/queued × movement/retirement variants and compares full refused receipts/typed entries on every member. Actual removed leader, new voter 4 resource home, same-target explicit joint and unsupported demotion cases are additional witnesses. |
| Configuration interruption | `joint_restart_and_lost_configuration_reply_are_recoverable`, `configuration_eligibility_lost_after_admission_retains_exact_refusal`, pending exact/changed-intent regression; `uncommitted_configuration_restart_keeps_unknown_then_commits_exact_original_entry` keeps original uncommitted Entry across reopen and explicitly manually recampaigns after dropped partitioned election messages; SIGKILL joint-before-apply below. `snapshot_original_accepted_refused_configuration_and_noop_replay_is_typed` compares complete accepted/refused/configuration/application/noop originals, changed envelopes conflict, missing remains distinct. |
| Snapshot semantic adversaries | `valid_format_materialized_maps_must_equal_full_original_replay` changes applied/policy/voters/resource generation/home/payload/tombstone/name/id within valid word framing; `valid_format_original_command_result_index_policy_permissions_and_cut_adversaries_quarantine` changes canonical original command, typed original result/request/index, complete permission keys/values, checkpoint index/term/ConfState/version with replay mismatch; accepted retirement fixture checks complete tombstone. Term-regressing history and actual valid coherent foreign committed history are separate regressions. |
| Snapshot plus suffix | `compacted_snapshot_restores_learner_before_joint_promotion` asserts actual received snapshot S at learner-add and later applied suffix beyond S; `actual_snapshot_ready_all_five_faults_preserve_coherent_join_cut_then_real_suffix` directly asserts incoming Ready.snapshot S, no immediate later committed entries, then sends real MsgAppend normal suffix; `actual_append_after_snapshot_reconciles_uncommitted_original_suffix_without_applying_it` restores cp2/uncommitted original3 and replaces it with actual higher-term MsgAppend3, preserving original create and never applying old payload99. Original policy/retirement and historical retry cases apply current disclosure after later policy. |
| Publication faults | `all_publication_faults_preserve_full_or_unknown_quarantined_history`, `all_publication_faults_keep_checkpoint_configuration_and_suffix_in_one_cut` publish full checkpoint+configuration+suffix images together at all five faults. BeforeWrite leaves original usable; others poison. Reopen is exact prior, complete new, or quarantine for torn partial record. Actual snapshot Ready and `actual_light_ready_commit_all_five_publication_faults_gate_apply_and_messages` each exercise all five faults: no failed-lifecycle messages or candidate/application install. Genuine LightReady commit-only publication is observed using legal Ready/advance_append_async sequencing; complete original expected Resource and Entry survive successful/recoverable cuts. |
| Actual process termination | `process-crash-q3.py` + `proof/tests/q3_process_crash.rs`: actual SIGKILL **ack**, **joint-before-apply**, **snapshot-before-apply**; parent retains full originals and fresh-process full lookup/exact retry/typed replay must match. Exact cuts/oracles are detailed below. Self-test defeats ten changed recovered fields and missing/changed parent evidence. |
| Bounds/rollback | `whole_frame_capacity_refusal_preserves_prior_usable_store`; actual-file parser test checks every truncated genesis cut, checksum failure, oversized header/counts, revision/MAX, term/MAX, cut/ConfState invalidity and valid-checksum hostile words without rewriting files; `trusted_floor_detects_valid_old_journal_without_claiming_intrinsic_detection`; original V2 suffix/immutability/historical-removed-vote cases; `terminal_term_and_predecessor_refuse_before_campaign_without_resetting_evidence` covers actual MAX create refusal and MAX-1 manual-campaign refusal byte-for-byte. Checkpoint term regression and legitimate cross-term older applied cut both remain checked. Capacity refusal never evicts original evidence. |
| Combined acceptance | `combined_actual_snapshot_suffix_joint_restart_policy_retirement_and_explicit_exit`: acknowledged original create2; source compaction; accepted learner3; normal mutation4; actual target snapshot3 plus suffix4; verified full target cut; joint transition; later disclosure revocation and accepted retirement; joint reopen preserves complete original retries/typed create replay and current denial; new explicit leave succeeds and retains full tombstone. The two joint-majority partition tests establish both data-bearing quorum obligations separately. |

Carrier modes are distinct: `actual_matching_snapshot_fast_forward_and_stale_rejection_keep_original_history` asserts empty Ready snapshot on matching index/term fast-forward and later stale refusal, preserving every original Entry/result without installing candidate state. `actual_snapshot_recipient_rejection_never_installs_or_claims_join_readiness` demonstrates actual missing-recipient carrier refusal with no local application/cut/checkpoint install. `Q3Storage` supplies saved full snapshot bytes/ConfState at S, first_index S+1, exact cp/suffix terms and original entries; it does not rely on MemStorage's current empty-data snapshot.

## Actual SIGKILL and full external oracles

The explicit ignored worker is **not** a passing standalone durability test. The external parent executes it with only the named injected environment, waits for bounded complete record lines and the actual cut marker, sends SIGKILL, requires the signal exit, and starts a fresh worker on existing files. Private Rust Debug records carry every owned contract field and exact Entry byte; these records are test-oracle material, not a product protocol.

| Cut | Actual interruption and parent-held complete expectations |
| --- | --- |
| `ack` | After actual joint commitment and acknowledged normal mutation, with a persisted checkpoint. Parent retains original full Command, application Receipt/Entry, ConfigIntent, accepted ConfigReceipt/Entry. Fresh lookup, exact retry and typed original-index replay must all equal the parent originals. |
| `joint-before-apply` | Leader V2 synced original joint Entry at index4/commit4 while applied remains earlier, before the publish call returns to application. Parent retains acknowledged original creation and full config intent/original committed Entry. No configuration receipt exists yet. Parent independently constructs complete expected ConfigReceipt: original intent, Accepted, index4/version4, incoming `[1,2,4]`, outgoing `[1,2,3]`, empty learner lists, auto_leave false. It never trusts a speculative worker receipt or two recovered values as this oracle. |
| `snapshot-before-apply` | Actual receiver 4 has synced incoming Ready checkpoint at learner-add S=3 before candidate installation; source has later original normal suffix4. Parent already retains full leader ACKs/commands/intents and original APP4/CONFIG3 Entry bytes. Fresh process restores checkpoint, receives later suffix and compares original full APP/CONFIG lookup/retry/typed replay. Receiver durable snapshot and full applied cut are asserted. |

All three cuts PASS. The tests exercise process termination with the kernel/storage stack still running. The suite MUST NOT be described as power-loss, independent-machine, transport, cryptographic/bootstrap or automatic-election certification. Complete application original history and append-only journal remain retained; no eviction, memory saving, faster startup or physical disk reclamation is claimed.

## Reproduction commands and final results

From `/Volumes/projects/limbo/glade-wz`, use the cached compiler:

```sh
export PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib -- --ignored
cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace --all-targets -- -D warnings
./proofs/raft-adoption/check.sh
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test process_crash --test q3_process_crash --no-run
python3 proofs/raft-adoption/process-crash.py --self-test
python3 proofs/raft-adoption/process-crash-q3.py --self-test
python3 proofs/raft-adoption/process-crash.py --worker proofs/raft-adoption/target/debug/deps/process_crash-304030f150fc974e
python3 proofs/raft-adoption/process-crash-q3.py --worker proofs/raft-adoption/target/debug/deps/q3_process_crash-57c67e4971c8507c
```

Cargo prints the exact worker executable names at `--no-run`; other hosts MUST use those printed names rather than assuming these local hashes.

Final default fixture workspace: **113 PASS, zero failures, four explicit ignores** (two existing Q2 lib tier cases and the two external crash workers). Original Q3 nineteen: **19/19 actual PASS**; extra q3-spec carrier/combined cases: **5 PASS**; private Q3 host/semantic/Ready regressions: **21 PASS**; V2 actual lifecycle/fault cases: **12 PASS**, actual V2 parser suite: **1 PASS**. Prior Q1a qualification **17 PASS**, Q2 recovery **13 PASS**, Q2 disk conformance **12 PASS**, compatibility **2 PASS**, contract compiler tests all PASS. Both separately ignored Q2 disk witnesses were explicitly executed: **2 PASS**. Q2 oracle **4 PASS**, actual Q2 SIGKILL **2 cuts PASS**; Q3 adversarial oracle PASS and actual Q3 SIGKILL **3 cuts PASS**. Strict all-target Clippy, architecture, formatting and source/cfg boundary checks PASS. Process-global guard: **44 owned Rust files, zero exceptions/debt/permanent entries**.

## Measured tiers and source organization

Host: arm64 macOS 26.6.2 (25G83), Rust/cargo 1.96.0, aarch64-apple-darwin; cached offline third-party sources/protoc. Actual fixture stores are on local APFS (`/Volumes/projects`; crash temporary roots are on system APFS). These are observations from this run, not performance guarantees.

Fast contract command is `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api --test public_contract`: it compiles only the bounded contract consumer and performs no carrier/filesystem composition. Affected consumer command is the same command with `-p glade-raft-q3-spec --test configuration_snapshot`. Additional named `proof --lib q3`, disk I/O, full workspace and externally supervised SIGKILL are system-assurance commands; they are not required on every minor contract edit. Default workspace tests remain complete, including additional assurance tests; there is no hidden filtering or exception to the original nineteen.

| Measurement | Observed wall time |
| --- | ---: |
| Contract cold build `--no-run`, entirely fresh Cargo target directory | 0.459 s |
| Contract warm build `--no-run` | 0.032 s |
| Contract warm Cargo build+tests / direct test execution | 0.037 s / 0.003 s |
| Carrier/system cold `--workspace --no-run` in same fresh target (only contract artifacts already built) | 8.450 s |
| Carrier/system warm `--workspace --no-run` | 0.050 s |
| V2 lifecycle warm Cargo+tests / direct execution | 0.086 s / 0.059 s |
| Original nineteen actual consumer warm Cargo+tests / direct execution | 1.585 s / 1.005 s |
| Additional twenty-one actual Q3 host/semantic/Ready direct execution | 1.220 s |
| Complete default workspace warm Cargo+tests | 4.954 s |
| Two Q2 actual SIGKILL cuts / three Q3 actual SIGKILL cuts | 0.733 s / 1.143 s |

The final fresh-target cold/warm no-run measurements, full workspace run and twenty-one host direct run were repeated after the final empty-carrier parser closure. Contract direct/warm-test, V2 direct/warm-test, consumer direct/warm-test and timed crash observations were recorded just before that final closure; the affected full suite and both crash runners were rerun afterward and passed. These earlier individual timing observations remain explicitly observations, not exact final-object performance guarantees.

Cold staging used fresh `/tmp/glade-q3-cold-settled-20261003`; contract and carrier cold phases are reported separately, avoiding a claim that the latter rebuilt the already measured API artifacts. Machine load/timing varies. Measurement logs and JSON are at `/tmp/glade-q3-{api-cold,api-warm,system-cold-carrier,system-warm,workspace-final}.log`, `/tmp/glade-q3-measurements.json`, `/tmp/glade-q3-execution-measurements.json`, and `/tmp/glade-q3-crash-measurements.json`; repository commands/tests are the durable evidence and the owner independently reruns them before freezing review.

The split-files skill was applied at GREEN checkpoints. Syntax-aware explode manifests reconstructed machine/session sources byte-for-byte; generated candidate groups were inspected rather than accepted with broadened visibility. Cohesive reviewed moves separated exact carrier grammar/encoding (`carrier.rs`) and injected lifecycle/manual campaigns (`session/lifecycle.rs`) from deterministic replay/message routing. V2 framing, validation, journal and lifecycle remain separate modules. Production/private source modules remain below 500 LOC; the largest additional actual snapshot regression module is 460 LOC and contains the related restore/fast-forward/reconciliation cases. Formatting and affected tests were separate checks after relocation. No broader existing-code migration is claimed.

## Owner independent pre-freeze verification

After the drafter stopped edits, the owner independently ran the documented
default workspace command: **113 PASS, zero failures, four explicit tier
ignores**, with the original nineteen concrete cases default-selected and zero
filtered. The two ignored Q2 library cases were explicitly run: **2 PASS**.
Both external runners passed again: **2 actual Q2 SIGKILL cuts** and **3 actual
Q3 SIGKILL cuts**, using the Cargo-reported executables listed above. Both
parent-oracle self-tests passed. `check.sh` and strict all-target Clippy passed;
the owned-source scan found **44 Rust files and zero exceptions**. This is
pre-review verification, not independent reviewer acceptance. The owner freezes
these source/evidence bytes and supplies the exact tuple in the canonical
Code/State prompts.
