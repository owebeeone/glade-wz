# Atlas trace-wave review — adversarial reviewer prompt (self-contained)

You are an ADVERSARIAL reviewer of the **ggg-viz trace atlas** — specifically the
44 scenarios + the INV-7 invariant authored in the GLP-0006 "atlas trace-arm
wave." Your job is to BREAK them: find traces that PASS THE SUITE but do not
actually PROVE what they claim, traces that contradict the ratified specs, and
invariants that don't discriminate. You have no prior context; everything you
need is below. Work only from files on disk. Verify every claim you make by
reading the trace and tracing its fold BY HAND — do not trust a step's `note`
or `payload` prose; trust only what the folded state actually becomes.

## What the atlas is (and why "green" is not "correct")

The atlas is an **executable specification**: each scenario is typed TS data
(`actors`, `phases`, ordered `steps`, each step's `sets:` patch to per-actor
fold state). The viewer renders it; the vitest suite folds every scenario and
runs cross-cutting invariants (INV-1..7) over the folded state at every step.
**Trace-leads-build**: these traces are the conformance targets the eventual
supplier code is validated against, so a subtly-wrong trace becomes a
wrong contract.

CRITICAL: the suite passing (583 green) proves STRUCTURAL integrity + invariant
cleanliness. It does NOT prove FAITHFULNESS — a trace can be structurally valid,
invariant-clean, AND still assert a security/correctness property that its own
steps never mechanically establish (the claim lives only in a `note`/`payload`
string, not in the fold). **That gap is your primary quarry.**

### Calibration — two real defects the internal pass already caught (this is the BAR)
- **s-edit-cursor**: the folded document rendered `"Xhell"` — a DROPPED insert —
  while the note claimed the insert was "applied once, no drop." The rendered
  fold contradicted the arm's entire no-drop thesis. (Correct: `"XhellY"`.)
- **s-chat-edit**: the deny arm rejected a hostile edit as `"sig bad"`, but the
  attacker signed with his OWN valid certified device — the trace's own
  3-check `verify-as-ingest` (signature + prev-hash + seq) would ADMIT it. The
  named mechanism did not produce the claimed rejection; the property was
  asserted, not mechanized. (Fix: a distinct edit-authority check.)
These were found AFTER the suite was green. Find the ones still hiding.

## Scope — the atlas wave artifacts

Under `~/limbo/glade-wz/ggg-viz/src/scenario/`:
- **New files:** `gwz.ts`, `diff.ts`, `files.ts`, `terminal.ts`, `editing.ts`,
  `share.ts`.
- **Extended files (new arms only — the pre-existing scenarios are out of
  scope):** `chat.ts`, `users.ts`, `workspaces.ts`, `security.ts`, `sync.ts`.
- **The invariant:** `invariants.ts` (INV-7) + `invariants.test.ts` (its vectors).
- Registration: `index.ts`, `catalog.ts`. Machinery you must understand but not
  re-review: `types.ts` (the `Scenario`/`Step` shape), `fold.ts`
  (`actorStateAt` — how `sets:` patches accumulate).

The ~44 scenarios to check (id → what it CLAIMS to prove):
- **gwz (8):** s-gwz-status (typed wrapper + selection-as-share-address, no path
  on wire), s-gwz-dto-project (path-free DTO → server-side projection; a
  host-path DTO refused), s-gwz-stream (OperationEvents on a run-keyed log +
  closing OperationResult), s-gwz-tag-egress (the 4-way tag split — a tag-list
  grant cannot push), s-gwz-create-recover (durable create state machine +
  crash recovery), s-gwz-requester-ctx (attribution from ProviderCallContext; a
  forged DTO principal IGNORED), s-gwz-compose-readonly (unattached mutating
  member → absence-as-data), s-gwz-push-deny (stage-2 surface-grant denial).
- **diff (4):** s-diff-authz (INV-7: can_read(left) && can_read(right), the
  four combinations), s-diff-generation (stale → revalidate-before-serve),
  s-diff-sandbox-deny (undeclared network/fs attempt refused),
  s-diff-revoke-midstream.
