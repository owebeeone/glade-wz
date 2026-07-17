# glade-files — file tree, windowed reads, and the blob strategy (supplier spec)

Status: full spec v1 (2026-07-12); **rulings landed 2026-07-12** from GLP-0006
`RulingWorksheet.md` §IV (D6/D7/D8/D12/D13/D14/F-GAP10) + §I (B3/B4), which
RATIFIED the window, blob, save, storage-truth, path, retention, and attribution
contracts this spec had proposed. Read-first viewing; the **window contract (D8)**
and the **blob strategy (D6)** were the two forcing contracts — now ruled. Common
supplier contract: `dev-docs/glade/GladeSupplierModel.md`. Grounded in: **s-window**
(ggg-viz `scenario/window.ts` — the behavioral spec for windowed loads),
GladeSubstrateV1 §3 (shapes + retention) · §6 (strict-priority scheduler, built) ·
§7 (reassembler / interest regions), GladeDeclSurface §Contents (`window` is
already in the Shape enum), the **P3-gate** (blob strategy) + Plan P3.S1/S2/S3,
GladeAuthzModel §2/§6/§8 (AZ-1 path-scoped grants — deferred), grazel-app.glade
(`ws.tree` / `ws.files` already declared), GAP-10 (retention). glade-diff
(cross-surface diff) consumes `ws.tree`-shaped surfaces — REFERENCED here,
specced in parallel; this spec does not define it.

Ruled judgment calls are tagged **(RULED — <id>)** inline and closed in §10; any
residual **(PROPOSED)** is impl-level, not a design choice.

## 1. Posture — read-first, app-owned tree

1. **Viewing is stage-1; write GATING is stage-2** (the write CONTRACT — D12
   compare-and-replace — is defined now, §4, and exercised via glade-editing's
   save). glade-files serves the selected
   workspace's real file tree and file contents as declared surfaces; it never
   spawns processes (that is glade-terminal) and never computes derivations
   (that is glade-diff). It is the first APP-OWNED-STORAGE read consumer
   (`SupplierRequirements.md` glade-files; GladeSupplierModel §5).
2. **The file↔surface mapping stays grazel's** (the data seam, GladeSupplierModel
   §5): path roots come from grazel config; a subscription key names a path
   WITHIN the workspace, never an absolute host path, and a path is never
   derivable from a request. Storage engine (files-for-now / SQLite-later,
   GDL-036) is supplier-internal.
3. **Genericity:** depends on base glade (window + blob carrier + scheduler),
   **glade-workspaces** (selection → which tree; root mapping) and **glade-users**
   (read attribution via the B3 `ProviderCallContext`, §4/§7) only — never on
   grazel. grazel COMPOSES it; any app could.

## 2. The window contract (RULED — D8: full mutable typed window, v1)

Ground truth is **s-window** (`window.ts`): a viewer names a window
`{from:3000, len:100}` over a 41,200-line `build.log` and paints it in ONE
round-trip while the rest backfills. **D8 rules the FULL mutable typed window for
v1 — not a logs-only substitute** — and assigns the previously-unowned reassembler:

1. **Window identity is `{workspace_id, path, revision}` (RULED — D8).** The
   byte/line range `{from, len}` is an INTEREST over that identity, NOT a new
   binding. Routing keys on the identity; the range only refines interest. (This
   supersedes the old "`key = {path, from, len}`" framing — the revision now rides
   identity, so a window always names WHICH generation it reads.)
2. **Viewport preempts backfill** via the strict-priority scheduler that already
   exists (SubstrateV1 §6; s-window W2): the window ships at INTERACTIVE priority,
   the full body trails at BULK (chunked, size-capped). **First-paint latency =
   one window round-trip, independent of file size** — the outline's promise.
3. **Backfill makes scrolling local** (s-window W3/B9): the body replicates
   BEHIND the window so the next window ask never leaves the machine; head-of-line
   blocking is the enemy this design was built against.
