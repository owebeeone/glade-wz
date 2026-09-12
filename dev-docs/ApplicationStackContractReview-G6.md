# Declarative Application Stack — Contract and Boundary Review

Evidence snapshot: 2026-09-05, Australia/Sydney. Independent GPT-6 Astra review. This is architecture advice for owner discussion, not an approved design. All requirements introduced here are **unratified reviewer recommendations**. Existing rulings are identified separately. The companion is [Owner Decision Worksheet](ApplicationStackContractDecisions-G6.md).

## Architectural finding

The smallest coherent contract is a **versioned surface definition, resolved binding, and independently authorized serving advertisement**, with two adapter families underneath: source execution and replica storage. The surface gives consumers a stable vocabulary; it does not make sources interchangeable or confer permission to modify them. Glade should enforce distributed address, authority, and access contracts; Glial should assemble and retain client state; Taut should define delivery semantics; each supplier should translate its particular source into those semantics.

**ASCR-F01 — refine the supplier/Tap analogy.** A Tap is a producer inside Grip's graph and a consumer of a distributed surface. A supplier is the authority-side counterpart, but need not implement Grip lifecycle, matching, or graph execution. Their useful commonality is declaration, capability, and lifecycle visibility—not a universal bidirectional `get/set/subscribe` interface. Existing Tap share hooks and the supplier kit already demonstrate different roles.[^tap][^supplier]

The owner's hypothesis is therefore substantially useful, with one correction: Glade owns the **validated distributed binding to a source**, while the supplier owns the physical source binding and execution. A database connection, filesystem root, or PTY descriptor must not become substrate configuration. The supplier contract must also state when replication represents original authoritative operations and when it represents a rebuildable projection.

```text
Authored application composition ──authorized record appends──▶ Glade registry fold
       │ surface contract                                      │ bindings/grants/claims
       ▼                                                       ▼
Grip Tap ◀── Glial assembly + client store ◀── Glade delivery ◀── supplier adapter
                    │                               │                │
             canonical Taut engine            replica store    source execution
                                                                   │
                                                          Garns / files / PTY
```

## What is already decided

The root decision log is itself a working document, but individual entries have explicit statuses. GDL-031 through GDL-038 are ratified on 2026-07-07: operator trust and placement; three-layer discovery; user-owned authentication policy; creation-root ownership; Glial as the local-first client kernel; RegistryApi/StoreApi seams; app-agnostic Glade and record-based declarations; ordinary management bindings. GDL-040 is **ruled** supplier vocabulary. GDL-041 is **ratified 2026-08-28**, adopting the canonical shape catalogue. GDL-039 remains **open, implemented pending ratification**. Its accompanying “uncommitted” code note is historical; current member status must be obtained separately.[^gdl]

GLP-0006's worksheet says recommendations await acceptance, and sections II/IV retain recommendation headings. However, the later plan decision explicitly records owner ratification of **all sections I–VIII**, including file authority, compare-and-replace save, full mutable windows, and retention. These are accepted plan-conversion rulings, not still-open choices merely because an older heading says otherwise. GDL-041 subsequently supersedes the old shape categories while preserving file and editing guarantees. This review does not erase that chronology.[^rulings]

**ASCR-F02 — record the two genuine compatibility decisions.** Formal domain/zone vocabulary still needs the GDL-039 ruling. Changes to SWMR writer turnover or CRDT retention need versioned shape/adapter decisions; they cannot be hidden in a new generic supplier interface. Existing GLP file-at-rest authority and terminal scrollback decomposition stand unless explicitly amended. Prior assistant proposals for a terminal spool or different file authority are not accepted by implication.

## Vocabulary and the single declaration authority

| Term | Meaning and owner |
|---|---|
| Surface definition | Stable identity, payload/key schema references, delivery engine/profile, operations and recovery contract. Shared leaf vocabulary in `glade-decl`; application owns its particular definition. |
| Binding instance | A definition resolved to a concrete domain, partition, source and policy references. Application composition authors it; Glade validates distributed records; Glial mounts it locally. |
| Supplier | Module standing behind declared surfaces, statically composed or demand-instantiated; owns source adaptation. GDL-040, not a synonym for any storage plugin. |
| Provider advertisement | Expiring statement of current serving ability. Glade owns validation/routing; publisher supplies evidence. Distinct from Grok enumeration. |
| Replica | Holder of accepted operations/checkpoints with explicit retained coverage. Read service does not imply source ownership or command authority. |
| Source | Semantic truth being represented: shared operations, an authority database, saved files, or a live process. Application/supplier identifies it. |
| Store | Physical persistence implementation. Owned by source runtime, Glade, or Glial according to the data stored; never an authority merely because it has bytes. |
| Domain / zone | Replicated world / convergence partition inside it; proposed mapping `share` / `key`. Glade addresses and enforces them. |
| Authority | Right to originate or perform a particular effect, plus the resource's conflict/fencing discipline. “External/share” describes provenance, not a complete authorization. |
| Origin | Operation writer/chain identity. Related to an authenticated principal through verified attribution; not interchangeable with principal, device, tab, source or physical store. |

