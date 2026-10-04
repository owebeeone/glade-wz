# Atlas trace-wave adversarial review — reviewer 56s

Scope: the GLP-0006 atlas-wave scenarios, registration, and INV-7 named by `AtlasGladeReviewPrompt.md`.
I folded `sets:` inclusively through each cited step; prose was not treated as state.

Verification baseline: `pnpm exec vitest run` passes 583/583 and `pnpm exec tsc -b --noEmit` exits 0.
The findings below therefore survive the structural suite.

## F56S-01 — FAITHFULNESS-BREAK — file window manufactures revision 8 locally

- Scenario/step: `ggg-viz/src/scenario/files.ts`, `s-file-window`, step 8
  (`A4.b`, lines 127–134), then step 9 (`W2.c`).
- Claim: “reassembler refuses the splice — re-drives at gen 8”; the consumer
  “never saw a mixed generation.”
- Computed fold: after step 7 only `peer1` knows revision 8. At step 8,
  `local1.replica` is still `src/big.rs[0..68,000] @rev 7 (backfilling)`, while
  `local1['window ws1/ws.files']` is overwritten to `gen 8 ... coherent`.
- There is no revision-8 `SUBSCRIBE`, no revision-8 `OPS` from `peer1`, and no
  revision-8 bytes in `local1` before step 9 serves a revision-8 view.
- The fold proves a phantom generation, not a held/re-driven window. It could
  conceal precisely the revision-7/revision-8 splice the arm claims to forbid.
- Fix (lean): mark the revision-7 assembly stale/held, emit a new gen-8 interest,
  receive a gen-8 region, then reassemble and serve. Add an assertion that the
  served generation equals the local replica generation.

## F56S-02 — FAITHFULNESS-BREAK — cursor result is not produced by the sender fold

- Scenario/step: `editing.ts`, `s-edit-cursor`, steps 13–17; especially step 14
  (`L7.a`, lines 406–412) and step 17 (`A5`, lines 433–438).
- Claim: identity-based application of reordered/duplicated operations yields
  `"XhellY"`, with no duplicate or dropped element, and supplies that whole fold.
- Computed fold: step 8 sets `local1['fold doc.body{plan.md}'] = "Xhello"`.
  Step 13 adds separate `Y` and tombstoned-`o` keys but never refolds that value.
- Step 14 writes the desired result directly into `gryth1.view`; step 17 writes
  `"XhellY"` directly into `gryth2.view`. At both serves, the sender's only
  folded document value remains `"Xhello"`.
- The aggregate initial key also still says `e0..e4` are live while the separate
  `e4` key says tombstoned. No executable fold resolves that contradiction.
- Fix (lean): model the element set without a contradictory aggregate shortcut,
  add a sender `FOLD` to derive `"XhellY"`, and require delta and refresh views to
  equal that same derived value before either delivery.

## F56S-03 — FAITHFULNESS-BREAK — CRDT replicas and readers do not converge after delete

- Scenario/step: `editing.ts`, `s-edit-crdt`, steps 16–20; payoff at step 20
  (`A4.a`, lines 244–250).
- Claim: the tombstone converges the text to `"H"`; duplicate delivery is a
  no-op and does not resurrect `W`.
- Computed fold at step 20: `local2` folds `"H"`, but `local1` still folds
  `"HW"`; `gryth1.view` and `gryth3.view` also remain `plan.md = "HW"`.
- Both replicas do hold `crdt doc.body/gryth3:1 = tombstoned`, so the divergent
  render is not an arrival-order excuse: one replica was simply never refolded.
- The duplicate arm only overwrites `local2`'s desired text and never checks the
  other replica or either subscribed reader.
- Fix (lean): fold both replicas from the same element/tombstone set, serve the
  post-delete result to both readers, and add a scenario assertion comparing all
  fully-synchronized replica renders.

## F56S-04 — FAITHFULNESS-BREAK — link restore serves two refs through one subscription

- Scenario/step: `share.ts`, `s-link-share`, steps 6–7 (`C1`/`A5`, lines
  777–790).
- Claim: clicking restores both `ws-razel/ws.tree` and `doc-7/doc.body` from the
  two-ref link closure.
