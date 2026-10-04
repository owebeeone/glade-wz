# IC-3A semantic production-integration design — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT production-adapter semantics and scoped amendments in `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` and `dev-docs/GladeIndependentCrdtProductionIntegrationPlan.md`, at workspace root `4d641dd179e5b0bd94e84df9cb334d7a21dae371`.

**Baseline:** Root `4d641dd179e5b0bd94e84df9cb334d7a21dae371`; Glade `37dff286ce1eb9690204d4a7d14c940333396a30`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources were inspected with `cat`, `nl`, `sed`, `rg` and targeted `git show HEAD:`; scoped `git diff HEAD` checks found no changes to inspected source objects.

**Date:** 2026-10-04.

**Axis:** Internal coherence, consistency with accepted contracts and actual sources, exact amendment scope, and satisfiability of the proposed delivery gates. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block. Two architectural roots and one nonarchitectural root are identified. I pre-commit to GO on a revision that resolves P2-1, P2-2 and P2-3 as specified, preserves the retained contracts, and introduces no material additional change.

---

## 0. Evidence base

The complete generated Consistency prompt was read. Its SHA256 matched `653e595b393f5be1ab71ac772b30bf399fd0e22bb7469572c7b39b5383b8ba4f`.

The complete design, plan and IC-3 ledger were read. Their design/plan hashes matched the ledger:

| Object | SHA256 |
| --- | --- |
| Design, 450 lines | `e96d4ffc0bf8a8a213d4bb0d58d9f5e595604e80ca4ff71925dd4df5904eafc0` |
| Plan, 195 lines | `3cd2255bbc0e124bb7ffae77db41bba9b26abadfa977fe29585d40a355b96f1c` |

Below, **D** denotes the [design](/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md), and **P** the [plan](/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationPlan.md).

Controlling evidence inspected included:

- Root instructions and `AGENTS_GWZ.md`, Glade member instructions, external Gyld instructions, the complete review-loop skill and canonical template.
- BuildEntry, its linked source-qualified problem capture, LibraryBoundaryAndTestingPolicy and GladePackageArchitecture.
- AdmissionPlan, ResourceConsistencyProfiles and DecisionLog GDL-054–058.
- Accepted storage contract, particularly §§2–7 and its historical supersession table; archived semantic design §§2–11; archived storage lifecycle design §§1–9; archived internal contract §§1–6; archived IC-2 implementation record, including its frontier/busy-input limitations and acceptance record.
- Actual core `types.rs`, `lib.rs`, `admission.rs`, `staging.rs`, `callbacks.rs`, `projection.rs` and `encoder.rs`; storage-attempt API `lib.rs`.
- Actual node signing, assembly, lifecycle, acceptance, server, sysdir, RecordsFile, peer HELLO and Iroh carrier seams; CarrierLink API.
- Canonical Substrate session/write material, NodeSigning and CRDT adapter requirements.
- External Gyld architecture, independent-CRDT and storage-attempt allocations; the released-engine Glial consumer and its pinned canonical-corpus checks.

The five HEADs were verified at both start and end and remained exactly the specified tuple. Inspection only: no builds, tests, writes, network actions or Git mutations were performed. Existing execution claims were treated as recorded evidence, not rerun qualification.

## 1. Findings

### [P2-1] Failed loss-marker persistence has no durable predecessor that can keep reopen unavailable

**Location:** D:224–251, 276–282, 340–349 and 353–385; P:103–115 and 145–150.

D:346–348 requires: “Unretained observed input causes permanent loss before complete reads; if marker cannot persist, instance remains unavailable including reopen.” The physical grammar does not establish a durable condition *before* such observation that would let a subsequent process distinguish this failure from an ordinary complete persisted cut.

**Violated invariant:** An observed, unretained qualifying cut cannot disappear across process restart and permit a complete read. Storage contract §7 expressly requires persisted incompleteness before a restarted complete read; D retains that obligation.

**Credible sequence:**

1. A fresh instance has a valid selected image and floor, no sticky flag or loss marker, and a complete kernel/adapter cut.
2. An authorized peer supplies an authenticated inventory/input exposing missing history. Its obligation cannot be retained because a storage write fails.
3. The attempted permanent-loss marker also fails before a durable floor intent or new image exists. This is permitted by D:227–228; preallocation does not exclude later I/O failure.
4. The process correctly refuses reads while alive, then receives SIGKILL.
5. Storage becomes writable again. Reopen sees exactly the previously valid image/floor, with continuous persisted provenance through the old watermark and no recorded loss. The stable lock and incremented physical lease serial do not encode whether an unretained observation occurred.
6. Reconstruction can therefore produce the old complete cut, despite the required permanent loss.

