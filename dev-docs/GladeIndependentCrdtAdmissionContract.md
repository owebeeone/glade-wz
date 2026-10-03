# Independent CRDT admission — IC-1 internal contract

**DRAFT for contract review — 2026-10-04.** This is the compiling contract and
behavioral RED tranche. Successful kernel behavior MUST NOT be implemented until
this contract/package/allocation gate is accepted. The semantic design is accepted
at root `4d735893c8db0a9ac9b4de9cde01600873b20ce3`, reviewing `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9` with Glade
`c65a6e87f0c257c15de8db080c29d365a883af85`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` and Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35` unchanged. Full review pins are in the
[semantic design](GladeIndependentCrdtAdmissionDesign.md) and DecisionLog GDL-054–057.
This document applies its remediation1 without changing the accepted semantics.

This is an **internal Rust event/effect contract**, not a user-facing protocol/API
freeze. Existing generated `glade_wire::Op` is consumed unchanged. No root, node,
grant, sysdata, wire schema, store, launcher, enrollment or runtime activation is
modified. The evidence record is
[GladeIndependentCrdtAdmissionContract-Evidence.md](GladeIndependentCrdtAdmissionContract-Evidence.md).

## 1. Boundary and classification

The new package is `glade/contracts/crdt-admission-core`, Cargo name
`glade-crdt-admission-core`. It shares the existing small contracts workspace and
its adopted architecture checker, rather than introducing another workspace or
extracting legacy node code. Its role is **Pure**: deterministic `step(&State,
Event) -> Transition`, explicit owned state, no I/O, ambient time, globals,
cryptographic key custody, merge engine, executor or application runtime. A marker
trait would add no replaceability/behavior contract, so none is introduced.

Normal dependencies are existing `glade-wire` at `glade/wire-rs` and `sha2` for
derived immutable byte digests. Dev dependencies are `serde_json` for actual
supported text payloads/harness traces and `syn` for a source syntax guard.
`architecture-policy.json` explicitly adopts this reviewed classification and exact
allowlist; no existing classification or allowlist is weakened. No discovery runtime
or Gyld implementation dependency is added. JSON is test trace output, not a new
remote schema or public serialization promise. SHA256 here binds exact internal
bytes; production signature/hash domains remain IC-3.

The scaffold implements only safe refusal: every event returns the unchanged state
and `Report(Unavailable)`. It cannot accept, commit, quarantine, advance a head,
produce a read cut or issue a successful receipt. GREEN scaffold/source tests MUST
remain separate from the intentionally RED domain consumers.

## 2. Typed values and immutable byte binding

The definitive compiling types are
[`types.rs`](../glade/contracts/crdt-admission-core/src/types.rs). Table entries below
state their semantic obligations; a public Rust data struct is not cryptographic
verification or authority to consume a peer DTO as trusted state.

| Value | Internal meaning and mandatory binding |
| --- | --- |
| `Bytes` | Owns private bytes and SHA256 derived in its constructor; exposes shared read-only slices and the derived digest. No caller-supplied digest can disagree with these bytes. |
| `Operation` | Owns private existing decoded `Op`; canonical inner CBOR is derived with existing `Op::to_cbor`/`cbor::encode`. Neither mutable decoded fields nor independently claimed canonical bytes are exposed. No CRDT merge is implemented here. |
| `Instance` | Authenticated descriptor digest plus resource incarnation. Equal display names, wire origin strings or shares do not confer rights across instances. |
| `Descriptor` | Complete typed creation/root/resource/share/glade_id/key/profile/mode/bounds snapshot and immutable evidence bytes. Its fields are proposed internal evidence-provider inputs, not a frozen descriptor wire encoding. |
| `OriginCertificate` | Instance, never-reused canonical `Op.origin`, certified epoch, display name, writer key, authenticated requester and exact certificate bytes. Display names may repeat; canonical origins may not reset across epochs. |
| `EvidenceSet` | Exact certificate, permit, policy-cut and optional prior admission bytes. `Some(admission)` alone proves nothing. |
| `Facts` | Explicit trusted evidence-provider result for one pending query. Validity booleans are test injection/adapter obligations, not a cryptographic implementation. |
| `Candidate` | Instance plus immutable operation and evidence; no authority follows from its arrival or caller claims. |
| `Slot` | Instance + canonical origin + strict Glade zero-base sequence. It is not a display identity. |
| `Receipt` | Exact instance/op/admission/stable retry identity and explicitly achieved storage class. `VolatileTest` is a fixture claim only; `LocalProcessRestart` requires a qualified physical provider. |

