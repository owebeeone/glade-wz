# Glade Raft first production profile decisions — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeRaftProductionProfileDecisions.md` at root `4702e283e1adac5863c5d542e57a9ceea9454541`; proposal for owner selection, without canonical freeze or activation.  
**Baseline:** Root `4702e283e1adac5863c5d542e57a9ceea9454541`; glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Material sources read using `git show <exact SHA>:<path>`.  
**Date:** 2026-10-03  
**Axis:** Internal consistency and agreement with the controlling contract/design graph. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero findings. Fit to present for owner selection of the semantic proposal only.

---

## 0. Evidence base

Verified all four HEADs using the specified `git rev-parse` commands at review start and end. Every result matched the exact baseline above; the tuple remained unchanged.

Read the committed review object in full, lines 1–74, and these controlling sources:

- Root `AGENTS.md` and `AGENTS_GWZ.md`; the local review-loop skill, with QualificationPlan §6 supplying its local binding.
- `GladeRaftProductionIntegrationPlan.md`, lines 1–100.
- `GladeRaftQualificationPlan.md`, lines 1–197.
- `GladeRaftAdoptionContract.md`, lines 1–97.
- `GladeBuildEntry.md`, lines 1–88.
- `LibraryBoundaryAndTestingPolicy.md` and `GladePackageArchitecture.md`, including library roles, behavioral RED, affected-consumer obligations and explicit gate adoption.
- `glade/GladeAuthzModel.md`, particularly §§3a, 3b, 4, 4a, 7a, 7b and the ratified rules reproduced in §§9 and 11.
- `GladeRaftQ4-BoundaryAudit.md`, lines 1–161, and `GladeRaftQ4-CarrierAudit.md`, lines 1–155. Treated these as factual audits with their stated source limitations, not runtime acceptance evidence.

Checked related canonical reconciliation locations in the committed BuyBuild matrix, WorkspaceDirectory and DiscoveryModel. Read pinned Glade SubstrateV1 receipt clauses and CrossNodeWritesPlan’s routing/authentication limits. Inspected pinned external Gyld Admission/Profile/Records/Delivery/ApplicationSupplier/NodeAssembly allocations.

The packet’s quotation at lines 48–49 matches QualificationPlan’s Q4 entry at line 47 verbatim. Its BuildEntry summary at lines 50–52 agrees with BuildEntry lines 12–14.

No writes, builds, tests, network requests or Git mutations were performed. No current peer prompt or report was read.

## 2. Invariant analysis

**Selection does not become activation.** The strongest attempted counterexample was an owner selecting the table and treating that as permission to deploy. Lines 55–62 explicitly authorize preparing amendments and consumer contracts, require reviewed allocations, generated Taut bytes, both clients and applicable reviews before implementation, and reserve deployment approval and operational inputs. This agrees with IntegrationPlan Q4-C through Q4-E and QualificationPlan §2. No completion claim bypasses those gates.

**Complete data and durable application remain distinct obligations.** Line 12 requires majority retention of the complete command and ordered application with retained exact terminal outcome. Line 13 requires coherent log/application/outcome reconstruction and actual synchronization. Metadata witnesses cannot satisfy the promise. After one voter is lost, further progress is conditional on a remaining majority meeting the same promise; the text does not authorize degraded one-copy acceptance. The APFS/process-crash qualification remains explicitly narrower than power-loss or malicious-storage guarantees, and private Q2/Q3 storage is not promoted into a production adapter.

**Read freshness does not manufacture authorization freshness.** Lines 16 and 40–44 distinguish permitted cached observations from authoritative reads, require a separately qualified quorum barrier and local application, prohibit lease-based read authority, and preserve local disclosure checks and the unseen-revocation boundary. A barrier cannot claim knowledge of governance outside the declared frontier. This is consistent with RA-006/007 and Authz’s local evaluation model.

**Authentication does not collapse into node possession or consensus authority.** Lines 14–15 preserve certified user-root authentication, signed genesis binding, authenticated private `self` at every hop, issuance ancestry and separate requester/host/voter rights. The intact RA-001–012 incorporation preserves nonconflicting retained genesis, conflicting-mapping quarantine, distinct leadership/home/generation, and authenticated retry identity. Lines 34–38 retain operator-vouched sessions as a wider product direction while requiring user-owned authentication policy and attenuation before their admission. Unsupported caveats are refused rather than erased.

**A narrow fixture does not waive lifecycle or recovery.** Lines 21–24 explicitly retain Create, Mutate, BeginMove, Activate and Retire, plus complete retry/policy/configuration/snapshot history. RA-005 supplies the committed move cut and verified successor application readiness; RA-004 and RA-009 preserve historical exact retry before current-generation rejection, subject to current disclosure permission. Line 17 preserves accepted/refused outcomes and safety evidence through compaction, prohibits age-based eviction, and requires capacity refusal rather than identity recreation. No one-shape exception contradicts those obligations.

**Migration is broader than Store sealing.** Line 19 requires reconciliation of relevant copies, preservation of verified history and pending/unknown work, and closure of legacy writers, old binaries and rollback paths. It explicitly denies seal sufficiency. Together with unchanged RA-012 and IntegrationPlan Q4-E, this covers the separate Registry, startup-root and external paths identified by the boundary audit. External effects remain outside acceptance pending sink qualification.

**The canonical graph remains controlling.** The proposal does not pretend its brief table supersedes canonical clauses. Lines 55–59 require the next object to enumerate superseded clauses and revise Gyld allocation before implementation. This preserves AdoptionContract §3’s scoped reconciliation obligations and §4’s Records-versus-source/effect boundary. The two-node development launcher and dynamic-resource/authenticated-discovery objectives remain intact.

No user-facing command, option, editable format or API is frozen here. Surface review remains a mandatory later gate where applicable.

## 3. Risks and next action

The proposal supplies no production adapter, cryptographic consumer proof, independent-host fault evidence, carrier selection or migration closure. Those are correctly identified as subsequent mandatory work, not evidence established by this review.

The next action is to present this semantic packet for owner selection. Any selected profile must then become an exact canonical amendment and consumer-contract object under the gates at lines 55–62. This GO does not accept production integration or activation.