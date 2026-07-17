# Review 56-2 — `GladeDiscoveryDesign.md` v2

Reviewed: 2026-07-17  
Scope: re-review of the current `GladeDiscoveryDesign.md`, including its claimed
closure of GD56-01..15. Cross-checked against the discovery/authz/diff models and
the current node, registry, store, peer, and wire shapes.

## Verdict

V2 is materially better than the first draft. It fixes the missing event time,
removes the caller-asserted principal, separates contention from equivocation,
separates wall time from monotonic scheduling, and chooses a much clearer
resolution-only kernel boundary.

It is still not buildable as written. The most important remaining issues are
that discovery invokes an authorization fold it explicitly does not hold; the
synthetic `svc` namespace has no owner capable of issuing the required
serve-grant; the pure kernel has no way to sign or append the `DirOp`s it emits;
and clock rollback can resurrect an expired claim. The revised anti-entropy,
epoch fencing, grant/revocation, definition matching, and compaction rules also
remain incomplete. The statement at lines 3–6 that v2 closes all 15 findings is
therefore premature.

## Blockers

### GD56R2-01 — BLOCKER — routing still calls an authorization fold the kernel does not possess

- **Spec claim:** lines 143–150 call
  `may_read(principal, resource, verb, policy_fold, read_time)` before returning
  routing metadata. Lines 100–108 place content ACLs in the workspace and say
  discovery does not hold them. `State` at lines 187–193 contains only retained
  directory records, definitions, own claims, and pending requests.
- **Contradiction:** neither `ctx`, `event`, nor `State` supplies the
  `policy_fold` required by `may_read`. For a derived `svc` binding there is no
  target ACL that can substitute for INV-7; its authority is the source closure,
  which lines 147–150 deliberately defer to the serving hop.
- **Failure:** two conforming implementations can either deny every request
  because policy is unavailable or silently treat target access as plausible.
  The latter returns `needs-instantiation{def_ref}` to the source-blind principal
  explicitly admitted by `s-disc-inv7-delegate`, contradicting lines 175–183's
  claim that an unauthorized caller never learns the definition or triggers the
  handoff.
- **Required disposition:** choose one boundary. Either the trusted caller MUST
  provide a verifiable authorization verdict/capability and the exact policy
  revision used, or discovery MUST return resolution without consumer authz and
  the node/service-manager MUST suppress metadata and authorize before acting.
  For derived bindings, define whether a source-blind caller may learn `def_ref`;
  if not, source-closure authorization MUST precede definition naming. This
  reopens GD56-04/05.

### GD56R2-02 — BLOCKER — no authority can issue the required grant for `svc` instance claims

- **Spec claim:** lines 95–105 require every claim's grant to be issued by “the
  share owner.” The discovery model defines a diff instance claim over
  `(svc, ws.diff, key)`, while `glade-diff.md` defines `svc` as a reserved,
  structurally-derived namespace—not a stored share with an owner.
- **Failure:** a valid service manager can instantiate a diff but cannot obtain a
  grant from the nonexistent owner of `svc`. If the home owner is implicitly
  substituted, the same global instance observed from another home has a
  different authority. If a source owner is substituted, a multi-source service
  has more than one candidate owner.
- **Identity gap:** `CapabilityGrant.principal` is a `Principal`, while a claim
  routes to `NodeId`; v2 no longer normatively states that the claim signer,
  envelope origin, claim node, and grant subject are the same certified key or
  describes the binding among them.
- **Required disposition:** define a distinct instance-advertisement authority
  rooted in the authorized `DemandServiceDefinition` plus its D5 execution
  grant, including the derived service principal and its node binding. State the
  exact equality/chain checks among signer, origin, subject, and `claim.node`.
  Add forged-node, wrong-definition, revoked-execution, and multi-owner-source
  scenarios. This is a remaining part of GD56-08.

### GD56R2-03 — BLOCKER — a pure kernel cannot emit the signed directory records it promises

- **Spec claim:** all records ride a B5-signed envelope (lines 87–90), and
  `DirOp` emits own claims (line 120). The pure kernel state and context contain
  no signing key, signer interface, append authority, or per-origin chain writer.
- **Failure:** on renewal the kernel must choose `seq`, `prev`, sign the canonical
  envelope, durably append it, and only then gossip those exact bytes. It can do
  none of those through `Addressed{to,msg}`. Emitting an unsigned `DirOp` violates
  ingest; keeping `node.key` in the generic discovery state violates the stated
  boundary and key-custody model.
