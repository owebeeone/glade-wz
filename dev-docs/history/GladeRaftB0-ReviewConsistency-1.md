# Q4-B0 remediation 1 — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT private contract/allocation/compiling RED packet at root `256be2dd0fd652b34dfffa753fcba481f5fb842b`, reviewed against `dev-docs/GladeRaftB0-RemPlan-1.md` and the scoped diff from `544c83d8cd07165cfeec2f0db64a78c8417849f3`. This gate does not accept source adaptations, real engines, B0 runtime or production use.

**Baseline:** Revised root `256be2dd0fd652b34dfffa753fcba481f5fb842b`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were inspected through pinned `git show`, scoped `git diff`, and verified matching local bytes.

**Date:** 2026-10-03

**Axis:** Consistency with controlling requirements, source footprints, ownership and lifecycle contracts. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — all three original Consistency findings are closed, but one new P2 finding blocks. The revised RPC completion rule also requires fresh full review under the remediation plan.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| Consistency P2-1 | Carrier-specific source-access cuts | Executed once-only OpenRaft witness and extra-draw rejection; retraced B0-03’s revised oracle and raft-rs leader quorum-loss reset schedule | Closed |
| Consistency P2-2 | Shared stop/current-incarnation validation | Executed old endpoint emit/request/control/take counterexamples and separate/fresh-handle stop checks; refusals preserve queues and sequence allocation | Closed; new completion defect below |
| Consistency P2-3 | Drop detached futures outside scheduler borrow | Executed parent cancellation and bulk-stop guards that cancel/inspect child work; exact-once destruction and terminal inventories pass | Closed |
| Safety P2-1 | Carrier-specific source-access cuts | Same counterexample independently checked as above | Verified here; formal closure belongs to Safety finder |
| Safety P2-2 | Shared endpoint lifecycle authority | Executed shared-stop and replaced-incarnation regressions | Verified here; formal closure belongs to Safety finder |
| Safety P2-3 | Reconcile own-scope stop during poll | Executed selected future that stops its scope and returns Pending; drop count is one and saved wake cannot revive it | Verified here; formal closure belongs to Safety finder |
| Safety P2-4 | Release borrow before future destruction | Executed individual and bulk reentrant cleanup regressions | Verified here; formal closure belongs to Safety finder |

## Changed-range analysis

The scoped diff changes fixture source/work/transport enforcement, B0-02/03 fault consumers, allocation/README evidence descriptions, measurement tooling, and architecture conformance target selection. It adds two focused test targets and remediation evidence.

Public API declarations, all package manifests, lockfile, refusing provider implementations, library roles and dependency edges remain unchanged. The comparison contract, adaptation inventory and governing documents also remain unchanged. Existing evidence files are preserved. Moving saved RPC wakes outside borrows is a bounded cleanup refactor.

However, requiring a live remote incarnation for a caller’s **Timeout/Cancel** is more than enforcing stale delivery rejection. It changes ownership of local RPC completion. Tests and allocation text explicitly endorse that new rule.

**NEW ARCHITECTURAL ROOT CAUSE:** delivery validity and caller-owned RPC termination are conflated. P2-4 concerns this completion ownership boundary, rather than reopening the original entropy or destructor findings. Because the correction changes lifecycle behavior despite unchanged signatures, the remediation plan’s fresh-review requirement applies. Record this classification in the review-cycle accounting; this report does not itself assert that the remediation cap has been exhausted.

## 0. Evidence base

Read the full canonical remediation prompt, both original reports and merged plan. The prompt SHA-256 matches `042bef61cf7d35aef9660e56be25ceee95a95be41f3bc2b471964199a93b3256`.

Reviewed the complete scoped diff, revised allocation, README, fixtures, fault consumers, new lifecycle targets, measurement-tool changes and historical RED evidence. Relevant ranges include:

- `spec/src/b0/sources.rs:1–124,273–374`
- `spec/src/fixture/transport.rs:41–80,249–355,380–413`
- `spec/src/fixture/work.rs:62–103,133–265`
- `spec/tests/endpoint_lifecycle.rs:10–237`
- `spec/tests/scheduler_lifecycle.rs:12–215`

Confirmed previously read governing documents are byte-identical to the original review pin. Rechecked the unchanged API’s `RpcId`, `PendingRpc` and transport contract at lines 160–251. Inspected pinned OpenRaft vote and append timeout paths at `raft_core.rs:1076–1100` and `replication/mod.rs:455–462`.

Independently executed the six current README command groups:

- API compiler witnesses: **2 passed**.
- Provider compiler witnesses: **2 passed**.
- Spec unit/compiler/fixture/oracle/RPC/lifecycle witnesses: **31 passed**.
- B0 election target: exit **101**, **20 ordinary behavioral assertion failures**, zero passed, ignored or filtered.
- Structural gate: PASS, including five architecture negatives, 23 Rust source files and zero local global exceptions.
- Four-package all-target Clippy with denied warnings: PASS.

Historical remediation logs contain **13 distinct compiling RED regression names**: twelve in `rem1-red.log`, plus the unique overflow/destructor regression in `rem1-red-overflow.log`. The authoring compiler errors remain separately recorded and are not counted as RED. Historical RED was inspected, not reconstructed.

At start and end, all four HEADs matched the tuple. All **82 current inventory entries plus its manifest** matched committed bytes and hashes. Manifest SHA-256 is `f6ce42f4423062a5baeb7eb12273db62b63debd9455e326cdfa5e2eb2d10d88c`. Only authorized ignored build artifacts were produced. No files, tests, evidence or Git state were changed. The separate SourceAdaptationPlan was not reviewed or adopted.

## 1. Findings

### [P2-4] Remote incarnation validity prevents a live caller from terminating its own pending RPC

**Location:** `spec/src/fixture/transport.rs:63–69,340–355`; `spec/tests/endpoint_lifecycle.rs:104–120,124–179`; allocation’s remediation table, endpoint correction row.

**Violated invariant:** An issued, pending RPC belongs to its caller scope. Explicit local timeout/cancellation must release that ownership without requiring remote participation. Stale delivery must remain refused independently.

**Reproduction:** The current GREEN test `timeout_and_cancel_validate_live_rpc_peer_before_removing_ownership` executes this sequence:

1. Live caller A issues RPC R to peer B.
2. B is replaced; A’s scope and R’s local identity remain current.
3. A selects `Timeout(R)` or `Cancel(R)`.
4. `Network::rpc` validates R’s request through `token`, which requires B’s old incarnation to remain live.
5. The action returns `InvalidToken`; R remains pending.

The stopped-peer test reproduces the same outcome when B stops. Stopping A subsequently removes R, but terminating the caller is the only exposed successful cleanup in this state.

These passing tests independently demonstrate the defective behavior; no additional test was written during review.

**Impact:** Peer failure or replacement makes ordinary timeout/cancellation unusable for an otherwise healthy caller. Its response future and RPC registry retain pending ownership. A faithful adapter must either leak that ownership, treat normal peer loss as an invalid control, or stop the caller to clean up. That contradicts the supplied pending-future lifecycle and cannot faithfully represent OpenRaft’s ordinary caller-side timeout paths.

`RpcId` contains the caller scope and sequence; the live caller’s R is neither a foreign nor consumed RPC merely because its destination stopped. Message-token delivery validity must not silently become RPC cancellation authority.

**Required correction:** Separate local RPC ownership validation from request/reply delivery validation. Timeout/Cancel must check the current live caller, actual issued pending RPC and caller ownership, then remove and settle it exactly once, even when the peer stopped or was replaced. Respond/Resolve and stale message delivery must retain strict remote-incarnation checks. Update the tests and allocation statement that currently require the opposite behavior.

**Closure test:** For both peer stop and replacement, issue R, poll its response into Pending, then explicitly timeout/cancel from live A. Require the corresponding typed future result at the next selected poll, empty pending ownership, unchanged protocol bytes, and no stopped/replacement-node mutation. Reuse must refuse. Foreign or replaced caller controls and stale replies must still refuse unchanged. A mutant requiring peer liveness for cleanup must fail.

## 2. Invariant analysis

The original source-cadence contradiction is removed: OpenRaft post-construction poison now requires no additional sampling and a healthy node; raft-rs deliberately reaches its check-quorum leader reset. Persistent work no longer requires artificial registration. These source/work witnesses remain honestly distinguished from real engine qualification.

Scheduler cleanup now detaches futures before destruction and reconciles terminal scope state after selected polling. Original destructor and in-flight stop counterexamples pass directly.

Shared endpoint stop and current-incarnation checks prevent the original stale participation. Existing correlation, wrong-peer/kind/carrier, terminal reuse, explicit polling and message-drop tests remain GREEN. Their stronger delivery validation does not justify the new restriction on local cleanup.

The four std-only packages, identical ten-case selections, mandatory B1/B2/B3 comparison and separate production gates remain intact. No candidate selection, production authority, receipt reinterpretation or default activation is claimed.

## 3. Risks and next action

The downstream B0 consumers still stop at refusing-provider assertions; no actual engine, source adaptation, runtime or production result is established.

Prepare one corrected packet addressing P2-4 with regression-first evidence, then obtain fresh full Consistency/Safety review as required for the changed lifecycle rule, preserving the original-finder closure record.