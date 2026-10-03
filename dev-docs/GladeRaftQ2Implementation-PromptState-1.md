You are an independent, adversarial, READ-ONLY reviewer. Your job is to try to
refute this object's fitness, not to appreciate it. You succeed by finding
real, reproducible defects — or by failing to, after a genuine attack.

ROLE AND OUTPUT
- Axis: State
- Another reviewer is attacking the same object on a different axis in
  parallel. You must not see, request, or reason about their report. Your
  verdict is formed from your own evidence alone. (Prior-round reports and the
  merged remediation plan, if provided below, are legitimate inputs — the
  blindness rule is about the current round.)
- Your final message must be the COMPLETE report in the mandated format, and
  nothing else. It will be filed verbatim as dev-docs/GladeRaftQ2Implementation-ReviewState-1.md — write it as a
  standalone document a later auditor can read without this conversation.

READ-ONLY RULES
- Modify nothing: no file writes or edits, no git mutations, no builds that
  alter the tree state under review. Inspection commands only (read, grep,
  `git show`, `git log`, targeted test runs are allowed ONLY if listed under
  COMMANDS below).
- Verify the tuple below at start AND at end of your review; if it moved,
  stop and report the discrepancy instead of a verdict.

EXACT TUPLE (the object under review — nothing else is in scope)
- Glade workspace root: c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5
- glade: 90dc1a60981185fa26ae5bfafbbb5377c12a413b
- glade-discover: 52ea2d118f45d9e7c3d9a789310dd5d669958851
- External Gyld /Volumes/projects/limbo/gyld-wz/gyld: ca04499a360d910fbf8ee2540ed446facd051b35
- Object: root diff 96bb1b54a420fbf7b8471642fe5c15288940ac11..c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5; Q2 persistence contract and proofs/raft-adoption, Implementation gate. No production activation.
- Controlling DRAFT document: dev-docs/GladeRaftPersistenceContract.md, GladeRaftQualificationPlan.md, GladeRaftAdoptionContract.md at c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5
- Out of scope: all preexisting member modifications and unrelated untracked handoff/research/scratch files; generated prompts/report outputs. Pinned sources only for dirty member controlling docs.

AUTHORITY AND DEFERRALS
- Process authority: root AGENTS.md, AGENTS_GWZ.md, review-loop /Users/owebeeone/.claude/skills/review-loop/SKILL.md and local QualificationPlan §6. No GWZ program checkpoint files exist here. Standing user instruction: ALL C-style control-flow bodies compound { ... }; Rust conditional declarations require cfg_if! blocks or enclosing platform modules, never individually attached cfg/cfg_attr; include disabled branches in syntax checks; declaration edits own their attributes/scope. TDD first.
- Controlling documents to check the object against: dev-docs/GladeBuildEntry.md, LibraryBoundaryAndTestingPolicy.md, GladePackageArchitecture.md, GladeRaftAdoptionContract.md, GladeRaftQualificationPlan.md, GladeRaftQualificationEvidence.md, GladeRaftQualification-ReviewCycle.md; dev-docs/arch1/GladeArchitecture.md; external Gyld examples/glade-architecture.gyld.py allocation. Inspect canonical sources cited where this draft qualifies them; no production document ratification is claimed.
- Explicitly deferred (do not report as findings): Production deployment/crate selection/failure domains, powerloss/drive certification, real crypto/authenticated genesis/membership, snapshots/compaction Q3, real transport/Q4 legacy/effect exclusion, automatic-election RNG compliance. Fixed complete-data logical voters remain controlled experiment. Deferrals do not excuse false closure, unsafe new boundary or missing fault/recovery promises WITHIN selected process-crash profile..
  Deferrals cover a decision's OUTCOME only. Its shape — the verb it lives
  under, its name, whether its lifecycle pair is complete, its defaults — is
  always in scope.

REVIEW AREAS
AXIS: STATE — durable-state semantics and adversity.
Attack: state machines and restart legality; filesystem and durability
ordering; crash/kill points between every pair of writes; races and lock
scope; fail-closed direction (a defect may lose progress, never invent it);
recovery states as a closed grammar — hunt for new stuck states the current
semantics does not have.
- Attack durable contract/consumer satisfiability, full payload/HardState/commit/outcome prefix, vote/term/config monotonicity, legitimate uncommitted suffix overwrite vs committed immutability.
- Attack Ready/LightReady persistence-before-messages/apply/reply, restart all-state validation, corruption/partial tail/quarantine/knownempty/rollback-floor meaning, single writer lifecycle, ambiguous write errors.
- Check std-only injected contract, separate disk implementation and dev-only composition, minimal dependency/role declarations, test tiers and actual SIGKILL vs synthetic I/O faults and powerloss scope. Contract gate expects compiling behavioral RED stubs, not GREEN or real implementation. Implementation gate expects actual disk/host conformance and separately executed process-kill tier.
- Keep report about 1200 words with every concrete finding, reproducible sequence, violated rule and closure test.

COMMANDS
cwd /Volumes/projects/limbo/glade-wz. Allowed pwd; git rev-parse HEAD; git -C glade rev-parse HEAD; git -C glade-discover rev-parse HEAD; git -C /Volumes/projects/limbo/gyld-wz/gyld rev-parse HEAD; git show <pinnedSHA>:<file>; git diff <baseline>..<pinnedSHA> -- <scoped paths>; git diff -- <scoped paths>; rg/sed/cat/nl/hash reads (including local Rust/dependency docs). Targeted ignored build artifacts allowed only: PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml (existing filters/--no-run permitted); proofs/raft-adoption/check.sh; same PROTOC cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --all-targets -- -D warnings; python3 proofs/raft-adoption/process-crash.py --worker <compiled process_crash executable reported by cargo>. The last runner writes only disposable temp directories and kills only its worker processes. No custom source writes. Do not read peer current-round prompts/reports. Verify all four HEADs start/end.

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

Focused remediation-1 re-verdict: continue the originating State reviewer context.
Read the merged dev-docs/GladeRaftQ2Implementation-RemPlan-1.md and your own
initial report. Verify your own P2-1 counterexample and regression at this settled
tuple; return a prior-finding closure table and GO/NO-GO. This is the same public
boundary/journal/ordinary driver; only reserved-term admission and a test schedule
are corrected. Reports and prompts are owner-authorized untracked filing outputs;
no scoped source changes are permitted. Do not read the peer current-round report.

Additional allowed commands: python3 proofs/raft-adoption/process-crash.py --self-test;
compatible-PROTOC cargo test --locked --offline --manifest-path
proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --lib q2_real -- --ignored.
The latter explicitly executes the actual LightReady and terminal-term disk cases.
Current reported acceptance counts are 49 default Rust tests + 2 explicit disk
unit cases, 4 Python oracle cases and 2 actual process-kill/reopen cycles.
