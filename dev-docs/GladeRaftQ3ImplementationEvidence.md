# Q3 implementation evidence — 2026-10-03

Status: **remediation 1 implemented and verified by the drafter; all five originating findings remain open pending independent closure and fresh Code/State renewal**. The initial reviewed implementation received Code/State NO-GO; the corrected object has not been accepted. This document records the implementation drafter's evidence; it is not a review verdict or production approval. The owner MUST pin the settled implementation source tuple and file the independent reviews in [GladeRaftQualification-ReviewCycle.md](GladeRaftQualification-ReviewCycle.md). No commit, push, desktop rebuild or production activation was performed by the drafter.

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
| V2 lifecycle/compatibility | `explicit_create_sync_reopen_and_exact_binding`, `open_missing_never_creates`, `empty_old_format_and_wrong_instance_never_reset`, `held_lock_and_exclusive_create_prevent_overwrite`, `invalid_instance_is_refused_before_file_creation`, `real_q2_file_is_incompatible_and_preserved_byte_for_byte`; `join_authority_missing_reopen_and_incomplete_restart_never_reset_or_vote` checks unauthorized/wrong-group admission leaves node-4 absent, missing learner reopen remains missing, and incomplete restart revalidates external join. The original `snapshot_valid_foreign_binding_corrupt_state_and_capacity_refuse` includes an outer-only foreign binding mismatch. The distinct internally coherent foreign fixture is covered by `coherent_foreign_checkpoint_envelope_history_and_binding_refuse_without_publication`, including independent full-history/materialized-state validation and fixed-profile normalization. |
| Learner and dual quorum | Original `membership_authorized_namespace_and_exact_intent_retry`, `learner_unavailable_cannot_promote_or_supply_quorum`, both `joint_*_majority_alone_cannot_accept`, and `compacted_snapshot_restores_learner_before_joint_promotion`; owning actual vote/pre-vote and incomplete-restart gates; `ordered_stale_promotion_refusal_agrees_at_every_replica_and_demotion_is_invalid`; `direct_and_queued_home_exit_refusals_are_retained_at_every_actual_replica` executes all direct/queued × movement/retirement variants and compares full refused receipts/typed entries on every member. Actual removed leader, new voter 4 resource home, same-target explicit joint and unsupported demotion cases are additional witnesses. `correctly_versioned_nonjoint_leave_and_nested_joint_refuse_without_logging_or_outcome` checks fresh correctly versioned NotJoint/JointInProgress admission, unchanged complete logs/files/configuration, absent rejected-key outcomes and subsequent legitimate work. |
| Configuration interruption | `joint_restart_and_lost_configuration_reply_are_recoverable`, `configuration_eligibility_lost_after_admission_retains_exact_refusal`, pending exact/changed-intent regression; `uncommitted_configuration_restart_keeps_unknown_then_commits_exact_original_entry` keeps original uncommitted Entry across reopen and explicitly manually recampaigns after dropped partitioned election messages; `partitioned_joint_exit_reopens_fresh_eligible_voter_and_preserves_full_originals` physically reopens stale voter 2 versus current voters 3/4 after the incoming234 joint exit, retains complete originals/home2/generation1, accepts a new mutation, repeats restart and verifies real quorum loss. `disconnected_freshest_voter_does_not_trap_manual_restart_with_an_available_quorum` excludes disconnected 1 and campaigns available 2 through legal RawNode voting. SIGKILL joint-before-apply below. `snapshot_original_accepted_refused_configuration_and_noop_replay_is_typed` compares complete accepted/refused/configuration/application/noop originals, changed envelopes conflict, missing remains distinct. |
| Snapshot semantic adversaries | `valid_format_materialized_maps_must_equal_full_original_replay` changes applied/policy/voters/resource generation/home/payload/tombstone/name/id within valid word framing; `valid_format_original_command_result_index_policy_permissions_and_cut_adversaries_quarantine` changes canonical original command, typed original result/request/index, complete permission keys/values, checkpoint index/term/ConfState/version with replay mismatch; accepted retirement fixture checks complete tombstone. Term-regressing history and actual valid coherent same-group conflicting committed history are separate regressions. `valid_future_checkpoints_validate_overlap_and_refuse_without_any_publication` uses two actual sessions, validates common committed originals before recipient selection, quarantines conflicting future history, explicitly refuses nonconflicting unsupported future installation and preserves successful same-history compaction/restart. |
| Snapshot plus suffix | `compacted_snapshot_restores_learner_before_joint_promotion` asserts actual received snapshot S at learner-add and later applied suffix beyond S; `actual_snapshot_ready_all_five_faults_preserve_coherent_join_cut_then_real_suffix` directly asserts incoming Ready.snapshot S, no immediate later committed entries, then sends real MsgAppend normal suffix; `actual_append_after_snapshot_reconciles_uncommitted_original_suffix_without_applying_it` restores cp2/uncommitted original3 and replaces it with actual higher-term MsgAppend3, preserving original create and never applying old payload99. Original policy/retirement and historical retry cases apply current disclosure after later policy. |
| Publication faults | `all_publication_faults_preserve_full_or_unknown_quarantined_history`, `all_publication_faults_keep_checkpoint_configuration_and_suffix_in_one_cut` publish full checkpoint+configuration+suffix images together at all five faults. At the adapter level BeforeWrite leaves the original store usable; other injected uncertain faults poison it. An attached host voter MUST stop on any publication error until physical reopen, including BeforeWrite. Reopen is exact prior, complete new, or quarantine for torn partial record. Actual snapshot Ready and `actual_light_ready_commit_all_five_publication_faults_gate_apply_and_messages` each exercise all five faults: no failed-lifecycle messages or candidate/application install. Genuine LightReady commit-only publication is observed using legal Ready/advance_append_async sequencing; complete original expected Resource and Entry survive successful/recoverable cuts. |
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

