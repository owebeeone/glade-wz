# Manager Prompt: Two Independent Rust Declarative Async API Designs

## Assignment and authority

Act as a design/evaluation manager. Launch **exactly two API-design agents**, in parallel where supported, to independently devise idiomatic Rust APIs for declarative async lifecycle orchestration. You own the shared evaluation setup, independent adversarial assessment, comparison, and handoff. The two agents own their separate proposals.

This assignment authorizes those two subagents, design documents, contract/test specifications, and small isolated signature/compiler experiments if useful. It does **not** authorize production implementation, demo changes, new user-owned tasks, a new GWZ repository, or additional review agents. Use the configured model unless the user explicitly specifies otherwise. If subagent tools are unavailable, report the limitation rather than presenting two personas from one generation as independent agents.

The desired outcome is not “port Python SDAX.” It is two credible Rust-native designs that maximize useful declarative leverage while satisfying the evaluator's correctness, coverage, explainability, and compositionality constraints.

## Required reading

You and **each designer** MUST read the following in full before design work:

1. `/Users/owebeeone/limbo/bizscad/dev-docs/DeclarativeApiEvaluatorV2.md`
   - This filename contains V2, but the instrument is internally **v3**. Apply that instrument, not an older rubric or a remembered summary.
   - Treat it as normative. Do not rewrite its weights, definitions, floors, or evidence requirements to improve a proposal's rating.
2. `/Users/owebeeone/limbo/glade-wz/dev-docs/LibraryBoundaryAndTestingPolicy.md`
3. `/Users/owebeeone/limbo/glade-wz/dev-docs/GladePackageArchitecture.md`

Follow applicable workspace instructions. If the evaluator is inaccessible, ask for the file before assigning score-driven design; do not invent its criteria. Other missing context MUST be disclosed. The requirements below supplement rather than replace the full evaluator.

SDAX reference: <https://pypi.org/project/sdax/>. Inspect its documented semantics as inspiration and distinguish source-backed claims from design choices. If a Rust lifecycle research report is already available, freeze one version as common input to both designers. Do not wait for an unfinished research task; record that it was unavailable. Findings arriving later belong in a separately identified follow-up, not an undisclosed change to the comparison inputs.

## Shared owner brief

- Glade will make many async calls. Repeated startup, failure, cancellation, and shutdown logic should be centrally orchestrated through explicit lifecycle contracts.
- Derive as much safe concurrency and necessary sequencing as practical from declarations, respecting dependency, ownership, and resource constraints. Do not equate maximal task count with useful parallelism or impose global phase barriers without justification.
- Use Rust idioms and ownership/type checking where they help. Do not reproduce Python-shaped dictionaries, reflection, or untyped result bags merely to resemble SDAX.
- Channels, actors, dedicated tasks, a runtime, and macros are choices to justify, not required architecture. Prefer asynchronous I/O; justify bounded worker/offload use where needed.
- Keep dependency boundaries small and tests fast. Public replaceable services need meaningful traits and shared conformance tests; pure data/logic do not need artificial marker traits.
- Canonical contracts and tests MUST precede production implementation, followed by adversarial review. The working demo MUST remain untouched; any future replacement will coexist until a reviewed cutover.
- The generic lifecycle library MUST NOT depend on Glade's concrete transport, discovery, trust, storage, or node packages. Glade is a use case, not the entire API's vocabulary.
- Cleanup of owned resources, compensation for externally visible effects, and durable crash recovery are separate concerns. State their boundaries honestly.

## Phase 1 — Freeze a fair evaluation before launching designers

Prepare a versioned evaluation charter using A.0 of the evaluator:

- RET v1 plus a named/versioned lifecycle extension if needed; include all universal classes, and register domain additions and tiers before design. Each candidate declares its actual archetypes; document any difference in live E-classes rather than silently changing the denominator.
- Primary author model `llm`; secondary `human` assessment only where evidence supports it.
- `design-stage` as the default mode. A compiler experiment does not turn the unimplemented lifecycle engine into a measured candidate.
- Explicit ship boundary: when effects first become externally observable or persisted, not when a convenient validation command finishes. Distinguish pure planning from effectful acquisition.
- Default evaluator floors. Do not invent scalar weights; the user has not supplied any.
- A shared corpus of **at least 30 distinct, source-grounded intents**, with at least one third held out. Use owner requirements, SDAX/source documentation, established substrate contracts, issues, or prior hand-coded solutions. Do not pad the count by relabeling the same case.
- Each intent includes ID, natural-language request, canonical minimal structured intent, expected result/trace or an explicit unresolved oracle, provenance, RET tags, and split assignment. Analyst-derived expectations are not automatically independent golden outputs.

Freeze the corpus, split, measurement units, floors, and source versions/hashes before design iteration. Expose only the training portion to designers. Keep held-out cases and expected outputs manager-only until the two initial proposals are frozen. Broad required capability families below are public; exact held-out cases are not.