There is an indistinguishable execution with no step 2. A memory flag and a marker whose write failed cannot discriminate the two. The current floor handshake only protects transitions whose durable intent was successfully written.

**Impact:** The advertised restart guarantee is unsatisfiable as written. An implementation can satisfy all stated image validation rules yet forget the failure and return false completeness.

**Required correction:** Specify a durable ingress/observation guard before potentially qualifying input can be observed, or another explicit persistent ownership/provenance mechanism covering this window. Define its interrupted and failed-write states, reopening rules, and whether/how an unresolved guard can be discharged. If the guard cannot be established, do not enter the observation path. Do not use empty/latest peer inventory to clear unknown observation loss.

**Closure test:** Begin from a persisted complete cut; inject failure before the first durable observation/loss write; fail the marker too; kill the actual process; restore writable storage and reopen. Assert incomplete/unavailable survives and a later empty inventory cannot produce completeness. Include the paired no-input execution and every guard establishment/discharge crash boundary.

**Root classification:** **Architectural.** This changes the durable boundary between transport observation and Records ownership. It is not merely an omitted error check.

### [P2-2] The genuine descriptor grammar omits accepted declaration and key-schema identity

**Location:** D:76–87, 106–113 and 387–401; P:30–32 and 50–59. Controlling archived semantic design:82–97.

The accepted identity table requires an “Authoritative declaration hash/version, immutable for this incarnation” and says the canonical parameter/key schema version is pinned. Its next paragraph makes the full authenticated tuple the namespace for operations, proofs and replication.

D:106–113 now selects the genuine signed creation/descriptor grammar. It lists root key, nonce, incarnation, resource/private owner, share/glade ID/key, profiles, mode and bounds, but supplies no declaration identity or canonical parameter/key schema identity. These are not engine/admission profiles. The actual Pure `Descriptor` likewise has no such typed fields; it was explicitly an internal snapshot while the genuine remote encoding remained unresolved.

**Violated invariant:** The signed descriptor must bind the entire previously accepted identity tuple. D’s “retain” disposition cannot silently reduce that tuple when choosing the real encoding.

**Credible sequence:** Provision two nodes with the same root, intent nonce, resource/incarnation, share/glade ID/key bytes, profiles and bounds, but different authoritative declaration revisions or parameter schemas. Following the enumerated IC-3 intent grammar produces the same intent and descriptor identity. Proofs and HELLO can therefore agree while the omitted authoritative declarations disagree. Different parameter schemas can also assign different meanings to identical key bytes.

No mandatory verification or matrix row in this packet rejects that particular mismatch. “Parsed fields MUST equal held descriptor fields” checks equality of the fields supplied; it does not supply the omitted identity.

**Impact:** The selected genuine profile can join incompatible declaration/key contexts under one authenticated namespace, contrary to the accepted semantics and RC-001.

**Required correction:** Explicitly bind declaration hash/version and canonical parameter/key schema identity into the signed creation/descriptor contract and trusted open/verification inputs. State how the remaining accepted tuple components are represented or fixed by the selected versioned profile. Preserve inner Op bytes and existing Pure consumer paths. Replace the general retention assertion with an exact mapping to the accepted identity table.

**Closure test:** Add distinct negative witnesses changing only declaration hash, declaration version, parameter/key schema version and schema identity. Each must change the authenticated descriptor identity or be rejected before opening, admission and exchange. Show that equal names and equal key bytes cannot bypass these checks.

**Root classification:** **Architectural.** The defect concerns authenticated namespace composition and its proof/open interfaces.

### [P2-3] Remote-use completion gates omit the mandatory Rust/TS/Python canonical vectors

**Location:** D:97–104, 403–433 and 437–445; P:45–75 and 169–181. Controlling archived semantic design:91–97 and 579–585.

The accepted semantic design states: “Canonical serialization and hashing MUST have Rust/TS/Python vectors before remote use.” The IC-3 packet retains the semantics but its executable-contract and aggregate completion lists require codec round trips, field mutations, real signatures, two Rust processes and the unchanged text corpus. They contain no gate for cross-language canonical proof/container serialization and hashing.

