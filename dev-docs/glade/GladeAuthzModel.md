# Glade Authorization Model — grants as data, checks as folds

Status: working draft — **design direction, not yet a contract**

Purpose: pin the authorization leg of the security model: how a user's access
to workspace features (read files, run shells, push/pull, build) is granted,
carried, evaluated, and revoked — with **minimal impact on the glade
substrate** and no online authority on any hot path. This instantiates the
capability track of `glade/dev-docs/GladeGrythSecurityModelAnalysisPrompt.md`
(§3.3/§3.4, constraints §4), builds on `GladeWorkspaceDirectory.md` (§2 record
kinds, §5 grants) and `glade/dev-docs/GladeSubstrateV1.md` (§11 retrofit
seams). Touches GDL-004 (delegated references), GDL-007 (declaration
validation), GDL-009 (revocation), GDL-016 (provisioning authority).

Authentication (who is asking) is the directory design's job: device certs
chained to the user root from the genesis ceremony. This document is about
**authorization** (what they may do) and deliberately keeps the two separate.

## 1. The decomposition: reads and effects enforce differently

| Class | Wire | Executor | Enforcement point | Must work offline? |
| --- | --- | --- | --- | --- |
| Reads | `SUBSCRIBE` / replicate / fan-out (`OPS`) | any replica of the stream | **every serving hop** — including a local node fanning out cached data to a second user | **yes** — a local node must decide alone |
| Writes | `APPEND` | the receiving replica (converges by fold) | the accepting hop + every folder (validity-filter-before-fold) | yes |
| Effects | `EXCHANGE` (`gwz.*`, `shell.exec`, `git.push/pull`, `razel.build`) | the claim-holder only | **the authority, at execution time** | no — the executor is by definition present |

Two consequences. First: read checks MUST be evaluable locally from replicated
state — an online policy oracle would put the authority on the read hot path
and break local-first. Second: effect granularity is cheap — the enforcer is
always present when an effect happens, so arbitrarily fine (even
machine-local) policy costs nothing to distribute.

**The issuance/decision split:** the workspace authority *doles out grants*
(signed records, ahead of time); it MUST NOT dole out *decisions* (per-request
verdicts). Decisions are local, everywhere, always.

## 2. Not JWT — and what survives of it

JWT-with-claims fails four ways here: (1) **no offline attenuation** — agents
and service instances need narrower-than-sponsor rights derivable without an
online issuer; (2) **wrong verification topology** — one audience/one verifier
vs our replicated multi-hop serving; (3) **revocation** — `jti` blacklists are
a central live list, the exact anti-pattern; (4) **encoding** — taut-everywhere
means canonical CBOR with Rust/TS/Python parity, not JOSE base64-JSON.

What survives: the *signed-statement-of-claims* primitive, upgraded to a
**capability chain** (UCAN/biscuit-shaped semantics, our own taut encoding):
each link is signed by the parent principal, names a subject key, and may only
**narrow** scope. JWT keeps exactly one job: at the **edge**, an OIDC/JWT from
an external IdP MAY serve as *authentication input* to the pluggable trust
backend (bind a human to a principal at `HELLO`). It never travels inside
glade as authorization.

## 3. The grant record

Taut-declared, canonical CBOR, ed25519 — the same signature substrate as
device certs. Sketch (schema pinned at build time, not here):

| Field | Content |
| --- | --- |
| `issuer` | principal key + its chain to a root the resource trusts (owner chain) |
| `subject` | grantee principal key (user root, device, agent, or service instance) |
| `resource` | `(workspace/share, binding-class)` — key/path scoping deferred (§8) |
| `verbs` | verb-set (§5), wildcards allowed (`read.*`) |
| `caveats` | optional: TTL, attenuation-only markers |
| `parent` | for delegation links: hash of the parent grant this narrows |
| `sig` | issuer signature over the canonical form |

**Attenuation rule (MUST):** a link's `(resource, verbs, caveats)` ⊆ its
parent's. A chain is valid iff every link verifies and every step narrows.
This is how "agent X may read A,B and run `gwz.status` on C — and nothing
else" is expressed: user root → agent key, two verbs, TTL.