4. **Ownership split (RULED — D8):** **base glade** owns window request / routing /
   generation; **glial** owns REASSEMBLY (this is the assignment — the reassembler
   was unowned); **glade-files** owns the authoritative snapshot. The reassembler
   serves ONE coherent generation, tagged by revision — a window that would splice
   two generations is held or re-driven, **never delivered mixed**: a consumer MUST
   NOT observe mixed generations. This closes the old "Shape or projection" open —
   the window is one uniform typed contract and the reassembler FILLS it
   (append-only surfaces → a from-cursor range of the causal log; a mutable
   `ws.files` → an interest region over the reassembled, generation-stamped
   snapshot; §7 `ReassemblerTap`: same-region viewers collapse to one key).
5. **Window unit (RULED — D8):** line-based `{from, len}` for text, byte-offset for
   binary — the range is an interest over identity, owned by the surface's shape
   contract, not the supplier. An edit advances the revision/generation; the reader
   re-drives its interest against the new generation cursor-stably (never a splice).

## 3. The blob strategy — one fetch EXCHANGE, delivery-time authz (RULED — D6)

The standing rule (SupplierOutlines; Plan P3.S2): large binaries must NEVER ride
the fold as ops-in-chains. **D6 rules the fetch as ONE declared exchange authorized
at delivery** — killing the bare-hash bearer-token shape and the "is
`window/exchange` one shape or two" error (it is ONE exchange, distinct from the §2
window). Modeled on iroh-docs (GLRustiesP2PStory §D) + the plane split
(GladeScaleModes: blobs/chunks are DATA-plane, records are control-plane):

1. **A blob rides as a content-addressed REFERENCE in an ordinary record.** A
   `ws.tree` entry / `ws.files` metadata record carries `BlobRef {hash: BLAKE3,
   size, media_type}` — a small, verifiable, dedupable value on the fold plane.
   The record replicates everywhere cheaply, exactly like any other op.
2. **`ws.blob.fetch` is an EXCHANGE (RULED — D6).** The request NAMES a workspace
   binding or canonical path + revision + `BlobRef`; the response ships over a
   **bounded carrier/stream channel** — NOT a bare hash-addressed carrier op.
   Native nodes: iroh-blobs; browser: a Chunk-frame channel over the session — the
   specific transport is an impl choice UNDER the bounded-carrier contract, not a
   surface question.
3. **Authorization at DELIVERY time (RULED — D6).** The provider RE-RESOLVES the
   workspace and authorizes the fetch against the B3 principal (§4) at delivery. A
   content hash proves **integrity, not authority** — possession of a hash grants
   nothing, so a revoked or unauthorized principal is DENIED at delivery even while
   holding the hash. This is what kills the bearer-token shape.
4. **Bytes follow interest + dedupe:** a BlobRef replicating to a node does NOT
   drag its bytes; only an interested, authorized viewer fetches them (lazy), and a
   second authorized viewer of the same hash dedupes for free (content-addressing).
   The blob analogue of "backfill trails the viewport."
5. **Bring-up parity:** the conformance ladder already exists — "blob impl ≡
   future fold impl" (GladeProgramStatus Lane R.1). A blob store now, fold-parity
   later, is a supplier-internal swap behind the ref.

## 4. Writes, storage truth, paths, and read authz (RULED — D12/D13/D14/D7 + B3/B4)

The write ceremony, the at-rest truth, the path type, and the tree read-authz are
all ruled. The write CONTRACT is defined now and exercised via editing's save;
AZ-1 grant GATING stays stage-2 (§7):

1. **`files.write`/`replace` is the coarse-write authority — compare-and-replace
   (RULED — D12).** A `files.write` exchange (SubstrateV1 §3: `authority: share` →
   writes go through an exchange to the claim-holder) carries coarse/structural
   verbs — `replace` (whole file / region), `create`, `delete`, `rename` — and
   takes an **EXPECTED BASE REVISION** plus any required lock/lease. Conflict is
   EXPLICIT (`ExchangeRes` conflict outcome, GladeSupplierModel §6): last-writer-
   wins MUST NOT silently overwrite a changed base. **glade-editing's `doc.save`
   delegates INTO this** — glade-files OWNS the write, editing CALLS it (no
   duplicated enforcement). Coarse/structural mutations here; convergent
   character-level CRDT deltas stay glade-editing (P4).
