# IC-3A semantic production integration design remediation 1 — SAFETY-AXIS REVIEW

**Review object:** SAME IC-3A semantic production-adapter design, remediation 1: `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md`, `GladeIndependentCrdtProductionIntegrationPlan.md` and governing ledger at workspace root `06b16c9e17ff5507268a5823fad9a0be70a36790`. DRAFT, dated 2026-10-04. Complete changed range: `4d641dd179e5b0bd94e84df9cb334d7a21dae371..06b16c9e17ff5507268a5823fad9a0be70a36790`.

**Baseline:**

| Repository | HEAD verified at start and end |
| --- | --- |
| Glade workspace root | `06b16c9e17ff5507268a5823fad9a0be70a36790` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld application | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were read through `cat`, `sed`, `nl`, `rg`, scoped Git inspection and SHA256 checks. External Gyld is `/Volumes/projects/limbo/gyld-wz/gyld`, not its enclosing workspace repository. Scoped working-tree diffs for the controlling documents and inspected Glade implementation paths were empty.

**Date:** 2026-10-04.

**Axis:** Safety: attack degraded and mixed-version paths, authorization, irreversible publication, custody, recovery, completeness, ownership and reachable stuck states. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero new P0, P1, P2 or P3 findings. This accepts the reviewed semantic packet on this axis only. It establishes no typed-interface, cryptographic, physical-storage, network or activation qualification.

---

## Prior-finding closure table

This is a fresh full review. The table independently retraces the previous Safety finding; it does not replace the originating reviewer’s required closure testimony.

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| Initial Safety P2-1, sharing the architectural root of initial Consistency P2-1 | Establish a durable receive guard before history consumption; preserve it through failed observation/loss writes, old-slot fallback and reopen | D:371–402 prohibits consumption without confirmed durable establishment. D:432–456 preserves uncertainty after both later writes fail and after process death. D:458–478 explicitly covers the original no-input/observed-loss pair, absent/torn next slot, lost reply and positive controls. P:118–131 requires later typed and actual-process witnesses | Original semantic counterexample is resolved on this independent retrace. Formal originating closure remains a separate ledger prerequisite; physical execution is not claimed |

## Changed-range analysis

The range contains ten changed paths: design, plan, ledger, initial canonical prompts, prior completed reports, RemPlan-1 and two GWZ marker records. It contains no source implementation, manifest, accepted API, old consumer or corpus change.

The substantive correction adds three connected boundaries:

- IC3-GUARD-001–003 introduces Records-owned establishment, consumption, settlement and qualified abandonment, including floor-only metadata and crash recovery.
- IC3-ID-001 completes the authenticated namespace through signed declaration/schema identities and a production IdentityBinding.
- IC3-CANON-001 makes independent Rust/TypeScript/Python canonical vectors a prerequisite for remote use.

The revised replacement table at D:584–602 names these changes explicitly. It retains storage issuance, protected start, four-kind coupling, callback authentication, sticky kernel incompleteness, canonical inner Op bytes and legacy routes. The Surface language at D:653–659 and P:209–215 prevents “private qualification” from bypassing an actual operator-facing freeze.

I checked the corrections against the surrounding original contracts, rather than treating their new wording as sufficient authority. The guarded observation path does not acquire permission to clear the kernel flag; the identity wrapper does not acquire authority from merely carrying additional bytes; the vector gate does not substitute for genuine signatures or physical recovery.

No genuinely new root was established. The existing accounting remains **two unique architectural roots, one nonarchitectural root, one completed merged remediation round**. Guard and identity corrections address the two recorded roots; canonical-vector correction restores the recorded verification prerequisite. Nothing in this review resets historical caps or authorizes another patch beyond them.

## 0. Evidence base

The complete generated Safety-2 prompt, review-loop skill and canonical template were read. Root and Glade instructions, external Gyld workspace/application instructions, and both relevant review ledgers were inspected. No current peer report or current originating closure report was read or requested.

The full corrected design, plan and IC-3 ledger were read. SHA256 checks matched the ledger:

| Document | Lines | SHA256 |
| --- | --- | --- |
| Production integration design | 1–659 | `c476dd10dab4618710ba02620ea71a75f20b11570c373bb685ea13320b3909f2` |
| Production integration plan | 1–232 | `81566df8b4eec445fe3fd1fd163af4534f8240c0a823cc348ae577510ee62f62` |

Here **D** denotes the production integration design and **P** its plan.

Controlling evidence inspected included:

- BuildEntry and its source-qualified problem capture; LibraryBoundaryAndTestingPolicy and GladePackageArchitecture.
- AdmissionPlan, ResourceConsistencyProfiles, DecisionLog GDL-054–058 and admission review history.
- Accepted storage-attempt contract §§1–9, especially exact supersession, full request histories, independent protected-start observation, publication/fencing, restoration and sticky incompleteness.
- Archived admission design §§2–9, storage-attempt design ownership/lifecycle material, historical IC-1 typed-value and callback obligations, and IC-2 implementation scope/frontier limitations.
- Storage-attempt API `src/lib.rs`, including complete identities, bindings, phases, limits, Recovery and required Host/Session lifecycle.
- Actual core types and admission, staging, lifecycle, callback and projection seams: exact retry, full query/request matching, historical qualification, busy historical refusal and sticky frontier behavior.
- Node signing, RecordsFile, sysdir lock, NodeAssembly, lifecycle, acceptance, mesh and Iroh carrier seams. Concrete anchors included custom-domain strict verification, RecordsFile sync/rename distinctions, sysdir’s unlinking Drop, holder-based acceptance and owned task draining.
- CarrierLink’s duplex, whole-frame, cancellation and transport-acknowledgment contract.
- Canonical Substrate §6 write/read material, CRDT adapter GCA-01–09 and NodeSigning D4–D9.
- External Gyld architecture, independent-CRDT and storage-attempt declarations, including Records/StorageAdapter lifecycle allocation.
- Glial’s exact ten-row admission consumer, syntax guard and digest-pinned released-Taut concurrent-siblings corpus control.

The full changed-path inventory and scoped design/plan/ledger diffs were inspected. Both GWZ marker additions recorded root-only commits and the unchanged relevant member pins. All five HEADs matched at both checks.

No files were written. No builds, tests, network requests, live actions or Git mutations ran. Recorded successful fixture evidence was not treated as executed physical evidence.

## 2. Invariant analysis

**Pre-consumption custody and the original crash pair.** Start with complete selected image G. If guard establishment fails, D:399–402 forbids the first application receive or parse, so the original observation-loss sequence cannot begin. If establishment succeeds and all subsequent observation/loss writes fail, the active guard already exists independently of G. Process death therefore leaves a durable discriminator. Reopen must merge the floor guard before exposing a cut and suppress completeness. The paired execution with no received input remains conservatively incomplete after death; it cannot falsely prove never-started.

The valid empty-round control is different: it selects retained authenticated evidence for that exact round and retires only its guard. It cannot erase older loss or obligations. Qualified live abandonment requires Records-owned evidence that receive/parse was never invoked and future invocation is excluded. Timeout, EOF, Drop, sender retry and restart provide no such proof.

**Interrupted settlement and stale images.** A floor intent carries active guards and proposed settlement. A missing or torn next image cannot apply that settlement. D:329–333 preserves the old selected image plus guard uncertainty, with advanced IDs retained. A valid next image can be finalized only as the exact coupled transition. An ordinary checkpoint, fence or lease update cannot erase a newer floor guard by supplying a stale guard-free image. Lost acknowledgment yields uncertainty or validated exact recovery, not invented NonCommit.

**Cancellation and ownership.** The receive gate must close before cancellation, and all reads, parses and callbacks must be joined before release. A caller’s canceled waiter cannot retire the capability or detach work. D:443–456 retains a residual guard after SIGKILL and prohibits complete reads even when a separately valid local append can proceed. Physical close still retains ownership while Pending. These rules fit the existing storage contract’s separation of invocation retirement from attempt lifetime.

