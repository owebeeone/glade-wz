# Glade build entry — architecture, then delivery

Date: 2026-09-08. **Owner priority: build Glade; park Gyld's next phase.**
The first-slice proposal below is not a new protocol ruling or a claim of readiness.

Owner correction, 2026-09-09: define the architecture in a **Gyld declaration**
before proceeding to implementation. Start with
[glade-architecture.gyld.py](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py),
especially `GladeArchitecture`, which allocates the retained problem's 24 responsibilities.
The [supporting design notes](arch1/GladeArchitecture.md) explain the proposal;
they do not replace the declaration. The candidate is structurally captured and
checked, not runtime-proven or ratified. New/recomposed ports MUST have compiling
consumer/conformance specifications and adversarial review before implementation.
The first working slice below follows that gate, not a bypass around it.

Current composition checkpoint: [Shaku/Dill evaluation](arch1/DependencyInjectionEvaluation.md)
and [graph revision 2](arch1/InjectionGraphRefinement.md). The owner subsequently
selected **Shaku**, added at NodeAssembly in revision 3. The real async-port/startup/
cleanup witness remains open; keep the demo untouched. For a less dense explanation,
use the [find / call / follow reading map](arch1/ReadingMap.md).

## Current map

Use the [source-qualified Gyld capture](/Users/owebeeone/limbo/gyld-wz/dev-docs/case-studies/glade/ProblemSpaceCapture.md)
and its [responsibility SVG](/Users/owebeeone/limbo/gyld-wz/gyld/artifacts/glade-problem-space-v1/responsibilities.svg).
It exposes 24 responsibilities, existing draft contracts, external constraints,
open choices and failure journeys without choosing a crate for every function.
Canonical Glade decisions/specifications remain authoritative; Gyld is a model
and review tool, not a replacement source of product requirements.

Before a build task, read this entry, the applicable source references, and
[LibraryBoundaryAndTestingPolicy.md](LibraryBoundaryAndTestingPolicy.md).
Do not restart the whole architecture discussion or require the full document
corpus to be read before one bounded implementation. Do not revive Gyld engine
development as a prerequisite for Glade.

## Recommended first working slice

**A configured provider registers; another participant discovers it with an honest
receipt and authenticated, scoped metadata.** One namespace, fixed authorized
peers/mapping, explicit development trust configuration. The target demonstration
uses real Iroh and real local persistence; deterministic fixtures come first.
The graph's `locate_providers`, `reconcile_metadata`, `peer_connectivity`,
`verify_identity`, `enforce_policy`, `retain_replica`, `validate_ingress` and
`compose_runtime` are relevant—not eight mandatory new crates.

1. **Pin the slice contract and write executable consumer tests.** Reconcile the
   existing discovery core/node-adapter and seven draft host/registry contracts.
   Identify the exact canonical records, trust/namespace proof profile, clock
   inputs and local-acceptance guarantee. Record each choice as a bounded profile;
   do not silently invent a new wire grant or weaken the frozen discovery contract.
2. **Build the smallest injectable assembly.** Exercise publish, exact retry,
   renewal, expiry, wrong scope, unknown/denied authority, partial lookup and lost
   acknowledgement using deterministic, contract-faithful providers. One explicit
   dependency scope MUST reach every cooperating consumer. A fixture must not
   pretend to be physical durability or cryptography.
3. **Replace its boundary providers with real adapters.** Verify genuine signing,
   durable-local acceptance before gossip, restart/retry recovery, then a fixed-peer
   Iroh route. Reuse the applicable conformance fixtures; add the actual I/O/fault
   evidence they cannot supply. Keep these tests out of pure-library edit loops.

The final slice passes only when a real registration is discoverable across the
selected peer route, expired/unauthorized entries are excluded, loss/retry/restart
outcomes are honest, and the fast independent development path is demonstrated.
Exact source proof/encoding incompatibilities are explicit blockers to the affected
adapter—not permission to substitute synthetic evidence in the real demonstration.

Next, add an authenticated read-only supplier call and one declared shape through
Glial. Do not mix source mutation/publication recovery into the first discovery
milestone. Existing application contracts remain inputs for that following slice.

## Keep visible, but outside the first slice

Dynamic shard election/migration, million-node performance, a general trust product,
arbitrary offline authorization, the entire shape × storage matrix, service placement
at fleet scale, a new lifecycle framework, and Gyld optimization remain separate.
Fixed configuration is a development slice profile, not a claim to solve these.
Garns integration into Gyld is not a Glade dependency.

Preserve the existing demo and uncommitted work. New assembly/adapters SHOULD start
alongside it; no bulk extraction or deletion is implied. Public trait changes MUST
start with failing tests, check affected consumers and run the adopting boundary's
architecture gate. Keep shared semantic validation and attachment/scope policy
central where appropriate; retain proof only for unchanged validated observations,
not for stale authorization. Record new findings against the capture's stable IDs.

This documentation task did not implement the slice, select a consensus/async
framework, migrate the demo, launch agents, commit or push.
