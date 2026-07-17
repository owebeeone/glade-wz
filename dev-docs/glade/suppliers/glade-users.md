# glade-users — identity, onboarding, and access lifecycle (supplier spec)

Status: full spec v1 (2026-07-12; GLP-0006 §VI + B5 rulings landed 2026-07-12)
— expands the `SupplierOutlines.md` entry; identity model ruled in discussion
with Gianni 2026-07-12 (root-key-canonical identity, SSH-rootable, no central
authority, certified device keys). Common supplier contract:
`dev-docs/glade/GladeSupplierModel.md`.
Ratified context: GDL-034 (ownership), GDL-038 (management = ordinary
bindings), AZ-15 (enrollment ceremony), AZ-16/17 (membership/account
semantics), s-grant / s-idp / s-admin traces.
GLP-0006 rulings landed here: **E-users-1** (identity = root-key fingerprint;
device keys certified), **B5** (device proof at session establishment + signed
governance ops), **E-users-2** (invites = durable log record + exchange),
**E-users-3** (canonical names registry deferred), **H-C1** (principals live in
their HOME share), **H-R3** (client submits intent; the authority appends the
canonical result).

## 1. Identity model (the ruled core)

1. **The principal IS the key.** A principal's canonical **account** id is the
   fingerprint of its **root** public key (ed25519). A browser/device key is a
   root-*certified* subordinate, never itself the account identity (§1.4,
   E-users-1). Names, avatars, emails are attributes; none of them is identity.
2. **Sequence-independent convergence** (the u1/u2/u3 requirement): u1
   invites u2, u3 invites u2, u3 invites u1 — u2 sees exactly ONE u1 and ONE
   u3 regardless of order, because every invite path delivers the same
   root pubkey and the principal fold is keyed by root fingerprint (set-union
   dedup).
   An invite NEVER mints identity; it introduces an existing key (or invites
   the recipient to mint one at accept).
3. **No central authority.** Keys are self-certifying; trust derives from
   the introduction path: who invited/sponsored whom is recorded (the
   sponsorship chain — AZ-15's model generalized to people). A community's
   view of "who is here" is a fold over signed records, not a registry
   service.
4. **Root keys + certified devices (E-users-1, B5).** An ed25519 SSH key MAY
   be a principal's **root** key; signing goes through ssh-agent (no private-key
   extraction — the git ssh-signing precedent). **Browsers cannot reach
   ssh-agent**: a browser session mints a **device** keypair which the root key
   **certifies** (a device-cert record). The account identity is the root
   fingerprint; a device is usable only once certified by that root, and
   **session establishment MUST prove possession of a device key certified by
   the account root** (B5) — an uncertified or revoked device fails closed.
   Account **merge / root transition** (rotate the root, fold two accounts into
   one) is a signed ceremony that MUST land **before any governance depends on
   it** — the root of trust cannot be retrofitted under live grants. Root
   custody/recovery + the merge ceremony detail belong to WD-1 (open, Gianni).
5. **OIDC stays an optional edge** (AZ-3, s-idp): an org may bind an IdP
   assertion to a principal as an *attribute/attestation*; authn ≠ authz;
   never required.

## 2. Names (the two-freds problem)

**v1 scope (E-users-3):** identity is the fingerprint; display is
**non-authoritative** local display names + petnames + fingerprint suffixes.
The canonical naming/alias registry (§2.4) is **DEFERRED** — petnames +
fp-suffix are sufficient for correctness; authoritative handles are UX, not v1.

1. **Display name = a signed, self-asserted attribute** on the principal
   (the key signs its own profile record). Anyone can assert any name —
   so the UI discipline below is normative, not cosmetic.
2. **Rendering rule:** a bare display name is NEVER shown where principals
   can collide; render `name·fp6` (first 6 of the fingerprint) unless a
   petname or scoped handle applies.
3. **Petnames (local):** each user may alias any principal ("fred (work)")
   — stored in the user's OWN account domain (AZ-17 territory), never
   replicated to others, always wins over the self-asserted name in that
   user's UI.