Descriptor/certificate fields intentionally remain typed snapshots while their
canonical remote encodings are unresolved. The verifier MUST validate that the
**exact descriptor bytes** authenticate **every supplied parsed field**, and that
`descriptor.canonical.digest() == instance.descriptor`. `descriptor_valid` MUST be
false on any mismatch, unsupported version or unauthenticated genesis. The kernel
MUST also check instance equality, explicit profiles, wire scope equality and numeric
bounds; it MUST NOT infer genesis from discovery/placement or independently trust a
mutable parsed field. A trusted verifier MUST authenticate the exact certificate
bytes and bind its instance/origin/epoch/key/requester fields to them. The kernel
MUST check `Facts.certificate.canonical == query.evidence.certificate`, canonical
origin equals the operation origin, instance equals the selected descriptor and
issuance uniqueness holds. A host cannot satisfy this by echoing DTO fields.

Fixture descriptors use `name.as_bytes()` with separate typed fields; certificates,
permits, policies, admissions and batches are explicitly synthetic byte strings.
Those fixtures exercise binding/decision semantics, **not canonical signed encoding**.
`State` construction/preloaded accepted records are trusted test/recovery inputs.
A production host MUST restore a validated state or re-verify exact stored records;
serializing trusted `Facts` booleans and reading them back is insufficient.

## 3. State and event/effect obligations

`State` owns separate `InstanceState` maps, outstanding verification continuations,
staged commits, uncertain commits, outstanding lookups and a checked monotonically
allocated effect ID. No process-global namespace/cache/counter is involved. IDs MUST
NOT wrap or be reused while a callback can still arrive. Rehydration MUST preserve
or safely invalidate outstanding identities. Any future internal persisted encoding
MUST bind the complete staged values and version; this gate does not select a disk
format or make the generated wire API change.

Each `InstanceState` holds descriptor, storage promise, local revision, accepted
custody/facts/receipts, bounded candidates and security evidence, qualified fork
records, certified origins, contiguous common-prefix heads, missing slots,
quarantine, policy/time cut and committed charges. `heads` MUST NOT select an
arrival-dependent hash at a convicted fork slot. A projection cut is derived from
custody, never used to recursively define historical qualification.

| Event | Required decision/effects after the gate |
| --- | --- |
| `SubmitLocal(Candidate)` | Exact existing identity retry first returns the original receipt without reauthorization/reminting. New intent requires explicit independent descriptor, currently authorized requester/writer/permit and wholly eligible structural closure; emit bound verification, seal, then atomic commit. No claim-holder inquiry. |
| `OfferReplica(Candidate)` | Verify historical admission/evidence, retain qualified records/proof closure using recovery capacity, request missing ancestors, derive eligibility; current revocation does not erase a historically valid admission. No source invocation or metadata-only substitute. |
| `EvidenceResolved(EvidenceReply)` | Match an outstanding query in all fields before consuming facts/seal; invalid/mismatched/unavailable results cannot admit. Retain continuation until resolved or explicitly abandoned without success. |
| `CommitResolved(CommitReply)` | Match exact outstanding plan/instance/batch/revision and promised storage class. Apply only after a complete atomic committed barrier; known failure and unknown outcome are distinct. |
| `LookupResolved(LookupReply)` | Match exact outstanding lookup and retained uncertain plan, including expected revision/op/batch and returned receipt. Recover original committed batch/receipt, release only a proven absent batch, or retain unknown reservations. |
| `ObservePolicy` | Explicit trusted policy/time input; reject malformed intervals/regression below the retained floor. New-intent queries pin this cut and entire conservative interval; a stale verification reply cannot seal or stage a new commit; commit-start ordering is specified below. Historical replay uses the recorded admission cut. |
| `ObserveFrontier` | Bounded peer inventory hint only: validate numeric bounds, request missing records, and offer local missing records. Advertised heads do not become qualified custody, unique fork heads, authorization or complete history. |
| `OfferFork` | Pair of candidates, not proof of conviction. Reuse already qualified retained records; separately verify missing rival admission/proof closure. Convict only after both independently qualify historically. |
| `ResumeRecovery` | Resolve retained unknown commits/outcomes and pending qualified closure without reminting identities; no new epoch using an old canonical origin. |
| `Read` | Return explicit local cut, complete-local status, pending slots, qualified custody, common-prefix heads, fork slots and eligible operation identities; no global-currentness or peer-replication claim. |

