# IC-3 pre-remote qualification prerequisite

Date: 2026-10-04. Status: **strict pre-remote prerequisite GREEN; C actual-node integration remains unimplemented.**

Combined B acceptance was actually committed at root `434df3f4aae0cfc116072db95a294283978d0c6e`, naming the reviewed source root `f0b1c325f9e0eff27b4eefadac8084bff3505e73` / Glade `a47691598df648eb8c9554b27f3d06b0cffcf596`. Its seven independent GO reports and factual Fence erratum are retained verbatim. [Acceptance](GladeIndependentCrdtProductionIntegrationPhysicalHost-Acceptance.md) accepts genuine scoped authentication and bounded Unix LocalProcessRestart persistence only.

## Genuine committed evidence and source pins

`glade/node/qualification/ic3-b1.json` maps eight required genuine-provider witnesses to actual strict-signature, current-policy/time, possession/replay and historical-admission assertions in `node/tests/ic3_auth.rs`. Its accepted_review_record is the real root acceptance commit above, not a synthetic identifier. The unchanged authentication implementation remains Glade `a47691598df648eb8c9554b27f3d06b0cffcf596`.

The final qualification JSON is committed at Glade `a0e1cb7cdd26ea9cde9328889a0d15a44a77ef8d`; SHA256 `193601087da049dfad2b5726cbee55967045f1af841d8167267cc309b6817f72`. The final representation implementation pin is `9cc4ac318b70b9e3a359c4142f0bbe3314c1dfed`, and the containing final pins commit is `3bfdea5921d0e500476379557dc79813ca568c07`. All27 committed codec hashes match. The authentication source hashes are unchanged across these commits.

## Negative-fixture correction and chronology

The first full prerequisite run passed the production strict gate but failed two negative tests that read the live qualified manifest while assuming it remained unqualified. The correction changed only `compat/test_gate.py`: the uncommitted-candidate test now explicitly constructs that state in a copied fixture and proves non-strict acceptance before strict refusal; the absent-crypto test explicitly clears the evidence in its own negative fixture. Original test names, refusal assertions and production gate/schema/bytes are preserved. The fixture hashes are derived from their own source bytes so source re-pinning cannot accidentally substitute a different failure. This test-only correction belongs to the pending aggregate C review; it is not a new authentication implementation or acceptance.

Executed first RED and the intermediate stale-fixture-hash failure remain chronological evidence. The final focused two tests passed before parent committed/re-pinned. The final complete command was:

`sh glade/contracts/crdt-recovery-codec/check-vectors.sh --pre-remote`

It exited0 in1.524s wall: committed corpus/genuine B1 accepted-evidence gate PASS, all6 gate tests PASS, all4 Rust vector tests PASS, Python21positive plus negative controls PASS, TypeScript21positive/18negative PASS. No zero-selected witness, synthetic Facts or missing-evidence bypass is used. This is a measured warm local prerequisite, not a cold-build/scale guarantee. Node emitted its existing experimental transform-types warning.

Raw logs: first RED `c1c56f840e3733304c273a02aec726489c300454d20f7272f9d9955b79d94fdc`; fixture correction `2a164f83b4dbcd1e35bca4b3ec146d1cb51e4b2c5c9b1447a33b6eabb9aa4c60`; complete GREEN `7dcda3450f4d8f84d5a920982a3e1a74cc88ae01ded02159c47a951edc0504dc`. The [compressed chronology](GladeIndependentCrdtProductionIntegrationPreRemoteQualificationRunLog.md.gz) retains all three logs without editing; SHA256 `ac3da4ea67b1df51a2fea5cedf5c4a48a5a95260378bec1b53514b6f447dc9d3`.

## Next authorized scope

The existing canonical values now may be used by C's actual-node development witness. A changed remotely used canonical value MUST satisfy its independent language/committed-content prerequisite before history execution; this GREEN does not cover future source or format changes. C still requires actual Settings/NodeStart/node_plan/Assembly routing, automatic guarded two-node exchange, complete retained history/obligations, physical process witnesses and all aggregate reviews. Generic orphan Active guards remain pending/incomplete and cannot be reset by restart.

No IC-4/client activation, Raft integration, live keys/stores/floors, launcher defaults, desk rebuild, enrollment, migration, seal, push, or arbitrary rollback/power-loss/quorum qualification is implied. Historical caps and original A2 closure deferral to ABC remain unchanged.