- Computed fold: the link and closure contain both refs and both grants exist,
  but step 6 creates only `sub ws-razel/ws.tree = guest1`.
- Step 7 is one `OPS` serve with payload `share=ws-razel`, `gladeId=ws.tree`, yet
  directly sets `guest1.view = ws-razel tree + doc-7 (restored ...)`.
- There is no `doc-7/doc.body` subscription, route, provider response, replica,
  or serve. The document half of restore exists only in prose/the final view.
- Fix (lean): expand each closure ref into its own subscribe/authz/serve path and
  mark restore complete only after every required ref resolves; add a partial-
  closure failure arm.

## F56S-05 — FAITHFULNESS-BREAK — diff freshness transition is asserted, not detected

- Scenario/step: `diff.ts`, `s-diff-generation`, steps 2 and 4–6; source change
  at step 4 (`C5`, lines 253–258).
- Claim: a source operation makes generation 1 stale, revalidation prevents its
  delivery, and generation 2 is computed from revision 19.
- Computed fold: step 2's “revalidate” has no `sets:`. Step 4's source-change
  message also has no `sets:`, so after it the entire fold still says generation
  1 is ready/cached and contains no source revision 19 or stale marker.
- Step 5 atomically replaces that string with `gen 2 ... gen 1 STALE`; no folded
  revision comparison, pending state, or input delivery causes the transition.
- Step 6 then serves generation 2 without another source-data receipt or explicit
  pre-delivery freshness fold. The marquee stale-before-serve property is prose.
- Fix (lean): persist each source revision in fold state, make the source op set
  revision 19 and invalidate gen 1, represent gen 2 as pending until computed,
  and run the freshness/authz check immediately before its serve.

## F56S-06 — FAITHFULNESS-BREAK — compaction straggler arm constructs no operation

- Scenario/step: `editing.ts`, `s-edit-compaction`, steps 8–9 (`B8`/`A4.a`,
  lines 870–883).
- Claim: a late pre-frontier insert anchored at `e5` either resolves safely or
  deduplicates after compaction.
- Computed fold: step 8 has no `sets:` at all—no element id, anchor, sequence,
  dependency, or already-seen marker enters `local1`.
- Step 9 only overwrites `text doc.body` with a sentence saying the straggler was
  safe. The fold cannot decide which advertised branch (“resolve” or “dedup”)
  occurred and cannot prove the anchor was consulted.
- There is also a false dichotomy: if every actor acknowledged through seq 30,
  a seq≤30 operation is already represented and must dedup; if it is genuinely
  unseen, the acknowledged frontier premise was false.
- Fix (lean): split into explicit vectors: re-deliver a represented pre-frontier
  element and prove idempotence; separately attempt an operation anchored to a
  compacted element and prove it cannot coexist with a valid all-actor frontier.

## F56S-07 — SPEC-DRIFT — B1 trace tests the retired `gwz.ops` surface

- Scenario/step: `security.ts`, `s-attach-authn`, initial state and steps 1–16;
  the binding begins at lines 243–247 and attach at lines 271–277.
- Claim: the final composition wall grants and attaches an exact declared
  provider surface, then epoch-fences calls to it.
- Computed fold: every grant, provider entry, handoff, and call is keyed to
  `ws-razel/gwz.ops`; a `gwz.status` verb is tunneled through that one surface.
- Ratified `glade-gwz.md` §2 defines 25 interfaces and describes `gwz.ops` only
  as the bring-up shape being reshaped. The exact final member is `gwz.status`.
- Consequently this trace blesses the multiplexed surface whose retirement is
  the capability-grain ruling; it cannot prove grants do not transfer between
  the 25 final members.
- Fix (lean): retarget the entire B1 arm to `gwz.status`, then attempt the same
  authenticated provider/grant against a second member and require exact-surface
  denial.

## F56S-08 — WEAK-INVARIANT — INV-7 is fail-open when derived metadata is absent or malformed

- Location: `invariants.ts` lines 84–103 and `invariants.test.ts` lines 232–240.
- Claim: INV-7 discriminates derived serves and enforces read on every source.
- Constructed stage-2 `A5 OPS` fold: sender has `sub svc/ws.diff=gryth1` and
  `grant gryth1 svc`, receiver lacks both source grants.
