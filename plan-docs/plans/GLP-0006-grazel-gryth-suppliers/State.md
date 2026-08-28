# State — GLP-0006

Status: **ACTIVE** — P00 gates ruled 2026-07-12 (Gianni: "OK - build these
out in parallel"): P00-a wire-attached sessions CONFIRMED; P00-b new glade-wz
members (repo `grazel-node`, member path `grazel` — bare `owebeeone/grazel`
is the June razel spike, untouched, glial-runtime precedent); P00-c F2
BUILD (fused with F1 in P0.S2: creation mints the records). P0 wave launched
2026-07-12.

Baseline: E2E stage-1 audited MET (root `f9671a1` pins; glade `39bd59a`,
glial `bb53122`). P0 S1-S7 built (waves 1-3, 2026-07-12).

Current checkpoint: `grazel1/p1-first-suppliers` MET 2026-07-12 (P0 also
met; neither tag minted — Gianni's call). gryth-ui branch awaits Gianni's
merge.

Shape-catalogue checkpoint (2026-08-28): **CLOSED and materialized**. GWZ now
pins the canonical Taut `0.9.*` release train and exact fail-closed Glade/Glial
dispatch (`GDL-041`, `dev-docs/TautShapeCatalogAdoption.md`). Contract creation
and the three-language matrix are complete. P3 started with the Glade `swmr`
adapter + file-window projection recorded below; P4 may start at Glade/Glial
`crdt`/`text_crdt` integration. Neither lane may bypass the B1–B5 security
substrate.

Next governance checkpoint: P2 sharing + stage-2 enforcement remains gated on
WD-1 + AZ-1/2/3. `PlanGladeUsers.md` Phases 0–4 MAY proceed; only its Phase 5
and the P2 enforcement checkpoint remain behind that gate.

P3.S1 vertical slice (2026-08-29): **BUILT, P3 checkpoint still open**. Glade
wire/declaration contracts append `swmr`; node and both clients validate
`glade.swmr.adapter/v1`; the node rejects malformed actions, mixed shapes, and
second writers before persistence; Glial assembles through released
`SwmrNode`; `ws.files` now declares SWMR; and the live demo proves
snapshot→delta→reset→new-snapshot epochs without cross-generation bytes.

Next P3 checkpoint: turn the demo's bounded full-image projection into the
path-addressed `{workspace_id,path,revision}` viewport/backfill seam, then rule
the large/binary blob strategy before building `glade-files`. Authoritative
write acknowledgement remains an explicit residual; node rejection MUST NOT be
treated as an acknowledged optimistic local write.
