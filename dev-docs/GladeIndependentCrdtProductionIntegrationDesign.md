# Glade independent CRDT production integration — IC-3 design

Date: 2026-10-04. Status: **DRAFT for independent Consistency/Safety review**.
The owner authorized IC-3ABC. This document does not self-accept; implementation
follows semantic and compiling contract gates. The [delivery plan](GladeIndependentCrdtProductionIntegrationPlan.md)
and [review ledger](GladeIndependentCrdtProductionIntegration-ReviewCycle.md) control
this new object. Historical stopped objects retain their caps and content.

## 1. Authority, baseline and selected witness

Controlling sources: [admission plan](GladeIndependentCrdtAdmissionPlan.md),
[accepted storage contract](GladeIndependentCrdtStorageAttemptContract.md),
[semantic design](history/GladeIndependentCrdtAdmissionDesign.md),
[attempt design](history/GladeIndependentCrdtStorageAttemptDesign.md),
[accepted Pure implementation](history/GladeIndependentCrdtAdmissionKernelImplementation.md),
[boundary policy](LibraryBoundaryAndTestingPolicy.md), and
[package architecture](GladePackageArchitecture.md). Existing/archive references
MUST remain unchanged after the owner's documentation cleanup.

Inspected baseline: root `8c8332ad94ba42b43144830b6c3809c45d0b7bbd`,
Glade `37dff286ce1eb9690204d4a7d14c940333396a30`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external app-owned Gyld
`95a426595bba8e248a5f484272e483a070c73918`. Review uses a newly settled exact tuple.

The initial witness is two independently owned **actual glade-node processes**,
one identical authenticated fresh resource, distinct certified node-writer origins,
and released `crdt.oracle/v1` / `text_crdt.profile/v1`. Both nodes MUST accept valid
local edits while disconnected and automatically acquire the same historically
qualified operations/evidence and text after reconnect. Text is a consumer witness,
not a general CRDT engine limit. No client ferries operations between nodes.

Choose an explicit bounded development profile: direct signed resource root,
genuine Ed25519 keys, finite permits/quotas, injected trusted time uncertainty,
and two known authorized peer routes. Scratch provisioner creates disposable
new data/floor roots; no production key, live enrollment or existing resource
is changed. Current binary defaults/launchers/desk/browser admission remain
unchanged until IC-4. Independent capability MUST NOT be inferred from CRDT shape.

## 2. Source-grounded reuse and allocation

| Existing source | Reuse | Limit |
| --- | --- | --- |
| `glade/contracts/crdt-admission-core/src/{types,admission,staging,lifecycle,callbacks,projection,encoder}.rs` | Actual deterministic decisions, immutable plans, all four kinds, callback binding | No crypto, physical decoder, restart ownership or transport |
| `glade/contracts/crdt-storage-attempt-api/src/lib.rs` | Actual Host/Session, complete bindings, original receipts and terminals | Recovery is not a complete kernel image; no physical provider |
| `glade/node/src/signing.rs::{sign_in,verify_in}` | Existing pinned strict Ed25519 custom domains | Signature validity is not permission; SignerPort has only three legacy purposes |
| `glade/node/src/peer.rs`, `iroh_carrier.rs`, carrier API | Real node identity, CarrierLink endpoint/TLS exporter and signed HELLO pattern | Peer protocol4 has no independent capability/proof/format negotiation |
| `glade/node/src/records_file.rs` | File sync/rename/directory sync distinctions | Snapshot-only records.json does not qualify owned attempts or origin floor |
| `glade/node/src/{sysdir,store/legacy_seal}.rs` | Lock and actual process-kill patterns | sysdir lock unlinks on Drop; cannot blindly reuse as stable floor lock |
| `glade/node/src/{assembly,lifecycle,server,accept,mesh}.rs` | Actual Shaku scope/sdax task ownership and dispatch seams | Existing application admission still depends on claim holder |
| Core text trace and `glial/test/independent_admission_contract.mjs` | Exact ten rows and released Taut corpus | Memory fixtures are not crypto/disk/multi-process evidence |

Proposed package allocation MUST receive typed architecture review before creation:

| Package | Role / rationale | Minimal normal edges and contract |
| --- | --- | --- |
| existing storage-attempt API | Contract: owned attempt lifecycle | Retain sha2 and every existing required method |
| new `glade-crdt-admission-data` | Protocol/data: extract owned internal values/Operation so new contracts do not depend on concrete Pure algorithm | Wire + storage-attempt API; no marker trait |
| existing core | Pure: unchanged deterministic step and private batch encoder | Data/API/wire only as actually needed; re-export old public paths |
| new `glade-crdt-evidence-api` | Contract: replaceable proof and trusted observation provider | Data/API; required EvidencePort verify, seal, observe |
| new `glade-crdt-recovery-api` | Contract: complete reconstruction and observation retention, distinct from opaque storage attempts | Data/API; required ReplicaRecoveryHost open_replica; ReplicaRecoverySession load, checkpoint, begin_ingress, settle_ingress, abandon_ingress, record_observation, reconstruct_cut, close |
| new `glade-crdt-recovery-codec` | Protocol/data: bounded canonical physical/exchange representation | Data/API/wire/sha2 only as needed; explicit encode/decode, no artificial service trait |
| existing glade-node | Integration: concrete evidence/disk/duplex providers and owned runtime | Add preceding narrow contracts/data/codec/core; reuse pinned Ed25519/Iroh/Tokio/Shaku/sdax |

Data extraction MUST preserve old core import paths, canonical Op/batch v1 bytes,
all original tests/assertions and engine semantics. Codec performs structural
canonical decoding; deterministic semantic replay/validation using step belongs
in core. Boundary fields MUST NOT expose runtime, transport, injector or file types.
New contracts require compiling consumers, required meaningful methods, explicit
unknown/ownership/lifecycle semantics and reusable bounded conformance BEFORE real
implementations. Node and contracts architecture gates adopt exact reviewed edges
and roles; no dependency/role/process-global allowlist or test-budget relaxation.
Disabled cfg branches MUST be inspected. Broader existing declaration debt is not
claimed complete.