## Reproduction commands and initial implementation results

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

Historical initial default fixture workspace (reviewed implementation `b61197602e5594bdf89770bda069ce7d30fdb222`): **113 PASS, zero failures, four explicit ignores** (two existing Q2 lib tier cases and the two external crash workers). Original Q3 nineteen: **19/19 actual PASS**; extra q3-spec carrier/combined cases: **5 PASS**; private Q3 host/semantic/Ready regressions: **21 PASS**; V2 actual lifecycle/fault cases: **12 PASS**, actual V2 parser suite: **1 PASS**. Prior Q1a qualification **17 PASS**, Q2 recovery **13 PASS**, Q2 disk conformance **12 PASS**, compatibility **2 PASS**, contract compiler tests all PASS. Both separately ignored Q2 disk witnesses were explicitly executed: **2 PASS**. Q2 oracle **4 PASS**, actual Q2 SIGKILL **2 cuts PASS**; Q3 adversarial oracle PASS and actual Q3 SIGKILL **3 cuts PASS**. Strict all-target Clippy, architecture, formatting and source/cfg boundary checks PASS. Process-global guard: **44 owned Rust files, zero exceptions/debt/permanent entries**.

## Historical initial measured tiers and source organization

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

The initial settled fresh-target cold/warm no-run measurements, full workspace run and twenty-one host direct run were repeated after the final empty-carrier parser closure. Contract direct/warm-test, V2 direct/warm-test, consumer direct/warm-test and timed crash observations were recorded just before that final closure; the affected full suite and both crash runners were rerun afterward and passed. These earlier individual timing observations remain explicitly observations, not exact final-object performance guarantees.

Cold staging used fresh `/tmp/glade-q3-cold-settled-20261003`; contract and carrier cold phases are reported separately, avoiding a claim that the latter rebuilt the already measured API artifacts. Machine load/timing varies. Measurement logs and JSON are at `/tmp/glade-q3-{api-cold,api-warm,system-cold-carrier,system-warm,workspace-final}.log`, `/tmp/glade-q3-measurements.json`, `/tmp/glade-q3-execution-measurements.json`, and `/tmp/glade-q3-crash-measurements.json`; repository commands/tests are the durable evidence and the owner independently reruns them before freezing review.

