# glade-diff — cross-surface diff, the first demand-instantiated supplier (supplier spec)

Status: full spec v1 (2026-07-12) — expands the `SupplierOutlines.md` entry;
demand-instantiation ruled **IN, not deferred** (2026-07-12); **D1–D5 RATIFIED
2026-07-12** (`GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md` §III — see
§10). Common contract: `dev-docs/glade/GladeSupplierModel.md` (§1 draws the
supplier / `DemandServiceDefinition` line — renamed by D1). Behavioral spec = the
atlas, cited step-by-step: **ggg-viz `s-diff-service`** (`scenario/services.ts`,
G1→G5) and **`s-svc-shared`** (`scenario/sync.ts`, discovery-before-spawn +
per-node variant). Ruling context: **WD-8** (`GladeWorkspaceDirectory` §8 —
ReplicaHint + ServiceInstanceClaim; `per-node` dedup default, `global` opt-in;
home-node repair). Open rows: **GDL-014** (placement/affinity) still open;
**GDL-016** (who may *define* a service) RESOLVED by D5; **GDL-026** (source
handles + view binding) RESOLVED by D2. grip-lab precedent: cross-surface diff
EXISTS there — parity, not invention.

## 1. What glade-diff composes (supplier vs instance)

glade-diff is the **first supplier whose composed artifact is a
`DemandServiceDefinition` record, not a standing served surface** — a
**structural, versioned** definition, not a provider label (**D1**). There is no
`ServeClaim`, no long-lived provider session. What an app composes:

1. **The `DemandServiceDefinition(diff)` record** — standing, composed,
   replicated in the home share (§2). This IS the supplier.
2. **The diff program** — the only bespoke code: fold two `ws.tree` snapshots →
   a diff result. Content-addressed so every node runs the same bytes;
   determinism makes per-node duplication *correct* (`s-svc-shared` SS3). Runs
   under the **D5 sandbox contract** (§2) — read-only declared inputs, no ambient
   authority.
3. **The UI-facing surface contract** — the `ws.diff` result shape + how a
   consumer forms the canonical compute key it subscribes (§3, §6). Identity is
   **two-level (D1):** a shared **compute instance** (keyed structurally, viewer
   identity excluded) vs a **per-viewer delivery binding** (§6).

An **instance** is the ephemeral other half (GladeSupplierModel §1's
supplier/`DemandServiceDefinition` line): one per compute key, it subscribes both
sources as an *ordinary session* (G3) under **exact read-only source
capabilities** (§5), folds + emits a **generation-bound** result (G4; state
`pending|ready|stale|absent|denied|error`, **D4**), holds a lease + refcount and
**no durable state** — spawn G2 → teardown G5 nulls everything. You COMPOSE the
definition; the node INSTANTIATES on demand. **Reuse of a warm instance never
implies authority reuse (D1)** — every viewer is authorized independently (§5).

## 2. The `DemandServiceDefinition` record + compute key + sandbox

Registered as an ordinary home-share record (GDL-037/038 — no privileged plane;
same posture as `BindingDecl`), now **versioned** and structural (**D1**).
**Defining a service and authorizing its execution are SEPARATE capabilities
(D5, resolving GDL-016):** appending the definition is one grant; a distinct
grant authorizes a node to *execute* it. Neither is a provider label.

```
DemandServiceDefinition {                              # versioned, structural (D1)
  name:    "diff"
  version: <def revision>
  match:   ws.diff whose key {left, right} both resolve to ws.tree-shaped surfaces
  program: <content-addressed diff compute>          # the only bespoke code
  sandbox: <sandbox-policy-version>                   # D5 record ref (below)
  emits:   { glade_id: ws.diff, shape: value }       # per-viewer delivery, zone private (§6)
  policy: {
    dedup:     per-node        # WD-8 default (recompute; determinism => correct)
    placement: consumer        # GDL-014 default; data-side for big sources
    recompute: debounced(250)  # per-op | debounced(ms) | on-demand   (G4)
    teardown:  grace(5000)     # immediate | grace(ms) | retained      (G5 / F3)
  }
}
```