Typed shapes MUST cover: EvidenceObservation (exact instance/policy generation/
digest, conservative interval, current status and admitting node); ReplicaOpen
(provisioned identity/owner/limits/tuple/floor qualification); ReplicaImage (full
State, complete Recovery, requests, continuations, origin/intent/floors, observation
ledger, receive guards and loss markers); Observation (exact scoped immutable
inventory or bundle, authenticated source, digest and charges); Checkpoint (expected image generation,
new finite image and exact effects); ReconstructionRequest/Result (generation,
observation watermark/manifest digest and missing/classified identities, never an
unqualified complete switch); ReceiveGuard/IngressPermit (scoped durable receive
reservation and owned consumption capability); and ReplicaCut (kernel cut plus
exact adapter obligations and combined completeness). Paths/key handles are injected concrete
construction inputs, not peer data. Verification returns existing fully matched
EvidenceReply; no network DTO carries trusted Facts or storage callbacks.

The app-owned Gyld declaration MUST add a source-qualified IC-3 overlay retaining
all frozen prior allocations/obligations: Admission owns evidence decisions with
signer/policy cooperation; Records owns full recovery/obligations with StorageAdapter;
existing sync/transport owns duplex and obeys the Records-issued pre-consumption
gate; NodeAssembly owns scope/lifecycle. No Gyld
engine extension or runtime-satisfaction claim is authorized.

## 3. Genuine signed evidence and scoped authorization

IC3-AUTH-001: proof bodies are bounded canonical CBOR maps with fixed increasing
integer keys, shortest integers, finite lengths, no duplicates/unknown v1 keys or
trailing bytes, and encode/decode equality. Use strict pinned Ed25519 through
existing signing functions. Exact full zero-terminated domains are
`glade/ic3/v1/descriptor`, `origin-certificate`, `offline-permit`, `policy-cut`,
`application-op`, `local-session`, `admission`, and `inventory`, each with the
full `glade/ic3/v1/` prefix and a terminal zero byte. Domain/body version/bytes
are signed. No domain is a prefix of another; legacy Purpose need not change.

The creation intent is canonical unsigned data containing root key, unique nonce,
incarnation, resource/private owner, canonical zone, share/glade ID/key,
authoritative declaration hash/version, canonical parameter/key schema identity/hash/version, exact profiles,
independent mode and every bound. Its computed digest is creation_root.
The descriptor body binds those exact intent bytes and digest; the root signs that body. The complete
signed descriptor envelope is Descriptor.canonical and its computed digest is
Instance.descriptor. The intent cannot contain the descriptor envelope digest or
its own digest. Trusted provisioner supplies expected root/intent; human names or
discovery do not establish identity. Parsed fields MUST equal held descriptor fields.

### 3.1 Complete authenticated namespace mapping

IC3-ID-001: the genuine production envelope MUST bind every accepted identity-table
component below. These are immutable values of this fresh incarnation, not names
inferred from an operation. The provisioner supplies the exact authoritative
canonical declaration bytes and version and exact schema bytes/identity/version,
whose digests the verifier recomputes. They are authenticated root-authorized
inputs, not a replacement declaration chosen by a peer. Declaration shape MUST be
Crdt; unsupported declaration or key schema refuses before open or admission.
Parameters MUST validate against the pinned schema and derive exactly the signed
canonical key bytes under its versioned canonicalization rule. Equal key bytes do
not prove equal schema. No schema migration or remapping is inferred.

| Accepted semantic design section2 component | Exact IC-3 representation |
| --- | --- |
| Resource identity / creation-root ancestry / canonical resource ID / genesis intent | Signed intent root public key + unique nonce + resource/private-owner ID; retained root-authorized intent bytes/digest (`creation_root`). Direct-root/v1 fixes ancestry depth to this authenticated root; any delegated ancestry refuses |
| Instance incarnation / canonical share / Glade ID / canonical zone/key | Explicit intent incarnation, zone, share, glade_id, key bytes; IdentityBinding carries canonical zone and schema-validated parameters/key derivation, never a browser origin |
| Canonical parameter/key schema | Explicit `schema_identity`, `schema_version`, `schema_hash` from retained canonical schema bytes and explicit validated canonical parameters; exact canonical key derivation is profile-bound |
| Authoritative declaration hash/version / Crdt shape | Explicit `declaration_hash`, `declaration_version` from retained authoritative declaration bytes; `shape=Crdt` is a signed fixed v1 value and checked against declaration |
| Merge engine / payload / corpus versions | Explicit engine=`crdt.oracle/v1`, payload=`text_crdt.profile/v1`, `merge_corpus_identity`, `merge_corpus_version`, `merge_corpus_digest`; the selected corpus is the exact existing released-Taut concurrent_siblings corpus, whose source digest is independently pinned in the current Glial control |
| Admission capability | Explicit mode=Independent and admission_profile=`glade.crdt.independent-admission/v1`; Legacy/Strong refuse on this route |
| Authorization profile/version / trusted roots / authentication / permit / placement | Explicit authorization_profile=`bounded-offline-permit/v1` and proof_profile=`ic3-direct-root-ed25519/v1`; intent names expected root key; signed policy/permit enumerate original principal, actions, admitting nodes and replica holders; signed-session/v1 fixes possession strength; unsupported delegation refuses |
| Receipt class / bounds / retention / rejoin / initial chain base / migration generation | Explicit storage_class=`LocalProcessRestart`, all descriptor/session/decode/observation/guard bounds; signed recovery_profile=`ic3-records-floor/v1`, retention_policy=`full-history-no-gc/v1`, rejoin_policy=`exact-evidence-full-cut/v1`, initial_chain_base=0, migration_generation=0 for fresh nonimported incarnation |

