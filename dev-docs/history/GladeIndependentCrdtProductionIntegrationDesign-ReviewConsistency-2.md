# IC-3A semantic production-integration design remediation 1 — CONSISTENCY-AXIS REVIEW

**Review object:** The SAME IC-3A semantic production-adapter object, remediation 1, comprising `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md`, `GladeIndependentCrdtProductionIntegrationPlan.md` and the review ledger at root `06b16c9e17ff5507268a5823fad9a0be70a36790`. Status: DRAFT, settled for independent review. Complete changed range: `4d641dd179e5b0bd94e84df9cb334d7a21dae371..06b16c9e17ff5507268a5823fad9a0be70a36790`.

**Baseline:**

| Repository | Verified HEAD at start and end |
| --- | --- |
| Glade workspace root | `06b16c9e17ff5507268a5823fad9a0be70a36790` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld application | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were inspected with `cat`, `sed`, `nl`, `rg`, scoped `git diff`, `git show` and SHA256 checks. External Gyld is `/Volumes/projects/limbo/gyld-wz/gyld`.

**Date:** 2026-10-04.

**Axis:** Consistency with the controlling graph, internal coherence, exact amendment scope and satisfiability of required evidence. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — no new P0–P3 findings established. The corrected semantics exclude the three original Consistency counterexamples. This verdict accepts semantic consistency only; it does not qualify typed interfaces, cryptography, disk recovery, network behavior or production activation.

---

## Prior-finding closure table

This is a fresh reviewer’s independent retrace. It does not replace the required originating reviewers’ closure testimony.

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| Consistency P2-1; convergent Safety P2-1 | Durable guard before consumption; coupled settlement and conservative reopen | D:370–477 requires durable establishment before raw history receive, including partial decoding. Failed observation and loss writes leave the existing floor guard active. Missing next-slot fallback preserves that guard. No-input kill and observed-input/all-writes-failed kill both remain incomplete. | Semantically resolved in this independent retrace. Physical closure witnesses remain required at B/C. Existing architectural root. |
| Consistency P2-2 | Complete declaration/schema identity and accepted-tuple mapping | D:108–160 binds declaration hash/version, schema identity/hash/version, canonical zone/key and the remaining accepted tuple. Trusted provision/open, evidence verification, images and HELLO must check it. Equal names/key bytes cannot establish equality. | Semantically resolved in this independent retrace. Typed and negative-mutation witnesses remain required. Existing architectural root. |
| Consistency P2-3 | Mandatory independent Rust/TS/Python vectors before remote use | D:164–188 and P:A2/C2 make complete independently derived canonical bytes/digests and malformed-input controls prerequisites to IC3C remote qualification. Missing language/category/pin/assertion fails the gate. | Semantically resolved in this independent retrace. Execution remains future work. Existing nonarchitectural root. |

## Changed-range analysis

The range changes the design, plan and ledger; adds the completed initial prompts/reports and merged remediation plan; and records two GWZ markers. No source implementation is introduced. The four source pins remain unchanged.

The material corrections are:

- A Records-owned pre-consumption lifecycle, including `begin_ingress`, owned `IngressPermit`, exact settlement and qualified never-started abandonment.
- A production `IdentityBinding` carrying the complete authenticated namespace alongside the existing internal descriptor.
- A mandatory three-language canonical-vector prerequisite, with explicit negative controls.
- Surface review for an actual operator/user-facing freeze, including a surface called private qualification.

I reviewed the corrected documents in their full original context, including retained authorization, storage attempts, node dispatch, inventory, reconstruction, compatibility and delivery obligations. The verdict is not limited to the added closure paragraphs.

Fresh full axes are justified: the mutation boundary and authenticated identity interfaces changed materially. Retained-context closure alone would not cover those changes.

No genuinely new architectural or nonarchitectural root was established. The cumulative classification remains **two architectural roots and one nonarchitectural root, with one completed merged remediation round**. Durable ingress and namespace composition remain the existing roots; the vector correction restores an accepted verification prerequisite. No third-root STOP is triggered by this report. Historical stopped objects and IC-2 counts remain intact.

## 0. Evidence base

The canonical round-2 Consistency prompt, root/member instructions, review-loop skill and canonical template were read. No current-round peer report or current originating closure report was read or requested.

