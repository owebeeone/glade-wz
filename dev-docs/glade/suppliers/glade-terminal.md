# glade-terminal — terminal sessions over glade (supplier spec)

Status: full spec v1 (2026-07-12); RATIFIED rulings landed 2026-07-12 from
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md` — **D9**
(stream envelopes), **D10** (authority), **B3/B4** (authenticated caller context
+ identity-bound keys), **F-GAP10** (scrollback retention). Expands the
`SupplierOutlines.md` entry; needs stated in
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/SupplierRequirements.md`
(glade-terminal: owner-only/local-only stage-1, spawns processes, `shell.exec`
canonical deny). Common contract: `dev-docs/glade/GladeSupplierModel.md`.
Substrate context: `glade/dev-docs/GladeSubstrateV1.md` §3 (read/write
asymmetry; `stream` = ephemeral, never replicated), §6 (`CHANNEL` open/data/
close in the frame vocabulary), §7 (reassembler), §8.1 + §11–12 (terminal slice
is a named proof target; `EXCHANGE`/`CHANNEL` proven via the echo provider;
reassembler DEFERRED post-LIMP). Design lineage: `GladeTerminalSliceProposal.md`
(three-surface decomposition) + the grip-lab declaration
`dev-docs/examples/griplab-terminal/provider.glade` (OpenTerminal / TerminalPty /
TerminalOutput / TerminalScreen; `affinity must_same_provider_until_close`).
Ratified/adjacent: `GladeAuthzModel.md` §1 (effects enforce at the authority, at
execution time) + §3 (`term.write` ≈ `shell.exec`, cosign(sponsor)); s-takeover
(`ggg-viz/src/scenario/lifecycle.ts` — epoch-fenced claim handoff); the
glade-workspaces §2.3 ruling that `gwz forall` routes THROUGH here; Plan.md
P3.S4 (channels become real) + P3.S5 (this supplier).

## 1. The three-surface decomposition (the read/write asymmetry, ruled)

A terminal is NOT one surface. It splits by latency class
(`GladeTerminalSliceProposal.md` §3–5) — and the split is the design:

