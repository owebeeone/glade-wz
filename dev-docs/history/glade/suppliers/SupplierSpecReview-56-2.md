# Supplier specification adversarial review — 56-2

Fresh review of the current on-disk supplier specs against the required normative documents, atlas traces, canonical gwz-core/razel sources, and built glade/glial/supplier code. Existing review files were not consulted before this section was completed.

## CONFLICT findings

### SR56-2-01 — CONFLICT — the claimed gwz/taut pin is not reproducible from current disk

- **Spec/claim:** `glade-gwz.md` §1/§10, lines 3–8 and 290–298: “gwz-core v0.9.2 on taut v0.8.0 … PINNED.”
- **Ground truth:** canonical gwz-core is 0.9.2 (`gwz-dev/gwz-core/Cargo.toml:1–4`), but `protocol/regen.py:28–33,102–106,221–229` defaults to latest PyPI; the current `.regen-venv/.../taut_proto-0.8.1.dist-info/METADATA:3` is 0.8.1, and `Cargo.toml:12–15` pins `taut-shape` only by an unrelated git rev.
- **Failure:** regenerating the same “pinned” release on another day can use a different codec generator and produce or validate different artifacts; the in-flight one-codec convergence has no reproducible release gate yet.
- **Disposition:** pin one ratified taut version in regen and release CI, stamp/assert it in generated artifacts, regenerate corpus, then re-review; or change the spec pin to the actual ratified version. Lean: finish the one-codec work and make the pin executable.
### SR56-2-02 — CONFLICT — malformed wire or declaration data can panic instead of failing as data

- **Spec/claim:** `GladeSupplierModel.md` §2/§6, lines 71–81 and 143–157 requires bounded failure data and a usable session.
- **Ground truth:** `glade/node/src/frame.rs:58–77` returns `Result` but calls panic-prone `cbor::decode`; `glade/wire-rs/src/cbor.rs:267–368` indexes, unwraps, asserts, and panics. Accepted `Ops` are persisted without schema checks (`server.rs:242–260`), then `exchange.rs:60–78` panic-decodes `dir.services`/`dir.bindings` on every classification.
- **Failure:** one truncated frame kills a session task; one valid-chain op with a malformed declaration payload can become restart-stable denial of service for later exchange lookup.
- **Disposition:** regenerate glade-wire on the fail-closed codec, validate typed system records before persistence, and quarantine invalid records; or explicitly withdraw failure-as-data until that lands. Lean: the former, with arbitrary-byte and restart tests.
### SR56-2-03 — CONFLICT — unauthenticated provider attachment defeats gwz’s structural composition wall

- **Spec/claim:** `glade-gwz.md` §8/§11, lines 264–269 and 342–344: an unattached mutating/egress member has no provider and is structurally absent.
- **Ground truth:** any `Subscribe` to a declared exchange attaches a provider (`glade/node/src/server.rs:181–195`), and `exchange.rs:81–90` blindly last-writer-wins overwrites `(share, glade_id) → session`; `grazel-app.glade` is intended to declare the whole family.
- **Failure:** an untrusted session attaches itself to omitted `gwz.push`, or replaces a trusted status provider, and receives/answers effect requests despite the composition’s intended wall.
- **Disposition:** omit dangerous declarations as well as processes, or authenticate/authorize provider attach and reject replacement. Lean: both—composition controls declaration presence and the node binds providers to trusted identities.
### SR56-2-04 — CONFLICT — normative gwz sources still specify the superseded wire API

- **Spec/claim:** `glade-gwz.md` §3–§5, lines 126–183 and 195–211 requires glade-owned path-free DTOs, method-specific wrappers, events plus a retrievable final result, and a no-selection creation arm.
- **Ground truth:** normative `SupplierOutlines.md:62–80` and `SupplierRequirements.md:42–48` still require the canonical gwz `Request` as payload, `ResponseEnvelope` as answer, and say all create/materialize members operate on selection.
- **Failure:** two compliant builders produce incompatible clients, codecs, response handling, and creation routing.
- **Disposition:** update both normative files to v2 before plan conversion; or retract the v2 projection. Lean: update the normative files and trace the new contract.
### SR56-2-05 — CONFLICT — one request type per supplier contradicts the proposed 21-member family