The signed descriptor binds the complete intent and therefore the complete table.
`IdentityBinding` is a new production boundary value containing these exact
additional identities; trusted ReplicaOpen/provision inputs, EvidencePort queries
and image validation MUST check it. Every certificate/permit/op proof/admission,
receipt, cursor, cache, pending request and replication session binds the entire
tuple directly or via exact Instance.descriptor. HELLO capability negotiation MUST
compare descriptor digest AND the expected supported profile/declaration/schema
metadata before any history receive. Unknown/different field/version refuses.

Core's current internal Descriptor need not gain fields merely to simulate these
checks: its immutable canonical bytes already bind the full new signed envelope.
A typed production wrapper MUST carry IdentityBinding alongside it and authenticate
both against trusted provision; the Pure provider never treats extra bytes as
verified by their mere presence. This is an explicit production interface/data
adaptation under fresh typed review, preserving existing inner Op, core public
re-exports and every original behavioral assertion. Four isolated negative
witnesses mutate declaration hash, declaration version, schema identity/hash and
schema version while holding names/key bytes constant; each changes authenticated
identity or refuses trusted open, verification/admission and exchange.

### 3.2 Cross-language prerequisite for remote use

IC3-CANON-001: before ANY IC3C remote use, pinned independent Rust, TypeScript and
Python consumers MUST agree on exact canonical bytes and SHA256 digests for all new
namespace/proof/transfer representations: creation intent, signed descriptor,
certificate, inner permit and operation-proof wrapper, policy cut, local session,
admission, HELLO/capability transcript, inventory header/pages/entries and transfer
bundle. The vector artifact records version, inputs, exact expected bytes/digests,
keys/signature bytes where applicable, producer/consumer source pins and assertion
IDs. All three consumers derive bytes independently from the semantic input; no
consumer may pass by copying stored expected encoded bytes or invoking another
language's encoder. Shared explicit vectors, not the encoder being checked, supply
the expected results. Preserve frozen canonical inner Op controls and unchanged
released-text corpus. This gate does not require browser UI activation.

Positive/edge vectors include empty/Unicode/binary values, finite integer boundaries,
canonical field ordering, repeated blobs, same-slot rivals and exact profiles.
Negative vectors include duplicate/unknown fields, reordered/nonshortest encodings,
invalid UTF-8, overlimit/deep/truncated lengths, trailing bytes, mutated scope/domain
or signature and altered declaration/schema identity. Each consumer MUST reject
specified malformed/noncanonical input, never normalize it into valid proof.
Checklist/source gate MUST fail if any required language implementation, vector
category, pin or assertion is missing. Three Rust nodes cannot substitute for the
three independent languages. A2 defines/fixes these vectors; B1 checks genuine
signature controls; C cannot start remote qualification until the complete gate is
GREEN. Physical Rust-only Records envelopes retain their bounded codec/field tests;
any physical representation exposed remotely is also covered by this gate.

### 3.3 Original writer and current/historical authorization

The root signs certificates binding instance, fresh canonical origin/epoch, writer
key, original requester, display name and resource. Issuance refuses canonical
origin reuse. Direct-root profile has no delegated issuer/operator-vouched append
or arbitrary attenuation; unsupported ancestry refuses rather than flattening.
Signed policy grants explicit requester/resource actions, admitting nodes and
replica holders; no blanket grant or client governance/effect authoring. Permits
bind instance/certificate/requester/key/origin/append action/admitting nodes/policy
generation/finite validity and sequence range. Writer proof signs exact canonical
Op and scope plus certificate/permit digests/requester. Op remains unchanged.

Use existing EvidenceSet: certificate is its signed envelope; permit is a v1
wrapper containing signed permit and writer operation proof; policy is signed
cut; admission is signed original admission. Writer signs the inner signed permit
digest, not the outer wrapper containing its signature, avoiding recursion.

IC3-AUTH-002: local ingress proves certified writer possession on a fresh signed
challenge binding node, instance/resource, requester, op digest, ingress role,
connection/session nonce and finite lifetime. Bounded nonce state is consumed once;
a challenge cannot authenticate another connection/node/operation. Writer operation
proof remains reusable for exact retry. Forwarding preserves writer/requester proof;
HELLO is not requester authority. Resolve self from authenticated principal/grants;
literal another owner's scope refuses. Current append, replica.hold and serve/read
permissions are distinct. Historical validation does not grant current serving.

IC3-AUTH-003: inject current signed policy and trusted clock interval. SystemClock
instant alone does not establish uncertainty; development config provides a finite
trusted bound. Unknown/regressed time, bad interval, missing policy/key evidence
fails pending/unavailable; known denial refuses. Entire interval fits each finite
permit window/range. Retain policy/time floor before new admission. Payload validation
uses exact released text profile syntax; no new engine. Facts bind all Query fields
and real certificate issuance; peer-supplied booleans never authenticate.

IC3-AUTH-004: verification/sealing and protected start use exact immutable cut.
Records compares fresh independently injected authoritative policy/time observation
under instance serialization just before durably Started; Begin.current never
updates the provider. Earlier queued policy changes apply before this barrier.
Valid Started survives later revocation, delayed reply and restart; no reseal/remint.
Admission body binds instance/op/certificate/permit/policy digests, original
conservative interval, admitting node, storage class and stable ID (op digest,
as current kernel). It does not recursively include its own digest. A seal is
conditional until atomic commit and MUST NOT be exported as accepted before that
terminal. Final trusted start must exactly match sealed cut, else fence/refuse.

IC3-AUTH-005: historical verification authenticates original admission/policy/cut,
finite time/range, writer proof, authorized admitter and structural closure, without
retroactive denial from today's expiry/revocation. Current revocation stops new
admission/serving. An unaware disconnected authorized node may admit within its
remaining finite window; learned revocation or expiry stops it. Retain accepted
trusted-admitter/time assumption; do not claim global nonrevocation/Byzantine safety.
Bare signed rival is security custody; only two independently historically qualified
rivals convict. Fresh E1 has a new canonical origin; old exact receipts stay unchanged.

