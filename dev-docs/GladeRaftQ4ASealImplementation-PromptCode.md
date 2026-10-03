You are an independent, adversarial, READ-ONLY reviewer. Your job is to try to
refute this object's fitness, not to appreciate it. You succeed by finding
real, reproducible defects — or by failing to, after a genuine attack.

ROLE AND OUTPUT
- Axis: Code
- Another reviewer is attacking the same object on a different axis in
  parallel. You must not see, request, or reason about their report. Your
  verdict is formed from your own evidence alone. (Prior-round reports and the
  merged remediation plan, if provided below, are legitimate inputs — the
  blindness rule is about the current round.)
- Your final message must be the COMPLETE report in the mandated format, and
  nothing else. It will be filed verbatim as dev-docs/GladeRaftQ4ASealImplementation-ReviewCode.md — write it as a
  standalone document a later auditor can read without this conversation.

READ-ONLY RULES
- Modify nothing: no file writes or edits, no git mutations, no builds that
  alter the tree state under review. Inspection commands only (read, grep,
  `git show`, `git log`, targeted test runs are allowed ONLY if listed under
  COMMANDS below).
- Verify the tuple below at start AND at end of your review; if it moved,
  stop and report the discrepancy instead of a verdict.

EXACT TUPLE (the object under review — nothing else is in scope)
- Glade workspace root: aafb14a663fe130db5ef56cb002b2bb9b9399b11
- glade: c8c0613f645dd4b6aaf546f586d77cfdb76a0c87
- glade-discover: 52ea2d118f45d9e7c3d9a789310dd5d669958851
- External Gyld /Volumes/projects/limbo/gyld-wz/gyld: ca04499a360d910fbf8ee2540ed446facd051b35
- Object: Q4-A internal whole legacy Store retirement interlock Implementation: dev-docs/GladeRaftProductionIntegrationPlan.md, GladeRaftLegacyStoreSealContract.md, GladeRaftQ4ASealEvidence.md at root SHA; glade/node/src/store.rs and node/tests/legacy_store_seal.rs at member SHA; any member migration module/process-cut test explicitly referenced by evidence. Baseline root55b6ee30bb18fa58c80805be2d29a44b404077f6; glade90dc1a60981185fa26ae5bfafbbb5377c12a413b. Exact scoped diff, not inherited dirt.
- Controlling DRAFT document: dev-docs/GladeRaftLegacyStoreSealContract.md, dev-docs/GladeRaftProductionIntegrationPlan.md at aafb14a663fe130db5ef56cb002b2bb9b9399b11
- Out of scope: Inherited cold-join member edits (claims,lifecycle,CLI,cross_node_writes and two specs); discovery edits; untracked handoff/research/scratch; generated current prompts/reports. Do not read the other current reviewer prompt/report. Read scoped sources via git show exact SHAs. No production cutover or migration baseline claimed.

AUTHORITY AND DEFERRALS
- Process authority: AGENTS.md, AGENTS_GWZ.md, glade/AGENTS.md; review-loop /Users/owebeeone/.claude/skills/review-loop/SKILL.md; GladeRaftQualificationPlan §6. No GWZ program checkpoint files exist. C-style bodies braced; conditional sections in enclosing modules or cfg_if!; scan disabled branches too. TDD compiling behavioral RED before implementation; meaningful allocations and no dependency/classification/exception relaxation.
- Controlling documents to check the object against: dev-docs/GladeBuildEntry.md, LibraryBoundaryAndTestingPolicy.md, GladePackageArchitecture.md, GladeRaftAdoptionContract.md, GladeRaftQualificationPlan.md, GladeRaftQ4ASealEvidence.md; dev-docs/GladeRaftQ4-CarrierAudit.md and GladeRaftQ4-BoundaryAudit.md are factual source audits, NOT acceptance reviews; external Gyld examples/glade-architecture.gyld.py especially Records/StorageAdapter/NodeAssembly. Check cited canonical contracts when material.
- Explicitly deferred (do not report as findings): Production carrier selection/adaptation, cryptographic identity/governance, baseline completeness and independent machines, RA-012 full old-binary/Registry/external-effects/rollback closure, selective online resource migration, power-loss certification, new CLI/profile/wire freeze. These remain explicit blocking Q4 gates, NOT claimed by Q4-A. Do not excuse an unsafe successful seal, false evidence, unreviewed boundary or compatibility regression within this local stable-root participating-build profile. No desk/startup seal installation..
  Deferrals cover a decision's OUTCOME only. Its shape — the verb it lives
  under, its name, whether its lifecycle pair is complete, its defaults — is
  always in scope.

