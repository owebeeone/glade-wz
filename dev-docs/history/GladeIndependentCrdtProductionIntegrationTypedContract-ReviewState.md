# IC-3A2 production adapter typed contract — STATE-AXIS REVIEW

**Review object:** IC-3A2 implementation candidate controlled by `dev-docs/GladeIndependentCrdtProductionIntegrationTypedContract.md` at workspace root `c3608e286272e7de412f8de52496cbfc0a355a75`, dated 2026-10-04. Scope includes the full Glade change from `37dff286ce1eb9690204d4a7d14c940333396a30`, external Gyld change from `95a426595bba8e248a5f484272e483a070c73918`, and typed documentation/evidence.

**Baseline:** The five-repository tuple below was verified at START and END. Sources were inspected with `cat`, `sed`, `nl`, `rg`, scoped `git diff`, `git show`, hashes, and read-only Python. Working implementation bytes match the filed source manifest.

**Date:** 2026-10-04.

**Axis:** Durable-state semantics, recovery legality, ownership, fail-closed behavior, bounded decoding and the credibility of refusing integration witnesses. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on their testimony. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block: one architectural typed-representation root and two nonarchitectural implementation/evidence roots. I pre-commit to GO on a revision that resolves P2-1, P2-2 and P2-3 as specified, subject to verification of the corrections and absence of newly introduced defects.

---

## 0. Evidence base

### Settled tuple

| Repository | START HEAD | END HEAD |
| --- | --- | --- |
| Glade workspace root | `c3608e286272e7de412f8de52496cbfc0a355a75` | `c3608e286272e7de412f8de52496cbfc0a355a75` |
| Glade | `8b0595551dd90f32da4698fff9358aa30dbf2548` | `8b0595551dd90f32da4698fff9358aa30dbf2548` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `03428fb36649d71541fd76f2471533e384be8209` | `03428fb36649d71541fd76f2471533e384be8209` |

All workspace-relative paths below resolve under `/Volumes/projects/limbo/glade-wz`; external Gyld paths resolve under `/Volumes/projects/limbo/gyld-wz/gyld`.

Read the complete canonical State prompt, root instructions, Glade member instructions, review-loop skill/template, build entry, library boundary policy, package architecture, typed contract and review ledger. Read production design §§1–9, production plan, admission plan, storage-attempt contract, resource-consistency requirements, and relevant archived identity/lifecycle authority. Glial and discovery contained no member `AGENTS.md` found by the scoped inventory.

Implementation inspection included:

- Admission-data original types and complete production/proof types.
- Evidence API; recovery API and ingress implementation, with their consumer/source checks.
- Recovery codec root, bounds, primitive/container values, batch decoder, profile/proof codecs, schema records/enums, roundtrip/image/batch/vector witnesses and independent Python/TypeScript consumers.
- NodeAssembly’s new method, independent adapter constructors, refusing evidence/recovery implementations and `node/tests/ic3_boundary.rs`.
- Contract/node policy and selector changes; original-type extraction and source-check changes.
- Gyld instructions, README, architecture policy, engineering policy, production declaration/capture, sidecar and changed runner/tests.

Read the filed TypedEvidence, including its chronological command index and limitations. Its reported executions were not rerun. Read-only hash inspection established:

- All **58 Glade implementation hashes** match the filed manifest.
- All **four root-document hashes** match.
- Of **45 protected compatibility files**, only the declared core-types extraction and appended source-boundary test differ.
- The original core-types source occurs byte-for-byte in admission-data.
- Gyld’s sidecar contains **25 requirements**, and its embedded semantic-source hashes match.
- The compressed run-log hash matches the filed `4fa3539d7d3dbb9275fe8cf250afa148488686f4b4b757e425436424e6105199`.
- The scoped process-global allowlist diff is empty.

No files were written. No builds, tests, network operations, live actions or Git mutations were performed. Counterexamples below are source traces, not claimed executed regression results. No current peer report was read or requested.

## 1. Findings

### [P2-1] Generic physical envelopes erase the concrete identity type

**Location:** `glade/contracts/crdt-recovery-codec/src/values.rs:109–160`, particularly `Box<T>` at lines 135–142; `src/lib.rs:26–55`; `src/schema/storage.rs:58` and `src/schema/production.rs:8`.

**Violated invariant:** The physical envelope’s type discriminator must reject wrong contexts. The existing regression explicitly requires that a plan identity cannot substitute for a receive-guard identity. The typed contract describes `[type-name, typed-value]` and wrong-context refusal at lines 79–83.

