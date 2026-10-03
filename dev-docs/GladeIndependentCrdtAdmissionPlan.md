# Glade independent CRDT admission — near-term delivery plan

Date: 2026-10-04. Status: **owner authorized design, review and gated implementation;
design and implementation not yet accepted**. After the sequencing recommendation,
the owner directed a dedicated design, review and implementation, then said go.
The owner wants separate Glade nodes
to accept CRDT edits during disconnection and reconcile after reconnection.
[Resource consistency requirements](GladeResourceConsistencyProfiles.md),
GDL-054/055, establish the general capability direction; text and preferences
are examples, not its product limits.
The [dedicated design](GladeIndependentCrdtAdmissionDesign.md) and
[review/delivery ledger](GladeIndependentCrdtAdmission-ReviewCycle.md) control
the semantic, contract/RED, component and live-integration checkpoints.

## Recommendation and timeline

Prioritize this bounded path **before the first Raft production integration**.
Waiting until after Raft would leave the requested partition availability
unavailable and risk integrating mergeable resources through exclusive admission.
Raft remains the strong-profile direction; its carrier qualification MAY continue
independently. Independent CRDT admission MUST NOT require a Raft write quorum or
exclusive holder round trip for each local edit to an already authorized binding.
Bootstrap/governance evidence remains a separate requirement, not supplied by CRDT.

| Order | Milestone | Concrete completion evidence |
| --- | --- | --- |
| IC-1 — next specification tranche | Exact independent-admission profile, canonical amendments and Gyld responsibility allocation. | Declare binding identity/profile, authenticated writer/origin, disconnected authorization, local receipt/read guarantees, causal/gap recovery, retry, bounds and migration. Compiling behavioral RED consumers plus applicable Consistency/Safety review; Surface review before a user-facing freeze. |
| IC-2 — first implementation/proof tranche | Deterministic admission and reconciliation proof over the existing supported text CRDT profile. | Both isolated replicas accept valid local edits without consulting an exclusive holder; delivery resumes and valid operation sets/text converge. Requirement-linked success, failure and edge tests pass, including a rejecting mutant. This is a proof, not production adapter qualification. |
| IC-3 — real-node integration | Two actual Glade nodes retain their own admitted operations and exchange missing app operations in both directions. | Real scoped authentication, qualified local persistence, restart/exact retry, connection loss/reconnect and honest receipts. Existing Taut/Glial text projection is reused; a fixture signature/storage promise MUST NOT stand in for a real adapter. |
| IC-4 — compatibility and bounded activation readiness | Affected clients/Glial consumers, profile negotiation and explicit legacy transition. | Rust/TS and affected consumer conformance, I/O/fault evidence, separate-instance isolation, legacy/mixed-version refusal and rollback fences on one reviewed tuple. Activation remains a separate reviewed step; no running stores are changed by this plan. |
| Then Q4-D/E | Strong-profile Raft provider integration and its migration/activation. | Preserve all Q4 gates and RA-001–012. Select the first strong consumer separately; appearance is no longer assumed to require exclusive Raft admission. |

Q4-B carrier qualification can overlap IC-1–4; Q4-C's identity/authority/profile
contracts MUST be reconciled with IC-1 so they do not become incompatible admission
systems. This table proposes milestone order, not calendar estimates or a claim
that those milestones are finished. Near-term priority is owner-requested; the
specific sequence was subsequently authorized as the design/review/implementation
lane under GDL-057. No runtime activation follows from that authorization.

## Scope and canonical boundary

The first proof uses two authorized replicas of a known authenticated canonical
binding and the existing supported text profile. It MUST also prove isolation
between two distinct instances using that profile. This bounds the first witness,
not the general engine or dynamic-resource objective. Preference maps/registers,
additional payload profiles and general offline creation are separate contract
work; the initial witness MUST NOT imply those capabilities exist.

The current admission path is `glade/node/src/accept.rs::placement`: a client
write follows the share route to its claim holder. The Store accepts several
CRDT writer origins; Glial/Taut own the text merge. Independent admission requires
changing the admission/synchronization contract, not inventing another text merge.
See [CRDT adapter](../glade/dev-docs/GladeCrdtAdapter.md) GCA-01–09 and
[multiwriter evaluation](GladeMultiwriterSettingsEvaluation.md) §§2/4–7.

IC-1 MUST explicitly reconcile Substrate §6 W1–W5/W7 and
[cross-node writes](../glade/dev-docs/GladeCrossNodeWritesPlan.md) §2 option B,
§3 and §7. A known binding's declaration/profile MUST govern admission and shape;
the first locally observed operation or visible peer count MUST NOT establish it.
Legacy and strong bindings MUST retain their existing admission contracts until
their own qualified transition. No implicit timeout-based profile conversion.

Replica convergence MUST use the same valid operation set and declared profile.
The design MUST specify policy eligibility and revocation behavior during a
partition, rather than treating syntactic CRDT convergence as authorization.
Missing dependencies and origin forks need bounded buffering/quarantine and
consistent eligibility; a peer MUST NOT silently drop a locally acknowledged
operation and report complete convergence. Local acceptance does not imply an
independent replica copy; permanent loss of the only copy remains possible under
that receipt. Exact receipt strength and acceptable-loss policy remain open.

## Required failure journeys

The IC contract/test matrix MUST include:

- One replica; two connected replicas; both disconnected and editing; either
  replica permanently lost, with no consensus timeout needed for the survivor's
  authorized local edits.
- Concurrent edits, causal delivery permutations, duplicates, missing ancestors,
  fork evidence and reconnect without a client manually ferrying the history.
- Lost acknowledgement, exact retry, restart, restored-origin custody and storage
  failure without an invented success or stronger durability claim.
- Wrong principal/scope/profile, unknown governance, disconnected revocation and
  capacity exhaustion with explicit declared outcomes.
- Two separate instances with overlapping writer names and distinct policies;
  history and rights MUST NOT leak between them.
- Legacy/new-profile coexistence, unsupported profiles, downgrade and delayed old
  operations; pending/unknown histories MUST NOT be discarded during migration.

Use the applicable MW-004–007/010–012 obligations and GCA text corpora; MW-001's
appearance-only scope and preference-specific field/reset tests are not a general
CRDT admission definition. Preserve those for a later preference-profile tranche.
Before implementation, IC-1 MUST give these journeys stable test IDs and exact
expected outcomes; the bullet list is not executed test evidence.

Allocation follows [BuildEntry](GladeBuildEntry.md),
[library policy](LibraryBoundaryAndTestingPolicy.md) and
[package architecture](GladePackageArchitecture.md). No crate/dependency or
allowlist change is selected here. Implement via TDD after the contract gates;
keep pure admission/profile logic separate from injected storage/authentication/
transport adapters, run affected-consumer and adopted architecture checks, and
measure the bounded fast loop. Existing no-process-globals and explicit-boundary
rules apply.

The Q4 legacy Store seal is whole-store retirement preparation. Before either
profile activates, reconcile its writer inventory and store layout with the
independent CRDT path; installing a seal MUST NOT be treated as a per-binding
switch. Raft migration and CRDT transition cannot silently erase or reinterpret
each other's histories. No seal, launcher or production enrollment is changed.