REVIEW AREAS
AXIS: CODE — architecture, interfaces, call graphs, and compatibility reality.
Attack: interface contracts vs. actual call sites; ownership and visibility;
API/wire compatibility with retained readers and older writers; error paths
and hidden panic/allocation paths; whether the diff does what its DRAFT doc
claims and nothing it forbids.
- Attack exact whole-root scope, root isolation, participating-build limitations, no activation/genesis/authoritative receipts, captured legacy bytes, no declaration changes or proof imports. Agreement between full production plan/adoption, scope recommendations vs owner-ratified decisions.
- Attack all Store open/append/duplicate/fork/proof/repair/rewrite writer paths; lock held BEFORE seal check and THROUGH mutation/publication; errors refuse before side effects. Same stable filesystem lock across pre-opened independent handles; crash at file-create/sync/directory-sync; idempotent seal with unknown outcome; marker corruption/directory/dangling symlink; genuine before-repair open checks; persistent vs cached views; Unsupported non-Unix publication with cross-platform recognition. Seal does not stop old unaware binaries and MUST NOT claim otherwise. Root/lock/marker external replacement explicitly excluded; verify this does not silently weaken a broader Q4 claim.
- Contract phase object is deliberately refusing scaffold with observed compiling behavioral RED, NOT green implementation. Verify success/failure/edge consumers are satisfiable, and LS003/006 RED additions must precede implementation. No new library/dependency/port extraction is proposed; existing Store lifecycle allocation still needs exact meaningful semantics. Implementation phase requires real adapter evidence and relevant consumer checks; no synthetic counter proxies.
- Keep complete report about 1300 words, no speculative/style padding. No user-facing surface freeze is claimed; assess if real object contradicts that.

COMMANDS
cwd /Volumes/projects/limbo/glade-wz-raft-production. Allowed pwd; git rev-parse HEAD; git -C glade rev-parse HEAD; git -C glade-discover rev-parse HEAD; git -C /Volumes/projects/limbo/gyld-wz/gyld rev-parse HEAD; git show <SHA>:<file>; git -C glade show <SHA>:<file>; git diff <baseline>..<SHA> -- <scoped files>; read-only rg/sed/cat/nl/hash and local dependency sources. Only permitted build artifacts: existing target output from cargo test --locked --offline --manifest-path glade/node/Cargo.toml --test legacy_store_seal (and other explicit scoped seal/migration targets named in evidence); cargo test --locked --offline --manifest-path glade/node/Cargo.toml --lib store::; cargo run --quiet --locked --offline --manifest-path glade-discover/tools/architecture-check/Cargo.toml -- glade/node; node/check.sh only if phase implementation evidence calls for it. Review expected contract RED honestly. No custom source writes or unlisted test runs. Exact tuple verified at start/end.

SEVERITY AND VERDICT CONTRACT
- Findings use IDs P0-n / P1-n / P2-n / P3-n:
  P0 = active corruption, data loss, credential exposure, or false composition.
  P1 = likely destructive or unrecoverable release blocker.
  P2 = concrete correctness, recovery, compatibility, parity, or
       diagnosability defect.
  P3 = bounded robustness, coverage, maintainability, or documentation defect
       with a concrete consequence.
- Verdict is GO or NO-GO. NO-GO while any P0, P1, or P2 is open.
- Each finding: ONE root cause, exact location, violated invariant, credible
  reproduction or state/interleaving sequence, impact, required correction,
  and a closure/regression test. Separate independent root causes.
- Style preferences and speculative unease are not defects. Do not pad.
  Interface shape is not style: wrong command placement, a misleading name,
  a missing half of a lifecycle pair, or an option without a default is a
  finding (P2 or P3), on every axis.
- If your verdict is NO-GO but every blocking finding has a bounded,
  text-or-code-fixable remedy, you may pre-commit: "I pre-commit to GO on a
  revision that resolves {IDs} as specified." This makes the re-verdict cheap
  and is encouraged when honest.

Use this report template (fill fields):

# {OBJECT} — {AXIS}-AXIS REVIEW

**Review object:** {object at exact SHA / doc path + status + date}
**Baseline:** {per-repo SHAs; note how sources were read, e.g. `git show HEAD:`}
**Date:** {date}
**Axis:** {one line: mandate}. Independent, adversarial, read-only. The other
axis runs in parallel; nothing here relies on it. Filed verbatim by the lane
owner.

