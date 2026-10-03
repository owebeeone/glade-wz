# Q3 contract remediation 1

Date: 2026-10-03. Initial root: `fa1ff8b9fd0be53932300730ff925d0e41c76b1a`, unchanged member/Gyld pins in the generated prompts. Consistency GO; Safety NO-GO with three distinct P2 findings. No blind convergence. All findings are accepted for correction, not self-closed.

| Finding | Disposition in one patch | Closure specification |
| --- | --- | --- |
| Safety P2-1 | Correct Create command preconditions to generation/home 0/0, retaining Action's selected home. Require complete Accepted resource in success setup. | First observe failing exact fixture regression; verify the corrected command against the existing application with nonzero generation/home negatives, without changing application semantics or adding a normal contract-to-implementation edge. |
| Safety P2-2 | Define deterministic current-live-home revalidation at the actual LeaveJoint predecessor cut; a committed exit which would strand a home retains Refused(HomeInUse), does not apply ConfChange, and remains joint. Admission refusals cannot replace ordered revalidation. | Compiling RED schedules for placement on outgoing-only voter during joint, restart, placement queued between exit admission/application, exact refused retry, qualified movement or retirement then successful exit. |
| Safety P2-3 | Introduce typed index replay result for application/configuration/noop original outcomes; preserve complete original entry/envelope identity. | Compile every result variant consumer; RED original accepted/refused config replay after checkpoint/install/restart, noops and changed-envelope rejection; update application replay consumers. |

The patch MUST remain contract/specification-only. No Q3 Raft/store/codec implementation, assertion inversion, hidden skip, dependency-role relaxation, global exception or production change is authorized. All behavioral specs MUST remain intentionally RED on NotQualified, with fixture compatibility checks independently GREEN after observed RED. Run focused compiler/fixture/spec tests, affected Q2 tests and structural/Clippy gates.

P2-3 changes the shared replay interface and P2-2 clarifies its mutation rule. Under review-loop §5.5 the revised object therefore requires a fresh dual Consistency/Safety review, in addition to originating Safety verification of its original counterexamples. One remediation round is in progress; the object is not accepted until reviewers return GO on the corrected settled tuple. Record evidence and exact tuples in the qualification ledger. Preserve initial reports verbatim.


## Verified disposition

All three findings are closed for the contract/specification gate at corrected root `ed243db983c485e46a27aa870ec745de16a56d7a`, unchanged member/Gyld pins. [Originating Safety](GladeRaftQ3Contract-ReviewSafetyClosure-1.md) verifies its original counterexamples and closure specifications; fresh [Consistency](GladeRaftQ3Contract-ReviewConsistency-1.md) and [Safety](GladeRaftQ3Contract-ReviewSafety-1.md) both GO on the changed shared boundary. One remediation round; zero open findings. Actual algorithms remain unimplemented, 19 behavioral specs intentionally RED, and Q3 implementation acceptance remains a separate gate. Earlier pending language is historical, not the final verdict.
