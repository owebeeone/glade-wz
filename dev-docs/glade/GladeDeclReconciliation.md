# Glade Declaration Reconciliation — `glade-decl` vs the node, 2026-09-21

Status: assessment + amendment proposal. **No code, schema or corpus changed by
this document.** It exists to make the amendment a SINGLE edit.

Trigger: owner instruction "reconcile `glade-decl` with the node first", ahead of
`dev-docs/PackageExtractionPlan.md` step 1.1, which publishes `glade-decl-ts` as
the first leaf of the extraction. Publishing freezes the contract (L1-07,
`gwz-dev/dev-docs/AgentProcessRules.md`), and `glade-decl/corpus/decl.v0.json` is
already declared the FROZEN byte oracle (`glade-decl/README.md:52-58`). So the
amendment must be made ONCE: this document enumerates every divergence and splits
it into what is already decided and what the owner must rule.

Prior art this supersedes in scope, not in fact: `dev-docs/glial/GlialFitAssessment-2026-09-15.md`
§1 found four divergences. All four are confirmed below; thirty-odd more are not
in that document.

---

## 1. Scope and rule of precedence

**"Reconcile with the node" means**: make the typed declaration surface
(`glade-decl/ir/glade_decl.taut.py`) agree with (a) the records the node actually
stores (`glade/node/ir/sysdata.taut.py:78-84`), (b) the grammar the node actually
parses (`glade/node/src/appdecl.rs:15-22, 43-47, 107-140`), (c) the app files
actually loaded (`grazel/apps/*.glade`, `glade-gyld/tests/fixtures/*.glade`,
`glade-gwz/tests/fixtures/*.glade`), and (d) the vocabulary the two clients and
glial actually admit — *before* any of it is frozen by a publish.

It does **not** mean changing the wire. The node's wire IR
(`taut/corpus/glade.ir.json`, rendered at `glade/wire-rs/src/generated.rs`) is a
different contract with a different, frozen `Shape` enum. Both enums are spelled
`Shape` and they are **not** the same enum:

| | declaration `Shape` (`glade-decl/ir/glade_decl.ir.json`) | wire `Shape` (`taut/corpus/glade.ir.json`) |
| --- | --- | --- |
| members | `value:0 log:1 message:2 stream:3 exchange:4 window:5 swmr:6 crdt:7` | `value:0 log:1 stream:2 swmr:3 crdt:4` |
| `stream` | `3` | `2` |
| carries | what a tap declares | what an `Op` is |

Nothing compares them numerically and nothing should start. The wire is out of
scope for this amendment unless a ruling below says otherwise.

### Precedence applied, in order

1. **A ratified decision-log entry outranks both contract and code.** Ratified
   and in force here: GDL-029 is **not** one (root `dev-docs/DecisionLog.md:47`,
   status `open`); GDL-032, GDL-035, GDL-037, GDL-038 are ratified 2026-07-07
   (`:50, :53, :55, :56`); GDL-041 is ratified 2026-08-28 (`:58`); **GDL-039 (the
   zones vocabulary) is NOT ratified** — `:59` reads "open (implemented, pending
   ratify)". That single fact decides the shape of §3: the shape catalogue has a
   ruling to apply, and the zones/domain model has a ruling to *make*.
2. **Where no ratified decision exists, what the node implements and the app
   files already use is EVIDENCE of intent, not a decision.** `grazel-app.glade`
   has said `retention windowed` since P1.S3 and nothing has ever read it; that
   is a fact about the file, not a ratification of a retention vocabulary.
3. **The contract as written is a claim to be tested, not the truth.** Every
   element below was checked against a consumer. An element with no producer and
   no consumer is reported as unexercised, not as "already agreed".
4. **`dev-docs/OpenNotes.md` (N1–N6) is the authoring record, not authority.** It
   documents calls the spec left open; several are exactly the calls this
   document asks the owner to confirm or reverse.

### Method

Files read and cited by line. No builds, no writes outside this file. One
read-only command was run: `python3 corpus/build.py --check` in `glade-decl/`
(finding A2 below).

---

## 2. The divergence table

**51 elements audited: 21 agree, 7 SETTLED, 23 need a RULING.** Elements with no
divergence are listed so this is a complete audit, not a complaint list.
"node" = `sysdata.taut.py` + `appdecl.rs`; "files" = the five live `.glade` files;
"clients" = `glade/client-ts/src/shapes.ts` + `glade/client-rs/src/session.rs`;
"glial" = `glial/src/{shapes,manifest,instance,events}.ts`; "UI" =
`gryth-wz/gryth-ui`.

### 2a. `Shape` — 8 members + 1 absent

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `Shape.value=0` | present | `KNOWN_SHAPES` + `BINDING_SHAPES` (`appdecl.rs:43-44`); 13 of the 28 binding lines in the five live app files | wire `value:0`; `requireOpShape` ✓; `requireMountShapeAdapter` ✓; 6 UI surfaces | GDL-041 engine | — | — | **agrees** |
| 2 | `Shape.log=1` | present | authorable; 13 binding lines | wire `log:1`; both clients ✓; glial ✓; 3 UI surfaces + chat's, via glade-chat | GDL-041 engine | — | — | **agrees** |
| 3 | `Shape.message=2` | present | recognized (`appdecl.rs:43`) but REFUSED as a binding (`:119-124`; test `:324-330`) | absent from wire; `requireShapeAdapter` throws (`glial/src/shapes.ts:72-75`) | GDL-041: "`message` is unsupported" | contract carries a shape every decision and every runtime rejects | delete the member, or retain it reserved-and-unbindable | **RULING (R3)** |
| 4 | `Shape.stream=3` | present | recognized, refused as a binding (`:119`) | wire `stream:2`; `requireOpShape` ✗; `requireMountShapeAdapter` ✗; `requireShapeAdapter` ✓ (`shapes.ts:39-43`) | GDL-041 engine; "recognition MUST NOT imply runtime support" | none in principle — recognized engine with no durable adapter | keep; restate recognition≠support in the schema comment | **SETTLED** |
| 5 | `Shape.exchange=4` | present | refused as a binding with "exchange uses `service`" (`appdecl.rs:121`); authored by the `service` line | not a wire `Shape` (it is `FrameType.exchange_req/res`); `requireExchangeShape` (`shapes.ts:101-103`); `client-rs/src/supplier.rs:156`; `M.gwzOps.shape === "exchange"` (glial GAP-14) | GDL-041: "separate correlated service path" | — | — | **agrees** |
| 6 | `Shape.window=5` | present | recognized, refused as a binding; test `:324-330` | absent from wire; not in glial's `ADAPTERS` | GDL-041: "an application projection over an explicit base shape"; `GlialClientRuntime.md:26-27` "not an engine" | same as row 3 | delete, or retain reserved | **RULING (R3)** |
| 7 | `Shape.swmr=6` | present (added `99a04e0`) | authorable (`appdecl.rs:44`, test `:350-356`); 2 binding lines — `grazel-app.glade:23` `ws.files swmr` | wire `swmr:3`; both clients ✓; glial mount ✓; node `store.rs:160-180` | GDL-041 engine | — | — | **agrees** |
| 8 | `Shape.crdt=7` | present (added `bbce73d`) | **NOT in `KNOWN_SHAPES`** (`appdecl.rs:43` lists 7 names, no `crdt`) → `binding x crdt …` fails "unknown shape" | wire `crdt:4`; `requireOpShape` ✓; `requireMountShapeAdapter` ✓; `store.rs:171` | GDL-041 engine | a shape every runtime carries cannot be declared in an app file | add `crdt` to `KNOWN_SHAPES` and `BINDING_SHAPES` | **SETTLED** |
| 9 | `Shape.atom` *(absent)* | **missing** | absent | `glial/src/shapes.ts:19-23` `atom.oracle/v1`; `gryth-ui/packages/glade/src/runtime.ts:15` `createAtomValueTap` | GDL-041 engine | contract omits a ratified engine its own exported IR's catalogue lists (`glade_decl.ir.json` `shapes`: `unary value atom log stream swmr snapshot_delta crdt text_crdt`) | add `atom=8` — additive, byte-safe | **SETTLED** |