Every instantiation of `Box<T>` uses the same literal `KIND = "Box<T>"`. `Vec<T>`, `Option<T>`, tuples and maps similarly omit their actual type arguments. Nested record maps contain field positions, without a second record-type discriminator.

**Reproduction:** Construct the existing regression’s `PlanKey { namespace: d, instance: i, number: 1 }`, box it, encode it, then call `decode::<Box<GuardId>>` on those bytes using adequate limits.

Both envelope checks compare against `"Box<T>"`. `Box<GuardId>::decode_value` delegates to `GuardId`. The encoded PlanKey fields are structurally identical to GuardId: `namespace: Digest` becomes `store: StoreIncarnation`, whose codec also consumes exactly a Digest; instance and number match. Consequently the decoder returns a GuardId, rather than `Invalid`. The same attack works through nonempty vectors and optional values.

**Impact:** The public physical codec permits identity-class substitution through supported container roots despite its direct-root refusal witness. This does not itself issue a trusted ingress permit, but invalidates the structural type-boundary guarantee offered to recovery consumers.

**Required correction:** Give permitted root types concrete, recursively unambiguous discriminators, or restrict envelope encode/decode to explicitly named concrete roots while keeping container codecs available for nested fields. Do not retain a generic placeholder as an exact type identity.

**Closure tests:** Preserve direct PlanKey→GuardId refusal and add boxed, vector, optional, tuple and map substitution controls. Valid same-type container roundtrips must remain green; wrong-type decodes must refuse before returning values.

**Root classification:** **Architectural, typed representation boundary.** The defect lies in what the public envelope binds, requiring a root-type/interface decision. It is a typed implementation counterexample, not a newly established flaw in the accepted semantic floor/guard design.

### [P2-2] Embedded operation decoding bypasses injected name and item limits

**Location:** `glade/contracts/crdt-recovery-codec/src/values.rs:192–204`; `src/bounds.rs:55–60`; `src/lib.rs:38–55`; `src/profile.rs:35–59`. Supporting call: `glade/wire-rs/src/generated.rs:218–250`.

**Violated invariant:** TypedContract line 83 and IC3-DISK-001 require preflight of injected byte/item/depth/name limits before typed allocation. Existing Operation bytes may retain their released encoding; that does not exempt their parsed contents from the outer decoder’s trusted limits.

The outer preflight treats an Operation as an opaque byte string. `Operation::decode_value` then clones those bytes and invokes `Op::decode`, which uses its own fixed depth and encoded-length bounds. It receives neither `DecodeLimits.name_bytes` nor `DecodeLimits.items`.

**Reproduction:** Encode a canonical Operation with short share/Glade ID, a 256-byte origin, and empty refs. Decode its physical envelope with:

`DecodeLimits { bytes: 8192, items: 8, depth: 2, name_bytes: 128 }`.

The outer array, `"Operation"` name and bytes occupy only three preflight items. The origin is inside the byte string, so the name check never sees it. `Op::decode` accepts and allocates the 256-byte origin; reencoding equality holds. The root returns success despite the 128-byte name limit. An operation containing enough canonical refs likewise bypasses the outer item budget.

The same path is reachable through Candidate records in physical images and remote TransferBundle profiles. The batch decoder separately preflights embedded operation CBOR; that protection is absent here.

**Impact:** Validly encoded input can exceed configured typed name/cardinality limits during recovery or transfer decoding. Post-decode semantic validation cannot supply the promised pre-allocation refusal.

**Required correction:** Apply the injected limits to embedded operation parsing before allocating its parsed fields, including an explicitly defined accounting rule for nested items. Carry the required context through nested decoding or perform a schema-aware preflight that covers every Operation field. Preserve released Op bytes.

**Closure tests:** Add the root Operation counterexample and nested Candidate/TransferBundle equivalents. Test exact name-limit acceptance, one-byte excess refusal, item-limit excess through refs, and valid bounded roundtrips. The tests must attribute refusal to the injected budget rather than unrelated malformed data.

**Root classification:** **Nonarchitectural implementation omission.** The accepted bounds contract is sufficient; this decoder path fails to apply it. Repair need not change admission semantics, package roles or released operation encoding.

### [P2-3] Node boundary witnesses do not consume the assembled adapters

**Location:** `glade/node/tests/ic3_boundary.rs:6–20`, `51–58`, and `62–92`; `node/src/assembly.rs:314–316`. Claims: TypedContract line 93 and TypedEvidence’s “actual assembled” controls.

