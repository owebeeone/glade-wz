# Glade — Problem Inventory Before Architecture

Date: 2026-09-05. Status: **PROPOSED — discussion inventory, not an architecture ruling or implementation plan.**

Purpose: categorize the problems Glade must solve before choosing layers, packages, protocols, or deployment topology. The categories describe responsibilities. A single operation can cross several categories.

This inventory draws on both application-stack reviews and the subsequent comparison. New handling suggestions remain proposals. Existing requirements retain their recorded authority; implementation gaps do not reopen them. Decision references such as **G6 D04** refer specifically to [the G6 worksheet](/Users/owebeeone/limbo/glade-wz/dev-docs/ApplicationStackContractDecisions-G6.md), whose IDs differ from the first worksheet.

Follow-up: [Architecture Discussion Record](/Users/owebeeone/limbo/glade-wz/dev-docs/GladeArchitectureDiscussion.md) captures the subsequently agreed operating environment and separate trust workstream, the owner's Iroh/sharded-registry proposal, and reviewer refinements that remain unadopted. This inventory remains the problem map; the discussion record tracks answers and proposal status.

## 1. The classification

For every problem, identify **scope, responsible party, authoritative evidence, acceptable staleness, recovery obligation, and required coordination**. “Responsible party” assigns a guarantee, not a final module boundary. “Evidence” distinguishes facts that confer authority from observations that merely help locate something.

| Family | Central question | Problems |
|---|---|---|
| A. Connectivity and bootstrap | How can this participant establish and maintain contact? | GPI-01–03 |
| B. Shared knowledge and discovery | What exists, who offers it, and how complete is our knowledge? | GPI-04–06 |
| C. Identity, permission, and isolation | Who is acting, and which information or operations may they access? | GPI-07–09 |
| D. Authority and effects | Who may change this source now, and what happened to an attempted change? | GPI-10–12 |
| E. Durability and recovery | What has survived, what can be replayed, and what may be discarded? | GPI-13–15 |
| F. Source and consumer semantics | What does the delivered state mean, and which lifecycle does it represent? | GPI-16–18 |

The primary discriminator is the **consequence of stale information**. A stale address causes a failed connection; a stale advertisement can select an unavailable provider; stale authority can admit an invalid effect. A shared transport or synchronization mechanism does not make these guarantees equivalent.

## 2. Inventory

Each row combines the six fields above. Its final column is a concrete failure scenario for evaluating a future architecture, not a claim that a test has been implemented.

### A. Connectivity and bootstrap

| ID / problem | Scope, responsibility, and evidence | Staleness, recovery, and coordination | Failure scenario |
|---|---|---|---|
| **GPI-01 — First authenticated contact** | New device/account. Bootstrap provides contact; identity validation checks an independently obtained trust anchor or authenticated invitation. | Stale hints may fail: try another contact or expose unavailability. Admission needs an initial trust ceremony, not an existing directory. Key recovery is separate. | A new device has an invitation, but every suggested peer is asleep. |
| **GPI-02 — Placement and reconnection** | Session and peer pair. Placement selects an eligible host; node discovery resolves it. Evidence: placement grant and authenticated peer identity. | Replace stale addresses. Restore connectivity, then authenticate and recover application sessions. Connectivity confers no provider authority. | A laptop changes network while its source provider remains healthy. |
| **GPI-03 — Traffic budgets and overload** | Link/peer/node. Communication runtime allocates queues, memory, bandwidth, and decode work under negotiated limits. Peer-asserted urgency is untrusted. | Queue estimates age immediately. Bound admission, expose overload, and budget progress for control, interactive, and bulk traffic. Scheduling cannot repair partitions. | Bulk data or hostile advertisements starve lease renewal and terminal input. |

Ground: GDL-032 fixes discovery separation. The directory's bootstrap/home-node mechanisms are design direction; exact cold-start availability, key recovery, and resource budgets still need choices. [S1], [S2]

### B. Shared knowledge and discovery

| ID / problem | Scope, responsibility, and evidence | Staleness, recovery, and coordination | Failure scenario |
|---|---|---|---|
| **GPI-04 — Definition and binding knowledge** | Application/domain. Application authors semantics; Glade validates records. Evidence: versioned definitions, bindings, provenance, and policy. | Cached immutable versions remain identifiable; mutable selections may be outdated. Fetch dependencies or refuse incompatibility. Define concurrent-update rules. | Two providers use one surface name for incompatible payloads. |
| **GPI-05 — Provider selection and uncertainty** | Binding/role. Discovery derives candidates; consumers state freshness needs. Evidence: authorized claims, terms, expiry, and coverage. | Expiry invalidates availability claims. Retry or expose uncertainty/conflict/missing history. Absence requires known coverage; diagnostics respect visibility. | Peers disagree whether a provider exists, is reachable, or is authoritative. |
| **GPI-06 — Metadata synchronization and visibility** | Authorized account/domain/partition. Replication reconciles records; discovery projects them. Evidence: validated histories, heads, and checkpoints. | Partial knowledge is expected. Reconcile missing records and preserve withdrawals/revocations. Choose permitted recipients before dissemination topology. | An offline device returns with an old grant; a relay requests private metadata. |

