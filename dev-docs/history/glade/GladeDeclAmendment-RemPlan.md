# GladeDeclAmendment — Remediation plan, round 1

Date: 2026-09-23. Lane owner's plan. Object: the glade-decl contract v1 amendment, plan
Steps 2.1–2.8 of `dev-docs/GladeFirstSlicePlan.md`, at the tuple the three reports name
(glade-wz root `cd6a498bd2c1`; glade `dddf8b89cad7`, glade-decl `784840acfb05`,
glade-decl-rs `9507c1233c0a`, glade-decl-ts `37d9b7d8a680`, glade-decl-py `433215acc4dc`,
grazel `b604a06970fe`, glade-gyld `c9ef7a66b6e4`, glade-gwz `fc0bb990ef09`, glial
`4c6e8561c174`, grip-core `97ff6c26f12e`, gryth-wz `3c64e7a59064`). Reports filed verbatim
at root `87ce46f`:

- `-ReviewCode.md`: **NO-GO**, 1 P2, 6 P3. Pre-commits to GO on a revision resolving COD-P2-1 as specified.
- `-ReviewState.md`: **NO-GO**, 2 P2, 3 P3. Pre-commits to GO on STA-P2-1 and STA-P2-2.
- `-ReviewSurface.md`: **GO**, 0 P2, 8 P3. SUR-P3-5 is classified ARCHITECTURAL.

**Round accounting.** This is round 1 of 2. One architectural root cause is on record for
this object: SUR-P3-5, a P3. It is the missing retract for `service` and `workspace` lines,
which R9 already declined as option (s). None of the reports calls a P2 architectural.

**Owner, 2026-09-23 ("all recommended"):** the refusal variant for COD-P2-1 = STA-P2-1, State's variant for STA-P2-2 (a `v1` file keeps the refusal), and the deferral of SUR-P3-4 and SUR-P3-6 to Step 4.3 are confirmed.

**Blind convergence.** The reviewers could not read one another's reports.

- **Two `--app` files naming one app.** All three axes found it:
  - COD-P2-1;
  - STA-P2-1, reproduced: `records.json` grew from 5 to 29 records over five boots with no edit;
  - SUR-P3-1 case 2, predicted from the page alone. The lane owner confirmed it in the code (`glade-node.rs:89-101`, `appdecl.rs:479-504`) before the other two reports arrived.
- **A `v0` file with `=` in its zone or retention is refused at boot.** COD-P3-1 and STA-P2-2. The graders differ, P3 and P2, and the higher grade governs.
- **The TypeScript regeneration's `cp` still writes through pnpm's hard links.** COD-P3-3 and STA-P3-3.
- **Renaming an app's `app` line brings back the old declaration.** SUR-P3-1 case 1 and STA-P3-2.
- **The warn-then-refuse flip has no owner and no trigger.** COD-P3-2 and SUR-P3-2.
- **The contract texts still say a file cannot state a TTL duration.** COD-P3-4 and SUR-P3-3.
- **Multi-origin order and scope.** STA-P3-1 files it as a finding; Code §4 records it as a risk.

Rules:
- ONE patch, meaning one revised tuple, never a series.
- The implementer closes nothing. Each raising reviewer re-checks its own counterexamples on the revised tuple.
- Surface re-verdicts too, because page and message texts change.
- The patch changes no contract path. `CONTRACT_VERSION` stays `7d18cd3`, and the renderings' code bytes do not move.

## Blocking findings

