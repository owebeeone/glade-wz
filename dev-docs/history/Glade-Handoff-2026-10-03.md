# Glade Handoff — 2026-10-03

Status: handoff note for the next session, written because this one ran out of
token quota mid-task. Everything below reflects state at the commits pinned
here, checked just before writing. Nothing in this note is itself a decision —
where the owner (Gianni) ruled something, it says so and names the ruling.

**Read this first, then go to the live plan documents it points at — this note
is a map, not the territory.** The two documents that actually carry the
current build are:

- `glade/dev-docs/GladeFirstSlicePlan.md` — the root plan; its tail (search
  "Cross-node writes" and "claim tie-break") is the newest work.
- `glade/dev-docs/GladeCrossNodeWritesPlan.md` — the plan this session mostly
  executed (steps X1.1–X4.3, all done).

**`dev-docs/GladeProgramStatus.md` and `dev-docs/DecisionLog.md` are stale** —
last touched in the July–September GDL-035/047/049 era, before the first-slice
and cross-node-writes work. Don't trust their "three lanes" framing or their
decision list as current; GDL numbers there are real but the log hasn't been
kept up since. If you fix this, it's a worthwhile but separate task.

A path with no leading repo name is from the glade-wz workspace root
(`~/limbo/glade-wz`, now also reachable at `/Volumes/projects/limbo/glade-wz`
— the whole `~/limbo` tree moved to `/Volumes/projects/limbo` with a symlink
left in place, mid this session; both spellings resolve).

## 1. The stack, in one paragraph each

- **taut** (`~/limbo/taut-dev/taut`): the schema/codegen tool. glade's wire
  schema (`ir/glade.taut.py`) lives here, so changing it is a taut commit.
  Gianni's rule: *keep taut unchanged unless critical*, and any feature used
  for Rust must be done for every generated language in one change, never
  Rust-only (`taut-dev/dev-docs/TautGeneratedBounds.md`).
- **glade** (`glade-wz/glade` + `glade-wz/grazel`, `glade-gwz`, `glade-gyld`,
  `glade-decl-ts`, `taut-shape`): the substrate. A Rust node (`glade-node`)
  replicates CRDT/log state over iroh, with a claim-based directory deciding
  which node serves which share. `grazel` is the app host; `glade-gwz` and
  `glade-gyld` are suppliers (gwz panel, the Gyld research app) that talk to
  a node over its websocket.
- **glial** (`glade-wz/glial`): the TS binding layer between grip (the
  reactive atom runtime) and glade — mounts, binders, CRDT profiles.
- **Gyld (the app)**: `glade-gyld` (Rust supplier process) + the `gyld`
  plugin in gryth-ui (`packages/plugins/gyld`). This is **not** the same
  thing as the `~/limbo/gyld-wz` Python workzone this session's default
  directory sits in — that's an unrelated case-study project (see its own
  `AGENTS.md`). Don't conflate the two "Gyld"s.
- **gryth** (`~/limbo/gryth-wz/gryth-ui`): the desktop UI Gianni's live desk
  runs. **Never write there directly** — see §4.

## 2. Repo state right now