The split-files skill was applied at GREEN checkpoints. Syntax-aware explode manifests reconstructed machine/session sources byte-for-byte; generated candidate groups were inspected rather than accepted with broadened visibility. Cohesive reviewed moves separated exact carrier grammar/encoding (`carrier.rs`) and injected lifecycle/manual campaigns (`session/lifecycle.rs`) from deterministic replay/message routing. V2 framing, validation, journal and lifecycle remain separate modules. Production/private source modules remain below 500 LOC; the largest additional actual snapshot regression module is 460 LOC and contains the related restore/fast-forward/reconciliation cases. Formatting and affected tests were separate checks after relocation. No broader existing-code migration is claimed.

## Historical owner independent pre-freeze verification

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

## Remediation 1: corrected candidate, independent closure pending

The merged plan is [GladeRaftQ3Implementation-RemPlan-1.md](GladeRaftQ3Implementation-RemPlan-1.md); originating reports are [Code](GladeRaftQ3Implementation-ReviewCode.md) and [State](GladeRaftQ3Implementation-ReviewState.md). These reports reviewed `b61197602e5594bdf89770bda069ce7d30fdb222`. Remediation began from root `95b29caa7376fabf1dca0bf3044b8a74a900cd90`, retaining the accepted member/Gyld pins in the plan. The owner MUST freeze the corrected source/evidence tuple before review; this drafter does not self-close findings or supply a GO verdict.

| Originating finding | Behavioral evidence and bounded correction |
| --- | --- |
| Code P2-1 | Actual V2 PartialWrite direct install returned IoUnknown yet the original in-memory receipt was still returned: observed behavioral RED. Central `Voter::publish` now records the exact store error as a stopped-voter flag. A second behavioral RED showed a previously queued failed-source heartbeat still changed a follower's term; routing and snapshot-result delivery now suppress stopped sources as well as stopped targets. The owning `install_fault_tests.rs` test covers direct install and CatchUp × all five faults, including BeforeWrite. It checks no memory replies/resources, no outgoing source influence, physical close/open, exact prior/full-image recovery or PartialWrite quarantine, and complete original APP/CONFIG/ENTRY retries/replay. Recoverable CatchUp cases complete real target snapshot/suffix delivery after reopen. The bounded actual-V2 malformed-success follow-up below extends the same stop path to response-validation errors without broadening contracts. |
| State P2-1 | The exact actual-file partition/joint-exit counterexample recovered no leader by selecting stale incoming voter 2 despite current voters 3/4: observed RED. A separate available123 quorum with disconnected freshest voter 1 also observed RED. Manual campaigns now select an available, nonfailed, locally complete incoming voter by actual durable last-log `(term,index)` and lowest-ID tie-break. Both `restart_freshness_tests.rs` cases use legal `RawNode::campaign`, not term/counter assignment. Complete original receipts/Entry bytes and home2/generation1 survive, new mutation succeeds, repeated physical restart retains originals, and genuine quorum loss stays unknown. Removed 1 retains its earlier payload outside later group work; the test does not require removed replicas to receive later writes. |
| State P2-2 | A's acknowledged payload11@2 versus B's internally valid payload99@2 checkpoint advanced to3 returned Ok when no target was eligible: observed RED. `install` now checks all retained common committed history before recipient selection; conflict returns Quarantined. An internally valid nonconflicting future cut returns explicit InvalidImage when no local compaction recipient is eligible. `future_install_tests.rs` checks exact unchanged files/images/view/receipts/Entry bytes and preserved same-history compaction plus physical restart. No hidden successful no-op remains. |
| Code P3-1 | Coverage was GREEN against existing binding refusal, not a fabricated RED. `foreign_checkpoint_tests.rs` changes both outer and application envelope to foreign group71, independently parses the complete supported envelope/profile and original noop/Create history, compares every typed result and the full configuration/materialized application tail, and normalizes both bindings back to70 to obtain byte-identical original checkpoint plus successful fixed-profile restore. The two-entry fixture has no configuration-intent group fields to mismatch. Actual target refuses WrongBinding before any file/state/reply exposure. The old consumer case is accurately labelled outer-only mismatch. Foreign runtime support was not added. |
| Code P3-2 | Coverage was GREEN against existing admission guards. `configuration_admission_tests.rs` uses fresh correctly versioned stable LeaveJoint and nested EnterJoint, obtaining NotJoint and JointInProgress rather than StaleVersion. It compares complete physical files, durable images, logs/configuration and view, verifies no outcomes for rejected keys, then succeeds at legitimate AddLearner/EnterJoint/LeaveJoint and retains the original application retry. |