## 4. Physical format, owned floors and restart

IC3-DISK-001: fresh independent root is outside legacy scanning, with fail-closed
`ic3-records/v1` marker/resource inventory and stable lifetime OS lock. Lock inode
is never unlinked/replaced on ordinary close; validate canonical root/path/object
identity and regular nonsymlink files. Namespace registration is trusted provisioner
input, not copied-image inference. Fresh root rejection by old binary MUST be tested,
not assumed from naming. No import/legacy seal/existing-resource activation is selected.

Each instance owns its own State (one instance), independent StoreIncarnation,
plan/invocation namespaces, next IDs, Recovery/session, worker, image slots and
floor files, including receive-guard counters/records. Root lock is acquisition/lifetime custody only, never a shared mutable
CAS or root allocator held while X worker blocks. Instances have separate typed
sessions with scoped namespaces; Y operations do not need X's lock/floor/worker.
A common filesystem outage can affect both; independence is logical with Y's
storage functional, not separate hardware.

Physical envelope v1 contains header/version tuple, root/instance/logical-owner,
generation/predecessor digest, EVERY kernel State field, complete API Recovery,
preparing/attempt/batch history, outstanding/retired full requests, terminal/conflict
history, descriptor/evidence/origins/intent/policy/time floors, exact inventory/inbox
ledger, active/settled receive-guard bindings and permanent-loss markers, usage and critical reservations. Maps/sets canonical,
enums/options explicit; all Bytes digests recomputed. Unsupported versions, tears,
trailing bytes, duplicate identities, overflow, invalid strings, malformed binding or
noncanonical encoding refuse before usable restore. Allocation is capped BEFORE
allocation by injected trusted decode limits, then descriptor/session bounds match.

Complete batch-v1 decoder reconstructs every CommitPlan field, not just operations;
canonical round-trip plus existing complete-field mutation matrix covers it. Cached
State is not authoritative over a retained terminal. No Debug/serde seed qualifies
physical state. Full history has no GC; finite exhaustion stops new work while
preserving all custody/receipts/outcomes/reservations.

IC3-DISK-002: independently configured trusted floor directory lies outside
replaceable data root and is exclusively owned. It binds canonical root path,
root registration, instance/store incarnation, issuance maxima, logical owner,
private physical lease serial, monotonic floor sequence distinct from image
generation, bounded active receive guards and per-instance selected image
generation/digest or interrupted exact intent. Every floor update preserves those
guards unless exact qualified settlement selects its coupled observation image. Trusted scratch provisioner registers it before startup.
Copied data at another path cannot register itself or mint a floor. Wrong/missing/
stale/foreign/untrusted floor or concurrent lock refuses writable recovery.

This profile explicitly TRUSTS floor against copying/rollback together with data
and keys. Disk hashes/locks cannot detect rollback of both. Test data-only rollback,
copy to a different root and absent floors; do not claim arbitrary antirollback.
Unqualified backups stay nonwritable. A new writable epoch requires trusted issuance
of a never-reused canonical origin; never reset old origin seq0. Retain origin/epoch,
next seq/prev/Lamport/causal maxima and exact pending intent/receipts; persist immutable
new intent before submit and retry exact bytes after restart.

Qualified process restart retains logical API Owner and plan/invocation namespaces
and full old requests. Private physical lease serial increases under floor+lifetime
lock. This is reattachment to same logical custody, not request rebinding or replacement
owner. Workers are owned in-process local filesystem only; killed process cannot
leave external I/O workers. New live process cannot attach before lock acquisition.
Owner replacement, detached/external storage workers and writable clone are unsupported.
Typed reviewers MUST verify this distinction against accepted ownership clauses.

IC3-DISK-003: use two per-instance finite reserved image slots and trusted floor
transition. Provision actual slot bytes/write/sync and reserve maximum full encoded
critical attempt/outcome/request/fence/observation footprint before ordinary edit
quota. Sparse set_len is not allocation proof. I/O failure may remain Pending; no
guaranteed terminal deadline or resilience to exhausted/broken device is promised.

Serialized atomic transition schedule:

1. Validate exact current generation/image/owner, complete next image/request/
   precondition, finite encoded size and critical reservations.
2. Durably write floor intent naming exact old selected slot/generation/digest,
   exact next slot/generation/digest, operation kind, next issuance floors, all
   active guards and any proposed exact guard settlement.
   A lost acknowledgment is unresolved; old selected slot remains untouched.
3. Write inactive slot complete envelope then sync file. Torn/wrong-digest slot
   is not valid. Never overwrite selected slot while intent is unresolved.
4. Atomically replace floor with selection of exact next image/floors; sync file
   and directory. Floor selection retains each active guard or retires it ONLY with
   the matching exact observation/loss image. Only confirmed durable selection
   permits committed acknowledgment or guard discharge.
5. Retain old slot until next validated transition; never decrease reserved IDs.

Floor replacement before rename and after rename/directory-sync have distinct
unavailable/unknown outcomes. Reopen exclusively validates floor and both images.
Interrupted intent plus fully valid next image re-syncs that exact slot and then
finalizes that exact next selection; unreadable or failed sync keeps recovery
unavailable/unknown. A missing or invalid next image preserves exact old image
and classifies the interrupted metadata transition without reusing advanced IDs.
It MUST also preserve every guard from the selected floor/intent: fallback never
restores an earlier guard-free floor or applies a proposed settlement whose image
is absent. Ordinary checkpoint/fence/lease updates cannot omit an active guard.
This does NOT invent NonCommit for old Reserved/Started plans. A valid old image still carries those plans for original Inspect/Fence.
Recovery tests every intent/slot/floor write/sync/rename/reply cut, including absent,
old, new, torn and corrupt variants. Missing/unreadable floor/image proof fails closed.

