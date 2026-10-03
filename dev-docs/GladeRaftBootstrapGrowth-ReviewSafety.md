# Glade Raft bootstrap and growth — SAFETY-AXIS REVIEW

**Review object:** DRAFT `dev-docs/GladeRaftBootstrapGrowthDesign.md`, with GDL-053 and the production-profile/integration clarification callouts, at workspace `761f691a17d65a5b450bf5e8d502a3121e104bc1`; 2026-10-03.

**Baseline:** Workspace `761f691a17d65a5b450bf5e8d502a3121e104bc1`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Canonical sources were read using `git show <exact SHA>:<path>`.

**Date:** 2026-10-03  
**Axis:** Safety: attack unsafe permission, disclosure, irreversible transitions, degraded operation and reachable stuck states. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This accepts the fitness of the bounded semantic proposal for subsequent selection and contract work. It does not accept a production profile, implemented bootstrap, carrier qualification or activation.

---

## 0. Evidence base

Read the complete canonical Safety prompt and the review-loop skill, plus the supplied AGENTS instructions and local `AGENTS_GWZ.md`.

Inspected the complete restricted four-file diff from `9c0510690050e6edc928c09ecc7b414baba58ba1` to the reviewed workspace revision:

- `GladeRaftBootstrapGrowthDesign.md`, lines 1–299.
- `DecisionLog.md`, GDL-053, lines 73–82.
- `GladeRaftProductionProfileDecisions.md`, especially lines 9–14, 19–35 and 57–73.
- `GladeRaftProductionIntegrationPlan.md`, especially lines 38–82 and 107–116.

Checked controlling requirements in AdoptionContract §§1–4, including RA-001–012; ConfigurationSnapshotContract §§1–5; QualificationPlan §§1–6 and its accepted-through boundaries; AuthzModel §§3a/3b/4/4a/6/7/7a/7b; DiscoveryModel §§0/2/3/4/7; and WorkspaceDirectory §§1–6, 7b and WD-8.

Read BuildEntry, LibraryBoundaryAndTestingPolicy and GladePackageArchitecture. Inspected the relevant architecture revision 3 responsibilities and the pinned external Gyld declaration for Admission, Policy, Directory/DirectoryRules, Records, StorageAdapter, Runtime and NodeAssembly. Checked the pinned Glade substrate’s core model, receipt meanings and W1–W8, and CrossNodeWritesPlan §3, against the proposed protected-profile boundary.

All four HEADs were verified at the beginning and end and matched the exact tuple above. No files were written; no builds, tests, network operations or Git mutations were performed. The counterexamples below are analytical design attacks, not executed qualification evidence. No current peer report was read.

## 2. Invariant analysis

**Disconnected startup cannot create a replacement authority.** I considered two devices starting with the same apparent name, a known device with missing storage, an expired directory claim, and arbitrarily delayed election wakeups. Design lines 65–103 require retained exact genesis and distinguish independent roots from the same canonical identity. Lack of naming authority leaves same-scope requests pending; empty discovery and elapsed time cannot authorize genesis or reset. Conflicting signed mappings quarantine both recognition paths. Discovery therefore cannot merge independently committed histories or retrospectively declare them one group.

**Bootstrap interruption cannot silently change the intended group.** In the sequence “retain genesis → create store → initialize → lose reply,” lines 66–85 require recovery of the original exact intent and outcome. Changing initial voters or group bytes under the same identity is prohibited; exclusive-create failure is not proof of noncommit. Cloned keys and a valid old custody image do not solve uniqueness: lines 73–80 require serialized custody and an independent rollback floor, and block recovery when that evidence is lost. The concrete mechanisms remain mandatory later work, without permission to substitute a weaker semantic outcome.

**Growth does not count a candidate or incomplete learner as a voter.** I considered a metadata-only catch-up, a snapshot from before learner admission, readiness delayed behind another command, and a restarted node reusing another incarnation’s proof. Lines 107–121 require complete durable/applied state and predecessor-cut revalidation; lines 123–128 deny learner voting and independent acceptance. The controlling Q3 contract additionally requires retained deterministic readiness evidence, rejects stale predecessor proofs and preserves actual snapshot eligibility. These constraints block promotion based on caller counters or live local observations that would produce different decisions across replicas.

**Joint growth preserves both authorities and can honestly stop.** For `{A}`→`{A,B}`, the joint requirement is A plus both A and B; B loss cannot leave A issuing joint receipts alone. For `{A,B}`→`{A,B,C}`, A and B remain necessary for the outgoing majority, even if A and C satisfy the incoming majority. Stable three- and five-voter sets require two and three voters respectively. Lines 149–166 distinguish carrier activation from the external complete-data guarantee, so predecessor-only retention of EnterJoint cannot be presented as the new guarantee. Loss of the required quorum may strand a transition; the text promises neither unilateral shrink nor timeout rollback. That is an explicit availability boundary, not an undisclosed recovery escape.

**Home removal remains an application constraint throughout joint authority.** I placed a resource on an outgoing-only voter after entry admission, then queued LeaveJoint. Lines 168–176 require the actual exit predecessor state to be checked. The exit produces a retained refusal without configuration application or version advancement. Qualified movement or retirement must precede a new exit intent; retrying the old refused intent still returns its original refusal. This preserves the Q3 admission-race case and prevents membership change from silently moving a home or generation.

**Receipt promises remain honest across cardinalities and partitions.** An earlier singleton acknowledgment remains a one-domain promise after learners catch up or membership grows (lines 180–207). Two voters require both; the alternative sole-voter-plus-learner deployment explicitly lacks automatic takeover and needs a separately selected receipt profile. A minority or isolated former leader cannot accept new mutations, but may recover a previously committed original outcome subject to local disclosure permission. Lines 209–228 distinguish that historical disclosure from a current-state claim, which requires the qualified barrier. They also preserve the unseen-revocation boundary instead of equating fresh data with fresh authorization.

**Disclosure and governance are not conferred by replication roles.** The attempted shortcut “valid node signature plus discovery plus majority approval permits private transfer” fails under lines 107–108, 127–134 and 264–270, together with AuthzModel’s operator-placement rule. Automatic onboarding requires rooted policy and explicit predicates; hosting, voting and requester rights stay separate. Dynamic co-grouping therefore remains constrained by applicable placement policy rather than making every group member an accepted operator for arbitrary new data.

**Recovery and evidence cannot be enlarged by this document.** Lines 230–236 require quarantine for missing, corrupt or rolled-back stores and prohibit cloned active identities. Q3’s coherent snapshot/replay and hard-state ordering obligations remain controlling. BG-001–014 are explicitly future executable obligations, with real carrier, storage, signing and independent-domain gates retained. No private numerical fixture, synthetic refusal or Store seal is promoted into production bootstrap or all-writer exclusion evidence. The allocation introduces no library classification, dependency or process-global waiver.

## 3. Risks and next action

Concrete custody/floor maintenance, authenticated evidence formats, carrier activation boundaries, mixed-version compatibility and capacity remain substantial qualification work. Full-data co-grouping must preserve operator acceptance for every affected resource, including later dynamic creates. An explicit automatic-growth policy must also make the availability consequence of promoting a second voter observable. These are retained contract obligations and selection questions; the reviewed draft does not authorize unsafe defaults.

The next action is to select the provisional semantic options and prepare the exact canonical amendment and consumer-contract object, preserving the BG trace and later production gates. No production integration or activation acceptance follows from this GO.