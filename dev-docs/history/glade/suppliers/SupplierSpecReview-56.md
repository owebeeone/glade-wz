# Supplier specification adversarial review — 56

Date: 2026-07-12  
Scope: every supplier spec named by `SupplierSpecReviewPrompt.md`; normative docs,
atlas traces, invariants, and claimed built code were checked from disk. Findings
below were written independently; `SupplierSpecReview-F5.md` was not consulted.

## CONFLICT findings

### SR56-01 — CONFLICT — glade-gwz is pinned to a protocol inventory it does not describe

- **Spec + claim:** `glade-gwz.md` §2, lines 38–40: “The 12 `role="in"` request types verified in the protocol.”
- **Contradiction:** the ruled source is the pinned GitHub release (`Decisions.md` lines 381–385). The on-disk canonical release mirror, `gwz-dev/gwz-core/Cargo.toml` lines 1–7, identifies v0.9.1 and its GitHub home; its `protocol/gwz.taut.py` lines 9–124 has **24** `role="in"` methods. The 12-method convenience copy has a different hash.
- **Failure scenario:** generation from the spec silently omits `clone_workspace`, `repo_sync`, member attach/detach/clone, `ls`, list-snapshots, commit, stage, stash, branch, and the ruled-in working-tree `diff` plus its output log.
- **Disposition:** finish the in-flight gwz/taut single-codec convergence, then regenerate the inventory from that one pinned artifact; alternatively freeze the spec to an explicitly older release. **Lean: regenerate and re-review before plan conversion.**

### SR56-02 — CONFLICT — “canonical typed Request” cannot also be path-free

- **Spec + claim:** `glade-gwz.md` §3, lines 72–73 and 86–92: payload “IS” the canonical request, while the client sends only path-free fields.
- **Contradiction:** canonical v0.9.1 requires `CreateWorkspaceRequest.workspace_root` (`gwz.taut.py` lines 953–959), `AddExistingRepoRequest.repository_path` (981–990), and `StageRequest.cwd` (1130–1138). The spec's own §12, lines 241–244, admits the host-path breach.
- **Failure scenario:** omitting the fields makes the canonical message undecodable; accepting client values lets a caller escape the selected root and falsifies the app-owned-storage seam.
- **Disposition:** define a Glade-owned, path-free DTO projected server-side into the canonical request, or stop claiming the exchange payload is the canonical request. **Lean: explicit trusted projection, regenerated with the unified codec.**

### SR56-03 — CONFLICT — the proposed gwz merges cross their own structural egress wall

- **Spec + claim:** `glade-gwz.md` §2, lines 49 and 53–58, merges capture/snapshot/tag as one local “versioning panel”; §1, lines 20–27, says composition structurally separates local mutation from egress.
- **Contradiction:** canonical `TagRequest` has `fetch`, `push`, and `delete` operations plus `remote`/`all` controls (`gwz.taut.py` lines 155–161 and 1101–1114).
- **Failure scenario:** granting/composing the merged “capture” supplier for local snapshots also admits remote tag push/delete, recreating the payload-discriminator authorization plane the split was ruled to eliminate.
- **Disposition:** use strict one-method-per-supplier, or merge only methods proven to have identical authority and blast radius. **Lean: strict 1:1 until the unified protocol is capability-audited.**

### SR56-04 — CONFLICT — effect suppliers never receive a trusted requester identity

- **Specs + claim:** `glade-terminal.md` §5, lines 114–123, and `glade-gwz.md` §4, lines 112–119, require authority-time caller checks and attribution.
- **Contradiction:** `ExchangeReq` is only `{share, glade_id, corr, payload}` (`taut/ir/glade.taut.py` lines 150–155; generated Rust lines 410–433). `exchange.rs` lines 113–123 forwards it unchanged. Yet `s-verbs` S6 (`authz.ts` lines 417–423) assumes the remote authority knows whose grant to check.
- **Failure scenario:** Eve calls `term.open` or `gwz.push`; the provider cannot distinguish Eve from the owner. A principal inside the payload is forgeable—the built gwz supplier currently trusts exactly that (`supplier.rs` lines 146–147).
- **Disposition:** carry node-authenticated requester/capability context to the provider, or have the node enforce and attach signed attribution before dispatch. **Lean: trusted provider-call context, never caller payload.**