1. **Scrollback = a `log` surface** (the grip-lab TerminalOutput path). Durable,
   single-writer (the provider), replayable from a cursor, windowed ("last
   100K"). This is what makes reattach and late-join work.
2. **The live session = a 1:1 `stream`/channel** (TerminalPty). Keystrokes up,
   output down; ordered, reliable, **never replicated** — replication latency on
   the input path is unacceptable for a terminal (§3 normative).
3. **The broker = an `exchange`** (OpenTerminal). Open-or-attach; owns
   addressing by `session_id` and attribution to the owner principal.

**Output rides BOTH paths** (§5, the scaling story): the driver sees output on
its live channel (low latency); the same bytes append to the scrollback log
(durable, reattachable, and — stage-2 — fan-out to N watchers at N log
subscriptions, **zero** extra channels). Stage-1 the "crowd" is just the owner's
own second session; the log still earns its place (reattach, §4).

## 2. Surfaces (declared per GladeSupplierModel; keyed per session)

| glade id | shape | zone / authority | content |
| --- | --- | --- | --- |
| `term.open` | exchange | — (directed); the driver leg needs `term.write` + `shell.exec` (D10, stage-2) | OpenTerminal broker: request `{workspace-ref, cols, rows, attach?: session_id}` → `{session_id, channel = (share, term.pty, key)}`. `session_id` is UNGUESSABLE (D10 — possession alone grants nothing). Owner identity + attribution come from the B3 `ProviderCallContext`, never a payload principal; failure as data. |
| `term.sessions` | log | owner-private index, keyed `self:<owner>` — identity-bound from the B3 session (B4) | TerminalSessionRecord: `session_id`, owner **fingerprint** (glade-users §1, from B3 context), workspace-relative cwd, created ts, state `live \| detached \| closed`, `driver_epoch` (§4). |
| `term.scrollback` | log, keyed unguessable `session_id` | **commons** (D10 — not a private zone); a watcher requires `term.read` (stage-2 gate) | TerminalOutput durable append log (grip-lab path); single-writer = the provider; each record is `TermOut{generation, offset, bytes}` (D9). All watchers share ONE live-output/scrollback space; windowed reads. |
| `term.pty` | stream (live channel), keyed unguessable `session_id` | directed 1:1 to the driver (never replicated); the driver needs `term.write` ≈ `shell.exec` (D10) | TerminalPty: input up as `TermIn` carrying `driver_epoch` (§3, D9), output down as `TermOut{generation, offset, bytes}` (D9). Opened by `ChannelOpen(share, term.pty, key=session_id)`. |
| `term.screen` | materialized (RESERVED) | — | TerminalScreen reassembler over the log (substrate §7); DEFERRED post-LIMP (§7 here). |

`session_id` is an UNGUESSABLE high-entropy token (D10 — possession alone grants
nothing; grants gate access); the canonical `key` (deterministic CBOR, substrate
§4) is that id for BOTH the `term.pty` channel and the `term.scrollback` log —
one id fronts the pair.

## 3. The live channel: stream envelopes + resize-as-control (D9 RULED)

The wire is **ready**: `ChannelOpen{share, glade_id, channel, key}` /
`ChannelData{channel, data}` / `ChannelClose{channel, reason}` exist with
generated codecs and corpus vectors `edge/channel-open|data|close`
(`taut/ir/glade.taut.py` §162; `glade/wire-rs/src/generated.rs`) — the open
vector already addresses `(share=sh, glade_id=pty, channel=ch1)` and carries a
`keystroke` data frame. What is **stubbed**: the node routes channel frames to
the **echo provider only** (`glade/node/src/server.rs` — the `ChannelOpen |
ChannelData | ChannelClose` arm calls `echo.handle`); there is NO path routing a
channel to the attached **authority** provider. This supplier FORCES **P3.S4**:
grow a channel route by `(share, glade_id, key)` to the attached provider and
back, 1:1 like `exchange.rs::handle_request` — a **third attach path** beside
value/log (ServeClaim + ops) and exchange (provider map) in GladeSupplierModel
§2. State this honestly: glade-terminal is the driver that makes CHANNEL real.

**Stream envelopes — the `TermOut` the spec lacked (D9 RULED).** Input frames
MUST carry `driver_epoch`; output MUST carry `TermOut{generation, offset,
bytes}`. Both the live `term.pty` channel and the `term.scrollback` log carry the
SAME `TermOut`, so a cutover can align them: `offset` is the monotonic
per-session byte position (§4 dedup), and `generation` distinguishes a
truncation/reset (F-GAP10, §6) from a continuation. All watchers share ONE
live-output/scrollback space (D10). Gaps, truncation, a closed process, and an
authorization lapse are EXPLICIT typed outcomes on the output stream — never
silent drops. This is the envelope that makes reattach (§4) and takeover
lossless.

**WINCH/resize is a control message ON the channel, never a wire frame**
(item a; Plan P3.S4 "WINCH rides here"). `ChannelData.data` is opaque bytes;
the terminal supplier defines a small **`TermIn` union** serialized into it —
PROPOSED shape: `Bytes(keystrokes) | Winch(rows, cols) | Signal(sig)`, every
frame stamped with the sender's `driver_epoch` (D9 — the fence, §4) — a taut
addition, corpus-gated, additive (the ChatLine v2 discipline). The wire
vocabulary does NOT grow a resize frame; resize is last-writer-wins on the pty
(grip-lab precedent). *Judgment call:* the typed `TermIn` union vs an in-band
escape (OSC-style) inside the raw byte stream — flagged in Open questions.

## 4. Session ownership + attach / re-attach / handoff (D9 + B3/B4 RULED)

**Ownership** identity and attribution come from the node-authenticated
`ProviderCallContext` (B3 — requester principal, certified device, and grant
evidence delivered BESIDE the request DTO), **never** a payload principal field;
a remote caller cannot masquerade as the local owner by spelling one. The owner
**fingerprint** (glade-users §1) is stamped from that context on every session
append (attribution, GladeSupplierModel §4). The `term.sessions` index is the
per-owner private surface, keyed `self:<owner>` and **identity-bound** — the node
derives the key from the authenticated session and rejects a caller-supplied
literal (B4); the `term.scrollback`/`term.pty` space is **commons** keyed by the
unguessable `session_id` (D10). Stage-1 the supplier opens and serves a session
ONLY for its B3-authenticated owner (§5).

**Attach / re-attach** goes through `term.open` with `attach: session_id`. The
**re-attach cutover is the primary CORRECTNESS seam** (`GladeTerminalSliceProposal.md`
Open Q1) and D9 now RULES it: replay `term.scrollback` from a **cursor**, then
splice to the live tail, the client trimming the replay/live overlap by
`TermOut.offset` (the monotonic per-session byte position) and keying off
`TermOut.generation` across a truncation. The cutover MUST neither duplicate nor
omit a byte — "a second session of MINE re-attaches, same scrollback, live again"
(the done criterion) is exactly this. All watchers share ONE live-output/
scrollback space (D10), so there is a single offset line to align against.

**Handoff = s-takeover-adjacent, over the DRIVER slot** (D9 RULED). The input
channel is 1:1 — exactly one session drives. Reuse s-takeover's shape
(`lifecycle.ts` — epoch fence, no election): the driver slot carries a
`driver_epoch`; a second owner-session takes over by an ATOMIC handoff that
advances the epoch (`epoch+1`), and the stale driver's `TermIn` frames **fail
closed off the epoch** — the fence rejects them and fences the old driver's
buffered bytes (the fencing-token move, not STONITH). **But the HOST does not
migrate**: the PTY is provider-local and non-migratable (grip-lab `affinity
must_same_provider_until_close`) — unlike a `ServeClaim`, which s-takeover moves
to another node. Terminal takeover shifts *who types*, never *which box runs the
process*.

**Owner disconnect** (PROPOSED): the session goes `detached` — the process
**survives**, output keeps appending to scrollback (F-GAP10 pins live/detached
sessions against eviction, §6), and the owner re-attaches later (the reattach
path above). It is NOT killed on disconnect (a dropped websocket is not intent to
kill). *Judgment call:* survive-and-detach vs a disconnect grace-timer that
signals the process group — flagged in Open questions.

## 5. Security posture — the sharpest surface in the program (item c)

The supplier **spawns processes on the host**. Effects (EXCHANGE + terminal
writes) enforce **at the authority, at execution time** — the executor is always
present, so granularity is cheap (GladeAuthzModel §1). Precise postures:

- **Stage-1 = B3-authenticated owner-only + local-only; COMMONS, not private
  zone** (D10 + B3/B4). Two things are now REAL at stage-1, not deferred: the
  owner is the node-authenticated `ProviderCallContext` (B3), never a payload
  principal — a remote caller cannot spell its way into being the local owner —
  and any per-owner private key (the `term.sessions` `self:<owner>` index) is
  **identity-bound**, derived from the authenticated session (B4). Scrollback is
  a **commons** binding keyed by an UNGUESSABLE `session_id`; this resolves the
  old "private-zone AND watchable" contradiction (F5-1) — it is NOT a private
  zone, it is commons that grants gate. Local-only holds by MECHANISM: the
  supplier **neither forwards nor advertises** a local session across the mesh
  (no cross-peer ServeClaim), and possession of the `session_id` alone grants
  nothing (D10) — so owner-only/local-only is real BEFORE the `term.read`/
  `term.write` grant CHECKS run (those are stage-2). The local-only mechanism is
  **answered** (no-forward/no-advertise + unguessable id — the old §10 judgment
  call is closed).
- **The `gwz forall` handoff stays inside that posture, now under B3-
  authenticated ownership** (glade-workspaces §2.3 ruling — this spec OWNS the
  handoff). The gwz panel composes a `gwz forall … -c '<cmd>'` line and hands it
  to a terminal session **owned by the same B3 principal, in the selected
  workspace's cwd**. It is `TermIn.Bytes` on the owner's OWN pty —
  **owner-self-exec**, exec-equivalent to the owner typing it, so it needs **no
  new grant** and crosses no principal boundary (the same authenticated principal
  is on both legs). glade-gwz itself never executes arbitrary commands (its
  `forall` exclusion, P1.S2, is resolved here permanently). *Judgment call:* a
  fresh ephemeral owned session per forall vs reusing the owner's existing
  session — flagged.
- **Stage-2 = the grant gates fire** (D10; the s-verbs canonical deny,
  SupplierRequirements). A watcher requires `term.read` to stream scrollback; a
  second **driver** requires `term.write` AND `shell.exec` (GladeAuthzModel §3 —
  one danger class) plus any configured **co-sign** rule. Agent drivers ride
  **cosign(sponsor)**: the human approves each command in-loop while `term.read`
  streams (§3, approval-in-the-loop). Handoff to a DIFFERENT principal is stage-2
  (a grant, not just an epoch).

## 6. The pty / process data seam (item d)

The seam is grazel's rule generalized (GladeSupplierModel §5): what leaves is
the declared surface; the backing process is invisible.

- **App-owned (never on the wire):** the **cwd mapping** — selected-workspace
  name → real gwz root path — is grazel config (glade-workspaces §1.2, "never
  rides a request, never derivable from one"); the process **env**; the shell
  binary/argv; the pty fd and process handle. A `term.open` request or a
  `ChannelData` frame **NEVER carries an absolute host path** and cannot escape
  the workspace root (the gwz-family invariant, "a request never carries a
  path").
- **Declared surface (all a subscriber sees):** the unguessable `session_id`,
  owner fingerprint, **workspace-relative** cwd, session state, and the output
  bytes (as `TermOut{generation, offset, bytes}`, §3/D9). The storage engine
  (files-for-now / SQLite-later) never leaks into glade's model.
- **Retention (F-GAP10 RULED):** the scrollback storage declares
  `max_bytes`/`max_age`/pin conditions like every retained class. Terminal
  default: **16 MiB or 100,000 lines per session**; **retain 7 days after
  close**; **live/detached sessions are PINNED** (never LRU-evicted while open);
  on truncation the log **records the new first `offset`** — aligned with D9's
  `TermOut{generation, offset}` so a reattaching client (§4) sees an EXPLICIT gap
  (a bumped `generation` + a new first offset), never a silent hole. Eviction
  MUST NOT silently rewrite authoritative history.

## 7. Stage split + what is deferred

- **Stage-1 (buildable now):** `term.open` broker with an UNGUESSABLE
  `session_id` (D10); `term.pty` channel routed to a REAL provider (grow the
  route past the echo stubs today, P3.S4) carrying the D9 envelopes — `TermIn`
  with `driver_epoch` up, `TermOut{generation, offset, bytes}` down;
  `term.scrollback` as a **commons** `log` fold + windowed read; the `TermIn`
  resize control; reattach from a second owner-session with offset/generation
  dedup; the forall handoff; the atomic epoch-fenced driver handoff. **Security
  substrate is stage-1** (the §B reframe): the B3-authenticated
  `ProviderCallContext` and B4 identity-bound keys are prerequisites, not
  deferred — owner identity is REAL. Owner-only + local-only holds by
  no-forward/no-advertise + unguessable id, not by a grant check.
- **Stage-2:** the grant CHECKS fire — `term.read` gates a watcher, `term.write`
  ≈ `shell.exec` gates a second driver, plus cosign(sponsor) for agents;
  scrollback replicates to watchers; cross-principal handoff needs a grant, not
  just an epoch.
- **Deferred (not v1):** the `term.screen` TerminalScreen **reassembler**
  (substrate §7 / §12 — post-LIMP) and the **segmented/anchored lazy log**
  (`GladeTerminalSliceProposal.md` §7). v1 serves raw scrollback + windowed
  reads; the screen-materialization and GC-able segments are later
  optimizations, not correctness. PROPOSED: keep them reserved, don't build
  ahead of a multi-watcher need.

## 8. Traces to author before building (atlas leads)

- **s-term-open** — open in the selected workspace: `term.open` → session record
  (owner fp, workspace cwd from selection mapping) → `ChannelOpen(share,
  term.pty, session_id)` → keystrokes up, output down on the channel AND appended
  to `term.scrollback` → run **vim**. Proves: channel routed to a REAL provider
  (not echo), output-rides-both-paths, cwd-from-mapping (no path on the wire).
- **s-term-resize** — a `TermIn.Winch(rows, cols)` on the SAME channel; provider
  applies it to the pty; vim reflows. Proves: resize is a channel control
  message, not a wire frame (the `TermIn` union is the seam).
- **s-term-reattach** — a second session of MINE attaches → scrollback replay
  from a cursor → live-tail cutover keyed on `TermOut.offset`/`.generation` (D9).
  Proves the primary correctness seam (Open Q1); INV: no output byte doubled or
  dropped across the cutover, and a truncation shows as a bumped generation + new
  first offset (F-GAP10), never a silent gap.
- **s-term-forall** — glade-gwz composes `gwz forall … -c '<cmd>'`, hands it to
  an owner-owned session in the selected workspace; per-member output streams.
  Proves forall's home is the terminal (workspaces §2.3), owner-self-exec, no new
  grant, still owner-only/local-only.
- **s-term-takeover** — driver-slot handoff (D9): an ATOMIC handoff advances
  `driver_epoch` to `epoch+1`; the stale driver's `TermIn` **fails closed off the
  epoch** (rejected, old buffered bytes fenced); the HOST does not move (contrast
  s-takeover, which moves the ServeClaim). Proves epoch-fence reuse and the
  host-pinned distinction; plus an owner-disconnect → detached → survive arm.
- **s-term-remote-denied** — a REMOTE caller tries to reach a local session:
  `term.open`/`attach` from another principal finds no route (the session is not
  forwarded or advertised, D10) and cannot masquerade as the owner (identity is
  the B3 `ProviderCallContext`, not a payload principal). Proves owner-only/
  local-only is real by MECHANISM — possession of a `session_id` alone grants
  nothing.

## 9. Dependencies + user-testable-when

- **Depends on:** glade-workspaces (selection → the app-side cwd mapping;
  `ws.selection`), glade-users (owner fingerprints, §1), and the §B security
  substrate — the B3 node-authenticated `ProviderCallContext` + B4 identity-bound
  keys (stage-1 prerequisites). **Forces:** the channel-provider route (P3.S4 —
  the echo-only gap), the reattach cutover, the `TermIn`/`TermOut` taut envelopes
  (D9).
- **User-testable when** (normative, `SupplierOutlines.md`): I open a terminal in
  the selected workspace, run **vim**, **resize** the window and it reflows, and
  a **second session of MINE re-attaches** — same scrollback, live again — with
  no output doubled or lost across the cutover.

## 10. Open questions (Gianni)

- **Resize payload:** typed `TermIn` union in `ChannelData.data` (PROPOSED) vs an
  in-band OSC-style escape in the raw byte stream? D9 RULED the envelope (every
  input frame carries `driver_epoch`, output is `TermOut{generation, offset,
  bytes}`); this open is only the union-vs-OSC *shape* of the input payload, still
  a judgment call. The union costs a taut/corpus addition but keeps resize legible
  and off the wire vocabulary.
- **Owner disconnect:** survive-and-detach (PROPOSED, the process outlives a
  dropped socket) vs a grace-timer that signals the process group? And who may
  reap a long-detached session's process — the owner only, or a host policy?
  (F-GAP10 pins live/detached sessions against eviction but does not rule the reap
  policy.)
- **Driver handoff = epoch fence only? — RESOLVED (D9/D10).** The owner's-own
  second-session handoff is an ATOMIC advance of `driver_epoch`; the stale driver
  fails closed off the epoch. A cross-principal driver handoff needs a `term.write`
  ≈ `shell.exec` grant plus any co-sign rule (D10), not just an epoch — that is
  stage-2, no longer an open question.
- **forall session:** a fresh ephemeral owned session per `gwz forall` (clean
  teardown) vs reusing the owner's standing session (shared scrollback)?
- **local-only mechanism — ANSWERED (D10).** Enforced by **no-forward/no-advertise
  + an unguessable `session_id`** (commons gated by grants), not by an explicit
  local-only marker on the ServeClaim and not by a private-zone claim — zero new
  machinery beyond the supplier never advertising a local session.
- **Segmented/anchored scrollback:** stay on the plain `log` fold for v1
  (PROPOSED) and adopt the segment/anchor scheme (`GladeTerminalSliceProposal.md`
  §7) only when a real multi-watcher / long-history need lands? (F-GAP10 rules the
  retention numbers — 16 MiB/100k lines, 7-day post-close — but leaves the
  segment/anchor *mechanism* deferred.)