The common publication stop flag and routing guard change shared failure/mutation paths under existing obligations. The owner has required fresh Code/State renewal under review-loop §5 in addition to originating counterexample closure. Public interfaces, library roles, dependency edges, compatibility profile, third-party dependencies, process-global exceptions, platform boundaries and test selection retain their reviewed shapes. No policy relaxation, production integration or desktop change is part of remediation.

### Corrected verification

The initial remediation verification below was run after those source changes and formatting; the later bounded follow-up has its own final verification recorded below. The reproduction commands above remain applicable; additionally run these focused/affected checks without a cross-target name filter:

```sh
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib q3
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot
```

Remediation checkpoint before the bounded malformed-success follow-up below: **119 PASS, zero failures, four explicit tier ignores**. Private Q3 host cases: **27 PASS** (six added owning cases); original concrete consumer tests **19/19 PASS**, zero ignored/skipped/filtered. The default run includes all preserved Q1a/Q2, V2, compiler-contract and additional consumer witnesses listed in the historical checkpoint above. The two explicit Q2 library witnesses were separately run: **2 PASS**. Strict all-target Clippy, formatting and `check.sh` pass; architecture/process-global/source boundaries report **49 owned Rust files, zero exceptions/debt/permanent entries**. Both external worker executables were freshly selected with Cargo `--no-run`.

After this patch, Q2 parent-oracle self-tests **4 PASS**; Q3 adversarial parent-oracle self-test **PASS** (ten recovered-field mutations plus missing/changed parent originals). Actual external Q2 SIGKILL **write-ack/write-cut: 2 PASS**. Actual external Q3 SIGKILL **ack/joint-before-apply/snapshot-before-apply: 3 PASS**, each comparing complete parent-held APP/CONFIG/ENTRY lookup/exact retry/typed replay. The joint-before-apply expected ConfigReceipt remains independently derived in the parent, not taken from recovered agreement. Within-process fault injections, actual process termination and unqualified power-loss remain distinct evidence classes.

| Pre-follow-up remediation run on the same local host, cached target | Observed wall time |
| --- | ---: |
| Focused private host Cargo build+tests, 27 cases | 3.216 s |
| Complete default workspace Cargo build+tests, 119 cases | 15.141 s |
| Two explicit Q2 library witnesses | 0.141 s |
| Worker `--no-run` | 0.046 s |
| Q2 parent-oracle self-tests / Q3 parent-oracle self-test | 0.051 s / 0.027 s |
| Actual two Q2 SIGKILL cuts / three Q3 SIGKILL cuts | 0.290 s / 0.915 s |

These remediation observations include Cargo's work for each command; no new cold-build claim is made. Historical initial cold/warm measurements are preserved above and predate remediation. Logs are `/tmp/glade-q3-rem1-{host-focused,workspace,q2-explicit,workers-build,q2-oracle,q3-oracle,q2-crash,q3-crash}.log`; measurements are `/tmp/glade-q3-rem1-measurements.json` and `/tmp/glade-q3-rem1-crash-measurements.json`. The durable evidence is the named repository cases and reproducible commands, which the owner independently verifies before freezing.

Source organization: the split-files skill was used at GREEN on the initial paired coverage module. Syntax-aware `rust-split explode` and ordered split manifests reproduced its bytes exactly; the two owning test items, including their ordinary `#[test]` attributes, were moved into separate foreign-checkpoint and configuration-admission modules with no visibility widening. The transient combined module was removed. Formatting and both relocated tests were checked separately, then the full matrix passed. New modules remain below 500 LOC; no unrelated refactor or broader cfg migration is claimed.

All five originating findings remain **open pending independent closure**. Neither the focused GREENs nor the complete matrix is reviewer acceptance. The owner records the exact corrected object and final decisions in the canonical review cycle.

### Bounded malformed-success follow-up within Code P2-1