### SR56-05 — CONFLICT — predictable `self:` keys do not make private zones private

- **Spec + claim:** `glade-editing.md` §2, lines 71–77: “A foreign `self:` never matches,” with no grant or check.
- **Contradiction:** the node accepts the key supplied by any `Subscribe` and registers it verbatim (`server.rs` lines 181–215; `router.rs` lines 29–31). Fingerprints and `self:alice` are not secrets.
- **Failure scenario:** Bob is a document member and subscribes to `doc.selection` with `key=self:alice`; Alice's future cursor ops now match Bob's subscription exactly.
- **Disposition:** derive/validate `self` from the authenticated session at ingress, or encrypt private-zone payloads to the owner. **Lean: identity-bound key validation plus a malicious-foreign-self trace/invariant.**

### SR56-06 — CONFLICT — `users.invites` is assigned two incompatible shapes

- **Spec + claim:** `glade-users.md` §3.1, lines 64–68, persists an `InviteRecord`; §4 line 95 declares `users.invites` as an `exchange`.
- **Contradiction:** the model separates op-bearing value/log/window surfaces from provider-attached exchanges (`GladeSupplierModel.md` lines 56–77). `s-invite` declares the id as exchange (`users.ts` lines 64–69) and then appends the record to the same id (88–93).
- **Failure scenario:** subscribing attaches a provider and has no replicated record gap; appending an invite either violates the declaration or leaves the provider with no folded records to validate.
- **Disposition:** split `users.invite.records` (log) from `users.invites` (exchange), or make mint/accept a log-driven state machine without exchange. **Lean: split surfaces.**

### SR56-07 — CONFLICT — delayed root certification changes the canonical principal

- **Spec + claim:** `glade-users.md` §1, lines 13–15, says the principal is the key fingerprint; lines 27–32 and 69–71 let a browser device key become the principal before a CLI root certifies it.
- **Contradiction:** `GladeAuthzModel.md` lines 15–17 makes device certificates chain to the user root. `s-invite` U2 (`users.ts` lines 98–104) nevertheless treats the temporary device fingerprint as the canonical id.
- **Failure scenario:** Dana onboards on two browser devices, or later attaches SSH root R to device D. Either D and R become two Danas, or changing D→R strands grants, sponsorship edges, chat attribution, and the fingerprint-keyed fold.
- **Disposition:** make the first browser key the enduring root, or define a stable principal id plus an explicit root/device merge ceremony before v1. **Lean: stable principal/root model first, with delayed-root and two-device traces.**

### SR56-08 — CONFLICT — glade-share replaced, rather than expanded, its normative supplier contract

- **Spec + claim:** `SupplierOutlines.md` lines 49–58 requires direct workspace/group share-to-principal, onboarding for a non-principal, appearance, and revoke/disappearance.
- **Contradiction:** `glade-share.md` §§1–9 specifies captured page-state links carried through chat; its dependencies omit glade-workspaces (lines 156–160) and its user test is a link/knock flow (161–166).
- **Failure scenario:** a builder can complete every glade-share section yet still cannot perform “share this workspace with the user I onboarded,” while glade-chat simultaneously depends on an unspecified create-a-share/membership ceremony.
- **Disposition:** restore the direct membership/share-point ceremony and layer links on it, or split membership-sharing and link-sharing into separate suppliers and update the normative outline. **Lean: restore the core ceremony.**

### SR56-09 — CONFLICT — the knock assumes read authority implies append authority

- **Spec + claim:** `glade-share.md` §5.2, lines 102–106: “where you can read commons, you can append.”
- **Contradiction:** authz separates reads, writes, and effects (`GladeAuthzModel.md` lines 19–25) and names `read.subscribe` separately from `write.append` (lines 161–168).
- **Failure scenario:** a read-only link viewer cannot append `share.requests`; if the implementation silently equates read with append, every read-only grant is widened into write authority and a spam channel.
- **Disposition:** grant a narrow `share.request.append` capability, or route knocks through a dedicated exchange/relay. **Lean: narrow append-only request capability.**