The released text corpus checks operation payloads, coordinates and merge behavior. It does not validate the new signed descriptor, certificate, permit wrapper, policy, admission or inventory encodings.

**Violated invariant:** Real remote use cannot precede the accepted compatibility evidence merely because browser activation remains IC-4.

**Credible sequence:** A2 passes one Rust encoder/decoder and its mutations; B passes strict-signature tests using those same bytes; C passes two Rust binaries using that implementation. Every new matrix row and the stated Done conditions can pass without an independent TS or Python rendering agreeing on the new identity/hash bytes. The packet then declares bounded remote qualification before the retained prerequisite has been demonstrated.

**Impact:** The delivery plan permits premature protocol qualification and leaves canonical discrepancies invisible to its completion evidence. This is a specific omitted accepted gate, not a demand to implement browser UX now.

**Required correction:** Add a stable requirement and explicit pre-remote-use gate for pinned Rust/TS/Python canonical encoding/hash vectors of the new relevant descriptor/proof/transfer representations. Include the frozen inner Op compatibility control and malformed/noncanonical negative vectors. Alternatively, propose and independently review an exact scoped amendment to that accepted prerequisite; IC-4 deferral alone does not amend it.

**Closure test:** The delivery checklist must fail when any required language consumer/vector set is absent. All three consumers must agree on exact canonical bytes and digests for representative positive/edge inputs and reject the prescribed negative inputs. Record source pins and assertion identities.

**Root classification:** **Nonarchitectural.** This restores an already accepted verification prerequisite and its traceability; it requires no change to responsibility allocation or admission semantics.

## 2. Invariant analysis

Several substantive attacks did not establish findings:

- **Ownership and callbacks:** Same-logical-owner reattachment is explicitly distinguished from owner replacement. It can preserve the complete old requests required by `callbacks.rs:34–43`, while the physical lifetime lock and local-worker restriction exclude a concurrent execution. The typed gate must still prove the implementation, but the semantic distinction itself is coherent.
- **Attempt finality:** Reserved/Started are never interpreted as absent terminal negatives. D:253–274 preserves exact PlanKey recovery, one winner, original requests, contradiction evidence and once-only accounting.
- **Atomic custody:** Terminal-ahead-of-core recovery retains both the original plan and genuine issued continuation, then uses actual callbacks. This agrees with the current kernel’s installation prerequisites and avoids manufacturing authority from AttemptId.
- **Policy/time:** Current authorization and historical qualification remain separate. The independent protected-start observation is not supplied by `Begin.current`; valid Started cuts survive later revocation. Unsupported delegated ancestry refuses.
- **Completeness:** D:353–385 adds no sticky-flag clearing event. The adapter’s combined cut can remain false while a kernel cut is true, and its narrow reconstruction excludes imported/unknown provenance. This directly addresses the IC-2 frontier and busy-input limitations without claiming they were repaired.
- **Independent instances:** Per-instance State, sessions, workers, floors and images avoid a shared mutable root CAS that would make unknown X block functional Y.
- **Production path:** The packet identifies the actual holder/Store seam to bypass and requires the real core/runtime, assembly ownership and duplex CarrierLink. A fixture’s successful response is expressly insufficient.
- **Allocation and checks:** Proposed data/codec roles and contract interfaces are justified; exact edge adoption, meaningful consumers, retained assertions, disabled-platform inspection and unchanged budgets/allowlists remain mandatory. Gyld’s prior Records/Admission/Pure allocations are preserved as obligations, not claimed runtime satisfaction.
- **Receipt scope:** LocalProcessRestart remains a qualified process-reopen promise. Neither fsync, peer acknowledgment nor a copied fixture is promoted to quorum, machine-loss or arbitrary rollback protection.

## 3. Risks and next action

Physical execution and compiling typed boundaries remain future gates; their absence is not a finding at this semantic checkpoint. Those gates must examine root/path replacement during ownership, old-binary exclusion, alternate admission proofs for one immutable operation, full-image capacity and actual shutdown ordering.

The single next action is a merged semantic remediation covering P2-1–P2-3, with the ledger recording **two architectural roots, one nonarchitectural root and zero completed remediation rounds**. No third architectural root is established by this report. Material durable-ingress or identity-interface changes require fresh full review under the existing cap; production adapters must remain behind the gates.