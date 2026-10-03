# Independent CRDT storage-attempt typed contract

Date: 2026-10-04. Status: **DRAFT — new typed contract, package/allocation and
compiling RED checkpoint. Successful kernel behavior, a physical storage host,
wire/API freeze and activation are not accepted by this object.** The unchanged
admission `step` still returns unchanged state and `Report(Unavailable)`.

This implements the next gate of the owner-authorized [accepted lifecycle
design](GladeIndependentCrdtStorageAttemptDesign.md), reviewed at root
`d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6` and recorded accepted at inspection
root `01602d89a7130df9cc09c6f4ba889d2b7ab4a4bc`. The [accepted semantic
CRDT design](GladeIndependentCrdtAdmissionDesign.md) remains controlling.
The [failed IC-1 contract](GladeIndependentCrdtAdmissionContract.md), its
[evidence](GladeIndependentCrdtAdmissionContract-Evidence.md),
[remediation evidence](GladeIndependentCrdtAdmissionContract-Remediation1-Evidence.md),
[escalation](GladeIndependentCrdtAdmissionContract-Escalation.md) and
[review-cycle history](GladeIndependentCrdtAdmission-ReviewCycle.md) remain
unchanged historical records. This is a new redesign object; it does not reset
that object's three architectural roots or remediation cap.

