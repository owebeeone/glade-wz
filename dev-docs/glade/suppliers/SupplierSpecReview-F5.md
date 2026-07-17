# Supplier spec review — F5 pass (adversarial)

Reviewer: Claude Fable 5 — **the session that commissioned the spec wave and
verified each spec at landing. This review is adversarial but NOT independent**;
it complements an independent pass (see `SupplierSpecReviewPrompt.md`).
Date: 2026-07-12. Scope: all of `dev-docs/glade/suppliers/`: glade-users,
glade-workspaces, glade-share, glade-files, glade-terminal, glade-editing,
glade-chat, glade-diff, glade-razel (stub); glade-gwz.md was in flight at
first writing and is covered by the F5-10/F5-11 addendum (its 12-request-type
count and the `repository_path` host-path field were verified against
`gwz.taut.py` at landing).
Method: each spec was verified against its cited sources at landing (trace
steps read, node/glial/glade-chat source grepped, `.glade` fixtures checked);
this pass hunts what per-spec verification cannot see — cross-spec seam
conflicts, spec-vs-ruling conflicts, and unowned questions between specs
written in parallel, blind to each other.

Findings ranked. CONFLICT = two specs (or a spec and a ruling) cannot both be
right. GAP = a build agent would stall or guess. FORCED = an "open" question
that existing rulings already substantially decide. MINOR / CONSOLIDATION as
named. Each carries a suggested disposition; Gianni rules.

---

## F5-1 (CONFLICT) glade-terminal: scrollback cannot be both private-zone and watchable

- **Claim A** (glade-terminal §2 table): `term.sessions` / `term.scrollback`
  live in the "owner's private zone (local-only, stage-1)".
- **Claim B** (same table row + §5): "Stage-2 a `term.read` grant replicates
  it to watchers."
- **The ruling that breaks it**: AZ-16 — *no zone-scoped grant ever exists*;
  privacy is a KEY (`self:<principal>`), and a membership grant carries the
  commons plus the recipient's OWN private zone, never someone else's. A
  foreign `self:` never matches routing (s-zones). A watcher can therefore
  never be granted into the owner's private zone; a private-zone scrollback is
  structurally unwatchable, and stage-2 `term.read` as specced can serve
  nothing.
- **Disposition**: (a — lean) scrollback rides **commons keyed by
  `session_id`**, stage-1 local-only enforced by the spec's own other
  mechanism (no mesh advertise, no cross-peer ServeClaim), stage-2 `term.read`
  grants gate it — one surface, no migration; or (b) keep private and add a
  stage-2 supplier re-publish to a separate watch surface (two copies of every
  byte). This also answers the spec's own §10 local-only open: the
  zone-convention arm is the one that breaks watchers, so it should lose.

## F5-2 (CONFLICT/GAP) glade-files ↔ glade-editing: one file, two truths, no coherence story

- glade-editing §4.2: while a session is open **the live fold is truth**;
  `doc.save` flushes to the working tree; "autosaving every keystroke would
  churn it and fight gwz."
- glade-files serves the AT-REST file (`ws.files`), and §2.5 promises "a
  mutable file republishes its region cursor-stably on edit (stage-2)" — but
  editing-session keystrokes never touch glade-files' surface until save, so
  that republish cannot fire for them.
- **Consequence**: during a live editing session, a files viewer, a
  `gwz status`/`diff`, and a glade-diff instance over `ws.tree` all see stale
  saved bytes while editors see the fold. No spec owns the seam or even states
  it. A build agent implementing files §2.5 would wire an edit-hook that the
  editing spec forbids.
