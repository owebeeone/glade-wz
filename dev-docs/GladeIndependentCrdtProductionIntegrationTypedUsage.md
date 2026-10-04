# IC-3A2 typed API consumer guide

Status: corrected checkpoint proceeding as owner accepted; independent Code, State and Surface re-review and finding closures are deferred to the wider IC3ABC review. The initial reports remain unchanged. These libraries define representations and replaceable provider contracts. The unchanged default `glade-node` assembly supplies refusing providers: `open_replica` returns `Fault::Unavailable`; default verification, sealing and observation cannot succeed. The B candidate adds explicitly configured genuine authentication and disk providers, described below and in the standalone persistence guide. There is no new binary flag, configuration file, browser activation or peer route in this checkpoint.

B candidate update: the default assembly still refuses; the explicit configured Rust factory and real disk lifecycle are described in the standalone [PersistenceUsage](GladeIndependentCrdtProductionIntegrationPersistenceUsage.md). Binary routing/peer exchange remains C-pending. The utilities `can_settle(&permit)`, `can_settle_received(&permit,&bytes)` and `can_abandon(&permit)` return nonmutating eligibility results; normal settlement additionally requires the exact issuer-recorded successful bounded receive bytes. `received_digest()` is read-only and returns None for absent/error/over-bound completion; it does not grant settlement authority. Check before I/O and retire only after durable Committed selection. Foreign/closed/not-drained/started-abandon capabilities refuse. The disk provider retains exact by-value unknown/refused continuations; see the B guide for its concrete retry, loss and combined close outcomes. This is a B candidate addition, not independent closure of the original typed findings.


## Packages and construction

Use `glade-crdt-admission-data` for owned inputs and records, `glade-crdt-evidence-api` for an injected evidence provider, `glade-crdt-recovery-api` for an injected recovery host/session, and `glade-crdt-recovery-codec` for structural encoding. Existing `glade-crdt-admission-core` type imports remain available as reexports. The data crate does not depend on the kernel algorithm.

Nothing reads environment variables, chooses keys, picks a policy, starts transport, or chooses a storage path on the caller's behalf. Supply the host implementation and evidence implementation explicitly. `ReplicaOpen` requires the exact instance, complete descriptor and identity, store owner, plan and invocation namespaces, finite `ReplicaLimits`, and expected authoritative floor digest. `ReplicaLimits` requires storage accounting limits, decoding limits, image bytes, retained guard history, inbox items/bytes and inventory items/bytes. There is no `Default` for these public configuration inputs. `DecodeLimits` requires byte, item, nesting and name-byte caps; nesting above 128 or zero is refused. Explicit key and floor-root configuration belongs to the B configured host, documented in PersistenceUsage; it is not a typed API default.

An `IngressAuthority::new(capacity)` is an implementation utility with an explicit finite retained-history capacity. A capacity of zero issues nothing. Closing a local utility entry does not reclaim its identifier or history capacity. Constructing this utility yourself does not grant access to a host session: that session must reject a foreign permit before invoking the receive callback.

## Open and recovery

Call `ReplicaRecoveryHost::open_replica(&mut host, request)` to obtain a session. An error supplies no usable session and no permission to consume history. `Fault::Unavailable` means that the operation cannot currently be provided; ownership, integrity and capacity failures retain their respective meaning. The unchanged default assembled provider returns `Unavailable`; the explicit B configured factory has its own documented success and refusal lifecycle.

On a successfully opened configured provider, call `session.load()`. `LoadResult::Unavailable` supplies no recovery assertion. Only `LoadResult::Validated(Box<ReplicaRecovery>)` is a trusted host assertion: it contains the complete owned image and its independently authoritative current floor. A successful structural decoder returns an untrusted Rust value, not this host assertion. Do not convert a decoded image to validated recovery yourself.

