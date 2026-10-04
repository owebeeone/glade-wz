# Review 56 — `GladeDiscoveryDesign.md`

Reviewed: 2026-07-16  
Scope: `GladeDiscoveryDesign.md`, cross-checked against `GladeDiscoveryModel.md`,
`GladeAuthzModel.md`, `GladeRecordEnvelope.md`, `GladeWorkspaceDirectory.md`,
`suppliers/glade-diff.md`, and the current node/wire implementation.

## Verdict

The separation of a pure discovery kernel from transport is a strong design
direction, and authorized most-specific matching is a useful routing primitive.
The document is not yet a buildable protocol, however. Four issues block a safe
implementation: inbound events have no current time, lease records and the
production clock use incompatible time domains, the requester principal is
self-asserted, and demand instantiation is referenced by the algorithm but is
absent from the state and protocol. Equal-epoch resolution, capability record
identity/revocation, and the anti-entropy contract also need normative closure
before implementation.

## Blockers

### GD56-01 — BLOCKER — inbound work cannot observe the time it must use

- **Spec claim:** lines 18–28 define `Inbound { from, msg }` without time and say
  `now` arrives only on `Wakeup`; lines 43–54 require current time for scheduling
  and projection; lines 102–105 require lease filtering.
- **Failure:** suppose the last wakeup observed time 0, then at virtual time 100
  the kernel receives a claim expiring at 50 followed by a `RouteReq`. Neither
  inbound event advances `State.clock`, so the kernel can route the expired
  claim. An inbound event that starts a retry also schedules relative to stale
  state time and may emit an already-past absolute deadline.
- **Required disposition:** every event MUST carry its observed time, or `step`
  MUST be `step(state, now, event)`. The kernel MUST reject backwards time and
  MUST use that event time only for projection/actions, never as folded data.
  Add traces for an already-expired first claim, a route arriving between
  wakeups, and a retry created by an inbound event.

### GD56-02 — BLOCKER — persisted lease timestamps cannot be compared with the stated clock

- **Spec claim:** `lease_expiry_ms` is an absolute record value (lines 68–69),
  while lines 51–54 map production to a monotonic clock.
- **Conflict:** current `sysdir.rs` lines 175–177 and 206–210 stamp Unix wall-clock
  milliseconds. A monotonic instant is local to one boot and cannot be compared
  with a remote or persisted Unix timestamp. If `T` instead means Unix time, it
  is not guaranteed monotone under clock correction, and issuer/reader skew can
  make a lease die early or remain live too long.
- **Required disposition:** define distinct `WallTime` and `MonoInstant` types.
  The signed record MUST use a specified interoperable expiry representation;
  local scheduling MUST use monotonic deadlines. The spec MUST state clock-skew
  bounds and fail-closed behavior for uncertain, backward, rebooted, and extreme
  timestamps. The simulator MUST model inter-node wall-clock skew separately
  from late timer delivery.

### GD56-03 — BLOCKER — `requester` is an unauthenticated authority assertion

- **Spec claim:** an inbound event authenticates only `from: NodeId` (line 20),
  while `RouteReq` carries caller-chosen `requester` (line 93) and the routing
  algorithm trusts it for the consumer ACL (lines 107 and 115–117).
- **Failure:** Mallory sends `RouteReq{requester: Alice, ...}`. Nothing binds
  Alice to the sending session, node, signature, or a delegated principal
  chain, so Mallory receives Alice's routing result and can pass the stated ACL.
  This also bypasses the B4 rule for identity-derived `self` keys.
- **Required disposition:** the trusted node/session layer MUST supply an
  authenticated B3 principal context to the kernel; a wire field MUST NOT be
  treated as that context. Forwarding MUST preserve a verifiable, attenuated
  principal proof. Correlations MUST be scoped to the authenticated return path,
  not accepted as globally unique caller data. Add forged-principal and
  caller-supplied-`self` scenarios.

### GD56-04 — BLOCKER — the demand-instantiation transition does not exist

- **Spec claim:** routing step 6 (lines 125–126) matches a `ServiceDefinition`
  and triggers instantiation, and the scenario matrix requires the full diff
  lifecycle (lines 179–180).