- **Disposition**: (a — lean) rule `ws.files` **at-rest-only** and say so in
  both specs: viewers/diff/gwz see saved state, plus a cheap "open editing
  session by X" marker record on the doc so staleness is visible; or (b)
  files serves the live fold when a session exists (couples files to editing's
  fold engine — against the seam discipline). Either way the files §2.5
  sentence needs a rewrite to name WHOSE edits republish (files.write's own,
  not editing's).

## F5-3 (FORCED) glade-chat Q1: the ruled membership-snapshot already leans group-as-share

- glade-share §3 (RULED): "sharing enumerates **the group's membership fold**
  NOW and mints N grants to those fingerprints."
- Under the built group-as-KEY model there IS no per-group membership fold —
  membership is share-wide (one `chat` share, N keys), so a link shared into
  `#dev` would snapshot-mint to **every member of every group**.
- So the share ruling presupposes per-group membership, which exists only
  under group-as-share (or under a per-key grant machinery that would also
  need per-key membership folds — heavier than chat §3.1 presents it).
- **Disposition**: answer chat Q1 with this dependency visible. Group-as-share
  is not merely the lean; the alternative requires amending an already-ruled
  glade-share mechanism.

## F5-4 (GAP) glade-files §3: blob fetch has no authorization story

- The spec rules the mechanics (BlobRef in records, bytes fetched by hash,
  cross-viewer dedupe) but never the stage-2 GATE. Content-addressed fetch by
  bare hash is the classic capability leak: anyone holding (or replaying) a
  hash fetches the bytes regardless of share grants — and the hash store
  dedupes globally, across shares.
- **Disposition**: (a — lean) the fetch carries the REFERENCING record's share
  context and is gated by that share's read grant (grants remain the one authz
  model; dedupe stays a storage detail); or (b) possession-of-ref IS the
  capability (then refs are secrets, revocation of blob access is impossible,
  and links carrying refs become bearer tokens — interacts badly with
  glade-share's whole disposition lifecycle). Add to files §10; it is absent.

## F5-5 (GAP) The reassembler has no build owner

- glade-files §2.4 needs the §7 reassembler for mutable-file window fill;
  glade-terminal §7 explicitly DEFERS its reassembler consumer (`term.screen`,
  post-LIMP); no spec claims to BUILD the reassembler, and files §9 "Forces"
  lists window/blob/GAP-10 but not it.
- **Disposition**: (a — lean) descope mutable-file windows from files v1
  (logs-only windows — the s-window trace's actual case — cover build logs and
  big append files; `s-file-window`'s mutable arm becomes the reassembler's
  driver when someone owns it); or (b) add the reassembler to files' Forces
  and cost P3 accordingly.

## F5-6 (GAP) glade-diff §5: the leak guard's enforcement home is unassigned

- diff: "read on `ws.diff` MUST subsume read on both sources… computing that
  closure is glade-share's job." But share's §3 closure is a SHARE-TIME UI
  computation for links; the subsumption must hold at grant time for `ws.diff`
  and at serve time — neither spec owns either check.
- **Disposition**: (a — lean) serve-time: extend the INV-4 family — a stage-2
  serve of a derived surface requires the receiver to hold grants on the
  definition's source closure (mechanical over the sender fold; candidate
  **INV-7** for the atlas, enforceable in `invariants.ts` like INV-4/5/6); (b)
  grant-time: the s-grant ceremony refuses minting diff-read without
  source-reads (leaves later source-revoke creating a stale-subsumption hole —
  serve-time still needed). Assign it; today it is between specs.

## F5-7 (MINOR) glade-chat §5/§6 ↔ glade-users §5: legacy string principals

- Post-users-v2 `ChatLine.principal` is a fingerprint; the demo node PERSISTS
  chains (records.json) whose existing lines carry user-string principals. The
  forward-cut posture covers ephemeral gryth-ui only. One sentence needed in
  chat §5: legacy string-principal lines render as unverified legacy (users §5
  posture applied to chat), never reinterpreted.

## F5-8 (MINOR) group-as-share vs the typed-manifest compile wall

- Runtime-created groups (chat §3.1) mint share-level bindings the
  compile-time typed manifest cannot know. The manifest wall needs a stated
  posture for runtime-minted binding families (parameterized share family vs
  escape hatch). Queue with the Q1 ruling — it rides the same decision.

## F5-10 (GAP) glade-gwz: the create-family breaks the kit's two selection assumptions

*(Added after glade-gwz.md landed — the review's scope note said to give it the
same treatment.)*

- The shared kit (glade-gwz §3.3) rules: "the `Request`'s `workspace_root` …
  FILLED BY THE SUPPLIER from the **selected** workspace's app-side name→path
  mapping," and §4 puts each member's events log in "commons (**the workspace
  share**)."
- Both assumptions fail for `glade-gwz-create` / `glade-gwz-init`: a NEW
  workspace has no selection to map (the root must be **app-ALLOCATED** for the
  new name — the mapping gains an entry, per glade-workspaces §1.2's app-owned
  mapping), and its workspace share does not exist yet, so create/init events
  have nowhere to ride under §4's rule. A build agent implementing §3.3/§4
  literally stalls on the first create.
- **Disposition**: add a creation arm to the kit — root = app-allocated (the
  data seam allocates, never the request); create/init events ride the HOME
  share (where `workspace.create` ceremonies already live) or stay
  answer-only until the workspace share exists (lean: home share — the
  s-ws-create trace already runs the ceremony there).

## F5-11 (MINOR) glade-gwz: events-log visibility outruns the verb grant

- Per-member EXCHANGES are gated by per-member grants (stage-2), but every
  member's events log rides the workspace share **commons** — so any workspace
  member can watch `push`/`pull` progress (including remotes and errors in
  `OperationEvent`s) without holding that verb's grant. Possibly intended
  (operational transparency); should be a stated choice, not an accident.
  Confirm, or key events visibility to the member grant.

## F5-12 (CONFLICT) glade-gwz v2: "egress events keyed to the member grant" is a zone the grant model cannot express

*(v2 addendum — the respec introduced new surface neither reviewer has seen;
these findings are against glade-gwz.md v2, 2026-07-12.)*

