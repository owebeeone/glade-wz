# Q4-A legacy Store seal implementation — CODE-AXIS REVIEW

**Review object:** Internal whole-Store retirement interlock at workspace root `aafb14a663fe130db5ef56cb002b2bb9b9399b11` and Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`, governed by `dev-docs/GladeRaftLegacyStoreSealContract.md` and `dev-docs/GladeRaftProductionIntegrationPlan.md`. Implementation candidate awaiting independent acceptance; no production activation claimed.

**Baseline:** Workspace root `55b6ee30bb18fa58c80805be2d29a44b404077f6`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`. Scoped sources and controlling documents were read through `git show` at exact commits, with the scoped baseline diffs inspected. Builds used the exported source fixture, excluding inherited dirt.

**Date:** 2026-10-03

**Axis:** Architecture, interfaces, ownership, mutation call graphs, error paths, and compatibility reality. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its report or conclusions. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2, or P3 findings. This accepts the Q4-A implementation within its stated stable-root, participating-build macOS/APFS process-interruption profile.

---

## 0. Evidence base

The following HEADs matched at both the start and end of review:

| Repository | Exact revision |
| --- | --- |
| Workspace root | `aafb14a663fe130db5ef56cb002b2bb9b9399b11` |
| Glade | `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87` |
| glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| External Gyld | `ca04499a360d910fbf8ee2540ed446facd051b35` |
| grazel | `c839fe87c9d18ebb6e995964d2e79aef7cbd380e` |
| glade-gyld | `327d62c0033db0fae145d002a00663a3826b6d53` |
| glade-gwz | `35b38ba0845a7cb7034a4af3609975ea1bd48741` |
| glade-decl-rs | `b85044e1f6631114dbb290c02298e644f8363055` |

Inspected root/member process instructions and the review-loop skill; the seal contract, production plan and evidence; BuildEntry; LibraryBoundaryAndTestingPolicy; GladePackageArchitecture; adoption requirements RA-001–012 and qualification §6. The carrier and boundary audits supplied factual inventories, with material Store call sites checked against committed sources. Gyld’s Records, StorageAdapter and NodeAssembly allocation was read at its pinned revision.

Code inspection covered the complete scoped diff, `store.rs` entry points and mutation helpers, all 343 lines of `store/legacy_seal.rs`, all 212 lines of the integration target, and relevant existing Store recovery/checkpoint/proof tests. Consumer tracing included `Server::open`, registry seeding, `accept::place`, mesh ingestion, public peer pull, both composition roots, and session error mapping. The prior contract closure was legitimate historical input; no current implementation peer prompt/report was inspected.

All builds ran from:

`/var/folders/02/bn9c9g2x5qj8bb42zb857p7c0000gn/T/glade-q4-exact-source.ok310g7t`

Before testing, SHA-256 comparisons against pinned Git bytes matched:

| Scoped file | SHA-256 |
| --- | --- |
| `node/src/store.rs` | `e9c7d0eadcd8a52e056fcbc63135252a7c7ee97391061ab88313b7b618e0e75c` |
| `node/src/store/legacy_seal.rs` | `9f1e887ccfd85d2947c925667d843d43a21ebcaa5835ac6f1972849d42656062` |
| `node/tests/legacy_store_seal.rs` | `e16a87ba8bc364388f7363c70f26434a929b6a4050dfd44f63497c32c4ab73b0` |

The fixture hashes remained unchanged afterward. Cargo commands used the permitted external `CARGO_TARGET_DIR`, `--locked`, and `--offline`.

Independently executed:

- `--test legacy_store_seal`: **8 passed, 0 failed, 0 ignored**; Cargo reported 0.13 seconds warm completion and 0.03 seconds test execution.
- `--lib store::legacy_seal::tests -- --nocapture`: **4 passed, 0 failed, 1 explicitly ignored worker**; execution 0.07 seconds. The parent reported actual SIGKILL at all four distinct cuts.
- After receiving the exclusive test window, `--lib store::`: **26 passed, 0 failed, 1 explicit worker ignored**; execution 0.10 seconds.
- `sh glade/node/check.sh`: **exit 0**, ending with `GATE: PASS -- all 9 components passed`. This independently exercised both composition roots, affected consumers, contract gates, architecture negative fixtures, confinement, process globals, and existing style dispositions.
- Standalone architecture checker: **PASS**.
- Exact-fixture process-global checker: **79 files, 3 permanent allowlisted items, 0 debt, nothing new**.
- Existing raw-AST audit over all three scoped Rust files: **PASS**, parsing disabled branches without evaluating cfg.

