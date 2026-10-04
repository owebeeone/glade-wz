# IC-3A semantic production-integration design, remediation 1 — CONSISTENCY-AXIS REVIEW

**Review object:** SAME-object originating-finding closure review of the DRAFT [production integration design](/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md) and [plan](/Volumes/projects/limbo/glade-wz/dev-docs/GladeIndependentCrdtProductionIntegrationPlan.md), dated 2026-10-04, at workspace root `06b16c9e17ff5507268a5823fad9a0be70a36790`.

**Baseline:**

| Repository | Verified HEAD at start and end |
| --- | --- |
| Workspace root | `06b16c9e17ff5507268a5823fad9a0be70a36790` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld application | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were inspected with read-only `cat`, `sed`, `nl`, `rg`, `shasum`, `git show` and scoped `git diff`. External Gyld is `/Volumes/projects/limbo/gyld-wz/gyld`.

**Date:** 2026-10-04.

**Axis:** Independently retrace this reviewer’s three original counterexamples, check the complete changed range against retained controlling context, and classify any additional root. Independent, adversarial, read-only. Current peer and fresh-full testimony were neither read nor requested. Filed verbatim by the lane owner.

**Verdict: GO** — all three originating P2 findings are closed at the semantic level. No additional P0–P3 finding is established. This is originating closure testimony, not aggregate semantic acceptance or typed, disk, cryptographic or network qualification.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| Consistency P2-1 — missing durable predecessor for failed observation/loss persistence | IC3-GUARD-001–003 adds authoritative pre-consumption floor guard, owned permit, coupled settlement, qualified abandonment and conservative reopen | Replayed complete image → observed history → observation and loss writes both fail → SIGKILL → writable storage/peer absent. The permitted history read now requires an already durable guard. Reopen merges that guard before exposing a cut; old-slot fallback cannot discard it. Failed establishment permits no consumption. | **CLOSED, semantic. Architectural root retained in cumulative count.** |
| Consistency P2-2 — declaration/schema absent from genuine authenticated identity | IC3-ID-001 adds complete accepted-tuple mapping, signed declaration/schema identities and trusted production IdentityBinding | Replayed equal root/names/key/profile/bounds with different declaration hash/version or schema identity/hash/version. These components now change the signed namespace or fail trusted provision/open/proof/exchange equality. The negative witnesses explicitly hold names and key bytes equal. | **CLOSED, semantic. Architectural root retained in cumulative count.** |
| Consistency P2-3 — remote completion possible without Rust/TS/Python vectors | IC3-CANON-001 adds independent pinned three-language vectors and mandatory pre-remote prerequisite | Replayed A2/B1/C with only a Rust codec and two agreeing Rust nodes. Missing language/category/pin/assertion now fails the prerequisite, and first C remote use remains blocked. The released text corpus cannot substitute for the new proof/transfer vectors. | **CLOSED, semantic. Nonarchitectural root retained in cumulative count.** |

These closures are independent retraces of the original findings, not adoption of writer completion statements. Their required executable regressions remain obligations of the typed and physical gates.

## Changed-range analysis

The complete range inspected was `4d641dd179e5b0bd94e84df9cb334d7a21dae371..06b16c9e17ff5507268a5823fad9a0be70a36790`.

It contains changes to the design, plan and ledger; additions of initial prompts/reports and RemPlan-1; and two workspace commit markers. No implementation, manifest, retained test, archived controlling specification or member source is changed. Historical initial reports are legitimate prior-round inputs; current testimony remained excluded.

The material changes are the durable receive boundary and complete authenticated namespace. They address the two already counted architectural roots. The vector requirement restores an existing compatibility prerequisite and remains nonarchitectural. Surface wording now explicitly covers actual operator-facing configuration/API/protocol shape even when qualification is called private; it does not authorize activation.

Context retention is valid because the four source repositories are unchanged, the complete original object was inspected during the originating review, and the corrected design and plan were reread in full. The new grammar was checked through its allocation, persistence, exchange, reconstruction, witness matrix and delivery conditions, rather than only through the inserted clauses.

