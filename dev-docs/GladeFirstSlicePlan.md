# Glade First Slice Plan — the fixed-peer route under the eleven rulings

Status: **DRAFT plan**, 2026-09-22. Lane: glade-wz. Written against glade-wz root
`9d19ee6`, glade `559cb2c`, glade-decl `d671f10`, glade-decl-rs `21eefa1`,
glade-discover `fd94a1f`; rulings stream `rulings@1`, snapshot `d25aa2058723`.
Nothing here is implemented by this document; every "today" statement below was
read in the tree at those revisions on 2026-09-22. Phase 2 was refreshed on
2026-09-23 against the accepted reconciliation (revision 4, `3b60234`) and the
rulings recorded at `a0e6ce4` and `972d4d0`; its statements were read then.

## 0. What this plan delivers, and why now

The decision graph has no answerable question left (desk build
2026-09-22T05:00:52Z, "0 answerable now"). What it waits on is one event, the
trigger `first_real_route_slice` — "the fixed-peer iroh route slice from
GladeBuildEntry passes" (`gyld/examples/glade-decisions.gyld.py:756-761`) — which
makes `simulator_tooling` answerable at once (`:626`) and `registry_growth`
together with `failure_model` (`:590`). This plan delivers that slice as
[GladeBuildEntry.md](GladeBuildEntry.md) defines it (`:37-70`), under the eleven
rulings the owner recorded on 2026-09-21/22 (glade-wz
`decisions/glade-decisions-rulings.gyld.py` at `9d19ee6`).

The build entry's three steps are the spine (`:47-60`): pin the slice contract
and write executable consumer tests; build the smallest injectable assembly on
deterministic providers; replace the boundary providers with real adapters,
ending with a fixed-peer iroh route. Its acceptance sentence is the slice's
definition of done (`:62-66`):

> The final slice passes only when a real registration is discoverable across the
> selected peer route, expired/unauthorized entries are excluded, loss/retry/restart
> outcomes are honest, and the fast independent development path is demonstrated.
> Exact source proof/encoding incompatibilities are explicit blockers to the affected
> adapter—not permission to substitute synthetic evidence in the real demonstration.

The rulings say what each step must contain (§1). Phase 1 is the held review loop
on the declaration contract, resumed, by the owner's instruction of 2026-09-22.

