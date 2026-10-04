# IC-3A semantic production integration design — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` and `GladeIndependentCrdtProductionIntegrationPlan.md`, DRAFT, at workspace root `4d641dd179e5b0bd94e84df9cb334d7a21dae371`, dated 2026-10-04. This review assesses semantic implementability and preserved contracts, not typed-interface acceptance or physical qualification.

**Baseline:**

| Repository | Verified HEAD at start and end |
| --- | --- |
| Glade workspace root | `4d641dd179e5b0bd94e84df9cb334d7a21dae371` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld application | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were inspected using `cat`, `sed`, `nl`, `rg`, hashes and scoped `git diff`. External Gyld means `/Volumes/projects/limbo/gyld-wz/gyld`, not its enclosing workspace repository. The scoped design/plan/ledger and inspected Glade source diffs were empty.

**Date:** 2026-10-04  
**Axis:** Safety: attack degraded paths, irreversible transitions, authorization, custody, crash recovery, completeness and reachable stuck states. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks. No P0, P1 or P3 findings. I pre-commit to GO on a revision that resolves P2-1 as specified while preserving the other reviewed contracts.

---

## 0. Evidence base

The complete generated Safety prompt was read. Its SHA256 matched:

`6834162a81ea2a135b721155825311cdc6d97648fd829f4e9c0ebb60e184d5a8`.

The complete new design, plan and IC-3 ledger were read. Their recorded design and plan hashes matched:

| Document | SHA256 |
| --- | --- |
| Production integration design, lines 1–450 | `e96d4ffc0bf8a8a213d4bb0d58d9f5e595604e80ca4ff71925dd4df5904eafc0` |
| Production integration plan, lines 1–195 | `3cd2255bbc0e124bb7ffae77db41bba9b26abadfa977fe29585d40a355b96f1c` |

Authority inspected included root `AGENTS.md`, `AGENTS_GWZ.md`, Glade and external Gyld application instructions, the review-loop skill and canonical prompt template, BuildEntry and linked problem capture, library boundary policy, package architecture, resource consistency profiles and DecisionLog GDL-054–058. No member-specific AGENTS files were found in Glial or discovery.

The controlling admission plan, IC-3 review ledger, admission review ledger and archived admission design, storage-attempt design, IC-1 contract and IC-2 implementation record were inspected alongside the accepted storage-attempt contract. Particular attention went to the contract’s exact supersession table, owned issuance and callback histories, protected Started barrier, terminal coupling, restoration and sticky incompleteness.

Source inspection covered:

- `crdt-storage-attempt-api/src/lib.rs`, lines 1–266: actual identities, bindings, phases, limits, Recovery and required lifecycle methods.
- Admission core `types.rs`, `lib.rs`, `admission.rs`, `staging.rs`, `lifecycle.rs`, `callbacks.rs`, `projection.rs` and `encoder.rs`: immutable operation bytes, exact request authentication, once-only installation, historical qualification, sticky flags and complete batch encoding.
- Node signing, RecordsFile, sysdir locks, assembly, lifecycle, accept/server and mesh/carrier seams. Relevant anchors include `signing.rs:76–93`, `records_file.rs:141–199`, `sysdir.rs:109–160`, `accept.rs:82–253`, `lifecycle.rs:520–640` and the actual CarrierLink contract.
- Canonical Substrate §6, CRDT adapter GCA-01–09 and node signing decisions D4–D9.
- External Gyld architecture, independent-CRDT and storage-attempt declarations and their source-qualified allocation records.
- Glial’s exact ten-row independent admission consumer, source guard and digest-pinned released-Taut concurrent-siblings corpus.

All five HEADs matched at both checks. No files were written; no builds, tests, Git mutations, network requests or live actions ran. The counterexample below is a semantic crash sequence, not claimed physical execution.

## 1. Findings

### [P2-1] Observation persistence failure has no durable witness that can survive restart

**Location:** Production integration design lines **322–325**, **340–349**, and **365–382**, combined with the physical transition and reopen grammar at **232–251**. The plan inherits this requirement at lines **145–149**.

**Violated invariant:** Once an authenticated history observation affects the locally observed cut, failure to retain it must never disappear into an apparently complete earlier image after process restart. IC3-SYNC-004 explicitly requires permanent loss for unretained observed input and says that, if its marker cannot persist, the instance remains unavailable “including reopen.” IC3-CUT-001/003 and accepted storage contract §7 require the same conservative direction.

The design places manifests, inbox entries and loss markers in coupled Records images. It does not specify a durable receive-in-progress boundary before consuming history-bearing input. Consequently, the obligation to remember a failed marker after restart has no implementable witness in the described recovery grammar.

**Credible sequence:**

1. Receiver R has a durably selected, internally consistent image G. Its kernel and adapter cut are complete.
2. An authenticated, currently authorized peer transfers a correctly scoped inventory or bundle B describing additional history. R receives enough to observe that history, but B is not yet in a selected image.
3. Retaining B fails. The attempted permanent-loss update also fails before the first durable floor-intent replacement. This is permitted by the profile: critical reservation does not promise success on a broken or unavailable filesystem.
4. R correctly reports unavailable in its running process and sends no successful retention acknowledgment.
5. Kill R. The device becomes usable again; the peer remains absent.
6. Reopen finds the unchanged valid floor and image G. Neither contains B, a loss marker, an interrupted intent nor a receive-in-progress record.
7. The documented reconstruction checks cannot distinguish this execution from one where R never observed B. G contains continuous retained provenance and no outstanding obligation. Returning its former complete cut forgets the observed loss; refusing it requires information not present in the format.

