# Decisions — GLP-0006

Plan-local decisions + revision history. Corpus-level rulings get GDL rows;
this file records how they landed in the plan.

## 2026-07-11 — plan creation

- **Supplier vocabulary RULED (Gianni)** → GDL-040: "supplier" = the
  authority-side module behind declared surfaces; drip-irrigation register
  (suppliers feed the mainline, taps draw); registered as "glade suppliers".
  Rejected: provider (collides with grip's declaration/provider/facet
  vocabulary + colorless), spring (Java collision), spout (Storm), well
  (passive), main (`main()` fatal).
- **Repo naming RULED (Gianni)**: repos say what, terminology says role —
  `glade-terminal`, `glade-chat`, …; never `supplier-*`.
- **Attachment model — working assumption, P00-a confirms**: wire-attached
  session (R4's provider mechanism). In-process embedding = grazel-internal
  optimization only.
- **Grazel data seam stated** (plan §Architecture): app-owned storage glade
  never sees; shared = declared surface; private = undeclared. One-page
  design doc lands at P0.S1.
- Endpoint list ↔ shape mapping (from the 2026-07-11 discussion): chat=log,
  gwz=exchange, files=window+blobs, terminal=log+channel(+WINCH),
  editing=crdt/swmr (P4-gate), users/ACLs+share=stage-2 management surface,
  razel=exchange (floats).
- Typed-manifest compile wall (glade ids as typed identifiers, GQ-6-aligned)
  accepted into P0.S5 — from the "so many moving parts" design review, point 1.
- grip-lab is reference-only (lessons harvested); iroh+react already the
  decided substrate/front-end pairing.
