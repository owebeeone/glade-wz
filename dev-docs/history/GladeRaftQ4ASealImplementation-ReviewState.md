# Q4-A internal whole legacy Store retirement interlock Implementation — STATE-AXIS REVIEW

**Review object:** Implementation candidate at workspace root `aafb14a663fe130db5ef56cb002b2bb9b9399b11`, Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`: production integration plan, legacy Store seal contract, Q4-A evidence, `node/src/store.rs`, `node/src/store/legacy_seal.rs`, and `node/tests/legacy_store_seal.rs`. Implementation acceptance only; production activation remains unaccepted.

**Baseline:** Root `55b6ee30bb18fa58c80805be2d29a44b404077f6`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`. Controlling sources were read through `git show <exact SHA>:<path>` and scoped baseline diffs. Inherited working-tree edits were excluded.

**Date:** 2026-10-03

**Axis:** State: durable-state semantics, mutation ordering, lock ownership, interruption, restart legality, and fail-closed recovery. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2, or P3 findings within the specified stable-root, participating-build, local process-interruption profile.

---

## 0. Evidence base

The complete tuple matched at the beginning and end:

| Repository | Exact revision |
| --- | --- |
| Workspace root | `aafb14a663fe130db5ef56cb002b2bb9b9399b11` |
| glade | `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87` |
| glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| grazel | `c839fe87c9d18ebb6e995964d2e79aef7cbd380e` |
| glade-gyld | `327d62c0033db0fae145d002a00663a3826b6d53` |
| glade-gwz | `35b38ba0845a7cb7034a4af3609975ea1bd48741` |
| glade-decl-rs | `b85044e1f6631114dbb290c02298e644f8363055` |
| External Gyld | `ca04499a360d910fbf8ee2540ed446facd051b35` |

Read root/member instructions and the review-loop skill; the seal contract’s complete LS-001–007 table and mechanics; production integration plan; implementation evidence; adoption contract RA-001–012; qualification plan, particularly §§5–6; BuildEntry; LibraryBoundaryAndTestingPolicy; PackageArchitecture; and both factual Q4 audits. Read the prior contract Consistency/Safety reports and Consistency closure as legitimate earlier gates. No current implementation peer prompt, report, or testimony was accessed.

Inspected Store open/replay/rename/rewrite at `store.rs:187–278`, append and proof/checkpoint branches at `:285–411`, cached readers, and journal/proof decoding and repair at `:668–759`. Read the complete private seal module, including disabled platform source and subprocess driver, and all 212 integration-test lines. Checked `Server::open` at `server.rs:90–111` and seed admission at `:130–143`. Read SubstrateV1 §6’s existing receipt guarantees and pinned Gyld Records, StorageAdapter, and NodeAssembly allocations.

All builds used the owner-created source fixture:

`/var/folders/02/bn9c9g2x5qj8bb42zb857p7c0000gn/T/glade-q4-exact-source.ok310g7t`

Before testing, paired SHA-256 checks matched all three scoped fixture files against pinned Git contents. They remained unchanged afterward. Node and wire lockfiles also matched their committed bytes. The owner’s broader 588-object verification remains owner evidence; I independently checked these five files.

With `CARGO_TARGET_DIR=/Volumes/projects/limbo/glade-wz-raft-production/glade/node/target`, independently executed:

- `cargo test --locked --offline --manifest-path glade/node/Cargo.toml --test legacy_store_seal`: exit 0; **8 passed**, zero failures or ignores. Cargo reported 0.29 seconds; execution 0.02 seconds.
- `cargo test --locked --offline --manifest-path glade/node/Cargo.toml --lib store::legacy_seal::tests -- --nocapture`: exit 0; **4 passed**, one explicitly ignored worker, 357 filtered. The parent reported actual SIGKILL at BeforeCreate, AfterCreate, AfterFileSync, and AfterDirectorySync. Execution reported 0.09 seconds.
- `sh glade/node/check.sh`, after the owner explicitly granted a serialized window: exit 0; **all nine components passed**. Both composition roots reported **507 passing tests across 20 binaries**.
- Existing raw-AST audit against the three exact fixture files: exit 0, all branches parsed without cfg evaluation. Its existing bare conditional-import negative fixture returned exit 1 as expected.

