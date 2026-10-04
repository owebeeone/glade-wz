# GladeDeclAmendment — SURFACE-AXIS RE-VERDICT (round 2)

**Review object:** the glade-decl contract v1 amendment (plan Steps 2.1–2.8) with its single round-1 remediation patch. The patch is planned in `dev-docs/glade/GladeDeclAmendment-RemPlan.md` (root `8056056`). The owner confirmed "all recommended" at `0e4d882`. Status: remediated; publish pending. Reviewed 2026-09-23.

**Baseline:** the revised tuple, read at the start (23:51) and at the end (23:59), unchanged:

| Repository | Commit |
| --- | --- |
| glade-wz root | `73c2bf7e9a72` |
| glade | `1b9ac3fd5bd6` |
| glade-decl | `3d109177a222` |
| glade-decl-rs | `d3be799db4f3` |
| glade-decl-ts | `85ec18da1cb7` |
| glade-decl-py | `ee2f9602b2c1` |
| grazel | `e1a4078d211f` |
| glade-gyld (unchanged) | `c9ef7a66b6e4` |
| glade-gwz (unchanged) | `fc0bb990ef09` |
| glial (unchanged) | `4c6e8561c174` |
| grip-core (unchanged) | `97ff6c26f12e` |
| gryth-wz root | `3c64e7a59064` |
| gryth-wz/glade-decl-ts | `37d9b7d8a680` |

- Every source page and app file, the RemPlan, and the two node source files the grep reads showed no working-tree change against HEAD, at start and at end.
- How sources were read:
  - pages: Read, cat, sed and grep;
  - the node's messages: only through the one permitted grep;
  - `git log` and `git diff` over the remediation ranges, restricted to the permitted paths;
  - `cmp` on the two grazel-app.glade copies.

**Date:** 2026-09-23

**Axis:** Surface — the interface as an author meets it: the user-facing pages, the app files and their in-file grammar, and the node's message texts. No code and no design or plan document was read beyond the RemPlan. Independent, adversarial, read-only. Filed verbatim by the lane owner.

**Verdict: GO.**
- Round-1 findings: 6 CLOSED, 2 DEFERRED-ACCEPTED, 0 NOT CLOSED, 0 DISPUTED.
- New findings: 0 P0, 0 P1, 0 P2, 3 P3, none ARCHITECTURAL.
  - SUR-P3-9 is in text this patch added.
  - SUR-P3-10 and SUR-P3-11 are older behaviour the lane owner disclosed.

---

## 0. Evidence base

**What changed in my sources since round 1.**

| Repository | Commit | Changed |
| --- | --- | --- |
| glade | `1b9ac3f` | `docs/AppFileFormat.md` (+125/−65), `README.md`, the attach notes' format section. Both `apps/grazel-app.glade` copies are unchanged and still byte-identical. |
| glade-decl | `3d10917` | `README.md`, `dev-docs/OpenNotes.md` (N3, N7, new N10 and N11), the `DeclSurface.md` mirror |
| glade-wz root | `47d3568` | `GladeDeclSurface.md` rows `Retention` and `ShapeProfileDecl` |
| glade-decl-rs, -ts, -py | `d3be799`, `85ec18d`, `ee2f960` | `README.md` only; the code bytes did not move |
| grazel | `e1a4078` | `README.md` only; the app files are unchanged |

glade-gyld and glade-gwz did not move.

**Read in full:**
- the RemPlan (157 lines);
- the revised `AppFileFormat.md` (355);
- `glade/README.md` and `glade/docs/README.md`;
- the attach notes' format section (L25-84);
- the changed rows of `GladeDeclSurface.md`, plus its status line;
- the changed passages of `glade-decl/README.md` (L47-60, L104-112, L132-140);
- OpenNotes' preamble, N3, N7, N10 and N11;
- the three rendering READMEs;
- `grazel/README.md` L19-42 and L71-90;
- the declaration lines of all five app files.

**Messages.** The one permitted grep printed about 150 distinct strings.

These new texts are visible:
- the v1 escalation suffix: `` {t}; a later node release refuses the line `` and `` line {n}: {}; {V1_REFUSED_LATER} ``;
- the header error: `` line {n}: expected a header, got `{line}`: {HEADER_TO_WRITE} `` and `` empty file: expected a header: {HEADER_TO_WRITE} ``, where the header text is `` write `glade-app v1` (`glade-app v0` is accepted for old files) ``;
- the corrected `crdt` refusal: `` shape `crdt` needs `shape-profile=text_crdt` (a `crdt` line must name its profile) ``;
- the v0 misplaced-entry warning: `` `a=b` is a key=value entry where <zone> goes; {MISPLACED_V0} ``;
- the `external` warning: `` line {n}: {EXTERNAL_WARNING} ``.