- **Contradiction:** `State` has no service-definition index; section 3 does not
  define a service-definition record; `DirOp` admits only claims, grants, and
  revocations; scenario inputs use only those same messages; and the vocabulary
  has no instantiate request, placement decision, acceptance, completion, or
  failure message. The current `ServiceDefinition{app,name,glade_id}` in
  `sysdata.rs` cannot perform the structural match required by the richer
  `DemandServiceDefinition` in `glade-diff.md`.
- **Failure:** no candidate is found, but `step` has no legal outbound value that
  can start the instance and no later event that can resolve `pending`; therefore
  the required diff scenario cannot be expressed, much less pass.
- **Required disposition:** either remove instantiation from this kernel and
  define an explicit `NeedsInstantiation` result for an owning component, or
  specify the complete signed definition schema, definition fold/index,
  placement rule, instantiate protocol, idempotency key, D5 execution check,
  timeout/failure arms, and claim-publication handoff. Unauthorized callers MUST
  NOT be able to trigger compute merely by producing a routing miss.

## High-priority findings

### GD56-05 — HIGH — the consumer authorization rule is weaker than INV-7 and lacks inputs

- **Spec claim:** lines 115–117 reduce consumer authorization to “read on the
  target binding”; `State.grants` is described only as serve-grants plus
  revocations (line 34).
- **Conflict:** `GladeAuthzModel.md` lines 375–390 requires derived reads to check
  the entire source closure for the authenticated principal on every subscribe,
  replay, cache delivery, and forwarding hop. It also requires re-evaluation
  when policy changes. Target-binding read is not an equivalent check.
- **Failure:** a principal who can name `svc/ws.diff/{key}` but cannot read one
  source can be routed to a warm global instance. Conversely, the kernel cannot
  perform even the stated target check because it has neither the principal
  chain nor the target/source policy fold in its state or event contract.
- **Required disposition:** specify an exact interface to the common pure authz
  checker, including principal context, resource/source closure, verb, policy
  fold, and read-side time. Discovery authorization MAY suppress routing
  metadata, but the serving and cache-delivery hops MUST still re-check INV-7.

### GD56-06 — HIGH — equal epochs make the routing fold non-deterministic

- **Spec claim:** lines 102–105 select highest epoch and lines 118–120 assert
  ties are impossible.
- **Failure:** during a partition, authorized nodes A and B both observe maximum
  epoch 7 and independently claim epoch 8. These are valid records from distinct
  origins, so neither signatures nor the per-origin chain remove either one.
  With no total tie-break, replicas may select different nodes based on arrival
  or map iteration, contradicting byte-identical replay and convergence.
- **Required disposition:** define a deterministic total order for claims in a
  slot and state how an epoch is allocated and fenced. If takeover has an owner
  term or predecessor requirement, put it in the signed schema and validate it.
  The serving path MUST also define how stale losers learn the winning fence and
  bounce requests. Add simultaneous equal-epoch takeover and maximum-value tests.

### GD56-07 — HIGH — ordinary claim conflict is incorrectly called equivocation

- **Spec claim:** line 149 says two conflicting claims for one slot are
  “envelope-signature-convictable,” and the scenarios promise conviction with no
  oscillation (line 174).
- **Conflict:** current chain equivocation is a signed fork at the same
  `(origin, stream, seq)` (`peer.rs` lines 163–165). Two nodes claiming the same
  slot, or one node publishing two sequential claims, is not cryptographic
  equivocation. Both records may be valid.
- **Required disposition:** distinguish chain equivocation from advertisement
  contention. Define the exact evidence predicate, routing consequence, and
  recovery for each. Do not “convict” an origin for records that the protocol
  permits; resolve ordinary contention with the deterministic claim order from
  GD56-06.

### GD56-08 — HIGH — the reused grant/revocation shapes cannot express the claimed chain

- **Spec claim:** lines 75–80 say no new grant type is needed and reuse
  `CapabilityGrant{principal,share,verbs}` plus
  `CapabilityRevocation{principal,share}`; lines 137–142 nevertheless require an
  owner-rooted referenced grant.