## 3a. Ownership and administration

**Creation mints the root — not a grant.** Creating a resource (node,
workspace, org) makes the creator the root of that resource's authority
chain. There is no record making the creator admin, and **what has no record
cannot be revoked** — the "bad actor cannot remove the creator" rule holds by
construction, not by policy exception.

- **Admin is a verb-set**: `admin.grant`, `admin.revoke`, `admin.config`,
  `admin.delegate` (the meta-right to mint further admin chains — in-band,
  like every other verb). "Owner admin" = a chain from the resource root
  carrying full `admin.*` including `admin.delegate`.
- **The anti-coup rule — revocation follows issuance ancestry.** A revocation
  of an admin chain is valid iff its signer is an ancestor of that chain (or
  the root). The root can remove any admin; an admin can remove only their
  own delegation subtree; **siblings cannot remove each other**; nobody can
  remove the root. An out-of-ancestry revocation is an invalid op: every fold
  ignores it and it is preserved as attributable **evidence** of the attempt.
- **Governance is hierarchical; access management is cooperative.**
  `admin.revoke` over *user-level grants* works regardless of issuer (any
  admin can remove a regular user — reversible in one op, attributable, and
  the acting admin answers to their ancestor). *Admin chains* follow ancestry
  strictly. The asymmetry is justified by recoverability: a wrongly-removed
  user is re-granted; a decapitated governance chain is a coup.
- **Revoking an admin severs their subtree** — chains they issued die with
  them. Delegation depth is therefore a real decision, not bookkeeping.
- **Quorum is opt-in, declared at creation**: a governance policy record MAY
  state "M-of-N owner admins may revoke any non-root chain" for resources
  that want co-equal admins. The default stays ancestry (no admin wars).
- **Honest edges**: a lost root key plus a rogue admin has no in-band answer
  — root custody/recovery (WD-1) is the real mitigation, quorum the opt-in
  fallback. For nodes, physical control trumps records (the hardware holder
  can wipe and re-key; chains govern the network's recognition, not the
  silicon). For shares, the fork escape always exists (copy ops you hold into
  a new share with new policy; the old identity stays behind).
- **Co-signed verbs (approval-in-the-loop).** A delegation caveat MAY require
  a per-op co-signature: `verb: requires cosign(<principal|role>)`. The
  enforcement point holds the op, publishes a pending-approval record, and
  executes only when the co-signer's approval op lands. Motivating case:
  `term.write` for agents — writing to a live terminal is exec-equivalent
  (`term.write` ≈ `shell.exec`; the taxonomy treats them as one danger
  class), so an agent's terminal writes can be granted as
  "cosign(sponsor)" — the human approves each command while `term.read`
  streams freely. Generalizes to `git.push` on protected branches and to
  admin actions.

Every admin action is a signed op in the share: the audit log is the storage,
and coup attempts are permanent, attributable data.

## 4. The pivotal placement rule: policy rides the share

Grant and revocation records for a workspace live **in the workspace's own
share** (a policy binding), and therefore replicate exactly as far as the data
does. Closure property (MUST): **any node holding a share's ops also holds the
current grant fold for that share.** Fan-out checks never need the network.

- Fold semantics: set-union, **revocation-wins** (tombstone precedence), the
  directory doc's rules unchanged: validity-filter-before-fold, no time in the
  fold (TTL caveats evaluate read-side).
- Revocation is **forward-only** (GDL-009, stated honestly): new flow stops at
  every hop as the revocation op replicates; already-replicated history is not
  clawed back.
- The home share stays the *trust* substrate (device certs, principals); ​per-
  workspace policy deliberately does NOT centralize there — a workspace shared
  to me carries its own policy with it.

## 4a. Zones refinement (reconciled 2026-07-06)

