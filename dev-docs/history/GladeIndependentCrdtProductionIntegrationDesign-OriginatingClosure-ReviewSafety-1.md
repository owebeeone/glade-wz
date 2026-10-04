# IC-3A semantic production integration design, originating closure 1 — SAFETY-AXIS REVIEW

**Review object:** The SAME IC-3A semantic object after remediation 1: `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` and `GladeIndependentCrdtProductionIntegrationPlan.md`, DRAFT, dated 2026-10-04, at workspace root `06b16c9e17ff5507268a5823fad9a0be70a36790`. This testimony independently verifies my original Safety finding and checks the complete changed semantic range. It does not assert aggregate acceptance.

**Baseline:**

| Repository | HEAD verified at start and end |
| --- | --- |
| Glade workspace root | `06b16c9e17ff5507268a5823fad9a0be70a36790` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld application | `95a426595bba8e248a5f484272e483a070c73918` |

External Gyld denotes `/Volumes/projects/limbo/gyld-wz/gyld`. Sources were inspected with read-only `cat`, `sed`, `nl`, `rg`, `shasum`, `git rev-parse` and scoped `git diff`.

**Date:** 2026-10-04  
**Axis:** Safety: independently retrace the original observation-loss counterexample and attack corrected ownership, persistence, cancellation, recovery and completeness boundaries. Independent, adversarial, read-only. Other reviews run in parallel; nothing here relies on current peer or fresh-full testimony. Filed verbatim by the lane owner.

**Verdict: GO** — my original P2-1 is closed at the semantic-design stage. No additional P0, P1, P2 or P3 finding established. This is an originating closure verdict; fresh full axes and subsequent typed and physical gates remain required.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| Safety P2-1 — failed observation and loss-marker persistence leaves no durable restart discriminator | Accept; establish a durable, bounded, owned ingress guard before history consumption; settle it with exact retained evidence; preserve it through failure, cancellation and old-slot fallback | Independently retraced both original boundaries. If observation and loss writes fail before floor intent, the previously acknowledged floor guard survives and blocks complete reads after peer-absent restart. If an observation intent is durable but its next slot is absent or torn, recovery preserves the old image **plus** the active guard and cannot apply proposed retirement. The paired no-input execution also remains conservatively incomplete after an established guard and process death. | **Closed semantically.** Required typed behavioral RED and actual process-kill witnesses remain future qualification, not evidence claimed here. |

## Changed-range analysis

I inspected the complete design and plan and their full changes from `4d641dd179e5b0bd94e84df9cb334d7a21dae371` to `06b16c9e17ff5507268a5823fad9a0be70a36790`, together with the ledger changes and merged remediation plan.

The correction materially changes the receive-to-Records boundary. Recovery methods now include `begin_ingress`, `settle_ingress` and `abandon_ingress`; typed shapes include ReceiveGuard and an owned IngressPermit. The floor has a monotonic sequence independent of image generation and an authoritative guard ledger. Ordinary checkpoints, lease changes and interrupted image publication must preserve guards. Production receive, cancellation, reconstruction and combined completeness clauses explicitly consume those obligations.

This replaces the original unenforceable post-failure marker instruction through the normative replacement table at design lines 584–602. It does not introduce a kernel clearing event or change the accepted sticky-incompleteness contract.

The other material changes restore complete authenticated declaration/schema identity and make independent Rust/TypeScript/Python vectors mandatory before remote use. Their additions are consistent with the retained production scope: trusted provision, open, verification and HELLO must check identity; inner Op bytes and original Pure controls remain protected. Vector qualification is a prerequisite independent of browser activation. Surface review now explicitly applies to actual operator-facing configuration or APIs even when described as private.

The complete range also records the initial prompts/reports, merged plan and GWZ marker artifacts. Its changed-path inventory contains no member implementation, dependency, existing test or accepted source-contract change. The current scoped design/plan/ledger working-tree diff against the corrected commit was empty. The four source repositories retain their original pins, so the prior source-qualified context remains applicable.