**ASCR-F03 — split meanings without creating another configuration authority.** `glade-decl` already contains `BindingDecl(glade_id, shape, authority, source?, domain, zone, retention)` and an `AdvertisementRecord(binding, package, grip_key)`. The latter is **Grok enumeration metadata**, not provider discovery. The node has a different `BindingDecl(app, glade_id, shape, authority, zone, retention)` with strings and no equivalent source/domain fields. Glial's `Fill(domain, zone?, key?)` creates the concrete instance key. These are related but not identical contracts.[^decl][^records][^binder]

Use the proposed three-part distinction as a normalization of these existing declarations and records. Do not introduce an independently editable catalog that competes with `.glade` or the registry fold. Static files compile to authorized record appends; runtime declarations use the same path. Generated artifacts must preserve source ownership, identity and revocation semantics.

Illustrative data sketches—not a new grammar:

```text
Surface: id, contractVersion, payloadSchema, parameterSchema,
         engine + optional profile + adapterVersion,
         operations, recoveryContract, optional viewContract
Binding: surfaceRef, concreteDomain, partitionExpression, canonicalParameters,
         sourceRef, authorityPolicyRef, retentionPolicyRef
Advertisement: bindingRef, providerIdentity, role, capabilityVersions,
               authorityEvidenceRef, term, validity, coverage, routeRef
```

A definition may constrain allowed authority/retention choices; the instance resolves them. The same ID cannot mean writable shared text at one provider and a read-only database projection at another without an explicit contract distinction. Binding identity must cover semantic parameters, source generation and policy partition, not local socket or database filename. Scope resolution must not silently alias two source instances into one Glial store.

Authored facts are application surfaces, Garns worlds/questions, source mappings and policy intent. Generate codecs, typed handles, schema compatibility checks, canonical key functions and registration records. Runtime facts are concrete bindings, grants/revocations, claims, provider terms, observed coverage, cursors and command results. Dynamic runtime authority remains the validated fold, never a regenerated ACL seed file.[^declsurface]

## Two adapter contracts, with shape-specific recovery

**ASCR-F04 — separate source adaptation from replica persistence.** A source adapter executes reads, watches changes and accepts authorized commands. A replica store durably retains already accepted delivery state and proves what can be recovered. They may share codecs, lifecycle vocabulary and checkpoint envelopes, but accepting replicated data into a store does not invoke a source write.

```text
SourceAdapter.open(binding, authenticatedContext) -> session | typed refusal
session.read / subscribe(parameters, shapeCursor?) -> shape outcomes
session.command(operation, input, expectedRevision?, idempotencyKey, authorityTerm)
    -> rejected | committed(result, sourceRevision) | outcomeUnknown
session.close(reason) -> closed; cancellation semantics are explicit

ReplicaStore.open(bindingIdentity, exactAdapterContract) -> retained state
accept(validatedBatch) -> durableReceipt | duplicate | conflict | storageFailure
restore() -> checkpoint + retained operations + verified heads + pending intents
resume(cursor) -> retained coverage | shape-specific reset/bootstrap/expiry
checkpoint / collect(retentionPolicy, proof) -> durable boundary | refusal
```

These are capability families, not mandatory methods with no-op implementations. Unsupported writes, resume, or persistence MUST fail before effects. A read-only supplier need not implement commands. A live channel may explicitly have no restore. Read subscription cancellation must release interest even if an underlying conformance implementation lacks unsubscribe.

| Delivery contract | What a conforming adapter must preserve |
|---|---|
| `value`, `log` | Exact value/log semantics; a log cursor is not an arbitrary array index. Retention must expose missing history. |
| `atom`, `stream` | Their canonical delivery/lifecycle semantics; recognition alone does not enable Glade durable mounts. A terminal channel is not automatically Taut `stream`. |
| `swmr` | Single writer identity, snapshot baseline, delta sequence and reset epoch; repair stays in-band under the canonical contract. |
| `snapshot_delta` | SWMR profile with out-of-band `refresh_required`; consumer restarts without a cursor. It must not quietly receive ordinary SWMR repair. |
| `crdt` / `text_crdt` | Causal vector, per-origin identity, pending dependencies, bootstrap floor; text profile additionally owns merge and cursor identities. No generic last-write-wins fold. |
| `unary`, Glade `exchange`, application `window` | Interaction, correlated service path, and view respectively. None is a delivery engine; `window` names a base engine and generation policy. |