An unacknowledged sender retry is insufficient: the peer may remain unreachable, and completeness is a claim about what R already observed. This attack uses ordinary process death and a transient persistence failure, not simultaneous rollback of both trusted roots or power-loss durability.

A related boundary needs explicit treatment: after a durable observation intent but before a valid next slot, the general old-image fallback must preserve observation uncertainty rather than merely recover G and advance counters.

**Impact:** A qualifying implementation can recover all retained custody correctly yet falsely report complete local history after losing an observation. Alternatively, an implementation must invent an undocumented restart rule. Either outcome leaves the semantic recovery boundary unaccepted.

**Required correction:** Specify a durable observation lifecycle before history-bearing input can be consumed as an observation. For example, establish a finite, scoped receive/dirty guard in trusted retained state before receiving inventory or bundle data; settle that guard atomically with the exact observation/classification image. If establishing the guard fails, do not consume new history-bearing input. Interrupted, partially received or unretained observations must leave durable uncertainty or permanent loss on reopen. Define how missing next-slot recovery handles this guard, its bounds, ownership and settlement conditions.

An equivalent mechanism is acceptable if it demonstrates the same indistinguishability case cannot occur. A process-only unavailable flag, post-failure marker attempt or instruction to retry does not close the finding. Do not add a kernel sticky-flag clearing event.

**Closure/regression test:** Add an explicit semantic transition/cut table now, then compiling behavioral RED at the typed gate. The real-provider gate must inject persistence failure and process death:

- after receiving history but before retaining it;
- before and after durable receive-guard publication;
- while persisting the observation or loss classification;
- after durable floor intent with an absent/torn next slot.

Reopen with the peer absent and verify the instance cannot return a complete cut that forgets the observation. Include a positive control showing a fully settled retained observation recovers normally, and confirm independent Y progresses.

**Root classification: architectural.** This is a missing durable mutation boundary and recovery state for observation acceptance, not a validator typo or isolated test omission. It affects the recovery API/image and the receive-to-Records call path. This review identifies **one architectural root** in the new IC-3A object. Initial remediation count remains zero; historical object counts and caps remain unchanged.

## 2. Invariant analysis

The other principal attacks did not establish additional findings.

**Authority and signed identity.** The creation-intent/descriptor construction avoids a recursive digest dependency. Root signatures bind the canonical intent; the complete signed descriptor supplies the instance digest. Operation bytes remain unchanged. Separate zero-terminated proof domains, strict Ed25519 verification and canonical bounded decoding prevent purpose substitution and alternate encodings from becoming equivalent authority. Local possession challenges bind node, scope, operation and session; authenticated peer HELLO does not substitute for requester permission.

**Current versus historical permission.** The design requires an independently injected authoritative policy/time observation at protected start, rather than echoing `Begin.current`. Entire uncertainty intervals must fit finite permit windows. Unknown or regressed time fails conservatively. A previously valid Started cut survives later revocation, while current serving rights remain separate. The accepted trusted-admitter/time assumption is stated honestly; the packet does not claim instantaneous disconnected revocation or Byzantine freshness.

**Custody and callback finality.** The full physical image includes outstanding and consumed requests, plan/attempt bindings, terminal/conflict history, original receipts, charges and continuations. Actual core callback handling authenticates the complete issued request before interpreting a terminal. Pending, absent indices, canceled waiters and lost replies do not prove NonCommit. Publication and fencing share one predicate, and all four commit kinds require coupled custody/outcome persistence. Terminal-before-cached-core recovery must use actual callbacks and genuine retained issuance.

**Floor, ownership and interrupted publication.** Exact generation/digest selection prevents an unrelated inactive slot from becoming authoritative. Missing or unreadable proof fails closed. Stable lifetime locks, exclusive external floors and retained logical ownership distinguish qualified reattachment from writable cloning or request rebinding. The design explicitly limits antirollback trust and receipt strength. Neither hashes nor a new private lease serial is presented as protection against rolling back both roots. Per-instance workers, images and floors avoid a hidden shared CAS that lets unknown X block otherwise functional Y.

**Inventory and reconstruction.** Full inventory includes accepted history, candidates, security rivals and qualified fork pairs, preserving distinct same-slot digests. New or empty manifests cannot replace earlier missing obligations. Reconstruction binds generation, watermark and manifest digest; unavailable proof and missing ancestry remain unresolved. Kernel sticky flags have no clearing action. Composite reads account for pending adapter obligations even when the kernel alone appears complete. P2-1 concerns the boundary before those obligations become durable, not their retained classification grammar.

**Production path and scope.** The packet names the actual NodeStart/NodeAssembly/sdax path and actual core, evidence and storage ports. It expressly bypasses holder placement for authenticated independent instances and prohibits successful fixture substitution. CarrierLink supports concurrent send/receive, and the selected protocol requires separate capability negotiation and current scoped transfer authorization. The witness requires two real processes, partitioned local admission, automatic duplex healing, exact operation/evidence sets and original receipts. Equal text alone cannot qualify it.

**Delivery safeguards.** Semantic acceptance, typed/compiler RED, physical component qualification and aggregate review remain separate gates. Existing consumers, ten text rows, canonical corpus, architecture/source/process checks and fast selection remain protected. No activation, migration, seal, browser compatibility or stronger durability is silently approved.

## 3. Risks and next action

No additional finding is assigned to the explicitly deferred physical, typed or IC-4 outcomes. Their future tests remain necessary; this inspection establishes none of their execution claims.

The single next action is to merge this report with the independent Consistency verdict and settle the observation lifecycle correction in one reviewed semantic revision. P2-1 must be independently closed before proceeding to typed production boundaries or adapters. Material changes to the durable observation or recovery boundary require fresh full axes under the existing ledger and cap rules.