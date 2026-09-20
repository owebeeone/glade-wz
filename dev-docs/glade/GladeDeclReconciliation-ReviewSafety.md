# GladeDeclReconciliation — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/glade/GladeDeclReconciliation.md` at glade-wz root `b132b7e6d365` (619 lines; status "assessment + amendment proposal"; a DRAFT design ahead of an interface freeze and the first publish of the `glade-decl` contract to a public package registry). Reviewed 2026-09-21.

**Baseline:** glade-wz root `b132b7e6d365`. Members: `glade-decl bbce73d67146` · `glade-decl-ts 7e16e324630a` · `glade-decl-rs 555a97746fc6` · `glade-decl-py 1b0f6d1f7886` · `glade 960c9b0fa038` · `glial 0dfe4b930063` · `grip-core 97ff6c26f12e` · `grazel 924cb3c4bab9` · `glade-gyld 024d2a8ae061` · `glade-gwz e53c87dddb8f` · `glade-chat 9238d21f6a36` · **`taut 7a5f616c3a9f`** (read and recorded per the prompt). `/Users/owebeeone/limbo/gryth-wz/gryth-ui 3af64c2bab74`. Sources read with `git -C <repo> show <sha>:<path>` for every repo with a dirty or possibly-dirty tree (`glade`, `glade-decl`, `glade-decl-*`, `glial`, `grip-core`); working-tree reads used only for clean paths (`taut/src`, app files, `dev-docs`). Tuple verified identical at start and at end of the review; root `git status --short` clean. No writes, no builds, no runs — inspection commands only.

**Date:** 2026-09-21

**Axis:** Safety — what the text permits to go wrong: degraded and mixed-version paths, irreversible steps and their preconditions, disclosure scale, stuck states reachable under the text's own rules, and whether its "never worse than the status quo" claims survive concrete interleavings. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: NO-GO** — 2 × P1, 3 × P2, 3 × P3 open. Every blocking finding has a bounded, text-fixable remedy. **I pre-commit to GO on a revision that resolves P1-1, P1-2, P2-3, P2-4 and P2-5 as specified below.**

---

## 0. Evidence base

**Object:** `dev-docs/glade/GladeDeclReconciliation.md` (619 lines), read in full. Sections load-bearing for this axis: §1 precedence, §2 rows 3–9, 11, 16, 18, 20, 29–34, §2m A1–A4, §3 R1–R8, §4.1–§4.8, §5.

**Contract and its encoding**
- `glade-decl/ir/glade_decl.taut.py` (at `bbce73d67146`) — `BindingDecl` fields 1–7; `source` (4) and `Retention.ttl_ms` (2) are the existing `optional=True` cases.
- `taut/src/taut/wire/codec.py:12` — *"optional -> always emitted; None -> CBOR null (deterministic; no omission)"*; `:88` — `out[f.tag] = None if fv is None else _to_wire(...)`.
- `taut/docs/Reference.md:79-84, 95-104, 224-236` — tags are the stable contract; `optional` encodes as CBOR `null`; `reserved` / `next_id` are the retirement facility; forward-compat captures unknown tags under `__unknown__`.
- `taut/src/taut/ir/validate.py:45-58, 70-75, 77-80` — reservations are enforced only when declared; enums are checked only for duplicate wire values (gaps are legal).
- `taut/src/taut/corpus/synth.py:32-34` — `members[seed % len(members)]`; seed is the field tag.
- `glade-decl/corpus/build.py:33-36` (`GOLDEN`, `RS_VECTORS`), `:73-140` (hand-authored `curated_values`), `:170-184` (`--check` compares exactly `[IR_JSON, GOLDEN, RS_VECTORS]`).
- `glade-decl/corpus/decl.v0.json` — 26 vectors; 9 typed `BindingDecl`, 2 typed `AdvertisementRecord`.
- `glade-decl-ts/src/api.ts:19-27` — `source: string | null` (required property, nullable); `glade-decl-ts/src/index.ts:18` — `CONTRACT_VERSION = "99a04e0b…"`.
- `glade-decl-ts/src/corpus.test.ts:16-19` and `glade-decl-py/tests/test_corpus.py:24-25` — both gates read **local** copies of `glade_decl.ir.json` and `decl.v0.json`.
- `glade-decl/README.md:52-58` — the frozen-oracle rule and its "every shape / domain / zone / retention policy" coverage claim.

**Hand-decoded corpus bytes** (the decisive evidence for §1 of the findings)