| ID | Disposition | Closure test (verified by the raising reviewer) |
|---|---|---|
| **COD-P2-1 = STA-P2-1** (with SUR-P3-1 case 2) | **ACCEPT, the refusal variant.** New rule of the format: an app is declared by one file.<br><br>**Preflight (L1-14).** The node reads every `--app` file before `boot()` opens the instance and before any `register`. `appdecl::load_all` loads each file, then refuses when two files name one app. The refusal is one line naming the app and both paths. It comes before every durable write, so a refused start writes nothing.<br><br>**The union variant is not taken.** It keeps split apps loading, but a left-out file's surfaces would then be retracted by its sibling, which would be a second new rule. Refusal is the smaller change and fails closed, and no shipped or fixture setup splits an app.<br><br>**Texts:** the format page (the `app` rule, the `--app` paragraph, "Changing or deleting a line"), the attach notes' R9 paragraph, the module doc, and `register`'s doc. | (1) `load_all` over two files naming one app is refused, in both orders; the message names the app and both paths.<br><br>(2) A binary test with `GLADE_HOME` set to a temp dir runs `glade-node --profile local --name t --app a1 --app a2 0`. It exits non-zero, stderr names both files, and nothing is written under the temp `GLADE_HOME`.<br><br>(3) Control: two files naming different apps, registered through a `BlobStore` on three boots, append +N, +0, +0; one file loaded twice appends +0. |
| **STA-P2-2 = COD-P3-1** | **ACCEPT, State's variant.** The slot guard (`tail.rs:54-60`) applies to `glade-app v1` files only.<br><br>**In a `v0` file,** a zone or retention holding `=` is stored as written, as at `559cb2c`. It is warned through the `v0` channel with a line-numbered warning that says a key=value entry belongs in the tail, after all five tokens.<br><br>**In a `v1` file,** today's refusal stands on both sides of `V1_TOKEN_CHECKS_REFUSE`.<br><br>**Code's variant is not taken.** Code proposed that a `v1` file warn until the flip. But the `v1` header arrived in this object, so no `v1` file predates it and no deployed population needs a warning release. And in `v1`'s grammar, a key=value entry in a positional slot is a malformed line, not an unaccepted token.<br><br>**Texts:** the page's refusal statement (`AppFileFormat.md:118-123`) names `v1`, and the tail's doc comment matches. The "never refused" sentences (`:192-196`, `:252-256`, `appdecl.rs:36-39`) then hold as written. | A `v0` file with retention `ttl=10m`, and one with zone `a=b`, each parse Ok, store the token raw, and carry a line-numbered `v0` warning.<br><br>Their `v1` twins are refused with today's message. This is asserted under both values of the constant, the way the existing tests read it.<br><br>`a_tail_entry_where_a_token_goes_is_refused` moves to a `v1` header, and a `v0` twin asserts the warning. |

## Riders accepted on the same patch

- **SUR-P3-1 (cases 1 and 3) and STA-P3-2: the rename orphan and cross-app precedence.**
  - **Texts:** the page and the attach notes state the rule and its consequences.
    - The fold is per `(app, glade_id)`. Per glade id, the newest live declaration across apps stands.
    - Renaming the `app` line leaves the old name's declarations live, so a line deleted later can bring back the old app's declaration.
    - An app is retired by loading, once, a file that names it and has no binding lines.
  - **No new warning** when one id is declared by two apps. The fold allows it by design, and the page states the precedence.
  - **Tests:**
    - the rename-then-delete sequence asserts the documented outcome;
    - the retirement file retracts every declaration of the old app.
- **COD-P3-2 and SUR-P3-2: the flip's lifecycle.**
  - (1) The plan's Step 2.6 Done line takes the code's statement: the warnings ship in the first node release (the first version above `0.0.0`), and the release after it sets the constant to `true`. *(Done in the plan with this document.)*
  - (2) **A mechanical check.**
    - A constant `V1_WARNING_RELEASE: Option<&str>` stays `None` until the first release is cut.
    - A test asserts:
      - at version `0.0.0`, nothing is required;
      - at any other `CARGO_PKG_VERSION`, the constant names a release;
      - once the version is past it, `V1_TOKEN_CHECKS_REFUSE` is `true`.
    - So a version bump that has not decided the flip fails a test.
  - (3) **Warning texts.** Each `v1` zone and retention warning says a later node release refuses the line.
  - (4) **The header error** names `glade-app v1` as the header to write, and names `v0` as accepted for old files.
  - (5) **The page states:**
    - `v0` loading has no scheduled end;
    - what a `v0` file may use of `v1` (the tail, and `crdt`), stated from what the parser actually allows;
    - a bare `ttl` stays legal and names no duration (R11(a); `GladeDeclReconciliation.md:1763-1768`).
