# Glade Discovery Model — the routing spine, reified as `glade-discover`

Status: design (2026-07-12). The security / topology / advertisement model for
glade discovery, and the proposal to REIFY it as **`glade-discover`** — a
base-glade spine component behind a swappable **comms-substrate seam**, so the
whole routing kernel (and every failure case) is testable against a deterministic
network simulator. Design-only; Gianni's rulings are marked **OPEN**. Synthesises
+ extends: `GladeWorkspaceDirectory.md` (WD-8 advertisement vocab; WD-6 entry-node
locality), `GladeP2PFirstTopology.md`, `GladeProviderPlacement.md`, B1/B5 (the
substrate security ruling, `GladeSupplierModel.md` §8 / `GladeAuthzModel.md`
§3b/§9), `GladeAuthzModel.md` §1 (read/write split), glade-diff (demand
instantiation), E-users-1 (owner = root-key fingerprint), E-ws-1 (stable-ID
authority).

## 0. Thesis — discovery is the spine, so make it the testable kernel

Every supplier is *found* through discovery and every consumer *routes* through
it; nothing in glade works if discovery is wrong. Two consequences drive this
doc:

1. **Discovery is a fold, not a service** (the north-star point): "who serves X?"
   is answered by folding the replicated home share (`registry.rs::who_serves`
   folds `dir.claims`), local-first, never by querying a registry. There is no
   consensus quorum on the critical path (WD-8 rejected the Paxos cell for
   replication + rendezvous).
2. **The hard part is the failure behaviour** — spoofed claims, stale leases,
   partitions, slow links, precedence conflicts, teardown races. The trace atlas
   (ggg-viz) proves routing/authz STRUCTURE well but **cannot derive
   computation** (the AtlasGladeReview-56s finding). Convergence and failure
   handling are computation. So the right home for them is an **executable
   discovery kernel run against a simulated comms substrate** — not a narrated
   trace. **Get this kernel stable and exhaustively failure-tested, and the rest
   of the system is already well-defined** (Gianni), because every supplier is
   then just "advertise" and every consumer just "route."

## 1. `glade-discover` — what it is (and isn't)

**It is the routing kernel of base glade, reified as a named, independently
testable component.** It owns: folding the home-share directory; verifying
advertisement authority (§4); most-specific-match routing (§6); rendezvous /
peer selection (§5); lease/epoch lifecycle. **RULED (Gianni 2026-07-12):
`glade-discover` is a new gwz **member repo** in glade-wz** — its own repo,
built + tested with the same typed-protocol, test-first discipline as the gwz
family. It is **spine / base-glade substrate, not a workspace-tool supplier**
(it does not operate "on a selected workspace"; every other supplier depends on
it). Still OPEN (§9): whether it ALSO exposes a thin observability supplier
surface (routing table, claim health) the way glade-workspaces surfaces
`dir.workspaces` — the kernel is the deliverable either way.

## 2. The kernel boundary — taut messages + a clock (NO transport substrate)

**RULED reframe (Gianni 2026-07-12): there is no `CommsSubstrate`/transport
interface.** An earlier draft folded send/resolve/dial into a substrate the
kernel depends on; that leaked transport into discovery. The kernel's entire
boundary is **taut messages + a clock**:

- **The kernel is a pure state machine:** `(inbound taut msg | timer wakeup) →
  (outbound ADDRESSED taut msgs, new wakeups)`. Zero I/O — no dial, no resolve,
  no bytes. It NAMES a destination node-id in an outbound message; whether and
  when that message arrives is the ENVIRONMENT's business, never the kernel's.
- **Transport is not discovery's concern.** resolve / dial / QUIC belong to the
  node layer; discovery speaks records + messages (the glade discipline), so its
  test boundary IS the taut protocol that already exists — there is no wire to
  emulate. We do not simulate byte delivery; we schedule taut-message events.
