# glade-chat — group chat over glade (supplier spec)

Status: full spec v1 (2026-07-12); **E-chat-1..5 RATIFIED** (GLP-0006
`RulingWorksheet.md` §VI + §I B3/B5 identity) — expands the `SupplierOutlines.md` entry.
**The one PARTIALLY-BUILT supplier**: stage-1 shipped as `@owebeeone/glade-chat`
(GLP-0006 P1.S1) and is live-verified; this spec PINS that baseline precisely
and specs the upgrade path. Common supplier contract:
`dev-docs/glade/GladeSupplierModel.md` (NOTE §2's two mechanisms — chat is NOT
in the message hot path). Built sources verified: the `glade-chat/` repo
(`src/manifest.ts`, `supplier.ts`, `post.ts`), the `s-chat` trace (ggg-viz
`src/scenario/chat.ts`, catalog `Z2`), and `Decisions.md` (P1.S1 build notes,
P1.S4 interop caveat). Upgrade context: `glade-share.md`, AZ-16, glade-users §2.

## 1. The built stage-1 baseline (PINNED — do not drift)

The stage-1 supplier is honest and small. Four facts are load-bearing and the
upgrade MUST preserve their consumer-facing shape unless it explicitly migrates:

1. **A group is a KEY, not a glade id.** One glade id `chat.msgs` (shape `log`,
   domain `document`, zone `commons`, share `chat`) carries every group; the
   **group id is the wire key** (`groupKey(id)` = UTF-8 bytes). `chatManifest`
   yields one typed `Surface` handle per group — same glade id, distinct key.
   Isolation is pure routing (catalog `Z2`, "keyed-commons isolation"): a
   `#general` subscriber's interest never matches a `#dev` op. NO grant is
   consulted in stage-1; the key does all the work.
2. **Per-line principal attribution.** `ChatLine = { ts, user, text, principal? }`
   — `principal` is **taut tag 4, OPTIONAL** (additive beside `user`, never a
   reinterpretation — the recorded P1.S1 rule; absent ⇒ decodes to `null`).
   Attribution rides EACH line, never the connection: two principals posting into
   one group's log each self-attribute; `user` may differ from `principal` (the
   agent-acting-as-Gianni case). **B3/B5 correction (consume):** the built stage-1
   trusts a caller-payload `principal`; the upgrade STAMPS `ChatLine.principal`
   from the node-authenticated `ProviderCallContext` (B3) — the B5 device-certified
   **fingerprint** — and a caller-supplied `principal` field MUST NOT override that
   context. `user` stays the self-asserted display name (§7).
3. **The supplier is OUT of the message hot path** (GladeSupplierModel §2). It
   does exactly two things: (a) appends a `BindingDecl`-shaped record per
   pre-declared group on `chat.decl` (keyed by group id — GDL-037); (b) SERVES the
   `chat.groups` metadata VALUE (op-append — the only surface it serves). **Posting
   is a CLIENT append** (`postChat`) the node folds + replicates to every subscriber
   of the group's key. The supplier never appends `chat.msgs` (asserted in the unit
   test). **E-chat-2 retraction (SR56-10/2-15):** step (a)'s `chat.decl` append is
   **NON-AUTHORITATIVE** — the node never consumes it; the built "runtime-declares
   each group's surface" claim is WITHDRAWN. Authoritative declaration is a typed,
   authz-checked HOME `dir.bindings` record (§3.4); `chat.decl` survives as
   decorative metadata at most.
4. **Late-joiner history via from-cursor replay.** The history IS the log — a
   late session subscribes the group key **from-cursor** (the log-shape backlog
   primitive, grip-share `log_binding`) and folds the full history from the
   node's replica, attribution intact, no supplier hop and **no store beyond the
   share itself** (SupplierRequirements: none).

Groups are **PRE-DECLARED config** (in `grazel-app.glade` / grazel config) in
stage-1 — the stopgap the upgrade replaces. `chat.groups` codec is **JSON**
(glial default, language-neutral); `ChatLine` is **taut** (§6 unifies clients).

