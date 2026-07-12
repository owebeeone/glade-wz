# glade-users — identity, onboarding, and access lifecycle (supplier spec)

Status: full spec v1 (2026-07-12) — expands the `SupplierOutlines.md` entry;
identity model ruled in discussion with Gianni 2026-07-12 (key-canonical
identity, SSH-rootable, no central authority, per-domain name claims).
Common supplier contract: `dev-docs/glade/GladeSupplierModel.md`.
Ratified context: GDL-034 (ownership), GDL-038 (management = ordinary
bindings), AZ-15 (enrollment ceremony), AZ-16/17 (membership/account
semantics), s-grant / s-idp / s-admin traces.

## 1. Identity model (the ruled core)

1. **The principal IS the key.** A principal's canonical id is the
   fingerprint of its public key (ed25519). Names, avatars, emails are
   attributes; none of them is identity.
2. **Sequence-independent convergence** (the u1/u2/u3 requirement): u1
   invites u2, u3 invites u2, u3 invites u1 — u2 sees exactly ONE u1 and ONE
   u3 regardless of order, because every invite path delivers the same
   pubkey and the principal fold is keyed by fingerprint (set-union dedup).
   An invite NEVER mints identity; it introduces an existing key (or invites
   the recipient to mint one at accept).
3. **No central authority.** Keys are self-certifying; trust derives from
   the introduction path: who invited/sponsored whom is recorded (the
   sponsorship chain — AZ-15's model generalized to people). A community's
   view of "who is here" is a fold over signed records, not a registry
   service.
4. **SSH keys as roots (CLI/desktop).** An ed25519 SSH key MAY be a
   principal's root key; signing goes through ssh-agent (no private-key
   extraction — the git ssh-signing precedent). **Browsers cannot reach
   ssh-agent**: a browser session mints a device keypair which the root key
   certifies — exactly the device-cert model already in the authz doc. Root
   custody/recovery questions belong to WD-1 (open, Gianni).
5. **OIDC stays an optional edge** (AZ-3, s-idp): an org may bind an IdP
   assertion to a principal as an *attribute/attestation*; authn ≠ authz;
   never required.

## 2. Names (the two-freds problem)

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
4. **Scoped handles — first-valid-claim-wins per domain** (the
   "blockchain-shaped" mechanism, scoped): a domain MAY carry a
   `users.names` claim registry — signed claim records folded
   first-valid-claim-wins (deterministic: earliest by chain order; ties are
   impossible within one chain, cross-origin ties resolve by the fold's
   existing lamport/origin order). Glade already has every primitive this
   needs — signed append-only chains, equivocation proofs, revocation-wins —
   so this is ordinary records + a fold, NOT a consensus system. The second
   `fred` in a domain visibly loses the handle there and renders `fred·b7c9`
   until they claim another. **Global uniqueness is deliberately not
   promised** (the honest Zooko trade); cross-domain reference = petnames or
   `handle@domain`.

## 3. Ceremonies

### 3.1 Invite → onboard (NEW person; stage-1 buildable as data)
1. Inviter mints an **InviteRecord** (nonce/token, inviter fingerprint,
   optional target-domain grants-to-be, expiry) + a joinable URL that lands
   on the session-placement bootstrap (GDL-032 — grazel's `/bootstrap.json`
   flow is where the token is presented).
2. Recipient opens the URL: presents an existing pubkey OR mints one
   (browser device key; root certification can follow later from their
   CLI root — the add-a-device ceremony inverted).
3. Accept appends: the **PrincipalRecord** (keyed by fingerprint, carrying
   the signed profile) + an **IntroductionRecord** (inviter → invitee — the
   sponsorship edge) + (stage-2) the membership grants the invite promised.
4. Convergence: both sides' home folds now contain the same principal;
   any later invite of the same key anywhere dedups (§1.2).

### 3.2 Introduction of an EXISTING principal
Same flow minus key-minting: the invite carries the fingerprint; accept is
the recipient acknowledging (or auto, policy later). This is how u3's invite
of u1 becomes an introduction edge rather than a duplicate.

### 3.3 Lifecycle (stage-2, rides GDL-034 + WD-1)
Grant / attenuate / revoke via the s-grant/s-admin ceremonies; ancestry-based
admin revocation; device-cert add/remove; key rotation = new-key-certified-
by-old + alias record (the GQ-6 rename idiom applied to identity). Detail
deferred to the stage-2 expansion of this spec.

## 4. Surfaces (declared per GladeSupplierModel; management = ordinary bindings)

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `dir.principals` | log | commons (home) | PrincipalRecords keyed by fingerprint + signed profile attributes |
| `users.introductions` | log | commons (home) | IntroductionRecords (sponsorship edges) |
| `users.invites` | exchange | — | mint/accept ceremony (the supplier answers; token validation) |
| `users.names` | log | commons (per participating domain) | name-claim registry (first-valid-claim-wins fold) — OPTIONAL per domain |
| petnames | value | the user's account domain | local aliases — not this supplier's data; documented here for the rendering rule |

The supplier is the authority for the invite exchange and serves the
directory surfaces; principal/introduction records are ordinary appends any
authorized session can make (GDL-038 — no privileged plane).

## 5. Stage split + migration

- **Stage-1 (buildable now):** the full onboarding FLOW as data — invites,
  key-presenting accept, principal/introduction records, name claims,
  rendering rules. Nothing enforced; signatures structural where the crypto
  seam is still stubbed (same posture as the node: structure real, ed25519
  swaps in behind the seam).
- **Stage-2 (gated on WD-1 + AZ-1/2/3):** grants gate, lifecycle verbs,
  real signature verification end-to-end, invite promises become enforced
  grants.
- **Migration from P0.S7:** today's `PrincipalRecord{principal: str}` is a
  bare-string stub where the string IS the id. Upgrade: new fields
  (fingerprint, profile, sig) with the string demoted to display-name;
  additive on the wire, but the FOLD KEY changes (string → fingerprint) —
  treat as a v2 record kind beside v1 rather than reinterpreting (the
  ChatLine lesson), with v1 records rendered as unverified legacy
  principals.

## 6. Traces to author BEFORE building (atlas leads)

- **s-invite** — 3.1 end-to-end: mint → URL → key-present/mint → records →
  both sides converge; a replayed/expired invite fails as data.
- **s-name-clash** — two freds: second claim loses the domain handle,
  renders fingerprint-suffixed; petname override shown as the viewer-local
  fix.
- **s-converge-identity** — the u1/u2/u3 sequence-independence scenario as
  an invariant-bearing trace (candidate INV: no two principal records with
  one fingerprint after fold).
- s-grant / s-admin / s-idp already exist for the stage-2 half.

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

- WD-1 root custody + recovery (blocks stage-2 lifecycle, not stage-1 flow).
- Is the `users.names` claim registry v1 or later? (Petnames + fp-suffix
  alone are sufficient for correctness; handles are UX.)
- Invite expiry/one-shot semantics default; whether accepts need inviter
  countersign (AZ-14 adjacent) in v1.
