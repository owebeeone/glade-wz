# Independent CRDT admission design — CONSISTENCY-AXIS CLOSURE REVIEW 1

**Review object:** Originating closure of Consistency P2-1 in `dev-docs/GladeIndependentCrdtAdmissionDesign.md`, DRAFT semantic design dated 2026-10-04, at root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`.

**Baseline:** Original reviewed root `bba04ad27311db50e3e6aedddb4d91780e4e4483`; revised root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`. Unchanged members: Glade `c65a6e87f0c257c15de8db080c29d365a883af85`, Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources inspected through exact-pin `git show` and the scoped design diff.

**Date:** 2026-10-04

**Axis:** Focused originating verification of the canonical-origin/epoch collision and its required closure witness. Independent, adversarial, read-only. Fresh reviewers separately perform full revised acceptance; nothing here relies on their reports. Filed verbatim by the lane owner.

**Verdict: GO** — Consistency P2-1 is closed at the semantic design tier. No new finding arose within this focused closure. This verdict does not replace fresh full-object acceptance or establish executed implementation evidence.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Consistency P2-1 | Every certified epoch receives a fresh, never-reused canonical `Op.origin`, consistently bound through certification, operations, refs, receipts, quarantine and text identity. | Revised §§2/3/5/7 explicitly reject canonical-origin reuse before mutation and preserve old identities. The specified E0/E1 witness maps to distinct Taut operation identities and requires exact `AB` projection in both delivery orders. | **Closed — semantic correction verified; execution remains future work.** |

## Changed-range analysis

The scoped design diff adds the canonical-origin rule, propagates it into certificates and recovery, changes fork slot identity to the effective canonical origin, and gives ICD-T07/T09 an exact old-prefix/new-origin witness.

The patch also changes qualification of projection-affecting fork evidence and adds the `AD` witness described in the merged remediation plan. That is a material security-eligibility revision associated with another originating finding; it is not independently closed by this report. Its interaction with E0/E1 identity was inspected. It does not reintroduce the original collision.

The identity and eligibility revisions are architectural changes, explaining the fresh full review requirement. No **new architectural root cause** was identified in this focused closure.

## 0. Evidence base

Read in full:

- Committed `GladeIndependentCrdtAdmissionDesign-RemPlan-1.md`.
- Scoped design diff `bba04ad27311db50e3e6aedddb4d91780e4e4483..dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`.

Read revised design sections at the committed root:

- §2, lines 67–137, especially canonical origin and zero-based mapping at 92–114.
- §3 attribution, lines 139–166.
- §5, lines 306–405, including fork identity and exact recovery witness.
- §7 recovery, lines 461–479.
- ICD-006/007/009 rows, lines 639–642.

The original finding’s controlling source remains unchanged: `GladeCrdtAdapter.md` maps `Op.origin` directly to `CrdtOp.origin`. The root-lock-pinned Taut-shape-ts engine keys operations by `(origin, seq)`. Its text projection source, inspected at `1f2d6c4a9f7049afb7c0b183338cbf82d2618db3`, lines 24–75, projects the specified parent/child insertions as `AB`.

All four HEADs matched the revised tuple at both start and end. Inspection was read-only. No builds, tests, network calls, Git mutations, current fresh reports or peer closure reports were accessed.

## 2. Invariant analysis

The original counterexample is now expressly forbidden. A certificate for E1 naming E0’s canonical `writer-e0` must refuse before admission or store mutation (§2 lines 92–104; §5 line 405). An outer epoch field cannot be used to distinguish otherwise identical inner operation identities.

The permitted recovery trace is coherent:

1. E0 retains its eligible `A` insertion at Glade `(writer-e0,0)`.
2. Two qualifying E0 seq1 rivals and their causal descendants are quarantined; E0 seq0 remains eligible.
3. Authority certifies E1 under distinct canonical origin `writer-e1`.
4. E1’s initial insertion of `B` after `atom-a` carries a causal ref to Glade `(writer-e0,0)`.
5. The unchanged adapter maps these operations to Taut `(writer-e0,1)` and `(writer-e1,1)`, with the dependency mapped to `(writer-e0,1)`.

Those keys cannot collide. Prefix-first delivery integrates both operations; recovery-first delivery can buffer E1 until E0 supplies its dependency. The text payloads form the parent/child sequence `A` followed by `B`, yielding the required exact `AB`. This conclusion is a source trace, not an executed test result.

The correction also preserves custody boundaries. Old history, refs, text actor identities and exact retry/outcome keys remain attached to E0. New counters start only under E1’s fresh origin. Missing or contradictory issuance evidence leaves recovery pending (§7 lines 475–477), preventing a repeated display label from authorizing identity reuse.

ICD-T07 and ICD-T09 now require the formerly missing positive and negative witnesses: successful distinct-origin recovery with exact identities and text, and rejection of a new epoch reusing the old canonical origin.

## 3. Risks and next action

Canonical serialization, certificate issuance/custody, executable RED consumers and released-engine regression execution remain mandatory later evidence. No adapter or runtime is qualified by this semantic closure.

File this report as originating closure of Consistency P2-1 and use the separate fresh Consistency/Safety reviews to determine acceptance of the complete revised design.