The replacement table now acknowledges both the ingress lifecycle amendment and identity/vector restoration. Its former claim that the combined cut was the only new semantic output has been corrected. The plan likewise carries the new obligations through A2, B2, C2 and aggregate review. No independent new architectural or nonarchitectural root is established. The ledger therefore retains **two unique architectural roots, one nonarchitectural root and one completed merged remediation round**. No cap reset or third-root STOP is justified by this report.

## 0. Evidence base

The complete originating-closure prompt, original Consistency report, prior completed Safety report, RemPlan-1, corrected design, corrected plan and current ledger were inspected. Prior Safety testimony was used only as historical scope/count context; it did not determine these closures.

Verified document hashes:

| Document | Extent | SHA256 |
| --- | --- | --- |
| Corrected design | Lines 1–659 | `c476dd10dab4618710ba02620ea71a75f20b11570c373bb685ea13320b3909f2` |
| Corrected plan | Lines 1–232 | `81566df8b4eec445fe3fd1fd163af4534f8240c0a823cc348ae577510ee62f62` |

Below, **D** denotes the design and **P** the plan. Principal closure locations are:

- D:108–160: creation intent, complete identity mapping and trusted IdentityBinding.
- D:162–188: independent canonical consumers and mandatory remote-use gate.
- D:261–336: complete images, authoritative floor guards and interrupted selection.
- D:369–481: establishment, consumption, settlement, abandonment and crash table.
- D:518–578: guard placement in actual exchange and combined completeness.
- D:584–659: exact replacement scope, requirement witnesses and remaining gates.
- P:A2, B2, C2 and C3: typed RED, real persistence failures and pre-remote qualification.

The originating review’s unchanged source evidence remains applicable: root/member/external instructions; review-loop skill/template; BuildEntry and linked problem capture; boundary/package policies; AdmissionPlan, profiles and GDL-054–058; accepted storage contract §§2–7; archived admission/storage/internal contract and IC-2 implementation records; actual admission core and storage API; node signing, RecordsFile, sysdir, assembly, lifecycle, accept/server and carrier seams; canonical Substrate/signing/CRDT adapter requirements; external Gyld allocation; and released Glial/Taut consumers and corpus controls.

The accepted identity table at archived admission design:82–97 was retraced against the corrected table. Storage contract §7’s conservative restart and sticky-incompleteness obligations were retraced against the new guard and combined-cut rules.

All five HEADs matched at both verification boundaries. No writes, builds, tests, network operations, live actions or Git mutations were performed. Crash sequences below are specification retraces, not claimed physical executions.

## 2. Invariant analysis

**The all-writes-failed restart counterexample now has a durable discriminator.** D:371–402 places the guard before application receive, partial decode, first inventory header and overlimit body handling. Establishment requires durable floor acknowledgment; unknown establishment returns no usable permit. Consequently, the original execution cannot reach its history-observation step while retaining only the old guard-free floor.

After successful establishment, observation and loss writes may still both fail. D:432–441 deliberately leaves the guard active. Reopen validates the authoritative floor before serving the selected image, and active uncertainty suppresses complete reads. This works with the peer absent and does not depend on sender retry.

The paired no-input execution is also treated honestly. Killing after establishment but before receiving leaves uncertainty because restart cannot prove the gate never started. Before failed establishment, both pair members are prohibited from consuming history. Conservative refusal therefore follows persisted evidence rather than an invented fact about missing bytes.

**Interrupted settlement cannot discharge evidence it failed to retain.** D:303–336 carries active guards through every floor intent/replacement and explicitly preserves them on old-image fallback. A proposed retirement without its matching valid image is not authoritative. D:421–430 couples exact observation or qualified loss, inbox/classification, core/attempt history and guard settlement in the selected image. Successful retirement concerns only that guard.

This also closes the original intent-written/next-slot-missing variation. Advanced counters alone do not erase observation uncertainty, and recovery does not fabricate NonCommit for existing plans.

