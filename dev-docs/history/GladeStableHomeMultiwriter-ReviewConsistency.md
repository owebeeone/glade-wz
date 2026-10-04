# Glade stable-home design and multiwriter evaluation — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT `dev-docs/GladeStableHomeDesign.md`, DRAFT `dev-docs/GladeMultiwriterSettingsEvaluation.md`, and the added GDL-050 note in `dev-docs/DecisionLog.md`, at root commit `d6f994e1e4d3c4458be9b577336fe849ae628e2c`.

**Baseline:** Root `d6f994e1e4d3c4458be9b577336fe849ae628e2c`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Repository evidence was read with `git show <exact pin>:<path>`, with numbered lines. Live HEAD and working-tree changes were not review evidence.

**Date:** 2026-10-03.

**Axis:** Internal coherence, agreement with the controlling source graph, amendment-target accuracy, and satisfiability of the proposed evidence requirements. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0/P1/P2 findings; one bounded P3 citation defect. This accepts the combined semantic design/evaluation scope for the next contract/proof tranche only. It does not ratify a candidate implementation, executable interface, wire profile, migration, or production activation.

---

## 0. Evidence base

Read the complete generated Consistency prompt and verified its SHA-256:

`4d4ad87e88f24af32cab3bc3e1ca330184d7a62bdf49d3f5217865c26cd6d224`.

Read `AGENTS_GWZ.md`, `AGENTS.md`, the review-loop skill, and its canonical prompt template. No files were written, no Git state was mutated, and no tests, builds, or runtime operations were performed. No current-round peer report was accessed.

Start and end `git rev-parse` checks returned identical results:

| Checked object | Start and end result |
| --- | --- |
| Root revision | `d6f994e1e4d3c4458be9b577336fe849ae628e2c` |
| Glade revision | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Glade-discover revision | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| H1 reviewed blob | `e1cdb3660c1d7c993880d19120b829c4ab5af407` |
| Multiwriter reviewed blob | `83784016d3e418ddf4fab4a087aa1ccc9067adb4` |
| DecisionLog reviewed whole blob | `c8ce426b3aa125457f28f778ac212424a5c03873` |

Read H1 lines 1–304, Multiwriter lines 1–226, and DecisionLog’s controlling entries and GDL-050 at lines 73–83. Root sources included the resource-home comparison and alternatives, Authz §§1/3a/3b/4/4a/7a, WorkspaceDirectory genesis/locking/WD-8 passages, DiscoveryModel §§0/3 and its takeover test, matrix D-06/R7/R9/R16, LibraryBoundaryAndTestingPolicy, GladePackageArchitecture, GladeBuildEntry, TautShapeCatalogAdoption, and qualified historical appearance-plan passages.

At the Glade pin, inspected Substrate §§2/3/6 and W1–W8, CrossNodeWrites’ options/rules/limits, ShapeDispatch, CrdtAdapter, Zones, and targeted code: `client-ts/src/fold.ts:22–61`, `session.ts:20–129`, `node/src/accept.rs:46–255`, `mesh/route.rs:27–68,304–431`, `registry.rs:754–775`, `store.rs:291–385,696–701`, and `claims.rs:271–351`. At the discovery pin, read RegistryContractDraft lines 1–164.

The external Gyld capture and declaration were read only as qualified allocation context. Their descriptions of DirectoryRules, Directory, Admission, Records, StorageAdapter, and runtime ownership support H1’s proposed responsibility mapping; they were neither changed nor treated as ratified contracts.