Effects are concrete owned values, not arbitrary callbacks: `Verify(Query)`,
`Seal(Query)`, `Commit(CommitPlan)`, `Lookup(LookupRequest)`, `RequestMissing`,
`OfferMissing`, `ProjectionChanged` and `Report(Outcome)`. The host executes them
and reports typed results; it does not mutate the kernel's private conceptual
algorithm or replace admission with successful mock outputs. The current structs
are public internal data for compiling downstream consumers; a future wire ingress
MUST NOT be allowed to author `Facts`, `Accepted` or staged state.

`Query` carries full immutable `Operation`, complete descriptor, instance, all exact
evidence bytes, ID and validation mode. The provider has the bytes it must check,
not only a digest requiring hidden host state. `PendingVerification` retains the
same query/candidate, verified facts and original verified query through the seal
phase. `AdmissionSeal` queries additionally bind those exact verified facts and
`validated_under`, the original Local policy/time or Historical validation mode.
Verify requests have no validated_under; a seal cannot recursively claim
AdmissionSeal as its original validation mode. A delayed, foreign, duplicate or changed query response
MUST yield `CallbackMismatch` without consuming a valid continuation/reservation.
The same rule applies to commit and lookup callbacks. No arbitrary event may
supply trusted facts outside its matching outstanding query.

## 4. Local authorization, historical qualification and fresh origins

`Mode::Local` pins current policy digest and conservative time interval. Its
requester/writer certificate and permit MUST be signature-valid, instance/profile/
action/sequence scoped and currently allowed. **Every point** of the supplied
interval MUST be inside the finite permit window; expiry, straddling uncertainty,
revocation, unknown evidence/time, issuance conflict and missing eligible closure
refuse or remain explicitly pending. Fixtures use window `[1,100]` ms, interval
`[20,21]` ms, sequence range `[0,100]`, not deployment defaults. A supplied prior
admission byte string MUST NOT bypass new-intent checks/sealing. An exact retained
retry is already admitted and returns its original identity/receipt even after
expiry, without a new effect or new canonical sequence.

`Mode::Historical` instead requires authenticated admission bytes, authorized
historical admitter, its recorded policy cut/time, historically valid scoped writer
permit, signatures and finite acyclic structural closure. Later current revocation
or expiry cannot retroactively erase it. This relies on the design's trusted
admitter/time assumption: a compromised admitter/backdated forged trust assertion
is not solved by this pure component. Real provenance, revocation ordering,
antirollback and clock uncertainty adapters remain live gates.

Qualification MUST be a finite closed structural/evidence predicate independent
of **derived quarantine**. A dependent replay whose support is historically
qualified but quarantined MUST retain custody/evidence while remaining projection
ineligible. A local new intent with that same quarantined support MUST refuse
`IntegrityConflict`. Missing/nonqualified ancestors remain candidates/pending and
cannot advance the complete head or project. Bounded recovery stops explicitly on
capacity; an unavailable proof is not a grant.

