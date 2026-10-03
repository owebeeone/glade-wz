# Independent CRDT storage-attempt lifecycle

Date: 2026-10-04. Status: **DRAFT, new owner-authorized redesign object; no
contract, successful kernel, physical provider or activation is accepted.** The
owner chose redesign after the [IC-1 architectural stop](GladeIndependentCrdtAdmissionContract-Escalation.md).
The failed object's reports, three-root accounting and remediation cap remain
intact. This document is a new design for its storage-attempt boundary, not a
second correction patch to that object or acceptance of the known defect.

Inspected baseline: root `1fd0fb025606ee61edb33c1e385f558280aa6a62`; Glade
`6a0cc5a78da38a023615f6adc5fc354bba4af0b4`; Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld
`64666e8b1caadde8922b9d42163afbab90655c65`. The lane owner MUST settle the
new document and record its review tuple before dispatching two fresh,
peer-blind Consistency/Safety reviewers. This baseline is inspection provenance,
not the future review pin.

## 1. Retained semantics and the changed responsibility

The [accepted CRDT semantic design](GladeIndependentCrdtAdmissionDesign.md),
identity/profile, historical authorization, qualified-fork quarantine, fresh
origins, immutable operation bytes, exact retries, full-history quotas and
honest local receipts remain controlling. No holder or consensus round trip is
added. The [IC-1 internal contract](GladeIndependentCrdtAdmissionContract.md)
remains unaccepted. Its 27 compiling behavioral RED tests and ten released-Taut
rows remain obligations, not executed successful behavior. Its prior evidence
MUST NOT be rewritten to claim this design was tested.

Fresh [Safety P2-1](GladeIndependentCrdtAdmissionContract-ReviewSafety-2.md)
shows that an absent outcome entry can precede a late physical commit. Retiring
the attempt at that observation strands custody, accounting and receipt recovery.
The [root classification](GladeIndependentCrdtAdmissionContract-RootClassification-1.md)
also establishes why callback identity and whole-batch retention recovery are
architectural concerns. They are addressed here as one lifecycle rather than
three independently plausible callback rules.

**Recommendation:** Records owns a narrow storage-attempt host, with durable
attempt/outcome state in the same atomic unit as the batch's visible custody.
The host serializes preparation, physical start, finalization, lookup, fencing
and recovery. The pure kernel stages immutable intent and consumes this trusted
host's explicit lifecycle results. It neither performs I/O nor authenticates
storage finality from an arbitrary DTO.

The host MUST own the policy/clock ordering already required at physical start.
A peer, client, network decoder or caller-supplied boolean MUST NOT create a
trusted terminal storage result. The later typed port MUST identify the actual
producer and consuming assembly; tests inject a contract-faithful simulation of
that producer. A Rust struct, signature over a claim, path existence or fsync
alone is not proof that old storage work cannot commit.

## 2. Identity and separate lifetimes

| Identity | Lifetime and mandatory binding |
| --- | --- |
| Plan key | Immutable kernel staging identity in a validated kernel namespace; binds instance, exact batch bytes/digest, kind, expected application revision, promised storage class, charges, complete continuations/receipts and any start precondition. |
| Storage attempt | Store incarnation + instance identity + checked finite scalar attempt number; permanently binds exactly one plan key and its complete binding. The host allocates it, retains it and never rebinds or reuses it. |
| Callback invocation | Independent checked finite identity for one prepare, start, lookup or fence reply. It includes its invocation namespace and binds its operation plus plan/attempt binding. It can be retired while the storage attempt remains Pending. |
| Execution generation | Host/provider ownership token used by the atomic mutation predicate. New ownership must prevent old generations from publishing. It is not operation origin, application revision or callback identity. |

The store incarnation MUST be an explicitly issued, retained namespace under the
qualified store's ownership protocol. Randomness or a UUID collision assumption
MUST NOT substitute for retained issuance and antirollback. Reopening the same
store retains its incarnation and high-water marks. A new name or restored backup
MUST NOT silently reset them. Exhaustion refuses before issuing an identity;
it never wraps. Per-instance counters are sufficient when their namespace
contains the complete instance and store incarnation. Cross-instance identities
cannot authorize or resolve one another.

An exact repeated prepare for the same plan key/binding MUST recover the same
attempt. Reuse of a plan key with different bytes or fields MUST refuse. This
deduplication mapping is durable with the attempt, including when the prepare
acknowledgement is lost. Recovery MUST support lookup by the original plan key
when the caller never learned the allocated attempt number. It MUST NOT allocate
a second attempt merely because that acknowledgement did not arrive.