`glade/dev-docs/GladeZones.md` (implemented 2026-06-14) sharpens the grant
unit: **grants gate commons-zone joins** — "share this document" = "you may
join `(domain, commons)`" — while **private zones need no grant at all**:
they are private by *keying* (`self:<user>`), not permission, with the honest
caveat that keying is routing-privacy (holds on operator-trusted nodes;
untrusted hops for private zones are exactly the AZ-10 encryption tier).
*Sharing is a grant; privacy is a key.* The read-grant `resource` of §3
canonically names `(domain, commons)`; the D8 refinement (each zone is its
own contiguous chain) is what makes private zones filterable from what a
peer receives without breaking chain verification.

Two rulings (2026-07-10) close the INV-4 questions this raised: **AZ-16** —
the private-zone serve rides the membership grant (no zone-scoped grant
exists; revoking membership cuts commons and private together); **AZ-17** —
account domains serve their OWNER by identity with no grant record
(owner-scoped, not blanket: non-owner access stays gated). One principle:
grants gate access to *other people's* replicated worlds; identity alone
gates your own.

## 5. Verb taxonomy

Verbs name glade's units, not app features. Wire verbs: `read.subscribe`,
`read.window`, `write.append`. Exchange verbs are namespaced by provider:
`gwz.status`, `gwz.mutate`, `gwz.create`, `git.push`, `git.pull`,
`shell.exec`, `razel.build`, `razel.run`. Verb-sets support prefix wildcards
(`read.*`, `gwz.*`). New providers mint new namespaces — the taxonomy is open,
the *shape* is fixed.

## 6. The check function — one pure function, everywhere

```
check(principal_chain, grant_fold, resource(share, glade_id, key), verb)
  → allow | deny(reason)
```

- **Pure** (no clock inside beyond a read-side `now` argument for TTLs, no
  I/O): corpus-testable with golden vectors, byte-identical verdicts across
  Rust/TS/Python — the fold discipline applied to authz.
- Evaluated at the substrate's **existing** enforcement seams (SubstrateV1
  §11 shipped them as no-op hooks): `HELLO` (authn + principal bind),
  `SUBSCRIBE` (read), fan-out serve (the cached-≠-allowed point), `APPEND`
  (write, plus every folder re-checks validity), `EXCHANGE` dispatch at the
  authority (effect verbs), and **re-evaluation when the grant fold changes**
  (live streams cut on revocation).
- The envelope's `capability_ref` slot carries the hash of the chain the
  sender claims; verifiers resolve it against their own fold — a *reference*,
  never a bearer secret.

**The authority overlay (MAY):** the claim-holder may apply stricter
machine-local policy on top of `check()` for effects on its own hardware
("no remote `shell.exec` on this box, granted or not"). Overlays only ever
narrow; they never widen.

## 7. Multi-user nodes and the honest limits

A local node serving multiple users runs the same check per session at
fan-out — one replica, N sessions, per-session verdicts (the s-sec-fanout
trace). Node↔node replication is governed by node trust (device certs), not
user grants; therefore, stated plainly:

- **A compromised node leaks what it legitimately replicated.** Enforcement
  guards sessions, not stolen disks. Mitigations, in order: replicate a share
  only to nodes with at least one granted user (policy, cheap); per-share
  payload encryption (a separate, later decision — analyzed independently per
  the security prompt §8).
- **Revocation cannot reach offline caches** (GDL-009) — the UI says so
  rather than pretending otherwise.
- Verb granularity below binding level (per-path file reads) is **deferred**:
  binding-granularity first; path scoping arrives later as a caveat type, and
  its cost (key-pattern matching in the hot path) gets measured, not assumed.

## 7a. Operators, placement, and node trust

