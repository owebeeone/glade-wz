# PlanGladeUsers — the glade-users supplier build plan

Status: proposed (2026-07-12). Expands GLP-0006 **P2.S1** into a phased build,
but honours the spec's insistence that the **onboarding FLOW is stage-1
buildable NOW** and is the FIRST supplier in the user-flow spine — so stage-1
(Phases 0–4) is NOT gated on the P2 decision wall; only stage-2 enforcement
(Phase 5) is.

Spec (normative): `dev-docs/glade/suppliers/glade-users.md` (all sections incl.
§5 migration, §6 traces, §8 open questions). Common contract:
`dev-docs/glade/GladeSupplierModel.md`. Spine + user-testable-when:
`dev-docs/SupplierOutlines.md`. House plan + conventions: `Plan.md`,
`SupplierRequirements.md`, `Decisions.md` (this dir).

Owner: maintainer (coordinating); agent-parallel, repo-disjoint by construction.
Prereq: GLP-0006 P0 complete (supplier kit, rust + TS clients, typed manifest,
demo tab chassis, grazel skeleton, principals-minimal P0.S7) — all landed.

## Goal

Build **glade-users**, the first foundation supplier: **key-canonical identity,
invite→onboard, and the two-freds name discipline** as data (stage-1), then the
grant/lifecycle/enforcement half (stage-2, gated on WD-1 + AZ-1/2/3). The
north-star landing for this supplier is the user-testable line from the
outlines: *I mint an invite, a second real person opens it on another machine,
onboards with their own key (or their SSH key from a CLI), and both of us appear
in the same user list — regardless of who invited whom first; a name collision
renders discriminated.* That two-real-machines line — not a green suite — is the
phase-exit gate.

## What exists (build ON this; verified 2026-07-12)