**Cancellation has a complete ownership pair.** `begin_ingress`, `settle_ingress` and `abandon_ingress` are explicit responsibilities, with an owned noncloneable permit and joined callbacks. D:443–456 permits live abandonment only with Records-owned evidence that receive/parse never ran and cannot run later. Timeout, EOF after a started read, Drop, absent inbox and caller assertions are excluded. Restart cannot recreate the live proof. Normal exact empty/nonempty rounds have positive settlement controls, so the grammar does not require every connection to remain permanently dirty.

**The complete accepted identity tuple is now represented.** D:118–160 binds authoritative declaration bytes/hash/version and canonical schema identity/hash/version, validates parameters under that schema and derives the exact canonical key by a versioned rule. Equal resulting key bytes cannot erase a schema difference.

The mapping also accounts for resource/genesis ancestry, incarnation/share/glade ID/zone/key, engine/payload/corpus, admission version, authorization roots/strength/permit/placement semantics, durability/bounds/retention/rejoin/base/migration generation. These are authenticated fields or explicitly fixed versioned-profile choices. D:142–149 propagates the namespace through provisioning, open, proofs, requests, images, sessions and HELLO.

The old Pure descriptor and Op bytes are preserved as internal controls. The production wrapper must obtain and validate the fuller binding from trusted provision/open input; verifying signatures over attacker-supplied bytes alone is expressly insufficient. This resolves the original omission without silently rewriting archived kernel semantics.

**Cross-language compatibility precedes actual remote use.** D:162–188 and P:A2/C2 require each pinned Rust, TS and Python consumer to derive bytes and hashes independently from semantic inputs. Copying expected bytes or delegating to another encoder is excluded. Scope covers the new namespace, signed proofs, session/admission and transfer values, with frozen inner Op controls and malformed/noncanonical negatives.

Missing consumers or assertion categories fail the prerequisite. A successful two-Rust-node witness cannot waive it. Browser deferral and Rust-only physical images do not exempt remotely transmitted representations. This closes the precise completion-checklist escape identified in P2-3.

**Retained storage and admission semantics remain intact.** The correction does not change four-kind custody/terminal coupling, original receipt strength, complete request authentication, terminal winner rules or independent protected-start observation. Pending/absence/cancellation remain insufficient for NonCommit. Same-logical-owner process reattachment remains distinct from owner replacement. No new event clears kernel `recovery_incomplete`.

Combined completion still requires the actual kernel cut plus all retained adapter obligations, now explicitly including active/uncertain guards. Exact later inventory can settle its own retained work but cannot clear permanent loss or unrelated earlier uncertainty. Completeness remains local to the captured generation/watermark, not global enumeration.

**Allocation and production witnesses remain coherent.** Records owns durable guard metadata and lifecycle; the node owns receive execution and callback drainage; Pure retains admission/classification semantics; the codec remains structural data handling. The specified call graph continues through actual NodeStart/NodeAssembly/runtime/core/provider callbacks and real Iroh duplex, with holder-path and fixture substitutions rejected by consumer witnesses.

Per-instance floors, workers, guards and capacity preserve functional Y while X is held. Full-history exhaustion refuses additional work instead of dropping proof. Existing tests, selectors, disabled-platform checks, dependency classifications, budgets and process-global allowlists remain protected.

## 3. Risks and next action

This semantic closure does not establish a working permit type, durable file operation, authoritative clock/signature provider, independent language encoder or real-node exchange.

The next gates must demonstrate that cached images and unrelated checkpoint/lease/append transitions preserve authoritative active guards and their base bindings; that no receive callback escapes the owned gate; and that all enumerated filesystem cuts retain exact custody and uncertainty. They must also prove the expanded namespace at actual producers/consumers while preserving the original core and text controls.

The single next action is to merge this originating closure testimony with the separately required same-tuple originating Safety closure and fresh full Consistency/Safety reviews. Proceed to A2 only when the combined semantic gate is accepted. **This reviewer closes Consistency P2-1, P2-2 and P2-3; aggregate acceptance remains unclaimed.**