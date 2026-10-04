# Declarative Application Stack — Independent Contract Review

## Assignment

Review the architecture afresh and recommend the smallest coherent declarative
contract for this application stack. This is an architecture investigation for
owner discussion, not an implementation task or an approved design.

Primary workspace: `/Users/owebeeone/limbo/glade-wz`.
Garns workspace: `/Users/owebeeone/limbo/garns-wz`.

We are building a new declarative application stack spanning Grip/Grok, Glial,
Glade, Taut/taut-shape, Garns, and suppliers for files, terminals, databases, and
other sources. The current running demo is evidence that some mechanisms work;
it is not production code or the architecture specification. Preserve useful
mechanisms without letting incidental demo choices define future contracts.

The central question is:

> What is the smallest coherent declarative contract that lets consumers use
> standard surfaces, lets nodes advertise what they can provide, and preserves
> source authority, domain isolation, security, and shape-specific recovery
> across different sources and stores?

The owner's hypothesis is that suppliers need standard interfaces analogous to
Grip Taps, connecting stores/sources to canonical Taut shape semantics, while
Glade manages distributed domains, authority, source bindings, and security.
Test and refine this hypothesis. Do not simply endorse it.

## Working constraints

- Read and follow each workspace's `AGENTS.md`, `AGENTS_GWZ.md`, and applicable
  member instructions before inspecting or working there.
- Only write the two reports listed below. Do not modify implementation,
  declarations, existing specs, decision logs, plans, or workspace configuration.
  Do not launch implementation work or mutate the running demo or its stores.
- Preserve all existing work. If an output already exists, inspect it and avoid
  overwriting another agent's report; use a clearly identified revision suffix.
- Use `gwz status` for workspace status. Root searches can skip nested member
  repositories: search explicit member paths or use `rg -uu` with exclusions for
  `.git`, build output, dependencies, and caches. Negative findings need a stated
  search boundary.
- Safe isolated tests are permitted when useful. State exactly what was run;
  distinguish code inspection, existing test evidence, and reproduced results.
- Separate ratified requirements, owner proposals, reviewer recommendations,
  implementation facts, and inference. Do not silently resolve contradictions.
  New proposed requirements use MUST/SHOULD/MAY and remain explicitly unratified.

## Starting evidence

Inspect the current files and code; do not assume earlier reviews are current.
Begin with these sources, then follow their relevant references:

- `dev-docs/DecisionLog.md`, especially GDL-031 through GDL-041. Determine each
  entry's actual status, including the distinction between implemented and ratified.
- `dev-docs/glade/GladeDeclSurface.md`, `GladeSupplierModel.md`,
  `GladeSystemDataSeam.md`, `GladeAuthzModel.md`, `GladeWorkspaceDirectory.md`,
  and `GladeInstanceLifecycle.md`.
- `glade/dev-docs/GladeZones.md`, `GladeSubstrateV1.md`, and the shape adapter docs;
  `dev-docs/TautShapeCatalogAdoption.md` and the pinned canonical shape contracts.
- Grip's Tap interfaces, Glial's binder/assembly/persistence interfaces,
  `glade-decl`, Glade node/client code, and actual supplier implementations.
- Current Garns architecture, integration, limitations, contracts, source,
  and `garns-rust`; distinguish ratified Garns from Grazel's older generated P8 store.
- Relevant supplier specs and GLP-0006 rulings, checking whether each is accepted,
  recommended, or still open.
- `dev-docs/GladePersistenceReview.md` as a prior investigation to verify and
  challenge, not an authority for the new architecture.

## Questions the review must answer

### 1. Vocabulary and ownership

Define surface definition, binding instance, supplier, provider advertisement,
replica, source, store, domain, zone, authority, and origin. Identify synonyms,
overloaded terms, and which layer owns each meaning.

Evaluate the proposed distinction between:

1. A stable surface definition: identity, payload, shape, operations, recovery.
2. An instance binding: domain/partition, source, authority, policies.
3. A provider advertisement: who currently serves which capabilities for it.

Map this onto existing declarations and runtime records; do not create a parallel
configuration authority accidentally. Evaluate whether the declared surface can
be the common contract across Grip, Glial, suppliers, and Glade.

### 2. Shape-to-source and shape-to-store interfaces

Determine whether source adaptation and replica persistence require distinct
interfaces. Explain what is common and what must remain specific to each shape.
Provide small illustrative interface sketches, not implementation code or a new
DSL grammar. Cover snapshot/read, subscription, changes, commands/writes,
restore/resume/reset, checkpoints, lifecycle, and unsupported capabilities.

Respect the canonical distinction between engines, profiles, views, and
interactions. Do not assume a universal bidirectional conversion: accepting a
replicated projection does not confer write authority over its source.

