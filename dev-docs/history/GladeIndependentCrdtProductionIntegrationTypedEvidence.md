# IC-3A2 typed implementation evidence

Status: frozen source-qualified candidate for fresh Code, State and docs-only Surface review. No acceptance is asserted here. Parent owns review dispatch, ledger and commits. IC3B genuine evidence/disk success and IC3C real two-node transport remain pending.

## Source object and authority

Semantic source: workzone `06b16c9e17ff5507268a5823fad9a0be70a36790`; acceptance record `3bff20351895b82e2f805e7f8939dab6b3296604`. Implementation source: Glade `7d26ba6e6d133db650a37ff49eba71644ea370a6`. Metadata-only Glade head: `8b0595551dd90f32da4698fff9358aa30dbf2548`. All 58 implementation hashes match both the implementation commit and the final working tree; the Glade working tree is clean. The later metadata revision does not replace the implementation pin.

The exact source-file list/hashes and root document hashes are in [TypedSourcePins](GladeIndependentCrdtProductionIntegrationTypedSourcePins.json). This is the complete source object; the vector metadata file is explicitly excluded from its implementation hash set. The external Gyld object is six exact files listed below. Root review ledger and existing archived documents/reports were never edited by this drafter.

The [typed contract](../GladeIndependentCrdtProductionIntegrationTypedContract.md) names exact package roles, interfaces, codec grammars, replacements and trust limits. The [cold consumer guide](../GladeIndependentCrdtProductionIntegrationTypedUsage.md) is the separate Surface object and stands on its own.

## Executed result and retained controls

- Contracts adopting gate `sh glade/contracts/check.sh crdt-production`: PASS architecture, scoped tests, formatting and Clippy `-D warnings`. New suites: data 4, evidence 5, recovery 11, codec 19. Core 106 = original 105 plus one new extraction-boundary assertion; storage-attempt API retains all 34. No whole-workspace default was substituted.
- `sh glade/contracts/arch002-fixture.sh`: PASS; framework injection refused in every one of the 16 classified packages. `test-selection.sh`: PASS old selectors and explicit new affected sets.
- Actual node architecture gate: PASS. Actual assembled IC3 boundary: 3 negative controls PASS; 1 future persistence capability test remains ignored after its explicit RED. Old affected node assembly 30, registration 1 and doctests 5 PASS. The existing assembly's cryptographic tests are compatibility controls, not proof that the new IC3 evidence provider works.
- Released text contract and JavaScript source check: PASS. The actual trace consumer requires exactly its original ten text witnesses; original operation/private batch encoder bytes and text consumers are unchanged.
- Process-global source guard: PASS, 107 files, original three permanent entries and no new allowlist/global state. Selected syntax-aware tests inspect disabled conditional branches and the new node files without pretending broader existing migrations are complete.
- Rust typed producer, independent TypeScript and independent Python: all 21 named semantic positives / 18 categories agree with exact bytes/digests; 18 malformed/shape controls retained. Five pin/refusal controls PASS. These are representation witnesses with symbolic signatures, not genuine authenticated admission.
- Candidate/committed-content representation gate PASS. Actual `check-vectors.sh --pre-remote` returns exit 1 after committed source verification with `genuine B1 crypto qualification absent`. This is the required pending gate, not a successful remote qualification.
- Gyld exact frozen 34/135 plus new 39/160 allocation/obligation controls, corrupted/missing pins, missing requirement, inherited declaration and standalone load controls PASS. Actual standalone capture CLI emits 39/160 and explicit no-runtime/no-host limits. Source-sidecar-focused five controls PASS after final document/pin update; adopting architecture PASS.
- Gyld full architecture selector: 181 tests PASS, execution 2.860 seconds against owner-approved 5.0 seconds. Pinned Ruff 0.13.0 repository-wide check and format PASS (52 files). Mypy 1.18.1 targeted check is **not green**: six pre-existing errors, with the exact unchanged-HEAD baseline comparison reporting the same six. No ignore, allowlist or budget was used to conceal that debt.