"Local node" vs "backend node" is not an architectural distinction — the
protocol participant is identical. It is a **trust binding**: every node key
chains to an **operator principal** (personal nodes → the user's root; a DC
fleet → the operator org's root), and node trust is a relation, not a boolean:

> A node is an acceptable home for a share's plaintext **iff the share's
> policy accepts the node's operator.**

- **Placement is a granted verb.** `replica.hold` (and `session.host` for
  session admission) are grants to *operator principals*, carried in the same
  policy binding as user grants, replicated the same way. Owner-operated
  nodes hold their owner's shares implicitly (AZ-8 pins the default); any
  other placement is an explicit record.
- **The exposure calculus is directional.** Serving *my* data to a guest
  through *my* node adds no exposure (the plaintext was already on my
  hardware). What must never happen silently is the reverse: the guest's
  private share replicating onto hardware whose operator they never accepted.
  Cross-user serving on personal nodes is therefore fine; cross-user
  *placement* requires the grant.
- **Three sensitivity tiers per share:** operator-accepted plaintext (the
  default — folds, windows, services all work) → blind relay/E2E (owner-keyed
  payload encryption; nodes route and store ciphertext; node-side compute is
  forfeited) → and the gap between them (confidential compute) is named, not
  built.
- **Why the DC node is cheap:** many users accepting ONE operator is what
  permits colocation of their shares on shared hardware. The personal node is
  the degenerate case (operator = the one user). The "one node per user"
  spectre only haunts the E2E tier.
- **Shared local services (MCP daemons, assistants) are NOT "other users".**
  They are distinct principals whose rights arrive by attenuation chains from
  their sponsor(s) (§3) — one daemon serving two humans carries two chains. A
  free-standing "user" identity for an agent would bypass attenuation, orphan
  revocation, and wreck attribution.

Integrity never depended on node trust in the first place: per-origin hash
chains + op signatures make ops self-verifying end-to-end (GQ-9), so an
untrusted node in the path can withhold (DoS) or observe what it stores —
never forge. Operator trust governs **confidentiality and placement** only.

## 7b. Roaming and authentication binding

Roaming (the café case) composes existing pieces: web bootstrap to an **entry
node** (DNS + TLS, before glade starts) → HELLO admission → directory from the
operator-held home-share replica → ServeClaim routing → iroh dial to the
personal node. Interactive authentication happens ONCE, at the entry node;
every other hop verifies the session's principal/capability chain from its own
fold — *you log in at the door you walked through; every other door checks
your papers, not your face.*

Two session strengths, and the difference MUST be policy-visible:

- **Key-signed session** (your device, your cert): chains to your root; the
  operator never touches identity. Full cryptographic self.
- **Operator-vouched session** (borrowed browser, IdP/recovery bind): no
  device key exists, so downstream trust in the binding = trust in the
  operator's honesty. Therefore: (1) **the set of acceptable authn methods is
  the USER'S data** — a record in the user's home share, not operator config —
  else an operator's trust plug could bind anyone to your principal; (2)
  vouched sessions SHOULD be attenuated by that same policy (e.g. read-only,
  sensitive shares barred) — a bounded guest-of-yourself.

Warm-vs-blind is the placement rule felt at the café: if the workspace share
grants `replica.hold` to the entry node's operator, the entry node is a warm
replica and reads are local; if not, it degrades to a relay and every read
rides to the personal node — slower, by the owner's own policy, visibly.

## 8. Minimal impact on glade — the whole diff

1. **Two record kinds + fold rule** (`CapabilityGrant`, `CapabilityRevocation`
   with revocation-wins) — ordinary ops in shares; no new replication
   machinery.
2. **One pure function** (`check()`) + its golden-vector corpus (Rust/TS
   parity gate, like the fold oracles).
3. **Wire the existing seams**: principal at `HELLO`, `capability_ref` slot,
   per-frame hooks flip from no-op to `check()`; re-evaluate on fold change.

Zero new frames. Zero new server roles. Zero online dependencies. Allow-all
remains a mode (stage 1) — the hooks return allow when no policy binding
exists, which is also the migration story.

## 9. Open questions

| # | Question | Owner |
| --- | --- | --- |
| AZ-1 | Path/key-scoped read grants: required for v1 or caveat-type later? (cost: pattern-match on canonical keys in the serve path) | Gianni (product) |
| AZ-2 | Multi-user local nodes: day-one requirement or door-open? (decides how much principal store HELLO needs now) | Gianni (product) |
| AZ-3 | OIDC/AD bridge in v1, or single-user + manual grants first? (the trust-plug interface exists either way) | Gianni (product) |
| AZ-4 | Directory (home share) visibility to guest principals — what does a grantee of one workspace see of the list? | design (with AZ-2) |
| AZ-5 | Service-instance sponsorship default: instantiating-user's chain (assumed in s-diff-service) vs workspace-owner's | security analysis |
| AZ-6 | Grant issuance UX: who besides the owner may grant (admin verb `grant.issue` as just another verb?) | design (GDL-016) |
| AZ-7 | Operator/org root custody and rotation (the DC fleet's chain root) — mirrors WD-1 at org scale | Gianni + design |
| AZ-8 | Default placement: is owner-operator `replica.hold` implicit, and what is the min-replica declaration shape (fault tolerance as policy)? | design |
| AZ-9 | Authn-method policy record shape + default attenuation for operator-vouched sessions | security analysis |
| AZ-10 | Blind-relay tier semantics: store-and-forward ciphertext vs pure pipe; what survives of resume/heads | design (later) |
| AZ-11 | Policy-record schema versioning: bootstrap-model rules (current-validates-next, feature negotiation) + FAIL-CLOSED on unparseable enforcement records; fleet rollout sequences readers before writers | design |
| AZ-12 | ~~Checkpoint trust~~ **RULED 2026-07-05**: origin-signed checkpoints, with the checkpoint head ALSO appended to the home share — rewrite becomes fold-detectable (countersigning-by-replication, no new protocol). Explicit countersigning deferred until a threat model demands it. | ruled |
| AZ-13 | Quorum governance record shape (M-of-N over non-root chains) and whether root HANDOVER (transferring rootship itself) ever exists | Gianni (product) |
| AZ-14 | Co-sign caveat mechanics: pending-approval record lifetime, approval UX surface, offline sponsor behavior | design |
| AZ-15 | Agent enrollment ceremony: keypair minted at install, sponsor approves the pubkey (the add-a-device ceremony, for agents) — UX + storage of agent keys | design (with AZ-9) |
| AZ-16 | Private zones vs INV-4. **RULED 2026-07-10**: a private-zone serve is authorized by the receiver's `(domain, commons)` MEMBERSHIP grant — no zone-scoped grant ever exists ("joining a doc auto-grants your own private zone" means membership IS the entitlement). Which zone you receive is routing (keying), not policy. Consequence: revoking membership cuts commons AND private in one act — offboarding is one clean cut (forward-only caveat unchanged); your private-in-someone-else's-domain data is tenant, not freehold. | ruled |
| AZ-17 | Account-domain custody vs INV-4. **RULED 2026-07-10**: owner-scoped carve-out — the owning principal reads/writes their own account domain by IDENTITY, no grant record (no self-grant to mint at bootstrap, lag behind replication, or be revoked: self-lockout is unrepresentable). Any NON-owner access to an account domain stays grant-gated as usual (support/admin views need a grant). NOT a blanket exemption. | ruled |

## 10. ggg-viz mapping (the executable side of this document)

Stage-2 traces demonstrate each section: `s-grant` (issuance ceremony — §3/§4:
denied → owner appends grant → fold flips at every hop → served from cache),
`s-agent` (§3 attenuation: sponsored chain, in-scope verbs flow, out-of-scope
`shell.exec` denied at the authority), `s-verbs` (§5/§6: same user,
`gwz.status`/`git.pull` allowed, `shell.exec` denied + authority overlay),
`s-idp` (§2 edge: OIDC binds the principal; authn ≠ authz), plus the earlier
`s-sec-fanout` (§7 cached ≠ allowed) and `s-sec-revoke` (§4 forward-only).
The suite's invariant INV-4 enforces §4's closure mechanically: in stage-2
traces, no serving `OPS` reaches a client/service session unless the sender's
folded state holds a matching grant at that step.

§7a/§7b land as `s-roam` (the café ladder: entry-node HELLO, directory at the
operator replica, warm-vs-blind placement, and the vouched-session
attenuation), `s-tenant` (two user roots colocated on one DC node — works
because both accepted the operator), and `s-local-guest` (the directional
exposure rule: my data flows to a guest through my node; the guest's private
share refuses placement on my hardware). INV-5 enforces placement: a node may
carry `replica <share>` state only when its `hold <share>` entry names its
operator.