| Repo | HEAD | vs origin/main | Notes |
|---|---|---|---|
| glade-wz root | `a4a8550` | ahead 2 (not pushed) | `?? dev-docs/GladeCapabilitiesForApplicationResearch.md` untracked — not mine, leave it, ask Gianni before touching |
| glade (member) | `e3cf1fe` | ahead 2 (`c1a6764`, `e3cf1fe`, not pushed) | the tie-break fix + its plan note |
| taut-dev root | `dd9c3b2` | in sync, pushed | locks glade `a82fbcd`, glade-decl-ts `a393ecb`, taut `3c6d07e` |
| taut (member) | `3c6d07e` | = origin/main | public; kept deliberately, see §6 |
| gryth-wz (owner's live checkout) | root `afa5766`, gryth-ui `2a47721` | — | **do not write here** |
| gryth-wz-appearance (lane) | root `43fb722`, gryth-ui `2a47721` | gryth-ui in sync with main; root 1 commit ahead (lane-only doc) | clean, ready to merge |
| grazel / glade-gwz / glade-gyld / glial | various, last touched Sep 23–28 | — | earlier work, predates this session, already reflected in glade-wz root history |

**Nothing from this session is pushed.** `glade-wz` root and the `glade` member
each have 2 unpushed commits (the claim tie-break). Gianni hasn't asked for a
push yet — ask, per the "push = push, nothing bundled" rule.

## 3. What's actually running (the owner's desk)

PIDs `54544` (grazel) / `54546` (glade-node) / `54558` (glade-gwz) / `54559`
(glade-gyld) / `54595` (gryth-ui vite on 5173) have been up since **2026-10-01
00:04**, started from Gianni's own terminal. The node binary is still built
from glade `ad0855c` — **before** Step R (the wall-clock renewal fix), before
all of cross-node-writes (X2–X4), and before the claim tie-break. The desk has
no peer links today, so none of that gap is currently visible to it, but a
rebuild+restart is needed before the desk can do anything cross-node, and only
Gianni restarts it (from his own terminal — never from this shell or the app's
Terminal pane).

Never touch ports 5173/8080/9099, `~/.gyld-ui`, or these PIDs/children.

## 4. The two live threads of work

### 4.1 Cross-node writes — done, over loopback

`GladeCrossNodeWritesPlan.md` steps X1.1 through X4.3 are all built (glade
`efcce3f`..`6108673`, landed across several chunks 2026-10-01/02). The journey
test (`node/tests/cross_node_writes.rs`) proves three nodes, writes from both
sides, forwarding, SWMR, and resend-after-restart, all over loopback. **Not
done, and ruled deferred:** running it across two physical machines (the Pi +
dabeest, from `glade-crossing-machines` memory) — Gianni ruled 2026-10-02 that
NAT/firewall traversal is iroh's job, not glade's (he'd already ruled that
2026-09-24), so a second node on one machine is enough for now; the two-machine
run is "a later integration test."

**The claim tie-break (this session's last piece of work, done):** reviewing
the settings-demo design turned up that two nodes which each serve a share
before either has seen the other's claim both mint epoch 1 — and glade had two
separate folds of the claims that broke that tie in opposite directions
(`mesh::route::who_serves` kept the first claim seen; `Registry::who_serves`,
which `Server::serves`/the assembly host port use, kept the last). Fixed as one
shared ranking, `registry::rank_claims` (highest epoch, then lower node id),
used by both folds. Test-first (`registry.rs`,
`an_equal_epoch_is_held_by_the_lower_node_id_in_every_fold`), gate 9/9 at 491
tests, all six downstream suites at their expected counts, rustfmt baseline
lowered 253→252, clippy 10→9. Commits: glade `c1a6764` (fix) + `e3cf1fe` (plan
note); root `a4a8550`. **Not pushed; desk not rebuilt with it.**

### 4.2 The settings demo — designed and ruled, not built

Gianni's ask (2026-10-02): the Gyld desk's zoom/font/theme/wallpaper settings
should follow the user across sessions (already true on the Gyld desk, see
§6), and a session **duplicated in two browsers** should also share its window
layout and grid locations (not true today — each browser keeps its own
localStorage blob). He asked for every grip in gryth-ui to be enumerated and
scoped.

**Delivered:** `gryth-wz-appearance/dev-docs/GrythGripScopes.md` (lane
`appearance`, commit `43fb722`, not merged into gryth-wz main). Every one of
gryth-ui's 214 `defineGrip` calls (216 grips at run time) has a stated scope.
Headline:

- New **session** scope, between environ and instance: one desk, shared by
  every page that opens it. 36 grips land there, mostly the 12 desk-layout
  grips (currently environ, in the per-browser blob) and 24 per-tab "what does
  this window show" grips (currently instance, inside tab records).
- **environ narrows to just the user's settings** (already roamed on the Gyld
  desk via the Glial appearance plan, merged `gryth-ui b93ebe3`/root `e5b8b48`,
  2026-09-28) — no grip changes needed there.
- A page would name its session with `?session=<name>`, read beside the
  existing `?principal=`.

**Gianni ruled "yes to all"** on the document's 8 questions (session scope:
yes; naming: `?session=`; storage: one `<entry>.desk` value per user+session on
`ws-razel`; `Desktop.Current` session-scoped, focus stays instance; tab
records re-applied live; the reveal-flash cue moved off the shared record
first; `Gyld.Set` session-promotable-to-doc; the demo runs two nodes on one
Mac). The tie-break above was fixed first, per his explicit instruction,
because the demo's two gyld-ui nodes both serving `ws-razel` is exactly the
scenario that bug hits.

**Not started: the implementation.** Per standing rule, a plan request gets a
phased dev-docs plan before code. The natural next step is a
`GladeSettingsSessionPlan.md` (or similarly named) phased plan in whichever
repo's dev-docs fits best — the work spans gryth-ui (grips, UI) and possibly
glade (none expected — `ws-razel` and the claim model already support it).
That plan has not been written. Candidate first steps, from
`GrythGripScopes.md` §3.3–3.4:
1. Record the new session scope in `GrythVision.md`'s tiers and against
   GDL-030.
2. Move the reveal-flash cue (`WindowRecord.attention`) off the shared desk
   record onto a per-page grip — do this *before* the desk record becomes a
   session value, or two browsers will fight over clearing it.
