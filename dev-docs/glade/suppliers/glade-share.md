# glade-share — links, sharing ceremonies, and the grant-request lifecycle (supplier spec)

Status: full spec v1 (2026-07-12) — expands the `SupplierOutlines.md` entry;
link model + the four rulings from Gianni 2026-07-12 (membership snapshot ·
auto-request-the-rest · link values are ID types, developer's discipline ·
requests = pending grants, same shape + same place, with flippable
disposition history). Common contract: `dev-docs/glade/GladeSupplierModel.md`.
Context: AZ §4/§4a (policy rides the share; AZ-16 membership semantics),
s-grant (the approval beat, already traced), GDL-004/030 (the deferred LIVE
context-sharing this is the frozen-slice sibling of), grip-lab's drag/drop
grip:value links (the precursor).

## 1. The link model

A **link** is a restorable slice of page state. In grip-lab it was all
values (`{grip → value}` restored at atom taps). In glade it splits — and
the split is the design:

| entry kind | carries | restore | access implications |
| --- | --- | --- | --- |
| **refs** | binding **instance identities** `(glade id, domain/zone/key fill)` | glial mounts them | THE access problem — the value lives in a share |
| **values** | inline `(grip → value)` pairs for local-only atoms | set at atom taps (the grip-lab move) | none |

Capture needs no new introspection: every mounted `GlialTap` already knows
its `(decl, fill)` — capture = walk the shared context's mounts, emit the
instance identities + the local atom values. Restore = mount refs, set atoms.

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
dedicated link-share exists. This choice powers the knock rendezvous (§5).

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
lifecycle, entered early). One dialog, honest semantics.

## 4. The grant-request database (RULED: requests ARE pending grants)

**Shape + place:** an access request is a **CapabilityGrant-shaped record
with a `disposition` lifecycle, stored where the ACLs live — the target
share's own policy binding** (AZ §4: policy rides the share; replicates
exactly as far as the data). There is no separate request store.

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

1. A non-member clicks a link → mounts of its refs are denied (stage-2) →
   glade-share's tap intercepts the denial *for link-carried refs* and
   offers "request access".
2. **Rendezvous:** the requester cannot write the target's policy binding —
   but can ALWAYS write the share where the link was found (you can only
   click where you can read; where you can read commons, you can append).
   The `AccessRequest{requester-fp, link-hash, domains}` rides the
   **carrying share**.
3. **Ingestion:** any grantor's glade-share tap, on seeing a rendezvous
   request it has authority over, appends the canonical
   `disposition: requested` record into the **target share's policy
   binding** (§4 — the database of record). The rendezvous copy is just the
   knock.
4. **Notification:** grantors' glade-share tap surfaces the pending fold —
   "bob requests access to workspace `razel` (via your link in #dev)" —
   approve / deny.
5. **Approve** = the s-grant beat exactly (already traced): grant record
   appends → fold flips at every hop → the requester's pending mounts
   unblock. Deny = disposition record; requester's UI reports it as data.

This unifies all channels: chat sharing is the knock ceremony
**pre-approved for a membership snapshot**; a link pasted anywhere else
(email, elsewhere) has no enumerable membership, so every click knocks.

## 6. Surfaces

| glade id | shape | where | content |
| --- | --- | --- | --- |
| link records | (attachment record) | the carrying share | LinkRecord (content-addressed) |
| policy binding entries | log | EACH target share (existing home of grants) | grant-instances incl. `disposition` lifecycle (§4) |
| `share.requests` (rendezvous) | log | the carrying share | AccessRequest knocks (pre-ingestion) |
| notifications | grip (client) | glade-share tap | pending-requests fold for grantors; request-status for requesters |

The supplier is thin by design: closure computation, ceremony orchestration,
and the notification taps. The records are ordinary appends in shares that
already exist (GDL-038 — no privileged plane, no new store).

## 7. Stage split

- **Stage-1 (buildable now, as data):** capture/restore, LinkRecord,
  closure computation + confirm UI, membership-snapshot grant minting,
  rendezvous requests, ingestion, disposition lifecycle + history,
  notification taps. In allow-all nothing blocks, but every flow exercises.
- **Stage-2:** the deny that triggers the knock; grants actually gate;
  request auto-filing becomes load-bearing rather than advisory.

## 8. Traces to author before building

- **s-link-share** — capture (refs+values) → share into a chat → closure
  confirm → membership-snapshot grants minted → a member restores the link
  (mounts + atoms) → a LATER joiner does NOT hold the grants.
- **s-link-knock** — late joiner clicks → denied → knock rides the carrying
  share → ingestion → grantor notification → approve → fold flips → mounts
  unblock (s-grant's beat with the request/notification framing); plus a
  deny arm and an approve→revoke→re-approve history arm (§4 instances).
- s-grant (existing) remains the underlying approval mechanics.

## 9. Dependencies + user-testable-when

- Depends on: **glade-users** (principals to grant to; the membership fold),
  the carrying suppliers (chat first), **glial** (tap interception, mount
  model — refs are instance identities), typed manifest (glade ids).
- **User-testable when:** I drag a link out of a live page, share it into a
  chat; the confirm dialog names exactly the domains being granted; current
  members click and land in restored page state; a user who joins the chat
  later clicks, knocks, I get the notification, approve, and their page
  restores; I later revoke from the history view and their access ends;
  re-approving mints a new grant and works again.

## 10. Open questions (Gianni)

- Request **expiry/withdrawal**: does a `requested` instance age out, and
  may the requester append a withdrawal? (Lean: withdrawal yes — it is just
  another disposition; expiry = a read-side TTL caveat like leases, no
  fold-time clock per the no-time-in-fold rule.)
- Approval **scope**: an approval covers that requester for that link's
  closure (lean, closure-scoped) vs the requester generally.
- Whether the ingestion step (§5.3) requires the grantor tap to be online
  (it does — offline grantors ingest on next attach; acceptable?) or wants
  a node-side relay later.