Describe what is authored, what can be generated, what is runtime data, and how
contract versions and conformance tests prevent adapters from inventing semantics.

### 3. Advertisement, routing, and security

Distinguish a source authority, a replica serving retained data, a provider
accepting commands, and a host able to instantiate a supplier. Identify the
minimum information each must advertise and what must remain private.

Explain who validates advertisements and how capabilities, authority terms,
freshness, withdrawal, expiration, and takeover affect routing. An advertisement
MUST NOT grant its own authority. Separate service discovery from node discovery
and placement. Assess existing discovery implementations only insofar as they
support or constrain this contract; do not turn this into a general discovery audit.

### 4. Domains, sources, and persistence

Distinguish Glade domain/zone, Garns relational scope, query/path/range parameters,
principal, device/tab identity, origin chains, and physical storage identity.
Explain their explicit mappings and where authorization is checked.

Use the earlier five dimensions—authority/source, domain/zone, shape/fold,
retention, physical store—as a provisional checklist. Challenge its completeness
and show dependencies and invalid combinations. It is not a ratified model.

Classify authoritative data, replicated state, recoverable projections, private
supplier state, caches, and live process state. Explain recovery obligations at
each boundary, including deletion, retention pressure, and offline writes.

### 5. Garns boundary and declaration composition

Identify where Garns belongs and where it does not. Evaluate a Garns authority
store, captured external relational data, and Garns-backed projections of Glade
state. Assess any proposed use as a replica store against Glade's op semantics.

Compare explicit mappings between Garns and Glade declarations with generated
integration alternatives. Establish one owner for every shared semantic fact;
do not assume either language should generate the other. Assess actual current
language/runtime capabilities without treating present gaps as permanent limits.

Cover the failure between committing an authority transaction and publishing its
Glade result, including retry, duplicate effects, and recovery. Explain why a
Garns ledger, Glade op journal, and shape cursor are or are not interchangeable.

### 6. Worked journeys and architecture choices

Trace three contrasting journeys through declaration, registration/advertisement,
authorization, source execution, delivery, client assembly, and persistence:

- A scoped Garns live question, plus a command that changes its source rows.
- Filesystem content, collaborative editing, and saving against a base revision.
- Terminal creation, input authority, live output, scrollback, and reconnect.

For each, include restart, disconnect, source change, authority turnover, and
revoked access. Identify what survives, what can be rebuilt, what is lost, and
which contract makes that outcome explicit. Include one private/account-scoped
consumer state example and a compact control-plane state example to test the
boundaries beyond workspace commons data.

Compare at least two plausible ownership/interface arrangements. Recommend one
with concrete tradeoffs and counterexamples. Identify where it conflicts with
ratified ground and requires a new owner ruling. Treat prior assistant proposals
about terminal spools, file authority, or retention defaults as unaccepted options.

Finally classify demo mechanisms as reusable, adaptable, experimental, or absent,
with evidence. Recommend the smallest next proof of the proposed contract and
its observable acceptance criteria. Do not produce an implementation backlog.

## Deliverables

Write exactly these two reports under `/Users/owebeeone/limbo/glade-wz/dev-docs/`:

### `ApplicationStackContractReview.md`

Title: **Declarative Application Stack — Contract and Boundary Review**

Include a dated evidence snapshot, plain-language architectural finding,
vocabulary/ownership map, declaration and interface sketches, advertisement
contract, the worked journeys, alternatives, recommended arrangement,
compatibility with ratified decisions, demo disposition, and next proof.

Use a few compact diagrams and matrices where they expose relationships. Cite
source paths and line numbers and record relevant member revisions and dirty
state. Assign stable findings IDs `ASCR-F01`, etc. Keep detailed evidence out of
the main explanatory flow where possible. Target 3,000–5,000 words; prioritize
clarity and coverage over hitting the target.

### `ApplicationStackContractDecisions.md`

Title: **Declarative Application Stack — Owner Decision Worksheet**

This is a local record of unresolved proposals, not an update to the canonical
decision log. Cross-reference existing GDL decisions and review finding IDs.

For each decision, assign `ASCR-D01`, etc., and record: the exact question, why
it matters, plausible alternatives, recommended answer with tradeoffs, affected
contracts, dependencies, and the evidence or conformance scenario that would
validate it. Set status to **PROPOSED — awaiting owner decision** unless an
existing authoritative ruling already answers it; cite that ruling when it does.

Order by dependency. Identify the small set of owner decisions necessary before
contract design can proceed, and separate them from implementation choices and
work that can proceed independently. Do not ask the owner to choose numerical
cache defaults or framework details before the semantic model is settled.

## Completion

Return links to both reports, the recommended architecture in a few sentences,
the most consequential uncertainties, and the first decisions for discussion.
State inspection/testing limits. Do not implement, ratify proposals, create a
plan folder, change canonical decisions, or start another task.