IC3-DISK-004: prepare persists full mapping/binding/reservation before Prepared;
Started retains checked cut before publication; Committed atomically retains complete
batch custody/original receipts/charges/terminal in selected image. All four kinds
have same coupling without synthetic Candidate/Security/Fork app admission. A durable
terminal may lead cached core State: envelope then MUST retain decoded original plan
and genuinely issued exact request and recovery MUST replay that terminal through
actual core callbacks before any read/append, preserving once-only accounting.

Every effect's exact issued request/continuation and issuance floor is checkpointed
before external provider invocation. Restore validates complete outstanding/retired
requests, owner/namespaces, actual cardinality vs high-water, phase payloads, revisions,
continuous committed history, charges and original receipts. Missing/foreign request,
unbacked revision, duplicate contradictory terminal or changed receipt refuses;
AttemptId alone never manufactures callback authority. Same-image atomic observation/
checkpoint transitions retain inbox classification and core state together, preventing
lost obligations after restart or terminal recovery.

IC3-DISK-005: pending preparation recovers original PlanKey. Absence/timeout/cancel/
lost reply is not NonCommit. Inspect/fence/publication share retained predicate,
one terminal winner; genuine delayed Started/Pending cannot downgrade it. Contrary
bound terminals retain original/first contrary evidence plus integrity stop. Per
instance at most one unresolved plan; X never holds Y's per-instance resources.

IC3-DISK-006: close stops fresh input, drains/fences/joins all workers, checkpoints
uncertainty, releases lifetime lock only after Closed. Pending close keeps custody;
sdax waiter cancellation cannot detach a worker and call it fenced. Actual SIGKILL
and fresh process reopen, not Drop/new MemoryHost, proves restart. Qualified receipt
is LocalProcessRestart only after durable selection: same qualified local storage
process reopen, no quorum/remote copy/machine loss/arbitrary rollback or hardware
power-loss claim. Old VolatileTest receipt strength remains unchanged.

### 4.1 Durable pre-consumption ingress grammar

IC3-GUARD-001: Records MUST establish a finite durable ReceiveGuard BEFORE the
application transport can consume any history-bearing inventory, page or bundle.
This includes the first inventory header identifying a remote cut/digest, and
local qualification/control ingress that introduces external historical input.
The earlier post-failure loss-marker instruction is replaced by this precondition;
after-only marker attempts cannot protect restart. Ordinary OS/Iroh opaque encrypted
buffering before application reads is not an observed cut and MUST NOT be parsed,
inspected or dispatched as history before the permit. Fixed authenticated channel
negotiation containing ONLY the already-provisioned identity/version tuple can
precede a guard; it MUST NOT carry remote operation heads, inventory digests or
history. Raw frame/body receive, partial decode and dropped-overlimit bodies are
inside the guarded consumption boundary, not just successful verifier dispatch.

`begin_ingress` atomically persists a floor-only guard under that instance's own
floor sequence, binds instance/store/logical owner/physical lease, fresh monotonic
guard ID, authenticated peer+channel nonce/role, selected base image and finite
round byte/item/deadline bounds, then returns an owned noncloneable IngressPermit.
No initial body/manifest digest is needed; it is unknown before consumption.
Durable establishment requires floor file+directory sync and acknowledgment.
The floor guard ledger is authoritative Records metadata and may lead the last
selected image: a guard-only floor update intentionally does not rewrite that
image. Load/checkpoint/reconstruction MUST merge and validate every such guard
against its selected-base image and monotonic floor sequence before exposing any
cut. A cached image with no guard cannot overwrite or erase a newer floor guard.
Images retain settled guard bindings/history; matching floor selection is the
only authority to retire active entries, never image absence. Retained guard
identity/history has finite declared capacity and no implicit GC; exhaustion
prevents new receives without dropping old guard/obligation evidence. A
failed/unknown establishment returns no usable permit: no application recv/parse
may start until the exact guard is validated. Guard IDs/cardinality and worst-case
unknown/loss/discharge records consume finite critical reservation before receive;
capacity or inability to persist establishment prevents new consumption entirely.

The production receive call graph is `scoped channel -> begin_ingress -> owned
IngressPermit -> bounded recv/parse -> settle_ingress(image+exact observation)`.
Only the node's owned gate can execute receive; peers cannot mint permits. A
permit cannot be reused on another round/link/instance, and the coordinator owns
all receive callbacks and bytes until settlement or qualified loss. Same-instance
rounds are serialized with at most one active guard; Y has independent guard/floor/
worker capacity. No shared floor CAS or global ingress allocator blocks Y.

IC3-GUARD-002: a round is a finite single immutable message or an exact bounded
snapshot transaction with header, declared pages/bundles and authenticated end
binding the same cut/transcript digest. Receiver ends consumption after the exact
message/end; future async input needs a new guard. A timer may cancel the transport
waiter but cannot prove no observed history. Pause application receives between
rounds; do not leave a long-lived idle guard merely because a link is connected.
Normal authenticated exact empty snapshot is one retained observation of that
specific round, not proof that earlier observed history never existed.

`settle_ingress` selects one coupled image containing exact authenticated received
observation bytes/manifest (or exact malformed bytes plus rejection result), inbox/
obligations/classification, core checkpoint/attempt history, guard identity and
settled watermark. The same successful floor selection retires ONLY that guard.
Stored guard/evidence digest and floor retirement must agree. It can settle while
work remains in a retained inbox; combined completeness remains false until those
obligations classify. A rejected or oversize/partial message cannot be treated as a
complete empty manifest: retain bounded rejection/loss status, and if full necessary
bytes cannot be retained, atomically settle permanent observation loss instead.
No app custody acknowledgment precedes required exact retained evidence.

Uncertain/failed settlement leaves the guard active, even if a later attempt to
persist a loss marker also fails. Every replacement floor and intent carries it.
Reopen validates selected floor first, then image. An active guard suppresses
complete reads before any reconstructed old image can be served. After drain/
termination of the old physical receive owner, recovery can only retain it pending
or convert it to a permanent untracked-loss marker in a new coupled image; it
cannot infer a zero-byte receive from absent inbox, peer absence or a new empty
inventory. Old-image fallback explicitly preserves guard uncertainty. Inability
to persist classification leaves guard/unavailable status, never old completeness.
Once lost, later successful full inventories do not clear that loss.

