# glade-share — links, sharing ceremonies, and the grant-request lifecycle (supplier spec)

Status: full spec v1 (2026-07-12; GLP-0006 §VI rulings landed 2026-07-12) —
expands the `SupplierOutlines.md` entry; link model + the four rulings from
Gianni 2026-07-12 (membership snapshot · auto-request-the-rest · link values
are ID types, developer's discipline · requests = pending grants, same shape +
same place, with flippable disposition history). Common contract:
`dev-docs/glade/GladeSupplierModel.md`.
Context: AZ §4/§4a (policy rides the share; AZ-16 membership semantics),
s-grant (the approval beat, already traced), GDL-004/030 (the deferred LIVE
context-sharing this is the frozen-slice sibling of), grip-lab's drag/drop
grip:value links (the precursor).
GLP-0006 rulings landed here: **E-share-1** (the share family owns direct
membership — `share.create/invite/grant/revoke/status`; links layer on top),
**E-share-2** (v1 links carry portable commons grants + inline IDs only;
private/account refs rejected at capture), **E-share-3** (the knock test runs at
the first gated stage, proving revocation + wrong-principal denial), **D11** (a
directed, authenticated access request + durable offline queue; read ≠ append).

## 1. The share model — direct membership is the core; links layer on top

**Direct membership (E-share-1) is the normative flow.** "Share this workspace
with that principal" is a first-class ceremony the share family OWNS — not a
by-product of links. The family exposes five exchanges over **ordinary grant
records** (AZ §4 — policy rides the share; GDL-038 — no privileged plane):

| exchange | effect |
| --- | --- |
| `share.create` | mint a new share (its commons + policy binding) |
| `share.invite` | offer membership to a principal — a pending grant (`requested`, §4) |
| `share.grant` | admit a principal: mint the membership grant(s) (the AZ-16 unit) |
| `share.revoke` | cut a principal's membership — revocation-wins (§4); cuts the commons grant **and** that recipient's private zone together |
| `share.status` | read a share's membership + pending-request fold |

Each is a **submitted intent** (H-R3): the client asks, the share **authority**
validates against the target policy binding and appends the canonical grant
record, preserving the B3 requester context. `share.grant` / `share.revoke`
carry privileged effect and are never direct client appends.

**Links layer ON TOP of this** — a convenience capture/restore path that
ultimately mints the *same* grant records (§3 closure → `share.grant`), not the
only way to share.

A **link** is a restorable slice of page state. In grip-lab it was all
values (`{grip → value}` restored at atom taps). In glade it splits — and
the split is the design:

| entry kind | carries | restore | access implications |
| --- | --- | --- | --- |
| **refs** | binding **instance identities** `(glade id, share/zone/key fill)` — **v1: portable commons + inline share/glade IDs only** (E-share-2) | glial mounts them | THE access problem — the value lives in a share |
| **values** | inline `(grip → value)` pairs for local-only atoms | set at atom taps (the grip-lab move) | none |

**Capture discipline (E-share-2, RULED):** v1 link capture admits **only
portable commons grants + inline share/glade IDs** (plus explicit local atom
values). A ref into a **private or account zone** (e.g. `self:alice`) is
**REJECTED at capture** — it is not portable and cannot be validly rebound for
another principal. This closes the `self:alice` private-ref leak: an emailed or
chat-pasted link never carries a reference only its author could resolve.

Capture needs no new introspection: every mounted `GlialTap` already knows
its `(decl, fill)` — capture = walk the shared context's mounts, emit the
**admissible** instance identities + the local atom values, rejecting private/
account refs. Restore = mount refs, set atoms.

`LinkRecord = { refs, values, meta }`, **content-addressed** (hash) — links
are stable, dedupable, verifiable.

**Values discipline (RULED):** link `values` are **ID-type payloads** — a
number of k:v pairs is fine; the developer decides what belongs. Fat inline
state is a **smell** (it replicates with every share that carries the link):
this paragraph is the doc, and the capture API carries a comment saying the
same. No enforcement, no size machinery — deliberately.

## 2. Where a link lives

A link is **a record in the medium that carries it** — shared into a chat,
it rides that group's commons share (an attachment beside the ChatLine). No
dedicated link-share exists. The knock a link can trigger is a **directed
request to the target authority**, not a write-back into this carrying share
(§5, D11).

## 3. Share-time: closure + the confirm UI

At share time glade-share computes the **access closure** of the link's
refs: the set of `(domain, commons)` memberships required — deduped, minus
what the audience already holds, minus the carrying share itself. That set
IS the confirmation dialog, verbatim:

> "Sharing this link will grant the 3 members of #dev access to:
> workspace `razel` · doc `7`."

