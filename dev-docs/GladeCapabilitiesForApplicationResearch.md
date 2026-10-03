# Glade Capabilities for SaaS Application Research

Date: 2026-10-01. Status: capabilities brief and proposed research criteria.

Purpose: choose a real application that delivers customer value while testing
Glade, Glial, Grip, and Gryth under daily use. The application SHOULD expose
important weaknesses in the infrastructure through a useful workflow, with a
first release small enough to reach users.

The strongest architectural fit is a shared operational workspace: people and
agents work on the same artifacts, inspect live activity, and invoke explicit
actions through replaceable providers. Whether a particular market values that
combination remains a research question.

This brief distinguishes implemented mechanisms, incomplete integration, and
architectural direction. It is based on current source, tests, and contracts in
`glade-wz` and `gryth-wz`; tests and live deployments were not rerun for this
document. Older program summaries contain historical readiness statements, so
the more specific sources below take precedence for this inventory.

## Research Summary

The platform supports applications built from typed, reactive data surfaces and
declared service actions. A view requests the data it needs through a typed
interface; the runtime resolves its provider. Providers can supply local state,
mock data, asynchronous sources, or shared state without changing the consuming
view when their contracts remain compatible.

The stack includes shared values, append logs, incremental source projections,
and a collaborative text adapter. It supports multiple independent views of the
same kind, and tools can be connected through their data contexts. Gryth provides
a working desktop shell and a plugin boundary for those tools. Live Gyld decision
workflows and GWZ run integrations already exercise parts of the composition.
[Live composition runbook][runbook].

The intended product advantage is that humans, agents, and different tools can
work within one declared environment instead of exchanging copied snapshots.
Agent delegation, authenticated SaaS users, reliable offline delivery, and
general cross-node writes still require integration. Research MUST distinguish
those opportunities from features available to a new application today.

## Responsibilities in the Stack

| Component | Responsibility | Implication for an application |
| --- | --- | --- |
| Grip and Grok | Typed data handles, producers, reactive propagation, and provider resolution through a context graph. | Views and local tools can be implemented independently of their data providers. |
| Glial | Binding instances, local stores, shared-state assembly, incremental events, and optional Glade connectivity. | Several views can share one live binding while keeping their own selection and presentation state. |
| Glade | Shared operation storage and delivery, peer connectivity, provider claims, routing, and correlated service exchanges. | Shared data and source-backed actions have explicit protocol boundaries. |
| Taut and Taut Shape | Language-neutral contracts, generated representations, and delivery engines with behavioral corpora. | Implementations can be checked against exact contracts across languages. |
| Gryth | A desktop shell with multiple tool windows, context wiring, links, and a plugin API. | A product can compose task-specific tools over shared data rather than building one fixed screen. |
| Application suppliers and sources | Business rules, external-system adapters, authoritative effects, and application data. | The product still needs its own domain model and source guarantees. |

The [stack map][stack-map] defines these ownership boundaries, and the
[Taut Shape contract][taut-shape] separates the behavioral catalogue from
capabilities actually exposed by an application runtime.

Glade does not supply the application's relational queries, financial rules,
cross-record transactions, billing, or domain-specific conflict resolution.
An application MAY use a conventional database or external service behind a
supplier. Replicating its projection does not confer authority to mutate that
source.

## Capabilities and Application Value

Here, **implemented** means source and relevant tests exist for the named
mechanism. **Partial** means some mechanisms exist but the complete product
journey is not established. **Direction** means a documented architectural goal.
None of these labels is a production support or performance guarantee.