Forty-five protected original source hashes were captured before editing; exactly two differ: the core types file (moved exactly into data with public reexports) and core source-boundary tests (one appended extraction assertion). The other 43 original files retain their bytes, including every original behavioral assertion, storage-attempt API, kernel algorithm, private encoder and affected Glial consumers. [CompatibilityPins](GladeIndependentCrdtProductionIntegrationTypedCompatibilityPins.json) preserves the original 45-file hashes.

## TDD chronology and corrections

The complete chronological runner output is retained byte-for-byte in the compressed run log linked below. Its headings sometimes described an intended GREEN/RED outcome before execution; the actual exit/output is authoritative. The table records every runner attempt and exit, including zero-selected, argument, compilation and wrong-baseline attempts. Such attempts do not qualify as behavioral RED.

The actual compiling assertion REDs covered independent data ownership; codec refusal/roundtrip; owned receive including async cancellation and foreign issuer; old batch decoder; explicit type-domain identity; cumulative interned-blob expansion; duplicate receive identity; exact proof body; accepted full zero-terminated domains; canonical remote maps; missing vector categories; malformed signature/domain shape; Gyld 34-versus-39 overlay; missing representation pin/language evidence; strict pre-remote candidate refusal; and owner-approved timing threshold. A skeleton type/interface was sometimes needed to compile its first behavioral assertion; successful physical adapters were never introduced.

Corrections are retained honestly:

1. The first data filter selected zero tests; rerunning the complete exact name produced the actual assertion RED.
2. An early sync gate sketch was changed back to refusal before its RED. The subsequent owned asynchronous gate had its own compiling RED before implementation. Cancellation/Pending controls remain.
3. The first batch decode GREEN attempt actually failed descriptor key field order. Correcting order made the unchanged-encoder witness pass. The amplification RED's enormous assertion output is preserved compressed rather than pasted into this report.
4. Wrong test argv and compiler failures (including optional `Head.hash` and stale floor schema) are structural/error attempts. They are not counted as behavioral failures.
5. The first node lockfile regeneration updated unrelated registry pins. That run was cancelled/disqualified; original pinned dependency bytes were restored and the actual persistence placeholder RED rerun at that graph. No registry pin change survives.
6. The new architecture checker initially refused unclassified roles and later an unsupported `#[path]` conformance target. The reviewed package roles were adopted; the old-path batch fixture remains an explicitly executed test. No allowlist was broadened.
7. A raw reordered-map test initially used a serializer that sorts maps; the test was corrected to actual raw noncanonical bytes. This was a test construction correction, not evidence that noncanonical decoding was allowed.
8. The first strict-candidate test passed for the wrong reason (changed source pin). Pins were refreshed, the actual strict refusal test then failed, and only then strict mode was implemented. A following generated newline caused a syntax error before the corrected gate passed.
9. A Gyld run lacked `PYTHONPATH=src:.` and failed import. The corrected explicit environment run passed. A relative rustfmt attempt from the Gyld cwd found no file; the correct root path was formatted before the passing adopter gate. A baseline temp-workdir command was rejected before execution because the directory did not yet exist; it was created from read-only copies and rerun.
10. Original 179-test Gyld gate passed assertions but failed its two-second budget at 2.978 seconds. The owner explicitly said “up it to 5 seconds” on 2026-10-04. Executable boundary regression first failed at 4.999 seconds against two; the smallest runner edit then passed five-second lower/upper edges while retaining multi-selection ten, separately measured startup and unbounded I/O.
11. An exploratory unchanged-snapshot reuse test/API was added and produced RED, but abandoned after that owner decision. It was removed; no cache or selector redesign survives. Its multi-MB snapshot assertion remains in the compressed history.
12. The first five-second full selector attempt ran during concurrent Rust adopter/node builds and passed assertions but failed at 5.251 seconds. No threshold was increased again. An isolated cost diagnosis found original 176 tests at 2.081 seconds and the five additions at 1.092; after the competing jobs completed, the normal unchanged selector passed 181 at 2.860 seconds. Both failures remain evidence; this report does not relabel them GREEN.

## Mypy baseline debt

The candidate and a read-only temporary copy restoring the exact pre-change HEAD runner/test bytes both report:

| File | Existing diagnostics |
| --- | --- |
| `scripts/check_architecture.py:350,354` | object iteration / `.items()` types |
| `scripts/test_fast.py:118,120` | old `suite` string/TestSuite variable collision |
| `tests/test_architecture.py:400,1111` | optional cleanup / old unannotated offered map |

These six diagnostics remain visible and unresolved; fresh reviewers must assess the scoped candidate with this exact baseline. No type-check success is asserted for those files.

## Limits requiring B/C

Codec success proves exact bounded structural custody, not that the structural test image is a semantically valid recovered State. The image fixture deliberately contains symbolic facts and some inconsistent cross-record combinations to prove all fields survive. No node provider currently returns `LoadResult::Validated` or an authenticated IC3 `Facts`/seal. B must validate every saved request, issuance floor, attempt phase, terminal/receipt, accounting, identity and genuine proof, with actual process-kill/reopen and all-crash-cut witnesses.

The owned gate proves ownership/drain state of the supplied future. The actual B/C node continuation must include and join every carrier task; arbitrary detached work is not prevented by a generic Rust future's type alone. No disk writes, stable exclusive root lock, trusted external rollback/clone protection or successful guard persistence are qualified here. Unknown/failed writes must retain guard/intent/loss and cannot falsely clear old history. The placeholder persistence RED uses the actual assembly refuser and a refusing session; it is not a configured kill/restart physical witness.

Historical admission/current policy/time, authenticated transport versus resource permission, exact duplicate receipts/forks, whole bounded inventory/backpressure and instance Y independence remain mandatory accepted obligations. C must use two independently owned actual `glade-node` processes through the production assembly/runtime path. No ferry, browser activation, live migration or deployment/key changes occurred. IC4 remains separate.

## Chronological actual command index

