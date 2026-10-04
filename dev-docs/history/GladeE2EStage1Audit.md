# Glade E2E Stage-1 — Definition-of-Done Audit (2026-07-11)

Status: audit record — the deliberate check that turned "all builders done"
into a verdict. Two halves: a **live composed run** (all pieces together for
the first time) and a **trace-conformance sweep** (every stage-1 trace vs the
built system). Verdict at the end.

Auditors: live run driven interactively (this session); conformance sweep by a
read-only agent over the test suites + implementation. Workspace state:
glade `39bd59a` · glial `bb53122` · root `75b641a` pins.

## 1. The DoD checklist (as ratified in GladeProgramStatus)

| Clause | Verdict | Evidence |
| --- | --- | --- |
| browser (glial binder) ↔ glade-local ↔ glade-peer over iroh | **MET (live)** | Two booted nodes (`--profile local` ⇄ `--profile peer`, real iroh dial, `peer-connected`); both stores held home-share chains from BOTH origins; demo (two tabs, alice/bob) live against node A through the glial binder — notes (commons value), activity (log), selection events converged; alice's private `self:alice` selection never reached bob. Screenshot in session record. |
| workspace list from the registry | **MET with a named gap** | Node A served, from its LOCAL replica, grazel's directory records registered on node B (4 `dir.bindings` + 1 `dir.services` + 2 `dir.grants`, origin = B, over iroh) — the mechanism is live. `dir.workspaces` itself was EMPTY: **no production path mints WorkspaceEntry/ServeClaim** (the R3/R4 E2E tests mint them via the registry API). See finding F1. |
| one grazel binding live | **MET (test) / MET (live, adjacent)** | `exchange.rs::grazel_attach_end_to_end` (b): ws.tree ops converge post-registration. Live: grazel's binding records served cross-node; the demo's four bindings live through the same serve path. |
| one gwz exchange round-trips | **MET (live, local leg) + MET (test, forward leg)** | Live: provider attached as authority by subscribing declared `(ws-razel, gwz.ops)`; requester's `gwz.status` → `pong:gwz.status`, corr intact; no-provider case answered `ok:false` as data (immediately, session usable). Forward-over-iroh leg pinned by the cargo E2E. Cross-NODE live forward blocked only by F1 (no live claim to route on). |
| all snapshot files under `~/.glade/sys/` | **MET (live)** | Both nodes: `sys/<name>/{node.key (0600), instance.lock, records.json, cache/store/…}` under isolated GLADE_HOMEs; per-origin chain logs present on both sides after sync. |
| every behavior matching its trace | **SUBSTANTIALLY MET** — see §2 | 11 stage-1 traces: 4 CONFORMS, 5 PARTIAL (mechanism built, edge or variant open), 1 KNOWN-DEFERRED, 1 leaning-CONTRADICTS on an unbuilt head (s-create). |

## 2. Trace-conformance sweep (stage-1, 11 traces)

| Trace | Verdict | Headline gap |
| --- | --- | --- |
| s-discovery | CONFORMS | pkarr publish/resolve not independently pinned (gate is `designed`) |
| s-boot | CONFORMS | class-3 `local.json` self-sig is a structural stub (stage-2 posture) |
| s-app-register | CONFORMS | — |
| s-fanout | PARTIAL | replica-attach (F1) mechanism exists (`Mesh.forwarded`) but not E2E-pinned; F3 retention open by the trace itself |
| s-fanout-exchange | CONFORMS | — |
| s-diff-service | KNOWN-DEFERRED | service instantiation (SPAWN/compute/TEARDOWN) entirely unbuilt; trace gates are `open-question`; deferral only weakly documented |
| s-3ui-2node | PARTIAL | C2.a nearest-REPLICA routing not built (always routes to claim-holder); trace marks it open-question |
| s-window | PARTIAL/DEFERRED | priority scheduler built+tested; windowed DELIVERY (reassembler §7) unbuilt — explicitly outside M-LIMP |
| s-takeover | PARTIAL | epoch-fencing + lapse-at-reader-clock built; the workspace.lock half lives outside the substrate (gwz layer) |
| s-offline | PARTIAL/DEFERRED | O1/O2 conform (incl. GAP-9 reload-resume); O3 MV conflict-as-data contradicts today's pure-LWW fold — deferral documented (SubstrateV1 §11 / GQ-1) |
| s-create | PARTIAL, leaning CONTRADICTS | D1–D3 target-based creation routing NOT built and NOT documented as deferred (see F2) |

Full per-step evidence citations live in the sweep agent's report (session
record); the named tests are the durable anchors — every CONFORMS row cites
specific `cargo`/`node:test` tests that ran green on audit day.

## 3. Findings (ranked)

- **F1 — no production path mints WorkspaceEntry/ServeClaim.** `--app` loads
  bindings/services/grants, but nothing live claims a workspace share; the
  E2E tests append those records via the registry API. Consequence: live
  cross-node claim routing / exchange forwarding can't be composed outside
  tests. The natural fix rides Lane R: the app-loading node mints its
  WorkspaceEntry + ServeClaim (or a CLI/config path does). Small, well-fenced.
- **F2 — s-create's target-routed creation is unbuilt and undocumented.**
  Exchange routing is claim/share-based; a `workspace.create` aimed at a
  named target with no claim yet has no path. Either build target routing or
  record the deferral — the only *undocumented* behavioral gap found.
- **F3 — s-diff-service deferral should be recorded explicitly** (it is
  trace-flagged open-question and outside M-LIMP, but no doc names it).
- **F4 — SubstrateV1 §11's "not built" list is stale** relative to the code
  (iroh mesh, multi-node convergence, grazel attach, boot seam are all built
  and tested). Doc sweep needed so the deferral list stays trustworthy.
- **F5 — audit-method note:** an apparent notes-convergence failure during
  the live run was a probe artifact (`innerText` excludes textarea values);
  the notes DID converge. Recorded so future audits don't re-trip it. LWW
  behaved deterministically throughout (a fresh low-lamport session's write
  correctly lost to the live session's later value).

Standing recorded gaps (unchanged by this audit, tracked elsewhere): GAP-11
offline outbox (glial DecisionLog), GAP-10 retention tail, GC-2 conflation,
taut `--legacy-codec` migration (dies at taut v0.10).

## 4. Verdict

**E2E stage-1: MET, with F1 as the one substantive live-composition gap.**
The composed system — browser through glial binder, two booted nodes over
iroh, grazel declared as data, directory records converging, exchange
round-tripping, zones semantics holding live — works end to end. The traces
that specify built behavior are conformant; the PARTIAL rows are open
design questions or explicitly staged work, with exactly one undocumented
gap (F2) and one small build item (F1) to close before the milestone is
unqualified.

Recommended closure order: F1 (small, unlocks live cross-node exchange) →
F2/F3 (deferral records or a ruling) → F4 doc sweep → then stage-2 switch-on.
