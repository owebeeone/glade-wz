# Glial Workflow Prompts

Use these prompts directly with Codex for consistent, high-signal collaboration.

## 1) Inspect Only (No Edits)
```text
Inspect these files only and report gaps/inconsistencies. Do not edit files yet.
Scope: <paths>
Output: findings by severity, assumptions, open questions, and recommended next steps.
```

## 2) Plan Then Wait
```text
Create a step-by-step plan for this task and stop for approval before editing.
Goal: <goal>
Constraints: <constraints>
Done-when: <acceptance criteria>
```

## 3) TDD Feature Implementation
```text
Implement this using strict TDD:
1) write failing tests first,
2) implement minimal code,
3) refactor with tests green.
Task: <task>
Scope: <paths>
Verification: run tests/lint/typecheck for affected modules.claude
Report exact commands run and results.
```

## 4) TDD Bug Fix
```text
Fix this bug with TDD:
1) add a regression test that fails on current code,
2) apply minimal fix,
3) confirm regression and related tests pass.
Bug: <description>
Scope: <paths>
```

## 5) Review Mode (Findings First)
```text
Review this change with a code-review mindset.
Focus on bugs, regressions, missing tests, contract breaks, and security/perf risks.
Do not edit files.
Output findings first with file:line references and severity.
```

## 6) Spec Consistency Pass
```text
Audit docs for consistency against GLIAL requirements.
Primary source: /Users/owebeeone/limbo/glial-dev/docs/glial-specs/GLIAL-DOC-013_Requirements.md
Scope: <doc paths>
Output: contradictions, undefined behavior, missing acceptance criteria, and proposed fixes.
```

## 7) Contract Hardening Pass
```text
Harden this spec to implementation-ready quality.
Add/clarify: message schema, required fields, error codes, retries, ordering, and compatibility notes.
Target: <doc path>
Keep normative language explicit (MUST/SHOULD/MAY).
```

## 8) Apply Approved Plan Step
```text
Execute only Step <N> of the approved plan.
Do not perform other steps.
Run verification for this step only.
Return changed files, command results, and any blockers.
```

## 9) Traceability Matrix Build
```text
Build/update a requirements traceability matrix.
Map each requirement ID -> owning spec -> owning module -> required tests.
Source requirements: /Users/owebeeone/limbo/glial-dev/docs/glial-specs/GLIAL-DOC-013_Requirements.md
Output file: /Users/owebeeone/limbo/glial-dev/docs/GLIAL_TRACEABILITY_MATRIX.md
```

## 10) Pre-Merge Validation
```text
Run pre-merge validation for changed scope.
Include: tests, lint, typecheck, contract compatibility checks, and doc impact review.
Return pass/fail summary, failing commands, and residual risks.
```

## 11) Safe Refactor
```text
Refactor for readability/maintainability without behavior changes.
Require existing tests to pass, and add characterization tests where coverage is weak.
Scope: <paths>
```

## 12) Grip-Core Gap Analysis (Augment vs Fork)
```text
Compare Glial requirements against current grip-core capabilities.
Classify each gap as:
- augment in grip-core,
- adapter in glial layer,
- requires fork.
Provide decision criteria, risks, and migration implications.
```

