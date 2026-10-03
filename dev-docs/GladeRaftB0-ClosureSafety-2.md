# Q4-B0 remediation 2 — SAFETY-AXIS FINDER CLOSURE

**Review object:** Original-finder closure of Safety P2-5 on the correction from `256be2dd0fd652b34dfffa753fcba481f5fb842b` through root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`, controlled by `dev-docs/GladeRaftB0-RemPlan-2.md`. DRAFT private contract/allocation/compiling RED object.

**Baseline:** Root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources read through pinned `git show` and scoped `git diff`; matching local bytes verified.

**Date:** 2026-10-03  
**Axis:** Safety, limited to original-counterexample closure and correction integrity. Independent, adversarial, read-only. Other reviews run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: GO — finder closure only.** Safety P2-5 closes; no new finding within this focused review. This is not aggregate acceptance or engine qualification.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| Safety P2-5 | Separate caller-owned termination from remote delivery validity | Executed all eight held/consumed × stopped/replaced peer × Timeout/Cancel cases. Correct terminal future errors, selected ownership removal, continued caller traffic and negative controls passed. Restoring peer-liveness validation failed exactly those eight cases. | Closed |

The earlier Safety P2-1–P2-4 closures remain intact: their source-footprint, shared-stop/stale-traffic, selected-poll stop and destructor regressions were included in the passing current commands. This report formally closes only P2-5.

## Changed-range analysis

`Network::owned_rpc` now checks exact issued pending ownership and original request linkage independently of remote liveness. Timeout/Cancel use it after validating the current live caller and matching caller scope. Respond/Resolve retain `Network::rpc`, which adds live remote-token validation.

The patch corrects two prior tests that endorsed the defect, adds the eight-cell termination matrix and two negative/late-reply witnesses, adds an isolated mutation checker, and clarifies allocation/evidence documentation.

Public signatures, manifests, lockfile, four package roles/edges, refusing providers and B0 case selections remain unchanged. No engine or production dependency was installed.

**New architectural root classification:** None identified within this focused correction. The patch resolves the previously classified completion-ownership root; it does not introduce another ownership boundary. Two architectural remediation rounds remain used. Fresh full review remains required by RemPlan-2 and is separate from this closure.

## 0. Evidence base

Read the full canonical closure prompt, RemPlan-2, legitimate prior reports and the scoped committed diff. Relevant corrected sources:

- `spec/src/fixture/transport.rs`: ownership/delivery validators and Timeout/Cancel branch.
- `spec/src/fixture/transport/termination.rs:3–222`: eight-cell matrix.
- `termination.rs:224–340`: replaced-caller and live-peer late-reply refusal.
- `spec/tests/endpoint_lifecycle.rs`: corrected original expectations.
- `check-termination-mutant.py:1–37`: isolated restoration of the exact defect.

Independently executed current README commands:

- API/provider compiler witnesses: **4 passed**.
- Spec unit/compiler/fixture/oracle/lifecycle witnesses: **41 passed**.
- Total GREEN: **45**, zero ignored/filtered.
- B0 election target: exit **101**, **20 ordinary behavioral assertion failures**, zero passed/ignored/filtered.
- Architecture, five negative fixtures, source scope, globals and formatting: PASS.
- Four-package all-target denied-warning Clippy: PASS.
- Termination mutant: compiled successfully; **eight matrix assertion failures and six passing unit tests**, zero ignored/filtered. Wrapper confirmed rejection.

Inspected historical `rem2-red.log`: the eight matrix cells and two corrected prior expectations failed behaviorally after compilation. Historical RED was inspected, not reconstructed. The mutation run independently reproduced the original defect.

Verified **102 inventory entries plus the manifest**, documentary-context hashes and committed/local subtree equality before and after execution. Manifest SHA-256:

`5415b79ebaed75de6c5bf49d7619b0661516651c6982a427aef70b6fbef8e789`

All four HEADs matched at start and end. Only authorized ignored test/mutant artifacts were produced; no source, evidence or Git mutation occurred. Current peer/finder/full-review reports were not read.

## 2. Invariant analysis

The original sequence now succeeds: a live caller issues an RPC, its peer stops or is replaced, and explicit local Timeout/Cancel removes that RPC while preserving unrelated pending ownership.

Each response was first polled Pending. Terminal control marks its task Runnable without executing the observing future inline. The next selected poll completes once with precisely `TimedOut` or `Cancelled`. Original protocol bytes and peer/replacement message, RPC and work inventories remain unchanged by local termination. The caller subsequently delivers valid traffic to another live endpoint.

Foreign and forged RPC controls refuse unchanged. Replaced-caller controls cannot terminate old ownership or wake its future. Repeated terminal controls refuse. Stale-peer replies and a still-live peer’s late reply cannot reopen the terminated RPC.

The mutant changes only the Timeout/Cancel validator back to remote-liveness validation. All eight cells then fail at the original valid-local-termination assertion, confirming that these tests detect P2-5.

## 3. Risks and next action

This establishes controlled-fixture ownership and lifecycle behavior only. No actual engine, source adaptation, automatic-election runtime, durability or production result is qualified.

File this original-finder closure and use the separate fresh full Consistency/Safety verdicts to determine aggregate acceptance. No further architectural correction is authorized by this report.