| vector | hex | reading |
|---|---|---|
| `edge/binding-account-commons` | `a7 01 a1016d61636374…  02 00 03 00 04 f6 05 00 06 00 07 a2 0100 02 f6` | `a7` = **map of 7**; tag 4 (`source`, optional, unset) = **`f6` = CBOR null**; nested `Retention` = `a2 … 02 f6` |
| `BindingDecl` (synth) | `a7 01 a101627331 02 02 03 01 04 627334 05 02 06 00 07 a2 0101 02 19012c` | tag 2 = `02` → `Shape` member **index** 2 (`message`), confirming `seed % len(members)` with seed = tag |

**Node**
- `glade/node/ir/sysdata.taut.py` — `sysdata.BindingDecl` fields 1–6 (`app, glade_id, shape, authority, zone, retention`); regeneration note names `--legacy-codec` (fail-open).
- `glade/node/src/sysdata.rs:128-137` — `to_cbor` emits a fixed **6-entry** `Cbor::Map`; `:138-147` `from_cbor` is fail-open.
- `glade/node/src/appdecl.rs:43-47` (`KNOWN_SHAPES`/`BINDING_SHAPES`/`AUTHORITIES`), `:107-140` (token 4/5 stored raw at `:137-138`), `:204-211` (`load` → `io::ErrorKind::InvalidData`), `:228-271` (`register`), **`:264`** (`if existing.iter().any(|(g, p)| *g == glade_id && *p == payload)`), `:280-281` (tests read `CARGO_MANIFEST_DIR/../apps/grazel-app.glade`), `:404-416` (`registering_twice_appends_nothing`).
- `glade/node/src/registry.rs:44-50, 60-80, 190-210, 236-255, 330-336` — `RegistryApi` exposes `who_serves / replicas_of / grants_for / nodes_of / snapshot`; **there is no binding query and no LWW fold for `dir.bindings`**.
- `glade/node/src/exchange.rs:69-77` — the only production reader of `dir.bindings`, an `any()` over every op; `:651-658` — a client SUBSCRIBEs to `dir.bindings` and receives the records as Ops.
- `glade/node/src/bin/glade-node.rs:49` (`async fn main() -> std::io::Result<()>`), `:88` (`appdecl::load(path)?`), `:89-91` (`register` then `store.save`).
- `glade/node/src/router.rs:126-137`, `store.rs:402-421,570`, `server.rs:382-404` — privacy-by-keying implemented and tested; **every `self:` literal is inside `#[cfg(test)]`**.

**App files** (census run against the working tree)

| file | repo | `from-cursor` | `windowed` | bindings |
|---|---|---|---|---|
| `grazel/apps/grazel-app.glade` | grazel | 4 | 1 | 7 |
| `glade/apps/grazel-app.glade` | glade | 4 | 1 | 7 |
| `grazel/apps/gyld-app.glade` | grazel | 2 | 0 | 7 |
| `glade-gyld/tests/fixtures/gyld-test-app.glade` | glade-gyld | 2 | 0 | 6 |
| `glade-gwz/tests/fixtures/gwz-test-app.glade` | glade-gwz | 1 | 0 | 1 |

`diff grazel/apps/grazel-app.glade glade/apps/grazel-app.glade` → identical. Totals 13 and 2 — the object's counts are correct. `grazel/apps/grazel-app.glade` is exercised only by `grazel/tests/integration.rs:109,280,454`.

**Consumers**
- `glial/src/shapes.ts:8-10, 18-52, 72-75, 101-122` (`atom` is a delivery adapter but not a mount adapter); `src/manifest.ts:64-80` (`toSurface`, `DEFAULT_RETENTION`); `src/binder.ts:50-53` (the `crdtProfile` throw); `src/instance.ts:29-41` (`Fill` is free strings; `instanceKey` is a **local** store key).
- `grip-core/src/core/share_decl.ts:17,28-40` — type-only import; its own flat `ShareDecl`, not a `BindingDecl` producer.
- Zero `self:` key producers in `glial/src`, `glade/client-ts/src`, `glade-chat/src`, `gryth-ui/packages`, `gryth-ui/src` (only CSS `align-self`).

**Controlling documents**
- root `dev-docs/DecisionLog.md:47` GDL-029 `open`; `:58` GDL-041 **ratified 2026-08-28**; `:59` GDL-039 `open (implemented, pending ratify)` — its text states *"privacy is a key (AZ §4a)"*.
- `dev-docs/PackageExtractionPlan.md:147-157` — Phase 1 order; step 1.1's exit check is *"`pnpm test` in `glade-decl-ts`; the contract hash in `src/index.ts` and `glade-decl-rs/src/lib.rs` still agree"*; step 1.2 is the publish.
- `gwz-dev/dev-docs/AgentProcessRules.md:240-252` (L1-07), `:370-380` (L1-17).
- `git -C glade-decl-rs status --short` → ` M src/api.rs`, ` M src/vectors.rs` (A2 confirmed red; **out of scope** per the prompt, reported by the object itself).

