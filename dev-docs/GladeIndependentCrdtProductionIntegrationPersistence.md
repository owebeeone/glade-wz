# Independent CRDT persistence adapter — B candidate

Status: implementation/qualification candidate; combined independent Code, State and Surface acceptance is pending. This document describes the actual B provider, rather than claiming C binary routing, exchange, migration or activation. The semantic authority remains [the accepted design](GladeIndependentCrdtProductionIntegrationDesign.md) and [implementation plan](GladeIndependentCrdtProductionIntegrationPlan.md). The standalone consumer object is [PersistenceUsage](GladeIndependentCrdtProductionIntegrationPersistenceUsage.md). Initial B1 and typed review reports remain unchanged.

## Allocation and actual call path

`node/src/assembly.rs::NodeAssembly::independent_configured` constructs `ConfiguredIndependentAdapters<P>` from an explicit signed `EvidenceConfig`, provider-owned `PolicyClock`, and disposable `DiskRootConfig`. Its `recovery` is the actual `DiskHost<P>`, with actual `ReplicaRecoveryHost` and `StorageAttemptHost` implementations. The returned `DiskSession<P>` implements both session contracts. Verification/sealing/observation use its owned `GenuineEvidence<P>` through real `EvidencePort` calls; storage callbacks use the actual selected Pure core and native provider. The default `independent_adapters()` retains its refusing provider selection. No new binary route is enabled in B.

Glade node remains Integration. The only added normal package edge is the semantic-selected `glade-crdt-admission-core` replay/projection edge; there are no new external versions or architecture/dependency/process-global allowlist relaxations. API/data/codec roles remain unchanged. Native disk primitives, scoped trusted observations, process ownership and assembly stay in node. No physical success is supplied by MemoryHost or caller-created Facts.

## Owned roots and exact envelope

`DiskRootConfig` MUST name separate absolute fresh data and trusted-floor directories, a nonzero registration, and explicit provision/reopen intent. Registration markers bind both canonical paths, directory identities and stable lock-file identities. Both lifetime root locks remain held; their paths are never unlinked on Drop. Substituted paths, copied data at another path, competing owners, missing floor and mismatched registration MUST refuse. Provision writes the compatibility-refusal marker only in a newly owned disposable data root. The actual previously committed legacy executable was tested against that fresh root; existing legacy stores are not migrated or sealed by this work.

The private physical floor envelope is closed, versioned, bounded canonical data. It retains `ReplicaFloor`, pending transaction metadata, exact completion floor and unmaterialized interrupted intents. Private `DiskOperation` is retained beside typed `FloorIntent`, so missing-next fallback still knows whether the interrupted operation was prepare, start, publication, fence, ingress or trusted observation. Every floor advancement preserves complete issuance maxima, namespaces, active guards and previously retained interrupted metadata.

The selected `ReplicaImage` embeds its retained prior floor, never the future floor selecting itself. Image bytes are encoded first, their digest is computed, and the external floor selects that digest. Selection requires no hash fixed point. Loss records use the closed physical tuple `("glade/ic3/ingress-loss/v1", (ReceiveGuard, (generation, observed-prefix-digest-bytes)))`; the digest is exactly 32 bytes. They mark permanently untracked history and cannot discharge that uncertainty into completeness.

## Actual transaction/recovery order

The bounded transaction is:

1. Validate ownership, exact expected floor/image selection, complete proposed image and capacity before I/O. Retain the exact original native request/plan/receipt and counter custody.
2. Persist and sync the full intent floor, including operation kind, projected maxima and still-active guards. A lost reply is unresolved.
3. Write the inactive fixed-capacity image, then sync its file. The old selected slot stays untouched.
4. Replace the trusted completion floor by bounded temporary-file write, file sync, rename and directory sync. Only the completed selection can acknowledge custody.

Reopen validates the authoritative floor and its nonrecursive image binding first. An intact next image MUST pass full authenticated State/Recovery/session/batch/callback/projection validation before re-syncing that exact image and selecting it. The invalid-but-structural pending-next regression proves failure leaves the trusted floor unchanged. Failed re-sync stays unavailable. A missing or torn next image preserves the old exact image plus every advanced counter/guard and explicit unmaterialized intent; it does not infer terminality or reset issuance. Source-owned recovery reconstructs the actual signed policy body when its retained floor is ahead, before usable reads/appends.

