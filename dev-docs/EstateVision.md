# Estate Vision — the program in one document

Status: **DRAFT for adversarial review** (Fable dual requested by the operator,
2026-08-30). Drafted by the F5 lane from the operator's statement of the four
thrusts (2026-08-30, §2 — authoritative) and the estate's own vision corpus:
`gryth-wz/dev-docs/GrythVision.md` (C1–C3), `glade-wz/CLAUDE.md` (north star),
`glymik-dev/docs/Unified Platform Vision.md`,
`bizscad/dev-docs/DeclarativeManifesto.md`. Ground-level facts are in the
companion review `bizscad/dev-docs/EstateReview-2026-08-30.md` and are cited,
not repeated. Everything outside §2 is drafting, subject to the review and the
operator's correction.

---

## 1. The vision

**One sentence:** software an AI can safely see, extend, and operate while it
runs — assembled from declarative parts an AI can safely build.

**One paragraph:** A SaaS application here is not written; it is *declared and
assembled*. Every surface — data, UI, terminals, chat, workspaces, the
operator's own desktop — is typed, declared, flow-shaped data with providers
behind clean seams, so the whole running system is legible to a machine: an AI
participant can read the app's internals the way it reads a schema, not the
way it reverse-engineers a binary. Because humans and agents enter through the
same protocol door with the same capability checks and attribution, "the
operator clicks a button" and "an AI does it on command" are one feature.
Because the parts are declarative, an AI can also *build* new parts reliably —
that is not assumed but measured, by an instrument and an adversarial review
culture that treat the AI's output as a claim to be attacked. The endgame is a
loop: tell the AI what the product should do next; it inspects the declared
system, builds the feature as new declarations and providers, joins them to
the running app, and launches — live, attributed, gated. The stack, the tools,
and the method are all the same substance, so each product built makes the
next one faster.

This is the through-line of the estate's own founding documents: Grip made
UIs declarative because imperative UI state was unmaintainable; glade exists
because the GripLab *server* was "an imperative, stateful mess" (glade-wz
CLAUDE.md north star); gryth commits that everything on screen is a declared
surface (C1), the desktop itself is data (C2), and humans and agents are the
same kind of participant (C3); the manifesto and the (Δ,C) instrument make
"AI builds it reliably" a measured property instead of a slogan.

---

## 2. The four thrusts (operator-stated, 2026-08-30)

Stated by the operator; the wording below preserves his formulation. Some
items are experiments, some are tools, some are both.

1. **T1 — A very fast to build and launch SaaS stack using declarative
   systems.**
2. **T2 — Enable an AI to view my SaaS app internals (glade, glial, gryth)
   and take commands to build new features, join them up, and launch live.**
3. **T3 — Tools for that, built on the same framework (gryth).**
4. **T4 — Experiment with AI-built declarative systems (wyred, garns).**

And **gwz helps pull the bits together** — the multi-repo substrate that lets
AI lanes build across the estate as if it were one workspace.

### Proposed acceptance tests (drafting — for ratification)

- **T1:** one real application is assembled from the stack and launched for
  actual users, and the build is *timed*. The claim "very fast" acquires a
  number.
- **T2:** an AI, given a command, adds a feature to a **running** application
  — new declarations + provider — and it goes live without a redeploy-by-human,
  with the write attributed and capability-gated. (The *view/operate* half has
  a weaker test, already passing — see §4.)
- **T3:** the console used to direct T2 is itself a gryth application — the
  tooling eats its own substrate.
- **T4:** an experiment reaches the instrument's target quadrant (Δ≥3.5,
  C≥4 measured) *and at least one experiment graduates into a T1 component*.
  A hit that never becomes a part is a paper, not a thrust.
- **gwz:** already met — shipped (v0.11.1, three channels), dogfooded at both
  the meta level (agent lanes build with it) and the object level (the
  glade-gwz supplier and the gryth gwz panel put gwz *inside* the product).

---

## 3. The flywheel