- **Spec/claim:** `glade-gwz.md` §1, lines 20–22 says each `role="in"` method is its own supplier.
- **Ground truth:** §2, lines 76–124 proposes 21 members by merging method pairs and splitting the single canonical `tag` method into two suppliers; §14 still leaves 21 versus 24 open.
- **Failure:** declaration generation, grant units, provider count, and UI ownership cannot be derived while the same spec mandates both 1:1 and non-1:1 grains.
- **Disposition:** ratify 24 strict members or the explicitly enumerated 21-interface partition, then remove the contradictory absolute. Lean: decide before any generated family work.
### SR56-2-06 — CONFLICT — workspace creation is both a public gwz exchange and a private materializer leg

- **Spec/claim:** `glade-gwz.md` §2/§7, lines 103–105 and 241–249 exposes create/init/clone members as ordinary exchanges, but §5, lines 219–225 says `glade-gwz-create` is only the host-local leg invoked by node-intercepted `workspace.create`.
- **Ground truth:** `glade-workspaces.md` §2.2/§3–§4, lines 37–50 and 80–100 owns the public ceremony; `GladeSupplierModel.md:47–85` permits only wire-attached supplier choreography. Authz §3a, lines 74–80 also requires creation to mint the creator as structural authority root, while the proposed commit names only `WorkspaceEntry`/`ServeClaim`.
- **Failure:** direct `gwz.create` either bypasses directory/authority creation, or is not actually callable; the node has no declared provider-call surface by which to invoke the allegedly private leg.
- **Disposition:** keep exactly one public `workspace.create` and define an authenticated internal materializer interface, including creator-root minting; or retire `workspace.create` and give gwz the whole ceremony. Lean: the first.
### SR56-2-07 — CONFLICT — member-repository creation has two public owners

- **Spec/claim:** `glade-workspaces.md` §2.2/§3–§4, lines 47–50 and 82–100 assigns create/add/clone to `ws.ops` and glade-workspaces.
- **Ground truth:** `glade-gwz.md` §1–§2, lines 43–46 and 89–105 exposes the same mutations through `gwz.repo` and `gwz.clone-member`, without the creation-leg reconciliation supplied for workspaces.
- **Failure:** two grants and result contracts can race changes to one manifest and disagree about when member enumeration is complete.
- **Disposition:** retire `ws.ops` in favor of typed gwz members, or make gwz routines private execution legs behind `ws.ops`. Lean: one public typed gwz ceremony, with workspaces consuming its result.
### SR56-2-08 — CONFLICT — selected workspace has three sources of truth and a non-unique root map

- **Spec/claim:** workspaces §2.1/§3, lines 26–35 and 82–89 makes `ws.selection` a grip-side value; gwz §5, lines 197–201 says the addressed share is selection; terminal §2, line 45 still accepts `workspace-ref` in `term.open`.
- **Ground truth:** `s-ws-host` (`workspaces.ts:181–210`) says the request does not name the workspace and selection resolves the share. Workspaces §1.2, lines 17–20 calls the map name→path while §7, lines 124–126 permits duplicate names; gwz requires share→root.
- **Failure:** UI selection A, addressed share B, and terminal payload C can disagree; duplicate display names cannot identify one filesystem root.
- **Disposition:** make addressed stable workspace/share ID the sole authority input and map ID→root, rejecting redundant mismatches; or define precedence and signed selection tokens. Lean: addressed share + stable-ID map.
### SR56-2-09 — CONFLICT — stage-1 gwz attribution is impossible without the context it defers

- **Spec/claim:** `GladeSupplierModel.md` §4, lines 107–123 says every supplier MUST attribute in stage 1; `glade-gwz.md` §8 calls the full family stage-1 buildable.
- **Ground truth:** gwz §6, lines 227–239 bans payload principals and admits `ExchangeReq` has no requester; wire `ExchangeReq` is only `{share,glade_id,corr,payload}` (`generated.rs:410–433`), and the requester trace is scheduled only for stage 2.
- **Failure:** `OperationAttribution.actor` is absent, forged, or stamped as the supplier rather than the caller throughout the claimed stage-1 flow.
- **Disposition:** make trusted Hello-bound provider-call context a pre-family prerequisite, separate from enforcement; or weaken stage-1 attribution. Lean: build the context first.
### SR56-2-10 — CONFLICT — the tag-local egress wall is a name, not a type boundary