The host reattaches only exact retained queries/requests. Actual `core::step` callbacks rebuild projection/accounting before usable load. Original retired requests remain available to authenticate duplicate versus contrary results. Complete original Prepare custody is required even after a later Resolve callback exists. No callback authority is recreated from an AttemptId alone.

## Critical capacity and terminal strength

Both image files are fully written and synced at their fixed declared capacity. Qualification checks actual file length and allocated backing blocks, not only a sparse logical length or arithmetic counter. Subsequent images cannot resize either slot. Ordinary images MUST fit `image_bytes - critical_bytes`. Before native preparation, the provider serializes the exact possible Reserved, core-callback, Started, terminal and final callback images; before receive, it reserves worst-case guard/loss-discharge growth. Active guard discharge remains included in every native future-frame capacity check. Native and ingress critical writes cannot borrow undeclared image space; a validated permanent-loss discharge may use its reserved critical region even when ordinary admission is full. Finite guard/attempt/request histories have no implicit garbage collection.

Critical reservation covers the owned files and bounded complete-image representation. Shared-device outage, external volume exhaustion or failed write/sync still cause unknown/unavailable status, without a false terminal or successful receipt. This is process-restart durability on the qualified Unix filesystem profile, not power-loss, corruption repair, network quorum or protection from arbitrary simultaneous rollback of data and the independently trusted floor.

Reserved remains reversible only through the retained native predicate: fence winning before Started selects immutable NonCommit. Started winning first protects the original cut and receipts; later current expiry/revocation cannot rewrite it. Inspect/fence/late exact Begin return the same original terminal. Authenticated contrary terminal callbacks retain an integrity diagnostic and cannot reopen the attempt or admit data. Current policy/time observations are durably retained, including unavailable verification; historical signed cuts/receipts remain independently valid after current expiry/revocation.

## Receive, loss and close

`begin_ingress` MUST durably establish the exact bounded guard before any application receive/parse factory executes. Its floor-write failure yields no permit. `receive_ingress` rejects foreign/reused/closed capabilities before constructing work. The owned future returns the permit only when its bounded `Result<Bytes,Fault>` is Ready. A dropped pending future or task is not drain evidence.

Nonmutating `can_settle` and `can_abandon` utility queries check issuer-owned status. The host uses them before I/O and retires status only after committed selection. Unknown or refused owned settlement retains its exact by-value permit, observation and checkpoint. Exact owned continuation retry cannot fabricate a new permit. `record_observation` can resume the exact stored drained continuation after a pre-I/O checkpoint refusal, or deduplicate an already selected exact observation; fresh unguarded observations refuse.

Exact signed bounded inventory observations and missing obligations are coupled to guard settlement. A drained partial/unclassifiable result can select a compact permanent-loss record through `retain_ingress_loss`. Failed observation and failed loss writes leave the pre-existing floor guard. A later empty or successful inventory cannot erase earlier loss or an unknown frontier. `reconstruct_cut` is complete only when the original kernel cut, floor guards, retained observations/obligations and interrupted metadata all justify it; no kernel sticky bit is cleared.

`close` first stops fresh input. Both session close methods retain physical ownership and return Pending while a receive, exact continuation, query, native request/plan, nonterminal attempt or uncertain floor remains. Already-owned continuations may drain and settle. The node coordinator MUST own and join its read/parse/callback tasks before clean release. Actual cancellation/join qualification demonstrates X retaining its locks while Y, with separate root/store/plan/invocation/guard namespaces, commits independently.

A generic reopen cannot prove a prior external receive future was drained merely from lease change, lock release, absence of inbox bytes or restart. An orphan Active guard remains conservatively pending: local writes may proceed when integrity/capacity permit, and `complete_local` remains false. No replacement permit or reset API is supplied. C must retain this limitation after SIGKILL during a receive; clean shutdown can cooperatively cancel/join and select loss first.

## Qualification boundary

The new [B2 evidence](history/GladeIndependentCrdtProductionIntegrationB2Evidence.md), source pins and chronological compressed run log will capture the entire combined B implementation range and consumer hashes. Executed candidate controls include all four native kinds, 228 native SIGKILL cuts, 65 ingress SIGKILL cuts, original exact receipt retry/callback replay, both terminal winners, corruption/binding controls and X/Y independence. Final source gates and reviewer acceptance are recorded in the evidence, not inferred from this document.

The canonical three-language representation prerequisite and genuine B review prerequisite remain fail-closed. No accepted review record has been invented. Actual two-process transport/routing and history-bearing remote execution belong to C after the required committed qualification record; IC4 live activation remains separate.