Note on row 9: the *same file* that carries the offending `Shape` enum already
carries the GDL-041 catalogue verbatim in its `shapes` block. The contract
contradicts itself inside one artefact.

### 2b. `Authority` — 2 members

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | `Authority.share=0` | present | `AUTHORITIES` (`appdecl.rs:47`); all 28 binding lines | glial default `"share"` (`manifest.ts:70`); all 9 UI surfaces (defaulted) | DeclSurface §Contents | — | — | **agrees** |
| 11 | `Authority.external=1` | present | accepted by the parser; **zero uses**; the grammar has no token to name the source | never set by any consumer | DeclSurface §Contents: "`share` \| `external(source)`" | declarable but unusable: an `external` binding cannot name its source node-side | keep + extend the grammar, or drop the pair from v1 | **RULING (R5)** |

### 2c. `DomainAnchor` — 3 members

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 12 | `DomainAnchor.account=0` | present | **no `domain` field exists node-side** (`sysdata.taut.py:75-84`); **no token** in the grammar (`appdecl.rs:18`) | `glade/demo/src/manifest.ts:29-30` (`app:status`) mapped by a demo-local `WORKSPACE_MANIFEST.domains` table, not by the contract | GDL-039 **not ratified**; `GladeZones.md:122` "account/universal domain's exact shape" open | the anchor exists only in the contract | see R1 | **RULING (R1)** |
| 13 | `DomainAnchor.document=1` | present | same — absent | glial default (`manifest.ts:72`); all 8 non-default UI surfaces say `'document'` and then hard-code an unrelated `share` | as above | the only value in live use, and it decides nothing | see R1 | **RULING (R1)** |
| 14 | `DomainAnchor.deployment=2` | present | absent | **no consumer anywhere** in either workzone | `GladeZones.md:42` lists a deployment row | speculative; unexercised at freeze | see R1 / R7 | **RULING (R1)** |

### 2d. `ZoneKind` — 2 members

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 15 | `ZoneKind.commons=0` | present | stored as a free `STR` (`sysdata.taut.py:83`), **never validated** (`appdecl.rs:137` takes `toks[4]` raw); 26 of the 28 binding lines say `commons` | glial default (`manifest.ts:73`); `chat/src/live.ts:30` is the only UI *fill* that passes `zone` | GDL-039 not ratified, but the word is used verbatim everywhere | vocabulary agrees; the value never becomes the wire `key` (GlialFitAssessment §5; `GlialFitAssessment` S6 owns the fix) | keep | **agrees** |
| 16 | `ZoneKind.private=1` | present | `grazel-app.glade:24` `ws.diff log share private from-cursor`; unvalidated | node keys private zones `self:<id>` (`glade/node/src/router.rs:133`, `store.rs:409`); **nothing client-side produces that key** | as above | a declared `private` surface is not keyed private by anything on the client path | keep; the mapping is a glial step, not a contract change | **agrees** |

### 2e. `RetentionPolicy` — 3 members + 1 absent

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 17 | `RetentionPolicy.latest=0` | present | free `STR`, unvalidated; 13 binding lines | glial `DEFAULT_RETENTION` (`manifest.ts:64`); nothing enforces it (`glial` GAP-10, GC-4 `GlialClientRuntime.md:87`) | none | vocabulary agrees; wholly unenforced | keep | **agrees** |
| 18 | `RetentionPolicy.from_cursor=1` | present | **13 binding lines across all five files spell it `from-cursor`** (`grazel-app.glade:23,24,32,33`; `gyld-app.glade:47,48`; both fixtures); `appdecl.rs:352` spells it `from_cursor` — both parse, because nothing validates | `glade/demo/src/manifest.ts:30,35,40,46,51,56` uses `from_cursor` on six surfaces, five of them `value`-shaped | none | two spellings for one policy, coexisting only because the field is unchecked | normalize on `from_cursor` (a taut enum member must be an identifier); 13 app-file lines; validate node-side | **SETTLED** |
| 19 | `RetentionPolicy.ttl=2` | present | **no app file uses it** | no consumer; wire `ErrorCode.Retention=5` exists and is never produced | none | unexercised at freeze | see R2 | **RULING (R2)** |
| 20 | `windowed` *(absent)* | **missing** | `grazel-app.glade:25` `binding term.log log share commons windowed` — 2 lines (the file is dual-maintained), loaded by the running demo | no consumer | none. GDL-041 says a window is a projection over a base shape, i.e. not retention | the app file names a retention policy the contract does not have | see R2 | **RULING (R2)** |

### 2f. `ChangeKind` — 2 members

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 21 | `ChangeKind.refresh=0` | present | n/a (client-side vocabulary) | `glial/src/events.ts:57,70,99` | GC-1 ruled 2026-07-07 (`GlialClientRuntime.md:83`) | — | — | **agrees** |
| 22 | `ChangeKind.delta=1` | present | n/a | `glial/src/events.ts:84,123` | as above | — | — | **agrees** |

