# Runtime ownership, failure semantics and architecture acceptance

Status: proposed architecture contract, 2026-09-09. No tests described here are
newly implemented or passing merely because this document exists.

## One authority per state and transaction

| State | Owner and lifetime | Recovery / authority |
|---|---|---|
| Keys and trust-root configuration | Key adapter / node instance | Never replicated as ordinary records; explicit recovery/rotation |
| Canonical accepted records, retry identities, sequence/clock watermarks and durable pending handoff | Records profile host + its transactional store, scoped by profile and origin/resource as defined by the protocol | Persist the required atomic unit before the corresponding receipt; exact retries preserve identity/outcome |
| Discovery policy/claim state | One discovery profile state, hosted through Records or the retained existing driver | Rebuild projections from authenticated records; preserve protocol-required state/watermark/outbox atomicity |
| Catalogue/policy views | Immutable, revision-identified views supplied to Binding/Policy | Derived caches; no independent source of grants; invalidation/proof scope explicit |
| Attachment and provider epoch | Supplier host for routing; source for physical fencing | Durable epoch/takeover evidence as required; do not revive a live process from a stored `live` label |
| Correlations and waiters | Invocation, per session/call | Lost reply is Unknown after possible dispatch; durable command identity belongs to the source |
| Interest, delivery buffers and cursors | Delivery, per binding/session/profile | Bounded; explicit gap/reopen behavior; closing local delivery is not remote source deletion |
| Consumer assembly and client persistence | Glial, per canonical binding instance | One assembly shared by its mounted taps; honest local-store failures and shape recovery |
| Running tasks/resources/budgets | Component-owned scopes under Runtime supervision | Retain partial cleanup/progress; no detached work counted as successfully closed |

### Acceptance and publication

For the existing discovery draft, acceptance durably records exact signed operation,
intent identity, cursor, watermark and pending downstream handoff together. Downstream
derived state/outbox commits before handoff acknowledgement. Replay after a crash
deduplicates; acknowledgement cannot erase the original retry identity. The existing
node-adapter's combined durable snapshot is an integration constraint, not an
invitation to scatter the transaction over several independent writes.

For a source command, the supplier owns source commit and durable outcome/publication
intent where the backend supports it. Glade acknowledgement does not mean the
external source committed; source commit does not mean a consumer observed it.
Nontransactional sources need a named uncertainty/reconciliation policy. There is
no cross-database atomicity claim.

## Admission and proof ownership

The proposed order is bounded framing → canonical decode/profile validation →
origin/integrity evidence → scoped policy decision → domain acceptance or dispatch.
Some profiles require multiple stages; they MUST preserve their actual ordering
and exact signed-byte identity rather than fit an artificially universal parser.

Validated structural values MUST bind immutable bytes, profile/version and relevant
identity. Trusted constructors or an equivalent module boundary prevent ordinary
callers from accidentally manufacturing proof. In-process type safety is not a
sandbox against malicious native code. Synthetic test evidence is explicitly a
test provider, never a production verifier or serialized permission token.

Admission is reusable code at each trust boundary, not "checked once somewhere
upstream, therefore trusted everywhere." Local, forwarded, replay and disk paths
share validators, but fresh observations each validate. Internal consumers reuse
the same validated value without whole-object revalidation. Cross-language peers
share schema/corpus semantics, not a Rust trait object.

Time-sensitive permission carries exact subject/action/source scope and evidence
frontier. Queued operations are reevaluated at the relevant acceptance/dispatch
boundary. The local coordinator must serialize or generation-check policy change
against local use according to its profile. A local guard cannot prove absence of
an unseen remote revocation, nor atomically fence an external source; insufficient
freshness must produce the contract's explicit refusal/unknown/evidence-needed
outcome. The first live profile must specify that boundary before implementation.

## Dependency injection and execution

`glade-node` selects providers and injects **narrow, typed dependencies** according
to explicit binding roles and sharing scopes. This does not select an app-wide
context or require manually propagated dependency bags. The owner requested an
evaluation of automated injection: [Shaku/Dill results](DependencyInjectionEvaluation.md)
now distinguish compile-time wiring, runtime scopes, coordinated fakes and async
ownership. The owner subsequently selected Shaku for assembly; real async-port
integration and cleanup conformance remain open. The [graph refinement](InjectionGraphRefinement.md)
names six bindings and a partial cleanup order; exact constructors remain design work.

There is no global `get_service<T>()`, string-keyed bag or constructor fallback to
real I/O. A caller may supply a complete test composition. Contracts state whether
the dependency scope is node, account, binding, session or operation; equal trait
types alone do not establish that two instances are the same provider.

Use ordinary async calls within an ownership scope. A bounded queue is appropriate
when independent producers or a single-owner state machine require one. Queue
capacity/admission, cancellation and overload are part of that boundary's contract.
Queueing is not a substitute for backpressure; control priority cannot grant rights.

