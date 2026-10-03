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