The host owns exclusive store/floor custody, trustworthy policy/time observation and recovery validation. The consumer must keep the supplied instance, owner, namespaces and request bindings exact. An old slot alone cannot justify reuse of identifiers, forgetting a receive guard or declaring the resource complete. The current floor may retain an interrupted intent whose newer slot is missing; load/reconstruction may remain unavailable or incomplete. Disk content alone cannot establish protection from arbitrary external rollback.

## A finite receive round

One guard covers one bounded inventory, page or transfer round. It is not a permanent idle listener. Opaque OS buffering may exist before the guard, but the application must not read, parse or otherwise consume resource history before `begin_ingress` has successfully returned its owned permit. An authenticated exact empty snapshot may settle its own round; timeout or a later empty snapshot cannot settle an earlier missing obligation.

1. Construct the exact `ReceiveGuard`, including instance/store guard ID, owner/lease, peer and channel, role, base image generation/digest, caps and deadline.
2. Call `session.begin_ingress(guard)`. Only success authorizes starting this round. The host must have durably registered the guard before success.
3. Pass that owned permit to `session.receive_ingress(permit, make)`. `make` constructs a future with output `Result<Bytes, Fault>`. The session checks issuer/session ownership before calling `make`. The returned `GuardedReceive<F>` owns both that future and the permit.
4. Await the returned continuation. `Pending` means the receive work remains owned and cannot be abandoned or closed as drained. On `Ready`, obtain `(permit, Result<Bytes, Fault>)`. All carrier work for this round, including spawned handles, must belong to this future and be joined before it completes; a callback must not launch detached history work.
5. Only a successful bounded result may settle normally. Classify those exact returned bytes as the observation and prepare its complete `Checkpoint`; a different signed snapshot cannot replace them. Call `settle_ingress(permit, observation, checkpoint)` after all work has drained. The host must atomically retain the observation, missing/rejected/loss obligations and selected checkpoint before retiring the guard.

`make` is considered started as soon as the authority invokes it, even if the constructed future has not yet been polled. Dropping or cancelling a started `GuardedReceive` does not prove that work drained. Its issuer retains outstanding status; the durable host must retain the corresponding guard/loss state across restart. A started failure must be settled with honest custody or retained as loss, never relabelled as a never-started cancellation.

This is the actual generic call shape, assuming a host implementation, guard and checked observation/checkpoint builder have already been supplied:

```rust
use glade_crdt_admission_data::{Bytes, Checkpoint, Fault, Observation, ReceiveGuard};
use glade_crdt_recovery_api::{PersistResult, ReplicaRecoverySession};
use std::future::Future;

async fn receive_round<S, F>(
    session: &mut S,
    guard: ReceiveGuard,
    make: impl FnOnce() -> F,
    classify: impl FnOnce(ReceiveGuard, Bytes) -> Result<(Observation, Checkpoint), Fault>,
) -> PersistResult
where
    S: ReplicaRecoverySession,
    F: Future<Output = Result<Bytes, Fault>>,
{
    let permit = match session.begin_ingress(guard) {
        Ok(permit) => permit,
        Err(fault) => return PersistResult::Refused(fault),
    };
    let receive = match session.receive_ingress(permit, make) {
        Ok(receive) => receive,
        Err(fault) => return PersistResult::Refused(fault),
    };
    let (permit, bytes) = receive.await;
    let bytes = match bytes {
        Ok(bytes) => bytes,
        // The host still owns the durable guard. This is not normal completion.
        Err(fault) => return PersistResult::Refused(fault),
    };
    let (observation, checkpoint) = match classify(permit.guard().clone(), bytes) {
        Ok(pair) => pair,
        // The consumed round remains outstanding until trusted recovery retains loss.
        Err(fault) => return PersistResult::Refused(fault),
    };
    session.settle_ingress(permit, observation, checkpoint)
}
```

This helper demonstrates ownership flow; it does not implement a physical host, authentication, parsing/classification or loss recovery. The actual current provider refuses before it invokes `make`.

## Never-started abandonment and uncertain persistence

