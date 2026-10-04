# Q4-B0 remediation 1 — SAFETY-AXIS REVIEW

**Review object:** Focused correction from root `544c83d8cd07165cfeec2f0db64a78c8417849f3` through `256be2dd0fd652b34dfffa753fcba481f5fb842b`, controlled by `dev-docs/GladeRaftB0-RemPlan-1.md`. DRAFT private contract/allocation/compiling RED gate; no runtime or production acceptance.

**Baseline:** Revised root `256be2dd0fd652b34dfffa753fcba481f5fb842b`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources read through pinned `git show` and scoped `git diff`. All four HEADs matched at start and end.

**Date:** 2026-10-03  
**Axis:** Safety: original-counterexample closure, ownership, terminal lifecycle and newly introduced failure paths. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — all four original Safety findings close, but one new P2 finding blocks. I pre-commit to GO on a revision resolving P2-5 as specified.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
|---|---|---|---|
| Safety P2-1 | Respect actual entropy cadence | Retraced B0-03: OpenRaft requires unchanged constructor-only attempts and healthy state; raft-rs selects the established leader’s quorum-loss reset. Once-only witness and extra-read detector passed. | Closed within this contract/RED gate |
| Consistency P2-1 | Same cadence correction | Same original impossible-runtime-read sequence retraced; unconditional runtime entropy failure requirement removed. | Verified closed |
| Safety P2-2 | Shared endpoint stop/current-incarnation enforcement | Executed separate/fresh handle stop, stale issuer/destination and unchanged-token regressions. Original stale emit/request/release/take sequences now refuse. | Closed; separate new restriction is P2-5 |
| Consistency P2-2 | Same stale ordinary-traffic correction | Executed replacement and ordinary-token tests; validated shared registry before mutation. | Verified closed |
| Safety P2-3 | Reconcile in-flight work with stop | Executed original own-scope-stop-during-poll sequence: selected future drops once, inventory is Cancelled, saved wake cannot revive it. | Closed |
| Safety P2-4 | Drop futures outside scheduler borrow | Executed parent destructor cancelling/inspecting child during cancel and bulk stop; no panic, exact-once drops, terminal inventories. | Closed |
| Consistency P2-3 | Same destructor correction | Same original nested-borrow counterexamples executed; rejected registration and completed-future cleanup also passed. | Verified closed |

## Changed-range analysis

The correction changes source-attempt instrumentation, B0-02/03 fault selection, fixture endpoint/scheduler enforcement, allocation/evidence documentation and measurement tooling. It adds two focused integration targets and unit regressions. Architecture policy adds target names only.

Verified unchanged public API, package manifests, lockfile, provider implementations and comparison contract. Governing sources and adaptation inventory also remain unchanged. No engine or production dependency was introduced. The separate source-adaptation plan is DRAFT/out of scope and was not adopted.

Most changes enforce existing obligations. However, allocation’s remediation table and new endpoint tests also establish a **new remote-liveness precondition for caller-owned Timeout/Cancel**. This changes lifecycle semantics despite unchanged Rust signatures.

**NEW ARCHITECTURAL ROOT CAUSE: P2-5.** It concerns which scope controls termination of an owned pending operation. It is separate from the original stale-traffic root cause. The new lifetime restriction cannot be accepted through original-finder closure alone; fresh full review is required for a revision retaining or redefining that restriction. Restoring local termination rights is the bounded correction specified below.

## 0. Evidence base

Read the full canonical remediation prompt, both original reports and merged plan. Retained the previous governing-source review, and verified those documents’ bytes are unchanged at this pin: AGENTS files, ProductionIntegrationPlan, QualificationPlan including §6, AdoptionContract, ImplementationEvaluation, Q4-CarrierAudit, library policy, package architecture and BuildEntry.

Inspected the complete scoped implementation diff and both new test targets. Material ranges:

- `spec/src/b0/sources.rs:4–126,277–374`
- `spec/src/fixture/transport.rs:41–81,247–355,380–414`
- `spec/src/fixture/work.rs:61–101,133–224,248–299`
- `spec/tests/endpoint_lifecycle.rs:10–237`
- `spec/tests/scheduler_lifecycle.rs:12–215`
- unchanged API transport contract at `api/src/lib.rs:215–251`

