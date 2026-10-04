# IC-1 independent CRDT admission contract — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionContract.md`, DRAFT internal contract and compiling behavioral RED tranche, at root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`.

**Baseline:** Root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`; Glade `885249a2a093e082aad6e1dc9936a7fd5c54052b`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; Glial `5fd46ba5180051eb20d7b5547f59f52c0f3ebe06`; external Gyld `64666e8b1caadde8922b9d42163afbab90655c65`. Inspected committed root sources and member sources at verified HEADs, with scoped baseline diffs.

**Date:** 2026-10-04

**Axis:** Internal consistency, controlling-contract agreement, continuation semantics, and satisfiability of behavioral evidence. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block; one P3 documentation finding. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified, provided its changed range introduces no further blocking defect.

---

## 0. Evidence base

All five HEADs matched the specified tuple at review start and end. The complete generated prompt was read; its SHA256 matched `d8e686ecae732fd13f6bbb21962a9ae73e106f07e3ea8913cb922b3aebc2d2a1`.

Inspection included:

- Complete contract and committed evidence; root diff from `4d735893c8db0a9ac9b4de9cde01600873b20ce3`.
- Complete `src/types.rs` and refusing `src/lib.rs`; `records_host_contract.rs` lines 1–904, support lines 1–332, Rust trace lines 1–252, and both JS consumers.
- Representation/source guards, manifests, architecture policy, selectors, ARCH-002 fixture, tooling regressions, and process-global scan roots.
- Accepted semantic design §§2–11; original remediation and originating closure records; fresh semantic GO reports; resource requirements, delivery plan, GDL-054–057, library policy, package architecture, build entry and architecture allocation.
- Relevant Substrate W1–W8, Authz, discovery/directory, CRDT adapter, shape/zone/catalogue, BindingResolver/ReplicaSync, and Raft coexistence/seal sections; current acceptance, Store and envelope source.
- Complete Gyld supplemental declaration, capture host and new test class; supplemental ledger provenance. Read-only hashing confirmed the frozen design text matches the accepted source. Base architecture declaration, base source ledger and base capture host are unchanged; the pre-existing test-module prefix is byte-identical.

No builds, tests, writes, network calls, Git mutations or current peer material were used. Executable results are drafter-recorded evidence audited against source, not independently rerun results.

## 1. Findings

### [P2-1] Repeated lookups cannot identify their callback invocation

**Location:** `glade/contracts/crdt-admission-core/src/types.rs:263–278`; contract lines 106, 128–130 and 249–257; lookup consumer at `records_host_contract.rs:717–779`.

**Violated invariant:** A duplicate/stale callback MUST be distinguishable from the current outstanding continuation and MUST NOT consume that continuation or its reservation.

`LookupRequest` and `LookupReply` contain only the retained commit’s `plan_id`, expected revision, instance, operation and batch digest. They have no independently allocated lookup identity.

**Counterexample:** Unknown commit P remains reserved. Lookup L1 returns `Unknown`. Recovery later issues L2 for the same unchanged P. Both requests have identical representable fields. A duplicate L1 reply arriving while L2 is outstanding is indistinguishable from a legitimate L2 reply. Remembering that an identical result occurred previously cannot distinguish a duplicate from a legitimate repeated `Unknown`.

The typed boundary therefore cannot enforce its callback rule. An implementation must either accept an old callback as current or reject a legitimate current callback; continuation consumption and recovery progress become dependent on behavior outside the declared event contract.

**Required correction:** Add a distinct monotonic lookup invocation ID to request/reply and outstanding lookup state, while retaining the original immutable commit plan identity. Define retirement, repeated-Unknown retry and restoration/invalidation semantics.

**Closure test:** Issue L1, resolve it as Unknown, then issue L2. Deliver duplicate L1 replies while L2 is outstanding and assert `CallbackMismatch`, unchanged L2 and unchanged reservation. Resolve L2 with the exact committed receipt and verify original acceptance recovery without sealing or recommitting.

This requires an internal continuation-shape correction; it does not require redesigning the accepted admission semantics.

### [P2-2] The required post-fork fresh-origin AB regression is absent

**Location:** Accepted design lines 402–412 and ICD-T09 at line 649; contract §4 and §7; Rust trace lines 40–65 and 108–159; `records_host_contract.rs:145–188` and 243–309.

**Violated invariant:** The executable closure witness MUST establish that a fresh certified origin can continue from the eligible old prefix after genuine qualified fork quarantine, preserving old identities and producing exact released text `AB`.

The current AB rows start from an empty instance and deliver A/E1-B. The fresh-origin Rust admission test seeds only A. Separate qualified-fork rows establish reduced projection A, then stop. No consumer attempts E1 recovery in the resulting quarantined instance.

**Concrete escaping behavior:** A future kernel could admit fresh E1 when quarantine is empty, but refuse every new local edit whenever the instance has any quarantine entry. It could satisfy the current AB tests, qualified-fork tests, and the existing local refusal test for quarantined support. The accepted post-fork recovery journey would nevertheless remain broken.

This is a bounded regression-coverage defect, not a demand for live custody or broader adversarial qualification. The originating semantic closure explicitly left this combined executable witness mandatory.

**Required correction:** Extend the actual AB consumers to retain two qualifying E0 seq1 rivals and derived quarantine, then admit/import E1 seq0 referencing eligible E0 seq0. Keep both evidence/delivery orders, exact eligible identities and released `AB`. Preserve old receipts/retries and reject E1 certification that reuses E0’s canonical origin.

**Closure test:** The combined witness must compile and fail behaviorally on the refusing scaffold. At IC-2 it must become GREEN through kernel output and reject a mutant that refuses fresh-origin recovery merely because the instance contains quarantine.

### [P3-1] Blanket rejection of delayed callbacks contradicts retained delayed commits

**Location:** Contract lines 128–130 versus 230–240; `records_host_contract.rs:830–904`.

Section 3 says a “delayed” response MUST produce `CallbackMismatch` and applies that rule to commit callbacks. Section 6 and its RED consumer require a matching delayed prior-cut committed callback to install historical acceptance after revocation.

The same event consequently has contradictory normative outcomes. A reader implementing Section 3 literally would reject a required recovery case.

**Correction:** Define stale callbacks by retired/mismatched continuation identity, rather than elapsed delay. Explicitly allow delayed matching callbacks under the specified policy/commit ordering.

**Closure:** Reconcile the text and retain the existing delayed prior-cut commit/unknown-recovery consumer as the regression.

## 2. Invariant analysis

**Evidence strength is honestly bounded.** The scaffold preserves state and returns only Unavailable. The Rust host consumes public effects; the trace uses only kernel-returned eligibility. No expected-operation fallback supplies successful RED output. JS invokes released Taut and checks the pinned concurrent-siblings bytes and exact mandatory row inventory.

**Identity and eligibility remain coherent apart from the missing combined witness.** Immutable operation bytes and derived digests prevent independently asserted operation identities. Descriptor/certificate verification obligations bind parsed fields to exact bytes. Local authorization, historical qualification and derived quarantine are distinct; qualified fork custody survives projection exclusion, and common-prefix heads stop before the fork.

**Atomicity and receipt scope are appropriately limited.** Plans retain candidates, facts, queries, seals, receipt identity and reservations. Known failure differs from unknown outcome. Prior-cut physical commit ordering is explicitly a trusted-host obligation, and VolatileTest is not presented as restart durability.

**Canonical coexistence is preserved.** This tranche changes no existing node/wire admission path. Legacy/strong bindings remain excluded; canonical amendments, negotiation, separate roots and whole-store seal interaction remain mandatory before live integration.

**Architecture and provenance checks retain their purpose.** The Pure classification matches deterministic event/effect ownership and minimal dependencies. Existing policy entries are unchanged. ARCH-002 retains its positive control and exact refusal across actual members, with the synthetic additional-member regression. Syntax guards inspect disabled declarations; macro limits are disclosed. Gyld inherits existing ownership and checks exact combined owner/requirement relationships against frozen app-owned sources.

## 3. Risks and next action

Cryptography, canonical remote evidence, time/custody trust, physical atomicity/restart, automatic duplex mesh, client/editor lifecycle, capacity/loss and activation remain genuine later qualifications. The baseline Mypy limitations remain disclosed; they are not admission evidence.

Prepare one bounded remediation covering the lookup identity and combined post-fork AB witness, reconcile delayed-callback wording, record compiling RED and scoped GREEN evidence, then submit the revised tuple for originating verification before successful kernel implementation.