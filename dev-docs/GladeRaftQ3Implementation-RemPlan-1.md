# Q3 implementation remediation 1

Date: 2026-10-03. Status: **planned, all findings open; Q3 acceptance pending**.
Reviewed implementation source: `b61197602e5594bdf89770bda069ce7d30fdb222`.
Unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.

Fresh [Code](GladeRaftQ3Implementation-ReviewCode.md) and
[State](GladeRaftQ3Implementation-ReviewState.md) both NO-GO. Reports are filed
verbatim. Three P2 defects block; two Code P3 findings identify mandatory
witnesses and are also included before qualification. IDs below are namespaced
by originating axis. There was no blind convergence on one defect: the axes
found distinct failure surfaces. This is implementation remediation round 1,
separate from earlier semantic-contract/allocation rounds.

## One merged patch and exact closure obligations

| Finding | Disposition | Required regression/closure |
| --- | --- | --- |
| Code P2-1: poisoned direct compaction remains serving | Accept. Stop the affected voter on failed direct checkpoint publication, preserving the exact error and durable evidence; physical reopen/validation is required before service. Keep the reviewed create/open and publication boundaries. | First reproduce with actual V2 PartialWrite returning an original receipt after `install` failure. Cover all five faults for direct install and CatchUp regeneration; no receipts/resources or messages from the failed voter. Reopen must recover exact prior/full image or quarantine a torn record, preserving originals where recovery succeeds. |
| State P2-1: stale first voter strands restart | Accept. Select an authorized manual recovery candidate using actual durable last-log term/index and a deterministic tie-break, not the first numeric ID. Preserve legal RawNode lifecycle, term guards, incomplete/removed-voter exclusion and quorum-loss unknown behavior. | Actual home-2 create, AddLearner4/catch-up, joint incoming234/outgoing123, disconnect2, commit leave through1/3/4, then reopen real stores. Observe stale-candidate RED; recover an eligible leader, full original receipts/Entry bytes and unchanged home2/generation1, accept a new mutation, repeat restart and verify genuine quorum loss. |
| State P2-2: ahead checkpoint silently succeeds without overlap validation | Accept. Validate all retained common committed history before selecting compaction recipients. Explicitly refuse unsupported ahead-of-local installation if no publication can occur. No successful no-op may stand for durable installation. | Two actual sessions: A creates payload11 at2; B creates99 at2 and advances to3. A installing B's internally valid future checkpoint MUST quarantine without any file, receipt or Entry mutation. A valid nonconflicting future checkpoint MUST explicitly refuse or actually install durably. Preserve same-history successful compaction/restart. |
| Code P3-1: claimed valid foreign fixture is internally mismatched | Accept the evidence gap. Preserve the existing outer-only mismatch case and label it accurately. Add a distinct coherent foreign envelope/history/binding fixture; do not add foreign-group runtime support or weaken fixed group70 admission. | Matching foreign outer/envelope/history bindings, original cut/term/configuration and complete materialized evidence; concrete target rejects without publication/reset/exposure. Demonstrate internal fixture coherence separately from expected target binding refusal. Update the matrix to name both classes accurately. |
| Code P3-2: nested joint/nonjoint leave witnesses absent | Accept the mandatory coverage gap; existing guards are not self-qualifying. Add concrete correctly versioned requests, distinct from stale-version tests. | Fresh EnterJoint while joint returns JointInProgress; fresh LeaveJoint while stable returns NotJoint. Assert unchanged complete log/configuration/files, no terminal outcome for the rejected keys, and successful subsequent legitimate configuration work. |

The drafter MUST run compiling behavioral RED before every implementation
correction. Coverage already green is recorded as coverage, not invented RED.
All corrections and witnesses form one patch, not a series of accepted
checkpoints. No public interface, role/edge classification, dependency,
process-global exception, production integration, push or desktop rebuild is
added. If satisfying a finding requires boundary recomposition, pause that
dependent edit and return the concrete conflict to the owner for the appropriate
review; do not weaken the controller.

## Verification and independent closure

Run the focused new actual regressions, original nineteen consumers, affected
carrier/disk tests and preserved Q1a/Q2 tiers. Then run the complete private
fixture workspace, the two explicit Q2 library cases, architecture/format/source
and process-global checks, strict all-target Clippy, both parent-oracle
self-tests and all two Q2/three Q3 SIGKILL cuts. Record exact final counts,
commands and changed-range evidence; retain historical checkpoint counts as
historical. No global exception or test-selection relaxation is authorized.

The owner independently verifies and freezes the combined patch at one exact
tuple. Originating reviewers MUST re-run/re-trace their original counterexamples
and supply prior-finding closure tables plus changed-range analysis. The drafter
cannot self-close any finding. A materially changed shared interface,
architecture, mutation/compatibility/platform boundary or call graph requires
fresh numbered reviewers under review-loop §5. Implementation remains pending
until Code/State both GO on the same corrected tuple. The two-round cap applies
to this object; reviewers classify any new architectural root causes.