Ground: GDL-037 keeps the registry fold as runtime authority; advertisements cannot grant their own rights. A complete global directory is not assumed. The directory proposal separates deterministic record folding from clock-dependent expiry checks. Existing `glade-discover` has its own contract; these rows concern stack integration, not an assertion that its mechanisms are absent. [S1], [S2], [S3]

### C. Identity, permission, and isolation

| ID / problem | Scope, responsibility, and evidence | Staleness, recovery, and coordination | Failure scenario |
|---|---|---|---|
| **GPI-07 — Principal, device, and origin identity** | Account/device/session/chain. Identity and operation validators check certification, possession, signatures, and attribution. | Rotation/revocation may invalidate credentials. Restore trusted identity and chain continuity or refuse; define recovery authority separately. | A DTO impersonates another user, or a restarted writer reuses sequence numbers. |
| **GPI-08 — Permission and policy freshness** | Resource/action/reader/replica. Receiving replicas enforce access; sources enforce effects. Evidence: authenticated context and validated grants/revocations. | Enforce known revocation. Disconnected peers cannot prove no newer revocation exists: specify freshness and offline rules. Previously copied bytes remain. | A revoked user retains a stream or retries a queued command. |
| **GPI-09 — Domain and parameter isolation** | Domain/zone/source/scope/interest. Application owns mappings; validators enforce boundaries. Evidence: canonical binding identity and authenticated scope mapping. | Old mappings cannot alias new sources/readers. Rebind or rebuild incompatible instances. Derive `self` from identity; preserve distinct device/tab/origin meanings. | Two Garns scopes collide in one client store. |

Ground: B3–B5 establish authenticated context, identity-bound zone keys, and device proof. GDL-039's final domain/zone vocabulary and remaining axes are still open. Stack-wide freshness and offline guarantees require refinement beyond simply checking the locally available grant set. [S1], [S3], [S4]

### D. Authority and effects

| ID / problem | Scope, responsibility, and evidence | Staleness, recovery, and coordination | Failure scenario |
|---|---|---|---|
| **GPI-10 — Exclusive authority and takeover** | Actual source resource/driver slot. Attachment validates routes; execution enforces exclusivity. Evidence: authorized epoch and resource-checked fence. | Old providers must lose write ability. Recover ownership before effects. Select coordination per resource; a local lock cannot coordinate independent copies. | A partitioned old provider writes after a higher-epoch takeover. |
| **GPI-11 — Command admission and outcome** | Logical effect/retries. Source supplier checks caller, preconditions, and authority. Evidence: authenticated context, idempotency identity, recorded result. | Reauthorize queued intent. Lost responses require outcome lookup or explicit uncertainty. Resolve concurrency using source-specific conflicts. | A save commits, its response disappears, and the client retries. |
| **GPI-12 — Commit and publication** | Source transaction/projection. Source owns commit; adapter owns publication. Evidence: durable revision, result, and publication intent. | Readers may lag. Recover publication without repeating effects; rebuild projections where permitted. Nontransactional effects need separate recovery. | Garns crashes after committing rows but before publishing to Glade. |

Ground: B1 rules attachment and epochs; D12 rules conflict-aware save; D9 rules terminal driver handoff. Source-enforced fencing and publication recovery remain integration decisions. SWMR adapter v1 accepts only one origin per `(share, glade_id, key)`: advancing a lease or reset epoch alone does not permit a different writer at that address. [S3], [S5], [S6]

### E. Durability and recovery

| ID / problem | Scope, responsibility, and evidence | Staleness, recovery, and coordination | Failure scenario |
|---|---|---|---|
| **GPI-13 — Meaning of acceptance** | Memory/durable replica/remote/source. Each boundary owns its receipt. Evidence: successful persistence or commit under a stated failure model. | Memory success can precede durability. Recover acknowledged durable work; expose failure before claiming it. Select replica acknowledgement policy explicitly. | IndexedDB fails silently, then the browser crashes. |
| **GPI-14 — Resume, repair, and generation** | Shape/profile/version/cursor. Taut defines semantics; adapters/stores preserve them; Glial assembles. Evidence: generation, coverage, heads, bootstrap provenance. | Old cursors require contract-specific replay, repair, refresh, bootstrap, or refusal. Recovery preserves promised observations. No universal snapshot/cursor substitution. | Snapshot-delta receives SWMR repair, or CRDT dependencies are missing. |
| **GPI-15 — Retention, deletion, and pressure** | Data class/history. Binding policy sets limits; source/shape constrains collection. Evidence: deletion authority, covered history, proofs, pins. | Preserve irreplaceable accepted work and revocation evidence. Expose allowed gaps or refuse further durable acceptance. Coordinate collection as the exact contract requires. | Compaction removes a revocation and restore revives an old grant. |