---

## 1. Findings

### [P1-1] The amendment's load-bearing byte-compatibility claim is false: taut emits absent optional fields as explicit CBOR null, so adding `BindingDecl.profile` changes 11 of the 26 oracle vectors

**Location.** §4.1 item 2 (`add F("profile", 8, STR, optional=True)`); §4.1 closing sentence ("Field numbers 1–7 … stay exactly where they are, which is what keeps §4.2 cheap"); §4.2 "Compatibility, if wanted" ("the new `profile` field is absent on every existing vector, and every v0 vector re-encodes identically"); §3 R4 option (a) ("absent on every existing vector so v0 bytes are unaffected"); §3 R3 recommendation, third reason ("(a) makes the v1 corpus a strict superset of v0's bytes, which makes the compatibility proof in §4 free").

**Root cause.** The document treats taut `optional=True` as protobuf-style *omittable*. Taut defines it as *nullable and always present*.

**Violated invariant.** A frozen byte oracle's stated compatibility property must be true of the bytes the codec actually produces. L1-07 forbids using freeze words as claims the implementer may not verify.

**Reproduction (byte-exact).** `taut/src/taut/wire/codec.py:12` states the rule; `:88` implements it: `out[f.tag] = None if fv is None else _to_wire(...)` — the key is written unconditionally. The frozen corpus confirms it: `edge/binding-account-commons` begins `a7` (a **7**-entry map, one per declared field) and carries `04 f6` — tag 4 is `source`, which that vector leaves unset, encoded as an explicit CBOR null. `glade-decl-ts/src/api.ts:23` mirrors it in the type system: `source: string | null`, a *required* property.

Adding `F("profile", 8, STR, optional=True)` therefore rewrites every `BindingDecl` encoding from `a7 …` to `a8 … 08 f6` (and to `08 62 7338` for the synth vector, which `synth.py` fills). `decl.v0.json` holds **9** `BindingDecl`-typed vectors and **2** `AdvertisementRecord`-typed vectors (which embed a `BindingDecl` — `edge/advert` literally opens `a3 01 a7 01 …`). 11 of 26 vectors change bytes.

**Impact.** The single argument the document gives for preferring R3(a) over R3(b), and for calling the §4.2 compatibility artefact "the cheapest argument in favour of R3a", is false. If §4.2's optional `compatibility.v1.json` is actually written it will assert a property that does not hold and will fail on first run; if it is not written, the false "strict superset" claim ships in the README and the DecisionLog into a publish that cannot be withdrawn. The owner is also being asked to rule R4 on a stated consequence ("byte-safe, additive") that is the opposite of the truth: R4(a) is the most byte-invasive option in the document, more so than R2(c) or R5(c).

**Required correction.** (1) Add one sentence to §1 or §4.2 defining taut `optional` as *nullable, always emitted* — this is the fact the whole §4.2 argument turns on. (2) Rewrite R4(a)'s consequence column to "breaking for every `BindingDecl` and `AdvertisementRecord` vector (11 of 26)". (3) Choose explicitly among: accept a v1 that is **not** a byte superset; carry `profile` in a separate message keyed by glade id (leaving `BindingDecl` untouched); or defer R4 past the freeze. (4) Delete or restate R3's third reason, which no longer distinguishes (a) from (b).

**Closure/regression test.** Add to `corpus/build.py` a `--compat` mode asserting `decl.v1.json[n].cbor == decl.v0.json[n].cbor` for every `n` present in v0, and run it. Closure is either a green run, or the removal of every superset claim from §4.2, R3, R4, the README and the DecisionLog together.

---

### [P1-2] On first boot after the amendment the node appends a duplicate of every stored `BindingDecl`, because `register` diffs on payload bytes and `dir.bindings` has no fold

**Location.** §4.4 bullets 3 and 4; §4.6 final bullet ("The running demo's behaviour … fields nothing reads"); §4.7 gate row `cargo test -p glade-node`.

**Root cause.** The amendment changes the encoded bytes of `sysdata.BindingDecl` records, while the only idempotence mechanism the node has is byte equality against the existing fold — and `dir.bindings` has no LWW, no dedupe and no superseding rule.

**Violated invariant.** §4.6's own "what must NOT change": the running demo's behaviour, and the claim that the edited tokens are "fields nothing reads".