| Capability | What a customer-facing product could do with it | Current evidence and boundary |
| --- | --- | --- |
| Typed surfaces with replaceable providers | Build a usable mock, then connect a real source; substitute providers without rewriting every view. | **Implemented** in Grip and Glial. Substitution requires compatible semantics, not just matching field names. [Grip][grip], [manifest][manifest]. |
| Reactive and incremental views | Keep a queue, detail view, timeline, and summary current as work changes. | **Implemented** refresh/delta events and shape-aware assembly. Consumers choose how to apply events; universal backpressure and conflation are incomplete. [Events][events], [Glial gaps][gaps]. |
| Shared records and histories | Coordinate shared status, retain activity, and let late participants catch up. | **Implemented** value/log paths and session assembly. A whole-value last-writer-wins update is not a business transaction or a merge of every concurrent field edit. [Session tests][session-tests]. |
| Collaborative text | Have several people edit a shared note or narrative while preserving local selection through remote edits. | **Implemented** CRDT/text vertical slice. The supported Glial mount uses the explicit text profile; general structured-document collaboration and saved-file conflict handling are additional work. [CRDT adapter][crdt], [text tests][text-tests]. |
| Source-owned incremental projections | Present a changing tree, result set, or working set without treating every source event as a durable user edit. | **Implemented** SWMR assembly and file-window projection. The exact adapter has a single-writer constraint; it is not a general multi-writer record store. [Shape dispatch][shapes], [SWMR adapter][swmr]. |
| Local execution and persistence | Make views useful locally and restore retained operations after a reload. | **Implemented mechanisms; partial application integration.** Memory and IndexedDB stores exist. Gryth's shared-data runtime still uses memory; its desk layout uses interim localStorage persistence. [IndexedDB][idb], [Gryth runtime][gryth-runtime], [Gryth README][gryth-readme]. |
| Declared actions with explicit outcomes | Run a background job or source operation, display its progress, and associate its result with its request. | **Implemented** correlated exchanges, supplier handlers, and client append/subscribe outcome APIs. Source commit, deduplication, and publication recovery remain application obligations. A node acceptance is not proof that an external effect committed. [Supplier tests][supplier-tests], [client APIs][client]. |
| Multiple tool instances and connected views | Compare two cases; inspect different sources side by side; connect a browser to a detail view and pin an independent copy. | **Implemented** Gryth window/context mechanisms and shell intents. Tool capability varies: some providers are live and others are mock. [Plugin API][plugin-api], [tab contexts][tab-contexts]. |
| Components supplied through a Grip plugin interface | Add a specialist tool with its own taps and context, without making the window manager import its implementation. | **Implemented** Grip-keyed runtime plugin registry, component factories, and tab taps. The application composition root loads the packages. Remote code delivery, untrusted plugin isolation, and seamless live feature deployment are not established. [Registry][plugin-api], [composition][composition]. |
| Peer delivery and provider routing | Reach a source hosted elsewhere and recover subscriptions when routes change. | **Partial.** Node/peer transport and read forwarding exist; cross-node application writes remain a separate, partly built plan. Global placement, fleet failover, and arbitrary network performance are not demonstrated here. [Read routing][read-route], [cross-node plan][cross-node]. |
| A common interaction model for people and agents | Give an agent access to declared records and actions, and let it leave results in the same environment people use. | **Implemented local primitives; direction for complete delegation.** Grip exposes read/write handles and the desktop exposes tool intents. End-to-end scoped agent attachment and delegated remote context access are not established. [Plugin contract][plugin-contract], [Gryth vision][gryth-vision]. |
| Explicit scope, grants, and attribution | Separate shared work from personal presentation, and record who acted and on whose behalf. | **Partial.** Scope and attribution contracts exist; peer grant enforcement and optional client read checks exist. Authenticated client identity, comprehensive mutation policy, and Glial private-zone mapping remain incomplete. [App-file contract][app-format], [declaration limits][decl]. |

### Choosing a Delivery Shape

The choice is semantic: the application MUST decide what an update means before
choosing how to deliver it. A catalogue entry does not imply support through
every app-file, transport, storage, or client path.

| Mechanism | Useful application data | Important distinction |
| --- | --- | --- |
| `value` | A shared preference, selected configuration, or small status object. | Whole-value writes resolve last-writer-wins; concurrent edits may overwrite one another. |
| `log` | Comments, job output, or activity records. | Append and replay are appropriate; retention and domain interpretation need explicit policies. |
| `swmr` | A source-owned working set, tree, or incremental projection. | One producer owns the state; readers consume snapshots, deltas, and resets. |
| `crdt` with `text_crdt` | Collaborative notes or narrative text. | Glial's current mount supports this explicit payload profile, not arbitrary application objects. |
| `atom` | A provider's latest status. | Taut's latest-only service is distinct from a Grip atom tap used for local application state. |
| `stream` | Disposable metrics or live observations. | Loss is observable and late join is live-only; use a log when history matters. |
| `exchange` | A bounded action and its response. | A service interaction rather than a storage shape; source authority and recovery must be defined. |