### SR56-10 — CONFLICT — chat's claimed runtime declaration is decorative data

- **Spec + claim:** `glade-chat.md` §1.3/§2, lines 31–37 and 54, says `chat.decl` carries ordinary runtime `BindingDecl` records that make groups routable.
- **Contradiction:** `glade-chat/src/supplier.ts` lines 45–50 writes incomplete JSON to `(chat, chat.decl)`. The node recognizes taut-CBOR `BindingDecl` on `(home, dir.bindings)` (`registry.rs` lines 35–52; `sysdata.rs` lines 118–146). The fixture statically declares chat surfaces and contains no `chat.decl`.
- **Failure scenario:** dynamic group declaration passes fake-session tests but the real node never registers it; static fixture declarations mask the failure in the demo.
- **Disposition:** append the real registry record and add a spawned-node test, or delete the runtime-declaration claim and keep groups static. **Lean: real record plus real-node regression.**

### SR56-11 — CONFLICT — `doc.save` bypasses the file supplier's write authority

- **Specs + claim:** `glade-files.md` §4, lines 101–116, owns structural writes through AZ-1-gated `files.write`; `glade-editing.md` §4, lines 121–127, independently writes the same working-tree file through `doc.save`.
- **Contradiction:** editing membership (§5, lines 129–145) is a document-commons entitlement, not necessarily a workspace path-scoped file-write grant.
- **Failure scenario:** Bob may edit a shared document and invoke `doc.save` on the host even though he lacks `files.write` for that path; concurrent `files.write`, `gwz pull`, and save also have no common version/lock ordering.
- **Disposition:** make `doc.save` call `files.write replace` with caller capability and expected base hash, or make files relinquish writes and duplicate equivalent enforcement. **Lean: delegate through files.**

### SR56-12 — CONFLICT — `ws.diff` cannot be private, pair-keyed, and globally deduplicated

- **Spec + claim:** `glade-diff.md` §2/§6, lines 67–72 and 150–156, calls the result private per viewer; §3, lines 82–105, makes `{left,right}` alone the byte-identical dedup key.
- **Contradiction:** AZ-16 private delivery is keyed to `self`; `s-svc-shared` SS1/SS2 (`sync.ts` lines 198–249) sends the same pair key and replicated value to another viewer/node.
- **Failure scenario:** include `self` and Alice/Bob no longer share a global instance; omit it and both receive the same commons-like binding, so it is not private.
- **Disposition:** separate compute-instance identity from per-viewer delivery identity, or declare a commons derived surface with source-derived read checks. **Lean: separate keys and make authorization explicit.**

### SR56-13 — CONFLICT — later suppliers are specified for allow-all after the plan ends allow-all

- **Specs + claim:** files §7, terminal §7, editing §5, and diff §7 each define their first build as stub-allow-all and postpone gates to “stage-2.”
- **Contradiction:** `Plan.md` P2, lines 137–150, switches on `check()` before P3 files/terminal and P4 editing; P3/P4 therefore begin after the security floor has risen.
- **Failure scenario:** implementing a P3 “stage-1 terminal” either bypasses already-live enforcement or cannot reproduce the spec's stated posture.
- **Disposition:** separate machinery maturity from authorization mode, or reorder the plan to keep an unsafe deployment mode. **Lean: P3/P4 first executable releases integrate enforcement; allow-all survives only in isolated tests.**

## GAP findings

### SR56-14 — GAP — gwz responses, completion, and event visibility do not match the canonical contract

- **Spec + missing contract:** `glade-gwz.md` §3, lines 72–85, returns a bare `ResponseEnvelope` and says `OperationResult` “closes” the events log; §4 puts events in workspace commons.
- **Ground truth:** canonical responses are method-specific wrappers—e.g. status adds `workspace_git_status` (`gwz.taut.py` lines 1185–1267). `events.subscribe` and `operation.result` are separate methods (lines 100–107); `OperationEvent` has no final result (917–950).
- **Failure scenario:** status cannot render its promised dashboard; an accepted pull never yields final member/errors; a reader lacking `gwz.push` can still watch commons push events while an authorized caller may lack result-read authority.
- **Disposition:** mirror canonical response + events + final-result surfaces with paired access, or define and version a complete Glade adaptation. **Lean: mirror the unified canonical trio.**