**Reproduction / state sequence.**
1. Today: the owner's live instance under `~/.glade/sys/<name>/records.json` holds 7 grazel `BindingDecl` records, four with `retention: "from-cursor"` and one with `"windowed"` (`grazel/apps/grazel-app.glade:23-25,32-33`).
2. The amendment lands. Under R4(a), `sysdata.BindingDecl` gains field 7, so `sysdata.rs:128-137`'s `to_cbor` emits a **7**-entry map where every stored record is a 6-entry map. Independently of R4, the `from-cursor` → `from_cursor` and `windowed` → `from_cursor` edits change `retention`'s bytes on 5 of grazel's 7 lines. (The token edit is classed SETTLED and the document says it "should ride the same commit" under *every* R2 option, so this path is not avoidable by any ruling.)
3. Boot: `glade-node.rs:88-91` calls `appdecl::load` then `appdecl::register`. At `appdecl.rs:264` the guard is `*g == glade_id && *p == payload` — a **byte** comparison. No amended record matches any stored record. Every one is appended (`:267`) and the snapshot is persisted (`glade-node.rs:91`).
4. Result: `dir.bindings` permanently holds two contradictory records for `term.log` (`windowed` and `from_cursor`), for `ws.diff`, `ws.files`, and — under R4(a) — for all 7 of grazel's surfaces and every other app's. Nothing resolves them: `registry.rs`'s `RegistryApi` has no binding query at all, and the sole production reader, `exchange.rs:69-77`, is an `any()`. The duplicates are ordinary home-share ops, so they fan out to every `--peer` and to every client that subscribes to `dir.bindings` (`exchange.rs:651-658`).

**Impact.** §4.6's stated invariant fails at the first boot. The durable directory silently accumulates contradictory declarations of the same surface with no resolution rule and no migration path back — the exact hazard the object names for R4 and then under-scopes. Worse, the moment R2's own recommendation lands ("the node starts validating the token"), or GlialFitAssessment S6's `(DomainAnchor, ZoneKind, principal) → (share, key)` mapping lands, a reader that is *not* an `any()` must answer "what is `term.log`'s retention?" from a stream that says both. §4.7's gate cannot see any of this: `registering_twice_appends_nothing` (`appdecl.rs:404-416`) registers the **same** decl twice into a **fresh** `Registry`; no test registers a pre-amendment decl and then a post-amendment one.

**Required correction.** (1) Correct §4.6: the retention and zone tokens *are* read — by `register`'s byte diff at `appdecl.rs:264` — so the app-file edits are not inert. (2) Attribute the durable reach correctly: §4.4 bullet 3 says this is "the one place a ruling reaches into stored data; weigh it in R4"; the token normalization reaches stored data on its own, under every R2 option. (3) Specify the outcome for an existing directory — either a stated migration (rewrite or supersede the prior records), or an explicit "duplicates accepted" decision that comes with a fold rule for `dir.bindings`. A design that leaves this unstated cannot be executed as "a SINGLE edit". (4) Add the missing gate to §4.7.

**Closure/regression test.** A `glade-node` test that registers the **pre-amendment** parse into a `Registry`, then registers the **post-amendment** parse, and asserts the intended `Registered{appended, unchanged}` and the resulting `dir.bindings` content. Closure requires that test to exist and to encode the decision made in (3).

---

### [P2-3] Node-side token validation turns every un-migrated `<app>.glade` into a hard boot failure, and §4.7's gate cannot see the copy that actually ships

**Location.** §4.4 bullets 2 and 4; §4.7 gate row `cargo test -p glade-node`; §4.6 final bullet.

**Root cause.** `appdecl::load`'s error propagates with `?` out of `main`, while the six app-file copies live in four repositories that cannot land atomically.

**Violated invariant.** "Never worse than the status quo" — today an unrecognized retention or zone token is inert (`appdecl.rs:137-138` stores `toks[4]`/`toks[5]` raw); after §4.4 it is fatal.

**Reproduction / interleaving.** `glade-node.rs:49` is `async fn main() -> std::io::Result<()>`; `:88` is `let decl = glade_node::appdecl::load(path)?;`; `appdecl.rs:204-211` wraps a parse error as `io::ErrorKind::InvalidData`. So one bad token = process exit before the listener binds. Now interleave: the `glade` repo lands the validation **and** its own `glade/apps/grazel-app.glade` edit. `cargo test -p glade-node` reads `CARGO_MANIFEST_DIR/../apps/grazel-app.glade` (`appdecl.rs:281`) — i.e. the `glade` copy — and is **green**. Meanwhile `grazel/apps/grazel-app.glade`, `grazel/apps/gyld-app.glade`, and the two fixtures in `glade-gyld` / `glade-gwz` still say `from-cursor`. The node now refuses to boot on the declaration files grazel actually ships (`grazel/src/lib.rs:150` defaults to `apps/grazel-app.glade`), and the only suite that would catch it, `grazel/tests/integration.rs:109,280,454`, is in a different repo and is not named anywhere in §4.7.