The adopted catalogue and adapter gates require exact version/corpus coverage before registration. Current SWMR adapter v1 exposes snapshot/delta/reset only and permits one origin for the entire zone-surface. Changing provider identity to a new origin cannot therefore be a transparent SWMR takeover. Either retain a securely transferred source writer identity, or explicitly introduce a new binding/generation or versioned handoff adapter. Do not bypass GSA-03 with a lease update.[^shapes][^swmr]

CRDT v1 retains every post-bootstrap operation; its canonical decision defers compaction to a new contract version. Glade checkpoint ideas cannot automatically authorize CRDT tail pruning. Snapshot state plus a cursor is sufficient only where the exact shape and trust contract say it is.[^crdt]

**ASCR-F05 — durability needs an observable acceptance level.** Glial currently appends synchronously to memory, then queues IndexedDB writes whose errors are swallowed. A successful local append is not proof of crash durability. Its store exposes `append/all`, not checkpoint receipts or recovery floors. Keep this useful seam, but explicitly distinguish memory acceptance, durable acceptance, remote acceptance and source commit before exposing “saved” or safely discarding offline work.[^store]

## Advertising, routing and security

**ASCR-F06 — advertise roles separately.** One host may perform several roles, but clients must not infer one from another.

| Role | Minimum role-specific claim |
|---|---|
| Source authority | Source/binding reference, authorized principal, source generation, current term, supported production semantics and source fence evidence. |
| Retained-data replica | Exact adapter/profile, heads or checkpoint coverage, earliest recoverable position, freshness observation, read/placement authorization. |
| Command provider | Explicit operations, provider class, authenticated identity, authorized attachment epoch, source fence and retry/result contract. |
| Supplier host | Implementable supplier/contract versions, permitted placement class and instantiation capability. Capacity is a hint, not a live provider claim. |

Common information includes a versioned advertisement identity, binding reference, expiry/withdrawal ordering and resolvable route. Keep credentials, absolute roots, database paths, environment, process handles and signing keys private. Advertising existence may itself reveal sensitive data; discovery views need authorized visibility, particularly for account/private bindings.

Advertisements MUST NOT grant their own authority. A receiving node validates schemas, signatures, declaration compatibility, granted role, scope and term before using them; the effect authority rechecks at execution. Replicas check read entitlement at delivery, including after policy changes. Epoch fencing must reach the source itself: a current route does not stop a partitioned old process executing SQL or writing a file. A local lock protects one resource where all actors share that lock; it does not fence independent database authorities.[^auth][^directory]

Withdrawal and newer terms supersede older advertisements. Expiry is evaluated against explicit read-time clock assumptions, not baked into the deterministic fold. A disconnected replica can describe stale retained state but cannot claim fresh source truth. Where authority or policy freshness is uncertain, effect routing MUST refuse or return uncertainty; it must not select the last reachable claimant as an implicit promotion. Conflicting live claims require explicit conflict handling.

Keep GDL-032's layers: placement selects a session host; service discovery resolves authorized capabilities/bindings; node discovery resolves transport reachability. The discovery component has its own pure state machine and injected host boundary, while the inspected running-node carrier is direct-address localhost iroh with discovery disabled. Neither observation warrants redesigning discovery here.[^discovery]

The current exchange provider map silently inserts by `(share, glade_id)`; peer and origin verification are explicitly stubbed; generated system records still use legacy infallible decode. These are implementation gaps against B1–B5, not permissions granted by the model. A declaration record or successful demo request is insufficient evidence of source authority.[^securitycode]

## Domains, persistence classes and invalid combinations

**ASCR-F07 — treat identity mappings as contracts.** A Glade domain is not Garns relational scope. Garns scope follows declared relation paths and supplies the engine's `_scope` predicate; `scope deployment` removes row partitioning. The integration must map an authorized Glade binding to a specific Garns world/scope and query parameters. A parameter named `workspace` is not itself proof of membership. Reads authorize before engine execution and again at delivery; commands authorize the verb, source rows and fence before commit.[^garnsScope]

