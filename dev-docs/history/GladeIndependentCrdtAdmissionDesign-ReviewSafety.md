# Glade independent CRDT admission design — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionDesign.md`, DRAFT semantic design at root `bba04ad27311db50e3e6aedddb4d91780e4e4483`, dated 2026-10-04. Restricted new-lane changes from `9baa7b8cd1c876ff9e608ecdffdd7490b58ebe9b`.
**Baseline:** Root `bba04ad27311db50e3e6aedddb4d91780e4e4483`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Committed normative sources were read with `git show <pin>:<path>`.
**Date:** 2026-10-04.
**Axis:** Safety — degraded paths, authorization, projection integrity, custody, recovery, and irreversible transitions. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified, without materially widening the object.

---

## 0. Evidence base

The generated Safety prompt was read in full. Its SHA256 matched `a087cad0576072976503f36be6264f57f72673d7787120207794cbdaf2bee16a`.

All four HEADs matched the required tuple at the beginning and end. The first external-Gyld lookup used an incorrect relative path and failed; the correct `/Volumes/projects/limbo/gyld-wz/gyld` path then verified the required pin.

Read:

- The complete design, lines 1–681; the complete lane ledger and delivery plan; resource requirements RC-001–007 and DecisionLog GDL-054–057.
- AGENTS.md, AGENTS_GWZ.md, and the referenced review-loop skill.
- BuildEntry; LibraryBoundaryAndTestingPolicy; GladePackageArchitecture; arch1 architecture §§1–7; the external Gyld declaration’s contract/component definitions.
- Authz §§1–7b and its ratified private-scope rulings; DiscoveryModel §§0–2; WorkspaceDirectory §§1–4.
- Glade CRDT adapter and ShapeDispatch; Zones; relevant Substrate session/write rules and CrossNodeWrites; NodeSigning decisions.
- BindingResolver and ReplicaSync contract source, particularly current-access checks, canonical-byte preservation, and known-error atomicity.
- Raft adoption requirements and canonical reconciliation; production integration gates; the complete legacy Store seal contract and its lock/marker implementation.
- Pinned acceptance-path source; supporting Glial text mapping and Taut CRDT corpus/model source. Those latter working-tree reads supplied context, not additional reviewed member pins.

Inspection used `git show`, scoped `git diff`, `git rev-parse`, `cat`, `sed`, `nl`, `rg`, and hashing. No builds, tests, network calls, writes, or current peer reports were used.

## 1. Findings

### [P2-1] A signed but never authorized rival can invalidate admitted history

**Location:** Design lines 185–204, 281–307, and ICD-009/T09 at line 546.

**Violated invariant:** Historical eligibility is supposed to distinguish valid admission from current authorization. A cryptographic signature establishes authorship, but does not establish that an operation qualified for admission.

The fork-proof definition requires two distinct signed operations with matching instance/origin/epoch/seq. It does not require qualifying admission evidence for the competing operation. Consequently, a candidate that no node could lawfully accept can nevertheless remove a valid operation and all its transitive dependents from projection.

**Reproduction:** A legitimately admits writer W’s seq0 operation O. Other writers admit descendants referencing O. W is subsequently revoked, and its permit expires. W retains its signing key and signs a different seq0 operation O′. O′ has no valid admission record and cannot obtain one. An enrolled, currently authorized peer carries the signed pair as integrity evidence. Under lines 300–303, the pair proves a fork; under lines 297–307, O and its descendants become quarantined.

This requires neither signature forgery nor a trusted admitter lying about time. The same problem exists for a signed rival whose payload or permit was already invalid before revocation.

**Impact:** A former writer can trigger indefinite retroactive projection loss outside its finite offline window. Retained receipts prevent physical deletion, but users lose the eligible text and subsequent legitimate dependent edits. This expands the explicitly described trusted-admitter risk into a separate power held by any former writer key.

**Required correction:** Specify which authorization/admission evidence makes a rival eligible to convict an admitted chain. A bare writer signature or known-invalid candidate MUST NOT by itself invalidate otherwise valid admitted history. Retain unauthorized rivals as attributable security evidence. If the intended policy deliberately grants this retroactive quarantine power, state and obtain explicit acceptance of that threat model rather than treating it as an ordinary consequence of bounded offline authorization.

