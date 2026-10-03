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

## Owner implementation verification — awaiting independent acceptance

The original Consistency reviewer [closed P3-1](GladeRaftQ4ASealContract-ClosureConsistency.md)
at root `3bf731994c8f18e4e27ed568e123bed099dca2f0`, Glade
`638cca4b2784cc3e51c1e47b1fea47d026b57734`. Both contract gates remained GO
before implementation. Marker creation and sync are isolated in the existing
Store's private `legacy_seal` module; no dependency/policy/allowlist changed.
Open retains its guard before all replay/repair; append retains its guard through
classification, proof writes, journal append and checkpoint rewrite. Seal syncs
the existing/new marker and root directory under the same stable lock.

Owner targeted GREEN: eight integration consumers passed, 0 failures/ignores;
four boundary tests passed, 0 failures, one explicitly ignored subprocess worker.
The parent test separately observed actual SIGKILL signal 9 at all four distinct
cuts: BeforeCreate, AfterCreate, AfterFileSync, AfterDirectorySync. Each checked
journal bytes, presence/absence, next admission and idempotent resync. The first
parent run exposed a libtest stdout-prefix handling error (cut signal could appear
after the test name); the parent matcher was corrected against that failing case
before all four cuts passed. This was a test-driver error, not a recovered success.
Targeted final runs: integration build-plus-run 1.56 seconds, execution 0.02 seconds;
boundary build-plus-run 1.98 seconds, execution 0.08 seconds. Logs are
`/tmp/glade-q4-seal-green.log`, `/tmp/glade-q4-seal-boundary-green.log`.

Full live-tree `sh glade/node/check.sh` passed all nine components, including
architecture/negative fixtures/all-target confinement, both roots, contracts,
process-global ratchet, fmt and Clippy dispositions. Each root ran **510 passing
cases across 20 test binaries**. Existing fmt debt remains node252/wire1 hunks;
Clippy debt remains node9/wire7 warnings. An initial gate run passed eight
components but failed the Clippy ratchet on one new unused test import; that
import was removed and the complete gate then passed. No baseline was relaxed.
Logs: `/tmp/glade-q4-node-gate.log`, `/tmp/glade-q4-node-gate-final.log`.
These live-tree runs include inherited cold-join dirt. They are compatibility
observations, not qualification of that unrelated diff. An exact committed-source
fixture check is still required before implementation acceptance.

Process-global ratchet inspected 79 production files: three permanent existing
entries, zero debt, nothing new. New module and integration target are rustfmt
clean. A temporary development-only raw `syn2.0.118` AST audit parsed all three
scoped Rust files without evaluating cfg, rejected conditional attributes outside
modules, and passed including disabled non-Unix branches; an explicit bare
`#[cfg(windows)] use ...` negative fixture was rejected. Commands/source/logs:
`/tmp/glade-q4-syntax-audit`, `/tmp/glade-q4-scopes.log`,
`/tmp/glade-q4-scopes-negative.log`. This audit is not macro expansion or CI adoption;
Rust grammar supplies braced control-flow syntax. Broader source/dependency
migration is not claimed. Actual non-Unix execution remains open.

Machine: macOS arm64, rustc1.96.0, APFS source volume `projects` and APFS Data volume
backing `/private/var` temporary fixtures. Physical process interruption is
qualified separately from modeled injected errors; power-loss/media certification,
independent machines, authenticated authority, Raft crate selection and complete
old-binary/Registry/effect/rollback exclusion remain unqualified. No seal was
installed outside disposable test roots; no running desk was rebuilt or changed.

## Exact committed-source verification

Accepted-source candidate is Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`.
Owner built a disposable source-only verification fixture by read-only `git archive`
of exact commits, outside the workspace/GWZ family, with no `.git`, workspace
configuration or dirt. This is a test fixture, not a clean local-clone workaround.
Its supporting pins are discover52ea2d118f45d9e7c3d9a789310dd5d669958851,
grazelc839fe87c9d18ebb6e995964d2e79aef7cbd380e,
glade-gyld327d62c0033db0fae145d002a00663a3826b6d53,
glade-gwz35b38ba0845a7cb7034a4af3609975ea1bd48741 and
glade-decl-rsb85044e1f6631114dbb290c02298e644f8363055.
All exported tracked objects were compared byte-exact against those Git archives
after the gate. CARGO_TARGET_DIR reused ignored build cache only.

The complete adopted node gate passed **all nine components** on these exact
sources; each root ran **507 passing cases across 20 binaries**. Runtime for the
final full gate was 66.644 seconds, not a pure-library fast-loop budget or a clean
cold build. APFS process-kill cases execute by the parent in the default unit tier;
its explicit ignored worker is invoked only in that parent. Existing style debt
remains node252/wire1 fmt hunks and node9/wire7 Clippy warnings. The gate notes
that exported lockfiles have no Git tracking metadata; their bytes were separately
verified against committed sources, not generated or weakened to pass.

Two precursor fixture attempts are retained: the first missed glade-decl-rs, so
four gate components failed dependency resolution; adding its exact committed
source repaired the fixture. The second passed eight components but exposed a
new multiline callback-signature fmt hunk (253 against252), previously masked
by an inherited cold-join formatting reduction. Correcting only that new signature
made the exact-source full gate pass. No inherited code was formatted and no
style/dependency/process-global allowance was relaxed. The clean fixture has
three fewer tests than the live tree because inherited cold-join cases are absent.

Fixture path and pins: `/tmp/glade-q4-exact-source-location.json`; initial builder
`/tmp/glade-q4-exact-source.py`; final log
`/tmp/glade-q4-exact-source-gate-accepted.log`; measured result
`/tmp/glade-q4-exact-source-result-accepted.json`. Implementation acceptance still
requires independent Code/State verdicts against the exact tuple; passing owner
checks do not supply those verdicts.