IC3-GUARD-003: cancellation/shutdown first closes the permit's gate against future
reads, cancels and JOINS/drains every owned read/parse/callback, then attempts exact
settlement/loss persistence. A live never-started permit may use `abandon_ingress`
with Records-owned gate evidence that NO receive/parse was ever invoked and no
future callback can invoke one; this atomically retires only that guard. Caller
booleans, timeout, channel EOF after a started read, Drop, sender retry or a missing
body are not such evidence. If a read was invoked but full observation cannot be
classified, retain unknown/permanent loss even if live code believes no bytes came.
After restart the never-started proof is unavailable, so a residual guard remains
conservative. Close can release physical resources only after receives are joined
and guard retained/settled; Pending close retains ownership. SIGKILL leaves floor
guard for qualified reopen. Guard uncertainty need not prohibit a separately valid
local append after floor recovery, but MUST prohibit complete=true; integrity or
unavailable floor still stops mutation. No kernel bool is cleared.

| Receive/crash cut | Durable witness and required reopen result |
| --- | --- |
| Before begin_ingress; establishment fails before replacement | No permit and no application history consumption. Prior complete image may reopen complete if every other obligation is settled |
| Guard replace/sync uncertain | No consumption until validated acknowledgment; if replacement installed, active guard remains on reopen even in paired no-input execution |
| Durable guard, before any receive | Guard active. Kill cannot prove never-started; incomplete/pending/loss on reopen. Live qualified never-started abandon is the positive no-input control |
| Receive invoked, partial header/body/decode, timeout or cancel | Active guard predates bytes; drain then retain exact observation or loss; restart cannot become complete from absence of bytes |
| Full observation received; observation write and loss write both fail before floor intent | Pre-existing active floor guard survives. Recover storage with peer absent: remain incomplete/unavailable |
| Durable observation floor intent; next slot missing/torn | Preserve old selected image PLUS guard from selected floor/intent; never apply intended settlement without its exact image |
| Valid next observation slot synced; floor select unknown | Validate/re-sync next image; either select exact coupled observation+guard retirement, or retain old image+active guard; no disconnected retirement |
| Complete observation image and floor retirement selected; reply lost | Reopen exact retained observation/inbox/obligations; exact duplicate dedup, reconstruction remains pending until closure; no fabricated admission |
| Exact complete empty round selected | Retires only that round guard; older guard/loss/obligation still blocks complete |
| Never-started live abandon intent/select fails or is interrupted | Guard remains or exact persisted qualified abandon is validated; no future recv permitted on the retired permit |
| Guard classification fails while close/drain runs | Join read tasks; durable guard remains. No cleared lock/state can authorize falsecomplete on reopened old slot |
| X guard/floor write or receiver held | X incomplete; functional independent Y remains able to receive/admit/reconstruct within its own bounds |

The indistinguishable pair is deliberate: after successful guard establishment,
no-input-kill and observed-input-with-all-writes-failed-kill both retain the same
uncertainty and cannot return complete. Before a failed establishment, neither
execution may consume history. Normal full retained round and qualified live
never-started cancellation recover without permanent loss. Typed RED and real
process tests MUST cover both pair members and every table boundary.

## 5. Production node path and automatic bounded duplex

IC3-NODE-001: private explicit independent configuration is parsed once at binary
entry into Settings/NodeStart. Absence preserves current default. Actual call graph:

`glade-node::start -> NodeStart -> node_plan Assembly/Storage -> NodeAssembly
independent evidence/recovery/admission bindings -> per-instance IndependentReplica
owned runtime -> actual core::step -> persist effect continuations -> genuine
EvidencePort / StorageAttemptSession -> actual core callback -> ReplicaCut/response`.

Actual accept.rs/server profile dispatch selects independent route ONLY from exact
authenticated descriptor capability. It bypasses legacy placement/holder/home/Store
append; legacy/strong routes remain their existing paths. Private qualification
local control ingress uses signed session/op envelopes and same production runtime,
not test-supplied Facts/receipts or a second successful model. Browser/UI DTO freeze
is deferred IC-4. Test launches two actual binaries with explicit scratch bindings.

IC3-NODE-002: single Shaku scope owns providers, Records sessions and real carrier;
validated open/recovery precedes admission/exchange. sdax owns runtime, retry timer,
links and disk workers. Stop admission, settle runtime/disk work then release floor/
root locks and carrier. Partial startup, missing provider and release-order tests
cover failure; no ambient configuration reads/new process globals.

IC3-SYNC-001: real Iroh CarrierLink uses separately negotiated independent channel.
HELLO/capability binds both node IDs/endpoints/TLS exporter/roles/fresh channel nonce
and exact descriptor/proof/merge/storage/recovery tuple. Protocol4 alone is not
support. Before inventory/payload, current scoped replica.hold and endpoint/node
binding must pass. Identity alone grants nothing. Wrong scope/incarnation/tuple or
unauthorized peer refuses without custody mutation. Current revocation closes
exchange/serve permission without erasing valid historical operations.

IC3-SYNC-002: signed immutable full inventory names instance/source/cut and complete
ordered entries/aggregate digest. Entries cover accepted operations with original
admission/receipt/evidence, candidates, security rivals and explicit fork pairs;
distinct same-slot rivals remain distinct digest entries. Highest heads/eligible set
alone is insufficient. Each entry binds kind/op/evidence bundle digest/coordinate.
Changed history yields a new immutable inventory, not replacement of old pages.

Paged header binds exact finite count/bytes/page count/manifest digest; pages bind
index and same cut/digest. Establish the durable guard before even consuming this header. Retain whole
manifest within configured bounds before processing promise. Refuse oversized lengths before allocation/unbounded body read;
partial/unretained manifests are incomplete. Empty/newest inventory cannot erase
older missing obligations. Full-history exhaustion stops sync honestly rather than
truncating history. All limits injected finite development values, no production
scale/default promise.