**Limit of the message evidence.** The grep cannot show three texts, because none of their words match its keywords:
- what `EXTERNAL_WARNING` says;
- the tail of the v0 misplaced-entry warning;
- the two-files refusal, which lives in `appdecl.rs`.

For these three I rely on the page (L42-47, L108, L136-138, L291-294), the attach notes (L63-71), N11 and the RemPlan's tests. That they exist is established; their exact wording is not verifiable on this axis.

**Not read:** any code; the other axes' reports (the RemPlan's summary of them is the input this round allows); any other file in the review scratchpad.

## 1. Closure of round-1 findings

| ID | RemPlan disposition | Status | Evidence |
| --- | --- | --- | --- |
| SUR-P3-1 | Case 2 goes to the refusal variant of COD-P2-1 = STA-P2-1. Cases 1 and 3 get page and attach-notes text; no warning for cross-app ids. | CLOSED | See **SUR-P3-1 evidence** below. |
| SUR-P3-2 | Rider (1)-(5): warn-then-refuse lifecycle, warning texts, header error, v0 statements, bare `ttl` | CLOSED | See **SUR-P3-2 evidence** below. |
| SUR-P3-3 | Rider: correct the contract texts; `crdt` refusal parenthetical; page says where the profile comes from | CLOSED | See **SUR-P3-3 evidence** below. |
| SUR-P3-4 | Deferred with the owner to Step 4.3, the grant check at the serve hop | DEFERRED-ACCEPTED | See **SUR-P3-4 evidence** below. |
| SUR-P3-5 (ARCHITECTURAL) | Page text only; whether (s) comes later is recorded at Step 4.6 | CLOSED | See **SUR-P3-5 evidence** below. |
| SUR-P3-6 | Deferred to Step 4.3; since then the owner ruled a seed's share is the workspace share (disclosure 3) | DEFERRED-ACCEPTED | See **SUR-P3-6 evidence** below. |
| SUR-P3-7 | Warning under both headers; page text; OpenNote | CLOSED | See **SUR-P3-7 evidence** below. |
| SUR-P3-8 | Rendering "Contract v1" blocks; glade status and link; grazel link and stderr | CLOSED | See **SUR-P3-8 evidence** below. |

**SUR-P3-1 evidence.**
- *Case 2 (two files, one app).* The page states the rule three times:
  - L42-47: "The flag may repeat, once per app … a node given two files that name the same app refuses to start, naming both files".
  - L76-79: the `app` bullet.
  - L291-294: "would withdraw each other's surfaces at every start … refuses to start, before it writes anything".
  
  The attach notes say the same at L70-71. The refusal's wording is not visible through the grep (§0).
- *Case 1 (rename).* L315-319 states that an old declaration can come back. L320-324 gives a retirement procedure.
- *Case 3 (one id, two apps).* L298-310 gives the precedence:
  - the fold is per `(app, glade id)`;
  - "the newest declaration still live across apps is the live one";
  - "if one is still live it becomes the surface's live declaration".
- *Declined warning.* The owner declined the warning I asked for in case 3 ("the fold allows it by design"). The page now states that nothing warns (L302-304). The invariant I filed — the rules are stated — holds.
- *Residual.* The new retirement bullet overstates its effect; filed as SUR-P3-9.

**SUR-P3-2 evidence.**
- The v1 zone and retention warnings now end "; a later node release refuses the line".
- The page names the schedule (L211-221, L271-283): "glade-node has had no release yet (its version is `0.0.0`): its first release reports such a zone as a warning, and the release after that refuses the file".
- On `v0` (L66-75): "no release is scheduled to stop loading it", and it has one grammar, "the keyword tail and `crdt` included".
- Bare `ttl` "stays legal under either header and names no duration" (L254-255).
- The header error steers to v1.
- Residual, not blocking: see §4 on the "no release yet" sentence.

