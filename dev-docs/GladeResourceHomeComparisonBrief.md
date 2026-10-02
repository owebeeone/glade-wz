# Glade resource home selection — decision comparison brief

Date: 2026-10-03. Status: **DRAFT comparison mandate; owner selection pending**.

The owner authorized a two-agent comparison following acceptance of the
[alternatives packet](GladeResourceHomeAlternatives.md). This is a selection
exercise: reviewers MUST rank candidates and explain tradeoffs under explicit
assumptions. Another bare GO on document quality is insufficient.

## 1. Evidence and fixed requirements

Use the alternatives packet accepted at root
`cb87e4af820d5ead2ee0461ad966589bed60007f`, blob
`a34acb46b27beb164ac67194fae1227905228033`. Its bytes are unchanged at the
comparison checkpoint. Source-qualified references in its §2 control, with
Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b` and Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`. The new review root SHA is recorded
in the generated prompts and final comparison record.

The owner requires dynamic creation of resources/scopes and discovery without
manual per-resource home mappings. The development launcher defaults to two
Glade nodes. That default is a deployment constraint for this comparison,
not proof of two independent machine/storage failure domains. Browsers are
clients; Glade nodes serve resources. Settings live within a share/zone and
MUST NOT silently become one election group per key or browser.

Automatic failover, immediate authoritative acceptance while disconnected,
durability across permanent machine loss, and planned home movement have NOT
been chosen as mandatory product requirements. Reviewers MUST label any such
assumption and show how changing it changes their ranking. Authority ownership
MUST remain distinct from hosting and discovery.

H0 is a disqualified baseline. Compare H1 (dynamic stable home), H2 (dynamic
creation plus cooperative fenced transfer) and H3 (consensus-backed conditional
failover) as described in the packet. Do not quietly strengthen their guarantees.
If the exclusive-home premise is unnecessary for a particular settings class,
identify a scoped additional alternative and its merge/authorization/effect
obligations; do not apply that conclusion to physical copies or external effects.

## 2. Shared deployment profiles

| Profile | Assumptions and question |
| --- | --- |
| DP-1 | One node starts alone. A new authorized independent root may be created. Joining or creating within a known unavailable scope is a distinct case. Explain whether H3 needs an existing group or an explicitly authorized singleton; no automatic singleton-to-multi-voter upgrade is assumed. |
| DP-2 | Two nodes, the development default; also analyze them on two separate machines. No third service is presumed. One node is lost or the link partitions. Distinguish loss of a non-home from loss of the home and distinguish existing service from new placement/creation. |
| DP-3 | Three authorized voters in independent failure domains, with an explicitly stated data-replica layout. This is a possible additional operational commitment, not an owner-approved default. A witness holding placement state alone does not restore absent settings history. |

No latency, implementation-time or cost measurements are available. Use
qualitative effort/dependency comparisons with reasons. Do not invent numerical
scores or sum weights that hide an unmet hard requirement.

## 3. Shared scenarios

Every reviewer MUST compare H1/H2/H3 against all scenarios and distinguish
authoritative writes, local pending edits, cached reads and data recovery.

| ID | Scenario |
| --- | --- |
| DC-01 | One node/client starts and creates a new scope/resource dynamically; then retries after a lost create reply. Contrast independent-root creation with naming inside an existing scope. |
| DC-02 | A second node/client joins through an authorized invite, finds existing settings and writes. Its empty local view cannot justify defaults or a new identity. |
| DC-03 | Two creators race for one canonical existing-scope resource; aliases and delayed advertisements differ. |
| DC-04 | Lose the non-home, then lose the home, in DP-2. State what happens to already active service, new creation, reads and writes, including any lease grace period. |
| DC-05 | Partition the nodes and attempt writes on both sides; distinguish cached/pending behavior from authoritative admission. |
| DC-06 | Permanently lose the home and one locally acknowledged operation absent from the survivor. Preserve the resource identity only where protocol and data guarantees permit it. |
| DC-07 | Pause the old home, move or fail over where supported, then resume it with delayed operations and stale claims. Trace actual fencing. |
| DC-08 | Deliberately move an existing resource while the old home cooperates; crash or lose replies at each transition boundary. |
| DC-09 | Start as DP-1, add a second node, then consider DP-3. Trace authorized membership, migration and rollback; no independently formed group may usurp an existing scope. |
| DC-10 | Revoke a host/voter, retire a resource, replay old proofs/claims/create retries, or activate a new protocol beside legacy writers. State freshness and compatibility limits. |

The packet's HF-01..10 remain adversarial acceptance obligations. This review
does not claim those tests are implemented or that current claims already fence
authority. The acknowledged-data policy is a separate selection dimension from
placement consensus.

## 4. Independent review axes and required output

Two fresh, concurrent, peer-blind, read-only agents MUST use the same exact
committed brief, alternatives packet, source pins, scenarios and profiles:

- **Product/availability:** rank user-visible behavior, dynamic onboarding,
  outage and reconnect expectations, pending/conflict UX, and operational
  assumptions exposed to users. Challenge whether a proposed feature solves
  the actual browser/settings need.
- **Correctness/engineering:** rank enforceable ownership, data safety,
  creation/transfer recovery, authorization, migration, implementation seams,
  deterministic verification and ongoing operational burden.

Each report MUST provide a side-by-side scenario matrix, deployment profile
analysis, ordinal ranking under the confirmed baseline plus at least two
alternative requirement profiles, its preferred next design and the requirement
that would change that choice. Rank eligibility first; an unavailable feature
cannot be compensated by a lower implementation cost.

Separate verified source facts, protocol deductions and proposed product policy.
Explain residual unknowns and any strong objection to a candidate. Report GO/
NO-GO only as fitness of the evidence/mandate for a decision; the recommendation
is the substantive result. No candidate is ratified merely by a reviewer ranking.

## 5. Filing and decision boundary

Reuse the [review-loop skill](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
with its canonical prompt template, binding Consistency to Product/availability
and Safety to Correctness/engineering for this comparison. Preserve read-only
inspection, exact start/end tuple verification, fresh peer blindness, verbatim
reports and a lane-owner synthesis. Earlier review reports and their rankings
are excluded from reviewer input to reduce anchoring; the packet is the common
candidate definition. Its staging hypothesis is explicitly open to challenge.

The synthesis MUST expose agreement and disagreements without claiming consensus
where assumptions differ. It MUST make one conditional recommendation for the
next design, identify the smallest owner decisions needed, and preserve the
distinction between a recommendation and an owner-approved contract. Any blocking
defect in the comparison mandate uses the skill's bounded remediation procedure.

This is documentation and comparison only. No mechanism selection, dependency,
API/wire change, source implementation, push, merge or runtime restart is
authorized. Scoped local GWZ commits settle the inputs and file the evidence;
unrelated working-tree work and generated GWZ metadata are out of scope.