- **Conflict:** the current flat grant has no issuer, grant id, parent, caveats,
  or authority chain. The current revocation identifies no grant, issuer, or
  ancestry; it revokes every grant for `(principal,share)` forever, including a
  later re-grant. `GladeAuthzModel.md` lines 56–72 and 76–104 requires issuer,
  subject, resource, parent/attenuation, signatures, and ancestry-aware
  governance. A claim's `grant_ref` cannot repair a revocation that does not
  identify what it revokes.
- **Placement conflict:** `GladeAuthzModel.md` lines 152–167 requires workspace
  policy to ride in that workspace, while the current registry and
  `GladeWorkspaceDirectory.md` place grants in the home directory. The design
  does not say which fold resolves `grant_ref` or how a directory-only replica
  obtains enough proof without centralizing all workspace policy.
- **Required disposition:** state the v2 deltas for grants and revocations,
  including stable grant identity, issuer/subject, parent chain, resource/verb,
  signature coverage, `revokes_ref`, re-grant semantics, and record location.
  Add revoke-one-of-two, revoke-then-regrant, out-of-ancestry revoke, and
  missing/reordered referenced-grant scenarios.

### GD56-09 — HIGH — “fold” mixes time-free state, validity, and live projection

- **Spec claim:** section 2 says time never enters the fold, but lines 102–105
  define the per-slot fold winner as unexpired and authorized. Lines 112–114
  then authorize the already-folded winner again.
- **Conflict:** `GladeWorkspaceDirectory.md` lines 64–69 requires cryptographic
  and capability validity filters before a time-free fold, with lease expiry
  evaluated only at projection. Live authorization can also change when a grant
  or revocation arrives, so it cannot be baked ambiguously into a cached winner.
- **Failure:** an unauthorized high-epoch claim may either suppress a lower
  authorized claim (if epoch wins before authorization) or disappear permanently
  when its referenced grant merely arrived later (if admission discards it).
  Different reasonable implementations produce different answers.
- **Required disposition:** normatively separate (1) structurally and
  cryptographically valid op ingestion, (2) the time-free retained claim set,
  and (3) `project(now, authz_fold)` that filters live authorization and expiry
  before selecting by epoch/total order. State whether temporarily unresolved
  claims are retained and re-evaluated when their proof arrives.

### GD56-10 — HIGH — state keys cannot represent the specified concurrency

- **Spec claim:** `mine: Map<Share, OwnClaim>` manages own claims and instance GC
  (line 35), and `pending: Map<Corr, RouteReq>` manages in-flight routes (line
  36).
- **Failure:** one node can run many `ServiceInstanceClaim`s in the reserved
  `svc` share, but inserting the second overwrites the first. Two independent
  requesters can choose the same `corr`, causing one pending route to overwrite
  or receive the other's answer. Renewed claims also leave older scheduled
  wakeups active because the scheduling contract has neither cancellation nor a
  specified generation check.
- **Required disposition:** own-claim state MUST be keyed by full advertisement
  slot plus claim/generation identity. Pending state MUST be keyed by an
  authenticated return-path id plus correlation and MUST define duplicate
  behavior. Wakeups MUST carry a generation/token whose stale instances are
  harmless, or the output contract MUST support cancellation.

### GD56-11 — HIGH — the routing protocol has no complete request/response path

- **Spec claim:** `RouteReq` is inbound-only and `RouteAns` outbound-only (lines
  89–94); handling may answer immediately “or forward a subscribe,” while state
  contains pending routes.
- **Gap:** the protocol does not define the answer destination, inbound answers,
  a `Subscribe`/forward message, provider acknowledgement, delivery failure,
  retry, timeout, or late-answer deduplication. `Addressed` output only says a
  message was handed to the environment; it provides no failure event from
  which the kernel can produce the model's unreachable-host status.
- **Required disposition:** choose and specify one boundary. A resolution-only
  kernel SHOULD synchronously return `RouteAns` to the authenticated ingress and
  remove `pending`. A forwarding kernel MUST add the full forward/ack/result/
  timeout protocol and explicit terminal absence reasons. Every accepted request
  MUST have exactly one terminal answer despite loss, retry, and late delivery.

### GD56-12 — HIGH — `Heads / Gap` is not the existing wire contract