Strict Glade seq/ref coordinates are zero based: `seq >= 0`, seq0 has no prev,
seq>0 has an exact predecessor digest. All seq/ref values MUST permit the checked
`+1` mapping to the existing Taut coordinates: values are strictly below
`9_007_199_254_740_991`, with bounded refs/bytes/origins and no negative refs.
Lamport/profile payload validity also MUST be checked; no unchecked narrowing or
new merge/lamport interpretation is introduced. Byte-preserving supported payload
validation/projection belongs to the existing profile/provider/Taut code.

Epoch recovery MUST issue a fresh canonical origin throughout certs, chain keys,
refs, quarantine, retries and Taut actor IDs. The test deliberately retains old
`writer-e0:0` inserting atom A, then uses `writer-e1:0`/epoch1 referencing the old
A and inserting atom B after A: released text MUST be **AB** in both arrival
orders. Repeated display names do not permit same-canonical-origin seq0 reset.

## 5. Forks, read cuts and identity sets

Two *historically qualified*, distinct canonical records at the same
(instance, canonical origin, seq) slot convict an origin fork. An authenticated
signature without qualifying admission is insufficient. The kernel MUST keep
security evidence for a bare unadmitted, historically unauthorized or expired rival
and preserve legitimate A + dependent D custody, receipts and projection **AD**,
regardless of delivery order. Two qualifying seq1 rivals after A instead produce
quarantine floor1: both branches and their transitive dependents leave eligibility,
A remains eligible, and both branch records/dependents stay in qualified custody.
The released text witness is then **A** in both orders.

`ReadCut.contiguous_common_heads` explicitly ends at the common prefix before a
fork; `fork_slots` and `historically_qualified` expose retained conviction/custody.
`eligible` identifies the deterministic eligible **set**, not an arrival-ordered
Vec contract. Consumers MUST compare canonical identity sets; any returned sequence
MUST contain each identity once. Actual Taut handles profile causal delivery/buffer
semantics. `complete_local` MUST be false for unresolved observed history,
uncertain commit, pending policy/proof or bounded recovery refusal. Convicted fork
evidence is explicit; a fully retained locally classified fork may have a complete
local classification with a reduced eligible projection. Neither case means all
remote history has arrived or there is one globally unique frontier.

`Operation` digest is immutable derived inner identity; a same-slot/different-op
local retry conflicts rather than remints. Read/receipt namespaces include the
canonical descriptor/incarnation. No origin/certificate/permit/query/commit from
instance X may authorize instance Y, even with identical display actors.

## 6. Atomic staging, unknown outcome and reserved recovery

`CommitPlan` binds ID, instance, expected local revision, immutable batch bytes,
commit kind, all retained candidates, complete `StagedAcceptance` continuations and
capacity charges and optional `LocalPrecondition` (exact policy/time cut for new intent). Each staged acceptance carries immutable operation/evidence,
verified query/facts, optional seal query, exact admission bytes and its stable
receipt preview. Thus a matching committed/lookup callback can reconstruct the
accepted record without hidden host state or interpreting arbitrary future facts.
A receipt preview MUST NOT be exposed as success before the barrier.

The future kernel MUST derive its internal batch from the complete staged values;
its byte digest MUST change if descriptor/candidate/evidence/facts/seal/outcome
changes. The host MUST commit those exact bytes/records atomically under the stated
revision and promise. Choosing the internal versioned batch encoding is IC-2
engineering, not an unreviewed remote wire schema; synthetic preloaded test batches
are trusted state fixtures, not an encoder/disk conformance demonstration.

Policy observation and **physical commit start** MUST be ordered per instance by
the trusted host. Immediately before beginning a new-intent batch, the host MUST
atomically check its LocalPrecondition against the current supplied policy/time
cut and permit validity. Revocation/change observed **before start** invalidates
the queued plan: the host returns KnownNotCommitted, or the intent is freshly
verified/sealed/restaged before any mutation. A stale matched verification query
returns PolicyPending and cannot seal/commit. A policy observation MUST NOT delete
an emitted/staged/uncertain plan or assume that cancellation rolled it back.