Zone `self` resolves from the authenticated principal, not a caller-provided user string. AZ-16 membership governs delivery of both commons and the member's private zone in that domain. Device and tab identify sessions/writers; account identity persists beyond them. Origin chains preserve operation attribution, not relational ownership. Physical storage IDs only locate bytes. Path/range/query parameters select source data or views and must not accidentally widen the security partition.[^auth]

The five proposed dimensions are a useful checklist, **not a ratified model**:

| Dimension | Dependency or invalid combination |
|---|---|
| Authority/source | A writable replica of an external query cannot imply database command authority. |
| Domain/zone | Two scopes cannot share one keyed materialization if they have different data or reader sets. |
| Shape/fold | A profile/view cannot supply a missing engine; SWMR provider turnover cannot introduce a second writer. |
| Retention | Pruning CRDT v1 post-bootstrap history violates its contract; TTL cannot discard the only accepted offline write without an explicit failure. |
| Physical store | A database row projection cannot replace a Glade operation journal unless it preserves its op, proof, head and recovery semantics. |

Add **contract/version, authorization/placement, lifecycle, commit acknowledgement, and recovery provenance**. These are not all independent axes: retention follows recoverability, capabilities follow exact versions, and placement follows grants.

| Data class | Recovery obligation and deletion meaning |
|---|---|
| Authoritative source data | Recover committed truth and command outcomes; semantic delete is a source transaction, not cache eviction. |
| Replicated state | Preserve accepted operations, causal identity and allowed checkpoints. Removal must not cause deleted values or revoked grants to resurrect. |
| Recoverable projection | Rebuild from named source generation; expose stale/unavailable/reset if that source or coverage is gone. |
| Supplier-private state | Credentials, cursors, dedup/outbox metadata remain private but may be essential to correct recovery. “Undeclared” does not mean disposable. |
| Cache | Evict only when recovery remains truthful; expose a cold miss or explicit gap. |
| Live process state | A dead PTY cannot be restored from its transcript. Persisted metadata must transition to lost/closed/unknown, not invent a running process. |

Deletion, retention pressure and revocation are different events. Revocation prevents subsequent authorized delivery/effects; it cannot retract bytes already copied. Local purge may satisfy device policy but cannot guarantee remote erasure. Under pressure, pin irreplaceable accepted work, refuse further durable acceptance when required, and report any permitted loss. Offline source commands are pending intents, not committed source changes; revalidate membership, base revision and authority on reconnect.

## Garns and declaration composition

**ASCR-F08 — Garns belongs behind a relational supplier, not underneath every surface.** The current Garns repository records provenance from the ratified v9-5 B2 build. Its Python engine provides qualified reads, scoped live questions, governed transactions and external capture. This ratification does not turn its generated helpers into a production service or its new Rust crates into an execution runtime. The Rust workspace presently recognizes artifact IDs and diagnostic stages; portable IR/results still need independent schemas. Grazel instead depends on the older generated **P8** store and opens an application database into an unused binding.[^garns][^p8]

Three valid uses differ materially. A Garns authority store accepts governed row changes via a command supplier. Captured external relational data is observed through declared changelog capture; capture does not grant write control of the external writer. A Garns-backed projection of Glade state can support queries if it consumes validated operations and remembers its rebuild frontier. It must not become the authoritative ACL/config database. A Garns-backed Glade replica store is possible only as an implementation of Glade's op-store semantics, not by substituting Garns ledger rows for Glade ops.

Prefer explicit **integration mappings** first: surface reference → qualified question/command adapter → world/source binding → scope/parameter mapping → payload and recovery contract. Garns owns relational identities, fields, constraints, SQL and footprints; Taut owns delivery; Glade owns distributed address/policy; the application owns the cross-system mapping. Neither language needs to generate the other wholesale. Generation may later emit the repetitive mapping artifacts from versioned Garns descriptors plus authored Glade policy, with drift checks. A query descriptor does not contain enough information to infer domain ownership, reader sets or offline command policy.[^garnsContracts]

**ASCR-F09 — commit-to-publication is a durable boundary.** Garns executes `COMMIT` before invoking listeners. A crash, publish error or listener exception may follow a successful source transaction. Retrying the command blindly may duplicate effects; replaying only in-memory live batches may miss the committed change. Garns `transaction_id` is metadata here, not demonstrated end-to-end idempotency.[^commit]

Recommended contract: atomically record command identity, outcome and a publishable source revision/outbox intent with authority mutation; retry publication with deterministic event identity; retain the result for retry lookup. For a rebuildable question, a fresh snapshot at a new declared generation may recover delivery without replaying every transient question batch. Preserve event history only where promised. Do not promise exactly-once network delivery; promise an idempotent source effect under a defined key and replay-safe publication. External effects outside a Garns transaction need a separately recoverable intent/result protocol or an honest unknown outcome.