### SR56-15 — GAP — workspace creation is three uncomposed ceremonies and no transaction

- **Specs + missing contract:** workspaces §2.2 and gwz §5 promise disk and `WorkspaceEntry`/`ServeClaim` commit-or-fail together; gwz says its create member runs first.
- **Ground truth:** the node intercepts `workspace.create` before provider routing (`exchange.rs` lines 35–38 and 93–101); current `claims.rs` lines 174–191 explicitly creates records only and leaves materialization external. GDL-034 also requires creation to mint the creator's immutable root.
- **Failure scenario:** `workspace.create` never reaches `glade-gwz-create`; calling `gwz.create` directly has no record/root leg. A crash after disk success or between the two record appends leaves an orphan or half-created workspace despite the atomicity claim.
- **Disposition:** make `workspace.create` a durable state machine that invokes a host-local materializer and records creator/root, or retire it in favor of a gwz-owned orchestrator. **Lean: one idempotent state machine with recovery, not fictional filesystem/share atomicity.**

### SR56-16 — GAP — workspace selection has no path to the authority and no supplier owns it

- **Spec + missing contract:** `glade-workspaces.md` §2.1/§3 makes `ws.selection` a grip-local “client-context surface”; tool requests then omit the workspace because selection supposedly selects the target.
- **Ground truth:** `GladeSupplierModel.md` lines 47–81 requires a wire-attached authority session. The atlas itself admits glade-workspaces is not one supplier session (`workspaces.ts` lines 11–21). `ws.selection` is not a `Shape` in `GladeDeclSurface.md` lines 24–30.
- **Failure scenario:** a remote gwz/files/terminal provider knows neither the grip-local selection nor which app-owned root to open; the trace's X4 resolution occurs without a frame carrying the choice.
- **Disposition:** address the exchange/subscription in the selected workspace share at mount, or define a real selection authority/surface. **Lean: selection fills the binding route; classify the grip selector separately from the authority supplier.**

### SR56-17 — GAP — `WorkspaceEntry` LWW loses concurrently eligible hosts

- **Spec + missing contract:** workspaces §2.2 requires clone→register eligibility and convergence to multiple hosts.
- **Ground truth:** WD §2, lines 54–60, assigns LWW semantics to `WorkspaceEntry`; current `replicas_of()` selects one latest whole record (`registry.rs` lines 351–363).
- **Failure scenario:** peer2 and peer3 clone from the same `[peer1]` state and append `[peer1,peer2]` and `[peer1,peer3]`; LWW drops one successful replica from eligibility.
- **Disposition:** use per-(workspace,node) eligibility add/remove records, or serialize all edits through one writer. **Lean: convergent per-host records/OR-set.**

### SR56-18 — GAP — the grant-request lifecycle lacks signer, instance, and return-path semantics

- **Spec + missing contract:** `glade-share.md` §§4–5 makes requests “CapabilityGrant-shaped,” auto-files for other recipients, carries caller-supplied `requester-fp`, stores truth in an inaccessible target policy, and promises requester-visible deny/reapprove.
- **Ground truth:** the authz grant sketch (`GladeAuthzModel.md` lines 54–72) has no disposition or request instance; current revocation is principal/share-wide and permanently suppresses later grants (`registry.rs` lines 366–386).
- **Failure scenario:** Alice cannot sign “as Bob” when auto-filing; Mallory can forge `requester-fp`; denied Bob cannot read the target policy to learn the result; revoke then N+1 reapprove remains dead under the current fold.
- **Disposition:** add signed `grant_instance_id`, requester/sponsor/requested-for provenance and a status receipt to the carrying share, or keep requests separate from grants. **Lean: versioned instance schema plus signed receipt.**

### SR56-19 — GAP — link capture and external links have no valid private/account treatment