The thrusts compose as a loop, not a list:

```
        T4 experiments mint declarative parts, under measurement
                             │ (graduation)
                             ▼
        T1 assembles parts into a launchable product stack
                             │
                             ▼
        T2 an AI operates and extends the running product
                             │
                             ▼
        T3 the operator directs T2 through tools on gryth
                             │
                             ▼
        gwz lets AI lanes do all of the above across many repos
                             └────────► back to T4/T1: each turn
                                        shrinks build time
```

The load-bearing property is that **all five run on the same discipline** —
typed declarations, provider seams, corpus-gated contracts, adversarial
review — so evidence earned in one thrust transfers to the others. That
transfer claim is itself reviewable (§5, V-6).

---

## 4. Map — vision to what exists (2026-08-30)

Evidence grades: **PROVEN** (a test, demo, release, or measurement exists) ·
**BUILT** (code exists; the vision-level claim is untested) · **ABSENT**.

### T1 — the declarative SaaS stack

- PROVEN: every layer exists and composes — taut 0.9 (released, 9 languages,
  corpus-gated) → grip-core/react (npm 0.2.x, four consumer workspaces) →
  glial (client kernel) → glade (Rust node, folds, iroh p2p, E2E stage-1
  audit) → grazel → gryth-ui. Composition exhibit: the 2026-08-29
  collaborative-text CRDT demo landed as one atomic six-repo gwz commit, the
  merge algorithm pinned to a released contract rather than reimplemented.
- BUILT: two data engines (garns: DSAPI→SQLite+Rust, M1–M5 oracle-gated;
  sledger: schema-ledger fold, Δ=3.3/C=5) — mutually unreconciled.