The full gate independently confirmed architecture, negative dependency fixtures, confinement, contracts, and process-global ratchets. Historical formatting debt remained node252/wire1 hunks; Clippy debt remained node9/wire7 warnings. These are counted dispositions, not a claim that historical sources are clean.

Historical RED logs were inspected, not rerun: shared consumers compiled with one pass/seven behavioral failures; boundary consumers compiled with zero passes/four failures and one worker ignore. Pinned precursor `638cca4b2784cc3e51c1e47b1fea47d026b57734` contains refusing publication/append scaffolds and the consumers before implementation.

No source/report files or Git state were modified.

## 2. Invariant analysis

**The admission decision and mutation share one lock lifetime.** `legacy_seal::locked` opens the stable root lock and acquires an exclusive OS lock. `unsealed` returns that owning File only after marker inspection. Store open retains it across journal enumeration, torn-tail trimming, HOME renaming, proof replay, and checkpoint rewrite. Append retains it across validation, duplicate classification, proof persistence, ordinary append, and checkpoint publication. Error returns drop the guard without reaching subsequent mutation.

The race consumer attacks the actual critical section: an independently opened rival File receives WouldBlock inside append’s callback; a competing seal cannot finish until append resumes and completes. The callback neither owns nor releases the guard. Production supplies an immediate callback. Source inspection confirms the guard remains through the final publication branches, beyond the tested pause.

**Pre-opened handles cannot reuse an old admission result.** Each append reacquires the filesystem lock and inspects current marker presence. The successful-seal consumer fences a second handle opened beforehand, including duplicate and fork attempts. Refusal occurs before proof creation. Cached reads remain available under the stated stale-view contract; they cannot authorize another append or establish migration completeness.

**Marker ambiguity consistently closes admission.** Recognition uses `symlink_metadata`; only NotFound establishes absence. Empty bytes, unknown bytes, directories, and dangling symlinks refuse open/append. Unexpected lock kinds and I/O failures return errors. Publication refuses nonregular markers, while an existing regular marker is opened without truncation and resynced. The unknown-marker retry test confirms its bytes survive unchanged.

**Repair cannot precede the fence.** The Store guard is acquired before any journal read capable of trimming a tail. The torn-tail consumer retains exact malformed bytes across refused open. The same enclosing guard covers proof-tail repair, HOME set-aside renaming, and covered-history rewriting. No independent public Store mutation entry point bypassing open/append/seal was found.

**Publication follows a monotonic recovery grammar.** Under the lock, Unix publication creates or opens the marker, syncs it, then syncs its containing directory before success. It never removes the marker. Before-create failure leaves admission available; after-create failure may leave irreversible refusal. Retrying through an existing handle resyncs the retained marker and directory.

Both callback-error tests and actual process-kill tests exercise these boundaries with real files and retained journal-byte comparisons. The parent requires signal 9, so modeled errors cannot substitute for process interruption. Injected callback errors are not claims of physical sync-syscall failure or power loss. Source propagation of create/open/sync errors remains fail-closed.

**Platform handling does not invent durability.** Non-Unix publication returns Unsupported before marker mutation. Recognition is shared unconditional source; its consumers remain selected outside Unix. Disabled branches passed raw parsing and explicit-module scope checks. Actual non-Unix execution remains unqualified.

**Scope and allocation remain bounded.** This retires one whole physical Store root, including HOME journals. Separate-root behavior passes. No scoped code installs a seal during startup, activates Raft, changes declarations, imports proof packages, or introduces new dependencies or policy exceptions. Its irreversible lifecycle is explicit and has no implied unseal operation.

## 3. Risks and next action

Advisory locking coordinates participating builds only. Old unaware binaries, separate Registry snapshots, external effects, raw lock/marker replacement, rollback restoration, and divergent legacy copies remain outside this interlock. The production plan retains their stronger exclusion and reconciliation obligations under Q4-E; this GO does not close RA-012.

The demonstrated profile remains local process interruption with ordered real filesystem sync calls. Independent storage domains, power-loss certification, non-Unix execution, authenticated authority, and production carrier selection remain separate gates. General cache coherence between independent unsealed Store handles is also not promised.

The next action is for the lane owner to merge the independent implementation verdicts and record acceptance only for this exact Q4-A preparation scope if both are GO. No migration cut or production activation follows from this verdict.