Before the owner freeze, the owner identified that the central stop rule still covered store Err but not an Ok response failing the host's instance/revision/image validation or checked revision arithmetic. `actual_publication_with_malformed_success_stops_until_physical_reopen` in the existing `install_fault_tests.rs` adds one bounded owning case. Its actual V2-backed wrapper first publishes and syncs the valid image, then corrupts only the returned State. The instance-mismatch branch observed behavioral RED: direct install returned Quarantined, yet `outcome` returned the full original acknowledged receipt from memory. The exact compiling RED is saved at `/tmp/glade-q3-rem1-malformed-success-red.log`.

The existing publication result and post-publication response validation now share one failure handler. The exact originating Error is retained in the voter and returned: Quarantined for malformed instance/revision/image success, CapacityExhausted for checked revision overflow. The original instance/revision/image validation order is preserved. No malformed returned state is installed into local memory. The four-case test asserts stopped receipt/resource replies, stopped queued source messages, unchanged cached state, genuine close/open of the valid durable files, exact persisted checkpoint/revision/binding and complete original application receipt/Entry lookup/retry/typed replay after reopen. The fourth case deliberately sets the private cached host revision to MAX and allows the wrapper to publish at the genuine V2 revision; it isolates post-publication checked arithmetic. It is not a claim that the real adapter permits revision wrap or that this malformed state occurs during ordinary recovery.

Final corrected default workspace after the follow-up: **120 PASS, zero failures, four explicit tier ignores**. Focused private Q3 host: **28 PASS**, including all ten direct/CatchUp real-fault cases and this four-case malformed-success regression. The workspace again includes all **19 original concrete consumer tests**, unchanged and default-selected. Strict all-target Clippy, formatting and `check.sh` pass: **49 owned files, zero exceptions**. No new module, dependency, contract, allowlist or test-selection change was needed; the existing owning test module is 357 LOC.

| Final follow-up command, cached local target | Observed wall time |
| --- | ---: |
| New malformed-success regression Cargo build+test | 1.404 s |
| Focused private Q3 host Cargo+tests, 28 cases | 2.408 s |
| Complete default workspace Cargo build+tests, 120 cases | 11.546 s |
| Strict all-target Clippy | 0.534 s |
| `check.sh` / formatting check | 0.793 s / 0.116 s |

Final follow-up logs are `/tmp/glade-q3-rem1-{malformed-success-green,host-final,workspace-followup,clippy-followup,gates-followup,fmt-followup}.log` and `/tmp/glade-q3-rem1-followup-measurements.json`. Earlier explicit Q2 library/oracle/SIGKILL and Q3 oracle/SIGKILL PASS observations above predate this bounded follow-up; they are preserved as such, not relabelled final-source executions. The owner MUST rerun the complete independent verification on the settled object before freeze/renewal. All originating findings remain open pending independent closure; this follow-up is part of the same merged remediation round 1.

### Owner independent verification of the complete corrected patch

After the drafter's final STOP EDITS, the owner independently reran the whole
private workspace: **120 PASS, zero failures, four explicit tier ignores, zero
filtered**, including the unchanged nineteen original concrete consumers. The
separately selected two Q2 library cases pass. Architecture, formatting, explicit
source boundaries and strict all-target Clippy pass; process-global scan reports
**49 owned Rust files, zero exceptions**. Cargo `--no-run` again reports the exact
Q2/Q3 worker executables listed above. Both parent-oracle self-tests pass and the
actual Q2 write-ack/write-cut and Q3 ack/joint-before-apply/snapshot-before-apply
SIGKILL cuts all pass on the final corrected source. These are owner executions,
not reviewer verdicts. Logs: `/tmp/glade-q3-owner-rem1-{workspace,q2-explicit,gates,
clippy,workers,q2-oracle,q3-oracle,q2-crash,q3-crash}.log`; measured commands are
recorded in `/tmp/glade-q3-owner-rem1.json`. Observed workspace time was 5.261 s;
actual Q2/Q3 kill batches 0.276/0.917 s; no performance guarantee follows.

All four repository HEADs retain the remediation-start tuple. Only the intended
root correction, owning tests and evidence/remediation documents are staged for
the next frozen object; unrelated member/untracked work is excluded. Fresh
Code/State renewal and originating finder closures MUST verify that exact
corrected source tuple before acceptance.