```text
authorized command → source transaction [rows + result/idempotency + publication intent]
                                      COMMIT
                                        ↓ crash can occur here
                         replayable publisher → Glade accepted ops → Glial assembly
```

The **Garns ledger** records relational deltas/revisions; the **Glade journal** records attributed, chained operations and causal references; the **shape cursor** identifies a consumer's engine-specific progress. Live Garns batches have instance-local sequence and memory-resident history. None of these three is interchangeable; the adapter must record explicit mappings or declare reset/rebuild.[^commit][^live]

## Worked journeys

**ASCR-F10 — the three journeys need the same boundary vocabulary and different outcomes.** The following are recommended target journeys constrained by existing rulings, not claims that all steps run today.

### Scoped Garns question and command

Declare a surface for `sales.open_orders`, its result key/payload and SWMR-based delivery; separately declare an order-change command. Compose a binding mapping domain A to Garns scope 17. Append authorized declaration/binding records; advertise a read producer and command provider with distinct grants. Authenticate the consumer, resolve scope on the authority side, execute the question through `Engine`, and publish its snapshot plus mapped batches. Glial assembles the selected engine and persists accepted delivery state; Grip consumes it without SQL knowledge.

A command carries an idempotency key and relevant row/base precondition. The supplier checks context and source fence, commits rows/result/publication intent, and returns source outcome independently of whether every subscriber has observed it. Garns live-bound refusal becomes an explicit error, never truncated success.

On client disconnect, retained data is marked stale and queued commands remain pending. On restart, source rows survive; live instance IDs/batch logs do not, so recover from committed revision/outbox or issue an explicit fresh baseline. A source schema/scope change invalidates incompatible bindings rather than reusing their cursor. Authority turnover requires source fencing and the explicit SWMR identity/generation decision. Revocation stops delivery and new execution, including queued commands; a previously committed result is handled under current result-read policy. Source rows are recoverable; unrecorded transient batches may be lost without violating a snapshot-rebuild contract.[^live]

### Filesystem, collaborative editing and save

Declare directory views, path/revision file views over SWMR, `text_crdt` editing, editing presence and save exchange. Resolve the workspace to a private host root; normalize and safe-open a root-relative path. The supplier advertises file read/save authority; clients receive authorized generation-coherent bytes. Range selection changes viewport, not source authority. Opening an edit session captures the saved base revision; Glial assembles collaborative operations and cursor identities independently of the saved-file view.

Save submits the edited result against that base revision and the current fence. The file authority compares and replaces or reports conflict; it does not replay arbitrary projection deltas into the filesystem. The at-rest file remains truth for saved content while the explicit editing generation represents unsaved shared work, per D12/D13.

After client restart, retained CRDT state can resume; after supplier restart, reopen the saved file and reconcile the editing generation/base. External file changes create a new revision and stale-base conflict, never silently overwrite edits. Disconnect permits declared offline collaborative operations, not a claim that the file was saved. Authority turnover requires access to the same resource and fencing; another clone is not necessarily the same source. Revocation cuts future file/blob delivery, edit admission and save; existing private copies remain. Saved bytes survive according to filesystem durability; accepted unsaved operations survive only promised replica retention. Lost editing history cannot be repaired by pretending the last saved file included it.[^rulings][^swmr]

### Terminal creation, input, output and reconnect

Declare broker exchange, owner-private session index, live driver channel and retained scrollback log. Authorize creation and shell execution, select the workspace root privately, create the PTY, and return its opaque session ID. Advertise the actual live provider with attachment/driver epoch; possession of the ID grants nothing. Validate driver epoch on input. Output carries `{generation, offset, bytes}` on both live and retained paths; Glial uses those identities to avoid duplicate rendering on reconnect. Scrollback read authority is distinct from input authority.[^terminal]

Client disconnect can detach while the process remains alive under policy; reconnect reauthorizes attachment and replays from known coverage before joining live output. Client restart rebuilds display from retained data. Supplier/process restart may leave only the transcript; report process loss unless a separately specified supervisor preserved the actual PTY. Starting a new shell is a new generation, never replaying old input. Authority turnover may transfer a driver slot for a surviving process, but cannot move process state just by transferring a claim. Revocation closes the affected input/read paths. Shell exit seals its output; retention pressure records the new floor/gap under the accepted policy. Source output not durably captured before a crash may be lost; the publication/durability contract must say where acceptance occurred. A private spool is one possible future implementation, not a ratified replacement for scrollback replication.

