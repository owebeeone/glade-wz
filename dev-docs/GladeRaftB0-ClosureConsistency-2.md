# Q4-B0 Remediation2 original-finder closure — CONSISTENCY-AXIS REVIEW

**Review object:** Consistency P2-4 closure in the DRAFT private B0 contract/allocation/compiling RED object at `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`, controlled by `dev-docs/GladeRaftB0-RemPlan-2.md`.

**Baseline:** Workspace root `b2df51042ae01afd1b42cb12e2de8b28f1106ec5`; glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Committed sources were read through pinned Git objects; working copies were matched against committed bytes before execution. All four HEADs matched at start and end.

**Date:** 2026-10-03

**Axis:** Consistency, original-finder closure only. Independent, adversarial, read-only. No current peer or full-review report was read; nothing here relies on those reviews. Filed verbatim by the lane owner.

**Verdict: GO** — Consistency P2-4 is closed on this tuple, with no new finding identified within this focused closure. This verdict does not grant aggregate contract/allocation acceptance, runtime qualification, carrier selection, source-adaptation acceptance or production acceptance.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Consistency P2-1 | Source-fault expectations follow actual carrier access cadence. | Re-ran constructor, oracle and source witnesses, including constructor-only OpenRaft entropy, rejection of extra runtime sampling and persistent-work registration behavior. | Earlier closure retained. |
| Consistency P2-2 | Shared live-scope registry governs ordinary endpoint traffic. | Re-ran all seven endpoint lifecycle tests, including separately obtained handles sharing stop, replaced issuer/destination refusal before mutation and valid replacement participation. | Earlier closure retained. |
| Consistency P2-3 | Owned futures are detached before arbitrary destructor execution. | Re-ran scheduler lifecycle tests and overflow cleanup witness: reentrant child cancellation, exact-once destruction and own-scope stop during poll remain GREEN. | Earlier closure retained. |
| Consistency P2-4 | Local Timeout/Cancel validates caller-owned pending RPC independently of remote liveness. | Executed all eight original-counterexample matrix cells, two additional negative witnesses and the exact peer-liveness mutant. Corrected tree passes; mutant produces eight compiling assertion failures. | Closed. |

## Changed-range analysis

The scoped diff from `256be2dd0fd652b34dfffa753fcba481f5fb842b` changes transport validation, corrects two previously wrong endpoint expectations, adds the termination regression module and isolated mutation checker, and updates allocation/README/evidence.

`Network::owned_rpc` now validates exact issued pending ownership. `Network::rpc` adds remote-token validity to that validation. Only Timeout/Cancel switches to `owned_rpc`; Respond/Resolve retain `rpc` and live-token checks. The control entry point still requires a current, live caller and the matching caller scope.

Public API signatures, package classifications, declared dependencies, lockfile, provider implementations and B0 case selections remain unchanged. The comparison contract itself is unchanged.

This corrects the previously classified **NEW ARCHITECTURAL ROOT CAUSE** concerning completion ownership. No additional architectural root cause was identified in this closure. Two architectural remediation rounds remain recorded. Because the lifecycle ownership rule required correction, the mandated fresh full reviews remain necessary; unchanged signatures do not waive them.

## 0. Evidence base

I read the complete canonical closure prompt and verified SHA256 `cfa45caba09ecd6ce52ccf91cd4804e29889e133ba41243fe2db46fd20230822`. Legitimate inputs included both original reports, both remediation1 reports and RemPlan2. Governing sources included AGENTS instructions, review-loop authority, QualificationPlan §6, the comparison contract/allocation and their controlling production, adoption, library-boundary and package documents. The separate DRAFT SourceAdaptationPlan was not adopted.

Focused implementation inspection covered:

- `spec/src/fixture/transport.rs:43–104`, `196–237`, `261–368`: caller/peer validation, settlement and control boundaries.
- `spec/src/fixture/transport/termination.rs:3–222`: eight-cell matrix; `224–341`: stale-caller and live-peer late-reply witnesses.
- `spec/tests/endpoint_lifecycle.rs`, `check-termination-mutant.py`, allocation lines 95–105 and 341–400, and README’s current commands/evidence.

I executed the exact README API, provider and spec GREEN commands: **45 passing tests**, comprising 41 spec witnesses and four API/provider compiler witnesses. The unchanged election target produced **20 ordinary assertion failures**, zero ignored or filtered, at the refusing providers’ stated `NotQualified` boundary. Structural/global/source-scope/format checks and all-target denied-warning Clippy passed. The global check reported zero allowlist entries; architecture negative witnesses passed.

The allowed mutation command returned wrapper exit 0 after actual copied-fixture Cargo exit 101: **six passing units and exactly eight failed matrix assertions**, zero ignored or filtered.

Recorded pre-fix `rem2-red.log` contains ten compiling behavioral failures: eight matrix cells and two corrected endpoint expectations, with ten passing tests. Current evidence correctly distinguishes the two later GREEN edge witnesses from observed RED.

Before and after execution, all 102 entries in `rem2-files.sha256`, plus the manifest itself, matched committed bytes. Manifest SHA256 was `5415b79ebaed75de6c5bf49d7619b0661516651c6982a427aef70b6fbef8e789`. Reviewed sources and evidence were unchanged. Only authorized ignored build artifacts were produced.

## 2. Invariant analysis

The original counterexample was a live caller with a pending RPC whose peer stopped or was replaced. Remote delivery became invalid, and the old validator consequently rejected local Timeout/Cancel, stranding caller-owned completion.

The corrected matrix independently exercises held/consumed request × stopped/replaced peer × Timeout/Cancel. Each cell polls the actual response future to Pending before the fault. Local control then removes exactly the selected RPC, preserves unrelated caller ownership, and leaves queued views, original opaque bytes, old/replacement peer RPC inventories and peer work unchanged.

Completion is observable through the actual future: control records no inline execution, marks its task Runnable, and the next explicit selected poll completes once with precisely `TimedOut` or `Cancelled`. The healthy caller subsequently delivers ordinary traffic to another peer.

Foreign, forged, repeated-terminal and stale-caller controls refuse without mutation. Stopped/replaced peers cannot Respond or Resolve. A separate live-peer late-reply test proves refusal depends on terminal RPC ownership even when remote liveness remains valid. Stale caller replacement cannot settle or wake the old pending future.

The mutant restores only Timeout/Cancel’s former `network.rpc` precondition. All eight cells fail at the positive local-termination assertion, returning `InvalidToken` instead of `RpcResolved`. This directly demonstrates detection of the original defect.

## 3. Risks and next action

These results establish controlled fixture completion behavior and honest compiling RED evidence. They do not execute either consensus engine or qualify source adaptations, real callbacks, production allocation or later mandatory gates.

The next action is the already mandated fresh peer-blind full review of the complete corrected object. This report supplies only the originating Consistency finder’s closure; aggregate acceptance remains pending those reviews.