3. The `<entry>.desk` session value + `?session=` parsing.
4. Move the 12 layout grips from the blob to that value.
5. Re-apply a changed tab record live (today `tabContexts.ts` seeds once).
All build in the `appearance` lane; Gianni merges.

## 5. Everything else pending from earlier in the session

- **taut's generated-bounds feature** (`max_encoded_len`) is deliberately
  parked — see `taut-dev/dev-docs/TautGeneratedBounds.md`. taut `3c6d07e`
  (the bound + a header change) is public and kept rather than reverted
  ("easier to release a patch than pull it back"); glade realigned to it
  (`b801192`) but doesn't use the bound (`MAX_FRAME_BYTES` is still glade-wire's
  own constant). Doing this for real means doing it for every generated
  language at once, in a taut change, when there's another reason to cut a
  taut release. **A taut patch release is pending — only Gianni runs `gearu
  release`.**
- **The route scan costs ~3 ms per write frame at 1,000 claims** — a cached
  fold is the fix, not urgent, noted in the cross-node-writes plan.
- **A forward's-end race** (a newer forward's handle removed after a link
  release) is noted as carried-over, never observed, in the X3 chunk notes.
- **No publish of grip/glial/glade packages** before an end-to-end app is
  built — ruled 2026-09-24, still stands (see `no-publish-before-e2e-app`
  memory).

## 6. What's already true and working (don't re-derive this)

- The Glial appearance plan (Phases 1–3) is merged into gryth-wz main
  (`b93ebe3`/`e5b8b48`, 2026-09-28): on the Gyld desk, with a named
  `?principal=`, the four appearance settings are one roamed glial value
  (`gyld.appearance` on `ws-razel`, private zone, key `self:<principal>`).
  The full desktop still keeps them in its blob (ruled "later," question 9).
- 4.6 (the fixed-peer route) passed all 13 checks, locally and across the Pi
  + dabeest (glade `41bc10d`, 2026-09-30).
- Step R (claims renew on the wall clock, not a sleeping tokio clock) is
  built (glade `a4a023a`) but **not yet in the desk's running binary**.

## 7. Hard rules, recapped (full text in `~/.claude/.../memory/*.md` and
   `AGENTS.md`/`AGENTS_GWZ.md` — this is just the headline list)

- Never touch the desk's ports/PIDs/`~/.gyld-ui`; never restart it yourself.
- Never write directly in `~/limbo/gryth-wz` — use the `appearance` gwz lane
  at `~/limbo/gryth-wz-appearance`; Gianni merges.
- gwz workspaces: use `~/.cargo/bin/gwz`, never plain git, for status/commit/
  push. A member commit auto-stages the root's lock; commit that separately
  with `--target @root`.
- git verbs mean exactly that verb — no bundled tag/push/merge. Never touch a
  released tag.
- No AI attribution trailers in any commit or PR, ever.
- pnpm, never npm.
- Brace every control-flow body; `cfg` only inside a braced `mod` or
  `cfg_if!`.
- Disk is usually tight — delete scratch targets in the same command you
  built them (now less urgent: the move to `/Volumes/projects` freed space,
  `/System/Volumes/Data` currently shows ~99 GB free, vs. ~20 GB before).

## 8. The node-lane review scripts (scratch, may not survive)

This session built a frozen verify harness for glade-node chunk review at
`/private/tmp/claude-501/-Users-owebeeone-limbo-gyld-wz-gyld/820a2550-bd57-4649-b370-1f843ddd65cb/scratchpad/lane/`
(`RULES.md` + `verify.sh`, `fast`/`gate`/`downstream`/`all`/`clean` verbs). It
was still present under that **old** path (keyed to the pre-move working
directory) as of this write. The new working directory produces a *different*
scratchpad path
(`/private/tmp/claude-501/-Volumes-projects-limbo-gyld-wz-gyld/...`), so if a
fresh session can't see the old one, recreate it from `RULES.md`'s description
(it's short — chunk rules + the gwz member-commit procedure + hard rules) or
from this document's §4.1 process description; the expected downstream counts
are: client-rs 37+13+1, client-ts 60, grip-share 19, glade-gwz 10+1+10+1,
grazel 30+3+5+1 (no SKIP), glade-gyld 251 (1 ignored)+1+1+41+1.

## 9. Immediate next actions, in order

1. Ask Gianni whether to push glade (`e3cf1fe`) and the glade-wz root
   (`a4a8550`) — the tie-break fix sits unpushed.
2. Ask whether to write the phased plan for the settings/session-scope
   implementation (§4.2) — his "yes to all" ruled the design, not a build.
3. Note for Gianni: the desk needs a rebuild+restart (his call, his terminal)
   to pick up Step R, all of cross-node-writes, and the tie-break — currently
   invisible to the desk only because it has no peer links yet.