Compared committed/local bytes before running the README commands. Independently observed:

- API/provider compiler witnesses: **4 passed**.
- Spec unit/compiler/fixture/oracle/lifecycle witnesses: **31 passed**.
- Total GREEN: **35**, zero ignored/filtered.
- Ordinary B0 target: exit **101**, **20 behavioral assertion failures**, zero passed/ignored/filtered.
- Structural, source-scope, empty-global-allowlist, five architecture negatives and formatting: PASS.
- All-four-package all-target Clippy with denied warnings: PASS.

Inspected `rem1-red.log` and overflow supplement: recorded compilation followed by 12 failures and one additional distinct failure respectively; authoring compile errors are correctly separated. These historical RED executions were inspected, not reconstructed. Current GREEN was independently replayed.

Verified all **82 current inventory entries plus inventory itself**, documentary-context digests, and clean committed subtree outside ignored target/cache artifacts before and after. Inventory SHA-256: `f6ce42f4423062a5baeb7eb12273db62b63debd9455e326cdfa5e2eb2d10d88c`. Original evidence remains separate. No source, test, evidence or Git writes were performed.

## 1. Findings

### [P2-5] Remote peer termination disables the live caller’s RPC timeout and cancellation

**Location:** `spec/src/fixture/transport.rs:54–69,340–355`; `spec/tests/endpoint_lifecycle.rs:104–121,124–179`; allocation’s remediation endpoint row.

**Violated invariant:** A live caller owns its issued pending RPC and must be able to terminate it through explicit Timeout/Cancel. Preventing stale traffic or stale replies must not remove local cleanup when the remote endpoint becomes unavailable.

**Reproduction:** Create caller 1 and peer 2. Caller issues a pending RPC. Stop peer 2, or replace its incarnation while caller 1 remains live. Invoke `Timeout(rpc.id)` or `Cancel(rpc.id)` through caller 1.

The new branch invokes `network.rpc(rpc)`, which invokes `token(state.request)`. That requires the request destination to remain live. Peer stop/replacement therefore produces `InvalidToken` before removing caller-owned RPC state or settling its response future.

This behavior was directly exercised by the existing new tests: they deliberately assert refusal and unchanged pending ownership. `stopped_peer_and_caller_reject_all_rpc_and_queue_mutations` confirms that only stopping the entire caller subsequently removes the stranded RPC.

**Impact:** Remote failure prevents local timeout/cancellation from completing. An awaiting future remains pending indefinitely unless the healthy caller stops its entire endpoint. Repeated requests to failed/replaced peers can retain unreachable work and RPC ownership. The new tests ratify the defect rather than establish closure.

**Required correction:** Separate local RPC-ownership validation from live-peer validation. Resolve/respond and traffic delivery must still reject stale peers/replies. Timeout/Cancel must validate the live caller, exact issued pending RPC and nonterminal ownership, then settle locally without requiring the remote incarnation to remain live. Update the allocation and tests accordingly.

**Closure test:** For held and consumed requests, stop and replace the peer while keeping the caller live. Explicit Timeout and Cancel must each remove only that RPC, wake its future, and yield the correct typed terminal error at the next selected poll. Late replies and repeated terminal controls must refuse. Foreign, forged and stale-caller RPC IDs must refuse unchanged. Healthy caller traffic must continue without whole-endpoint stop.

## 2. Invariant analysis

The original failures no longer reproduce. Shared endpoint lifecycle blocks stale participation; scheduler metadata preserves in-flight ownership; cancelled/completed futures are detached before arbitrary destructors execute.

Fault selection now preserves OpenRaft’s constructor-only entropy cadence and deliberately reaches raft-rs’s documented leader reset. Pure footprint witnesses establish consumer/oracle behavior only; actual adapted-engine cadence remains unexecuted.

The four std-only package roles and edges remain intact. Both labels retain all ten common scenarios. No global exception, source substitute, production authority, receipt reinterpretation or default activation was introduced. B1/B2/full comparison and Q4-C/D/E remain mandatory.

## 3. Risks and next action

GREEN fixtures do not establish real-engine behavior, source adaptation, complete dependency compliance or production qualification.

The next action is a regression-first correction of P2-5, followed by the applicable review on a newly settled object. Original-finding closure remains valid; this revised object is not accepted while the new lifecycle defect remains.