AZ-16 keeps the unit honest: one membership grant per domain (commons +
each recipient's own private zone); zones are never fiddled individually.

**Membership snapshot (RULED):** "the users attached to that chat" = the
group's **members at share time**. Mechanism: grants are per-principal
records, so sharing enumerates the membership fold NOW and mints N grants
to those fingerprints. Future joiners were never granted — the snapshot
semantic is the *default behavior* of principal-scoped grants, not an
expiry mechanism. (Presence-based sharing was considered and not chosen.)

**Sharer-isn't-owner (RULED — auto-request the rest):** the closure splits
into `{domains the sharer can grant}` → minted at confirm, and `{domains
the sharer cannot}` → the dialog says so and sharing **auto-files access
requests** to those domains' grantors on each recipient's behalf (§5's
directed-request lifecycle, entered early — D11). One dialog, honest semantics.

## 4. The grant-request database (RULED: requests ARE pending grants)

**Shape + place:** an access request is a **CapabilityGrant-shaped record
with a `disposition` lifecycle, stored where the ACLs live — the target
share's own policy binding** (AZ §4: policy rides the share; replicates
exactly as far as the data). There is no separate request store.

**Identity provenance (D11):** the `requested` record's requester is the **B3
authenticated principal**, appended by the target authority — never a
caller-asserted `requester-fp` field. Filing a request is a **directed,
authenticated** act to that authority (§5); read access to a share does **not**
confer append access to its policy binding (the confused-deputy hole is killed).

**Disposition lifecycle (per grant-instance, append-only):**

```
requested ──▶ granted ──▶ revoked      (revoked is terminal PER INSTANCE)
     └──────▶ denied                    (terminal per instance)
```

- Every transition is a **new record**; the fold yields the current
  disposition per instance; **the chain IS the history** — free, complete,
  and auditable.
- **Approve↔revoke flipping (RULED)** is instance-mint, not mutation: the
  approver revokes instance N (revocation-wins, AZ §4 fold unchanged) and
  re-approval mints instance N+1 referencing the same (principal, resource).
  The revoked instance stays dead; the effective state is the fold across
  instances; history shows every flip with attribution. This preserves the
  ratified revocation-wins fold rule while giving the approver a
  change-my-mind UI.
- **Pending queue** = fold over `disposition: requested` in shares the
  viewer can grant. **History view** = the instances chain per
  (principal, resource).

## 5. The knock ceremony (late-comers + auto-filed requests)

1. A non-member clicks a link → mounts of its refs are denied (the gated
   stage) → glade-share's tap intercepts the denial *for link-carried refs* and
   offers "request access".
2. **Directed authenticated request (D11).** The knock is a **directed request
   to the relevant authority** — the grantor of the target share — NOT a write
   into whatever share the link was found in. **Read permission does NOT imply
   append permission** (the earlier "where you can read commons, you can append"
   was a confused-deputy hole — retracted). The requester's identity is the
   **B3 authenticated principal** (`requester-fp` comes from the caller context,
   **never caller-asserted** in the payload). The `AccessRequest{link-hash,
   target refs}` is delivered to the authority through a **durable offline
   queue**: an offline grantor ingests it on next attach — nothing is dropped.
3. **Ingestion:** the grantor's glade-share authority, on a directed request it
   has authority over, validates the B3 requester and appends the canonical
   `disposition: requested` record into the **target share's policy binding**
   (§4 — the database of record).
4. **Notification:** grantors' glade-share tap surfaces the pending fold —
   "bob requests access to workspace `razel` (via your link in #dev)" —
   approve / deny.
5. **Approve** = the s-grant beat exactly (already traced): grant record
   appends → fold flips at every hop → the requester's pending mounts
   unblock. Deny = disposition record; requester's UI reports it as data. A
   **wrong-principal** request (whose B3 identity holds no valid path to the
   closure) is **denied** — the knock test MUST exercise this arm (E-share-3).

This unifies all channels: chat sharing is the knock ceremony
**pre-approved for a membership snapshot**; a link pasted anywhere else
(email, elsewhere) has no enumerable membership, so every click knocks.

## 6. Surfaces

| glade id | shape | where | content |
| --- | --- | --- | --- |
| `share.create` | exchange | — | mint a share (commons + policy binding) — direct membership (E-share-1) |
| `share.invite` | exchange | — | offer membership: mint a `requested` pending grant (E-share-1) |
| `share.grant` | exchange | — | admit a principal: mint membership grant(s) (E-share-1) |
| `share.revoke` | exchange | — | cut membership: commons + private zone, revocation-wins (E-share-1) |
| `share.status` | exchange | — | read a share's membership + pending-request fold (E-share-1) |
| link records | (attachment record) | the carrying share | LinkRecord (content-addressed; v1 = portable commons + inline IDs, E-share-2) |
| policy binding entries | log | EACH target share (existing home of grants) | grant-instances incl. `disposition` lifecycle (§4) |
| access request | **directed exchange + durable queue** (D11) | to the target **authority** (queued if offline) | `AccessRequest{link-hash, target}`; requester = B3 principal, not caller-asserted |
| notifications | grip (client) | glade-share tap | pending-requests fold for grantors; request-status for requesters |