## 2. Surfaces

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `chat.msgs` | log | commons | `ChatLine` records; CLIENT-appended, supplier out of path; the history is the log. **Stage-1:** keyed by group id (a group = a key). **Stage-2 (E-chat-1):** carried by the group's OWN share; keyed-commons survives only as the stage-1 migration source |
| `chat.groups` | value | commons | group-list metadata (JSON) — the ONLY surface the supplier serves (op-append); the fold is the authority, not the seed list |
| `chat.decl` | log | commons | **NON-AUTHORITATIVE (E-chat-2):** the built append the node never consumes; decorative metadata at most, NOT the registry |
| HOME `dir.bindings` group decl | log | HOME share | **stage-2 (E-chat-2)** — the AUTHORITATIVE typed, authz-checked `BindingDecl` declaring a group's surface; the real registry path (§3.4) |
| group share (via `share.create`) | (share) | (per group) | **stage-2 (E-chat-1)** — each group is its OWN share, minted by the glade-share `share.create` ceremony (E-share-1); §3.1 |
| group-share membership binding | log | (per group-share) | **stage-2** — AZ-16 membership grant-instances per group (§3.2); home of the join/revoke lifecycle |
| `ChatQuotaSettingsV1` | value | (composition-pinned) | **stage-2 (E-chat-4)** — IMMUTABLE, versioned application-policy record (`max_owned_groups_per_principal = 50`); the group-creation admission input (§4) |
| chat tombstone/supersede | log | commons | **stage-2 (E-chat-5)** — a SIGNED edit/delete record beside the original `ChatLine`; the original stays immutable + auditable (§3.5) |
| link attachment | (attachment record) | commons | **stage-2** — a `LinkRecord` beside a `ChatLine`, riding the group's commons (§5, rides glade-share) |

`chat-sup` is a wire-attached authority session (P00-a), pool role `provider`,
drawn distinct from the node so its declare + serve-metadata role reads as out
of the hot path.

## 3. The upgrade: real groups as data

Stage-1's pre-declared config is the stopgap. A **real** group is created,
declared, joined, revoked, and edited as data — reusing the ceremonies
glade-share already owns.

### 3.1 Group creation = the `share.create` ceremony (E-chat-1 RATIFIED)
**RULING (E-chat-1 — F5-3/SR56-28): a real group owns ITS OWN SHARE.** Creating a
group is **exactly the glade-share `share.create` ceremony** (E-share-1 — the share
family owns `share.create`/`invite`/`grant`/`revoke`/`status`), NOT a bespoke chat
verb. The ceremony mints the group's share + seeds its ACL; the declare-a-surface
step becomes a typed HOME `dir.bindings` record write (§3.4), not a boot-time config
replay. A created group appears in `chat.groups` the moment its records replicate
(the fold is the only authority) — the pre-declared list becomes the initial seed,
not a fixed ceiling.

**Keyed-commons is retired to stage-1.** The built one-share-N-keys model (a group
= a KEY inside one `chat` share) survives ONLY as the stage-1 migration source: a
single AZ-16 grant on that one share would admit ALL groups, which the per-share
model forbids. The group key MAY survive as the in-share address for migration
continuity, but membership is henceforth the group-share's own AZ-16 grant (§3.2).
This is the ruling that decides keyed-commons does NOT survive stage-2 as an
authority.

### 3.2 Membership = AZ-16 grants
Joining a group is an **AZ-16 membership grant**: one grant carries the group's
**commons** (read/post the shared log) AND **your own private zone** in that
group's domain (per-member scratch — cursors, drafts, read-marks; the s-zones
pattern). **Revoke is one clean cut**: removing the membership grant severs
commons AND private in a single act (AZ-16's offboarding property — your
private-in-someone's-domain data is tenant, not freehold; forward-only caveat
unchanged). Routing (`Z2`) is UNCHANGED — the same keyed delivery, now GATED at
the join: stage-2's `check()` on the commons subscribe is where the built
stub-allow-all seam lands with no supplier rewrite.

### 3.3 Who may create groups (E-chat-4 RATIFIED)
**RULING (E-chat-4): any AUTHENTICATED principal MAY create a group and becomes its
owner** (create-a-share is self-serve; the owner then controls membership via §3.2),
**subject to the `ChatQuotaSettingsV1` admission contract (§4).** The creating/owning
principal is the B3 authenticated principal — a caller-supplied owner field MUST NOT
set ownership. The grant-gated alternative (only a domain/workspace owner may spin up
groups) is not taken.

