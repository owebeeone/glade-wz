# Combined IC-3B physical-host/API cold consumer surface — SURFACE-AXIS REVIEW

**Review object:** Combined IC-3B physical-host/API consumer surface, including nonmutating ingress eligibility queries, successful-receive provenance, retained loss, close/reopen stops, and fresh-root compatibility. Controlling consumer guide: `dev-docs/GladeIndependentCrdtProductionIntegrationPersistenceUsage.md`, B candidate at workspace root `c8d778e7c24b68690e27bfac8ca4457f647e747f`, reviewed on 2026-10-04.

**Baseline:**

| Repository | Exact START and END SHA |
| --- | --- |
| Glade workspace root | `c8d778e7c24b68690e27bfac8ca4457f647e747f` |
| Glade | `d3fded040d6e3c459cd5f975d6c672873356cc23` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

Consumer documents were read using `git show c8d778e7c24b68690e27bfac8ca4457f647e747f:path`, with numbered output for citations. No implementation source was read.

**Date:** 2026-10-04

**Axis:** Surface: cold discovery, construction, defaults, API names, ownership, lifecycle pairs, failure handling, and supported recovery stops. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on their reports. Filed verbatim by the lane owner.

**Verdict: GO** — no P0, P1, or P2 findings; one bounded P3 documentation finding. This verdict covers the specified consumer surface only. It does not independently qualify implementation behavior, executed physical witnesses, remote exchange, or release acceptance.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| None owned by this fresh reviewer | Not applicable | Not applicable | Fresh full cold Surface review; no originating-finding closure claimed |

Original typed closures remain owner-deferred to wider IC3ABC. Original B Code/State counterexample closures are separate obligations. This report neither substitutes for them nor treats the submitted correction as their closure.

## Changed-range analysis

No prior Surface baseline was supplied for a retained-context re-verdict. The review therefore read all three permitted consumer guides in full rather than relying on a correction diff or previous reasoning. No remediation plan, implementation diff, design, evidence file, or original/current peer report was consulted.

The complete reviewed documentation scope was:

- `GladeIndependentCrdtProductionIntegrationTypedUsage.md`, lines 1–123.
- `GladeIndependentCrdtProductionIntegrationAuthentication.md`, lines 1–31.
- `GladeIndependentCrdtProductionIntegrationPersistenceUsage.md`, lines 1–95.

Scoped working-tree comparisons against the pinned root reported no changes to these documents. All five repository HEADs matched the specified tuple at START and END.

The one new root identified below is **nonarchitectural**: the consumer documentation omits the names and call shapes of the utility’s mutating retirement operations. Its remedy is documentation of the existing surface, not a new transition, ownership boundary, dependency, compatibility rule, or recovery mechanism.

The prompt’s recorded accounting is preserved: semantic accounting unchanged; typed two completed remediations; B one completed remediation. This fresh review does not reset those counts or independently audit their ledger.

## 0. Evidence base

The canonical generated Surface prompt was read completely. Process instructions read were workspace `AGENTS.md` and `AGENTS_GWZ.md`, Glade and external Gyld `AGENTS.md`, and the review-loop skill plus its canonical prompt template. Attempts to read member-local `AGENTS.md` in Glial and Glade-discover found no such file.

The consumer evidence was limited to the three documents above:

| Document | Sections and principal evidence |
| --- | --- |
| TypedUsage | Construction and explicit limits, lines 8–22; bounded receive and generic example, 24–76; abandonment/results, 78–90; failure and shutdown handoff, 92–107; storage/close, 109–115; representation and evidence boundaries, 117–123 |
| Authentication | Explicit development profile, lines 5–7; trusted clock, configuration, possession, verification and historical evidence, 9–21; qualification limits and physical-coupling distinction, 23–31 |
| PersistenceUsage | Configured factory and fresh/full/narrow opens, lines 5–45; append/authentication, 47–53; successful receive, loss and utility queries, 55–68; ownership/retry, 70–81; close/reopen and compatibility stops, 83–95 |

Inspection commands included `pwd`, read-only directory listing to locate repositories, `cat`, pinned `git show`, numbered document output, scoped `git diff --name-only`, and `git rev-parse HEAD`.

There was no build, test execution, network access, live API operation, file write, Git mutation, or implementation inspection. Links from the permitted guides to design, contract, persistence design, and evidence documents were not followed.