| # | Recorded stage | Exact runner command (argv) | Actual exit; wall |
| --- | --- | --- | --- |
| 1 | Data extraction structural RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-admission-core', '--test', 'source_boundaries', 'ic3_data_boundary', '--', '--exact']` | 0; wall: 1.375s |
| 2 | Data extraction actual structural RED (full exact name) | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-admission-core', '--test', 'source_boundaries', 'ic3_data_boundary_is_independent_of_the_pure_algorithm', '--', '--exact']` | 101; wall: 0.060s |
| 3 | Data extraction GREEN and original behavioral controls | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-admission-data', '-p', 'glade-crdt-admission-core', '-p', 'glade-crdt-storage-attempt-api', '--all-targets']` | 0; wall: 12.316s |
| 4 | Codec behavior RED against refusing scaffold | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip']` | 101; wall: 1.064s |
| 5 | Ingress behavior RED against refusing scaffold | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-api', '--test', 'public_contract']` | 101; wall: 0.786s |
| 6 | Canonical identity and inventory behavior GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip']` | 0; wall: 1.460s |
| 7 | Owned asynchronous ingress RED before capability implementation | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-api', '--test', 'public_contract']` | 101; wall: 0.885s |
| 8 | Owned async receive GREEN with finite retained issuer history | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-api', '--test', 'public_contract']` | 0; wall: 0.871s |
| 9 | IC2 version-one batch decoder behavior RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'batch_decode']` | 101; wall: 1.244s |
| 10 | IC2 unchanged batch decoder GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'batch_decode']` | 101; wall: 1.144s |
| 11 | Correct encoder-order IC2 batch GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'batch_decode']` | 0; wall: 0.930s |
| 12 | Representation type-domain regression RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip', 'type_domain_cannot_substitute_plan_identity_for_receive_guard', '--exact']` | 1; wall: 0.031s |
| 13 | Representation type-domain actual compiling RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip', 'type_domain_cannot_substitute_plan_identity_for_receive_guard', '--', '--exact']` | 101; wall: 0.791s |
| 14 | Representation explicit type-domain GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip']` | 0; wall: 1.208s |
| 15 | New exact affected selectors structural RED | `['sh', 'glade/contracts/test-selection.sh']` | 1; wall: 0.103s |
| 16 | New reviewed contract package gate structural RED | `['cargo', 'run', '--quiet', '--locked', '--offline', '--manifest-path', 'glade-discover/tools/architecture-check/Cargo.toml', '--', 'glade/contracts']` | 1; wall: 0.585s |
| 17 | Exact new selectors GREEN retaining original selectors | `['sh', 'glade/contracts/test-selection.sh']` | 0; wall: 0.151s |
| 18 | Reviewed package classifications GREEN | `['cargo', 'run', '--quiet', '--locked', '--offline', '--manifest-path', 'glade-discover/tools/architecture-check/Cargo.toml', '--', 'glade/contracts']` | 1; wall: 0.453s |
| 19 | Package architecture GREEN with executable source-inspected targets | `['cargo', 'run', '--quiet', '--locked', '--offline', '--manifest-path', 'glade-discover/tools/architecture-check/Cargo.toml', '--', 'glade/contracts']` | 0; wall: 0.407s |
| 20 | Node IC3B actual compiling persistence success RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/node/Cargo.toml', '--test', 'ic3_boundary', 'production_composition_recovers_a_genuinely_persistent_validated_image', '--', '--ignored', '--exact']` | -2; wall: 29.921s |
| 21 | Pinned original node graph actual persistence RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/node/Cargo.toml', '--test', 'ic3_boundary', 'production_composition_recovers_a_genuinely_persistent_validated_image', '--', '--ignored', '--exact']` | 101; wall: 9.188s |
| 22 | Batch interning allocation-amplification regression RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'batch_decode', 'interned_blob_references_cannot_amplify_owned_decoding_past_budget', '--', '--exact']` | 101; wall: 1.728s |
| 23 | Batch bounded cumulative allocation GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'batch_decode']` | 101; wall: 0.370s |
| 24 | Full-body codec and cumulative batch allocation GREEN after schema update | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'batch_decode', '--test', 'roundtrip']` | 0; wall: 1.986s |
| 25 | Duplicate consume identity regression RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-api', '--test', 'public_contract', 'issued_guard_identity_is_not_reissued_by_same_session', '--', '--exact']` | 101; wall: 0.999s |
| 26 | Finite owned-byte receive lifecycle GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-api', '--test', 'public_contract']` | 101; wall: 0.895s |
| 27 | Actual NodeAssembly refusing seam and foreign capability GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/node/Cargo.toml', '--test', 'ic3_boundary']` | 0; wall: 5.747s |
| 28 | Finite unique consume history GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-api', '--test', 'public_contract']` | 0; wall: 0.721s |
| 29 | Exact signed-proof body structural behavior RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip', 'proof_body_requires_exact_purpose_version_signature_shape_and_type', '--', '--exact']` | 101; wall: 1.163s |
| 30 | Exact signed-proof body structural GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip']` | 0; wall: 0.957s |
| 31 | Complete owned image and nonrecursive floor representation controls | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'image']` | 0; wall: 1.731s |
| 32 | Accepted exact signature domain regression RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip', 'accepted_signature_domains_are_full_and_zero_terminated', '--', '--exact']` | 101; wall: 0.803s |
| 33 | Accepted exact domains and versioned body contexts GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip']` | 0; wall: 1.392s |
| 34 | Canonical remote profile map behavior RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip', 'remote_profile_is_exact_canonical_integer_key_map_without_recovery_magic', '--', '--exact']` | 101; wall: 1.269s |
| 35 | Remote proof map and full owned-image representation GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'roundtrip', '--test', 'image', '--test', 'batch_decode']` | 0; wall: 4.050s |
| 36 | Required independent vector-category behavioral RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'vectors']` | 101; wall: 0.782s |
| 37 | Python independent profile bytes/digests and bounded negatives | `['python3', '-B', 'glade/contracts/crdt-recovery-codec/compat/python.py', 'glade/contracts/crdt-recovery-codec']` | 0; wall: 0.030s |
| 38 | TypeScript independent profile bytes/digests and bounded negatives | `['node', '--experimental-transform-types', 'glade/contracts/crdt-recovery-codec/compat/typescript.ts', 'glade/contracts/crdt-recovery-codec']` | 0; wall: 0.096s |
| 39 | Rust actual typed independent canonical vector agreement | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'vectors']` | 101; wall: 0.824s |
| 40 | Rust typed canonical vectors GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'vectors']` | 101; wall: 1.408s |
| 41 | Malformed signature/domain envelope shape RED | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'vectors', 'malformed_signature_length_and_wrong_exact_domain_refuse_signed_envelope_shape', '--', '--exact']` | 101; wall: 1.009s |
| 42 | Exact Rust typed vector and malformed-envelope controls GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--test', 'vectors']` | 0; wall: 1.473s |
| 43 | Python canonical profile and malformed-domain negatives GREEN | `['python3', '-B', 'glade/contracts/crdt-recovery-codec/compat/python.py', 'glade/contracts/crdt-recovery-codec']` | 0; wall: 0.071s |
| 44 | TypeScript canonical profile and malformed-domain negatives GREEN | `['node', '--experimental-transform-types', 'glade/contracts/crdt-recovery-codec/compat/typescript.ts', 'glade/contracts/crdt-recovery-codec']` | 0; wall: 0.106s |
| 45 | Refactored typed/codec/source branch suites GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-admission-data', '-p', 'glade-crdt-evidence-api', '-p', 'glade-crdt-recovery-api', '-p', 'glade-crdt-recovery-codec', '--all-targets']` | 0; wall: 11.483s |
| 46 | Focused typed contract Clippy | `['cargo', 'clippy', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-admission-data', '-p', 'glade-crdt-evidence-api', '-p', 'glade-crdt-recovery-api', '-p', 'glade-crdt-recovery-codec', '--all-targets', '--', '-D', 'warnings']` | 101; wall: 1.147s |
| 47 | Focused typed contract Clippy GREEN | `['cargo', 'clippy', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-admission-data', '-p', 'glade-crdt-evidence-api', '-p', 'glade-crdt-recovery-api', '-p', 'glade-crdt-recovery-codec', '--all-targets', '--', '-D', 'warnings']` | 0; wall: 0.281s |
| 48 | Gyld production allocation overlay behavioral RED | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests']` | 1; wall: 0.432s |
| 49 | Exact Gyld 39 allocations/160 obligations GREEN | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests']` | 0; wall: 0.453s |
| 50 | Final owned image, remote profile, unsigned boundaries and vectors GREEN | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/contracts/Cargo.toml', '-p', 'glade-crdt-recovery-codec', '--all-targets']` | 0; wall: 6.340s |
| 51 | Pinned representation gate actual behavioral RED | `['python3', '-B', 'glade/contracts/crdt-recovery-codec/compat/test_gate.py']` | 1; wall: 0.102s |
| 52 | Pinned independent language representation gate GREEN | `['sh', 'glade/contracts/crdt-recovery-codec/check-vectors.sh']` | 0; wall: 0.300s |
| 53 | Gyld exact ownership, negative pins and standalone source GREEN | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests']` | 1; wall: 0.109s |
| 54 | Gyld exact ownership, negative pins and standalone source corrected environment GREEN | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests']` | 0; wall: 0.880s |
| 55 | Frozen selectors and explicit new affected sets | `['sh', 'glade/contracts/test-selection.sh']` | 0; wall: 0.152s |
| 56 | Gyld adopting architecture gate | `['python3', '-B', 'scripts/check_architecture.py']` | 0; wall: 0.160s |
| 57 | Glade process globals exact unchanged allowlist gate | `['python3', 'glade/scripts/checks/check_process_globals.py']` | 0; wall: 0.582s |
| 58 | Affected node architecture adoption | `['cargo', 'run', '--quiet', '--locked', '--offline', '--manifest-path', 'glade-discover/tools/architecture-check/Cargo.toml', '--', 'glade/node']` | 0; wall: 0.656s |
| 59 | Unchanged text admission consumer and canonical trace | `['sh', 'glade/contracts/crdt-admission-core/check-text-contract.sh']` | 0; wall: 0.683s |
| 60 | Production contracts adopting architecture tests fmt clippy | `['sh', 'glade/contracts/check.sh', 'crdt-production']` | 1; wall: 18.772s |
| 61 | Gyld architecture affected fast selector unchanged budget | `['python3', '-B', 'scripts/test_fast.py', 'architecture', '--affected']` | 1; wall: 3.171s |
| 62 | Gyld exact unchanged validated snapshot reuse behavioral RED | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests.test_unchanged_validated_production_snapshot_reuses_without_recapture']` | 1; wall: 0.870s |
| 63 | Owner-approved five-second selector actual behavioral RED | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.FastExecutionBudgetTests']` | 1; wall: 0.197s |
| 64 | Owner-approved five-second selector behavioral GREEN | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.FastExecutionBudgetTests']` | 0; wall: 0.119s |
| 65 | Strict pre-remote candidate refusal actual behavioral RED | `['python3', '-B', 'glade/contracts/crdt-recovery-codec/compat/test_gate.py']` | 0; wall: 0.053s |
| 66 | Strict pre-remote refusal actual behavioral RED with valid candidate pins | `['python3', '-B', 'glade/contracts/crdt-recovery-codec/compat/test_gate.py']` | 1; wall: 0.044s |
| 67 | Candidate representation and strict pre-remote refusal GREEN | `['sh', 'glade/contracts/crdt-recovery-codec/check-vectors.sh']` | 1; wall: 0.026s |
| 68 | Candidate representation and strict pre-remote refusal corrected GREEN | `['sh', 'glade/contracts/crdt-recovery-codec/check-vectors.sh']` | 0; wall: 0.495s |
| 69 | Gyld owner-approved five-second architecture selector GREEN | `['python3', '-B', 'scripts/test_fast.py', 'architecture', '--affected']` | 1; wall: 5.686s |
| 70 | Production contracts scoped gate after new test formatting | `['sh', 'glade/contracts/check.sh', 'crdt-production']` | 0; wall: 8.273s |
| 71 | Contracts all sixteen classification refusal fixtures | `['sh', 'glade/contracts/arch002-fixture.sh']` | 0; wall: 9.169s |
| 72 | Actual assembled node refusing typed boundaries | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/node/Cargo.toml', '-p', 'glade-node', '--test', 'ic3_boundary']` | 0; wall: 9.315s |
| 73 | Gyld preserved baseline versus IC3 additions isolated timing diagnosis | `['python3', '-B', '/tmp/ic3-a2-profile.py']` | 0; wall: 3.545s |
| 74 | Candidate vector provenance and mandatory future remote refusal controls | `['sh', 'glade/contracts/crdt-recovery-codec/check-vectors.sh']` | 0; wall: 0.714s |
| 75 | Gyld final single architecture selector isolated after measured contention diagnosis | `['python3', '-B', 'scripts/test_fast.py', 'architecture', '--affected']` | 0; wall: 3.046s |
| 76 | Gyld pinned Ruff affected formatting | `['uvx', '--quiet', 'ruff@0.13.0', 'format', '--check', 'scripts/capture_glade_crdt_production.py', 'scripts/test_fast.py', 'tests/test_architecture.py']` | 0; wall: 0.079s |
| 77 | Gyld pinned Ruff affected checks | `['uvx', '--quiet', 'ruff@0.13.0', 'check', 'scripts/capture_glade_crdt_production.py', 'scripts/test_fast.py', 'tests/test_architecture.py']` | 0; wall: 0.087s |
| 78 | Gyld exact unchanged HEAD baseline Mypy comparison | `['uvx', '--quiet', '--python', '3.13', 'mypy@1.18.1', '--python-version', '3.11', '--check-untyped-defs', '--explicit-package-bases', 'scripts/capture_glade_crdt_production.py', 'scripts/test_fast.py', 'tests/test_architecture.py']` | 1; wall: 1.598s |
| 79 | Retained released text consumer source contracts | `['node', 'glial/test/independent_admission_source.mjs']` | 0; wall: 0.314s |
| 80 | Glade process globals final unchanged allowlist | `['python3', 'glade/scripts/checks/check_process_globals.py']` | 0; wall: 0.754s |
| 81 | Gyld repository pinned Ruff format check | `['uvx', '--quiet', 'ruff@0.13.0', 'format', '--check', 'scripts', 'tests', 'src']` | 0; wall: 0.197s |
| 82 | Gyld repository pinned Ruff check | `['uvx', '--quiet', 'ruff@0.13.0', 'check', 'scripts', 'tests', 'src']` | 0; wall: 0.224s |
| 83 | Gyld final scoped Mypy unchanged six-debt comparison candidate | `['uvx', '--quiet', '--python', '3.13', 'mypy@1.18.1', '--python-version', '3.11', '--check-untyped-defs', '--explicit-package-bases', 'scripts/capture_glade_crdt_production.py', 'scripts/test_fast.py', 'tests/test_architecture.py']` | 1; wall: 1.517s |
| 84 | Affected actual node assembly consumers and registration | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/node/Cargo.toml', '-p', 'glade-node', '--test', 'assembly', '--test', 'assembly_registration']` | 0; wall: 4.935s |
| 85 | Committed implementation representation pins and independent language controls | `['sh', 'glade/contracts/crdt-recovery-codec/check-vectors.sh']` | 0; wall: 0.498s |
| 86 | Strict pre-remote gate honestly refuses absent genuine B1 qualification | `['sh', 'glade/contracts/crdt-recovery-codec/check-vectors.sh', '--pre-remote']` | 1; wall: 0.208s |
| 87 | Source-qualified Gyld pin ownership negative and standalone gates | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests', 'tests.test_architecture.FastExecutionBudgetTests']` | 0; wall: 0.918s |
| 88 | Actual standalone Gyld production capture source-qualified CLI | `['python3', '-B', 'scripts/capture_glade_crdt_production.py', '--output', '/tmp/ic3-a2-production-capture-final']` | 0; wall: 0.435s |
| 89 | Actual node assembly compiler negative witnesses | `['cargo', 'test', '--locked', '--offline', '--manifest-path', 'glade/node/Cargo.toml', '-p', 'glade-node', '--doc']` | 0; wall: 1.666s |
| 90 | Final frozen Gyld doc/source sidecar exact ownership and source-pin controls | `['python3', '-B', '-m', 'unittest', 'tests.test_architecture.ProductionCrdtCaptureTests', 'tests.test_architecture.FastExecutionBudgetTests']` | 0; wall: 0.979s |
| 91 | Final frozen Gyld architecture adoption | `['python3', '-B', 'scripts/check_architecture.py']` | 0; wall: 0.164s |

