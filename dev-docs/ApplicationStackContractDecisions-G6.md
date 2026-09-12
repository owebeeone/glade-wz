# Declarative Application Stack — Owner Decision Worksheet

Date: 2026-09-05. Independent GPT-6 Astra review. Companion: [Contract and Boundary Review](ApplicationStackContractReview-G6.md). This local worksheet does not amend the canonical decision log, ratify proposals, or authorize implementation.

The first owner discussion should settle **D01–D04**: normalize the existing declaration authority, resolve domain/partition identity, separate source and replica roles, and define source fencing. D05–D07 then settle durable acknowledgement, recovery and source transaction publication. D08 selects the bounded Garns integration. D09 records existing rulings that do not need reopening; D10 selects proof scope. Numerical cache defaults and framework choices are not prerequisites.

## ASCR-D01 — One common declaration contract

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F01, ASCR-F03. Related rulings: GDL-035, GDL-037, GDL-040.

**Question:** Should surface definition, concrete binding and serving advertisement become distinct semantic records within the existing `glade-decl`/registry model?

**Why:** Current client `BindingDecl`, node `BindingDecl` and Grok `AdvertisementRecord` differ. Treating them as one runtime provider declaration would hide missing source/parameter/version facts and could establish competing configuration authorities.

**Alternatives:** Keep the existing combined declaration and document its contextual meanings; normalize its records and generate compatible projections; add an independent universal catalog.

**Recommendation:** Normalize existing ownership, retaining `.glade`/runtime appends as inputs to the validated fold. Surface identity, payload/key schema, operations and exact engine/profile/version MUST have one semantic owner. Instance resolution MUST reference that definition; provider advertisements MUST remain current observations. Compatibility projections MAY preserve existing serialized forms during a versioned migration. This adds composition detail but avoids a second authority.

**Affected contracts:** `glade-decl`, node registry declarations, `.glade`, typed manifests, Glial mount and supplier kit. **Dependencies:** None; preserve GDL-037 revocation-aware record authority.

**Validation:** ASCR-P01/P02; compile one authored surface for consumer and supplier, reject conflicting versions, re-register unchanged seeds without restoring revoked grants. Evidence: [declaration schema:93](/Users/owebeeone/limbo/glade-wz/glade-decl/ir/glade_decl.taut.py:93), [node record:118](/Users/owebeeone/limbo/glade-wz/glade/node/src/sysdata.rs:118).

## ASCR-D02 — Domain, partition and source identity

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F02, ASCR-F07. Related rulings: GDL-039 still open; AZ-16/B4 already ruled.

**Question:** What exact binding identity and canonical mapping distinguish a domain/zone, a source instance, Garns scope and query/path/view parameters?

**Why:** A namespace string or UI parameter is not authorization. Aliasing two scopes into one client store leaks or corrupts materialized state. Account state must outlive a tab without confusing principal and origin.

**Alternatives:** Adopt domain/zone over existing wire fields plus explicit source/parameter mapping; overload one key with all meanings; replace wire vocabulary and addresses immediately.

**Recommendation:** Ratify or amend GDL-039 explicitly. Preserve existing wire mapping where compatible, but author a canonical identity mapping covering semantic parameters and source incarnation. `self` MUST resolve from authenticated identity under existing B4; scope mapping MUST be authority-side and explicit. View range SHOULD remain separate from source generation where one coherent object supports multiple windows. This preserves interoperability while requiring collision/version rules.

**Affected contracts:** DomainAnchor/ZoneKind/Fill, canonical keys, Garns adapter, file view identity, private zones. **Dependencies:** D01; B4 is a constraint, not a new approval request.

**Validation:** Two scopes with identical local row IDs produce distinct instances; private-key spoofing fails across subscribe, append, replay and forwarding; account preferences survive a new tab identity. ASCR-P01/P02. Evidence: [GDL-039:59](/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md:59), [Fill:34](/Users/owebeeone/limbo/glade-wz/glial/src/instance.ts:34), [Garns scope:155](/Users/owebeeone/limbo/garns-wz/garns/ARCHITECTURE.md:155).