- **Spec + missing contract:** share §1/§3 captures every exact `(decl, domain/zone/key fill)`; §2 stores the record only in its carrying share; §5 claims an emailed link can knock.
- **Ground truth:** AZ-16 gives a recipient only *their own* private zone and AZ-17 exempts only an account-domain owner (`GladeAuthzModel.md` lines 140–159).
- **Failure scenario:** restoring Alice's captured `self:alice` ref either leaks her private data or substitutes Bob and no longer restores the slice. An emailed recipient cannot read the carrying share's `LinkRecord` or append the rendezvous knock.
- **Disposition:** restrict v1 capture to portable commons/explicit values and readable Glade carriers, or define private-ref rebinding plus an external bootstrap rendezvous. **Lean: restrict and reject unsupported refs/carriers explicitly.**

### SR56-20 — GAP — denial-triggered knock is not stage-1 exercisable

- **Spec + missing contract:** share §7, lines 138–143, calls the whole knock lifecycle stage-1 buildable while saying “nothing blocks”; §5.1 starts only after mounts are denied.
- **Ground truth:** the plan's enforcement switch is P2.S3 (`Plan.md` lines 145–150), and failure-as-data must supply the denial event the tap intercepts.
- **Failure scenario:** under allow-all, the late joiner's mount succeeds, so no knock, ingestion, notification, denial, or approval path is entered; the claimed stage-1 end-to-end exercise is fictitious.
- **Disposition:** move knock user-testability to the gated stage, or add an explicit simulation/test-only denied result. **Lean: gated stage, with the records unit-tested earlier.**

### SR56-21 — GAP — a window key cannot both identify one viewport and populate the base file

- **Spec + missing contract:** files §2.1–2.3 uses canonical key `{path,from,len}` and promises full-body backfill makes the next window local.
- **Ground truth:** `s-window` W2 is keyed to the viewport but W3 omits the key (`window.ts` lines 56–76); current routing/store scope streams by exact `(share,glade_id,key)` (`server.rs` lines 215–237).
- **Failure scenario:** backfill under the viewport key is invisible to `{from:3100,len:100}`; backfill under the base-file key is outside the subscribed stream. A mutable file can also mix viewport and body revisions because no generation/hash is named.
- **Disposition:** define base-resource identity plus separate interest parameters and immutable revision, or permit a specified cross-key reassembly protocol. **Lean: path+revision instance with viewport as interest, not identity.**

### SR56-22 — GAP — blob fetch is neither a valid declaration nor authorization-preserving

- **Spec + missing contract:** files §6 line 136 declares shape `window/exchange`; §3 and §10 also allow a bare hash carrier.
- **Ground truth:** one `BindingDecl` has one Shape (`GladeDeclSurface.md` lines 24–30), and authz checks `(share,glade_id,key)` rather than possession of a content hash (`GladeAuthzModel.md` lines 170–188).
- **Failure scenario:** after `/secret` is hidden or revoked, anyone knowing its BLAKE3 hash fetches it through the bare carrier; a hash-only `ws.blob` key cannot prove which authorized path referenced it.
- **Disposition:** choose one declared fetch exchange and bind it to an authorized source reference/path, or use an attenuated fetch capability. **Lean: declared exchange with reference-derived authorization and explicit failure arms.**

### SR56-23 — GAP — terminal reattach and takeover lack the envelopes their guarantees require

- **Spec + missing contract:** terminal §4, lines 86–100, promises output-offset dedup and driver epochs; §9 promises no byte lost/doubled.
- **Ground truth:** the only proposed payload is input-only `TermIn` (§3, lines 70–77); `ChannelData` is raw `{channel,data}` (`glade.taut.py` lines 164–173). GDL-028 remains open.
- **Failure scenario:** reattach during output cannot locate replay/live overlap, and a stale driver sends bytes with no epoch the authority can fence. The done criterion can pass superficially while duplicating commands or output.
- **Disposition:** define `TermOut{offset,generation,bytes}`, identical scrollback offsets, and channel→driver-epoch binding, or weaken reattach/takeover from v1. **Lean: define the envelopes before implementation.**