- **Claim** (v2 §4/§7): read/local members' events logs ride workspace-share
  commons, but egress members' events are "keyed to the member grant."
- **The ruling that breaks it**: the zone vocabulary is commons | private
  (keyed `self:`); AZ-16 — grants are per-surface/membership, and "which zone
  you receive is routing, not policy." There is no "grant-keyed" zone; this
  is F5-1's terminal mistake (a visibility class the zone/grant model can't
  express) reappearing in new clothes one day later.
- **Disposition**: egress events logs are **commons** like the rest; their
  VISIBILITY is a stage-2 **read grant on that log surface** (the ordinary
  per-surface grant machinery the family already committed to — grant
  `gwz.push` and `gwz.push.events` together, or fold events-read into the
  member grant's verbs). Same outcome F5-11 wanted, expressed in machinery
  that exists.

## F5-13 (GAP) glade-gwz v2: nobody declares the creation members' surfaces

- v2 §5 correctly moves creation-member events/results to the **home share**
  (the workspace share doesn't exist yet). But `grazel-app.glade` declares
  workspace-share and app surfaces; home-share system surfaces
  (`dir.workspaces`, `workspace.create`) are base-glade/node territory. No
  spec or section says WHO declares `gwz.create.events` / `.result` on the
  home share — the app file, a base-glade reserved binding (like
  `workspace.create`), or a runtime declare by the supplier.
- **Disposition**: lean — reserved base-glade bindings beside
  `workspace.create` (they are legs of that same ceremony, SR56-15's state
  machine); the app file declares only workspace-share surfaces. Needs a
  line in v2 §5/§7 either way.

## F5-14 (MINOR) glade-gwz v2: the fixture pattern collapses at 21×3 surfaces

- v2 §10 says the reshape "MUST update the counts" in the dual-maintained
  `grazel-app.glade` (byte-identical copies + hand-updated record-count
  assertions in node tests). At ~21 services + ~21 events + ~21 result
  bindings + `gwz.diff.output`, the app file grows ~64 gwz records: the
  hand-maintained count-assertion + dual-copy discipline that survived +3
  will not survive +60. The §9 "generatable from the protocol IR" option
  stops being optional at this scale — codegen for the `.glade` gwz block
  (and deriving the test assertions from it) should be stated as the
  migration mechanism, not a later nicety. Also minor: §7's result-surface
  row gives an exchange a zone ("same as the member's events") — exchanges
  are directed, zone "—" by the family's own convention.

## F5-9 (CONSOLIDATION) GAP-10 has four consumers; rule it once

- files (window backfill caches + fetched blobs), chat (first heavy unbounded
  append log), terminal (scrollback; the fixture already says `windowed`
  retention for `term.log`), diff (`teardown: retained` explicitly "mirrors
  F3's retention, for compute"). One ruling — the `retention` field grows an
  eviction knob (TTL / size-cap) once — and all four adopt; do not let four
  specs invent four policies.

---

## What held (seams checked, consistent)

- **forall**: glade-workspaces §2.3 and glade-terminal §5 describe the SAME
  handoff from both sides (composed line → owner-self-exec on the owner's own
  pty; glade-gwz never executes arbitrary commands). Consistent, and the
  terminal side correctly adds why no new grant is needed.
- **AZ-16 revoke-is-one-cut**: share §3 / chat §3.2 / editing §5 all state
  membership revoke severing commons + the member's private zone identically.
- **s-takeover reuse**: terminal (driver slot, host-pinned) and editing
  (EditClaim lease) both reuse the epoch fence with the right distinctions;
  the shared-kit question is queued on both sides (editing Q4).
- **diff canonical key ↔ share ref shape**: diff's per-side
  `(share, glade_id, key-fill)` is exactly glade-share's ref instance-identity
  triple. One vocabulary.
- **ws.diff fixture drift** (grazel-app.glade:24 `log … from-cursor` vs trace
  `value`) was caught at landing and is already on the ruling queue — noted
  here for completeness, not re-opened.
- Per-spec landing verification (what was already checked before this pass):
  files — s-window trace/key/CONTENTIOUS note, Shape enum, fixture bindings;
  terminal — echo-only channel routing in `node/src/server.rs:148`, ChannelOpen
  codec (frame kind 8); chat — `chat.decl`/`groupKey`/declare-only supplier in
  `glade-chat/src`; editing — `logDelta` built (`glial/src/events.ts:64`,
  `instance.ts:161`), s-zones `self:` keying; diff — WD-8 text, both trace
  files read in full, fixture line 24.

## Disposition summary for the ruling queue

F5-1 and F5-2 need rulings BEFORE their specs go to plan (they change surface
zones and seam ownership). F5-3 should be folded into the chat Q1 ruling.
F5-4/5/6 are additions to files/diff §10 queues (F5-6 proposes INV-7). F5-7/8
are one-line spec amendments. F5-9 is the standing GAP-10 ruling, now
four-armed. No finding invalidates a spec wholesale; all are seam-level.
