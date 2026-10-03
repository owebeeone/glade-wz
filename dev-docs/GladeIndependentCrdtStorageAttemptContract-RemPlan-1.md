# Storage-attempt typed checkpoint — remediation 1

Date: 2026-10-04. Status: **planned; all findings open pending originating
verification and fresh settled-tree review. Kernel remains refusing.**

Reviewed root `a696f0eef38614fe0cfe2a6b053c352470e800b3`; Glade
`3cf1fa79cd752012acd0d2ff66d595e293b3433c`; Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld
`95a426595bba8e248a5f484272e483a070c73918`.

Both [Code](GladeIndependentCrdtStorageAttemptContract-ReviewCode.md) and
[State](GladeIndependentCrdtStorageAttemptContract-ReviewState.md) returned NO-GO.
Reports are filed verbatim, SHA256 respectively
`4f54848c6e496ad4855cf34937e8cb24f1287dbbfaf84a06365c546068dc3729` and
`3ad781dcc5a298c25b547ad1e73349fe1a3e40a2dbd45d36c24c56a8b25735f7`.
Blind convergence occurred on authoritative policy observation composition:
Code P2-1 and State P2-2 identify the same root. Five finding IDs represent four
distinct roots. State P2-1 is architectural; all others are non-architectural
fixture defects. This is the first remediation round on this redesigned typed
object, with one architectural root so far. The failed original IC-1 object's
separate three-root stop remains historical and unchanged.

## One merged correction

| Finding | Disposition | Executable closure |
| --- | --- | --- |
| State P2-1 | Retain actual invocation-history consumption or sufficient authenticated history independently of the scalar identity floor. Validate restored capacity and identity consistency; sparse IDs MUST remain legal and consume one entry per actual issued request. | Prepare with sparse ID 128 under history limit 128, restore authentic state, reject stale ID, allow fresh inspect/fence, retain equal remaining capacity and exhaust exactly the declared actual count. Corrupt cardinality/floor/limit restoration MUST refuse. |
| Code P2-1 / State P2-2 | Configure the actual test producer from explicitly trusted fixture policy/time inputs before driving work; deliver independent policy changes through that provider. MUST NOT convert each Begin caller claim into an authoritative observation. | A public-port journey using the exact core fixture policy digest/time publishes the original receipt; a separately changed provider cut causes stale Begin to return NonCommit. Preserve independent validation and keep kernel assertions RED. |
| Code P2-2 | Retain one caller-owned storage session per logical replica for all mandatory multi-call journeys. Seeded states MUST use explicit consistent trusted recovery, including revision, custody/outcomes, issuance and reservations; a fresh empty host MUST NOT silently accompany nonempty kernel state. | Public-port consecutive publications reach revisions 1 and 2 and retain the first outcome. ABC, buffering, rival and post-fork journeys use retained sessions or explicit qualified test recovery. Preserve every original assertion, loop and all ten released-Taut rows. |
| Code P2-3 | Register injected lookup continuations through actual emission or a complete trusted fixture helper that populates both maps, retains the full issued request, and maintains finite history/counter accounting. Secondary lookups MUST NOT become alternative callback authority. | Pair a valid issued lookup that retires unchanged into consumed history with an identical unissued reply that preserves state. Run each malformed-terminal mutation from the issued fixture so its intended payload mismatch is discriminating. |

Write regression consumers before fields/helper changes and record compiling
behavioral RED before any fixture implementation. No successful admission
implementation is authorized; `crdt-admission-core/src/lib.rs` MUST stay
byte-identical. Fixture GREEN qualifies only the development port/assembly.
Preserve all 40 existing kernel behavior tests and their coverage; adding focused
regressions is permitted. Keep the canonical text corpus, exact ten text assertions,
Gyld frozen sources, package roles, minimal edges, test selection/budget and
process-global allowlists. No unrelated refactor or engine expansion.

The sole drafter MUST make one merged patch, update the controlling contract and
file separate remediation evidence, including explicit tested paths and timings.
Relevant API conformance, core source/representation, compiling domain RED, real
text RED/control, fmt/clippy, selection, architecture and process checks MUST be
recorded. Run external Gyld checks only if its source/contract allocation is
affected; do not broaden or repeat tests without a changed concern.

Both originating reviewers MUST verify their own counterexamples on the corrected
settled tuple. Because Recovery's shared interface changes, fresh full peer-blind
Code/State review is also required. The drafter and owner MUST NOT self-close
findings. At most two remediation rounds; a reviewer-classified third architectural
root on this object stops for owner redesign-or-accept. No push or live operation.