Once the host has begun under the valid serialized cut, a later policy event and
a delayed Committed callback refer to an already admitted historical record. The
kernel MUST retain that record/original receipt, also when the reply was unknown
and later recovered. It MUST NOT reauthorize it using current revocation or
retroactively cancel physically committed custody. The same prior-cut identity
remains eligible subject to ordinary historical/fork rules. The admission seal and
plan bind the validated start cut; no successful receipt precedes the durable
barrier. A storage provider sending Committed despite a failed start precondition
violates this contract. The pure RED tests inject this ordering/commit assertion;
actual policy/clock serialization with physical commit and cancellation cuts is
an IC-3/ICD-014 adapter gate, not established by a volatile reply.

Only `Committed { revision: expected_revision + 1, storage: promised_class }`
installs custody/head/outcome/projection and yields success. `KnownNotCommitted`
releases that exact staged reservation and reports no success or partial visible
accepted tail. `OutcomeUnknown` retains the entire plan, facts/seal/receipt identity
and reservation; it yields uncertainty and exact lookup/recovery. It MUST NOT
re-authorize, create a replacement operation or advance its ambiguous origin.

The minimal host serializes commit barriers per instance. An uncertain instance
MAY pause all new commits in that instance until exact revision/outcome recovery;
it MUST NOT stall independent instances. Reservations count exactly once across
staged/unknown maps. This permits strict CAS/revision recovery without an unexplained
arrival-selected revision. Lookup replies bind plan, instance, expected revision,
operation and batch; a committed lookup includes actual revision and exact stored
receipt. Mismatch/stale/unknown cannot release reservations. A proven absent batch
may be rescheduled only as the original intent after applicable new-intent checks,
never with a reminted canonical identity while its outcome remains uncertain.

Committed byte charges and staged/unknown reservations are separate. Admission
MUST reserve enough bounded space for operation, evidence, outcome and required
recovery metadata before emitting Commit. Full edit capacity refuses local new
intent; it cannot spend recovery/evidence reserves. Replica closure/rejoin uses
reserved recovery budget with zero edit charge. Exhausted recovery/evidence/pending
origin/ref/byte capacity returns explicit `Capacity`/bounded missing-history status,
never eviction of accepted ops/tombstones/retry/fork evidence or fabricated complete
reads. Arithmetic MUST be checked; allocation/request/event work MUST be bounded.
Actual allocator/RSS/physical quota guarantees require adapter tests beyond these
fixture byte accounting obligations.

## 7. Compiling consumers and meaningful RED

[`records_host_contract.rs`](../glade/contracts/crdt-admission-core/tests/records_host_contract.rs)
is an external Rust integration consumer importing only the crate's public internal
values/`step`, not private helpers. Its host follows emitted Verify/Seal/Commit
results with explicitly injected facts and **VolatileTest** barriers. Its bounded
32-event driver refuses unavailable unknown evidence. Preloaded accepted and staged
state is labelled trusted model state. No crypto, real Records implementation,
fsync, restart or network result follows from it. Behavior assertions deliberately
fail on the refusing scaffold; compilation/missing symbols are not the RED evidence.

[`text_admission_trace.rs`](../glade/contracts/crdt-admission-core/examples/text_admission_trace.rs)
is a separately compiled downstream consumer emitting only operations actually
eligible in a `Read` returned by the kernel. Missing reads yield an explicit
unavailable trace; expected accepted operations are never substituted. The JS
consumer [`independent_admission_contract.mjs`](../glial/test/independent_admission_contract.mjs)
feeds these immutable bytes into **released `@owebeeone/taut-shape` 0.9.1** and uses
its real `CrdtNode`/`projectText`, with checked zero-base coordinate mapping.
No new text merge or JS admission implementation is introduced.

