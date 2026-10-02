# Q2 persistence contract — remediation 1

Date: 2026-10-03. Source root `0d2649369b91e21ddb8ed557b28032c89334fa9a`;
member/Gyld pins unchanged. Combined initial gate NO-GO: Consistency GO with
one P3, Safety NO-GO with one P2 and the same P3. Reports are filed verbatim.

| Finding | Disposition | Closure |
| --- | --- | --- |
| Safety P2-1 | Correct the external crash oracle. Preserve the full original receipt in the parent, compare fresh-process lookup and retry with it; the before-apply cut instead uses an independently specified full expected receipt. | Regression rejects changed accepted home/generation and accepted-to-rejected substitutions with unchanged identity/index/payload; compile and retain scaffold RED, later run actual SIGKILL/recovery. |
| Safety P3-1 and Consistency P3-1 | Use Result expect/unwrap directly at recovery call sites to retain StoreError diagnostics. | Warnings-denied Clippy PASS; intended behavioral RED still shows NotQualified. |

This is one merged test/oracle correction. The storage traits, dependency roles,
journal design and production authority boundary MUST remain unchanged. Both
originating reviewers re-verify the corrected tuple; only Safety can close its
blocking finding. No adapter/integration implementation precedes acceptance.
