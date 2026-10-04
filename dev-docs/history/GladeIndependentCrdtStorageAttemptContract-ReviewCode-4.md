# Redesigned CRDT storage-attempt typed contract — CODE-AXIS REVIEW

**Review object:** Final originating Code confirmation after confined nonarchitectural correction3; controlling DRAFT `dev-docs/GladeIndependentCrdtStorageAttemptContract.md` at root `fac445d74025f00c3d59574b2bbea1ebe14c665a`. Typed/model/compiling RED checkpoint only.
**Baseline:** Root `fac445d74025f00c3d59574b2bbea1ebe14c665a`; Glade `52fcbe5043d8178a917677d6c9461d771d3543e4`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources read directly, through scoped Git diffs and exact-pin comparisons.
**Date:** 2026-10-04
**Axis:** Architecture, interfaces, call graphs, ownership and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — prior Code GO is confirmed on this exact tuple. Code-2 P2-1, P2-2 and P2-3 remain closed. No new P0–P3 finding or architectural root.

---

## 0. Evidence base

Read completely: canonical Code-4 prompt, RemPlan-3, Remediation3-Evidence, corrected Contract §§1–9 and legitimate prior State-3 report. Retained the preceding full Code audits of controlling designs, process authority, package/testing boundaries, original consumers, production coexistence and frozen Gyld provenance.

Inspected the entire Glade correction range `c6c4239beecb129aa0585fe74006cc287dff3b87..52fcbe5043d8178a917677d6c9461d771d3543e4`:

- Recovery validator and surrounding ownership/restoration code, API `tests/support/mod.rs:40–310`.
- All five added public regressions and authentic-history helper, API `tests/public_contract.rs:856–1111`.
- Unchanged new-plan revision refusal and publication predicates.
- Root controlling contract changes against `fb6d69ff5153865266782f0b5c11d7a00086552c`.

The member diff contains exactly two development files: provider support and public tests. Both working files match the pinned objects. Shared API remains byte-identical, SHA256 `86cf4baf80aa532848a9a016e3c18a34a8896fd39a266919b84cf24ca06ac03d`; refusing kernel remains byte-identical, SHA256 `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.

Read-only comparisons confirm all23 prior public tests remain, with five added. Records22, recovery6 and storage-attempt15 sources are byte-identical to the preceding GO tuple. No core assertion, loop, text row or corpus payload changed.

Recorded evidence was audited against source, not rerun: API34 and core fixture/representation/source12 GREEN; all43 kernel consumers and ten text rows deliberately compiling assertion RED; canonical corpus control, selected compilation, formatting, lint and architecture checks GREEN. The chronology records executable negative regressions before corresponding validator changes and preserves earlier disclosed limitations.

No writes, builds, tests, Git mutations, network or live actions occurred. All five HEADs matched at start and end. No current State-4 report was read.

## 1. Findings

No new or open Code findings.

| Prior Code finding | Exact-tuple confirmation | Status |
| --- | --- | --- |
| **Code-2 P2-1: unissued historical lookup required to succeed** | Corrected Inspect40 registration, both issuance indexes, next41 and the issued/unissued historical pair are unchanged. The original receipt assertion remains compiling RED against the refusing kernel. | **CLOSED; nonarchitectural.** |
| **Code-2 P2-2: revision mismatch fabricated NonCommit revision** | New-plan expected-revision refusal remains before queued/Reserved exposure, after exact deduplication and unresolved-owner checks. All32 regression combinations remain. The new terminal check additionally rejects a NonCommit revision differing from its immutable binding. | **CLOSED; nonarchitectural.** |
| **Code-2 P2-3: reopen admitted incompatible limits** | Open still validates retained state and reservations before changing ownership/floors. The nine reduction cases and compatible same-generation retry remain unchanged. The strengthened historical validator executes through that existing path. | **CLOSED; nonarchitectural.** |

## 2. Invariant analysis

The State-3 counterexample is discriminated by [public_contract.rs:874](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/public_contract.rs:874): two authentic commits retain distinct identities and batches; changing only the second expected/result revision from1/2 to0/1 must now return Integrity.

[Provider:107](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs:107) collects committed revisions separately per complete instance and rejects duplicates. Existing expected+1 and terminal≤current checks bound each revision positively. Comparing finite set cardinality with the current head at139 therefore detects missing head, gaps and unbacked positive heads without iterating a caller-carried revision range.

Positive controls preserve sequential revisions1/2, repeated NonCommit outcomes at revision2, and independent X/Y commits at revision1. Recovery, compatible reopen and inspection repeat original bindings and terminal outcomes; the validator does not rewrite history.

Adjacent checks preserve closed phase grammar: NonCommit equals its binding’s unchanged expectation; Started/Committed cuts fit every immutable window. Reserved remains eligible for a later Begin refusal. Historical authorization is checked against its retained cut, without reauthorization against current policy.

The correction remains local validation using existing fields and standard collections. Shared interfaces, package roles, dependencies, wire/platform assumptions, assembly seams and application mutation boundary are unchanged. The accepted full-history/no-GC premise already excludes an import/base-revision exception. I found no contrary evidence requiring a new architectural context.

The correction is **nonarchitectural**, consistent with the legitimate prior classification. No new architectural root was found. The retained architectural count remains one. This is the expressly confined third-round exception under review-loop step5; it resets neither the object nor its cap. Any architectural root found in this round would require STOP for owner decision. This report authorizes no further corrective round.

Original lifecycle, finite issuance, callback authentication, receipt/custody, fork and merge obligations remain applicable. The twelve-package inventory, framework refusals, selectors and frozen Gyld allocation/provenance are unchanged.

## 3. Risks and next action

This GO confirms the internal typed contract, bounded provider and compiling RED checkpoint. It supplies no successful admission, physical persistence, external rollback, crypto, asynchronous cancellation, live duplex or activation qualification.

The next action is the owner’s aggregate exact-tuple gate using this Code confirmation and the independently filed originating State verdict. State-3 closure remains for that reviewer; this report does not substitute for it.