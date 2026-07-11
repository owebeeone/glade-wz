# Supplier Requirements — GLP-0006

What each supplier NEEDS (identity, shapes, node capabilities, storage,
grants), what it PROVIDES, and its stage-1 scope vs stage-2 upgrade. This is
the dependency truth the phase ordering must respect; Plan.md's phases were
re-checked against it 2026-07-12 (the chat→users inversion prompted it).

## The identity split (the correction that created this file)

"Users" is two layers, not one:

1. **Principals minimal (identity + attribution)** — sessions bind to a
   principal (`Hello.principal` already exists on the wire; `dir.principals`
   is already named by GDL-038's `glade-sys.glade`); principal records are
   ordinary registry appends; suppliers ATTRIBUTE actions to principals.
   Stage-1 compatible: identity as data, nothing enforced. **P0.S7.**
2. **User management + enforcement** — CRUD over principals, grant
   ceremonies, admin verbs, `check()` enforcement. **P2, as planned.**

Nearly every supplier needs layer 1; only sharing/administration needs
layer 2. The demo's `user=` URL param + per-tab origin is the stage-1 stub
that principals-minimal replaces with real records.

## glade-chat (P1)

- **Needs — identity:** principal attribution per message (P0.S7). `ChatLine`
  today is `{ts, user: str, text}` — `user` becomes a principal ref (additive
  taut change, corpus-gated).
- **Needs — shapes:** log (exists). Group = one keyed commons surface per
  group.
- **Needs — creation flow:** dynamic "create a group" is a create-a-share
  ceremony — exactly F2's target-routed create. **Stage-1 scope: groups are
  PRE-DECLARED** (in `grazel-app.glade` / grazel config); dynamic creation
  arrives with F2 + P2.
- **Needs — storage:** none beyond the share itself (history IS the log);
  retention ruling (GAP-10) applies eventually.
- **Provides:** group chat surfaces; the first multi-principal attribution
  consumer.
- **Stage-2 upgrade:** membership grants per group (AZ-16 semantics: join =
  commons grant); moderation verbs later.

## glade-gwz (P1)

- **Needs — identity:** attribution of who ran what (P0.S7); stage-2 will
  gate per verb (the s-verbs trace; `gwz.*` seeds already in
  grazel-app.glade).
- **Needs — shapes:** exchange (built, live-proven) + log for long-op output.
- **Needs — storage/host access:** real gwz workspace directories on the
  grazel host — the first APP-OWNED STORAGE consumer; path roots come from
  grazel config (the data seam), foreshadowing AZ-1 path-scoped grants.
- **Provides:** gwz command execution surfaces; the exchange-supplier
  reference implementation.
- **Stage-2 upgrade:** verb taxonomy enforcement (`gwz.status` vs mutating
  verbs), path scoping per grant.

## glade-users (P2)

- **Needs:** the trust substrate (home share, principals from P0.S7 as the
  data it manages), ownership model GDL-034 (ratified), **WD-1 custody
  ruling** (blocking), admin verbs.
- **Provides:** the management surface (GDL-038: reads = subscriptions,
  writes = record appends) that every other supplier's stage-2 upgrade
  consumes. s-grant/s-admin traces are the spec.
- **Note:** P0.S7 provides principal *records*; this supplier provides their
  *lifecycle* (enroll, attenuate, revoke) — do not smear the two.

## glade-share (P2)

- **Needs:** glade-users (grants to mint), session placement (invite URL →
  json bootstrap → session, the GDL-032 layer grazel serves), stage-2
  enforcement (an invite is meaningless while allow-all), enrollment
  ceremonies (AZ-9/14/15 adjacent).
- **Provides:** share points — invite mint → accept → membership grant
  (which carries commons AND your private zone, AZ-16).

## glade-files (P3)

- **Needs — shapes:** window (P3.S1 contract + delivery) for fast viewport
  loads; value/log for tree/metadata (`ws.tree`, `ws.files` already declared
  in grazel-app.glade); **blob strategy (P3-gate)** for large binaries —
  never ops-in-chains.
- **Needs — identity/grants:** read attribution (P0.S7); write authz is
  stage-2; AZ-1 path-scoped grants decide how much of the tree a grant sees.
- **Needs — storage:** THE app-owned storage case — serves grazel's real
  file tree; the file↔surface mapping stays grazel's (data seam doc).
- **Needs — retention:** GAP-10 must be answered by here at latest (windowed
  caches + blobs multiply store growth).
- **Provides:** file viewing surfaces; the window-shape and blob reference
  consumer.

## glade-terminal (P3)

- **Needs — shapes:** log (scrollback — the original grip-lab path) +
  **channels** (P3.S4: ChannelOpen/Data/Close become real; WINCH is a
  control message on the channel, never a special frame).
- **Needs — identity:** pty OWNER principal (P0.S7); attach/handoff semantics
  are s-takeover-adjacent.
- **Needs — host capability:** the supplier SPAWNS PROCESSES — the sharpest
  security surface in the program. **Stage-1 scope: owner-only, local-mode
  only** (never served beyond the owner while allow-all); sharing a terminal
  REQUIRES stage-2 verbs (`shell.exec` is the s-verbs canonical deny).
- **Provides:** terminal session surfaces (scrollback + live + resize).

## glade-editing (P4)

- **Needs — shapes:** crdt/swmr contracts (P4-gate rules `swmr` vocabulary);
  glial delta path (GAP-8 completion) for cursor-stable remote deltas;
  possibly window for very large documents.
- **Needs — identity:** per-editor attribution + private-zone cursors
  (s-zones pattern, already proven for selection).
- **Provides:** collaborative editing; the ChangeEvent delta machinery's
  motivating consumer.

## glade-razel (P4, floats)

- **Needs:** razel readiness (external), exchange + log (build output),
  `razel.*` verb taxonomy at stage-2. No glade-side prerequisites beyond the
  supplier kit; slot reserved.

## Dependency spine (what the phases must honor)

```
P0.S7 principals-minimal ──▶ chat, gwz, files, terminal, editing (attribution)
F1 live claims ─────────────▶ any cross-node supplier serving
F2 target-create ───────────▶ dynamic group/workspace creation (chat P2 tail)
glade-users ────────────────▶ glade-share; every supplier's stage-2 upgrade
window + blobs ─────────────▶ glade-files
channels ───────────────────▶ glade-terminal (live half)
crdt/swmr + glial deltas ───▶ glade-editing
```
