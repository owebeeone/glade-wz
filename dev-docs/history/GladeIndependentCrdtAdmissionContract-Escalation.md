# IC-1 typed contract — architectural stop

Date: 2026-10-04. Status: **NO-GO; lane stopped pending owner redesign-or-accept
decision. No successful kernel implementation is authorized.**

Reviewed object: root `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`, Glade
`6a0cc5a78da38a023615f6adc5fc354bba4af0b4`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`64666e8b1caadde8922b9d42163afbab90655c65`.

## Evidence and stop condition

Restored Consistency and Safety roles independently verified all original
counterexamples corrected at the internal-contract/compiled-RED tier. Their
reports disclose that the original processes were unavailable; context was
restored from verbatim filed reports. Fresh full Consistency reports GO with
zero findings. Fresh full Safety reports NO-GO with one new P2.

| Architectural root | Reviewer classification | State |
| --- | --- | --- |
| Repeated lookups lacked distinguishable invocation identities | [Consistency classification addendum](GladeIndependentCrdtAdmissionContract-RootClassification-1.md) | Corrected, verified at contract/RED tier |
| Committed retention recovery required an invented application receipt | [Initial Safety report](GladeIndependentCrdtAdmissionContract-ReviewSafety.md) | Corrected, verified at contract/RED tier |
| Negative recovery does not distinguish temporary absence from terminal noncommit | [Fresh Safety P2-1](GladeIndependentCrdtAdmissionContract-ReviewSafety-2.md) | OPEN; blocks acceptance |

The restored Consistency reviewer explicitly classifies the first root as
architectural and confirms the three-root accounting. Corrected roots remain in
the review history. The [review-loop skill](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
requires: “If a reviewer … identifies a third new architectural root cause on the
same object, stop the lane. Do not draft another patch.” One merged remediation
round has occurred; the architectural-root stop is separate from round count.
The lane owner MUST NOT bypass it by relabeling this finding or resetting the
object's history. No second correction patch has been drafted.

## New counterexample

An original atomic write P can remain in flight after its waiting task loses the
answer. A lookup observes no outcome entry and answers KnownAbsent; the kernel
releases P's reservation. P then commits physically, but its delayed callback is
retired. In-memory custody, accounting and physical revision can diverge.
Matching identities and atomic installation do not prevent this interleaving.

A timeout, cancellation or missing index entry MUST NOT be interpreted as proof
that the original attempt cannot commit. The reviewer requires terminal-negative
semantics for both lookup KnownAbsent and direct KnownNotCommitted, with a pending
write/absent observation regression. Actual storage fencing, cancellation and
reopen qualification remain mandatory at IC-3.

## Recommended owner decision

**Recommend redesign of the IC-1 storage-outcome lifecycle, preserving the accepted
general CRDT design and existing regression obligations.** The redesign brief
should cover commit attempt ownership, outcome finality, ordering/fencing, restart
recovery, retained reservation/receipt identity, and the physical host's proof
obligations as one coherent lifecycle. It should include pending-write/negative
lookup/later-commit and terminal-noncommit journeys before a fresh contract gate.
This recommendation is an operator choice, not a new implementation patch or
permission to start that redesign without the required decision.

Accepting the known defect is not recommended. The current refusing scaffold is
safe and unchanged. Its 27 Rust and ten released-text behavior obligations remain
RED. The accepted semantic CRDT design and Gyld allocation are preserved;
Raft production integration, live stores, keys, enrollment, activation and pushes
remain untouched. Reports are filed verbatim; the review ledger records hashes.
