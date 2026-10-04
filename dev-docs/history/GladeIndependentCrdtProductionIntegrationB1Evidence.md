# IC-3B1 genuine authentication evidence

Candidate checkpoint, 2026-10-04. Sole source writer; no Git, deployment, live key/store or parent review-ledger changes. The source tuple before this patch is Glade `c69e6416f5f5155d4bb570bf272e796b2deae0a3`, corrected implementation `3bf66efecc59fd24261e13787e4c8fc3f822ca57`. Working bytes are pinned separately and do not belong to that old commit. [Source pins](GladeIndependentCrdtProductionIntegrationB1SourcePins.json) are candidate content pins until parent settlement. Combined B review remains pending. The owner-deferred typed review is not substituted for B acceptance.

Exact commands/output/exit/wall durations, including failed and abandoned attempts, are preserved chronologically in [the gzip run log](GladeIndependentCrdtProductionIntegrationB1RunLog.md.gz). No original A2/remediation report, evidence, source pin or run log was rewritten.

| Requirement | Actual behavioral witness |
| --- | --- |
| IC3-ID-001 / AUTH-001 | Full root-signed namespace/declaration/schema/corpus validation; exact canonical signatures; wrong key/domain/scope/identity/trailing controls; strict small-order Ed25519 rejection |
| AUTH-002 | Same-provider node challenge and writer response bound to channel/session/op; replay/foreign-provider refusal; each local verification needs fresh possession |
| AUTH-003 | Provider-owned trusted signed policy and conservative interval, expired/revoked/uncertain controls; observed in-process clock/policy regression refusal |
| AUTH-004 | Exact provider-owned verified/sealed query and current cut, recomputation rejects changed caller Facts; independently observed protected Started and its physical retention remain B2-pending |
| AUTH-005 | Genuine writer-signed op-proof/root-signed certificate/permit and original-admission preservation under changed current cut; exact operation/resource/policy/sequence bindings, old receipt body identities, reused origin/epoch/issuance refusal and authenticated bare rival distinct from qualified admitted rival |
| CANON-001 | Existing canonical proof-body grammar and actual released consumer payload rows; strict remote prerequisite remains refused pending accepted B evidence |

The first compiling RED used existing `RefusingEvidence` against genuine signed historical proof/current observation. Bad-signature/domain/namespace and authenticated invalid-payload tests also produced assertion RED before implementation. The initial genuine adapter made four groups GREEN. Later source-floor regression and repeated-local-verification tests produced compiling RED before the corresponding in-process fixes. One intermediate missed helper parameter was a compilation failure, explicitly not behavioral RED. Unsupported genuinely signed corpus, duplicate JSON-property behavior and oversized genuine operation each produced assertion RED before correction. Every such output is in the run log.

The owner excluded newly introduced BOM-specific rows/support/documentation. Three rows and special Rust support were removed; Glial source remained untouched. Earlier raw attempts remain historical, but current qualification is the retained 32-row scope and makes no blanket byte-level JSON equivalence claim. The unrelated broader JSON/profile/parser behavior was not redesigned.

Final checks:

- `ic3_auth`: 16/16 pass, test execution 0.05 seconds; actual released Glial consumer: 32 rows pass.
- Existing default `ic3_boundary`: five pass, one physical success witness remains explicitly ignored until B2/C; it is not a durability witness. Actual assembled default providers remain refusing.
- Existing actual `assembly` target: 30 pass; pinned signing unit target: three pass. The first mistaken `--lib node_assembly` selector ran zero tests and is recorded; corrected `--test assembly` actually selected 30.
- `contracts/check.sh crdt-production`: PASS for architecture, retained tests, formatting and strict Clippy, including original core, complete operation encoder, storage-attempt and codec controls. The existing source gate recursively parses the new node independent subtree, including disabled sections.
- Node adopting architecture: PASS. Process-global ratchet: 112 files, unchanged three permanent entries, no new debt/allowlist change.
- Node strict Clippy remains blocked by nine unchanged baseline diagnostics in claims/exchange/iroh_carrier/mesh/server/ws. A diagnostic enumeration found one new single-element test-loop warning; it was removed. Final enumeration has exactly the nine baseline library warnings and no new provider/test warning. No global lint exception, dependency allowance or classification was loosened.
- `check-vectors.sh --pre-remote`: expected refusal, `genuine B1 crypto qualification absent`. No candidate artifact was relabeled accepted.

Authentication facts are derived by the actual provider using genuine signatures. Test fixtures provision deterministic scratch seed material and signed bodies, never successful Facts or storage receipts. Historical bodies identify the configured LocalProcessRestart promise, but this B1 checkpoint does not assert that the physical promise has been satisfied. Conditional seals are not durable acknowledgements. No disk host, durable restore, monotonic across-restart floor, two-process exchange, production configuration or live activation is claimed. B2 must supply those physical witnesses before combined component qualification.

The auth module was split by the installed syntax-aware `rust-split` tool after tests were GREEN into provider types/port methods and operation/lifecycle helpers; exploded ordered chunks reproduced the old source byte-exact before applying the bounded split. Only internal sibling access was widened. The 12-group test checkpoint passed before and after the split; final 16-group checkpoint includes the later controls.

## Parent source settlement

Glade implementation commit `e8097861c9559ce9753b75231ee2aac8f9e28064` contains all 14 source files at
the recorded hashes. Parent verified committed and working bytes and unchanged
external dependency pins. This settles the genuine in-process implementation,
not physical B2 qualification or an accepted pre-remote review record. The root
commit filing this evidence settles the documentary checkpoint.