**Impact.** A reachable intermediate state that is strictly worse than today and invisible to the document's own check table: a node that rejects the app files it ships with, for every developer and for the live demo, with a green gate.

**Required correction.** (1) Name `cargo test -p grazel` (integration) in §4.7, and name all four repositories that hold app files. (2) State the landing order explicitly and give the reason: the **app files land first**, in all four repos, because an old node accepts `from_cursor` (the token is unvalidated) while a new node rejects `from-cursor` — the reverse order is the only one with a stuck state. (3) Decide and state whether an unknown token is a hard error or a logged warning for one release; §4.4 currently implies a hard error by analogy with the shape check and never says so.

**Closure/regression test.** `cargo test -p grazel` added to §4.7 and green, plus a `glade-node` unit test asserting the line-numbered diagnostic for each of `from-cursor`, `windowed` and an unknown zone.

---

### [P2-4] `ZoneKind.private` is frozen, published, and newly node-validated as a privacy guarantee that no production code implements

**Location.** §2d row 16 — classed **agrees**, i.e. explicitly *not* a ruling; §4.4 bullet 2 (validate token 4 against `{commons, private}`). Contrast R5(b), which recommends recording `external` as "declared, not yet authorable" precisely so it "is not mistaken for working".

**Root cause.** The document applies its own "declared, not yet authorable" discipline to the harmless member (`external`) and withholds it from the member whose misreading has a confidentiality consequence.

**Violated invariant.** A frozen, publicly published vocabulary must not assert a security property no implementation provides. The contract comment in `glade_decl.taut.py` states it as fact: *"`private` (keyed to a self, `self:<id>` at runtime)"*, and root `DecisionLog.md:59` (GDL-039) states *"privacy is a key (AZ §4a)"*.

**Reproduction / state sequence.** Every `self:` literal in `glade/node/src` is inside test code — `router.rs:133-136`, `store.rs:409-418,570`, `server.rs:400-404`. A grep across `glial/src`, `glade/client-ts/src`, `glade-chat/src`, `gryth-ui/packages` and `gryth-ui/src` returns **zero** producers (only CSS `align-self`). `glial/src/instance.ts:29-41` shows why: `Fill.zone` is a free string that feeds `instanceKey` — a *local* store key — and never the wire key. So: `grazel/apps/grazel-app.glade:24` declares `binding ws.diff log share private from-cursor`; a client mounts it and subscribes with key `[]`; `router.rs`'s own `keys_isolate_subscribers` test establishes that key `b""` is one shared commons audience. A surface declared *private* converges in the commons partition.

**Impact.** Disclosure scale. The privacy-by-keying mechanism exists, is correct, and is tested — but no declaration is wired to it, so `private` is decorative today. The amendment increases reliance on it in two ways: it makes `private` a node-**accepted** value (validation elevates it from an unread string to a checked vocabulary), and it freezes the name into a public registry where it can no longer be renamed. A downstream consumer of `@owebeeone/glade-decl` reading `ZoneKind.private` and its schema comment will declare confidential surfaces private and get commons fan-out. Note this is a pre-existing gap; the finding is that the document *classifies it as no-divergence* and proposes to freeze and publish it without the caveat it grants to `external`.

**Required correction.** Apply the R5(b) treatment to `private`: one sentence in the `ZoneKind` schema comment and in `README.md` stating that `private` is declaration-only, that no client produces the `self:<id>` key today, and that it must not be relied on for confidentiality until the glial mapping (GlialFitAssessment S6) lands. Add it as an OpenNote beside the N7/N8 the document already proposes. Reclassifying row 16 from "agrees" to a ruling is the cleaner fix, but the caveat alone closes the safety defect.

**Closure/regression test.** A glial test asserting that mounting a `zone: "private"` surface either produces a `self:`-prefixed wire key or throws — today it silently does neither; plus a check that the README caveat sits beside the published enum.

---

### [P2-5] §4.7's check table cannot detect a rendering left behind: the `-ts` and `-py` gates read their own local corpus copies, and `build.py --check` never looks at them

**Location.** §4.3 shell block, the trailing comment *"then copy `glade_decl.ir.json` + `decl.v1.json` into each rendering's gate location"*; §2m A4 (classed SETTLED as a documentation fix); §4.7 rows 1–4.

**Root cause.** The copy step is a manual, ungated action sitting on the critical path of an irreversible publish; the document upgrades it from undocumented to documented, but not from ungated to gated.

**Violated invariant.** §4.7 claims these checks "prove it" — that the contract, the corpus and the three renderings are in lockstep.