- **Required disposition:** add an explicit local effect such as
  `AppendClaimIntent{slot,generation,payload}`. The node's directory authority
  MUST allocate the chain position, sign, persist, and feed the accepted signed
  op back to the kernel before gossip. Specify sign/append rejection and crash
  behavior. Scenarios MUST cover delayed signing, append failure, restart between
  append and gossip, and a stale generation completing after supersession.

### GD56R2-04 — BLOCKER — wall-clock rollback resurrects expired leases

- **Spec claim:** only `ctx.mono` must be monotone (lines 29–32); projection
  compares expiry directly with the current `ctx.wall` (lines 51–55).
- **Failure:** a claim is expired when wall time is 10,000, then NTP or an
  operator adjustment moves wall time back to 8,000 while monotonic time
  continues forward. The next projection makes the same claim live again. This
  violates the late-renewal “never resurrect” requirement despite satisfying
  every stated `ctx` rule.
- **Additional gap:** `SKEW_MARGIN` and `MAX_LEASE` have no normative values or
  configuration authority. A valid maximum-duration claim from a clock ahead by
  the allowed margin fails `W > wall + MAX_LEASE`; the check does not include
  the margin. Arithmetic behavior at `i64`/`MonoInstant` limits is unspecified.
- **Required disposition:** maintain a monotone effective wall watermark or
  define a bounded rollback/uncertainty state that fails closed until wall time
  catches up. Use checked arithmetic and specify the allowed skew, lease limit,
  and issuer/reader inequalities together. Add backward-step, forward-step,
  reboot, excessive-skew, and integer-boundary scenarios. GD56-02 is only
  partially closed.

## High-priority findings

### GD56R2-05 — HIGH — `SyncComplete` does not make anti-entropy a complete protocol

- **Spec claim:** lines 121–134 add per-stream `SyncComplete` and say it
  distinguishes an empty gap from a dropped reply.
- **Gap:** `Heads` can advertise many streams and contains per-origin
  `(origin,seq,hash)` heads in the current wire, not one unspecified
  `head-version`. `Heads`, `Ops`, and `SyncComplete` carry no sync/round id, so
  overlapping retries cannot associate chunks or completion with the initiating
  vector. A dropped `SyncComplete` is still indistinguishable from delay unless
  a timeout/retry fires.
- **State gap:** `State` has no sync rounds, peer set, chunk progress, retry
  budget, or gossip-cadence state. The design never says who receives a periodic
  `Heads`, how a broken chain suffix is re-requested, or when a multi-stream
  round is complete.
- **Required disposition:** define `SyncId`, initiator/responder roles, exact
  current `Head` schema, chunk and final-round messages, duplicate/reorder rules,
  per-chain rejection/re-fetch, timeout/retry wakeups, and sync state. Completion
  SHOULD cover the announced round, not an uncorrelated stream tuple. GD56-12 is
  not yet closed.

### GD56R2-06 — HIGH — the new epoch rule can oscillate and its tuple is not total

- **Spec claim:** lines 154–160 order claims by `(epoch, lamport, origin)` and say
  the loser reclaims at a higher epoch after heal.
- **Oscillation:** if every still-serving loser reclaims, A loses epoch 8 and
  publishes 9, B then loses and publishes 10, and so on. This contradicts
  `glade-diff.md`'s D2 rule that a global-instance race loser MUST tear down and
  MUST NOT publish output.
- **Ordering gap:** the tuple can tie for two sequential records from the same
  origin because current ingest validates `seq/prev` but does not require
  `lamport == seq` or even strictly increasing lamport. It also permits an
  authorized node to publish a maximal epoch/lamport and fence the slot until
  expiry. Overflow of `max_seen_epoch + 1` is unspecified.
- **Required disposition:** define which authority may initiate takeover and
  when; ordinary renewal MUST NOT cause a loser to counter-claim. Use an
  immutable unique op identity (`origin,seq` or record hash) as the final
  tie-break, validate epoch/lamport bounds, and define exhaustion. Add sustained
  partition/heal with both claimers still alive and assert no post-heal ping-pong.
  GD56-06 is only partially closed.

### GD56R2-07 — HIGH — grant identity and revocation authorization remain under-specified

