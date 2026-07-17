# GLP-0006 ruling worksheet — one decision per row

Consolidates every open decision blocking spec→plan conversion: both adversarial
reviews (`dev-docs/glade/suppliers/SupplierSpecReview-F5.md`,
`SupplierSpecReview-56.md`, `SupplierSpecReview-56-2.md`), the glade-gwz v2
respec dials, and the accumulated per-spec `§10` opens + earlier program
rulings (R1–R3, C-confirms, the P4 gate). Overlapping findings are MERGED
(tag ⊆ gwz-grain, SWMR ⊆ P4-gate, R1 ⊆ create-durability, selection appears once).

**How to use:** fill the **Ruling** cell per row (or write inline under it). Lean
notation: `both→X` = both reviewers + F5 agree on X · `56→X, F5→Y` = split ·
`F5→X` / `56→X` = only that pass raised it. F5 = this session (verified each
spec + both gwz respecs on disk); 56 = the independent reviewer.

**Suggested order** (dependency, not priority): **§A actions** (no ruling
needed) → **§B substrate prerequisites** (they move items out of "stage-2" and
gate everything) → **§C gwz family** → **§D cross-cutting authz** → **§E
per-supplier** → **§F retention** → **§G stale premises** → **§H prior opens**.

**Load-bearing few** (rule these first — each unblocks many): B3 (requester
context), B1 (provider auth), C-gwz-1 (grain), P4-gate (H-P4), E-share-1
(direct membership), F-GAP10 (retention).

---

## §A — Actions, not decisions (tracked, no ruling)

| ID | Item | State |
| --- | --- | --- |
| A1 | **glial logDelta out-of-order bug** (SR56-2-21, verified) — positional `slice(emittedLen)` over a re-sorted list dups/drops records | spawned as a background task; fix = identity-set diff + out-of-order regression test. Independent of specs. |
| A2 | **taut pin reproducibility** (SR56-2-01 second half) — version ruled **v0.8.1**; but regen floats to latest PyPI, no in-artifact stamp/CI gate | pre-plan: stamp version into generated artifacts + regen corpus. Not a design choice, a hygiene gate. |

---

## §B — Substrate prerequisites (the reframe: these are STAGE-1, not stage-2)

The model makes attribution a **stage-1 must**, but attribution needs machinery
the specs defer to stage-2 — so "stage-1 allow-all" is neither safe nor honestly
exercisable until these land. Ruling these first re-scopes every supplier's
stage split.

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| B1 | **Provider-attach auth** (SR56-2-03) — any Subscribe to a declared exchange becomes THE provider (LWW, no auth) | (a) authenticate+authorize attach, reject replacement, AND composition controls which surfaces are declared · (b) leave open, drop the "structural composition wall" claim | both→a | gwz composition wall; every exchange supplier |
| B2 | **Fail-closed decode** (SR56-2-02) — `cbor::decode`/`from_cbor` panic on malformed persisted records = restart-stable DoS | (a) validate typed system records before persist + quarantine invalid + regen wire on fail-closed codec · (b) withdraw "failure as data" until later | both→a | node stability; "failure as data" honesty |
| B3 | **Requester context** (SR56-04, SR56-2-09) — `ExchangeReq` carries no requester; built supplier trusts a caller-payload `principal` | (a) node-authenticated Hello-bound provider-call context, delivered to providers, as a STAGE-1 prerequisite · (b) weaken stage-1 attribution to none | both→a | ALL attribution; gwz; terminal; every effect supplier |
| B4 | **Identity-bound `self:` keys** (SR56-2-05/20) — node accepts any subscribe key; a member subscribes `self:alice` and gets Alice's private zone | (a) derive/validate `self:` from the authenticated session at ingress, reject mismatch · (b) stop calling routing-only keys "private" | both→a | editing cursors; terminal private logs; AZ-16 "privacy is a key" phrasing |
| B5 | **Stage-1 identity trust level** (SR56-2-14) — `Op` has no signature, store accepts absent `prev`; glade-users §2 claims signed primitives exist | (a) mark authenticated sessions + per-op signatures as new load-bearing work; gate identity governance on it · (b) narrow stage-1 to explicitly-untrusted demo data, defer governance | both→a | glade-users names/invites; the "blockchain-shaped" name claim |

*Note B3+B4+B5 are one arc: real identity at the session + signed ops + delivered
requester context. Ruling them together (build the identity substrate first) is
the coherent move; ruling them "stage-2" leaves users/share/terminal unbuildable
as specced.*

---

