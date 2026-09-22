# Glade First Slice Plan — the fixed-peer route under the eleven rulings

Status: **DRAFT plan**, 2026-09-22. Lane: glade-wz. Written against glade-wz root
`9d19ee6`, glade `559cb2c`, glade-decl `d671f10`, glade-decl-rs `21eefa1`,
glade-discover `fd94a1f`; rulings stream `rulings@1`, snapshot `d25aa2058723`.
Nothing here is implemented by this document; every "today" statement below was
read in the tree at those revisions on 2026-09-22.

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

Phase 1 exit: accepted status plus eleven rulings. Nothing in glade-decl, the
renderings, the node or the app files has changed.

## Phase 2 — The contract pinned (build entry step 1)

Milestone: the amendment landed as one edit across the contract repositories,
every row of the document's §4.7 green on a settled tree, the acceptance review
passed, the publish recorded; and the discovery-side slice profile pinned with
executable consumer tests.

Landing order is the one revision 2 fixed under SAF-P2-3: app files first, in all
four repositories, node validation last, because an old node accepts any spelling
and a new node refuses the old one.

### Step 2.1 — Schema and corpus

Goal: `glade-decl/ir/glade_decl.taut.py` edited per R1–R8; `corpus/decl.v1.json`
replacing `decl.v0.json`; `build.py --compat` in place.

- §4.1 schema edits; §4.2 corpus, with every `decl.v0` reference retargeted (20
  hits, CON-P2-3, the first being `glade-decl-py/pyproject.toml:20`);
  `build.py --compat` asserting `v1[n].cbor == v0[n].cbor` for every `n` in v0 —
  or, under R4(a), failing by design with no superset claim left in any text
  (SAF-P1-1); the `--check` artefact list extended from three (`build.py:219-221`)
  to the four rendering copies (SAF-P2-5); an `atom` curated vector and the
  assertion that every `Shape` member has one (SAF-P3-8); any delete reserves its
  tag and name and sets `next_id` (SAF-P3-6). taut `optional` is nullable and
  always emitted (`GladeDeclReconciliation.md:103-117`): no field is free.
- Done when: §4.7 rows 1 and 2 are green and `git grep 'decl\.v0'` over the four
  contract repositories and root `dev-docs` is empty.
- Depends on 1.4.

### Step 2.2 — The three renderings

Goal: Rust, TypeScript and Python regenerated into their `src/`, gates green.

- The rendering READMEs' form (CON-P2-2): generate to scratch, copy each
  language's files into `src/`, then the ir and corpus copies for ts and py, then
  `build.py`. `CONTRACT_VERSION` (`glade-decl-rs/src/lib.rs:38`, today
  `99a04e0…`) advances to the amended commit. `pnpm test` in glade-decl-ts (its
  npm lockfile is left alone; the migration is raised at extraction step 1.2, not
  done — document §4.8), `cargo test` in glade-decl-rs, `pytest` from a built
  wheel in glade-decl-py; the `atom` grep in all three generated APIs; no
  `typescript/`, `rust/` or `python/` path in `git status`.
- Done when: §4.7 rows 3, 4, 5 and 11 are green.
- Depends on 2.1.

### Step 2.3 — App files and the format page

Goal: every `.glade` file migrated and headed per R10, with the page an author
needs written before any validation exists.

- Five app files in four repositories (`grazel/apps`, `glade/apps`,
  `glade-gyld/tests/fixtures`, `glade-gwz/tests/fixtures`) rewritten per R2, R9
  and R11. The format page states the meaning of `commons` and `private`, the
  rule for choosing, the behaviour when the token is absent, whether a mount
  overrides an authored zone (SUR-P2-1), what a changed and a deleted line do
  (R9), a members table with a gloss per enum value (SUR-P3-1), and whether an
  app-file glade id is authored or derived (SUR-P3-5). `GladeGrazelAttachNotes.md:49`
  and `:98` corrected.
- Done when: `cargo test -p grazel` boots on the shipped files (§4.7 row 8)
  against the pre-amendment node.
- Depends on 1.4 only; runs beside 2.1 and 2.2.

### Step 2.4 — Node validation and the binding fold

Goal: `appdecl.rs` refuses with line-numbered diagnostics that name replacements;
`dir.bindings` gets R9's rule.

