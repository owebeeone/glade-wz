# Glial Glossary

Status: transitional draft

Purpose: preserve the earlier consolidated Glial vocabulary while the scoped
stack documents absorb its authoritative content.

Transitional note:
- The authoritative stack split now lives in
  `/Users/owebeeone/limbo/glade-wz/dev-docs/StackMap.md`.
- `Glade` kernel terms now belong primarily in
  `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeKernel.md`.
- This document remains useful source material until the scoped docs fully
  absorb its authoritative content.

Non-goals:
- This is not a glossary for generic distributed-systems terms.
- This is not a glossary for raw `grip` internals unless a term is part of
  Glial's core mechanics.
- This does not define transport-level terms such as HTTP, WebSocket, SSH, or
  TCP.

## Scope Rule

This glossary only includes:
- Glial runtime concepts
- Glial-owned collaboration concepts
- core underpinnings that Glial depends on directly

## Terms

| Term | Meaning | Owner |
| --- | --- | --- |
| `HybridApp` | A declarative application composition that combines one or more services, workflows, and graph spaces into one user-facing experience. | `glial` |
| `HybridAppInstance` | One concrete running or shared instance of a `HybridApp`. | `glial` |
| `Session` | One attached working scope for one participant inside a `HybridAppInstance`. A participant may be a browser, a local app, a backend process, or an agent. | `glial` |
| `HeadlessSession` | A `Session` with no human-facing UI. It is typically attached to an agent or backend worker principal. | `glial` |
| `Principal` | A permission-bearing actor such as a human user, an AI agent, or a service account. | `glial` |
| `AuthContext` | The authenticated principal plus any delegated effective principal or policy context used when attaching a `Session`. | `glial` |
| `ServiceMount` | A configured attachment to an external SaaS capability, account, workspace, or infrastructure resource. | `glial` |
| `GraphSpace` | One isolated replicated application-state domain hosted inside a `HybridAppInstance`. This is the primary Glial shareable state unit. | `glial` |
| `Share` | Working term: a collaborative state domain presented at the Glial layer. A share is usually realized by one or more `GraphSpace` values plus policy about who may attach and what is visible. | `glial` |
| `Share Scope` | Working term: the subset of state intentionally replicated together for one collaboration purpose. A share scope may map to a whole `GraphSpace` or to an explicit subspace chosen by Glial policy. | `glial` |
| `Private Scope` | State intentionally not shared by default. This is usually session-private or principal-private state such as transient UI details, unless explicitly promoted into a share scope. | `glial` |
| `Interest Spec` | A replicated declaration of what mutable external data a share currently wants. It is state, not an imperative transport command. | `glial` |
| `Source Binding` | One concrete binding record inside an `Interest Spec`, describing a target data source, query, range, window, or other upstream selection details. | `glial` |
| `Shared Projection` | The replicated value-level view exposed to non-primary followers. It carries materialized grip values and enough tap metadata to reconstruct a passive local view without replaying full tap internals. | `glial` with `grip/grok` support |
| `Primary Tap Role` | The runtime role assigned to a tap instance when it is the one allowed to perform side-effectful execution, singleton work, or upstream interest registration. | `grip/grok` used by `glial` |
| `Tap Execution Mode` | The declared distributed execution model of a tap. Current modes are `replicated`, `origin-primary`, and `negotiated-primary`. | `grip/grok` used by `glial` |
| `Replica` | One local materialization of a `GraphSpace` or `ControlSpace`. | `glymik` |
| `ControlSpace` | A replicated state space used for control-plane information such as route claims, placement leases, and capability claims. | `glymik` / `glial-mesh` |
| `RouteClaim` | A time-bounded statement that a node can serve, terminate, or relay for some scope. | `glial-mesh` |
| `PlacementLease` | A time-bounded statement that a node is currently hosting a workload or shared state domain. | `glial-mesh` |

## Relationship Notes

- A `Session` is not itself the shareable state. A `Session` attaches to one or
  more `GraphSpace` values and may also carry private scope state.
- A `GraphSpace` is the current primary Glial-level unit of shared replicated
  state.
- A `Share Scope` is the collaboration boundary that designers reason about.
  A `GraphSpace` is the concrete state container that usually realizes that
  boundary.
- An `Interest Spec` is distinct from the materialized values it causes to
  appear in a `Shared Projection`.
- `Primary Tap Role` is a role of a tap instance, not a separate object kind.

## Excluded Terms

The following are intentionally excluded here because they belong to lower
layers or are generic:
- `Grip`
- `Tap`
- `Drip`
- `Grok`
- transport protocol names
- generic mesh and networking terminology beyond the Glial-facing nouns above