**Root classification:** My closed P2-1 remains architectural: it concerned a missing durable consumption boundary and recovery state. It is the same root identified in the merged plan, not a new root introduced by naming guard operations. The identity correction addresses the already recorded second architectural root. The vector prerequisite remains the recorded nonarchitectural root. I established no additional independent root. This object therefore retains **two unique architectural roots, one nonarchitectural root and one completed merged remediation round**. Historical caps and stops remain intact; no third-root STOP is triggered by this testimony.

## 0. Evidence base

The complete canonical originating-closure Safety prompt was read. Its SHA256 is:

`01095ae0259f772fc42c239dc247ace497107df2543ed926d59aeed09726f3f6`.

The corrected documents matched the ledger:

| Document | Scope read | SHA256 |
| --- | --- | --- |
| Production integration design | Lines 1–659, including complete changed range | `c476dd10dab4618710ba02620ea71a75f20b11570c373bb685ea13320b3909f2` |
| Production integration plan | Lines 1–232, including complete changed range | `81566df8b4eec445fe3fd1fd163af4534f8240c0a823cc348ae577510ee62f62` |
| Original Safety report | Complete report and original counterexample | `7dda23b87d73a36eac9db41f8aaed2bfc3c180a616a68ffcad58231db69b5f46` |

I read the full IC-3 ledger and merged RemPlan-1. Particular corrected locations were design lines 55–95, 108–188, 246–336, 369–478, 480–602 and 604–659, and plan A1/A2, B2, C2/C3 and cap discipline.

Authority was rechecked against root `AGENTS.md`/`AGENTS_GWZ.md`, Glade instructions, the review-loop skill and canonical template. The original review’s source-qualified controlling context remains on unchanged pins: BuildEntry and its capture, library/package policy, resource profiles and GDL-054–058; admission/storage plans, accepted storage-attempt contract and archived design/implementation records; canonical substrate, CRDT adapter and signing documents; external Gyld allocation declarations; and the released Glial/Taut consumer and corpus.

Focused source retracing rechecked:

- Accepted storage-attempt contract §§4–7: authoritative start observation, coupled publication, full issued-request authentication, ownership, finite retention and sticky incompleteness.
- Actual storage-attempt API Recovery and required session lifecycle.
- Node `signing.rs:76–93`: strict verification through custom domains.
- `records_file.rs:141–199`: whole-snapshot CAS and rename/directory-sync uncertainty.
- `sysdir.rs:109–160`: existing unlinking lock lifecycle, which the new profile does not claim is sufficient.
- CarrierLink’s actual receive and close contract, including EOF semantics.

All five HEADs matched at both checks. No files were written. No builds, tests, Git mutations, network requests or live actions ran. The verification below is a semantic retrace, not physical fault execution.

## 2. Invariant analysis

**Original all-writes-failed sequence.** Begin with selected complete image G. Under design lines 371–410, application receive cannot start until Records has durably established and acknowledged the guard. Now receive authenticated history B, fail its observation write and the later loss write before either creates a floor intent, kill the process, restore storage and reopen without the peer.

The corrected durable state is G **and the active floor guard**. Reopen validates the floor first; guard uncertainty suppresses completeness before G can be served. Absent inbox data cannot discharge it. Recovery may preserve it pending or durably convert it to permanent loss. If classification persistence remains unavailable, completeness remains unavailable or false. The original false-complete outcome is prohibited by a retained witness rather than a process-only instruction.

**Indistinguishable no-input control.** Kill after acknowledged guard establishment but before any receive. Its durable state can be identical to the failed-observation execution. The correction deliberately gives both conservative results. It does not attempt to recover an unavailable never-started proof after restart. Conversely, failed or unknown establishment grants no usable permit, so history consumption cannot occur without the discriminator. These rules resolve the original information gap.

