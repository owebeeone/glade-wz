# Glade Supplier Model — the authority-side counterpart of a tap

Status: working draft — records GDL-040 + P00-a + substrate security B1–B3
(RULED 2026-07-12); GLP-0006 P0.S1 (§8). Doc-only.

Purpose: fix what a **supplier** is and the one contract it attaches by, so
every GLP-0006 supplier (`glade-chat`, `glade-gwz`, `glade-files`,
`glade-terminal`, …) is built to the same shape. Per-supplier needs (identity,
shapes, storage, grants) are NORMATIVE in
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/SupplierRequirements.md` —
this doc owns the COMMON model, not that matrix.

Normative language per AGENTS.md: MUST / SHOULD / MAY.

## 1. What a supplier is

A **supplier** (GDL-040) is the authority-side module standing behind one or
more declared surfaces and answering for them — the counterpart of a **tap**.
Drip-irrigation register: suppliers FEED the mainline (glade), taps DRAW at the
ends. Registered as **glade suppliers**. It is the server-side denominator of
the declarative discipline: each endpoint is a declared surface with a supplier
behind it, never bespoke per-app server code (the north-star landing).

The unit a supplier stands behind is a **surface** — a `BindingDecl`
`(glade id, shape, authority, domain, zone, retention)` (`GladeDeclSurface.md`).
One supplier MAY back several surfaces of several shapes.

A supplier is **static composed authority**, NOT a dynamic computation — the
line that separates it from a `ServiceDefinition`:

| | Tap | Supplier | `ServiceDefinition` |
| --- | --- | --- | --- |
| role | consumer / draw | authority / feed | instantiated computation |
| lifetime | per mount | standing (composed) | ephemeral (spawn/teardown) |
| count | many | one authority per surface | many instances |
| you… | mount it | COMPOSE it | INSTANTIATE it |

A supplier MAY *serve* a service's exchange glade id, but the supplier is the
standing thing; the service instance is the ephemeral one. A supplier may also
be **demand-instantiated** (ruled 2026-07-12 — this is IN the program, not
deferred): what you compose is its `ServiceDefinition` record; instances spawn
per canonical key and tear down at zero interest (s-diff-service), deduped per
the WD-8 policy (`per-node` default, `global` opt-in — s-svc-shared).
glade-diff is the first such supplier. Base glade owns the instantiation
machinery (record-driven); an app composes suppliers either way.

## 2. Attachment contract (RULED P00-a — wire-attached sessions)

P00-a is RULED (Decisions.md 2026-07-12): a supplier attaches over the WIRE as
an ordinary authority session. It depends on the wire + a client lib only — no
node internals, GDL-038-aligned. In-process / loopback embedding (grazel
running a supplier against its own embedded node) is a COMPOSITION
OPTIMIZATION, never the contract: identical session semantics, a shorter
transport.

A supplier session MUST:

1. **Hello** with its `principal` (§4) and protocol.
2. **Subscribe the declared surface → become THE authority.** The node routes
   by the surface's SHAPE (`server.rs`; `exchange.rs::declared_exchange`):
   - **value / log / window** surfaces — the supplier's node holds the share's
     `ServeClaim` and **serves ops**: it appends `Ops` into the surface's
     streams, which fold + replicate to every subscriber (the audit's `ws.tree`
     leg). "Being the supplier" here IS being the claim-holding authority whose
     op-appends are the surface content.
   - **exchange** surfaces — a Subscribe to a declared exchange glade id
     registers the session as **THE provider** in the node's `providers` map
     (`attach_provider`); the keyed entry map IS the routing table. One provider
     per `(share, glade_id)`. The former last-writer-wins replacement is
     OVERTURNED by **B1** (§8): attach is authenticated + grant-checked, an
     existing attachment is not replaced implicitly, and takeover advances a
     monotonic epoch — the check slot at Subscribe is now B1's, not a stub.
3. **Answer `ExchangeReq` with `corr` preserved.** The node forwards each
   request to the attached provider and relays its `ExchangeRes` back 1:1 by
   correlation id (`exchange.rs::handle_request` / `handle_response`); the
   supplier MUST echo `corr` unchanged.
4. **Answer failure as DATA.** No-provider, timeout, absent-claim — each is an
   `ExchangeRes{ok:false, error}` carrying reason + corr; the session stays
   usable. A supplier MUST NOT hang; absence is a value (§6).
5. **Reattach on drop.** On disconnect the node drops the provider entry
   (`providers.retain`) and the `ServeClaim` lapses at its lease clock (§6). A
   supplier SHOULD reattach (re-Hello, re-Subscribe, renew claim) on link loss;
   consumers see an absent-answer window, never a corrupted stream.

The contract is language-neutral: the TS client + the stage-1 audit harness are
the reference choreography; the rust client (P0.S3) and the supplier kit
(P0.S4) wrap exactly this — no new wire.

## 3. Registration = ordinary records

A supplier's surfaces are DECLARED as ordinary runtime records (GDL-037) — no
node internals, no privileged plane:

- **Statically** via an `<app>.glade` file (grazel's `grazel-app.glade`) LOADED
  as data and registered as `BindingDecl` / `ServiceDefinition` appends + ACL
  seeds compiled to `CapabilityGrant`s (`appdecl.rs`,
  `GladeGrazelAttachNotes.md`); OR
- **At runtime** by the same record appends (a session appending declarations +
  grants it is authorized to append — the hook for dynamic sharing, GDL-037).

The **fold is the only authority**: registration is diff-idempotent
(byte-identical records skip), runtime revocations win by ordinary fold rules,
re-registration can never clobber a later revocation. `declared_exchange` /
`who_serves` are folds over the LOCAL replica, so a declaration is routable the
moment its records replicate — from wherever they were written. A supplier
CONTRIBUTES records like any principal; it never requires a side channel into
the node.

## 4. Identity + attribution (P0.S7 posture)

- A session binds a principal via `Hello.principal` — the wire field exists
  (`Hello { principal: Option<String>, capability: Option<Vec<u8>>, … }`).
  `dir.principals` (named by GDL-038's `glade-sys.glade`) holds principal
  records as ordinary registry appends.
- Suppliers **attribute**: they stamp actions/records with the acting principal
  (chat lines carry it; gwz records who-ran-what). Stage-1 posture (P0.S7):
  identity as DATA — attribution is recorded, nothing is ENFORCED.
- **Enforcement is stage-2's** (P2 / glade-users): `check()` on commons joins,
  per-verb gating, path scoping. A supplier MUST attribute in stage-1 and MUST
  NOT assume a `check()` has run — every seam is stub-allow-all
  (`GladeGrazelAttachNotes.md` §6). The `capability` slot at Hello/Subscribe is
  where enforcement lands with no supplier rewrite.
- **B3 tightens this (§8):** the node delivers an authenticated
  `ProviderCallContext` beside each request, so the principal a supplier
  attributes is node-established from the Hello/session — not a caller-payload
  claim. Stage-1 "nothing ENFORCED" narrows to *no per-verb `check()` yet*:
  session authentication (B5, `GladeAuthzModel §3b`), attach authorization
  (B1, §8), and the authenticated caller context (B3, §8) ARE stage-1. A DTO
  `principal` field MUST NOT be trusted over the context.
- P0.S7 provides principal *records*; user *lifecycle* (enroll / attenuate /
  revoke) is glade-users (P2). The two MUST NOT be smeared
  (`SupplierRequirements.md`).

## 5. The app data seam (grazel's rule, generalized)

A supplier's own storage is INVISIBLE to glade:

- **Shared = declared + served.** Data leaves a supplier only as a declared
  surface it serves ops for; a subscriber sees exactly the surface content,
  never the backing store.
- **Private = undeclared.** Anything not declared is app-owned and unreachable
  through glade. grazel's database ("files for now") is app-owned storage; the
  file↔surface mapping is grazel's alone.
- Consequence: the storage ENGINE cannot leak into glade's model — files-for-now
  or SQLite-later (GDL-036) is a supplier-internal choice. gwz and files are the
  first app-owned-storage consumers (path roots from grazel config —
  foreshadowing AZ-1 path-scoped grants); see `SupplierRequirements.md`.
- This is the producer/consumer seam of the north star on the authority side:
  complexity lives in the declarative producer (the supplier); consumers stay
  thin projections.

## 6. Lifecycle + failure

| Stage | Mechanism |
| --- | --- |
| Attach | Hello(principal) → Subscribe declared surface → authority (ops) / provider (exchange) |
| Serve | value/log/window: append `Ops` (fold + replicate); exchange: answer `ExchangeReq`, corr 1:1 |
| Claim | the supplier's node holds a `ServeClaim {node, share, lease_expiry_ms, epoch}`; `who_serves(share, now)` picks the holder judged at the READER's clock — a lease LAPSES at a later clock with no fold change (lease-at-reader-clock); `epoch` fences takeover (s-takeover) |
| Renew | the claim holder renews its lease before expiry — **F1 / P0.S2** is the gap: no production path mints + renews the live `WorkspaceEntry` + `ServeClaim` yet (tests append them via the registry API) |
| Detach | disconnect drops the provider entry (`providers.retain`); the `ServeClaim` lapses at its lease clock; `who_serves` → next holder or `None` |
| Node restart | records persist under `~/.glade/sys/` and re-fold on boot; the supplier reattaches and renews |
| Supplier crash | provider entry drops / claim lapses; routing answers **absence as data** — `ExchangeRes{ok:false}` (exchange) or `Route::Absent` / no-live-claim (subscribe): a bounded answer, never a hang |

Absence is uniformly a value: a missing supplier, an unrenewed claim, and a
crashed provider all resolve to a bounded error answer with corr intact — never
a stuck session (the phase-E posture, `exchange.rs`).

## 7. Naming

- **Repos say WHAT; terminology says ROLE** (GDL-040). A supplier repo is named
  for its endpoint — `glade-terminal`, `glade-gwz`, `glade-chat`, `glade-files`
  — beside `glade-decl`; NEVER `supplier-*`.
- Each supplier repo's README first line: **"a glade supplier: …"** — the role
  convention lives in the README, not the repo name.
- **grazel is NOT a supplier** — it is the app / node / authority that COMPOSES
  suppliers (repo `grazel-node`, member path `grazel`; P00-b — bare
  `owebeeone/grazel` is squatted by the June razel spike). The README-first-line
  convention is for supplier repos; grazel's README says it is the gryth node.

## 8. Substrate security contracts (RULED B1–B3, stage-1)

Three rulings — `RulingWorksheet.md` §I, RULING A each, ratified 2026-07-12 —
move the security substrate INTO stage-1. They are prerequisites, not stage-2
deferrals, and they tighten §2 (attachment) and §4 (attribution): the stage-1
"attribution only, stub-allow-all" posture now means *no per-verb `check()` yet*
(that stays stage-2, `GladeAuthzModel §6`), NOT "no authenticated identity".
Provider attach, decode, and the caller context are authenticated and authorized
from day one. B4/B5 are the authz-side companions (`GladeAuthzModel §9 AZ-16`
and `§3b`).

**B1 — provider attachment (overturns §2's LWW attach).** Attachment changes who
may act for a glade; possessing an identifier is not authority, and silent
replacement lets a racing or compromised provider seize a live surface. A
provider MUST authenticate and present an exact `provider.attach(share_id,
glade_id)` grant; the glade composition MUST declare the provider class and
surface being attached. An existing attachment MUST NOT be replaced implicitly.
Handoff, detach, or lease takeover MUST be authorized, recorded, and advance a
**monotonic provider epoch**. Every provider call MUST bind the provider
principal, surface, attachment epoch, and glade id; stale epochs MUST fail
closed. This is what makes the **structural composition wall real** — the wall
the spec claimed but LWW attach defeated: the composition, not a first-come
Subscribe, controls which surfaces exist and who backs them. The rule governs
all THREE attach paths of §2 — the `ServeClaim` authority (value/log/window),
the exchange `providers` map, and the **channel→authority** path
(glade-terminal's driver binding, P3.S4) — each an attach B1 authorizes and
epoch-fences.
*Gates:* one shared attach-policy/check utility; the epoch observable in routing
and traces; the same check on local and forwarded paths; attach / detach /
authorized-handoff exposed in the common supplier kit before providers migrate.
*Negative tests:* foreign-share attach; attach without a grant; undeclared
provider class; silent replacement; stale-epoch call after handoff;
detach/reattach spoofing the former provider.

**B2 — decode boundary.** Wire, persisted, and replayed bytes are all hostile
input; a panic or partially-folded malformed record turns corruption into a
node-wide availability or integrity failure (the restart-stable decode DoS).
Every decode entry point MUST be fallible. Typed system records — `dir.bindings`,
service definitions — MUST be schema-validated AND authorized BEFORE persistence
or fold; invalid records MUST NOT change authoritative state. A decoder SHOULD
quarantine bounded evidence and MUST return a typed, bounded protocol/correlation
error, never panic. Replay MUST continue past quarantined records where the log
format permits safe resynchronization. This hardens §3's "the fold is the only
authority" into *validate-and-authorize before fold*, and extends §6's
bounded-answer discipline to malformed input.
*Gates:* make the taut runtime fail closed; remove infallible public decode
paths; teach exchange + replay to report quarantine outcomes; run one
malformed-input corpus against the Rust and TS codecs. *Negative tests:*
arbitrary + truncated bytes; malformed system records; restart with a poisoned
persisted record; malformed-then-valid traffic in one session; fuzz/property
tests proving no panic and no partial fold.

**B3 — `ProviderCallContext` (the stage-1 form of §4's attribution).** DTO
fields are caller-controlled data; a supplier needs a transport-independent
statement of who called and what proof accompanied it. The node MUST construct a
`ProviderCallContext` from the authenticated Hello/session, carrying requester
principal, certified device, capability/grant evidence, session assurance,
correlation id, and forwarding provenance. Forwarders MUST preserve it; suppliers
MUST receive it **beside, not inside**, the request DTO. A DTO `principal` field
MUST NOT override the context. Any effectful follow-on MUST use the same
authenticated principal unless an explicit delegation is verified. This is the
"who called" every effect supplier consumes — the node-established identity §4
attributes, delivered per call rather than re-asserted in the payload.
*Gates:* publish matching Rust + TS context types, constructors, and
authorization helpers; update the supplier harness; prove local/forwarded parity
before changing any effectful supplier. *Negative tests:* forged DTO principal;
request/context substitution; context dropped during forwarding; vouched identity
used beyond its grant; provider code attempting to replace the requester.

## Cross-references

`GladeDeclSurface.md` (surface / `BindingDecl` / `<app>.glade`) ·
`DecisionLog.md` GDL-037/038/040 · `glade/dev-docs/GladeGrazelAttachNotes.md`
(the R4 provider mechanics = this contract's implementation) ·
`GladeE2EStage1Audit.md` (F1 live-claim gap) ·
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/SupplierRequirements.md`
(the per-supplier needs matrix — not duplicated here) ·
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md` §I (the
B1–B3 source rulings) · `GladeAuthzModel.md` §3b / §9 (AZ-16) / §11 (the
B4 / B5 / D3 / D5 / H-R3 companions).