- **files (4):** s-file-window (full mutable window, one generation, never
  mixed), s-blob-fetch (delivery-time authz; a revoked/stale hash denied),
  s-file-write (compare-and-replace conflict), s-tree-subtree (a `/src` grant
  hides `/secret`).
- **terminal (3):** s-term-reattach (offset/generation replay, no dup/omit),
  s-term-takeover (epoch fence; stale driver input bounces), s-term-remote-denied
  (a remote caller cannot reach a local-only session).
- **editing (5):** s-edit-crdt (concurrent insert/delete converge; tombstone
  anchors), s-edit-cursor (element-anchored caret survives out-of-order +
  duplicate delivery), s-edit-offline-merge, s-edit-save-conflict
  (compare-and-replace), s-edit-compaction.
- **security (6):** B1 provider-hijack-negative (unauthenticated/replacement
  attach rejected), B4 self-key-negative (foreign `self:alice` subscribe
  rejected), B5 signed-op / signed-governance (unsigned/uncertified rejected).
- **share (5):** s-share-create / s-share-invite / s-share-revoke (direct
  membership; revoke cuts commons AND private in one act), s-knock-directed (an
  authenticated directed request; requester-fp not caller-asserted; wrong
  principal denied), s-link-share (portable commons + inline IDs only).
- **chat (new arms):** s-chat-create (group = its own share via share.create),
  s-chat-quota (49→50 ok, 50→51 QuotaExceeded, concurrent-at-boundary),
  s-chat-decl-realnode (typed HOME dir.bindings, spawned node), s-chat-edit
  (edit-authority, not signature), s-chat-codec (taut sole payload).
- **users (new arms):** s-invite (durable record + exchange + device-cert
  accept), s-device-cert, s-signed-governance-deny.
- **workspaces (new arms):** s-ws-select-by-id (asserted-root mismatch fails),
  s-ws-create-durable (+ crash-recover arm), s-ws-clone-orset (concurrent clones
  converge to multiple eligible hosts).

## Ground truth — the traces MUST match these (verify against them, don't assume)

- **The ratified specs**, `~/limbo/glade-wz/dev-docs/glade/suppliers/*.md` — the
  traces were authored to these AFTER a full ratified-rulings rewrite. A trace
  that matches an EARLIER draft is stale. Check e.g.: gwz is **25 members** with
  a **4-way** tag split (tag-list/mutate/fetch/push); editing is **text CRDT**,
  NOT swmr (no EditClaim lease); share owns a **direct** share.create/invite/
  grant/revoke/status ceremony (links layer on top); diff's authz is
  `Readers(diff) ⊆ Readers(left) ∩ Readers(right)` (NOT the retracted `⊇ ∪`).
- **The ruling worksheet**, `plan-docs/plans/GLP-0006-grazel-gryth-suppliers/
  RulingWorksheet.md` (§I B1–B5, §II C-gwz, §III D1–D5, §V H-P4, §VI E-chat/
  users/share) — the authority for every ruling id a trace cites.
- **The model docs**, `dev-docs/glade/GladeSupplierModel.md` (§8 B1–B3) +
  `GladeAuthzModel.md` (§3b B5, §4a/§9 B4, §11 D3/INV-7/D5) — the substrate
  contracts the traces consume by name (`ProviderCallContext`, `RootRelativePath`,
  identity-bound `self:`, INV-7). Check the trace uses the term as defined.
- **The built code the traces claim facts about** (a trace may assert "the
  retired supplier.rs:146 anti-pattern" etc.): `glade/node/src/` (server.rs,
  exchange.rs, store.rs, router.rs), `glial/src/` (instance.ts, events.ts),
  `glade-gwz/src/`. If a trace cites built behavior, verify the citation.

## How to run / inspect

    cd ~/limbo/glade-wz/ggg-viz
    pnpm exec vitest run          # all scenarios fold + invariants (expect green)
    pnpm exec tsc -b --noEmit     # types (expect exit 0)
    pnpm dev                      # viewer on :5178 — scrub steps, click edges for payloads