Invocation counters/high-water marks and namespaces MUST be preserved by trusted
restoration, or replaced through a proven retired namespace that excludes every
old callback. Clearing a callback map or restarting a process does not provide
that proof. A retired invocation's reply cannot consume a newer invocation,
change an attempt, release capacity or install a batch.

## 3. Monotonic state and finality

Before durable registration, a received prepare can have **Pending preparation**:
the host may be reserving/persisting metadata, and the caller cannot infer whether
registration occurred. Once registered, the durable attempt has these states:

| State | Meaning and legal successors |
| --- | --- |
| Reserved | Exact attempt/binding, recovery information and capacity reservation retained; no application batch is visible. May become Started or terminal NonCommit. |
| Started | Physical-start precondition was checked under serialized host ownership and its exact cut is retained. Work may still be pending. May become terminal Committed or terminal NonCommit if noncommit is established. |
| Committed | Whole original batch and terminal outcome installed atomically at application revision `expected + 1` and the promised storage class. No successor or contrary result. |
| NonCommit | Retained terminal tombstone/fence proves this exact attempt cannot publish now or later. Application revision/custody were not changed by it. No successor or contrary result. |

All nonterminal observations are **Pending**, including temporarily missing
entries, timeout, unreadable/corrupt evidence, transport loss, unknown prepare,
unknown commit and incomplete fencing. Pending is an observation, not a state
that rewinds Started to Reserved. A well-formed registered attempt has at most
one terminal result. It eventually has exactly one only if the provider can
complete or prove noncommit; no termination guarantee is made for unavailable
or damaged providers.

A terminal result MUST bind the complete original attempt/plan/batch/instance,
expected and achieved application revision, storage class and exact outcome.
Committed includes the complete original genuine admission receipt list, which
is empty for candidate/security retention. NonCommit has no admission receipt.
Both MUST be retained and recoverable at the declared class. Terminal answers
are repeatable observations of the same retained result, not new finalizations.

Missing index entries MUST NOT mean NonCommit. Direct write errors MUST NOT mean
NonCommit unless the same lifecycle establishes irrevocable noncommit. A provider
with no terminal-negative mechanism MUST answer Pending. Cancellation of a
future, waiter, executor task or RPC MUST NOT mean cancellation of owned storage
work. Cancellation is a request to resolve/fence, with a possibly indefinite
Pending answer; this design makes no generic cancel-safe write promise.

## 4. Preparation, start and atomic publication

There MUST NOT be overlapping unresolved storage attempts for one instance.
This includes pending preparations whose acknowledgements are lost. The host
MUST route all such work through one per-instance lifecycle owner; a second
different plan cannot slip through before the first prepare becomes durable.
Exact retries query the original attempt. Other instances remain independent.

1. The kernel retains an immutable staged plan and its reservation before
   handing it to the host. No successful receipt is exposed.
2. The host validates the binding and reserves finite critical space for the
   complete prepare, terminal outcome/tombstone, identity high-water, ownership
   fence and restart recovery. It durably records Reserved, complete staged
   values and the plan-to-attempt mapping **before application mutation or
   exposing an attempt as durably prepared**. Lost/uncertain recording remains
   Pending; a retry resolves that same plan key.
3. Under the same instance owner, the host checks the expected application
   revision and, for new intent, current supplied policy/time against the exact
   LocalPrecondition and entire finite permit window. The check and physical
   start MUST be serialized with policy/clock observation. A failure leads to
   NonCommit only after the attempt is durably terminal/fenced. No queued worker
   may later begin using the stale cut. Historical retention keeps its recorded
   historical qualification; it does not acquire today's local-admit check.
4. The host retains the valid start cut and enters Started before permitting
   owned mutation. Publication uses a provider-enforced atomic predicate binding
   attempt identity, active status, execution generation and expected revision.
   Beginning the protected Started barrier is the physical-start boundary; a
   queued request remains Reserved. Policy/time checking cannot be separated
   from that begin by an unfenced queue or another policy observation. A delayed
   barrier answer does not move the recorded start cut forward.