If the source corpus or independent oracle evidence is insufficient, record the deficit. Do not invent evidence, claim a passed R5 floor, or call a provisional comparison a fully calibrated evaluation. You may still obtain clearly labeled exploratory proposals.

Part G requires calibration beyond comparing two designs. Record available anchor/held-out-surface evidence and any unmet calibration requirements. Do not claim full Part G compliance from this two-agent exercise or silently expand into extra agent assignments.

## Phase 2 — Dispatch two genuinely independent proposals

Launch Designer A and Designer B with fresh/empty conversational context where the tools support it (`fork_turns="none"` or equivalent). Supply each the same complete owner brief, required-reading paths, frozen charter, training corpus, deliverable contract, and its separate output directory. Do not rely on an inherited manager conversation to communicate requirements.

Tell both designers:

> Develop your own organizing model and idiomatic Rust API. Optimize the evaluator's separate dimensions without gaming them. You may choose a typed builder, resource/dependency graph, capability/constraint declarations, macros, or another defensible approach. Explain alternatives you rejected. Do not read the other designer's files, drafts, scorecards, or the manager's held-out material. Do not spawn other agents. Return a frozen proposal before any cross-comparison.

Do not seed one agent with the other's answer or preassign both to the same architecture. If they independently converge, report that; do not manufacture novelty through cosmetic syntax changes.

Use separate A/B files. In a shared filesystem, this is instruction-based blinding, not a security boundary; state that limitation. Record any accidental exposure and do not continue describing that portion as blind.

While they work, perform useful independent work: prepare the evaluation procedure, requirement-to-test map, and adversarial probes. Send substantive clarifications to both designers identically. Keep progress updates concise.

## Required content of EACH designer's proposal

### A. Semantics and declarative surface

- Define the meaning of declarations independently of the execution engine: states, transitions, effects, traces, and invariants. Identify where multiple schedules are valid and what results are promised to remain equivalent.
- Identify every coherent surface, its denotation, gate, and escape boundary. Split off imperative callbacks rather than claiming the entire library is declarative. Name and evaluate seams separately.
- Demonstrate programs-as-data, inspectability before execution, explicit/local ordering, and non-trivial derivation. Explain what the author says versus what the engine derives.
- Prefer declarations of intent, capabilities, requirements, ownership, and policy over manually assembled control flow where the inference is safe. Do not hide necessary intent in surprising defaults.
- Explain resolved-plan inspection, “why this dependency/selection/order,” dry-run boundaries, and diagnostics. For state-dependent plans, state what can be known before effectful execution.
- Show nested reusable components, closure/composition, configuration/version evolution, and visible, contained imperative escapes. Do not claim arbitrary Rust callbacks are sandboxed or side-effect-free.

### B. Rust API and architecture

- Provide representative public traits/types and coherent usage examples: happy path, partial initialization, cancellation, cleanup failure, nested composition, and bounded concurrent work. Include at least one non-Glade example.
- Explain ownership, borrowing, lifetimes, typed outputs, error representation, `Send`/`Sync`/`'static` constraints, generic versus dynamic dispatch, and allocation/type-erasure tradeoffs. Label proposed syntax and uncompiled examples honestly.
- Distinguish the authoring surface, inspectable plan/IR if any, deterministic planning/state-machine logic, execution implementation, and runtime adapters. Propose only package splits with a real isolation benefit; avoid a large universal common crate.
- Evaluate metaprogramming explicitly. If using macros, justify them against an ordinary Rust API, show representative generated code, describe diagnostics and compiler checks, and keep runtime semantics independently testable. If not using macros, explain why.
- Our existing source architecture checker does not expand macros. Any proposed generated public contracts or async test forms need explicit compiler witnesses and a tested checker-support plan, not a blanket exemption.

### C. Lifecycle guarantees and limits

Specify, rather than leave to implementation intuition:

1. Dependency/readiness/completion distinctions; finite work versus long-lived services; shared/exclusive resource conflicts; ready-work scheduling and limits.
2. Acquisition bookkeeping, cleanup ownership, and partial initialization, including failure/cancellation after an effect but before a resource handle is returned.
3. Cancellation before start, during an await, after external success but before acknowledgement, and during cleanup; dropping the caller future versus cancelling owned child work.
4. Sibling failure policy, panic policy, joining/draining, multiple-error aggregation, retry attempt boundaries, idempotency, and ambiguous external outcomes.
5. Which cleanup runs, prerequisites it needs to remain alive, ordering, possible concurrent cleanup, cleanup failure, shielding policy, and shutdown deadlines. Reverse graph order alone is not proof of correctness.
6. What happens when cleanup is non-cooperative, a child outlives its intended scope, the executor stops, or the process crashes. Do not promise that every async action can be safely aborted or rolled back.
7. Dynamic declarations/graphs, scope lifetime mismatches, cycles, missing and ambiguous providers, and static versus runtime validation. Do not infer hidden callback dependencies magically.

### D. Canonical tests before implementation