**Violated invariant:** A2 requires actual NodeAssembly refusing-provider consumers and a genuine future-success RED at that seam. Constructing the assembly alongside an unrelated concrete provider is not consumption of the assembly’s binding.

**Reproduction:** Trace any of the tests. The first two obtain `node.independent_adapters()` into unused `_adapters`, then construct `glade_node::independent::RefusingSession` directly. The future-success test repeats exactly that pattern. The foreign-permit test also constructs RefusingSession directly.

A wiring error in the returned adapter bundle cannot change the tested session’s outcomes. Conversely, replacing the assembled recovery provider with a successful qualified B provider leaves the future-success test permanently asserting against the standalone RefusingSession.

**Impact:** These assertions test the refuser implementation, while the evidence attributes them to assembled consumption. They neither protect the assembly’s refusal boundary nor provide a success RED that follows that boundary into B.

**Required correction:** Route controls through providers obtained from `NodeAssembly::independent_adapters()` and their actual contract operations. At this refusing stage, a valid open request should exercise assembled `open_replica` refusal and prove that refusal prevents downstream receive work. The future-success witness should obtain its session through that same assembled host path. Keep its physical qualification deferred and its RED status explicit.

**Closure tests:** An assembly-binding mutant must invalidate the relevant consumer control. Direct refuser tests may remain separately labelled. Verify that the future-success assertion follows the assembled host/session rather than a hard-coded refuser; correct the evidence’s attribution accordingly.

**Root classification:** **Nonarchitectural witness/evidence defect.** The existing seam supports a consumer test. The correction is to test that seam, without inventing a successful adapter or changing production defaults.

## 2. Invariant analysis

The floor/image graph avoids a hash fixed point: ReplicaImage embeds a prior floor, while ReplicaRecovery carries the independently authoritative current floor. FloorIntent retains advanced issuance, attempt/invocation/guard floors and guard history. I found no representation-level requirement to reconstruct those advances solely from an older image. This is structural sufficiency, not proof of write/sync/restart behavior.

The ingress utility checks issuer identity before invoking its factory, marks started before work escapes, retains finite guard history, rejects duplicate IDs and distinguishes never-started abandonment from completed settlement. Pending/drop does not mark drained. I investigated the lost-capability cancellation path but did **not** establish a finding: a correctly owned supplied future can include cancellation and join completion before returning, whereas dropping it deliberately retains conservative uncertainty. That conservative result must not be relabelled successful cancellation qualification.

Guard/floor/observation representations preserve the discriminator needed when all post-consumption writes fail. Combined-cut requirements retain exact per-record obligations, same-slot rivals, active guards and permanent loss. No reset operation clears kernel sticky incompleteness. An empty later inventory is not represented as authority to erase earlier obligations.

Current adapters return unavailable/refused outcomes, never Validated recovery, genuine Facts, seals or consume permits. Thus I found no current successful path inventing durability or authorization. P2-3 concerns the strength of their assembly evidence.

Unsigned counters use eight-byte unsigned representations; usize conversion is checked. Digests require 32 bytes. Fixed maps reject extra/reordered fields, and signed-body decoding distinguishes concrete contexts even where signature domains coincide. These protections held for named concrete roots; P2-1 identifies their container-root exception.

The unchanged batch decoder preserves original fields and first-use interning and charges repeated owned expansion. Its recorded amplification regression is relevant evidence. I do not extend that result to the separate embedded-operation path in P2-2.

Gyld preserves explicit 34/135 inheritance and adds five allocations/25 obligations without asserting runtime satisfaction. The runner change is the owner-authorized single-selection five-second threshold; multi-selection ten seconds, startup separation and I/O behavior remain. Filed failed attempts and six baseline Mypy diagnostics remain visible.

## 3. Risks and next action

Actual cryptography, trusted time, stable locks, floor synchronization, physical allocation, crash/restart and duplex transport remain B/C obligations. Structural roundtrips cannot qualify those outcomes. No finding here demands implementing them prematurely.

The next action is a lane-owner verdict merge and one bounded remediation disposition covering P2-1–3. Preserve the semantic object’s existing two architectural/one nonarchitectural roots and one remediation round, and the typed object’s recorded accounting. This report establishes no additional semantic-design counterexample. The typed representation correction requires review of its changed boundary; none of these findings may be self-closed from writer GREEN or bypassed by renaming the object.