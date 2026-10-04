# IC-2 kernel — merged remediation 2

Date: 2026-10-04. Status: **one new correction authorized; Code-2 P2-5 open**.
Originating reviewers inspected root `aa4f7e28334baa4ef515487792ca785061de597c`,
Glade `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494`, unchanged other pins.
[State-2](GladeIndependentCrdtAdmissionKernelImplementation-ReviewState-2.md)
returned GO and closed all five original State findings.
[Code-2](GladeIndependentCrdtAdmissionKernelImplementation-ReviewCode-2.md)
closed all four original Code findings but returned NO-GO for one new P2.
Every original finding is closed by its originator; aggregate acceptance remains
blocked by Code-2 P2-5. No architectural root was identified.

This is the second merged remediation on the IC-2 implementation object;
architectural count stays zero. The cap and all historical stops remain intact.
The same source drafter MUST execute the new regression RED before correction.

| Finding | Disposition | Closure evidence |
| --- | --- | --- |
| Code-2 P2-5 — busy-instance diversion forgets observed historical work | Accept. Busy handling MUST preserve the distinct historical continuation within existing bounds, or explicitly refuse it and record sticky recovery incompleteness. Apply to fresh historical offers and already matched successful verification. Merely inspecting another plan does not classify the incoming record. | Actual-port two-query A/B sequence: hold A's Started publication, deliver B's historically valid verification while A is unresolved, settle A by Committed Inspect. B must remain recoverable and eventually install its exact identity, or an explicit refusal plus persistent incomplete status must survive Read, ResumeRecovery and validated fixture continuation. Check fresh historical offer during unresolved A, A's original receipts/charges, independent Y and local-intent-only refusal control. |

Preserve the nine verified original closures, accepted types/interfaces,
producer/assembly, immutable A reservation and pending/terminal evidence. No new
field, lifecycle operation, dependency, classification, selector, allowance or
clearing witness is selected. A conservative existing-field refusal MUST be
documented honestly; it is not eventual exact-cut reconstruction. Do not hide
the refused B by claiming A's settlement completed all observed history.

Run the focused regression, core/API all-targets, adopted architecture/test/fmt/
clippy, exact released-text/corpus and relevant source/process/whitespace gates.
No broad workspace or unchanged Gyld rerun. Recheck protected sources and record
the unchanged-mutant decision. Settle a new exact five-repository tuple before
originating Code closure and State confirmation on the same tuple. Both axes
MUST inspect the full correction and assess context-retention validity. New
material boundary changes require fresh full axes. No self-closure, push or live
activation. No additional round is silently authorized by this plan.