### Boundary checks beyond workspace commons

Account-scoped editor preferences use an account binding keyed to authenticated identity, not tab origin or current workspace. Glial can retain them offline and later replicate under account policy. Private workspace selection instead remains tenant data: AZ-16 workspace membership revocation cuts future service even though it is “mine.” Account recovery and writer-origin continuity need separate identity handling.

A compact control-plane projection might expose the current workspace host and lease expiry from ordinary registry records. Management writes append authorized grant/claim records; Garns may index their fold but cannot replace it. On restart, recover verified records and evaluate lease expiry at read time. Compaction must retain proof of revocation/terms, or rebuild from an authorized checkpoint; discarding historical grant removals can revive access. The current renewal loop appends claims and persists the directory; neither a small UI projection nor a SQLite index proves bounded authoritative history.[^directory][^claims]

## Alternatives, disposition and the next proof

| Arrangement | Benefit | Counterexample / cost |
|---|---|---|
| One universal bidirectional surface/store adapter | Very small consumer API; easy generic generation | Query projection cannot determine a lawful SQL update; PTY restore cannot recreate a process; shape-specific cursors disappear behind misleading uniformity. |
| **Shared declaration, separate source and replica adapters** | Preserves one consumer contract and clear authority; independently testable recovery | More explicit composition and capability checking; source-specific logic remains authored. Recommended. |
| Garns-centered generation of the application stack | Strong relational schema reuse | Cannot infer file save conflicts, terminal lifetime, or Glade grants from a relational world. Appropriate for relational subsets, not the root contract. |

**ASCR-F11 — retain mechanisms, not incidental demo policy.**

| Disposition | Mechanism and evidence |
|---|---|
| Reusable | Typed leaf declarations, local-first binder/refcounts, exact shape preflight, SWMR generation checks, CRDT causal assembly; existing tests cover positive/negative cases.[^binder][^tests] |
| Adaptable | Op journal, heads/replay, IDB store, provider request correlation, claim registry: useful seams, but durable receipts, checkpoint policy, source fences and authenticated attachment need explicit completion.[^store][^securitycode] |
| Experimental | Whole-image first-4096-byte file demo; GWZ subprocess output appends with ignored append errors; P8 application store opened but not integrated into source execution.[^demo][^p8] |
| Absent in inspected integration | Production path-addressed filesystem supplier, PTY supplier, complete source-to-surface Garns integration and atomic source-commit/publication protocol. Negative search boundary below; this is not an estate-wide assertion. |

The earlier persistence investigation is useful evidence to challenge. Its file-demo, dropped-output, P8 and declaration/parser observations are supported by current inspection. Its suggestion that canonical `stream` is “exactly” terminal transport must be qualified by GDL-041's explicit adapter requirement. Its broad “nothing exists” language for auth/discovery is too strong: contracts, records, seams and a separate discovery implementation exist even where running-node enforcement/integration is incomplete. Its claim that filesystem adaptation is “entirely undesigned in code” should not erase the implemented SWMR boundary.[^prior]

**ASCR-F12 — smallest next proof:** one isolated Python Garns authority with a scoped live question, one idempotent command, a Glade replica and a Glial consumer. Use an explicit mapping and exact SWMR adapter, not a new DSL or a new Rust database runtime. Two scopes and two principals are necessary to test the central contract.

Observable acceptance criteria, proposed **ASCR-P01–P06**: P01, one declared surface serves two non-aliasing scoped instances with matching recomputed results; P02, wrong scope, forged context and unsupported version fail before registration/effects; P03, crash after source commit but before publication recovers one source effect and a query-consistent result; P04, duplicate command/publication retries do not duplicate effects or consumer changes; P05, expired resume and authority turnover produce the explicitly selected reset/refusal, never mixed generations or two writers; P06, revoked access stops subsequent delivery/commands and a storage failure cannot be reported as durable success. Existing GSC/GSA and B1–B5 conformance rows are dependencies. These are proof criteria, not an implementation backlog or authorization to build.

## Evidence and inspection limits

Actual HEADs read on 2026-09-05:

| Repository | Revision |
|---|---|
| glade-wz root | `e74a86af2749192ecad401ec0457e8d08604f785` |
| glade | `71509c1f701d10d7ca7086cdf8c635683793a050` |
| glial | `0dfe4b930063bb1b04c126c2b020e494290aa631` |
| grip-core | `97ff6c26f12e2808cd794d2947a852d7c16e4e22` |
| glade-decl | `bbce73d67146124780c3f5fc226d032618aa446c` |
| glade-discover | `65fc18bebc56562b9b43db5254b087e28f3ff3f9` |
| taut | `7a5f616c3a9f72e143b6e20dab41ffa6e20e240a` |
| taut-shape | `9a752094dbede65babcc6c44185f64cef4c6e974` |
| grazel | `a07af54e837b2243425d91c6b07869632369d606` |
| glade-gwz | `e53c87dddb8f40f050d3bd18cb77d96e21a73ed1` |
| garns-wz root | `7c706b11a40aa476d1fb545baf5e445649183086` |
| garns | `34f2f146a28a90d9ef4bfe9ef86dcc5f788275c6` |
| garns-rust | `7e8e31913a5da7defa18064d9d904a8fde0ae2fb` |

`gwz status` reported three pre-existing modified discovery documents (`GladeDiscoverPlan.md`, `P7ReviewDisposition.md`, `RequirementTrace.md`), two untracked discovery reviews/plans and root untracked architecture/review documents. Garns had only untracked `dev-docs/GarnsNextSteps-Rev1.md`. No inspected implementation member was reported dirty. Existing work was preserved. The two original unsuffixed application-stack reports were excluded from reading and content search; status revealed their filenames only. These G6 output paths did not exist before creation.

This was local source/document inspection, not a live-demo audit. Explicit member searches excluded `.git`, `target`, `node_modules` and `dist`; root searches were not treated as proof about nested repositories. PTY/watch negatives are bounded to `glade/node/src`, `glade-gwz/src`, `grazel/src`, `glial/src`, plus corresponding filename inventories; unrelated repositories and installed services were not searched. Discovery inspection was limited to its README, inventory and running-node carrier boundary, not its security correctness. Canonical adoption pin/version and relevant SWMR/profile/CRDT documents were read; no independent whole-corpus regeneration or hash attestation was performed.

Executed in `garns-rust`: `cargo fmt --check` and `cargo test --workspace --offline`; formatting passed, four unit tests passed, zero doctests. These only verify current artifact/stage recognition, not Garns execution or the proposed integration. Existing Glial/SWMR/IDB tests were inspected, not rerun. No demo, user store, implementation, canonical decision, plan or declaration was modified. Report verification checks source-link existence/line bounds, unique IDs, worksheet cross-references, and workspace status; there is no claim of executable conformance for the proposed architecture.

Final document checks passed: 76 source links resolve within file line bounds; all 12 finding IDs and 10 decision IDs are unique; worksheet finding references and review footnotes resolve. The review is approximately 4,800 words including evidence notes. Final `gwz status` showed only the two intended new reports in addition to the unchanged pre-existing work.