The general durable Glial mount path currently accepts `value`, `log`, `swmr`,
and `crdt`. Atom and stream support has separate service adapters. App-file
recognition, bindability, and profile propagation have their own limits.
[Glial dispatch][shapes], [declaration contract][decl].

## Boundaries That Affect Product Choice

These are selection and delivery constraints, not reasons to postpone an
application indefinitely. A controlled pilot can start with one hosted source
node, explicit users, a small dataset, and a bounded workflow, then exercise
additional topology as the infrastructure becomes ready.

**Authentication and isolation.** The node currently accepts a client's claimed
principal name without proving its identity. Client read-grant checks are opt-in;
client application writes and exchanges are not checked by that switch. Glial's
declared private zone does not automatically produce a private wire key. A real
SaaS pilot MUST establish authenticated users and source-enforced access rules
for its data and actions; declaring a zone or grant alone is insufficient.
[Current grant behavior][app-format], [private-zone limitation][decl].

**Offline work and durability.** Local operation retention and reload recovery
exist, but Glial records an unresolved outbox gap: locally minted operations do
not automatically ship when connectivity is attached later. IndexedDB
write-through also swallows persistence failures. Client-library retry of
unacknowledged network operations is a different mechanism. Research SHOULD
distinguish offline viewing, local editing, and reliable later submission rather
than treating all three as one delivered feature. [Glial gaps][gaps],
[store implementation][idb], [client outcomes][client].

**Distribution and source effects.** Read forwarding does not yet imply that an
application write connected through another node reaches the claim holder.
Likewise, an accepted operation or a successful response does not establish
exactly-once external effects. The first product MUST state its authoritative
source, accepted-versus-committed meanings, retry behavior, and conflict policy.
[Cross-node work][cross-node], [source obligations][problem-inventory].

**Lifecycle and growth.** Closing a view is separate from terminating a source.
History retention, quota handling, CRDT compaction, and source recovery have
contract-specific constraints. Desk restoration is currently local; complete
cross-device desktop roaming and delegated remote UI control remain direction.
No scale, latency, hosting-cost, or availability benchmark was established by
this inventory. [CRDT remainder][crdt], [Gryth persistence][gryth-readme],
[lifecycle model][gryth-vision].

## What a Strong First Application Looks Like

The initial product SHOULD have a recurring unit of work: a case, incident,
project, experiment, job, or dossier. Several people need to inspect or change
that unit, and its progress matters enough that they will return to the tool.
An agent SHOULD perform a bounded useful action, such as preparing a proposed
change, assembling evidence, or running an analysis; its result remains attached
to the work rather than disappearing into a chat response.

Useful fit signals are:

- Shared artifacts and concurrent work are common, rather than rare exceptions.
- A live source, job, or observation feed changes what the team should do next.
- Different roles benefit from different views over the same underlying work.
- At least one provider can be replaced or added without a redesign of the UI.
- There is an identifiable buyer, observable cost of the current workflow, and
  a reachable group of users for a pilot.
- The first useful release requires few integrations and does not depend on
  every distributed-system ambition being complete.

A first slice SHOULD combine shared records, one live or incremental source,
one bounded action, and several useful views. It need not use every delivery
shape or force a desktop onto users whose work is primarily mobile.

Investigation and evidence workflows, incident coordination, project delivery,
field-work coordination, and research operations are search directions, not
validated markets or selected products. Research MUST test whether their actual
work benefits from the capabilities above. A generic CRUD application may be
commercially useful but provide little pressure on the distinctive mechanisms.

## Research Criteria

Candidates MUST be specific workflows for specific users, with a plausible payer.
Research MUST identify an initial user journey and the infrastructure it needs,
including missing capabilities. Existing products are evidence about the market;
the proposal is an application to build, not merely a list of SaaS vendors.