5. One atomic publication installs all custody/proof/fork/outcome data, charges,
   original receipts and terminal Committed. The batch's application revision
   changes exactly once. No separately committed best-effort outcome/outbox is
   allowed after a visible accepted tail. Synchronization handoff remains
   derivable from retained accepted history/enrollment, as the semantic design
   requires.
6. Only a trusted terminal result permits the kernel's shared installation path
   to expose success, release the one reservation and account charges once.
   An unknown reply keeps the exact plan, attempt and reservation. A later
   policy observation cannot cancel a commit begun under the valid retained cut.

The storage provider's internal snapshot/CAS generation MUST be distinct from
the application revision. Preparing, marking Started or fencing NonCommit may
advance internal metadata generations without advancing an admission head or
application revision. Application revision overflow MUST be detected before
start. Any failed publication predicate MUST NOT be rebased by an old worker
onto a new attempt/current generation; it resolves its original attempt instead.

## 5. Lookup, fencing and the late-write counterexample

Lookup is ordered by the lifecycle owner, and observes either a retained terminal
result or Pending. It MUST account for queued preparation and mutation work,
not just inspect the current accepted/outcome index. A same-process missing entry
before an owned write starts is Pending even when its disk read was correct.

NonCommit is permitted only when the host/provider establishes one of these
qualified routes and retains its terminal tombstone:

- The work never started, all queues/continuations capable of starting it are
  drained or fenced, and a durable terminal record prevents late prepare/start.
- Work started but terminated without atomic publication, every outstanding
  publication is completed or fenced, and a durable terminal record prevents
  that work or restart recovery from publishing afterward.
- A provider atomically installs a terminal fence under the same publication
  predicate that protects the entire batch. The commit/fence race has exactly
  one winner; a losing old worker cannot retry around the fence.

An OS lock or the termination of a waiting task alone does not establish these
routes. Releasing an owner-process lock and completion of an already submitted
I/O are distinct events. A provider MUST prove that its fencing predicate covers
the latter, or join/drain that work before answering terminal NonCommit. External
or detached I/O that can publish after the lock is released requires provider
fencing; process ownership is insufficient.

For a lost prepare, fencing MUST cover late registration too. If no attempt
entry is visible yet, the host cannot answer terminal by absence: it must resolve
the original plan key, or durably register its attempt and NonCommit tombstone
while fencing all late preparation for that key. Unknown registration cannot be
discarded to make room for a replacement.

Required original counterexample: P is Started, its waiter loses the reply, and
L observes no terminal outcome before the batch write. L MUST answer Pending and
preserve P. If P later commits, a fresh invocation recovers its exact Committed
batch. If a fence wins, the retained result is NonCommit and P can never commit.
The same rule applies to direct negative callbacks and all retention kinds.

A delayed commit callback and a lookup can both report the same terminal result.
The kernel installs that result once; a subsequent retired invocation is a
callback mismatch/no mutation. A committed lookup cannot coexist with a valid
NonCommit callback. Contradictory terminal assertions indicate host/storage
integrity failure: preserve evidence/reservations, stop new instance work and
require repair; do not choose arrival order or invent a replacement operation.
The redesigned kernel MUST retain enough terminal identity to detect this
contradiction separately from an innocuous duplicate after reservation release.

Only terminal NonCommit releases the unresolved attempt for rescheduling.
Rescheduling uses the original canonical operation/intent, a **fresh storage
attempt identity and fresh plan key**, and applicable fresh local policy/time
verification/sealing. Repeating the old plan key continues to recover its old
terminal NonCommit; it is not a reschedule operation.
It cannot reuse a terminal attempt or silently create a replacement canonical
origin/sequence/payload. After Committed, exact retries return the original
receipt/admission and never reauthorize/remint. While Pending, no replacement
canonical intent is permitted.

## 6. Restart, old owners, antirollback and finite capacity

A provider MUST acquire exclusive store ownership before recovery and MUST
prevent legacy/other writers from touching this new versioned root. All mutating
workers MUST remain inside that ownership/fencing regime. Restart MUST not
begin new instance work until it has validated the store incarnation, format,
identity/ownership high-water marks, complete ledger, retained reservations,
application revisions and coupled custody/outcomes.

Old executions MUST be excluded by qualified drain/termination semantics or
atomic generation fences. Obtaining a newly released OS lock is not a blanket
proof that external submitted work has disappeared. In the recommended local
file provider, qualification MUST establish that no asynchronous/detached
publication can outlive its protected CAS operation and that reopen settles
the interrupted whole-file replacement before decisions. Other providers must
demonstrate their own fence. A lock file's mere presence is not an ownership
claim or permission to delete a lock.