- Zone and retention validation under R1(31b) and R2(18b), with the
  hard-error-or-warn sub-choice; diagnostics for `from-cursor`, `windowed` and an
  unknown zone in the shape of `appdecl.rs:121` (SUR-P3-2); the arity diagnostic
  at `appdecl.rs:113` updated for the sixth token (SUR-P2-4); the R9
  fold/supersede/retract for `dir.bindings`, with the test that registers the
  pre-amendment parse and then the post-amendment parse and asserts the intended
  `Registered{appended, unchanged}` and the resulting `dir.bindings` (§4.7 row 9),
  and the deleted-line test (row 10). `appdecl.rs`'s seven tests (`:292-418`)
  stay green.
- Done when: §4.7 rows 6, 7, 9 and 10 are green.
- Depends on 2.1 and 2.3; lands last by the landing order.

### Step 2.5 — Consumers

Goal: glial, grip-share with the demo, and gryth-ui unchanged in behaviour under
the new types.

- The document's §4.5 order. `Surface extends BindingDecl` still compiles in
  glial and the compile wall still errors on an undefined key (row 12);
  `manifestScope` still yields `share="account:<user>"` and `key=utf8("self:<user>")`
  (row 13); the glial `zone: "private"` test is filed as the pre-freeze item it is
  (row 14); 60 vitest suites and `pnpm build:gyld` in gryth-ui (row 15); the
  running demo untouched (row 16, manual). `glade/contracts/binding-api`, the one
  crate in `glade` that depends on the rendering (its `Cargo.toml` path
  dependency on `glade-decl-rs`), runs its `public_contract` tests (BI-001..003).
- Done when: rows 12, 13, 15 and 16 are green and row 14 is filed.
- Depends on 2.2.

### Step 2.6 — Acceptance review and publish

Goal: the freeze.

- Acceptance review on the settled tuple: the dual axes (Code, State) that an
  interface freeze makes mandatory, plus Surface because `<app>.glade` is a file
  format people edit (`GladeDeclReconciliation.md:1294-1312`). Strongest tier by
  the process rule unless the owner keeps Opus. Reports filed as
  `GladeDeclAmendment-Review<Axis>.md`. Then
  [PackageExtractionPlan.md](PackageExtractionPlan.md) step 1.1 — "Decide and
  record the GDL-041 catalogue question" (`:155`) — is closed by the amendment
  itself, and step 1.2 — "Make it a real package" (`:156`) — is the publish;
  `@owebeeone/glade-decl` and `@owebeeone/grip-core` republish per the
  document's §5.
- Nothing is published until every row of §4.7 is green on a settled tree (the
  Safety residual, §4.0).
- Done when: the publish is recorded with the tuple and the three GO files.
- Depends on 2.1–2.5.

### Step 2.7 — The discovery-side slice profile and its consumer tests

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
- Depends on nothing in 2.1–2.6; runs beside them from the start of the phase.

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
  registry, fake carrier, no-op signer, fixed configuration. Eager construction
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

### Step 3.4 — The eight journeys as consumer tests

Goal: publish, exact retry, renewal, expiry, wrong scope, unknown or denied
authority, partial lookup, lost acknowledgement — each a test against
deterministic providers, contract-faithful.

- Two commits if the budget needs it: (i) publish, exact retry, lost
  acknowledgement; (ii) renewal, expiry, wrong scope, unknown or denied
  authority, partial lookup. The fake clock drives renewal and expiry
  (`claims.rs` leases); the fake carrier injects loss and duplicate delivery; the
  denied-authority journey is the first consumer of `GrantPort` and, once 2.4
  lands, of the R9 fold. AR-05 is the criterion: publish, renew and expiry with a
  fixed authorized locator, a foreign namespace or referral rejected
  (`RuntimeAndAssurance.md:123`).
- A fixture must not pretend to be physical durability or cryptography
  (`GladeBuildEntry.md:55-56`): each fake states what it does not prove, and
  each journey names the real provider that Phase 4 substitutes.
- Done when: eight named tests are green in the fast loop and the loop's
  measured budget is recorded (LBT-010).
- Depends on 3.2; on 3.3 for the retry and lost-acknowledgement timing.

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

Phase 3 exit: assembly, lifecycle, eight journeys and the gate, all deterministic.
This phase depends on Phases 1–2 in one place only: 3.4's denied-authority journey
reads the R9 fold, and that one test waits for 2.4.