### SR56-24 — GAP — SWMR does not fence appends or define the handoff cut

- **Spec + missing contract:** editing §1.2, lines 35–49, says only the lease holder appends and therefore the ordinary log is totally ordered.
- **Ground truth:** `doc.body` remains ordinary multi-origin appends (§6, lines 147–160); the node accepts every incoming op without consulting `EditClaim` (`server.rs` lines 242–260). Unlike `ServeClaim`, there is no filesystem lock.
- **Failure scenario:** A's final position-op is delayed, B takes epoch+1 on an older body, and both origins land. Reader-clock expiry cannot remove the stale op, and `doc.owner` as a latest-value does not prove a body-head cut.
- **Disposition:** every body op carries lease epoch/base head and accepting hops reject stale epochs, or all edits route through an exchange serializer. **Lean: epoch+base fenced ops with catch-up-before-handoff.**

### SR56-25 — GAP — stage-1 ServiceDefinition execution is an allow-all code loader

- **Spec + missing contract:** diff §2, lines 37–46, makes `program` an ordinary content-addressed definition field; §7, lines 167–174, dynamically spawns it under stub-allow-all.
- **Ground truth:** GDL-016 still leaves service-definition authority open (`DecisionLog.md` line 34); content addressing proves bytes, not safety or authorization.
- **Failure scenario:** an untrusted home-share appender registers a matching definition pointing at malicious code, subscribes its key, and G2 executes it consumer-side on the entry node.
- **Disposition:** allow only preinstalled/trusted definitions in the machinery stage, or enforce `service.define` plus a specified sandbox/ABI/resource policy before G2. **Lean: trusted static definitions until the security/runtime contract exists.**

### SR56-26 — GAP — diff teardown cannot erase replicated output or represent staleness

- **Spec + missing contract:** diff §1, lines 29–33, says G5 nulls everything and holds no durable state; §6 defines only a ready result value.
- **Ground truth:** value ops fold and replicate (`GladeSupplierModel.md` lines 61–65), and `s-svc-shared` leaves a warm `ws.diff` replica (`sync.ts` lines 239–243).
- **Failure scenario:** after teardown, source change, or revocation, reopening immediately replays the old value before recompute. Source lapse, denied source, program-fetch error, and compute trap have no result arms.
- **Disposition:** define generation-bound `pending|ready|stale|absent|denied|error` state with claim expiry/tombstone rules, or make output wholly ephemeral/non-replicated. **Lean: explicit state plus revalidation before replay.**

### SR56-27 — GAP — glade-diff is ruled in but has no executable plan step

- **Spec + missing contract:** `SupplierRequirements.md` lines 118–130 calls diff the P3 tail; diff §7 says it drives definition matching, spawn, refcount, claims, and teardown.
- **Ground truth:** `Plan.md` P3 S1–S5 (lines 152–166) ends at terminal; P4 (168–177) has editing and razel only.
- **Failure scenario:** every named GLP-0006 step can complete while no diff runtime, supplier, demo, denial arm, or zero-interest teardown exists, despite diff being normative and user-testable.
- **Disposition:** add an explicit P3-tail sequence, or remove diff from the current program with a new ruling. **Lean: add schemas/runtime/unsubscribe/supplier/demo/live-verify steps.**

## FORCED findings

### SR56-28 — FORCED — AZ-16 already decides chat's group-as-share question

- **Spec + open question:** chat §3.1/§10, lines 76–85 and 199–204, offers own-share groups or new per-key membership grants.
- **Existing ruling:** AZ-16 says membership is `(domain,commons)` and **no zone-scoped grant ever exists** (`GladeAuthzModel.md` lines 153–159 and 313–315). AZ-1 key scoping remains open, not a ratified group-membership plane.
- **Failure scenario:** keeping all groups as keys in one `chat` share grants every group at once; inventing per-key group grants overturns AZ-16 and the ordinary-surface grant model.
- **Disposition:** make each real group its own share, or explicitly reopen/overturn AZ-16. **Lean: group-as-share; keep keyed groups only as the stage-1 migration source.**