**Verdict: {GO | NO-GO}** — {counts, e.g. "two P1 and three P2 findings
block"}. {If NO-GO and honest: pre-commit-to-GO clause naming the finding IDs.}

---

## 0. Evidence base
{What was actually read/run: files with line ranges, documents with sections,
commands with results. This section is what makes the verdict auditable.}

## 1. Findings
### [P1-1] {one-line root-cause title}
{Location · violated invariant · reproduction or state sequence · impact ·
remedy · closure test.}
{… one subsection per finding, severity-ordered. Omit section if none.}

## 2. Invariant analysis
{The invariants attacked and the evidence they held — attacks that FAILED are
part of the result; they are what a GO rests on.}

## 3. Risks and next action
{Residual risks below the finding bar; the single next action this verdict
implies.}


IMPLEMENTATION GATE — exact committed source, no inherited dirt
Read-only source builds MUST use the owner-created byte-exact fixture at /var/folders/02/bn9c9g2x5qj8bb42zb857p7c0000gn/T/glade-q4-exact-source.ok310g7t, never claim acceptance from the live dirty clone's build. Its source packages were exported from pinned Git commits, not edited/generated. No Git/GWZ metadata exists in this build fixture. It is NOT a workspace local clone. Root controlling docs are read via git show at aafb14a663fe130db5ef56cb002b2bb9b9399b11 in /Volumes/projects/limbo/glade-wz-raft-production.
Additional source tuple entries in that clone (verify at start/end): grazelc839fe87c9d18ebb6e995964d2e79aef7cbd380e; glade-gyld327d62c0033db0fae145d002a00663a3826b6d53; glade-gwz35b38ba0845a7cb7034a4af3609975ea1bd48741; glade-decl-rsb85044e1f6631114dbb290c02298e644f8363055. Use read-only git -C <member> rev-parse HEAD and git show for these entries as well. Supporting application/declaration members are unchanged, only fixture pins.
Before tests verify at least all 3 scoped source files in the fixture byte-exact against git -C /Volumes/projects/limbo/glade-wz-raft-production/glade show c8c0613f645dd4b6aaf546f586d77cfdb76a0c87:<path>. All 588 tracked objects across six exported members were owner-verified after the gate. Scoped actual implementation: node/src/store.rs, node/src/store/legacy_seal.rs, node/tests/legacy_store_seal.rs.
Prior contract Consistency/Safety GO and ClosureConsistency are legitimate controlling gates; this is a NEW independent Code/State review, not their private-context continuation. Keep current implementation peer prompt/report blind. All promised LS-001..007 implementation cases are now expected GREEN. Attack actual OS locks, marker lifecycle, complete pre-repair/pre-proof mutation boundaries, callback/closure guard ownership, real SIGKILL vs modeled faults, truthful unavailable/non-Unix handling. No production activation, Raft carrier selection, authenticated profile or RA-012 closure is being accepted.
Allowed test commands, cwd the clean fixture above: CARGO_TARGET_DIR=/Volumes/projects/limbo/glade-wz-raft-production/glade/node/target cargo test --locked --offline --manifest-path glade/node/Cargo.toml --test legacy_store_seal; same target cargo test --locked --offline --manifest-path glade/node/Cargo.toml --lib store::legacy_seal::tests -- --nocapture; same target --lib store::; same target sh glade/node/check.sh. cargo fmt -- --check allowed source-read only; process-global script and existing raw-AST audit may run as explicitly recorded in evidence. Build artifacts outside source only. No new fixtures/source writes. Source/diff/metadata/actual dependency-code inspection remains permitted.
IMPORTANT COMMAND COORDINATION: existing unmodified Store unit tests use globally fixed temp labels. Do not run Store units or full node/check.sh until owner explicitly grants your test window. Read-only inspection and the PID-scoped targeted legacy_store_seal and legacy_seal boundary tests may proceed independently. Your final report must state what you independently ran, and distinguish owner's full-gate result if you do not run it. Owner will serialize full gates if you request one. This scheduling is only to prevent unrelated fixture interference, never to shape a verdict. Root/members remain pinned while reviewers inspect.
Final source status: full exact-source node gate all9PASS, 507 tests per root across20binaries, 4 actual parent SIGKILL boundaries, no policy baseline relaxed. This is owner evidence to attack, not a substitute for your independent judgment. New code fmt and process globals clean; historical ratcheted style debt remains. Non-Unix source branches parsed without cfg evaluation; actual non-Unix execution explicitly not qualified. Return full standalone report about1300 words, exact tuple, command outputs, credible findings and binary verdict. No current peer testimony.
