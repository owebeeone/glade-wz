# Glial Orchestration

Status: working draft

Purpose: define the thinner orchestration layer that sits above Glade.

## Core Claim

Glial is not the share kernel.

Glial is the environment and orchestration layer that:
- composes environments from many application facets
- mounts collaboration domains and services
- attaches participants and agents
- applies policy and capability issuance
- exposes user-facing working environments over Glade shares

## Glial Responsibilities

Glial owns:
- environment composition
- participant and session attachment policy
- headless session attachment for agents and background workers
- authenticated and delegated policy context
- service mount configuration and lifecycle
- workspace membership and capability issuance
- visibility policy over mounted shares
- product-facing rules for which facets may project or mutate which shares

## Glial Does Not Own

Glial does not own:
- replication state convergence
- primary ownership protocol semantics
- projection envelope semantics
- Grip tap or context semantics

Those responsibilities belong elsewhere:
- Glade owns share semantics
- Grip Share owns Grip-to-Glade mapping
- Grip/Grok owns local execution

## Relationship To Glade

Glial uses Glade as its collaboration substrate.

That means:
- Glial decides which workspaces are mounted into one environment
- Glial decides which sessions may attach to which workspaces and shares
- Glial decides which private references may be delegated
- Glial decides which facets may consume the same share across application
  boundaries
- Glade decides how those shares behave mechanically once those choices are
  made

## Working Direction

Glial SHOULD treat data as belonging to mounted workspaces and shares rather
than to one application silo.

Applications are therefore better understood as facets or modules inside one
working environment.

The more detailed working model for this now lives in
`/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialEnvironmentModel.md`.

## Relationship To Agents

Agents should appear to Glial as ordinary attachable participants with policy,
not as a side-channel.

This means Glial should be able to:
- attach a `HeadlessSession`
- grant narrow delegated capabilities
- revoke delegated capabilities
- distinguish human-facing and headless participation for policy purposes

It should not require a separate ad hoc architecture path just because the
participant is an AI agent.

## Product Boundary

The major reason to keep Glial thin is to avoid polluting the share substrate
with product-specific composition concepts.

If a concept only matters because one app wants:
- a certain service mounted
- a certain workflow composed
- a certain participant policy applied

then it likely belongs in Glial rather than Glade.
