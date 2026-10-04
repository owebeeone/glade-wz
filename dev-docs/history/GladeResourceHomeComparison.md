# Glade resource home selection — comparison result

Date: 2026-10-03.

Status: **comparison complete; owner mechanism selection pending**. Both
[Product/availability](GladeResourceHomeComparison-ReviewProduct.md) and
[Correctness/engineering](GladeResourceHomeComparison-ReviewEngineering.md)
returned GO on the same committed inputs, with zero P0–P3 findings. This accepts
the evidence and mandate for a decision, not a resource-home mechanism or its
implementation.

## 1. Recommendation

**Design H1 next if home-dependent availability is acceptable:** dynamically
choose and durably bind a home when an authorized resource is created, then
discover that binding through replicated records. Keep resource identity
separate from host identity. Start with share-level placement and settings zones
inside it, matching current routing; do not add one election per settings key.

Both fresh reviewers independently reached that ranking. H1 meets the confirmed
dynamic creation/discovery requirement with fewer transition and membership
mechanisms. H2's planned movement and H3's automatic failover solve additional
requirements the owner has not yet made mandatory. This is a qualitative scope
comparison, not a measured delivery-time or performance claim.

The condition is consequential: **an unreachable home means no authoritative
write progress for that resource**, although declared cached reads and local
pending edits may continue. Permanent loss is not solved by assigning a new home.
The owner has NOT yet accepted that availability policy. Treat H1 as the
recommended next design under that policy, not a selected mechanism.

## 2. What changes the choice

| Product requirement | Preferred direction | Decisive consequence |
| --- | --- | --- |
| Dynamic creation/discovery; home outages may leave edits pending; planned moves optional | **H1 > H2 > H3** | H1 avoids a transfer recovery protocol and quorum deployment before either is needed. Canonical creation, retry, authorization and enforcement still require real design work. |
| Planned maintenance/rebalance MUST move a resource without changing identity; old home can cooperate | **H2 > H3; H1 ineligible** | H2 supplies the missing operation. It must verify a data cut, fence old admission and recover every interrupted transition. It cannot promise takeover of a lost old home. |
| Automatic takeover after one machine failure MUST preserve identity and all acknowledged data; additional independent domains accepted | **H3 conditionally eligible; H1/H2 ineligible** | Requires ownership quorum, successor data readiness and actual stale-home exclusion. Data replication and acknowledgement policy must be designed separately. |
| Same automatic guarantee, exactly two majority voters, no additional authority | **None of H1/H2/H3 satisfies it** | Neither surviving voter has a majority. Promoting it unilaterally changes the fault model and risks conflicting authority. |

Configured per-resource homes (H0) remain disqualified by the owner's dynamic
creation requirement. Fixed bootstrap trust or authorized group configuration
is a different matter; it MUST be explicit but need not preassign resources.

In the Product report's baseline ranking, “pending outages acceptable” is an
assumption used to rank candidates. It is not a new owner ruling. The table above
and the recommendation make that condition explicit.

## 3. Practical differences in the same situations

Full DC-01..10 matrices and DP-1..3 analyses are in both verbatim reports. The
differences most likely to decide the choice are:

| Situation | H1 | H2 | H3 |
| --- | --- | --- | --- |
| First node creates a new independent scope; second joins by invite | Durable create/retry and discovery of the same identity/home | Same initial mechanism | Authorized existing group or deliberate singleton genesis; discovery alone cannot bootstrap authority |
| Two clients race to create one named resource inside an existing scope | Identified naming/creation authority serializes or reports conflict/unknown | Same requirement | Group serializes the entry; namespace and alias policy still needed |
| Two-node link partitions | Home side may serve under its declared permission policy; other side pending/cached | Same; required cooperation/fencing can block movement | Two voters have no majority; new ownership decisions stop. Existing service depends on a separately proven validation/lease policy |
| Lose the non-home | Home service can continue if needed authority evidence remains valid | Same | With two voters, even this loss removes quorum for new placement; some existing service may continue under a proven lease |
| Lose the home | Authoritative service unavailable | No unilateral handoff; unavailable without necessary participants/enforcement | Failover only with surviving quorum, sufficient data and fencing |
| Move while old home cooperates | Unsupported under H1; fork/import changes identity | Supported after safe, recoverable handoff | Supported through serialized transition plus data/effect recovery |
| Old home resumes after a move | H1 has no move protocol | Old generation must be rejected at the actual store/effect boundary | Same; election or directory winner alone does not fence effects |
| Survivor lacks an operation already acknowledged | Explicit data loss/unavailable recovery | Same; no truthful preserved-history handoff | Same unless the selected independent data guarantee already preserved that operation elsewhere |