## ASCR-D03 — Source adapter versus replica store

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F01, ASCR-F04, ASCR-F08. Related rulings: GDL-035/036/040/041.

**Question:** Should the common contract define distinct source-execution and replica-persistence capabilities, sharing only definitions and shape contracts?

**Why:** A query replica cannot infer an SQL mutation, nor can a terminal transcript restore a process. A universal bidirectional interface would falsely broaden authority.

**Alternatives:** One generic read/write/store interface; separate source and replica adapters; per-supplier bespoke contracts without common capability declarations.

**Recommendation:** Separate adapter families. Source adapters MUST declare reads, subscriptions, commands and unsupported operations. Replica stores MUST preserve accepted delivery identity, durable recovery and coverage without invoking source mutations. Taut MUST remain the semantic engine owner; Glial owns client assembly. Extra interface types buy testable authority boundaries.

**Affected contracts:** Supplier kit, Glial store, Glade node store, Taut adapter corpus, Garns integration. **Dependencies:** D01/D02.

**Validation:** Installing a replicated Garns query snapshot never modifies source rows; a read-only provider refuses commands; replica backend replacement preserves op/recovery transcripts. ASCR-P02/P06. Evidence: [store seam:28](/Users/owebeeone/limbo/glade-wz/glial/src/store.ts:28), [GDL-036 seam:23](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeSystemDataSeam.md:23).

## ASCR-D04 — Advertising roles and fencing the source

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F04, ASCR-F06. Related rulings: GDL-031/032, B1–B5, GSA-03.

**Question:** What source fence and writer-identity rule must accompany provider attachment/turnover, and how do advertisements distinguish authority, replica, command provider and supplier host?

**Why:** Advertisement expiry can change routing without stopping an old authority from executing effects. Current SWMR accepts one origin; a new provider origin fails its contract even with a higher lease epoch.

**Alternatives:** Treat live claim as sufficient authority; require source-enforced fencing with explicit role claims; forbid turnover for initial suppliers.

**Recommendation:** Role-specific advertisements MUST reference independently validated authority, exact capability versions, term, validity and relevant retained coverage. Source mutations MUST enforce their fence. For the initial SWMR proof, an unfenced takeover MUST refuse; a source identity transfer or new generation requires an explicit contract decision before it is enabled. Clocks and stale-policy handling must be specified. The tradeoff is reduced availability when authority cannot be established.

**Affected contracts:** Provider attachment, ServeClaim, source transaction execution, SWMR adapter, routing. **Dependencies:** D01–D03; B1–B5 already mandate authenticated attachment/context.

**Validation:** An expired partitioned writer cannot mutate after takeover; replica read service cannot acquire command rights; provider replacement cannot bypass single-writer checks. ASCR-P02/P05. Evidence: [attachment ruling:185](/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md:185), [source lock:127](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeWorkspaceDirectory.md:127), [writer check:177](/Users/owebeeone/limbo/glade-wz/glade/node/src/store.rs:177).

## ASCR-D05 — Acceptance, durability and offline intent

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F05, ASCR-F07. Related rulings: GDL-035/036, F-GAP10.

**Question:** Which observable receipt distinguishes memory acceptance, durable replica acceptance, remote acceptance and source commit?

**Why:** Current IDB failure can leave memory serving while persistence silently fails. “Saved” must not collapse these states. Accepted offline edits may be irreplaceable even when their projection appears cache-like.

**Alternatives:** Keep best-effort local persistence implicit; expose receipt levels and storage failure; require remote acknowledgement for every local interaction.

**Recommendation:** Expose receipt/failure states. Durable acceptance MUST mean a committed recovery boundary; memory-only success MAY support interaction but MUST NOT be presented as durable save. Pending source commands MUST stay separate from committed source data and be reauthorized on reconnect. This adds consumer states but preserves local-first interaction.

**Affected contracts:** InstanceStore, IndexedDB implementation boundary, supplier command results, offline intent retention. **Dependencies:** D03/D04.