**Compute key (D1).** The instance identity MUST be derived structurally from the
**definition/program digest + sandbox-policy version + ordered source
identities/revisions**. **Viewer identity MUST NOT be in the compute key** — that
lives at the delivery level (§6). This is exactly what lets a warm instance be
reused without reusing authority (§5).

**Sandbox policy (D5).** A **NEW versioned record, distinct from the legacy
`{app,name,glade_id}` shape**, composition-pinned until changed by signed
governance. It pins the execution contract: **read-only declared inputs; NO
ambient filesystem, network, clock, randomness, environment, or child-process;
bounded CPU / memory / output / wall-time; typed timeout/resource/policy
errors.** Its version rides the compute key above, so a policy change forks a new
instance rather than silently re-scoping a running one.

`.glade` surface (compiles to the record above, exactly as `binding` compiles to
`BindingDecl`) — the grammar gains a `servicedef` line beside `service`:

```
servicedef diff match(ws.tree, ws.tree) emits ws.diff sandbox=diff-sbx:1 \
           dedup=per-node placement=consumer recompute=debounced:250 teardown=grace:5000
```

The four **policy fields** are the spec-time opens the trace flags, each ruled as
definition data (not substrate):

- **dedup** `per-node | global` — the ruled WD-8 axis. `per-node` recomputes
  locally (`s-svc-shared` SS3: "locality beats sharing"); `global` discovers a
  running instance via its leased **monotonic-epoch** claim before spawning
  (SS2, §4; **D2**). Default `per-node`. Dedup is over the **compute** key; the
  per-viewer `private` delivery binding is authorized separately (§5, §6).
- **placement** `consumer | data | auto` — GDL-014 (still open). `consumer`-side
  default (G2: "data is small after diff"); `data`-side ships the compute to a
  peer "when sources are big"; `auto` = a size heuristic (open).
- **recompute** `per-op | debounced(ms) | on-demand` — G4 CONTENTIOUS. Default
  `debounced` for a live UI diff.
- **teardown** `immediate | grace(ms) | retained` — G5 CONTENTIOUS, "mirrors
  F3's retention, for compute." Default `grace` (a second viewer inside the
  window re-uses the warm instance); `retained` pairs with `global`. Zero-viewer
  → bounded grace before reclamation (**D2**).

## 3. The canonical compute key = the dedup key

Each subscribe names a derived binding by a `{left, right}` pair; that pair,
canonicalized and folded with the **definition/program digest + sandbox-policy
version (D1)**, IS the **compute** dedup key — `s-svc-shared` C1: "byte-identical
to ui1's ask on the other node." Viewer identity is excluded (it lives at the
per-viewer delivery level, §6). Normalization rules so dedup actually fires:

1. **Each side = `(share, glade_id, key-fill)`** — the full binding *instance
   identity* (the ref shape glade-share captures). The trace's sides are
   `(ws-razel, ws.tree, {root:"/"})` and `(ws-glade, ws.tree, {root:"/"})`.
2. **Key-fill canonicalization** — canonical CBOR over each key map, fields
   sorted (the home-share record encoding, `GladeWorkspaceDirectory` §6). So
   `{root:"/"}` encodes identically on every peer; field-order variants collapse.
3. **The pair is ORDERED, never sorted** — diff is non-commutative
   (`diff(A,B) ≠ diff(B,A)`: adds/removes swap). Opposite-order asks are
   *different* diffs, correctly two instances; sorting the pair would silently
   merge distinct results. The reverse-is-negation optimization is a later
   economics choice, not a key rule (§10).
4. **Handle = key** — the canonical compute key hashes to the instance identity
   (structurally derived, **D2**); the leased `ServiceInstanceClaim` is keyed by
   it (§4). GDL-026's "canonical handle" answered concretely: the key IS the
   handle binding view → provider instance; no separate route-key registry.

INV (**D2**): **at most one live compute instance per canonical key per dedup
domain** (per-node → per node; global → per home share, behind the leased
monotonic-epoch claim).

## 4. Where the derived binding lives — RATIFIED structural derivation (D2)

The trace parked `ws.diff` in a synthetic `svc` share and flagged it (G1 note,
GDL-026). **D2 ratifies structural derivation** (over registering derived
bindings in the directory): the instance key is **derived structurally from the
D1 inputs**, advertised by a leased claim.