Exactly eight rows MUST exist; omission, duplicates or empty traces fail:
AB/AD/qualified-fork A in both orders, ABC and independent-instance isolation.
ABC preloads the existing `concurrent_siblings` A, then requires both isolated local
receipts for B/C and opposite-direction app-record offers, exactly `{a:0,b:0,c:0}`
and **ABC** both ways. Instance X remains A while authorized Y projects Y and a
separate denied Y writer cannot borrow X's rights. These are delivered deterministic
host events, not actual live automatic mesh sync. ICD-012/T12 remains IC-3; no
browser/subscription/network/physical custody claim follows. The exact A/B/C
payloads are checked byte-for-byte against an app-owned digest-pinned copy of
`taut-shape/corpus/scripts_text_crdt/02_concurrent.json`; a released Taut reference
positive control projects all three corpus orders before the RED assertions. Other
fixture payloads use the same supported codec without inventing a numerical surrogate merge.

| ICD rows | Compiling RED behavior owned here | Remaining qualifications |
| --- | --- | --- |
| 001/003/015 | Explicit descriptor/mode/profile refusal; legacy/strong remain unsupported in this pure lane. | Authenticated canonical genesis/capabilities, mixed binaries, root and whole-store seal enforcement. |
| 002/004/005 | Isolated receipt, two instances, foreign cert rejection, exact candidate/evidence/descriptor/mode callback matching. | Actual authenticated requester/writer/admitter crypto and node/client integration. |
| 006 | Entire finite window/uncertainty, current revocation versus historically valid replay, stale validation, delayed prior-cut commit/unknown recovery, and unadmitted/unauthorized/expired rivals either order retain AD. | Real historical cuts, clocks, revocation/custody trust and security evidence durability. |
| 007/008/009 | Fresh origin AB; reused ID, numeric/ref/zero-base failures; predecessor requests; two qualified rivals and dependent quarantine/common-prefix; historical dependent custody versus local refusal. | Canonical certificate issuance/antirollback, broader profile corpora, adversarial delivery/state bounds. |
| 010/011 | Stable retry, bound commit/lookup callbacks, known failure/unknown reservation, original receipt recovery, explicit read cuts. | Restart/sole-copy loss/declared physical receipt qualification. |
| 012/013 | Actual app operations in deterministic opposite-direction offers; edit/recovery budgets/exhaustion. | Automatic real-node duplex operation transfer, bounded production ledger/cursors/enrollment. |
| 014/016/017 | Staged typed custody, real released text consumer, classified package/tooling/source guards. | Physical interruption atomicity, tombstone/cursor/editor/Glial lifecycle integration, live adapter conformance. |

## 8. Gyld combined graph and frozen provenance

Gyld app files are owned by `/Volumes/projects/limbo/gyld-wz/gyld`; they are an
inherited **overlay**, not an overwrite of its frozen baseline. Existing
`glade-problem-sources.json`, problem declarations, architecture v3, Iroh evaluator,
weights and engine APIs are untouched. The supplemental
`examples/glade-independent-crdt-sources.json` freezes the full accepted semantic
design, digest/inspection scope/accepted tuple and digest of the unchanged base
ledger. Normal capture/tests use only app-owned frozen sources; a standalone
examples copy has no workzone filesystem dependency.

`examples/glade-independent-crdt.gyld.py` inherits `GladeArchitecture` unchanged,
adding Pure `IndependentAdmissionRules`, its internal contract/state, seventeen
source-qualified ICD concerns and four journeys. Existing responsibility definitions
and 24 allocation owners remain unchanged. Seven additional capabilities allocate:

| New capability | Owner | Exact source requirements |
| --- | --- | --- |
| `independent_admit` | `independent_rules` (Pure) | ICD-001/002/003/004/006/007/008/009/010/011/013 |
| `independent_verify` | existing `admission` | ICD-005 |
| `independent_sync` | existing `records` | ICD-012 |
| `independent_durable` | existing `records` | ICD-014 |
| `independent_exclusion` | existing `admission` | ICD-015 |
| `independent_consumer` | existing `glial` | ICD-016 |
| `independent_conformance` | existing `conformance` | ICD-017 |

