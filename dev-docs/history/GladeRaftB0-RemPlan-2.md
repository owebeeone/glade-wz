# Q4-B0 remediation 2 — caller-owned RPC termination

Date: 2026-10-03. Status: **correction authorized; acceptance pending**.
Revised object root `256be2dd0fd652b34dfffa753fcba481f5fb842b`; member/Gyld pins
unchanged from [remediation 1](GladeRaftB0-RemPlan-1.md).
[Consistency](GladeRaftB0-ReviewConsistency-1.md) and
[Safety](GladeRaftB0-ReviewSafety-1.md) independently closed all initial findings
and converged on a NEW ARCHITECTURAL ROOT CAUSE: remote delivery validity was
incorrectly made a precondition of caller-owned local termination. The revised
object remains NO-GO. This is the second bounded architectural remediation round.

| Finding | Disposition | Closure |
| --- | --- | --- |
| Consistency P2-4; Safety P2-5 | Split exact issued local RPC ownership from live peer/request/reply validity. A live current caller MUST be able to select Timeout/Cancel for its issued pending RPC even when the peer stops or is replaced. Respond/Resolve/message delivery MUST retain live peer/incarnation checks. | For held AND consumed requests, peer stop AND replacement, Timeout AND Cancel: poll response Pending, terminate locally, verify only that RPC removed, correct typed terminal error after an explicit selected poll, wake eligibility without inline polling, unchanged original bytes and no peer mutation. Repeated terminal/foreign/forged/stale-caller controls and late replies refuse unchanged. Healthy caller continues traffic. A peer-liveness-for-cleanup mutant fails. |

The wrong passing expectations in the prior endpoint tests MUST be corrected;
they are evidence of the bug, not a contract to preserve. Existing no-mutation
refusals apply to stale/foreign delivery or caller controls, not a live caller's
valid local cancellation. Stop/cleanup does not require a participating peer.
The actual public signatures, std-only roles/edges, lockfile, both refusing
providers and all ten case IDs per provider remain unchanged. Clarify allocation
text if needed; do not change consensus or install engines.

The drafter MUST record compiling behavioral RED before fixing implementation.
Preserve prior logs/inventories and record current measured GREEN/20 ordinary RED,
structural/global/source-scope/format and all-target denied-warning Clippy results.
One patch MUST cover both finding IDs; no self-closure.

The originating reviewers MUST verify their original counterexamples on the
new settled tuple. In addition, fresh peer-blind full Consistency/Safety reviewers
MUST review the complete corrected object because the prior patch changed the
lifecycle ownership rule. File all reports verbatim and identify which are
original-finder closure versus the full acceptance gate. A third new architectural
root cause on this object stops the lane for owner redesign-or-accept; no third
architectural patch is authorized by this plan. No runtime or production exit is
closed by this correction.