**SUR-P3-3 evidence.**
- `glade-decl/README.md` L52-60 and L136 now say the node "validates … and records neither".
- `GladeDeclSurface.md` L30 and L32 are corrected (`47d3568`), and the mirror is re-copied.
- OpenNotes N3, N7 and the new N10 agree.
- The `crdt` refusal no longer claims glial needs the profile.
- The page (L146-156) names the path that actually works: glial's `MountConfig.crdtProfile`, which "the application mounting the surface sets … to `text_crdt`, or glial refuses the mount".
- The stale schema comment is recorded in N10. That is a contract path, outside my round-1 locations; accepted.

**SUR-P3-4 evidence.**
- The deferral reasoning holds on the surface:
  - grants are inert until enforcement;
  - the revocation route lands in the same step that makes grants live;
  - fixing the seed lines earlier would append grants that could not be withdrawn.
- The page's `seed` text is unchanged (L165-171, L344-345), and still accurate.

**SUR-P3-5 evidence.**
- L333-336: "`glade-app v1` gives `service` and `workspace` lines no retraction. That was decided, not missed … whether a later format adds one is an open question". This is my round-1 remedy's "none in v1, with the reason" option.
- L340-343 states what happens to the node's claim: "from its next start the node no longer serves the share … The registered entry … stays".

**SUR-P3-6 evidence.**
- Same reasoning as SUR-P3-4.
- Residual, not blocking, text only, no effect on any store:
  - The owner's ruling can be stated on the page now.
  - The page still calls `grazel-app.glade` "a longer example" (L45-47). That file's `seed owner grazel read.*` and `seed owner grazel gwz.*` (L48-49) are what the ruling makes wrong.
  - One sentence on the page, or a comment in both copies (comments register nothing), would stop new authors copying the wrong convention before Step 4.3.

**SUR-P3-7 evidence.**
- The page (L108) now says "`external` is accepted with a warning: the file cannot name the source yet, so the binding registers and nothing acts on it". It also says the warning appears "under either header".
- N11 records what such lines become once a source token exists — refused, or kept by loosening the "iff" — tied to that token's design.
- The warning is emitted, line-numbered, through `` line {n}: {EXTERNAL_WARNING} ``. Its wording is not visible through the grep.
- Nit, not a finding: the contract README's `external` row (L109) still says only "parses … declared, not yet authorable" and does not mention the warning.

**SUR-P3-8 evidence.**
- Each rendering README gains a "Contract v1" block (rs L13-31, ts L14-31, py L13-30) with:
  - the package or crate name and how to depend on it (path dependency, `file:../glade-decl-ts`, `pip install ../glade-decl-py`);
  - the pin to `7d18cd3`;
  - "`AdvertisementRecord` is removed", with R7(b)'s reason;
  - the members declared but not yet authorable;
  - a pointer to the contract README.
- `glade/README.md` L17-19 has a current status line and links `docs/AppFileFormat.md`. The stale libp2p "Current Focus" block is replaced.
- `grazel/README.md` L81-87 links the format page. It also says the node's stderr is forwarded "line by line as it arrives, with a `[node] ` prefix", and that the exit tail is the last 20 lines. So the one release of warnings now reaches grazel operators.

## 2. New findings

### [SUR-P3-9] The new retirement procedure promises to withdraw "every surface an app declared", but services, the workspace entry and seed grants stay

**Location**
- `glade/docs/AppFileFormat.md` L320-324:
  - "To withdraw every surface an app declared, start the node once with a file that names the app and has no `binding` lines: each of the app's declarations is retracted".
  - "Retiring the old name this way is how to rename an app without leaving its declarations live".
- Contradicted by the same page:
  - L76-77: "Every `binding` and `service` record the file registers carries this name";
  - L160-161: an exchange is "a directed request/response surface";
  - L333-345: service and workspace lines get no retraction; a seed's grant stays until revoked.

**Invariant.** A documented procedure states its whole effect.

**Reproduction** (from the page alone):
1. Take the page's own example app `notes`, which has `service notes notes.ops`, `seed owner ws-notes …` and `workspace ws-notes notes`.
2. Retire it as L320-322 says: start once with `glade-app v1` / `app notes` and no binding lines.
3. By L338-345, the result is:
   - `notes.ops` stays declared and routable;
   - the `WorkspaceEntry` stays;
   - the grant stays.
   
   The bullet promised that every surface and "each of the app's declarations" would be withdrawn. A rename done this way likewise leaves the old name's service declarations live.

**Impact.** An author who retires or renames an app that serves an exchange believes the exchange is gone. It stays declared and routable, to a node that may no longer have a provider for it.

