# Glade stable-home design and multiwriter evaluation — SAFETY-AXIS REVIEW

**Review object:** DRAFT `dev-docs/GladeStableHomeDesign.md`, DRAFT `dev-docs/GladeMultiwriterSettingsEvaluation.md`, and the added GDL-050 note in `dev-docs/DecisionLog.md`, at root commit `d6f994e1e4d3c4458be9b577336fe849ae628e2c`, dated 2026-10-03.

**Baseline:** Root `d6f994e1e4d3c4458be9b577336fe849ae628e2c`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. Reviewed documents and canonical/member sources were read using `git show <exact-pin>:<path>`.

**Date:** 2026-10-03.

**Axis:** Safety — unsafe allowed paths, degraded operation, irreversible transitions, disclosure, recovery and mixed-version behavior. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. The combined semantic design/evaluation packet is coherent for the next contract/proof tranche. This verdict does not accept an executable interface, wire profile, production adapter, migration, security implementation or deployment.

---

## 0. Evidence base

I read the complete generated Safety prompt, the review-loop skill and canonical prompt template, and workspace `AGENTS.md`/`AGENTS_GWZ.md`. The generated prompt’s SHA-256 matched:

`80724bab8344d8b53e710dd72aa7beb72adbbed4a1d818361391b6ac615224c2`.

The reviewed documents were read completely: H1 lines 1–304; Multiwriter lines 1–226; GDL-050 at DecisionLog lines 73–83. I also inspected DecisionLog GDL-031/032/034/036/037/038/042/043/045.

At the root pin, controlling evidence included:

- `GladeResourceHomeComparison.md` and `GladeResourceHomeAlternatives.md`, including home dependence, independent genesis, receipt limits and exclusive-admission requirements.
- `glade/GladeAuthzModel.md` §1/3a/3b/4/4a/7a, covering accepting/folding enforcement, creation-rooted ownership, signed governance, policy closure, membership/private-key rules and operator-approved placement.
- `glade/GladeWorkspaceDirectory.md` §3/4 and WD-8; `glade/GladeDiscoveryModel.md` §0/3 and routing/security context.
- `GladeBuyBuildMatrix.md` D-06/R7/R9/R16; `LibraryBoundaryAndTestingPolicy.md`; `GladePackageArchitecture.md`; `GladeBuildEntry.md`; `TautShapeCatalogAdoption.md` GSC-03–08.
- Historical `glial/GlialAppearanceSettingsPlan.md` §1, whole-value/migration discussion, §6 staging and §8 exclusions, treated as qualified history rather than current implementation evidence.

At the Glade pin, I inspected SubstrateV1 §2/6, including R1–R8 and W1–W8; CrossNodeWritesPlan’s dual-admission rejection, W1–W8 and limits; ShapeDispatch; CrdtAdapter; and Zones. Targeted code inspection covered `client-ts/src/fold.ts` and `session.ts`, `node/src/accept.rs`, store chain/shape/equivocation and append-persistence paths, `mesh/route.rs`, registry claim ranking, and claims creation.

At the discovery pin, I read `RegistryContractDraft.md`, including atomic acceptance, cancellation uncertainty, exact retries, capacity, partial resolution, authorization and composition limits. Linked external Gyld capture/declaration excerpts were inspected only for allocation obligations: pure directory rules, Records ownership, Admission and StorageAdapter atomicity. They were not treated as ratified specifications.

Start and end `git rev-parse` checks returned identical results:

| Checked object | Start and end result |
| --- | --- |
| Root revision | `d6f994e1e4d3c4458be9b577336fe849ae628e2c` |
| Glade revision | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Glade-discover revision | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| H1 reviewed blob | `e1cdb3660c1d7c993880d19120b829c4ab5af407` |
| Multiwriter reviewed blob | `83784016d3e418ddf4fab4a087aa1ccc9067adb4` |
| DecisionLog whole blob | `c8ce426b3aa125457f28f778ac212424a5c03873` |

No files were modified. No builds, tests, runtime operations or Git mutations were performed. No current-round peer report was accessed.

## 2. Invariant analysis

### Creation, identity and recovery: SH-001–004

I attacked concurrent creation in two forms. First, disconnected devices create independent roots using the same human label. H1 lines 52–57 makes these distinct identities and requires another device to join the existing root through its reference/invite. It does not promise global name uniqueness. Second, clients request the same canonical name inside an existing share, proposing different outcomes. Lines 47–50 and 59–71 place those requests at one bound home under administrative creation authority. The commit gate revalidates policy/declaration revision; occupied names conflict rather than merge or replace.