### 2g. `GladeId`, `Retention` — 3 fields

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 23 | `GladeId.id: STR` | a message wrapping one `STR` | a bare `STR` on the record (`sysdata.taut.py:80`) and on the wire `Op` | `decl.glade_id.id` everywhere (`glial/src/instance.ts:101`; `gryth-ui` 11 sites); `grip-core/src/core/share_decl.ts:28` uses a flat `gladeId: string` | — | wrapper-vs-bare, deliberate (`OpenNotes.md` N6); every consumer unwraps at the boundary | keep | **agrees** |
| 24 | `Retention.policy` | `Ref(RetentionPolicy)` | a free `STR` (`sysdata.taut.py:84`) — there is no `Retention` message node-side | glial always supplies the default; **no UI call site ever writes it** | none | typed message vs free string; no reader on either side | see R2 | **RULING (R2)** |
| 25 | `Retention.ttl_ms` | optional `INT` | absent | no consumer | `OpenNotes.md` N3 records the modelling call | unexercised; its stated coupling to `policy == ttl` is documented, not enforceable in taut | see R2 | **RULING (R2)** |

### 2h. `BindingDecl` — 7 fields + 2 absent

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 26 | `.glade_id` | `Ref(GladeId)` | `STR`, token 1; duplicate ids refused (`appdecl.rs:195-201`, GQ-6) | `glial/src/binder.ts:54`; UI `DECLS` lookup keyed off `glade_id.id` (`gyld/src/live.ts:238-244`) | GQ-6 pinning | as row 23 | keep | **agrees** |
| 27 | `.shape` | `Ref(Shape)` | `STR`, token 2, validated against two lists | gated by `requireMountShapeAdapter` (`binder.ts:50`) | GDL-041 | the member set diverges (rows 3–9); the field does not | keep | **agrees** |
| 28 | `.authority` | `Ref(Authority)` | `STR`, token 3, validated (`appdecl.rs:125-130`) | defaulted, never chosen | DeclSurface | none at field level | keep | **agrees** |
| 29 | `.source` optional `STR` | present; "set iff `authority == external`" | **absent** from the record and from the grammar | `glial/src/manifest.ts:71` sets `null`; no consumer ever sets it | DeclSurface §Contents | the other half of row 11 | see R5 | **RULING (R5)** |
| 30 | `.domain` | `Ref(DomainAnchor)`, field 5 | **absent since the first commit** (`glade b9b1126`) — deliberate: `sysdata.taut.py:73` "no share/key here — the ServeClaim selects the node, the mount fills domain/zone/key" | vestigial in glial: `Fill.domain` (`instance.ts:34-41`) is a *concrete string*, enters only the instance key, and is unrelated to the anchor. Four different meanings in the UI: `'gwz'`, `'gyld'` (`gyld/src/ops/surfaces.ts:64`), a chat group id (`chat/src/live.ts:30`), `'gryth-local'` (`taps.ts:34`) | GDL-039 **not ratified**; `GladeZones.md:123` "Domain anchoring" open | the contract's field 5 has no node counterpart, no app-file syntax, and no consumer that uses it as an anchor | see R1 | **RULING (R1)** |
| 31 | `.zone` | `Ref(ZoneKind)`, field 6 | `STR`, token 4, **unvalidated** — `binding g value share frobnicate latest` parses | glial `Fill.zone` is a separate free string | GDL-039 not ratified | node accepts anything; contract accepts two things | validate node-side against the contract vocabulary (which every file already uses) | **SETTLED** |
| 32 | `.retention` | `Ref(Retention)`, field 7 | `STR`, token 5, unvalidated | never read by any fold, store or engine in either language | none | message vs string; vocabulary mismatch (rows 17–20); zero readers | see R2 | **RULING (R2)** |
| 33 | `app` *(absent from contract)* | — | `sysdata.taut.py:79` field 1 — the node's record carries the owning app | `appdecl.rs:133` sets it from the `app` line | GDL-037 | the node's record is structurally `AdvertisementRecord`-minus-`grip_key`, not `BindingDecl` | document the mapping `sysdata.BindingDecl.app ≡ AdvertisementRecord.package`; no schema change | **SETTLED** |
| 34 | `profile` *(absent everywhere)* | — | absent | **glial REQUIRES it at mount**: `binder.ts:51-53` throws unless `config.crdtProfile === "text_crdt"` for a `crdt` decl | GDL-041 ratifies profiles (`snapshot_delta`, `text_crdt`) as a distinct axis | a `BindingDecl{shape: crdt}` is unmountable without out-of-band config the contract cannot carry | see R4 | **RULING (R4)** |

### 2i. `AdvertisementRecord` — 3 fields

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 35 | `.binding` | `Ref(BindingDecl)` | no producer | no consumer | cites GDL-029, which root `DecisionLog.md:47` lists **open** | the contract encodes an unratified decision's record format | see R7 | **RULING (R7)** |
| 36 | `.package` | `STR` | — | — | as above | grok enumeration does not exist in either workzone | see R7 | **RULING (R7)** |
| 37 | `.grip_key` | `STR` | — | — | as above | — | see R7 | **RULING (R7)** |

### 2j. `OriginMeta`, `ChangeEvent` — 8 fields

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 38 | `OriginMeta.origin` | `STR` | wire `Op.origin` | `glial/src/events.ts:60` | GC-1; `OpenNotes.md` N5 | — | — | **agrees** |
| 39 | `OriginMeta.seq` | `INT` | wire `Op.seq` | `events.ts:60` | as above | — | — | **agrees** |
| 40 | `ChangeEvent.glade_id` | `Ref(GladeId)` | n/a | `events.ts:56` etc. | GC-1 | — | — | **agrees** |
| 41 | `ChangeEvent.shape` | `Ref(Shape)` | n/a | set to the delivery shape; a `text_crdt` profile still emits `"crdt"` (`events.ts:122`) | GDL-041 | — | — | **agrees** |
| 42 | `ChangeEvent.kind` | `Ref(ChangeKind)` | n/a | `events.ts:57,70,84,123` | GC-1 | — | — | **agrees** |
| 43 | `ChangeEvent.base_seq` | optional `INT`; schema comment: "the refresh baseline a **delta** applies against" | n/a | glial populates it on **refresh** too — `events.ts:58` (value), `:71` (`records.length`), `:100` (swmr assembly seq) — and `null` on a crdt refresh (`:124`) | GC-1 | the documented meaning is narrower than the implemented one | amend the comment to "the position this event is anchored at"; no byte change, no consumer change | **SETTLED** |
| 44 | `ChangeEvent.origin_meta` | optional `Ref(OriginMeta)` | n/a | set only when origin+seq are known (`events.ts:60`) | GC-1; N5 | — | — | **agrees** |
| 45 | `ChangeEvent.payload` | `BYTES`, opaque | n/a | interim glial encoding, explicitly temporary (`glial` GAP-5) | GC-1 split | none — the shell is what is frozen | keep | **agrees** |