The direct-membership exchanges (`share.*`) are the core; links + the knock
**layer on top**, minting the same policy-binding grant records. The supplier
stays thin: closure computation, ceremony orchestration, the authenticated
request path, and the notification taps. Grant records are authority-appended
(H-R3); no new privileged store (GDL-038).

## 7. Stage split

- **Stage-1 (buildable now, as data):** the direct-membership exchanges
  (`share.create/invite/grant/revoke/status`) as records, capture/restore,
  LinkRecord, closure computation + confirm UI, membership-snapshot grant
  minting, the directed access-request records, ingestion, disposition
  lifecycle + history, notification taps. In allow-all nothing blocks, but
  every flow exercises; the request/grant/revoke **records** are unit-tested
  here (E-share-3).
- **Gated stage (first stage that actually enforces):** the deny that triggers
  the knock; grants actually gate; `share.revoke` actually cuts access; the
  directed request path becomes load-bearing. **The knock user-test lives here,
  honestly** (E-share-3): allow-all can never fire a denial-triggered knock, so
  the end-to-end knock test — including its **revocation** and **wrong-principal
  denial** arms — MUST run at this first gated stage.

## 8. Traces to author before building

- **s-share-create** — `share.create` mints a share (commons + policy binding);
  the creator holds membership; `share.status` reads exactly one member.
- **s-share-invite** — `share.invite` a principal → a `requested` pending grant
  → `share.grant` admits them → `share.status` shows both members; the admitted
  principal mounts the commons. (Direct membership, no link involved — E-share-1.)
- **s-share-revoke** — `share.revoke` a member → revocation-wins fold → their
  **commons grant AND private zone** are both cut; a later re-`share.grant`
  mints a fresh instance and re-admits (§4 instances).
- **s-link-share** — capture (**portable commons + inline IDs only**, private/
  account refs rejected at capture — E-share-2) → share into a chat → closure
  confirm → membership-snapshot grants minted → a member restores the link
  (mounts + atoms) → a LATER joiner does NOT hold the grants.
- **s-knock-directed** — late joiner clicks → denied → knock is a **directed,
  authenticated request to the target authority** (B3 identity; durable queue if
  the grantor is offline — D11) → ingestion → grantor notification → approve →
  fold flips → mounts unblock; plus a **deny** arm, a **wrong-principal denied**
  arm (E-share-3), and an approve→revoke→re-approve history arm (§4 instances).
- s-grant (existing) remains the underlying approval mechanics.

## 9. Dependencies + user-testable-when

- Depends on: **glade-users** (principals to grant to; the membership fold),
  the carrying suppliers (chat first), **glial** (tap interception, mount
  model — refs are instance identities), typed manifest (glade ids).
- **User-testable when:** I `share.create` a workspace share and `share.invite`
  a principal directly — they appear as a member and mount the commons (the
  normative direct path, E-share-1); AND I drag a link out of a live page, share
  it into a chat; the confirm dialog names exactly the domains being granted;
  current members click and land in restored page state; a user who joins the
  chat later clicks, knocks (a directed authenticated request), I get the
  notification, approve, and their page restores; I later revoke and their
  access ends (commons + private both cut); re-approving mints a new grant and
  works again.

## 10. Open questions (Gianni)

- **RESOLVED (E-share-1):** the normative "share this workspace with that
  principal" flow is the direct-membership ceremony the share family owns
  (`share.create/invite/grant/revoke/status`, §1/§6); links layer on top.
- **RESOLVED (E-share-2):** v1 link capture carries only portable commons
  grants + inline share/glade IDs; private/account refs are rejected at capture.
- **RESOLVED (D11):** the knock is a directed, authenticated request to the
  target authority with a **durable offline queue** — an offline grantor ingests
  on next attach, nothing is dropped; read access never implies append. (This
  supersedes the old "does ingestion require the grantor online" open.)
- **RESOLVED (E-share-3):** the knock user-test runs at the first gated stage
  and proves revocation + wrong-principal denial (§7).
- Request **expiry/withdrawal**: does a `requested` instance age out, and
  may the requester append a withdrawal? (Lean: withdrawal yes — it is just
  another disposition; expiry = a read-side TTL caveat like leases, no
  fold-time clock per the no-time-in-fold rule.)
- Approval **scope**: an approval covers that requester for that link's
  closure (lean, closure-scoped) vs the requester generally.