One node can create a new independent root without a global registry. Creating
inside a known existing scope is different: its naming authority may be
unavailable. An empty local lookup never proves that the scope/resource is new.
All three candidates must recover the original creation intent after a lost reply.

Placement policy cannot make an absent data copy reappear. Current substrate
`Ok` is limited: local process-crash survival without fsync, with an additional
holder/forwarder copy on the forwarded path. That is not a promise to preserve
every accepted operation through permanent machine loss. A stronger guarantee
can also be considered with H1/H2; it is not exclusive to H3.

## 4. The default two nodes

Two development processes on one machine exercise routes and lifecycle; they
do not provide two independent machine/storage failure domains. Two machines
would improve that separation but still provide only two voters if both vote.

Three independent ownership voters can retain a majority after one failure.
A third placement-only witness does not supply missing settings history.
With two data nodes plus a witness, acknowledging only after both data nodes
store an operation can protect that operation against one data-node loss under
the declared storage guarantee; maintaining that same two-copy rule cannot
continue after either data node is gone. Three data-bearing voters are another
deployment possibility, but application data still needs its own durable
commit/catch-up contract.

H3 also has distinct validation branches: per-operation quorum, bounded leases,
or a shared authoritative sink. They impose different outage behavior. The
comparison has not chosen one or a lease duration. “Quorum unavailable” does
not automatically mean existing leased service stops instantly; it does mean
new ownership cannot safely be acquired without the required authority.

## 5. Agreement, staging and the settings question

The reviewers independently agree on the baseline ranking and both ranking
flips. Neither proposes implementing H3 solely because clients discover one
another dynamically. Neither considers H1 a trivial wrapper around current
claim tie-breaking: creation serialization and resource enforcement are missing
proof obligations, not already delivered guarantees.

There is no material ranking disagreement. Engineering stresses preserving
identity/generation and migration seams when designing H1; Product stresses that
H2 is not an inevitable second stage without user value. These are compatible:
keep a reviewed route to future transfer without implementing it prematurely.
If planned moves are required in the first supported release, both recommend
designing H2 directly rather than freezing an immutable H1 contract first.

**Both independently identified a scoped alternative for mergeable preferences:**
authorized replicas could accept operations independently and converge through
declared merge rules. That warrants a separate comparison if offline settings
use is a primary requirement. It needs consistent schema/shape and authorization
validation, operation identity/retry, honest durability receipts, revocation
limits and visible conflict behavior. It cannot simply reuse rejected dual
admission where one replica acknowledges operations another refuses.

This is an investigation recommendation, not an accepted H4 protocol. Do not
apply it to grants, membership, single-writer bindings, physical working copies
or external effects. A preference that triggers an exclusive effect needs that
effect's own enforcement boundary. Before calling all “settings” exclusive
resources, distinguish appearance/layout preferences from policy and actions.

## 6. Smallest owner decisions before a selected design

1. **Availability:** may settings edits remain pending while the home is
   unreachable? If yes, H1 is the recommended starting direction. If no,
   distinguish automatic exclusive failover from mergeable multiwriter preferences.
2. **Movement:** must planned moves preserve resource identity now? If yes,
   choose H2's design scope directly when cooperative movement is sufficient.
3. **Data guarantee/deployment:** must acknowledgement survive permanent
   machine/storage loss, and what independent data/authority domains are acceptable?
   H3 needs an adequate quorum deployment for automatic failover, plus separate
   data guarantees; extra placement voters alone do not satisfy this requirement.