**Authenticated identity and purpose separation.** D:108–160 binds declaration hash/version and schema identity/hash/version, retained canonical source bytes, validated parameters and exact key derivation. Equal names or key bytes cannot join differing schemas. The table also accounts for creation ancestry, incarnation, zone, share, merge/corpus, admission, authorization, storage, retention, rejoin and migration identities. Trusted provision/open, verification, image validation and capability negotiation must check this binding.

The descriptor construction avoids recursive digest inputs. Existing custom-domain signing functions support the selected zero-terminated proof domains without pretending the legacy three-purpose port already expresses them. Strict verification, complete bounded decoding and encode/decode equality reject purpose substitution and alternate encodings.

**Current versus historical authority.** Transport authentication does not become append permission. Writer possession, scoped certificate, finite permit, original requester and current local policy remain separate checks. Records obtains its own authoritative observation under instance serialization at protected start; echoing Begin.current is forbidden. Entire uncertainty intervals must fit permit windows. A valid Started cut survives later revocation and restart, while current serving and transfer rights can end. The disconnected trusted-admitter/time assumption remains explicit rather than becoming a promise of instantaneous global revocation.

**Terminal finality and reconstruction.** Complete requests, consumed requests, immutable bindings, original receipts, terminal conflicts and charges remain retained. Pending, absence, timeout and cancellation never prove NonCommit. Terminal-ahead-of-core recovery must replay a genuinely issued matching callback through actual core logic before reads or append. All four commit kinds retain custody and terminal outcome together.

Combined completeness requires the real kernel cut plus every retained adapter obligation, pending inbox/query/attempt, guard and loss state. Unknown proof and missing ancestry remain unresolved. Empty/latest inventory cannot replace previous observations. The design adds no clearing Event for the kernel’s sticky flags and does not manufacture trusted restoration from serialized Facts booleans.

**Inventory, bounded degradation and privacy.** Full immutable inventory includes accepted operations, candidates, security rivals, qualified forks, original admission/receipt evidence and distinct same-slot digests. Head-only or eligible-only exchange is insufficient. Scoped channel negotiation and current replica permission precede history transfer; historically valid custody does not itself grant current serving access. Finite framing, manifest bounds, critical reservation and acknowledgment-after-retention prevent an oversized or interrupted round from becoming successful empty reconciliation.

Guard/history exhaustion stops new receives without deleting evidence. This can reduce availability, but it is a declared bounded profile outcome, not hidden successful truncation. Independent Y has separate worker, floor, image and guard capacity; X’s unknown work cannot hold a shared mutable allocator or CAS.

**Actual production path and retained controls.** D:480–546 requires the real NodeStart/NodeAssembly/sdax runtime, actual core and providers, exact independent profile routing, and genuine CarrierLink duplex. It identifies the holder/legacy Store seam to bypass. A private harness cannot inject successful Facts/receipts or substitute another admission algorithm.

P:185–215 requires separate real processes, partitioned local edits, automatic healing, exact operation/evidence/receipt comparisons, restart and identical-byte retry. Equal text alone cannot pass. Original core/API and ten released-text controls, architecture/source/process gates, disabled-platform inspection and fast test selection remain protected. Surface and IC-4 activation boundaries remain explicit.

## 3. Risks and next action

Typed and physical qualification remain open. In particular, the next gates must demonstrate that active guard/base-image provenance stays verifiable across ordinary metadata progress, that receive capabilities cannot escape ownership, and that every floor/slot/cancellation cut produces the specified result. Conservative permanent loss after interrupted consumption and finite no-GC exhaustion are real limitations of the selected profile; this verdict does not soften them.

The single next action is for the lane owner to merge this fresh verdict with the independently produced required reviews and originating closure testimony on this same tuple. Only after semantic acceptance may A2 create the reviewed typed boundaries and compiling behavioral RED witnesses. No production adapter, remote-use qualification or activation follows from this report alone.