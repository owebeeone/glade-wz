# Consuming the explicit independent CRDT disk adapter

Status: B candidate API/consumer guide for independent Surface review. Default `NodeAssembly::independent_adapters()` still returns refusing adapters. The explicit configured factory below supplies real authentication and disk providers in an owned Rust scope; B does not add a CLI command, binary ingress route, configuration-file format or live deployment default. Binary routing/automatic peer exchange is C work.

## Construct one owned scope

Supply all three values explicitly:

| Input | Required meaning |
| --- | --- |
| `EvidenceConfig` | Signed descriptor/creation identity, trusted root key, node signing seed, canonical decode limits, genuine origin certificates, finite session capacity |
| `PolicyClock` implementation | Provider-owned signed current policy and trusted bounded time interval/floor; uncertain/invalid observations refuse instead of inventing an instant |
| `DiskRootConfig` | Separate absolute data/trusted-floor paths, nonzero registration, explicit `provision` choice |

The signed creation intent fixes the resource, declaration/schema identities and versions, parameters, payload/authorization/recovery profiles, corpus, incarnation, storage strength and finite limits. Equal empty key bytes do not replace those checks. Development qualification uses injected disposable identities, keys, times and paths; they are not production defaults.

The factory validates the genuine provider. The actual host then obtains both stable lifetime root locks and validates full state before returning a usable session:

```rust
use glade_crdt_admission_data::{Fault, ReplicaOpen};
use glade_crdt_recovery_api::ReplicaRecoveryHost;
use glade_node::assembly::NodeAssembly;
use glade_node::independent::{
    DiskRootConfig, DiskSession, EvidenceConfig, PolicyClock,
};

fn open_scope<P: PolicyClock>(
    node: &NodeAssembly,
    evidence: EvidenceConfig,
    source: P,
    roots: DiskRootConfig,
    request: ReplicaOpen,
) -> Result<DiskSession<P>, Fault> {
    let mut adapters = node.independent_configured(evidence, source, roots)?;
    adapters.recovery.open_replica(request)
}
```

`provision=true` requires fresh roots and `ReplicaOpen.expected_floor=[0;32]`; it creates the signed identity's initial image and floor. Provisioning also writes `data/legacy-store.sealed` in that newly owned disposable root. The marker belongs to the new adapter's compatibility boundary: the qualified old legacy executable refuses this root. Preserve the marker and use this configured independent adapter for supported reopen; deleting it is not a qualified downgrade. This work does not seal or migrate an existing legacy store. Existing/partial roots refuse fresh provisioning. A lost creation reply does not authorize a new creation identity or a fresh-provision reset. Preserve the original registration/identity and inspect the owned roots for qualified reopen; an incomplete bootstrap can remain unavailable.

For `provision=false`, preserve the original root paths/registration, owner, plan/invocation namespaces and signed limits. Full `ReplicaOpen.expected_floor` is the digest of the **exact complete bytes of `trusted_floor/replica.floor`**, obtained from the trusted owned floor location with a bounded read. The file contains private envelope metadata as well as the public floor. Do not compute this value by re-encoding only `ReplicaFloor`, or obtain it from a peer, cached image or untrusted data directory. A trusted controller can read the registered floor file up to the signed decode-byte bound and hash those exact bytes with `Bytes::digest`; the host reacquires/validates lifetime locks and refuses if the floor changed. Symlink/path substitution, malformed bytes, missing floor or a competing owner refuse. Preserve the roots on unknown failure rather than resetting them.

`StorageAttemptHost::open(OpenRequest)` is the narrower consumer option. The configured host already owns the signed descriptor/identity/full limits and trusted roots. The caller supplies owner, invocation namespace and exact storage limits. On fresh creation the host derives a concrete plan namespace from its registered identity/owner/invocation inputs. On reopen it reads the validated authoritative floor under the held root locks and restores the **retained** plan namespace; it does not derive a new one. This includes roots originally created by `open_replica`. Its `OpenRequest` has no hidden caller plan-namespace or expected-floor field. This restoration trusts the independently protected floor; it does not detect arbitrary simultaneous rollback of that floor and data.

A host is a single-use owner of its evidence provider. After an open attempt, construct a fresh host/provider for another qualified attempt; no consumed provider is silently replaced. A session owns its roots until a successful close. Obtain state through `load()`/`recover()`: only Validated is usable; Unavailable requires preserving custody and resolving/reopening the original scope.

## Local append and authentication