IC3-SYNC-003: independent symmetric drivers advertise, compute exact digest diff,
request missing bundles/ancestors, serve authorized data and retry on reconnect.
Operation/certificate/permit/admission/policy/fork closure travels unchanged.
Genuine historical verifier/core qualifies incoming bundles; receiver does not reseal
remote write or replace original requester by peer identity. Exact duplicates are
idempotent; changed bytes under old ID/wrong coordinate fail. Each node's committed
history plus authorized peer config derives handoff; no best-effort-only outbox
after AcceptedLocal. Peer acknowledgment is not local app admission/quorum; original
admitting receipt is never replaced by remote custody strength.

IC3-SYNC-004: require the durable pre-consumption guard, then retain bounded
inbox/inventory obligations durably BEFORE dispatch.
Serialize same-instance verification+storage: park B/its reply while A unresolved,
then reschedule exact B; do not hit old busy branch and forget history. Fair bounded
scheduler prioritizes owned recovery, then retained historical/local work; Y has
separate session/inbox/worker. Backpressure refuses credit/ack until input retained;
peer retries exact unacknowledged bundle. Disconnect/cancel does not erase obligations.
Unexpected kernel historical Capacity stays sticky. Unretained observed input
leaves its pre-existing active floor guard; retain permanent loss when possible,
but if all later writes fail that guard still prohibits complete reads on reopen. Persist inbox/classification/attempt transitions in
coupled Records images, not a process-only flag or independent best-effort file.

## 6. Full-cut reconstruction with narrow eligibility

IC3-CUT-001: add NO event clearing kernel recovery_incomplete/integrity stop. Existing
true, unknown provenance/imported state and permanent untracked loss stay incomplete;
retry/restart/empty peer cut cannot clear them. This retains accepted contract section7.

Reconstructible cohort is authenticated fresh empty genesis plus continuous durable
observation provenance. Exact expected digest/coordinate/kind is retained before
candidate dispatch. Do not call old ObserveFrontier with unknown hashes; it cannot
retain them. Adapter ReplicaCut exposes exact missing obligations and real duplex
fetches them. Ordinary core candidates still determine custody/projection; frontier
Event may be used only for already-retained exact hashes. Busy historical inputs
are dispatched after owned work settles. No prior sticky image is magically repaired.

IC3-CUT-002: on captured generation/observation watermark/manifest digest, Records
validates descriptor/continuous provenance, complete requests/outcomes/custody and
original receipts; replays exact qualified terminals through core; recomputes cuts/
charges; accounts for EVERY observed inventory/input up to cut. Exact bundle identity
AND coordinate must classify as accepted, pending dependency, security evidence,
qualified fork or genuinely invalid attributed bytes with retained verifier result.
Missing ancestry/unknown/unavailable proof remains unresolved; rejected bytes do
not erase an advertised qualifying predecessor. A peer complete assertion is not
proof. Result is tied to exact generation/watermark; stale result after new transition
is rejected and all classifications checkpointed before response.

IC3-CUT-003: combined complete_local requires actual kernel cut complete, all exact
observed obligations classified with required closure accounted, no omitted pending
inbox/query/attempt or active/uncertain receive guard, no permanent loss/integrity and continuous validated provenance.
Adapter retained obligations can progress false to true after full reconstruction;
NEVER change kernel bool. Kernel-alone complete while inbox/inventory pending must
be exposed incomplete. New observations beyond cut remain outstanding. Complete is
locally observed cut, not global peer/resource enumeration/no unseen concurrent edit.
This leaves old sticky/overflow images without automatic repair; general repair/import/
GC/backup recovery needs separate reviewed contract. Valid admission/projection may
continue with incomplete reads if integrity permits.

## 7. Exact normative replacement and compatibility table

| Accepted source clause | IC-3 treatment |
| --- | --- |
| Storage sections2–5 immutable binding/issuance/start/four-kind coupling/monotonic terminal/full callback authentication | Retain unchanged; actual providers implement, no absence-as-NonCommit/request rebinding |
| Storage section6 ownership/floors/restoration/finite reserves/full history | Retain; selected physical profile specifies qualified same-logical-owner process reattachment with private lease. Replacement ownership unsupported; typed review checks compatibility |
| Storage section7 sticky incomplete/no clear Event | Retain EXACTLY; new combined cut accounts only continuously retained adapter obligations |
| Historical IC-1 values superseded by storage contract; current Pure public types | Retain semantic assertions/old re-export paths; data extraction and new contracts require fresh typed gate, no success bypass |
| IC-2 internal encoder v1 and Operation byte binding | Retain bytes; add full bounded decoder and separate physical/exchange version |
| Semantic identity table/canonical pre-remote prerequisite | Restore exact declaration/schema/complete-table binding via production IdentityBinding; mandatory independent Rust/TS/Python vectors before remote use, preserving Op and Pure inputs |
| Initial IC-3 post-failure marker instruction / observation retention API | Replace with IC3-GUARD-001–003 pre-consumption floor guard and owned permit, begin/settle/abandon operations and crash table; old-slot fallback retains guard; same object remediation1, material boundary amendment |
| Semantic design offline historical validity/process custody/separate roots | Retain; explicit direct-root/trusted clock/external-floor development assumptions, no global revocation/arbitrary rollback promise |
| Peer protocol4/client holder path/ReplicaSync no-mutation errors/whole legacy seal | Retain legacy; separately negotiated fresh independent route and explicit observation/custody outcomes, no migration/hidden partial mutation |

New semantic outputs are combined ReplicaCut and durable ingress guard/permit
lifecycle. Additional authenticated IdentityBinding fields complete the existing
namespace; no accepted tuple component is removed. Further clause replacement
requires exact counterexample/replacement and fresh review. Do not weaken old tests or
rewrite archived provenance to accommodate an adapter.