## §C — The gwz family (v2 dials + review fixes)

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| C-gwz-1 | **Grain** | (a) 21 members split-by-interface · (b) strict 1:1 = 24 · (c) coarser: merge the 4 read members into one "inspect" | F5→a | declaration gen, grant units, provider count, UI |
| C-gwz-2 | **Tag partition + least-privilege** (SR56-03/2-10/2-11) — one canonical `TagRequest`; fetch≠push | (a) disjoint generated `TagLocalOp`/`TagRemoteOp` DTOs AND split tag-fetch from tag-push (3 tag surfaces) · (b) 2 surfaces local/remote, per-op check inside remote · (c) 1 tag surface, per-op authz | 56→a(split fetch/push), F5→split local/remote | egress wall integrity |
| C-gwz-3 | **Path-free DTO projection** (SR56-02) — canonical requests carry host paths | (a) ratify per-member path-free DTO projected server-side; `repository_path` constrained ws-relative + selection-rooted; arbitrary-host adds defer to AZ-1 · (b) retract, keep canonical Request on the wire | both→a | every gwz member's wire contract |
| C-gwz-4 | **Result surface** (SR56-14/2-32) — `OperationEvent` has no final result | (a) final `OperationResult` as the **closing replicated log record** (survives provider restart) · (b) a paired `.result(operation_id)` exchange (dies with the provider) | 56→a, F5→b(lean) — **56's durability point wins** | accepted-op result retrievability |
| C-gwz-5 | **Events-log visibility** (F5-11/12, SR56-2-12) — v2 invented a "grant-keyed zone" | (a) events logs are **commons**; egress-event visibility = ordinary stage-2 **read grant on that log surface** · (b) private per-caller | F5→a (v2's "grant-keyed zone" is not expressible) | gwz surface zones; push/pull progress exposure |
| C-gwz-6 | **Packaging + dispatch** | packaging: (a) one repo + one binary attaching N sessions · (b) N repos/processes. dispatch: (c) gwz-core in-process library · (d) protocol subprocess | F5→a + (c/d behind the data seam, either) | repo/build layout |
| C-gwz-7 | **Public ceremony collision** (SR56-2-06/07) — create/repo mutations have two public owners (workspaces `ws.ops` + gwz members) | (a) ONE public `workspace.create`/`ws.ops`, gwz members are internal materializer legs · (b) gwz members are the public typed ceremony, workspaces consumes their result · (c) retire `ws.ops` | 56→a (create), split on repo-ops | glade-workspaces §2.2 vs gwz §5 |
| C-gwz-8 | **Create durability** (SR56-2-31, F5-13, subsumes R1) — "idempotent replay, no orphan" has no basis | (a) durable intent→materialized→registered→claimed state machine w/ replay owner + `init` partial rule; creation events on HOME share via reserved base-glade bindings · (b) compensating rollback | both→a | R1 materializer; create/init/clone members |

---

## §D — Cross-cutting authz + correctness (spec holes)

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| D1 | **diff leak-guard formula** (SR56-2-24) — F5 accepted a backwards relation | restate per-principal `can_read(left) && can_read(right)`; reader-set `Readers(diff) ⊆ Readers(left) ∩ Readers(right)`; enforced per serving hop | both→adopt (F5 concedes) | glade-diff §5; candidate INV-7 |
| D2 | **diff derived-binding home** (diff CONTENTIOUS #1, GDL-026) | (a) structural derivation — `svc` reserved namespace, never a stored share; only the definition (durable) + leased instance claim (ephemeral) · (b) register derived bindings in the directory | F5→a | glade-diff §4 |
| D3 | **diff private-vs-dedup** (SR56-2-23) — can't be per-viewer-private AND globally pair-deduped | (a) two-level identity: compute-instance key (pair) ≠ per-viewer delivery binding, each fan-out authorized · (b) commons derived surface + source-derived read checks | 56→a, F5-diff assumed single-level | glade-diff §3/§6 |
| D4 | **diff teardown stale replica** (SR56-2-25) — value ops replicate; teardown can't erase them | (a) generation-bound `pending\|ready\|stale\|absent\|denied\|error` state + tombstone/revalidate-before-replay · (b) genuinely transient non-store path | 56→a | glade-diff §1/§4; teardown policy |
| D5 | **service-def exec safety** (SR56-2-26) — `program` is an allow-all content-addressed append that spawns code | (a) versioned demand-service record + migration AND execute only composition-pinned programs until signed authority + sandbox/resource policy · (b) trusted static defs only until the contract exists | both→gate before spawn | glade-diff stage-1; GDL-016 |
| D6 | **blob shape + authz** (F5-4, SR56-2-17) — bare-hash fetch = bearer token; `window/exchange` isn't one shape | (a) ONE declared fetch exchange bound to an authorized source reference/path (reference-derived auth) + typed failure · (b) attenuated object capability · (c) defer blob reads | both→a | glade-files §3/§6 |
| D7 | **ws.tree subtree redaction** (SR56-2-18) — one value keyed `{root}` can't hide `/secret` from a `/src` grant | (a) subtree-keyed surfaces the same path policy checks · (b) authority-filtered projection + revision | 56→a | glade-files §4; AZ-1 |
| D8 | **window shape + revision identity** (F5-5, SR56-2-34) — no provider request path, no revision, no reassembler owner | (a) typed window contract `{workspace,path,revision}` + generation-bound reassembler (assign its build owner) · (b) descope mutable-file windows from v1 (logs-only), reassembler waits for an owner | F5→b(v1), 56→a(full) | glade-files §2; glade-diff sources |
| D9 | **terminal reattach/handoff envelopes** (SR56-2-33) — offset-dedup promised, only input `TermIn` exists | (a) define `TermOut{offset,generation}` + epoch bound into channel/`TermIn`, atomic old-channel close · (b) drop lossless reattach/handoff from v1 | 56→a | glade-terminal §4 done-criterion |
| D10 | **terminal scrollback zone** (F5-1) — can't be private-zone AND stage-2 watchable | (a) commons keyed by `session_id`, local-only via no-advertise, stage-2 `term.read` grant · (b) private + supplier republish to a watch surface | F5→a | glade-terminal §2/§5 |
| D11 | **knock append capability** (SR56-09, SR56-2-16) — "read implies append" is a confused deputy; `requester-fp` forgeable | (a) authenticated directed request exchange + offline queue · (b) narrow `share.request.append` entitlement + cryptographic requester proof | 56→a, F5→narrow-cap | glade-share §5 |
| D12 | **doc.save delegation** (F5-2, SR56-2-22) — editing writes the tree bypassing files' AZ-1 | (a) `doc.save` delegates to authenticated `files.write replace` + expected base revision + workspace lock · (b) editing owns its own write w/ duplicated enforcement | both→a | glade-editing §4; glade-files seam |
| D13 | **files/editing truth seam** (F5-2) — one file, two truths mid-session | (a) `ws.files` at-rest-only + a "being edited by X" marker record · (b) files serves the live fold when a session exists | F5→a | glade-files §2.5; viewers/gwz/diff staleness |
| D14 | **workspace-relative path algorithm** (SR56-2-36) — no shared containment/symlink/TOCTOU rule | (a) ONE typed root-relative path algo + symlink/safe-open policy shared by gwz/files/terminal · (b) server-issued opaque file/repo handles | 56→a | gwz `repository_path`, files, terminal |

---

## §E — Per-supplier dials

**glade-chat**

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| E-chat-1 | **Group-as-share vs group-as-key** (F5-3, SR56-28) — AZ-16 grants a share; membership-snapshot needs a per-group fold | (a) real group = its own share; keyed-commons only as stage-1 migration source · (b) per-`(domain,commons,key)` grant scope (overturns AZ-16) | both→a (AZ-16 forces it) | chat §3; keyed-commons survival |
| E-chat-2 | **chat.decl reality** (SR56-10/2-15) — runtime declaration never consumed by the node | (a) append a real typed home `dir.bindings` `BindingDecl` + spawned-node test · (b) relabel `chat.decl` non-authoritative, keep groups static | both→a | chat §1/§2 dynamic groups |
| E-chat-3 | **Codec unification** — taut `ChatLine` everywhere | forward-cut (gryth-ui is ephemeral); make it a hard P2 gate before mixing clients | F5→hard gate | gryth-ui interop |
| E-chat-4 | **Who creates groups** | (a) any onboarded principal, self-serve owner · (b) grant-gated to domain/workspace owner | F5→a | chat §3.3 |
| E-chat-5 | **Message edit/delete** | (a) tombstone/supersede (the flip-instance idiom) · (b) append-only forever | open | chat §10 |

**glade-users**

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| E-users-1 | **Browser-first onboarding identity** (SR56-2-07/37) — device key becomes canonical before root certifies | (a) root-anchored canonical id + device certs + signed merge ceremony before v1 · (b) first browser key IS the enduring root | both→a | users §1/§3; two-device + delayed-root traces |
| E-users-2 | **users.invites shape** (SR56-06) — declared exchange but §3.1 persists a record | split `users.invite.records` (log) from `users.invites` (exchange) | 56→split | users §3.1/§4 |
| E-users-3 | **Names registry v1?** (users §8) | (a) v1 · (b) later (petnames + fp-suffix suffice for correctness) | F5→b | users §2 handles |

**glade-share**

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| E-share-1 | **Direct membership ceremony** (SR56-08, SR56-2-30) — the normative "share this workspace with that principal" flow is MISSING; spec only does links | (a) glade-share owns direct `share.create`/invite/revoke exchanges over ordinary grant records; links layer on top · (b) assign direct-share to workspaces/users, split link-sharing out + update the outline | both→a | the normative outline; chat's create-a-share dependency |
| E-share-2 | **Link private-ref rebinding** (SR56-2-19) — capturing `self:alice` refs / emailed links have no valid private/account treatment | (a) restrict v1 capture to portable commons/explicit values + readable glade carriers; reject unsupported refs · (b) define private-ref rebinding + external bootstrap rendezvous | 56→a(restrict v1) | share §1/§5 |
| E-share-3 | **Knock stage-1 exercisability** (SR56-20) — allow-all means mounts never deny, so no knock fires | (a) move knock user-testability to the gated stage; unit-test the records earlier · (b) test-only denied result under allow-all | 56→a | share §7 stage split honesty |

**glade-workspaces**

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| E-ws-1 | **Selection authority** (SR56-16, SR56-2-08) — grip-local `ws.selection` vs addressed share vs terminal `workspace-ref`; three sources of truth | addressed **stable workspace/share ID** is the sole authority input, map ID→root, reject redundant mismatches; classify the grip selector separately from the authority supplier | both→addressed-share-ID | gwz/files/terminal targeting; name-dup ambiguity |
| E-ws-2 | **Eligible-host fold** (SR56-17, SR56-2-29) — `WorkspaceEntry.eligible_hosts` LWW loses concurrent clones | per-(workspace,node) eligibility records / OR-set with explicit removal, vs serialize through one writer | both→OR-set | workspaces §2.2 clone convergence |

---

## §F — Retention (consolidate, rule ONCE)

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| F-GAP10 | **GAP-10 retention** — four consumers name it independently: files (window caches + blobs), chat (unbounded log), terminal (scrollback), diff (`retained` teardown) | ONE TTL/size/pressure eviction policy — the `retention` field gains an eviction knob — ruled **before P3**; OR make stage-1 bytes explicitly ephemeral+bounded | both→one policy before P3 | P3 files/terminal, chat, diff |

---

## §G — Stale premises

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| G-razel | **glade-razel deferral** (SR56-27/2-27) — protocol EXISTS (razel-wire-api 0.1.0, 10 methods); the stub's "no protocol" premise is false | (a) keep deferred but change the trigger to a pinned RELEASE + correct the premise · (b) expand now against 0.1.0, marking unsupported verbs honestly | 56→b, F5→a (per the wait-for-release rule) | glade-razel scope |

---

## §H — Prior program rulings still open (pre-review backlog)

| ID | Decision | Options | Lean | Blocks |
| --- | --- | --- | --- | --- |
| H-P4 | **P4 gate: editing shape** (subsumes SR56-2-35 SWMR fence) — does "edit live, neither losing our cursor" need simultaneous keystrokes? | (a) **swmr v1** as write-ownership policy over `log`; REQUIRES a designed write fence (EditClaim schema + epoch on every body op + stale-writer reject + accepted-head handoff — not "cheap") · (b) crdt v1 (text-crdt Shape) | F5→a, 56→"don't call swmr cheap until the fence is designed" | glade-editing whole build |
| H-R2 | **glade-workspaces = distributed role** (not one session) — client taps + host ceremonies | confirm | F5→confirm | workspaces plan shape |
| H-R3 | **Accept/mint record authorship** — client-appends vs supplier-appends the ceremony records | rule | open | users/workspaces ceremonies |
| H-C1 | `dir.principals` replication pin (how far principals replicate) | confirm | F5→home-share | users |
| H-C2 | `ws.selection` grip-only v1 (folded into E-ws-1) | see E-ws-1 | — | — |
| H-C3 | Workspace display-name dups tolerated (fingerprint/ID-suffixed) | confirm (ties to E-ws-1 ID map) | F5→tolerate + ID map | workspaces §7 |
| H-C4 | Clone manifest travels with glade records | confirm | F5→confirm | workspaces §7 |
| H-C5 | Clone = eligible+warm, not seize | confirm | F5→confirm | workspaces §2.2 |
| H-C6 | Per-verb gating list (now per-member surface grants, gwz split) | subsumed by C-gwz-1 + gating model | F5→per-surface | gwz stage-2 |

---

## Coverage note

~40 decisions. Not all are equal: **§B (5) + C-gwz-1 + C-gwz-8 + E-share-1 +
H-P4 + F-GAP10** are the ~10 that gate the most and should be ruled before any
plan-conversion agent runs. §D/§E/§G refine specs that are otherwise plan-ready
once §B lands. §A needs no ruling. Every row cites its finding id so a
disposition can be written straight back into `Decisions.md` and the relevant
spec `§10`.

---

## Recorded rulings and recommendation packets

`RULING` below is final for plan conversion. `RECOMMENDATION` is the proposed
v1 answer and remains pending until accepted. Requirements use normative terms
deliberately.

### I. Security substrate — final rulings (B1–B5)

#### B1 — Provider attachment and replacement: RULING A

**Rationale.** Provider attachment changes who may act for a glade. Discovery
or possession of an identifier is not authority, and silent replacement would
let a racing or compromised provider seize a live surface.

**Normative requirements.** A provider MUST authenticate and present an exact
`provider.attach(share_id, glade_id)` grant. The glade composition MUST declare
the provider class and surface being attached. An existing attachment MUST NOT
be replaced implicitly. Handoff, detach, or lease takeover MUST be authorized,
recorded, and advance a monotonic provider epoch. Every provider call MUST bind
the provider principal, surface, attachment epoch, and glade identity. Stale
epochs MUST fail closed.

**Implementation gates.** Provide one shared attachment-policy/check utility;
make the epoch observable in routing and traces; use the same check in local and
forwarded paths; and expose attach, detach, and authorized handoff in the common
supplier kit before migrating providers.

**Required negative tests.** Foreign-share attach; attach without a grant;
undeclared provider class; silent replacement; stale-epoch call after handoff;
and detach/reattach used to spoof the former provider.

#### B2 — Decode boundary: RULING A

**Rationale.** Wire bytes, persisted bytes, and replayed bytes are all hostile
inputs. A panic or partially folded malformed record turns corruption into a
node-wide availability or integrity failure.

**Normative requirements.** Every decode entry point MUST be fallible. Typed
system records, including `dir.bindings` and service definitions, MUST be
schema-validated and authorized before persistence or fold. Invalid records
MUST NOT change authoritative state. Implementations SHOULD quarantine bounded
evidence and MUST return a typed, bounded protocol/correlation error rather
than panic. Replay MUST continue past quarantined records when the surrounding
log format permits safe resynchronization.

**Implementation gates.** Make the taut runtime fail closed; remove infallible
public decode paths; teach exchange and replay to report quarantine outcomes;
and run a common malformed-input corpus against Rust and TypeScript codecs.

**Required negative tests.** Arbitrary and truncated bytes; malformed system
records; restart with a poisoned persisted record; malformed then valid traffic
in one session; and fuzz/property tests proving no panic and no partial fold.

#### B3 — Authenticated caller context: RULING A

**Rationale.** DTO fields are caller-controlled data. Suppliers need a
transport-independent statement of who made a call and what proof accompanied
it.

**Normative requirements.** The node MUST construct a `ProviderCallContext`
from the authenticated Hello/session. It MUST carry requester principal,
certified device, capability/grant evidence, session assurance, correlation
identity, and forwarding provenance. Forwarders MUST preserve that context;
suppliers MUST receive it beside, not inside, the request DTO. A DTO principal
field MUST NOT override the context. Any effectful follow-on operation MUST use
the same authenticated principal unless an explicit delegation is verified.

**Implementation gates.** Publish matching Rust and TypeScript context types,
constructors, and authorization helpers; update the supplier harness; and prove
local/forwarded parity before changing effectful suppliers.

**Required negative tests.** Forged DTO principal; request/context
substitution; context dropped during forwarding; vouched identity used beyond
its grant; and provider code attempting to replace the requester.

#### B4 — Symbolic self zones: RULING A

**Rationale.** A literal `self:<name>` trusts caller spelling and permits
cross-user reads. `self` is an authorization expression, not a routable user
name.

**Normative requirements.** `self` MUST be symbolic in the request. The node
MUST derive its concrete zone key from the B3 principal and MUST reject a
caller-supplied literal identity that does not match. The derivation/check MUST
apply to subscribe, append, replay, and forwarding. Membership remains a
separate entitlement and MUST satisfy AZ16. Revocation MUST cut subsequent
resolution and forwarding.

**Implementation gates.** Ship one canonical zone-key/`self` resolver in the
common security utilities and replace supplier-local parsing.

**Required negative tests.** Bob requesting `self:alice`; anonymous `self`;
forged forwarded identity; alternate encodings/aliases; and access after
membership revocation.

#### B5 — Device proof and signed security operations: RULING A

**Rationale.** A claimed device key is not proof of possession, and unsigned
grant/revoke records cannot safely govern authority.

**Normative requirements.** Session establishment MUST prove possession of a
device key certified by the account root. Security-sensitive operations MUST
be signed by an authorized certified device, include the strict predecessor
required by their log, and be verified before persistence or fold. Revoked or
unknown devices MUST fail closed. Legacy unsigned records MAY be retained as
unverified history but MUST NOT create, extend, or revoke governance authority.

**Implementation gates.** Define the signed-operation envelope and validation
corpus once; implement it in Rust and TypeScript; enforce it in store replay and
users/share suppliers; and add conformance traces for certification,
revocation, and predecessor validation.

**Required negative tests.** Forged Hello proof; wrong signature; revoked or
uncertified device; missing/wrong predecessor; replayed signed operation;
unsigned legacy record winning governance; and vouched identity attempting a
strong security action.

### II. GWZ family packet — recommendations (C-gwz-1–8, E-ws-1)

The governing rule is: a merge is legal only when operations have the same
**UI contract AND capability class**. A difference on either axis requires a
separate interface. Do not split merely because implementations differ, and do
not merge read and mutation authority merely because one command groups them.

| ID | Recommended final answer | Worksheet lean requiring attention |
|---|---|---|
| C-gwz-1 | Generate **25 current GWZ supplier interfaces**: 24 canonical input methods − 4 accepted twin-merge reductions + 3 for replacing one `tag` method with four surfaces + 1 for splitting `stash` read/mutate + 1 for splitting `branch` read/mutate. The generator MUST recalculate this count from the IR and fail on drift. | **Do not accept 21, 22, or 23.** Each leaves at least one read/mutation capability exception. |
| C-gwz-2 | Use four interfaces: `tag-list` (read), `tag-mutate` (create/delete), `tag-fetch` (remote read), and `tag-push` (remote write). DTO operations MUST be closed enums. | **Do not accept option A unchanged.** It preserves only two capability classes. |
| C-gwz-3 | Keep ordinary DTOs path-free. Local import MAY use a selected-root-relative path or a server-issued, scoped, expiring `RepoImportHandle`; arbitrary caller-supplied host paths are forbidden. | **Do not accept the lean automatically.** AZ1 alone cannot make arbitrary host paths safe. |
| C-gwz-4 | Append a closing, replicated `OperationResult` terminal record to the operation log. A result query MAY be a derived convenience view, not the authority. | Accept the append-result lean. |
| C-gwz-5 | Use commons logs. Local reads require membership; egress events, results, and output require exact surface grants. Grant identifiers MUST NOT become zone keys. | Accept the commons-zone lean with explicit egress checks. |
| C-gwz-6 | v1 uses one repo per workspace, one process with N sessions, and in-process `gwz-core`. A hard process boundary remains an optional deployment profile. | **Do not accept one listed option wholesale.** The coherent v1 answer is A+C. |
| C-gwz-7 | `glade-workspaces` owns public `workspace.create`; GWZ owns the internal create/init/clone materializer. Existing-workspace typed GWZ repo members remain public. Retire the `ws.ops` façade. | **Do not accept the whole-family ownership lean.** Ownership splits at workspace lifecycle vs repo operation. |
| C-gwz-8 | Persist a durable create state machine: `intent → materialized → registered → claimed → complete`. Recovery MUST be forward-only and idempotent; HOME logs remain authoritative. | Accept the durable workflow lean. |
| E-ws-1 | A stable share/workspace ID is the sole routing and authorization identity. The node resolves ID→root; an asserted root mismatch MUST fail. Display names MUST never route. | Accept the ID-authority lean. |

The two additional interfaces required beyond the former 23 ruling are:

- `stash-list` (read) and `stash-mutate` (push/apply/pop/drop); and
- `branch-list` (read) and `branch-mutate` (create/delete/merge).

This is not a new product split: the canonical stash and branch DTOs already
contain those arms. It is the least-privilege projection of those DTOs under the
same rule that produced `tag-list` and `tag-mutate`. Keeping 23 would require an
explicit stash/branch exception and would let a list-only UI hold mutation
authority. The ruling makes no such exception.

**Packet gates.** Generated DTOs, `.glade` declarations, manifests, provider
registration, and test expectations MUST arise from the same IR. Migration
MUST replace `gwz.ops` traces and add invariant authorization tests for every
surface. Create-workflow tests MUST cover every crash window and repeated
recovery.

### III. Diff/service — final rulings (D1–D5)

#### D1 — Service identity and reuse

`svc` is a structural, versioned `DemandServiceDefinition`, not a provider
label. Its compute key MUST include the definition/program digest, sandbox
policy version, and ordered source identities/revisions. Viewer identity MUST
NOT be part of the compute key. Delivery identity MUST be distinct and scoped
to each authorized viewer. Reuse MUST never imply authority reuse.

#### D2 — Instance location: structural derivation

The instance key MUST be derived structurally from D1 inputs. Execution is
per-node by default. A global instance MAY exist only behind a leased,
monotonic-epoch claim; a race loser MUST tear down and MUST NOT publish output.
Zero-viewer instances SHOULD enter a bounded grace period before reclamation.

#### D3 — Authorization contract

Every subscribe, replay, cache delivery, and forwarded delivery MUST establish
`can_read(left) && can_read(right)` for the requesting B3 principal. The service
MUST receive only exact, read-only source capabilities. Authorization MUST be
re-evaluated when grants or membership change; an already computed artifact
MUST NOT bypass a later denial. INV7 tests MUST cover users authorized for only
the left source, only the right source, neither source, and both sources.

#### D4 — Generation contract

Generation state MUST be one of `pending | ready | stale | absent | denied |
error` and MUST identify service-definition and source revisions. Cached
`ready` output MUST be revalidated before delivery. Grant lapse, source change,
worker loss, and deterministic program failure MUST be distinguishable typed
outcomes. A stale generation MUST NOT be relabelled current.

#### D5 — Sandbox contract

The sandbox definition MUST be a new versioned record, distinct from the legacy
shape, and composition-pinned until changed by signed governance. Defining a
service and authorizing its execution MUST be separate capabilities. Execution
MUST have read-only declared inputs; no ambient filesystem, network, clock,
randomness, environment, or child-process access; bounded CPU, memory, output,
and wall time; and typed timeout/resource/policy errors.

**Implementation gates and negative tests.** Publish service-definition,
instance, delivery, generation, and sandbox types before implementing workers.
Test structural-key stability and collision separation; lease races and stale
epochs; refcount teardown; all INV7 combinations at live and cached paths;
revocation between compute and delivery; stale-source output; undeclared input,
network/filesystem/process attempts; and every resource limit.

### IV. Files and Storage — v1 recommendations

#### D6 — Blob fetch

`ws.blob.fetch` SHOULD be an exchange request naming a workspace binding or
canonical path plus revision and `BlobRef`. The response SHOULD use a bounded
carrier/stream channel. The provider MUST re-resolve the workspace and perform
authorization at delivery time. A content hash proves integrity, not authority.

#### D7 — Tree listing

`ws.tree` SHOULD be keyed by canonical workspace-relative directory path and
return one bounded listing with an explicit revision and continuation token.
Names MUST NOT serve as workspace identity.

#### D8 — Window loading

v1 SHOULD implement the full mutable typed window, not a logs-only substitute.
Window identity is `{workspace_id, path, revision}`; byte/line range is an
interest over that identity, not a new binding. Base glade owns
request/routing/generation, glial owns reassembly, and `glade-files` owns the
authoritative snapshot. A consumer MUST NOT observe mixed generations.

#### D12 — Save

`doc.save` MUST call authenticated `files.write/replace` with the expected base
revision and any required lock/lease. Conflict MUST be explicit; last-writer
wins MUST NOT silently overwrite a changed base.

#### D13 — Authoritative storage

Workspace files are the at-rest truth. `doc.editing` marks active collaborative
state. Files, GWZ, and diff consumers read the last successfully saved revision
unless an API explicitly requests the live editing generation.

#### D14 — Path contract

All filesystem-facing suppliers MUST use one `RootRelativePath` type and safe
open utility. It MUST normalize separators, reject absolute paths and parent
traversal, constrain symlink resolution to the selected root, and avoid
check/use races. External imports MUST use scoped handles from C-gwz-3.

#### F-GAP10 — Retention and pressure policy

Every retained class MUST declare `max_bytes`, `max_age`, pressure priority,
pin conditions, and terminal-state behavior. Eviction MUST NOT silently rewrite
authoritative history.

Recommended v1 defaults:

| Class | Default |
|---|---|
| Blob/window cache | 1 GiB/workspace and 4 GiB/node, LRU after 7 days; active windows and referenced blobs pinned |
| Diff/service output | 64 MiB/instance; 5-minute zero-viewer grace; unpinned ready generations expire after 30 minutes |
| Terminal scrollback | 16 MiB or 100,000 lines/session; retain 7 days after close; live/detached sessions pinned; truncation records the new first offset |
| Chat | Authoritative records are never silent-LRU evicted; segment/anchor before pruning. Local hot cache: 256 MiB or 90 days/group; older history remains explicitly fetchable |
| Pressure trigger/order | Trigger below 10% or 5 GiB free, whichever is safer. Evict compute output, then blobs/windows, then closed terminal, then anchored chat cache. Never evict pinned/current authorization state |

Required tests SHOULD cover threshold boundaries, simultaneous byte/age limits,
pinning, crash during eviction, missing authoritative backing, terminal gap
reporting, chat anchor recovery, and recovery after pressure clears.

### V. Terminal and Editing — final rulings

#### D9 — Terminal stream

Input MUST carry `driver_epoch`; output MUST carry `{generation, offset, bytes}`.
All watchers share one live-output/scrollback space. Driver handoff MUST be
atomic and advance the epoch. Replay MUST define a cursor and a live cutover
that neither duplicates nor omits bytes. Gaps, truncation, closed process, and
authorization lapse MUST be explicit outcomes.

#### D10 — Terminal authority

The commons binding MUST be keyed by an unguessable `session_id`. Local sessions
MUST neither forward nor advertise. Watchers require `term.read`; the driver
requires `term.write` and `shell.exec`, plus any configured co-sign rule.
Possession of the session ID alone grants nothing.

#### H-P4 — Text CRDT for v1

v1 MUST use a text CRDT; single-writer/multi-reader is not an allowed fallback.
The protocol MUST define first-class identities and operations rather than an
opaque patch blob:

- element IDs are `{actor_id, counter}` with causal dependencies;
- insert names an anchor; delete names element IDs and creates tombstones;
- duplicates are idempotent and concurrent siblings have deterministic order;
- cursors/selections use element IDs plus affinity and are private under B4;
- deltas remain identity-based and tolerate out-of-order delivery;
- open records a saved base revision; save uses the D12 compare-and-replace
  contract; and
- compaction requires a causal checkpoint acknowledged past the compacted
  frontier.

Required tests MUST cover concurrent insert/delete, duplicate and reversed
delivery, offline merge/heal, cursor affinity, compaction/reopen, conflicting
save, and isolation between editing and terminal stream contracts.

### VI. People and Sharing — forced/consensus rulings

| ID | Final ruling |
|---|---|
| D11 | An access request is a directed, authenticated request to the relevant authority with a durable offline queue. B3 supplies identity. Read permission does not imply append permission. |
| E-chat-1 | Each chat group owns its own share. |
| E-chat-2 | HOME `dir.bindings` is typed and authorization-checked. `chat.decl` JSON is non-authoritative. Tests MUST cover a spawned node, not only an in-memory provider. |
| E-chat-3 | Taut `ChatLine` is the sole chat payload. The legacy forward-cut is a hard migration gate. |
| E-chat-4 | Any authenticated principal MAY create a group, subject to the immutable v1 `ChatQuotaSettingsV1` admission record below. |
| E-chat-5 | Edit/delete uses a signed tombstone or superseding record. Original records remain immutable and auditable. |
| E-users-1 | Account identity is the root-key fingerprint. A browser/device identity MUST be certified; account merge/root transition MUST land before governance depends on it. |
| E-users-2 | Invites use durable log records plus exchange delivery. |
| E-users-3 | Defer the canonical naming/alias registry. v1 uses fingerprint identity plus non-authoritative local display names and fingerprint suffixes. |
| E-share-1 | The share family owns `share.create`, `share.invite`, `share.grant`, `share.revoke`, and `share.status`. |
| E-share-2 | v1 invitations carry only portable commons grants and inline share/glade IDs. Private/account zones are rejected. |
| E-share-3 | The knock test runs at the first gated stage and MUST prove revocation and wrong-principal denial. |
| E-ws-2 | Eligible workspace providers form an observed-remove set. Concurrent add/remove and stale replay MUST converge. |
| H-R2 | Role state is distributed/replicated, not node-local authority. |
| H-R3 | A client submits intent. The authority validates, performs the effect, then appends the canonical result while preserving B3 context. Direct client appends are allowed only for record kinds with no privileged effect. |
| H-C1 | Principals use their HOME share. |
| H-C3 | Duplicate display names are allowed; UI disambiguates with an ID suffix. |
| H-C4 | Clone manifest/lock data travels with glade records. |
| H-C5 | Clone makes a provider eligible and warm; it does not seize the active claim. |
| H-C2, H-C6 | Subsumed by the GWZ split and per-surface gating rulings above. |

#### E-chat-4 quota contract

`ChatQuotaSettingsV1` MUST be an immutable, versioned application-policy record
with `max_owned_groups_per_principal = 50`; its version and digest MUST be
composition-pinned. Peers MUST NOT edit or override it. v1 needs no
quota-management UI or general quota subsystem. A later trusted app upgrade or
signed governance operation MAY install a new version; it MUST NOT mutate the
existing record.

The authoritative group-creation service MUST atomically admit and create based
on the count of live groups owned by the B3 principal. Creation number 51 MUST return typed
`QuotaExceeded { limit: 50 }` with no share, binding, invite, or partial group
record created. Deleted/tombstoned groups SHOULD stop consuming quota only once
their terminal state is authoritative. Caller-supplied owner fields and forged
group records MUST NOT affect the count. Tests MUST cover 49→50 success, 50→51
denial, concurrent attempts at the boundary, restart/replay, forged ownership,
and an unauthorized quota-record replacement.

### VII. Razel-facing Grazel interfaces — final deferral ruling

The canonical Razel 0.1.0 IR contains ten methods, but they are not ten product
features at the same maturity:

| Protocol method | Current reality | Deferred treatment |
|---|---|---|
| `build` | Implemented end-to-end | Candidate future `razel.build.submit` exchange; not specified or declared yet. |
| `events.subscribe` | Implemented invocation log | Candidate future `razel.build.events`; not specified or declared yet. |
| `hello`, `version`, `ping` | Implemented protocol/control methods | Keep inside the provider adapter for compatibility, health, and readiness; do not expose them as user supplier features. |
| `run` | Authored but returns typed `Unsupported` | Reserve `razel.run.submit`; do not declare or grant it in v1. |
| `query` | Authored but returns typed `Unsupported` | Reserve read-only target/deps/rdeps/somepath query contracts; do not declare them in v1. |
| `affected` | Authored but returns typed `Unsupported` | Reserve `razel.affected`; do not declare it in v1. |
| `cancel` | Authored; synchronous v1 build cannot be cancelled | Reserve `razel.operation.cancel`; declare only after the daemon has real interrupt semantics and a terminal cancelled event. |
| `shutdown` | Implemented daemon lifecycle control | Provider-local administration only; MUST NOT be a peer-facing Glade supplier surface. |

`ws.relations` also remains reserved until Razel's dependency query is
implemented; Glade MUST NOT invent a second dependency model.

**G-razel RULING.** Defer all Razel-facing Grazel supplier interfaces until the
GWZ supplier family is working. “Working” means the generated GWZ interface
inventory, provider composition, per-surface authorization, operation-result
closure, and at least one real local and forwarded integration path all pass.
Before that gate, no `razel.*` interface is normative, generated, declared in a
`.glade` composition, granted, or implemented. Candidate names in this section
are reservations only and MUST NOT be treated as compatibility commitments.

After the GWZ gate passes, re-read the then-pinned Razel protocol and make a new
grain/capability ruling from its actually supported methods. GWZ's supplier kit,
context propagation, result closure, and authorization tests SHOULD be reused;
the current ten-method Razel inventory MUST NOT be copied mechanically.

### VIII. Implementation ruling — maximum safe parallelism

1. **Security interfaces first.** Define provider attachment policy, fail-closed
   decode, `ProviderCallContext`, device/signed-operation envelopes, capability
   checks, symbolic-self resolution, and matching Rust/TypeScript corpora.
2. **Red tests in parallel.** Each security property MUST have failing local,
   forwarded, replay, and cross-codec tests before its implementation changes.
3. **Start every unblocked lane immediately:** DTO/code generation; text CRDT;
   window/path/reassembler; retention; demand-service/sandbox interfaces;
   chat/users/share schemas; eligibility OR-set; and the GWZ supplier kit.
4. A lane waits only on a contract or security property it actually imports,
   not on roadmap ordering. Integration proceeds in small waves with relevant
   tests, lint, and type checks green.
5. Temporary adapters MAY preserve compile/test progress, but an allow-all
   authorization path, infallible production decoder, or unsigned governance
   operation MUST NOT satisfy a live implementation gate.