- ABSENT: the *fast-launch* property (no app ever assembled end-to-end and
  launched; no timing datum); security **enforcement** (stage-1 allow-all;
  glade P2 gated on WD-1/AZ-1..3); users/billing/hosting; a named first
  product (Everbility appears only as sledger's demo fixture).

### T2 — AI views internals, takes commands, launches live

- PROVEN (the view/operate half, at demo grade): declared surfaces make the
  system machine-legible; a headless participant drives the desktop through
  the same ops as the human chrome (passing seam test); the glade-gwz
  supplier executes allow-listed verbs over a declared exchange with
  failure-as-data; mock→real cutover with zero consumer edits
  (GlialWorkspaceNameCutover). C3's one-protocol-door with
  `(principal, on_behalf_of)` attribution is designed and stubbed.
- BUILT: the supplier pattern — the natural unit an AI would add — exists
  with two instances (chat, gwz) and ten written specs.
- ABSENT: the *build-and-join-live* half entirely — no mechanism for adding a
  supplier/surface to a **running** node (hot join, live launch); grants
  unenforced; attribution stubbed; **and no document names this thrust as a
  goal** — it exists in fragments (GrythVision north star, C1–C3, glymik's
  HybridApp vision). This document is the first place it is stated directly.

### T3 — tools on the same framework

- PROVEN: gryth-ui hosts three external consumers (wyred-ui as a full plugin
  family; the gwz panel; glade demo tabs) — the "tools on gryth" property is
  demonstrated, machine-enforced (no-React-state lint + scanner), and
  CI-checked at the consumer contract.
- BUILT: the desktop/window manager itself (7k LOC, 74 tests).
- ABSENT: desktop-as-data (C2) — the desktop is not yet serialized, persisted,
  or roaming, so the surface an AI would *drive* (environ scope) has no
  implementation; the integration branch is unmerged (`bffbdd0`, 7 weeks).

### T4 — experiments in AI-built declarative systems

- PROVEN: ga019 hit the target quadrant (Δ=5.0, C=5, oracle 5) on two real
  fabricated boards; wyred rebuilt it as ~41k LOC across 7 repos by 22 agent
  lanes with zero errors, gate-green on independent verify; garns ran M1–M5
  oracle-gated and honestly recorded a miss (Δ=3.0, C=4); sledger measured
  Δ=3.3/C=5 with 0% silent failures; the method is codified (instrument v2,
  `design-dsl` skill, BlindSide protocol, lobotomy tests, mutant registries).
- BUILT: the graduation path — wyred reclassified research-instrument
  (2026-08-05 kill); garns/sledger await the graduation decision into T1.
- ABSENT: external validity (BlindSide2 prescriptions unexecuted: no
  inter-rater kappa, no external author, no user contact — 6 of 7 triage
  decision cells blank); and any *causal* measurement of "AI builds more
  reliably WITH the declarative surface than without" — no A/B or ablation
  exists anywhere.

### gwz — the connective tissue

- PROVEN: v0.11.0/v0.11.1 shipped on three channels; R2-E accepted
  (2026-08-29); the gwz-log project is executing its adopted plan through the
  review loop right now; dogfooded meta- and object-level.

---

## 5. Load-bearing claims (what the review should attack)

| # | Claim | Current evidence |
|---|---|---|
| V-1 | Declared surfaces make a running system safely machine-legible and machine-operable | Demo-grade PROVEN (seam test, gwz supplier); enforcement ABSENT |
| V-2 | The stack's layers compose without bespoke glue | PROVEN once (six-repo CRDT commit); n=1 composition exhibit |
| V-3 | Declarative parts make AI *building* reliable | Measured for artifacts ((Δ,C)); causal delta UNMEASURED; external validity ABSENT |
| V-4 | Assembly from the stack is *fast* — days, not months | ABSENT (never attempted, never timed) |
| V-5 | An AI can extend a running app live, gated and attributed | ABSENT (no join mechanism; grants stubbed) |
| V-6 | Evidence transfers across thrusts because the discipline is shared | Asserted here; never examined |
| V-7 | One person + AI lanes can operate an estate this size, with gates as the compensating control | PROVEN in gwz; contradicted elsewhere by the ratification backlog and unversioned flagships (EstateReview §4–5) |
| V-8 | The two data engines are experiments, not duplication — a graduation decision, not a conflict | Framing adopted 2026-08-30; the decision itself is open |

## 6. What this vision is not

- Not a bid to win occupied product lanes with T4 artifacts — wyred competes
  with nobody; it is an instrument (ruling 2026-08-05, BlindSide2 thread 3).
- Not multi-backend/multi-cloud infrastructure — SQLite and a single node
  first; pluggability arrives via IR seams (sledger's compiled-artifact
  proposal), not up-front generality.
- Not a claim of external validation — that is T4's open ledger, worked on
  its own clock; T1–T3 ship regardless.
- Not a personal-productivity suite — tools exist to serve T2's loop, not as
  ends (the gvid/krona class stays parked unless it earns a thrust).

## 7. Decisions this vision forces (operator queue, vision-level only)

1. **Name the first product** (T1's acceptance test needs a subject —
   Everbility practice management is the standing candidate).
2. **Graduate one data engine** into T1 (garns vs sledger — or garns for
   query/bindings + sledger for evolution, decided explicitly).
3. **Charter T2** — this document is its first naming; the build-and-join-live
   mechanism needs a design doc and a first vertical slice.
4. **The glade P2 security decisions** (WD-1, AZ-1/2/3) — allow-all cannot
   meet any acceptance test above.
5. **Harvest gryth** (merge `bffbdd0`; then desktop-as-data is T2/T3's
   critical path).

## 8. Review charter (for the Fable dual)

Attack, at minimum: the coherence of §1–§3 (is the flywheel real or
narrative?); each §5 claim's evidence grade (especially V-3, V-4, V-6);
whether the §2 acceptance tests are the *right* falsifiers; what the vision
omits that will bite (sustainability of the one-operator model, the
external-validity debt, the unnamed product); and whether §6's fences are
honest or convenient. The operator's §2 wording is authoritative and not in
scope for rewrite — but its feasibility is.