- **Spec/claim:** gwz §2/§11, lines 98–118 and 339–341 says splitting `tag-local` from `tag-remote` makes the wall structural.
- **Ground truth:** §3, lines 142–149 still carries canonical `TagOp`; gwz-core’s one `TagRequest` admits create/list/fetch/push/delete (`protocol/gwz.taut.py:155–161,1101–1114`). No distinct DTO IR, narrowed enum, or hostile cross-arm trace exists.
- **Failure:** a caller sends `TagOp.push` to the attached local provider; projection reopens egress unless the provider adds the per-op discriminator the split claims to eliminate.
- **Disposition:** define generated disjoint `TagLocalOp`/`TagRemoteOp` DTOs and reject cross-arm values before projection; or retain one tag surface with explicit per-op authz. Lean: disjoint DTO types.
### SR56-2-11 — CONFLICT — tag fetch and tag push are still one least-privilege unit

- **Spec/claim:** gwz §2, lines 101 and 111–118 puts both tag fetch and push behind `tag-remote`.
- **Ground truth:** Authz §5, lines 161–168 distinguishes `git.pull` from `git.push`; executable `s-verbs` (`authz.ts:335–360,438–455`) grants pull while denying push.
- **Failure:** a principal permitted to fetch tags necessarily gains remote write, defeating the ratified read/egress separation.
- **Disposition:** split tag-fetch and tag-push surfaces, or retain distinct verb checks inside tag-remote. Lean: split them because suppliers are the advertised grant unit.
### SR56-2-12 — CONFLICT — gwz result artifacts have no legal authorization zone and diff bytes remain attackable

- **Spec/claim:** gwz §4/§7, lines 189–193 and 245–250 labels egress logs “keyed to the member grant,” but declares `gwz.diff.output` unconditional workspace commons.
- **Ground truth:** DeclSurface lines 24–30 permits one real zone per binding; Authz §4a, lines 140–159 says keying is routing while grants are policy. The spec never pins the exact `(resource,verb)` check, INV-4 checks only `OPS` and only share presence (`invariants.ts:33–71`), and gwz-core `diff/output.rs:255–269` panic-decodes a record that current node clients may append.
- **Failure:** a member without diff/path authority reads a known `log_id`, or injects malformed output that panics a consumer; “grant-keyed” cannot be encoded as a `BindingDecl`.
- **Disposition:** declare real zones, exact read/write grants and writer identity for every event/result/output, and make record decode fallible; or use authenticated private result bindings. Lean: explicit per-artifact grants plus fail-closed decode.
### SR56-2-13 — CONFLICT — the atlas and plan still validate the retired architecture and the wrong security chronology

- **Spec/claim:** gwz §10–§11, lines 300–349 retires `gwz.ops` and requires new traces; diff §7 requires P3-tail instantiation; later files/terminal/diff/editing specs call their first build stub-allow-all.
- **Ground truth:** registered `workspaces.ts:191–210,267–287,470–489` and `authz.ts:404–455` still exercise `gwz.ops`; Plan P1.S2, lines 127–135 has only the completed multiplexer, no family or diff step, while P2.S3, lines 148–150 ends allow-all before P3/P4.
- **Failure:** every named plan gate can go green while the reviewed gwz family and diff do not exist, and later suppliers either bypass the raised security floor or cannot implement their stated stage 1.
- **Disposition:** add a pre-P2 family/context/DTO/trace migration and an explicit diff sequence; define later “stage 1” as machinery maturity under already-enabled checks. Lean: re-plan before coding.
### SR56-2-14 — CONFLICT — glade-users claims signed primitives that built code does not have