- **Spec claim:** line 92 calls `Heads / Gap` “the existing wire.”
- **Conflict:** the current wire sends `Heads`, then zero or more `Ops` chunks;
  stream close is the gap-complete marker (`peer.rs` lines 168–210 and 216–243).
  There is no `Gap` message. A pure message kernel has no implicit stream close
  unless completion becomes an explicit event/message.
- **Failure:** an implementation cannot tell an empty gap from a dropped reply,
  cannot know when reconciliation completed, and cannot map reordered/gapped
  suffix handling to the current verify-as-ingest behavior.
- **Required disposition:** either reuse the exact `Heads`/`Ops`/completion
  protocol with full schemas and limits, or define a new explicit gap request,
  chunk, completion, and retry protocol. Specify chain-gap buffering/re-fetch,
  duplicate handling, suffix rejection, and signature verification order.

## Medium-priority findings

### GD56-13 — MEDIUM — deterministic replay is under-specified

- **Spec claim:** scenario plus seed replays byte-identically (lines 25–27), and
  ties use event insertion sequence (lines 48–50).
- **Gap:** no canonical ordering is defined for map/set iteration, outbound
  arrays, simultaneous initial inputs, records within gossip, or wakeups emitted
  by one step. The PRNG algorithm and seed width are unspecified, as are the
  semantics and parameter types for loss, reorder, duplication, and drift.
  Generated messages also have no stable `msg-ref`, although expectations refer
  to one.
- **Required disposition:** freeze the PRNG, event-id allocation, all ordering
  rules, canonical scenario encoding, fault parameter schemas, and stable
  message identities. Route expectations SHOULD support an explicit observation
  instant or bounded interval rather than assuming that `at_ms` alone defines
  whether the assertion occurs before or after same-time events.

### GD56-14 — MEDIUM — append-only leases and unbounded queues create a trivial DoS

- **Spec claim:** renewals and gossip continually append records, while state
  uses unrestricted `RecordSet`, `pending`, outbound arrays, heads, and gap
  payloads.
- **Failure:** expired renewals remain in the set forever; a permitted server can
  grow claim history indefinitely, and an unauthenticated caller can grow
  pending correlations or expensive exact keys. One oversized `Heads` or gap
  can monopolize memory and CPU even when decoding is fail-closed.
- **Required disposition:** define maximum record/message/key/head sizes,
  per-principal pending and rate limits, gap chunking, retry budgets, lease-
  history compaction/snapshot rules, and behavior on limit breach. Compaction
  MUST preserve chain/equivocation evidence and convergence semantics.

### GD56-15 — MEDIUM — malformed and unauthorized input needs explicit outcomes

- **Spec claim:** lines 189–190 say malformed `DirOp` is quarantined and treated
  as absent.
- **Gap:** “absent” does not say whether the op is retained as evidence, whether
  its origin suffix is rejected, whether a gap is requested, or whether the
  sender receives an error. It also risks converting malformed directory input
  into a demand-instantiation miss.
- **Required disposition:** define typed ingest verdicts for malformed,
  unsupported-version, bad-signature, bad-chain, unresolved-proof,
  unauthorized, and duplicate input. Only a genuine projected absence MUST
  trigger demand matching; decode/authentication failure MUST NOT.

## What held

- The pure `step(state,event) -> effects` boundary is an appropriate way to make
  discovery computation testable without embedding transport.
- Keeping time out of replicated fold data is correct once event time, wall
  time, and monotonic scheduling are separated precisely.
- Authorization before specificity ranking prevents a more-specific forged
  advertisement from winning when the authorization/projection order is made
  normative.
- Scoped longest-match precedence correctly avoids letting an instance claim
  shadow unrelated bindings in the anchor share.
- Failure-as-data and data-driven network scenarios are good acceptance-test
  principles; the scenario contract needs the determinism and typing closures
  above.

## Minimum closure before planning

The design SHOULD not be sequenced into implementation until GD56-01 through
GD56-12 have normative resolutions. The revised document MUST include traceable
requirements and data-driven scenarios for each resolution, especially forged
requester identity, cross-clock lease behavior, equal-epoch partition/heal,
revoke/re-grant, missing proof arrival, demand authorization, correlation
collision, anti-entropy completion, and late stale wakeups.