If you have obtained a permit but have not passed it to `receive_ingress`, you may call `session.abandon_ingress(permit)`. The host must prove issuer ownership and that work never started, then durably record abandonment. Once `receive_ingress` invokes the factory, this operation is forbidden. A caller boolean, timeout, dropped future or process restart is not evidence of never-started work.

`checkpoint`, `record_observation`, `settle_ingress` and `abandon_ingress` return the same persistence result:

| Result | Consumer meaning |
| --- | --- |
| `Committed { generation, digest }` | The exact owned transition was durably selected for the stated local process-restart failure domain. This does not assert remote replication or power-loss durability. |
| `Unknown` | The caller lacks a final answer. The host retains the exact intent and ownership/issuance/guard obligations. Do not issue replacements, clear incomplete status, report success, or infer noncommit. |
| `Refused(fault)` | The requested transition was refused. An already registered/started guard still needs custody or loss recovery; refusal is not permission to erase it. |

Settlement and abandonment consume the permit. On an uncertain reply you do not own a replacement permit and cannot retry by fabricating one. Reopen/load/reconstruction must consult the host's retained exact intent and floor; the host resumes or exposes the retained transition. The consumer may retry an exact idempotent checkpoint only when the host's recovered binding permits it. A fresh guard or new checkpoint is not a substitute for resolving the old request.

## Failure, shutdown and reopen handoff

The concrete B `DiskSession::retain_pending_ingress_loss(&guard_id)` can retain conservative loss from its own already-drained refused settlement continuation, including malformed input or aggregate capacity refusal. Committed loss permits close but never completeness; failed publication retains the exact original continuation and Pending close. It cannot remint orphan/foreign permits, clear undrained work or bypass uncertain physical custody. See [PersistenceUsage](GladeIndependentCrdtProductionIntegrationPersistenceUsage.md) for protected observation-marker and supported physical reopen stops.

The generic API has no `dispose_pending`, guard reset or forced-close operation. Keep the session and every still-live continuation until a qualified host can retain their exact obligations. Dropping a Rust handle or ending a process is not a `Closed` result and cannot erase durable guard/intent/attempt custody. The current A2 assembly cannot open a session or perform qualified recovery; the explicit A2 stop below is the supported outcome, not a hidden runnable cleanup command.

| Observed outcome | Permitted next action and ownership |
| --- | --- |
| `open_replica` returns `Unavailable` | Stop this resource workflow. No session/permit exists and no history may be consumed. This is the actual A2 assembled result. |
| Receive returns `Err`, or classification fails after draining | Keep the exact guard/observation/operation and original request bindings in host custody. Do not call never-started abandon. The generic API has no caller-authorized loss/ownership reset. Hand the still-owned session and those bindings to the qualified host's recovery supervisor; at A2 that handoff is unavailable, so stop without asserting clean close or issuing a replacement. |
| A started continuation is cancelled/dropped | Treat drain as unproved. If it is still owned, drive its cooperating cancellation/join work to Ready, then settle with an honest observation/checkpoint. If its capability was lost, retain the original guard and use the same qualified recovery handoff; at A2 stop. Timeout alone is not settlement. |
| Persistence returns `Unknown` | The host retains the exact intent and consumed capability's obligations. Call `session.load()` to obtain the retained transition only on a qualified provider; `Unavailable` means stop and preserve custody. Retry an exact checkpoint only if the validated recovered binding authorizes it. Never issue a fresh guard/attempt to replace this uncertainty. |
| `ReplicaRecoverySession::close` returns `Pending` | Retain the session. Resolve the original owned work or exact attempts with the existing settle/inspect/fence operations only where their retained bindings permit them. Retry qualified close after that progress. If a lost capability or unavailable host prevents progress, stop and hand ownership to the qualified supervisor; no generic forced disposal is supported at A2. |
| Qualified close returns `Closed` | The session may be disposed. The supervisor may then call `ReplicaRecoveryHost::open_replica` with the trusted reopen owner and latest authoritative floor, preserving resource/store identity and namespaces. Call `load`; only `Validated` recovery supplies the retained original requests and outcomes. An unavailable load is a stop, not an empty new store. |