The complete corrected design, plan and ledger were inspected. Pinned committed bytes and working-tree document hashes agree:

| Document | Lines | SHA256 |
| --- | ---: | --- |
| Design | 659 | `c476dd10dab4618710ba02620ea71a75f20b11570c373bb685ea13320b3909f2` |
| Plan | 232 | `81566df8b4eec445fe3fd1fd163af4534f8240c0a823cc348ae577510ee62f62` |

The completed initial Consistency/Safety reports and RemPlan-1 were legitimate prior-round inputs. Their report hashes match the ledger.

Controlling evidence inspected included:

- BuildEntry and its actual source-qualified problem capture; LibraryBoundaryAndTestingPolicy; GladePackageArchitecture.
- AdmissionPlan, ResourceConsistencyProfiles RC-001–007 and DecisionLog GDL-054–058.
- Archived AdmissionDesign, especially §§2–3, 5–9; StorageAttemptDesign §§2–8; retained historical AdmissionContract material; accepted StorageAttemptContract §§1–7.
- Admission review ledger and archived IC-2 implementation record, including acceptance scope, unknown-frontier and busy-historical-input limitations.
- Actual admission-core types, admission, staging, callbacks, projection and encoder; storage-attempt API identities, limits, Recovery and required methods.
- Actual node signing, RecordsFile, sysdir lock, NodeStart/node_plan, NodeAssembly, accept/server, peer HELLO, mesh and Iroh carrier seams.
- CarrierLink contract, particularly duplex operation, transport identity, channel binding and cancellation semantics.
- Canonical Substrate §6, NodeSigning decisions and CRDT adapter GCA-01–09.
- External Gyld architecture, independent-CRDT and storage-attempt declarations, including Admission/Records/StorageAdapter/NodeAssembly allocation.
- Glial’s exact ten-row independent-admission consumer and its digest-pinned released-Taut `concurrent_siblings` corpus.

Important source anchors include core `admission.rs:53–189`, `callbacks.rs:6–175`, `projection.rs:176–219`, API `lib.rs:200–266`, node `signing.rs:52–93`, `records_file.rs:141–199`, `sysdir.rs:110–150`, `accept.rs:195–253`, `lifecycle.rs:537–650`, carrier API `lib.rs:137–185`, and Iroh `recv` at `iroh_carrier.rs:848–883`.

Both start and end checks matched all five HEADs. Scoped document/source diffs against HEAD were empty. No writes, builds, tests, network actions, live actions or Git mutations ran. Recorded execution claims were inspected, not rerun or promoted into new qualification evidence.

## 2. Invariant analysis

**Durable observation and the original indistinguishability attack.** The corrected grammar supplies the previously missing durable discriminator. Guard establishment is floor metadata that can lead the selected image; image absence cannot erase it. Failed or uncertain establishment yields no usable permit. Once established, a guard survives failed observation persistence, failed loss-marker persistence, ordinary checkpoint/fence/lease updates and old-slot fallback.

The no-input-kill control deliberately retains uncertainty after restart. Live abandonment is narrower: Records-owned evidence must establish that receive/parse was never invoked and cannot occur later. Timeout, EOF, Drop, absent inbox and sender retry are explicitly insufficient. Therefore the two originally indistinguishable executions no longer permit the old complete response.

**Settlement and completion.** A normal exact empty round settles only itself. A malformed, partial or oversized round cannot become an empty manifest; bounded rejection or permanent loss must be retained. Successful settlement may retire the guard while inbox obligations remain, but combined completeness remains false. Guard retirement and observation selection share the same floor transition. A missing settlement image cannot authorize retirement.

D:550–582 retains the accepted kernel’s monotonic incomplete and integrity flags. Reconstruction accounts for exact digests, coordinates, kinds, requests, outcomes and closure under a captured generation/watermark. It cannot clear unknown provenance, permanent loss or an existing sticky kernel flag. This agrees with StorageAttemptContract §7 and IC-2’s documented limitations.

**Authenticated namespace.** The new table covers every component of the accepted AdmissionDesign §2 tuple. Declaration and schema identity are distinct from merge/admission profiles and key bytes. Canonical declaration/schema bytes are trusted provisioning inputs whose hashes must be recomputed; peer-selected replacement declarations are forbidden. Parameters must validate under the pinned schema and derive the signed key.

