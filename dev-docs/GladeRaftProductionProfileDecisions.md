# Glade Raft first production profile — decisions for the owner

Date: 2026-10-03. Status: **proposal; no owner ratification or activation**.
This packet makes the remaining decisions in [Q4](GladeRaftProductionIntegrationPlan.md)
concrete. Raft is already the algorithm direction. Carrier selection follows the
separate comparison; these recommendations do not select a crate.

## Recommended semantic profile

| Decision | Recommendation and observable consequence | Approval/evidence still needed |
| --- | --- | --- |
| Deployment and durable writes | Three authenticated complete-data voters on independent machines/storage domains. A successful mutation MUST have its complete command durably retained by a majority and its exact terminal outcome retained after ordered application under the named process-crash profile. One lost voter permits progress only while a remaining majority can meet that same promise. No metadata witness counts. | Owner selects actual machines and custody; real failure-domain and adapter witnesses. Three local processes are only a development fixture. The two-node development launcher remains unchanged. |
| Persistence profile | APFS process-crash recovery first, with real file/directory synchronization and coherent log/application/outcome reconstruction. MUST declare this on the receipt profile; no power-loss or malicious-storage promise follows. | Concrete production adapter, filesystems and measured independent-host tests. Q2/Q3 private storage does not qualify a production adapter by import. |
| Authority and login | Key-signed user/device sessions first, genuine Ed25519 signatures and device-to-user-root validation. The creator retains a single exact signed genesis intent before sending it; voters authenticate its scope/declaration/group/configuration binding. Private `self` MUST derive from that authenticated user root at every serving/forwarding hop. | Owner chooses root issuance, key custody and recovery. Exact certificate/genesis/command schemas and signature purposes need canonical amendments, vectors and consumer review. Browser URL principals and forwarding-node possession do not qualify. |
| Authorization and ordering | Keep creation-rooted ancestry and separately granted host/voter/requester rights. Validate signed governance before deterministic application; order applicable governance and commands through one identified evidence frontier. Initial fixtures SHOULD use grants without time caveats; unsupported caveats MUST refuse, never be discarded. | Ratify the restricted first profile and its exact governed frontier. Future expiry/freshness semantics need explicit authenticated time inputs and tests. Consensus voters cannot issue permission by majority. |
| Reads | Offer permitted cached reads with an explicit application frontier. A read claiming current authoritative state MUST use a qualified quorum-confirmed log barrier and wait for local application through it. No lease-based read authority. Every serving hop MUST check current locally known disclosure policy and state its unseen-revocation limit. | Owner decides whether the first released profile requires the stronger read service. Its API, quorum-loss behavior and revocation frontier require review; old subscription heads remain local observations. |
| Retention | Keep exact accepted/refused outcomes, retirement, root/configuration and necessary policy evidence through snapshots and compaction. No age-based safety-history eviction. Set explicit capacity limits from measured workloads; reject new work when preservation would exceed them. | Owner selects supported retention/capacity expectations after measurement. No numerical capacity or performance guarantee is asserted here. Outcome recovery cannot silently become a new request when capacity is exhausted. |
| First application | One complete canonical `value` binding, with Gyld appearance as the affected consumer. Preserve `(share, glade_id, key)`, exact declaration/version/parameters and all payload bytes. Dynamic resources/scopes and authenticated discovery remain objectives; the first known group is an integration fixture. | Exact declaration/profile and root mapping review. `ws-razel`, a browser origin and a label do not authenticate an existing scope or make a new home. |
| Migration | Drain and reconcile all relevant legacy copies; preserve complete verified history and pending/unknown work. Close every legacy writer and old-binary/rollback path before activation. A seal is one preparation interlock, not sufficient exclusion. | Owner names the actual baseline/cut and supported rollback policy after reconciliation. No automatic cutover, seal installation or reset. External effects remain excluded until separate sink qualification. |

The [adoption contract](GladeRaftAdoptionContract.md) RA-001–012 remains intact.
The first implementation MUST still represent Create, Mutate, BeginMove,
Activate and Retire and complete retry/policy/configuration/snapshot history.
One shape or known initial group is not permission to omit these invariants.

## What each alternative changes

Two complete voters require both to accept new writes; loss of either stops
progress. A witness can help a placement decision but cannot replace missing
application data or meet this proposed complete-data receipt. Three local nodes
are suitable for deterministic and local process-interruption qualification,
with no claim of independent failure-domain resilience.

Operator-vouched borrowed-browser sessions are an existing product direction,
but need user-owned allowed-authentication policy and attenuation before this
protected path can admit them. Key-signed sessions first limits the initial
integration; it MUST NOT silently redefine that wider direction. Root key loss
does not authorize replacement genesis for the old identity.

Cached-only reads can be an explicit narrower first service if the owner chooses
it. They MUST NOT appear as current group state after a partition. A stronger
read barrier improves the claim that a successful settings refresh observes the
ordered state; it cannot reveal governance that has not entered the declared
frontier. Authorization freshness and data freshness are separate obligations.

## Ratification and implementation boundary

[QualificationPlan](GladeRaftQualificationPlan.md) Q4 requires
"Review/ratify exact canonical amendments and production profile" before
production integration/activation acceptance. [BuildEntry](GladeBuildEntry.md)
requires compiling consumer/conformance specifications and adversarial review
before implementing new/recomposed ports. This packet is a decision proposal,
not either of those completed gates or a wire freeze.

Owner selection of this semantic profile authorizes preparing its exact canonical
amendments and consumer contract. That next object MUST enumerate superseded
clauses, revise the Gyld Records/Admission/Profile/Delivery/NodeAssembly allocation,
generate versioned protocol bytes from Taut, test both clients, and pass applicable
Consistency/Safety/Surface reviews before implementation. Existing unactivated
identities retain their current contract. Actual hosts, secrets/custody, capacity,
legacy cut and deployment approval MUST remain explicit later inputs; silence
does not supply them.

Independent work can continue meanwhile: Q4-A seal qualification and Q4-B carrier
comparison contracts/experiments. Neither activates or changes the desk.

## Source basis

- [Production integration plan](GladeRaftProductionIntegrationPlan.md), ordered gates and proposed profile.
- [Adoption contract](GladeRaftAdoptionContract.md), §§1–4 and RA-001–012.
- [Authorization model](glade/GladeAuthzModel.md), §§3a/3b/4a/7a/7b: creation roots, ancestry, authenticated private self, operator placement and session strength.
- [Boundary audit](GladeRaftQ4-BoundaryAudit.md), canonical settings, actual writers, existing authentication and receipt gaps. This is a fact map with declared live-source limitations, not acceptance evidence.
- [Carrier audit](GladeRaftQ4-CarrierAudit.md), unqualified instance entropy/time seams and application-owned outcomes.