## MINOR findings

### SR56-29 — MINOR — Razel's deferral is sound but its stated premise/trigger is stale

- **Spec + claim:** `glade-razel.md` lines 3–7 and 28–33 says no request taxonomy/protocol exists and expansion starts when one is published.
- **Contradiction:** on disk, `razel-dev/razel-wire-api/protocol/razel.taut.py` lines 1–18 and 40–137 is a ratified single-source IR defining build/run/query and an event log. What is not established by the reviewed files is a pinned released contract.
- **Failure scenario:** the trigger “protocol exists” is already true, so the stub can float indefinitely for the wrong reason.
- **Disposition:** retain deferral but change the trigger to a tagged/pinned release, or begin against an explicitly unstable pre-release. **Lean: wait for the release; correct the premise.**

## What held

- The common two-path attachment model—claim-holder op appends for value/log and provider attachment for exchange—matches `server.rs`/`exchange.rs`; specs that preserve that split are coherent.
- The built chat message path holds: one `chat.msgs` log keyed by group, client appends, optional additive `ChatLine.principal`, supplier out of the hot path, and late replay agree across spec, source, and `s-chat`. Runtime declaration does not hold (SR56-10).
- WD-8's single-definition dedup axis is preserved: `per-node` default and `global` opt-in agree across the ruling, diff spec, and `s-svc-shared` SS2/SS3.
- Terminal honestly identifies the current echo-only channel implementation (`server.rs` lines 143–153) and correctly assigns making channel→authority routing to P3.S4.
- Workspaces consistently permit zero-member roots, treat clone as eligibility rather than seizure, and keep name→root storage app-owned; the happy-path traces agree.
- Files consistently keep large bytes out of op chains and put path roots behind the app seam; the unresolved carrier/auth contracts are the defect, not that direction.
- Policy-rides-the-share and AZ-16's commons+recipient-own-private coupling are repeated consistently outside the findings above.
- The glade-gwz per-request-supplier ruling, diff being IN, and gwz-core release canonicality are all cited accurately; the spec's stale/incomplete protocol derivation is the problem.
- The two `grazel-app.glade` fixtures are currently byte-identical. The spec also honestly flags their existing `ws.diff` log/private declaration versus value-shape trace mismatch.
- Razel implementation work remains honestly deferred pending a stable pinned contract, notwithstanding SR56-29's stale wording.

## Disposition summary

Before plan conversion, rule/fix in this order: (1) finish the single gwz/taut codec and regenerate the complete gwz family; (2) close the common requester-context and identity-bound-private-key security seams; (3) restore glade-share's core membership ceremony and take AZ-16's forced group-as-share outcome; (4) define one recoverable, creator-rooted workspace creation path and make selection an actual binding-route input; (5) settle window/blob authorization, editor save/SWMR fencing, and terminal reattach envelopes; then (6) make demand-instantiated code trusted, its replicated result state honest, and add glade-diff to the executable plan. Until those are ruled, the specs cannot be converted into an implementation plan without builders inventing incompatible authority, key, and failure semantics.

## Concordance with F5 (read after the independent findings)

- **Agree:** F5-2/3/4/10/11 correspond to SR56-11/28/22/15/14; both passes independently found the file/edit truth seam, forced group-as-share result, bare-hash leak, create-selection break, and events visibility gap.
- **F5 found additional valid seams:** F5-1 (private-zone scrollback cannot later be granted to watchers), F5-5 (mutable-file reassembler has no build owner), and F5-9 (one retention ruling must cover four consumers) should join the ruling queue.
- **Disagree:** F5's landing verification treats `chat.decl` as a real declaration and calls EditClaim takeover reuse sound; SR56-10 verifies the former never enters `home/dir.bindings`, while SR56-24 shows the latter does not fence body appends or define a handoff cut.
- **This pass found beyond F5:** the moving/stale gwz protocol derivation, lost effect caller context, forgeable private keys, dual-shaped invites, root/device identity split, missing normative direct-share flow, unrecoverable create atomicity, diff code-execution/stale-state/plan gaps, and Razel's stale trigger.