**Closure test:** Extend T06/T09 with an admitted O and valid dependent text edits, followed by revoked/expired W signing O′ without qualifying admission. Opposite evidence orders MUST preserve the valid projection while retaining the attempted rival. A genuinely qualifying same-slot fork MUST still produce equal quarantine sets and degraded receipts.

### [P2-2] Fresh origin epochs have no defined collision-free identity in the canonical engine

**Location:** Design lines 92–100, 131–137, 297–316, 374–383, and ICD-007/T07 at line 544. Controlling GCA mapping preserves `Op.origin` as canonical `CrdtOp.origin`.

**Violated invariant:** A recovery lifecycle must preserve unique canonical operation identities and remain compatible with the unchanged released engine.

The design identifies a fork slot using origin **and epoch**, and prescribes a fresh certified origin epoch after fork or unsafe backup restoration. It does not require that each epoch have a new canonical `Op.origin`, nor define an injective epoch-to-origin mapping. The existing engine’s identity is `(origin, seq)`; the proof wrapper’s epoch is absent from that identity and from ordinary causal refs.

**Reproduction:** Epoch E0 has retained operation `(writer-a, Glade seq0)`. After a fork or unsafe restore, authority certifies fresh epoch E1 for origin ID `writer-a`. E1 starts its initial chain at seq0 as required. The security proof considers E0 and E1 different slots, but both map to canonical Taut identity `(writer-a, 1)`.

If both epochs remain eligible, the unchanged engine sees duplicate identity or equivocation. If the implementation instead applies T07’s “reused origin fails,” the prescribed fresh-epoch recovery cannot resume edits. Causal refs also cannot distinguish the two epochs.

**Impact:** The proposed recovery path either stalls or collides with retained history. Choosing behavior during implementation would settle a missing semantic contract, not merely allocate a wire field.

**Required correction:** Require a fresh, immutable canonical origin ID for every certified epoch, with the epoch distinction already embodied in `Op.origin` before signing and hashing. Define how refs and certificates bind that ID. Alternatively, specify another collision-free mapping that preserves canonical inner bytes and the released engine contract. Display names MAY repeat; canonical origins MUST NOT.

**Closure test:** Extend T07/T09 with old-epoch retained prefix and dependents, fork/restore recovery, and an authorized new epoch using the same display label. The new seq0 MUST have a distinct canonical identity, preserve eligible old history, and converge through the actual released text engine. Reuse of the old canonical origin MUST fail before mutation.

## 2. Invariant analysis

Several attacks failed against explicit requirements:

- Equal names, empty discovery, and first-observed shape cannot mint or combine identities. The authenticated descriptor and exact capability tuple govern admission.
- Forwarding-node authentication cannot substitute for requester proof. Private `self` derivation, membership, and placement checks remain separate.
- Partitioned acceptance is bounded by finite permits, injected trusted time, current local policy, and capacity. Instantaneous disconnected revocation is explicitly excluded.
- Exact retry, lost replies, and partial I/O have distinct outcomes. Coupled commit/handoff and process-restart qualification prevent fixture success from becoming a physical durability claim.
- Reconciliation must exchange actual app history bidirectionally and survive browser unsubscribe. Directory convergence alone cannot satisfy T12.
- Full-history retention, recovery reserves, and explicit incomplete states prevent quota pressure from silently evicting accepted evidence or advancing false frontiers.
- New roots, profile negotiation, admission fencing, and migration cuts preserve legacy/strong contracts. The whole-store seal is correctly distinguished from per-instance activation.
- The pure-kernel tranche has meaningful text witnesses, compiling-consumer prerequisites, explicit package roles, and separate real-adapter gates. No synthetic crypto or storage result is presented as live qualification.

These protections do not resolve the two finding sequences.

## 3. Risks and next action

Trusted-admitter/time honesty, finite retention, sole-copy loss, and production custody remain substantial but explicitly bounded, deferred deployment decisions. This review establishes no implementation or adapter qualification.

The next action is one scoped semantic remediation: define admission-qualified fork conviction and collision-free epoch identity, update their normative requirements and future RED witnesses, then obtain the originating reviewer’s closure verdict on a newly pinned tuple.