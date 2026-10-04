# Redesigned CRDT storage-attempt typed contract — STATE-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtStorageAttemptContract.md`, DRAFT correction 3, and its typed/model/compiling RED checkpoint at root `fac445d74025f00c3d59574b2bbea1ebe14c665a`.

**Baseline:** Workspace root `fac445d74025f00c3d59574b2bbea1ebe14c665a`; Glade `52fcbe5043d8178a917677d6c9461d771d3543e4`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources were inspected directly, through scoped diffs and against exact-pin Git objects.

**Date:** 2026-10-04

**Axis:** Durable-state semantics, restoration grammar, historical outcomes, ownership, finality and bounded capacity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — State-3 P2-1 closes. All originating State-2 closures remain valid. No new P0–P3 findings. This accepts the typed/model/compiling RED checkpoint only.

---

## 0. Evidence base

All five HEADs matched the prescribed tuple at start and end. No writes, builds, tests, network actions or Git mutations occurred. No current Code-4 report was read. Code-3 was read only as legitimate prior-round input.

Read completely: canonical State-4 prompt, RemPlan-3, Remediation3-Evidence and corrected Contract §§1–9. Rechecked review-loop step5, including its express third-round exception and architectural STOP condition.

Inspected the complete Glade correction range `c6c4239beecb129aa0585fe74006cc287dff3b87..52fcbe5043d8178a917677d6c9461d771d3543e4`:

- Recovery validator, API `tests/support/mod.rs:40–191`, and adjacent preparation, resolution and publication paths.
- All added public regression/helper source, `tests/public_contract.rs:856–1111`.
- Unchanged owned fixture restoration, core `tests/support/mod.rs:553–750`, including revision, terminal, custody and accounting coupling.

Inspected the root correction’s Contract changes, remediation evidence, review-history additions and member-pin updates. Retained the preceding full independent audit of controlling designs, repository instructions, policies, failed-IC1 history, consumers, physical reuse boundaries, architecture refusal fixtures and frozen Gyld allocation/provenance.

Read-only comparisons confirm exactly two changed Glade files, both matching their pinned objects. The Contract also matches its pinned object. Shared API and refusing kernel remain byte-identical, with the recorded SHA256 digests `86cf4baf80aa532848a9a016e3c18a34a8896fd39a266919b84cf24ca06ac03d` and `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`. Four other API files, twelve core files and six scripts have no changes.

Recorded verification was audited, not rerun: API34 and core fixture/representation/source12 GREEN; all43 domain consumers and ten actual-Taut rows compiling assertion RED; affected compilation, formatting, lint and architecture checks GREEN. The canonical corpus positive control remains GREEN. Evidence records behavioral RED before validator edits and preserves earlier TDD limitations.

### Originating closure table

| Finding | Disposition and evidence |
| --- | --- |
| **State-3 P2-1 — Duplicate committed revision** | **CLOSED.** The exact authentic two-commit mutation is retained at public test874. Per-instance committed-revision insertion now rejects its duplicate at provider107–117. Legal history and terminal repetition remain covered. Nonarchitectural. |
| **State-2 P2-1 — Unissued historical Inspect40** | **Remains CLOSED at contract/RED tier.** The complete registration helper, both authoritative/index assertions, original historical ExactRetry and paired unissued CallbackMismatch obligation are unchanged. |
| **State-2 P2-2 — Duplicate identity/overlapping unresolved owners** | **Remains CLOSED.** Complete identity uniqueness, queued/Reserved/Started exclusivity, revision agreement and combined instance bounds remain intact, with their negative images and historical/current/Y positive control. |
| **State-2 P2-3 — Unrestorable revision refusal** | **Remains CLOSED.** New-plan revision refusal still precedes queued/Reserved exposure and follows existing-plan recovery and unresolved ownership checks. Its all-four-kind matrix and valid publication controls are unchanged. |

## 2. Invariant analysis

The original State-3 counterexample is retraced exactly. The regression obtains authentic X commits at revisions1/2, verifies distinct attempt identities and batches, then changes only the second binding expectation1→0 and terminal revision2→1. The old validator accepted that contradictory history. The corrected validator rejects the second revision1 insertion without altering either binding, terminal or application head.

The required positive controls are substantive: authentic sequential commits1/2 restore; two NonCommit outcomes at unchanged revision2 remain legal; X and Y independently commit at revision1. Compatible reopening preserves exact attempts and revisions, and subsequent inspection repeats every original terminal binding/outcome.

Adjacent head checks preserve the existing full-history/no-GC grammar. Every retained committed revision must be positive, unique and no greater than the instance head. Requiring its finite set cardinality to equal that head consequently establishes coverage of1 through the head. Missing first/middle/latest terminal, raised head, erased history and unbacked positive Y head are rejected. The validator compares bounded cardinality rather than iterating a caller-carried revision range; numeric conversion is checked.

Adjacent immutable phase checks also hold. Started/Committed cuts must equal the original cut and fit all original windows; malformed intervals remain rejected by existing binding validation. Four window mutations distinguish lower/upper failures in both phases. Reserved remains recoverable when a later Begin would refuse its cut. Historical Started work is not reauthorized against current policy. NonCommit must preserve its binding’s unchanged expected revision, with an authentic altered-negative regression.

These checks do not weaken uncertainty or fencing. Temporary absence, unknown preparation and Started work remain unresolved; terminal fencing excludes later publication. Validation refuses inconsistent images before constructing ownership and never repairs them by deleting history or rewriting identities.

**Architectural-root classification:** No new architectural root or boundary defect was found. Duplicate-revision, head-coverage and phase-payload corrections enforce existing invariants using existing fields. Shared API, assembly, dependencies, roles, compatibility/platform assumptions and application publication boundary are unchanged. Originating context retention remains justified. The redesigned object retains **one architectural root**; failed IC1 history and the cap are preserved. The expressly confined third nonarchitectural exception is satisfied, and this verdict authorizes no additional patch round.

## 3. Risks and next action

The provider remains a bounded VolatileTest model; the kernel still preserves state and reports Unavailable. Physical barriers, external-I/O fencing, restart/antirollback, genuine proof domains, storage encoding and live duplex remain later qualifications. Existing locks/fsync do not establish composite attempt finality.

The next action is to combine this originating State GO with the independently filed Code confirmation on this exact tuple. Only that aggregate outcome can accept the typed/model checkpoint and advance the separately reviewed implementation gates.