## Phase 4 — Real adapters, then the route (build entry step 3)

Milestone: each fake replaced by the real thing, one at a time, with the I/O and
fault evidence a fixture cannot supply (`GladeBuildEntry.md:57-60`); ends with
two nodes on different networks completing a home sync round through n0's
relays. Every adapter runs its contract's conformance suite (LBT-009). These
tests stay out of the pure loop (`:60`).

### Step 4.1 — Genuine signing and the key

Goal: the three stub seams become ed25519 under the node key; grants are signed
under the node chain.

- One ed25519 crate, pinned; `verify_peer` (`peer.rs:99-101`) checks a signature
  over the domain-separated digest (`peer.rs:76-81`) with the claimed node's key;
  `verify_origin_sig` (`peer.rs:143-145`) real; `sysdir.rs:263` real. `NodeId`
  stays `sha256(key)` unless the owner rules otherwise (§4); this plan does not
  change identity derivation. The proof profile from 2.7 names the encoding, so
  an incompatibility with the discovery `Signer`/`Verifier` types is an explicit
  blocker to this adapter, not a fixture.
- Key custody: `node.key` at 0600 stays; recovery material is minted at first
  setup and written where the operator names, offline; rotation is out of scope
  and recorded as a gap in 5.1.
- Done when: a tampered HELLO is refused; a forged origin signature is rejected
  at ingest; the signature-api conformance suite passes on the adapter.
- Depends on 3.1 (`SignerPort`).

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
- Done when: an unknown endpoint key cannot complete HELLO; a bound one can; a
  revoked binding is refused once the fold has seen the revocation.
- Depends on 4.1.

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
- Measurement: two nodes on two networks — the owner names the machines (§4) —
  complete HELLO and a home sync round through the relay; whether the direct
  path is taken afterwards is recorded; port release after close is measured as
  in the witness.
- Done when: the crossing is recorded with the relay used, the round-trip, and
  what n0 could see (endpoint ids, IP addresses, timing, volume; nothing else).
- Depends on 4.2 — or, if run before it, on a private endpoint id and a note
  that says so.

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

## Phase 5 — The slice passes and the graph moves

### Step 5.1 — Result document and acceptance review

- `dev-docs/GladeFirstSliceResult.md` on the witness result's shape: what passed,
  measurements, caveats, what is never a caveat, how to rerun; the recorded gaps
  (key rotation; session identity; the checker blind spot). Acceptance review at
  a settled tuple: Code and State (durable state and wire behaviour are both
  touched), plus Surface if 4.5's configuration file is a format people edit.
  Reports filed as `GladeFirstSlice-Review<Axis>.md`.
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
Phase 2's 2.1–2.6 wait on 1.4; 2.3 and 2.7 run beside 2.1 and 2.2. Phase 3's
3.2–3.4 are one lane in sequence. Phase 4's 4.1→4.2, 4.3 (independent of 4.2
with a stated key), 4.4 (after 3.4), 4.5 (after 4.2), 4.6 last.

```
1.1 → 1.2 → 1.3 → 1.4 → 2.1 → 2.2 → 2.5 → 2.6
                         2.3 ─┘  ↑
                         2.4 (after 2.1 and 2.3)
                    2.7 ─────────────────────────→ (feeds 3.4, 4.6)
3.1 → 3.2 → 3.3 → 3.4 → 4.4
 └→ 3.5              4.1 → 4.2 → 4.5 → 4.6
                     4.3 ───────────┘
                                          5.1 → 5.2 → 5.3
```

Each step is one agent on Opus with one brief that names the files it may touch,
the gate it must run, the commit-message shape and no push; the lane owner
verifies before the next step, as in the witness. Reviews happen at 1.2, 2.6 and
5.1 only; interior steps are gated by their tests and the gate.

## 3. What this plan does not do

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

1. Reviewer tier for 1.2, 2.6 and 5.1: Opus by the quota instruction, or the
   strongest tier the process rule asks for at a freeze.
2. R1–R11 (Step 1.4).
3. The two machines for 4.5's crossing (dabeest and which other), and whether
   the owner's own machine may be one of them.
4. Whether `NodeId = sha256(key)` stays through the slice (4.1 keeps it).
5. Whether the assembled composition root is a second binary or a path in
   `glade-node` (3.2; either satisfies "alongside the demo").

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