**Validation:** Forced storage failure yields explicit non-durable state; restart recovers every durably acknowledged edit; a revoked pending command does not execute. ASCR-P06. Evidence: [IDB error handling:124](/Users/owebeeone/limbo/glade-wz/glial/src/store_idb.ts:124).

## ASCR-D06 — Recovery and collection conformance

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F02, ASCR-F04, ASCR-F07. Related rulings: GDL-041, GSC-05/07/08, F-GAP10.

**Question:** How are checkpoint provenance, retained coverage and collection policy bound to each exact engine/profile rather than inferred from generic retention tokens?

**Why:** SWMR repair, snapshot-delta expiry, CRDT bootstrap and terminal gaps have different obligations. CRDT v1 explicitly disallows hidden post-bootstrap compaction.

**Alternatives:** Universal snapshot-plus-cursor recovery; per-shape versioned recovery/collection capabilities; permanently retain every operation everywhere.

**Recommendation:** Every adapter MUST name its recovery contract and conformance corpus. Stores MUST report coverage loss explicitly. Shape semantics constrain retention: CRDT v1 post-bootstrap pruning remains unavailable until a new accepted contract; snapshot-delta MUST retain its out-of-band refresh behavior. Existing policy defaults remain in force where applicable; this question concerns semantics, not new numbers.

**Affected contracts:** Taut profiles, node/client checkpoints, Glial restore, retained advertisements. **Dependencies:** D03/D05.

**Validation:** Compare uninterrupted and restored transcripts; exercise expired cursor, missing source, deletion and replay of old revocation records; prove no resurrection or mixed generation. ASCR-P05/P06. Evidence: [CRDT retention:76](/Users/owebeeone/limbo/glade-wz/taut-shape/dev-docs/TautShapeCrdtDecision.md:76), [expiry profile:7](/Users/owebeeone/limbo/glade-wz/taut-shape/dev-docs/TautShapeSnapshotDeltaDecision.md:7).

## ASCR-D07 — Authority transaction to publication

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F09. Related ruling: GDL-036 keeps replication as ops.

**Question:** What must survive the gap between source commit, result acknowledgement and Glade publication, and which projections may recover by a fresh baseline?

**Why:** Garns notifies after COMMIT. Process loss or listener failure can leave a successful transaction unobserved; a blind command retry can duplicate effects.

**Alternatives:** Best-effort callback publication; atomic source-side idempotency/result/outbox records plus retry; a distributed transaction spanning source and Glade.

**Recommendation:** Use durable source-side command identity/outcome and publication intent committed atomically with the mutation where possible. Publication MUST be replay-safe; retries MUST return a known outcome or explicit uncertainty. A declared recoverable question MAY restart from a fresh source snapshot/generation. This avoids a distributed transaction but requires result/outbox retention and adapter mapping. It is not a claim that existing Garns implements these additions.

**Affected contracts:** Garns transaction integration, supplier results, publication identities, shape generation/resume. **Dependencies:** D04–D06.

**Validation:** Crash immediately after COMMIT, after publication but before receipt, and during duplicate retry; prove one source effect and consumer equivalence. ASCR-P03/P04. Evidence: [commit ordering:217](/Users/owebeeone/limbo/garns-wz/garns/src/garns/engine.py:217), [live sequence state:100](/Users/owebeeone/limbo/garns-wz/garns/src/garns/live.py:100).

## ASCR-D08 — Garns integration ownership

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F03, ASCR-F08. Related rulings: GDL-035–038; Garns independent artifact ownership.

**Question:** Should the first integration use explicit mappings from versioned Garns artifacts/Python API to Glade surfaces, with generation limited to facts owned by those inputs?

**Why:** Garns owns relational meaning but does not know distributed reader sets or file/process authority. Grazel P8 is not the ratified v9-5 runtime; current `garns-rust` is not a database implementation.

**Alternatives:** Explicit integration package; Garns generates all Glade declarations; Glade generates Garns worlds; independent handwritten schemas on both sides.