- **COD-P3-4 and SUR-P3-3: texts that disagree with what landed.**
  - **The texts to correct:**
    - `glade-decl/README.md:55-56` and `:132`;
    - `glade-decl/dev-docs/OpenNotes.md` N3;
    - the root `dev-docs/glade/GladeDeclSurface.md:30` and `:32`, then re-copy the mirror, which `--check` enforces.
  - **What they say instead:** the node validates the tail keys `ttl=<duration>` and `shape-profile=<profile>` and records neither, so no record carries a duration or a profile yet.
  - **The `crdt` refusal's parenthetical** states what is true: the line must name the profile.
  - **The page says where a `crdt` mount gets its profile today:** glial's `MountConfig.crdtProfile` (`glial/src/binder.ts:98-106`), since no node registers a `ShapeProfileDecl` yet.
  - **The schema comment is not edited.** `glade_decl.taut.py:120-124` is a contract path, and editing a comment there would re-pin three renderings. OpenNotes records the comment as stale since Step 2.5, to be corrected with the next contract change.
- **COD-P3-3 = STA-P3-3, and COD-P3-6: the TypeScript regeneration.**
  - `build.py` gains `--ts-from <dir>`. It copies `<dir>/*.ts` into `../glade-decl-ts/src/` through `write_artifact`.
  - `glade-decl-ts/README.md`'s two `cp` lines become that one command, with one sentence on why (pnpm hard-links installed copies). The IR and corpus line goes, because `build.py` already writes both.
  - `corpus/test_build.py`'s `if __name__ == "__main__":` block moves to the end.
  - **Tests:**
    - `--ts-from` runs against a scratch tree whose `src/` file is hard-linked to an "installed" copy; the installed copy is unchanged and the source's link count is 1;
    - `python3 corpus/test_build.py` and `python3 -m unittest corpus/test_build.py` report the same count.
- **SUR-P3-5 (ARCHITECTURAL, P3): no retract for `service` and `workspace`.**
  - **No mechanism in this patch.** The decision exists: R9 was ruled (b2)+(a), with (s) not taken (`GladeDeclReconciliation.md:994`).
  - **The page says so,** and says what deleting a `workspace` line does:
    - the node stops serving the share from its next start, because it serves the shares its loaded files declare (`glade-node.rs:139`);
    - the registered `WorkspaceEntry` that names it as an eligible host stays.
  - **Recorded as an owner question** in the plan: whether a later format adds (s).
- **SUR-P3-7: `external` loads silently.**
  - `external` gets a warning under both headers, saying the binding names no source yet, registers, and is acted on by nothing.
  - The page states the same. What such a line becomes once a source token exists is recorded as an OpenNote, tied to the source token's design.
- **SUR-P3-8: the package front pages don't mention v1.**
  - **Each rendering README** gains a short "Contract v1" block:
    - the package name and how to depend on it;
    - `AdvertisementRecord` removed (R7(b), held out until GDL-029);
    - the members that are declared but not yet authorable (`source`/`external`, the domain anchor);
    - a link to the contract README.
  - **`glade/README.md`:** a current status line and a link to `docs/AppFileFormat.md`.
  - **`grazel/README.md`:**
    - a link to the format page;
    - a statement that the node's stderr, app-file warnings included, is forwarded to grazel's stderr while the node runs (`grazel/src/main.rs:89-90`, `:121-135`).
- **STA-P3-1: multi-origin order and scope.**
  - **The claims are scoped to one registry:**
    - `registry.rs:472-475` and `:497-498`;
    - the `sysdata.taut.py:100-102` comment (regenerate `sysdata.rs` only if the comment is carried; record bytes are unchanged);
    - the attach notes `:104-113`.
  - **Recorded at the plan's Step 4.6** as an open question: a merged clock (the maximum over the served store's binding family at boot), the origin in the retraction's scope, or both.
  - **Test:** a two-origin fold test pins today's outcome, under a name that does not call it revival.