Rules depend on existing WireProtocol and the internal contract, and cooperate with
Admission/Records; this does not assert implemented calls or change the existing
discovery transaction. Live admission/Records consume the pure effects in IC-3;
Glial/Taut consume only eligible operation sets/projection notifications. The
original ownership graph continues to describe all other shapes/strong resources.

Capture MUST preserve all inherited source-qualified relationships and exact
allocation owner labels, then require precisely 17 ICD links, seven new allocations
and four new journeys: **31 allocations, 124 obligations = 107 inherited + 17 ICD**.
Snapshot lineage/root/digests and occurrence/obligation generated IDs differ;
no raw ID equality is claimed. Unchanged inherited sources and original baseline
capture validation are checked before the exact link/owner comparison. Optional immutable baseline reuse MUST validate source-derived catalogue, ledger
pin and lineage/revision; fresh combined capture still checks all exact links/owners.
Changed source/ledger provenance refuses reuse. Mutated
source pins or omitted ICD links fail; no base gate/evaluator is weakened. Source
and allocation accounting does not mark any requirement satisfied.

Before IC-2 behavior code, run the standalone capture/checks in the evidence doc
and review these files with this contract tuple. No Gyld engine/evaluator expansion
or newly invented dependency/trait framework is needed.

## 9. Selection, gates and continuation

`check.sh --list crdt-admission` selects only the new Pure package; `--list all`
retains the ten existing contracts and adds it. The normal selected command is
`sh glade/contracts/check.sh crdt-admission`; at **IC-1 it intentionally exits RED**
after the architecture gate and compiling behavioral assertions. Separate GREEN
scaffold/source/format/clippy commands are mandatory now. `check-text-contract.sh`
compiles the real trace consumer and executes the JS contract, intentionally RED.
After contract acceptance, the same behavioral commands MUST turn GREEN through
the smallest successful kernel implementation; expected failure MUST NOT be
silently disabled, filtered, inverted or replaced with a private successful model.

Selector/framework regression fixtures were written and observed failing BEFORE
the selector/guard was changed. ARCH-002 still requires an untouched positive
control and **exact** undeclared-shaku refusal for EVERY workspace member, including
Pure and a synthetic twelfth member. The temporary copy now resolves the existing
wire dependency at its actual `glade/wire-rs` path. No framework exemption is added.

The syn syntax guard inspects disabled inline module contents and all visited
attribute contexts (including associated/foreign/field/variant declarations),
allowing conditional attrs only on an enclosing inline platform module. Negative
fixtures precede checker corrections. JS uses existing TypeScript AST tooling to
reject unbraced if/else/loop bodies, with negative fixtures. Rust's parser rejects
unbraced control bodies. Macro token bodies are opaque to syn; `cfg_if!` is an
explicit permitted boundary, not a claim of a universal macro-expansion lint.
No broader legacy conditional-boundary migration is claimed. The existing Glade
process-global checker scans the added src root through its unchanged wildcard;
no allowlist is added/loosened.

The next accepted IC-2 outcome is a reusable production-bound **pure component**:
valid partitioned local decisions and stable receipts at its supplied storage
class, qualified retained closure/repair plans, deterministic eligible sets and
actual released-text witnesses across independent instances. Its injected providers
and event transport remain fixtures until qualified live adapters exist. IC-3/4
must still settle canonical evidence/container domains/schema/negotiation, real
requester/node/origin certification and custody, finite offline time/policy,
atomic operation/outcome journal and restart, automatic duplex app-op mesh,
clients/Glial intent/receipt behavior, capacity/loss, version/root/seal exclusion and
physical failure tests. Existing Store/SnapshotStore/signer/envelope/grant/Hello
APIs do not constitute those adapters and cannot unlock the complete live feature.

Actual deployment owner choices (authority/operator keys and recovery custody,
permit durations/time trust, replicas/loss tolerance, quotas, existing-store
inventory/migration and activation) remain later inputs. They do not block this
parameterized pure component. No production values/defaults are inferred from
fixtures. A later new user-facing surface requires its own Surface/product review;
this internal contract gate is not that freeze.