2. **Workspace files are the at-rest truth; `doc.editing` marks live state (RULED —
   D13).** A `doc.editing` marker record signals active collaborative editing.
   Files, gwz, and diff consumers read the **LAST SAVED revision** unless an API
   explicitly requests the live editing generation — resolving the mid-session
   two-truths seam (one file is not silently two things).
3. **One `RootRelativePath` type + safe-open (RULED — D14).** Every
   filesystem-facing supplier (gwz/files/terminal) uses ONE `RootRelativePath`
   type and safe-open utility: normalize separators, reject absolute paths and
   parent traversal, constrain symlink resolution to the selected root, and avoid
   check/use (TOCTOU) races. External imports use scoped `RepoImportHandle`s (from
   C-gwz-3) — never arbitrary caller-supplied host paths.
4. **`ws.tree` is per-directory-keyed so subtrees redact (RULED — D7).** `ws.tree`
   is keyed by a canonical workspace-relative DIRECTORY path (not a monolithic
   `{root}` value, GladeAuthzModel §2/§6) and returns one bounded listing + explicit
   revision + continuation token. The same path policy check runs per directory
   key, so a `/src` read grant can hide `/secret` (impossible over one `{root}`
   value). Names never serve as workspace identity — the stable workspace/share ID
   routes (E-ws-1).
5. **Attribution is real from day one (RULED — B3/B4).** Read authz uses the B3
   `ProviderCallContext` principal — node-authenticated from the Hello/session and
   delivered BESIDE the request DTO, never a caller-payload `Hello.principal`
   field. Any private-zone key is identity-bound (B4): a `self:<user>` key is
   derived from the authenticated session and a caller-supplied literal that does
   not match is REJECTED. **AZ-1 grant enforcement stays stage-2** (§7), but the
   structure that makes it expressible — per-directory keying (D7) + the path type
   (D14) — lands now; a denied write/read fails AS DATA (`ExchangeRes{ok:false}`).

## 5. Retention and pressure (RULED — F-GAP10) — glade-files is the forcing function

