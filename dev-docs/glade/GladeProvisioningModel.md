# Glade Provisioning Model

Status: working draft

Purpose: define the Glade declaration surface for provisioning backend
services, publishing routing state, and connecting dynamically allocated
providers to sessions and work.

## Core Claim

Backend provisioning configuration SHOULD become Glade declarations.

The same information that would normally live in deployment manifests, service
registries, routing tables, and backend config should be represented as
validated declarations inside the system declaration space.

Environment-specific admission, ingress, and TLS configuration SHOULD also be
represented as declarations, but SHOULD live in swappable environment imports
or provider-owned overlays rather than in application, workspace, or content
model declarations.

Glade does not run containers or allocate infrastructure by itself. Glade
provides the declaration language that an authorized application manager uses
to provision, scale, drain, roll back, and publish routing information.

## Relationship To Bootstrap

Genesis and bootstrap are responsible for:
- validating the root declaration space
- authorizing the application manager
- exposing system declarations
- seeding initial routing information

Genesis SHOULD NOT become the long-running application orchestrator.

After bootstrap, provisioning work SHOULD be performed by authorized
application manager sessions or providers.

## Provisioning Concepts

| Concept | Meaning |
| --- | --- |
| `ProvisioningBackend` | A trusted service capable of starting, stopping, and inspecting provider backends. |
| `ProvisioningConsole` | Visibility and operations application that exposes provisioning projections and publishes authorized intent records. |
| `ServiceDefinition` | Durable declaration of a deployable backend service. |
| `ServiceInstance` | One running backend process, container, VM, local process, or equivalent. |
| `DeploymentPlan` | Desired deployment state for one service or service group. |
| `SessionAllocation` | Assignment of a session to a service instance, provider instance, or provider group. |
| `RoutingPublication` | Declaration that tells routing nodes where a service, provider, share, or session can currently be reached. |
| `RollbackPolicy` | Rules for replacing a bad deployment with a previous good version. |
| `ProvisioningCapability` | Capability allowing a principal to create, update, or remove provisioning declarations. |
| `EnvironmentOverlay` | Deployment-specific declarations for admission, ingress, TLS references, and provider policy. |
| `NamespaceAdmission` | Provider or self-hosted policy deciding which package namespace claims are accepted. |
| `IngressDefinition` | Listener, join-routing, and TLS-reference configuration for incoming connections. |

## Service Definition

A `ServiceDefinition` describes something that can be started.

It SHOULD include:
- service id
- version
- image or executable reference
- runtime type
- exposed provider definitions
- required capabilities
- required secrets or key refs
- workspace access requirements
- health check
- capacity model
- session affinity policy
- scaling limits
- rollback policy

Example:

```ts
declareServiceDefinition({
  serviceId: 'service:griplab-backend',
  version: '0.1.0',
  runtime: {
    kind: 'container',
    image: 'ghcr.io/example/griplab-backend:0.1.0',
  },
  provides: ['provider-def:griplab-backend'],
  requires: ['provider.griplab', 'workspace.mount'],
  healthCheck: { kind: 'provider-claim', timeoutMs: 10_000 },
  sessionAffinity: 'prefer-same-provider',
  scaling: { min: 1, max: 10, targetSessionsPerInstance: 20 },
  rollback: { strategy: 'previous-good-version' },
});
```

## Service Instance

A `ServiceInstance` declares that one backend service instance is running.

It SHOULD include:
- service instance id
- service definition id
- service version
- provider instance ids
- provisioning backend id
- transport peer id if applicable
- started at
- lease
- health state
- routing refs

Example:

```ts
declareServiceInstance({
  serviceInstanceId: 'svc-inst:griplab:001',
  serviceId: 'service:griplab-backend',
  version: '0.1.0',
  providerInstances: ['provider-inst:griplab:boot-17'],
  provisionedBy: 'app-manager:main',
  peerId: 'peer:12D3...',
  lease: { ttlMs: 30_000, renew: true },
});
```

## Deployment Plan

A `DeploymentPlan` declares desired state.

It SHOULD include:
- target service
- desired version
- min and max instances
- scaling policy
- drain policy
- rollout policy
- rollback policy
- placement constraints
- session affinity constraints

Example:

```ts
declareDeploymentPlan({
  deploymentPlanId: 'deploy:griplab-backend:prod',
  serviceId: 'service:griplab-backend',
  desiredVersion: '0.1.0',
  instances: { min: 1, max: 10 },
  rollout: { strategy: 'rolling', maxUnavailable: 1 },
  drain: { strategy: 'finish-active-work', timeoutMs: 60_000 },
  rollback: { strategy: 'previous-good-version' },
});
```

## Session Allocation

`SessionAllocation` binds a session to a provider group or service instance.

It SHOULD include:
- session id
- workspace id
- application bundle id if relevant
- provider group
- preferred service instance
- fallback policy
- affinity mode
- lease

Example:

```ts
declareSessionAllocation({
  sessionId: 'session:alice-browser',
  workspaceId: 'workspace:repo-alpha',
  applicationBundleId: 'app:griplab',
  providerGroup: 'provider-group:griplab-backend',
  serviceInstanceId: 'svc-inst:griplab:001',
  affinity: 'prefer-same-provider',
  fallback: 'same-provider-group',
  lease: { ttlMs: 30_000, renew: true },
});
```