- **Spec claim:** lines 97–111 add caller-supplied `grant_id`, `issuer`, and
  `{revokes: GrantId}` and assert owner signing plus one-grant revocation.
- **Gaps:** the spec does not define canonical `GrantId` derivation, uniqueness
  scope, collision handling, or whether ids may be reused. It also does not state
  who may sign a revocation or how the verifier proves that the claimed issuer
  is the stable owner of the target workspace. `issuer` plus a valid signature
  proves identity, not ownership or revocation authority.
- **Failure:** a certified but unauthorized principal can publish a signed
  revocation for someone else's grant unless the undefined `Unauthorized`
  ingest verdict supplies a rule. Two grants with the same chosen id make
  `grant_ref` and `revokes` ambiguous.
- **Required disposition:** derive `GrantId` from an immutable canonical record
  identity or specify an issuer-scoped collision-resistant id; include the
  resource in resolution; define the owner/root proof and authorized-revoker
  predicate, including admin ancestry if supported. Add id-collision,
  wrong-owner grant, unauthorized revocation, revocation-before-grant, and id-
  reuse tests. GD56-08 remains partial.

### GD56R2-08 — HIGH — ingest and retained-fold semantics still contradict each other

- **Spec claim:** lines 68–79 say only `Accepted` enters the retained fold,
  `UnresolvedProof` goes to “retained-pending,” and the fold is authz-free.
  Lines 81–85 then say the unresolved claim is retained and becomes routable
  without resend.
- **Gap:** there is no pending-proof collection or proof-dependency index in
  `State`. `Unauthorized` is also ambiguous: cryptographically unauthorized
  publication must be rejected permanently, while a structurally valid claim
  with a currently absent/revoked grant must remain in the time-free set for
  projection. Calling both ingest authorization obscures that distinction.
- **Required disposition:** specify separate verdicts for structural/envelope
  validity, governance authority to publish the record kind, and live referenced
  capability state. Put structurally valid unresolved claims in a named durable
  collection/index and define restart behavior and dependency re-evaluation for
  grant and revocation arrival in either order. GD56-09/15 remain partial.

### GD56R2-09 — HIGH — service-definition matching is still not specified

- **Spec claim:** `DirOp` includes `servicedef` (line 120), `defs` is used to name
  a match (lines 161–164 and 173–183), but the records table contains no
  `ServiceDefinition`/`DemandServiceDefinition` schema or v1→v2 delta.
- **Gap:** the document does not define `def_ref`, definition identity,
  governance authority, supersession/revocation, match semantics, or a total
  order when multiple definitions match. The current
  `ServiceDefinition{app,name,glade_id}` cannot evaluate the structural matcher
  described by `glade-diff.md`.
- **Failure:** different replicas can choose different definitions for the same
  miss, or a signed but unauthorized/stale definition can turn `no-claim` into an
  actionable instantiation handoff.
- **Required disposition:** either move definition matching entirely to the
  service-manager and return a plain miss, or fully specify the signed structural
  definition fold/index and deterministic matcher in this design. Include
  overlapping definitions, supersession, forged definition, and definition-
  arrives-after-query scenarios. GD56-04 is only partly closed by moving spawn
  out of the kernel.

### GD56R2-10 — HIGH — the authenticated return path is referenced but absent from the API

- **Spec claim:** a route answer is synchronous to the authenticated return path
  (lines 127–133), and pending keys use `(ReturnPath,Corr)` (lines 187–202).
- **Contradiction:** `Inbound` contains only `from: NodeId`, principal, and
  message; `Addressed` can target only a `NodeId`. No `ReturnPath`/session id is
  supplied or addressable. Multiple sessions on one node therefore cannot be
  distinguished.
- **Boundary conflict:** a synchronous resolution-only operation needs no
  `InFlight` entry, retry budget, or durable idempotency cache, yet all three are
  still claimed. “One answer per request under input dup” does not define
  whether a duplicate is a second request requiring an answer or a replay that
  receives the cached answer.
- **Required disposition:** add an opaque authenticated `IngressId` to the event
  and a local reply effect addressed to it, or make `RouteAns` a direct return
  value rather than an `Addressed` node message. Then delete `pending` or specify
  its lifecycle, eviction, and duplicate semantics. GD56-10/11 remain partial.

### GD56R2-11 — HIGH — lease-history compaction breaks the append-chain protocol

- **Spec claim:** lines 212–215 require dropping expired claims past a horizon
  while preserving convergence and equivocation evidence.