**Remedy.** Scope the bullet to what it does: "each of the app's `binding` declarations is retracted; its `service` lines, its workspace entry and its seed grants are not (see Other lines)".

**Closure test.**
- A check that the page's retirement text names what stays.
- Optionally, the existing retirement test also asserts that the retired app's service record is still there, which pins the documented behaviour.

**Classification:** not ARCHITECTURAL (a text fix).

### [SUR-P3-10] A missing `--app` file is reported without its path

This is older behaviour that was not changed, disclosed by the lane owner. The patch's preflight is now the code path that reads the files.

**Location.** The node's start-up file read (lane-owner disclosure 2; its text is not visible through the grep), against `AppFileFormat.md` L42-47 and L86-87.

**Invariant.** A start-up failure caused by an app file names that file.

**Reproduction.**
- Run `glade-node --profile local --app pantry.glade --app pantyr-extra.glade`, with the second path mistyped. The node exits with an I/O error that names no path.
- The same happens under grazel: run it from another directory with the gyld leg on, so `--gyld-app apps/gyld-app.glade` resolves against the wrong working directory. grazel prints the tail, and the tail does not say which file.

**Impact.**
- The page now tells authors the flag repeats "once per app", so several `--app` files are the normal case. A mistyped path is among the likeliest first-day mistakes, and the author has to guess which one is wrong.
- The page's "the message names the file" (L86-87) covers rule breaks only.

**Remedy.** Prefix the I/O error with the path, e.g. `<path>: No such file or directory`, as the refusals already do.

**Closure test.** A binary test with a nonexistent `--app` path asserts that stderr contains the path.

**Classification:** not ARCHITECTURAL.

### [SUR-P3-11] Every refusal reaches the author as a Rust `Debug` dump

This is older behaviour that was not changed, disclosed by the lane owner.

**Location.** The node's top-level error print (disclosure 2): `Error: Custom { kind: InvalidData, error: "…" }`, against `AppFileFormat.md` L86-90. The page shows the warning's form (`<file>: warning: line N: …`) but never the refusal's.

**Invariant.** The page's description of what the node prints matches what the author sees.

**Reproduction.**
1. A v1 file line `binding x value shared commons latest` is refused with `` unknown authority `shared` (one of ["share", "external"]) ``.
2. The author actually sees something like `Error: Custom { kind: InvalidData, error: "<file>: line N: unknown authority \`shared\` (one of [\"share\", \"external\"])" }` (inner text per the page).
   - The message's own quotes are escaped.
   - On Windows, the two paths in the new two-files refusal would show doubled backslashes.
3. Under grazel the same dump appears after `[node] `.

**Impact.**
- A newcomer reads a refusal of their file as an internal crash.
- Every refusal the amendment added — the tail errors, the header error, the two-files refusal — arrives with noise around it.

**Remedy.** Either:
- print refusals with `Display` and a non-zero exit, or
- show the refusal's shape on the page.

**Closure test.** A binary test asserts that a refused file's stderr line starts with `<file>: line N:` and contains no `Custom {`.

**Classification:** not ARCHITECTURAL.

## 3. Walkthrough redo and attacks

### The `pantry` app, from the revised page alone

```text
glade-app v1
app pantry
binding pantry.settings value share commons latest
binding pantry.events   log   share commons from-cursor
binding pantry.doc      crdt  share commons from-cursor shape-profile=text_crdt
binding pantry.cache    value share commons ttl ttl=15m
seed owner ws-pantry read.*,pantry.*      # workspace share, as in the page's example (owner's ruling not yet on the page)
workspace ws-pantry pantry
```

- Start it with `glade-node --profile local --app pantry.glade`.
- The `crdt` document now has a next step: the application mounting `pantry.doc` sets glial's `MountConfig.crdtProfile` to `text_crdt` (L150-156). In round 1 there was none.

### Mistakes