The owner’s recorded 507 cases across 20 binaries per composition root and 66.644-second full-gate measurement remain owner evidence; no independent whole-gate timing is claimed here. Historical RED logs were inspected, not rerun: compiling integration failures and four compiling boundary failures agree with the earlier committed refusing scaffolds.

## 2. Invariant analysis

**The fence precedes every production Store mutation.** `Store::open` acquires its guard at lines 202–203, before journal reads, torn-tail trimming, HOME set-aside renames, proof replay or checkpoint recovery rewrite. `append_with` acquires its guard at line 307 before validation and classification. Duplicate, below-retention, fork, rewrite and new-op branches therefore cannot bypass marker recognition. The empty-marker consumers exercise duplicate, fork and next-op refusal and verify that no proof directory appears.

The lower-level journal append, proof persistence, repair and rewrite helpers are private. Their production callers remain inside guarded open/append scopes. The unguarded journal helper is enclosed in a test-only module. I found no production helper exposed as an alternative writer path.

**Guard ownership closes the check-then-write gap.** The returned `File` is bound as `_guard`, rather than discarded, and remains in the enclosing function through mutation or publication. Error propagation drops it only when leaving that scope. Production callbacks are immediate no-ops; private test callbacks expose boundaries without changing ownership. The race test checks a separately opened rival OS lock inside append and holds append while a competing seal waits. Inspection of the local standard-library Apple implementation confirms that these calls reach an exclusive filesystem lock, rather than an in-memory proxy.

**Marker recognition fails closed.** `symlink_metadata` treats only NotFound as absence. Every present filesystem kind closes admission without interpreting marker contents. Unknown bytes, empty files, directories and dangling symlinks all passed refusal consumers. Nonregular lock paths and other inspection/acquisition errors return before replay or append. Root/lock creation is the expressly permitted lifecycle setup; those failures do not enter journal/proof repair.

**Successful publication is monotonic.** The Unix implementation holds the same lock while creating or reopening the marker, syncing the file, and syncing the containing root directory. Existing regular markers are opened without truncation. Unexpected kinds produce an error while remaining fenced. There is no removal, rollback or automatic unseal path.

The injected-error and actual-process-interruption cases are distinct witnesses. The parent kills its env-cleared worker and verifies signal 9 at BeforeCreate, AfterCreate, AfterFileSync and AfterDirectorySync. Journal bytes remain unchanged; admission stays open only before marker creation; later cuts refuse; retry completes the same seal. These tests demonstrate process interruption, not power-loss durability.

**The affected consumers remain covered centrally.** Both composition roots call `Server::open`, which calls the guarded Store open. Client/provider placement, forwarded placement, mesh ingestion, public peer pull and seeding call Store append. The existing I/O error channel carries refusal without adding or changing wire tags. Unsealed Store recovery, proof and checkpoint behavior passed the isolated Store run, and the full gate passed consumer tests under both roots.

**The operation’s shape agrees with its limited lifecycle.** `seal_legacy(&mut self)` permanently retires one Store root; its name and documented error semantics do not imply enrollment, genesis, activation or a protected receipt. Deliberately omitting an unseal operation is consistent with this irreversible preparation contract. The distinct-root consumer passed. No production caller automatically installs a seal, and the scoped diff changes no declaration, manifest, dependency, classification or policy allowance.

Non-Unix publication returns Unsupported before fence mutation. Recognition and its common consumers remain outside platform conditions. Conditional sections have enclosing modules; the AST check inspected disabled branches. Actual non-Unix execution remains unqualified.

## 3. Risks and next action

The seal coordinates participating builds on a stable root. It cannot exclude unaware old binaries, external replacement of reserved files, Registry snapshots, external effects or divergent legacy copies. Existing handles retain explicitly permitted stale cached reads. None of these limitations was converted into a broader readiness claim.

Only macOS/APFS runtime behavior was exercised. The broader Unix source branch does not confer broader platform qualification; future qualification must verify actual independent-handle lock semantics and directory publication. Power-loss, independent storage domains, authenticated authority, carrier selection and RA-012 migration closure remain separate blocking gates.

The next action is to file this verdict and complete the owner’s independent acceptance decision for Q4-A only. Production activation remains blocked by Q4-B through Q4-E.