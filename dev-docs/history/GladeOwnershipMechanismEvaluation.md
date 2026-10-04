# Glade ownership mechanisms — decision evaluation

Date: 2026-10-03. Status: **DRAFT analytical evaluation; no mechanism, availability policy, deployment, dependency or activation selected**.

The [controlling plan](GladeOwnershipMechanismEvaluationPlan.md) corrects the interpretation of earlier H1 work: [stable home](GladeStableHomeDesign.md) is an explored candidate. The owner has not accepted home-dependent outages. This document compares M0–M5 under that unresolved choice. Its protocols are analytical proposals, not formal proofs, executed models, measurements or production adapter evidence.

## 1. Evidence and the question being decided

Local evidence was read through `git show`: root `4785cf516d24dab2d49c3ff79dc517a839dc2f45`, Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`. The uncommitted controlling plan supplies this task's mandate. Member working changes, cold-join fixes, launchers and scratch reproductions supply no implementation evidence here.

The [build entry](GladeBuildEntry.md) requires architecture allocation and consumer/conformance witnesses before implementation. [Alternatives](GladeResourceHomeAlternatives.md) §3 supplies RH-01–12; [comparison](GladeResourceHomeComparison.md) §1–6 supplies conditional availability choices. [Authz](glade/GladeAuthzModel.md) §1/3a/3b/4/4a/7a distinguishes creation-rooted authority, signed governance, local permission, private principal binding and operator-approved placement. [WorkspaceDirectory](glade/GladeWorkspaceDirectory.md) §3/4 distinguishes genesis/join and a physical-copy lock. [DiscoveryModel](glade/GladeDiscoveryModel.md) §0/3/5 defines replicated local routing. None makes independent copies mutually exclusive merely by ranking claims.

The pinned [substrate](../glade/dev-docs/GladeSubstrateV1.md) §6 R1/R2/R7 and W1–W8 limits current receipts: application `Ok` covers unsynced local storage surviving process crash; forwarded `Ok` adds holder and forwarder copies, without an independent machine-loss commitment. [CrossNodeWrites](../glade/dev-docs/GladeCrossNodeWritesPlan.md) §2/3/7 rejects independent admission that can acknowledge operations the holder refuses. Equal-epoch ranking agrees only for equal eligible observations; it is not ownership acquisition. [RegistryContractDraft](../glade-discover/dev-docs/RegistryContractDraft.md), “Atomic acceptance and recovery” and “Registry, trust and placement”, provides local exact-retry contracts, not the whole resource transaction or fencing implementation.

Six questions MUST remain separate:

| Layer | Decision required |
| --- | --- |
| Identity and namespace | Which canonical scope/name identifies one resource? Who serializes creation in a known scope? |
| Eligibility and placement | Which authorized operators may hold data and serve? Which policy chooses among them? |
| Acquisition and transition | What durable event grants one generation and excludes its predecessor? |
| Coordination leadership | Which process proposes commands for a particular group? |
| Store/effect enforcement | Where is a stale or duplicate operation rejected atomically with the real mutation? |
| Data and acknowledgement | Which complete application history exists on which independent durable copies before success? |

A coordination leader can manage many resources whose homes differ. Changing that leader need not change any home. A home election neither issues grants nor establishes missing data. Discovery MAY continue as a local signed-record projection in every candidate.

## 2. Shared assumptions, records and receipts

Baseline faults are authorized honest operators, process crash/restart, lost/duplicate/reordered messages, partitions, delayed effects and uncertain recovery. Network delay is unbounded. Progress requires the candidate's available participants and eventual useful communication. A timeout MUST NOT prove death. Crash-fault consensus does not solve copied keys, Byzantine voters or a root signing contradictory genesis.

The comparison initially places whole shares, retaining zones `(share, glade_id, key)` within them; an explicit future shard profile MAY differ. No per-key election or manual per-resource mapping is presumed. New independent genesis uses a fresh identity and authorized root intent; equal display labels remain different roots. Joining a known scope MUST recover its existing authority/configuration, never create a new group because local discovery is empty.

Common semantic records, not wire definitions, are: canonical scope/resource/incarnation; signed creator/admin intent; eligible host; durable request ID plus exact payload digest; active home/generation; configuration identity/revision; declaration/profile; application frontier; terminal operation outcomes; retirement fence. A reused request ID with changed bytes MUST fail. Exact retries MUST recover the original outcome, subject to present disclosure permission, before stale-revision rejection. Cancellation or lost reply means unknown until recovery. Capacity exhaustion MUST refuse explicitly; history needed for deduplication, configuration or retirement MUST NOT disappear through time-based eviction.

Receipt classes are **binding committed**, **command durably committed**, **application mutation accepted**, **effect completed**, and **pending/unknown/conflict/denied/unavailable**. Their names are explanatory, not proposed status codes. Metadata or command success MUST NOT be translated into current application `Ok` unless its actual storage guarantee is met. An effect whose sink cannot deduplicate or determine an interrupted outcome remains unknown; retry is not permission to repeat it.

Permission checks remain local under Authz §4's forward-only revocation. Each enforcement boundary MUST validate its available complete applicable policy, requester, hosting permission, active declaration and incarnation. Unknown proof fails closed. An unseen remote revocation is not instantly known. Stronger freshness requires a separately selected policy stream/barrier and availability cost; an ownership group MUST NOT be described as a live grant-decision oracle by default.

## 3. Concrete candidate protocols and counterexamples

### M0 — discovery-only deterministic winner

Authorized nodes publish signed claims and each folds its local eligible set, filters by its local clock and chooses a deterministic rank. Client operations enter the locally chosen store. A create reply can commit a local namespace entry but cannot reserve a known-scope name across partitions. There is no global acquisition commit or actual cross-copy fence.

Worked EM-04: A and B initially know A; partition; A renews unseen by B; B times out A and publishes B at a higher epoch. A's store sees A, B's sees B. Both accept the same exclusive mutation. Reunion selects one routing answer but cannot retract either effect or truthful past acknowledgement. Even identical records can give different live sets under different evaluation times. M0 is **ineligible** for exclusive admission across every EM journey: it supplies no valid exclusive-acquisition or exclusive-effect receipt. A common checkout lock would supply the missing authority, changing the mechanism to a resource arbiter; mergeable preferences have a different contract.

### M1 — stable home with a durable creation gate

For fresh genesis, an authorized creator fixes identity, root, declaration and eligible home in one signed intent; the creator MUST durably retain that exact intent before transmission. A crash-model custody assumption prohibits submitting contradictory genesis or concurrently running cloned home credentials/journals. For a known scope, requests reach its already bound home and namespace gate.

The home atomically commits name/binding, exact receipt, declaration/incarnation and advertisement outbox. Its metadata adapter MUST provide crash/power-loss recovery on retained healthy media; this is a proposed guarantee, stronger than current app storage. Different requests for one canonical name conflict; exact retry returns the original binding. Publication follows commit. No reply/publication is itself the commit.

The home exclusively owns its journal and served store under a real lifecycle/storage lock. Every append passes the immutable binding, policy, declaration and chain checks atomically with local commit. A queued external effect MUST check at its actual sink/checkout boundary; a process lock alone does not stop already dispatched work. Restart MUST establish exclusive custody and resolve outstanding effects before admitting conflicting work. Where that cannot be demonstrated, the affected effect stays blocked. No other node may become home for that identity.

Worked EM-02: A commits intent X and crashes before replying. B has an empty directory and receives retry X. B returns unknown/unavailable or relays to A; it MUST NOT initialize S. A reopens its ledger, returns the exact receipt and republishes once. If A's media is permanently missing, S stays loss/unavailable. M1 permits home service during a remote outage when local permission evidence suffices, but no failover. Its advantages are resource-local coordination and a small transition grammar; its costs are home dependence, custody/recovery discipline and unavailable known-scope creation.

### M2 — cooperative transfer with a durable irreversible cut

Creation follows M1. The old home A serializes signed transfer intent T under its current generation g and records `Preparing(T,B)`. It closes new admission, drains admitted operations and obtains terminal outcomes for dispatched effects. It commits a cut C covering every operation promised by its selected acknowledgement guarantee, with hashes/frontiers, declarations, policy and exact retry/outcome history.

B stores and verifies C under its operator permission, records `Prepared(T,C)`, and remains inactive. A verifies that preparation, then durably commits `Fenced(T,g,C,B,g+1)` while its enforcement gate is closed. That fence is irreversible and survives restart. A's exact certificate permits B to commit `Active(T,g+1,C)`; only then may B accept new work and advertise. The successor fence, baseline data and activation receipt MUST be one recoverable local transition. A cannot revive g if activation's reply is lost. Before fencing, cancellation can return to g only through a recorded abort and exclusion of prepared B; after fencing, recovery completes T or remains unavailable.

Worked EM-06: B prepares; A commits its fence; the certificate reply is lost and A crashes. B remains inactive without recoverable fence evidence. On A's return, lookup T recovers the original certificate; B activates exactly once. A's stale queued effect arriving afterward MUST have been terminal before C or rejected by its sink's irreversible fence. If that sink cannot prove either, transfer never reaches the fencing step. Independent stores each accepting their own highest counter are insufficient.

M2's no-overlap argument depends on actual old-side quiescence and durable custody, not transfer messages alone. Permanent loss or partition of the required old side blocks movement, even with an administrative request. Its benefit is planned identity-preserving maintenance without steady-state quorum traffic; its cost is recovery pauses and a multi-boundary transaction that cannot promise automatic takeover.

### M3 — consensus orders every protected mutation

An authorized, durably configured scope/shard group commits `Create`, `Mutate`, `Retire`, `BeginMove` and `Activate` commands with resource, generation, requester, exact intent and complete required payload. State-machine application validates declaration, generation and complete canonical policy inputs in order and retains exact outcomes; it MUST produce the same decision at each replica, independent of local discovery/time. A group leader proposes this order; home selection is an application command, not that leader's election. Activation requires a verified complete successor application frontier, including declarations, policy and retry outcomes, under the selected acknowledgement guarantee.

**Profile M3-D:** the consensus state machine contains the actual authoritative application log/data, not just ownership metadata. An accepted mutation commits its canonical op and outcome under the selected durable quorum policy; application advances a recoverable cursor. Local node stores are ordered projections of that committed history. They MUST NOT admit independent authoritative writes. Old replicas may apply old committed entries to catch up, but cannot report a new authoritative acceptance without a new committed command. Reply requires the stated data commit/application guarantee; replay of an already committed intent does not append another op.

**Profile M3-E:** external effects additionally require one enforceable sink contract. Commands carry per-resource sequence, generation and operation ID. The sink atomically checks its active generation/next sequence, validates permission, performs or durably records the effect and saves its outcome. `BeginMove` closes generation g's command admission at cut k; recovery resolves every committed command through k at the sink. The sink then atomically records g fenced and the successor generation/baseline, deduplicating retries of this barrier. Group `Activate(g+1,k)` follows a verified sink barrier and successor data readiness. Later commands cannot overtake unresolved earlier effects. Lost sink replies recover by exact outcome query; arbitrary external actions lacking this transaction/deduplication contract remain blocked/unknown. This profile shares M5's sink prerequisite; consensus does not manufacture it.

Worked EM-05: command q in g commits; its dispatch stalls; `BeginMove` commits at k. A new proposer cannot activate B merely because the ownership command committed. It first recovers q's data/effect outcome, installs the sink barrier, verifies C and commits activation. A delayed q then returns the saved outcome or is rejected under the sink's sequence/generation rule; it cannot perform a new effect after the barrier. If q is an opaque shell action with no recoverable terminal outcome, activation waits. M3-D's replicas instead replay q before the cut and acknowledge no conflicting local new command.

A quorum preflight—“am I still home?” followed by a local append—is **not M3**. A pauses between check and append; a majority activates B; A resumes its old append. That is TOCTOU even if the check was linearizable. Likewise a committed grant token checked against independent local counters is not a fence. Metadata-only M3 with unfenced stores/effects is ineligible. M3 offers timing-independent serialized data decisions and majority progress; its costs are per-operation coordination, application integration, durable log/snapshot recovery and the external-effect limitations above.

### M4 — consensus-granted bounded authority lease

The configured group commits an exclusive grant `(S,g,A,session,grant-ID,horizon)` and its exact renewal history. Every renewal is a new intent; exact retry cannot extend its original horizon. The group MUST never issue overlapping authority intervals and MUST recover outstanding promises before issuing a successor. New leadership or restart does not reset the lease history.

Two enforcement profiles need separate evidence. **M4-L**, for genuinely independent local stores, requires a conservatively derived client deadline from a documented issuer/client relative clock-rate bound, clocks that account for suspension, and a proved bound on admission-to-terminal-effect duration/drain. Local admission MUST stop early enough that every admitted effect is finished or irreversibly prevented before the authority interval ends. Scheduler pauses, VM suspension, queued I/O and remote effect execution MUST be covered; a timer callback or “recheck before send” is insufficient. Restart invalidates the old session and restores no authority from a persisted local deadline. The group waits through its safe outstanding horizon before reacquisition; the new holder also proves data readiness.

**M4-S** instead requires the actual shared sink to atomically enforce generation/expiry at mutation, prevent an old admitted operation from completing across a successor barrier, and deduplicate retries. Its safety depends on that sink's lifecycle/time semantics; it approaches M5, with a lease optimization rather than independent-copy fencing.

Worked EM-09: A validates a lease, pauses after validation, and leaves a write queued; the group expires g and grants B g+1. A resumes and the queued write reaches its store/effect. Without M4-L's demonstrated terminal-effect bound/exclusion or M4-S's sink rejection, both generations act. Merely promising that A will stop, waiting a guessed interval, or observing a discovery lease lapse fails this trace. A delayed renewal reply cannot resurrect an expired session. Existing valid lease service may continue without quorum until its conservative deadline; new acquisition/renewal needs quorum. No lease duration is selected. **M4-L remains blocked as a Glade deployment proposal until its platform and effect-boundary assumptions have evidence.** Its potential benefit is removing quorum from ordinary writes; its cost is stronger environment assumptions, failover waiting, drain proof and uncertain external effects.

### M5 — one common authoritative resource arbiter

All authoritative paths reach one logical sink that owns the resource data and atomic admission ledger. The sink serializes canonical create/name conflicts, `Acquire`/`Move`/`Retire`, and operations. Each operation carries incarnation, generation, exact ID/digest and authorized requester; one transaction checks current sink policy/declaration/generation, mutates actual data and retains its outcome. Atomic acquisition changes the active generation and returns the exact receipt. A highest-generation cache on separate Glade stores does not satisfy this protocol.

Worked EM-05: A's g operation is delayed; B's generation transition commits at the sink. If the old operation committed first, it appears in the sink's cut/outcome and B cannot lose it. If it arrives after the transition, it is rejected without mutation. A lost operation reply returns the original outcome. For asynchronous external actions, the sink MUST resolve/drain them before transition or offer equivalent irrevocable exclusion; an atomic metadata row plus an uncontrolled action queue is insufficient.

A successor loads a complete verified cut from the sink before serving state-dependent operations. The sink's transaction/durability contract determines whether committed data survives failure. It MAY be a separately replicated service, whose voting domains, recovery and acknowledgement costs count in deployment. An unreplicated sink fails closed when lost and may lose data. M5's benefit is a direct common enforcement point and thin Glade clients; its costs are runtime dependence, shared failure/load domain, plaintext/operator approval and the sink's own consensus/storage work. It does not grant broad offline exclusive progress.

## 4. Algorithm and service choice, without selecting an implementation

Primary precedent is narrower than Glade's proposal. Burrows describes a replicated Chubby cell, client advisory locks and sequencers checked by receiving resources; locks alone do not stop delayed requests. Session lease uncertainty requires conservative client handling, and lock-delay is an imperfect fallback. This supports separating coordination from effect enforcement, not adopting Chubby's filesystem/session API or deployment. [Chubby, §§2.1/2.2/2.4/2.6/2.8/2.9, proceedings pp.335–350](https://static.usenix.org/events/osdi06/tech/full_papers/burrows/burrows_html/).

| Algorithm | Primary-source mechanism | Glade-specific evaluation |
| --- | --- | --- |
| Raft | Leader election, ordered replicated log and leader-completeness rules; joint configuration requires both majorities; client serials/outcomes prevent repeated application; linearizable reads need additional safeguards. | A coherent log model suits ordered creation/mutation/cuts. It still requires durable adapter contracts, authorized membership, payload completeness and external-effect enforcement. A term is not a resource generation or grant. |
| Paxos/Multi-Paxos | Durable acceptor promises/accepted values constrain later proposals. A distinguished proposer improves progress; agreement does not require unique leader belief. Repeated slots form a state machine; a stable proposer amortizes preparation and recovery fills gaps. | A valid alternative for the same order. A chosen slot is not application success. Ballot/slot persistence, prefix application, deduplication and a complete reconfiguration design remain explicit engineering obligations. |

Algorithm summaries derive from [Ongaro/Ousterhout, extended Raft paper §§5.2–5.4/6/8, pp.5–10/13](https://raft.github.io/raft.pdf) and [Lamport, Paxos Made Simple §§2.2–2.5/3, printed pp.2–10](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf). Both address crash-fault agreement; neither supplies Glade's authority, data/effect contract or deployment policy. Raft's log presentation gives a conditional design-clarity preference for M3; Paxos/Multi-Paxos remains equally eligible if its complete protocol/adapters meet the same obligations. No throughput, superiority, delivery-time or crate/version recommendation is inferred.

| Packaging | Benefits | Costs and prerequisites |
| --- | --- | --- |
| Embedded scope/shard groups | Can place M3-D application commands and authority in one order; no separate generic service API; failure domains can follow resource shards. | Every group needs durable bootstrap/configuration, quorum, catch-up, bounded group lifecycle and authorized reconfiguration. Two Glade nodes remain two voters. Unbounded one-group-per-resource creation is not a scalability answer. |
| Separate coordination service | Independent electorate can continue while one of two Glade data nodes is down; reusable acquisition/outcome service. | Operated service, trust, upgrades, backup and independent voters become runtime requirements. Metadata service cannot certify absent application data or fence independent stores. M3-E/M4-S still need the sink; M4-L still needs its time/effect proof. |

For M3/M4, scope genesis MUST bind exactly one authorized configuration/group and resource-to-group mapping. Known-scope joins authenticate that binding and catch up as nonvoters before admission. Configuration changes MUST use a protocol with intersecting decision authority across transition; independently replacing “majority of current discoveries” can create disjoint quorums. Lost change replies recover the original configuration intent. Lost quorum cannot be repaired by unilateral membership reset. Same-scope purported groups or rollback evidence mean conflict/quarantine, not election by label. M1/M2 have analogous immutable-home/transfer custody; M5 delegates this to its sink, which must state it.

All three primary sources above were accessible on 2026-10-03. They are design publications, not current implementation audits. No inaccessible external evidence is used; the external Gyld declaration/capture linked by BuildEntry was not evaluated as an ownership proof. No verbatim paper passages are required. Algorithm precedent and the clearly labeled Glade protocols have different evidentiary status.

## 5. Deployment matrix

P1 is one node; P2 two independent authority/data nodes; P3 two data nodes plus an independent placement-only voter; P4 three independent authority/data nodes; P5 externally operated common coordination/sink. Two loopback processes demonstrate neither machine-loss independence nor storage independence. The rows below separate new ownership from continuing service and data acknowledgement.

| Candidate | P1 | P2 | P3 | P4 | P5 |
| --- | --- | --- | --- | --- | --- |
| M0 | One visible node masks the unsafe mechanism; known-scope join still unresolved. | Either side may claim itself during partition: ineligible. | Voter unused: still unsafe. | More observations do not fence: unsafe. | Common enforcement would change the family. |
| M1 | Fresh genesis possible; known scope needs its bound home. | Non-home loss can leave service running; home loss blocks; reunion retries same identity. | Witness supplies no replacement; home dependence remains. | Extra data copies improve retention only; no takeover. | External discovery/backup can help, but stable home still required. |
| M2 | Transfer to a new node needs old cooperation. | Planned transfer possible; missing required old side blocks; partition never authorizes takeover. | Witness does not replace old fence/cut. | Same cooperative requirement; extra copies aid verified cut. | A sink may fence transfer, making sink guarantees part of the profile. |
| M3 | Singleton order works while present; no failure tolerance. | Majority needs both; either loss/partition blocks new commands and authority. | Two surviving authority voters can commit, but witness holds no app payload. | Majority can progress and minority cannot accept; M3-D needs a data-bearing durable majority. | Service quorum can order commands despite Glade-node loss; data/effect sink or M3-D integration still required. |
| M4 | Grant while authority exists; crash/restart reacquisition obeys horizons. | Existing valid lease may serve briefly; new grant/renewal needs both; no survivor promotion. | Authority majority can renew/regrant after safe horizon; missing data still blocks activation. | Majority acquisition possible after exclusion; isolated holder only within proven valid interval. | Service may supply independent quorum; lease enforcement and data remain separate. |
| M5 | Local sink works; sink loss stops service. | Both clients can use common reachable sink; independent local sinks do not qualify. | Placement witness supplies neither common mutation nor data. | Replicated sink can progress with its declared quorum/data policy. | Natural profile: count sink domains, durability, permissions and outage dependence. |

P3 is decisive: if A alone acknowledged x and fails, B plus witness cannot honestly activate a complete successor. To preserve acknowledged x against either data-node loss, a chosen policy could require both A and B durably hold x before receipt. After one loss, that same two-copy policy cannot acknowledge new operations until replacement; continuing with one copy is a separately approved reduced guarantee. For M3-D, a metadata-only witness MUST NOT count as the second full application replica. A voter that stores complete command payloads is data-bearing and changes P3's stated profile. P4 also requires a data commit rule: three metadata voters plus asynchronous app copies do not establish it.

## 6. Common analytical journeys and failure closures

Notation: S is a known canonical scope, A generation g, B possible successor, x an exact intent/operation, C a verified data/effect cut. The events below are shared across families; each result cell inherits its concrete commit/fence/recovery protocol from §3. `Blocked` is an honest result. These are analytical schedules, not passing tests.

| ID | Initial state and event order | Required future closure test |
| --- | --- | --- |
| EM-01 | S exists; two clients create the same canonical name concurrently. Separately, two creators deliberately mint independent same-label roots. | Overlapping create race, one binding/conflict; independent identities; unauthorized creator denied. |
| EM-02 | x commits; reply/publication lost; requester/home restart; local lookup empty; exact x retried. | Recover exact intent/outcome, reject changed retry, no empty-store genesis. |
| EM-03 | A serves; A or non-home fails; survivor requests authority in P1–P5. | Each deployment schedule asserts promised progress or blocked status, independent data check. |
| EM-04 | Partition; stale/delayed claims; A/B attempt writes and renewal; reunion. | No two authoritative paths; pending differs from accepted; duplicate/reordered replay. |
| EM-05 | Transition starts; old queued x delays; B activates; A returns/x reaches sink. | Actual store/effect rejects stale generation or returns saved pre-cut outcome; no duplicate action. |
| EM-06 | Crash/lost reply before/after each create, quiesce, prepare, fence, grant, sink and activation commit. | Enumerate every durable boundary and exact recovery; unknown never becomes abort. |
| EM-07 | A acknowledged x; B lacks x; witness/quorum has only metadata; A lost. | Activation blocked; missing history is not empty; test selected independent data receipt. |
| EM-08 | Fresh genesis versus known-scope join; overlapping purported groups; membership change reply lost. | Authenticate one configuration; reject second known-scope genesis; catch-up and safe transition retry. |
| EM-09 | Check/lease succeeds; process pauses/drifts/restarts; renewal delayed; effect queued beyond expiry. | Fail preflight mutant; prove sink exclusion or all declared lease/drain assumptions with adapter faults. |
| EM-10 | Hosting revoked/access denied/retired; old create, append, renewal and effect replay. | Local current policy denies; retirement ordered with effects; unseen revocation freshness stated. |
| EM-11 | Media destroyed; stale backup/cloned/rollback journal restored; completeness uncertain. | Quarantine/no reset; recover from verified committed history only; copied custody fails. |
| EM-12 | Legacy writer overlaps proposed activation; restart; rollback; delayed legacy operation. | Actual enforcement rejects legacy path; baseline cut complete; rollback keeps fence. |

| Journey | M0 | M1 | M2 | M3 | M4 | M5 |
| --- | --- | --- | --- | --- | --- | --- |
| EM-01 | Local winners can duplicate known name. | Home transaction chooses one. | Same creation gate. | Ordered Create chooses one. | Group Create chooses one; lease only follows. | Sink transaction chooses one. |
| EM-02 | Local retry cannot resolve other copies. | Original home ledger/outbox. | Same; transfer T lookup. | Committed intent/outcome recovery. | Grant lookup; retry never renews. | Sink exact outcome recovery. |
| EM-03 | Unsafe survivor promotion. | No takeover. | Old cooperation required. | Quorum plus §3 data/effect gates; §5 limits. | Quorum, safe horizon/exclusion, data. | Reachable sink and complete cut. |
| EM-04 | Both sides accept. | Bound home only; permission limits. | Transfer waits for cooperation. | Only committed commands; minority pending. | Valid lease service only; new grant quorum. | Reachable sink serializes. |
| EM-05 | Reunion cannot undo effects. | No successor; restart drains/resolves. | Durable old fence before activation. | Ordered cut and sink barrier/application. | Expiry alone insufficient; §3 proof required. | Atomic sink transition rejects stale x. |
| EM-06 | No common recovery grammar. | Atomic create/reopen; unknown query. | Before fence abort possible; after fence complete/wait. | Log plus applied/sink outcome recovery. | Durable promise; unknown grant waits/query. | Atomic transaction and exact outcome. |
| EM-07 | Empty promotion loses x. | Loss/unavailable. | No verified C: blocked. | Metadata quorum insufficient: blocked. | Lease grant insufficient: blocked. | Sink history required; missing: blocked. |
| EM-08 | Discovery cannot bootstrap authority. | Signed genesis/home custody; no reset. | Same plus unique T chain. | Authorized config; transition quorums. | Same plus outstanding horizons. | Sink namespace/config authority. |
| EM-09 | Clock filters cannot fence. | No ownership expiry; execution gate. | Drain before C; uncertain effect blocks. | Preflight fails; committed order enforced. | Unproved pause/effect profile blocked. | Sink checks atomically at mutation. |
| EM-10 | Claim validity not effect authority. | Local gate and retained retirement. | Gate before cut and activation. | Ordered local policy/retirement; remote lag honest. | Local permission plus lease; no revocation exemption. | Sink policy/retirement; no assumed global freshness. |
| EM-11 | Higher epoch hides uncertainty. | Restore completeness/custody or blocked. | Recover cut/fence evidence or blocked. | Recover committed prefix/config; no unilateral reset. | Never reuse session; recover promises/data. | Sink recovery guarantee or loss. |
| EM-12 | Legacy remains unfenced. | All legacy paths excluded before binding. | Exclude legacy before transfer generation. | No out-of-log writer; sink rejects bypass. | Legacy cannot bypass lease/sink gate. | Every writer must reach sink gate. |

For EM-10 every eligible family distinguishes local policy validation from instantaneous remote revocation; a lease is not a permission exemption. For EM-11 backups need verified completeness, retry/fence history and exclusive custody, not just a familiar identity. For EM-12 an offline legacy writer that can still mutate blocks activation. A rollout flag, new advertisement or reader upgrade does not close the old path. Rollback MUST retain the enforcement boundary; unknown legacy splits MUST be reconciled or reported before any preserved-history receipt.

## 7. Canonical impact and conditional choice

No candidate amends existing sources by being described here.

| Exact controlling clauses | Required reconciliation before activation |
| --- | --- |
| [Buy/build](GladeBuyBuildMatrix.md) §1 D-06; §2.B R7 | Discovery remains a signed local fold. Any new live ownership dependency MUST be explicitly qualified as a separate resource authority. Replacing resolution with Chubby/etcd-style service violates the present boundary. |
| Buy/build §2.B R9; §4 Q12 | Deferred shard coordination is an opening for reviewed reconfiguration, not approval for per-resource consensus or a new runtime service. M3/M4 require owner-reviewed scope and deployment amendments. |
| Buy/build §2.D R16; WorkspaceDirectory §4 and WD-8 | Local locks fence a physical copy only. M1/M2 must state custody; M3/M4/M5 need explicit added cross-copy/effect authority. R16 currently rejects distributed lock services under D-06; adopting one requires amendment, not relabeling. WD-8's advertisement/repair ruling does not itself grant exclusive acquisition. |
| DiscoveryModel §3/7 | Higher epoch routing/takeover scenarios MUST be qualified per activated profile: routing claims cannot activate a stable binding, consensus grant or sink generation. Preserve unmodified legacy semantics until migration. |
| Substrate §2 and §6 W1/W2/W7; CrossNodeWrites §3 | Replace live-claim/unclaimed-local admission only through explicit profile amendments. Stable declaration, home/generation and selected arbitration MUST precede authoritative acceptance; legacy fallback is excluded for activated identities. |
| Substrate §6 R1/R2/R7/W3/W4/W5/W6 | Define new receipt mappings, unknown/exact retry, cuts and order. Strong metadata/consensus/sink durability MUST NOT silently upgrade application `Ok`; data-copy policy needs its own amendment and consumer tests. |
| Authz §1/3a/3b/4/4a/7a | Preserve signed governance, root authority, local read checks, self derivation and placement approval. Acquisition eligibility does not issue grants. Stronger permission freshness, if required, needs a named exception/profile. |
| RegistryContractDraft, atomic acceptance/placement | Reuse exact-retry principles; independent ports cannot compose into an assumed atomic resource transaction. Frozen protocol bytes are not implicitly changed. |

Rankings follow invariant eligibility first, then required availability/data profile, operational prerequisites, recovery/migration complexity, and only then qualitative cost. There are no weighted numeric scores.

* If home outages may leave authoritative edits pending and movement is optional: **M1 first, M2 second**. M3/M5 add unrequired operational authority; M4 adds unproved time/effect assumptions. This condition is pending owner acceptance.
* If planned identity-preserving movement is mandatory and old-side cooperation is available: **M2 first**; M1 is ineligible. M3 or M5 becomes preferable only if the chosen deployment already needs their stronger authority. M2 cannot be promoted into lost-home takeover.
* If automatic identity-preserving takeover with all acknowledged data is mandatory: **M3-D first for self-operated independent replicated data**, or **M5 first when an approved common sink already owns actual mutation/data**. M3-E is eligible only with its sink/effect recovery contract. M4 is a later optimization only after its extra proof obligations close. M1/M2 are ineligible for that requirement.
* If exactly two independent voters and no additional authority must tolerate either loss while preserving exclusive writes: **none qualifies**. P3 can repair authority availability but not missing data or continued two-copy acknowledgement. P4/P5 are conditional deployment choices, not assumed owner preferences.

[Multiwriter settings](GladeMultiwriterSettingsEvaluation.md) remains a separate option for bounded mergeable appearance preferences under an explicit conflict/authorization/data profile. It is not an exclusive-ownership family and cannot replace grants, membership, physical-copy or external-effect authority.

Smallest owner decisions: (1) whether home-dependent pending edits are acceptable, and whether cooperative movement or automatic takeover is required; (2) which P1–P5 authority/data failure domains and post-failure acknowledgement guarantees are acceptable; (3) whether an external common service/sink is acceptable, and which actual effects must survive uncertain recovery. The following contract tranche must then settle canonical genesis/name authority, permission freshness, exact receipts, retirement, bounds, membership and activation. Lease duration and implementation choice come after those contracts and evidence.

Future delivery MUST begin with failing EM-01–12 conformance schedules covering success, failure and edges, then the smallest implementation. Pure rules inject time/randomness/delivery; concrete adapters prove actual atomic storage, crash/power-loss recovery, cryptography, overlapping admission, suspension/effect exclusion and sink outcomes. [Library policy](LibraryBoundaryAndTestingPolicy.md) LBT-001–012 and [package architecture](GladePackageArchitecture.md) govern role allocation, narrow replacement contracts, fast checks and affected consumers. The adopting discovery repository must run `./scripts/check-architecture.sh`; no whole-workspace replacement or allowlist relaxation is implied. Code must obey explicit control-flow/conditional-compilation boundaries and process-global checks in all affected platform branches.

Verification here is source inspection, primary-paper access and analytical tracing only. No implementation, dependency, public interface, executable model, test selection, migration or runtime activity was performed. Review acceptance can establish this packet's fitness for an owner decision; it cannot select a mechanism or establish its operational safety.