## Routing Publication

`RoutingPublication` shares routing state with Glial routing nodes and peers.

It SHOULD include:
- routing publication id
- target kind
- target id
- workspace id if scoped
- provider or service instance id
- transport addresses or peer refs
- route priority
- route capability requirements
- expiry

Example:

```ts
declareRoutingPublication({
  routingPublicationId: 'route:svc-inst:griplab:001',
  targetKind: 'provider-group',
  targetId: 'provider-group:griplab-backend',
  workspaceId: 'workspace:repo-alpha',
  serviceInstanceId: 'svc-inst:griplab:001',
  peerId: 'peer:12D3...',
  priority: 100,
  expiresInMs: 30_000,
});
```

## Provisioning Backend

A `ProvisioningBackend` is a trusted executor for deployment operations.

It MAY be implemented by:
- Docker
- local process runner
- Kubernetes-like orchestrator
- cloud function manager
- custom p2p service launcher

The declaration model SHOULD NOT require one specific infrastructure backend.

## Environment Overlay

An `EnvironmentOverlay` declares deployment-specific behavior that is not part
of the application contract.

It MAY include:
- accepted package ids or namespace patterns
- namespace proof requirements
- public or LAN ingress listeners
- TLS certificate references
- trust anchor references
- join routing defaults
- provider-managed route publications
- development defaults for local teams

It MUST NOT include:
- inline TLS private keys
- application behavior changes
- workspace data model changes
- exchange contract changes
- facet contract changes

Hosted multi-tenant providers SHOULD require namespace proof before accepting
globally meaningful package ids. Self-hosted and LAN deployments MAY use a
wildcard local namespace policy when the operator controls the trust boundary.

Example provisional DSL:

```glade
environment local_dev {
  id "env:local-dev"
  applies package griplab

  admission allow package "pkg:griplab"
  admission allow namespace "pkg:local:*"

  ingress lan_https {
    listen "0.0.0.0:8443"
    accepts join for package griplab

    tls local_team {
      mode local_ca
      certificate ref "secret:local-dev/tls-cert"
      private_key ref "secret:local-dev/tls-key"
      trust_anchor ref "secret:local-dev/root-ca"
    }
  }
}
```

This example is not final syntax. It marks where the configuration belongs so
application declarations do not need to absorb TLS, ingress, or provider
admission details.

## Lifecycle

Provisioning lifecycle SHOULD look like:

```text
ServiceDefinition exists
DeploymentPlan requests desired state
ApplicationManager evaluates plan
ProvisioningBackend starts ServiceInstance
ServiceInstance publishes ProviderInstance and ProviderClaim
ApplicationManager publishes SessionAllocation and WorkAssignment
RoutingPublication tells routing nodes where to send traffic
Health and lease records keep the state current
Drain or rollback declarations replace unhealthy versions
```

## Client Bootstrap Flow

A client joining through a genesis Glade node SHOULD receive enough validated
state to continue through the p2p substrate.

Expected flow:

```text
Client contacts genesis Glade node
Genesis provides bootstrap material and system declaration refs
Client validates genesis and opens the system declaration space
Client discovers application definitions, workspace capabilities, and routing publications
Client registers interest in the application and workspace data it is allowed to see
Application manager observes the session and publishes allocation or assignment state
Routing nodes and provider peers observe routing publications
Client starts direct or relayed p2p participation where possible
```

Genesis participates in the start of this flow. It SHOULD NOT remain the
center of all application traffic once routing and provider declarations are
available.

## Permissions

Provisioning declarations MUST be capability checked.

Separate rights SHOULD exist for:
- defining services
- deploying services
- scaling services
- draining services
- rolling back services
- publishing routes
- assigning sessions
- granting provider capabilities
- reading provisioning projections
- creating operator intents

## Provisioning Console

A provisioning console SHOULD exist from the first development implementation.

It SHOULD expose:
- desired service state
- observed service instances
- service allocations
- node claims and capacity
- provider claims
- routing publications
- lease and health state
- diagnostics and decision history

It MAY initially be a local dev server that watches an in-memory or file-backed
Glade record store.

It MUST still model its outputs as projections over Glade records. Operator
actions MUST publish intent records such as `RequestReconcile`, `DrainIntent`,
`RollbackIntent`, or `RouteRefreshIntent`.

The console MUST NOT directly mutate hidden provisioning state.

## Relationship To Provider Placement

Provider placement describes who serves work.

Provisioning describes how those providers come into existence and how routing
state is published.

These are related but distinct declaration families.

## Relationship To Distributed Control Plane

Provisioning records are one input to the distributed control plane.

At larger scales, authorized controllers SHOULD reconcile:
- desired deployment records
- observed node and provider records
- quota and admission policy
- health and load samples
- route publications

The controllers then publish decision records such as `ServiceAllocation`,
`SessionAssignment`, `ResourceRoute`, `DrainIntent`, or `RollbackIntent`.

Those decision records are consumed by node agents, providers, and routing
nodes. The Glade kernel supplies record validation, leases, capability checks,
and replication; it does not become a global scheduler.

## Open Issues

This document does not yet define:
- exact service health model
- secret distribution
- container image trust
- route publication privacy
- multi-provisioner conflict resolution
- cost and quota policy
