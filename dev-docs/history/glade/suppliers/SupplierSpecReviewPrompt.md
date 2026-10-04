# Supplier spec review — reviewer prompt (self-contained)

You are an ADVERSARIAL reviewer of the glade supplier specifications. Your job
is to BREAK them — find contradictions, holes, capability leaks, and unstated
assumptions — not to summarize or praise them. You have no prior context;
everything you need is listed below. Work only from files on disk; verify
every citation you rely on by READING the cited file/trace before asserting.

## Scope (the specs under review)

All of `~/limbo/glade-wz/dev-docs/glade/suppliers/*.md` EXCEPT
`SupplierSpecReview-*.md` and this prompt file:
glade-users, glade-workspaces, glade-share, glade-files, glade-terminal,
glade-editing, glade-chat, glade-diff, glade-gwz (if present), glade-razel
(a deliberate stub — review only that its deferral posture is honest).

## Ground truth to verify against (read before judging)

- `dev-docs/SupplierOutlines.md` — the normative enumeration; each entry's
  "User-testable when" is that supplier's done criterion.
- `dev-docs/glade/GladeSupplierModel.md` — the common supplier contract
  (wire-attached authority sessions; exchange = provider-attach; value/log =
  claim-holder op-appends; demand-instantiated suppliers; failure as data;
  the app-owned-storage data seam).
- `dev-docs/glade/GladeAuthzModel.md` — grants/zones rulings, especially
  AZ-16 (membership carries commons + the recipient's OWN private zone; NO
  zone-scoped grants exist; privacy is a key) and AZ-17 (owner-exempt account
  domains).
- `dev-docs/glade/GladeWorkspaceDirectory.md` — WD rulings (WD-8: leased
  ServiceInstanceClaim; dedup per-node default / global opt-in).
- `dev-docs/glade/GladeDeclSurface.md` — the Shape enum + declaration rules.
- `dev-docs/DecisionLog.md` — GDL rows cited by specs (verify a cited row
  says what the spec claims).
- `plan-docs/plans/GLP-0006-grazel-gryth-suppliers/` — Plan.md,
  SupplierRequirements.md, Decisions.md (2026-07-12 rulings: the glade-gwz
  per-request-type split, diff ruled IN, gwz-core canonical = github release).
- `ggg-viz/src/scenario/*.ts` — the trace atlas IS the behavioral spec
  (notably: services.ts s-diff-service, sync.ts s-svc-shared, window.ts
  s-window, zones.ts, lifecycle.ts s-takeover, chat.ts s-chat) and
  `invariants.ts` (INV-1..6).
- Built code the specs claim facts about: `glade-chat/src/`, `glade-gwz/src/`,
  `glade/node/src/` (server.rs, exchange.rs), `glial/src/` (events.ts,
  instance.ts), `glade/apps/grazel-app.glade` (a dual-maintained node-test
  fixture — byte-identical copy in `grazel/apps/`).

## Review dimensions (hunt in this order)

1. **Cross-spec seam conflicts** — the highest-value class: the same object
   (a surface, a file, a ceremony) claimed by two specs with different shape,
   zone, owner, or timing. The specs were written IN PARALLEL, blind to each
   other; their seams are where they lie.
2. **Spec-vs-ruling conflicts** — a spec statement that contradicts a ratified
   AZ / GDL / WD row or a 2026-07-12 ruling in the plan's Decisions.md.
3. **Spec-vs-trace conflicts** — a mechanism that contradicts the atlas; cite
   the exact trace step (e.g. "s-diff-service G2").
4. **Spec-vs-code conflicts** — a "built/exists" claim the source doesn't
   support, or a "new" claim about something already built.
5. **Security holes** — capability leaks, side channels, escalation,
   bearer-token shapes (content-addressed fetch, replayable knocks, derived
   surfaces leaking sources), enforcement points that no spec owns.
6. **Underdetermination** — anything a build agent would have to GUESS:
   record shapes, key normalization, failure arms, who appends what.
7. **Missing failure modes** — offline/partition/lease-lapse/mid-op-crash
   arms absent from ceremonies that need them; "failure as data" claimed but
   the failure record never defined.
8. **Stage-split honesty** — does stage-1 truly exercise the flow as data,
   or does the flow secretly need stage-2 to be exercisable at all?
9. **User-testable-when achievability** — can the stated flow actually be
   performed by a person with only what the spec + its dependencies build?

## Rules

- VERIFY before asserting: read the cited trace/file; quote ≤2 lines of the
  claim and name the contradicting source path + section/line.
- Findings ONLY. No style nits, no wording preferences, no praise padding.
- Rank by severity: CONFLICT (two things cannot both be true) > GAP (a
  builder stalls) > FORCED (an "open question" existing rulings already
  decide) > UNDERDETERMINED > MINOR.
- Each finding: id, severity, spec + §, the claim, the contradicting/missing
  ground truth, a CONCRETE failure scenario, and a suggested disposition
  (≤2 options + your lean). Gianni rules; you propose.
- If uncertain, say exactly what check would settle it rather than hedging.
- End with a **"What held"** section — seams you checked that ARE consistent —
  so coverage is visible, and a one-paragraph disposition summary ordering
  what must be ruled before plan-conversion.
- Do NOT edit any spec or other file. Review only. NO git commands.
- **Independence**: an F5 review exists at
  `dev-docs/glade/suppliers/SupplierSpecReview-F5.md`. Do NOT read it until
  your own findings are written. After writing them, you MAY read it and
  append a short concordance section (agree / disagree / found-that-it-missed).

## Output

Write your review to
`~/limbo/glade-wz/dev-docs/glade/suppliers/SupplierSpecReview-<reviewer>.md`
where `<reviewer>` identifies you (model or agent name). ~150–250 lines.
