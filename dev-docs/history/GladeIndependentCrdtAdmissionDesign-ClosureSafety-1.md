# Glade independent CRDT admission design — SAFETY-AXIS CLOSURE REVIEW 1

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionDesign.md`, DRAFT semantic design at root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`, with committed `GladeIndependentCrdtAdmissionDesign-RemPlan-1.md`. Focused originating closure of Safety findings P2-1 and P2-2 from root `bba04ad27311db50e3e6aedddb4d91780e4e4483`.
**Baseline:** Root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Revised normative sources were read with `git show <pin>:<path>`.
**Date:** 2026-10-04.
**Axis:** Safety — originating counterexamples and changed-range consequences. Independent, adversarial, read-only. No current fresh acceptance report or peer closure report was read. Filed verbatim by the lane owner.

**Verdict: GO — focused semantic closure.** Both original Safety P2 findings are closed at this revision. No new architectural root cause was identified in the inspected changes. This verdict does not replace the separately required fresh full acceptance review or qualify an implementation.

---

## 0. Evidence base

Verified all four HEADs at review start and end; each matched the required tuple without movement.

Read the complete committed remediation plan and the scoped design diff from `bba04ad27311db50e3e6aedddb4d91780e4e4483` to `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`. Re-read revised design lines:

- 84–114: namespace, canonical origin per epoch, and sequence mapping.
- 143–160: certificate, permit, and operation attribution.
- 216–245: historical admission versus current revocation.
- 261–287: outcomes, retained receipts, and projection.
- 306–405: historical qualification, fork quarantine, and exact future text witnesses.
- 463–479: restore and origin issuance custody.
- 639–642: revised ICD-006/007/009 requirements and T06/T07/T09.

Rechecked supporting Glial text-profile encoding/identity code and the existing canonical `concurrent_siblings` corpus to assess whether the proposed `AD` and `AB` witnesses require a different payload family or engine.

Inspection used `git rev-parse`, `git show`, scoped `git diff`, `sed`, `nl`, and `cat`. No writes, Git mutations, builds, tests, network calls, or live actions occurred. The witnesses remain future RED obligations; no execution result is asserted.

## 1. Prior-finding closure table

| Finding | Disposition | Closure evidence |
| --- | --- | --- |
| **P2-1 — A signed but never authorized rival can invalidate admitted history** | **Closed for the semantic design.** | Lines 225–231, 268, and 324–369 require independently qualifying historical admissions for both rivals. Bare, invalid, expired-without-prior-admission, and unknown-admission rivals cannot convict legitimate history. Lines 384–394 and T06/T09 require retained legitimate identities, unchanged receipts, and exact text `AD` in either order. |
| **P2-2 — Fresh origin epochs have no defined collision-free identity in the canonical engine** | **Closed for the semantic design.** | Lines 92–104 require a fresh canonical `Op.origin` for every epoch before signing/hashing, preserve the unchanged engine mapping, and reject origin reuse before mutation. Lines 395–405, 463–479, and T07/T09 require distinct old/new identities and exact text `AB`, while preserving old receipts and refs. |

No additional finding is raised by this focused closure review.

## 2. Changed-range and counterexample analysis

### P2-1: revoked/expired bare signed rival

The original sequence was: valid O and its dependent text are admitted; W is revoked and its permit expires; W signs a different same-slot O′ without any valid admission; the signed pair nevertheless causes retroactive quarantine.

The revised text blocks the decisive step. O′’s signature establishes attribution only. Without a qualifying signed admission record, historically authorized admitter, valid permit/time evidence, policy cut, payload, and proof closure, O′ cannot form a projection-affecting fork proof. Its arrival produces bounded security evidence or pending qualification, rather than degrading O, its dependents, or their receipts.

This applies in both evidence orders. A rival received first cannot enter Taut or advance an accepted head merely through its signature. When O’s qualifying evidence arrives, O remains the legitimate projection input. Conversely, a rival received after established O cannot remove that projection.

The correction also preserves the opposite case: a genuinely qualifying earlier admission remains qualifying despite subsequent revocation or expiry. Thus the remedy does not silently turn current authorization into a retrospective veto.

### Qualification remains independent of quarantine

Lines 337–347 address the circularity risk introduced by requiring both rivals to qualify. Historical qualification uses finite acyclic structural and authorization evidence. Supporting predecessor/dependency records require their own historical evidence, but need not remain projection-eligible after fork exclusion.

Consequently, two qualifying rivals continue to prove their conflict after quarantine. Quarantining them does not invalidate their historical qualification, undo the proof, and oscillate eligibility. Own predecessor hashes still bind exact bytes; disputed cross-origin slots do not acquire an arrival-selected winner. The resulting quarantine remains monotonic from the earliest qualifying fork.

The `AD` witness retains actual dependent text and exact operation identities. It also covers proven invalid payload/permit and unresolved evidence. This is a meaningful regression obligation, rather than a signature-only or operation-count assertion.

### P2-2: old/new epoch collision

The original sequence retained E0’s `(writer-a, seq0)` and certified E1 under the same `Op.origin`; both became Taut `(writer-a, 1)` despite distinct outer epochs.

Lines 92–104 now prohibit that representation. E1 must receive a fresh canonical origin before inner bytes are signed or hashed. Certification binds that origin, epoch, instance, and key. Refs, chain slots, retries, quarantine keys, and text writer identities use the same canonical origin; old histories retain their original identities.

The recovery witness therefore maps E0 `(writer-e0,0)` and E1 `(writer-e1,0)` to distinct Taut identities. E1 references E0’s eligible prefix without renaming or rehashing it. Delivery before the prefix is handled through dependency buffering. A new certificate naming `writer-e0` is rejected before mutation.

The exact `AB` witness also tests continuation after genuine quarantine while preserving the old common prefix. The revised custody rule prevents missing issuance evidence from being treated as permission to reset counters under a reused origin.

## 3. Risks and next action

The revision closes the two semantic defects without selecting production keys, storage, time adapters, wire encoding, or deployment defaults. Trusted-admitter/time honesty remains an explicitly declared assumption; this correction does not claim protection against a trusted admitter fabricating qualifying admission evidence.

The future tests must still be written RED and executed through the actual released text consumer. Physical custody, cryptography, and durable synchronization require their separate later gates.

The next action is to file this closure report and complete the fresh peer-blind full acceptance gate on the revised tuple. Implementation must continue to respect the remaining canonical-amendment, allocation, internal-contract, and compiling-RED prerequisites.