- Result: with no `derived svc/ws.diff` key, `checkInvariants` returns `[]`; with
  `derived svc/ws.diff = sourcez: ws-razel, ws-glade`, it also returns `[]`.
- Omitting `payload.gladeId` likewise returns `[]`. The included test explicitly
  locks in “no derived entry never trips INV-7.” A leaky author can evade the
  invariant by omitting the very declaration that opts into it.
- A second false-safe exists: `account ws-glade = gryth1` suppresses the missing
  source grant even though `ws-glade` is a workspace source, not an account
  domain; the test at lines 232–235 endorses this untyped exemption.
- Fix (lean): make derived-source metadata typed and mandatory for every declared
  derived binding, fail closed on missing/malformed metadata or glade id, and
  restrict owner exemption to typed account-domain shares.

## F56S-09 — DISHONEST-VOCAB — terminal channel is modeled as replicated subscribe/OPS

- Scenario/step: `terminal.ts`, `s-term-reattach`, steps 9–13; lines 146–184.
- Claim: a directed 1:1 live channel is opened, routed to the authority, and is
  “never replicated.”
- Computed fold/vocabulary: the live leg is `SUBSCRIBE` → `ROUTE` → `SUBSCRIBE`
  → `OPS` → `OPS`, with `sub ws-razel/term.pty` fan-out keys at both nodes.
- `types.ts` has no channel frame kind, although ratified `glade-terminal.md` §3
  names existing `ChannelOpen`, `ChannelData`, and `ChannelClose` wire envelopes
  and requires a third authority-provider attach path.
- The trace therefore proves lossless arithmetic over two hand-authored ranges,
  but not channel correlation, 1:1 routing, nonreplication, close behavior, or
  that identical `TermOut` records ride the channel and scrollback paths.
- Fix (lean): add channel frame/catalog vocabulary and trace the real open/data/
  close route. Keep scrollback on `SUBSCRIBE/OPS`; join the two paths only by the
  same explicit `TermOut{generation,offset,bytes}` identities.

## F56S-10 — DISHONEST-VOCAB — `log` lets editing bypass required CRDT semantics

- Location: `types.ts` line 71; all `doc.body` subscriptions/serves in
  `editing.ts`, e.g. lines 89–108 and 191–198.
- Claim: these are conformance traces for the ratified v1 text CRDT.
- Computed vocabulary: `Shape` has no `text-crdt`; every body payload declares
  `shape:'log'`. Element identity, anchor validity, tombstone dominance, causal
  dependency, and deterministic render are free-form state strings.
- This is not an honest closest primitive: F56S-02, F56S-03, and F56S-06 all
  pass because a client view or text key can be overwritten without deriving it
  from the purported element set.
- Ratified `glade-editing.md` §§1/6 requires a first-class `text-crdt` Shape and
  its taut contract; it explicitly says this is required, not deferred.
- Fix (lean): add `text-crdt` to the typed atlas vocabulary with structured
  element/delta/checkpoint payloads and invariants for render convergence,
  anchor resolution, and tombstone non-resurrection before using these traces as
  build targets.

## F56S-11 — MISSING-ARM — blob denial never revokes or stales prior authority

- Scenario/step: `files.ts`, `s-blob-fetch`, steps 10–14; deny begins at lines
  265–285.
- Claim: “delivery-time authz; a revoked/stale hash denied.”
- Computed fold: `guest1` starts the negative arm with a bare hash and never has
  a path reference or grant. The provider denies `NO authorized path reference`.
- Nothing is revoked, no authorized path revision becomes stale, and the cached
  bytes are never tested after an authority transition. This proves only that a
  never-authorized bare hash is not a capability.
- The positive cache-dedupe arm also writes `requester blob-2 = ... authorized`
  at `local1` without a folded grant/reference or an authority round trip.
- Fix (lean): authorize a viewer, populate the cache, revoke the path/grant (and
  separately advance the binding revision), then request the resident hash and
  require delivery-time re-resolution to deny without emitting bytes.

## F56S-12 — MISSING-ARM — quota trace omits two normative security/recovery tests

