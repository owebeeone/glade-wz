# Glial Application Manager

Status: working draft

Purpose: define the Glial application that manages application backend
provisioning, session allocation, provider placement, rollback, and routing
publication.

## Core Claim

The application manager is itself an application.

It runs with elevated capabilities, observes system and application
declarations, and produces provisioning, provider, session allocation, and
routing declarations.

Genesis authorizes the application manager. Genesis should not become the
long-running application orchestrator.

## Responsibilities

The application manager owns:
- evaluating application definition collections
- evaluating deployment plans
- starting backend services through provisioning backends
- assigning sessions to provider groups or service instances
- publishing provider placement declarations
- publishing routing declarations
- draining services
- rolling back failed deployments
- reporting provisioning diagnostics

The application manager does not own:
- genesis trust roots
- base declaration validation
- Glade record envelope semantics
- transport mechanics
- application-specific business logic

## Inputs

The application manager observes:
- genesis-approved system declaration spaces
- application definition collections
- service definitions
- deployment plans
- provider definitions
- provider instance health
- session attachment declarations
- workspace policy
- capacity and diagnostics records

## Outputs

The application manager may declare:
- `ServiceInstance`
- `ProviderInstance`
- `ProviderClaim`
- `SessionAllocation`
- `WorkAssignment`
- `RoutingPublication`
- `DeploymentStatus`
- `DrainIntent`
- `RollbackIntent`
- diagnostics

Every output MUST be capability checked.

## Boot Flow

Expected flow:

```text
Genesis Glade node exposes system declaration space
Application manager receives provisioning capability
Application manager observes application and service definitions
Application manager starts required backend services
Backends attach and publish provider instances
Application manager assigns sessions and work
Routing nodes observe routing publications
Clients discover routes and participate over p2p
```

## Session Allocation

When a client session joins an environment, the application manager SHOULD
evaluate:
- workspace membership
- application bundle requirements
- existing provider instances
- session affinity policy
- provider capacity
- fallback policy

It then SHOULD publish `SessionAllocation` and `WorkAssignment` declarations.

Session allocation MAY be sticky, but the stickiness mode MUST be explicit.

## Backend Provisioning

The application manager SHOULD use provisioning backends through declarations,
not through hidden imperative state.

It MAY ask a provisioning backend to start:
- a container
- a local process
- a cloud service
- a p2p service worker

Once started, the backend SHOULD publish service and provider instance
declarations with leases.

## Rollback And Drain

Rollback and drain SHOULD be declaration-driven.

Drain should declare:
- target service instance
- reason
- deadline
- allowed active work to finish
- new assignment policy

Rollback should declare:
- failed version
- target previous version
- policy trigger
- migration or restart behavior

The application manager SHOULD preserve diagnostics explaining why a rollback
or drain occurred.

## Routing Nodes

Glial routing nodes observe routing publications.

They SHOULD NOT independently invent provider placement. They route according
to validated declarations from the application manager, provider claims, and
workspace policy.

Routing nodes MAY cache routing publications, but expiry and revocation MUST be
honored.

## Relationship To Glade

The application manager does not bypass Glade. It is a privileged producer and
consumer of Glade declarations.

It uses Glade to:
- observe desired state
- declare actual state
- publish routing and assignment state
- expose diagnostics

## Relationship To Glial

The application manager is part of Glial's environment and policy layer.

It understands:
- application bundles
- facets
- sessions
- workspaces
- capability policy
- operational policy

It should not need to understand Grip internals.

## Open Issues

This document does not yet define:
- exact application manager election model
- whether multiple app managers can coordinate one environment
- how app manager failure is detected
- how manager state is reconstructed after restart
- how cost/quota policy affects provisioning