- **Spec/claim:** `glade-users.md` §2, lines 49–56 says signed chains, equivocation proofs, and revocation already provide every primitive; §5, lines 103–112 simultaneously admits structural signatures.
- **Ground truth:** wire `Op` has no signature (`generated.rs:182–223`); `Hello.principal` is caller text; `server.rs:164–179,242–260` binds it and accepts arbitrary op origin; `store.rs:121–147` checks sequence/hash only and accepts missing `prev`.
- **Failure:** Mallory claims Alice’s Hello/origin and wins a first-valid name/profile/invite record; a fork proof proves only that a string-origin forked, not that Alice acted.
- **Disposition:** mark authenticated sessions and per-op signatures as new load-bearing work and defer identity governance until present; or narrow stage 1 to explicitly untrusted demo data. Lean: the former.
### SR56-2-15 — CONFLICT — chat’s claimed runtime declaration is never consumed

- **Spec/claim:** `glade-chat.md` §1/§2, lines 31–37 and 48–55 says `chat.decl` carries `BindingDecl` records that make groups routable.
- **Ground truth:** `glade-chat/src/supplier.ts:45–50` appends JSON under the app share; node registration accepts typed `Record::Binding` only on home `dir.bindings` (`registry.rs:47–80`, `exchange.rs:60–78`). The fixture predeclares `chat.msgs`/`chat.groups`, not `chat.decl` (`grazel-app.glade:27–39`).
- **Failure:** adding a runtime group changes only decorative app data; no declaration fold or routability changes, while raw log appends happen to work for unrelated reasons.
- **Disposition:** append a typed authorized home `BindingDecl`, or relabel `chat.decl` as non-authoritative metadata and retract the built claim. Lean: use the real registry path.
### SR56-2-16 — CONFLICT — the knock ceremony assumes read implies append and creates a confused deputy

- **Spec/claim:** `glade-share.md` §5, lines 99–111: anyone who can read the carrying share can “ALWAYS write” an `AccessRequest{requester-fp,...}` there.
- **Ground truth:** Authz §1, lines 19–35 separates read and append enforcement; Hello/origin/requester payloads are not authenticated on current code (`server.rs:164–179,242–260`).
- **Failure:** a read-only recipient cannot knock; if append is opened, Mallory claims `requester-fp=Bob` and a grantor tap launders it into a canonical pending grant under the grantor’s authority.
- **Disposition:** use an authenticated directed request exchange, or grant a narrow request-append entitlement with a cryptographic requester proof. Lean: directed exchange plus offline queueing.
### SR56-2-17 — CONFLICT — blob fetch has neither a legal shape nor path-derived authorization

- **Spec/claim:** `glade-files.md` §3/§6, lines 79–90 and 130–137 declares `ws.blob | window/exchange`, keyed only by hash, while promising AZ-1 hides `/secret`.
- **Ground truth:** one `BindingDecl` has one Shape (`GladeDeclSurface.md:24–30`); Authz checks `(share,glade_id,key)` and path scoping is the required visibility boundary (`GladeAuthzModel.md:170–188,209–211`).
- **Failure:** a caller who learned a secret blob hash from stale/link metadata fetches it without any authorized path reference; builders also cannot declare `window/exchange` as one surface.
- **Disposition:** choose one typed request/result shape and require an authorized live path/reference or attenuated object capability; or defer blob reads. Lean: declared exchange with reference proof and typed failures.
### SR56-2-18 — CONFLICT — a monolithic `ws.tree` value cannot enforce subtree visibility

- **Spec/claim:** files §4/§6, lines 110–116 and 130–137 says a `/src` grant hides `/secret`, but `ws.tree` is one value keyed `{root}`.
- **Ground truth:** Authz §6, lines 170–188 makes one allow/deny decision for the whole `(share,glade_id,key)`; it does not redact fields inside a folded value.
- **Failure:** allowing `{root:"/"}` leaks secret entries; denying it hides the permitted subtree as well.
- **Disposition:** key tree entries/subtrees separately, or serve an authority-filtered projection with revision semantics. Lean: subtree-keyed surfaces that the same path policy can check.
### SR56-2-19 — CONFLICT — terminal locality and private watcher semantics have no routable representation