## 8. Stable requirements and exact witnesses

| ID | Success | Failure / edge |
| --- | --- | --- |
| IC3-ID-001 | Full accepted namespace validated in provision/open/proof/HELLO | Separate declaration hash/version and schema identity/hash/version mutations with equal names/key bytes |
| IC3-CANON-001 | Pinned Rust/TS/Python independently reproduce exact namespace/proof/transfer bytes/digests; frozen Op controls | Missing language/category/pin/assertion fails prerequisite; malformed/noncanonical negative corpus rejected before remote use |
| IC3-AUTH-001 | Genuine descriptor/cert/permit/op/policy/admission round-trip | Wrong domain/key/body/scope/permit; weak key; duplicate/noncanonical/trailing bytes |
| IC3-AUTH-002 | Signed challenge enters actual production independent ingress | Replay/foreign node/session/requester/self scope; missing possession; peer-as-writer |
| IC3-AUTH-003 | Entire interval/seq range allows local edit | Unknown/regressed/straddled clock; expired/future permit; denied/revoked/unavailable policy |
| IC3-AUTH-004 | Real trusted Started cut survives later revocation | Queued Begin after provider change; echo-as-provider; stale seal; kill before/after start |
| IC3-AUTH-005 | Prior genuine historical admission remains eligible | AD bare rival vs A genuine fork; E1 AB; exact old receipt |
| IC3-DISK-001 | Fresh root opens correct profile | Old binary/new root; bad marker/symlink/nonregular/legacy/strong/tuple; seal bypass |
| IC3-DISK-002 | Restart retains origins/requests/floors | Rival lock; copied data/wrong path; rollback/missing floor; no automatic epoch reset |
| IC3-DISK-003 | Every floor/slot cut settles exact prior/next | Actual process kill at intent/write/sync/rename/select/reply; corrupt/truncated/oversized/allocation failure |
| IC3-DISK-004 | All four kinds recover coupled custody/outcome/receipts once | Lost replies; missing/mutated request/binding; revision gap; reminted receipt; terminal ahead of core |
| IC3-DISK-005 | Fence/commit retain sole winner; Y progresses with X held | Both race orders; delayed/contrary terminals; unknown absence; counter/cardinality exhaustion |
| IC3-GUARD-001 | Durable bounded guard before any history recv, independent Y | Guard establishment unavailable/unknown forbids consume; paired no-input vs observed-write-loss kill survives restart |
| IC3-GUARD-002 | Atomic exact retained round plus guard retirement, normal empty and nonempty control | Observation/loss persist both fail; old-slot fallback/torn next slot retains guard; wrong round/empty latest cannot erase history |
| IC3-GUARD-003 | Owned drain and qualified never-started live abandon | Timeout/Drop/partial receive/restart cannot prove no input; failed cancel/loss write leaves floor guard, close owns every callback |
| IC3-DISK-006 | Close joins; actual SIGKILL reopen exact retry | Waiter cancel/Pending close/hung worker; false restart class before durable selection |
| IC3-NODE-001 | Two actual binaries/same production runtime | Facts injection/alternative model/holder dispatch rejected by runtime/source guard |
| IC3-NODE-002 | One scope and owned startup/release order | Partial startup/missing provider/storage worker not joined |
| IC3-SYNC-001 | Real TLS channel and scoped peer permission | Foreign endpoint/node/exporter/nonce/scope/tuple; revoked replica hold |
| IC3-SYNC-002 | Full immutable inventory includes rivals/candidates/receipts | Head-only mutant; changed pages/count/digest; partial/empty/latest erases old obligation |
| IC3-SYNC-003 | Partitioned nodes accept and heal autonomously to equal set/text | Disconnect every transfer phase; duplicate/gap/ref/fork; forbidden serve; no client ferry |
| IC3-SYNC-004 | Retained B resumes after unknown A; Y independent | Capacity/busy/backpressure/inbox persist failure; permanent loss survives restart |
| IC3-CUT-001 | Fresh continuous provenance reconstructible | Old sticky bool remains after retry/restart/empty inventory |
| IC3-CUT-002 | Exact full cut accounts every digest/request/custody | Wrong same-slot hash/omitted rival/proof/query; stale watermark/unissued terminal |
| IC3-CUT-003 | Combined complete after all obligations settle | Kernel-alone complete with pending inbox; untracked loss/new later observation |

Existing core105/API34, exact ten released text rows/canonical corpus MUST remain
unchanged and passing. Actual-node comparison checks canonical sets/evidence and
original receipts, not string equality alone. Meaningful mutants must be rejected
at real consumer seams for domain bypass, uncoupled terminal/custody, forgotten
obligation/head-only inventory and holder dispatch. Record actual RED before each
behavior fix, compiler errors separately, commands/tuple/timings/source hashes.

## 9. Delivery and remaining gates

IC-3A first freezes this semantic packet for fresh peer-blind Consistency/Safety.
Then actual types/data extraction/structural codec/refusing adapters and compiling
behavioral RED consumers get fresh Code/State on one settled tuple. Semantic GO
alone does not approve a typed ownership/wire interface or real adapter.
IC-3B implements genuine evidence and physical host after those gates, test-first,
with reusable trait conformance plus actual process-kill/reopen fault evidence and
dual Code/State components. IC-3C integrates actual node scope/dispatch/Iroh duplex
and two-process partition/restart/heal; aggregate Consistency/Safety and Code/State
review exact call graph/evidence. Material interface/architecture/wire/ownership
changes require fresh axes. Architectural roots/remediation caps never reset by
renaming. Surface review is required for any actual operator/user-facing CLI,
settings/config file, API/protocol or product promise being introduced/frozen,
even if called private qualification; naming it private is not an exemption.
Browser-facing freeze remains IC-4 unless this tranche introduces one. IC-4 owns
client/Glial intent, compatibility, enrollment,
migration/activation/readiness. No push/desk restart/live keys/stores/launch defaults/
seal/profile switch is authorized in this design lane.