| Mistake | What the node says | Page agrees? |
| --- | --- | --- |
| Token missing, no tail | Refused with the template (inside the `Debug` dump, SUR-P3-11) | Yes, L200-205 |
| Token missing, with tail, v1 file | Refused: `` `ttl=15m` is a key=value entry where <retention> goes (the tail follows all five tokens) `` | Yes, L130-135 |
| The same, in a v0 file | Stored as written, with a warning `` … where <retention> goes; … `` (the tail of that warning is not visible) | Yes, L136-138 |
| `windowed` (v1) | Warning `` unknown retention `windowed` (removed; use `from-cursor`); a later node release refuses the line `` | Yes |
| `from_cursor` or an unknown zone (v1) | Warning naming the fix, with the same suffix; stored as written | Yes |
| `crdt` without a profile | Refused: `` shape `crdt` needs `shape-profile=text_crdt` (a `crdt` line must name its profile) `` | Yes; the reason no longer contradicts L146-156 |
| Bad duration; `ttl=` on a `latest` line | Unchanged since round 1 | Yes |
| `glade-app v0` header | Unchanged warning. The page now says v0 has no scheduled end and uses one grammar. | Yes |
| No header | `` line 1: expected a header, got `app pantry`: write `glade-app v1` (`glade-app v0` is accepted for old files) `` | Yes |
| `external` authority | Line-numbered warning (wording not visible) | Yes, L108 |
| Two files naming `pantry`, or the same file twice (disclosure 1) | Refused before any write, naming both files (wording not visible) | Yes, L42-47 and L291-294 ("once per app") |
| Mistyped `--app` path | I/O error without the path | Page is silent — SUR-P3-10 |

### Edits

- **Changing a binding line:** stated. A change to the tail only still registers nothing new, which the reader has to work out from L146-148 (a nit carried over from round 1).
- **Deleting a binding line:** stated.
- **Renaming the `app` line:** stated, L315-319.
- **Retiring an app:** stated, but it overclaims — SUR-P3-9.
- **Deleting a `service` line:** stated; it is not retracted, by decision.
- **Deleting a `workspace` line:** stated; the node stops claiming the share.
- **Deleting a `seed` line:** stated; the grant stays, and the revocation route is deferred to Step 4.3.
- **Dropping a file from `--app`:** stated.

### Guesses that remain

- The operands of `service <name>` and of `seed` (deferred, SUR-P3-6).
- Whether `--app` needs `--profile`.
- What paths are relative to.
- That a display name must be one token.
- "The plain engine" on the page versus "the bare engine" in N7.
- Where `MountConfig` is documented: the page names it but links nothing.
- How to learn the node's version, which the page cites as `0.0.0`.

### Attacks on the new texts that failed

- **"One grammar" for both headers (L69-75).** The same sentence qualifies it: the only difference is the slot guard, v1 refuses and v0 keeps with a warning. The attach notes L65-70 agree.
- **Precedence against rename.** L298-310 and L315-319 are consistent: the resurrection follows from "newest still live across apps" plus "never touched".
- **"A refused start leaves its store as it was" (L87-89).** Consistent with "reads every file before it writes anything" and with the attach notes L70-71. I cannot test it.
- **Deleting a workspace line.** L340-343 holds by its "because" clause: the share is still served if another loaded file declares it, as grazel's two files both declare `ws-razel`. That is inferable; a nit, not a finding.
- **Rendering blocks.** All four consumer items are present in all three READMEs.
  - Nit: the Python README's Regenerate block (L55) still copies the IR and corpus, which `build.py` already writes. This is harmless; the TS block dropped that line.
- **Rider CON-P3-11**, now identifiable from the RemPlan: (s) is the retraction option for `service` and `workspace` lines that R9 declined. The page states it is not implemented and that the question is open (L333-336). Closed from the author's side.

## 4. Risks and next action

**Risks**
- **The page's version sentence will go stale.** At the first release, "glade-node has had no release yet (its version is `0.0.0`)" (L215, L278) becomes false. The RemPlan's mechanical check pins the constant, not the page, and the warning's "a later node release" never names the release. When the first release is cut, update the page, or have that check cover it.
- **gryth-wz/glade-decl-ts lags.** It is still at `37d9b7d`. It is behind `85ec18d` by a README-only step, so gryth-wz readers do not see the "Contract v1" block until it is fast-forwarded.
- **Three texts are not verifiable on this axis:** the two-files refusal, `EXTERNAL_WARNING`, and the tail of the v0 misplaced-entry warning. The grep cannot show them. Their wording rests on the page, N11 and the RemPlan's tests.

**Next action:** GO for the publish.
- SUR-P3-9 is a one-sentence page fix and can ride the publish.
- SUR-P3-10 and SUR-P3-11 are older behaviour in the node's start-up error path. Fix them, or record them against that path.
- The interim seed-share sentence (SUR-P3-6 residual) costs nothing and prevents copied mistakes before Step 4.3.
