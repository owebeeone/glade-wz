# Combined IC-3B genuine authentication and physical persistence — CODE-AXIS REVIEW

**Review object:** Fresh full acceptance review of the combined IC-3B component, Glade range `c69e6416f5f5155d4bb570bf272e796b2deae0a3` through `a47691598df648eb8c9554b27f3d06b0cffcf596`, after the second bounded B remediation. Controlling document: `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md`, DRAFT, at workspace root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`.

**Baseline:** Source was inspected through read-only working-tree reads, scoped diffs, pinned `git show`, and hash comparisons against recorded source maps. All five HEADs were checked at START and END:

| Repository | START | END |
|---|---|---|
| Workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` | `400cedcf1fff74128f366af758a0c389a23435d7` |

**Date:** 2026-10-04

**Axis:** Code — architecture, interfaces, call graphs, ownership, compatibility, and error paths. Independent, adversarial, read-only. Nothing here relies on the parallel axis or current focused/originating reports. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2, or P3 findings established. This is fresh full Code acceptance of the bounded combined B component. It does not supply another reviewer’s originating closure or qualify deferred IC-3C/IC4 outcomes.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
|---|---|---|---|
| None owned by this reviewer | Fresh full review requested because the persisted-cut helper changes the reviewed callback/publication call graph | Merged RemPlan-1 and RemPlan-2 counterexamples were used as attack inputs; corrected paths were independently retraced | No originating closure claimed |

## Changed-range analysis

I inspected the full combined component rather than treating the latest correction as sufficient context. I also examined the original reviewed `9dbc677a25d93af0c561817571ad7eb1990ddadc` through current-HEAD remediation range and the second correction’s changes from `d3fded` through current HEAD.

The important latest change is behavioral: `persist_observed` selects the complete observation and its original issued Fence request, then drains that Fence through native storage resolution and the real core callback before returning the retained cut. The caller therefore can encounter a terminal attempt and retired original request before its delayed Begin callback resumes. Reviewing only the helper’s return type would miss that ordering. I followed selection, native resolution, core callback, retired-request authentication, and restart reattachment together.

The correction implements the existing exact-cut and immutable-terminal contracts. It does not introduce a new authority model, storage protocol, public interface, dependency allowance, or package role. The merged classification of stale-cut B-R6 as nonarchitectural is supported by the code: the defect was reuse of an earlier observation at an already-required protected boundary. The oversized-loss correction remains the existing B-R4 nonarchitectural root. Documentation/evidence corrections remain nonarchitectural.

No additional root was established. Accounting remains semantic **2 architectural / 1 nonarchitectural / 1 completed remediation**, typed **2 architectural / 5 prior nonarchitectural / 2 completed remediations**, and B **0 architectural / 6 nonarchitectural / 2 completed remediations**. This review neither resets those counts nor invokes the confined third-round exception. A third new typed architectural root would still require STOP/owner disposition.

## 0. Evidence base

No tests, builds, network operations, live actions, writes, or Git mutations were performed. Results below are audited historical evidence, not executions by this reviewer.