### 2k. `GladeIdManifest` — 3 fields

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 46 | `.package_id` | `STR` | no producer; every glade id in every app file is a literal | `glial` GAP-14: "GQ-6 NOT implemented (seam left)" | GQ-6 named in DeclSurface; the algorithm is explicitly deferred (`OpenNotes.md` N4) | the record format is frozen before the function that fills it exists | see R6 | **RULING (R6)** |
| 47 | `.grip_key` | `STR` | — | — | as above | — | see R6 | **RULING (R6)** |
| 48 | `.glade_id` | `Ref(GladeId)` | — | — | as above | — | see R6 | **RULING (R6)** |

### 2l. The two documented interfaces + one absent message

| # | Element | Contract | Node / files | Clients / glial / UI | Ratified | Divergence | Resolution | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 49 | `canonical_key(param_shape_ir, params) -> BYTES` | a documented signature (`README.md:47`, `OpenNotes.md` N4) | keys are raw bytes on the wire; private zones are `self:<id>` (`router.rs:133`) | **no implementation anywhere.** Three ad-hoc conventions in use: `groupKey(id)=utf8(id)` (`glade-chat/src/manifest.ts:31-33`), `utf8(runId)` (`gryth-ui .../gwz/src/live.ts:47`), `utf8(conversationId)` (`gyld/src/ops/surfaces.ts:112`) | DeclSurface §Contents lists the interface | a named contract interface with no implementation and three incompatible substitutes | see R6 | **RULING (R6)** |
| 50 | `derive_glade_id(package_id, grip_key) -> GladeId` | a documented signature; golden vectors deferred (N4) | glade ids are literals in every app file | `glial` GAP-14 says the seam is left open on purpose | GQ-6 | all three languages must compute it identically; none computes it | see R6 | **RULING (R6)** |
| 51 | `ServiceDefinition` / ACL seed / workspace entry *(absent from contract)* | — | `sysdata.taut.py:89-92` `ServiceDefinition`; `:60-63` `CapabilityGrant`; `:45-48` `WorkspaceEntry`; grammar lines `service`, `seed`, `workspace` (`appdecl.rs:141-179`) | `client-rs/src/supplier.rs:156`; glial `requireExchangeShape` | **GDL-037 ratified**: the file form of this surface is "its `BindingDecl`s, `ServiceDefinition`s, and **ACL seeds**" (`dev-docs/glade/GladeDeclSurface.md:113-114`) | the contract covers one of the three record kinds a ratified decision assigns to it | see R8 | **RULING (R8)** |

### 2m. Artefact-level divergences (outside the element table, inside the amendment)