- **Spec/claim:** terminal §2/§5/§7, lines 41–49,118–141,159–167 says owner-only/local-only is real by private zone/no advertisement, then lets `term.read` replicate the owner-private scrollback to watchers.
- **Ground truth:** routing selects by share `ServeClaim`, not surface/locality (`node/src/mesh.rs:77–104`); an advertised selected workspace routes remote `term.open`, `ExchangeReq` has no requester, and AZ-16/private `self:` routing has no foreign-reader export.
- **Failure:** a remote caller reaches the terminal provider, which cannot prove local owner; conversely a legitimate watcher grant cannot route Alice’s private key without violating private-zone isolation.
- **Disposition:** enforce authenticated no-forward/local-transport policy for open and publish a separate gated watcher projection; or defer all remote terminal semantics. Lean: explicit local policy plus separate commons/session-share read surface.
### SR56-2-20 — CONFLICT — `self:` is caller-selected routing, not principal privacy

- **Spec/claim:** `glade-editing.md` §2, lines 66–80 says a foreign `self:` never matches and selections are already private.
- **Ground truth:** `router.rs:18–52` matches exact caller-provided keys; `server.rs:181–220` accepts any subscribe key without comparing it to the Hello-bound session principal.
- **Failure:** Bob subscribes `doc.selection,key=self:alice` and receives Alice’s caret; the same flaw reaches terminal private logs.
- **Disposition:** derive/bind private keys at the node from an authenticated principal and reject mismatched subscribe/write; or stop calling routing-only keys private. Lean: identity-bound keys plus negative tests.
### SR56-2-21 — CONFLICT — glial’s claimed log delta can duplicate old data and omit new data

- **Spec/claim:** editing §3/§7, lines 87–111 and 162–169 calls existing `logDelta` the cursor-stable SWMR basis.
- **Ground truth:** `glial/src/instance.ts:133–139` globally re-sorts by `(lamport,origin,seq)`, then lines 156–163 computes delta as `whole.slice(emittedLen)` rather than by stable record identity.
- **Failure:** B@lamport10 arrives first (`[B]`); late A@lamport5 reorders to `[A,B]`; `slice(1)` emits B twice and never emits A, corrupting text and caret state.
- **Disposition:** compute deltas by stable record identity/causal cut, or guarantee immutable append order. Lean: identity-set difference with interleaving/replay regression tests.
### SR56-2-22 — CONFLICT — `doc.save` bypasses the sole coarse-write/path authorization surface

- **Spec/claim:** editing §4/§6, lines 113–127 and 149–160 writes the working tree through `doc.save`.
- **Ground truth:** files §4/§6, lines 95–116 and 130–138 assigns whole-file replace to `files.write` and AZ-1 path checks; no delegation, base revision, or shared lock ordering is defined.
- **Failure:** a user with live-document edit membership but no `/secret` file-write right saves into `/secret`, or overwrites a concurrent gwz pull.
- **Disposition:** make `doc.save` delegate to authenticated `files.write replace` with expected base revision and workspace lock; or define an explicit derived save capability. Lean: delegate.
### SR56-2-23 — CONFLICT — `ws.diff` cannot be private per viewer and globally pair-deduplicated

- **Spec/claim:** diff §2/§6, lines 67–72 and 150–156 calls output private; §3/§5, lines 82–105 and 137–148 keys/dedups solely by ordered `{left,right}` across sponsors.
- **Ground truth:** private delivery identity includes viewer key; `s-svc-shared` (`sync.ts:198–249`) shares one pair-keyed instance/stream across nodes and principals.
- **Failure:** Alice and Bob either collide on one supposedly private binding, or viewer identity prevents the canonical keys from deduping.
- **Disposition:** separate compute-instance key from per-viewer delivery bindings and authorize each fan-out; or make output shared/gated. Lean: two-level identity.
### SR56-2-24 — CONFLICT — diff’s leak-guard formula grants the opposite of the prose

- **Spec/claim:** diff §5/§7, lines 137–148 and 175–178 writes `read(ws.diff) ⊇ read(left) ∪ read(right)` while saying every diff reader must read both sources.
- **Ground truth:** for reader sets the safety property is `Readers(diff) ⊆ Readers(left) ∩ Readers(right)`; Authz §1 requires every serving hop to decide locally.
- **Failure:** implementing the written relation can admit a left-only reader and reveal right-side content through additions/removals.
- **Disposition:** state the per-principal predicate `can_read(left) && can_read(right)` and the subset/intersection relation; add left-only/right-only/revoked-midstream tests.
### SR56-2-25 — CONFLICT — diff teardown cannot erase the replicated value it authored

