# Glade Raft Q4-B0 — private contract/scaffold acceptance

Date: 2026-10-03. Status: **contract, allocation and compiling behavioral RED accepted; actual B0 carrier qualification remains open**.

The lane owner records aggregate GO after the independent originating
[Consistency](GladeRaftB0-ReviewConsistency-3.md) and
[Safety](GladeRaftB0-ReviewSafety-3.md) reviewers closed every finding at the
same settled tuple. Reports are filed verbatim. This acceptance MUST NOT be
treated as engine adaptation, automatic-election or production acceptance.

## Exact accepted source

| Repository | Revision |
| --- | --- |
| Workspace root | `8553bc71b1f6bc9fe203729e61567e8b9bd37be2` |
| Glade | `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87` |
| Glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| Grazel | `c839fe87c9d18ebb6e995964d2e79aef7cbd380e` |
| Glade-Gyld | `327d62c0033db0fae145d002a00663a3826b6d53` |
| Glade-GWZ | `35b38ba0845a7cb7034a4af3609975ea1bd48741` |
| Glade declaration Rust | `b85044e1f6631114dbb290c02298e644f8363055` |
| External Gyld | `ca04499a360d910fbf8ee2540ed446facd051b35` |

The reviewed object comprises the [comparison contract](GladeRaftCarrierComparisonContract.md),
[B0 allocation](GladeRaftB0Allocation.md), and the four std-only packages in
`proofs/raft-carrier-comparison/`, including their retained evidence. The current
`evidence/rem3-files.sha256` contains 136 root-relative entries, excluding itself;
its SHA-256 is `bf01cd40a7b2c9a4e1eb45abd03c60d7b62aaadbe2830d7ba0665d26c6276fb7`.
Earlier inventories remain evidence of their earlier checkpoints. Inherited
working-tree edits are excluded. Proposal headers at the source pin remain
unchanged; this separate record supplies the acceptance disposition.

## Verified result and closure

Both reviewers independently reproduced **58 GREEN witnesses**, **20 ordinary
B0 assertion failures** against the refusing providers, structural/source/global/
format gates and denied-warning Clippy PASS. No ordinary B0 case is ignored or
filtered. The exact RPC-termination defect mutant compiled and produced eight
required failures and nineteen unrelated passes. The eleven distinct compiling
regression failures preceding the third correction are retained separately from
authoring compiler errors; GREEN does not erase historical RED.

The initial seven P2 IDs mapped to four defects. Remediation 1 closed those but
revealed a fifth defect: caller-local RPC termination incorrectly depended on
peer liveness. Remediation 2 corrected it; fresh full reviewers found two further
non-architectural defects in work selection and clock-domain encoding.
Remediation 3 corrected both, with originating-finder closure and independent
changed-range verification. **Seven distinct defects, eleven reviewer IDs, zero
open acceptance findings** are recorded across the cycle. No production escape
is observed; no provider was activated.

Two architectural remediation rounds and one confined non-architectural round
were used under the [review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>).
Both final reports found no new architectural root cause. The refreshed work
drain preserves separate selected polls and actual logical timing points;
bounded bit fields preserve domain isolation. API, package roles, dependency
edges, lockfile, providers, allowlists and twenty common case labels did not
change in the final correction.

Final [Consistency prompt](GladeRaftB0-PromptConsistency-3.md) SHA-256:
`50f588a080c8acfbf3a1f9b7c1322c22d602cc4af5c463c0a38d4e1a1ec6be05`.
Final [Safety prompt](GladeRaftB0-PromptSafety-3.md) SHA-256:
`096d58d26654b992aabde7f475057fade6e5e21d332be7665588cd50fdabf4b0`.
Verbatim report SHA-256 values respectively:
`51b0a15053b8f0886bd7261ac2b3fda79ba7650201dfc8cb2281b01e19e48772`
and `85ff7fb9ee10cd96153efdbaadbb1653bd05715fa6fa02628f5ceb07568858e8`.

## Remaining gates

Neither raft-rs nor OpenRaft is installed in this scaffold. Neither performs
actual elections here. Fixture mechanics and synthetic detectors MUST NOT count
as real carrier, runtime, durable receipt, cryptographic or dependency-security
qualification. The [source-adaptation plan](GladeRaftB0SourceAdaptationPlan.md)
remains a separate unadopted DRAFT. Its runner allocation, owned runtime/error/
logging sources, generated codec and complete resolved graph need their own
compiling contracts and reviews before implementation.

Both carriers MUST execute the common B0 journey after adaptation, then the same
complete application/disk/fault B1 and membership/snapshot B2 journeys. B3 crate
selection requires the full comparison; no candidate or mandatory journey is
dropped by this acceptance. Q4-C exact canonical amendments require owner
[profile selection](GladeRaftProductionProfileDecisions.md) and consumer review;
Q4-D provider integration and Q4-E migration/activation remain open. Accepted
Q4-A is only the separate legacy Store retirement interlock. This filing installs
no seal, merges or pushes no lane, enrolls no production group and changes no
development launcher or desktop build.