| Evidence | Inspection and result |
|---|---|
| Process and architecture authority | Root/member instructions; review-loop skill and canonical prompt template; BuildEntry and relevant Gyld capture/declaration; library-boundary policy; package architecture; committed review-cycle accounting and merged RemPlan-1/2 |
| Controlling component contracts | ProductionIntegrationDesign, Plan, TypedContract, TypedUsage, Authentication, Persistence, PersistenceUsage; admission/storage-attempt/resource-profile contracts and relevant linked design material |
| Complete relevant production implementation | `node/src/independent` authentication, namespace/text profile, paired physical store, floors/slots/metadata validation, records custody, genesis, bindings, guards, ingress observations, reserves, persistence, loss, storage, runtime, and recovery |
| Retained-cut/callback path | `records/authentication.rs:5–102`; `records/storage.rs:164–331`; `records/runtime.rs:5–280`; core `callbacks.rs:28–243` and lifecycle issuance/recovery; recovery-api ingress ownership |
| Regression witnesses | Public remediation-2 tests; retained-cut crash, oversized-loss, original-Fence, native-kind and native-cut tests; physical paired-store tests and actual legacy-executable witness |
| Source maps | B1’s 14 entries, B2’s 56, Remediation1’s 62, and Remediation2’s 65 matched their recorded source revisions; current remediation-2 entries matched the frozen tree. Its five consumer-document pins also matched |
| Compatibility preservation | Recorded 45 compatibility entries, 27 codec entries, and 23 preserved artifacts matched hashes. The retained legacy decoder fixture matched the pinned production decoder prefix. The preserved old executable existed and matched its recorded hash |
| Recorded run logs | All four compressed logs were read and their chronology audited. Remediation2 gzip SHA-256: `28c07bef73a2b83848418998b2218a8017fea5037887d575ca08890779eb1c03`; decompressed SHA-256: `4d95a09b37610a7e1908e0179c5eb3dd4f27d2d6c69881a44524c0e0c1d61476` |

The second correction’s log records four compiled failing regressions before the implementation correction: revocation and expiry during the later persistence consultation, a final full interval exceeding the ingress deadline, and drained M+1 loss settlement. Intermediate compiler failures were distinguished from those compiled RED results.

The later crash regression initially exposed incomplete Fence invocation accounting on reopen. Its corrected run recorded successful recovery across three actual child-death cuts: before observation intent, after intent synchronization, and after observation selection synchronization. These are distinct from in-memory structural fixtures.

Final recorded evidence includes:

- Public disk selection: 29 passing tests, with helper tests explicitly ignored in the ordinary selection.
- Records selection: 29 passing tests, including controls for 151 selected actual physical cuts: 57 native, 65 ingress, 26 observation, and three new retained-cut/Fence cuts.
- Genuine authentication, boundary and assembly selections; contract tests; architecture/source checks; process-global check with the unchanged three permanent entries and zero debt.
- Scoped formatting and the established broad-formatting ratchet; Clippy with the same nine recorded legacy warnings.
- Three-language candidate vectors: 21 positive, 18 negative, and six mutants.
- Strict pre-remote qualification’s expected failure because genuine B1 acceptance metadata remains absent.

Earlier broader fixed-cut witnesses remain separately pinned historical evidence; I did not count them as newly rerun remediation-2 tests. The actual old-executable refusal witness was separately invoked with its explicit ignored-test selection; an earlier zero-test filter is not credited as evidence.

## 2. Invariant analysis

**Current authority is the exact retained full cut.** I attacked provider drift between the earlier `observe()` result and the persistence consultation. `persist_observed` returns the validated `TrustedCut` and `PolicyBody` only after the physical barrier. Its unchanged-image fast path compares the complete cached policy and interval, session custody, and issuance observations before clearing the observation marker.

Protected Begin compares the returned policy digest and full interval with the sealed prestart cut. The earlier allowed observation cannot authorize a changed later policy or interval. The invalidating case resolves the original attempt to NonCommit; it does not produce a speculative refusal that permits a fresh attempt to bypass the retained decision. Previously Started or Terminal attempts bypass fresh prestart authorization and preserve their original historical outcome.

Ingress uses the final returned interval for deadline and window checks. If persistence changes the selected image, the guard path reloads and compares the new base before issuing authority. This defeats the witnessed narrow-earlier/wide-later deadline sequence.

**The original Fence precedes its callback.** Before observation publication, the selected image includes the full original Fence request, live-plus-retired invocation count, invocation high-water mark, and effect issuance maximum. Complete serialization and reserves are checked before physical intent. Consequently, a death after observation selection leaves replayable request custody rather than an unaccounted effect.

The helper drains only after selection. Storage authenticates the complete issued request against live or retired custody. Core callbacks independently authenticate owner, invocation namespace, operation, binding, and terminal payload. The delayed original Begin can therefore finish after the Fence without reminting authority or changing the sole terminal. Retired-request lookup does not authorize an altered request.