- **Spec/claim:** diff §1/§4, lines 29–33 and 114–125 says the instance has no durable state and teardown nulls everything.
- **Ground truth:** `ws.diff` is authored as a value op (§6, lines 150–156); store journals accepted ops (`store.rs:77–90,129–146`) and Subscribe backfills them (`server.rs:217–235`). `s-svc-shared` explicitly leaves a warm replica (`sync.ts:239–249`).
- **Failure:** after teardown/source change/revocation, a later mount receives stale diff bytes before a new instance validates sources.
- **Disposition:** use a genuinely transient non-store path, or generation-bound pending/ready/stale/error values with expiry/tombstones and revalidation. Lean: explicit state envelope and generation fence.
### SR56-2-26 — CONFLICT — proposed executable service definitions are both unsafe and incompatible with the built record

- **Spec/claim:** diff §2/§7, lines 35–62 and 165–174 makes `{match,program,emits,policy}` an ordinary allow-all append that triggers spawn.
- **Ground truth:** current `ServiceDefinition` is only `{app,name,glade_id}` (`node/ir/sysdata.taut.py:86–92`), parser accepts only `service` (`appdecl.rs:131–170`), and `exchange.rs:60–78` panic-decodes that old shape; any session’s Ops currently persist.
- **Failure:** the new record either crashes the old decoder, or Mallory appends a program hash and causes host code execution with ambient filesystem/network/time; content addressing proves bytes, not trust.
- **Disposition:** introduce a versioned demand-service record/stream plus migration, and execute only composition-pinned programs until signed authority and deterministic sandbox/resource policy exist. Lean: both gates before spawn.
### SR56-2-27 — CONFLICT — the razel stub’s deferral premise is stale

- **Spec/claim:** `glade-razel.md` lines 3–8 and 28–33 says no taxonomy/protocol exists and expansion waits for a wire surface.
- **Ground truth:** canonical `razel-wire-api` is 0.1.0 (`Cargo.toml:1–6`); `protocol/razel.taut.py:1–7,125–137` is ratified and defines ten methods; `razel-comms/src/wire.rs:40–130` has fail-closed typed requests and daemon dispatch is exhaustive (`razel-daemon/src/lib.rs:93–118`). Some verbs deliberately return typed Unsupported.
- **Failure:** the supplier remains an empty slot even though the trigger it names has fired; builders cannot plan the available build/events surface.
- **Disposition:** expand now against the canonical 0.1.0 protocol while marking unsupported verbs honestly; or change the trigger to a published release and document why current ratified IR is insufficient. Lean: expand.
### SR56-2-28 — CONFLICT — retention is deferred past its required gate

- **Spec/claim:** files §5/§7, lines 118–128 and 144–153, chat §7, lines 151–164, and terminal §7/§10 defer eviction/retention.
- **Ground truth:** `SupplierRequirements.md:80–93` says GAP-10 MUST be answered by files; Plan rider `Plan.md:179–185` says it is needed before P3 files at latest.
- **Failure:** P3 can ship backfilled files, blobs, chat logs, and detached terminal output with unbounded disk/IDB growth and no pressure behavior.
- **Disposition:** rule one TTL/size/pressure policy before P3, or make stage-1 bytes explicitly ephemeral and bounded. Lean: shared bounded retention contract before files.
### SR56-2-29 — CONFLICT — concurrent eligible-host registration is lost by the normative fold

- **Spec/claim:** workspaces §2.2 and gwz §5 use clone to add eligible hosts; `s-ws-clone` (`workspaces.ts:502–527`) rewrites `[peer1]` to `[peer1,peer2]`.
- **Ground truth:** `GladeWorkspaceDirectory.md:54–60` makes `WorkspaceEntry.eligible_hosts` LWW per field.
- **Failure:** peer2 and peer3 clone concurrently from `[peer1]`; their full-list writes race and LWW discards one real replica, making directory/routing state dishonest.
- **Disposition:** use per-host eligibility records or an OR-set with explicit removal; or serialize all changes through one owner. Lean: per-host records/OR-set.

## GAP findings

### SR56-2-30 — GAP — glade-share omits its normative direct membership ceremony