## Exact external source freeze

| Gyld application file | SHA-256 |
| --- | --- |
| `examples/README.md` | `59f627c642808f31c9add2cfc9b5a7378bd2c426f61a402ebbb8713201c02377` |
| `examples/glade-crdt-production.gyld.py` | `608b07ca2378d43b79805447e17e9d45d77ed94625760850350fbb80ecad94d6` |
| `examples/glade-crdt-production-sources.json` | `290b3c475e9e2f6f6b749996689916d4e1bf454a353089d9d6edac460db49e86` |
| `scripts/capture_glade_crdt_production.py` | `70965e7f018441ab059b62620e9fd5b1124df37af54beebd916aff1ed4c250ab` |
| `scripts/test_fast.py` | `cc0acacecf660c0d486f3e808d52814817389f857139cc88ade92a34f32c8287` |
| `tests/test_architecture.py` | `8be923f29c55f4a0f0b60d6ceadd21078cd304b287423d25823b753792fa8952` |

## Complete raw output

[GladeIndependentCrdtProductionIntegrationTypedRunLog.md.gz](GladeIndependentCrdtProductionIntegrationTypedRunLog.md.gz) contains 3,676,067 uncompressed bytes from 91 runner attempts. Raw SHA-256: `4d53c0ce70ec9ff9fa7461d7be04d1b5507db3148af55773a16301b334e16289`. Compressed SHA-256: `4fa3539d7d3dbb9275fe8cf250afa148488686f4b4b757e425436424e6105199`. It preserves failures rather than repeating enormous payload/Snapshot representations in the review report.
