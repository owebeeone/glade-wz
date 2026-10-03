# Q4-A legacy Store seal evidence

Date: 2026-10-03. Status: **compiling behavioral RED; no implementation acceptance**.

Contract and allocation: [seal contract](GladeRaftLegacyStoreSealContract.md).
[Production integration plan](GladeRaftProductionIntegrationPlan.md) retains all
Q4 selection/authority/failure-domain/migration gates. Two read-only source audits
are filed verbatim as [carrier facts](GladeRaftQ4-CarrierAudit.md) and
[Glade boundary facts](GladeRaftQ4-BoundaryAudit.md); neither is a GO review or
an executable carrier comparison. Boundary facts identify live cold-join dirt.

Before any seal implementation, `cargo test --locked --offline --manifest-path
glade/node/Cargo.toml --test legacy_store_seal` compiled successfully and executed
seven tests: **one compatibility case passed, six behavioral cases failed**, no
ignored/filtered cases. Initial fixture compiler mismatch (`refs` is Vec, not
Option) was corrected first and is not counted as RED. The compiling scaffold
only declares a deliberately refusing `seal_legacy` operation; open/append remain
unchanged. Failures observed actual `Ok(Appended)` despite preexisting marker,
unsupported successful seal, and open proceeding on a sealed torn journal.
Warm Cargo compile/run reported 0.73 seconds and test execution 0.00 seconds.
The preceding relocated-node dependency rebuild is not a clean cold-build metric.
Raw local log: `/tmp/glade-q4-seal-red.log`; normative cases LS-001/002/004/005/007
live in `glade/node/tests/legacy_store_seal.rs`. LS-003/006 lock/publication/crash
witnesses must be added RED before their implementations. Non-Unix refusal is
specified in a separate braced platform module, not run on this host.

The exact contract/specification checkpoints and reports will be recorded in
the qualification review ledger. Acceptance of this internal migration preparation
slice does not close RA-012, activate Raft, install any seal on the desk, select a
production carrier or certify independent storage domains/power loss.

## Contract gate and additional RED

Contract tuple root `6a35216a6d97aa22e9d53b54256f7395c306da3c`, Glade
`19a269dd12b5f109d03e2361e3af4108d4b1fcbc` received independent
[Consistency GO](GladeRaftQ4ASealContract-ReviewConsistency.md) and
[Safety GO](GladeRaftQ4ASealContract-ReviewSafety.md). Safety found no defects;
Consistency found nonblocking P3-1: marker-refusal cases were Unix-only.
The owner moved platform-independent recognition tests into a shared module,
added empty-marker duplicate/fork/new-write and no-proof checks, and retained
qualified Unix publication and non-Unix Unsupported tests in braced modules.
Before any guard implementation the revised consumers compiled and ran:
**one passed, seven failed**, no ignored/filtered cases (`/tmp/glade-q4-seal-shared-red.log`).
Actual non-Unix execution remains unqualified; shared source no longer excludes
recognition from that platform's test selection.

LS-003/006 boundary fixtures were then added against private refusing scaffolds.
`cargo test --locked --offline --manifest-path glade/node/Cargo.toml --lib
store::legacy_seal::tests` compiled and ran: **zero passed, four failed, one
explicit subprocess worker ignored**, 359 unrelated tests filtered, 7.44 seconds
build-plus-run and 0.02 seconds execution. Failures observed unexecuted real-I/O
cuts, refusing exact retry, absent held-lock entry and no subprocess cut signal.
No crash success is claimed by RED. Log: `/tmp/glade-q4-seal-boundary-red.log`.
The intended GREEN parent kills its own env-cleared worker at before-create,
after-create, after-file-sync and after-directory-sync boundaries and checks
actual signal 9, retained journal bytes, refusal after marker creation and
monotonic retry. The lock case introspects a real rival OS lock inside append
before mutation and forces a competing seal to wait. Private callbacks are
controlled test boundaries; production supplies immediate no-op callbacks.

Both contract reviewers independently observed a transient 15/7 Store-unit run
and a clean 22/22 repeat. Existing fixed-name Store fixtures permit interference
between concurrent test processes. Those initial failures remain in testimony;
no unrelated fixture rewrite was performed. Subsequent Store/gate commands MUST
be serialized between owner and reviewers. The new seal fixtures use process IDs.