For each actual connection/request, the trusted node ingress MUST perform a fresh signed local challenge/response using `issue_challenge(SessionRequest)` and `authenticate(response,channel,session_nonce)`. The request binds requester/writer key, exact operation digest, channel, session/challenge nonce and window. Copying a peer HELLO or reusing an operation-indexed authorization cache is not fresh possession. The node path owns the authenticated channel context; transport authentication alone grants no resource permission.

Then call `submit_local(candidate,channel,session_nonce)`. Its actual Pure/core/evidence/native-storage path returns `AcceptedLocal` only after durable selected terminal custody. `ExactRetry` preserves the original receipt and charges and still requires fresh possession. `StorageClass::LocalProcessRestart` names the actual strength: it is neither quorum durability nor power-loss/corruption/antirollback assurance. Foreign/bad/revoked/expired/uncertain admission refuses or remains pending according to the original contract; a timeout does not turn it into a successful write.

Native storage consumers can use the session's `prepare`, `recover_plan`, `begin`, `inspect` and `request_fence`, with the exact issued request values retained by the kernel. The complete issued Prepare may be retained in live or retired kernel custody. `prepare`/`recover_plan` look up its immutable existing AttemptBinding before fresh-allocation revision checks; replay changes no counters, charges or receipts. Caller-created invocation IDs or changed bindings refuse. `BeginRequest.current` is not authority; the provider independently obtains and durably retains its current cut. Preserve original PlanKey/AttemptId/request/receipt on unknown or lost replies. After reopen, the host reattaches original callbacks before exposing usable reads. A terminal remains immutable, including after expiry/revocation or late callbacks.

## Finite receive lifecycle

1. Obtain the current validated image/floor and build one exact `ReceiveGuard`: same store/instance/owner/lease, next guard number, current image generation/digest, authenticated source/channel, closed role, finite bytes/items and deadline. Same-instance rounds serialize. Retired history has finite capacity and no implicit GC.
2. Call `begin_ingress(guard)`. Only an Ok permit authorizes constructing application receive/parse work. Before consulting current policy/time, the host first durably establishes its protected observation marker. It retains a learned valid cut even when authorization is denied. A valid receive-only holder may advance time/policy with empty append grants. Advancing the image invalidates an old guard base: reload the validated image and build a fresh exact guard, then retry within your explicit finite retry budget (three attempts in the stable qualification witness). Never construct receive work on a refused attempt. If input remains changing/unsafe or the budget is exhausted, stop. A failure returns no consumption authority and may leave conservative durable custody.
3. Pass the permit to `receive_ingress(permit,make)`. The session rejects foreign/reused/closed input before calling `make`. The result is an owned bounded future returning `(permit,Result<Bytes,Fault>)` only when drained. Keep and join every node-owned task. Dropping a pending future or timing out an external waiter is not drain evidence.
4. For a complete authenticated inventory, prepare the exact `Observation` and coupled `Checkpoint`: retain its signed `(SignedRecord,Inventory)` bytes, exact expected references, missing obligations and watermark. Preserve the full kernel/native/session/receipt custody. Call `settle_ingress` with the returned owned permit and exactly the successful bytes returned by its owned receive. The host compares its private completion witness before authenticating the inventory. A different signed empty snapshot, malformed-prefix replacement, receive error or above-bound result cannot settle normally. A legitimate empty snapshot settles only that finite round; it cannot erase earlier uncertainty.
5. For a drained partial/unclassifiable result, call the concrete `retain_ingress_loss(permit,observed_bytes)`. It retains a compact hash-bound permanent-loss marker. An Err receive result can be conservatively classified using the owned drained permit and a bounded available prefix, including an empty prefix; this never asserts that no history was observed.
6. A live never-started permit can use `abandon_ingress`. Started/cancelled/orphaned input cannot use this shortcut.

If `settle_ingress` has already consumed the drained permit and refused, call `retain_pending_ingress_loss(&guard_id)` on that same live `DiskSession` to select conservative loss from its retained continuation. This includes malformed input and aggregate inbox/image capacity refusal. The method does not accept caller-created permits or reconstruct one after restart. It refuses unstarted/undrained, foreign, closed or unknown-physical custody. Failed loss publication keeps the original permit/observation/checkpoint owned and close Pending; `retry_ingress` can retry only when physical custody remains usable. On Committed the selected loss permits clean close but permanently keeps completeness false.