The lost-reply attack is concrete: home commits creation, publication/reply is lost, client reconnects with its original expected revision. H1 lines 121–133 requires one atomic binding/receipt/revision/outbox transaction and returns the original exact outcome before stale-revision rejection. Changed bytes under the same request ID reject. Cancellation remains queryable, and later revocation may conceal the receipt without undoing the binding or permitting another genesis (lines 146–152). This matches the discovery journal’s recovery discipline without falsely claiming that its independent ports already constitute the required namespace transaction.

Power loss between binding, receipt and advertisement would otherwise produce accepted-but-unrecoverable identity. H1 explicitly disallows partial acceptance and proposes OS/power-loss metadata durability separately from current application `Ok` (lines 137–144). Corrupt, missing or uncertain journals cannot initialize empty. Actual atomicity and durability are adapter obligations; the semantic requirement is clear.

For initialization, I traced: browser observes empty replay; another browser commits an edit; first browser imports stale legacy settings. Lines 194–202 requires authoritative absence at a verified cut and conditional admission, expressly prohibiting simulated CAS through a read/write gap. Partial directory emptiness and missing replies do not authorize defaults or genesis. SH-004 therefore has a meaningful required outcome rather than merely “subscribe before writing.”

### Exclusive admission, retirement and effects: SH-005–007

A partition gives nodes different live claims and clock readings. Current pinned code ranks live claims by epoch/node ID and retains an unclaimed-local fallback: `registry.rs` lines 754–775 and `mesh/route.rs` lines 40–67. These mechanisms can route differently under partial knowledge; they do not establish exclusive acquisition.

H1 does not reuse them as authority. Lines 167–180 requires the actual admission boundary to check the immutable committed binding, active declaration/incarnation, current locally validated permission and chain constraints. A non-home can relay or retain accepted copies but cannot independently acknowledge authoritative append. Lease expiry changes availability, not the binding.

I also tried a restarted or copied home. Lines 80–86 requires exclusive ownership of the actual journal/store, locking before admission and draining before release. Uncertain completeness, identity mismatch and detected rollback remain unavailable. An operator running cloned identity/journal instances is expressly outside the honest-operator crash model; it is not presented as safe transfer. Conflicting binding evidence quarantines rather than electing a winner. This is a bounded fault model, not a hidden failover promise.

For retirement, the adversarial order is: route permission succeeds; retirement or local policy update commits; queued append/effect later executes. Lines 135 and 167–174 places local retirement/policy/admission in one ordering and requires effect execution revalidation or enforcement by the actual sink. A physical checkout also retains its own lock. Earlier routing permission cannot authorize the later effect. Retired addresses cannot recreate zones; initially unsupported name reuse and retained retirement fences prevent delayed retries from reactivating old identity.

H1 additionally fixes shape/schema by declaration rather than first arrival. Wrong private principals must fail at ingress, and SH-007 demands agreement across client/relay/home. These are stronger proposed requirements than the pinned store’s history-derived shape/writer decisions, not claims that current code already satisfies them.

### Availability, capacity and activation: SH-008–010

The outage attack loses the home while another node has cached data and queued edits. H1 lines 210–215 permits pending edits under their actual client persistence guarantee, with honest denial/gap/conflict/Retention handling on reconnect. It neither upgrades memory-only edits to durable acceptance nor promotes the replica. Permanent missing history is loss/unavailable; fork/import requires a different identity and explicit intent.

Capacity exhaustion cannot silently discard deduplication or retirement history to permit new names. Lines 149–152 allows explicit refusal of new creation instead. SH-009 also requires bounded names, requests, proofs and outbox, clock uncertainty failure and no private referral disclosure. Conflict/outcome lookup evidence is authorization-filtered. Concrete limits remain contract work, but eviction-based resurrection and unauthorized disclosure are already forbidden.

The mixed-version attack starts an old node accepting legacy writes while a new node activates H1 for the same address. Lines 252–265 forbids simultaneous admission and requires a discriminator enforced by every serving adapter. Migration inventories acknowledged history, excludes all authoritative legacy paths and blocks on offline writers or unaccounted history. Rollback preserves the gate. A deployment flag or deterministic claim winner cannot satisfy these conditions.

### Multiwriter identity, order and validity: MW-001–007