PersistenceUsage line 3 expressly states that B introduces no CLI command, binary ingress route, or configuration-file format. Consequently there was no helper CLI whose help required inspection. The first-day walkthrough was an API walkthrough by document inspection.

## 1. Findings

### [P3-1] Utility retirement is ordered but its public calls are unnamed

**Location:** PersistenceUsage line 66; TypedUsage lines 5 and 14.

**Root classification:** Nonarchitectural documentation omission.

**Violated invariant:** A newly documented lifecycle must expose both its eligibility checks and the operations that complete it. A cold host implementer should be able to identify the next public call from the permitted consumer documentation.

**Reproduction:** A consumer implements a replaceable host and follows PersistenceUsage line 66. The guide names `IngressAuthority::can_settle(&permit)`, `can_abandon(&permit)`, and `can_settle_received(&permit,&bytes)`. It explains that these are nonmutating checks, requires the appropriate query before disk I/O, and requires “the mutating utility retirement” only after committed selection. Neither this paragraph nor the other two permitted guides identifies that utility retirement operation’s public method name, argument shape, or result handling.

The session operations `settle_ingress` and `abandon_ingress` are named elsewhere, but they are the host-facing persistence operations being implemented. The documents explicitly distinguish the implementation utility from the concrete session. Naming the session operations therefore does not identify the utility calls.

**Impact:** The host implementer reaches a documented next step that requires guessing or implementation-source inspection. This weakens discoverability precisely where eligibility must remain distinct from durable completion. The existing text does communicate the safe ordering, so the omission does not establish an unsafe transition or blocking compatibility defect.

**Required correction:** Document the actual existing public mutating retirement methods, their argument/result shapes, and their relationship to the corresponding eligibility queries. Keep the distinction explicit: eligibility leaves ownership unchanged; successful durable selection precedes retirement; `Unknown` preserves outstanding custody. State that concrete DiskSession consumers do not call these utility methods themselves.

**Closure check:** Repeat a docs-only host-implementation walkthrough. Using only the permitted guides, the reviewer must be able to name the exact utility calls for eligibility, committed settlement retirement, and committed never-started-abandonment retirement, and explain what remains owned after `Unknown`. No implementation change is required by this finding.

## 2. Invariant analysis

### Explicit construction and defaults

The cold path begins with the configured Rust factory in PersistenceUsage lines 19–37. The guide identifies the import location, the three explicit provider/root inputs, the `ReplicaOpen` request, the fallible factory, and the actual host open call. It distinguishes factory validation from root locking and session validation.

TypedUsage line 12 expressly denies defaults for public configuration inputs and lists instance, descriptor, identity, owner, namespaces, finite limits, and expected floor. Authentication lines 7 and 11–15 likewise deny implicit trusted roots, keys, clocks, resources, capacity, or possession. PersistenceUsage lines 9–15 require separate absolute roots, nonzero registration, and an explicit provision choice.

The attempt to find an unstated default that silently selects identity, authority, path, or capacity failed. The three-attempt witness mentioned at PersistenceUsage line 58 is identified as a qualification example; the caller must supply an explicit finite retry budget.

### Fresh creation and supported reopen

Fresh creation is explicitly `provision=true`, fresh roots, and zero expected floor. Existing and partial roots refuse that path. A lost creation reply does not authorize a replacement identity or reset.

Full reopen preserves original paths, registration, owner, namespaces, and signed limits. PersistenceUsage line 41 explains the otherwise easy-to-miss expected-floor computation: hash the exact complete trusted-floor file bytes with a bounded read, including private envelope metadata. Re-encoding only the public floor, reading a peer’s value, or using a cached data image is expressly rejected.

Narrow `StorageAttemptHost::open` is documented separately at line 43. It restores the authoritative retained plan namespace on reopen, including roots originally created through `open_replica`; it does not derive a replacement namespace. Its lack of caller plan-namespace and expected-floor fields is explicit.

These descriptions defeat the cold-use counterexamples of fresh-provisioning an old scope, silently replacing its owner, inventing a new namespace during narrow reopen, or treating unavailable recovery as an empty store.

### Authentication and append outcome

