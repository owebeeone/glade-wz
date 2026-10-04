# GladeDeclReconciliation — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/glade/GladeDeclReconciliation.md` at glade-wz root `b132b7e6d365` (619 lines, sha1 `afcf4e563d937b6ec72e0f0f7748423b6a5d5e2c`); status "assessment + amendment proposal", dated 2026-09-21. A DRAFT design ahead of the `glade-decl` interface freeze and first publish.

**Baseline (verified identical at start and at end of this review):** glade-wz root `b132b7e6d365` · glade-decl `bbce73d67146` · glade-decl-ts `7e16e324630a` · glade-decl-rs `555a97746fc6` · glade-decl-py `1b0f6d1f7886` · glade `960c9b0fa038` · glial `0dfe4b930063` · grip-core `97ff6c26f12e` · grazel `924cb3c4bab9` · glade-gyld `024d2a8ae061` · glade-gwz `e53c87dddb8f` · glade-chat `9238d21f6a36` · `/Users/owebeeone/limbo/gryth-wz/gryth-ui` `3af64c2bab74`. Supporting (not in the tuple, read for authority only): `/Users/owebeeone/limbo/gwz-dev` `ff431743cc4c`, `glade-wz/taut` `7a5f616c3a9f`, `glade-wz/taut-shape` (working tree). Every source was read with `git -C <repo> show <sha>:<path>`; only `glade-decl-rs` was dirty (the two out-of-scope rustfmt files named in the tuple), and it was byte-unchanged after the one command run.

**Date:** 2026-09-21
**Axis:** Consistency — the document against its controlling graph: internal contradictions, agreement with every contract/design it cites at the cited line, exactness of superseded-clause lists, satisfiability of its own evidence and test sections, and unstated impacts on documents it does not cite. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: NO-GO** — 0 P0, 0 P1, 5 P2, 6 P3. Every blocking finding is a bounded text edit to the object itself; no code change is implied by any of them. **I pre-commit to GO on a revision that resolves P2-1, P2-2, P2-3, P2-4 and P2-5 as specified.**

---

## 0. Evidence base

**Object:** `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclReconciliation.md` @ `b132b7e6d365`, read in full (619 lines).

**Decision authority**
- `/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md` @ `b132b7e6d365` — lines 47, 50, 53, 55, 56, 58, 59 (GDL-029, -032, -035, -037, -038, -041, -039).
- `/Users/owebeeone/limbo/gwz-dev/dev-docs/AgentProcessRules.md` @ `ff431743cc4c` — L1-07 (:240-253), L1-08 (:255-265), L1-09 (:267-277), L1-17 (:370-386), L1-18 (:387-421 incl. the 2026-09-18 D7 Surface amendment).

**Controlling designs**
- `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclSurface.md` (149 lines) — :22-32, :59, :111-117.
- `/Users/owebeeone/limbo/glade-wz/glade-decl/dev-docs/DeclSurface.md` (148 lines) — :27, :28-32, :36-38, :58; full `diff` against the root copy.
- `/Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeZones.md` — :37-45, :85-96, :107-117, :119-126.
- `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialClientRuntime.md` — :20-30, :79-86.
- `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialFitAssessment-2026-09-15.md` — §1 (:22-55), §5 (:101-124), recommended order S1-S6 (:145-154), "What can be skipped" (:156-168).
- `/Users/owebeeone/limbo/glade-wz/dev-docs/PackageExtractionPlan.md` — :68-76, :146-161, :177, :205.
- `/Users/owebeeone/limbo/glade-wz/glial/dev-docs/DecisionLog.md` — GAP-2 (:27), GAP-5 (:54), GAP-7 (:85), GAP-10 (:177), GAP-14 (:256-278).

**Contract + renderings**
- `glade-decl/ir/glade_decl.taut.py` (141 lines, read whole), `glade-decl/ir/glade_decl.ir.json` (`shapes` catalogue + 6 enums + messages), `glade-decl/corpus/build.py` (:25-60, :160-194), `glade-decl/corpus/decl.v0.json` (26 vectors, names enumerated), `glade-decl/README.md` (:30-86), `glade-decl/dev-docs/OpenNotes.md` (N1-N6, 60 lines read whole).
- `glade-decl-ts/{package.json, README.md, src/index.ts, src/api.ts, src/corpus.test.ts, src/decl.v0.json, src/glade_decl.ir.json}`; `glade-decl-rs/{README.md, src/lib.rs, src/api.rs, src/vectors.rs}`; `glade-decl-py/{README.md, pyproject.toml, src/glade_decl/{__init__.py, api.py}, tests/{conftest.py, test_corpus.py}}`.