Restored Reserved/Started entries stay Pending until recovered Committed or
qualified NonCommit. Restart MUST NOT auto-commit stale pre-start local intent
without the current start check, or retroactively reauthorize a valid Started
cut. Lost callbacks may be retired, but stored attempt identities, terminal
records, receipts and invocation high-water remain. No process restart changes
an application origin or grants a same-origin sequence reset.

Rollback of a store image can erase a newer terminal tombstone/high-water and
allow resurrection. LocalProcessRestart qualification therefore covers normal
same-store process termination/reopen; it does not imply safety of arbitrary
backup rollback, disk cloning or concurrent restored copies. If the trusted
issuance/ownership floor cannot be established, reopening for writes MUST refuse.
Repair requires recovering the authoritative retained ledger/floors or a
separately reviewed new-store/new-instance transition; a fresh UUID is not repair.

Critical lifecycle metadata MUST have a separately reserved finite budget.
Preparing an attempt MUST reserve its worst-case terminal/recovery footprint
before consuming edit capacity. Candidate, evidence and qualified-fork work use
their existing reserved budgets; they do not spend edit quota. Physical quota and
preallocation behavior remain adapter obligations: a nominal byte counter cannot
guarantee an emergency write succeeds. If the critical barrier cannot complete,
the honest result is Pending/Unavailable with admission stopped, not memory-only
NonCommit. Repair can restore capacity/access and retry exact lifecycle recovery;
when ownership or state is lost, the instance remains unavailable.

Terminal outcome/tombstone and issuance history is retained for this incarnation.
This first profile has **no automatic history deletion or terminal-record GC**.
Exhausting its finite metadata/attempt capacity stops new attempts, including
repeated negative attempts. An eventual collection/checkpoint scheme would need
its own reviewed proof that old work, callbacks and restored images cannot
resurrect retired identities. Capacity pressure cannot weaken that requirement.

## 7. Exact installation for every batch kind

All four kinds share prepare/start/finality, atomic publication, recovery and
once-only accounting. None uses an invented operation or placeholder receipt.

| Kind | Whole Committed result and kernel installation |
| --- | --- |
| AcceptedBatch | Complete original staged admissions, facts/seals, exact candidates/evidence, origins and genuine receipts. Direct local success is AcceptedLocal; recovered retry returns the same ExactRetry identity. |
| Candidate | Exact unresolved candidates and missing closure, empty admission list; custody is BatchRetained, not application admission/head advancement. |
| SecurityEvidence | Exact attributable nonqualifying records, empty admission list; no accepted record or conviction is invented. |
| QualifiedFork | Exact independently qualified unordered rival pairs and any complete historical admissions required for new custody; genuine original historical receipts only. Derive quarantine/common prefix while retaining all prior custody; report BatchRetained. |

NonCommit changes none of the batch's application custody, charges, receipts or
derived projection. Its lifecycle tombstone is retained control state. Pending
keeps reservations once, leaves committed custody unchanged and marks the local
cut incomplete. Recovery never re-seals a committed admission or substitutes
today's policy for its historical cut. Full binding and achieved storage class
must match for both direct and lookup installation; duplicates cannot charge twice.

## 8. Storage reuse decision

| Choice | Evidence and limitation | Disposition |
| --- | --- | --- |
| Use SnapshotStore unchanged as the lifecycle | Its contract has linearized revision CAS, atomic acknowledged restart retention and OutcomeUnknown. It explicitly lacks request deduplication; a load cannot attribute an unknown write if other writers intervened. No attempt/start/fence/terminal-negative operation exists. | Insufficient alone. Preserve this port's existing guarantees and consumers. |
| Use the existing RecordsFile/records.json unchanged | It locks CAS, writes/syncs a temporary file, renames, syncs the directory and exposes snapshot revision. Its current synchronous future completes in its first poll; this removes one present waiter-cancellation path but not missing persistent attempt identity or interrupted-process recovery. Node snapshot parsing/old binaries can drop unknown metadata. | Insufficient; do not place independent attempts in legacy records.json or claim the existing tests qualify this lifecycle. |
| Add a narrow Records-owned attempt host using qualified whole-snapshot CAS primitives in a separate versioned root | Store complete per-instance application custody plus attempt ledger/reservations/outcomes in one envelope; metadata CAS generations differ from application revision. Atomic compare/publish/fence predicates and retained tombstones provide finality. Existing file/lock/replace primitives can be reused after qualification. | **Recommended bounded path.** New host responsibility/meaningful port, not necessarily a new crate or DB. No snapshot contract relaxation. |
| Adopt a new journal/database or consensus system now | Could support the same lifecycle but brings independent representation, dependency and qualification choices without removing these obligations. | Not required by this design. Select only on demonstrated adapter need; no mandatory consensus or new database. |