- `svc` is a **reserved derived-binding namespace**, not a stored share: nothing
  authors it by ops, the *only* producer of `svc/ws.diff/{key}` is a matching
  `DemandServiceDefinition`, and the **absence of a `ServeClaim` is the G1
  trigger**. The binding's identity is purely structural: `(svc, ws.diff,
  compute-key)`.
- The only records in a *real* (home) share are the **`DemandServiceDefinition`**
  (durable) and, for `global` dedup, the leased **`ServiceInstanceClaim`**
  (ephemeral, self-GC'ing — `s-svc-shared` G6), now carrying a **monotonic
  epoch**: a global instance MAY exist only behind that leased claim, and a
  **race loser MUST tear down and MUST NOT publish output** (D2). Discovery stays
  a fold over the home share (WD-8); the derived *binding* is never an entry
  there.
- **Rationale:** registering every `(A,B)` anyone ever diffs would flood the
  directory with ephemeral per-view entries; it is for durable placement.
  Structural derivation keeps exactly one durable record (the definition) + one
  already-leased ephemeral record (the claim) — precisely the WD-8 shape.
- **Reclamation:** a zero-viewer instance enters a **bounded grace** before
  reclamation (D2); teardown is generation-bound so a stale replica cannot be
  served as current (§6, D4).

## 5. Whose capability the instance borrows — RATIFIED authorization (D3)

The trace flags it on every source leg (C3): "the service acts — with whose
capability? Its sponsor's, attenuated? The agent-on-behalf-of problem."
**D3 ruling: the service receives ONLY exact, read-only source capabilities** (a
per-instance derived agent principal attenuated to exactly the two source
bindings — read.subscribe only; GDL-004 delegated references; WD-2 agent shape):

- The instance reads *only* `left` and `right`, cannot write, cannot reach a
  third binding — it never exceeds what its sponsor holds.
- **Global dedup, one instance, N viewers** (`s-svc-shared`: gryth1 + gryth3):
  the warm instance is a shared *read amplifier*, **not** shared authority —
  **reuse never implies authority reuse (D1)**. Each viewer is admitted
  independently (below); the sources are subscribed **once**, under the exact
  read-only caps.
- **The leak guard (D3 — load-bearing, STAGE-1, RETRACTS the old direction):**
  every **subscribe / replay / cache-delivery / forward** MUST establish, for the
  requesting **B3 principal** (§I `ProviderCallContext`), **`can_read(left) &&
  can_read(right)`**. Reader-set form: **`Readers(diff) ⊆ Readers(left) ∩
  Readers(right)`** — only a principal who can read BOTH sources may read the
  diff.
  - **RETRACTED:** the earlier backwards relation `read(ws.diff) ⊇ read(left) ∪
    read(right)` (SR56-2-24) was exactly wrong — it would have let a left-only
    reader see the diff and infer `right`.
  - **Re-evaluated on grant / membership change:** an **already-computed
    artifact MUST NOT bypass a later denial** (revoke-midstream cuts delivery,
    §8).
- **Atlas invariant INV-7** — the four combinations a viewer can hold
  (**left-only / right-only / neither / both**): only *both* is served, the other
  three denied at their edge (`s-diff-authz`, §8).

## 6. Surfaces — two-level identity (D1) + generation state (D4)

Identity is **two-level (D1):** the shared **compute instance** (keyed by the D1
compute key, viewer-independent) produces a **generation**; each authorized
viewer gets a **per-viewer delivery binding** off that generation. Reuse of the
compute instance never reuses authority (§5).

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `DemandServiceDefinition(diff)` | record (home share, versioned) | commons (system/home) | the composed standing definition — match + `program` + `sandbox` + emits + policy (§2). NOT a served surface; a directory record. |
| compute instance (`svc`) | internal generation | — | the shared computed result, keyed by the **compute key** (def/program digest + sandbox-policy version + ordered sources; §3). Viewer-independent; carries **generation state** `pending\|ready\|stale\|absent\|denied\|error` (D4) + the service-def and source revisions it was built from. |
| `ws.diff` (per-viewer **delivery binding**) | **value** (see note) | private | the derived result `{ added, removed, renamed, files[] }` delivered to one authorized viewer off a compute generation (G4); no `ServeClaim`; **authorized per-principal at each subscribe/replay/cache-delivery (D3, §5)**. A cached `ready` generation is **revalidated before delivery** (D4). |
| `ServiceInstanceClaim` | record (home share, leased, monotonic-epoch) | commons (home) | ephemeral advertisement of a running instance, keyed by the compute key; the discovery fold for `global` dedup (WD-8; `s-svc-shared` G6; a race loser MUST NOT publish, D2). |
| sandbox policy | record (home share, versioned) | commons (system/home) | the **D5** execution contract (read-only inputs; no ambient authority; bounded resources), composition-pinned; its version rides the compute key (§2). Distinct from the legacy `{app,name,glade_id}` shape. |
| `ws.tree` (×2, `left`/`right`) | value | commons | NOT owned here — consumed as ordinary source subscriptions (glade-workspaces / glade-files own them). Listed for the input contract. |

**Generation states (D4).** `grant-lapse` / `source-change` / `worker-loss` /
`deterministic-program-failure` are **distinguishable typed outcomes**; a **stale
generation MUST NOT be relabelled current** (resolves the teardown-stale-replica
hole — a replicated `value` op from a torn-down instance is revalidated, never
served as current).

**Shape reconciliation (flag):** `grazel-app.glade:24` declares `ws.diff` as
`log ... from-cursor`, but the trace emits it as a **`value`** (A5 `shape:value`
— a latest-wins snapshot recompute *replaces*, G4) and the outline calls it "a
derived value surface." A diff is a current-state projection, so `value` is
right; fix the `.glade` to `value ... latest` before build (§10).

## 7. Stage split — security is STAGE-1 (not deferred)

The B-series rulings make authorization a **stage-1 must** (§VIII: "security
interfaces first"); **an allow-all authorization path MUST NOT satisfy a live
gate** (§VIII.5). So the old "stage-1 allow-all" is **RETRACTED**.

- **Stage-1 (single-node, per-node dedup — security ON).** The machinery *as
  data*: `DemandServiceDefinition` registration (§2); G1 routing miss → match →
  G2 spawn → G3 source subscribe → G4 compute + emit → recompute → G5 teardown at
  zero refs; compute-key dedup, `per-node`. **With, from day one:** the **B3
  `ProviderCallContext`** delivered beside the request (§I); the **D3 leak
  guard** — per-principal `can_read(left) && can_read(right)`, INV-7 (§5);
  **exact read-only source capabilities**; the **D5 sandbox contract** (no
  ambient fs/network/clock/randomness/env/child-process; bounded resources); **D4
  generation state**. **glade-diff is the DRIVER that builds base glade's
  demand-service machinery** (definition fold, spawn, refcount, teardown,
  instance-claim lease, sandbox host) — traced, not yet built
  (`SupplierRequirements` P3-tail). Publish the service-definition / instance /
  delivery / generation / sandbox **types before workers** (§III gates).
- **Stage-2 (scale-out).** `global` dedup with **leased monotonic-epoch** claim
  discovery (`s-svc-shared` G6; D2), race-loser teardown, cross-node refcount
  (SS2 F2), cross-node re-evaluation on grant/membership change; `data`/`auto`
  placement (GDL-014); signed-governance sandbox-policy version changes (D5).

## 8. Traces to author before building (the two core traces exist)

`s-diff-service` (G1→G5 happy path, per-node/consumer-side) and `s-svc-shared`
(global discovery + per-node variant) EXIST. Additional **arms** needed (the
authz + generation + sandbox arms are now **stage-1**, §7):

- **s-diff-authz** (INV-7; replaces `s-diff-denied` + `s-diff-multi-sponsor`) —
  one instance, viewers holding each of the **four** source combinations:
  **left-only / right-only / neither** denied at their edge; **both** served
  (`can_read(left) && can_read(right)`, §5). Covers the multi-principal case
  (`s-svc-shared` used one operator only).
- **s-diff-revoke-midstream** — a viewer's grant is revoked **between compute and
  delivery**: re-evaluation cuts the delivery; the **already-computed artifact
  MUST NOT bypass the denial** (D3, §5).
- **s-diff-generation** (subsumes `s-diff-source-lapse`) — a cached `ready`
  generation is **revalidated before delivery**; `source-change` → `stale`, a
  lapsed source `ServeClaim` → `absent`, `worker-loss` / deterministic
  `program-failure` as distinct typed outcomes; a **stale generation is never
  relabelled current** (D4, §6).
- **s-diff-sandbox-deny** — the program attempts an **undeclared network / fs**
  access (or blows a resource/wall-time bound) → a **typed policy error**, no
  ambient authority reached (D5, §2).
- **s-diff-teardown-grace** — G5 policy real: close → `grace` → a second viewer
  inside the window RE-USES the warm instance vs. lapse → zero-viewer
  reclamation (D2).
- **s-diff-recompute** — G4 policy: a burst of source ops under `debounced`
  coalesces to one recompute; `per-op` / `on-demand` arms.

INV-7 (§5, the four source combinations) + the D2 one-instance INV (§3: one
compute instance per compute key per dedup domain).

## 9. Dependencies + user-testable-when

- **Depends on:** glade-workspaces + glade-files (the `ws.tree` sources),
  glade-users (attribution; the requesting principal §5), and base glade's **B3
  `ProviderCallContext`** (§I — the principal the leak guard checks) + **sandbox
  host** (D5). **Forces** base glade's demand-service machinery — glade-diff is
  its build driver, not just a consumer. Working-tree diff *inside one* workspace
  stays a **glade-gwz** verb; glade-diff is strictly the cross-surface case.
- **User-testable when:** I pick two workspaces (possibly on two peers) and watch
  a live diff that updates when a source changes; **a viewer who can read only
  one side never sees the diff (INV-7, §5)**; revoking a source grant mid-view
  cuts the delivery; closing the panel tears the instance down; a second viewer
  of the same compute key dedups per policy (`per-node` recomputes identically;
  `global` shares the one instance, each viewer authorized independently).

## 10. Open questions (Gianni)

**RESOLVED — D1–D5 ratified 2026-07-12** (`GLP-0006` §III):

- **D1 — service identity/reuse:** `DemandServiceDefinition` is structural +
  versioned; two-level identity (compute key excludes viewer; per-viewer delivery
  is distinct) — resolves the old private-vs-dedup contradiction (§1/§3/§6).
  Reuse ≠ authority reuse.
- **D2 — location (was CONTENTIOUS #1, GDL-026):** structural derivation +
  reserved `svc` namespace + leased **monotonic-epoch** claim RATIFIED over
  directory registration; race-loser teardown; zero-viewer grace (§4).
- **D3 — authorization (was CONTENTIOUS #2):** the leak guard is FLIPPED to
  `Readers(diff) ⊆ Readers(left) ∩ Readers(right)` (per-principal
  `can_read(left) && can_read(right)`), re-evaluated on grant change; INV-7. The
  old `⊇`-union relation is RETRACTED (§5).
- **D4 — generation:** `pending|ready|stale|absent|denied|error`, revalidate a
  cached `ready` before delivery, typed distinct outcomes — resolves the
  teardown-stale-replica hole (§6).
- **D5 — sandbox / GDL-016:** a new versioned sandbox record (≠ legacy
  `{app,name,glade_id}`), composition-pinned; **defining ≠ authorizing
  execution** (separate capabilities); no ambient authority, bounded resources
  (§2).

**Still open:**

- **`ws.diff` shape:** confirm `value` (latest-wins) and fix `grazel-app.glade:24`
  from `log ... from-cursor` (§6).
- **Policy defaults + `auto` placement (GDL-014, still open):** are
  `debounced:250` / `grace:5000` the right defaults or must each definition
  choose; what size threshold flips consumer→data placement, and is it a field or
  a node judgment?
- **Reverse-is-negation (§3.3):** ever fold `diff(B,A)` onto `diff(A,B)` (present
  flipped), or always two instances? Lean: two (keep the key literal, ordered).
