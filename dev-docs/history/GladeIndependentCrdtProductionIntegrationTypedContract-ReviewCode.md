# IC-3A2 production adapter typed contract — CODE-AXIS REVIEW

**Review object:** IC-3A2 implementation candidate, controlled by `dev-docs/GladeIndependentCrdtProductionIntegrationTypedContract.md` at workspace root `c3608e286272e7de412f8de52496cbfc0a355a75`. Typed acceptance remains open.

**Baseline:** Glade change from `37dff286ce1eb9690204d4a7d14c940333396a30` to `8b0595551dd90f32da4698fff9358aa30dbf2548`; external Gyld change from `95a426595bba8e248a5f484272e483a070c73918` to `03428fb36649d71541fd76f2471533e384be8209`. Sources were inspected using filesystem reads, scoped Git diffs, and `git show` of the pinned implementation.

**Date:** 2026-10-04.

**Axis:** Architecture, interfaces, call graphs, and compatibility reality. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on their testimony. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block; two additional P3 findings identify bounded coverage and argument-validation defects. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified, provided verification establishes no new blocking defect.

---

## 0. Evidence base

The exact five-repository tuple matched at both START and END:

| Repository | START | END |
| --- | --- | --- |
| Glade workspace root | `c3608e286272e7de412f8de52496cbfc0a355a75` | `c3608e286272e7de412f8de52496cbfc0a355a75` |
| Glade | `8b0595551dd90f32da4698fff9358aa30dbf2548` | `8b0595551dd90f32da4698fff9358aa30dbf2548` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `03428fb36649d71541fd76f2471533e384be8209` | `03428fb36649d71541fd76f2471533e384be8209` |

Inspected authority included root `AGENTS.md`/`AGENTS_GWZ.md`, Glade and external Gyld instructions, the review-loop skill and canonical template, the production review ledger, build entry, library/package policies, production design/plan/typed contract, typed evidence, admission plan, storage-attempt contract, resource consistency profiles, and relevant archived identity, synchronization, ownership, recovery, and kernel contracts. External Gyld’s README, architecture policy, and workzone engineering policy were inspected.

Source inspection covered the complete new admission data/proof types, evidence and recovery contracts, ingress utility, codec implementations and schema assignments, associated tests, independent Python/TypeScript consumers, corpus manifests, prerequisite gate, actual node refusers/assembly addition, manifests, lockfile diffs, selectors, architecture policies, and the six-file Gyld change.

Read-only Python inspection established:

- All 58 implementation files matched their recorded hashes and implementation commit `7d26ba6e6d133db650a37ff49eba71644ea370a6`.
- All four root-document hashes matched.
- Exactly the two documented files differed from the 45 protected original hashes: extracted core types and the appended source-boundary test.
- The new data library contains the old core types as a byte-preserved prefix.
- Checked record field assignments matched the corresponding static schema fields.
- The compressed chronological log expands to 3,676,067 bytes with the recorded raw SHA-256 `4d53c0ce70ec9ff9fa7461d7be04d1b5507db3148af55773a16301b334e16289`.

Recorded evidence, rather than fresh execution, supplies the stated contract/node/vector/Gyld results. The raw log retains the original 2.978-second budget failure, the subsequent 5.251-second failure, and the final 2.860-second execution.

No files were written, builds or tests run, network accessed, or Git state mutated. No current peer report was read or requested. Counterexamples below are source-traced reproduction sequences, not claimed executed regressions.

## 1. Findings

### [P2-1] Generic physical envelopes erase the contained type identity

**Location:** [values.rs:109](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/src/values.rs:109), through the generic `Vec`, `Option`, `Box`, tuple, and `BTreeMap` implementations at lines 109–190; [lib.rs:26](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/src/lib.rs:26), especially the envelope discriminator check at lines 52–55. `PlanKey` and `GuardId` field assignments appear in the storage and production schema modules.

**Violated invariant:** The physical `[type-name, typed-value]` envelope must prevent identity/type substitution. The object explicitly demonstrates that a plan identity must not decode as a receive-guard identity.

**Reproduction:** Construct a nonempty `Vec<PlanKey>`, encode it through the public `encode` function, then decode the resulting bytes as `Vec<GuardId>` using generous limits.