**Reproduction / interleaving.** `corpus/build.py:33-36` defines exactly three artefacts — `ir/glade_decl.ir.json`, `corpus/decl.v0.json`, `../glade-decl-rs/src/vectors.rs` — and `:170-184` compares exactly those. Nothing under `glade-decl-ts/` or `glade-decl-py/` is in the list. Meanwhile `glade-decl-ts/src/corpus.test.ts:16-19` reads `here + "glade_decl.ir.json"` and `here + "decl.v0.json"`, and `glade-decl-py/tests/test_corpus.py:24-25` reads `resources.files("glade_decl")` — both purely local. A rendering repo left at the old commit therefore holds a **locally self-consistent** old IR + old corpus pair and its gate passes green, while `build.py --check` in `glade-decl` also passes green, because it never reaches across. Under GWZ these are separate per-member commits; "one commit per rendering" (§4.3) makes "the third commit did not land" an ordinary reachable state. Only `glade-decl-rs` fails closed, because `build.py` regenerates its `vectors.rs` from the workspace-relative path and a stale `api.rs` then mismatches.

**Impact.** A rendering can be published or shipped at the old contract with every check in §4.7 rows 1–4 green. `PackageExtractionPlan.md:155`'s step-1.1 exit check compounds this: it verifies only `pnpm test` in `-ts` plus agreement between the `-ts` and `-rs` contract hashes — `glade-decl-py` is not covered by any gate in either document, and `CONTRACT_VERSION` is an unchecked string constant in all three.

**Required correction.** Extend `build.py`'s `artifacts` list to include the per-rendering copies (`glade-decl-ts/src/{glade_decl.ir.json,decl.v1.json}`, `glade-decl-py/src/glade_decl/{glade_decl.ir.json,decl.v1.json}`) so the copy step is enforced rather than remembered — the object has already verified all copies are byte-identical today, so this is a list extension, not a redesign. Fold this into A4's "one procedure, including the copies": the procedure needs a gate, not only a doc line. Separately, make `CONTRACT_VERSION` checkable (assert it equals the commit `build.py` last ran at) or stop describing it as a pin.

**Closure/regression test.** `python3 corpus/build.py --check` reports `STALE` when any one rendering copy diverges — demonstrated by inspection that the artefact list names all five copy paths, and by a run after deliberately reverting one copy.

---

### [P3-6] The delete options instruct a field and member removal with no tag or name reservation, and taut has the facility

**Location.** R1(d), R2(c), R5(c), R7(b); §4.1's framing "Everything below assumes the recommended answers; **substitute freely**" and its closing "Nothing else in the schema moves."

**Violated invariant.** Taut's own retirement discipline: `taut/docs/Reference.md:95-98` — *"When you remove a field, reserve its tag and name."*

**Reproduction.** `taut/src/taut/ir/dsl.py:151,176-179` gives `Msg(..., reserved=(), next_id=None)`; `taut/src/taut/ir/validate.py:45-58,70-75` enforces reservations **only when declared**. `glade_decl.taut.py` declares neither `reserved` nor `next_id` on any message. So substituting R1(d) into §4.1 yields a frozen v1 in which tag 5 and the name `domain` are silently reusable; R5(c) does the same for tag 4 / `source`; R7(b) retires a whole message name.

**Impact.** A later v2 that reuses tag 5 for a different type decodes v1 bytes into the wrong field, and taut's validator will not object — under the node's `--legacy-codec` fail-open decoder this is silent, not loud. §4.1's "substitute freely" is what makes this reachable: the delete options are offered without the step that makes them safe.

**Required correction.** One line in §4.1: any delete option must also add `reserved=[<tag>, "<name>"]` to the message and set `next_id` above every used and reserved tag; for a retired **enum** member, record the retired number in the enum comment (taut has no enum `reserved`), which is also what R3(b)'s "hold the numbering" actually requires.

**Closure/regression test.** Assert `BindingDecl.reserved_tags` contains the retired tag after any delete option, and that `validate_or_raise` rejects a schema re-declaring it.

---

### [P3-7] R3(b)'s stated consequence is incomplete: holding the enum numbering does not keep the corpus bytes stable, because synth vectors select a member by index

**Location.** §3 R3 options table, row (b): *"two members go; numbering for `stream..crdt` must be held to stay byte-stable"*.