**Node, clients, consumers**
- `glade/node/ir/sysdata.taut.py` (:1-100), `glade/node/src/appdecl.rs` (435 lines, read whole), `glade/node/src/router.rs` (:128-137), `glade/node/src/store.rs` (:155-185, :404-414).
- `glade/client-ts/src/shapes.ts`, `glade/client-rs/src/{session.rs:1-45, supplier.rs:150-160}`, `taut/corpus/glade.ir.json`, `glade/decl/{glade_decl.taut.py, README.md}`.
- `glade/demo/src/manifest.ts` (whole), **`glade/grip-share/src/manifest.ts` (whole — not in the object's read list)**.
- `glial/src/{shapes.ts (whole), manifest.ts:50-90, binder.ts:40-70, instance.ts:25-45/95-110, events.ts:45-130}`; `grip-core/src/core/share_decl.ts` (whole); `glade-chat/src/manifest.ts:25-40/70-85`.
- All five `.glade` files read whole and counted by hand: `grazel/apps/{grazel,gyld}-app.glade`, `glade/apps/grazel-app.glade`, `glade-gyld/tests/fixtures/gyld-test-app.glade`, `glade-gwz/tests/fixtures/gwz-test-app.glade`.
- gryth-ui: `src/taps.ts`, `packages/glade/src/{runtime.ts, glade.ir.json}`, `packages/plugins/{gwz/src/live.ts, gyld/src/{live.ts, ops/surfaces.ts}, chat/src/{live.ts, groups.test.ts}}`, `pnpm-workspace.yaml`, `vite.config.ts`.
- `taut/src/taut/{cli.py, gen/scaffold.py:625-677}` — to establish the generator's output layout.

**Commands run (read-only)**
- `git -C <repo> rev-parse --short=12 HEAD` for all 13 repos, at start and at end — identical.
- `git -C glade-decl-rs status --short` → ` M src/api.rs`, ` M src/vectors.rs`; `git diff --stat` → 284 / 43 lines; `git diff --ignore-all-space` → line-wrapping only.
- `shasum` comparisons: the two `grazel-app.glade` copies (`2722256b…`, identical); contract vs ts vs py copies of `glade_decl.ir.json` (`1e7d4d16…`) and `decl.v0.json` (`f669da0b…`).
- **`/opt/homebrew/bin/python3 corpus/build.py --check` in `glade-decl/`** → `STALE: /Users/owebeeone/limbo/glade-wz/glade-decl-rs/src/vectors.rs`, exit 1. Authorised: reading `build.py:176-184` proves `--check` returns before the write loop at `:186-189`; `git status` after the run confirmed the tree was untouched.
- `diff` of the two DeclSurface documents; `git grep -n 'decl\.v0'` across the four glade-decl repos and root `dev-docs`; `ls taut-shape/corpus/*.json | wc -l`; `find` for lockfiles and `glade-sys.glade`.

---

## 1. Findings

### [P2-1] The domain/zone consumer survey omits `glade/grip-share`, so R1's evidence and option costs are wrong and row 16 is false

**Location.** Object rows 12, 13, 15, 16, 30 (lines 110-113, 118-119, 153) and §3 R1 (lines 215-246). Missing evidence: `/Users/owebeeone/limbo/glade-wz/glade/grip-share/src/manifest.ts:29-34, 58-70`; `/Users/owebeeone/limbo/glade-wz/glade/demo/src/manifest.ts:61-78`; `/Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeZones.md:107-117`. The object's Appendix (line 610) lists `glade/demo/src/manifest.ts` as read but never `glade/grip-share`.

**Violated invariant.** A "no consumer" claim must survive a search of the consumer set, and a RULING's option table must state each option's cost in every repo the option touches — especially when §4.6 declares "**The running demo's behaviour**" a must-not-change.

**Reproduction.** `glade/grip-share/src/manifest.ts:58-70` (`manifestScope`) does, per mounted surface:
`const domain = decl.domain ?? spec?.domain ?? ""` → `manifest.domains[domain]?.share` → the wire `share`; and `decl.zone` → `manifest.zones[zone]?.key` → the wire `key`.
`glade/demo/src/manifest.ts:69-76` supplies that table keyed by the **`DomainAnchor` member names**: `document: {share: "doc:{doc}"}`, `account: {share: "account:{self}"}`, `zones.private.key = "self:{self}"`. The demo's `M` surfaces (`:26-58`) carry `domain: "account" | "document"` and `zone: "commons" | "private"`. `GladeZones.md:110-117` records this as an implemented change (GLP-0006 P0.S5b, 2026-07-12): the table "is keyed by the canonical `DomainAnchor` the handle carries … the rekey just lets the scope resolve straight off the typed handle's `domain`". `grip-core/src/core/share_decl.ts:33-34` documents the same contract ("A binder's scope maps it to the wire `share`").

**Impact.** Four statements in the object are wrong, and one option is mis-costed:
- Row 12 "the anchor exists only in the contract" — false.
- Row 30 "no consumer that uses it as an anchor" — false.
- R1's question paragraph "every live consumer hard-codes a concrete `share` string instead" — false for the demo path, where `share` is a *template* selected **through** the anchor.
- Row 16 "**nothing client-side produces that key**" — false: `WORKSPACE_MANIFEST.zones.private.key = "self:{self}"` plus `manifestScope` produce exactly `self:<user>`, and `stubGrant` (`demo/src/manifest.ts:88`) allows it. (The source the object leans on, `GlialFitAssessment` §5 line 119-120, says only the narrower and true "`Fill.zone` never becomes that key"; the object broadened it.)
- R1 option **(d) Delete `domain` + `DomainAnchor`** is costed as "Node: node already agrees / App files: no edit" with glial-only work. It would in fact break `manifestScope`'s domain branch and `ShareDecl.domain`, i.e. the running demo's share resolution — which §4.6 forbids changing. R1 option **(b)** proposes glial "gains the `(DomainAnchor, ZoneKind, principal) → (share, key)` mapping … ~150 LOC; retires 4 hand-rolled mappings"; the generic, data-driven fifth mapping already exists in `grip-share` and is not counted or cited.

**Required correction.** Add `glade/grip-share/src/manifest.ts` and `glade/demo/src/manifest.ts` to the "clients/glial/UI" column definition (§2 legend, line 77-79) and to the Appendix. Restate rows 12, 13, 15, 16 and 30 against it. Add a `glade/demo + glade/grip-share` column to R1's option table, with (c) and (d) carrying the demo cost explicitly, and reconcile that cost against §4.6's "running demo" clause.

**Closure test.** A pre-freeze grep gate that fails if `git grep -n 'domains\[\|zones\[' glade/grip-share/src` returns a hit not accounted for in the ruling; or an assertion in `glade/grip-share/test/manifest.test.ts` that `manifestScope` resolves `{domain:"account", zone:"private"}` to `share="account:alice", key=utf8("self:alice")`, which will fail loudly under R1(d).

---

### [P2-2] §4.3's regeneration commands write to the wrong directories, and the TypeScript gate would stay green over stale `api.ts`

**Location.** Object §4.3, lines 460-471. Authority: `/Users/owebeeone/limbo/glade-wz/taut/src/taut/gen/scaffold.py:639`; the three rendering READMEs (`glade-decl-rs/README.md:28-33`, `glade-decl-ts/README.md:29-34`, `glade-decl-py/README.md:32-37`); `glade/node/ir/sysdata.taut.py:20-23`.

**Violated invariant.** A published regeneration procedure must place generated artefacts where the gates and the package consumers read them.

**Reproduction.** `scaffold.py:639` is `d = out_dir / lang`. The object's block passes `-o ../glade-decl-rs`, `-o ../glade-decl-ts`, `-o ../glade-decl-py`, so tautc writes `glade-decl-rs/rust/api.rs`, `glade-decl-ts/typescript/api.ts`, `glade-decl-py/python/api.py` — three new untracked directories — and leaves `src/api.rs`, `src/api.ts`, `src/glade_decl/api.py` untouched. Every existing documented form avoids this by generating into `/tmp/g` and copying (`cp /tmp/g/typescript/*.ts ../glade-decl-ts/src/`), and the node's own sysdata procedure does the same (`sysdata.taut.py:23`). §4.3 also omits that API copy step entirely; its only copy comment (line 470) covers `glade_decl.ir.json + decl.v1.json`.

**Impact.** Run as written at the freeze: `corpus/build.py` regenerates `glade-decl-rs/src/vectors.rs` from the new schema while `src/api.rs` is still v0 → `cargo test` fails (recoverable, noisy). **The TypeScript case fails silently.** `glade-decl-ts/src/corpus.test.ts:9-20` loads `glade_decl.ir.json` and drives `codec.ts`/`schema.ts`; it never imports `api.ts`, and `src/index.ts:15` re-exports `api.ts` as `export type *` (erased at build). So with the new IR and new corpus copied in, `pnpm test` is green, `CONTRACT_VERSION` is re-pinned to the amendment commit (§4.3, A1) — and `@owebeeone/glade-decl` publishes v1-labelled types that are the pre-amendment ones. That is exactly the false-composition the one-shot freeze cannot recover from cheaply.

**Required correction.** Replace §4.3's block with the rendering READMEs' form: generate to a scratch dir, `cp /tmp/g/<lang>/*` into each rendering's `src/`, then the ir/corpus copies for ts and py only, then `corpus/build.py`. Note in §4.7 that the ts corpus gate does **not** exercise `api.ts`.

**Closure test.** After regeneration and before any publish: `git -C glade-decl-ts status --porcelain` shows no `typescript/` path, and `grep -q '"atom"' glade-decl-ts/src/api.ts` (and the equivalent in `glade-decl-rs/src/api.rs`, `glade-decl-py/src/glade_decl/api.py`) succeeds. Add that grep to the §4.7 table as a row of its own.

---

### [P2-3] §4.2's corpus-retarget list is incomplete; the rename as specified breaks the Python gate's packaging

**Location.** Object §4.2, lines 439-445 ("retarget `build.py:35`, both language gates, and `README.md:52-63`").

**Violated invariant.** A rename inside a frozen artefact must enumerate every reference the gates and the packaging depend on.

**Reproduction.** `git grep -n 'decl\.v0'` across the four glade-decl repos and root `dev-docs` returns 20 hits. §4.2 names four. The un-named ones that matter:
- `glade-decl-py/pyproject.toml:20` — `"src/glade_decl/decl.v0.json" = "glade_decl/decl.v0.json"`. `tests/test_corpus.py:23` reads the oracle through `importlib.resources.files("glade_decl")`. Rename the file without this line and the corpus is not in the distribution → §4.7's `pytest` check cannot pass.
- `glade-decl-ts/README.md:16,33` and `glade-decl-py/README.md:15,36` — including the two `cp … corpus/decl.v0.json …` commands, i.e. the copy step itself.
- `glade-decl/corpus/build.py:4,16` (module + usage docstrings), `glade-decl/ir/glade_decl.taut.py:28`, `glade-decl/dev-docs/DeclSurface.md:58`, `glade-decl/dev-docs/OpenNotes.md:45` ("its vectors join `corpus/decl.v0.json`" — N4, which §4.1.6 already opens for a different reason).
- `glade-decl-ts/src/corpus.test.ts:3`, `glade-decl-py/src/glade_decl/__init__.py:11`, `glade-decl-py/tests/test_corpus.py:3`.
- Root `dev-docs/glade/GladeDeclSurface.md:59` (the ratified surface doc's repo-structure listing) and `dev-docs/GladeHandoff-260710.md:163`.

**Impact.** The one-shot amendment leaves a broken Python gate and eleven stale references, including two inside the ratified design text — in a repo whose whole premise is that the corpus name is the oracle's identity.

**Required correction.** Replace the four-item list with the enumerated set (or with the grep that produces it), and split it by "must change for the gate to pass" vs "must change for the text to be true".

**Closure test.** After the amendment commit, `git grep -rn 'decl\.v0' glade-decl glade-decl-ts glade-decl-rs glade-decl-py dev-docs` returns nothing, and `pytest` in `glade-decl-py` passes from a built wheel (not the source tree).

---

### [P2-4] Two rows are classed SETTLED although they impose node-side enforcement drawn from an unratified decision and pre-empt an open ruling

**Location.** Row 31 (line 154), row 18 (line 126), §4.4 bullet 2 (lines 487-489), R2's closing sentence (lines 270-272).

**Violated invariant.** The object's own §1 precedence rule 2: "Where no ratified decision exists, what the node implements and the app files already use is EVIDENCE of intent, **not a decision**." And its own R7 objection: "Freezing a record format for an open decision is the failure mode … exists to prevent."

**Reproduction.**
- Row 31's own *Ratified* column reads "GDL-039 not ratified" — verified verbatim at root `DecisionLog.md:59` ("open (implemented, pending ratify)"). Its resolution — "validate node-side against the contract vocabulary" — makes `appdecl.rs` *reject* any zone token outside `{commons, private}`. That is a new normative act on an unratified vocabulary. `glade-decl/dev-docs/OpenNotes.md:16-23` (N2) records the consequence: when the axis vocabulary ratifies, "`zone` grows from an enum to a message `{kind, axes: [...]}` — **a breaking change**". Row 31 is the only element in the audit whose class (SETTLED) is asserted directly against its own "not ratified" cell.
- Row 18 bundles two things. The spelling normalisation (`from-cursor` → `from_cursor`, forced by taut's identifier rule) is a genuine implemented fact and is rightly SETTLED. The second half — "validate node-side" — is not: R2 option **(d)** is "Demote `retention` to a free `STR`, matching the node", under which there is no vocabulary to validate. R2's own closing sentence says "Whatever is chosen, … the node starts validating the token — that part is settled (row 18/31)", which is false for option (d) that the same section offers.

**Impact.** A SETTLED row pre-empts a RULING the owner has not made, and node-side input validation would freeze against a vocabulary whose ratification is explicitly pending. Under the document's own framing, that is the R7 defect applied to GDL-039 instead of GDL-029.

**Required correction.** Split both rows. Keep as SETTLED only the halves decided by a ratified entry or by a unanimous implemented fact (`crdt` becomes declarable; the hyphen spelling normalises). Move "validate token 4" into R1's option set (or a new ruling) and "validate token 5" into R2's, and delete the "that part is settled" sentence at lines 270-272.

**Closure test.** Re-derive the class of all 51 rows under the stated rule "SETTLED iff (a) a ratified DecisionLog entry decides it, or (b) it changes no input the node currently accepts". Row 31 and row 18's second half must move to RULING; the arithmetic in the §2 header must be restated accordingly.

---

### [P2-5] `glade-decl/dev-docs/DeclSurface.md:27` contradicts ratified GDL-041 and the root copy; the audit does not report it and §4 does not touch it

**Location.** Object rows 1-9 (the `Shape` audit), §1 precedence rule 1, R8 (lines 390-397, which cites `glade-decl/dev-docs/DeclSurface.md:36-38`), §4.1 (no step for the repo-local design document), Appendix line 604 (lists the file as read).

**Violated invariant.** §1 rule 1 — "A ratified decision-log entry outranks both contract and code" — plus the §2 header's claim to be "a complete audit, not a complaint list". The object itself treats the same class of self-contradiction as reportable at row 9's note: "The contract contradicts itself inside one artefact."

**Reproduction.** `diff dev-docs/glade/GladeDeclSurface.md glade-decl/dev-docs/DeclSurface.md` → three differing lines (27, 30, 32). Line 27:
- root (`dev-docs/glade/GladeDeclSurface.md:27`): "canonical engines are `value`, `atom`, `log`, `stream`, `swmr`, `crdt`; `snapshot_delta`/`text_crdt` are profiles. Registry recognition does not grant runtime support. … `message` is unsupported; `window` is a view over an explicit base shape."
- glade-decl copy (`glade-decl/dev-docs/DeclSurface.md:27`): "the delivery-shape enum (`value`, `log`, `message`, `stream`, `exchange`, `window`, `swmr`, `crdt`) — names taut-shape engines, owns none of them; `text_crdt` is a profile over `crdt`."

The repo-local copy is pre-GDL-041: no `atom`, no recognition-vs-support clause, `message`/`window` as ordinary members. Lines 30 and 32 are also behind (no mount-instance clause, no GDL-040 Supplier row).

**Impact.** The repo being frozen ships a design document that contradicts the ratified catalogue the amendment exists to honour. §4.1.1 rewrites the schema comment to "name GDL-041 as the catalogue's owner" while leaving `dev-docs/DeclSurface.md:27` two directories away saying the opposite — a diagnosability defect that survives the freeze. R8 then proposes to amend "`GladeDeclSurface.md`" without saying which of the two divergent copies is controlling.

**Required correction.** Add a `Shape` row to table 2a (or an A-row to 2m) recording the divergence; add a §4 step for `glade-decl/dev-docs/DeclSurface.md` (at minimum :27, :30, :32, :58); and state in R8 which copy is controlling.

**Closure test.** `diff dev-docs/glade/GladeDeclSurface.md glade-decl/dev-docs/DeclSurface.md` is empty, or the glade-decl copy carries an explicit "mirror of `dev-docs/glade/GladeDeclSurface.md` @ `<sha>`" banner plus a drift check in `corpus/build.py --check`.

---

### [P3-1] L1-07 is cited for a proposition it explicitly contradicts; L1-08 for one it does not state

**Location.** Object §1 line 8 ("Publishing freezes the contract (L1-07, `gwz-dev/dev-docs/AgentProcessRules.md`)"), R6 line 355, R7 line 362.

**Violated invariant.** A cited clause must support the proposition it is attached to.

**Reproduction.** `AgentProcessRules.md:240-253` (L1-07, "Define freeze words precisely") reads: use "frozen" only when implementers may no longer choose its meaning … "**Never use any of these words as a synonym for implemented or released.**" The object's §1 uses it for precisely that equation (published ⇒ frozen). L1-08 (`:255-265`, "Amend; do not silently reinterpret") is triggered by "a frozen contract [that] is unimplementable, unsafe, ambiguous, or contradicted by evidence" — not by freezing something that is still open, which is what R7 attributes to it. R6's use ("the kind of claim L1-07 forbids") stretches a clause about four specific freeze words to a claim about `canonical_key`.

**Impact.** The document's trigger and urgency premise rest on an inverted citation. §5 gets the same point right via L1-09 (`:267-277`, "shared interface, compatibility rule"), so the document disagrees with itself about its own authority.

**Required correction.** Ground §1 on L1-09 plus `glade-decl/README.md:52-58` (the corpus oracle) and drop or restate the L1-07 claim; re-ground R7 on L1-07/L1-09.

**Closure test.** Every AgentProcessRules citation in the document quotes the clause verbatim beside its number.

---

### [P3-2] Eight file:line citations point at lines that hold something else

**Location and correction** (one root cause: citations transcribed without re-opening the line at the tuple):

| Object | Cited | Actually at the cited line | Correct location |
|---|---|---|---|
| Row 30, R1 (line 240) | `sysdata.taut.py:73` for "no share/key here — the ServeClaim selects the node, the mount fills domain/zone/key" | a bare `#` | near-text at `glade/node/ir/sysdata.taut.py:75-76` (semicolon, "**and** the mount"); the **verbatim** em-dash form is `grazel/apps/grazel-app.glade:20-21` |
| Rows 38, 39, 44 | `glial/src/events.ts:60` for `OriginMeta.origin`/`.seq`/`origin_meta` | `payload: value ?? new Uint8Array(),` | `glial/src/events.ts:59` |
| Row 40 | `events.ts:56` for `ChangeEvent.glade_id` | `shape: "value",` | `glial/src/events.ts:55` |
| Row 17, R2 (line 267) | `GlialClientRuntime.md:87` for GC-4 | past the table | `dev-docs/glial/GlialClientRuntime.md:86` |
| Row 49 | `gwz/src/live.ts:47` for `utf8(runId)` | `shape: 'log',` | `gryth-ui/packages/plugins/gwz/src/live.ts:48` |
| Row 49 | `gyld/src/ops/surfaces.ts:112` for `utf8(conversationId)` | `fill: { domain: GYLD_DOMAIN, key: { param: GYLD_ASK_CONVERSATION } }`; **no `utf8` anywhere in that file** | `gryth-ui/packages/plugins/gyld/src/live.ts:77` (also `:184`, `:257`) |
| §4.5.2 (line 514) | "`share_decl.ts:35,39` would break" if R1/R5 removed `DomainAnchor`/`Authority` | `:39` is `zone?: ZoneKind` | `grip-core/src/core/share_decl.ts:35` (DomainAnchor) and `:32` (Authority); `:39` is not at risk from either ruling |
| Row 16 | `router.rs:133`, `store.rs:409` for "node keys private zones `self:<id>`" | both lines are inside `#[test]` fixtures (`glade/node/src/router.rs:128-137`, `glade/node/src/store.rs:405-413`) | the node routes on opaque `key` bytes; it mints no `self:` prefix. (The claim is inherited from `GlialFitAssessment` §5:118-119, which words it more carefully as "routes on the `key` bytes") |

**Impact.** This document is the pre-freeze evidence record and is explicitly intended to be checked line by line. Each miss costs a later auditor a re-derivation; the `share_decl.ts:35,39` one and row 16's test-line pair also weaken the arguments they support.

**Required correction.** Re-open and correct each. **Closure test.** A mechanical pass that extracts every `<file>:<line>` from the document and prints that line, diffed against the claim, run before the document is filed as controlling.

---

### [P3-3] Three factual miscounts in the evidence tables

**Location and correction.**
1. Row 18 (line 126): "`glade/demo/src/manifest.ts:30,35,40,46,51,56` uses `from_cursor` on six surfaces, **five of them `value`-shaped**." Three are `value` (`:29`, `:34`, `:39`); the rest are `crdt` (`:45`), `log` (`:50`), `swmr` (`:55`). The six `from_cursor` line numbers are correct.
2. A5 (line 204): "`glade-decl-ts` is the **only** TypeScript member with an npm `package-lock.json` … and no `pnpm-lock.yaml`." `glade-wz/ggg-viz` also carries `package-lock.json` with no `pnpm-lock.yaml`; `grip-react-demo` carries both, like `grip-core`. The size (45 687 B) and `lockfileVersion 3` are correct.
3. §4.2 (line 436): "`taut-shape/corpus/` has **nine** corpora". `ls taut-shape/corpus/*.json` → ten. The load-bearing half — never two versions of the same corpus, one live file each — holds.

**Impact.** Low individually; together they weaken the "complete audit" claim, and A5's "only" is the sole premise of §4.8's pnpm note.

**Closure test.** Re-run the three counts; state the command beside each.

---

### [P3-4] The eight rulings are not disjoint, and no precedence over each other or over the SETTLED rows is stated

**Location.** §3 preamble line 213 ("Eight. Each is answerable in a line"), rows 14, 19, 33, R1(d), R2(a)/(c), R4 line 313, R7.

**Violated invariant.** "Answerable in a line" requires the eight answer-sets to be independent; otherwise a legal combination of owner answers is self-contradictory.

**Reproduction.**
- `DomainAnchor.deployment` is decided by R1 (option (d) deletes the enum) **and** by R7 (recommendation (a) ships it). Row 14 says "see R1 / R7"; nothing states which wins. R1(d) + R7(a) is a legal, contradictory pair.
- `RetentionPolicy.ttl` is decided by R2 (option (c) drops it) **and** by R7 (recommendation (a) ships it). Row 19 points only at R2; R7's question text claims it. R2(c) + R7(a) is the same collision.
- Row 33 (SETTLED) resolves to "document the mapping `sysdata.BindingDecl.app ≡ AdvertisementRecord.package`" — but §4.1.5 removes `AdvertisementRecord` under the recommended R7(b), voiding the resolution outright.
- R4's argument (line 313) says adding `crdt` to the node grammar happens "after **R3** adds `crdt` to its grammar". That is row 8, a SETTLED item; neither R3 option touches `crdt`.

**Impact.** The owner can answer all eight "in a line" and land on a set the document cannot execute; one SETTLED resolution is already void under the recommended set.

**Required correction.** Give each ruling an exclusive row list; add a one-line dependency statement (e.g. "R7 decides only `AdvertisementRecord`; the two enum members follow R1 and R2"). Fix R4's reference from R3 to row 8.

**Closure test.** Each of the 23 RULING rows appears under exactly one ruling, and no ruling's option contradicts another's recommendation under the recommended set.

---

### [P3-5] §4 is not the single edit it claims: three SETTLED items have no step

**Location.** §4 preamble line 403 ("One schema edit, one corpus version, three regenerations, the node, three consumers — in this order"), §§4.1-4.7.

**Reproduction.** Of the seven SETTLED rows and the five SETTLED artefacts, three appear nowhere in §4.1-§4.6 and have no gate in §4.7:
- **Row 33** — "document the mapping `sysdata.BindingDecl.app ≡ AdvertisementRecord.package`" (and see P3-4: void under R7b anyway).
- **A3** — "archive or banner [`glade/decl/*`] as superseded by `glade-decl/`". §4.6's must-not-change list does not mention it either.
- **A7** — `glade/client-rs/src/session.rs:27` error text still says "supported: value, log, swmr" while `:24` accepts `"crdt"`; A7 says it "rides the amendment", but §4.4 (the node section) has four bullets, none of them this.

**Impact.** The document's stated value is that the amendment is one commit. Three items missing from the enumeration mean a second pass after the freeze.

**Required correction.** Add the three steps, or state explicitly which SETTLED items ride a separate commit and why. **Closure test.** Every SETTLED row number and A-id appears in exactly one §4 step.

---

### [P3-6] A4 misstates how the rendering procedure is documented; §4.3 would delete three correct procedures

**Location.** A4 (line 203), §4.3's copy comment (line 470), against §4.2's own contradicting sentence (lines 432-434).

**Reproduction.** A4 makes three claims, all wrong:
1. "documented **twice** and differently" — it is documented in **five** places: `glade-decl/README.md:69-79`, `glade-decl/ir/glade_decl.taut.py:31-33`, and the three rendering READMEs (`glade-decl-rs/README.md:28-33`, `glade-decl-ts/README.md:29-34`, `glade-decl-py/README.md:32-37`).
2. "**Neither documents the copy step**" — `glade-decl-ts/README.md:33` is `cp ir/glade_decl.ir.json corpus/decl.v0.json ../glade-decl-ts/src/` and `glade-decl-py/README.md:36` is its py twin; `glade-decl-rs/README.md:32` documents the `build.py` step that stands in for it. These three are the *complete and correct* forms (see P2-2).
3. "**all three copies** are byte-identical to the contract's" — there are **four** copies in **two** renderings (`glade-decl-ts/src/{glade_decl.ir.json, decl.v0.json}`, `glade-decl-py/src/glade_decl/{glade_decl.ir.json, decl.v0.json}`), all four verified identical (`1e7d4d16…`, `f669da0b…`); `glade-decl-rs` has none — which §4.2 line 434 states correctly, contradicting A4.

**Impact.** A4 is classed SETTLED with the resolution "one procedure, including the copies". Collapsing onto the two incomplete contract-side forms would delete the three that are right, which is how §4.3's broken command block (P2-2) arose.

**Required correction.** Restate A4: five documented forms, three of them correct; the collapse must adopt the rendering READMEs' shape. Correct "three copies" to "four copies in two renderings; `glade-decl-rs` gates on the generated `src/vectors.rs`". **Closure test.** The single collapsed procedure is diffed against all five existing forms before any is removed.

---

## 2. Invariant analysis

Attacks that **failed** — these held and are part of the result:

1. **The arithmetic is exact.** I counted the rows of 2a-2m independently: 9+2+3+2+4+2+3+9+3+8+3+3 = **51**, and each sub-table header matches its row count ("8 members + 1 absent", "7 fields + 2 absent", …). Classes: **21 agrees** (rows 1,2,5,7,10,15,16,17,21,22,23,26,27,28,38,39,40,41,42,44,45), **7 SETTLED** (4,8,9,18,31,33,43), **23 RULING**. The 23 map onto the eight rulings as R1=4 (12,13,14,30), R2=5 (19,20,24,25,32), R3=2 (3,6), R4=1 (34), R5=2 (11,29), R6=5 (46,47,48,49,50), R7=3 (35,36,37), R8=1 (51) — total 23. Every total reconciles. (P2-4 challenges two *classifications*, not the count.)
2. **Every app-file number is right.** Counted by hand across the five files: **28** binding lines (7+7+7+6+1); shapes value **13** / log **13** / swmr **2**; retention latest **13** / `from-cursor` **13** / `windowed` **2**; zone commons **26** / private **2**; authority `share` **28**, `external` **0**. The line lists (`grazel-app.glade:22,23,24,25,32,33,34`; `:23` = `ws.files swmr`; `:24` = `ws.diff log share private from-cursor`; `:25` = `term.log … windowed`; `gyld-app.glade:42-48`, from-cursor at `:47,:48`) are all correct, and "13 lines across 5 files" / "2 lines" are exact. The two `grazel-app.glade` copies are byte-identical (`2722256b…`) with the dual-maintenance note at lines 10-13, as claimed.
3. **Every ratification claim is verbatim.** GDL-029 `:47` `open`; GDL-032 `:50`, GDL-035 `:53`, GDL-037 `:55`, GDL-038 `:56` **ratified 2026-07-07**; GDL-041 `:58` **ratified 2026-08-28**; GDL-039 `:59` "**open (implemented, pending ratify)**". All five GDL-041 quotes used in §2/§3 ("`message` is unsupported", "Recognition MUST NOT imply runtime support", "an application projection over an explicit base shape", "a separate correlated service path", the engine list including `atom`) are verbatim. GDL-037's file-form quote at `GladeDeclSurface.md:113-114` is verbatim. The whole §1 precedence paragraph survives its own check.
4. **The node citations are accurate.** `appdecl.rs`: `:15-22` grammar block, `:39-42` comment quoted verbatim, `:43` seven names with no `crdt`, `:44` `["value","log","swmr"]`, `:47` authorities, `:107-140` binding parse, `:119-124` the refusal, `:121` "exchange uses `service`", `:125-130` authority check, `:133` app, `:137` `toks[4]`, `:195-201` duplicate-id refusal, `:296` `bindings.len()==7`, `:352` `from_cursor`, `:408` `appended: 11`, seven `#[test]` functions. `sysdata.taut.py:78-84` (6 fields, no `domain`), `:79`, `:80`, `:83`, `:84`, `:19-25`. `store.rs:160-180`, `:171`. All correct.
5. **The shape-vocabulary reads are accurate.** `glial/src/shapes.ts` `atom` `:19-23`, `stream` `:39-43`, `requireShapeAdapter` throw `:72-75`, `requireExchangeShape` `:101-103`, `window` absent from `ADAPTERS`; `requireOpShape` admits `value|log|swmr|crdt` (`glade/client-ts/src/shapes.ts:26-28`) and `requireMountShapeAdapter` the same four — exactly matching every ✓/✗ in rows 1, 4, 8. `client-rs/src/supplier.rs:156`. A7's `session.rs:19-30` mismatch reproduces exactly.
6. **Row 9's self-contradiction claim is real and precisely stated.** `glade-decl/ir/glade_decl.ir.json` top-level `shapes` block lists exactly `unary value atom log stream swmr snapshot_delta crdt text_crdt`, verbatim as quoted, beside a `Shape` enum with no `atom`. The note "The contract contradicts itself inside one artefact" is correct.
7. **A1, A2, A3, A6, A8 all reproduce.** A1: all three pins are `99a04e0b960d03cbe92c0ec17321761eda860845` at `glade-decl-ts/src/index.ts:18`, `glade-decl-rs/src/lib.rs:38`, `glade-decl-py/src/glade_decl/__init__.py:21`; `99a04e0` ("Add SWMR declaration shape") is one commit behind `bbce73d` ("Add collaborative text CRDT demo", the commit that added `crdt=7`); all three renderings do render `crdt`. A2: reproduced — exit 1, `STALE: …/glade-decl-rs/src/vectors.rs`, and the 284/43-line diff is rustfmt line-wrapping; the tree was byte-unchanged after the run. A3: `glade/decl/glade_decl.taut.py:13,28-30` and `README.md:19-20` are exactly as quoted, including the "ttl / latest / from-cursor" hyphen spelling. A6: no `glade-sys.glade` in either workzone, and GDL-038's text does name it. A8: the vendored `{value:0, log:1, stream:2}` against source `{value:0, log:1, stream:2, swmr:3, crdt:4}` — exact.
8. **The two-`Shape` table in §1 is exact** for both enums, and §4.6's exclusion of the wire is consistent throughout (no ruling or §4 step touches `taut/corpus/glade.ir.json` or `wire-rs`).
9. **Every gryth-ui number is right.** Nine surfaces (1 gwz + 7 gyld + 1 taps), **6 value / 3 log**, **8** with explicit `domain: 'document'` plus 1 defaulted, `chat/src/live.ts:30` genuinely the only *fill* passing `zone`, **zero** glade-decl type names anywhere (the only mentions are `vite.config.ts:236,240` dedupe/override strings), the four `defineManifest` sites at `gwz/src/live.ts:32`, `gyld/src/ops/surfaces.ts:28`, `src/taps.ts:12`, `glade-chat/src/manifest.ts:77`, `DECLS` at `gyld/src/live.ts:238-244`, `runtime.ts:15` `createAtomValueTap`, `runtime.ts:31-32` vendor note, `groups.test.ts:23-40` decl-field assertions, `taps.ts:34` `'gryth-local'`, `surfaces.ts:64` `GYLD_DOMAIN='gyld'`. §4.5.3's sequencing warning checks out: `gryth-wz/glade-decl-ts` is a second checkout at `7e16e324630a`, clean, reached by `pnpm-workspace.yaml`'s `../glade-decl-ts` entry with `overrides: "@owebeeone/glade-decl": "workspace:*"` at `pnpm-workspace.yaml:25-29`.
10. **§4.2's corpus reasoning is sound.** `build.py:35` is a single hard-coded `GOLDEN`; `:33`/`:48-52` load exactly one schema; `README.md:52-58` describes one frozen oracle; the two gate lines (`corpus.test.ts:18`, `test_corpus.py:27`) are exact; `glade-decl-rs` gates on the generated `src/vectors.rs`; `taut-shape/release/compatibility.v1.json` exists as the cited precedent; and no taut-shape corpus carries two versions of itself. The conclusion "replacement, not coexistence" follows. (P2-3 is about the *list of sites*, not the conclusion.)
11. **R3(b)'s corpus-content claim checks out.** `edge/binding-message-private` and `edge/binding-deployment-window` both exist among the 26 vectors in `corpus/decl.v0.json`.
12. **§5's process claims are accurate** except for the §1 L1-07 slip (P3-1). L1-18's two-axis mandate at a release boundary, the 2026-09-18 D7 Surface amendment quoted verbatim including "a file format people edit", L1-17's settled-and-committed requirement, and L1-09's "shared interface … compatibility rule" — all verified at `AgentProcessRules.md:267-277, 370-386, 387-421`.
13. **The glial/FitAssessment citations hold.** `GlialClientRuntime.md:26-27` "not an engine" verbatim; `:83` GC-1 ruled 2026-07-07; GAP-14's "GQ-6 NOT implemented (seam left)" verbatim at `glial/dev-docs/DecisionLog.md:274`; `M.gwzOps.shape` is `"exchange"` at `:263`; `Surface … extends BindingDecl` at `:266`; FitAssessment §1 does carry four divergences and S6 is the `(DomainAnchor, ZoneKind, principal) → (share, key)` step at `:154`. `GladeZones.md:42, 119-126, 122, 123` and `glade-decl/dev-docs/DeclSurface.md:36-38` are verbatim.

---

## 3. Risks and next action

**Residual risks below the finding bar**
- §4.7 claims "58 vitest suites" in gryth-ui; the tree carries 60 `*.test.ts(x)` files. Not resolvable read-only (no `pnpm` runs permitted); if two are excluded by vitest config the number is right. Worth confirming before the check table is treated as a gate.
- Both DeclSurface copies list `BindingDecl` as `(glade id, shape, authority, domain, zone, retention)` — six fields, omitting `source`, which the contract has at field 4 and `glade-decl/README.md:31-32` lists. The audit never checks the contract against §Contents' own field list, so this divergence is unreported. It is adjacent to R5 and should be folded into whatever amendment sentence R5 produces.
- R6 asserts "`README.md:42-50` currently reads as if the interfaces are part of the contract". True for `canonical_key`; `README.md:49-50` already says derive_glade_id's "algorithm + its golden derivation vectors are deferred to the implementation step (N4)". The remedy still applies, but the diagnosis is half-right and should be narrowed to `canonical_key`.
- §1 describes `PackageExtractionPlan.md` step 1.1 as the step "which publishes `glade-decl-ts`". Step 1.1 (`:155`) is the decision-and-record step; publishing is 1.2 and 1.6. Harmless to the argument, but the object is cited as a pre-freeze record.
- Row 4 classes `Shape.stream` SETTLED on the ground of "recognized engine with no durable adapter". GDL-041 ratifies `stream` as an *engine*, yet `BINDING_SHAPES` refuses it, exactly as it refuses `message`/`window`. The class holds, but the reasoning makes `stream` indistinguishable from the R3 pair; a sentence saying why `stream` is not an R3 candidate would close it.

**Single next action.** Revise the object to resolve P2-1 through P2-5 — all five are text edits to the document, none of them touching code: add `glade/grip-share` to the consumer survey and re-cost R1; replace §4.3's command block with the rendering READMEs' generate-to-scratch-then-copy form; enumerate the full `decl.v0.json` retarget set (starting with `glade-decl-py/pyproject.toml:20`); split rows 18 and 31 so node-side enforcement moves under the open rulings; and add a §4 step plus an audit row for `glade-decl/dev-docs/DeclSurface.md:27`. Then run one mechanical pass over every `file:line` in the revised document (P3-2) and re-run the three counts (P3-3) before it is filed as controlling. The freeze must not proceed while `corpus/build.py --check` is red (A2, reproduced here), which L1-17 requires settled before the review loop rather than during it.