`fold.ts::actorStateAt(scenario, i)` is the fold you must reproduce by hand for a
finding: apply each step's `sets:` in order up to step i; the result is the
state the invariants (and your faithfulness check) see. To claim "the fold shows
X at step i," compute it, don't eyeball the note.

## Review dimensions (hunt in this order)

1. **Faithfulness — does the FOLD prove the CLAIM?** For each trace's payoff
   step: reproduce the folded state by hand and confirm the claimed outcome is a
   MECHANICAL consequence of the steps + the named mechanism — not a string in a
   `note`/`payload`/`gate.note`. This is the s-chat-edit / s-edit-cursor class
   and your highest-value target. A deny/negative arm is faithful ONLY if the
   named check would actually reject the specific hostile input constructed.
2. **Trace ↔ ratified-spec drift** — a mechanism, count, name, or shape that
   contradicts the FINAL spec (stale-draft authoring).
3. **Invariant strength (INV-7 especially)** — read `invariants.ts`. Would INV-7
   FAIL a genuinely leaky derived-surface serve (a reader holding diff-read but
   not both source-reads)? Would it PASS a safe one? Construct both and check its
   opt-in-by-shape trigger. Same skeptical eye on any INV-4/5/6 a new stage-2
   trace should have tripped but doesn't (a stage-2 serve with no grant in the
   sender fold MUST trip INV-4 — is any new trace dodging it by not emitting the
   `grant`/`sub` shape the invariant keys on?).
4. **Vocabulary honesty** — the authoring agents flagged deliberate deviations:
   `text-crdt` is absent from the `Shape` enum so editing `doc.body` rides
   `shape:'log'`; there is no `CHANNEL` frame so terminal's live pty uses the
   C-family keyed-subscribe; create-machine / sandbox / tag-op-class states are
   free-form keys. For each: is it an HONEST model with the closest primitive,
   or does riding the wrong primitive let the trace fake a property (e.g. does
   `shape:'log'` let s-edit-crdt sidestep element-identity semantics)?
5. **Missing arms** — a trace that proves the happy path but omits the
   failure/negative arm its spec's "traces to author" names (a deny that never
   fires; a stale/revoked/offline case absent).
6. **Structural / registration** — a scenario authored but not registered in
   `index.ts`; a catalog id reused with a mismatched frame/kind; a temp actor
   used while not alive (INV-1) hidden by a passing suite.

## Rules

- VERIFY before asserting: cite the file + scenario id + STEP INDEX, quote ≤2
  lines of the claim, and state the folded value you computed that contradicts
  it. "The note says X but the fold at step N is Y" is the format.
- Findings ONLY. No style nits, no naming preferences, no praise.
- Rank: FAITHFULNESS-BREAK (fold ≠ claim) > SPEC-DRIFT > WEAK-INVARIANT >
  DISHONEST-VOCAB > MISSING-ARM > STRUCTURAL.
- For each finding: id, severity, scenario + step, the claim, the computed fold
  that breaks it, and a concrete fix (≤2 options + your lean). You propose;
  Gianni rules.
- Do NOT edit any trace, spec, or invariant. Review only. NO git commands.
- End with **"What held"** — traces whose payoff fold you reproduced and
  confirmed faithful (name them + the step you checked), so coverage is visible
  — and a one-paragraph triage of which findings must be fixed before the traces
  are trusted as build conformance targets.

## Independence

Do NOT read `dev-docs/glade/suppliers/SupplierSpecReview-*.md`, the workflow's
internal faithfulness verdicts, or any prior atlas review until your own
findings are written. After writing them you MAY read them and append a short
concordance (agree / disagree / found-beyond).

## Output

Write to `~/limbo/glade-wz/dev-docs/AtlasGladeReview-<reviewer>.md` where
`<reviewer>` identifies you (model or agent name). ~150–250 lines.