Ground: F-GAP10 establishes retention obligations. CRDT v1 deliberately forbids post-bootstrap compaction pending a version change; `snapshot_delta` requires out-of-band refresh where SWMR repairs in-band. The present IDB store retains data on unmount but swallows write errors, so the concrete issue is the durability receipt. [S3], [S7], [S8], [S9]

### F. Source and consumer semantics

| ID / problem | Scope, responsibility, and evidence | Staleness, recovery, and coordination | Failure scenario |
|---|---|---|---|
| **GPI-16 — Source versus replica adaptation** | Supplier/source/delivery store. Source adapter owns source operations; replica store owns accepted operations. Evidence: capabilities and exact semantic mapping. | Serve stale projections only as declared. Rebuild from the named source or report unavailable. Replica ingest never implies source writeback. | A query replica is treated as authority to update SQL rows. |
| **GPI-17 — Independent lifecycles** | File/edit generation; PTY/transcript; provider/source. Each authority owns its transitions. Evidence: revisions, process status, driver epoch, retained output. | Restore only surviving state. Driver handoff cannot migrate a PTY. Disconnect survival requires policy; transcripts and saved files cannot recover lost live work. | A persisted `live` record falsely suggests the old shell survived restart. |
| **GPI-18 — Consumer assembly and visible state** | Glial instance/Grip consumer/interest. Glial interprets shapes and receipts; applications present their states. Evidence: validated changes, revisions, acknowledgements. | Cached views may be stale. Rehydrate, reconcile, release unused interest. Refresh, cancellation, and source deletion have separate effects. | Reconnect duplicates output, or a pending offline save appears committed. |

Ground: GDL-035 places client assembly and local persistence in Glial. GDL-041 fixes shape ownership. H-P4, D12/D13, and D9/D10 constrain editing, file saving, and terminals. The proposed source/replica split and the complete durability/lifecycle interfaces remain unratified. [S1], [S3], [S10]

## 3. What exactly needs synchronizing for discovery?

“Discovery metadata” should be classified before selecting one synchronization policy for it.

| Information | Who needs it / how it behaves | Consequence for synchronization |
|---|---|---|
| Definitions and compatibility references | Authorized consumers/providers of a surface; identified by exact version | Cache stable versions; fetch dependencies; validate which mutable binding selects them. |
| Membership, grants, revocations | Authorized validators/replicas according to placement and visibility policy | Preserve authenticated history or an adequate trusted checkpoint; define policy freshness explicitly. |
| Source assignments and terms | Nodes routing or executing for that resource | Converge knowledge while separately enforcing the resource's fence. |
| Provider advertisements and retained coverage | Interested, authorized consumers and routing nodes | Expire availability claims; communicate withdrawal and history loss; represent partial knowledge. |
| Addresses and relay hints | Participants attempting a connection | Refresh replaceable hints through node discovery; address churn need not rewrite source authority. |
| Subscription interests and live presence | Participating sessions/providers | Bound and expire transient state; do not retain every historical interest as authoritative application history. |

The candidate discovery answer is a set of independently supported facts: **known definition, granted role, advertised availability, retained coverage, and usable route**. Those facts may disagree. Keeping them separate makes replica reads, fresh source reads, and effectful commands select different eligible providers.

No global completeness assumption is required here. The unsolved choice is the necessary coverage and freshness for each operation in each supported deployment scope.

## 4. Four meanings of “tiered transport”

| Possible meaning | Problems it addresses | What it leaves to another contract |
|---|---|---|
| **Route fallback:** local/direct/relayed contact | GPI-01/02 | Permission, source authority, metadata completeness |
| **Traffic scheduling:** control/interactive/bulk budgets | GPI-03 | Whether a received claim or operation is valid |
| **Dissemination scope:** account/domain/interested peers | GPI-05/06/08 | Underlying connectivity and exclusive source ownership |
| **Bootstrap dependency:** initial trust/contact → authenticated metadata sync → application use | GPI-01/04/06/07 | Ongoing recovery and authority freshness |

These can coexist but need separate justification. Bootstrap, control synchronization, and application delivery are candidate communication roles; they do not yet imply three protocols or three services. Within application delivery, terminal input and bulk file data may need different scheduling. GDL-032's placement/service/node separation remains a semantic constraint regardless of scheduling choices.

