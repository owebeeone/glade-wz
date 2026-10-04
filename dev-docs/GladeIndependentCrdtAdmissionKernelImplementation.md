# Independent CRDT admission — IC-2 kernel implementation

Date: 2026-10-04. Status: **DRAFT implementation checkpoint; not accepted**.
The owner directed “go next” after the redesigned internal storage-attempt
contract received Code/State GO. This is the next Pure component object, not a
continuation that resets either historical contract object's review cap.

## Authority and scope

The [semantic design](GladeIndependentCrdtAdmissionDesign.md),
[storage-attempt lifecycle](GladeIndependentCrdtStorageAttemptDesign.md) and
[accepted internal contract](GladeIndependentCrdtStorageAttemptContract.md)
control. The contract's exact supersession table determines the retained clauses
of the historical [IC-1 contract](GladeIndependentCrdtAdmissionContract.md).
The [delivery plan](GladeIndependentCrdtAdmissionPlan.md) and
[review ledger](GladeIndependentCrdtAdmission-ReviewCycle.md) remain controlling.

Starting inspection tuple: root `81ba6b205195ee090a166c932b20f237d9dfef40`,
Glade `52fcbe5043d8178a917677d6c9461d771d3543e4`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`. The accepted contract was reviewed
at root `fac445d74025f00c3d59574b2bbea1ebe14c665a` with the same member tuple.
These are baseline pins; a new exact committed tuple MUST precede review.

Only `glade/contracts/crdt-admission-core` implementation and directly relevant
new tests are selected. The existing storage API, test producer/assembly,
released Glial/Taut consumer, canonical corpus and Gyld allocation MUST be
preserved. One obsolete scaffold-only test has the confined replacement below;
the 43 domain assertions/loops and producer remain fixed. No new library,
dependency, public interface, classification,
allowlist or test-budget relaxation is authorized by this implementation object.
An unsatisfiable contract MUST be reported with a counterexample before changing
that boundary. No live nodes, stores, launchers or enrollment are selected.

## Implementation obligations

1. Implement explicit deterministic `step(&State, Event) -> Transition` without
   I/O, ambient time, process globals, a holder inquiry or a new CRDT merge engine.
   Local admission, historical qualification, custody and projection eligibility
   MUST retain their separate accepted predicates.
2. Encode every staged descriptor, candidate, evidence, fact, seal, fork pair,
   receipt and continuation into immutable versioned batch bytes. Encoding MUST
   be deterministic and unambiguous; a changed semantic field MUST change bytes.
   This internal encoder does not freeze a public container or qualify a disk
   decoder, signature domain or restart adapter.
3. Authenticate each callback against its complete issued continuation. Retain
   consumed storage requests and terminal evidence within finite declared bounds.
   Pending/refused replies MUST preserve uncertainty; only qualified terminal
   NonCommit releases an unresolved plan. Committed installation MUST account
   once and preserve the original receipts for all four batch kinds.
4. Preserve bounded causal closure, security-only rivals versus qualified forks,
   common-prefix heads, deterministic eligible sets and instance isolation.
   Fresh certified E1 edits after an E0 fork MUST remain possible. Exact old
   retries MUST keep original receipts; eligibility changes MUST NOT erase custody.
5. Preserve independent quotas, checked counters, per-instance unresolved-work
   exclusion, sticky incomplete recovery and integrity stops. Capacity refusal
   MUST NOT falsely advance a frontier, erase reservations or upgrade durability.

The sole source drafter SHOULD organize cohesive encoder, semantic/projection
and storage-continuation modules. The split-files skill's cohesion guidance is
applied during new implementation, without an unrelated repository refactor.
All changed code MUST follow explicit control-flow and conditional boundaries.

## TDD and evidence

The original 43 compiled behavior failures MUST be rerun before implementation:
22 records/host, six recovery and 15 storage-attempt tests. The actual released
text consumer's exact ten RED rows and unchanged corpus control MUST also be
recorded. Existing assertions, fixtures and port calls MUST NOT be weakened to
make them pass. New encoder, edge and regression tests MUST run RED first.

The first all-targets implementation run exposed
`representation.rs::refusing_scaffold_preserves_state_and_emits_no_side_effects`:
it requires valid SubmitLocal, OfferReplica and Read to preserve State and return
only Unavailable. That assertion explicitly describes the former placeholder;
it is incompatible with the owner-selected successful Pure kernel. The lane
owner authorizes replacing only that obsolete test with deterministic transition,
caller-input immutability and nonmutating actual Read checks. The separate
operation byte/digest ownership test and all original 43 domain assertions/loops
MUST remain unchanged. The original test stays auditable at the accepted baseline.
This is a scaffold-to-implementation assertion replacement, not a relaxed domain
contract, new test selector or dependency/classification/allowlist exception.

Completion evidence MUST include:

- All owning-package targets and affected storage API targets passing, including
  the retained fixture/source and byte-ownership checks, the replacement
  deterministic-step representation checks and 34 API checks.
- Exact ten released-text rows and the unchanged canonical corpus passing through
  kernel-returned eligible operations, including isolated ABC convergence,
  missing ancestry, AD security-only rivals, common A forks and fresh E1 AB.
- A meaningful rejecting mutant whose concrete faulty behavior fails an actual
  consumer assertion. Mutants MUST NOT introduce a successful alternative model.
- Focused formatting, clippy, adopted architecture, process-global, source and
  whitespace checks. No whole-workspace test substitute or loosened gate.
- Measured focused-loop evidence distinguishing execution from compilation;
  cached dependencies and scheduling limitations MUST be stated honestly.

The bounded memory producer provides deterministic contract evidence only.
GREEN here MUST NOT be described as real cryptography, physical durability,
asynchronous cancellation/drain, automatic network exchange or live qualification.
Concrete executions will be filed in the implementation evidence document.

## Explicit frontier-completeness review question

Inspection found that ObserveFrontier supplies an expected digest, but retained
`Slot` contains only instance/origin/sequence. If an unknown advertised digest is
forgotten and a different qualifying operation later arrives at that slot,
clearing pending would falsely claim the announced cut is complete. No existing
qualified-head, candidate or callback namespace can truthfully stand in for that
missing expected-digest continuation.

The confined implementation uses the existing sticky recovery-incomplete field
when an advertised digest is not already retained, while retaining bounded actual
missing slots and requesting history. An already-retained exact advertised digest
does not cause that stop. Later arrival cannot clear the sticky field because
the accepted grammar has no clearing witness. This conservatively prevents false
completeness but means a formerly unknown digest inventory cannot become complete
under this grammar. It does not prevent valid independent local admission or
custody/projection convergence.

Code/State reviewers MUST explicitly determine whether that conservative
limitation satisfies the accepted Pure contract or requires a typed amendment
before acceptance. This DRAFT does not self-ratify a weaker completeness promise
or select permanent live synchronization behavior. A later live producer cannot
claim successful exact-cut reconstruction from this fixture behavior.

## Review and remaining gates

The recorded Pure component tier is **two fresh, peer-blind Code/State reviewers**
on one exact committed root/member/external-Gyld tuple. Prompts MUST be generated
from the review-loop canonical template; reports MUST be filed verbatim. The
lane owner merges findings, tracks architectural roots and applies the bounded
remediation cap without self-closing reviewer findings. User-facing Surface,
real adapter/aggregate and activation gates remain separately required.

After Pure acceptance, IC-3/4 still require actual scoped authentication,
qualified local persistence/restart/exact retry, automatic duplex application
exchange, affected clients and compatibility/fault/readiness evidence. This
object authorizes local settled checkpoints only: no push, desk rebuild, seal,
enrollment, migration or activation.