The selected design must also settle canonical naming authority, creation under
an unavailable existing scope, identity/alias rules, resource-specific fencing,
permission freshness, authorized bootstrap/membership and retirement/activation.
Those details are mandatory design obligations, but a wire format or library
choice is premature before the three product decisions above.

Under an H1 choice, the next tranche is a creation/identity/admission contract:
durable canonical binding, exact outcome lookup/retry, identity separate from
host, unavailable-versus-absent discovery, permission enforcement, retirement
and migration eligibility. It must identify exact canonical amendments and pass
its own design/contract review. Source implementation starts with failing tests;
current routing or loopback evidence is not acceptance of that future contract.

## 7. Review provenance and verification

Applied the [review-loop skill](/Users/owebeeone/.claude/skills/review-loop/SKILL.md)
and [canonical template](/Users/owebeeone/.claude/skills/review-loop/references/review-prompt-template.md).
The [comparison brief](GladeResourceHomeComparisonBrief.md) §5 binds the process:
Consistency → Product/availability, Safety → Correctness/engineering. Fresh
agents `/root/home_comparison_product` and `/root/home_comparison_engineering`
inherited the lane-owner model/effort, ran concurrently and were peer-blind.
No earlier ranking reports were provided. Each also received an identical
style-only follow-up targeting roughly 1,500–2,000 words while retaining full
scenario/profile coverage. Their complete outputs are filed verbatim.

The comparison inputs were settled and verified at both reviewer boundaries:

| Input | Exact review revision/object |
| --- | --- |
| Root | `313b3c85dd623943251a0a14df1de47a0a8bf666` |
| Glade | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| Glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| Comparison brief Git blob | `163fa8e93f93f247af026cf2ada750f4b84145fe` |
| Comparison brief SHA-256 | `9bd67caa0f099012a385bfb0645dec4f6fab3ddcb933f49a4535cd71e7a8df0d` |
| Alternatives packet Git blob | `a34acb46b27beb164ac67194fae1227905228033` |
| Alternatives packet SHA-256 | `45f635b9db055ccd010ec025b871e25616daa5cd4a7ec422079e8a06307790b8` |
| [Product prompt](resource-home-comparison/prompts/Round1-Product.txt) SHA-256 | `9663eba065dd48ba02ce44e97b20cd6c662f15dafb42a0785680bc557ec2c33a` |
| [Engineering prompt](resource-home-comparison/prompts/Round1-Engineering.txt) SHA-256 | `f8cdd351e5bb12bd8f3d26cbcaca1501356a97caa44f094004a3e47287a59964` |

Skill/template hashes remain those recorded in the
[earlier cycle](GladeResourceHomeAlternatives-ReviewCycle.md) §2. Prompts were
generated from the canonical template with exact tuple, common scenarios,
axis bindings and decision-ranking output requirements. These input pins are
immutable references; live working-tree motion was not review evidence.

| Report | Verdict | Findings | Independent recommendation |
| --- | --- | --- | --- |
| [Product](GladeResourceHomeComparison-ReviewProduct.md) | GO | Zero P0/P1/P2/P3 | H1 with accepted home dependence; H2 for required planned moves; H3 for required failover with suitable data/authority deployment |
| [Engineering](GladeResourceHomeComparison-ReviewEngineering.md) | GO | Zero P0/P1/P2/P3 | Same ranking; design real creation/admission and preserve future migration boundaries |

One dual comparison round completed, no remediation rounds, no open findings
and no blind defect convergence. Agreement on ranking and the preference-only
alternative is independent recommendation convergence, not proof of a protocol.
All quantitative performance, implementation-time and cost evidence remains
unmeasured.

The lane owner checked local Markdown links, whitespace, prompt hashes and
unchanged settled-object bytes. Reviewers performed read-only source traces;
no implementation tests, builds or runtime checks were run for this document
task. Scoped local GWZ commits settle inputs and file evidence. No code,
dependency, API, test-selection, push, merge or runtime restart is included.
Unrelated working-tree work and tool-generated GWZ metadata remain out of scope.