### 3.4 Group declaration = typed HOME `dir.bindings` record (E-chat-2 RATIFIED)
**RULING (E-chat-2 — SR56-10/2-15): dynamic group declaration is an AUTHORITATIVE
typed, authorization-checked `BindingDecl` on HOME `dir.bindings`** — the real
registry path the node actually consumes. The stage-1 `chat.decl` JSON app-share
append is **non-authoritative** (§1 fact 3, retracted): keep it only as decorative
metadata, if at all. The declaration write is authz-checked against the B3 caller
context, and the group becomes routable the moment the `dir.bindings` record folds.
Test obligation: this MUST be exercised on a **spawned node**, not only an
in-memory provider (the s-chat-decl-realnode trace, §9).

### 3.5 Message edit/delete = signed tombstone / supersede (E-chat-5 RATIFIED)
**RULING (E-chat-5): editing or deleting a posted line is a SIGNED tombstone or
superseding record** (the flip-instance idiom from glade-share §4), never an
in-place rewrite. The original `ChatLine` stays **immutable and auditable**; the
tombstone/supersede record is B5-signed by an authorized certified device and rides
the group's commons beside the original. Clients render the superseded state; the
audit trail is the full record history.

**OPEN (surfaced by the s-chat-edit trace): whose authority may edit a line?**
E-chat-5 says "an authorized certified device" but does not rule *whose*. A B5
signature only proves the SIGNER is a certified device — it does NOT prove the
signer authored the target line. So superseding/tombstoning a line needs a
**distinct EDIT-AUTHORITY check** beyond signature validity: the signer must be
the line's **author**, OR hold a **moderation grant** on the group. (The trace
models this as a separate ingest check: a validly-signed cross-author supersede
by a non-author is refused.) v1 lean: **author-only**, with moderation deferred.
Ruling needed — this is not covered by verify-as-ingest (signature + prev-hash +
seq), which a non-author's own-origin record would pass.

## 4. Group-creation quota — `ChatQuotaSettingsV1` (E-chat-4 RATIFIED)

Any-principal create (§3.3) is bounded by ONE immutable policy record, not a
general quota subsystem — v1 needs no quota-management UI.

**The policy record.** `ChatQuotaSettingsV1` MUST be an **immutable, versioned,
composition-pinned** application-policy record carrying
`max_owned_groups_per_principal = 50`. Its version and digest are composition-pinned;
peers MUST NOT edit or override it. A later trusted app upgrade or signed governance
operation MAY install a NEW version but MUST NOT mutate the existing record.

**The admission service.** The authoritative group-creation service MUST
**atomically admit-and-create** on the count of LIVE groups owned by the B3
principal (§3.3). Creation #51 MUST return typed **`QuotaExceeded { limit: 50 }`**
with **NO** share, binding, invite, or partial group record created. A
tombstoned/deleted group stops consuming quota **only once its terminal state is
authoritative** (§3.5). **Caller-supplied owner fields and forged group records MUST
NOT affect the count** — the count is over B3-attributed ownership, never a
self-declared owner field.

**Tests (s-chat-quota, §9).** 49→50 admitted; 50→51 denied with `QuotaExceeded`;
concurrent attempts at the boundary; restart/replay; forged ownership; and an
unauthorized `ChatQuotaSettingsV1` replacement rejected.

## 5. Links in chat (rides glade-share — not re-spec'd here)

A link shared into a group is a `LinkRecord` **attachment record beside the
`ChatLine`, riding the group's commons share** (glade-share §1–§2 — "a link is a
record in the medium that carries it"). **Chat sharing = the membership-snapshot
grant mint** (glade-share §3): sharing enumerates the group's membership fold NOW
and mints N per-principal grants to the closure the link needs; future joiners
were never granted (the snapshot is the default of principal-scoped grants, not
an expiry). **The chat share IS the knock rendezvous** (glade-share §5): a later
joiner who clicks knocks on the carrying group share, a grantor ingests + approves
(the s-grant beat), and the mount unblocks. Chat is the FIRST carrying medium
glade-share assumes — this spec supplies the medium; glade-share owns the link
model, closure, and disposition lifecycle. See `glade-share.md` §1–§3, §5.