One useful stress scenario is: **a new device joins while a large transfer is running, its first provider disappears, and a grant is revoked during retry**. This tests bootstrap independence, resource budgets, partial discovery, reauthorization, and recovery without assuming a particular topology.

## 5. Open questions to settle before drawing layers

This is a local record of proposed discussion questions, not a change to the canonical decision log.

| Question | Why it determines boundaries | Discussion status / references |
|---|---|---|
| **Operating scope:** which combinations of personal devices, always-on replicas, shared organizations, and untrusted relays must the first architecture support? | Determines bootstrap availability, placement, metadata visibility, and scale assumptions. | **Broad environment agreed:** independent users/devices, intermittent connectivity, explicitly granted trust. First-release coverage remains open. GAD-01; GDL-010/013/018/031/032; G6 D02/D04. |
| **Knowledge contract:** what coverage and freshness must discovery establish for a cached read, fresh source read, supplier instantiation, or command? | Determines which metadata is replicated where and what absence/uncertainty means. | **Owner proposal recorded:** Iroh substrate, dynamic registry shards, locality, and distributed indexes. Guarantees remain open. GAD-04/05; G6 D01/D02/D04. |
| **Partition contract:** which operations remain useful offline, and where must execution refuse until authority is established? | Determines fencing and policy-freshness obligations. Authorized local CRDT edits and remote filesystem saves have different answers. | **Discussed, exact contract open:** renewable advertisements with local acceptance and eventual replication proposed; source authority remains separate. GAD-06; GDL-043; G6 D04/D05/D07. |
| **Durability contract:** what has the system promised when it says accepted, saved, or completed? | Determines receipts, stores, publication recovery, and what pressure handling may discard. | **Partially grounded:** preserve existing discovery v3.1 durable local acceptance before gossip. New publisher/replica receipt semantics remain open. GAD-06; GDL-043; G6 D05/D06/D07. |

GAD references identify entries in the linked discussion record. Trust is agreed as a separate problem to develop alongside core infrastructure (GAD-02/03); no separate trust task has yet been launched in this conversation. Existing decisions constrain the remaining answers and do not require renewed approval. Protocol libraries, cache numbers, package splits, and consensus choices can follow once the required guarantees are explicit.

The next architecture sketch should account for every GPI row and identify which component owns each failure outcome. Discovery synchronization, transport scheduling, and source coordination should remain separately explainable even if one runtime hosts them.

Follow-up package work is recorded in [GladePackageArchitecture.md](GladePackageArchitecture.md), with reusable independence/TDD/fast-feedback requirements in [LibraryBoundaryAndTestingPolicy.md](LibraryBoundaryAndTestingPolicy.md). These organize implementation boundaries; they do not resolve every domain guarantee in this inventory.

## Evidence and limits

This document synthesizes the two reviews and follow-up source checks. It does not claim to re-audit discovery correctness or reproduce integration failures. No implementation tests were run for this documentation change. Root HEAD at creation: `e74a86af2749192ecad401ec0457e8d08604f785`; existing root reports and discovery documentation changes were present in `gwz status` and preserved.

The comparison corrections are incorporated: profile-specific refresh; no implicit CRDT v1 compaction; source fencing separate from advertisement epochs; IDB unmount retention separate from write durability; and recovery of a projection separate from replay of every source event.

[S1]: /Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md:49 "Recorded GDL-031–041 statuses"
[S2]: /Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeWorkspaceDirectory.md:47 "Directory records, bootstrap, expiry, and discovery layering; design direction"
[S3]: /Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:625 "Owner ratification of GLP-0006 worksheet"
[S4]: /Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeAuthzModel.md:201 "Enforcement boundaries and offline limits"
[S5]: /Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeSwmrAdapter.md:46 "GSA-03 single-origin constraint"
[S6]: /Users/owebeeone/limbo/glade-wz/dev-docs/ApplicationStackContractReview-G6.md:160 "Reviewed source commit/publication boundary"
[S7]: /Users/owebeeone/limbo/glade-wz/taut-shape/dev-docs/TautShapeCrdtDecision.md:89 "CRDT v1 retains post-bootstrap operations"
[S8]: /Users/owebeeone/limbo/glade-wz/taut-shape/dev-docs/TautShapeSnapshotDeltaDecision.md:9 "Snapshot-delta profile refresh semantics"
[S9]: /Users/owebeeone/limbo/glade-wz/glial/src/store_idb.ts:96 "Unmount retention and write-through failure handling"
[S10]: /Users/owebeeone/limbo/glade-wz/dev-docs/glade/suppliers/glade-terminal.md:97 "Terminal handoff and proposed disconnect behavior"