Authentication lines 15–19 distinguish challenge issuance, actual channel/session-bound response authentication, verification, and sealing. Neither a challenge nor a seal is a durable receipt. One-use local authorization must be refreshed even for identical operation bytes.

PersistenceUsage lines 49–53 carry that requirement into the physical session: fresh possession precedes `submit_local`; exact retries preserve original receipts and charges; `BeginRequest.current` is not authoritative policy/time input. Historical receipts and terminal custody remain distinct from fresh admission permission.

No consumer text permits transporting old custody into new authorization, accepting peer HELLO as local possession, treating timeout as commit, or inflating the named `LocalProcessRestart` strength into quorum or power-loss durability.

### Successful receive and substituted results

The receive path is discoverable as a finite lifecycle: build the guard, register it durably, obtain its owned permit, construct receive work through the checked session, join all work, and classify the exact successful result.

PersistenceUsage lines 59–60 expressly reject foreign/reused/closed permits before callback construction and require the host’s private completion witness before inventory authentication. Different signed snapshots, malformed-prefix replacements, receive errors, and above-bound results cannot settle normally.

The new utility queries and `received_digest()` are clearly nonmutating. An absent, erroneous, or above-bound completion does not gain settlement authority. A valid empty snapshot settles only its own finite round.

The attempt to turn a different signed empty inventory into completion of an earlier receive therefore fails on the documented surface.

### Error, refusal, loss, and cancellation

PersistenceUsage line 61 names `retain_ingress_loss(permit,observed_bytes)` for a drained partial/unclassifiable result, including an error with a bounded available prefix. It states that the resulting loss marker does not assert zero observation.

Line 64 supplies the public continuation path after settlement has already consumed the permit and refused: `retain_pending_ingress_loss(&guard_id)` on the same live session. Malformed input and aggregate capacity refusal are covered. Failed publication preserves the original continuation and Pending close; committed loss permits close while permanently preserving incomplete status.

`retry_ingress` and `record_observation` are constrained to retained exact custody. They are not permit reminting, changed-observation substitution, or physical reset operations.

Cancellation remains honestly bounded. Started work cannot become never-started abandonment. A dropped future or external timeout is not drain evidence; cooperative cancellation must retain and join the work. The generic example expressly excludes loss recovery and does not constitute a complete physical-host error-handling recipe.

### Close, release, and unavailable recovery

PersistenceUsage line 85 names the qualified combined close call, states that the first close stops fresh input, and retains the session and both locks on Pending. Already-owned work continues through drain, classification, exact result/loss settlement, or native callbacks. Only Closed authorizes clean physical release.

TypedUsage line 115 resolves the two-trait close ambiguity by naming the qualified call and requiring equivalent combined ownership meaning.

The docs expose the uncomfortable stops instead of concealing them. Orphan receive guards after SIGKILL can remain pending and incomplete. An unresolved protected observation marker can make both full and narrow reopen unavailable. Missing/torn next images do not authorize clearing that marker. Neither dropping a Pending session nor deleting roots is presented as recovery.

These are explicit limitations of the bounded B profile, not missing secretly promised repair commands.

### Compatibility and completeness

Fresh provisioning owns the `legacy-store.sealed` marker in the newly created disposable root. The guide identifies the supported direction: preserve that marker and reopen through the configured independent adapter. Marker deletion is not qualified downgrade, and existing-store migration is excluded.

PersistenceUsage line 91 documents independent floor v2 incompatibility in both directions rather than promising implicit migration. Line 93 bounds the profile to Unix process restart and independently trusted floor custody.

Active guards, loss, missing obligations, and sticky uncertainty keep `complete_local=false`; later successful or empty inventories cannot erase them. The documentary claims are appropriately narrower than global convergence or universal crash recovery.

## 3. Risks and next action

This Surface GO rests on discoverable API ownership and lifecycle rules. It does not verify that the implementation enforces them or that the claimed old-executable refusal and physical restart witnesses occurred. Those require their independent axes and evidence gates.

The guides retain A2 and standalone B1 historical framing alongside the combined B additions. The explicit default/configured and standalone/physical distinctions were sufficient to complete this walkthrough without a blocking ambiguity.

The next action is to file this report verbatim and include its GO in the same-tuple review merge, recording P3-1 as a bounded documentation follow-up. Original finding closures, the genuine accepted-B record, and the strict pre-remote gate remain separate requirements.