The primary [CRDT study’s indexed register excerpt](https://www.lip6.fr/Marc.Shapiro/papers/2011/Comprehensive-CRDTs-RR7506-2011-01.pdf) supports the LWW/MV distinction. Direct PDF retrieval timed out; full-paper metadata-collection analysis was therefore not independently reverified. [Lamport’s author-hosted publication explanation](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/) supports the distinction between causal partial order and an imposed total order.

## 1. Findings

### [P3-1] H1’s amendment table points to the wrong DiscoveryModel section for takeover

**Location:** `GladeStableHomeDesign.md:271`, amendment-table row naming “WorkspaceDirectory §4/WD-8 and DiscoveryModel §3 takeover.”

**Violated invariant:** An amendment inventory must identify the controlling passage it intends to qualify. DiscoveryModel §3, lines 144–154, describes binding identity, local directory resolution, routing, and home-node terminology. Its explicit takeover obligation appears instead in §7, lines 268–269: the higher-epoch claim wins and the stale holder’s serve bounces.

**Reproduction:** Follow the table’s DiscoveryModel §3 reference while preparing the next amendment. The cited section contains no takeover clause. A drafter can reconcile the routing recap while overlooking the separate §7 test expectation that still requires epoch-based takeover. The eventual contract package would then contain an obsolete test obligation unless its broader enumeration catches the error.

**Impact:** This is a bounded traceability defect. H1 itself expressly forbids lease expiry or a locally incremented generation from moving the binding (`:109–114,156–181`), and requires exact clause enumeration before freezing (`:276–278`). Those safeguards prevent this pointer error from authorizing an unsafe acceptance path in the present semantic packet. It consequently does not rise to P2.

**Required correction:** Name DiscoveryModel §7’s epoch-fence/takeover scenario as an amendment target. Retain §3 separately where its routing/home-role prose needs qualification for H1.

**Closure check:** Re-trace the corrected table to the pinned source and confirm that it explicitly accounts for both the §3 routing recap and §7 takeover expectation. The subsequent contract amendment’s traceability check should require an H1 counterpart that rejects takeover while preserving the legacy scenario for unactivated shares.

## 2. Invariant analysis

**Placement, root identity and namespace authority — SH-001/002.** I attacked two disconnected creators choosing different homes for the same label, then retried after a lost reply. H1 distinguishes fresh independent genesis from naming inside an existing share (`:47–71`). Independent roots remain distinct; existing-share naming reaches its fixed home and serialized namespace. Creation intent includes requester, scope, durable request identity, exact draft and proposed address (`:93–100`). Exact retry precedes stale-revision rejection after current access validation (`:127–131`). This agrees with RegistryContractDraft’s original-receipt-before-stale-CAS rule (`:26–32`) without pretending that its narrower journal already implements the complete creation transaction.

The fresh share ID, authority root, home binding, zone address, origin and transport endpoint remain separate concepts. Creation does not transfer root authority to the operator. Case-sensitive canonical bytes and authenticated private-key derivation also prevent display-label or caller-string aliasing from becoming identity.

**Atomic creation and honest durability — SH-003/009.** The commit-before-reply/publication attack is closed semantically by one transaction containing binding/declaration, receipt, revision and outbox (`H1:118–144`). A reply loss retains the original outcome; outbox delivery acknowledgement cannot erase deduplication. Corrupt, missing or uncertain state becomes unavailable rather than empty. Capacity stops admission rather than discarding replay fences (`:146–152`).

The stronger metadata guarantee is identified as new and separate. Substrate R2 (`:323–341`) promises process-crash survival without fsync; the inspected app-log append merely writes bytes (`store.rs:696–701`). H1 does not attribute OS/power-loss durability to that implementation, nor upgrade application receipts because metadata becomes stronger. Its required real adapter evidence is compatible with LBT-006/009.

**Discovery, empty state and conditional initialization — SH-004/008.** An empty directory, cold browser, or missing reply cannot justify replacement genesis or persistent defaults (`H1:156–165,194–215`). A seed requires authoritative absence at a verified cut and conditional admission; a read/write gap cannot simulate CAS. RegistryContractDraft explicitly says even an untruncated empty result proves neither global absence nor unreachability (`:59–65`).

The outage behavior matches the comparison’s conditional H1 recommendation: cached reads and pending edits may continue, authoritative writes wait, and permanent history loss stays loss/unavailable. GDL-050 records authorization of that design/evaluation work without claiming deployment or a completed availability proof.

**Exclusive admission and lifecycle — SH-005/006/007.** I attacked divergent lease clocks, a stale advertisement, retirement racing an append, and a queued effect whose earlier route check was valid. The binding governs actual admission (`H1:167–190`); discovery cannot appoint another home. Retirement and local policy updates share the commit ordering. Queued effects must revalidate at the sink or use a sink-enforced capability. Physical checkout mutation retains its actual lock.

The crash model explicitly excludes contradictory owner genesis and deliberately cloned home identities/journals (`:73–86`). Thus a local process/storage lock is not misrepresented as protection against independently cloned machines. Detected contradictory evidence is quarantined rather than ranked. Create/retire, reset/update, and startup/drain pairs are present; initial name reuse is explicitly unsupported.

**Migration and amendments — SH-010.** The attempted shortcut—activate H1 while an offline legacy writer can still acknowledge the same address—is prohibited (`:252–265`). Migration inventories acknowledged history and possible splits, excludes legacy paths, establishes a verified baseline, and blocks when exclusion cannot be proved. Rollback preserves the fence.

H1’s table correctly identifies stable binding versus live-claim selection, declaration-established shape versus first arrival, separate metadata receipts, and retained local locking. The inspected route still has unknown-share local fallback (`mesh/route.rs:40–68`), and holder admission still uses its own live claim (`accept.rs:244–254`); the packet presents these as amendment/integration obligations. The P3 section pointer above is the only concrete amendment-location defect found.

**Shared bootstrap, distinct admission modes — MW-001/008/012.** Multiwriter preserves H1’s canonical initial binding and explicitly cannot merge independently created “appearance” roots (`MW:98–112`). Its possible exception is subsequent preference admission, not creation, governance, SWMR or effects. Activation must exclude legacy admission and prevent rollback from interpreting new history as whole-value LWW (`:203–223`). These restrictions make the parallel recommendations compatible.

**Merge capability and schema — MW-002/003.** The whole-document lost-update example follows the implemented max-`(lamport,origin)` fold (`fold.ts:41–50`). Per-field preservation and MV conflict exposure are conditional candidates requiring profiles, codecs and corpora. ShapeDispatch permits durable value/log/SWMR/CRDT but locally folds only value/log; CrdtAdapter GCA-04..07 supplies explicit text semantics, not preferences. The evaluation correctly refuses to derive a map profile from historical `message` wording or `latest`.

Unknown fields, fallback rendering, and operation validation remain distinct. Wallpaper/toggle coupling requires a compound register or atomic operation rather than an unsupported independence claim. Historical workspace placement versus Zones’ account-wide example is explicitly a pending scope decision; layout stays local.

**Ordering, recovery and retry — MW-004/005/006.** Restore-with-old-high-clock, same-origin parallel writers, reversed predecessor delivery and byte-different retry attacks are required witnesses, not reported successes. `Session.restore` constructs a session whose clock begins at zero (`session.ts:20–31,124–129`); the evaluation identifies that gap accurately. Its rules require recovered sequencing/clock custody, pending-or-refusal for gaps, descendant blocking and convergent fork quarantine. Existing tail removal after refusal is cited as limited evidence, not sufficient independent-admission recovery.

**Authorization, receipts and retention — MW-007/009/010/011.** Partition/revocation cannot imply instantaneous freshness or retroactive erasure. The evaluation requires a chosen provisional/validity policy and preserves signed governance, AZ-16 membership, AZ-17 owner access and operator-approved placement. It accurately states that current forwarding authenticates the forwarding node rather than carrying the client principal (`accept.rs:219–241`; Substrate W2).

Reset, absence, explicit defaults, deletion and tombstones require separate semantics. Collection needs a reviewed frontier and rejoin policy; capacity cannot silently remove evidence required by offline replicas. Local acceptance cannot borrow W4’s holder-plus-forwarder promise. These demands are jointly satisfiable through a bounded profile, explicit refusal and refresh/retirement outcomes; they do not require unbounded storage or impossible disconnected revocation.

## 3. Risks and next action

The unresolved choices are substantive: authorization/action mapping, taut/profile/version discrimination, storage atomicity, lock custody, preference scope and merge policy, disconnected validity, bounds, checkpoints and rejoin floors. Their outcomes are expressly deferred, while the packet supplies reviewable lifecycle and failure shapes. No implemented witness is claimed.

The single next action is to proceed to the scoped contract/proof tranche, correcting P3-1 in its amendment inventory. That tranche must preserve the Gyld allocation prerequisite, compiling consumer/conformance witnesses, RED-first scenarios, real adapter evidence and affected-consumer architecture checks. This GO supplies no permission to activate either admission mode or to replace canonical contracts silently.
