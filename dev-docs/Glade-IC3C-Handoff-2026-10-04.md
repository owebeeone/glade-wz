# Glade IC-3C handoff

Date: 2026-10-04. Status: **frozen, locally committed UNREVIEWED IC-3C
candidate. IC-3C is not independently accepted.**

## Handoff point and owner direction

The owner requested a handoff to another LLM. The selected point is the frozen,
**unreviewed IC-3C candidate, before final aggregate review**. The finishing
narrow checks have completed, source and artifacts are quiescent, and the exact
evidence and remaining gaps are filed. No final reviewer was dispatched. Do not
start extra implementation or review rounds merely to make the handoff look complete.

Independent CRDT admission precedes Raft production integration. The owner wants
authorized replicas to edit while disconnected and reconcile later; resource
consistency profiles distinguish that behavior from exclusive authority. The
current witness uses the released text profile. General CRDT capability is not
limited to text, but other payload profiles are not implemented by this slice.

The workspace is `/Volumes/projects/limbo/glade-wz`; `~/limbo` is a compatibility
symlink. Do not default into the unrelated `gyld-wz` Python case-study repository.
App-owned Gyld architecture sources are in `/Volumes/projects/limbo/gyld-wz/gyld`.

No live node, browser, desk, launcher, key, store, trusted floor, migration, seal,
Raft activation or push is part of this candidate. Owner exclusions: ignore the
BOM-specific tangent and do not change the old handoff's date. The Gyld execution
budget is five seconds, not two. Previously moved historical references remain
untouched.

## Read first

1. Root/member `AGENTS.md` and `AGENTS_GWZ.md`; then
   [build entry](GladeBuildEntry.md),
   [library/testing policy](LibraryBoundaryAndTestingPolicy.md), and
   [package architecture](GladePackageArchitecture.md).
2. [IC-3 design](GladeIndependentCrdtProductionIntegrationDesign.md),
   [implementation plan](GladeIndependentCrdtProductionIntegrationPlan.md), and
   [review ledger](GladeIndependentCrdtProductionIntegration-ReviewCycle.md).
   These govern scope and acceptance; old program status documents are not the
   current state authority.
3. [Actual node usage](GladeIndependentCrdtProductionIntegrationNodeUsage.md),
   [typed contract](GladeIndependentCrdtProductionIntegrationTypedContract.md),
   [typed usage](GladeIndependentCrdtProductionIntegrationTypedUsage.md),
   [authentication](GladeIndependentCrdtProductionIntegrationAuthentication.md),
   [persistence](GladeIndependentCrdtProductionIntegrationPersistence.md), and
   [persistence usage](GladeIndependentCrdtProductionIntegrationPersistenceUsage.md).
4. The final C evidence/source-pin/run-log artifacts listed at the frozen
   checkpoint below. Follow their requirement-to-test map rather than
   reconstructing every earlier review.

Use GWZ for workspace status/staging/commits; never hand-edit `gwz.conf/` or
managed `AGENTS_GWZ.md`. TDD requires compiling behavioral RED, not compiler
errors or zero-selected tests. New/modified control bodies must be braced;
conditional Rust declarations need explicit enclosing modules/`cfg_if!` scopes.
Keep process-global allowlists and library classifications intact.

## Frozen checkpoint

| Repository / checkpoint | Actual revision |
| --- | --- |
| Glade C implementation | `1be9be62bbd743b9fe30cca529204f9edcdbd822` |
| Root evidence / plan / ledger / guide checkpoint | `2e1722bb2ad9f7f9caeab91a94069ba9f3d59638` |

This handoff is committed in a subsequent root documentation-only checkpoint;
its final HEAD can be obtained with `git rev-parse HEAD`. The named evidence
checkpoint deliberately precedes that commit, avoiding a self-referential hash.
GWZ owns the updated member lock. Source, evidence and handoff are locally
committed; nothing from this finishing step was pushed. All workspace members
are clean; the root is clean after the handoff-only commit. Recheck `gwz status
--json` at entry in case later work has changed that state.

Artifact SHA256s below identify the final parent-stamped candidate, with
`implementation_revision` set and `accepted_review_record` still null:

| Artifact | SHA256 |
| --- | --- |
| [C evidence](history/GladeIndependentCrdtProductionIntegrationC-Evidence.md) | `63820d1222655324caeeb2c3320140b8615f3759e865727d316408cd345d14b7` |
| [C source pins](history/GladeIndependentCrdtProductionIntegrationC-SourcePins.json) | `732d31fb09d3ec0ea8b8e59e163f5d564323b2b5ba4475fc23bc5309435fdb23` |
| [Compressed chronology](history/GladeIndependentCrdtProductionIntegrationC-RunLog.md.gz) | `4c1d9ea0820891f4bcc87d79391790f51c7d78de9dbb44fcb8362eda0cc48d1b` |
| [Node usage](GladeIndependentCrdtProductionIntegrationNodeUsage.md) | `0f8ba10adc632ef4ff10cb82f573a149e67b8d5c88a68e119afb4d4566417c01` |

The gzip expands to 1,444,946 bytes, SHA256
`6d78ba0455fc5bac644850f66399c10f081b61f1ed0551cbbb80ecbd74b509b5`.
It preserves the failed attempts as well as final passing checks. The parent
independently verified all 83 combined source pins against committed and working
bytes, all 21 C changes, 361 immutable prior objects, six consumer guides, 27
canonical representation files and compressed/decompressed chronology. These
pin checks are provenance checks, not independent implementation review.


Protected repositories, unchanged throughout C so far:

| Repository | HEAD |
| --- | --- |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External app-owned Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

C began at root `06675f8eac98f54aaf8ad71d4a568d3cd48dbfd7` and Glade
`3bfdea5921d0e500476379557dc79813ca568c07`. Use that Glade commit as the main
C diff baseline. Original deferred A2 counterexamples remain in scope separately.

## What is already accepted

**B authentication and physical persistence are accepted**, for bounded Unix
LocalProcessRestart only. The reviewed tuple was root
`f0b1c325f9e0eff27b4eefadac8084bff3505e73` / Glade
`a47691598df648eb8c9554b27f3d06b0cffcf596`; the actual acceptance filing is root
`434df3f4aae0cfc116072db95a294283978d0c6e`.
[B acceptance](history/GladeIndependentCrdtProductionIntegrationPhysicalHost-Acceptance.md)
records all seven GO reports and their hashes. The full Code report has a factual
Fence-label erratum; its original testimony is preserved.

The committed strict pre-remote prerequisite is GREEN:
[evidence](history/GladeIndependentCrdtProductionIntegrationPreRemoteQualificationEvidence.md).
It ran the real accepted B gate, all six negative-gate tests, four Rust vector
tests, and independent Rust/TypeScript/Python canonical consumers/negatives.
The two negative-fixture corrections were test-only and remain aggregate scope.
They did not change signing, canonical bytes or production gate semantics.

Reuse these exact accepted pins where source and assumptions are unchanged.
Changed C paths and dependencies still need fresh inspection. New/changed remote
representations would need genuine committed three-language qualification before
use. C uses the existing qualified SignedRecord, ExchangeHello, Inventory,
InventoryPage and TransferBundle representations.

## Current C implementation and executed milestones

The final artifact matrix is authoritative for counts/results. Executed milestones
at this checkpoint include:

- Actual `Settings`/`NodeStart`/`node_plan`/`NodeAssembly` opt-in via
  `--independent-config`; recovery and selected remote inbox replay precede
  Listening. Absence keeps legacy defaults. One retained runtime uses real
  evidence, Records and Iroh; no successful alternate model or caller Facts.
- Genuine fresh local challenge/response and append-only writer possession;
  durable guard/raw-intent custody before native admission; original sealed
  receipt returned after lost reply and actual SIGKILL/reopen/exact retry.
- Real TLS/exporter-bound signed HELLO and mutual ready/busy receive ownership
  before history. Both nodes initiate content-triggered work. Busy cannot
  preempt a started receive; inbound completion cannot retire another outbound
  worker. A later edit only at the former acceptor is delivered automatically.
- Immutable full manifests/pages, exact bundle requests, original accepted
  receipts/admissions, candidate/security/fork/rival custody and native
  classification. No eligible-only/head-only substitute or client ferry.
- Two real `env_clear` binaries: disconnected local writes, kill/reopen,
  autonomous reconnect, ordinary/fork history convergence, and released
  Glial/Taut projection of the validated eligible subset in both arrival orders.
- Real ancestry/security exchange in both startup orders; missing ancestor
  initially keeps the kernel cut incomplete and later arrival enables promotion.
  The original candidate identity remains accounted for.
- Foreign/revoked/expired writers, wrong node/tuple and missing hold/admit
  authority refuse. Unauthorized TLS holders receive no history. Capacity
  retains intent without a fabricated receipt. Corrupt X refuses while
  independently owned Y progresses.
