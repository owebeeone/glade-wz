# GladeDeclAmendment — SURFACE-AXIS RE-VERDICT (round 2)

**Review object:** The glade-decl contract v1 amendment as landed (plan Steps 2.1–2.8 of `dev-docs/GladeFirstSlicePlan.md`), revised by the round-1 remediation in `dev-docs/glade/GladeDeclAmendment-RemPlan.md` (root `8056056`; the owner's "all recommended" at `0e4d882`). This re-verdict covers the Surface axis. Its round-1 report, `dev-docs/glade/GladeDeclAmendment-ReviewSurface.md`, was filed at root `87ce46f`.

**Baseline:** The revised tuple was verified at the start and at the end.
- **Root:** `73c2bf7e9a72` at the start. During the review it moved to `690f2f6807f9` ("Lock glade-discover at its braced conformance modules").
  - `git diff --name-status 73c2bf7 690f2f6` lists three files, not the two the lane owner's note named:
    - `gwz.conf/gwz.lock.yml` — its only member change is `mem_glade_discover` 48bc104 → 9caac2e, which is out of scope;
    - `gwz.conf/markers/conf-integrity.yml`;
    - `gwz.conf/markers/01a0ce91-ec73-7a17-9492-a12c9c0b2426.yaml`, an added gwz marker.
  - All three are gwz bookkeeping, and no in-scope path changed. I treated the tuple as held, as the note allows.
- **Members:** glade `1b9ac3fd5bd6`, glade-decl `3d109177a222`, glade-decl-rs `d3be799db4f3`, glade-decl-ts `85ec18da1cb7`, glade-decl-py `ee2f9602b2c1`, grazel `e1a4078d211f`.
- **Unchanged:** glade-gyld `c9ef7a66b6e4`, glade-gwz `fc0bb990ef09`, glial `4c6e8561c174`, grip-core `97ff6c26f12e`, gryth-wz `3c64e7a59064`, gryth-wz/glade-decl-ts `37d9b7d8a680`.
- **Working tree:** no source file below had a working-tree change against its member's HEAD, at the start or at the end.

**Date:** 2026-09-24. The review ran from 2026-09-23 23:51 AEST to just after midnight.

**Axis:** Surface — the interface as an author meets it:
- the user-facing pages;
- the app files and their in-file grammar;
- the node's message texts, read only through the one permitted grep.

No code was read, and no design or plan document beyond the RemPlan. Independent, adversarial, read-only. I have not seen the other axes' round-2 reports. The RemPlan's citations of their round-1 findings were used as input, which is allowed. Filed verbatim by the lane owner.

**Verdict: GO.**
- **Round-1 findings:** 6 CLOSED (SUR-P3-1, -2, -3, -5, -7, -8), 2 DEFERRED-ACCEPTED (SUR-P3-4, -6), 0 NOT CLOSED, 0 DISPUTED.
- **New findings:** 3 P3 (SUR-P3-9, -10, -11); 0 P0, 0 P1, 0 P2; none is architectural.
  - SUR-P3-9 is in the remediation's own new text.
  - SUR-P3-10 and SUR-P3-11 are older behaviours, disclosed by the lane owner.

---

## 0. Evidence base

**Read at the revised tuple:**
- **`glade/docs/AppFileFormat.md`** (354 lines), in full, plus its diff `dddf8b8..1b9ac3f`.
- **`glade/README.md`** (38 lines) and its diff. `glade/docs/README.md` is unchanged.
- **`glade/dev-docs/GladeGrazelAttachNotes.md`**, format section L25-83, plus that section's diff.
- **`dev-docs/glade/GladeDeclSurface.md`**, the diff in `47d3568` (rows L30 and L32).
- **glade-decl:**
  - `README.md`;
  - the diff `784840a..3d10917`, covering the README, `dev-docs/OpenNotes.md` and the `DeclSurface.md` mirror;
  - OpenNotes: the preamble, N3, N7, N10 and N11.
- **The three rendering READMEs**, in full, plus their README-only commits d3be799, 85ec18d and ee2f960.
- **`grazel/README.md`** L19-42 and L71-88, plus the diff `b604a06..e1a4078`.
- **The five app files.**
  - `git diff --quiet` shows them unchanged in every range.
  - The two grazel-app.glade copies are still byte-identical (`cmp`).
- **The RemPlan** (157 lines), in full.
- **The node's messages:** the one permitted grep over `glade/node/src/appdecl.rs` and `appdecl/tail.rs`.

**Not visible through the permitted grep** (none of these texts contain its keywords):
- the wording of the two-files refusal;
- the wording of `EXTERNAL_WARNING` and of the v0 misplaced-entry constant;
- the node's top-level error print;
- its missing-file error.

What these say or do rests on the grep's templates (`` line {n}: {EXTERNAL_WARNING} ``, `` …; {MISPLACED_V0} ``), the page, the RemPlan and the lane owner's disclosures.

**No tests were run.** This axis is read-only, so the RemPlan's tests are cited, not observed.

## 1. Closure table (round-1 findings)

| ID | RemPlan disposition | Status | Evidence |
|---|---|---|---|
| SUR-P3-1 | Case 2 = COD-P2-1 = STA-P2-1, refusal variant: "an app is declared by one file". Cases 1 and 3: text rider. No warning when two apps declare one id (by design). | **CLOSED** | Case 2: page L42-47, L76-79 and L291-294 give the rule and its reason ("would withdraw each other's surfaces at every start"): refused before any write, naming both files. Attach notes L69-71 agree. Case 1: L315-319 state the rename's resurrection exactly as I reproduced it. L320-324 give the retirement route, which overclaims (see SUR-P3-9). Case 3: L298-310 define the fold per (app, glade id), the newest live declaration across apps, and "never touched … becomes the surface's live declaration", so the round-1 contradiction is gone. The declined warning is an owner decision, and the precedence is now stated; accepted. |
| SUR-P3-2 | Rider COD-P3-2 + SUR-P3-2, items (1)–(5) | **CLOSED** | New message templates `{t}; a later node release refuses the line` and `line {n}: {}; {V1_REFUSED_LATER}`. Header error `` expected a header, got `{line}`: write `glade-app v1` (`glade-app v0` is accepted for old files) `` (also for an empty file). Page L66-75: v0 has "no release … scheduled to stop loading it", one grammar, tail and `crdt` included. L211-221 and L271-283: at version `0.0.0` there is no release yet; the first release warns and the release after refuses. L254-255: bare `ttl` stays legal under either header and names no duration. Residual in §6. |
| SUR-P3-3 | Rider COD-P3-4 + SUR-P3-3 | **CLOSED** | glade-decl README L52-60 and L136, and GladeDeclSurface L30 and L32 (`47d3568`, mirror re-copied in `3d10917`), now say: "validates the tail keys … and records neither". OpenNotes N3 and N7 match; N10 records the stale schema comment. The `crdt` refusal now reads `` shape `crdt` needs `shape-profile=text_crdt` (a `crdt` line must name its profile) ``. Page L150-156 gives the next step: set `MountConfig.crdtProfile`, "or glial refuses the mount". |
| SUR-P3-4 | Deferred to Step 4.3 (grant check at the serve hop), with the owner | **DEFERRED-ACCEPTED** | The RemPlan's reason (L125) holds. Grants are inert until Step 4.3. Correcting seed lines before a revocation route exists would add grants and leave the old ones live. The revocation route lands in the same step that makes grants live, so the hazard and its remedy arrive together. Page seed text unchanged (L165-171, L344-345). |
| SUR-P3-6 | Deferred to Step 4.3, with the owner | **DEFERRED-ACCEPTED** | Same reason. The owner has since ruled that a seed's share is the workspace share (disclosure 3). Non-blocking residual in §6: the page could state that ruling now, text only, while it still points to grazel-app.glade (`seed owner grazel …`) as "a longer example" (L46-47). |
| SUR-P3-5 | Page text only; (s) recorded as an owner question at Step 4.6 | **CLOSED** | Page L333-336: "`glade-app v1` gives `service` and `workspace` lines no retraction. That was decided, not missed … whether a later format adds one is an open question." L340-343 give the claim's behaviour: the node stops serving the share from its next start, and the entry stays. This is my round-1 remedy's "none in v1, with the reason" option. |
| SUR-P3-7 | Warning under both headers; page; OpenNote | **CLOSED** | The template `` line {n}: {EXTERNAL_WARNING} `` is present; its wording is not visible. Page L108 says "accepted with a warning … the binding registers and nothing acts on it". N11 records what such lines become once a source token exists, tied to that token's design. The page tells the author the line is inert today, which is what they need. Nit in §4. |
| SUR-P3-8 | Rendering "Contract v1" blocks; glade status and link; grazel link and stderr | **CLOSED** | The rendering blocks (rs L13-31, ts L14-31, py L13-30) name the package and how to depend on it, the pin to `7d18cd3`, `AdvertisementRecord` removed with its reason (R7(b), GDL-029), and the declared-but-not-authorable members. `glade/README.md` L17-19 gives the status and links `docs/AppFileFormat.md`; the libp2p focus block is gone. `grazel/README.md` L81-87 links the format page and says node stderr is forwarded line by line as `[node] <file>: warning: line N: …`, with a 20-line tail on exit. |

**Rider CON-P3-11 (from round 1) — now closed from the author's side.**
- The RemPlan identifies option (s) as the retraction for `service` and `workspace` lines, declined in R9.
- The page states that no such retraction exists, and why (L333-336).
- **SAF-P3-13** is unchanged (L325-329) and still closed from the author's side.
- **The Surface residual** (no user-facing grammar page) is still closed.

## 2. First-day walkthrough, redone from the page alone

```text
glade-app v1
app pantry
binding pantry.settings value share commons latest
binding pantry.events   log   share commons from-cursor
binding pantry.doc      crdt  share commons from-cursor shape-profile=text_crdt
binding pantry.cache    value share commons ttl ttl=15m
seed owner ws-pantry read.*,pantry.*       # share = workspace share, as in the page's example
workspace ws-pantry pantry
```

- **Start:** `glade-node --profile local --app pantry.glade` (L42-43).
- **The `crdt` document now has a next step:** the application that mounts `pantry.doc` sets `MountConfig.crdtProfile` to `text_crdt`, or glial refuses the mount (L150-156). Round 1 found no next step here.

| Mistake | What the node says now (from the texts) | Page agrees? |
|---|---|---|
| Token missing, no tail | Refused, with the binding template, inside `Error: Custom { kind: InvalidData, error: "…" }` (disclosure 2) | Yes (L200-205), except the wrapper (SUR-P3-11) |
| Token missing, tail present, `glade-app v1` | Refused: `` `ttl=15m` is a key=value entry where <retention> goes (the tail follows all five tokens) `` | Yes (L130-135) |
| The same in a `glade-app v0` file | Warning `` … where <retention> goes; {MISPLACED_V0} ``; the token is stored as written and the node starts | Yes (L136-138) |
| `windowed` (v1) | Warning `` unknown retention `windowed` (removed; use `from-cursor`); a later node release refuses the line `` | Yes (L271-279) |
| `from_cursor` (v1) | Warning `` retention `from_cursor` is the contract's spelling (use `from-cursor` in a file) ``, plus the same suffix (RemPlan item (3)) | Yes |
| Unknown zone (v1) | Warning `` unknown zone `shared` (one of ["commons", "private"]); a later node release refuses the line `` | Yes (L211-217) |
| `crdt` without a profile | Refused: `` shape `crdt` needs `shape-profile=text_crdt` (a `crdt` line must name its profile) `` | Yes (L107, L133) |
| Bad duration; `ttl=` on a `latest` line | Unchanged refusals | Yes |
| `glade-app v0` header | Warning, unchanged; v0 now has no scheduled end and uses the same grammar | Yes (L66-75) |
| No header, or a misspelt one | `` line 1: expected a header, got `app pantry`: write `glade-app v1` (`glade-app v0` is accepted for old files) `` | Yes |
| `external` authority | Line-numbered warning under either header; the binding registers | Yes (L108) |
| Two files both saying `app pantry` | Refused before any write, naming the app and both files (text not visible) | Yes (L42-47, L291-294) |
| The same file given twice | Refused (disclosure 1) | Covered by "once per app" (L43-44) |
| A mistyped `--app` path | An I/O error that names no path (disclosure 2) | No: the page is silent (SUR-P3-10) |

**Edits.**
- **Change a binding line:** covered (L298-304).
- **Change only a tail value:** registers nothing, because the tail is not recorded. This can be deduced from L146-148 — a nit, as in round 1.
- **Delete a binding line:** covered (L305-310).
- **Change the `app` line:** now covered (L315-319).
- **Retire an app:** L320-324, which overclaims (SUR-P3-9).
- **Delete a `service`, `workspace` or `seed` line:** covered (L333-345).
- **Leave the file out of `--app`:** covered (L311-314).
- **Split an app across two files:** refused (L291-294).

**Where I still had to guess.**
- **Deferred to Step 4.3:**
  - what `service <name>` refers to;
  - which verbs exist and what principals are.
  - The page does not yet state the owner's ruling on a seed's share (the workspace share).
- **New:** where `MountConfig.crdtProfile` is documented. The name is given, but not linked.
- **As in round 1:**
  - `--app` without `--profile`;
  - what relative paths are resolved against;
  - that a display name must be one token;
  - "plain engine" on the page versus "bare engine" in N7;
  - `GDL-041` appearing on a public page;
  - whether the hyphen rule reaches names the author chooses;
  - how to read the node's version, which the page now cites (`0.0.0`).

## 3. New findings

### [SUR-P3-9] The retirement steps promise more than the node does

- **Location:**
  - `glade/docs/AppFileFormat.md` L320-324: "To withdraw every surface an app declared … each of the app's declarations is retracted … Retiring the old name this way is how to rename an app without leaving its declarations live".
  - It conflicts with:
    - L76-77: `service` records carry the app name;
    - L160-161: an exchange is "a directed request/response surface";
    - L333-345: `service` and `workspace` lines get no retraction, and a seed's grant stays.
- **Invariant:** A documented procedure states its whole effect, and the page does not contradict itself.
- **Reproduction:**
  1. Take the page's own example app. `app notes` declares `service notes notes.ops`, `seed owner ws-notes …` and `workspace ws-notes notes`.
  2. Retire it as L320-324 says: start once with a file holding only `glade-app v1` and `app notes`.
  3. By the page's own "Other lines" section:
     - `notes.ops` stays declared and routable;
     - the workspace entry naming the node stays;
     - the seed's grant stays.
  4. Only the binding declarations are retracted. The rename recipe has the same gap: the old name's `service` declarations stay live.
- **Impact:**
  - Both shipped authored apps declare an exchange (grazel-app and gyld-app). An author who retires or renames such an app believes it is gone.
  - Its exchange stays declared and routable to a node that may no longer have a provider. That is the state L333-339 describes, but the retirement bullet tells the author it does not apply.
- **Remedy:**
  - Scope the bullet: "each of the app's `binding` declarations is retracted; its exchanges, its workspace entry and its seed grants stay (see Other lines)."
  - Make the same change in the rename sentence.
- **Closure test:**
  - A page check.
  - Optionally, extend the RemPlan's retirement test to assert that the retired app's `service` record is still declared, which pins the documented behaviour.
- **Classification:** not ARCHITECTURAL.

### [SUR-P3-10] A missing `--app` file is reported without its path

This behaviour predates the amendment and was disclosed by the lane owner.

- **Location:**
  - The node's start-up file read. It is not visible to this axis; the lane owner disclosed it.
  - The page L42-47 now says the flag repeats "once per app".
  - The page L86-87 says a start-up refusal names the file.
- **Invariant:** An error about one of several inputs names which one.
- **Reproduction:**
  - `glade-node --profile local --app pantry.glade --app pantyr-extra.glade`, where the second path is a typo. The node stops with an I/O error that names no path (disclosure 2).
  - Under grazel, with the gyld leg on and grazel started from another directory, the relative default `--gyld-app apps/gyld-app.glade` fails the same way. grazel then prints the node's last 20 stderr lines, and none of them names the file.
- **Impact:**
  - The author must guess which of several paths is wrong.
  - This is the commonest start-up mistake, and the page's promise that the message names the file does not hold for it.
  - The remediation's preflight now reads every `--app` file before anything else, so a multi-file start can fail here first.
- **Remedy:** Attach the path to the read error (`<path>: <os error>`), as parse errors already do.
- **Closure test:** A binary test with a nonexistent second `--app` path, asserting that stderr names that path and that nothing is written under the temporary `GLADE_HOME`.
- **Classification:** not ARCHITECTURAL.

### [SUR-P3-11] A refusal reaches the author as a Rust Debug dump

This behaviour predates the amendment and was disclosed by the lane owner.

- **Location:**
  - The node prints a load refusal through `io::Error`'s Debug form, `Error: Custom { kind: InvalidData, error: "…" }` (disclosure 2).
  - The page L86-90 describes a refused start and shows how a warning is printed (`<file>: warning: line N: …`), but not how a refusal is printed.
- **Invariant:** What the node prints for a refused file is the message the page describes, readable as written.
- **Reproduction:**
  - Any refusal, for example a binding line missing its retention, prints its message inside `Error: Custom { kind: InvalidData, error: "…" }`.
  - Debug escapes the message's own quoting. `unknown authority`'s list `["share", "external"]` arrives as `[\"share\", \"external\"]`, and a backslash in a path, such as a Windows path in the two-files refusal, is doubled.
  - Under grazel the same line arrives as `[node] Error: Custom { … }`.
- **Impact:**
  - Every refusal the amendment added reaches a person wrapped in a Rust-internal structure: the tail refusals, the header refusal, and the two-files refusal that names both paths.
  - A newcomer reads a crash rather than "line N of your file".
  - It is cosmetic for an experienced reader, but it applies to every refusal, and the page shows no example of one.
- **Remedy:** Print a refusal in its Display form — one line and a non-zero exit — or show the actual printed form on the page.
- **Closure test:** A binary test asserting that a refused file's stderr reads `<file>: line N: …`, with no `Custom {` and no escaped quotes.
- **Classification:** not ARCHITECTURAL.

## 4. Invariant analysis — attacks on the new and changed texts

**Attacks that failed (the texts hold):**
- **The v0/v1 split.** The page (L66-75) claims the header decides only three things: how an unaccepted zone or retention is treated, how a misplaced `key=value` entry is treated, and the header warning itself. The message families match:
  - v1 zone and retention warnings carry the refusal suffix;
  - v0 ones name `glade-app v1` and carry no suffix;
  - a misplaced entry is a v1 refusal ("(the tail follows all five tokens)") and a v0 warning ("; {MISPLACED_V0}");
  - `external` warns under both headers (N11).
- **The fold and the retraction now agree.** The round-1 conflict between "folded by glade id" and "never touched" is resolved by the per-(app, glade id) fold in L298-310.
- **Two files, one app.** The page explains the rule by its reason and says the refusal comes before any write (L88-89, L291-294). The attach notes (L69-71) agree.
- **Deleting a `workspace` line.** The claim now follows the loaded files (L340-343).
  - Nit: grazel-app and gyld-app both declare `ws-razel`. Deleting the line from one of them leaves the share served. This follows from the page's "because" clause, but the page does not say it.
- **The `crdt` path.** It is stated with its failure mode (L150-156). The contract README (L52-60), GladeDeclSurface (L32) and N7 now say the same.
- **The header error** now points to v1.
- **v1 warnings** now say that a later release refuses the line, and the page ties that to release numbers.
- **The rendering blocks and grazel's stderr paragraph** match the pages they point to.

**Noted, not filed:**
- `glade-decl-py/README.md` L55 still copies the IR and the corpus, which `build.py` already writes; the TS block dropped this line. Harmless.
- The contract README's `external` row (L109) still reads only "parses … declared, not yet authorable". It does not mention the new warning; the page and N11 do.
- N7 opens "An app file writes it as a keyword tail…" before the same note says the node records nothing.
- The `crdt` refusal's parenthetical restates the rule. The page does not say why a token that "reaches no mount" is still required; N7 gives the reason, which is forward-looking.
- GladeDeclSurface.md's Status line still reads "working draft".
- The public page cites an internal design document (L335-336). That is acceptable for traceability.

## 5. Riders

CON-P3-11, SAF-P3-13 and the Surface residual are recorded below the closure table in §1.

## 6. Risks and next action

- **The release anchor will go stale.** The page's "glade-node has had no release yet (its version is `0.0.0`)" (L215-216; also L277-279) becomes false at the first release.
  - The RemPlan's `V1_WARNING_RELEASE` check pins the code constant, not the page, and the warning text never names the release.
  - Extend the check to cover the page, or change the page as part of cutting the release.
- **The page now promises cross-app duplicate glade ids.** L302-304 says two apps may declare one glade id: "the node does not warn, and the newer live declaration stands".
  - This sits beside GQ-6 ("frozen once shared") and the open multi-origin question at Step 4.6.
  - Withdrawing that permission later would be a compatibility break. If withdrawal is at all likely, decide before the publish.
- **Seeds, before Step 4.3.** The owner's ruling (a seed's share is the workspace share) could go on the page now, with a comment beside grazel-app.glade's seed lines.
  - Both are text only and write no record.
  - The page still presents that file as the example to copy, so new authors would stop copying `seed owner grazel …`.
- **gryth-wz's rendering copy lags.** gryth-wz/glade-decl-ts sits at `37d9b7d`, whose README lacks the Contract v1 block. `85ec18d` descends from it by a README-only commit, and the code bytes are identical. Fast-forward it when convenient.
- **Unchanged from round 1.** `#` starts a comment anywhere and tail values are single tokens. Both may collide with a future `source=` value if that value is a URL.
- **Next action:** GO for the publish.
  - SUR-P3-9 is a one-bullet text fix.
  - SUR-P3-10 and SUR-P3-11 predate the amendment: fix them or record them.
  - Re-verify the tuple at publish time.
