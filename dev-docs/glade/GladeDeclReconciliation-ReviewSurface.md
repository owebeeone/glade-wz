# GladeDeclReconciliation — SURFACE-AXIS REVIEW

**Review object:** the user-facing surface that the draft design `dev-docs/glade/GladeDeclReconciliation.md` (glade-wz root `b132b7e6d365`, DRAFT, unimplemented) would freeze — the hand-edited `<app>.glade` format, in particular its `binding <glade_id> <shape> <authority> <zone> <retention>` line, and the `glade-decl` contract's front page as a newcomer meets it.

**Baseline:** glade-wz root `b132b7e6d365` · glade-decl `bbce73d67146` · glade `960c9b0fa038` · grazel `924cb3c4bab9` · glade-gyld `024d2a8ae061` · glade-gwz `e53c87dddb8f`. Verified identical at start and at end of review; the draft file is clean in the working tree. Files read in full: `grazel/apps/grazel-app.glade`, `grazel/apps/gyld-app.glade`, `glade/apps/grazel-app.glade`, `glade-gyld/tests/fixtures/gyld-test-app.glade`, `glade-gwz/tests/fixtures/gwz-test-app.glade`, `glade-decl/README.md`, `glade/README.md`, `grazel/README.md`, `glade-gwz/README.md`, `glade/dev-docs/GladeGrazelAttachNotes.md`; `glade-gyld/README.md` read by targeted grep + its app-file section (`:930-981`). `glade/node/README.md` does not exist. One permitted grep over `glade/node/src/appdecl.rs` for quoted diagnostics; one permitted `sed -n '479,500p'` over the draft (§4.4 only — extraction confirmed to match the lane owner's statement verbatim). **I read no code, no design document, and no other part of the draft.** One caveat disclosed in §0.

**Date:** 2026-09-21
**Axis:** SURFACE — the interface as the person writing and editing an app file meets it: token position, cold-read naming, spelling consistency, lifecycle pairs, stated defaults, and the first-day walkthrough. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: NO-GO** — 5 P2, 5 P3, 0 P0, 0 P1. Every blocking finding is bounded and text-fixable inside the draft. *I pre-commit to GO on a revision that resolves P2-1, P2-2, P2-3, P2-4 and P2-5 as specified.*

---

## 0. Evidence base

The authored corpus is small enough to quote whole. Across all five app files there are exactly **22 binding lines**, drawn from this vocabulary and no other:

| position | token | values actually authored, anywhere |
| --- | --- | --- |
| 1 `<glade_id>` | hand-typed dotted word | `ws.tree` `ws.files` `ws.diff` `term.log` `gwz.output` `chat.msgs` `chat.groups` `gyld.streams` `gyld.stream` `gyld.decisions` `gyld.lens` `gyld.file` `gyld.output` `gyld.ask` |
| 2 `<shape>` | bare word | `value` `swmr` `log` |
| 3 `<authority>` | bare word | `share` (only value, 22/22) |
| 4 `<zone>` | bare word | `commons` (21/22), `private` (1/22 — `ws.diff`) |
| 5 `<retention>` | bare word | `latest` (12), `from-cursor` (13 across 5 files), `windowed` (1 per grazel copy = 2) |

Migration arithmetic in the brief and in draft §4.4 is confirmed exactly: 13 `from-cursor`, 5 files, 2 `windowed`.

Vocabulary coverage across the permitted user-facing doc set (`glade-decl/README.md`, `grazel/README.md`, `glade-gyld/README.md`, `glade-gwz/README.md`, `glade/README.md`, `glade/dev-docs/GladeGrazelAttachNotes.md`), by grep:

| token | named in any of those pages? |
| --- | --- |
| `crdt`, `atom`, `swmr`, `commons`, `external`, `from-cursor`, `from_cursor`, `windowed` | **no — zero hits** |
| `private`, `latest`, `ttl` | hits exist but **every one is an unrelated sense** (a "private or loopback address"; `latest.json`; a substring of "settled") |
| `exchange` (as a binding shape) | yes — `GladeGrazelAttachNotes.md:98` |

The node's diagnostics (permitted grep) establish the current failure surface and one good precedent:

```
:113  line {n}: `binding <glade_id> <shape> <authority> <zone> <retention>`
:117  line {n}: unknown shape `{}` (known: {KNOWN_SHAPES:?})
:121  line {n}: unsupported binding shape `{}` (implemented: {BINDING_SHAPES:?}; exchange uses `service`)
:127  line {n}: unknown authority `{}` (one of {AUTHORITIES:?})
```

There is no zone or retention diagnostic today — confirming the brief. Note `:121`: the format **already names the replacement** when it refuses a recognised-but-unauthorable token. That is the bar the amendment's own new refusals are measured against (P3-2).

Also established: **no CHANGELOG, migration, upgrade or release note exists in any of the five repos** (`git ls-files | grep -i 'changelog|migrat|upgrad|releas'` → empty in all five). And there is **no underscore anywhere in any authored token of any app file** — the corpus is hyphen, dot and bare word only (P3-3).

*Caveat, disclosed:* `glade/docs/` contains exactly one file, `docs/README.md` (8 lines), which `glade/README.md:23` advertises as "Public support contracts and user-facing documentation". It is not on my permitted read list and I did not open it. Every finding below is written so that it survives whatever those 8 lines contain — an 8-line index cannot enumerate four vocabularies. A later auditor should confirm.

---

## 1. Findings

### [P2-1] `zone` is switched from ignored to enforced while the file's own comment tells the author the mount fills it

**Location.** `grazel/apps/grazel-app.glade:19-21` (identical at `glade/apps/grazel-app.glade`, and `grazel/apps/gyld-app.glade:17`); `glade/dev-docs/GladeGrazelAttachNotes.md:49-50`; draft §4.4 bullet 2.

**Violated invariant.** A token the author must write *correctly or the node refuses to boot* must be documented as the author's to choose, with a stated meaning per value and a stated behaviour when omitted.

**What a user writes and meets.** The entire in-file documentation of the binding line is three consecutive lines:

```
# binding <glade_id> <shape> <authority> <zone> <retention>
# App-static surfaces: no share/key here — the ServeClaim selects the node,
# the mount fills domain/zone/key (GladeDeclSurface.md).
```

Line 1 requires the author to type a `<zone>`. Line 3 tells the same author that the mount fills the zone. `GladeGrazelAttachNotes.md:49-50` repeats both halves in one sentence pair — "the mount fills domain/zone/key … shape / authority / zone / retention ride as STRINGS". Today the contradiction is inert: zone is unvalidated, every author copies `commons`, nothing breaks. After the amendment a wrong zone is a line-numbered boot refusal, so the author must now resolve a contradiction that the surface does not resolve. Nothing in any page read states what `commons` means, what `private` means, which to pick, or whether a mount overrides what was written. The nearest gloss is `glade-decl/README.md:41` — `ZoneKind` is "who converges within it (→ wire `key`)" — which names no value.

**Impact.** A mandatory, validated, frozen token with no learnable semantics and no default. The corpus teaches "type `commons`" (21 of 22 lines) and the one deviation (`ws.diff … private`) carries no stated reason, so validation buys nothing while cementing the token in the grammar and in durable records. `commons` also sits immediately after the authority token `share` — two adjacent bare words that both read as "this is shared" — so a cold reader cannot tell which of the two carries the sharing decision.

**Required correction.** In the same wave that turns validation on: state, in the format's documentation, the meaning of `commons` and of `private`, the rule for choosing, the default when the token is absent, and whether a mount can override an authored zone; and delete or rewrite the "the mount fills domain/zone/key" sentence in both homes of `grazel-app.glade` and in `GladeGrazelAttachNotes.md:49` so the two statements agree.

**Closure test.** A reader who has seen only the app files and the format doc can state, without guessing, which zone token to write for a private per-principal surface and which for a shared one, and what happens if the token is absent; and no page tells them the mount fills a token they are required to type.

---

### [P2-2] The accepted language shrinks under an unchanged `glade-app v0` header, so `v0` permanently names two incompatible languages

**Location.** The header line `glade-app v0` (line 1 of every app file, e.g. `grazel/apps/grazel-app.glade:16`); `GladeGrazelAttachNotes.md:30`; `appdecl.rs:91` (`expected \`glade-app v0\` header`); draft §4.4 bullets 2 and 4.

**Violated invariant.** A format that carries an explicit version token must advance that token when the set of accepted programs shrinks — otherwise old and new files are indistinguishable and no targeted migration diagnostic is possible.

**What a user writes and meets.** A person outside these five repos has their own app file — `--app` takes data, and `grazel/README.md:73-79` documents that the node accepts `--app` repeatedly, so third-party app files are the intended shape of the product:

```
glade-app v0
app myapp
binding my.out log share commons from-cursor
```

That file boots today. After the amendment it fails at boot. The header still says `glade-app v0`; so does the new grammar. Neither the user nor the parser can distinguish a v0-old file from a v0-new one. There is no CHANGELOG or migration note in any of the five repos (verified), so the first notice the user gets is a node that will not start.

**Impact.** `v0` names both the language that accepts `from-cursor`/`windowed` and the language that refuses them. A node can therefore never accept both, and the parser can never emit the one message that would actually help — "`from-cursor` is the v0 spelling; v1 spells it `from_cursor`" — because there is no version to key it on. After the freeze this is unrecoverable by any additive change: the only fix is to break compatibility again.

**Required correction.** Either (a) the validated grammar becomes `glade-app v1`, with `v0` files still accepted and `from-cursor`/`windowed` producing a deprecation warning that names the v1 spelling; or (b) if `v0` is to be redefined in place, the draft must say so explicitly, state that every existing v0 file must be rewritten, and ship a migration note — the first such document in these repos.

**Closure test.** A file whose header says `glade-app v0` and whose retention says `from-cursor` either loads with a warning naming `from_cursor`, or fails with a message naming both the replacement token and the version in which the spelling changed. In either case a user can tell from the file alone which language it is written in.

---

### [P2-3] `ttl` joins the frozen retention set with no slot for its duration, and the line's one free positional slot is consumed by `profile`

**Location.** Draft §4.4 bullet 2 (token 5 validated against the policy set) and bullet 3 (optional trailing `profile` at position 6); `GladeGrazelAttachNotes.md:32` (the 5-token grammar); `glade-decl/README.md:24-25`.

**Violated invariant.** Every option must have a stated default or a way on the line to state its parameter; and a positional grammar must not spend its last unambiguous slot before the options that need one have been placed.

**What a user writes and meets.**

```
binding cache.entries value share commons ttl
```

This parses — `ttl` is in the frozen set. The time-to-live is: unstated. There is no token position for it, and no page read states a default. The user cannot express "10 minutes" and cannot discover what they got instead.

The contract's own front page shows why this is a hole and not an oversight. `glade-decl/README.md:24-25` lists `RetentionPolicy` as an **enum** and `Retention` as a separate **declared unit** — a record distinct from the policy it names — and `README.md:32` gives `BindingDecl` as `(glade id, shape, authority, source?, domain, zone, retention)`, carrying the *record*. The app-file grammar has one bare token there. So the file can express a `RetentionPolicy` and cannot express a `Retention`. That is a third declaration-only gap; the draft names only two (`external` authority and `source`).

**Impact.** Freezing the line with `profile` at position 6 closes the only unambiguous positional extension the format has. A duration added later at position 6 is ambiguous with `profile`; added at position 7 it makes `profile` mandatory whenever a duration is present, which the "optional trailing" design forbids. The format has no `key=value` form, so after the freeze `ttl` can never acquire a parameter without a compatibility break. `ttl` ships unusable-as-specified on day one.

**Required correction.** Before the freeze, choose one: (a) give the line a keyword tail after the five positional tokens (`… ttl=10m`, `… profile=rga`), which also solves P2-4's placement and leaves the grammar open; or (b) remove `ttl` from the authorable retention set until it has a parameter slot, and say in the grammar doc that it is reserved. *(Which retention words exist is deferred; that one of them is admitted with no way to parameterise it is not.)*

**Closure test.** An app file can express a TTL of 10 minutes and one of 1 hour, they parse to different records, and adding a seventh option later changes the meaning of no existing six-token line.

---

### [P2-4] The sixth token ships without its rules: a grammatically optional token that one shape requires, with the enforcement point unstated

**Location.** Draft §4.4 bullet 3 ("the grammar gains an optional trailing `profile` token and `sysdata.taut.py` a field 7 … a durable-record change"); `appdecl.rs:113` (the arity diagnostic, which shows five tokens).

**Violated invariant.** An optional token must have a stated legal-value set, a stated meaning when omitted for *every* shape that may carry it, and a stated point at which a shape-conditional requirement is enforced. The format's stated value (`GladeGrazelAttachNotes.md:38-47`) is precisely line-numbered diagnostics on a hand-edited file; a requirement the parser does not check forfeits that value.

**What a user writes and meets.**

```
binding doc.body crdt share commons latest
```

Five tokens, legal arity, legal shape, legal authority, legal zone, legal retention. Per the lane owner's statement, a `crdt` binding "needs [`profile`] in order to be mountable" — so this line is accepted and the surface does not work, with no line number and no message at parse time. The user then tries to supply the token:

```
binding doc.body crdt share commons latest ???
```

and there is no published list of `profile` values anywhere: `crdt` and `profile`-as-a-binding-token appear in none of the pages read. The arity diagnostic at `:113` prints the five-token template, so it does not reveal the sixth token exists. Separately, the word is already taken twice in the same product as the user meets it: `grazel/README.md:102-106` documents the node's `--profile local|peer` boot profile, and `glade-gyld/README.md:315` documents an LLM "compatibility profile". A sixth unglossed bare word on a line of five unglossed bare words, spelled as a word that already means two other things here, is not a guessable interface.

**Impact.** A file that validates and produces an unmountable surface. Tightening the parser afterwards — refusing a five-token `crdt` line that used to parse — is a compatibility break, and the token's spelling is by then in durable records (draft §4.4 calls field 7 "the one place a ruling reaches into stored data").

**Required correction.** Before the freeze, state in the grammar documentation: the token's legal values; what its absence means for **each** shape in `BINDING_SHAPES` (refused at parse for `crdt`? ignored for `value`/`log`? refused for them?); and update the arity diagnostic at `appdecl.rs:113` to show the optional token. If the requirement is shape-conditional, enforce it at parse with a line number, as every other binding check is. Prefer a keyword form (see P2-3) and a name that says what it selects — the merge strategy — rather than reusing `profile`.

**Closure test.** `binding doc.body crdt share commons latest` either parses to a working surface or is refused *at parse* with a line number naming the missing token and its legal values; and the template in the arity message matches the grammar the format accepts.

---

### [P2-5] The binding line offers a write with no documented change or retract half — and the forced rename is exactly that operation

**Location.** `GladeGrazelAttachNotes.md:56-61` (registration semantics); `appdecl.rs:197` (`duplicate glade id`); draft §4.4 bullet 4 (13 lines rewritten across 5 files).

**Violated invariant.** A token that can be written must be changeable and removable through the same surface, with the outcome stated.

**What a user writes and meets.** The user performs the mandated migration on a node that has already registered these bindings:

```
-binding ws.files  swmr  share commons from-cursor
+binding ws.files  swmr  share commons from_cursor
```

and reboots. The only account of what happens is `GladeGrazelAttachNotes.md:56-61`: registration "appends each declaration as an ordinary home-share record … Idempotence is by DIFF: a record whose **(glade_id, payload bytes)** already exist in the fold is skipped". The payload bytes have changed, so the diff does not skip: a **second** `BindingDecl` for `ws.files` appends alongside the `from-cursor` one. No page read states that the newer record supersedes the older, nor which one a consumer folds to. `glade-gyld/README.md:934` documents last-writer-wins folding for **`value` surfaces**, not for `dir.bindings` records, so it does not answer the question either.

Deleting a binding line is worse: registration only ever appends and skips, so removing the line appends nothing and retracts nothing. There is no `unbinding`, no retract form, no supersede marker in the grammar (`GladeGrazelAttachNotes.md:30-35` lists every directive). The parser's `duplicate glade id` refusal (`:197`) prevents re-declaring an id *within one file*, which is a different mechanism and does not help.

**Impact.** The amendment's headline user-visible action — rewriting a retention token on 13 already-registered lines — is the one operation the surface has no defined semantics for. A user cannot determine from the published surface whether their migrated file took effect, whether the old declaration is still live, or how to remove a binding they no longer want. Adding a retract or supersede form after the grammar is frozen is a grammar change.

**Required correction.** State in the format documentation what happens when an authored token changes between loads (which record wins, and how a consumer knows) and what happens when a binding line is deleted. If the answer is "the old record persists", the grammar needs a retract form before the freeze, not after. State this before any file is migrated, because the migration is the first mass exercise of it.

**Closure test.** Change one binding's retention token in a file, reboot, and the documented rule predicts which declaration is live; delete a binding line, reboot, and the documented rule predicts whether the surface is gone. Both are stated in a page a user reads, not inferred from fold mechanics.

---

### [P3-1] No page a user reads names a single legal value of `shape`, `zone` or `retention`

**Location.** `glade-decl/README.md:24` (enums named, members never listed); `GladeGrazelAttachNotes.md:30-35` (grammar with placeholders only); the app files' header comments (`grazel-app.glade:19-21`).

**Violated invariant.** Once a token is validated, its accepted set and each value's meaning must be discoverable from the published surface, not only by copying a neighbour.

**What a user writes and meets.** The contract's front page lists `Shape`, `Authority`, `DomainAnchor`, `ZoneKind`, `RetentionPolicy`, `ChangeKind` as enums and does not name one member of any of them. The format's grammar shows `<shape>` `<authority>` `<zone>` `<retention>` and names no value. So the only source of vocabulary is the 22 authored lines — which between them exhibit three shapes, one authority, two zones and three retentions. `swmr` is **already authored** (`grazel-app.glade:23`) and appears in no page. `crdt`, the amendment's headline addition, appears in no page and in no example file, so the first person to want one has nothing at all to copy (see §2, Walkthrough A, step 5).

**Impact, and the honest mitigation.** The refusal diagnostics print the accepted set — `unknown shape \`x\` (known: {KNOWN_SHAPES:?})` — and draft §4.4 gives zone and retention "the same line-numbered diagnostics". So after the amendment the *spelling* of each vocabulary becomes discoverable by failing. What a diagnostic cannot supply is **meaning**: a printed list of `["commons","private"]` does not tell a user which to pick, and a printed `["latest","from_cursor","ttl"]` does not tell them what any of the three does. That residue is the finding. Also note that the hand-edited format's only specification lives in `glade/dev-docs/GladeGrazelAttachNotes.md` — a document `glade/README.md:24` classifies as "Internal engineering design", titled after a different subject ("Grazel attach — engineering notes (Lane R step 4)").

**Required correction.** `glade-decl/README.md` grows a members table per enum with a one-line gloss each; the grammar block gains, per position, the accepted token set with its gloss; and the grammar block moves to, or is mirrored in, a page presented as user-facing.

**Closure test.** Grepping the user-facing doc set for each of `value`, `log`, `swmr`, `stream`, `crdt`, `atom`, `share`, `commons`, `private`, `latest`, `from_cursor`, `ttl` returns a hit with a gloss, in the correct sense, for every one.

---

### [P3-2] The renamed and removed retention tokens fail with a set listing that does not name their replacement — though the format already sets that precedent

**Location.** Draft §4.4 bullet 2 ("the same line-numbered diagnostics as the shape and authority checks"), against the precedent at `appdecl.rs:121`.

**Violated invariant.** A removed or renamed token must fail with a message that names its replacement. The format already does this: `unsupported binding shape \`exchange\` (implemented: …; **exchange uses \`service\`**)`.

**What a user writes and meets.**

```
binding term.log  log   share commons windowed
```

→ `line 7: unknown retention \`windowed\` (one of ["latest","from_cursor","ttl"])`. Nothing in that set resembles `windowed`, and none of the three is the obvious semantic neighbour. `term.log` is terminal scrollback; a reader migrating it will reach for `ttl` (time-bounded) or `latest`. The ruled answer is `from_cursor`. Picking `latest` silently converts an append log into a last-writer-wins value; picking `ttl` parses and leaves the duration unspecified (P2-3). The `from-cursor` case is milder — the replacement is visibly in the printed set — but still unnamed.

**Impact.** The migration is a one-shot event for every app file in existence outside these five repos, and a wrong pick at that moment is not undone by a better message later. Bounded rather than blocking because the fix is additive.

**Required correction.** Keep `from-cursor` and `windowed` as *recognised-but-refused* retention tokens whose diagnostic names the replacement, in the exact shape of `:121`: ``unknown retention `windowed` (removed; use `from_cursor`)``.

**Closure test.** Parsing a line with `windowed` yields a message containing `from_cursor`; parsing a line with `from-cursor` yields a message containing `from_cursor`.

---

### [P3-3] `from_cursor` would be the only underscore in the entire authored surface, against the format's own hyphen convention

**Location.** Draft §4.4 bullet 4; against `glade-app` (`grazel-app.glade:16`), `ws-razel`, `glade-gyld`, `gyld-test`, `gwz-test`, `from-cursor`.

**Violated invariant.** Within one hand-edited format, multi-word tokens use one joining convention.

**What a user writes and meets.** Verified by grep: **there is not one underscore in any authored token of any of the five app files.** Every joined token is hyphenated — the header keyword `glade-app`, the share `ws-razel`, the service name `glade-gyld`, the app names `gyld-test`/`gwz-test`, and today's retention `from-cursor`. `from_cursor` would be the format's sole underscore and a convention of one; the only multi-word *keyword* the format has, `glade-app`, stays hyphenated beside it. A user who has typed `glade-app`, `ws-razel` and `glade-gyld` will type `from-cursor` and be refused.

**Impact.** A permanent, recurring typo on the one token that appears on 13 of 22 binding lines — the most-written value in the vocabulary. Caught every time by a clear diagnostic, so the harm is friction rather than a broken file; that is why this is P3. *A reviewer applying the brief's "compat break to fix after the freeze" rule mechanically would rank it P2, since a retention token cannot be respelled after the freeze. I rank by realised harm and flag the alternative reading.* **Spelling is explicitly in scope on this axis; which words exist is not, and this finding does not touch that.**

**Required correction.** Spell it `from-cursor`, matching every other joined token in the format and — as a free consequence — reducing the mandated edit from 13 lines to 0. If underscore is chosen deliberately, state the convention in the grammar doc so the next token is not a second coin flip.

**Closure test.** Every multi-word token the format accepts uses one joining character, and the grammar documentation says which.

---

### [P3-4] The format's only specification documents a binding shape the validator refuses, and the draft ratifies the shape tiering without reconciling it

**Location.** `GladeGrazelAttachNotes.md:98` against `appdecl.rs:121`; draft §4.4 bullet 1 ("Keep the comment at `:39-42`; it is already the correct posture").

**Violated invariant.** The document that specifies the format must not describe a line the format refuses.

**What a user writes and meets.** `GladeGrazelAttachNotes.md:98` — in the section on attaching a supplier, which is that document's audience — states that a provider attaches to a glade id declared an exchange surface by "a `dir.services` record, **or a `dir.bindings` record with shape `exchange`**". A reader following that writes:

```
binding gwz.ops exchange share commons latest
```

and meets ``line N: unsupported binding shape `exchange` (implemented: [...]; exchange uses `service`)``. The diagnostic is good; the document that sent them there is wrong. The draft reviews the shape vocabulary in this wave and blesses the two-tier posture (`KNOWN_SHAPES` vs `BINDING_SHAPES`) without touching the one user-facing sentence that contradicts it. The same wave adds `atom` to the recognised-but-unauthorable tier, where — unlike `exchange` — no redirect exists for it, nor for `message` or `window`.

**Impact.** The one specification of the format misdirects the exact reader it is written for. Pre-existing, but this wave is the freeze and the ratification.

**Required correction.** Fix `GladeGrazelAttachNotes.md:98` to say a `dir.bindings` record with shape `exchange` is not authorable and that `service` is the authored form; and give `atom`, `message` and `window` a redirect in the `:121` message or accept that they print only the implemented list.

**Closure test.** No user-facing sentence describes an app-file line that `parse()` refuses; and every recognised-but-unauthorable shape either names its authored alternative or is documented as reserved with nothing to redirect to.

---

### [P3-5] The contract's front page says glade ids are derived and frozen-once-shared, with the derivation deferred, while the file asks the author to type one

**Location.** `glade-decl/README.md:28-30` and `:47-50`; token 1 of every binding line.

**Violated invariant.** The newcomer meeting the contract must be able to tell where the first token of the line comes from.

**What a user writes and meets.** The front page says a `GladeId` is a "stable share-space address; frozen once shared (GQ-6), **derived from package id + grip key**, pinned in a `GladeIdManifest`; renames are alias records, never new ids", and that `derive_glade_id(package_id, grip_key)` is "the GQ-6 pure function" whose "algorithm + its golden derivation vectors are **deferred to the implementation step (N4)**". Meanwhile 14 distinct ids — `ws.tree`, `gyld.ask` — were typed by hand into app files and are, by that same sentence, already frozen. A newcomer writing their first binding cannot tell whether to type an id or compute one; the function to compute one is documented as not existing. No page states a syntax rule for a hand-typed id (charset, the dot's meaning, length) or its relationship to the derivation.

**Impact.** The first token of the line has no stated rule on a front page that says the token is frozen forever once used. Ranked P3 rather than P2 only because the same sentence documents an escape hatch — "renames are alias records" — so a wrong hand-typed id appears recoverable without a compatibility break.

**Required correction.** State on the contract's front page whether an app-file glade id is authored or derived; if authored, give its syntax rule and say what `derive_glade_id` is then for; if derived, say what the 14 existing hand-typed ids are.

**Closure test.** A newcomer can produce a legal glade id for a new surface from the front page alone, and knows whether it is frozen the moment they write it.

---

## 2. The first-day walkthrough

### Walkthrough A — write a new app file with one `value`, one `log`, one `crdt` binding

**Step 1 — find the format.** `glade/README.md` says the repo "contains scaffolding only" and points at `docs/` for "Public support contracts and user-facing documentation"; `glade/docs/` holds one 8-line README. The grammar is in `glade/dev-docs/GladeGrazelAttachNotes.md`, classified by `glade/README.md:24` as "Internal engineering design", titled "Grazel attach — engineering notes (Lane R step 4)". **Guess #1:** that an internal note about attaching one supplier is the specification of the format. (P3-1.)

**Step 2 — header and order.** `glade-app v0`, then `app <name>`. Learnable from `GladeGrazelAttachNotes.md:30-34` and confirmed by the diagnostic "`app` must be declared before any binding". Directive order is otherwise free — `grazel-app.glade` puts bindings before `service`, both test fixtures put `service` first. **Held, no guess.**

**Step 3 — the `value` binding.** Copy `binding ws.tree value share commons latest` and substitute an id. **Guess #2:** what id to type. `glade-decl/README.md:28` says ids are derived from package id + grip key and frozen once shared, and `:50` says the derivation is deferred; 14 ids in the corpus are hand-typed dotted words. No syntax rule anywhere. I typed `doc.title` because it looked like the neighbours. (P3-5.) **Guess #3:** `share` — the only authority ever written; `external` exists in the contract with no token, so I could not have chosen it anyway. **Guess #4:** `commons` — meaning unstated, `private` used once with no stated reason, and the file's own comment says the mount fills it. (P2-1.) **Guess #5:** `latest` — meaning unstated; picked because 12 of 12 non-log bindings use it.

**Step 4 — the `log` binding.** Copy `binding gwz.output log share commons from_cursor`. **Guess #6:** that `from_cursor` is right for a log — 13 of 13 log/swmr bindings use it, so it is copy-safe, but nothing states what it does. `glade-gwz/README.md:59-77` explains how a log surface is *used* (ops keyed by `run_id`, folded by a subscriber) and never names a retention.

**Step 5 — the `crdt` binding. DEAD END.** `crdt` appears in no app file and in no page read. I could determine none of:
- the authority a crdt takes;
- the zone;
- the retention — and here the surface actively misleads. `glade-decl/README.md:41` glosses `ZoneKind` as "who converges within it"; `glade-gyld/README.md:934` says `value` folds "last-writer-wins by (lamport, origin)". A CRDT converges by merge, which is the opposite of last-writer-wins, so `latest` — the only retention ever written on a non-log binding — reads as wrong for a `crdt`, and there is no fourth option;
- the `profile` token's legal values (P2-4);
- whether omitting `profile` fails at parse or produces a surface that silently will not mount (P2-4);
- whether the arity check even permits six tokens, since the diagnostic template shows five.

I could not write this line at all. It is the amendment's headline addition and it is unwritable from the published surface.

### Walkthrough B — migrate an app file that uses `from-cursor` and `windowed`

**Step 1 — learn that I must.** No CHANGELOG, migration or upgrade note exists in any of the five repos (verified). The header still says `glade-app v0`, so nothing in the file signals a language change. **First notice: the node refuses to boot.** (P2-2.)

**Step 2 — `from-cursor`.** Diagnostic prints `["latest","from_cursor","ttl"]`; the replacement is visibly in the set and I spot it. **Resolved, one keystroke, every time, forever** — because the new spelling is the only underscore in a format whose other joined tokens (`glade-app`, `ws-razel`, `glade-gyld`) are all hyphenated. (P3-3.)

**Step 3 — `windowed` on `term.log`. GUESS, AND I GUESS WRONG.** The printed set contains nothing resembling `windowed`. `term.log` is terminal scrollback, so the intuitive migration is `ttl` (bounded by time) or `latest`. The ruled answer is `from_cursor`. `latest` would silently convert an append log into a last-writer-wins value; `ttl` parses and leaves the duration unsayable. No message names the replacement, although the format already names one for `exchange`. (P3-2, P2-3.)

**Step 4 — both copies of `grazel-app.glade`.** Learned correctly from the file's own header (`:10-13`) and from `grazel/README.md:7-8`, both of which name the two homes and say to edit them together. **Held — this is the surface working.**

**Step 5 — reboot. DEAD END.** The retention token changed, so the payload bytes changed, so by the documented diff rule (`GladeGrazelAttachNotes.md:58`, keyed on "(glade_id, payload bytes)") a second `BindingDecl` appends for each already-registered id. No page states which declaration is then live, and the grammar has no retract form. I cannot determine from the surface whether my migration took effect. (P2-5.)

**Step 6 — verify offline.** There is no `--check`, `--validate`, lint or dry-run for an app file in any page read; the only `--check` flags are garns codegen and the decl corpus drift gate. The sole verification of a 13-line, 5-file, mandatory rewrite is booting a node.

---

## 3. Invariant analysis — what I attacked and what held

- **Token *order* across the line.** Shape → authority → zone → retention is stable across all 22 lines, in all five files, and matches the grammar comment reproduced at the top of every binding block. A user copying a neighbour cannot get the order wrong. Held.
- **The grammar comment is present in every app file.** All five files carry `# binding <glade_id> <shape> <authority> <zone> <retention>` immediately above their binding block. A user editing any file has the grammar in front of them without leaving the file. Held, and it is the best thing about this surface.
- **Dual maintenance of `grazel-app.glade` (proposed surface item 7).** Verified byte-identical (`diff` clean). Both copies carry a loud `DUAL-MAINTENANCE NOTE` naming both paths, saying which repo owns the declaration and which is the demo's copy, and saying to edit them together; `grazel/README.md:7-8` repeats it; draft §4.4 flags it for the migration and cites a test that fails otherwise. The surface tells the user, in the file, at the moment of editing. **Attacked, held — no finding.**
- **`seed` and `service` line consistency (item 6).** `service <name> <exchange-glade-id>` and `seed <principal> <share> <verb[,verb...]>` use the same bare-word positional style as `binding`, the same dotted ids, and the same hyphen convention for names (`glade-gyld`, `ws-razel`). Their comment blocks in `grazel-app.glade` and `gyld-app.glade` explain the `verb.*` pattern and why a new surface needs no new seed. Leaving them outside the contract creates no spelling or placement inconsistency I could find. Held.
- **`message` and `window` staying reserved-but-unauthorable (item 1).** The two-tier shape model already exists and already produces a distinct, correct diagnostic (`:117` for unknown, `:121` for known-but-unimplemented). Adding `atom` to the recognised tier is consistent with it. Held, except the `exchange` documentation defect (P3-4).
- **Line diagnostics carry line numbers.** Every message in `appdecl.rs` begins `line {n}:`, including the arity template, the duplicate-id check and the ordering checks. For a hand-edited file this is the right posture and the amendment extends it to zone and retention. Held.
- **`external` authority and `source` as declaration-only (item 5).** No app file uses them, no page invites a user to, and the authority check refuses unknown tokens with the accepted set. A user cannot stumble into them. Held as a *surface* matter — though `Retention`'s parameters are a third such gap that the draft does not name (P2-3).

---

## 4. Risks and next action

The through-line of the five blockers is one habit: **the amendment turns tokens from ignored to enforced, and from free-form to frozen, without turning them from undocumented to documented in the same wave.** Validation and specification have to land together, because after the freeze only the diagnostic can teach, and a diagnostic can teach spelling but not meaning.

Two of the blockers are cheapest to fix now and expensive later in a specific, nameable way:

- **P2-3 / P2-4 (position 6).** The moment `profile` occupies the last positional slot, the format can never parameterise `ttl`, and `ttl` is in the frozen set. A keyword tail after the five positional tokens resolves both, keeps the grammar open, and costs one paragraph in the draft. This is the single highest-leverage change available before the freeze.
- **P2-2 (the version token).** `glade-app v0` is already there and already parsed. Using it costs a line in the parser and turns an unannounced breaking change into an announced one with a migration path. Not using it burns the mechanism permanently.

**Next action for the lane owner:** revise draft §4.4 to (1) state zone's per-value meaning, default and mount-override behaviour and reconcile the contradicting sentence in both app-file copies and `GladeGrazelAttachNotes.md:49`; (2) advance the header to `glade-app v1` with `v0` accepted-and-warned; (3) replace the positional sixth token with a keyword tail and place `ttl`'s duration in it, or hold `ttl` back as reserved; (4) state `profile`'s legal values, per-shape omission semantics and parse-time enforcement, and update the arity template at `appdecl.rs:113`; (5) state what a changed or deleted binding line does to an already-registered declaration, before any of the 13 lines is rewritten. The five P3s are text and can ride the same revision.

I pre-commit to GO on a revision that resolves P2-1, P2-2, P2-3, P2-4 and P2-5 as specified above.