- Scenario: `chat.ts`, `s-chat-quota`, steps 0–21 (lines 443–655).
- Claim: the immutable, versioned, composition-pinned `ChatQuotaSettingsV1`
  safely bounds self-service creation.
- Computed coverage: the fold tests 49→50, 50→51, a serialized concurrent pair,
  and forged owner records. It never restarts/replays the admission authority.
- It also never submits a peer-authored replacement policy, so “peers MUST NOT
  edit/override” exists only in the initial value string.
- Ratified `glade-chat.md` §4 lines 167–169 explicitly requires `restart/replay`
  and rejection of an unauthorized `ChatQuotaSettingsV1` replacement.
- Fix (lean): add restart with count reconstructed from authoritative live
  ownership and the pinned policy digest, then append an unauthorized version/
  replacement and prove it is quarantined while the original limit remains 50.

## F56S-13 — MISSING-ARM — B1 omits most required attachment negatives

- Scenario: `security.ts`, `s-attach-authn`, steps 7–16.
- Claim: B1's authenticated, exact-grant, declared-class attachment contract is
  the conformance target.
- Computed coverage: peer2 fails HELLO before making an attach; peer3 is certified
  and attempts replacement while it has no exact grant until the later handoff.
- No authenticated provider attempts (a) a foreign share, (b) the right share
  without a grant, or (c) an undeclared provider class. No detach/reattach spoof
  attempt by the former provider occurs after handoff.
- `RulingWorksheet.md` §I B1 lines 200–202 names all four as required negative
  tests in addition to silent replacement and stale-epoch calls.
- Fix (lean): extend one matrix over the final `gwz.status` binding so each
  hostile input reaches the exact B1 check independently and leaves the provider
  epoch/entry unchanged.

## What held

- `s-diff-authz`, steps 3/5/8/11: only the both-sources reader is served; each partial/no-source fold has the missing grant and no `OPS` delivery.
- `s-diff-revoke-midstream`, steps 2–4: the right-source grant is deleted and delivery is denied before any artifact ships; the peer later folds revocation.
- `s-file-write`, steps 2/7/12: revision 4→5, stale-base conflict preserves 5, retry advances 5→6.
- `s-tree-subtree`, steps 4–12: `/src` serves, `/secret` read/write deny, and no secret data enters the requester fold.
- `s-term-takeover`, steps 5/10: epoch 1→2, then stale epoch-1 input rejects without moving the output cursor.
- `s-term-remote-denied`, steps 3–7: no remote route exists and the spoofing caller is rebound to its B3 context before denial.
- `s-gwz-tag-egress`, steps 8–10: `tag-push` resolves to `NONE` when only list/mutate are attached.
- `s-gwz-compose-readonly`, steps 7–9: commit has no provider in the seven-read-member composition and returns absence.
- `s-gwz-requester-ctx`, steps 1–6: the forged DTO owner loses to the authenticated provider context in final attribution.
- `s-chat-edit`, steps 7–8: a validly signed cross-author edit fails the distinct edit-authority check without changing Alice's state.
- `s-share-revoke`, steps 1–5: one membership deletion cuts commons and private delivery before peer enforcement.
- `s-knock-directed`, steps 17–22: B3 replaces the forged requester and the authority denies the true caller's unauthorized closure.
- `s-ws-select-by-id`, steps 15–20: stable-id routing reaches the authority, where asserted-root mismatch fails closed before execution.
- `s-ws-create-recover`, steps 3–10: one WorkspaceEntry survives the crash, replays as a no-op, gains the missing claim, and exposes one live entry.
- `s-ws-clone-orset`, steps 6–16: both concurrent add tags survive and the directory converges to three eligible hosts.
- `s-device-cert` / `s-signed-governance-deny`: the uncertified device maps to no root principal; its governance op creates no authority; wrong-predecessor rejects too.

Triage: F56S-01 through F56S-08 must be fixed before these traces are trusted as build conformance targets: they encode impossible folds, bless a retired security surface, or let derived-data enforcement fail open.
F56S-09/10 require typed vocabulary before terminal/editing conformance can be claimed. F56S-11 through F56S-13 are required negative arms and should land in the same correction wave.