- **Spec/claim:** `SupplierOutlines.md:49–58` requires “share this workspace/group with that principal,” visibility, and revoke; chat also delegates create-a-group to create-a-share.
- **Missing contract:** `glade-share.md` surfaces (§6, lines 123–134) cover links, policy entries, knocks, and notifications, but define no direct `share.create`/membership invite request/result/failure ceremony and do not depend on workspaces.
- **Failure:** a user can only grant incidentally while sharing a link; there is no specified path to share a workspace directly or create/seed a group share.
- **Disposition:** add direct create/share/invite/revoke exchanges with idempotency and failure arms; or assign them explicitly to workspaces/users. Lean: glade-share owns orchestration over ordinary grant records.
### SR56-2-31 — GAP — “idempotent replay; no orphan” has no durable replay basis

- **Spec/claim:** gwz §5/§11, lines 206–217 and 331–338 mutates disk first, then records, and promises recovery after every crash window.
- **Missing contract:** no durable intent location, stable idempotency key independent of `corr`, phase record, recovery owner, rollback, or rule for `init_from_sources` returning `partial` is defined.
- **Failure:** crash after mkdir/clone but before `WorkspaceEntry`, or between entry and claim, leaves an uncorrelatable checkout or half-created directory state.
- **Disposition:** define an intent→materialized→registered→claimed state machine with replay/cleanup ownership; or provide compensating rollback. Lean: durable intent and idempotent forward recovery.
### SR56-2-32 — GAP — retrievable gwz final results have no durability or takeover route

- **Spec/claim:** gwz §4/§7, lines 177–183 and 247–249 requires a later `.result(operation_id)` exchange and omits the final result from the replicated events log; creation results live on home while execution is remote.
- **Missing contract:** supplier crash/takeover drops provider state (`GladeSupplierModel.md:143–157`); GDL-006/GDL-012 remain open, and no operation-owner claim, replicated result store, retention, or transfer rule is named.
- **Failure:** an accepted pull/create finishes, the provider restarts or claim moves, and `members[]`/`errors[]` becomes permanently unreachable.
- **Disposition:** append the final `OperationResult` as the closing replicated log record; or specify durable result storage plus owner routing/transfer/retention. Lean: closing record.
### SR56-2-33 — GAP — terminal reattach and handoff require envelopes that do not exist

- **Spec/claim:** terminal §4, lines 86–103 promises offset-deduped replay/live splice and epoch-fenced stale input.
- **Missing contract:** `TermIn` (§3, lines 70–77) is only Bytes/Winch/Signal; `ChannelData` is raw `{channel,data}` (`generated.rs:488–505`); no `TermOut{offset,generation}` or epoch-bearing input/channel capability is declared.
- **Failure:** buffered bytes from the old driver execute after takeover, and replay/live overlap cannot be distinguished from a gap or duplicate.
- **Disposition:** define matching output/log offsets and bind driver epoch into the channel route or `TermIn`, atomically closing old channels; or drop lossless reattach/handoff. Lean: typed envelopes plus delayed-frame traces.
### SR56-2-34 — GAP — a first file window has no provider request path or revision identity

- **Spec/claim:** files §2/§8, lines 34–69 and 155–162 promises unseen `{path,from,len}` first paint, full-file backfill, local later windows, and mutable republish.
- **Missing contract:** built Shape has only Value/Log/Stream (`wire-rs/generated.rs:113–130`); Subscribe merely registers/backfills the exact key (`server.rs:181–240`) and never notifies a file authority. Range is part of key, backfill W3 has no key (`window.ts:71–76`), and neither carries file revision/generation.
- **Failure:** the first unseen window receives empty Heads forever; if delivery is improvised, scrolling addresses a different key and an edit between viewport/backfill assembles mixed revisions.
- **Disposition:** define a directed interest/provider path plus stable `{workspace,path,revision}` identity with non-identity ranges and stale-retry; or make window an explicit exchange. Lean: typed window contract with generation-bound reassembler.
### SR56-2-35 — GAP — SWMR reuses none of the enforcement that makes a writer single