4. **Scoped handles — DEFERRED to a later version (E-users-3).** A domain MAY
   *later* carry a `users.names` claim registry — signed claim records folded
   first-valid-claim-wins, so the second `fred` in a domain loses the handle
   there and renders `fred·b7c9` until they claim another; **global uniqueness
   is deliberately never promised** (the honest Zooko trade), cross-domain
   reference = petnames or `handle@domain`. This is **not v1**: petnames +
   fp-suffix already give correctness.
   **Retraction (B5):** the earlier claim that "glade already has every
   primitive this needs — signed append-only chains, equivocation proofs,
   revocation-wins" was an **overclaim**. Those signed-governance primitives are
   **new, load-bearing work** (device proof + per-op signatures +
   verify-before-fold, §3.3 / §5), not something already sitting behind the
   seam. The registry waits on that substrate.

## 3. Ceremonies

### 3.1 Invite → onboard (NEW person)
1. Inviter mints an **InviteRecord** (nonce/token, inviter fingerprint,
   optional target-domain grants-to-be, expiry). It is BOTH a **durable log
   record** (appended to `users.invite.records` — audit/replay truth) AND
   delivered through the `users.invites` **exchange** (live mint/accept)
   (E-users-2), plus a joinable URL that lands on the session-placement
   bootstrap (GDL-032 — grazel's `/bootstrap.json` flow is where the token is
   presented).
2. Recipient opens the URL: presents an existing pubkey OR mints one. A browser
   mints a **device key** which the account **root certifies** (a device-cert
   record); root certification can follow later from their CLI root (the
   add-a-device ceremony inverted). The account identity is always the root
   fingerprint (E-users-1); the device key is its certified subordinate.
3. Accept is a **submitted intent** (H-R3): the recipient signs with the
   certified device; the **users authority** validates the token + device cert
   and **appends the canonical result** — the **PrincipalRecord** (keyed by root
   fingerprint, signed profile) + an **IntroductionRecord** (inviter → invitee
   sponsorship edge) + (gated stage) the membership grants the invite promised —
   preserving the B3 requester context. Introduction/grant records carry
   privileged effect, so the client never appends them directly.
4. Convergence: both sides' home folds now contain the same principal;
   any later invite of the same key anywhere dedups (§1.2).

### 3.2 Introduction of an EXISTING principal
Same flow minus key-minting: the invite carries the fingerprint; accept is
the recipient acknowledging (or auto, policy later). This is how u3's invite
of u1 becomes an introduction edge rather than a duplicate.

### 3.3 Lifecycle (gated stage, rides GDL-034 + WD-1)
Grant / attenuate / revoke via the s-grant/s-admin ceremonies; ancestry-based
admin revocation; device-cert add/remove; key rotation = new-key-certified-
by-old + alias record (the GQ-6 rename idiom applied to identity).

**Signed governance (B5) — stage-1 load-bearing, not deferred crypto.** Every
security-sensitive op (grant / revoke / name-claim, device add/remove, key
rotation) MUST be **signed by an authorized certified device**, carry the
**strict log predecessor** its log requires, and be **verified before
persistence or fold**. Revoked or uncertified devices **fail closed**. Legacy
unsigned records (the P0.S7 string stubs, §5) MAY be retained as **unverified
history** but MUST NOT create, extend, or revoke governance authority. The verbs
themselves are detailed in the gated-stage expansion; the *signed envelope +
verify-before-fold* is stage-1 substrate.

## 4. Surfaces (declared per GladeSupplierModel; management = ordinary bindings)

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `dir.principals` | log | commons (**HOME share**, H-C1) | PrincipalRecords keyed by **root** fingerprint + signed profile attributes |
| `users.introductions` | log | commons (HOME share) | IntroductionRecords (sponsorship edges) |
| `users.invite.records` | log | commons (HOME share) | durable InviteRecords — audit/replay truth (E-users-2) |
| `users.invites` | exchange | — | mint/accept ceremony delivery (the authority answers; token + device-cert validation) (E-users-2) |
| `users.devices` | log | commons (HOME share) | device-cert records — root-certified device keys, add/remove (B5) |
| `users.names` | log | commons (per domain) | name-claim registry — **DEFERRED, not v1** (E-users-3) |
| petnames | value | the user's account domain | local aliases — not this supplier's data; documented here for the rendering rule |