Both instantiations use the literal discriminator `"Vec<T>"`. Each contained record is an untagged integer-keyed map. `PlanKey`’s namespace and `GuardId`’s store both encode as a 32-byte digest; their instance and number fields have identical representations. Consequently the outer discriminator and every field decoder accept the substitution. `Box<PlanKey>` versus `Box<GuardId>` has the same defect. The existing direct `PlanKey`/`GuardId` regression succeeds because its two concrete outer names differ; it does not exercise generic wrappers.

**Impact:** A supported physical encoding can be read successfully under a different identity type. Structural decoding remains untrusted, but that limitation does not excuse losing the format’s promised type discriminator.

**Required correction:** Preserve the concrete type arguments in a frozen, deterministic envelope identity, or restrict physical envelope roots to explicitly named concrete types while retaining generic implementations solely for nested fields. Do not derive a portable format from unstable compiler/debug spelling. Document the chosen compatibility treatment.

**Closure test:** Add a compiling regression that rejects the nonempty vector and boxed substitutions above, then cover `Option`, tuples, and map key/value substitutions. Valid same-type roundtrips and existing concrete/profile bytes must retain their specified behavior.

**Root classification:** Architectural, belonging to the new typed object. This is a shared representation-boundary design defect: generic root encodings have no concrete identity. It does not refute the accepted semantic identity design.

### [P2-2] Embedded operations bypass the caller’s decoding limits

**Location:** [values.rs:192](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/src/values.rs:192), especially lines 197–200; [bounds.rs:55](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/src/bounds.rs:55); [lib.rs:38](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/src/lib.rs:38). The retained generated `Op::decode` at `glade/wire-rs/src/generated.rs:249` uses its own depth and encoded-length bounds.

**Violated invariant:** Typed-contract line 83 requires preflight of encoded byte/item/depth/name limits before typed allocation.

**Reproduction:** Encode a valid `Operation` whose origin is 33 bytes. Decode its physical envelope using `DecodeLimits { bytes: 8192, items: 3, depth: 1, name_bytes: 32 }`.

The outer envelope consists of an array, the text `"Operation"`, and one byte string. It passes all four limits. The preflight treats the operation bytes as opaque. `Operation::decode_value` then calls `Op::decode` without the caller’s limits and allocates the 33-byte origin. The inner map also contains substantially more than three items and an array nested inside it; those structures are absent from the outer item/depth accounting.

The same omission applies when operations are nested in candidates, transfer bundles, queries, or images. In contrast, the new batch decoder explicitly preflights operation bytes at `batch.rs:275`, confirming that this boundary needs separate treatment.

**Impact:** The caller’s limits do not bound all syntax the decoder subsequently interprets and allocates. A small outer item budget can conceal a much larger reference collection inside an operation byte string; a name limit cannot constrain operation names.

**Required correction:** Carry the decoding budget into embedded-operation validation and preflight those bytes before `Op::decode`. Account for nested interpreted content under the declared policy, including cumulative item usage. Preserve the released operation encoding.

**Closure test:** Add regressions for overlong operation names, excessive references under a small item budget, and inner depth beyond the supplied limit. Cover both direct physical operations and a nested remote `TransferBundle`; retain valid boundary controls and frozen operation bytes.

**Root classification:** Nonarchitectural implementation omission in the typed object. The accepted requirement already specifies bounded decoding; correcting the missing embedded preflight does not require changing that requirement.

### [P3-1] Node boundary assertions discard the assembled providers

**Location:** [ic3_boundary.rs:6](/Volumes/projects/limbo/glade-wz/glade/node/tests/ic3_boundary.rs:6), lines 6–20 and 51–58; the foreign-permit test at lines 62–92 also constructs its session directly.

**Violated invariant:** A2’s production-bound consumer evidence must exercise the providers returned by `NodeAssembly::independent_adapters()`.

**Reproduction:** Each “assembled” test obtains `_adapters` and never invokes it. Assertions instead operate on a separately constructed `glade_node::independent::RefusingSession`. The ignored future-success assertion follows the same sequence.

