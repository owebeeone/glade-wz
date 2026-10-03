# Glade Raft production integration — Q4

Date: 2026-10-03. Status: **implementation authorized; production activation not accepted**.
Q3 source is `468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`; acceptance filing is
`55b6ee30bb18fa58c80805be2d29a44b404077f6`. Raft is the algorithm direction.
Neither production library nor deployment/authority/migration profile is selected.
Q4-A's internal Store retirement interlock is implemented and reviewed. The
[Q4-B0 private contract/scaffold](GladeRaftB0Acceptance.md) is accepted at root
`8553bc71b1f6bc9fe203729e61567e8b9bd37be2`; actual carrier adaptations and
automatic-election qualification remain open. The reviewed
[production-profile packet](GladeRaftProductionProfileDecisions.md) awaits owner
selection before Q4-C canonical amendment work. These preparation gates do not
activate a Raft provider in Glade.
Subsequent GDL-054 requires partition-available, mergeable shared preferences;
[resource consistency profiles](GladeResourceConsistencyProfiles.md) captures the
owner direction. Reconsider appearance as the first strong-profile consumer
before Q4-C. This requirement does not relax Raft quorum rules or select a
multiwriter wire/receipt profile; the addendum is not yet reviewed.
This plan extends [qualification](GladeRaftQualificationPlan.md), preserving
[RA-001–012](GladeRaftAdoptionContract.md). It MUST NOT turn private Q3 fixture
identities, receipts, storage encodings or trust decisions into production contracts.

## Source and scope