- **COD-P3-5: BI-002 does not probe `Shape::Atom`.**
  - The list at `glade/contracts/binding-api/src/lib.rs:116` gains every non-`value` member it lacks: `Atom`, `Exchange`, `Swmr`.
  - **Test:** a wrong resolver that accepts `Atom` as `value` fails `bi_002_fail_closed`, as a `should_panic` test.

## Deferred, with the owner

- **SUR-P3-4 and SUR-P3-6: seeds and services.**
  - **The findings:**
    - SUR-P3-4: a `seed` line's grant has no documented route to revocation;
    - SUR-P3-6: nothing says what `service <name>` and a seed's `<share>` refer to, and the shipped `seed owner grazel …` lines disagree with gyld-app's convention.
  - **Why not now.** Grants are recorded, not enforced, so nothing acts on them today. Correcting the shipped seed lines now would append new grants to every store and leave the old ones live, because no revocation route exists. The correction has to land together with the revocation route.
  - **Moved to Step 4.3** (the grant check at the serve hop) as its preconditions:
    - the revocation route, documented beside `seed`;
    - the operand definitions;
    - the shipped files corrected;
    - a lint for a seed whose share no loaded `workspace` declares.
  - **Owner question, recorded there:** is a seed's share the workspace share (gyld-app's convention) or a share named after the app (grazel-app's)?

## Outside the object, recorded

- **Consumers still on contract v0:** glade-chat, grip-react and grip-react-demo (Code §4, State §4). §4.5 does not list them as consumers, and they switch silently at their next install. Recorded for PackageExtractionPlan step 1.6 (consumer parity).
- **CON-P2-2 durability:** `api.ts` has no durable gate, because row 11 was a one-time check (Code §4).
- **The owner's first boot** appends more than 7 if the store holds historical duplicates or lines deleted before the upgrade (Code §4, State §3.1). R9(a) expects this: each such boot converges to the file.

## The patch

- **Repositories:**
  - glade: node code, tests and docs, `README.md`, `contracts/binding-api`;
  - glade-decl: `build.py`, `test_build.py`, `README.md`, OpenNotes, the DeclSurface mirror;
  - the READMEs of glade-decl-ts, -rs and -py, and of grazel;
  - the root `GladeDeclSurface.md` and the plan.
- **Gates:**
  - glade-node: `cargo test -p glade-node`, including the new tests;
  - glade-decl: `build.py --check`, `--compat --deleted AdvertisementRecord,edge/advert`, and both `test_build.py` invocations;
  - glade: the contracts' `check.sh`;
  - glade-decl-ts: `pnpm test`;
  - the grazel, glade-gyld and glade-gwz suites, which run the node and read its messages;
  - the census, which must stay 15/7, with every shipped file loading with no warning.
- **Out of scope for the re-verdict:** Steps 2.10, 3.1 (glade `831eded`) and 3.5, which are committed or in flight in the same repositories.

## Re-verdict

The same three reviewers continue, with their contexts intact. Each receives this plan and the revised tuple, re-checks its own counterexamples, and files `-Review<Axis>-2.md` with a closure table.

## Outcome, 2026-09-24

All three axes re-verdicted **GO** at the revised tuple (root `73c2bf7`; glade `1b9ac3f`, glade-decl `3d10917`, renderings `d3be799`/`85ec18d`/`ee2f960`, grazel `e1a4078`): `-ReviewCode-2.md` (7 of 7 closed; new COD-P3-7), `-ReviewState-2.md` (5 of 5 closed; new STA-P3-4), `-ReviewSurface-2.md` (6 closed, 2 deferred-accepted; new SUR-P3-9, -10, -11). The Surface reviewer re-issued its report after the lane owner's tuple note (`-ReviewSurface-2-reissue.md`, same verdict and findings); both are filed verbatim. The root's move to `690f2f6` during the round was gwz bookkeeping for glade-discover only, and all three axes treated the tuple as held.

**Blind convergence, round 2:** the retirement bullet on the format page withdraws bindings only, not a service's exchange, the workspace entry or seed grants — SUR-P3-9 and STA-P3-4, and Code's §3 item 7 as an observation.

No finding in either round was classified architectural beyond SUR-P3-5. One remediation round of two was used.