- **Failure:** current chains start at sequence 0 and verify every `prev` hash.
  If one replica drops a prefix, a fresh peer sees the first retained op at
  sequence N as a gap and cannot verify it; if the peer already has a divergent
  retained prefix, heads alone cannot prove a common checkpoint. Independent
  wall-time compaction can also choose different horizons during skew or
  partition.
- **Required disposition:** specify signed checkpoints/snapshots with base
  sequence and hash, peer acknowledgement or a safe-retention rule, how gaps
  cross a checkpoint, and where fork evidence remains. Until that exists, v1
  MUST retain the chain and use finite lease/rate bounds rather than claiming
  safe history deletion. GD56-14 is not closed.

## Medium-priority findings

### GD56R2-12 — MEDIUM — determinism, bounds, and scenarios are still declarations, not schemas

- **Spec claim:** DR-13/14 (lines 204–219) “freeze” determinism and bounds, and
  the traceability table names scenarios.
- **Gaps:** no numeric/configured size or rate limits are given; fault kinds and
  parameters have no typed schema; the actual `Scenario` object from v1 has been
  removed; and `msg-ref` assigned at scenario load does not identify messages
  generated later by a kernel. “Canonical scenario encoding” is named but not
  defined. SplitMix64 is named without specifying how draws map to loss,
  reordering, duplication, or drift distributions.
- **Required disposition:** restore a complete typed scenario schema, define ids
  for generated effects, freeze fault parameter semantics and PRNG draw order,
  and provide normative/default limit values plus configuration authority. The
  named `s-disc-*` cases MUST exist as concrete data before the design can claim
  the test spine is buildable. GD56-13/14 remain partial.

## Closure audit of the first review

| Original finding | Re-review status | Note |
| --- | --- | --- |
| GD56-01 | **Closed** | Every step now receives time. |
| GD56-02 | **Partial** | Clock domains split; rollback/skew-limit behavior remains unsafe (R2-04). |
| GD56-03 | **Closed in principle** | Principal is trusted context, not a wire assertion; reply-path API remains incomplete (R2-10). |
| GD56-04 | **Partial** | Spawning moved out; definition matching/handoff authorization remain incomplete (R2-01/R2-09). |
| GD56-05 | **Open** | Required policy/source authority is not available at the claimed check (R2-01). |
| GD56-06 | **Partial** | A tie-break was added, but takeover can oscillate and the tuple is not total (R2-06). |
| GD56-07 | **Closed** | Contention and chain equivocation are now correctly distinguished. |
| GD56-08 | **Open** | Synthetic authority, owner proof, ids, and revocation authority remain incomplete (R2-02/R2-07). |
| GD56-09 | **Partial** | Three layers are named; unresolved-proof storage and verdict boundaries conflict (R2-08). |
| GD56-10 | **Partial** | Better keys/tokens, but return path and synchronous pending lifecycle are absent (R2-10). |
| GD56-11 | **Partial** | Resolution-only is the right boundary; its reply effect is not expressible (R2-10). |
| GD56-12 | **Open** | Completion was added without round identity, state, or recovery (R2-05). |
| GD56-13 | **Partial** | Some ordering choices are named; the executable schema remains missing (R2-12). |
| GD56-14 | **Open** | Limits are placeholders and compaction is incompatible with the chain (R2-11/R2-12). |
| GD56-15 | **Partial** | Verdict names exist, but their storage/recovery consequences are undefined (R2-08). |

## What improved and should remain

- `step(state, ctx, event)` fixes the stale-time-on-inbound defect cleanly.
- Separating `WallMs` records from `MonoInstant` scheduling is the correct model;
  it needs rollback closure, not replacement.
- Removing `requester` from `RouteReq` and accepting B3 principal context from
  the trusted session layer closes the principal-forgery hole.
- A resolution-only discovery kernel is a clearer ownership boundary than
  forwarding subscriptions or spawning services.
- Filtering claim authorization before specificity and retaining a time-free
  record fold remain sound goals.
- Contention versus same-origin/same-sequence equivocation is now stated
  correctly.

## Minimum closure before planning

GD56R2-01 through GD56R2-11 SHOULD be resolved normatively before implementation
planning. Each resolution MUST update the requirement/scenario traceability, and
the referenced `s-disc-*` tests MUST be written as concrete typed data before
the associated requirement is considered closed.