The runtime saves each core transition before running its resulting effects. Its queue has checked finite iteration accounting. On the corrected Reserved→NonCommit path, the Fence callback does not trigger another current consultation or recursive acceptance cycle. Immutable terminal installation and contrary-terminal detection remain in the real core callback path. No required authority or callback was found discarded by the new helper.

**Partial publication remains recoverable or conservatively unavailable.** I traced failures around intent, inactive-image synchronization, and selected-floor publication. The old selected slot is not overwritten. Intent retains issuance maxima and old Active guards, including when the proposed image retires them. Unknown poisons the active ownership path; retry cannot pretend uncertain I/O never happened.

Recovery validates the selected source before exposing a kernel, rebases retained maxima, and reattaches exact stored requests through native resolution and core callbacks. Protected incomplete observations cannot silently fall back to stale authority. Missing materialization remains incomplete rather than being reconstructed from an empty peer inventory or digest alone.

**All four native kinds retain immutable custody.** Prepare deduplicates the exact plan before fresh allocation. Full binding, store/instance identity, revision, charges, receipts, and request history remain coupled. AcceptedBatch, Candidate, SecurityEvidence, and QualifiedFork use the same physical attempt machinery without collapsing their semantic meaning.

Delayed callbacks, exact retries, and reopen preserve the original terminal and historical receipt. A contrary terminal records integrity failure. Independent X/Y roots remain separate authorities; publication at one is not evidence of publication at the other. The genuine native-kind tests supplement the physical death witnesses; structural support tests are not promoted to durability proof.

**Owned bounded ingress distinguishes receipt from drainage.** The permit’s private issuer and shared status bind the capability to its owning session. Normal receive evidence is issued only for a bounded successful result and contains exact length/digest custody. Foreign ownership is rejected before invoking the receiver factory. Factory start does not imply drain completion, and dropping Pending does not create drainage authority.

Normal settlement checks the exact received payload. Substitution cannot turn the payload into empty or unrelated input. Direct caller-prefix settlement retains its ordinary bound.

For a host-owned drained M+1 capture, loss settlement uses the existing compact permanent-loss representation. It does not encode another oversized observation. The original boxed pending observation/checkpoint remains owned until committed retirement; refused or Unknown settlement restores that same custody for exact retry. Active guard intent remains protected across actual death. Reopen cannot synthesize the lost in-memory drained capability, and closed-session settlement cannot reopen an ordinary acceptance path.

**Public shape and compatibility claims remain bounded.** The assembly’s explicit configured independent profile is the successful entry point; default refusal behavior and existing binary routes are preserved. No new external dependency version or broad package extraction was introduced. Existing library roles and process-global allowances were not relaxed to obtain passing checks.

The private floor/image protocol is closed and validated. Fresh-root compatibility-marker behavior and the actual retained old executable’s refusal are supported by distinct evidence. This does not claim comprehensive decoder equivalence or cover the owner-excluded BOM case.

New and changed conditional platform sections use explicit enclosing boundaries. Existing broad formatting and source debt are recorded rather than presented as a completed repository-wide migration.

## 3. Risks and next action

The physical qualification remains bounded to the tested Unix restart profile and trusted floor/root ownership assumptions. Unsupported platforms refuse the profile. Unmaterialized intent, orphaned Active guards, lost drained custody, and unresolved outcomes can conservatively prevent completeness; the code does not convert those conditions into success.

The existing formatting ratchet and unchanged Clippy warnings remain debt. Recorded historical tests cannot substitute for evidence on a later source revision. Automatic duplex exchange, actual production binary routing, live enrollment/migration, browser activation, and aggregate IC-3C remain deferred outcomes.

The next action is to collect the fresh parallel full-review verdict and all four reviewers’ own blocking-counterexample closures at this identical tuple, then apply the committed acceptance process. This Code GO alone does not authorize accepted-review metadata or strict pre-remote qualification success.