[^gdl]: [DecisionLog.md:49](/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md:49), entries GDL-031–041 through line 59.
[^rulings]: [Worksheet status rule:177](/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md:177); [later ratification:625](/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:625); [supplier requirements:5](/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/SupplierRequirements.md:5).
[^tap]: [Tap interface:42](/Users/owebeeone/limbo/glade-wz/grip-core/src/core/tap.ts:42).
[^supplier]: [supplier kit:35](/Users/owebeeone/limbo/glade-wz/glial/src/supplier/index.ts:35), source/controller at 100–116; [common model:16](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeSupplierModel.md:16).
[^decl]: [declaration schema:93](/Users/owebeeone/limbo/glade-wz/glade-decl/ir/glade_decl.taut.py:93).
[^records]: [node BindingDecl:118](/Users/owebeeone/limbo/glade-wz/glade/node/src/sysdata.rs:118); [parser:43](/Users/owebeeone/limbo/glade-wz/glade/node/src/appdecl.rs:43), excludes CRDT despite [dispatch inventory:14](/Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeShapeDispatch.md:14).
[^binder]: [binder:48](/Users/owebeeone/limbo/glade-wz/glial/src/binder.ts:48); [Fill and identity:30](/Users/owebeeone/limbo/glade-wz/glial/src/instance.ts:30).
[^declsurface]: [shared declaration contract:65](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclSurface.md:65); [file form:111](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclSurface.md:111).
[^shapes]: [adoption/version pin:5](/Users/owebeeone/limbo/glade-wz/dev-docs/TautShapeCatalogAdoption.md:5), capability gates at 48–90; [snapshot-delta decision:7](/Users/owebeeone/limbo/glade-wz/taut-shape/dev-docs/TautShapeSnapshotDeltaDecision.md:7).
[^swmr]: [SWMR adapter:9](/Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeSwmrAdapter.md:9); [node writer conflict:158](/Users/owebeeone/limbo/glade-wz/glade/node/src/store.rs:158).
[^crdt]: [CRDT contract:9](/Users/owebeeone/limbo/glade-wz/taut-shape/dev-docs/TautShapeCrdtDecision.md:9), bootstrap/retention at 76–91.
[^store]: [Glial store:23](/Users/owebeeone/limbo/glade-wz/glial/src/store.ts:23); [IDB:38](/Users/owebeeone/limbo/glade-wz/glial/src/store_idb.ts:38), retention at 96, swallowed errors at 124; [node journal:130](/Users/owebeeone/limbo/glade-wz/glade/node/src/store.rs:130).
[^auth]: [check sites:201](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeAuthzModel.md:201); [AZ-16/B4:345](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeAuthzModel.md:345); [zones caveat:64](/Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeZones.md:64).
[^directory]: [directory records/validation:47](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeWorkspaceDirectory.md:47); [source lock:115](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeWorkspaceDirectory.md:115); [store/registry seam:23](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeSystemDataSeam.md:23).
[^discovery]: [discovery boundary:3](/Users/owebeeone/limbo/glade-wz/glade-discover/README.md:3); [node carrier:30](/Users/owebeeone/limbo/glade-wz/glade/node/src/iroh_carrier.rs:30).
[^securitycode]: [attachment insertion:81](/Users/owebeeone/limbo/glade-wz/glade/node/src/exchange.rs:81); [peer verification:96](/Users/owebeeone/limbo/glade-wz/glade/node/src/peer.rs:96), origin stub at 140; [legacy codec:1](/Users/owebeeone/limbo/glade-wz/glade/node/src/sysdata.rs:1).
[^garnsScope]: [scope model:155](/Users/owebeeone/limbo/garns-wz/garns/ARCHITECTURE.md:155); [caller-asserted capabilities:61](/Users/owebeeone/limbo/garns-wz/garns/LIMITATIONS.md:61).
[^garns]: [ratified provenance:3](/Users/owebeeone/limbo/garns-wz/garns/PROVENANCE.md:3); [capabilities/limits:20](/Users/owebeeone/limbo/garns-wz/garns/README.md:20); [Rust boundary:9](/Users/owebeeone/limbo/garns-wz/garns-rust/README.md:9); [external capture:23](/Users/owebeeone/limbo/garns-wz/garns/src/garns/capture.py:23).
[^garnsContracts]: [portable contracts:7](/Users/owebeeone/limbo/garns-wz/garns/contracts/README.md:7); [proposed integration:353](/Users/owebeeone/limbo/garns-wz/garns/INTEGRATION.md:353); [artifact recognition:23](/Users/owebeeone/limbo/garns-wz/garns-rust/crates/garns-artifact/src/lib.rs:23).
[^p8]: [P8 dependency:26](/Users/owebeeone/limbo/glade-wz/grazel/Cargo.toml:26); [store opening:55](/Users/owebeeone/limbo/glade-wz/grazel/src/main.rs:55).
[^commit]: [Garns commit:217](/Users/owebeeone/limbo/garns-wz/garns/src/garns/engine.py:217).
[^live]: [Garns live instance:100](/Users/owebeeone/limbo/garns-wz/garns/src/garns/live.py:100), snapshot/refresh at 118–153 and scope registration at 176–202.
[^terminal]: [terminal decomposition:25](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/suppliers/glade-terminal.md:25), private process boundary/retention at 180–203; [lifecycle recovery distinctions:83](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeInstanceLifecycle.md:83).
[^claims]: [claim renewal:234](/Users/owebeeone/limbo/glade-wz/glade/node/src/claims.rs:234).
[^tests]: [SWMR tests:27](/Users/owebeeone/limbo/glade-wz/glial/test/swmr.test.ts:27); [IDB tests:35](/Users/owebeeone/limbo/glade-wz/glial/test/store_idb.test.ts:35); [file projection tests:8](/Users/owebeeone/limbo/glade-wz/glade/demo/test/files.test.ts:8).
[^demo]: [whole-image projection:6](/Users/owebeeone/limbo/glade-wz/glade/demo/src/files.ts:6); [GWZ output:173](/Users/owebeeone/limbo/glade-wz/glade-gwz/src/supplier.rs:173), ignored append at 244.
[^prior]: [prior persistence investigation:73](/Users/owebeeone/limbo/glade-wz/dev-docs/GladePersistenceReview.md:73), terminal inference at 117–127 and broad auth/discovery claims at 136–152. Used as a challenge source, not architectural authority.