The recommended host's operations are prepare/recover-by-plan, begin/resolve,
inspect and request-fence, plus exclusive open/recovery. Their future typed
contract MUST express Pending versus terminal outcomes and complete bindings;
the names here do not freeze Rust signatures. Its implementation role runs shared
behavioral conformance; the admission kernel retains its Pure role and injected
event/effect boundary. Package allocation follows Records/StorageAdapter, not a
universal common package or a marker trait. Classification/dependency selection
requires its own reviewed contract tranche under the existing library policy.

A snapshot-backed implementation can persist Reserved/Started and then CAS one
new envelope containing the full batch and Committed. A fence instead CASes
NonCommit against the same active attempt/generation. Commit and fence cannot
both succeed. Late prepare/start/publish must check retained status and must not
rebase around NonCommit. Internal CAS revisions and a qualified exclusive owner
keep later work from overwriting terminal records. This is a proposed mechanism,
not proof that today's RecordsFile provides the full predicate or schema.

VolatileTest simulates ordering and finality in bounded deterministic memory. It
proves neither restart nor physical fencing. LocalProcessRestart requires real
terminated-process/reopen qualification of this complete new host, its ownership
and every coupled barrier. It promises neither OS/power/machine-loss safety nor
an independent replica. File/directory fsync does not by itself prove an atomic
attempt/outcome ledger, rollback protection or terminal absence. No stronger
receipt class is selected here.

## 9. Recovery-overflow completeness is a separate proposal

Ordinary pending slots and unresolved lifecycle reservations make complete_local
false. Existing IC-1 state cannot truthfully retain observation of recovery
overflow when even its pending-slot budget is exhausted. Saturating a used-byte
counter is unsuitable: it confuses actual committed/reserved charges with an
unretained observation, and full quota alone does not make a fully classified
cut incomplete. An invented sentinel Slot would corrupt the namespace.

Propose one bounded sticky **recovery_incomplete** flag per instance, distinct
from storage-attempt status/accounting. Set it when observed recovery/evidence
cannot be retained or classified within bounds. It forces complete_local=false
and survives validated restoration, without evicting accepted history or spending
unbounded pending capacity. Exact attempt Pending still retains its actual plan;
this flag cannot replace lifecycle reservations or resolve uncertain writes.

No current event automatically clears the flag. Clearing requires a trusted
full reconstruction covering the previously observed cut and its exact retained
custody/outcomes, under restored capacity and existing full-history rules. If
that cut cannot be reconstructed, the flag remains set; capacity becoming
available, one arriving ancestor, an empty frontier or ResumeRecovery alone is
insufficient. The later contract MUST specify the reconstruction witness and
authoritative host action before exposing a clearing operation. This design
does not invent a truncated bootstrap/checkpoint or claim that action exists.
The field/clearing semantics require their own explicit contract review; no
public field or completeness reinterpretation has been made.

## 10. Requirement and future-test map

These are new **STA** requirements and planned tests, not executed evidence.
All tests MUST begin RED before revised types/implementation. Deterministic host
consumers exercise actual emitted kernel/port operations; a private successful
model cannot replace the admission or released-text consumers.