- **PrincipalRecord v1** = `{principal: String}` (CBOR tag 1) —
  `glade/node/src/sysdata.rs` (GENERATED) from `glade/node/ir/sysdata.taut.py`
  (regen command in that file's header: `taut.cli gen … --api-only
  --legacy-codec`; NEVER hand-edit the `.rs`). The string IS the id today.
- **`dir.principals` fold + auto-append** — `glade/node/src/claims.rs`
  `note_principal()` / `knows_principal()`: a Hello naming an unknown string
  principal auto-appends a v1 record (identity as data, nothing enforced).
  `G_PRINCIPALS = "dir.principals"` and the `Record::Principal` kind are in
  `registry.rs`; `grants_for(principal, share)` folds STRING-keyed grants.
- **`Hello.principal` wire field** — `client-ts/src/client.ts` `hello(principal?)`
  and `client-rs/src/client.rs` `hello(Option<&str>)` both send it; absent =
  origin-as-identity.
- **Supplier kits** (P00-a wire-attached authority sessions):
  - **rust** `glade/client-rs/src/supplier.rs` — `Supplier::{serve_exchange,
    serve_share}`, `ShareController::{set,append}`. The **exchange-provider loop
    is real and live-proven** by glade-gwz (`on_exchange_req` / `respond_exchange`,
    corr 1:1).
  - **TS** `glial/src/supplier/index.ts` — `attachSupplier`, `serveExchange`,
    `serveShare`. **BUT** `SupplierSession.onExchangeReq` / `respondExchange` are
    still stubbed **INTEGRATION POINTS**: "client-ts today decodes tag-6 frames
    but drops them." A TS supplier can serve LOG/VALUE surfaces today; it CANNOT
    serve an EXCHANGE surface until that client-ts path is completed.
- **Typed manifest (compile wall)** — `glial/src/manifest.ts` `defineManifest`;
  a `Surface` is a `BindingDecl` + `share`/`key`; an undefined id is a TS build
  error.
- **grazel bootstrap (invite-URL landing)** — `grazel/src/{lib.rs,main.rs}`:
  tiny_http static server + `GET /bootstrap.json` (`bootstrap_json(node_ws, mode,
  name)` — grant/handoff fields "arrive with P2"). App-owned storage is the
  `<data>/files` slot (the data seam); grazel owns invite secrets. grazel
  composes a child supplier exactly as it composes glade-gwz (P1.S3 notes).
- **Demo tab chassis** — `glade/demo/src/tabs.tsx` `TABS[]` (adding a tab = one
  entry). Identity flow: `demo/src/glade.ts` `client.hello?.(user)`; `user =
  params.get("user") ?? origin` (`demo/src/glial.ts`) — the `?user=` stub this
  plan replaces with a minted/presented principal. `demo/src/GwzPanel.tsx` is the
  reference for a TS panel consuming a RUST supplier over the wire.
- **Two reference supplier shapes**: **glade-chat** (TS on glial, log surfaces,
  repo `glade-chat`) and **glade-gwz** (rust on client-rs, exchange + log, repo
  `glade-gwz`). glade-users follows glade-gwz.
- **Traces**: `ggg-viz/src/scenario/` — `SCENARIOS` registry in `index.ts`;
  invariants `INV-1..5` in `invariants.ts`; **s-grant / s-idp / s-admin already
  exist** (`authz.ts`, `admin.ts`) as the stage-2 half. The three stage-1 traces
  (s-invite / s-name-clash / s-converge-identity) are being AUTHORED in a
  parallel wave NOW (see Phase 0).

## Repo plan (GDL-040 + the collision discipline)

- **New member `glade-users`** — repo `git@github.com:owebeeone/glade-users.git`,
  member path `glade-users`, added to `gwz.conf/gwz.yml` via **gwz member-add**
  (never `git clone`, never hand-edit the workspace file — the workspace-local
  convention, mirrors `glade-gwz`/`glade-chat`). README first line: **"a glade
  supplier: identity, onboarding, and access lifecycle."** (Repos say WHAT,
  README says ROLE — GDL-040; never `supplier-*`.)
- **Language = RUST on `client-rs`** (like glade-gwz). One-line rationale: the
  `users.invites` EXCHANGE provider loop is proven only in client-rs, while
  glial/client-ts's exchange-provider path is still a stub — and invites
  intersect grazel's rust HTTP bootstrap + app-owned invite-secret storage, so a
  rust supplier shares token types/validation with grazel; the browser/CLI accept
  and the user list are TS/CLI *consumers* regardless (P00-a: supplier language
  is orthogonal to consumer language, exactly as TS `GwzPanel` consumes rust
  glade-gwz).
- **Off the critical path (flagged, not owned here):** completing client-ts's
  `onExchangeReq`/`respondExchange` (glade repo) would let a *future* TS exchange
  supplier exist; it is independently valuable but MUST NOT gate glade-users.
- **The glade single-writer trap (matters for waves):** the `glade` repo holds
  BOTH the node IR (`node/ir/sysdata.taut.py`, `claims.rs`, `registry.rs`,
  `appdecl.rs`, `apps/grazel-app.glade`) AND the demo (`demo/src/*`). So every
  Phase-1 (node-IR) step and every demo-tab step (P3.S2, P4.S2) are the **same
  repo** — one glade agent, sequenced across waves; never two parallel agents on
  glade. Disjoint repos (`glade-users`, `grazel`, `ggg-viz`) run parallel to that
  glade lane; `gryth-ui` (gryth-wz) is Gianni-integrated.

## Decision gates (map §8 + WD-1 to the steps they actually block)

Stage-1 flow is deliberately clear of the P2 wall. Only Phase 5 sits behind it.

| Gate / open question | Owner | Blocks | Does NOT block |
| --- | --- | --- | --- |
| **WD-1 root custody + recovery** | Gianni | **Phase 5** only (real ed25519 root semantics, key rotation, recovery, admin revocation) | Phases 0–4 (stage-1: structural sigs, browser device keys, SSH-present) |
| **AZ-1/2/3 v1 scoping** | Gianni | **P5.S2** (grants gate, path scoping) | Phases 0–4 |
| §8 Q: **`users.names` v1 or later?** | Gianni | **P4.S1** *enablement* (whether a domain turns on the handle registry) | Phase 3 convergence + **P4.S2** (petname + fp-suffix are correctness-sufficient on their own; the fp-suffix + petname half ships regardless) |
| §8 Q: **invite expiry / one-shot default + accept countersign** (AZ-14 adjacent) | Gianni | the exact policy inside **P2.S2** | the FLOW — P2.S2 ships on a **chosen faithful default** (single-use token, N-hour TTL, no countersign in v1); surface the default, don't gate on it |

Rule: a stage-1 step that *touches* an open question picks the smallest faithful
default and records it (the glade-gwz allow-list precedent), so the flow lands;
the ruling later swaps the default with no consumer rewrite.

## Phases

Aspirational <500 hand-written LOC per step. Every step is **trace →
build → demo/CLI → live-verify**; each is tagged with its owning repo.

### Phase 0 — Traces landed (atlas leads; CONSUME + reconcile)

The three stage-1 traces are being authored in a parallel wave NOW; this phase
does not author them — it consumes and reconciles them so build may begin.

- **P0.S1 — consume + reconcile the landed traces.** `[ggg-viz]` `stage-1`
  Confirm **s-invite** (3.1 end-to-end: mint → URL → key-present/mint → records →
  both sides converge; replayed/expired fails as data), **s-name-clash** (second
  fred loses the domain handle, renders fp-suffixed; petname override is the
  viewer-local fix), and **s-converge-identity** (u1/u2/u3 sequence-independence)
  are registered in `SCENARIOS` and green. Wire the candidate **INV-6** in
  `invariants.ts`: *no two principal records with one fingerprint after fold*
  (the set-union dedup keyed by fingerprint, §1.2). If glade-users will add
  surfaces to `grazel-app.glade`, note the stale-`s-app-register` binding-count
  reconcile (the P1.S3 flag) as a rider here.
  - **Stage boundary:** s-grant/s-idp/s-admin (already present) are the Phase-5
    spec — untouched here.
  - **Verify:** ggg-viz suite green with all three traces + INV-6. **Reconcile
    check:** the traces' record kinds / surface ids / fold-key MUST match this
    plan's Phase-1 shapes — any disagreement is a **design event** surfaced to
    Gianni, not silently resolved either way.
  - **Parallel:** disjoint repo; overlaps Phase 1.

### Phase 1 — Identity records + the v1→v2 migration (node IR)

Milestone: `dir.principals` folds key-canonical v2 records beside v1 legacy;
introductions + name-claims route. **Single-writer `glade` lane** — S1 then S2 by
one agent.

- **P1.S1 — PrincipalRecord v1→v2 migration (its own step; §5).** `[glade]`
  `stage-1` Add a **v2 record kind BESIDE v1** in `sysdata.taut.py` (new fields:
  `fingerprint`, `profile`, `sig`; the v1 string demoted to display-name),
  regen `sysdata.rs` with the documented command. **Do NOT reinterpret v1** (the
  ChatLine lesson) — v1 stays a distinct kind. **The fold key changes
  string→fingerprint for v2:** `note_principal`/`knows_principal` (claims.rs)
  dedup v2 by fingerprint (INV-6); the key-less string-Hello path keeps minting a
  **v1 "unverified legacy" record** (string-keyed, rendered discriminated). The
  directory fold now yields a union {v1 legacy string-keyed, v2 fingerprint-keyed}.
  - **Stage boundary:** `grants_for` STAYS string-keyed here — grant-key
    migration to fingerprint is stage-2 (P5.S2), not stage-1 (grants are allow-all
    now). Signatures are structural (crypto seam stubbed, node posture).
  - **Verify:** node cargo green; corpus-gate the taut change (breaking=0);
    a v1 and a v2 record coexist in `dir.principals` and fold without collision;
    a legacy v1 renders "unverified legacy". Two v2 accepts of one key dedup to
    one principal (INV-6, unit).
- **P1.S2 — IntroductionRecord + name-claim kinds + surface decls.** `[glade]`
  `stage-1` Add `IntroductionRecord` (inviter-fp → invitee-fp sponsorship edge)
  and the name-claim record to `sysdata.taut.py`; regen. Declare the surfaces in
  the registry + `grazel-app.glade`: `users.introductions` (log, commons),
  `users.names` (log, commons — OPTIONAL per domain). Carry the
  `appdecl`/`exchange` binding-count reconcile the app-file growth forces (the
  P1.S3 pattern that touched 3 node tests).
  - **Stage boundary:** records + surfaces only; no fold enforcement.
  - **Verify:** node cargo green (binding-count assertions updated); the three
    surfaces route; a client subscribing sees appends fold.
  - **Sequential** after S1 (same repo). Disjoint from Phases 0/2 scaffolding.

### Phase 2 — The glade-users supplier (stage-1 flow as DATA)

Milestone: the supplier stands up, serves the directory, and the invite ceremony
round-trips over the wire. **`[glade-users]` — new repo, fully disjoint;** its
SCAFFOLD (S1) may start during Phase 1, its directory-serving needs Phase-1
surfaces declared.

- **P2.S1 — repo scaffold + directory serving.** `[glade-users]` `stage-1`
  New member (README "a glade supplier: …"), rust on `../glade/client-rs` +
  `glade-wire` (glade-gwz's dependency shape). Serve `dir.principals`,
  `users.introductions`, `users.names` as **log surfaces** (`serve_share`),
  read-through folds. CLI mirrors glade-gwz (`--node ws://… --share … --principal
  … --data DIR`).
  - **Stage boundary:** serves + reads only; no enforcement.
  - **Verify:** `cargo test` green + clippy clean (glade-gwz bar); against a
    booted node, a subscriber sees principal/introduction/name records fold.
  - **Parallel:** disjoint repo (overlaps Phase 1 for scaffold; serving waits on
    P1.S2 surfaces).
- **P2.S2 — the invites exchange (mint → accept ceremony).** `[glade-users]`
  `stage-1` Serve `users.invites` (**exchange**, `serve_exchange`):
  - **mint** → `InviteRecord{nonce/token, inviter-fp, target-domain grants-to-be?,
    expiry}` + a joinable URL payload (lands on grazel's bootstrap, §3.1.1);
  - **accept** → validate token (not-expired, not-replayed), then append
    `PrincipalRecord` v2 (key presented or minted) + `IntroductionRecord`
    (sponsorship edge). §3.2: introducing an EXISTING key is the same flow minus
    key-mint (the fingerprint is already carried) — becomes an introduction edge,
    not a duplicate.
  - Failure as DATA (§4/§6): expired/replayed/absent → `ExchangeRes{ok:false}`.
  - **Stage boundary:** token/expiry policy = the chosen faithful default
    (single-use, N-hour TTL, no countersign — see the decision gate); signatures
    structural; the invite's `grants-to-be` are **recorded but NOT enforced**
    (they become real grants in P5.S2).
  - **Verify:** mint→accept round-trips over the wire (glade-gwz's live-verify
    style); a replayed/expired token fails as data; **two accepts of one key dedup
    to one principal** (INV-6, the §1.2 convergence proven at the supplier); the
    result matches s-invite's shape.
  - **Sequential** after P2.S1 (same repo).

### Phase 3 — Onboarding ceremony end-to-end (the user-testable spine)

Milestone / **PHASE-EXIT GATE (user-testable, two real machines):** machine A
mints an invite; machine B opens the URL and onboards with ITS OWN key (browser
device key OR CLI SSH key); **both principals appear in the SAME user list on
both sides, regardless of who invited whom first** (the u1/u2/u3 convergence,
live). This is the outline's user-testable-when clause 1 — not a green suite.

- **P3.S1 — invite URL lands on the bootstrap + grazel composes the supplier.**
  `[grazel]` `stage-1` A `/join?token=…` route (or an `invite` field threaded
  through `bootstrap.json`) carries the token into the SPA's session placement
  (GDL-032). grazel owns invite-secret app storage (`<data>/files` — the data
  seam) and composes glade-users as a child supplier (the glade-gwz compose
  pattern: spawn, actual-node-port wiring, optional/loud-skip, SIGINT teardown).
  - **Stage boundary:** placement + composition; no grant handoff yet (that field
    "arrives with P2" per the bootstrap doc-comment).
  - **Verify:** opening an invite URL delivers the token to the client bootstrap;
    grazel integration test round-trips a mint→accept through the composed
    supplier; clean shutdown tears down node + supplier.
  - **Parallel:** disjoint repo (grazel); overlaps P3.S2/S3.
- **P3.S2 — the Users demo tab + browser accept / device-key mint.** `[glade]`
  (demo) `stage-1` One `TABS` entry "Users": a **user list rendered per §2**
  (`name·fp6`, never a bare colliding name), an **invite-mint** button, and the
  **accept flow** (browser device keypair minted via webcrypto — structural in
  stage-1; root certification can follow later from a CLI, §3.1.2). Replace the
  `?user=` stub: `glade.ts` hellos the minted/presented **principal** instead of
  the origin string. TS on glial taps (mirrors `GwzPanel` consuming rust
  glade-gwz).
  - **Stage boundary:** the list + mint + accept as data; no gating.
  - **Verify:** in-browser, mint an invite → open the URL in a second browser →
    mint a key → both principals appear in the list on both sides.
  - **Sequential** after Phase 1 (same glade repo); after P3.S1 for the real URL
    round-trip.
- **P3.S3 — CLI SSH-root accept (the second real machine, from a terminal).**
  `[glade-users]` `stage-1` A companion CLI `glade-users accept --url … [--ssh-key
  …]` that presents an existing ed25519 **SSH key as the root** — signing through
  ssh-agent, no private-key extraction (the git ssh-signing precedent; structural
  in stage-1). Satisfies the "or their SSH key from a CLI" clause of the
  user-testable line.
  - **Stage boundary:** presents/records the root; real signature verify is P5.S1.
  - **Verify:** from a real second machine's terminal, accept an invite; the
    principal appears in the browser list on machine A (cross-surface convergence).

### Phase 4 — Names + the two-freds discipline

Milestone / **PHASE-EXIT GATE (user-testable):** a name collision (two freds)
renders **discriminated** (`fred·fp6`) on both machines; a **petname override** is
shown as the viewer-local fix. Outline user-testable-when clause 2.

- **P4.S1 — `users.names` first-valid-claim-wins fold.** `[glade-users]`
  `stage-1` The claim registry fold (deterministic earliest-by-chain-order; ties
  cross-origin resolve by the existing lamport/origin order). Second claimant of a
  handle in a domain loses it and falls back to `fred·b7c9`. Ordinary
  records + a fold — NOT consensus (§2.4: glade already has the primitives).
  - **Stage boundary:** **enablement gated** on §8 (`users.names` v1 or later?) —
    build the fold; whether a domain TURNS IT ON is config. Correctness does not
    depend on it (P4.S2 covers the fallback).
  - **Verify:** two claims for "fred" → second loses; s-converge-identity's
    dedup unaffected (names are attributes, identity is the key).
- **P4.S2 — petname override + the `name·fp6` rendering rule.** `[glade]` (demo)
  `stage-1` Petnames stored in the user's OWN account domain (local, never
  replicated); a petname ALWAYS wins in that user's UI. Enforce `name·fp6`
  everywhere principals can collide (the discipline is normative, not cosmetic —
  §2.2). This is the correctness-sufficient half that ships even if `users.names`
  stays "later".
  - **Verify:** s-name-clash live in the demo — two freds render discriminated; a
    petname override is the viewer-local fix, on both machines.
  - **Sequential** after P3.S2 (same glade repo).
- **P4.S3 — reconcile s-name-clash to the shipped rendering.** `[ggg-viz]`
  `stage-1` If the trace drifted from the shipped fp6/petname behaviour, reconcile
  (disjoint repo).

### Phase 5 — Stage-2 lifecycle + enforcement (GATED on P2-gate: WD-1 + AZ)

The ONLY phase behind the wall. Everything above ships without WD-1. s-grant /
s-admin / s-idp are the spec here (already authored). Sequenced with GLP-0006
P2.S3 (the node-side `check()` switch-on).

- **P5.S1 — real ed25519 verification end-to-end.** `[glade]` (node) `stage-2`
  Swap the stubbed crypto seam for real signature verify (`PrincipalRecord.sig`,
  invite countersign, name-claim sig). **Gated on WD-1** (what a root signs +
  how recovery works).
- **P5.S2 — grants gate + `check()` + grant-key→fingerprint.** `[glade]` (node)
  `stage-2` `check()` on commons joins; invite `grants-to-be` become enforced
  `CapabilityGrant`s at accept; migrate `grants_for` from string- to
  fingerprint-keyed (the deferral from P1.S1). **Gated on AZ-1/2/3.**
- **P5.S3 — lifecycle verbs.** `[glade-users]` `stage-2` grant / attenuate /
  revoke (s-grant/s-admin), ancestry-based admin revocation, device-cert
  add/remove, key rotation (new-key-certified-by-old + alias — the GQ-6 rename
  idiom applied to identity, §3.3). **Gated on WD-1.** Demo tab gains a grant
  editor.

## Discipline (unchanged from GLP-0006)

Trace leads build; the demo / two-machines flow is the live gate (not green
suites); the supplier never imports node internals (wire-attached, P00-a);
commit-per-step with every owned repo's gates green (node cargo, glade-users
cargo+clippy, glial, client-ts, ggg-viz, demo build); no attribution trailers;
pnpm never npm; tests never touch the real `~/.glade` (temp GLADE_HOME/HOME, the
client-rs harness pattern); corpus/invariant gate red = design event. **Single
writer per repo per wave** — and remember `glade` = node IR + demo (they
serialize).

## Parallelism map

```
Phase 0  [ggg-viz]  ── consume+reconcile traces (overlaps P1)
Phase 1  [glade]    ── S1 → S2 (one agent; node IR lane)
Phase 2  [glade-users] ── S1(scaffold overlaps P1) → S2 (invites; disjoint)
Phase 3  [grazel] S1 ‖ [glade] S2 (after P1, after grazel S1) ‖ [glade-users] S3
Phase 4  [glade-users] S1 ‖ [glade] S2 (after P3.S2) ‖ [ggg-viz] S3
Phase 5  gated (WD-1 + AZ): [glade] S1→S2 · [glade-users] S3
```

Repo-disjoint lanes ([glade-users], [grazel], [ggg-viz]) run parallel to the
serialized [glade] lane throughout; gryth-ui wiring (gryth-wz) trails as Gianni
integrates, one plugin consuming the same surfaces (the P1.S4 pattern).