**Recommendation:** Use an integration package depending downstream on both contracts. Garns MUST own schema/query/scope facts; Glade MUST own distributed declaration/policy semantics; the application MUST own the mapping. Generated descriptors MAY remove duplication once portable schemas stabilize. Start the proof with the Python runtime; do not make a Rust engine a prerequisite or treat current runtime limits as permanent architecture.

**Affected contracts:** Garns portable IR/result schemas, application mappings, surface payload types and generators. **Dependencies:** D01–D03; D07 for effectful proof.

**Validation:** Change a qualified question/result schema and detect mapping drift; unknown artifact versions refuse; substitute storage bindings without changing Glade policy. Evidence: [portable contracts:7](/Users/owebeeone/limbo/garns-wz/garns/contracts/README.md:7), [Rust execution boundary:14](/Users/owebeeone/limbo/garns-wz/garns-rust/README.md:14).

## ASCR-D09 — Existing file, terminal, security and management constraints

**Status: ANSWERED BY EXISTING RULINGS — no new approval requested.** Findings: ASCR-F02, ASCR-F06, ASCR-F10.

**Question:** Are saved-file authority, compare-and-replace save, collaborative text, terminal read/input separation and authenticated attachment still options awaiting initial selection?

**Answer/evidence:** No. [GLP-0006 Decisions:625](/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0006-grazel-gryth-suppliers/Decisions.md:625) records worksheet ratification, with file/CRDT specifics at 648–662 and B1–B5 at 630–635. [GDL-041:58](/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md:58) supersedes obsolete shape terminology; [terminal spec:25](/Users/owebeeone/limbo/glade-wz/dev-docs/glade/suppliers/glade-terminal.md:25) separates broker, channel and retained log. GDL-038 already makes management ordinary bindings; GDL-032 separates discovery layers.

**Why it matters:** Reopening these implicitly would misclassify accepted requirements as reviewer preferences. A recommendation heading is not decisive when a later acceptance exists.

**Alternatives/tradeoffs:** Preserve the rulings, or explicitly amend specific guarantees with an owner decision and updated conformance evidence. A private spool is an implementation option only if it preserves the accepted externally visible contract; replacing replicated scrollback is a semantic amendment.

**Affected contracts:** Supplier specs, security model, management surfaces, Taut adoption. **Dependencies:** None for recognizing existing authority; D04/D06 must remain compatible.

**Validation:** Save conflict preserves unsaved CRDT edits; stale terminal driver is rejected; reconnect reports a retained gap; revoked membership stops private delivery. Current implementation gaps do not weaken these requirements.

## ASCR-D10 — Scope of the next proof

**Status: PROPOSED — awaiting owner decision.** Findings: ASCR-F11, ASCR-F12.

**Question:** Is one scoped Garns question plus one command, through a replica and Glial consumer, the smallest next contract proof?

**Why:** It exposes source authority, binding isolation, typed delivery, command retries and the commit/publication gap without requiring full filesystem/PTY products.

**Alternatives:** This relational proof; a read-only file demonstration; a complete terminal implementation; generic adapter framework first.

**Recommendation:** Select the relational proof with two scopes/principals and ASCR-P01–P06 from the review. It MUST include failure injection and refusal behavior, not just a happy path. It does not establish terminal transport or CRDT compaction conformance; those retain separate contracts.

**Affected contracts:** Mapping, source adapter, replica store, existing SWMR integration. **Dependencies:** D01–D08 sufficiently resolved; B1–B5 are prerequisites for any security claim.

**Validation:** All six observable proof outcomes must be recorded against exact versions. No implementation is authorized by this worksheet.

## Work independent of owner decisions

Read-only contract inventories, source/line verification and mapping of existing corpus requirements can proceed without choosing new semantics. Existing mandated security and fail-closed behavior does not require the owner to ratify it again; implementation remains a separate task. Cache sizes, framework selection, SQL pool design and generator packaging follow the semantic decisions rather than blocking them.

Verification for this review: source inspection plus `cargo fmt --check` and `cargo test --workspace --offline` in `garns-rust` (four passing unit tests); no new integration was executed. See the companion's evidence snapshot for revisions, dirty state, search limits and excluded evidence.
