# Glade Raft first production profile decisions — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeRaftProductionProfileDecisions.md` at root `4702e283e1adac5863c5d542e57a9ceea9454541`; proposal, without owner ratification or activation, dated 2026-10-03.

**Baseline:**

- Root: `4702e283e1adac5863c5d542e57a9ceea9454541`
- Glade: `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`
- glade-discover: `52ea2d118f45d9e7c3d9a789310dd5d669958851`
- External Gyld: `ca04499a360d910fbf8ee2540ed446facd051b35`

Material repository sources were read through `git show <exact SHA>:<path>`. All four HEADs matched these pins at the beginning and end.

**Date:** 2026-10-03

**Axis:** Safety: attack degraded operation, disclosure, irreversible transitions, retention and recovery, and the boundary between owner selection and production authorization. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. The packet is fit to present for owner selection of semantic recommendations. This verdict does not accept a production implementation, canonical amendment, interface freeze, migration or activation.

---

## 0. Evidence base

Read the complete generated Safety prompt and the review-loop skill. Read pinned `AGENTS.md` and `AGENTS_GWZ.md`.

Inspected these pinned root documents:

- `GladeRaftProductionProfileDecisions.md`, lines 1–74.
- `GladeRaftProductionIntegrationPlan.md`, lines 1–100, especially ordered gates and the proposed profile.
- `GladeRaftQualificationPlan.md`, §§1–6 and accepted-through records.
- `GladeRaftAdoptionContract.md`, lines 1–97, including all RA-001–012 and the persistence-profile requirements.
- `GladeBuildEntry.md`, `LibraryBoundaryAndTestingPolicy.md` and `GladePackageArchitecture.md`, complete.
- `glade/GladeAuthzModel.md`, especially §§3/3a/3b/4/4a/6/7/7a/7b and AZ-16/AZ-17.
- `GladeRaftLegacyStoreSealContract.md`, complete.
- `GladeRaftQ4-BoundaryAudit.md` and `GladeRaftQ4-CarrierAudit.md`, complete, as factual maps with their declared limitations.

Inspected pinned Glade `GladeZones.md`, lines 25–100, and relevant `GladeSubstrateV1.md` address, transport and receipt clauses. Inspected external Gyld declaration lines 140–200, 320–405 and 540–610 covering admission, profile, Records, Delivery, supplier and assembly allocations.

A requested root-pinned read of `taut/ir/glade.taut.py` failed because that path is not present in the root commit. No live copy was substituted. The existing wire gaps were assessed from the pinned audit and substrate documentation; exact protocol-byte verification remains a later mandatory gate.

Only inspection commands ran. No builds, tests, network access, writes or Git mutations occurred. Current peer material and excluded working-tree changes were not read.

## 2. Invariant analysis

**Durable acceptance and degraded operation.** Attacked a successful write followed by loss of its leader, delayed follower application and incomplete third-voter catch-up. Line 12 requires majority retention of complete commands and ordered application with retained exact outcomes; line 13 requires coherent log/application/outcome reconstruction. Incorporation of RA-003/007/011 prevents treating log acknowledgment, metadata votes or incomplete recovery as application success. The remaining majority must meet the same promise after a voter is lost. This does not require simultaneous application on every voter, but it forbids a success whose exact outcome cannot survive the declared recovery profile. The packet expressly excludes power-loss and malicious-storage guarantees.

**Authentication, governance and disclosure.** Attacked a node-authenticated forward carrying a substituted user, caller-supplied private identity, borrowed-browser impersonation and voter-majority permission manufacture. Lines 14–15, 34–38 and the intact adoption contract require authenticated user-root derivation on every hop, creation-rooted governance and separate host/voter/requester authority. Root-key loss cannot authorize replacement genesis for the old identity. Unsupported caveats refuse rather than disappear. The restriction to key-signed sessions preserves the broader operator-vouched product direction.

**Reads under partitions and revocation.** Attacked an isolated former leader serving a cached settings value as current, and a fresh data barrier combined with unseen governance revocation. Line 16 requires a qualified quorum-confirmed barrier and local application for authoritative-current claims. Lines 40–44 explicitly distinguish data freshness from authorization freshness. Every serving hop checks locally known disclosure policy and states the unseen-revocation limit. No lease or subscription head becomes authoritative read evidence.

**Retries, movement, retirement and exhaustion.** Attacked lost replies followed by generation changes, retry after retirement, snapshots dropping refused outcomes, and capacity exhaustion silently converting outcome recovery into new work. Lines 17 and 21–24 preserve exact outcomes and the complete RA trace. RA-004/005/009/011 retain original retry identity and outcomes, require staged movement readiness through the committed cut, and prohibit retirement or compaction from recreating authority. Capacity exhaustion stops new work rather than evicting safety evidence. A single shape or known group does not waive these requirements.

**Migration, mixed versions and scope.** Attacked activation after sealing only Store while Registry, an old binary, a restored backup or an external supplier remains writable. Line 19 requires reconciliation and closure of every writer and rollback path; the controlling Q4-E gate additionally requires a persistent fence and fresh activation review. The packet correctly treats the seal as preparation. It introduces neither automatic cutover nor reset. The existing two-node launcher remains unchanged, and the known group does not become a static per-resource home.

**Owner-selection boundary.** Lines 46–62 prevent selection from authorizing implementation or deployment. Exact canonical amendments, superseded clauses, Gyld allocation, versioned Taut bytes, both-client tests and applicable reviews remain prerequisites. Actual hosts, custody, capacity and migration cut require explicit later inputs. The alternatives state meaningful availability, authentication and freshness consequences without asserting qualification already achieved.

## 3. Risks and next action

The remaining risk lies in translating these semantics into exact canonical and consumer contracts. Production adapter durability, automatic elections, authenticated governance frontiers, compatibility negotiation and complete legacy exclusion remain unqualified. This review supplies no runtime evidence.

The next action is to present the packet for owner selection. Any selected profile must then proceed through Q4-C’s exact amendment, allocation and consumer-contract gate before implementation.