| ID | Requirement and bounded deterministic RED journey | Physical qualification still required |
| --- | --- | --- |
| STA-001 | Immutable namespaced attempt/plan binding; changed bytes/instance/revision/class refuse, scalar exhaustion preserves state, restored high-water rejects old callback. | Persist/reopen issuance and floors; cloned/rolled-back stores refuse writable ownership. |
| STA-002 | Lost prepare ack, absent mapping and queued late prepare remain Pending; exact plan retry recovers one attempt; no overlapping preparation/attempt for X, Y progresses. | Kill at allocation/prepare persistence/reply cuts; no two owners or stranded unindexed mutation. |
| STA-003 | Pending original write + absent outcome lookup cannot release reservation; later commit recovers original batch/receipt once. Repeat all four kinds. | Delay actual worker/I/O across lookup, termination/reopen and result loss. |
| STA-004 | Fence wins before late start/commit: terminal NonCommit tombstone rejects every old-generation publication and delayed prepare; original intent can restage only under fresh attempt. | Race real cancellation/drain/fence with publication; test old submitted I/O separately from owner lifetime. |
| STA-005 | Commit wins fence race: every trusted terminal observation is Committed; delayed direct callback versus lookup installs once; contradictory terminal assertions stop X and retain evidence. | Atomic outcome/custody at rename/journal/barrier cuts, exact recovered terminal result. |
| STA-006 | Current policy/time changes before start produce fenced NonCommit; valid start then revocation/lost ack recovers original historical cut without reauthorization. | Serialize real clock/policy observation and physical start; uncertainty/rollback and cancellation cuts. |
| STA-007 | Candidate/security empty receipts and QualifiedFork genuine historical admissions recover Committed/NonCommit/Pending exactly; no synthetic admission, no double charge. | Whole-batch retention/fork evidence and outcomes survive promised process restart. |
| STA-008 | Critical reserve full/failure cannot yield receipt or volatile terminal noncommit; unresolved reservations count once, ordinary edit quota cannot consume recovery metadata. | Actual quota/preallocation failure and restart recovery without overwriting retained outcomes. |
| STA-009 | Restored Reserved/Started cannot imply absence, reissue identity or bypass start/fence; unreadable ledger stays unavailable; repeated invocation IDs reject. | Exclusive open, old-host exclusion, interrupted state framing, floors, format and lock lifecycle. |
| STA-010 | Recovery overflow makes reads sticky incomplete without sentinel/false charges; partial repair cannot clear; only defined full reconstruction may clear. | Persist/reconstruct observation status; no false complete cut after resource loss. |
| STA-011 | Preserve all original 27 behavioral obligations and exact ten actual released-Taut rows, including post-fork fresh-origin AB and old exact retries. | Existing IC-3/4 crypto, physical receipts, duplex app sync, clients and compatibility gates. |

Meaningful mutants MUST be rejected: temporary absence as terminal NonCommit;
drop-waiter as cancellation; lookup IDs reused; old worker rebases past tombstone;
prepare duplicate creates a second attempt; outcome written after visible batch;
terminal metadata charged twice; policy checked after start; receipt reminted on
lookup; blanket fresh-origin refusal on any quarantine; incomplete flag cleared
by quota restoration alone. Real adapter tests additionally reject publication
that outlives its purported fence and reopen that guesses absent from lost ledger.

## 11. Supersession, gates and current evidence

After **this design's** fresh dual acceptance, a new typed-contract tranche MUST
define attempt ownership, prepare/start/inspect/fence operations, callback/state
representation, terminal-result retention and the bounded overflow proposal.
It MUST add compiling lifecycle RED consumers before implementation. That gate
may supersede only the unaccepted IC-1 §6 commit/lookup identity/finality grammar
and corresponding types/continuations/state fields, plus narrowly linked event
and completeness descriptions. It MUST identify every exact superseded clause,
retain the original reports/cap and preserve the accepted CRDT semantics. There
is no blanket replacement of the semantic design, Gyld allocation or other
library ports. Renaming KnownAbsent alone is not this redesign.

No successful kernel/type/test implementation is authorized by this document.
New typed-port review precedes IC-2 implementation; Code/State component review
and real-host/aggregate review still follow. IC-3/4 canonical proof domains,
real authentication/clock/custody, automatic duplex synchronization, client/Glial
intent lifecycle, compatibility/seal exclusion and truthful physical receipt
qualification remain mandatory. Production keys, quotas/time/loss values,
existing-store transition and activation remain owner decisions at their gates.
No push, live storage mutation, enrollment, launcher or activation occurs here.

Inspection evidence: workspace/member rules, BuildEntry, library/package policy,
accepted semantic design, failed IC-1 contract/remediation evidence and completed
escalation/Safety/root-classification reports; SnapshotStore's declared
semantics; `glade/node/src/records_file.rs` locked CAS/replace and port bridge;
existing admission types/consumers/actual-Taut trace. Root/member HEADs above
were read directly. No tests/builds or physical fault experiments were run for
this draft. Only this new document is authored by the redesign drafter; the lane
owner owns decision logging, settled tuple, review prompts/reports and commits.