- **The one dependency that stays is the clock** (§2.1): `now()` +
  `notify_at(t_abs)`. Leases / retries / claim-GC are intrinsic to discovery's
  OWN logic (not the environment's), so the clock is part of the kernel's
  contract; transport is not.

This is stronger than mock→real: a pure state machine has **nothing to mock**.
In production the real node's message loop drives it (deliveries in, addressed
messages out) over a monotonic clock; in test the scenario harness (§2.2) drives
it. Same kernel, zero transport code to differ. "Simulate other clients" =
scripting their taut messages (a client subscribing IS a taut SUBSCRIBE record)
or running peer kernels in the same harness — never bespoke client code.

### 2.2 Data-driven scenario tests (RULED — Gianni)

Tests are **DATA, not hand-written cases** — the ggg-viz discipline applied to
computation. A discovery test scenario is a typed object:

- **topology** — nodes + links; each link carries latency + **time-windowed
  faults** (loss, partition/heal, reorder, dup, congestion drift);
- **seed** — deterministic ordering + tie-break, for byte-identical replay;
- **inputs** — scripted taut messages at virtual times (a peer publishing a
  ServeClaim, a client SUBSCRIBE, a gossip round);
- **expect** — the converged directory fold PER node, the routing answers, AND
  **the declared failures**: a partition/drop is an EXPECTED outcome written
  into the scenario ("link C→A partitioned t=5..10, so C's claim never reaches
  A ⇒ expect A routes to B"). A "failure" is a confirmed assertion, never a
  test crash.

The harness runs N kernel state machines, owns the §2.1 virtual-time event queue
(deliveries + wakeups, ONE queue), applies the topology/fault schedule to decide
each delivery, and asserts `expect`. Deterministic given (scenario, seed) → a
true oracle. The §7 matrix is the first scenario set. This is the executable home
for the discovery-computation properties the trace atlas cannot derive
(AtlasGladeReview-56s): the atlas is the picture, this is the proof.

The kernel is deterministic given (substrate schedule, signed record set), so the
sim is a true oracle: drive adversarial schedules, assert the routing decision
and the converged directory.

### 2.1 Virtual time (RULED direction — Gianni 2026-07-12)

The kernel's clock dependency is not a `now()` getter; it is a **scheduling
contract** (the one thing that survives the §2 substrate dissolution):

- **API:** `now() -> T` (absolute) + `notify_at(t_abs) -> async wakeup`. The
  kernel asks to be woken at an **absolute** future time — absolute preferred
  over relative sleeps because glade deadlines ARE absolute (`lease_expiry_ms`)
  and relative sleeps compound drift across hops.
- **Kernel discipline:** ALL time-dependent behaviour — lease-expiry action,
  renewal scheduling, retry backoff, gossip cadence, instance-claim GC — rides
  `notify_at`. No sleeps, no polling loops. The kernel becomes a pure
  event-driven state machine: `(inbound frame | wakeup) → (state′, outbound
  frames, new wakeups)` — which is exactly what makes it simulable.
- **The virtual clock IS an ordered event queue.** The sim advances virtual time
  to the head event and fires it; no wall time passes (a 30-day lease-drift
  scenario runs in test milliseconds). Message delivery and timer wakeups are
  ONE queue, two event kinds: `send` = an event at `now + latency(congestion)`;
  `notify_at(t)` = an event at `max(t, now) + d`.
- **Congestion drift — late, never early:** a wakeup requested for `t` fires at
  `t + d`, `d ≥ 0` drawn from the seeded congestion model. Never early (early
  would break causality — code could observe a pre-deadline world as
  post-deadline). Late is realistic and MUST be handled: **correctness under
  late wakeups is a kernel obligation the sim deliberately stresses** (§7) —
  e.g. a renewal wakeup that fires after its own lease already lapsed must not
  resurrect the claim; it re-claims at `epoch+1` and the fence absorbs the gap.
- **Monotonicity (critical):** observed `now()` is **non-decreasing** — never
  backwards; equal timestamps allowed ("or same"). Ties are broken by a stable
  deterministic order (time, then insertion sequence) so the same seed replays
  the identical run.
- **The clock never enters the fold** (the standing no-time-in-fold rule):
  folds stay time-free; `now` enters only (a) projections as a parameter
  (`who_serves(share, now_ms)` — lease filtering at read time) and (b) the
  kernel's ACTION scheduling via `notify_at`. Time drives behaviour, never
  record semantics.
- **Real-impl mapping:** `notify_at` → tokio `sleep_until`/timer wheel over a
  monotonic clock source; the same late-never-early contract holds (a real OS
  also only ever fires timers late), so kernel code proven under the sim's
  drift model is proven for the real substrate.

## 3. The mechanism (recap, grounded)

Binding identity = `(share, glade-id, key-fill)` (`GladeDeclSurface`). The home
share holds `dir.claims` (ServeClaims), `dir.services` (ServiceDefinitions),
`dir.bindings`, `dir.workspaces`, `dir.principals` — an ordinary replicated share
(GDL-038, no privileged plane). Golden path (`s-discovery`): fold local replica →
if the serving node isn't reachable, resolve+dial via the substrate → forward the
keyed subscribe to the claim-holder → it serves ops (value/log) or answers the
directed exchange → a missing live claim / unreachable host is **STATUS data, not
silence** (failure-as-data). The **home node is a role** (durable replica +
relay), not a tier (WD-8).

## 4. Security — owner-rooted serve-grants (question a) — RULED (lean adopted)

**Threat:** node M publishes `ServeClaim{node: M, share: ws-razel}` it is not
authorized to serve; a consumer folds it, dials M, M serves poison or captures
traffic. **A signature alone does NOT stop this** — B5 proves M is a certified
device, not that M may serve `ws-razel` (the s-chat-edit lesson: valid signature
≠ authorization).

**Proposal — an owner-rooted serve-grant chain, verified at fold time:**

1. The share has an **owner** (principal = root-key fingerprint, E-users-1).
2. The owner issues a signed **serve-grant** (`CapabilityGrant`): "M may publish
   ServeClaims for `ws-razel`."
3. M's `ServeClaim` is signed by M **and references that serve-grant**.
4. A consumer routes to M **only if the chain verifies**: ServeClaim signed by M
   ∧ M holds an owner-signed serve-grant on this share ∧ the owner is the share's
   authority. A claim with no valid chain **never enters the routing set** — the
   GQ-9 "convict the origin" posture.

**Blast-radius bounds:** serve ≠ author (`GladeAuthzModel` §1) — a serve-grant
lets M *relay* + *answer subscribes*, not *append*; so for value/log surfaces a
compromised granted node can *withhold/reorder* (a liveness/censorship attack)
but **cannot forge** content (records are B5-signed). For **exchange** surfaces
(gwz), the supplier *computes*, so a compromised granted node *can* lie about
a result — inherent to delegating computation, and the owner's decision when
granting. Equivocation (two conflicting claims for one slot) is cryptographically
convictable. **RULED (Gianni 2026-07-12): the serve-grant chain IS the ServeClaim
admission rule** — a claim routes only with a valid owner-rooted signed
serve-grant, verified at fold time.

## 5. Topology — locality-aware rendezvous (question b) — DEFERRED by design

Premise correction: there is **no chubby/Paxos registry** (WD-8). The directory
is a replicated share folded locally — no consensus bottleneck. But the home
share must still **converge**, and convergence has a topology: one home node as
rendezvous disadvantages a node on a slow/distant link.

**The structural fact that makes this deferrable:** the directory is a **monotone
fold** (set-union of signed records), so **topology is a latency choice, not a
correctness one** — any connected topology converges to the same directory.
Therefore:

- **v1 = flat** (small p2p mesh + the public iroh relay); iroh already gives
  transport-layer locality (lowest-RTT path selection, hole-punching).
- **Multi-level rendezvous rides ON TOP of the same fold, with no data-model
  change:** generalize the home-node role to **locality-aware rendezvous
  clusters** — a node picks its primary rendezvous by proximity (external-IP
  prefix, iroh RTT), fast anti-entropy within a cluster, a thinner set of
  relay/bridge nodes carrying cross-cluster convergence. A gossip overlay with
  locality-biased peer selection (Kademlia/supernode territory). This is exactly
  **WD-6** ("entry-node fleet: nearest? sticky? what the SPA caches").
- Because it is swappable behind the fold, the **sim substrate (§2) is where the
  locality/partition behaviour is proven** before the real overlay is built.

**RULED (Gianni 2026-07-12): DEFERRED — reserve the locality-rendezvous overlay
(folds into WD-6); do NOT build for v1.** The kernel + sim MUST be
topology-parametric so the overlay drops in later. Concretely, "topology-
parametric" is nearly free here because of §2: the kernel is **topology-blind by
construction** — it only ever sees folded records + emits addressed messages, so
it has no wire-topology knowledge to hardcode. Locality then enters LATER as (a)
new record types the kernel folds (ReplicaHint / rendezvous-cluster records) and
(b) a smarter environment (real relay mesh, or a richer sim topology) — **zero
kernel change**. v1 = flat mesh + iroh relay; the sim just runs a flat topology
scenario set until the overlay's scenarios are added.

## 6. Advertisement — most-specific-match (question c) — RULED (lean adopted)

Today routing is per-SHARE (`who_serves(workspace)`) — the coarse "workspace is
the anchor" case. The general model, which makes demand-instantiated suppliers
(diff) fall out as **data, not code**:

- An advertisement is a **match record** `(share, glade-id?, key-pattern?)` +
  its serve-grant chain (§4).
  - a **workspace ServeClaim** matches `(ws-razel, *, *)` — the anchor / least-
    specific catch-all;
  - a **diff `ServiceInstanceClaim`** (WD-8 vocab) matches
    `(svc, ws.diff, {left:(ws-razel,ws.tree), right:(ws-glade,ws.tree)})` — the
    *most* specific: exact glade-id + exact key.
- Routing = fold advertisements → **filter to authorized** (valid serve-grant
  chain §4, AND the consumer holds read on the target — ACL both sides) →
  select **most-specific match** → route.
- **Precedence = specificity, SCOPED per-binding.** The diff instance-claim wins
  *only* for its exact `(svc, ws.diff, {pair})`; it does **not** override the
  workspace host for `(ws-razel, ws.tree, …)`. "Takes precedence over workspace"
  is per-binding longest-match, not a wholesale hijack — so one node running a
  diff never blackholes the workspace.

This gives all four properties Gianni named, by construction: **data-driven** (a
fold + a specificity comparison; "diff is special" is a more-specific *record*,
not code — a new demand-supplier needs zero routing changes); **secure** (the
authority filter runs *before* specificity ranking, so a malicious precise claim
is dropped, not preferred); **flexible** (advertise at any specificity;
diff-becomes-a-local-node = "no match → ServiceDefinition match → instantiate →
publish most-specific claim → later subscribers match it (global dedup) or
recompute (per-node)"); **ACL-respecting** (advertiser serve-grant + consumer
read-grant both gate). **RULED (Gianni 2026-07-12): authorized-most-specific-match
+ per-binding specificity precedence IS the routing rule** (generalises flat
per-share `who_serves`; WD-8 record vocab unchanged).

## 7. The test matrix — failure cases the sim substrate MUST prove

The point of `glade-discover` + the sim is to make ALL of these executable,
seeded, and asserted (convergence + the routing decision), not narrated:

- **Spoofing (§4):** an unauthorized ServeClaim (no/forged serve-grant) never
  routes; a valid-signature-wrong-authority claim (the s-chat-edit shape) is
  refused at fold.
- **Equivocation:** two conflicting claims for one slot → origin convicted, proof
  replicates; routing does not oscillate.
- **Stale claim / lease lapse:** a claim past `lease_expiry_ms` drops from the
  routing set; a subscriber re-routes; no silent stale serve.
- **Epoch fence / takeover:** a higher-epoch claim wins; the stale holder's serve
  bounces (WD-8; s-takeover shape).
- **Partition + heal:** two partitions each fold a consistent directory; on heal
  the set-union converges to one; no lost authorized claim (contrast the LWW
  eligible-hosts bug — OR-set, E-ws-2).
- **Slow link / locality (§5):** convergence latency under an injected slow edge;
  rendezvous-cluster selection routes to the nearest replica.
- **Precedence conflict (§6):** an anchor ServeClaim + a most-specific instance
  claim → the specific one wins for its binding, the anchor still serves the rest.
- **Diff lifecycle:** no-match → instantiate → most-specific claim published →
  second consumer dedups (global) / recomputes (per-node) → interest zero →
  claim lapses → re-instantiate on demand.
- **ACL-denied route:** a consumer lacking read on the target is not routed
  (INV-7 shape) even when a valid serving claim exists.
- **Offline-first:** fold the local replica with the substrate fully partitioned
  — reads that are locally held still resolve; unreachable ones are absence-data.
- **Reorder / dup / drop:** the substrate reorders/duplicates/drops frames; the
  fold still converges (monotone set-union) and routing is stable.
- **Late wakeup / renewal race (§2.1):** a lease-renewal wakeup drifts past its
  own `lease_expiry_ms` → the claim lapses honestly, consumers re-route, and the
  late renewal re-claims at `epoch+1` — it MUST NOT resurrect the dead epoch;
  no consumer ever observes time going backwards.
- **Congestion storm:** many wakeups + deliveries delayed under heavy seeded
  congestion — ordering stays monotone, ties fire in deterministic insertion
  order, and the run replays identically from the same seed.
- **Retry under drift:** a retry scheduled for `t` firing at `t+d` after the
  original answer finally arrived must not double-fire an effect (idempotency
  under late timers — the create state machine's crash-recovery cousin).

## 8. Relation to the atlas + the strategic bet

The ggg-viz trace atlas keeps its role — proving the distributed-systems
STRUCTURE (routing/authz/ceremony shapes) legibly. `glade-discover` + the sim is
the complementary **executable oracle** for the discovery COMPUTATION the atlas
can't derive (AtlasGladeReview-56s). Together: the atlas is the picture, the sim
is the proof. The bet (Gianni): **once this kernel is stable + failure-complete,
the rest is mechanical** — the integration surface that is the program's real
risk collapses to "advertise" + "route."

## 9. Open questions (Gianni's rulings)

RULED 2026-07-12: §4 serve-grant chain (lean adopted) · §6 most-specific-match
(lean adopted) · §5 topology DEFERRED (kernel topology-blind by construction) ·
§2 no transport substrate (kernel = taut messages + clock) · §2.1 virtual-time
semantics · §2.2 data-driven scenarios · glade-discover = a new gwz member repo
in glade-wz. Remaining:

- **Placement/naming:** does `glade-discover` ALSO expose a thin observability
  supplier surface (routing table / claim health) beyond the kernel, or stay
  pure kernel? (The kernel is the deliverable either way; this is additive.)
- **The taut protocol + record shapes** — the buildable spec: the ServeClaim /
  serve-grant / match-record / instance-claim taut messages, and the kernel's
  event-in/event-out signature. → the proper design doc (below).
- **Scenario schema + harness home:** pin the topology/fault/inputs/expect
  scenario shape; is the harness its own crate (`glade-discover-sim`) reusable
  by other components' failure tests? Rust or a lang-neutral scenario format?

**Next artifact:** a proper design-level document (in the `glade-discover` repo's
`dev-docs/` once created) — the taut message/record schemas, the kernel state
machine, and the scenario schema — is now fully specifiable (all model rulings
above are made). Then a phased plan.