Changing the assembly’s recovery selection would not change these assertions. A future correct provider wired through the assembly cannot make the ignored success test pass while the standalone refuser correctly continues refusing.

**Impact:** The current assertions establish standalone refuser behavior, but do not establish the claimed assembled behavior. This creates a concrete coverage gap at the composition seam. It does not establish an active runtime miswiring.

**Required correction:** Exercise opening and refusal through the returned recovery provider, and evidence operations through the returned evidence provider. Make the future-success RED target that same opening/session path. Keep direct refuser tests labelled as such.

**Closure test:** A consumer assertion must detect an assembly-selection mutant. The future-success test must fail at the current assembled refusal and become satisfiable by the future assembled implementation.

**Root classification:** Nonarchitectural coverage defect; correcting consumers need not change the production interfaces.

### [P3-2] Unknown qualification options silently select candidate mode

**Location:** [gate.py:101](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/compat/gate.py:101), lines 101–104; [check-vectors.sh:4](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-recovery-codec/check-vectors.sh:4), lines 4–5.

**Reproduction:** Invoke `check-vectors.sh --pre-remtoe`, or invoke `gate.py ROOT --pre-remote EXTRA`. Strict mode is selected only by one exact argument count/value combination. Other arguments select candidate validation; the shell wrapper also discards arguments after the first.

**Impact:** An invalid qualification invocation can return the candidate success status instead of reporting an argument error. The printed unqualified message limits the consequence, but automation relying on exit status receives no usage failure.

**Required correction:** Validate argument count and the closed option set at both entry points. Preserve no-option candidate mode and exact `--pre-remote` behavior.

**Closure test:** Unknown options and surplus arguments must return a usage error; candidate mode must remain available, and exact strict mode must still refuse the absent B1 qualification.

**Root classification:** Nonarchitectural argument-validation defect.

## 2. Invariant analysis

The extraction attack failed: original types are preserved, old public paths reexport them, and protected algorithm, encoder, storage API, behavioral assertions, and Glial controls retain their hashes. Lockfile changes add the intended local packages without unrelated registry upgrades.

Dependency direction remains coherent. Data owns representations; contracts depend on data/API; neither acquires the node or concrete physical adapters. New syntax tooling and JSON fixture reading remain development dependencies. Required evidence/recovery methods have no successful default implementation.

The floor graph is nonrecursive. The image retains prior floor metadata, while recovery separately returns the authoritative selecting floor. Interrupted intent retains advanced issuance and guard state. Structural roundtrip fixtures are expressly distinguished from validated physical recovery.

Ingress rejects a foreign issuer before invoking its factory, records started state before construction, and permits settlement only after Ready completion. Pending/drop do not manufacture drain. A cooperating production future can incorporate cancellation and join its work before returning Ready; arbitrary detached tasks and physical guard persistence remain B/C qualification duties. I therefore did not classify conservative Pending/drop behavior as a defect.

Concrete signed-body contexts, version checks, exact zero-terminated domains, signature shape, canonical equality, closed record keys, and ordered semantic maps resist the inspected direct substitutions. Full unsigned counters use eight-byte strings with target-range checks for `usize`.

The three positive vector producers derive bytes from semantic input rather than expected hex. The prerequisite gate checks content/commit pins and currently refuses missing genuine B1 qualification. Symbolic signatures establish representation only.

Gyld preserves the frozen 34/135 baseline and adds five allocations/25 obligations. Its runner changes only the owner-approved single-selection threshold; multi-selection, startup accounting, and I/O treatment remain intact. Baseline Mypy debt and failed runner attempts remain disclosed.

## 3. Risks and next action

B/C remain responsible for genuine evidence, semantic recovery validation, stable ownership, physical crash cuts, joined carrier work, and actual duplex nodes. None is inferred from A2 fixtures.

This review identifies **one architectural and three nonarchitectural typed roots**. The accepted semantic object retains its separate two architectural/one nonarchitectural roots and one merged remediation; this report resets none of that history and establishes no third semantic architectural root.

The next action is one bounded merged remediation addressing P2-1 and P2-2, preferably including both P3 corrections. The changed physical type boundary requires fresh applicable review on one settled tuple, alongside the required Surface gate. Acceptance remains open.