Score each criterion from 0 to 5 and apply the weights below. These weights are
proposed; the score is a comparison aid, not a substitute for customer evidence.

| Criterion | Weight | Evidence sought |
| --- | --- | --- |
| Recurring customer problem and willingness to pay | 25 | A costly repeated workflow, its owner, and evidence of spending or unmet demand. |
| A useful, contained first release | 15 | One end-to-end workflow that can deliver value without building a whole suite. |
| Shared work and concurrent participation | 15 | Concrete points where people need the same evolving state. |
| Useful agent actions | 10 | An action with inputs, an observable outcome, and a clear permission boundary. |
| Live or incremental source data | 10 | A source whose updates influence decisions during the workflow. |
| Multiple views and replaceable providers | 10 | Role-specific tools or alternative sources that benefit from composition. |
| Accessible data and manageable integration | 10 | APIs, exports, permissions, and a credible path to initial data. |
| Reachable pilot users | 5 | A practical way to recruit and observe a small user group. |

A high technical score MUST NOT compensate for no credible user problem.
Candidates that require unavailable integration rights, an entire new data
engine, or several unfinished infrastructure guarantees before their first
useful action SHOULD be deferred or narrowed.

For each finalist, research SHOULD propose an experiment showing whether these
mechanisms produce measurable workflow value compared with the current tool.
Building speed and operating complexity SHOULD be measured too: the infrastructure
must earn its place in the application.

## Pilot Scenarios

These are proposed acceptance scenarios for the selected application, not claims
that an application test suite already exists.

| Scenario | What the pilot SHOULD demonstrate |
| --- | --- |
| Concurrent work | Two people work on one artifact, with deliberate rules for conflicting updates and no unexplained loss. |
| Several views | Independent views stay current over shared data; changing one view's selection does not unexpectedly change another. |
| Agent participation | An agent produces a bounded result through the declared action contract, with its identity and authority established for that action. |
| Replace a provider | One mock or source provider is replaced while the consuming views retain the same contract. |
| Disconnect and reload | The application distinguishes retained, pending, committed, and unavailable state; any promised later submission is tested. |
| Denial or revocation | A prohibited read or effect is rejected at its authoritative boundary, including during retries and active subscriptions. |
| Lost acknowledgement | A repeated action can recover its outcome or report uncertainty without silently repeating an irreversible effect. |
| Source restart | Views recover from the source's actual surviving state; a replayed transcript is not mistaken for a still-running job. |
| View teardown | Closing a view releases its interest without accidentally deleting shared work or terminating its source. |
| Growing history | The chosen retention policy bounds storage or exposes its limits; the application does not assume compaction is free. |

The selected product MUST turn its applicable scenarios into executable tests
before implementing the corresponding behavior. Cross-node writes SHOULD be
added as an explicit pilot scenario when participants connect through different
nodes, rather than inferred from a successful single-node demonstration.

## Decisions for the First Application

This is a local product decision log. It records open choices and does not
change canonical Glade architecture decisions.

| Open choice | Proposed starting position |
| --- | --- |
| Buyer and domain | Keep the search broad; choose from evidence of a specific repeated workflow. |
| First deployment | One hosted source node and a controlled pilot; add peer topology when it has a user purpose. |
| First client | Reuse Gryth when comparison and several specialist tools are useful; use a simpler Grip client when the workflow calls for it. |
| Offline promise | State exactly what is available offline; reliable deferred effects require additional tested integration. |
| Agent role | Start with one bounded analysis or proposed change, with an explicit execution boundary. |
| Business storage | Use an appropriate source/database behind a supplier; choose its transaction and conflict guarantees deliberately. |
| Pilot dataset | Prefer data that the pilot team can legally access and safely use with the completed identity and isolation controls. |

## Deep Research Prompt

Use this document as the capability brief attached to the following prompt:

> Find candidate SaaS applications for a first real product on this platform.
> Identify specific users, repeated workflows, buyers, and current alternatives.
> Search broadly enough to discover unexpected domains; avoid selecting a
> generic product category simply because it can contain an AI chat panel.
>
> The strongest candidates make shared evolving artifacts, live source data,
> bounded agent actions, and multiple tools over the same work useful to their
> users. Treat the supplied readiness limits as constraints. Do not assume
> production SaaS authentication, universal offline delivery, arbitrary CRDT
> objects, remote plugin sandboxing, cross-node writes, or automatic live feature
> deployment are already complete.
>
> Produce 8-12 concrete candidates, then select three finalists. For each
> candidate, report the target user and payer, recurring problem, present
> workaround, competing products, evidence of demand, integration/data access,
> and a first useful end-to-end release. Map the workflow to named platform
> capabilities and identify missing infrastructure separately from product work.
>
> Apply the weighted criteria in the brief. Cite current primary sources such
> as product documentation, pricing, public APIs, and direct user or buyer
> evidence. Distinguish observed facts, inference, and untested hypotheses.
> Include reasons to reject each finalist and any weak or missing evidence.
>
> Recommend one discovery experiment per finalist, with a reachable pilot group,
> a measurable customer outcome, and a small build that meaningfully exercises
> the stack. Explain what ordinary web infrastructure would achieve just as well
> and where this platform could produce a practical advantage. End with the
> questions customer interviews must settle before choosing an application.

## Sources

The linked source and test files support the named mechanisms, not a claim that
their tests passed during preparation of this brief. Design documents support
intent and constraints where implementation is incomplete.

[grip]: /Users/owebeeone/limbo/glade-wz/grip-core/README.md
[manifest]: /Users/owebeeone/limbo/glade-wz/glial/src/manifest.ts
[events]: /Users/owebeeone/limbo/glade-wz/glial/src/events.ts
[gaps]: /Users/owebeeone/limbo/glade-wz/glial/dev-docs/DecisionLog.md
[session-tests]: /Users/owebeeone/limbo/glade-wz/glial/test/session.test.ts
[crdt]: /Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeCrdtAdapter.md
[text-tests]: /Users/owebeeone/limbo/glade-wz/glial/test/text_crdt_mount.test.ts
[shapes]: /Users/owebeeone/limbo/glade-wz/glial/src/shapes.ts
[swmr]: /Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeSwmrAdapter.md
[idb]: /Users/owebeeone/limbo/glade-wz/glial/src/store_idb.ts
[gryth-runtime]: /Users/owebeeone/limbo/gryth-wz/gryth-ui/packages/glade/src/runtime.ts
[gryth-readme]: /Users/owebeeone/limbo/gryth-wz/gryth-ui/README.md
[supplier-tests]: /Users/owebeeone/limbo/glade-wz/glial/test/supplier.test.ts
[client]: /Users/owebeeone/limbo/glade-wz/glade/client-ts/src/client.ts
[plugin-api]: /Users/owebeeone/limbo/gryth-wz/gryth-ui/packages/plugin-api/src/registry.ts
[tab-contexts]: /Users/owebeeone/limbo/gryth-wz/gryth-ui/packages/desktop/src/tabContexts.ts
[composition]: /Users/owebeeone/limbo/gryth-wz/gryth-ui/src/plugins/index.ts
[read-route]: /Users/owebeeone/limbo/glade-wz/glade/node/src/mesh/route.rs
[cross-node]: /Users/owebeeone/limbo/glade-wz/glade/dev-docs/GladeCrossNodeWritesPlan.md
[plugin-contract]: /Users/owebeeone/limbo/gryth-wz/gryth-ui/dev-docs/GrythPluginContract.md
[gryth-vision]: /Users/owebeeone/limbo/gryth-wz/dev-docs/GrythVision.md
[app-format]: /Users/owebeeone/limbo/glade-wz/glade/docs/AppFileFormat.md
[decl]: /Users/owebeeone/limbo/glade-wz/glade-decl/README.md
[problem-inventory]: /Users/owebeeone/limbo/glade-wz/dev-docs/GladeProblemInventory.md
[runbook]: /Users/owebeeone/limbo/gryth-wz/dev-docs/GrythGyldDemoRunbook.md
[stack-map]: /Users/owebeeone/limbo/glade-wz/dev-docs/StackMap.md
[taut-shape]: /Users/owebeeone/limbo/glade-wz/taut-shape/README.md