Provide requirement IDs mapped to success, failure, and edge tests, with inputs/schedules and expected observations, not only test names. Include malformed declarations and mutants for the live RET classes.

Separate:

- Compile-pass/compile-fail interface witnesses.
- Pure deterministic planner/state-machine tests.
- Shared implementation conformance tests with fake clocks, controlled outcomes, and injected schedules.
- Actual runtime/adapter tests and slower stress/model-checking experiments.

Tests MUST derive from declared semantics and external intent evidence, not from the same generator that emits the implementation. Specify no real sleeps or network requirements for the fast suite. Give proposed fast-test commands/budgets as targets, not fabricated measurements; distinguish test execution, warm incremental compilation, and cold compilation.

Do not implement the production engine. Any optional compiler witness must stay in an isolated experiment, start with its intended test/assertion, and report actual results. A stub, panic body, or passing signature check is not behavioral conformance.

### E. Evaluator card and honest optimization

Apply the full v3 instrument, including its Part H template:

- Report Δ.1 with `(L, S, T)` and expansion-ratio evidence, Δ.2 with routinely exercised derivation classes, and C separately. Never average them or assign a default scalar.
- Use `design-stage` C intervals, written detection procedures, and proposed mutants. Unknowns remain unknown; evidence-free fields are `unscored`, not zero. Any unmeasured projected quantities must be flagged.
- Separate `deleted` footguns from legitimate intents made inexpressible (`absent`); identify valid programs that might be spuriously rejected.
- Address R1–R7, seams, closure, engine/evolution/cost logs, and latent/silent bug debt. Do not invent engine LOC, harness coverage, held-out performance, or Part E authoring-harness measurements.
- Designers cannot score unseen held-out cases: leave those fields for the manager. Put the self-scorecard in a separate file so the manager can assess the proposal without reading its self-rating first.
- High generativity is not a license to add unnecessary search, surprising defaults, hidden ambient order, or unverifiable guarantees. Explain the implementation complexity bought by every major derivation feature.

Finish with the smallest useful staged implementation, open decisions, and the most serious objection to your own design. Preserve a generic-core path and an opt-in Glade integration path.

## Phase 3 — Freeze, challenge, and compare

1. Collect and freeze both initial proposals before inspecting their self-scorecards or exposing held-out cases. Record artifact versions/hashes.
2. Assess each proposal independently from its semantics/API/test specifications against the same charter and held-out corpus. Record your ratings before opening its self-rating. This is a second assessment, not a claim of full blinding to design identity or a measured engine run.
3. Attempt concrete counterexamples: partial acquisition leaks, cleanup dependency torn down too early, cancellation interrupting cleanup, ambiguous retry outcome, nested-scope lifetime escape, multiple simultaneous failures, conflicting inferred providers, and budget-induced deadlock/starvation. Document a trace or mutant and the exact contract that should reject or handle it.
4. Apply the design-stage rules and floors. A proposed check with no written decision procedure remains unknown. Mark missed legitimate intents as coverage failures rather than quietly adding imperative escapes to the declarative count.
5. Compare self-ratings with your assessment, retain disagreements and evidence, and do not average away uncertainty. With only two projected designs, do not manufacture measured rank statistics or frontier membership. Specify the future mutant/harness work needed to resolve disagreements.
6. Optionally allow **one bounded revision round** using the existing two agents. Give each its own findings and identical policy clarifications, not the rival design. Preserve initial results. Once a held-out case is revealed, mark it consumed; never present improvement on it as untouched held-out validation.
7. Recommend a candidate, a small evidence-justified combination, or further validation. Do not average incompatible surfaces into a “best of both” API. Identify concrete tradeoffs, unresolved blockers, and what the owner must decide before implementation.

Do not start production code after the comparison. The next gate is owner review of the interfaces, canonical tests, and adversarial findings.

## Deliverables and output scope

If the workspace is available, use a new run directory under:

`/Users/owebeeone/limbo/glade-wz/dev-docs/research/rust-async-api/`

Keep all new artifacts inside that run directory; check for existing work rather than overwriting it. Suggested contents:

- `EvaluationCharter.md` and versioned corpus/provenance/held-out evidence (manager-owned).
- `A/Proposal.md`, `A/CanonicalTests.md`, `A/SelfScorecard.md`.
- `B/Proposal.md`, `B/CanonicalTests.md`, `B/SelfScorecard.md`.
- `AdversarialReview.md` with requirement IDs, counterexamples, severity, and unresolved findings.
- `Comparison.md` with a short non-Rust-specialist executive recommendation, side-by-side syntax for the same intents, separate surface/seam assessments, projected scorecards, and an implementability/compile-cost discussion.

The final handoff MUST link both proposals and the comparison, distinguish evidence from projections, disclose blinding/calibration/access limitations, and state whether the next step is contract revision, an isolated validation experiment, or owner approval. Do not commit files, alter architecture policy to pass a check, modify the demo, or imply the proposed APIs are adopted.