The wrapper adaptation is explicit. It does not pretend the existing Pure descriptor already verifies added fields merely because opaque canonical bytes contain them. Inner Op bytes, old re-exports and behavioral assertions remain protected. The negative witnesses cover equal-name/equal-key mismatches at open, verification/admission and exchange.

**Canonical evidence and authorization.** Creation intent and descriptor hashing avoid recursion. Signed proof domains are separately prefixed and zero-terminated, using the actual strict custom-domain signing functions. The permit wrapper separates the inner permit digest from the operation-proof signature it contains.

Local possession challenges bind node, instance, requester, operation, role and session. Peer HELLO remains channel authentication rather than requester authority. Current policy/time at protected start comes from independently injected authoritative observation, not `Begin.current`. A valid Started cut survives subsequent revocation; historical import checks original qualification while current serving rights remain separate. The direct-root/trusted-admitter assumptions are explicit and bounded.

**Physical ownership and terminal recovery.** The complete image includes full State, Recovery, issued and consumed requests, plans, bindings, terminals/conflicts, receipts, charges and reservations. Terminal-ahead-of-cached-core recovery must replay the original genuinely issued callback through the actual core. This matches its complete-request authentication and once-only installation requirements.

Reserved, Started, absence, canceled waiters and lost acknowledgments never imply NonCommit. Publication and fencing retain one winner. Same-logical-owner reattachment preserves callback namespaces while exclusive physical ownership and local-only workers exclude old execution. Replacement owners and external/detached workers remain unsupported.

The floor/image handshake distinguishes metadata generation from application revision and preserves issuance maxima on fallback. Its external floor trust is stated honestly: data-only rollback and cloning must refuse, while simultaneous rollback of both trusted roots is not claimed detectable.

**Independent progress and bounded state.** State, workers, slots, floors, sessions and guard capacity are per instance. Root locking supplies lifetime custody rather than a shared mutable allocator or CAS held by X. The promised Y progress remains conditional on Y’s functional storage. Full-history/no-GC retention and finite exhaustion stop new work without deleting custody or manufacturing a stronger answer.

**Inventory and real production routing.** Inventory includes accepted operations, original evidence/receipts, candidates, security rivals and qualified fork pairs. Same-slot rivals remain distinct digests. Newest or empty inventories cannot replace older obligations. Busy historical work is retained and scheduled around unresolved work rather than intentionally entering the accepted core’s sticky busy refusal.

The required node path names the actual Settings/NodeStart, node_plan, NodeAssembly, core and provider seams. Independent routing is descriptor-selected and bypasses holder/home/legacy Store admission. Qualification ingress must use that same runtime. Real Iroh duplex and two actual processes are required; manual client ferrying, synthesized Facts/receipts and a second successful model cannot satisfy the witness.

**Evidence and compatibility gates.** The new vector gate prevents two Rust processes from qualifying a format against only their shared encoder. Independent Rust/TS/Python derivation, expected bytes/digests, pins and assertion identities are required before remote use.

Allocation and dependency changes remain behind typed review and explicit architecture-gate adoption. The plan protects original consumers, ten text rows/corpus, disabled-platform inspection, rejecting mutants, fast package loops and process-global rules. Surface applies to an actual freeze despite private naming. Activation and stronger receipt promises remain separate.

## 3. Risks and next action

Implementation must enforce the semantic boundary at the actual transport read path. Existing Iroh `recv` buffers stream chunks and can retain bytes beyond the returned frame. Negotiation followed by pipelined history must therefore be covered by the owned consumption gate; a guard only around later verifier dispatch would violate D:370–410. This is a required future call-graph/fault witness, not evidence that current source is qualified.

Typed/physical review must also verify active guards across repeated slot rotations and local appends, root/path replacement during ownership, old-binary exclusion, alternate valid admission evidence for one operation, full-image reservations and shutdown drain ordering. This review establishes no physical result for those obligations.

The single next action is for the lane owner to merge the fresh full-axis reports with the separately required originating closure testimony on this exact tuple. Only after semantic acceptance may the authorized A2 typed/allocation/compiling-RED gate proceed. No source implementation or activation follows from this report alone.