Mutable profile state has one serialized transition authority per declared scope;
independent scopes can progress concurrently. Prepare explicit effects, await host
results, then feed results back. Do not hold cross-component in-memory locks while
calling unknown code or awaiting external I/O. Pending storage is a visible state,
not a hidden race that lets a second writer reuse a sequence number. The exact
schedule is subject to deterministic interleaving tests.

Every externally running activity belongs to a scope before it can outlive its
caller. Shutdown stops admission, drains or cooperatively cancels children, then
releases the resources they use. Independent branches may clean up in parallel;
parents outlive dependents. Deadline expiry returns unfinished ownership. Dropping
a polled future means neither rollback nor completed cleanup. Blocking operations
use a bounded host executor and report completion/uncertainty through the same
ownership contract; no thread-per-call architecture is selected.

## Verification architecture

Each contract has one canonical, dev-only conformance suite with named fixtures.
Every fake and real implementation runs the applicable suite. Pure suites inject
time, evidence, faults and delivery schedules; they do not start the node, network,
database or executor. Adapter suites prove actual I/O and restart properties that
fakes cannot. A small wiring suite proves that every relevant path actually uses
the common service; that replaces copied deep semantic tests, not all integration
coverage. Source-specific effects still require source-specific tests.

Exact dependency allowlists and measured fast targets MUST accompany new crates.
An internal implementation edit runs its own tests; a public contract edit includes
affected implementations and consumers. Cross-language corpus changes include
their language consumers. The full stress/fuzz/system suite remains a separate
assurance tier, not the default local loop. Budgets need measurements before being
called achieved; the existing local gates do not yet cover these proposed crates.

## Architecture acceptance

| ID | Compiling contract / adversarial witness required before its implementation |
|---|---|
| AR-01 | One application declaration mounts twice with different fills and no scope alias; Glial assembly is shared only within the same binding instance. Grip has no node/transport import. |
| AR-02 | Same malformed/altered/scope-invalid input rejected via local, forwarded and disk paths; unchanged internal validated values do not trigger repeated structural validation; changed policy still reevaluates. |
| AR-03 | One injected provider scope observed on every caller path; a bypass to real I/O or a second uncoordinated fake fails a wiring test. |
| AR-04 | Existing discovery transition/persistence/outbox semantics survive the proposed profile-host bridge, including crash points and lost acknowledgement. No independent second authoritative fold appears. |
| AR-05 | Publish/renew/expiry and partial resolution remain correct with a fixed authorized locator; foreign namespace/referral or copied proof is rejected. |
| AR-06 | A cancelled or timed-out invocation after possible dispatch remains Unknown; source deduplication and stale-epoch physical fencing are exercised separately from correlation. |
| AR-07 | Two shape profiles share one applicable store implementation; incompatible store/profile pairs fail before activation. Distinct refresh/repair/retention behavior is preserved. |
| AR-08 | Parent resource remains owned while child cleanup is pending; partial shutdown retries without double-release. Concurrent independent cleanup can progress; no invisible detached work. |
| AR-09 | Interface/implementation dependency inversion and isolated fast targets fail closed for forbidden dependencies, unclassified crates or missing meaningful contracts. Shared conformance includes rejecting fake implementations. |
| AR-10 | Management uses ordinary grants/bindings; bounded diagnostics do not leak private metadata. New composition coexists with the old demo, and later binding cutover has explicit behavior checks. |

Compilation witnesses establish caller/type/dependency compatibility. Deterministic
schedules establish additional behavior. Real I/O, crash, security and integration
evidence establish their own narrower claims. None is a universal replacement for
the others. These witnesses must be reviewed adversarially, with failures addressed
before production implementations are assigned in parallel.

## Remaining architecture risks and decisions

| Risk / choice | Disposition before implementation |
|---|---|
| Profile-host abstraction might conceal materially different protocols | Prove AR-04 with the existing core; retain its cohesive driver if the bridge would weaken semantics. Do not create an oversized universal records abstraction by assumption. |
| New admission proof/context types may duplicate existing canonical types | Review existing wire/decl/discovery types; extend the owning contract only where a compiling consumer requires it. |
| Delivery draft is narrower than the catalogue | Pin first profiles and AR-07; do not claim all shapes fit `Subscriber` unchanged. |
| Policy-use ordering across independent stores/resources | Define a local freshness/serialization profile and external source fence; no universal distributed barrier is claimed. |
| Placement proof schema, dynamic authority and shard reconfiguration | Initial trusted mapping only; dynamic behavior remains behind the boundary until its protocol is separately reviewed. |
| Async trait generic/Send requirements exclude some browser/local runtimes | Keep wire/language contracts distinct; review adapters rather than leaking Tokio or requiring all clients to adopt Rust dispatch choices. |
| Too many crates or a new all-knowing runtime | Measure compilation/test cost and keep the explicit grouping rationale; do not split marker interfaces or move domain semantics into Runtime. |

This packet is a reviewable architecture candidate. It is not yet the owner's
stronger "compiling executable contracts" architecture milestone. Next is to
challenge this allocation and write those contracts, then start implementations
on the approved boundaries. No new review agent, engine or product code is part
of this documentation task.