Windowed backfill caches a whole big log/file locally; blob fetches cache big
binaries — **both multiply store growth** (today "per-tab IDB stores never
evicted, retention TTL unenforced" — GladeProgramStatus GAP-10). **F-GAP10 rules
ONE shared policy:** every retained class MUST declare `max_bytes`, `max_age`,
pressure priority, pin conditions, and terminal-state behavior; **eviction MUST
NOT silently rewrite authoritative history** (a cache is not the fold's truth).
glade-files owns the blob/window-cache row:

| Retained class | v1 default |
| --- | --- |
| Blob/window cache | **1 GiB/workspace and 4 GiB/node, LRU after 7 days; active windows and referenced blobs PINNED** |
| Pressure trigger/order | Trigger below 10% or 5 GiB free (whichever is safer); evict compute output → blobs/windows → closed terminal → anchored chat cache; NEVER evict pinned/current authorization state |

The rule is SHARED: **chat, terminal, and diff carry their own rows** in the same
F-GAP10 table (chat: authoritative records never silent-LRU, hot cache 256 MiB /
90 days; terminal scrollback: 16 MiB / 100k lines, 7 days after close; diff/service
output: 64 MiB/instance, 30-min unpinned expiry). The `retention` field
(`latest` / `from-cursor` / `windowed`, SubstrateV1 §3) gains this eviction knob,
distinct from the fold's authority.

## 6. Surfaces (declared per GladeSupplierModel; `ws.tree`/`ws.files` built)

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `ws.tree` | value | commons | directory listing keyed by canonical workspace-relative DIRECTORY path (RULED — D7: not `{root}`) + explicit revision + continuation token; the per-dir path policy redacts subtrees. glade-diff's source shape |
| `ws.files` | window (`log`/snapshot fill) | commons | file content; window identity `{workspace_id, path, revision}`, range `{from, len}` is an INTEREST over it (RULED — D8); glial reassembles, one generation, never mixed |
| `ws.blob.fetch` | exchange | commons | content-addressed blob FETCH — request names a workspace binding or path + revision + `BlobRef`, response over a bounded carrier/stream; authz RE-RESOLVED at delivery (RULED — D6; the bare-hash carrier op is killed) |
| `files.write` | exchange | commons | coarse write authority (`replace`/`create`/`delete`/`rename`) taking an expected base revision + lock/lease; explicit conflict, no silent LWW (RULED — D12); AZ-1-gated stage-2 |
| `doc.editing` | value | commons | "being edited by X" marker — signals a live editing generation exists; consumers still read the last saved revision unless the live generation is explicitly requested (RULED — D13) |

`build.log` / `term.log` are sibling window consumers (glade-terminal / razel own
those surfaces); the window MACHINERY is shared, the surfaces are not this
supplier's. Per-viewer interest regions are client-side dest params (§7), not
replicated surface content.

## 7. Stage split (security machinery is STAGE-1 — the §B reframe)

The security-substrate rulings move attribution into stage-1: "stage-1 allow-all"
is not honestly exercisable with a spoofable caller payload, so B3/B4 land now.

- **Stage-1 (buildable now):** `ws.tree` reads (per-directory-keyed, D7); windowed
  `ws.files` reads with viewport-first paint + bulk backfill + local scroll +
  generation-coherent reassembly (D8); `BlobRef` in records + on-demand
  `ws.blob.fetch` with delivery-time authz (D6); the `files.write` compare-and-
  replace CONTRACT (D12) exercised via editing's save. **Attribution is REAL, not
  stubbed:** the B3 `ProviderCallContext` principal is node-authenticated (not a
  `Hello.principal` payload) and private-zone keys are identity-bound (B4). Commons
  reads remain allow-all; `RootRelativePath`/safe-open (D14) is on from day one.
  Closes the audit's s-window PARTIAL (Plan P3.S1).
- **Stage-2:** AZ-1 path-scoped grants ENFORCE — a `/src` read grant redacts
  `/secret` (D7 keying makes it expressible), a write outside the granted subtree
  fails as data; `files.write` grant-gates; retention eviction (F-GAP10) becomes
  load-bearing rather than advisory.

## 8. Traces to author before building

- **s-file-window** (D8) — extends s-window from an append log to a MUTABLE FILE:
  viewer asks a range over identity `{workspace:ws1, path:src/big.rs, revision:R}`
  → glial reassembles the region and ships it INTERACTIVE (one round-trip,
  size-independent) → full file backfills BULK → a save advances the revision to
  R+1 and the reader re-drives against the new generation. Proves: the FULL mutable
  typed window, generation coherence (the reader **never observes a mixed
  generation**), glial reassembler ownership, file-size-independent first paint.
- **s-blob-fetch** (D6) — a big binary in `ws.tree` carries
  `BlobRef{hash,size,media}`; the record replicates to every replica while the
  bytes DO NOT; a viewer issues `ws.blob.fetch` naming the binding/path + revision
  + ref; the provider RE-RESOLVES the workspace and authorizes at DELIVERY; an
  authorized viewer gets the bytes over the bounded carrier and a second authorized
  viewer dedupes, but a **revoked/unauthorized principal holding the same hash is
  DENIED at delivery**. Proves: ONE exchange, delivery-time authz, the bearer-token
  shape is dead (hash = integrity, not authority), content-addressed dedupe.
- **s-file-write** (D12) — a `files.write replace` carries an EXPECTED BASE
  REVISION; when the base changed under it, the write returns an explicit conflict
  (`ExchangeRes` conflict), NOT a silent overwrite; the caller re-reads and
  retries. Proves: compare-and-replace, no silent last-writer-wins, conflict-as-data.
- **s-tree-subtree** (D7) — `ws.tree` keyed per directory path; a principal with a
  `/src` read grant lists `/src`, but the same per-directory path policy REDACTS
  `/secret` (impossible over a monolithic `{root}` value); a write outside the
  granted subtree fails as data (`ExchangeRes{ok:false}`). Proves: per-directory
  keying + subtree redaction, the AZ-1 enforcement point (stage-2),
  names-aren't-workspace-identity.
- Existing **s-window** (append-log case) and the `ws.tree` leg of s-fanout /
  s-authz remain the underlying mechanics — referenced, not re-authored.

## 9. Dependencies + user-testable-when

- Depends on: **base glade** (the `window` shape/contract P3.S1, the blob carrier
  P3.S2, the built priority scheduler), **glade-workspaces** (selection → which
  tree + root mapping), **glade-users** (read attribution). Forces: the window
  contract, the blob ruling (P3-gate), and GAP-10 retention.
- Consumers: **glade-diff** (`ws.tree`-shaped sources), **glade-editing** (the
  document to edit), the gryth-ui files plugin.
- **User-testable when:** I select a workspace; its REAL directory tree renders
  (honest on an empty/zero-member workspace); I open a large source file (or a
  40k-line build log) and the viewport paints in ONE round-trip regardless of
  size, scrolling staying local as the body backfills; I open a big binary (an
  image) and it fetches by hash without stalling the fold; (stage-2) a read grant
  scoped to `/src` hides the rest, and a write outside my granted path fails as
  data.

## 10. Open questions — RESOLVED by GLP-0006 (RulingWorksheet §IV + §I)

All design opens this spec raised are now ruled; residuals are impl-level, not
design choices.

- **Window: Shape or projection? — RESOLVED (D8).** The FULL mutable typed window
  ships v1 as one uniform contract; **glial** owns reassembly (the previously
  unowned reassembler) and serves one generation, never mixed (§2.4).
- **Window unit — RESOLVED (D8).** Line `{from, len}` for text / byte-offset for
  binary is an INTEREST over identity `{workspace_id, path, revision}`, owned by
  the surface's shape contract (§2.5).
- **Blob carrier (P3-gate) — RESOLVED (D6).** The fetch is ONE `ws.blob.fetch`
  exchange over a bounded carrier/stream; iroh-blobs (native) vs Chunk-frame
  (browser) is an impl choice UNDER that contract, not a surface question (§3.2).
- **Blob eagerness — RESOLVED (D6).** Lazy — bytes-follow-interest; a referencing
  record replicates without dragging bytes; a fetched, referenced blob is pinned
  in cache (F-GAP10) (§3.4).
- **`ws.blob` — declared surface vs bare carrier op — RESOLVED (D6).** A DECLARED
  `ws.blob.fetch` exchange; the bare hash-addressed carrier op is killed — it was a
  bearer token (§3, §6).
- **Write seam — RESOLVED (D12).** glade-files owns coarse/structural writes
  (`replace`/`create`/`delete`/`rename`); glade-editing owns live deltas and its
  `doc.save` DELEGATES into `files.write` (§4.1).
- **AZ-1 for v1? — RESOLVED (D7 + D14 + stage split).** The STRUCTURE lands now —
  per-directory `ws.tree` keying (D7) + `RootRelativePath`/safe-open (D14) make
  path-scoped redaction EXPRESSIBLE; the grant ENFORCEMENT (a `/src` grant hides
  `/secret`; write gating) stays stage-2 AZ-1 (§4.4, §7).
- **GAP-10 retention — RESOLVED (F-GAP10).** Every retained class declares
  `max_bytes`/`max_age`/pressure-priority/pin/terminal-state; blob/window cache =
  1 GiB/ws + 4 GiB/node, LRU after 7d, active windows + referenced blobs pinned;
  shared pressure trigger/order; eviction never rewrites authoritative history (§5).