**Interrupted observation publication.** A durable intent with missing or torn next image cannot retire a guard. Lines 311–336 preserve guards from the selected floor/intent during fallback and forbid earlier guard-free floor restoration. A valid next image must be validated and re-synced before exact selection. Unknown selection acknowledgment cannot independently discharge the guard. Lost reply after successful selection instead recovers exact retained evidence and obligations. The mechanism distinguishes safe completed settlement from proposed settlement.

**Normal progress and empty rounds.** Exact finite message or authenticated snapshot/end defines the observation boundary. A normally retained round settles through one coupled image and matching floor retirement. Pending inbox work can remain, but combined completeness stays false. An authenticated empty round retires only its own guard; it cannot erase an older loss or obligation. Pausing receives between rounds permits ordinary idle connections without an indefinitely active receive guard. These positive cases are specified alongside conservative failure cases.

**Cancellation and ownership.** The coordinator owns receive callbacks and bytes. Cancellation first closes future access and joins/drains read, parse and callback work. Live abandonment requires Records-owned evidence that no receive/parse was ever invoked and none can occur later. Timeout, EOF after an invoked read, Drop, caller booleans and sender retry are explicitly insufficient. Pending close retains ownership. Restart cannot manufacture that live proof. This blocks a delayed callback from introducing history after guard retirement.

**Bounds and independent Y.** Guard IDs, retained history, round sizes and worst-case loss/discharge metadata have finite declared capacity reserved before consumption. Exhaustion prevents new receives without deleting evidence. At most one active round exists per instance; Y has independent floor, worker, guard and inbox capacity. Holding X’s receiver or floor operation therefore does not require a shared allocator or CAS that blocks otherwise functional Y. Shared-device failure remains outside that availability claim.

**Custody, callbacks and floor trust.** Existing all-four-kind coupling, original receipt preservation and complete issued-request authentication remain unchanged. Guard settlement cannot fabricate admission or promote transport acknowledgment into a stronger receipt. Reserved/Started uncertainty still requires genuine Inspect/Fence resolution; absence remains insufficient for NonCommit. Stable lifetime ownership, trusted external floor qualification and exact image selection remain necessary. The correction does not claim protection against simultaneous rollback of both roots, arbitrary replacement ownership or machine/power-loss durability.

**Authorization and identity.** Scoped strict signatures, genuine requester possession, current authoritative policy/time at protected start and historically valid Started cuts remain separate from transport authentication. The added IdentityBinding requires authoritative declaration/schema bytes and versions, canonical parameter validation and exact tuple checks. Equal names or key bytes cannot establish schema equivalence. The new pre-remote language gate requires independent encoding and malformed-input rejection rather than copied expected bytes or another encoder.

**Production and reconstruction.** The guard is placed before actual application consumption, including the first inventory header and partial decoding. Fixed negotiation may precede it only without history. Actual NodeStart/NodeAssembly/core/Records/CarrierLink routing remains required. Full inventory retains candidates, rivals, forks and original receipts; automatic duplex work derives from retained history. Reconstruction remains tied to exact generation, watermark, provenance and obligations. Neither a peer’s complete assertion nor a newer empty inventory clears kernel sticky state, guard uncertainty or permanent loss.

## 3. Risks and next action

The semantic grammar is now sufficient to close my original counterexample. Its implementation remains unqualified: typed ownership must enforce the permit boundary, floor/image validation must preserve authoritative guards, and real cancellation must drain every callback. Mandatory typed RED and physical kill/reopen tests must cover the paired executions, every floor/slot boundary, normal settlement, qualified live abandonment and independent Y.

The next action is for the lane owner to file this originating closure and merge it with the separately produced fresh full-axis verdicts on this exact tuple. Proceed to typed contract work only after that semantic gate accepts the object. This report authorizes no provider qualification, production activation, migration, seal change or stronger durability claim.