**Reproduction.** `taut/src/taut/corpus/synth.py:32-34` — `members[seed % len(members)]`, with the seed being the field tag. Today `Shape` has 8 members and `BindingDecl.shape` is tag 2, so index `2 % 8 = 2` selects `message` — visible as `02` at tag 2 in the synth vector `a701a10162733102 02 0301…`. Under (b), `Shape` has 6 members: `2 % 6 = 2` now selects `stream`, wire value 3. The synth `BindingDecl` and `ChangeEvent` vectors change bytes **even with every surviving number held**. (Under the recommended (a), `2 % 9 = 2` still selects `message`, so (a)'s enum claim does hold — this finding is specific to the delete option.)

**Impact.** The owner weighs (a) against (b) partly on a byte-stability difference that is smaller than stated. `build.py --check` catches it loudly, so the consequence is a surprised re-run, not silent corruption.

**Required correction.** One clause in the (b) row: "and the synth `BindingDecl`/`ChangeEvent` vectors change regardless, because synth selects by member index."

**Closure/regression test.** None beyond `build.py --check`; correcting the sentence closes it.

---

### [P3-8] `atom=8` is added to a frozen oracle whose README promises a vector for "every shape", with no instruction to add one

**Location.** §4.1 item 1; §2a row 9 (classed SETTLED, "additive, byte-safe"); `glade-decl/README.md:55-58`.

**Reproduction.** `corpus/build.py:73-140` — `curated_values()` is hand-authored; the per-shape bindings (`edge/binding-doc-swmr`, `-doc-crdt`, `-exchange`, `-message-private`, `-deployment-window`, …) are literal entries. `synth_values` (`synth.py:51-53`) adds one vector per **message**, never per enum member. So adding `atom=8` adds no vector, and the README's stated coverage — "every shape / domain / zone / retention policy" — becomes false at the moment of freeze.

**Impact.** The one member being added and frozen ships with zero bytes in the oracle, so none of the three renderings' byte-parity gates exercise it. Bounded (the codecs are generic over enum values), but it is a coverage hole in the exact element the amendment introduces, published as part of a contract whose README asserts the opposite.

**Required correction.** §4.1 item 1 should also instruct adding an `edge/binding-*-atom` curated vector — and, under R3(a), explicitly keeping `edge/binding-message-private` and `edge/binding-deployment-window` as the relabelled recognition cases R3(a)'s own table promises.

**Closure/regression test.** A `build.py` assertion that every `Shape` member appears in at least one corpus vector; it must fail before the fix and pass after.

---

## 2. Invariant analysis

**Attacks that succeeded** are the eight findings above. **Attacks that failed** — these are part of the result, and the document earns credit for them:

- **R3(a)'s enum claim is true.** Enums encode as their integer wire value (`codec.py:10`, "enum -> its integer wire value"), and appending `atom=8` moves no existing value. I additionally checked the corpus generator's index-based member selection (`synth.py:32-34`): `2 % 8 = 2` and `2 % 9 = 2` both select `message`, and the other enum-typed fields (`authority` tag 3 / 2 members, `domain` tag 5 / 3, `zone` tag 6 / 2, `Retention.policy` tag 1 / 3, `ChangeEvent.kind` tag 3 / 2) are untouched by R3(a). Every synth vector is byte-stable under R3(a) alone. The failure in P1-1 is caused solely by R4(a)'s new field.
- **R1(d)'s claim survives.** `F("domain", 5, Ref("DomainAnchor"))` is non-optional and therefore present in all 9 `BindingDecl` vectors and both `AdvertisementRecord` vectors; deleting it does change every one of them, exactly as the table says. The document is careful to scope the claim to `BindingDecl` vectors rather than "every vector".
- **R3(b)'s gap-holding is mechanically possible.** `validate.py:77-80` checks enums only for duplicate wire values; gaps at 2 and 5 are legal. The option is achievable; only its byte consequence is mis-stated (P3-7).
- **R7(b) is genuinely additive.** Messages carry no ordinal in the wire encoding (`codec.py`: a message is a bare CBOR map keyed by field tag; nothing names the message on the wire), and `schema.messages` is a name-keyed dict. Removing `AdvertisementRecord` from v1 and re-adding it later changes no other message's bytes. Only its retired *name* wants reserving (folded into P3-6).
- **Decision-status claims all verified.** GDL-029 `open` (`DecisionLog.md:47`), GDL-039 `open (implemented, pending ratify)` (`:59`), GDL-041 ratified 2026-08-28 (`:58`). §1's precedence reasoning stands, and §2's classification of what is settled versus ruled follows from it correctly.
- **The dual-maintenance and token counts are exact.** 13 `from-cursor` across 5 files and 2 `windowed`, and the two `grazel-app.glade` copies are byte-identical — verified by census and `diff`. §4.4's warning "edit both or the node tests fail" is correct.
- **A1 and A2 are truthfully reported.** `CONTRACT_VERSION = "99a04e0b…"` in `glade-decl-ts/src/index.ts:18`, one commit behind `bbce73d`; `git -C glade-decl-rs status --short` shows the two modified files. The object reports both against itself.
- **`--check`'s stated scope is honest.** §4.7 row 1 claims exactly "IR + corpus + rs vectors in lockstep", which is precisely what `build.py:170-184` compares. The defect in P2-5 is that §4.7 as a *table* implies a lockstep proof the rows do not jointly deliver — not that any single row overstates itself.
- **Wire-IR exclusion holds.** §1's table and §4.6's first two bullets correctly separate the declaration `Shape` from `taut/corpus/glade.ir.json`'s wire `Shape`, and correctly hold `gryth-ui/packages/glade/src/glade.ir.json` (A8) out of scope. Nothing in the amendment as specified reaches the wire.
- **Review-loop framing is sound.** §5's reading of L1-18 (three axes at an interface freeze, because `<app>.glade` is "a file format people edit") and L1-17 (settle the tree first) matches `AgentProcessRules.md:240-252,370-380`. The document is right that this object needed a loop.

**Scope creep, assessed.** "Reconcile with the node" does widen the blast radius from a declaration contract into (i) durable node records, (ii) a hand-edited file format in four repositories, and (iii) the node's boot path. The widening is **named** — §4.4 exists, §4.6 lists what must not change, and §4.4 bullet 3 flags the durable reach — but it is **not bounded**: P1-2 shows the durable effect is larger and differently caused than the text states, and P2-3 shows the file-format effect turns a previously inert field into a boot-blocking one across repository boundaries the document's gate table does not cross. The widening is legitimate for the goal; the missing work is the migration statement and the two gates.

**Irreversibility, assessed.** Corpus replacement (§4.2) is properly reversible — `decl.v0.json` remains at `bbce73d` and the reasoning for one live oracle (`build.py:33` loads exactly one schema; each rendering's gate reads one fixed name) is correct. The genuinely irreversible step is the registry publish, and §4 never sequences it: §4.3 says "A2 must be green before anything is published" and §5 says the loop precedes any publish, but nothing states that §4.4's and §4.5's rows in §4.7 must be green first. `PackageExtractionPlan.md:155-156` makes that gap live — step 1.1's exit check covers only `-ts` plus a `-ts`/`-rs` hash comparison, and step 1.2 publishes. An executor could honestly publish with the node half of the reconciliation unlanded. This sits just below the finding bar because §5's mandatory-loop requirement covers it in practice; one sentence in §4 ("publish only after every row of §4.7 is green on a settled tree") would close it outright and is recommended alongside the P2 fixes.

**R6 / deferred key derivation, assessed.** The document's R6(a) hardening is the right shape: it requires `README.md:42-50` to say the interfaces are not implemented and not oracled in v1, and it names the three live conventions (`glade-chat/src/manifest.ts:31-33`, gryth-ui `gwz/src/live.ts:47`, `gyld/src/ops/surfaces.ts:112`). The damage bound is real and the document states it correctly: `GladeIdManifest` ships as a record format only, the derivation is deferred, and the caveat lands in both the README and the DecisionLog. The residual — that a later incompatible key derivation orphans data keyed under the old convention — is inherent to shipping the deferral at all and is a consequence of the R6 *outcome*, which is deferred to the owner. It is not a defect in the text.

---

## 3. Risks and next action

**Residual risks below the finding bar**

- **`atom` is contract-valid but unbindable.** §4.4 adds `atom` to `KNOWN_SHAPES` and not to `BINDING_SHAPES`, and `glial/src/shapes.ts:101-110`'s `requireMountShapeAdapter` also rejects it while `requireShapeAdapter` accepts it. So a consumer can legally emit `BindingDecl{shape: atom}` that no node will accept and no client will mount. This is exactly the posture R3(a)'s comment is meant to cover ("recognition ≠ support"), and §4.1 item 1 instructs that comment — but it is written about `message`/`window`, not about the member being added. One clause naming `atom` in that comment closes it.
- **No `publish`-ordering sentence in §4**, as analysed above.
- **A2 and the `glade-decl-rs` working-tree diff** are out of scope per the review prompt; the object names both and §5 correctly requires them resolved before the loop starts. Noted only so a later auditor sees they were checked, not overlooked.
- **A6** (`glade-sys.glade` does not exist despite ratified GDL-038) is correctly held out of scope and flagged as adjacent to R8.

**Single next action.** Before any ruling on R1–R8 is recorded, revise §4.2 and R4 against the real taut encoding — `optional` is nullable-and-always-emitted, so R4(a) rewrites 11 of the 26 oracle vectors (P1-1) — and, in the same revision, state what happens to an existing node data directory on the first boot after the amendment (P1-2). Those two answers change which options are cheap and therefore change the rulings themselves; the remaining fixes (P2-3, P2-4, P2-5) are additions to §4.4, §4.7, the schema comments and `build.py`'s artefact list that do not depend on them. The document should not enter the three-axis loop of §5 until P1-1 and P1-2 are answered, because both alter the premises the other axes will be reviewing.