- **Spec/claim:** editing §1.2/§7, lines 34–49 and 162–169 says `EditClaim=ServeClaim` and the whole SWMR substrate exists.
- **Missing contract:** system IR has no EditClaim/doc-owner record (`node/ir/sysdata.taut.py:35–118`); normal body Ops are unconditionally appended (`server.rs:242–260`); body ops carry no lease epoch/base cut, and ServeClaim/local filesystem lock fences nodes, not editor sessions.
- **Failure:** A’s delayed append lands after B takes epoch+1, so two writers’ positional ops fold in an order neither based on; “single writer ⇒ no merge” is false.
- **Disposition:** add EditClaim schema/fold/renewal, accepted-head handoff, epoch on every body op, and stale-writer rejection; or choose CRDT. Lean: do not claim SWMR is cheap until the write fence is designed.
### SR56-2-36 — GAP — “workspace-relative” has no shared containment algorithm

- **Spec/claim:** gwz §3, files §1.2, and terminal §6 all promise that relative paths cannot escape the app-owned root.
- **Missing contract:** no normalization rules cover absolute forms, `..`, Unicode/platform separators, symlinks, TOCTOU-safe opens, or whether `add_existing_repo` may name an external host path.
- **Failure:** `src/../secret`, a symlink under `/src`, or a race during resolution escapes the selected root and its AZ-1 grant while still looking relative.
- **Disposition:** define one root-relative typed path algorithm and symlink/open policy shared by all three suppliers; or use server-issued opaque file/repo handles. Lean: typed paths plus root-relative safe-open primitives.
### SR56-2-37 — GAP — browser-first onboarding can mint a second canonical principal

- **Spec/claim:** users §1/§3, lines 11–32 and 64–81 says principal is key fingerprint, browser device is normally certified by a root, but invite accept may mint a device and root-certify later; existing-principal invite carries only a fingerprint.
- **Missing contract:** no temporary-device→root merge, proof that a fingerprint owns a public key, collision rule, or canonical-id transition is specified.
- **Failure:** browser accept establishes `fp(device)`; later SSH root establishes `fp(root)`, yielding two principals for one person or an unverifiable fingerprint-only introduction.
- **Disposition:** require a root/public-key proof before canonical accept, or define a signed alias/merge with conflict traces. Lean: root-anchored canonical identity with device certs.

## What held

- The regenerated gwz inventory matches current canonical gwz-core v0.9.2: 24 `role="in"` and three `role="out"` methods (`events.subscribe`, `operation.result`, `diff.output`).
- `forall` remains absent as a `role="in"` method and is explicitly CLI-local, so terminal ownership is consistent.
- The v2 gwz spec correctly retracts the false “canonical Request is path-free” claim and recognizes server-side projection as necessary.
- Method-specific response wrappers, OperationEvent, OperationResult, and DiffOutputRecord are all present in the canonical protocol; the remaining issue is routing/durability, not their existence.
- Working-tree diff remains gwz-owned while cross-surface diff remains glade-diff-owned.
- `glade-terminal` accurately identifies the current channel route as echo-only and does not claim the real provider path is built.
- The two `grazel-app.glade` fixture copies are currently byte-identical; their declared `ws.diff` shape is still the known log/value mismatch the diff spec itself flags.
- Razel’s current protocol is real but partly unsupported; the deferral may still be product-appropriate, only the stated “no protocol exists” trigger is no longer honest.

## Disposition summary

Before plan conversion, rule and land in this order: (1) finish the one-codec pin and fail-closed glade/gwz decode boundary; (2) choose the gwz grain, single public creation/member ceremonies, stable share-addressing, exact grant tuple, and trusted requester/provider identity; (3) author DTO IR/corpora, hostile tag/output tests, new atlas invariants, and explicit family/diff plan steps; (4) close the identity-bound private-key, knock, blob, service-program, terminal, and save authorization holes; (5) rule durable create/results, retention, window generations, SWMR fencing, diff state, and eligible-host fold semantics. Gianni must choose the dials; builders should not infer them.

## Concordance with F5

Agree with F5-1/2/4/6/9/12 and independently extend them. This pass additionally found the codec-pin, fail-closed decoder, provider-hijack, public-ceremony, tag-least-privilege, logDelta, and stale-Razel-trigger failures; it disagrees with F5’s held claim that chat runtime declaration was verified, because current node code never consumes `chat.decl`.