| Id | Finding | Evidence | Class |
| --- | --- | --- | --- |
| A1 | All three renderings pin `CONTRACT_VERSION = "99a04e0b…"`, one commit **behind** `glade-decl` HEAD `bbce73d` — the commit that added `crdt=7`. All three nevertheless render `crdt`. The pin understates the contract it renders. | `glade-decl-ts/src/index.ts:18`, `glade-decl-rs/src/lib.rs:38`, `glade-decl-py/src/glade_decl/__init__.py:21`; `git -C glade-decl log`; `git diff 99a04e0..bbce73d -- ir/glade_decl.taut.py` | **SETTLED** — re-pin at the amendment commit |
| A2 | **The drift gate is red today.** `python3 corpus/build.py --check` exits nonzero: `STALE: glade-decl-rs/src/vectors.rs`. Cause: `glade-decl-rs` carries uncommitted **rustfmt-only** edits to `src/api.rs` (284 lines) and `src/vectors.rs` (43 lines), and the generator emits unformatted text. The gate cannot distinguish formatting from content. | run 2026-09-21; `git -C glade-decl-rs status --short` → ` M src/api.rs`, ` M src/vectors.rs`; `git diff` shows only line-wrapping | **SETTLED** — decide once: commit the generator's exact output, or add `cargo fmt` to the documented procedure and to `build.py`'s comparison. Must be green *before* the freeze; the whole lockstep story rests on it |
| A3 | A superseded SKETCH of the same schema lives inside the node repo: `glade/decl/glade_decl.taut.py` (`Shape { value, log, message, stream, exchange, window }`, `domain: STRING`, `zone: STRING`, `retention: STRING` with the vocabulary "ttl / latest / from-cursor") plus `glade/decl/README.md` marked "Status: SKELETON". This is the origin of the node's free-string fields and of the hyphen spelling in row 18. | `glade/decl/glade_decl.taut.py:13,28-30`; `glade/decl/README.md:19-20` | **SETTLED** — archive or banner it as superseded by `glade-decl/`; it is a second answer to a frozen question |
| A4 | The rendering procedure is documented twice and differently: `README.md:69-79` (three separate `-l rust` / `-l typescript` / `-l python` invocations, two with `--with-runtime`) vs `ir/glade_decl.taut.py:31-33` (`-l rust,typescript,python --api-only [--with-runtime]`). Neither documents the **copy** step that puts `glade_decl.ir.json` and `decl.v0.json` into each rendering (all three copies are byte-identical to the contract's, verified). | as cited | **SETTLED** — one procedure, including the copies, since the copies are what the gates read |
| A5 | `glade-decl-ts` is the only TypeScript member with an npm `package-lock.json` (45 687 B, lockfileVersion 3) and no `pnpm-lock.yaml`. `glial`, `grip-react`, `taut-shape-ts`, `glade-chat` and `gryth-ui` are pnpm; `grip-core` carries both. | `find` over both workzones | note only — see §4, tooling |
| A6 | GDL-038 (ratified) says base glade ships `glade-sys.glade`. **No such file exists** in either workzone. | `find . -name glade-sys.glade` → empty | out of scope; adjacent to R8 |
| A7 | `glade/client-rs/src/session.rs:19-30` — `shape_of` accepts `"crdt"` but its error text lists "supported: value, log, swmr". | as cited | **SETTLED** — message fix, rides the amendment |
| A8 | `gryth-ui/packages/glade/src/glade.ir.json` is the **node's WIRE IR** vendored by hand (`runtime.ts:31-32`), not the declaration IR — a different contract, a same-named different `Shape` enum. It is separately stale: `{value:0, log:1, stream:2}` against the source's `{value:0, log:1, stream:2, swmr:3, crdt:4}`. | agent-verified 2026-09-21; `PackageExtractionPlan.md` step 2.6 owns the fix | **out of scope** — do not touch it in this amendment |

---

## 3. The rulings needed

Eight. Each is answerable in a line.

### R1 — Does `BindingDecl.domain` (and `DomainAnchor`) survive, change, or go?

**The question.** The contract says a surface declares which replicated world it
anchors to. The node has never had the field, the app-file grammar has no token
for it, and every live consumer hard-codes a concrete `share` string instead.
Keep the anchor, replace it with the concrete thing, or delete it?

**Why it is a ruling and not settled.** GDL-039 — the zones vocabulary that
introduced domain/zone — is `open (implemented, pending ratify)` (root
`DecisionLog.md:59`), and `GladeZones.md:119-126` still lists "Domain anchoring"
and "whether `domain`/`zone` replace `share`/`key` as the wire field names" as
open. No ratified text decides this.

**Options.**

| | Contract | Node | glial | App files |
| --- | --- | --- | --- | --- |
| **(a) Keep as-is**, document it as declaration-only | no edit | no edit | no edit; `Fill.domain` stays unrelated | no edit |
| **(b) Keep + make it mean something**: glial gains the `(DomainAnchor, ZoneKind, principal) → (share, key)` mapping | no edit | no edit | GlialFitAssessment S6, ~150 LOC; retires 4 hand-rolled mappings | no edit |
| **(c) Replace the anchor with a concrete `domain: STR`** | breaking field-type change; new corpus vectors | grammar gains a token; `sysdata.BindingDecl` gains field 7 | `Fill.domain` becomes redundant | every binding line gains a token |
| **(d) Delete `domain` + `DomainAnchor`** | removes field 5 and an enum; every `BindingDecl` vector changes | node already agrees | `Surface.domain` and `SurfaceSpec.domain` go; `manifest.ts:72` default goes | no edit |

**Recommendation: (a) now, (b) as a separate step.** Reasons: (d) is the only
option that changes `BindingDecl`'s wire bytes for *every* vector, and it deletes
a vocabulary a ratification is still pending on — expensive and premature. (c)
contradicts the node's own stated design (`sysdata.taut.py:73`) and `GDL-037`'s
app-static rule. (a) costs one comment and freezes nothing new; (b) is the work
that makes the field earn its place, and it is glial-side, so it does not
re-freeze the contract. Say so in the schema: *the anchor is a declaration-time
hint; the binder resolves the concrete world, and until S6 lands the app supplies
it.* If the owner instead wants the field gone, say so now — after the publish it
costs a major version.

### R2 — What is the retention vocabulary, and is it enforced?

**The question.** The contract says `{latest, from_cursor, ttl}` + `ttl_ms`. The
app files say `latest`, `from-cursor`, `windowed`. Nothing anywhere reads any of
it. What is the v1 set, and is it a promise or a comment?

**Options.**

| | Contract | Node | glial | App files |
| --- | --- | --- | --- | --- |
| **(a) Keep the three; drop `windowed`** | comment only | validate token 5; normalize spelling | none | `term.log windowed` → `from_cursor` (2 lines, dual-maintained) |
| **(b) Keep three + add `windowed`** | additive `windowed=3` | as (a) | must then define what a window retains | 13 spelling edits only |
| **(c) Keep `{latest, from_cursor}`, drop `ttl`/`ttl_ms`** | removes a member and a field; `Retention` loses its only optional | as (a) | none | as (a) |
| **(d) Demote `retention` to a free `STR`, matching the node** | `Retention` message deleted; `BindingDecl.retention: STR` | already free | `DEFAULT_RETENTION` becomes `"latest"` | as (a) |

**Recommendation: (a), plus validate it node-side.** `windowed` is not a
retention policy — GDL-041 rules a window an application projection over a base
shape, so `term.log` wants `from_cursor` with an app-side window, and keeping
`windowed` would encode a category error in a frozen enum. `ttl` stays because
it is cheap, additive-safe, and GC-4 (`GlialClientRuntime.md:87`) names retention
enforcement as live future work — deleting it now would cost a breaking re-add.
(d) is the honest "it means nothing" answer but throws away the only typed
vocabulary either side has. Whatever is chosen, **the spelling normalizes to
`from_cursor` and the node starts validating the token** — that part is settled
(row 18/31) and should ride the same commit.

### R3 — `message`, `window`: delete the members, or retain them reserved?

**The question.** GDL-041 says `message` is unsupported and `window` is not an
engine. Every runtime refuses both. Does the enum lose the members, or keep them
as reserved-and-unbindable names?

**Options.**

| | Contract | Corpus | Node | glial | UI |
| --- | --- | --- | --- | --- | --- |
| **(a) Retain reserved**, comment says recognition ≠ support; add `atom=8` | comment + one additive member | every existing vector's bytes unchanged; two vectors get relabelled as recognition cases | no edit (`appdecl.rs:40-42` already implements exactly this posture) | no edit | no edit |
| **(b) Delete both members** | two members go; numbering for `stream..crdt` must be held to stay byte-stable | `edge/binding-message-private` and `edge/binding-deployment-window` are removed → the corpus content changes even where bytes do not | `KNOWN_SHAPES` loses two | no edit | no edit |

**Recommendation: (a).** Three reasons. GDL-041's own words are "Recognition MUST
NOT imply runtime support" — which is an argument for keeping the name, not
deleting it. The node already implements (a) verbatim and says why:
"Legacy wire/declaration names remain recognizable so diagnostics can be precise
and numeric wire values remain reserved" (`appdecl.rs:39-42`). And (a) makes the
v1 corpus a strict superset of v0's bytes, which makes the compatibility proof in
§4 free. Note `PackageExtractionPlan.md:155` phrases the option as "`message`/
`window` out, `atom` in" — that is a plan's shorthand, not a ratification, and
this is the ruling that settles it either way.

### R4 — Does `BindingDecl` carry the shape profile?

**The question.** GDL-041 ratifies `snapshot_delta` and `text_crdt` as profiles
over `swmr`/`crdt`. glial *requires* the profile at mount and throws without it
(`binder.ts:51-53`). The contract cannot express it, so a `BindingDecl{shape:
crdt}` is not independently mountable.

**Options.** (a) Add an optional `profile: STR` to `BindingDecl` — additive, one
field, absent on every existing vector so v0 bytes are unaffected; glial reads it
instead of `config.crdtProfile`; app files gain an optional trailing token.
(b) Leave it mount config — the contract stays smaller, and every `crdt` decl
keeps needing out-of-band setup that no app file can carry.

**Recommendation: (a).** A declaration that cannot be mounted from its own
contents is not a declaration. The field is additive and costs one optional
`STR`; the alternative is that the node can never author a `crdt` binding even
after R3 adds `crdt` to its grammar — which would make row 8's fix hollow.

### R5 — Do `Authority.external` and `BindingDecl.source` survive v1?

**The question.** `external(source)` is in the ratified DeclSurface §Contents, is
accepted by the node's parser, and has zero uses. The app-file grammar has no
token to name the source, so an `external` binding can never be complete.

**Options.** (a) Keep both and add an optional `source <name>` token to the
grammar (~15 LOC in `appdecl.rs`, one `STR` field on `sysdata.BindingDecl`).
(b) Keep both, declaration-only, and document that the node cannot author
`external` yet. (c) Drop `external` and `source` from v1 — removes an enum member
and a field from every `BindingDecl` vector.

**Recommendation: (b), with (a) when a bridged source actually appears.**
DeclSurface §Contents is ratified text naming `external(source)`, so (c) needs
its own amendment to that document and buys nothing at freeze time. (a) is real
work for a feature with no caller. (b) costs one sentence and leaves the pair
additive-safe. Record it explicitly as "declared, not yet authorable" so it is
not mistaken for working.

### R6 — Must `canonical_key` and `derive_glade_id` be fixed before the publish?

**The question.** Both are documented signatures with no implementation in any
language and no golden vectors (`OpenNotes.md` N4 defers them). `GladeIdManifest`
is frozen as the *record* of a derivation nothing performs, and three
incompatible key conventions are live (row 49).

**Options.** (a) Publish with both explicitly labelled DEFERRED in `README.md`
and the DecisionLog; `GladeIdManifest` ships as an unexercised forward
declaration. (b) Implement `derive_glade_id` + its golden vectors before the
freeze (the hash is unchosen; all three languages must agree — realistically a
package of its own). (c) Publish and *remove* `GladeIdManifest` until the
function exists.

**Recommendation: (a), with one hardening.** Blocking the first leaf of the
extraction on an unchosen hash would stall the whole TypeScript track
(`PackageExtractionPlan.md:150`). But `README.md:42-50` currently reads as if the
interfaces are part of the contract; amend it to say they are **not implemented
and not oracled in v1**, and add the same line to `dev-docs/OpenNotes.md` N4 with
the version it is deferred past. `canonical_key` additionally needs a named
owner: three live conventions are already diverging, and freezing a contract that
claims a canonical key while three exist is the kind of claim L1-07 forbids.

### R7 — Which unexercised elements ship in the frozen v1?

**The question.** `AdvertisementRecord` (3 fields) cites GDL-029, which the root
DecisionLog still lists **open**; `DomainAnchor.deployment` and
`RetentionPolicy.ttl` have no consumer either. Freezing a record format for an
open decision is the failure mode L1-08 exists to prevent.

**Options.** (a) Ship them, with each one's status written into `README.md` and
its citation corrected from "(GDL-029)" to "(GDL-029, open)". (b) Hold
`AdvertisementRecord` out of v1 and add it when GDL-029 ratifies — additive, so
no cost later. (c) Ratify GDL-029 first.

**Recommendation: (b) for `AdvertisementRecord`, (a) for the two enum members.**
A three-field record for an enumeration mechanism that exists nowhere in either
workzone is the cheapest thing in this document to defer, and adding a message
later is strictly additive. The two enum members cost nothing to carry and their
deletion would be breaking. Either way the citations must be corrected — a
contract that cites an open decision as if ratified is a precedence defect, not
a typo.

### R8 — Does the contract cover the whole `.glade` file form, or only bindings?

**The question.** GDL-037 (ratified) assigns three record kinds to this surface's
file form — `BindingDecl`s, `ServiceDefinition`s and ACL seeds
(`dev-docs/glade/GladeDeclSurface.md:113-114`). The contract has only the first.
The node has all three plus `WorkspaceEntry`.

**Options.** (a) Grow the contract with `ServiceDefinition` and an ACL-seed
record, rendered in all three languages — makes the ratified file form typed
end-to-end, and is additive. (b) Keep `glade-decl` as the tap/binding vocabulary
only; the other record kinds stay node `sysdata`; amend `GladeDeclSurface.md` to
say so.

**Recommendation: (b), amending the ratified text.** `glade-decl`'s stated
exclusion is "No runtime, no wire, no folds, no sessions, no persistence"
(`glade-decl/dev-docs/DeclSurface.md:36-38`); a `CapabilityGrant` is an ACL fold input and a
`ServiceDefinition` names a routed provider — both need the node to mean
anything, so both fail the module's own admission test. But GDL-037 says
otherwise, so this needs the owner's word and an explicit amendment sentence in
`GladeDeclSurface.md`, not a silent omission. Related and separate: GDL-038's
`glade-sys.glade` does not exist (A6).

---

## 4. The amendment, as it would be made once the rulings are in

One schema edit, one corpus version, three regenerations, the node, three
consumers — in this order. Everything below assumes the recommended answers; substitute freely.

### 4.1 Schema edits — `glade-decl/ir/glade_decl.taut.py`

1. `Enum("Shape", …)` — add `atom=8`. Keep `message=2` and `window=5` with their
   numbers (R3a). Rewrite the comment above it to state recognition ≠ support and
   to name GDL-041 as the catalogue's owner. *(rows 3, 6, 9)*
2. `Msg("BindingDecl", …)` — add `F("profile", 8, STR, optional=True)` (R4a).
   *(row 34)*
3. `RetentionPolicy` — unchanged under R2a; rewrite the comment to say the policy
   is declarative and unenforced pending GC-4. *(rows 17–20)*
4. `ChangeEvent.base_seq` comment — "the position this event is anchored at
   (the baseline a delta applies against; the current position on a refresh)".
   *(row 43)*
5. `Msg("AdvertisementRecord", …)` — removed under R7b, or its comment corrected
   to "(GDL-029, open)". *(rows 35–37)*
6. Module docstring + `dev-docs/OpenNotes.md` N3/N4 — record the deferrals
   (R6a) and the new notes N7 (profile) and N8 (domain is declaration-only).

Nothing else in the schema moves. Field numbers 1–7 of `BindingDecl` and every
existing enum number stay exactly where they are, which is what keeps §4.2 cheap.

### 4.2 The corpus — `decl.v1.json` **replaces** `decl.v0.json`

**What the repository's own rule is.** `glade-decl/README.md:52-58` names
`corpus/decl.v0.json` "the FROZEN oracle" and describes exactly one of them.
`corpus/build.py:35` hard-codes a single `GOLDEN` path; `--check` compares that
one file (`:172-184`). Each rendering holds one byte-identical copy at one fixed
name and its gate reads that name: `glade-decl-ts/src/decl.v0.json`
(`src/corpus.test.ts:18`), `glade-decl-py/src/glade_decl/decl.v0.json`
(`tests/test_corpus.py:27`), and `glade-decl-rs/src/vectors.rs` (generated table).
The sibling precedent agrees: `taut-shape/corpus/` has nine corpora but never two
versions of the same one — the suffix is *that contract's* version, one live file
each.

**Conclusion: the README implies replacement, not coexistence.** Add
`corpus/decl.v1.json` and delete `corpus/decl.v0.json` in the same commit;
retarget `build.py:35`, both language gates, and `README.md:52-63`. v0 stays
recoverable at commit `bbce73d`, which is its historical oracle. Two live
corpora would require `build.py` to hold two schema versions (it loads exactly
one, `:33, :48-52`) and would leave the renderings' gates ambiguous about which
file is authoritative — the opposite of what the oracle is for.

**Compatibility, if wanted.** Under R3a the v1 corpus is a strict superset of
v0's bytes: no enum number moves, the new `profile` field is absent on every
existing vector, and every v0 vector re-encodes identically. If the owner wants
that proven rather than asserted, the precedent is a **separate** artefact in the
taut-shape idiom — `taut-shape/release/compatibility.v1.json` — not a second
corpus. It is a ~40-line addition and it is the cheapest argument in favour of
R3a.

### 4.3 Regenerate the three renderings

By the documented procedure (`glade-decl/README.md:69-79`), after A4 collapses it
to one canonical form that includes the copy step:

```sh
# from glade-decl/
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o ../glade-decl-rs -l rust       --api-only --with-runtime
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o ../glade-decl-ts -l typescript --api-only --with-runtime
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o ../glade-decl-py -l python     --api-only
python3 corpus/build.py          # rewrites ir/*.ir.json, corpus/decl.v1.json,
                                 # and ../glade-decl-rs/src/vectors.rs
# then copy glade_decl.ir.json + decl.v1.json into each rendering's gate location
```

Then, in one commit per rendering: re-pin `CONTRACT_VERSION` to the amendment
commit in all three (A1), and settle A2 — either commit the generator's exact
unformatted output, or add `cargo fmt` to the procedure *and* to `build.py`'s
comparison. **A2 must be green before anything is published**; a drift gate that
is red for cosmetic reasons trains everyone to ignore it.

### 4.4 The node and the app files

Not a consumer — the other half of the reconciliation, and it lands in the same
wave as §4.1 so the two vocabularies are never out of step in a commit:

- `glade/node/src/appdecl.rs:43-44` — `KNOWN_SHAPES` gains `crdt` and `atom`;
  `BINDING_SHAPES` gains `crdt` (row 8). Keep the comment at `:39-42`; it is
  already the correct posture.
- `appdecl.rs:107-140` — validate token 4 against `{commons, private}` (row 31)
  and token 5 against the R2 policy set (rows 17–20), with the same
  line-numbered diagnostics as the shape and authority checks.
- Under R4a, the grammar gains an optional trailing `profile` token and
  `sysdata.taut.py` a field 7 — which means regenerating `sysdata.rs` with
  `--legacy-codec` and accepting a durable-record change. That is the one place
  a ruling reaches into stored data; weigh it in R4.
- App-file data: `from-cursor` → `from_cursor` on 13 lines across 5 files, and
  `windowed` → `from_cursor` on 2 (row 18, row 20). `grazel-app.glade` is
  dual-maintained byte-identical in two homes (`grazel/apps/` and `glade/apps/`,
  see its own header note) — edit both or the node tests fail.
- Gate: `cargo test -p glade-node` (`appdecl.rs`'s 7 tests, incl. the 7-binding /
  11-record grazel assertions at `:296` and `:408`).

### 4.5 Consumers, in order

1. **`glial`** (first — it is the only typed consumer of the changed names).
   `src/shapes.ts` gains nothing (it is already GDL-041-correct). `src/manifest.ts`
   — `SurfaceSpec`/`Surface` gain optional `profile`; `toSurface` defaults it;
   `DEFAULT_RETENTION` unchanged. `src/binder.ts:50-53` reads `decl.profile`
   ahead of `config.crdtProfile`, keeping the config path for back-compat.
   Gate: `vitest run` + `tsc --noEmit` (GAP-14's `@ts-expect-error` wall).
2. **`grip-core` types.** `src/core/share_decl.ts:17` is the single import line;
   nothing in it breaks under an additive change. It must still be republished at
   `0.3.0` (`PackageExtractionPlan.md:157`) because its published `dist/index.d.ts`
   references an unpublished `@owebeeone/glade-decl`. If R1 or R5 had removed
   `DomainAnchor` or `Authority`, `share_decl.ts:35,39` would break — they do not
   under the recommendations, which is itself an argument for them.
3. **`gryth-wz/gryth-ui`** (last, and probably a no-op). It names **zero**
   glade-decl types; its whole contact is untyped object literals passed to
   `defineManifest` at four sites (`packages/plugins/gwz/src/live.ts:32`,
   `packages/plugins/gyld/src/ops/surfaces.ts:28`, `src/taps.ts:12`, and
   `glade-chat/src/manifest.ts:77` on its behalf). All nine of its surfaces get
   `authority`, `source` and `retention` from glial's defaults — none chosen.
   Sequencing note: gryth-ui resolves the contract through a **duplicate
   checkout**, `gryth-wz/glade-decl-ts` (`pnpm-workspace.yaml` reaches outside the
   repo root; `overrides` forces the singleton). Both checkouts are at `7e16e32`
   and clean today. Either land Phase 0.3 of the extraction plan first, or update
   both checkouts in the same pass — otherwise the singleton override silently
   pins the old contract.

### 4.6 What must NOT change

- **The node's wire IR.** `taut/corpus/glade.ir.json`, `glade/wire-rs/src/generated.rs`,
  `glade/client-ts/src/taut/`. The wire `Shape{value:0, log:1, stream:2, swmr:3,
  crdt:4}` numbering is frozen and shares only a name with the declaration enum
  (§1). `atom` does **not** enter the wire in this amendment.
- **`gryth-ui/packages/glade/src/glade.ir.json`** — that is the wire IR vendored
  by hand, not the declaration IR, and it is separately stale (A8). Its fix is
  `PackageExtractionPlan.md` step 2.6. Touching it here would smear two
  unrelated contracts into one commit.
- **`glade/node/src/sysdata.rs`** unless R1/R2/R4/R5 add a record field; it is
  generated with `--legacy-codec` by its own procedure
  (`glade/node/ir/sysdata.taut.py:19-25`) and changing it changes durable records.
- **The running demo's behaviour.** The only app-file data edits under the
  recommendations are `from-cursor` → `from_cursor` (13 lines across 5 files) and `windowed` →
  `from_cursor` (2 lines) — fields nothing reads.

### 4.7 The checks that prove it

| Check | Where | Proves |
| --- | --- | --- |
| `python3 corpus/build.py --check` | `glade-decl/` | IR + corpus + rs vectors in lockstep (currently **red**, A2) |
| `pnpm test` (`vitest run`) | `glade-decl-ts/` | independent TS codec reproduces every v1 byte |
| `cargo test` | `glade-decl-rs/` | independent Rust codec reproduces every v1 byte |
| `pytest` | `glade-decl-py/` | reference codec round-trips |
| `cargo test -p glade-node` | `glade/` | `appdecl.rs` tests — 7 bindings, 11 records, the new `crdt`/zone/retention validation, both spellings |
| `vitest run` + `tsc --noEmit` | `glial/` | `Surface extends BindingDecl` still holds; the compile wall still errors on an undefined key |
| `pnpm test` + `pnpm build:gyld` | `gryth-ui/` | 58 vitest suites, incl. `packages/plugins/chat/src/groups.test.ts:23-40` (the only decl-field assertions in the repo) |
| manual: `gyld-ui.py start` reaches its published-builds line | — | the running demo untouched |

### 4.8 Tooling facts found

- `glade-decl-ts` carries `package-lock.json` only (A5). The owner's standing
  rule is pnpm for *commands* (`pnpm test`, `pnpm install`) and **no unasked
  lockfile migration** — so run pnpm, leave the npm lockfile alone, and raise the
  migration as its own question. `PackageExtractionPlan.md` step 1.2 is where it
  naturally belongs.
- `glade-decl` itself has no build manifest and is not a package
  (`PackageExtractionPlan.md:72`); `corpus/build.py` needs its siblings at fixed
  relative positions (`:36, :39`). Step 4.7 of that plan owns it.
- `glade-decl-py` bootstraps taut by `sys.path` at `tests/conftest.py:16`; per
  the workspace memo, use `/opt/homebrew/bin/python3` (the asdf 3.10 default
  lacks `tomllib`).

---

## 5. Should this go through the adversarial review loop before the freeze?

**Yes — the full loop, three axes, before any publish.** This amendment is
precisely the object `AgentProcessRules.md` L1-09 names: a change to a shared
interface and a compatibility rule, at a release boundary. L1-18 makes two
independent review axes mandatory there, and its 2026-09-18 amendment (operator
decision D7) adds a third, **Surface**, mandatory at "every interface freeze —
any package that fixes a command family, a flag set, a settings block, **a file
format people edit**, or an API". `<app>.glade` is a hand-edited file format and
this amendment changes its accepted vocabulary, so the Surface axis is not
optional here. L1-17 requires the tree settled and committed first — which means
A2 (the red drift gate) and `glade-decl-rs`'s uncommitted rustfmt diff must be
resolved *before* the loop starts, not during it.

The economic argument is the same one that produced this document: publishing
freezes the contract, the corpus is a byte oracle, and the amendment is
one-shot. A missed element does not cost a patch — it costs a second corpus
version, three regenerations, a republish of `@owebeeone/glade-decl` and
`@owebeeone/grip-core`, and a second freeze. Two reviewers reading this table
independently against the code is cheap by comparison, and the Surface reviewer
reading only the app-file grammar and the READMEs is the one most likely to
catch what a schema-focused pair would not: that `binding <glade_id> <shape>
<authority> <zone> <retention>` is a five-token line people type by hand, and
every ruling above changes what it accepts.

---

## Appendix — files read

Contract: `glade-decl/{README.md, ir/glade_decl.taut.py, ir/glade_decl.ir.json,
corpus/build.py, corpus/decl.v0.json, dev-docs/DeclSurface.md,
dev-docs/OpenNotes.md}`. Renderings: `glade-decl-ts/{package.json,
src/{index,api,corpus.test}.ts}`, `glade-decl-rs/src/{lib,api,vectors}.rs`,
`glade-decl-py/{src/glade_decl/{__init__,api}.py, tests/test_corpus.py}`.
Node: `glade/node/{ir/sysdata.taut.py, src/{appdecl,registry,store,router,sysdata}.rs}`,
`glade/{client-ts/src/shapes.ts, client-rs/src/{session,supplier}.rs,
wire-rs/src/generated.rs, decl/*, demo/src/manifest.ts}`, `taut/corpus/glade.ir.json`.
App files: `grazel/apps/{grazel,gyld}-app.glade`, `glade/apps/grazel-app.glade`,
`glade-{gwz,gyld}/tests/fixtures/*.glade`. Consumers:
`glial/src/{shapes,manifest,instance,events,binder,supplier/index}.ts`,
`glial/dev-docs/DecisionLog.md` (GAP-2, 5, 7, 10, 14), `grip-core/src/core/share_decl.ts`,
`glade-chat/src/manifest.ts`, `gryth-wz/gryth-ui/**` (agent-verified 2026-09-21).
Decisions: root `dev-docs/DecisionLog.md`, `dev-docs/glade/{GladeDeclSurface.md}`,
`glade/dev-docs/GladeZones.md`, `dev-docs/glial/{GlialClientRuntime.md,
GlialFitAssessment-2026-09-15.md}`, `dev-docs/PackageExtractionPlan.md`,
`gwz-dev/dev-docs/AgentProcessRules.md` (L1-07..09, L1-17..19, L2-02, L3-06/08).