Inspection tuple: Glade `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`64666e8b1caadde8922b9d42163afbab90655c65`. The lane owner MUST settle the
edited root/member tuple before fresh peer-blind Code/State review. These are
inspection pins, not the eventual review pins. Executions and chronological TDD
limitations are recorded in [new evidence](GladeIndependentCrdtStorageAttemptContract-Evidence.md).

## 1. Exact supersession and preserved authority

This DRAFT proposes only the following replacement of **unaccepted IC-1** clauses.
The historical document MUST NOT be edited to conceal its failed grammar.

| IC-1 location and exact subject | Replacement in this object | Retained obligation |
| --- | --- | --- |
| §2 table `Bytes`, `Instance`, `Receipt`, `BatchCommitted`; their storage-boundary ownership | Shared byte/identity/receipt values are moved and reexported; `BatchCommitted` becomes complete `Terminal`/`TerminalOutcome`. §§2–4 below. | Exact bytes and derived digest, original receipt identity and achieved class; no cryptographic authority from a struct. |
| §3 first paragraph: staged/uncertain commits, outstanding lookups and effect-ID restoration; CommitResolved/LookupResolved rows; `Commit`/`Lookup` effect list; callback matching paragraph | Semantic plans remain in Pure state; separately namespaced plan/attempt/invocation/owner continuations and retained terminal/conflict maps supplement them. Prepare/RecoverPlan/Begin/Lookup/Fence and their replies replace the old commit/lookup grammar. §§3–6. | Owned explicit state, checked identity allocation, full callback matching, retired invocation refusal, no cross-instance rights. |
| §3 ObservePolicy row and §6 paragraphs starting “Policy observation and **physical commit start**” and “Once the host has begun” | Trusted Records serializes its own observation with the protected begin barrier. Caller-carried `current` is an expectation, not freshness proof. §§4–5. | Local authorization before start, valid Started historical cut thereafter, no inferred cancellation. |
| §5 `complete_local` paragraph only insofar as bounded recovery refusal is remembered | Sticky `InstanceState.recovery_incomplete` and instance storage-integrity stop supplement existing pending/uncertain conditions. §7. | Full history, explicit local classification, no global completeness claim or false charges. |
| §6 `CommitPlan` binding/preview paragraph and internal batch paragraph | Namespaced `Binding` owns exact opaque bytes, kind, revision, charges, receipts and prestart windows; semantic continuations remain in the Pure plan and MUST be encoded into its batch. §§2–4. | Complete staged values, no receipt before the barrier, no host reconstruction from arbitrary future facts. |
| §6 CommitResult/LookupResult, request/reply identity, Unknown retirement, KnownAbsent/KnownNotCommitted release and direct/lookup installation paragraphs | Reserved/Started/Pending/terminal grammar, durable prepare deduplication, terminal NonCommit fencing and contradiction retention. §§3–6. | All four batch kinds, exact once-only installation/accounting and instance isolation; retained accepted/evidence/fork custody. |
| §6 final reservation/capacity paragraph only for added critical lifecycle footprint | Separately finite metadata reserve and no-GC attempt history. §§6–7. | Existing edit/recovery/evidence/pending quotas and checked arithmetic are unchanged. |
| §7 host description “Verify/Seal/Commit” and volatile reply plumbing; original27/ten-row obligations | The same consumers execute emitted lifecycle operations through the actual port fixture. §8. | All original assertions and loop cases, exact ten released-Taut rows and corpus bytes. |
| §9 gate/remaining-work references to old lifecycle types | This object's typed/package/allocation gate replaces that failed boundary; successful IC-2 and live IC-3/4 still follow. §9. | No successful kernel before typed dual GO, no physical receipt or activation claim. |

No accepted descriptor, operation, authorization, fork, eligibility, receipt,
origin, wire, SnapshotStore, legacy Store-seal or discovery contract is superseded.
In particular, bare signed rivals still preserve AD, qualified rivals retain the
common A prefix, fresh E1 after an E0 fork still permits AB, and exact old retries
keep their original receipts. No holder or consensus round trip is added.

## 2. Package boundary and actual producer/consumer

`glade/contracts/crdt-storage-attempt-api`, Cargo
`glade-crdt-storage-attempt-api`, is a proposed **Contract** library with two
meaningful required traits. `StorageAttemptHost::open` acquires a qualified
exclusive session. `StorageAttemptSession` requires `recover`, `prepare`,
`recover_plan`, `begin`, `inspect`, `request_fence` and `close`. None has a default
implementation. The production dependency is `sha2` for byte-derived digests;
`syn` is development-only source tooling. There is no database, runtime or
framework dependency. Classification and allowed edges are proposals requiring
this gate's review, not self-ratified exceptions.

The existing admission kernel stays **Pure**, depending on existing `glade-wire`
and the small new contract. It reexports genuinely shared `Bytes`, `Digest`,
`Instance`, `StorageClass`, `Receipt`, `Charges`, `CommitKind`, `TimeInterval`,
`Window` and `LocalPrecondition`. API owns no `Descriptor`, `Candidate`, `Facts`,
`Operation`, eligibility algorithm or canonical merge engine. This move avoids a
contract-to-core dependency/cycle and does not create a universal types package.
The core's former direct SHA dependency is replaced by that contract edge;
existing roles/allowlists are otherwise preserved.

The intended assembly flow is:

1. NodeAssembly supplies a validated kernel State, Records session, evidence
   provider and explicit policy/time observations in one dependency scope.
2. Pure `step` stages its immutable semantic plan and emits Prepare; Records
   receives its complete storage Binding, including opaque batch bytes.
3. Records owns preparation, issuance, start, worker continuations, atomic
   publication/fencing and retained outcomes through a qualified StorageAdapter.
4. Assembly reports only that producer's exact typed results to kernel events.
   The kernel checks matching continuations and installs the original semantic
   plan through one path after trusted terminal Committed.

A network decoder, client, peer or arbitrary DTO MUST NOT create trusted
PrepareReply/ResolveReply/Recovery, populate State, or attest physical finality.
The public in-process structs have no wire deserializer. Public constructors and
Rust typing do not authenticate them; trusted restoration/evidence checks belong
to the actual producer and assembly. A signed claim, correct file read, missing
path or fsync alone is not a terminal-negative proof.

The development producer is the bounded memory host under API `tests/support`.
Core tests and its real text trace include that same test-only implementation;
they do not import a production implementation library. Syntax scanning covers
that shared file in the API source suite; the architecture scanner does not
expand the core test's include macro. Both packages are selected together.
No production dependency on this fixture is created.

## 3. Identity, immutable binding and bounded lifetimes

`PlanKey = namespace + complete Instance + checked finite number` is immutable
kernel staging identity. `AttemptId = retained StoreIncarnation + Instance +
checked finite number` is host-issued storage identity. `InvocationId = retained
namespace + checked finite number` identifies one request/reply. `Owner = store
incarnation + execution generation` is the publication predicate's ownership.
None is a canonical operation origin, application revision or interchangeable ID.
A plan counter/namespace MUST NOT reset across close/reopen; exact restaging after
NonCommit uses a fresh plan/attempt while preserving canonical intent.

`Binding` carries PlanKey, private immutable Bytes/derived digest, expected
application revision, promised storage class, kind, exact charges, complete
original receipt list and optional `StartPrecondition`. The precondition contains
the exact local policy/time cut and **all** finite permit windows. The kernel MUST
encode descriptor/candidates/facts/seals/fork pairs/continuations into exact batch
bytes before emission; changing any stage MUST change those bytes. This checkpoint
does not implement the later encoder: synthetic fixture batches are labelled
trusted staged values, not encoding conformance or canonical signed data.

`AttemptBinding` permanently joins the allocated attempt to its whole Binding.
Repeated prepare/recover_plan for one exact PlanKey MUST recover the same attempt;
changed bytes or any bound field MUST refuse. Losing the prepare acknowledgement
MUST NOT allocate a second attempt. A temporarily absent mapping or queued late
prepare remains Pending. Another plan for X cannot overlap that uncertainty;
Y remains logically independent. Scalar or finite history exhaustion refuses
before allocation; counters never wrap.

For a new plan, Records MUST compare expected revision with its actual retained
application revision before exposing queued preparation or Reserved. An ahead or
stale expectation MUST return nonterminal Refused without creating an attempt,
terminal outcome or application revision. Exact existing-plan deduplication and
recovery take precedence over this new-plan check; refusal MUST preserve existing
unresolved ownership and reservations. A rejected invocation can still consume
its bounded issuance entry.

Preparation/attempt lifetime continues independently of invocation retirement.
All requests bind the owner and invocation namespace, operation and full original
binding; a resolve reply echoes that exact request and its terminal payload binds
it again. `State.storage_invocations` owns outstanding full requests; consuming one MUST
move it unchanged into `retired_storage_invocations`. Both maps share the finite
`Limits.invocation_history` budget. A retained high-water number alone does not
prove that a particular request was issued. Wrong or foreign callback identities
MUST NOT consume live work or release capacity. Validated restoration MUST retain attempt mappings,
plan/invocation namespaces, scalar floors and original complete values. Merely
clearing callback maps or selecting a random UUID does not retire old identities.

## 4. Outcome grammar and command scheduling

The ledger has Reserved, Started with retained start cut, or immutable terminal
Committed/NonCommit. `ResolveResult::Pending` is a nonterminal observation;
`Started` acknowledges the protected start cut without asserting admission.
Committed contains the original AttemptBinding, expected+1 application revision,
exact achieved class, entire original receipt list and retained start cut.
NonCommit contains the original AttemptBinding, unchanged application revision
and achieved terminal-record class; it has no application receipt.

`PrepareResult::Refused(Fault)` and `ResolveResult::Refused(Fault)` are rejected
invocations, **not terminal NonCommit**. In particular, a binding/ownership/
capacity/unavailable failure cannot establish that an earlier uncertain prepare
never registered or that owned work can never publish. Existing uncertainty and
reservation MUST survive unless a trusted terminal result resolves it. Terminal
results cannot be weakened to an error code or absent index entry.

Methods are deliberately synchronous **bounded command/observation operations**,
not synchronous disk-write promises. A qualified live implementation MUST avoid
blocking the assembly indefinitely: unavailable barriers/owned worker progress
produce Pending or Unavailable. It retains all outstanding work and ownership,
then later inspect/recover_plan/begin/fence observations resolve it. Executor,
waiter or caller-future cancellation does not cancel owned work. This avoids
introducing a runtime framework into the contract; real asynchronous producer
scheduling/cancellation/drain, sdax/Shaku scope lifetime and startup/shutdown
qualification remain later integration gates. The synchronous memory model does
not qualify live asynchronous cancellation.

`BeginRequest.current` is caller-carried expected policy/time data. The actual
Records producer MUST check its own latest authoritative observation from injected
policy/clock providers **under its per-instance serialization** and match it to
that expectation and the plan before the protected physical-start barrier.
Echoing the request proves nothing. A request queued before that barrier remains
Reserved; its old cut cannot later become current by replay. All points of the
trusted finite interval MUST fit every window; regression/unknown policy/time
refuses or stays Pending until qualified fencing. Historical retention carries
no new local-admit precondition.

The protected durable Started barrier and start check MUST be one serialized
transition with policy/time observation. A valid Started cut survives later
revocation, delayed replies and lookup recovery; repeated begin MUST preserve it
rather than reauthorize it. The simulation injects trusted observations explicitly
and compares them with the caller expectation; it does not claim a real clock,
cryptography or persistent ordering barrier.

## 5. Record-before-mutation, publication and fencing

Records MUST reserve worst-case finite critical metadata and retain the complete
Reserved binding/mapping before acknowledging preparation or permitting any
application mutation. Metadata generation is distinct from application revision.
Started MUST be retained before a worker can publish. Publication MUST atomically
predicate on exact attempt, active ledger status, owner generation and expected
application revision. Whole custody/proof/fork data, charges, original receipts
and terminal Committed are installed in that same atomic unit; no later best-effort
outcome/outbox write is permitted after visible acceptance.

Inspect accounts for queued and owned work. Temporary absence, lost acknowledgments,
corrupt/unreadable evidence, failed waiter and unknown registration remain Pending.
Only a retained tombstone/fence excluding **every** late prepare/start/publication
can establish NonCommit. After unknown preparation, recover_plan uses the original
PlanKey; fencing may wait indefinitely until its attempt can be resolved. This
profile does not promise a terminal answer by unknown-plan absence.

A fence and publication race against the same predicate and have exactly one
winner. If Committed wins, fence/lookup repeat that retained result. If NonCommit
wins, an old worker cannot start, publish, rebase or retry around it. Releasing an
OS lock or dropping a task does not prove submitted external I/O completed.
Qualified drain/termination or provider generation fencing MUST cover that work.
The original late-write attack is now an explicit all-four-kind Pending journey.

Direct and lookup terminal installation share exact validation and once-only
accounting. Retired duplicate assertions produce CallbackMismatch without a
second install/charge. A contradictory bound terminal assertion is distinct:
retain the original terminal identity and first contrary assertion in bounded
`terminal_results`/`terminal_conflicts`, mark the instance's integrity stop and
preserve reservations/evidence. Arrival order MUST NOT select a terminal truth.
Before considering a contrary terminal, the consumer MUST find its full original
issued request in the outstanding or consumed map and compare owner, namespace,
number, operation, complete attempt identity and every Binding field, then compare
the echoed terminal's complete AttemptBinding. A never-issued number, foreign
owner/namespace, changed operation, attempt or binding is CallbackMismatch with
unchanged state, even when its terminal DTO names a known AttemptId. A fully
matching consumed request can authenticate contradiction evidence; a matching
same-terminal duplicate remains CallbackMismatch without mutation. The retained
request is evidence of trusted local issuance, not peer-provided authority.
Other instances retain logical independence; physical shared-store outage may
still affect them.

## 6. Ownership, recovery, finite reserves and format gates

Open MUST establish exclusive qualified ownership, supported versioned root,
retained incarnation and issuance/invocation/owner floors before a usable session.
Recovery MUST validate complete custody/outcome coupling, revisions, reservations
and retained preparations. `Recovery` explicitly carries owner, invocation
namespace, limits, preparing bindings, revisions, attempts/phases and floors.
It additionally carries `invocation_count`: the actual retained request cardinality,
independent of `invocation_high_water`. Sparse scalar IDs are legal and consume
one entry per actually issued request. Restoration MUST preserve both values,
validate count against the declared finite history capacity and identity floor,
and retain enough consumption for all preparation/attempt history. A high-water
number MUST NOT be converted into a cardinality or used to require dense issuance.
Restored binding sizes, instance references and Started/terminal payloads MUST
satisfy the same limits and complete binding rules as the live session.
Complete AttemptIds MUST be unique across retained history. Each instance MUST
have at most one unresolved owner across queued preparations and Reserved/Started
attempts; multiple historical terminals remain legal. Unresolved bindings MUST
match the retained current revision, and queued/current instance unions MUST fit
the declared instance capacity. Recovery MUST reject inconsistent images rather
than reconstructing identities or deleting history.
Reserved/Started restore as Pending, not absent or auto-committed. Missing
ownership/floor/format/ledger evidence makes writable recovery unavailable.

Close reports Closed only after every worker is completed or fenced; Pending
close retains ownership. Dropping a session cannot authorize a new owner. New
ownership must exclude old generations/submitted I/O, not just obtain a released
lock. Rollback/clone/restored-copy qualification remains mandatory and cannot be
inferred from copying a trusted fixture Recovery. That fixture restore validates
bounded internal identities; it does not establish an external antirollback floor.
Before granting a new owner, open MUST validate the complete retained ledger,
bindings, issuance cardinality and critical reservations under proposed limits.
Incompatible reductions MUST refuse before changing ownership or scalar floors,
without dropping data or shrinking existing reservations. A compatible retry at
the same proposed generation MUST remain possible and export restorable recovery.

`Limits` finitely bounds instances, attempt history, invocation history, batch/
receipt/window/name sizes and critical byte reservation. State carries the explicit
session limits; zero/missing trusted configuration cannot enable production work.
Preparing MUST reserve its entire worst-case terminal/fence/high-water/recovery
footprint before edit capacity. Physical preallocation/quota failure remains a
qualification obligation; nominal memory counters do not guarantee a barrier.
Failed critical persistence keeps Pending/Unavailable, with admission stopped;
no volatile NonCommit may be promoted to a promised restart class.

Terminal records, mappings and issuance history have **no automatic GC**. Their
finite exhaustion stops new work, including repeated negative attempts. Kernel
maps/conflict evidence MUST stay within those declared retained-attempt and
invocation limits; no unbounded attacker callback history is retained. Consumed
full requests MUST survive validated restoration alongside terminal evidence;
there is no close/reopen clearing or history compaction in this profile. Capacity
is reserved before issuance, and exhaustion refuses new invocation allocation
without deleting outstanding or consumed evidence. Records owns its issuance
history; assembly MUST restore the kernel's authenticated request continuations
from trusted retained state before allowing callbacks. Ordinary
edit quotas cannot consume the separately reserved critical recovery metadata.
Existing candidate/evidence/fork/recovery budgets and full-history retention remain.

The later physical host SHOULD use independently serialized per-instance images
inside a separate versioned Records-owned root, excluding legacy writers. A hung
shared whole-snapshot CAS worker cannot be claimed independent Y progress. If a
shared CAS design is selected instead, its scheduler/fence/isolation proof requires
explicit review. Store-format bytes, framing, atomic replace/CAS behavior,
interrupted publication settlement, stable lock/root identity, mixed-version
exclusion and antirollback issuance mechanism are not selected or qualified here.
Existing SnapshotStore and legacy records.json do not already supply this lifecycle.

## 7. Sticky recovery incompleteness

`InstanceState.recovery_incomplete` records an observed recovery/evidence cut that
cannot be retained/classified within bounds. It forces complete_local=false,
without a sentinel Slot, false byte charge, eviction or loss of actual attempt
reservations. It is monotonic during this profile. Validated restoration MUST
preserve an observed true value, and Records' later physical representation MUST
retain it before permitting a restarted complete read.

There is **no clearing Event, host operation or reconstruction witness** in this
contract. Capacity restoration, an arriving ancestor, an empty frontier or
ResumeRecovery cannot clear it. A later clearing action requires separately
reviewed authoritative full-cut reconstruction and retained custody/outcome proof.
The representation/test does not claim that action exists. A storage-integrity
stop similarly makes the affected instance unavailable for new work; repair cannot
be inferred from a new callback or reordered terminal assertion.

## 8. Consumers, conformance and source/allocation gates

All 21 original Records and six recovery tests retain their assertions/loops.
The negative old KnownAbsent/KnownNotCommitted fixtures are now actual bound
terminal NonCommit; temporary absence has additional independent Pending tests.
The four-kind matrix still checks Committed/NonCommit/Pending, original receipts,
no synthetic Candidate/Security admission, qualified-fork custody, once charges,
lookup IDs/exhaustion/restoration, policy cuts, fresh-origin post-fork AB/old retries
and Y isolation. Thirteen new STA tests (eleven requirements plus callback-history
and caller-owned driver regressions), plus two issued/unissued lookup closures,
compile and fail behaviorally against
unchanged refusing step. Neither compile errors nor successful helper output is
presented as kernel RED/implementation evidence.

The bounded development host implements the actual traits and executes explicit
prepare/recover/begin/inspect/fence/recovery/close journeys. Its shared public-port
probe rejects six semantic mutants: absence-as-terminal, duplicate prepare,
lookup replay, rebase past fence, reminted receipt and decoupled visible custody/
outcome. Additional scheduler tests cover lost prepare acknowledgment/late queued
registration, pending worker, close ownership, both fence races, entire permit
window and Started cut after revocation. This is model conformance only.

The reusable `drive_with_port` consumer takes a caller-owned session retained
across continuations. `FixtureReplica` composes that session with explicit trusted
fixture State and a separately injected policy/time observation. Initialization
uses the exact fixture policy digest and interval. An independent trusted provider
change updates its observation; Begin caller data MUST NOT update that provider.
The valid core-cut port journey publishes the original staged receipt, and a
separately changed provider fences a stale Begin.

Every text call site now uses one retained session per logical replica: both
buffer/rival orders, qualified-fork A then fresh E1 AB/retry/refusal, isolation and
both ABC directions. Original multi-call rival/fork tests use the same lifetime.
Independent test branches use an explicit owned recovery seed rather than silently
pairing a nonempty kernel State with an empty host. The `drive` convenience wrapper
now constructs that declared fixture assembly and validates the supplied seed;
it is not a successful kernel fallback or a live restoration mechanism.

`OwnedFixtureSeed` is confined to the development provider. It carries shared
Recovery plus an immutable, finite genesis custody image for each fixture instance.
These images are deterministic debug snapshots of the explicit InstanceState,
including accepted records/original receipts, candidate/evidence/fork custody,
usage, policy and derived recovery state. They represent the existing revision0
fixture cut; they neither allocate a new attempt nor manufacture a new admission.
The provider retains them alongside all attempt/outcome bindings and critical
reservations, exports them in its development seed and validates aggregate bytes
against the finite critical capacity. The helper also checks exact issued/consumed
request maps, lookup indexes, counters, revisions/terminal coupling and original
receipt/accounting presence. Shared production Recovery gains **only cardinality**;
there is no production genesis/import API or exception to outcome coupling.
This test-owned continuation assumes the supplied execution ownership; it does
not acquire a real lock, exclude a clone, decode or encode a storage format, import
records or qualify antirollback. Such an image is never a peer-controlled DTO.

Every manually issued lookup uses `register_lookup`, which records the full exact
request in the authoritative outstanding map and the secondary lookup index,
checks owner/namespace/attempt/binding and finite shared history, and advances the
next issuance counter. The attempt fixture retains its consumed Prepare request
and issued Begin as well. Restoration moves invalidated live lookup requests into
consumed full history. Secondary lookup entries alone MUST NOT authorize callbacks;
terminal-field mutation tests begin from fully issued requests so rejection is
attributable to the intended mismatch.
The prior-cut delayed recovery fixture also registers Inspect40 through this
helper before advancing the issuance floor. Its original ExactRetry receipt
assertion remains, paired with the otherwise identical unissued callback's
unchanged state/reservation and CallbackMismatch obligation. This pair adds one
compiling kernel RED test to the existing42; it does not implement callback handling.

The real Rust text trace still follows public `step` results, using the same
actual port fixture for emitted operations. No private admission implementation
or successful fallback fills missing kernel reads. Exactly ten mandatory rows
remain AB, AB-buffer, AD and A in both orders, ABC and isolation. The unchanged
released-Taut consumer checks the unchanged digest-pinned canonical A/B/C corpus
payloads/refs/identities; its independent three-order corpus positive control is
merge evidence only. All ten kernel rows remain RED.

The external app-owned Gyld declaration/ledger adds a separately pinned lifecycle
overlay to the existing supplemental host. It preserves all **31 allocations and
124 obligations** of the frozen general semantic capture and adds three
responsibilities/eleven obligations: lifecycle ownership to Records with
StorageAdapter cooperation, overflow decision to existing Pure rules, and
conformance to the existing harness owner (**34/135** combined). Sources are
qualified by canonical document/revision and frozen text/hash, not raw generated
IDs or mutable sibling inputs. No Gyld engine, evaluator, weights or base policy
is expanded; allocation does not prove satisfaction.

The selected normal `crdt-admission`/`crdt-storage-attempt` commands include API
and affected Pure consumer. Existing other selectors remain exact; all12 actual
libraries retain ARCH002 refusal and the tooling copy adds a synthetic13th
negative fixture. Source suites inspect disabled cfg fields, associated/foreign
items and inline platform branches; API fixtures also require typed callback/
attempt/owner/terminal fields and required meaningful methods. JavaScript's
compound-body syntax guard remains unchanged. Broader existing-source migrations
are not claimed complete. Process-global allowlists and classifications are not
loosened. Timing distinguishes execution, warm build/runner and cold compilation;
Gyld's full affected runner retains its 2.0s budget and selection.

## 9. Stop boundary and next gates

This object MUST receive fresh dual Code/State GO at its settled tuple before
successful kernel behavior or production host implementation proceeds. Then IC-2
can implement the reviewed Pure transition/encoder and turn behavioral RED green;
component and aggregate reviews still follow. Real producer qualification remains
IC-3/4: genuine proof domains/certification/current clock policy, physical process
termination/reopen/fencing/barrier/quota/ownership/custody, automatic duplex app
sync, client/Glial durable intent, compatibility and seal exclusion.

No disk/restart/crypto, stronger storage class, production keys/quotas/time/loss
values, enrollment, migration, launcher, automatic duplex activation or push is
qualified here. Owner deployment inputs and new-store/existing-store transitions
remain decisions at their gates. [Remediation 1 evidence](GladeIndependentCrdtStorageAttemptContract-Remediation1-Evidence.md)
records the first merged correction and its shared-Recovery review requirement.
Fresh full reviews returned six nonarchitectural finding IDs representing four
defects. [Remediation 2 evidence](GladeIndependentCrdtStorageAttemptContract-Remediation2-Evidence.md)
records their merged validation/fixture correction, with shared API, architecture,
assembly seams and mutation boundaries unchanged. All six current finding IDs
remain OPEN until the current full reviewers independently re-verdict their
counterexamples and corrected range at the settled tuple. The retained
architectural root count remains one; this document does not self-close findings.
The implementation directive continues after
those gates; this tranche deliberately remains a refusing kernel checkpoint.