A qualified host supervisor must first join live carrier work or retain its conservative loss marker, keep all original immutable request/terminal bindings, and establish a safe release/reclaim of exclusive root custody before reopening. That physical handoff is a B qualification duty; it is not implemented by `IngressAuthority`, by dropping a pending session, or by this guide's sample helper. If the process is terminated instead, restart remains an unclean recovery case and must retain the old guards/floors; it is not the clean close→reopen sequence. No A2 command can turn these cases into success.

## Storage attempts, reads and close

A recovery session also implements `StorageAttemptSession`: `recover`, `prepare`, `recover_plan`, `begin`, `inspect` and `request_fence`. Preserve the complete owner, invocation, plan, batch, operation kind and attempt binding. `Pending` remains unknown; absence of a terminal is not noncommit. A fence requests durable exclusion of that exact attempt; it is not negative finality until the terminal outcome says so. A committed retry must retain the original receipt and storage promise. `BeginRequest.current` is a compatibility field and does not override the host's own injected policy/time observation.

Use `reconstruct_cut` with the exact instance, generation, observation watermark and manifest. A returned `ReplicaCut` combines the kernel cut with retained obligations, guards and loss. It can describe only that bounded local observation, not global convergence. `complete_local` must remain false for active/lost guards, missing history or the kernel's sticky incomplete history. Reconnection, an empty inventory and a newer peer head do not reset those facts.

Both `ReplicaRecoverySession` and its inherited `StorageAttemptSession` declare `close`. Use the explicitly qualified `ReplicaRecoverySession::close(&mut session)` for this combined lifecycle. A provider must give both trait methods the same combined ownership meaning: `CloseResult::Pending` while callbacks, attempts or guards remain unresolved; `Closed` only after their required custody is retained and no work remains live. Closing neither deletes terminal records nor resets issuance floors. The current refusing session reports `Closed` because it never opens a store or issues work.

## Representation and evidence

`encode`/`decode` use the physical recovery envelope. Named concrete roots retain their frozen names; container roots use a recursive identity tree that binds every element/key/value type, so a boxed/vector/optional plan identity cannot decode as a guard identity. Limits cover the outer syntax and the interpreted operation syntax together, including names, cumulative items and embedding depth; arbitrary payload/proof byte strings remain opaque until an authorized profile parses them. `encode_profile`/`decode_profile` use the separate remote canonical map grammar. `decode_signed_body` checks exact purpose, version, signature length and body shape; it does not verify the signature. Every decoded record remains untrusted until the injected evidence provider checks the entire descriptor/resource/origin/requester/session/policy binding.

`EvidencePort` requires `verify`, `seal` and `observe`; none has a success default. `observe` obtains current policy and a conservative time interval from the provider's own inputs. A transport-authenticated peer still needs resource permission. Current local admission must reject revoked, expired or time-uncertain permission; retained historical evidence is checked against its original admission contract rather than silently treated as fresh authority.

The corpus uses 21 named semantic inputs across 18 representation categories, with 18 malformed/shape controls. Its signatures are symbolic bytes, so it qualifies representation only. Run `sh glade/contracts/crdt-recovery-codec/check-vectors.sh` from the workzone for candidate representation checks. `check-vectors.sh --pre-remote` additionally requires committed implementation content pins and committed accepted IC3-B1 genuine-signature qualification evidence. It currently refuses. This tranche therefore supplies no permission to enable automatic remote exchange.

For an implementation using `IngressAuthority`, eligibility is nonmutating. After committed durable selection call `settle(&permit) -> Result<(),Fault>` for drained settlement/loss, or `abandon(&permit) -> Result<(),Fault>` for never-started abandonment. Ok retires the utility entry; Err must be handled, with an unexpected post-selection retirement failure treated as Unknown. Refused/Unknown publication preserves ownership and does not call retirement. Concrete DiskSession consumers use the session operations and never retire its private authority manually. The persistence consumer guide covers oversized normal refusal followed by host-owned compact permanent loss.