Only a committed floor selection retires utility status. If implementing a host, `IngressAuthority::can_settle(&permit)` and `can_abandon(&permit)` are **nonmutating** Result eligibility checks; foreign, closed, not-drained settlement and started abandonment refuse. Normal settlement additionally requires `can_settle_received(&permit,&bytes)`: it refuses absent successful completion, above-guard bounds, different lengths or different digests without changing ownership. `IngressPermit::received_digest()` is a read-only digest of the issuer-recorded bounded Ok result, or None; it grants no authority to settle an error. Conservative loss uses only drained eligibility. Run the appropriate query before disk I/O. Call the mutating utility retirement only after committed durable selection. The concrete disk session already enforces that order; consumers do not manually retire its authority.

For the concrete host, `checkpoint_template()` supplies current compare bindings and a next-generation image. It is not permission to alter arbitrary kernel state. `reconstruction_request()` supplies the exact full-cut manifest. A caller cannot clear the kernel's sticky recovery state, replace old receipts or label missing obligations as retained without stored proof.

## Results, ownership and retry

| Result | Consumer action |
| --- | --- |
| `Committed {generation,digest}` | The exact selected transition is durable under the named process-restart profile. The host has retired only the relevant guard. Preserve original receipt/request binding. |
| `Refused(fault)` before I/O | Fix only the stated precondition. An already consumed owned permit/observation remains host-owned; refusal cannot erase its guard. |
| `Unknown` | Preserve the registered scope and exact owned continuation. The physical host stays unavailable/pending until qualified recovery; never fabricate a permit, terminal, new identity or new guard to replace it. |
| `load/recover = Unavailable` | Stop serving validated state and preserve custody. Reopen the original registered roots with a fresh genuine provider and exact trusted floor digest when ownership/recovery permits. |

`retry_ingress(&guard_id)` retries only an exact by-value continuation retained by that live session. An unknown physical host remains unavailable; this is not a reset. `record_observation(observation,checkpoint)` can correct a pre-I/O checkpoint refusal **only** when the host still owns that exact drained observation/permit. It can also idempotently record an already selected exact observation without duplicating history. Fresh unguarded or changed observation bytes refuse. A new checkpoint can use only the current validated binding; it cannot erase old work. If both observation and attempted loss writes fail, the pre-consumption floor guard survives restart.

An exact `reconstruct_cut` combines kernel state, authoritative guards, retained observations/obligations and interrupted intents. Missing obligations, active guards, permanent loss or original sticky uncertainty keep `complete_local=false`. Later successful or empty inventories cannot clear unknown older history.

## Close and reopen stops

Call `ReplicaRecoverySession::close(&mut session)` explicitly (the native close method enforces the same combined lifecycle). The first close stops fresh challenge/authentication/local/receive input. Pending retains the session and both root locks while original queries/native requests/attempts, owned receives/continuations or unknown floor work remain. Continue only already-owned callbacks and cooperative cancellation/drain/classification. Join node-owned tasks; settle their exact result/loss; drive original native callbacks. Call close again. Only Closed permits clean physical release.

Do not drop the session as a substitute for Pending close. A dropped pending future has no joined proof; an external task's abort without joining is not a fence. Actual SIGKILL is qualified separately as process death.

After SIGKILL during a receive, generic reopen cannot manufacture the old permit or infer zero bytes from empty inbox/peer absence/lease change. It keeps the orphan Active guard pending and `complete_local=false`. A separately valid local append may still proceed if integrity and capacity allow. This bounded B provider does not supply a recovery reset or promise that every interrupted receive restart resumes exchange or completeness. C's coordinator must own clean cancellation/join and preserve this honest pending limit on unclean restart.

The private physical floor format is now closed `Ic3DiskFloor/v2`, including the protected current-observation marker; the previous independent v1 reader refuses it, and this adapter refuses v1 instead of migrating it. If that marker remains unresolved, both full and narrow reopen refuse. `load`, `recover` and reconstruction provide no usable old authority, and close is Pending. A complete authenticated pending observation image can recover and re-sync its exact next selection; a missing/torn next observation image cannot clear the marker. Do not remove a marker, edit the floor, substitute an older provider cut or reset the scope to make it open. This is an explicit unavailable recovery stop, not a promise of automatic repair.

The floor directory is an independently trusted custody input; copied data, missing/wrong floor, foreign roots, concurrent ownership and data-only rollback refuse. Arbitrary simultaneous rollback of data and that trusted floor is outside this profile. Failed sync/device/shared-volume capacity remains unknown/unavailable. Qualification is Unix process restart; other physical/platform guarantees require their own evidence.

The B implementation remains a candidate until the combined independent review is filed. C remote execution also requires the committed canonical Rust/TypeScript/Python prerequisite and genuine B acceptance record. Default binary behavior and live roots remain at their existing configuration.