## 6. Codec unification (E-chat-3 RATIFIED)

**RULING (E-chat-3): taut `ChatLine` is the SOLE chat payload for `chat.msgs`.**
The P1.S4 caveat is real: the glade demo encodes `chat.msgs` as taut `ChatLine`
while gryth-ui encoded it as JSON — the two UIs are **NOT wire-interoperable on
chat payloads**. JSON was bring-up. Unification: gryth-ui decodes `chat.msgs` via
the taut IR **vendored into `@grythjs/glade`** (the P1.S4 bootstrap seam;
`glade.ir.json` already vendored there, refreshed on wire change), retiring its
JSON path. `chat.groups` metadata stays JSON (glial default — a config value, no
cross-language message need).

**Migration posture (RESOLVED — the JSON→taut forward cut is a HARD gate):**
because the two are today SEPARATE grazel deployments and gryth-ui runs an
ephemeral `MemoryStoreEngine` (P1.S4 — no persisted corpus), the cutover is a
**forward cut**, not a data migration: no long-lived JSON `chat.msgs` records
outlive the switch. For any FUTURE persisted deployment, a legacy JSON `ChatLine`
decodes best-effort as a legacy line (the additive-field / v2-record-beside-v1
lesson from glade-users §5). The forward cut is a **HARD migration gate before
mixing clients on one node** — a P2 gate, not an opportunistic cleanup.

## 7. Identity rendering (per glade-users §2)

Chat is the heaviest multi-principal renderer — it is the proving ground for the
rendering discipline. Post glade-users migration, `ChatLine.principal` IS the
**fingerprint** (the canonical id — the B5 device-certified id stamped from the B3
context, §1 fact 2, never the caller payload); `user` is the demoted self-asserted
display name. Chat UIs MUST render **`name·fp6`** (name + first 6 of the fingerprint)
wherever principals can collide — NEVER a bare display name (anyone can assert any
name). **Petnames win locally** (the viewer's own alias for a principal overrides
the self-asserted name); a domain that carries a `users.names` claim registry MAY
show the scoped handle. This is glade-users §2 applied verbatim — chat renders it,
glade-users owns it.

## 8. Stage split + migration

- **Security stage-1 (PREREQUISITE — RulingWorksheet §B):** the identity substrate
  is stage-1, not stage-2 — the built "allow-all" is neither safe nor honestly
  exercisable without it. `ChatLine.principal` MUST come from the B3
  node-authenticated `ProviderCallContext` (§1 fact 2), and security-sensitive
  records (declaration, membership, tombstone) MUST be B5-signed by a certified
  device. Attribution is a stage-1 must; it depends on this machinery landing first.
- **Stage-1 (BUILT):** keyed-commons groups (group = key on `chat.msgs`),
  per-line principal attribution, supplier out of the hot path (declare + serve
  `chat.groups` only), late-joiner history via log replay, pre-declared group
  config, taut `ChatLine` + JSON `chat.groups`. Allow-all; nothing gated.
- **Stage-2 (this spec):** groups CREATED via `share.create` (§3.1, E-chat-1)
  replacing pre-declared config; declaration via typed HOME `dir.bindings` (§3.4,
  E-chat-2); AZ-16 membership gates each join, revoke cuts commons + private in one
  act (§3.2); any-principal create bounded by `ChatQuotaSettingsV1` (§4, E-chat-4);
  edit/delete via signed tombstone/supersede (§3.5, E-chat-5); links-in-chat via
  glade-share (§5); taut-`ChatLine` the sole codec (§6, E-chat-3); identity rendered
  `name·fp6` (§7). The built surfaces' consumer shape is preserved; the
  group-as-share migration (§3.1) is the one shape change, and it is
  additive-then-cutover, not a rewrite.
- **Retention (GAP-10):** the history is the log; a retention/TTL cap is deferred
  to the GAP-10 ruling. Chat is the first heavy unbounded-append consumer (§11).

## 9. Traces to author before building

- **s-chat** (BUILT) — the stage-1 baseline: supplier declares + serves, two
  principals converge in one keyed-commons log, `#dev` isolated by keying, late
  joiner folds full history. Remains the stage-1 executable spec.
