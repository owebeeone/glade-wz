# Supplier Outlines — the generic glade suppliers, enumerated

Status: outline spec (2026-07-12) — one brief entry per supplier + its
dependencies. Each entry expands later into a full spec
(`dev-docs/glade/suppliers/<name>.md`); this file is the enumeration and the
dependency truth. Common contract for ALL suppliers:
`dev-docs/glade/GladeSupplierModel.md` (wire-attached authority sessions,
registration = ordinary records, failure as data, attribution stage-1 /
enforcement stage-2).

**Genericity rule:** a supplier depends only on base glade and on other
*generic suppliers' declared surfaces* — never on grazel, never on another
supplier's internals. grazel is an app that COMPOSES suppliers and owns
app storage; gryth-ui is a client whose plugins consume supplier surfaces
through glial taps. Any other app could compose the same suppliers.

**User-testable when** is normative per entry — the lesson of P1: a supplier
is not done when its gates are green; it is done when a person can exercise
its flow end to end. Build order follows the dependency spine, user-flow
prerequisites first.

---

## Foundation suppliers (the user-flow spine)

### glade-users — identity, onboarding, and (stage-2) access lifecycle
- Requirements: principal directory (`dir.principals`) reads/serves; **invite
  mint → accept ceremony** (invite = a record + a joinable URL/token via the
  session-placement bootstrap) so a NEW person can onboard from another
  browser/machine and appear as a principal; principal profile basics (name);
  stage-2: grant/attenuate/revoke lifecycle + admin verbs (GDL-034 ownership
  model), `check()` enforcement arrives node-side.
- The onboarding *flow* is stage-1 buildable (identity + invites as data);
  only enforcement waits on WD-1/AZ rulings.
- Depends on: base glade only. **Everything multi-user depends on this.**
- User-testable when: I mint an invite, a second real user opens it elsewhere,
  onboards, and both of us appear in a visible user list.

### glade-workspaces — workspace directory, selection, creation
- Requirements: list workspaces (`dir.workspaces` fold) with liveness (claims);
  **select** a workspace as the client's operating context (a client-visible
  selection surface other suppliers key off); **create** a workspace on a
  target node (the built `workspace.create` ceremony); surface eligible-host /
  claim state honestly.
- Depends on: base glade only.
- User-testable when: I see my real workspaces listed, pick one, create a new
  one, and the selection visibly drives the other tools (gwz, files, terminal).

### glade-share — share points (per-resource access)
- Requirements: "share this workspace/group with that principal" — grant
  records minted through a ceremony (AZ-16 semantics: membership carries
  commons + your own private zone); share links that route through onboarding
  when the recipient isn't a principal yet; revoke = one act cuts commons AND
  private. Stage-1: records + flow; stage-2: the grants actually gate.
- Depends on: glade-users (principals to grant to), glade-workspaces (things
  to share).
- User-testable when: I share a workspace with the user I onboarded, they see
  it appear; I revoke and it visibly goes away (fully honest only at stage-2).

## Tool suppliers (operate ON a selected workspace)

### glade-gwz — gwz commands
- Requirements: allow-listed gwz verbs over exchange (built: {status, ls,
  diff}); streamed output for long ops (built: `gwz.output` keyed by run);
  **operates on the workspace selected via glade-workspaces** — the root comes
  from the selection surface mapped to a configured real workspace path
  (app-side mapping stays grazel's; the request never carries a path);
  stage-2: per-verb grants replace the allow-list.
- Depends on: glade-workspaces (selection), glade-users (attribution).
- User-testable when: I pick one of MY real gwz workspaces in the UI and run
  status/ls/diff against it. (P1 gap: today's root points at a non-workspace
  data dir; only harness-built scratch workspaces worked.)

### glade-files — file viewing
- Requirements: workspace file tree (`ws.tree`-style value/log surfaces);
  windowed fast loads for large files (window shape — P3 contract); large
  binaries via the blob strategy (P3 gate; never ops-in-chains); read-first,
  writes stage-2 with path-scoped grants (AZ-1).
- Depends on: glade-workspaces (which tree), glade-users (attribution);
  forces: window shape + blob ruling.
- User-testable when: I browse the selected workspace's real tree and open a
  big file with a fast first paint.

### glade-terminal — terminal sessions
- Requirements: scrollback as a log surface; live session over channels
  (ChannelOpen/Data/Close made real) incl. resize (WINCH as a control
  message); session ownership by principal; attach/handoff s-takeover-style.
  **Sharpest security surface (spawns processes)**: owner-only/local-only
  until per-verb enforcement exists (`shell.exec` is the canonical deny).
- Depends on: glade-workspaces (cwd/context), glade-users (owner identity);
  forces: channel semantics.
- User-testable when: I open a terminal in the selected workspace, run vim,
  resize the window, and a second session of MINE can re-attach.

### glade-editing — collaborative editing
- Requirements: crdt and/or swmr shapes (P4 gate rules the vocabulary);
  cursor-stable remote deltas into a live editor (the glial ChangeEvent
  delta path); per-editor private-zone cursors/selections (s-zones pattern).
- Depends on: glade-files (the document), glade-users, glade-share (who else
  can edit); forces: crdt/swmr contracts + glial delta completion.
- User-testable when: the user I invited edits the same file with me, live,
  neither of us losing our cursor.

### glade-chat — group chats
- Requirements: group messaging on keyed commons logs (built: chat.msgs
  keyed per group + chat.groups listing + per-line principal attribution);
  **group membership via glade-share invites** (create a group = create-a-share
  ceremony; pre-declared groups were the stage-1 stopgap); history for late
  joiners (built).
- Depends on: glade-users (real participants), glade-share (invite/membership
  once past pre-declared groups).
- User-testable when: I invite the onboarded user to a group and we chat from
  two machines — not two URL-stub tabs on one machine.

### glade-razel — razel commands
- Requirements: razel verbs over exchange + streamed build output (the gwz
  pattern applied to razel); slot reserved — floats until razel is ready.
- Depends on: glade-workspaces, glade-users; razel itself (external).
- User-testable when: I run a razel build on the selected workspace and watch
  output stream.

---

## Fit: gryth + grazel

- **grazel** (the gryth node) composes: users + workspaces + share as the
  standing spine, then the tools; owns app storage (the data seam: the
  workspace-name → real-path mapping, chat history files, invite secrets);
  serves gryth-ui + the bootstrap (session placement, GDL-032 — where invite
  links land).
- **gryth-ui**: one plugin per supplier consuming its surfaces via glial taps
  (@grythjs/plugin-chat and plugin-gwz exist; plugin-workspaces becomes the
  selector everything else keys off).
- **glade demo**: one tab per supplier remains the driving/verification
  surface — against the user-testable-when line, not just green gates.

## Dependency spine (build order follows it)

```
glade-users ──────────────┬─▶ glade-share ─▶ glade-chat (real groups)
                          │        │
glade-workspaces ─────────┼────────┴─▶ glade-editing (with glade-files)
      │                   │
      ├─▶ glade-gwz       ├─▶ glade-terminal
      ├─▶ glade-files ────┘
      └─▶ glade-razel (floats)
```

Corrected sequencing (P1 retro): **users + workspaces complete their
user-testable flows before any tool supplier is called done.** Chat and gwz
as built are engineering-complete but await their spine dependencies to be
user-testable; their entries above define what "done" now means.
