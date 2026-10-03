# Q4-A legacy Store seal contract — CONSISTENCY-AXIS CLOSURE REVIEW

**Review object:** Bounded closure of original finding P3-1 at root `3bf731994c8f18e4e27ed568e123bed099dca2f0`, Glade `638cca4b2784cc3e51c1e47b1fea47d026b57734`, discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`, and external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. The object remains a contract/specification checkpoint with refusing scaffolds; no guard implementation is accepted.

**Baseline:** Original review tuple: root `6a35216a6d97aa22e9d53b54256f7395c306da3c`, Glade `19a269dd12b5f109d03e2361e3af4108d4b1fcbc`; discovery and external Gyld unchanged. Revised sources were read through `git show` at the exact supplied SHAs and compared against that reviewed tuple.

**Date:** 2026-10-03

**Axis:** Consistency closure of the original platform-selection counterexample. Independent, adversarial, read-only. No peer prompt or report was inspected, and this conclusion does not depend on another reviewer’s findings. Filed verbatim by the lane owner.

**Verdict: GO** — P3-1 is closed for the requested source/specification scope. Zero original findings remain open. Non-Unix runtime behavior remains explicitly unqualified.

---

## 0. Evidence base

Verified all four tuple entries at the start and end of this review. They matched the requested revisions and did not move.

Inspected:

- The exact member diff for `node/tests/legacy_store_seal.rs` and `node/src/store.rs`.
- All revised integration-consumer source, `node/tests/legacy_store_seal.rs:1–213`, including both platform modules.
- Revised root evidence, `dev-docs/GladeRaftQ4ASealEvidence.md:32–65`.
- The private publication scaffold and opening test boundaries, `node/src/store/legacy_seal.rs:1–120`.
- The root diff for the controlling seal contract and production integration plan. Neither changed from the original reviewed contract tuple.

No builds or tests were run during this closure review, as requested. The reported revised integration result—one pass and seven failures before guard implementation—is owner-filed evidence, not an independently rerun result. Likewise, the additional LS-003/006 RED counts are recorded evidence rather than execution verified here.

No files, reports, dependencies, or Git state were modified.

## 1. Prior-finding closure

| Finding | Original defect | Revised evidence | Disposition |
| --- | --- | --- | --- |
| P3-1 | Non-Unix selection excluded every existing-marker refusal consumer, leaving only Unsupported publication coverage. | Unconditional `common` module at lines 2–137 contains compatibility and existing-marker consumers. Successful publication remains inside `unix_profile` at lines 139–197; Unsupported/no-publication remains inside `unsupported_profile` at lines 199–213. | **Closed for source/specification coverage.** Non-Unix runtime qualification remains open. |

The original counterexample no longer survives platform selection.

Previously, `cfg(not(unix))` removed the complete module containing marker-recognition cases. A non-Unix implementation could ignore existing markers in open/append and still pass the entire selected target.

The revised `common` module has no conditional attribute. Its tests and supporting imports use platform-independent APIs. Consequently, non-Unix selection retains:

- Unknown regular-marker contents, pre-opened append refusal, fresh-open refusal, and exact journal retention: lines 74–84.
- Directory-marker refusal and lock-target I/O failure consumers: lines 85–103.
- Empty-marker refusal before torn-journal repair, with retained torn bytes: lines 104–120.
- Empty-marker refusal for duplicate, fork, and new append, plus fresh-open refusal, no proof creation, and exact journal retention: lines 122–136.

An implementation that leaves non-Unix open/append unaware of markers can no longer satisfy these selected behavioral assertions merely by returning Unsupported from `seal_legacy`.

The successful-publication consumers remain Unix-specific, consistent with the contract’s qualified publication profile. The dangling-symlink fixture remains inside that enclosing Unix module. Non-Unix publication retains its separate Unsupported and absent-marker assertions.

## 2. Invariant analysis

The correction preserves the distinction that P3-1 required: unsupported publication does not waive recognition of an already present fence. Moving the common consumers changes test selection without broadening the durability claim.

The conditional sections retain explicit enclosing module boundaries. No conditional attribute was left attached to a moved individual import or unrelated declaration.

The revised evidence accurately distinguishes shared source selection from actual platform execution: lines 44–45 expressly retain non-Unix runtime as unqualified. This closure therefore establishes that the specification includes those consumers on non-Unix; it does not establish their runtime success there.

The inspected publication scaffold still returns Unsupported. The revised Store method delegates to that scaffold, and the private append boundary remains refusing. No successful seal implementation is being inferred from the new tests or their recorded RED results.

## 3. Risks and next action

The original coverage defect is corrected. Real guard behavior, cross-platform execution, lock races, publication faults, and process-interruption recovery require subsequent implementation evidence and applicable acceptance review.

The next action is to proceed through the remaining authorized implementation gates, retaining the shared recognition consumers and the explicit non-Unix qualification limit. This report closes P3-1 only; it does not accept implementation, production activation, or RA-012 migration closure.