Principals live in their **HOME share** (H-C1) — that pins how far
`dir.principals` and the ceremony logs replicate. The supplier is the
**authority** for the invite exchange and the signed governance ops (B5): for
any record with **privileged effect** (introductions, grants, device certs) the
client **submits intent** and the authority **validates + appends the canonical
result** (H-R3). Only record kinds with **no** privileged effect (a principal
editing its own signed profile attribute) are direct authorized appends
(GDL-038).

## 5. Stage split + migration

The §B reframe applies: the **identity substrate is stage-1 load-bearing**, not
deferred crypto. "Structural signatures behind a stub" is **retracted** for
**governance** records (B5).

- **Stage-1 (buildable now; substrate is load-bearing):** the full onboarding
  FLOW as data — invites (durable record + exchange), device-cert accept,
  principal/introduction records, rendering rules. **The signed-operation
  envelope, device proof at session establishment, and verify-before-fold for
  governance ops are stage-1** (B5): real, not stubbed — an unsigned governance
  op MUST NOT satisfy this gate. Non-governance data (a self-asserted profile
  name) may still ride the structural seam.
- **Gated stage (rides WD-1 + AZ-1/2/3):** grants actually gate, lifecycle
  verbs, full end-to-end enforcement, invite promises become enforced grants,
  revoked/uncertified devices denied end-to-end.
- **Migration from P0.S7:** today's `PrincipalRecord{principal: str}` is a
  bare-string stub where the string IS the id. Upgrade: new fields (**root**
  fingerprint, profile, sig) with the string demoted to display-name; additive
  on the wire, but the FOLD KEY changes (string → root fingerprint) — treat as a
  v2 record kind beside v1 rather than reinterpreting (the ChatLine lesson).
  These v1 string stubs are exactly the **legacy unsigned records** of B5:
  retained as **unverified history**, they MUST NOT create, extend, or revoke
  governance authority.

## 6. Traces to author BEFORE building (atlas leads)

- **s-invite** — 3.1 end-to-end: mint (**durable `users.invite.records` log +
  `users.invites` exchange**) → URL → device-key mint + **device-cert accept**
  → authority appends the canonical records (H-R3) → both sides converge; a
  replayed/expired invite fails as data.
- **s-device-cert** — a browser mints a device key; the account root certifies
  it; session establishment proves possession of the certified device (B5); an
  **uncertified** device is refused and a **revoked** device **fails closed** on
  its next session.
- **s-signed-governance-deny** — a governance op (grant/revoke/name-claim)
  signed by an **uncertified or revoked** device is rejected before persist/fold;
  an op with a **missing/wrong predecessor** is rejected; a **legacy unsigned**
  record does NOT create/extend/revoke governance (B5).
- **s-name-clash** — two freds render fingerprint-suffixed (`name·fp6`); a
  petname override is the viewer-local fix. (The domain handle-claim arm is
  deferred with the registry — E-users-3.)
- **s-converge-identity** — the u1/u2/u3 sequence-independence scenario as
  an invariant-bearing trace (candidate INV: no two principal records with
  one **root** fingerprint after fold).
- s-grant / s-admin / s-idp already exist for the gated-stage half.

## 7. Dependencies, consumers, user-testable-when

- Depends on: base glade only (first in the spine). Session placement
  (grazel bootstrap) carries the invite URL but the ceremony is generic.
- Consumers: every multi-user supplier (chat groups, share, editing,
  terminal ownership) keys on fingerprints and renders via §2.
- **User-testable when:** I mint an invite, a second real person opens it on
  another machine, onboards with their own key (or their SSH key from a
  CLI), and both of us appear in the user list — same list on both sides,
  regardless of who invited whom first; a name collision renders
  discriminated.

## 8. Open questions (Gianni)

- **RESOLVED (E-users-3):** the `users.names` claim registry is **later, not
  v1** — petnames + fp-suffix suffice for correctness.
- **RESOLVED (E-users-1):** account identity is the root-key fingerprint;
  browser/device keys are certified subordinates; account merge / root
  transition is a signed ceremony that MUST land before governance depends on it
  (mechanism detail → WD-1).
- WD-1 root custody + recovery + the merge/root-transition ceremony detail
  (blocks the gated-stage lifecycle, not the stage-1 flow).
- Invite expiry/one-shot semantics default; whether accepts need inviter
  countersign (AZ-14 adjacent) in v1.