What this plan does not touch: the wire IR (`taut/corpus/glade.ir.json`, a
different frozen contract — `GladeDeclReconciliation.md:89-101`); the running
demo and the owner's instance on ports 5173/8080/9099; the five uncommitted
dev-docs files in `glade-discover` (someone else's, left alone); the build entry's
out-of-scope list (`:72-78`).

Standing rules that bind every step: a phase is a milestone and a step is one
goal, aspirational < 500 LOC; foundational first, parallel where the coupling
allows (§2); one commit per step through gwz, member first then the root lock; no
attribution trailers; agents on Opus (the owner's quota instruction) and reviewer
tiers as Step 1.1 records; control-flow bodies braced and `#[cfg]` only inside
`cfg_if!` or a platform module; the boundary policy
[LibraryBoundaryAndTestingPolicy.md](LibraryBoundaryAndTestingPolicy.md) —
every behaviour change begins with a failing test (LBT-007, `:38`), pure-library
tests are deterministic and start no node, network or executor (LBT-008, `:39`),
every implementation runs its contract's shared conformance suite (LBT-009,
`:40`), each package exposes a measured fast-test command (LBT-010, `:41`),
public-contract changes test affected consumers (LBT-011, `:42`), and no public
boundary carries transport, database or runtime types (LBT-004, `:35`). Receiving
that policy authorises no refactor (`:108`); the build entry says new assembly
and adapters start *alongside* the demo, with no bulk extraction (`:80-81`).

## 1. The rulings as constraints

| Ruling | Selected | What the slice must contain | Lands in |
| --- | --- | --- | --- |
| `lifecycle_composition` | `sdax_rs` | start-up and cleanup owned by an sdax plan; every spawned task owned; reverse-order release; `report.incomplete` is the signal | 3.3 |
| `async_witness` | `shaku_confirmed` | Shaku at assembly only; every real provider in a test composition overridden or lazy; a handle whose close consumes it is given up by value | 3.1, 3.2 |
| `dissemination` | `sync_round_only` | one pull each way plus a best-effort push; no gossip crate | as built (`mesh.rs:405-433`, `:371-397`, `:233-249`; no gossip in `node/Cargo.lock`); 4.6 runs it over the route |
| `transport_key_binding` | `binding_record` | `NodeTransportBinding{node, endpoint_id, valid_from, sig}` in the home share; the HELLO `node_id` bound to `remote_id()`; unknown keys refused at accept (`IrohGladeMapping.md:387-395`) | 4.2 |
| `metadata_exposure` | `granted_shares_only` | a grant check at the serve hop, failing closed; the directory exempt; a revocation cuts a live stream | 4.3 |
| `relay_posture` | `community_dev_only` | `RelayMode` from configuration; n0's relays for the slice; no lookup service; endpoint ids private until 4.2 lands | 4.5 |
| `proof_family` | `taut_grants` | grants stay Glade's own CBOR records, signed under the node chain; no outside verifier | 4.1 |
| `identity_adapters` | `none_in_v1` | no OIDC, Kerberos or SPIFFE; principals by hand | nothing to build; confirmed absent from `node/src` |
| `key_custody` | `recovery_keys` | the node key minted at first setup; recovery material offline | 4.1; the rotation gap is recorded in 5.1 |
| `version_pin` | `bump_to_current` | iroh 1.2 floor; every 0.x crate pinned exactly | as built (`node/Cargo.toml:25`, resolved 1.2.0); sdax pinned by revision in 3.3 |
| `scope_model` | `node_trust` | node↔node by operator trust; the serve hop enforces per-session grants | 4.3 |

What the node has today, one line each (inventory of 2026-09-22): none of sdax,
Shaku, a binding record, an accept-time check, a grant consult on any serve path,
a relay configuration path, a signature, or a shutdown path exists. Composition
is a straight line in `glade/node/src/bin/glade-node.rs:48-142`. Tasks start at
nine bare `tokio::spawn` sites in `mesh.rs` (`:124,131,166,175,178,193,241,269,
328`) and one unbounded renewal loop (`claims.rs:115`); no `JoinHandle` is kept
and `PeerEndpoint::close` (`iroh_carrier.rs:138-140`) is never called outside
tests. The four paths that hand data out with no grant consult are
`peer.rs:193` (`serve_sync`), `mesh.rs:264` (`serve_peer_subscribe`),
`exchange.rs:232` (`serve_peer_exchange`) and `server.rs:215` (websocket
subscribe); `grants_for` (`registry.rs:366-387`) has zero production callers.
Three seams are pre-shaped for signing with no wire change: `peer.rs:99`
(`verify_peer`, accepts anything), `peer.rs:143` (`verify_origin_sig`, returns
true), `sysdir.rs:263` (local overlay self-signature). The endpoint recipe is
hardcoded to `presets::Minimal` on loopback (`iroh_carrier.rs:32-40`, `:103`),
and every two-node test binds `127.0.0.1`. The node key lives at
`<instance>/node.key`, mode 0600 (`sysdir.rs:190-203`), and `NodeId` is
`sha256(key)`, named in the code as a stand-in for the ed25519 public key
(`sysdir.rs:213-216`).

## Phase 1 — The declaration contract accepted (the held review loop, resumed)

Milestone: `dev-docs/glade/GladeDeclReconciliation.md` holds GO from all three
axes on one revision, and the owner's rulings R1–R11 are recorded, so the
amendment in its §4 is a single edit.

Where the loop stands (`GladeDeclReconciliation-RemPlan.md:100-122`): revision 1
(`b132b7e`) received three NO-GO verdicts, filed verbatim at `879de50` —
Consistency 5 P2 + 6 P3, Safety 2 P1 + 3 P2 + 3 P3, Surface 5 P2 + 5 P3 — with
five blind convergences (`:19-27`). Revision 2 (`1defe3b`) is the single
remediation patch, with a closure map by finding ID
(`GladeDeclReconciliation.md:24-46`). The re-verdicts were never dispatched; round
accounting is round 1 of at most 2 (`-RemPlan.md:92-98`). The one standing
precondition, the red drift gate (A2), was closed by the owner's ruling between
rounds: glade-decl `d671f10`, glade-decl-rs `21eefa1`, root lock `dc311ba`; the
contract bytes did not move (`GladeDeclReconciliation.md:50-62`).
`corpus/build.py --check` exits 0 today: "all 3 glade-decl artifacts in lockstep
with the schema."

### Step 1.1 — Settle the round-2 tuple and the tier

Goal: one recorded tuple and one reviewer tier before anyone reads anything.

- Tuple: the object is `GladeDeclReconciliation.md` at glade-wz root `1defe3b`,
  unchanged since (`git log 1defe3b..9d19ee6 -- dev-docs/glade/GladeDeclReconciliation.md`
  is empty; the root moved only decisions and arch1 documents); glade-decl
  `d671f10`; glade-decl-rs `21eefa1`; glade `559cb2c`; glade-decl-ts `7e16e32`;
  taut `7a5f616`; every other member as the round-1 reports' §0 lists it. Every
  member tree is clean except `glade-discover` (five dev-docs files, named out of
  scope). The gate line above is recorded with the tuple.
- Tier: the RemPlan records that round 1 ran one tier below the session's at the
  owner's instruction and leaves the tier to the resuming owner (`:120-122`).
  Default here: Opus for the re-verdicts (the standing quota instruction), any
  P0 or P1 adjudicated at the session's own tier. The owner overrides in a word.
- Lands: a dated "Round 2 — tuple and tier" block appended to `-RemPlan.md`.
  ~20 lines, one commit. No other file.
- Done when: the block names every revision, the gate result and the tier.

### Step 1.2 — Round-2 re-verdicts, three axes, peer-blind

Goal: a filed verdict per axis on revision 2.

- The round-1 reviewers' contexts are gone (this session was compacted), so each
  axis gets a fresh reviewer built from the canonical template
  (`~/.claude/skills/review-loop/references/review-prompt-template.md`): the
  prompt body, that axis's role section, the tuple from 1.1, `-RemPlan.md`, that
  axis's own round-1 report, and the diff of the object `b132b7e..1defe3b`. Never
  another axis's report, never the lane owner's suspicions.
- Each report opens with the prior-finding closure table — one row per round-1
  ID of that axis; "verified" means the original counterexample re-traced on
  revision 2 — and the changed-range analysis, in which any new architectural
  root cause is labelled as such; the cap turns on that label and it is the
  reviewer's call (template `:175-195`).
- Two drafter departures must be ruled on by the raising axes (`-RemPlan.md:113-116`):
  SAF-P1-1 asked the revision to choose a profile treatment and revision 2 lays
  out R4(a)/(b)/(c) with a recommendation instead; CON-P2-4's zone validation
  became an R1 sub-choice rather than a twelfth ruling.
- Reviewers are read-only, verify the tuple at start and end, and return the
  whole report as their final output.
- Lands: `GladeDeclReconciliation-ReviewConsistency-2.md`, `-ReviewSafety-2.md`,
  `-ReviewSurface-2.md`, verbatim, one commit.
- Done when: three reports, each with a GO or NO-GO, are filed.

### Step 1.3 — Verdict merge; remediation round 2 only if needed

Goal: GO on all three axes on one revision, or a stop.

- GO ×3: the document's status line becomes
  `accepted at <tuple> after <three files> reported GO; this accepts the assessment and the amendment shape only`,
  and 1.4 follows.
- Any NO-GO: `-RemPlan-2.md` maps every blocking finding to one disposition and
  one closure test; ONE patch produces revision 3; re-verdicts come from the same
  round-2 reviewers (context intact) and are filed as `-Review<Axis>-3.md`. This
  is remediation round 2 of at most 2.
- Stop rule: a reviewer-classified third architectural root cause on the object
  ends the lane; the report to the owner is redesign-or-accept and no further
  patch is drafted.
- Done when: the status lines in the document and the RemPlan say accepted, or
  the stop is reported.

### Step 1.4 — The owner rules R1–R11

Goal: every open choice in §3 of the document has one answer, recorded where the
amendment reads it.

- The eleven (`GladeDeclReconciliation.md:430-799`): R1 `BindingDecl.domain` and
  `DomainAnchor`; R2 the retention vocabulary and its enforcement; R3 `message`
  and `window`, delete or reserve; R4 the shape profile on `BindingDecl` —
  (a) a field, which rewrites 11 of 26 vectors, (b) a separate message keyed by
  glade id, (c) defer; R5 `Authority.external` and `BindingDecl.source`; R6
  `canonical_key` and `derive_glade_id` before the publish; R7 which unexercised
  elements ship (`AdvertisementRecord` only); R8 the whole `.glade` form or
  bindings only; R9 what a changed or deleted binding line does to a registered
  declaration; R10 the `glade-app v1` header; R11 a keyword tail on the line.
- Order that respects the document's own precedence (`:398-414`): R1, R2, R3, R4
  first (independent of each other); then R9 (constrains R2's landing), R10
  (constrains R1's and R2's landing), R11 (subordinate to R2 and R4); R5–R8 at
  any time.
- Recorded as a dated `RULED (gianni, <date>): <option>` line under each R heading
  in the document's §3, the convention `GladeDiscoveryModel.md` already uses.
  The desk is not used for these: they are eleven contract choices inside one
  document, framed there and read there by the amendment, and putting them on
  the graph would mean editing the base graph for choices it does not ask.
- Done when: eleven RULED lines exist and §4.0's landing table needs no option
  branches.
- **Done, 2026-09-23.** Every recommendation, and b2 within R9(b), recorded as
  RULED lines in the document's §3, with §4.0 stating how every conditional step
  and §4.7(b) gate resolves. Two questions the answers did not reach were ruled
  the same day and are recorded in §3: an app file writes the hyphen only
  (`from_cursor` in a file is warned for one release, then refused), and
  `canonical_key`'s owner is Gianni.

Phase 1 exit: accepted status plus eleven rulings. Nothing in glade-decl, the
renderings, the node or the app files has changed.

## Phase 2 — The contract pinned (build entry step 1)

Milestone: the amendment landed across the contract repositories, the node and
the app files; every gate row of the document's §4.7 green on a settled tree —
the (a) rows and the nine (b) rows the recorded answers make gates (its §4.0) —
with §4.7(c) recorded; the acceptance review passed; the publish recorded; and
the discovery-side slice profile pinned with executable consumer tests.

*(Refreshed 2026-09-23 against the accepted revision 4 (`3b60234`) and the
rulings recorded at `a0e6ce4` and `972d4d0`. The version of 2026-09-22 was
written against revision 2: it predated the header-order correction (SAF-P2-9),
the declared deletion list (SAF-P2-10) and the rulings, so its Step 2.1 ran the
compatibility gate with no deletion list, its 2.3 moved the headers against a
node that refuses them, and its 2.4 asked for a diagnostic on `from-cursor`,
which R9(b2) keeps. Its 2.1–2.4 are replaced by 2.1–2.7 below; its 2.5–2.7 are
now 2.8–2.10.)*

**Two lanes.** The contract (2.1, 2.2, then its consumers in 2.8) and the node
with its app files (2.3–2.7) share no build dependency — among the Rust crates
of `glade`, `grazel`, `glade-gyld` and `glade-gwz`, only
`glade/contracts/binding-api` depends on a `glade-decl` rendering — so the two
lanes run in parallel after Phase 1 and meet at the acceptance
review (2.9). Each step commits its members through gwz, then the root lock;
the lock after 2.1 has §4.7 row 1 red by construction (the renderings are
regenerated in 2.2), and no review or publish reads an intermediate lock.

**The node lane's order is the document's §4.4 landing order** (SAF-P2-9),
because a token and a header have opposite safe orders: an old node stores a
new token raw, and refuses a new header before it reads a token
(`appdecl.rs:89-95`). So: a node that accepts both headers (2.3); the app
files' tokens (2.4); the node's vocabulary and the binding fold (2.5);
validation (2.6); the headers last (2.7). Each is its own commit, and after
every one of them `cargo test -p glade-node` and `cargo test -p grazel` (§4.7
rows 6 and 8) are green, as are the integration tests of `glade-gyld` and
`glade-gwz`, which spawn the node binary on their own fixtures
(`glade-gyld/Cargo.toml:29`, `glade-gwz/Cargo.toml:22`). That last gate is this
plan's addition: §4.7 names only the first two, and the fixture headers move in
2.7 like the rest.

**Two constraints the recorded answers imply**, stated once here:

- *No new field on `sysdata.BindingDecl` in this amendment.* taut's optional
  fields are always emitted, so a new field changes the bytes of every stored
  binding record (`sysdata.rs:128-137`; the document's R4(a) node cell), and the
  first boot would append all 28, not the 15 that §4.7 row 9 asserts. A tail key
  that must reach a consumer takes a record kind of its own, keyed by glade id —
  R4(b)'s shape — which moves no stored byte and can be added later. None needs
  to in this phase: no app file declares a `crdt` binding, and nothing enforces
  retention (GC-4).
- *The file's spelling is the hyphen* (the document's §3, 2026-09-23). Pages and
  in-file comments gloss `from-cursor`; `from_cursor` is the stored and published
  spelling.

### Step 2.1 — Schema, corpus and the contract's gates

Goal: `glade-decl` carries the contract half of the amendment in one commit,
which is the amendment commit the renderings pin.

- §4.1 under the recorded answers: `atom=8`, with `message` and `window` kept
  with their numbers and the enum comment rewritten (item 1);
  `ShapeProfileDecl{glade_id, profile}` added and `BindingDecl` untouched
  (item 2); the `RetentionPolicy` and `ChangeEvent.base_seq` comments (items 3,
  4); `AdvertisementRecord` removed (item 5), its retired name recorded in the
  module docstring and `dev-docs/OpenNotes.md`, because taut has no
  message-name reservation (item 8); the deferrals in the docstring and OpenNotes
  N3/N4, naming `canonical_key`'s owner (Gianni) and the version both functions
  are deferred past (item 6, R6(a)); N7, N8 and N9, with `private` scoped in
  `ZoneKind`'s comment and `README.md:39-41` (items 6, 7).
- `corpus/build.py`: the curated `atom` vector, and an assertion that every
  `Shape` member has one (SAF-P3-8); `--compat` over `v0 ∩ v1` with a declared
  deletion list, v0 read from `git show bbce73d:corpus/decl.v0.json`, red on an
  undeclared absence and on a declared name still present (§4.2); the `--check`
  artefact list extended to the four rendering copies (SAF-P2-5);
  `CONTRACT_VERSION` made checkable (A1, §4.3). The two recognition-case vectors
  keep their keys (§4.1 item 1), so the deletion list is exactly
  `AdvertisementRecord,edge/advert`.
- `corpus/decl.v1.json` replaces `decl.v0.json` in the same commit, and
  glade-decl's own `decl.v0` references move (§4.2 items 2 and 5). No
  "superset" sentence anywhere unless it names the subset and the gate (§4.2).
- The front page gains a members table per enum with a gloss each, the
  `Retention` members included, and says whether an app-file glade id is
  authored or derived (§4.2 step 6). `dev-docs/DeclSurface.md` becomes a
  banner-marked mirror of the root `dev-docs/glade/GladeDeclSurface.md`, with a
  drift check in `--check` (§4.2 step 5, A12).
- The root page, in this step's root commit, because §4.4 bullet 15 makes these
  one edit and the mirror must match it: `source` in `BindingDecl`'s field list
  (R5); R8's record-kinds sentence; the `Retention` row (bullet 15); the zone
  sentence at `:29-30` (SUR-P2-1); the corpus name; and the rows the rulings
  make false (`AdvertisementRecord` held out, a `ShapeProfileDecl` row, the
  canonical-key and derivation deferrals). The mirror then carries A12's repair
  of the repository copy. The root `decl.v0` references move with it (§4.2
  item 8).
- Done when: `python3 corpus/build.py --compat --deleted AdvertisementRecord,edge/advert`
  is green, and red without the declaration (§4.7 row 2); `git -C glade-decl
  grep 'decl\.v0'` returns only `--compat`'s source (`V0_PATH`, read at
  bbce73d). Row 1 is not a gate of this step: it compares the
  rendering copies and `vectors.rs`, which 2.2 regenerates. `build.py` rewrites
  `glade-decl-rs/src/vectors.rs` when it runs; 2.2 commits that file.
- Depends on 1.4.

- **Done, 2026-09-23.** glade-decl `7d18cd3` (the amendment commit), `784840a` (writes replace files: pnpm hard-links `glade-decl-ts` into consumers, and 2.1's write had changed seven installs, since restored); root `112d30a`, `b75a953`. Row 2 green; row 1 green but for the three pins, which 2.2 set; 23 unit tests.

### Step 2.2 — The three renderings

Goal: Rust, TypeScript and Python regenerated into their `src/` from the 2.1
commit, in lockstep with it.

- §4.3's procedure (the rendering READMEs' form, CON-P2-2): generate to scratch,
  copy into each `src/`, `cargo fmt` in glade-decl-rs, then the IR and
  `decl.v1.json` copies for ts and py with their `decl.v0.json` copies removed
  (§4.2 item 4), then `build.py`. `CONTRACT_VERSION` in all three pins the 2.1
  commit (A1). The remaining `decl.v0` references move: the two gate reads
  (§4.2 item 3), `glade-decl-py/pyproject.toml:20` (item 1), and the text in the
  three repositories, the READMEs' copy commands included (items 6, 7).
  `glade-decl-ts/README.md:21-22`'s `npm` commands become `pnpm` ones — a command
  change, not a lockfile migration (§4.8).
- Done when: §4.7 rows 1, 3, 4, 5 and 11 are green, and §4.2's enumerating
  command, with this plan also excluded (`':!dev-docs/GladeFirstSlicePlan.md'`),
  returns only `--compat`'s source in `corpus/build.py` and the historical note
  in `GladeHandoff-260710.md:163`.
- Depends on 2.1.

- **Done, 2026-09-23.** Renderings `9507c12` (rs), `37d9b7d` (ts), `433215a` (py), pinned to `7d18cd3`; root `8f161dc`. Rows 1, 3, 4, 5, 11 green; Python row 5 from the built wheel.

### Step 2.3 — A node that accepts both headers (landing step 0)

Goal: the one node change that must come before any header moves, and the
warning channel that validation will use.

- §4.4 step 0 and bullet 3 under R10(a): `appdecl.rs:89-95` accepts
  `glade-app v0` and `glade-app v1`, and any other header is still refused with
  a line number; `parse()` gains a non-fatal channel — a `Vec<String>` of
  warnings on `AppDecl`, or an `eprintln!` in `load` — which touches
  `glade-node.rs:88` and grazel's integration path. No token validation, and no
  warning on `v0` yet: `v0` is warned once `v1` is the validated grammar (2.6).
- §4.7 row 17's evidence is taken first, against today's parser: a `v1` header
  refused with `:91`'s diagnostic. The step's tests then assert that both
  headers load and any other is refused.
- Done when: rows 6 and 8 are green with the fixture repositories' tests, the
  both-headers tests pass, and row 17's evidence is recorded beside them as the
  regression that pins this step ahead of 2.7.
- Depends on 1.4 only.

- **Done, 2026-09-23.** glade `4258c62`, root `b55914c`. Row 17 evidence at 559cb2c: "line 16: expected `glade-app v0` header, got `glade-app v1`". Rows 6, 8 and the fixture repos green.

### Step 2.4 — The format page and the app-file tokens (landing step 1)

Goal: the page an author reads exists before anything it describes is enforced,
and the app files carry the migrated token.

- The format page, `glade/docs/AppFileFormat.md`: new, and the user-facing page
  the Surface axis has asked for in every round (the standing residual);
  `glade/docs/` is the repository's home for "Public support contracts and
  user-facing documentation" (`glade/README.md:23`). It carries the grammar as
  the parser accepts it now, `workspace <share> <name>` included with the clause
  that it makes a declared surface routable (bullet 12, SUR-P3-10); the hyphen
  convention (bullet 13); which `.glade` dialect this is (bullet 14); token 4 —
  `commons`, `private` with row 16's caveat, the rule for choosing, absence as an
  arity refusal, a mount not overriding an authored zone; token 5 — `latest`,
  `from-cursor` and `ttl`, which to write per `BINDING_SHAPES` member, `crdt`'s
  answer and the warning against `latest` for it; and what a changed or deleted
  line does under R9 — the fold, the scoped retraction, a file not loaded
  retracting nothing, a deleted `service` or `workspace` line retracting nothing
  (bullet 5, which must be written before any file is migrated).
- `GladeGrazelAttachNotes.md`: its grammar block (`:29-36`) points to the page;
  `:49-51` (the zone sentence, SUR-P2-1), `:56-61` (R9's rule) and `:98`
  (bullet 8) are corrected. The eleven `dev-docs/examples/*.glade` files get a
  banner naming their language (bullet 14).
- The files: `term.log`'s `windowed` becomes `from-cursor` in both homes of
  `grazel-app.glade`, with their `:21` zone sentence rewritten (SUR-P2-1). The 13
  `from-cursor` lines stay (R9(b2)), and every header stays `v0`. The two
  fixtures gain the grammar comment, or the page says the in-file grammar is a
  3-of-5 property (bullet 12).
- Not yet: the tail, and `v1` in any grammar. Each would describe a line today's
  node refuses; they land with the parser in 2.5 and with the headers in 2.7
  (§4.7 row 19). Until 2.5 the page says that `ttl`'s duration cannot yet be
  written.
- Done when: rows 6 and 8 are green with the fixture repositories' tests (an old
  node stores the new token raw), row 18 is green in the file's spelling, and
  row 20 is green.
- Depends on 2.3 by the landing order; the page can be drafted from 1.4.

- **Done, 2026-09-23.** glade `7bd5f9d`, grazel `05553b4`, root `fe48ca3`. Census 15 `from-cursor`, 13 `latest`, 0 `windowed`; rows 6, 8, 18, 20 and the fixture repos green.

### Step 2.5 — The node's vocabulary and the binding fold (landing step 2)

Goal: `appdecl.rs` knows the v1 vocabulary and the tail, and `dir.bindings`
gets R9's rule, with nothing validated yet.

- §4.4 bullets 1, 4, 7, 9, 10 and 11: `crdt` and `atom` in `KNOWN_SHAPES`,
  `crdt` in `BINDING_SHAPES`, and a reserved note for `atom`, `message` and
  `window` (bullet 1); the keyword tail under R11(a) — `ttl=<duration>` and
  `shape-profile=<profile>`, an unknown key refused by name with a line number,
  the arity check a minimum and `:113`'s template showing the tail, the
  profile's legal values and per-shape omission rule enforced at parse
  (bullet 4), with no new `sysdata.BindingDecl` field; the recognised-but-refused
  table and its messages — `windowed` and a file's `from_cursor`, both naming
  `from-cursor` — which 2.6 switches on (bullet 7); `sysdata.taut.py`'s comment
  on `app ≡ package` (bullet 9); `glade/decl/*` bannered (bullet 10); and
  `session.rs:27`'s error text (bullet 11).
- R9 under the recorded answers: `parse()` normalises `from-cursor` to
  `from_cursor` on the way into `sysdata::BindingDecl` (b2); a `bindings_of()`
  fold by `glade_id`, newest wins, beside `grants_for`; `exchange.rs:62-78` folds
  instead of calling `any()`; a `BindingRetraction` record kind with its
  `Record::Retract` arm; `register` diffs the parsed file against the fold per
  `(app, glade_id)`, the scope R9(a) states; `sysdata.rs` regenerated with
  `--legacy-codec`. The normalisation and the fold land in one commit, because
  b2's appends are adjudicated by nothing else.
- The published grammar and the three in-file comments gain the tail, with the
  sentence on how an author learns it exists (bullet 12; R11(a)'s Docs cell),
  and the format page gains `ttl`'s duration.
- Done when: rows 6 and 8 are green with the fixture repositories' tests; row 9
  asserts `appended` = 15 over the census, its tree-wide units named in the test
  (SAF-P3-13); row 10's three tests pass; and row 19's tail half is green.
- Depends on 2.4.

- **Done, 2026-09-23.** glade `967fcdf`, grazel `bf86347`, root `d2d7b59`. `BindingRetraction` on `dir.binding-retractions`; one lamport clock across the two binding streams; the tail validated, not stored. Census 15 (5 + 5 + 2 + 2 + 1), 7 for a grazel + gyld store; rows 6, 8, 9, 10, 19's tail half and the fixture repos green.

### Step 2.6 — Validation (landing step 3)

Goal: the shrinkage switched on, as warnings for one release.

- §4.4 bullet 2, branch (ii) for both tokens, in the binding arm that R10(a)
  branches by header. In a `v1` file the zone is checked against
  `{commons, private}` and the retention against `{latest, from-cursor, ttl}`, in
  the file's spelling, each a line-numbered warning through 2.3's channel for one
  release and a hard error at the next, with `windowed` and a file's
  `from_cursor` warned by 2.5's messages. A `v0` file loads as it does today,
  warned that its header names the old language and, for each token v1
  changes, given the replacement and the version it changed in (R10(a)'s
  third-party cell).
- Done when: rows 6, 7 and 8 are green with the fixture repositories' tests,
  and every shipped file loads with no warning except the `v0` header's.
- Depends on 2.5.

- **Done, 2026-09-23.** glade `d4db2fc`, root `bc73bbd`. A `v1` file's bad token is a line-numbered warning; the warnings ship in the first node release (the first version above `0.0.0`) and the release after it sets `V1_TOKEN_CHECKS_REFUSE` (corrected per COD-P3-2: this line first put the flip one release early); `v0` files are warned, never refused. Rows 6, 7, 8 and the fixture repos green.

### Step 2.7 — The headers (landing step 4)

Goal: each file declares the validated language.

- All five files to `glade-app v1`, the two `grazel-app.glade` twins in one
  commit (§4.4 step 4); `GladeGrazelAttachNotes.md:30` and the format page's
  grammar to `v1` (row 19's header half).
- Done when: rows 6, 8 and 19 are green with the fixture repositories' tests,
  and every shipped file loads with no warning.
- Depends on 2.6.

- **Done, 2026-09-23.** glade `dddf8b8`, grazel `b604a06`, glade-gyld `c9ef7a6`, glade-gwz `fc0bb99`, root `332855a`. All five files `glade-app v1`, twins identical; every shipped file loads with no warning; census unchanged. Rows 6, 8, 19 and the fixture repos green.

### Step 2.8 — Consumers

Goal: glial, grip-core, grip-share with the demo, and gryth-ui unchanged in
behaviour under the new types.

- The document's §4.5 order. glial reads the profile by lookup under R4(b),
  ahead of `config.crdtProfile`, which stays for back-compat; `Surface extends
  BindingDecl` still compiles in glial and the compile wall still errors on an
  undefined key (row 12); `manifestScope` still yields `share="account:<user>"`
  and `key=utf8("self:<user>")` with no edit (row 13, R1(a)); 61 vitest suites
  and `pnpm build:gyld` in gryth-ui (row 15); `glade/contracts/binding-api`, the
  one crate in `glade` that depends on the rendering (its `Cargo.toml` path
  dependency on `glade-decl-rs`), runs its `public_contract` tests
  (BI-001..003). grip-core's `0.3.0` republish waits for 2.9.
- Recorded as evidence, not gates (§4.7(c)): the glial `zone: "private"`
  observation (row 14) and the running demo reaching its published-builds line
  (row 16).
- Done when: rows 12, 13 and 15 are green and rows 14 and 16 are recorded.
- Depends on 2.2.

- **Done, 2026-09-23.** glial `4c6e856`, glade `3a17b8e` (row 13's test), root `d35d62e`; glial and grip-core reinstalled with frozen lockfiles (no lockfile change), the demo reinstalled likewise to clear a stale v0 copy. Rows 12, 13 and binding-api green; gryth-ui (row 15) green after gryth-wz's `glade-decl-ts` and `glial` were fast-forwarded to `37d9b7d` and `4c6e856` at the owner's word: 61 files, 938 tests, `pnpm build:gyld` built. Row 14 recorded: a glial mount of a `private` surface neither produces a `self:` key nor throws (the mount never reads `decl.zone`; `instance.ts:40-42`, `session.ts:53-58`). Row 16 is the owner's manual check.

### Step 2.9 — Acceptance review and publish

Goal: the freeze.

- Acceptance review on the settled tuple, the lock after 2.7 and 2.8: the dual
  axes (Code, State) that an interface freeze makes mandatory, plus Surface,
  because `<app>.glade` is a file format people edit (the document's §5;
  `AgentProcessRules.md` L1-18 and its D7 amendment). Tier per §4 item 1.
  Reports filed as `GladeDeclAmendment-Review<Axis>.md`. The object includes the
  three riders the reconciliation's acceptance sent to this commit: SAF-P3-13
  (row 9's units, answered in 2.5's test), CON-P3-11 (R9's option (s), declined,
  has no row; its closure is the Consistency reviewer's) and the Surface
  residual, which 2.4's page answers.
- Then [PackageExtractionPlan.md](PackageExtractionPlan.md) step 1.1 — "Decide
  and record the GDL-041 catalogue question" (`:155`) — is closed by the
  amendment itself, and step 1.2 — "Make it a real package" (`:156`) — is the
  publish: `@owebeeone/glade-decl`, and `@owebeeone/grip-core` at `0.3.0`
  (`:157`).
- Nothing is published until every row of §4.7(a) and the nine §4.7(b) rows the
  recorded answers make gates are green on the settled tree (the document's
  §4.0: its publish ordering and its resolution table); §4.7(c) is evidence.
- Done when: the publish is recorded with the tuple and the three GO files.
- Depends on 2.1–2.8.

- **Accepted, 2026-09-24** at root `73c2bf7`, glade `1b9ac3f`, glade-decl `3d10917`, glade-decl-rs `d3be799`, glade-decl-ts `85ec18d`, glade-decl-py `ee2f960`, grazel `e1a4078`, glade-gyld `c9ef7a6`, glade-gwz `fc0bb99`, glial `4c6e856`, grip-core `97ff6c2`, after `glade/GladeDeclAmendment-Review{Code,State,Surface}-2.md` reported GO (round 1: Code and State NO-GO, one remediation, `glade/GladeDeclAmendment-RemPlan.md`). This accepts Steps 2.1–2.8 only. Open P3s, none blocking: SUR-P3-9 = STA-P3-4 (the page's retirement text overclaims), SUR-P3-10 (a missing `--app` file is reported without its path), SUR-P3-11 (refusals print as a Rust Debug dump), COD-P3-7 (the flip check misses an early flip). The publish waits on §4.7's gates and the owner.
- **§4.7 gates, 2026-09-24: all 19 green** on the accepted tree. (a) 1 `--check`; 3 ts 27; 4 rs; 5 py 27 from the built wheel (imported from site-packages, `src/` off the path); 6 node 121; 8 grazel 26+3; 11 no stray dirs, the new member in all three APIs in each language's spelling (`"atom"` ts, `Atom => 8` rs, `atom = 8` py; the row's literal grep fits ts only); 12 glial 99 + `tsc`; 15 gryth-ui 61 files / 938 tests + `build:gyld`, tree clean; 20 only corrected sentences remain. (b) 2 `--compat --deleted AdvertisementRecord,edge/advert` green, red without it; 7 the four `row7_*` tests; 9 `row9_*` 15 and 7; 10 the three `row10_*` tests; 13 `demo_scope.test.ts`; 17 `a_v1_header_loads_as_its_v0_twin` with 2.3's recorded evidence; 18 the retention table glosses `latest`, `from-cursor`, `ttl`; 19 the grammar and the three app files show `glade-app v1` and the tail; 21 the R8(b) sentence and the mirror banner. What remains of the publish is PackageExtractionPlan 1.2 (glade-decl-ts is not yet a buildable package: `0.0.0`, source entry points, no `dist`, LICENSE or publish workflow) and 1.3 (grip-core `0.3.0` on a caret range).
- **Owner, 2026-09-24 ("all recommended"):** package now, as local commits (PackageExtractionPlan 1.2 and 1.3); `@owebeeone/glade-decl` publishes first as `0.1.0`; glade-decl-ts moves from `package-lock.json` to `pnpm-lock.yaml` in 1.2 (grip-core's two lockfiles are left as they are); the owner publishes by pushing and cutting GitHub releases, and the publish workflows copied from `taut-shape-ts` publish with the repo-secret token. No agent pushes or publishes. The round-2 P3s are fixed now, in a small glade follow-up.
- **Round-2 P3s fixed, 2026-09-24,** glade `d922f8a`, grazel `e343b06`. SUR-P3-9 = STA-P3-4: the page says retiring an app withdraws its `binding` declarations only, and the retirement test now holds that a `service` record stays. SUR-P3-10: a `--app` path the node cannot read is refused naming the path. SUR-P3-11: a refused start prints its message, `<file>: line N: …`, not a Debug dump, and exits 1 (`node/tests/start_refusals.rs`, both tests red on the old code). COD-P3-7: `flip_decided` fails a flip made at or before the warnings' release. The page also states the seed ruling (a seed names the workspace share), and both `grazel-app.glade` copies carry a comment that their `seed owner grazel` lines are corrected at Step 4.3.
- **Packaged, 2026-09-24, local and unpublished:** glade-decl-ts `a393ecb` (`@owebeeone/glade-decl` 0.1.0) and grip-core `9889655` (0.3.0), PackageExtractionPlan 1.2 and 1.3; their notes are under that plan's Phase 1 table. What remains is the owner's: add an `NPM_TOKEN` repository secret to both repos (a new package cannot publish first through npm trusted publishing), push both, cut the glade-decl-ts release `v0.1.0` and let its workflow finish, then cut the grip-core release `v0.3.0` (its workflow installs glade-decl `^0.1.0` from npm).
- **Owner, 2026-09-24: "no publish until we have built a full end-to-end app using grip/glial/glade."** The two packages stay local commits, and the secrets, pushes and releases above wait for that app, so this step stays open. Nothing in Phases 3–5 waits on the publish. Until it happens, a fresh (non-frozen) install in grip-react, glial or ggg-viz cannot resolve `@owebeeone/glade-decl` `^0.1.0`, which only grip-core's own `pnpm.overrides` maps to the checkout; frozen installs are unaffected.

### Step 2.10 — The discovery-side slice profile and its consumer tests

Goal: the rest of build entry step 1 (`:47-51`) — the canonical records, the
trust and namespace proof profile, the clock inputs and the local-acceptance
guarantee the slice relies on, each recorded as a bounded profile with an
executable test.

- Reconcile `glade-discover-core` and `glade-discover-node-adapter` with the
  seven draft contracts landed at glade-discover `fd94a1f`: `Transport::send`
  (`glade-discover-transport-api`), `Signer`/`Verifier` (`-signature-api`),
  `DurableOperationStore` (`-operation-store-api`), `AcceptanceJournal`
  (`-acceptance-api`), `RegistryWriter`/`RegistryReader` (`-registry-api`),
  `TrustPolicy` (`-trust-api`), `ShardLocator` (`-placement-api`); each carries
  `src/conformance.rs` behind a `conformance` feature and `tests/public_contract.rs`
  (`glade-discover/dev-docs/DraftHostContracts.md:39-41`), with TR-001..003 as
  the transport's requirement IDs (`:43-45`), and `scripts/check-contracts.sh`
  as the gate. That commit says "no production cutover is authorised by these
  tranches": this step pins, it does not cut over.
- One document, `dev-docs/glade/GladeFirstSliceProfile.md`: the canonical records
  (the home-share kinds of `glade/node/src/sysdata.rs` plus `NodeTransportBinding`
  from 4.2; the registry records the discovery core writes), the proof profile
  (taut grants; ed25519 under the node chain; the encoding named exactly, so an
  incompatibility is an explicit blocker to the adapter, not a fixture), the
  clock inputs (one `ClockPort`, substituted once for every consumer in a test
  composition — `InjectionGraphRefinement.md:22-25`), and the local-acceptance
  guarantee (durable-local before any sync effect). Each choice cites the ruling
  or ratified entry that decides it; where none does, it says so and stops.
- Consumer tests: the profile's statements as tests against deterministic
  providers in the discovery workspace's pure loop (LBT-008), fixtures
  contract-faithful (LBT-009). No new wire grant; the frozen discovery contract
  is not weakened (`GladeBuildEntry.md:50-51`).
- Done when: the profile document exists, its tests are green in the pure loop,
  and `check-contracts.sh` passes.
- Depends on nothing in 2.1–2.9; runs beside them from the start of the phase.

- **Done, 2026-09-23.** glade-discover `48bc104`. `dev-docs/glade/GladeFirstSliceProfile.md`: records SP-R1..R4, proof SP-P1..P5, clock SP-C1..C3, local acceptance SP-L1, trust and namespace SP-T1..T3 and SP-N1; the seven drafts' divergences recorded, not refactored. `crates/glade-discover-node-adapter/tests/slice_profile.rs`: 14 tests (6 pins, 8 wrong fixtures), 0.03 s warm; `check-contracts.sh` and `check-architecture.sh` green. Twelve items the profile marks not decided (§8), among them the signature algorithm and the monotonic instant that 3.1's `ClockPort` does not supply.

Phase 2 exit: a published contract at a recorded tuple; a slice profile with
green consumer tests.

## Phase 3 — The smallest injectable assembly (build entry step 2)

Milestone: the node's composition is a Shaku assembly over its own contracts,
lifecycle owned by an sdax plan, every boundary a deterministic provider, and the
eight journeys the build entry names (`:52-56`) pass as tests. One explicit
dependency scope reaches every cooperating consumer (AR-03,
`RuntimeAndAssurance.md:121`). Nothing real stands behind any port yet.

Two things already exist and are reused, not rebuilt. First, `glade/contracts/`
is an independent workspace of six draft application contracts with conformance
suites, its own `check.sh` and `architecture-policy.json`: `BindingResolver`,
`Invoker`, `Subscriber`, `ReplicaSync`, `ManagedResource`, `SnapshotStore`
(`glade/contracts/README.md`). Second, the witness workspace
(`glade/dev-docs/async-witness`) is the template: a framework-free `ports` crate
gated by ARCH-002, `real/src/shaku_bridge.rs` for the facade form,
`real/src/peer_plan.rs` for the plan shape, `check.sh` and `arch002-fixture.sh`
for the gate. Carried forward from the witness ruling: every real provider in a
test composition is overridden or lazy; a provider whose close consumes its
handle gives it up by value; a leaked handle is invisible to the sdax report.

The composition root starts alongside the hand-written one (`GladeBuildEntry.md:80-81`):
a second binary or an explicit path in `glade-node`, and the demo keeps the
old path until the slice passes. Retiring the old path is not in this plan.

### Step 3.1 — The missing ports

Goal: the contracts the assembly needs and does not have, in the existing
contracts workspace, framework-free.

- Components.md names them as N (needed): the duplex/session carrier port with
  size, cancellation and ownership semantics (`Components.md:72`, `:32`), the
  clock and budget ports (`:31`), and the typed admission facade (`:25`). For the
  slice: `CarrierPort` (bind, dial, accept, close-by-value — the witness's
  contract, `async-witness/ports/src/lib.rs`, with no iroh type on the boundary,
  LBT-004), `ClockPort` (the witness's), `GrantPort` (the admission facade at its
  smallest: does this node or principal hold this verb on this share, folded with
  revocation-wins — the query `grants_for` already answers), and `SignerPort`
  (sign and verify under the node key — the three stub seams' shape; the
  discovery `Signer`/`Verifier` is reused where the types allow rather than
  duplicated).
- Bridging onto Shaku uses the form the lane owner measured
  (`AsyncWitnessResult.md` §5, caveat 4): a `Send + Sync` contract that does not
  name `Any` bridges with `impl<T: Port + 'static> Facade for T {}`; the
  witness's `impl<T: Port> Facade for T {}` fails with E0310 on such a contract.
- Done when: the crates compile in the contracts workspace; its `check.sh`
  passes; an ARCH-002 fixture on the witness's pattern refuses a framework
  dependency injected into any contract crate.
- Depends on nothing; can start today, beside Phase 1.

- **Done, 2026-09-23.** glade `831eded`, root `05b2025`. Four framework-free contract crates (`glade-carrier-api`, `-clock-api`, `-grant-api`, `-signer-api`), no dependencies, each with a conformance suite and wrong-fixture tests; `check.sh` 86 tests green; the Shaku bridge form proven by doctests, the E0310 form `compile_fail`; `arch002-fixture.sh` refuses an injected `shaku` in all 10 contract crates. `SignerPort` is local: discovery's `Signer`/`Verifier` take a `SignedOp` and are not dyn-safe. About 1,590 lines against the ~500 budget. Owner, 2026-09-23 ("all recommended"): carrier frames stay opaque bytes (the session owns the codec; no glade-wire in the crate); no budget port for the slice. The node's own grants: ruled at 4.3.

### Step 3.2 — The assembly

Goal: `bin/glade-node.rs:48-142`'s straight line becomes a composition root that
resolves from a module.

- `glade/node/src/assembly.rs`: a Shaku `module!` with the six binding recipes of
  `InjectionGraphRefinement.md:12-19` — `clock_binding`, `peer_carrier_binding`,
  `client_carrier_binding`, `record_transport_binding` (the same iroh occurrence
  as the peer carrier, `:22-25`), `directory_host_binding`,
  `directory_profile_binding` — all `AssembledBy[NodeAssembly]` and
  `SharedWithin[NodeScope]` (`:21`), plus the grant and signer bindings from
  3.1. Deterministic providers as components: fake clock, in-memory store and
  registry, fake carrier, a keyed test signer (an accept-anything verifier fails `SignerPort`'s SI-002 by design; 3.1's `signer-api` tests hold one), fixed configuration; each fake runs its contract's suite through the `conformance` feature. Eager construction
  unless overridden or `#[lazy]`; every real provider overridden in a test
  composition (the witness's DI-E01 and DI-E02 lessons). `DirectoryRules` breaks
  the Directory↔Records constructor cycle as `:35-40` describes; it need not be a
  crate.
- Done when: the existing node test suite passes unchanged through the
  assembled path; a missing binding, a construction cycle and an ambiguous role
  are refused as in the witness's three negative examples; no global
  `get_service<T>()`, string bag or constructor fallback to real I/O exists
  (`RuntimeAndAssurance.md:73-76`).
- Depends on 3.1.
- **Owner, 2026-09-24 (§4 item 5):** the assembled composition root is a path in `glade-node`, switched by the environment variable `GLADE_NODE_ASSEMBLED=1` and off by default, so the demo and grazel keep the hand-written path; the existing node tests run once each way, unchanged.
- **Done, 2026-09-24.** glade `b135e20`. `glade/node/src/assembly.rs`: the Shaku module `NodeAssembly` (shaku `=0.6.3`) with the six recipes plus the grant, signer and configuration bindings; the design, with each binding's providers and consumers, is `glade/dev-docs/GladeNodeAssembly.md`. The node suite, 157 tests, passes both ways with its test files unchanged, and the node gate now runs it both ways. On the assembled path the clock, the record host over the booted instance, the directory rules and the configuration are real; both carriers, the grant fold and the signer are fail-closed stand-ins until Phase 4, and the root still binds the iroh endpoint and the TCP listener for the `Server` to run. The test composition (fake clock, fake-network carriers, the node's `Registry` over `MemStore`, an in-memory grant fold, a keyed test signer) passes each port's conformance suite and builds no real provider (DI-E01); one scope, one occurrence, and sibling scopes isolated under concurrent first access (DI-E02). DI-E03 as four `compile_fail` examples: a missing binding (E0277), a cycle (E0275), an ambiguous role (E0119: roles are one interface each, since Shaku 0.6.3 cannot override keyed components) and a carrier asked for by port type (E0277); stable rustdoc checks the failure, not the code. Named gaps: AR-03 holds for the module's consumers only (the `Server` still reads `sysdir::now_ms()` and owns its transports); no real adapter implements a port yet. 619 lines of non-test code against the ~500 budget. The node policy change (shaku allowed in glade-node, the four port crates as dependencies) awaits the owner's re-review.
- **Owner, 2026-09-24 ("Accept"):** the node policy with shaku is accepted; its reason says so from glade `3f97a3d`. Step 3.3 started the same day.

### Step 3.3 — Lifecycle

Goal: the nine bare spawn sites in `mesh.rs` and the unbounded loop in
`claims.rs:115` become owned tasks in an sdax plan with a release order.

- `sdax`, `sdax-tokio` and `sdax-testkit` pinned at the witness's revision
  `ccf06e76a90e22a454471a71f0cf6f5cb878baac` until crates.io (version_pin: every
  0.x pinned exactly). A plan with acquire and release for endpoint, listener,
  accept loop, links, subscriptions and the renewal loop; the partial order of
  `InjectionGraphRefinement.md:42-45` — Records drains before storage and the
  peer carrier release, Sessions before the peer and client carriers — as the
  release graph; `PeerEndpoint::close(self)` on release; `server.run` awaits the
  plan; a stop signal drives cleanup, which today does not exist. Retry loops
  live above sdax (the release graph runs once). `report.incomplete` is the
  signal; `tracked()==0` is not. Shutdown stops admission, drains or
  cooperatively cancels children, then releases (`RuntimeAndAssurance.md:91-93`).
- Done when: a node starts, links to a peer, and stops with `report.incomplete`
  empty and the port re-bindable within the witness's 2 s bound; the witness's
  partial-order release test is reproduced on the node's plan.
- Depends on 3.2.
- **Done, 2026-09-24.** glade `12da2ac`. `glade/node/src/lifecycle.rs`: the assembled path as one sdax plan (sdax, sdax-tokio and, for tests, sdax-testkit, all at `ccf06e76…`), Resident, fail-fast, a 30 s shutdown budget. Resources `Instance`, `Storage`, `PeerCarrier` (closed with `PeerEndpoint::close(self)`) and `ClientCarrier`; services `Records` and `Sessions`; steps `Assembly`, `Peers`, `Workspaces`, `Listening`. The four partial-order edges hold, statically and in a simulated run (`tests/release_order.rs`). `src/tasks.rs`: every production spawn goes through one seam, the ten the plan names plus exchange.rs's two forwards and server.rs's client session and writer. It is detached on the hand-written path, as before, and on the assembled one owned by `Records` or `Sessions`, which close admission, cancel and join at their stop. The owners are the two services, not one sdax instance per task, since a run keeps history for every instance it spawns. On the assembled path SIGTERM or SIGINT stops the plan, exit 0 when `report.is_clean()` and 1 otherwise; the hand-written root still dies by the signal. A node links to a peer and stops with `report.incomplete` empty, its ports re-bindable within microseconds of the report, against the 2 s bound. 171 node tests pass both ways, and the grazel, glade-gwz and glade-gyld suites pass against the rebuilt default binary. tokio moves 1.52.3 → 1.53.1 on both paths, sdax-tokio's pin; the fmt baseline drops to 338. Named gaps (`glade/dev-docs/GladeNodeAssembly.md`): client sessions are cancelled, not drained; on the assembled path a taken port now fails before `peer` prints; the async witness's tracked lockfile has been stale since 3.2, so its `--locked` commands no longer run against today's node. 773 lines of non-test code against the ~500 budget. The node policy change (the three sdax crates in glade-node) awaits the owner's re-review.
- **Owner, 2026-09-24 ("Accept"):** the node policy with the sdax crates, and the tokio move they bring, is accepted; its reason says so from glade `0a8733d`. Step 3.4 started the same day.

### Step 3.4 — The eight journeys as consumer tests

Goal: publish, exact retry, renewal, expiry, wrong scope, unknown or denied
authority, partial lookup, lost acknowledgement — each a test against
deterministic providers, contract-faithful.

- Two commits if the budget needs it: (i) publish, exact retry, lost
  acknowledgement; (ii) renewal, expiry, wrong scope, unknown or denied
  authority, partial lookup. The fake clock drives renewal and expiry
  (`claims.rs` leases); the fake carrier injects loss and duplicate delivery; the
  denied-authority journey is the first consumer of `GrantPort` and, once 2.5
  lands, of the R9 fold. AR-05 is the criterion: publish, renew and expiry with a
  fixed authorized locator, a foreign namespace or referral rejected
  (`RuntimeAndAssurance.md:123`).
- A fixture must not pretend to be physical durability or cryptography
  (`GladeBuildEntry.md:55-56`): each fake states what it does not prove, and
  each journey names the real provider that Phase 4 substitutes.
- Done when: eight named tests are green in the fast loop and the loop's
  measured budget is recorded (LBT-010).
- Depends on 3.2; on 3.3 for the retry and lost-acknowledgement timing.

- **Done, 2026-09-24.** glade `5cc7438` (part (i)), `f39daa9` (part (ii)). `glade/node/tests/journeys/`: the eight journeys as named tests, `delivery::{publish, exact_retry, lost_acknowledgement}`, `leases::{renewal, expiry, partial_lookup}` and `admission::{wrong_scope, unknown_or_denied_authority}`, over two test compositions of `NodeAssembly` on one fake network, A's configuration naming B as its one fixed peer. Three test-only providers, each saying what it does not prove: a carrier whose dialed links can suffer one transport failure (it runs CA-001..004), an engine the test reads back, and a grant fold a journey changes between decisions (GR-001..003). One additive production seam, `Records::in_memory_over`. Each test names the Phase 4 provider that replaces its fakes; the design is `glade/dev-docs/GladeNodeAssembly.md`, "Journeys (plan Step 3.4)". Fast loop (LBT-010): `cargo test --offline --locked --manifest-path glade/node/Cargo.toml --test journeys --test assembly`, 35 tests, 0.14-0.18 s wall warm and 0.93-1.18 s after touching one journey file; the budget is 1.0 s and 3.0 s, two trees compared by CPU time over interleaved runs, and no gate component holds it, since wall time on this machine follows its load. Gate 8/8; 181 node tests on both paths. 887 lines of code (871 of them tests) against the ~500 budget. Named gaps: `claims.rs` is not run (it sleeps on tokio and stamps with the wall clock), so the journeys write its records at the fake clock; the node's lookup cannot report partiality (slice profile §8 item 11), so `partial_lookup` tests a lookup over a partial view; the authority journey reads the R9 fold for declared authority and retraction scope, not for admission, and since 2.5 `register` and `declared_exchange` already consume that fold, so "first consumer" above is stale. Pinned, not endorsed: `Registry::ingest` answers a byte-identical re-delivery `Equivocation` (`registry.rs:283-286`) where the wire store answers `Duplicate`, and at boot that sets aside the rest of the origin's chain; the fix is the owner's call, with the pin in `lost_acknowledgement`, turned round, as its failing test. For 4.4: `Records::append` folds before it saves, so a retry after a failed save answers `Ok(false)` with nothing saved; `claims.rs` has the same order.

### Step 3.5 — The gate

Goal: one script no step of Phases 3–4 can pass without.

- `glade/node/check.sh` on the witness's `check.sh` pattern: the architecture
  checker (`glade-discover/tools/architecture-check`, adopted as tooling, never a
  dependency), the ARCH-002 fixture, `cargo tree --invert` confinement of
  `shaku`, `sdax` and `iroh` to the crates allowed to see them, tests, `fmt` by
  package, `clippy`. The policy's five complementary checks (`LibraryBoundaryAndTestingPolicy.md:88-94`)
  are the checklist, and the gate says which it does not do; the checker's known
  blind spot — items under `#[cfg]` or `#[cfg_attr]` are skipped without
  evaluation, and `#[cfg_attr(all(), path = "…")]` bypasses its `#[path]`
  refusal — is recorded as a gap, not hidden. AR-09 is the criterion: dependency
  inversion and the isolated fast targets fail closed
  (`RuntimeAndAssurance.md:127`).
- Done when: the gate is green on 3.1–3.4 and red on the fixture's injected
  dependency.
- Depends on 3.1; written beside 3.2.

- **Done (the gate itself), 2026-09-23.** glade `97d6afc`. `glade/node/check.sh`: 8 components green on today's tree, run from the root with any `CARGO_TARGET_DIR`; red on an injected `shaku` (6 of 8 fail); `arch002-fixture.sh` refuses it as a normal and a `cfg(windows)` dependency. Confinement allowlist: iroh only in `glade-node`, no shaku or sdax anywhere yet. Counted gaps printed on every run: glade-node 339 rustfmt hunks and 11 clippy warnings, glade-wire 43 and 7; the checker's `#[cfg]` blind spot; untracked `node/Cargo.lock`; the policy's five checks, three only partly performed. `glade/node/architecture-policy.json` is provisional until the owner reviews it. Green on 3.2–3.4 waits for those steps.
- **Owner, 2026-09-24 ("all recommended"), on the gate's six questions:** the node policy is accepted as it stands (Step 3.2 re-reviews it when shaku arrives); the fmt and clippy gap counts ratchet, so a count that grows fails the gate; `node/Cargo.lock` and `wire-rs/Cargo.lock` are committed; one online `cargo fetch` lets confinement cover every platform; glade-wire stays in scope as a counted gap; iroh's per-module confinement stays reported, not enforced, until Step 4.2 gives the carrier its own home.
- **Done (the rulings), 2026-09-24.** glade `d922f8a`. The policy's reason says accepted. `fmt` and `clippy` gaps are `gap:N` baselines, glade-node 339 and 11, glade-wire 43 and 7: a count above its baseline fails the component and lists the new hunks or warnings, a count below it says the baseline can be lowered (both shown on a scratch copy). Blind spot, named in the gate's "Not checked" list: rustfmt merges nearby deviations into one hunk, so a new deviation beside an existing one need not raise the count. `node/Cargo.lock` and `wire-rs/Cargo.lock` are tracked (iroh 1.2.0). After one online `cargo fetch --locked`, confinement resolves every target offline, for the node and the contracts; a node dependency needing crates the cache lacks now fails the component, naming the fetch, rather than narrowing to the host. `client-rs/Cargo.lock` stays ignored (out of these rulings).
- **Done (green on 3.1–3.4), 2026-09-24.** At glade `f39daa9` the gate passes all 8 components, with shaku and the sdax crates confined to glade-node and the baselines at glade-node 338 and 11, glade-wire 43 and 7. With it, Phase 3's exit holds.

Phase 3 exit: assembly, lifecycle, eight journeys and the gate, all deterministic.
This phase depends on Phases 1–2 in one place only: 3.4's denied-authority journey
reads the R9 fold, and that one test waits for 2.5.

## Phase 4 — Real adapters, then the route (build entry step 3)

Milestone: each fake replaced by the real thing, one at a time, with the I/O and
fault evidence a fixture cannot supply (`GladeBuildEntry.md:57-60`); ends with
two nodes on different networks completing a home sync round through n0's
relays. Every adapter runs its contract's conformance suite (LBT-009). These
tests stay out of the pure loop (`:60`).

- **Owner, 2026-09-24: "start phase 4 when 3.4 lands."** The steps run one at a time:
  they share the glade-node checkout and its gate, so parallel agents would see each
  other's unfinished edits. 4.4 goes first, because 4.1 cannot start until the owner rules
  on what the slice profile leaves open (`glade/GladeFirstSliceProfile.md` SP-P3, SP-P4,
  §8 items 5 and 6). Among those: a verifier cannot get a node's public key from today's
  `sha256(node.key)` id; the wire `Op` has no field 11 for a signature, and adding one
  changes the wire IR, which §3 excludes; and clients send fully formed ops under their own
  origins (`server.rs:262-282`), so the node key alone cannot sign every op. While 4.4 runs,
  a read-only design note for 4.1 sets out those choices, each with a recommendation, for
  the owner to rule on.

### Step 4.1 — Genuine signing and the key

Goal: the three stub seams become ed25519 under the node key; grants are signed
under the node chain.

- One ed25519 crate, pinned; `verify_peer` (`peer.rs:99-101`) checks a signature
  over the domain-separated digest (`peer.rs:76-81`) with the claimed node's key;
  `verify_origin_sig` (`peer.rs:143-145`) real; `sysdir.rs:263` real. `NodeId`
  stays `sha256(key)` unless the owner rules otherwise (§4); this plan does not
  change identity derivation. The proof profile from 2.10 names the encoding, so
  an incompatibility with the discovery `Signer`/`Verifier` types is an explicit
  blocker to this adapter, not a fixture.
- Key custody: `node.key` at 0600 stays; recovery material is minted at first
  setup and written where the operator names, offline; rotation is out of scope
  and recorded as a gap in 5.1.
- Done when: a tampered HELLO is refused; a forged origin signature is rejected
  at ingest; the signature-api conformance suite passes on the adapter.
- Depends on 3.1 (`SignerPort`).

- **Decision note, 2026-09-24,** glade `18b8524`: `glade/dev-docs/GladeNodeSigning.md` sets out D1–D11, each with a recommendation and an open ruling line: Ed25519 through `ed25519-dalek =3.0.0` (stable, already cached, no new crate); NodeId becomes the node key's public key, so every id changes once; a signed envelope inside `home` records, with no wire change; HELLO bound to the connection's TLS session, ALPN `glade/node/2`; unsigned directory records set aside once at 4.1b's first boot; 4.1 split into 4.1a (key, identity, HELLO), 4.1b (directory records signed, after 4.4) and 4.1c (custody). Its findings: the iroh endpoint key is new on every start, and 4.2 needs a stable one; any websocket client could write `home` (closed by 4.3's part 1), and the handshake checks no `Origin`; `prev` is not required after seq 0 (B5 requires it); boot verification grows with renewals, about 8,640 records a day per served share.
- **Owner, 2026-09-24 ("all recommended"):** D1–D11 as the note recommends. 4.1 becomes 4.1a (the key, the identity and HELLO), 4.1b (`home` records signed, after 4.4) and 4.1c (custody and the local overlay's check). The id becomes the node key's Ed25519 public key (§4 item 4). `ed25519-dalek =3.0.0` joins the node policy with 4.1a, reported for the owner's review as shaku and sdax were.
- **4.1a done, 2026-09-24,** glade `322f606` and `54e5999`. `node/src/signing.rs`: the Ed25519 `SignerPort` adapter (`NodeSigner`, in place of `PendingNodeSigner`) with D7's tags and `verify_strict`; the id is the hex of `node.key`'s Ed25519 public key; a new key's seed comes from `getrandom =0.4.3`, which ends Windows' missing `/dev/urandom`; `ed25519-dalek =3.0.0` (the lock moves both dalek crates off their release candidates, no crate added). HELLO signs a CBOR transcript of protocol, role, node id, both endpoint ids and 32 bytes of TLS keying material, on ALPN `glade/node/2`: tampered, replayed and reflected HELLOs are refused. At the first boot on this build the node's old-id records go once to `records.legacy-<date>.json`, and at adoption the served store's old-id `home` journal is renamed aside (D8's store half, taken early; its question 1). Gate 8/8, 226 tests on both paths, fmt baseline 325; grazel, glade-gwz and glade-gyld at baseline; the Pi passes everything and dabeest everything but one test that read a Windows-locked file (fixed in `54e5999`), bar the two suites that need sibling repos. The desk's first restart on it prints a new `node` id and `set aside N record(s) of old node id …`, re-registers its apps and serves at epoch 1; principals come back as tabs say Hello. **Questions, each recommended by the step:** keep the early store set-aside; accept `ed25519-dalek` and `getrandom` in the node policy; keep a refused dialer's close silent and report refusals locally at 4.2; keep the legacy files as history and prune them by hand after 4.1b. Stale outside the step's files: `dev-docs/IrohReview.md:575` (ALPN 1), and the old ALPN or `sha256` id in `GladeSystemDataSeamNotes.md`, `GladeDirectoryNotes.md` and `GladePeerSyncNotes.md`.
- **Owner, 2026-09-24 ("all recommended"), on 4.1a's questions:** the served store's old-id `home` journal stays set aside at 4.1a; `ed25519-dalek =3.0.0` and `getrandom =0.4.3` are accepted in glade-node's policy; a refused dialer's stream closes without a reason, and 4.2 reports refusals locally; the legacy files stay as history, pruned by hand after 4.1b.

### Step 4.2 — The binding record and the door

Goal: `transport_key_binding` built — a node's iroh key bound to its Glade
identity by a record, checked at HELLO and at accept.

- `NodeTransportBinding{node, endpoint_id, valid_from, sig}` added to the system
  data IR (`glade/node/ir/sysdata.taut.py`; the `--legacy-codec` regeneration flag
  `GladeProgramStatus.md:29` records still applies), folded set-union with
  revocation-wins (`IrohGladeMapping.md:387-395`); minted at boot after 4.1; HELLO
  verification requires the presented `node_id` to be bound to
  `Connection::remote_id()`; accept-time refusal of unknown keys through iroh's
  `EndpointHooks` (`after_handshake`, the only hook iroh offers) when policy
  requires; the behaviour while the clock is uncertain is stated and, for the
  slice, fails closed. `NodeHello` keeps its three fields
  (`wire-rs/src/generated.rs:285-289`): the binding is a record, not a wire change.
- This is what the relay ruling calls the lock on the door; until it lands,
  endpoint ids stay on our own machines.
- From 3.1's `CarrierPort`: a link does not yet expose the remote transport identity;
  HELLO's check needs it, so this step adds the accessor. Closing an endpoint ends its
  links, so the iroh adapter tracks its links (the witness measured that a surviving
  connection keeps the socket bound).
- Done when: an unknown endpoint key cannot complete HELLO; a bound one can; a
  revoked binding is refused once the fold has seen the revocation.
- Depends on 4.1.
- **From the 2026-09-24 rulings:** a stable iroh endpoint key comes first (the signing note's F1): `bind_endpoint` mints a new one on every start, and the binding record, 4.5's `--peer` targets and the relay crossing all need it to last. It depends on 4.1a, not all of 4.1.
- **4.1b, first half, 2026-09-25,** glade `02e2a9e`: every `home` record is signed by its node: a `SignedRecord{record, sig}` envelope under the `origin-op` tag wraps all 11 record kinds (4.2a's transport records keep their inner signatures), sealed by a booted node's registry, and checked at every `home` ingest (records.json at boot, the registry, the served store's append and open, so the pull, the push and the seed): envelope, directory form, `prev` after seq 0, the stream's own kind, a node id as origin, a strict signature; a malformed signed record no longer panics a fold. D8: an unsigned store sets its records aside once (`records.legacy-<date>.json`, `*.log.legacy-<date>`) and mints them again; 4.1a's old-id pass is folded in. ALPN `glade/node/3`. The crypto crates build optimised in dev builds (a check 0.29 ms to 46 us; for the owner's review). Gate 8/8, 291 tests on both paths, rustfmt 300; the six downstream suites at baseline against the rebuilt binary (inode 401046031). Replayed on a stand-in of the desk: the first start sets 26 records and one journal aside and registers both apps again, the second changes nothing, no stderr. **The desk's next restart takes this one-way upgrade** (an older binary then exits on its store, until records.json and the `home` journal are moved aside). **Open for the owner:** part 2's known set (D9 as ruled, (a), recommended), and the built choices in the assembly note's 4.1b section. Part 2 (D9's deferred path) is next.
- **4.1b done, 2026-09-25,** its second half, glade `7cd2311`: D9 as ruled, option (a). The mesh knows itself and the peers whose HELLO it verified this run; each pull or push is one round, and a `home` chain from any other node is deferred with the rest of its chain (never stored or folded, asked for again at the next pull) and reported on stderr, as is a chain the store refuses. Hardening: a `home` record in a format this build cannot read refuses the start with exit 1 and a clear line, writing nothing, where an older build panicked. Gate 8/8, 297 tests on both paths, rustfmt 299; the six downstream suites at baseline against the rebuilt binary (inode 401201248); the desk's restart replayed from part 1's store and from a 4.3-era store. A third node's records wait until this node meets it, so 4.2b's introductions do not fire (ruling (a); (b) and (c) are with the owner). The pull-on-gap step is next.
- **4.2a done, 2026-09-24,** glade `40647d2`: `endpoint.key`, a second class-1 secret at 0600 never derived from `node.key`, gives the endpoint one id across starts; `NodeTransportBinding{node, endpoint_id, valid_from, sig}` and `NodeTransportRevocation{node, endpoint_id, sig}` go on their own `home` streams, signed under `glade/v1/transport-binding\0` and `glade/v1/transport-revocation\0`, and fold as a set union with revocation winning for good; a record counts only if canonical, strictly signed and in its node's own chain; an unreadable clock makes nothing live; a boot binds its key once and revokes a replaced one, and a restored revoked key refuses the boot. Gate 8/8, 246 tests on both paths; the six downstream suites at baseline. **4.2b, the door, waits on the owner:** a peer's binding arrives only by the sync its HELLO opens, so first contact needs a rule; the step recommends (a2), an accepting node lists a peer's endpoint id with no address, the door is closed by default on booted nodes, and bindings learned through a configured peer's directory count as known. Also open: separate tags (keep), the clock rule (keep), a restored revoked key refuses the boot (keep), and the link-tracking iroh adapter as a step of its own (4.2c) or folded into 4.5.
- **Owner, 2026-09-25 ("all recommended"), on 4.2:** first contact by (a2): an accepting node lists each peer's endpoint id with no address (known, not dialed), the door is closed by default on booted nodes, and bindings learned through a configured peer's directory count as known; the two records keep their own tags; the clock rule stands (uncertain means unreadable, no skew margin); a restored revoked key refuses the boot; the iroh adapter that tracks its links is a step of its own, 4.2c.
- **4.2b done, 2026-09-25,** glade `004a7d1`: the door, as ruled. A booted node refuses at accept, in iroh's `after_handshake`, every endpoint key that no binding record it holds names and no `--peer` entry configures; HELLO, both ways, refuses a node not bound to its connection's key, save a configured key on first contact; a revocation that lands closes the revoking node's live link on that key. `--peer <endpoint-id>` configures a key and dials nothing, and a one-sided `--peer` no longer links. The refusing node prints `peer refused: endpoint <id>: <reason>`; the dialer learns no reason. `CarrierLink::remote_id`, with conformance probe CA-005, and the contracts' policy requires it (for the owner's review). Gate 8/8, 256 tests on both paths; the six downstream suites at baseline, grazel now 29 + 3. The desk sees nothing new at its next restart: grazel passes no `--peer`. **Open for the owner:** a first-contact link that a later record contradicts stays up until it closes, and the next connection is refused (recommend: leave it for the slice); a node must start once before its peer can name its endpoint id (recommend: 4.5's configuration prints it without serving).
- **Owner, 2026-09-25 ("all recommended"), on 4.2b:** `remote_id` in the contracts' policy is accepted; a first-contact link that a later record contradicts is left for the slice (it stays up until it closes, and the next connection is refused); 4.5's configuration prints a node's endpoint id without serving.
- **4.2c done, 2026-09-25,** glade `cc3f158`: `IrohCarrier`, the iroh `CarrierPort` adapter, in place of `PendingIrohAdapter` as the peer role's provider. Each link is a QUIC connection on the adapter's own ALPN, `glade/carrier/1`, opened by the word `gcl1`; `remote_id` is the id the TLS session proved; closing the port ends every tracked link and frees its UDP port. It passes CA-001..005 on real iroh over loopback. Neither root lends it a key yet, so it refuses to bind: the mesh stays on `PeerEndpoint`, and the desk sees nothing new. Gate 8/8, 265 tests on both paths, rustfmt's baseline down to 317; the six downstream suites at baseline. **Open for the owner:** when the mesh moves onto the port (recommend a step of its own after 4.5: it needs HELLO's exporter bytes through the port, the door on the adapter's endpoint, the sync driver on link frames, and 4.5's bind address and relay); keep the adapter's own ALPN and first word (recommend yes); bound the wait for the first word when the adapter first faces other machines (recommend yes, at the mesh's move or 4.5). Named gap outside the gate: the async witness's `Cargo.lock` lacks glade-node's `ed25519-dalek` and `getrandom` entries since 4.1a, so `--locked` refuses it. Fixed 2026-09-25 in glade `5487fa2`, two dependency edges and no new package (locked at root `d5175f6`, whose message misnames it `7b0`).
- **Owner, 2026-09-25 ("all recommended"), on 4.2c:** the mesh moves onto the carrier port in a step of its own after 4.5, Step 4.5b; the adapter keeps its own ALPN, `glade/carrier/1`, and its first word, `gcl1`; the wait for the first word gets a bound when the adapter first faces other machines, which is 4.5b.
- **Loopback alone, 2026-09-25,** glade `b6a6498` and `1192bf2`, found when the owner reported macOS firewall dialogs for every new node and test binary. iroh 1.2 pre-binds `0.0.0.0` and `[::]`, and the node's loopback IPv4 bind replaced only the first, so every endpoint also listened on `[::]`, every interface; and iroh's portmapper opened a UDP socket on every interface and probed the router over UPnP. Both are off: a booted node listens on `127.0.0.1` alone, its endpoint's UDP socket and its websocket (lsof on the Mac; `ss` on the Pi, before and after). New test `an_endpoint_listens_on_loopback_alone`, red on the Pi with the fix removed (`[::]:42077 listens beyond this machine`). The portmapper has no automated test, since iroh reports no portmapper state without relays: the socket lists are its evidence. Gate 8/8, 266 tests on both paths. The Pi now holds grazel, glade-gwz and glade-gyld beside glade (pushed to GitHub for it, 2026-09-25), so its node suite runs whole: 266 passed.

### Step 4.3 — The grant check at the serve hop

Goal: `metadata_exposure` built.

- `GrantPort` consulted before any share leaves the node on the four paths:
  `peer.rs:193` (`serve_sync` zones — the comment at `:172-174` names the spot),
  `mesh.rs:264` (`serve_peer_subscribe`), `exchange.rs:232` (`serve_peer_exchange`),
  `server.rs:215` (websocket subscribe). The directory (`HOME`) is exempt: it is
  how grants arrive. Fail closed: no grant visible, nothing served. A revocation
  cuts a live subscription when the fold changes (the authorization model's §6
  re-evaluation rule). Keyed on the peer's node identity as 4.2 binds it, or on
  the session's principal, which is still claimed on the client's word
  (`server.rs:171-174`) until session identity lands — the test names say which.
- Done when: the end-to-end tests that today demonstrate the leak
  (`mesh.rs:520-584`, `exchange.rs:598-`) assert the refusal instead; a
  stale-fold test shows the fail direction; a revocation mid-stream ends the
  stream.
- Depends on 3.1 (`GrantPort`); may land before 4.2, keyed on the claimed
  identity, with the tests saying so.
- Preconditions carried from the v1 amendment's review (`glade/GladeDeclAmendment-RemPlan.md`,
  SUR-P3-4 and SUR-P3-6): before grants are enforced, the route that revokes a seeded
  grant is documented beside `seed` on `glade/docs/AppFileFormat.md`; the page defines what
  `service <name>` and a seed's `<share>` refer to, and the verb and principal vocabulary;
  the shipped `seed` lines follow that definition (corrected together with the revocation
  route, so the old grants can be withdrawn); and the node warns on a seed whose share no
  loaded `workspace` declares. **RULED (Gianni 2026-09-23): a seed's share is the workspace
  share** (gyld-app's convention), not a share named after the app. The format page's own
  example (`seed owner ws-notes notes.*`) and gyld-app's comment say so, and no file
  declares a share named after an app; grazel-app's `seed owner grazel …` is corrected with
  the revocation route.
- **RULED (Gianni 2026-09-23), from 3.1:** a node's own grant is an ordinary grant record
  whose principal is the node id (a node does not inherit its operator's grants). No new
  record kind; `GrantPort::check` stays one path over one fold.

- **Stopped at its tripwires, 2026-09-24,** glade `d838bd0` (a design note, no code): no route revokes a seeded grant; verbs and principals are undecided; and enforcing would refuse every client flow outside `home`, since the desk presents a principal per tab, the suites none and the suppliers `grazel`, while every seed grants `owner`. Eight owner questions, each with a recommendation, are in `glade/dev-docs/GladeNodeAssembly.md`, "Grant check at the serve hop (plan Step 4.3)". **Part 1, 2026-09-24,** glade `e0100dc`: a client's op on `home` is refused (`Unauthorized`) and never stored (ruling H-R3); no shipped client writes `home`; peers can until 4.1b. Gate 8/8, 207 tests on both paths; the grazel, glade-gwz and glade-gyld suites at baseline.
- **Owner, 2026-09-24 ("all recommended"):** revocation by an app-file line `revoke <principal> <share>`; a read asks `read.subscribe`, an exchange its own glade id, and a stored `p.*` admits every verb that begins `p.` (a sentence and a pattern probe in `grant-api`); a principal of 64 hex digits names a node and no session may claim one, a session with no principal holds nothing, and `owner` is the owner; the peer paths are enforced by default and the websocket path behind a switch that is off by default, until the `Origin` check lands and the desk presents a granted principal; a refusal is an empty `Heads` and then `Error{Unauthorized}`; the `Origin` check accepts no `Origin` or a loopback one and refuses the rest; 4.3 goes before 4.1b.
- **The refusal's form, settled 2026-09-24:** the empty `Heads` names no zone, `Heads{streams: []}`, then the `Error` (the client-writes plan's answer 2, the later ruling; `glade/dev-docs/GladeSubstrateV1.md` §6, R6). The 4.3 design note's wording, an empty ack for the zone, is superseded: that form is indistinguishable from an accepted ack of an empty zone.
- **Part 1, landing (i), 2026-09-25,** glade `dea365e`: `revoke <principal> <share>` parses, registers on `dir.revocations` under the registrant's chain and beats a seeded grant; a seed whose share no loaded `workspace` declares is warned, so grazel-app's two `seed owner grazel` lines warn at the desk's next restart until (ii); the format page defines the route, `service <name>`, the verbs and the principals. Gate 8/8, 270 tests on both paths; the six downstream suites at baseline against the rebuilt binary (inode 400427243), which the desk runs at its next restart. Landing (ii), the app files (grazel-app's seeds on `ws-razel` with `revoke owner grazel`, the two fixtures, the census counts), follows now that a binary that parses `revoke` is on disk: today's older binary refuses such a file. **Open for the owner:** five questions in the assembly note's "Part 1, landing (i)".
- **Part 1, landing (ii), 2026-09-25,** glade `1b62a69`, grazel `24cfc98`, glade-gyld `bfa30c3`, glade-gwz `9bfb3ab`: grazel-app's seeds name `ws-razel`, and `revoke owner grazel` withdraws the grants the old lines made, in both byte-identical homes; the two fixtures seed `ws-razel`; the census counts revocations and asserts no warning. No production code: the binary on disk (inode 400524643, built from this tree) is the one that parses `revoke`. At its next restart the desk registers `+2 record(s)` (the `gwz.*` grant on `ws-razel` and the revocation), with no warnings. Gate 8/8, 270 tests on both paths, rustfmt 315; the six downstream suites at baseline. Part 1 is complete; part 2, the check, is next. The fixtures changed one commit after the warning, not with it (precondition 3), because the `revoke` line had to wait for the desk's binary.
- **Part 2, first half, 2026-09-25,** glade `19fe640`: the check on the peer paths. `GrantPort` over the node's own grant fold (`PolicyView`, `node/src/grants.rs`), in place of `PendingGrantFold`; a forwarded subscribe asks `read.subscribe`, a forwarded exchange its glade id, and `serve_sync` leaves out a zone the peer may not read, each keyed on the node id the peer's signed HELLO proved, `home` exempt. A refused subscribe gets `Heads{streams: []}` then `Error{Unauthorized}`; a changed fold re-checks every admitted stream; a quarantined grant or revocation leaves the fold unreadable and every check refuses. `grant-api` gains `admits` (the `p.*` rule, written once) and a GR-001 probe that a principal named as a node id holds nothing. Three two-node tests now grant the reader. Gate 8/8, 278 tests on both paths, rustfmt 307; the six downstream suites at baseline against the rebuilt binary (inode 400680571). The desk sees nothing: no peers, and client sessions unchecked until the second half (the websocket switch, off by default). **Open for the owner:** four questions in the assembly note's part 2 section.
- **4.3 done, 2026-09-25,** part 2's second half, glade `4586e1b`: `--enforce-client-grants`, off by default on both roots (grazel does not pass it). With it on, a websocket subscribe to a share other than `home` needs the session's principal, as its Hello claimed it, to hold `read.subscribe`, and a session naming none holds nothing; the re-check pass covers client zones. Always on: a Hello naming 64 hex digits (a node id) binds no principal. Gate 8/8, 282 tests on both paths; the six downstream suites at baseline against the rebuilt binary (inode 400818912). The desk sees nothing. With the switch on today, every desk tab (a random principal) and the suppliers (`grazel`) would be refused on `ws-razel`, and only `owner` served: the gap the appearance plan's principal steps (gyld-ui 1.2, the page 1.3) close. Not checked: a client's writes and exchanges, provider attach; the principal is the client's word until session identity lands.

### Step 4.4 — Durable-local acceptance and restart

Goal: the real store behind the persistence and operation-store ports; restart
and retry honest.

- Durable-local acceptance before any sync effect; a node restarted mid-round
  resumes from its heads; the persistence-api conformance suite (PS-001..008) and
  the operation-store conformance suite run on the real adapter, plus what they
  cannot supply: interrupted writes, concurrent handles, cancellation of a
  pending future (`glade/contracts/README.md`). The Glade-core findings from the
  witness period — the fire-and-forget append and `subscribe()` discarding heads
  — are fixed here where they bite a journey, each with its failing test first;
  `latest` retention enforcement stays out unless R2 landed it.
- Done when: the restart journey passes with the real store and the eight
  journeys of 3.4 pass with the real store substituted.
- Depends on 3.4.

- **Done, 2026-09-24,** glade `f5d055f`. Durable-local acceptance (SP-L1): the node's own directory writes go through `Registry::accept`, which saves a staged copy to records.json, file and directory synced, before it becomes the fold, so nothing unsaved is read, published or pushed. `claims.rs`'s serve, renewal and principal mints and the assembly's record host all use it, and a failed save leaves nothing behind for its retry. The served store writes each record in one write and cuts a torn tail at open; before, the next append made such a log unreadable. **Owner, 2026-09-24:** a byte-identical re-delivery is a duplicate, taken as held (`Ingested::Duplicate`), where it was `Equivocation`, and boot no longer quarantines the rest of a chain that holds a repeat. `restart_mid_round` and `retry_after_a_failed_save` join the journeys; `tests/durable` runs all ten over records.json in temp directories, with three adapter tests, outside the fast loop (15 tests, 0.4 s). Fast loop: 37 tests, 0.14-0.15 s warm. Gate 8/8, 205 tests on both paths, the fmt baseline down to 337; the grazel, glade-gwz and glade-gyld suites pass against the rebuilt binary. Default-path changes (the design note lists five): records.json saves are synced, 3-10 ms each, one per renewal tick among others; a mint whose save fails is neither kept nor published; a torn log tail is repaired. Production code grows 147 lines net; tests add about 800 (150 moved). The design is `glade/dev-docs/GladeNodeAssembly.md`, "Durable store and restart (plan Step 4.4)". **Not done, for the owner:** PS-001..008 on the real store needs `glade-persistence-api` as a dependency and a revision stored with the bytes (recommended: an optional revision field in `SystemSnapshot`, as its own step); the operation-store suite is blocked by SP-P3 (a), (b); the served store as record host is its own piece of work, about 400-650 lines, deferred by the lane owner. **Named gaps:** a crash leaves `instance.lock` behind and boot then refuses; `claims.rs` publishes after releasing its lock, so two mints on one chain can reach the served store out of order and stall that chain there until the next boot (this predates the step); there is no outbox, so a lost push waits for the next pull; off Unix the rename is not synced. The witness period's two findings are on the WebSocket client path (an accepted append gets no answer and a refusal no correlation id; `subscribe()` drops the heads ack); they affect no journey and are recorded, not fixed.
- **Owner, 2026-09-24 ("all recommended"):** PS-001..008 run on records.json with an optional revision field in `SystemSnapshot` (today's files read as revision 1), as a step of its own; `instance.lock` becomes an OS lock (`File::try_lock`); `claims.rs` publishes under its lock, with a test that forces the race; a lost push waits for the next pull; the client libraries' two gaps (no answer to an append, the heads ack dropped) get a plan of their own.
- **Hardening, 2026-09-24,** glade `1501a67`: a websocket upgrade whose `Origin` is not loopback gets `403` (4.3's question 7); `instance.lock` is an OS lock (`File::try_lock`), so a crash no longer refuses the next boot, and on Unix a boot checks that the file it locked is still the one at the path (4.4's question 3); a mint keeps the directory lock until its records are in the served store, and pushes to peers after it (4.4's question 5). `rust-version = "1.91"`; the fmt baseline drops to 333. Gate 8/8, 213 tests on both paths; the grazel, glade-gwz and glade-gyld suites at baseline. **Transition:** a node still running the old binary holds only the file, which the new lock does not see, so the desk restarts with `gyld-ui.py stop` then `start`, never `start` alone. Open: a push that reaches a peer out of order still stalls that chain there until the next pull.
- **Owner, 2026-09-24 ("all recommended"):** `rust-version = "1.91"` stays; a node that refuses a pushed record as a gap pulls from the pusher at once, a small step of its own before 4.5; the lock's file-identity check stays Unix-only for the slice; the fmt baseline stays 333. The client libraries' plan (`glade/dev-docs/GladeClientWritesPlan.md`, glade `4cb4aaa`) is ruled as recommended too.
- **Pull on a gap done, 2026-09-25,** glade `d69fdce`: a pushed `home` op the store refuses as a gap (including a first op past seq 0, 4.1b's connect race) makes the receiver pull the pusher's `home` share from its heads at once, on the live link, through the pusher's existing `serve_home`: no wire change. One pull per pusher at a time; gaps noted meanwhile merge into it, and a gap noted during a pull that it did not heal gets one more. A deferred chain (D9), a chain break, a record that does not verify and a pull's own round start nothing; app shares are never pushed, so none. One line per pull reports what it healed. Gate 8/8, 300 tests on both paths; the six downstream suites at baseline against the rebuilt binary (inode 401399878); the desk's restart replayed, unchanged. **Open for the owner:** three answers as built, in the assembly note's "Pull on a gap" section.
- **The persistence suite, part 1, 2026-09-25,** glade `83ba787`: records.json stays the canonical CBOR `SystemSnapshot` and gains an optional key 3, `revision`; a file without it reads as revision 1 with its bytes unchanged, and an older node reads the new file and drops the revision when it saves. `RecordsFile` swaps against the revision it read, under an OS lock on `records.json.lock`, with 4.4's temp-file, sync, rename and directory-sync order; a damaged records.json is `Corrupt` with a reason and refuses the start (exit 1, a clear line), where it panicked. Gate 8/8, 310 tests on both paths; the six downstream suites at baseline against the rebuilt binary (inode 401590725); the desk's restart replayed both ways, a downgrade included, unchanged. **Stopped for the owner:** PS-001..008 commit arbitrary bytes, which a snapshot-shaped records.json cannot hold; five options in the assembly note, (a) recommended, snapshots only, the probes through a fixture. Part 2 (the dependency, the policy change, the port and the suite, ~60 production lines) follows the answer. Found on the way: the first boot's `home` claim has a 30 s lease that nothing renews, so later starts print `home served: false` (routing never reads it: `home` is always local).
- **The `home` claim renewed, 2026-09-25,** glade `01514b4` (the lane owner's fix): `home` joins the renewal set at adoption, at the node's own epoch (1), renewed once at once, so a claim that lapsed while the node was stopped is live before anyone connects; the start line prints after adoption, `registry ready (home served: true)`. Gate 8/8, 313 tests on both paths, rustfmt 298; the six downstream suites at baseline against the rebuilt binary (inode 401708181). **Cost, for the owner:** one more renewal record every 10 s, doubling the desk's to about 17,300 a day; nothing compacts them (AZ-12's checkpoints), records.json is rewritten whole at each renewal, and boot verifies every signature.
- **4.1c parked, 2026-09-25 (owner: "park this current work"),** to institute the no-globals rule (`dev-docs/ProcessGlobalsPlan.md`). The work in progress is in glade's stash, `stash@{0}`, "4.1c WIP, parked 2026-09-25 for the no-globals check": the design section "Custody and the local overlay's check (plan Step 4.1c)" in `GladeNodeAssembly.md` (sections 1-7), the `NodeRecoveryKey` record in the IR and `sysdata.rs`, `dir.recovery-keys` in `registry.rs`, and `envelope.rs`'s new stream, uncompiled. Still to build: `overlay.rs`, `recovery.rs`, the boot and both roots' wiring, the test updates and new tests, then the gate, the suites and the replay. Its warning for the desk will name `glade-node recovery --name grazel --out <a path outside GLADE_HOME>` with the desk's `GLADE_HOME`. Resume with `git stash pop` in glade, after `ProcessGlobalsPlan.md`'s node Steps 2.1 and 4.1 land (owner, 2026-09-26).
- **4.1c done, 2026-09-27,** glade `1ce4b46` (resumed 2026-09-26 after the process-globals node steps). D10(a): `glade-node recovery --name <name> --out <absolute path outside GLADE_HOME>` boots the stopped instance (refused while the node runs), writes the recovery secret, a bare 32-byte seed, 0600, never over a file, then commits a `NodeRecoveryKey {node, recovery_key}` on a new stream `dir.recovery-keys` in the node's chain; one key per node. `--recovery-out` works at a node's first boot only. Until a key is committed, each start prints one stderr line naming the command, with the instance's `GLADE_HOME` and the running binary, and starts. `local.json` must be the node's own envelope under `glade/v1/local-overlay\0`; this build knows no overlay entries, so anything else is discarded to the fail-closed defaults with one line. The instance root and the program path come from the entry point (no process globals; the checker stays at 0 debt; `env::args` is now read once). Gate 9/9, 326 tests on each path, rustfmt 295; the six downstream suites pass against the rebuilt binary (inode 405091294). Desk replay: the same lines, plus the warning on stderr. **One-way:** once a key is committed, a build from before 4.1c refuses that instance. About 1,000 lines against D11's 250. **Open for the owner:** five answers as built (an absolute `--out`; one key per node; the bare seed; `--recovery-out` at first boot only; when to run it on the desk), in the assembly note's 4.1c section.

### Step 4.5 — Relay configuration and the first crossing

Goal: `relay_posture` built and measured.

- A configuration (`ConfigPort`) carries `RelayMode` — `Default` is n0's relays,
  the slice's choice; `Custom(RelayMap)` is the later switch — the bind address
  (today hardcoded loopback, `iroh_carrier.rs:35` and `:103`), and the peer list;
  no lookup service; a peer is `endpoint-id@relay-url` or `endpoint-id@ip:port`
  (`bin/glade-node.rs:43-46` today). `IrohGladeMapping.md:420-426` (self-host for
  the first slice) and `GladeDiscoveryModel.md:198` (the public iroh relay) each
  get a one-line note that the ruling of 2026-09-22 governs. Endpoint ids stay in
  configuration files at 0600 and out of logs.
- **From the 4.2b ruling (owner, 2026-09-25):** a way to print a node's endpoint id
  without serving, so each machine's peer entry can be written before its first start.
  Since 4.2b's door, the accepting side must name the dialer's key with `--peer <id>`
  (or hold its binding), so both machines need each other's id beforehand.
- **The portmapper** (UPnP, PCP, NAT-PMP) is off since glade `1192bf2`, with the loopback bind: 4.5
  decides it with the bind address and the relay mode. It probes the router and opens a UDP socket on
  every interface, which is what raised macOS firewall dialogs.
- Measurement: two nodes on two networks — the owner names the machines (§4) —
  complete HELLO and a home sync round through the relay; whether the direct
  path is taken afterwards is recorded; port release after close is measured as
  in the witness.
- Done when: the crossing is recorded with the relay used, the round-trip, and
  what n0 could see (endpoint ids, IP addresses, timing, volume; nothing else).
- Depends on 4.2 — or, if run before it, on a private endpoint id and a note
  that says so.
- **First run on dabeest, 2026-09-24** (glade `39b8b25`, pulled from GitHub; Rust 1.98.1, MSVC): the node builds in 54 s. `sysdir.rs`'s `random_key` reads `/dev/urandom`, so no node can make its key on Windows: 15 unit tests, `assembled_path` and, by all signs, `lifecycle` fail on it. `binding_census` and `shipped_app_files` read the sibling repos' app files, which dabeest has not got. Every other suite passes, the journeys and the durable target among them. 4.1a makes key generation portable. Both machines are on the owner's LAN, so a two-network crossing needs one of them elsewhere, a phone hotspot say; 4.5 records which path is taken either way.
- **First run on the Pi, 2026-09-24** (a Raspberry Pi 5, Debian 13 aarch64, Rust 1.96; glade `39b8b25` pulled from GitHub): builds in 189 s; every suite passes, `lifecycle` and `stop_signal` among them, except `binding_census` and `shipped_app_files`, which need the sibling repos.
- **4.2b on both machines, 2026-09-25** (glade `63a5799`, pulled from GitHub): the node suite runs in 38 s on dabeest and in 133 s on the Pi. On both, every suite passes, 4.2b's door tests among them (`assembled_path` 7, `lifecycle` 4, and the mesh's over real iroh in the library), except `binding_census` and `shipped_app_files`, which need the sibling repos. `stop_signal` has no tests on Windows; the Pi passes its 4.
- **Owner, 2026-09-24:** hold off on firewall and NAT traversal checks. Hole punching is iroh's function, not glade's, and nodes that find each other have most likely punched through anyway. 4.5 runs on the two named machines on the owner's LAN, with n0's relays configured; the path iroh picks is noted, not measured. The full end-to-end check across two different networks comes later, outside Phase 4.
- **Design, 2026-09-27,** glade `5399664` (a design note, no code): `GladeNodeAssembly.md`, "Relay configuration and the first crossing (plan Step 4.5)". A `--config` file (absolute path, 0600, a line format, refused whole if a line is bad) carries `relay off|n0`, `bind` and `peer` lines; without one, every profile is unchanged: loopback only, relays off, portmapper off. `glade-node endpoint-id --name N` prints an id without serving (minting `endpoint.key` under the instance lock if absent), and ids travel between machines in 0600 files, never a terminal; lines name endpoints by a 10-hex tag. The node calls no iroh helper that reads the environment. The portmapper stays off with no switch. n0 sees ids, their pairing, public IPs and NAT mapping, timing, volume, and also the ALPN of relayed first packets and a 5 s relay ping beside a direct path. Crossing: the Pi accepts, dabeest dials; run 1 all through the relay, run 2 on each machine's Wi-Fi address to see whether iroh goes direct. Split: part 1 (~440 production lines), part 2 (status lines, ~160), then the crossing. The desk sees one change: its `peer` line shows a 10-hex tag. **Open for the owner:** eleven questions in the design section.

### Step 4.5b — The mesh on the carrier port

Goal: the node's peer mesh runs on `IrohCarrier`, 4.2c's `CarrierPort` adapter, in place of
`PeerEndpoint`. Ruled a step of its own after 4.5 (owner, 2026-09-25); placed before 4.6 so that 4.6's
journeys run over the carrier the node keeps.

- What it needs (`glade/dev-docs/GladeNodeAssembly.md`, 4.2c's question 1): HELLO's exporter
  bytes (D6) through the port, or a session-level replacement for them; the door's hook on the
  adapter's endpoint; the sync driver on a link's frames; 4.5's bind address and relay mode; both
  roots lending the adapter the node's endpoint key.
- A bound on the acceptor's wait for the first word, `gcl1` (ruled with 4.2c); the adapter keeps
  its ALPN, `glade/carrier/1`.
- Design first, in `GladeNodeAssembly.md`, with any owner questions, as for 4.2.
- Done when: the mesh's tests, the door's and 4.5's crossing pass with the mesh on the port.
- Depends on 4.2c and 4.5.

### Step 4.6 — The fixed-peer route end to end

Goal: the build entry's acceptance sentence, verbatim, as one script.

- A real registration on node A is discoverable from node B across the route
  (4.5); expired entries are excluded (lease expiry from `claims.rs`) and
  unauthorized ones are excluded (a principal without a grant, 4.3; an unbound
  key, 4.2); loss, retry and restart outcomes are honest (4.4's journeys over the
  real carrier); the fast independent development path is demonstrated (3.4's
  suite still runs in its measured budget with fakes).
- Done when: one script runs the route journey against two configured nodes and
  exits 0; its log is the evidence.
- Depends on 4.1–4.5.
- Open before two nodes load one app (STA-P3-1 of the v1 amendment's review): the binding
  family's order and retraction scope hold within one registry only. The binding lamport
  is per node (the registry never ingests a peer's op), and a `BindingRetraction` is keyed
  `(app, glade_id)` with no origin, so in a served store holding several nodes' records one
  node's retraction can outrank another's live declaration. Options: a merged clock (the
  maximum over the served store's binding family at boot), the origin in the retraction's
  scope, or both. **Owner question** from the same review (SUR-P3-5, classified
  architectural): whether a later format gives `service` and `workspace` lines a retract
  half, R9's option (s), declined for v1.

## Phase 5 — The slice passes and the graph moves

### Step 5.1 — Result document and acceptance review

- `dev-docs/GladeFirstSliceResult.md` on the witness result's shape: what passed,
  measurements, caveats, what is never a caveat, how to rerun; the recorded gaps
  (key rotation; session identity; the checker blind spot). Acceptance review at
  a settled tuple: Code and State (durable state and wire behaviour are both
  touched), plus Surface if 4.5's configuration file is a format people edit.
  Reports filed as `GladeFirstSlice-Review<Axis>.md`.
- Gaps added 2026-09-24 by the signing note: app ops stay unsigned; no account-root certification of the node key; boot verification grows with lease renewals (AZ-12's checkpoints are the remedy); bare `#[cfg]` attributes on four functions in `sysdir.rs` await migration.
- Done when: GO on every axis is filed.

### Step 5.2 — Record the trigger

- `first_real_route_slice` recorded as occurred in the rulings stream — an
  `Occurred[FirstRealRouteSlice]` ruling in the owner's notebook, through the
  same scratch-build check every ruling has had — and the desk rebuilt.
  `simulator_tooling` becomes answerable; `registry_growth` waits on
  `failure_model`.
- Done when: the desk shows the trigger occurred and the answerable count moved.

### Step 5.3 — Program status

- `GladeProgramStatus.md`'s "Where everything stands" table gains the slice row
  and its decision queue notes what the trigger opened.

## 2. Order and parallelism

Foundational first: Phase 1 (a document and the owner's word) and Steps 3.1 and
3.5 (ports and the gate) can start today, in parallel, by different agents.
Phase 2 has two lanes after 1.4 — the contract (2.1 → 2.2 → 2.8) and the node
with its app files (2.3 → 2.4 → 2.5 → 2.6 → 2.7, the document's landing order) —
which meet at 2.9; 2.10 runs beside both from the start. Phase 3's
3.2–3.4 are one lane in sequence. Phase 4's 4.1→4.2, 4.3 (independent of 4.2
with a stated key), 4.4 (after 3.4), 4.5 (after 4.2), 4.6 last.

```
1.1 → 1.2 → 1.3 → 1.4 → 2.1 → 2.2 ──────────────→ 2.8 → 2.9
                     └→ 2.3 → 2.4 → 2.5 → 2.6 → 2.7 ─────┘
                    2.10 ────────────────────────────────→ (feeds 3.4, 4.6)
3.1 → 3.2 → 3.3 → 3.4 → 4.4
 └→ 3.5              4.1 → 4.2 → 4.5 → 4.6
                     4.3 ───────────┘
                                          5.1 → 5.2 → 5.3
```

Each step is one agent on Opus with one brief that names the files it may touch,
the gate it must run, the commit-message shape and no push; the lane owner
verifies before the next step, as in the witness. Reviews happen at 1.2, 2.9 and
5.1 only; interior steps are gated by their tests and the gate.

**Order after the rulings of 2026-09-24** (the lane owner's; one agent at a time in the glade checkout, save the client plan's Phase 3, whose client-rs and client-ts lanes run side by side from 2026-09-25: their directories and gates are disjoint): hardening (landed, `1501a67`), 4.1a, the client-writes plan's Phase 2 (the node's answers, ruled to precede 4.3's websocket enforcement), 4.2 with a stable endpoint key first, the client plan's Phase 3 (both clients), 4.3's enforcement, 4.1b, the pull-on-gap step, the persistence suite with its revision field, 4.1c, 4.5 on the Pi and dabeest, 4.5b (the mesh onto the carrier port, ruled 2026-09-25), and 4.6. Beside them: the client plan's 1.1 at once (a document), glade-gwz's run ids right after 4.1a, and the client plan's Phase 4 in the supplier repositories once its client steps land. The cross-node writes plan (ruled 2026-09-24) writes its rules (X1.1) now, and its node steps follow 4.6.

## 3. What this plan does not do

- It does not carry a client's write to a share another node serves: today the write stays on the node the client reached, since the forward only reads and a node pushes only its own `home` records. The owner added cross-node writes as a planned item on 2026-09-24; its plan is `glade/dev-docs/GladeCrossNodeWritesPlan.md`, and the end-to-end app that gates any publish will likely need it.
- It does not change the wire IR, the demo, or the composition path the demo
  runs on; retiring the hand-written path is a later decision.
- It does not build address lookup, gossip, bulk transfer, the policy zone split
  or opaque wire ids; each waits on its own ruling or trigger.
- It does not resolve key rotation or social recovery; 4.1 mints recovery
  material and stops.
- It does not fix the glade-discover checker's `#[cfg]` blind spot; its tree
  carries someone's uncommitted edits. 3.5 records the gap.
- It does not carry the build entry's out-of-scope list: dynamic shard election
  or migration, million-node performance, a general trust product, arbitrary
  offline authorization, the shape × storage matrix, service placement at fleet
  scale, a new lifecycle framework, Gyld optimization (`GladeBuildEntry.md:74-78`).

## 4. Open decisions for the owner

1. Reviewer tier for 1.2, 2.9 and 5.1: Opus by the quota instruction, or the
   strongest tier the process rule asks for at a freeze.
2. R1–R11 (Step 1.4) — ruled 2026-09-23, recorded in
   `glade/GladeDeclReconciliation.md` §3.
3. The two machines for 4.5's crossing (dabeest and which other), and whether
   the owner's own machine may be one of them. Ruled 2026-09-24: a Raspberry Pi on the owner's
   LAN, which the code reaches through GitHub, and dabeest (Windows 11; its ssh shell is MinGW bash).
4. Whether `NodeId = sha256(key)` stays through the slice (4.1 keeps it). Ruled 2026-09-24: it becomes the
   node key's Ed25519 public key (`glade/dev-docs/GladeNodeSigning.md` D2); every id changes once.
5. Whether the assembled composition root is a second binary or a path in
   `glade-node` (3.2; either satisfies "alongside the demo"). Ruled 2026-09-24:
   a path in `glade-node`, off by default (Step 3.2).

## Appendix — where the facts came from

Read on 2026-09-22 at the revisions in the status line: `GladeBuildEntry.md`;
`PackageExtractionPlan.md:131-252`; `LibraryBoundaryAndTestingPolicy.md`;
`arch1/Components.md`, `arch1/RuntimeAndAssurance.md`,
`arch1/InjectionGraphRefinement.md`, `arch1/GladeArchitecture.md`,
`arch1/ReadingMap.md`, `arch1/AsyncWitnessPlan.md`, `arch1/AsyncWitnessResult.md`;
`IrohGladeMapping.md` §7; `glade/GladeDeclReconciliation.md` (revision 2, `1defe3b`),
`-RemPlan.md`; `glade/GladeMetadataExposureTable.md`; `GladeProgramStatus.md:1-36`;
`glade/node/src/*` and `glade/node/Cargo.toml`; `glade/contracts/README.md`;
`glade/dev-docs/async-witness/`; `glade-discover/Cargo.toml`,
`glade-discover/dev-docs/DraftHostContracts.md`, `RegistryContractDraft.md`;
`glade-decl/README.md:52-100`, `glade-decl/corpus/build.py`;
`glade-decl-rs/src/lib.rs`, `src/vectors.rs`; `glade-decl-ts/src/corpus.test.ts`;
`gyld/examples/glade-decisions.gyld.py`; the rulings notebook at `9d19ee6`.