- Actual receive SIGKILL retains an orphan Active guard and honest
  incompleteness. Settled remote inbox entries replay before Listening with the
  peer absent; old local intent still requires fresh possession.
- Compiling actual-consumer mutants reject domain bypass, missing selected
  custody, head-only history and forced legacy routing. A further mutant rejects
  one-sided cache suppression after successful synchronization. Source is
  restored before the final positive checks.

Important distinctions: native kernel completeness alone is insufficient.
An idle/settled network can coexist with a known missing ancestor and
`complete=false`; ordinary final heal still requires a complete cut. Guard/loss,
missing inbox obligations or unresolved native work cannot be called settled.
Never reset sticky kernel incompleteness, orphan guards or permanent loss.

## Evidence honesty and remaining work

Final review has **not** been dispatched. C/aggregate acceptance, original typed
finding closures and IC4 activation remain open.

The C evidence's **Directed interruption phase coverage — remaining work** table
is the exact gap list. Directed actual-peer disconnect qualification is missing
for HELLO/confirmation/readiness, full inventory header and page, missing-request
header and page, and peer bundle receive. Successful execution of those phases,
physical B floor crash cuts and generic Iroh faults do not discharge that matrix.
The selected-remote-inbox pre-dispatch OS termination/restart witness is already
qualified; a separate transport-coupled dispatch interruption is not claimed.
Exhaustive interruption at every byte is not required. Complete the required
phase witnesses without waiving them or relabeling unrelated tests.

Raft is not integrated into the production path. IC4 client/live activation is
also pending; neither should be the first task after this handoff.

Preserve chronology and baseline debt. Whole-node formatting is not clean:
the measured 251 existing hunks remain under the unchanged 252 ratchet. Clippy
has nine unchanged warnings; it is not warning-free. Gyld's six old Mypy errors
are not GREEN merely because the affected execution budget passes. Cold build
or token measurements not actually recorded must remain unmeasured.

Two misleading attempts were explicitly corrected: HELLO_DOMAIN already aliases
the accepted inventory domain, so no domain/codec repair was needed. A suspected
ancestry history defect was not established: a new fixture omitted the actual
route's classification step and ran a stale binary. Rebuilding and using that
same production route passed without the hypothesized extra history patch.
Compiler failures, abandoned proposals and fixture synchronization mistakes
must not be relabeled implementation REDs.

## Review trial and exact next step

The owner authorized a **Glade-only efficiency trial** in the plan/ledger, not
a shared-skill change. Prepare one compact frozen evidence packet. Two fresh,
peer-blind technical reviewers cover **Code+Consistency** and **State+Safety**;
a separate cold **Surface** reviewer reads consumer docs/help only. All five
explicit perspective verdicts are required. Record actual dispatches, distinct
blocking roots, justified repeated checks and available elapsed/token measures.
Do not claim measured savings in advance.

The original A2 Code/State/Surface finding owners must verify their own deferred
counterexamples; writer GREEN and fresh aggregate GO cannot substitute for those
closures. Their contexts are `/root/ic3_typed_code`, `/root/ic3_typed_state`,
`/root/ic3_typed_surface` in this chat. If moving to another LLM loses access,
surface that limitation; do not fabricate originator verification.

Historical accounting remains semantic **2 architectural/1 nonarchitectural/
1 completed remediation**, typed **2 architectural/5 nonarchitectural/
2 completed remediations**, B **0 architectural/6 nonarchitectural/
2 completed remediations**. Preserve caps and STOP rules; a new filename is not
a reset. Do not make P3-only findings into new packages.

Next LLM: verify the actual frozen tuple and clean state; inspect explicit
coverage gaps first; finish any necessary scoped TDD correction before freezing
the review tuple; then run the consolidated review and original closures using
`/Users/owebeeone/.claude/skills/review-loop/SKILL.md` and its canonical prompt
template. Source stays fixed until all required START/END inspections finish.

Preparation helpers exist under `/tmp`: `glade-ic3-review-prompts.py`,
`glade-independent-crdt-file-report.py`, `glade-ic3-C-review-config.json`,
`glade-ic3-C-surface-review-config.json`, typed-closure configs and a compact
packet draft. They have not generated/dispatched the final C reviews. They are
conveniences, not authority; `/tmp` may disappear. Recreate from the canonical
skill/template, exact pins and this packet if needed. Do not assume their final
artifact paths or tuple have already been filled.