The source root was cloned verbatim with `gwz local clone raft-production` into
`/Volumes/projects/limbo/glade-wz-raft-production`. Baseline member Glade is
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`, discover is
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld is
`ca04499a360d910fbf8ee2540ed446facd051b35`. Existing cold-join edits to claims,
lifecycle, CLI, cross-node tests and their two specifications, discovery edits,
untracked handoff/research and scratch scripts MUST remain outside this slice.
The future date on the handoff MUST remain unchanged. No push, desktop rebuild,
cutover, start-script change or production enrollment is performed by Q4-A.

Use the existing Gyld declaration before allocating new production ports:
[GladeArchitecture](</Volumes/projects/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py>),
Records/StorageAdapter and NodeAssembly, explained in [arch1](arch1/GladeArchitecture.md).
Read [build entry](GladeBuildEntry.md), [library policy](LibraryBoundaryAndTestingPolicy.md)
and [package architecture](GladePackageArchitecture.md). New/recomposed ports require
reviewed allocations and compiling behavioral RED consumers before implementation.
Proof harness packages MUST NOT enter production dependency paths.

## Ordered work and gates

Near-term owner priority: independent CRDT admission during disconnection and
reconciliation after reconnect. The [IC-1–4 delivery plan](GladeIndependentCrdtAdmissionPlan.md)
recommends specifying, proving and integrating that path before Q4-D/E, with
Q4-B carrier qualification continuing independently and Q4-C contracts coordinated
with IC-1. The owner subsequently authorized this design/review/implementation
lane (GDL-057). No gate is completed or strong Raft contract changed by that
authorization. The new plan is unreviewed; activation remains separately gated.

| Slice | Concrete output | Exit gate |
| --- | --- | --- |
| Q4-A: legacy Store retirement preparation | A one-way, durable, whole-store seal; current Store handles and reopen refuse further legacy mutation under a shared filesystem lock. Existing unsealed stores keep their behavior. | [Seal contract](GladeRaftLegacyStoreSealContract.md), compiling RED, Consistency/Safety GO before implementation; Code/State GO on real I/O and affected consumers. This is preparation, not RA-012 closure. |
| Q4-B: carrier comparison | Instance-owned election entropy/time adaptations for pinned raft-rs 0.7.0 and OpenRaft 0.9.25; identical automatic-election/application/disk/fault journeys and measured inventory/build/runtime costs. | Reviewed source adaptations; both execute the same application contract; exact resolved licenses/security/platform inventory. Select only on this evidence. |
| Q4-C: canonical profile and authority | Actual canonical `(share, glade_id, key)` plus declaration/version/parameters, creation-root/incarnation, group/configuration, authenticated requester/request ID, complete canonical commands and ordered policy/evidence frontier. | Exact canonical amendments and Gyld allocation reviewed and owner-ratified; real signing/verification, authenticated private self on every hop; consumer/conformance RED and compatibility review precede implementation. |
| Q4-D: Records/provider integration | Selected carrier, actual durable storage and authenticated transport; retained accepted/refused outcomes, Create/Mutate/BeginMove/Activate/Retire, membership and complete snapshots. | Real data-bearing quorum receipts, exact retry/current disclosure, fault/unknown/restart witnesses, automatic elections, measured fast checks. Old Ok/hash/Heads retain their old meaning. |
| Q4-E: migration and activation | Reconcile actual legacy copies; verified complete baseline; exclude every old writer and downgraded binary; persistent rollback fence; named independent machines/storage domains. | No divergent/unknown history discarded; owner production profile and cut approved; fresh dual activation review; no effect-bearing sink enabled without its own qualification. |

Q4-B and Q4-C specifications MAY proceed independently of Q4-A. No slice success
waives another gate. A loopback three-node run MUST NOT count as independent
failure domains. The owner deferred a two-machine cross-node-writes run; that
ruling does not constitute completed production failure-domain evidence.

## Proposed first production profile — for review and owner selection

The owner's subsequent cardinality requirement is one, two, three or more nodes,
including initially disconnected participants and unstable links. The next
design object is [authenticated bootstrap and safe growth](GladeRaftBootstrapGrowthDesign.md),
recorded under GDL-053. Three complete-data voters below are a resilience
recommendation, not a minimum installation size. Peer discovery does not change
committed voter membership. The new design must reconcile singleton genesis,
learner admission, promotion and configuration recovery before production
profile selection; it is not implemented or ratified here.

Recommendation: one authenticated known group with **three complete data voters**,
majority durable log/application acceptance, APFS process-crash guarantees first,
one complete canonical `value` binding (appearance is the affected-consumer case).
Two complete voters require both for writes and cannot survive either loss.
This recommendation does not change the existing two-node development launcher.
Power-loss certification, new groups, membership administration, linearizable
read promises and external effects need separate explicit profiles/gates.

Initial application work MUST retain the full RA-001–012 trace even with one shape.
A narrowly scoped deployment is not permission to omit root custody, exact retry,
policy, move readiness, retirement or retained configuration/snapshot evidence.
Dynamic resource/scope creation and authenticated discovery remain the product
objective; a known initial group is an integration fixture, not preconfigured
per-resource homes or a new product restriction.

Owner decisions still required before activation: actual independent machines,
root issuance/custody and authentication strength, governed policy frontier,
linearizable read requirement, retention capacity, production library and the
specific legacy baseline/rollback cut. The recommendations above remain proposals.

## Source-audit conclusions

Both pinned carriers contain ambient randomness. raft-rs resets use
`rand::thread_rng()` even during constructor/state transitions; its hidden timeout
setter is not injection. OpenRaft's runtime is type-selected with static methods,
its constructor accepts no instance context, and instant formatting also reads
ambient wall time. A reviewed instance-owned source adaptation is necessary;
no global/thread-local workaround or allowlist waiver is authorized.

Glade's client Hello principal is development input; peer HELLO authenticates the
forwarding node, not the original requester. Existing app Op is unsigned and
lacks protected group/root/generation/request ID/evidence/result fields. Existing
Ok means locally held, not durable Raft success. Genuine node signatures alone
are not creation-rooted governance authority.

Actual Store writers include client/provider admission, holder admission,
forward reply placement, forwarded replica ingestion, public peer pull and
seed/import. Home and Registry/declaration/claim writes have additional paths;
external suppliers are separate. A central Store seal covers that store's writers
in participating builds only. It cannot prevent a binary unaware of the seal,
separate records.json writes, already cached projections, or external effects.
The full Q4-E inventory MUST include both roots, recovery, old binaries and sinks.

## Review and evidence binding

The [review-loop skill](</Users/owebeeone/.claude/skills/review-loop/SKILL.md>) controls
peer-blind gates, verbatim reports and at most two architectural remediation rounds.
Q4-A is an internal storage lifecycle API, not a user-facing wire/configuration/CLI
freeze. No user-editable seal format or operator enable/disable interface is
introduced. A later user-facing freeze additionally requires Surface review.
Use GWZ scoped staging/commits. Review exact root/member tuples; inherited dirt is
explicitly excluded. File timings, RED/GREEN, limitations and original-reviewer
closure in Q4-A evidence and the qualification ledger.
