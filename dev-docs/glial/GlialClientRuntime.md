# Glial Client Runtime — persistence first, glade optional, assembly inside

Status: working draft — records the GDL-035 direction (Gianni, 2026-07-05)

Purpose: pin glial's client-side identity. OLD glial was a persistence layer
(local browser store) — that identity is KEPT and promoted: glial is the
client kernel between grip and (optionally) glade. The M-LIMP prototype wired
taps directly to glade sessions; that was scaffolding, not the architecture.

## The three rules

1. **Persistence first, connectivity configured.** Every bound tap gets a
   local destination (browser store / IndexedDB) by default — a grip app
   persists with ZERO glade anywhere. Glade connectivity is a *configured*
   destination glial adds when an environment/workspace mount says so
   (config-as-data; the composition story). This inverts nothing in the
   substrate — SubstrateV1 §5 already says persistence is the degenerate
   share — it assigns OWNERSHIP: the client store is glial's, not the tap's
   and not glade's.
2. **Assembly happens inside glial.** Glial is taut-shape aware: the delivery-
   shape engines (log/value/window/text-crdt folds, the §7 reassembler) run in
   glial, once per **binding instance** (see Boundaries) — not in taps, not in
   components. Taps declare (via `glade-decl`) and stay thin conduits; glial
   fans assembled results to every attached tap. Sharing machinery is never
   per-tap code.
3. **Rich change events, consumer's choice.** What glial emits to a tap is not
   a bare value but a shape-aware event: enough structure for the receiver to
   choose an incremental patch or a whole-field refresh against its LIVE UI
   state (cursor active ⇒ apply delta; unmounted ⇒ take refresh).

## The event envelope (sketch — schema lands in taut-shape)

| Field | Meaning |
| --- | --- |
| `shape` | which delivery shape produced this |
| `kind` | `refresh` (whole value present) \| `delta` (incremental) |
| `value?` | the assembled whole (always available on demand) |
| `delta?` | shape-specific: appended log entries; value replace; text-crdt ops with **stable position identity** |
| `baseSeq` / `meta` | what the delta applies against; origin attribution |

The text-crdt case is why this exists: a multi-line field must apply remote
deltas into a live editor without rewriting the field, and the cursor anchors
to CRDT element identity — position stability is the shape's gift, the event
envelope just delivers it. (Full editor-binding design deferred to the
`text-crdt` shape's turn in the contracts track.)

## Boundaries

- **glial does not own the wire**: sessions, frames, replication are glade's;
  glial *manages* sessions (opens/configures them per mount) and owns what
  reaches taps.
- **glade-decl is the only shared vocabulary** between grip-core, glial, and
  glade (`GladeDeclSurface.md`).
- **grip-share** shrinks to declaration plumbing; its direct glade coupling is
  deleted (the compile wall proves it).
- The orchestration half of glial (environments, mounts, capability issuance,
  facets) is unchanged and CONNECTS here: a mount is precisely the config
  that turns connectivity on for a set of bindings.
- **Declarations vs instances (clarified 2026-07-10).** A `BindingDecl` is
  app-static; a **binding instance** is `(decl, domain/zone/key fill)`,
  created at mount. Several instances of ONE declaration may be live at once
  (two columns mounted on two documents: same decl, same grips, different
  domain fill) — each with its own fold/assembly state and a refcounted
  lifecycle (s-fanout's interest counting, client-side).
- **The seam is mount/unmount, idiom-agnostic.** How the grip side selects
  and parameterizes an instance — param grips through one engine-global
  binding row (grip-react-demo WeatherColumn) or per-context matcher rows
  (CoinColumn) — is grip's business and never crosses into glial. Glial's
  API is mount/unmount of instances with fills; it must not reference
  `MatchingContext` or any matcher/scoring vocabulary. Neither idiom pushes
  into glade either: only what `BindingDecl` + the fill carry (→ wire
  `share`/`key` at bind time) crosses that seam.

## Open questions

| # | Question |
| --- | --- |
| GC-1 | ~~Event envelope home~~ **RULED 2026-07-07**: split — the generic envelope SHELL (`ChangeEvent{glade_id, shape, kind, base_seq, origin_meta, payload}`) lives in **glade-decl** (the grip↔glial tie; grip-core types events without glial); each shape's DELTA payload schema lives in its taut-shape contract, carried opaquely (GDL-003 resolves into those). |
| GC-2 | Backpressure/conflation for rapid deltas (value conflates; log batches; crdt coalesces?) |
| GC-3 | ~~Migration of the M-LIMP binder~~ **RULED 2026-07-10 (Gianni): per-binding cutover** — each M-LIMP binding moves to a glial mount individually and is verified against the running demo before the next; grip-share shrinks binding-by-binding to deletion. NO compatibility shim / no strangle layer (a shim exists only to be deleted). |
| GC-4 | Store engine + quota policy in browser (IndexedDB now; OPFS later?) and eviction vs declared retention |
