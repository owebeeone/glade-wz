# Glade legacy Store seal — Q4-A contract

Date: 2026-10-03. Status: **Q4-A contract and implementation accepted; exact implementation tuple root `aafb14a663fe130db5ef56cb002b2bb9b9399b11`, Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`, after [Code](GladeRaftQ4ASealImplementation-ReviewCode.md)/[State](GladeRaftQ4ASealImplementation-ReviewState.md) GO/GO; whole-Store retirement preparation only**.
Controlling plan: [Q4 production integration](GladeRaftProductionIntegrationPlan.md).
This is a preparation interlock for taking a legacy store offline for a later
verified cut. It does not enroll any scope, supply authenticated evidence,
activate Raft, acknowledge protected commands or close RA-012.

## Allocation and contract

Within the existing `glade-node` integration package, the existing Store owns
its on-disk journal lifecycle. Gyld Records owns accepted history/recovery through
StorageAdapter, and NodeAssembly constructs it. This slice introduces no new
package, trait, dependency, classification, policy exception or public wire schema.
`Store::seal_legacy(&mut self) -> Result<(), StoreError>` is a meaningful internal
lifecycle operation; it permanently closes this whole Store root to legacy
`open`/`append` in participating builds. It MUST NOT affect another root.
Existing Server::open is the real affected consumer shared by both node roots.

A sealed root remains physically present and its journals MUST remain byte-exact.
An existing handle MAY inspect its pre-cut cached contents, which are neither
fresh nor proof of production readiness. An offline migration tool MAY inspect
retained files. No unseal/delete/reset operation exists in this slice. A failed
seal call after marker creation MAY have sealed: the error MUST NOT promise
rollback or known noncommit. Exact retry on an existing handle MUST complete the
same monotonic seal without rewriting/removing the marker or application data.

| ID | Normative behavior | Consumer/fault witness |
| --- | --- | --- |
| LS-001 | Unsealed append/duplicate/reopen behavior MUST remain compatible. | Unsealed canonical value private key and restart; existing Store/session/mesh suites. |
| LS-002 | A successful seal MUST persist before returning; subsequent append including duplicates/forks and fresh open MUST refuse before any journal/proof/repair mutation. | Preexisting data, two pre-opened handles, duplicate/fork/next-op and server reopen. |
| LS-003 | Every participating Store open/append/seal MUST hold one stable root-specific OS file lock from fence inspection through its entire mutation/publication. | Competing handles; controlled race witness proving append cannot pass a pre-seal check then write after seal. No check-then-independent-append gap. |
| LS-004 | A fence of any contents or filesystem kind, including corrupt bytes, directory or dangling symlink, MUST close open/append. Inspection/I/O failures MUST fail closed; unknown format MUST NOT be treated as absent. | Corrupt/non-file/dangling marker cases and inaccessible lock target. |
| LS-005 | Open MUST check the seal under that lock BEFORE reading/repairing/renaming journals. It MUST NOT trim a torn legacy tail in a sealed root. | Exact file-byte comparison across refused open with a torn tail. |
| LS-006 | Failed publication MUST never delete the fence or reopen admission. Failures before creation leave no successful seal; failures after creation may leave an irreversible seal. Retry resyncs the marker and containing directory before success. | Injected boundaries after create, marker sync and directory sync; real file inspection/retry; explicit process interruption witness. |
| LS-007 | A distinct root MUST retain legacy behavior. No CLI/startup/settings auto-enrollment MAY occur in this slice. | Separate-root acceptance and unchanged compositions. |

## Mechanics to be independently reviewed

The proposed internal reserved files are `legacy-store.lock` and
`legacy-store.sealed`, non-journal files at root. Root Store::open creates the
root/lock if needed; existing replay skips non-directory entries. Each operation
opens the same lock file and exclusively locks it; closing its RAII guard releases
the lock even on errors. No process-global lock table or thread-local is allowed.
A failed/unknown lock acquisition MUST NOT continue replay/append.

`symlink_metadata` establishes marker absence only on NotFound; any other result
refuses. Seal publication uses create-new; an existing regular marker is resynced
without truncation, but unexpected kinds are refused while remaining fenced.
The marker file MUST sync, then its root directory MUST sync, before success.
The lock stays held throughout. Unix directory sync is real; non-Unix seal
publication MUST return Unsupported before mutating the fence until an adapter
is separately qualified. Recognition/refusal of an existing fence remains
cross-platform. No platform variant is implemented by a no-op successful sync.

The initial physical profile is macOS/APFS process interruption; fsync calls and
real I/O faults are observed separately from any power-loss claim. Independent
machine/storage failure is not established by this local operation. Root parents,
filesystem permissions, external deletion/replacement, malicious raw file writes
and storage corruption after successful sync are outside this interlock profile.
A stable root and no external unlink/replacement of the lock/marker are required.
The later production fence MUST define and enforce stronger rollback semantics.

## Scope limits and activation blockers

Advisory locks only coordinate participating builds. **An old binary that ignores
these files can still write.** Existing handles may retain stale reads. Separate
Registry snapshots and external source/sink writes are not covered. Sealing all
journals stops HOME gossip as well as app writes: use only after draining that
store, on the explicitly selected migration target, never automatically on the
running desk. The whole-store scope is intentional for taking one legacy
instance offline; it is not a selective online resource-migration contract.

This cut only orders participating local mutations before/after seal. It proves
neither global baseline completeness nor reconciliation of legacy copies. Data
loss/divergent/unknown/pending histories and older binary exclusion remain Q4-E
blockers. No protected acceptance path is enabled while those blockers remain.

## RED, review, verification

Add a compiling deliberately refusing seal signature, then run named behavioral
consumers against it and current open/append before implementation. Compiler
errors do not count as RED. Review the plan/contract/allocation/specification at
an exact root/member tuple on Consistency and Safety axes. Implement only after
both GO. Add failing adversarial boundary tests before each correction.

Fast command: `cargo test --locked --offline --manifest-path glade/node/Cargo.toml
--test legacy_store_seal`. Measure execution and warm build separately; cold
compilation is a separate measurement, not a budget claim. Run the node's adopted
architecture gate alongside relevant tests, the process-global ratchet, disabled-
branch syntax check and owning Clippy/fmt dispositions. Final Code/State review
MUST include real I/O, lock-race, fail-closed recovery and both shared consumers.
No proof-harness dependency or weakened gate is permitted.
