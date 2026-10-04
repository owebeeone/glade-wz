# Storage-attempt typed checkpoint — non-architectural correction 3

Date: 2026-10-04. Status: **planned; State-3 P2-1 OPEN; kernel remains refusing.**
Reviewed root `fb6d69ff5153865266782f0b5c11d7a00086552c`; Glade
`c6c4239beecb129aa0585fe74006cc287dff3b87`; unchanged Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`.

[Code-3](GladeIndependentCrdtStorageAttemptContract-ReviewCode-3.md) returned GO,
closing all three Code-2 findings. [State-3](GladeIndependentCrdtStorageAttemptContract-ReviewState-3.md)
closed all three State-2 findings but returned NO-GO for one distinct historical
revision-consistency validator defect. State explicitly classifies it
**non-architectural**, using existing fields and boundaries. Architectural count
remains one; there is no new redesign object or cap reset.

The [review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>)
step5 expressly permits this exception: “A third round confined to non-architectural
corrections is permitted; any architectural root cause found in it triggers the cap.”
This third correction is confined to the existing development recovery validator.
Any architectural root found in this round MUST stop the lane for owner decision;
no extra architectural patch is authorized.

## Exact correction and closure

State-3 P2-1: restoration MUST reject distinct Committed attempts sharing one
resulting application revision within an instance. It MUST preserve unique exact
identities, immutable terminal histories, independent instance revision spaces and
unchanged-revision NonCommit histories. No terminal, binding or application head
may be rewritten to make an inconsistent image acceptable.

Write and execute the reviewer’s authentic two-commit recovery mutation before
validator changes: revise only the second expected/committed revision from1/2 to0/1,
with its distinct identity/batch retained. It MUST fail rejection against the current
provider. Then minimally enforce historical committed-revision consistency using
existing fields. Positive controls MUST include sequential commits1/2, multiple
NonCommit outcomes at one unchanged revision, and X/Y each committing revision1.
Adjacent missing-head/gap mutations SHOULD be checked where the already-required
full retained history and custody/outcome coupling make them detectably inconsistent;
do not invent a new import/base-revision exception or external rollback proof.

Keep the shared API, refusing kernel, accepted lifecycle, package/assembly/mutation
boundaries and all existing test obligations unchanged. No new fields, dependencies,
roles, budgets, allowlists, external Gyld change or broad source refactor. Any need
for a new boundary or changed contract premise MUST be reported before writing it.
File separate test-first/gate evidence and narrowly update the Contract. Run affected
provider/consumer checks; no unchanged broad reruns. Sole drafter, one merged patch,
no Git mutations or self-closure.

The originating State reviewer MUST retrace P2-1 and inspect the corrected range.
Code's GO is retained as prior evidence, but its reviewer MUST confirm the final
exact tuple and unchanged invariants for aggregate GO/GO. Both reports are filed
verbatim. Success accepts the typed/model/compiling-RED checkpoint only; pure
admission, physical storage/auth/duplex integration and activation remain separate.