Multiwriter is restricted to the five appearance preferences, excluding governance, working-copy mutation and external effects (lines 9–30). MW-001 retains H1 binding/bootstrap authority; matching display names cannot merge independent histories.

I traced restored-origin ordering: persisted history contains a high Lamport timestamp; a restored session starts its clock at zero; the next user edit loses to old state. Pinned `session.ts` lines 21 and 124–129 confirms that present restoration does not recover the clock. The evaluation identifies this limitation and requires seq/predecessor/watermark recovery, causal observation, cross-language ordering, numeric bounds and overflow outcomes (lines 120–131; MW-004). It does not claim current restoration is sufficient.

For equivocation, two instances use one origin/sequence with different operations, delivered in opposite orders to two replicas. Lines 133–138 forbids folding through missing/rejected predecessors and requires retained fork evidence and convergent quarantine. Existing client refusal only removes an own tail; the evaluation explicitly says this does not suffice after several replicas acknowledge it. MW-005 therefore targets final eligibility, not just successful hash detection.

The principal dual-admission attack is A acknowledging an operation B later rejects because of different shape or policy. CrossNodeWritesPlan §2 identifies exactly this defect. Multiwriter lines 114–118 and 142–154 requires shared validation/profile, authenticated principal evidence and policy-based eligibility, while MW-007 requires a declared partition/revoke/heal outcome. Identical payload sets alone are insufficient. Unknown devices, missing policy and wrong operators remain fail-closed.

Disconnected revocation is honestly conditional: stale acceptance versus fresh-evidence blocking remains an owner decision, with provisional status and later eligibility specified before activation. Forward-only revocation does not silently erase replicated history. This is a gated decision input rather than an unsafe selected policy.

### Preference races, retention and receipts: MW-008–012

Whole-document LWW loses A’s theme change when B’s concurrent zoom document wins. The evaluation states that loss directly. Per-field LWW is preferred only after field independence and same-field loss are accepted; wallpaper/toggle coupling must use an appropriate compound register or atomic operation (lines 81–89). Invalid combinations cannot be justified by convergence. MW-002/003 requires codec parity and competing/permuted delivery evidence.

Unknown state cannot seed defaults. MW-008 separates unknown, confirmed absence, explicit default and user edit, and requires canonical idempotent import with a declared outcome for competing legacy values. Reset is attributed preference data, not journal erasure. MW-009 requires deterministic reset/update behavior and tombstones preventing delayed resurrection. Collection requires a reviewed frontier/checkpoint/rejoin policy, never elapsed local time.

MW-010 separates local holding, process-crash persistence, synchronized storage and independent copies. Pinned `store.rs` lines 696–701 writes application records without fsync; W4 promises holder/forwarder copies, not arbitrary failure-domain durability. The evaluation accurately preserves these limits and expressly forbids carrying W4’s promise into independent local success.

MW-011 bounds pending dependencies, conflicts, origins and retained evidence without evicting recovery-critical records. MW-012 rejects unsupported profiles before mutation and requires reader-first rollout, legacy exclusion and rollback that cannot reinterpret new history as whole-value LWW. The evaluation does not infer preference-map capability from `message`, `latest` or the existing text CRDT adapter.

## 3. Risks and next action

The dominant residual risks are downstream proof failures: truly atomic metadata persistence, overlapping admission/retirement execution, authenticated end-to-end origin evidence, stable validity under policy propagation, finite retention with safe rejoin, and activation enforcement across retained versions. Deterministic fixtures cannot establish filesystem durability, real lock custody or cryptographic correctness. The packet distinguishes these obligations from current evidence.

H1’s deliberate stuck state after unavailable or permanently lost home is visible and consistent with its proposed availability profile. Multiwriter’s possible outage acceptance is a separately gated appearance exception. Neither recommendation silently authorizes H3, broader settings mutation or automatic restoration.

GDL-050 lines 73–83 correctly records design/evaluation authorization while reserving implementation, canonical replacement and deployment. H1 §8 and Multiwriter §7 identify the affected canonical contracts and require explicit amendments before activation. I found no dangerous silent amendment or incompatible recommendation.

The next action is to assemble the narrowly scoped contract/proof object: H1 creation/admission recovery traces and compiling consumer witnesses, plus a bounded appearance baseline corpus and recorded conditional profile decisions. Preserve the activation prohibition until the named amendments, real adapter evidence and subsequent reviews pass.