- **s-chat-create** (E-chat-1) — group creation as the `share.create` ceremony: a
  principal creates a group (= its OWN share, E-share-1), it appears in
  `chat.groups` on replication, members join and post. Proves group === share via
  `share.create` and that pre-declared config is gone (candidate INV: a group is
  routable the moment its create records fold).
- **s-chat-quota** (E-chat-4) — the `ChatQuotaSettingsV1` admission: a principal at
  49 owned groups creates the 50th (admitted), the 51st returns
  `QuotaExceeded { limit: 50 }` with no partial record; two concurrent creates at
  the boundary admit exactly one; a forged owner field does not inflate the count.
  Proves atomic admit-and-create on the live-owned count (§4).
- **s-chat-decl-realnode** (E-chat-2) — declaration reality: a group is declared by
  a typed, authz-checked HOME `dir.bindings` record and becomes routable on a
  **spawned node** (not an in-memory provider); the `chat.decl` append is shown
  non-authoritative. Proves the real registry path (§3.4).
- **s-chat-join** — AZ-16 membership: join carries commons + the joiner's private
  zone; posting works; **revoke cuts BOTH in one act** and the revoked member's
  next op is denied (stage-2 gate) while `Z2` routing is unchanged. Proves the
  membership unit + the one-clean-cut offboarding.
- **s-chat-edit** (E-chat-5) — message mutation: an edit and a delete each land as a
  B5-signed tombstone/supersede record beside the original `ChatLine`; the original
  is unchanged and still folds in the audit history; clients render the superseded
  state. Proves immutable-original edit/delete (§3.5).
- **s-chat-codec** (E-chat-3) — codec unification: a taut-`ChatLine` client and a
  legacy JSON record; post-unification both decode via the vendored taut IR (the
  SOLE payload); the legacy record decodes best-effort. Proves one wire codec + the
  forward-cut hard gate (§6).
- **Links-in-chat needs NO new chat trace** — it is exercised by glade-share's
  `s-link-share` / `s-link-knock` with chat as the carrying medium (§5).

## 10. Dependencies + user-testable-when

- Depends on: **glade-users** (real principals — the membership fold + `name·fp6`
  rendering, §7), **glade-share** (group creation = `share.create`, AZ-16
  membership, links — §3/§5), base glade (keyed-commons log, fold/replicate,
  taut codec). Consumers: gryth-ui `@grythjs/plugin-chat`, the glade demo Chat tab.
- **User-testable when (NORMATIVE — SupplierOutlines):** I invite the onboarded
  user to a group and **we chat from two machines — not two URL-stub tabs on one
  machine.** Fuller: I create a new group (create-a-share), invite them (AZ-16
  membership), we converge in `#general` both ways with per-line attribution
  rendered `name·fp6`; I revoke and their access to commons AND their private zone
  ends in one cut; a link I share into the group grants the membership snapshot and
  a later joiner knocks for it.

## 11. Rulings landed + remaining opens

**RATIFIED (GLP-0006 RulingWorksheet §VI — E-chat-1..5 RESOLVED):**
1. **Group-as-share (was open) — RESOLVED E-chat-1:** a real group owns its own
   share (`share.create`, E-share-1); keyed-commons survives only as the stage-1
   migration source, and does NOT survive stage-2 as an authority (§3.1).
2. **Group declaration — RESOLVED E-chat-2:** authoritative declaration is a typed,
   authz-checked HOME `dir.bindings` record proven on a spawned node; `chat.decl`
   JSON is non-authoritative (§3.4).
3. **Codec migration (was open) — RESOLVED E-chat-3:** taut `ChatLine` is the sole
   payload; the JSON→taut forward cut is a hard migration gate before mixing
   clients (§6).
4. **Who may create groups (was open) — RESOLVED E-chat-4:** any authenticated
   principal, self-serve owner, bounded by the immutable `ChatQuotaSettingsV1`
   admission record (§3.3, §4).
5. **Message edit/delete (was open) — RESOLVED E-chat-5:** a signed
   tombstone/supersede record; the original stays immutable + auditable (§3.5).

**Remaining open:**
- **Retention (GAP-10)** — chat is the first heavy unbounded-append log; whether an
  old group's log gets a TTL/size cap or is append-forever is deferred to the
  F-GAP10 retention ruling (see the §8 retention note).
