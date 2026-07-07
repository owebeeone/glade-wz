# Glade Provider Placement And Work Assignment

Status: working draft

Purpose: define how backend services, provider instances, session affinity, and
work assignment are declared in a p2p-first Glade system.

## Core Claim

Providers are not just endpoints.

A provider is a trusted principal or session that declares what work it can
serve, which work it wants to observe, which sessions it is assigned to, and
how long its current process instance is alive.

Glade MUST distinguish durable service identity from running process lifetime.

## Provider Concepts

| Concept | Meaning |
| --- | --- |
| `ProviderDefinition` | Durable declaration that a provider kind or service capability exists. |
| `ProviderPrincipal` | Durable identity for a trusted backend service or service account. |
| `ProviderInstance` | One running process attachment, usually leased and restartable. |
| `ProviderClaim` | Statement that a provider instance currently serves a definition, scope, session set, or assignment. |
| `ProviderGroup` | A set of providers that can serve the same category of work. |
| `SessionAffinityBinding` | Declaration that a session prefers or requires a specific provider, principal, or group. |
| `AssignmentPolicy` | Rule set for assigning work instances to providers. |
| `WorkAssignment` | Concrete assignment of an instance or session to a provider instance or group. |

## Durable Identity Versus Runtime Instance

Glade MUST distinguish:
- durable provider principal
- durable provider definitions
- ephemeral provider instance
- leased provider claims

When a backend restarts:
- the durable provider principal SHOULD remain the same
- provider definitions SHOULD remain valid
- the old provider instance lease SHOULD expire
- a new provider instance SHOULD attach and claim work according to policy

This allows backend services to survive restarts without redefining the whole
application surface.

## Provider Definition

A `ProviderDefinition` declares what a service can do in principle.

It SHOULD include:
- provider definition id
- supported definition ids
- provider group
- required trust or certificate profile
- supported work assignment policies
- supported affinity modes
- capacity metadata if available
- diagnostics surface

Example:

```ts
declareProviderDefinition({
  providerDefinitionId: 'provider-def:griplab-backend',
  providerGroup: 'provider-group:griplab-backend',
  supports: [
    'exchange:BugsQuery',
    'exchange:RunCommand',
    'model:TerminalPty',
    'model:MutableFile',
  ],
  affinityModes: ['prefer-same-provider', 'must-same-provider'],
  requiredCapability: 'provider.griplab',
});
```

## Provider Instance

A `ProviderInstance` declares that one running process is present.

It SHOULD include:
- provider instance id
- provider principal id
- provider definition id
- transport peer binding if any
- started at
- lease
- advertised capacity
- health diagnostics

Example:

```ts
declareProviderInstance({
  providerInstanceId: 'provider-inst:griplab:boot-17',
  providerPrincipalId: 'service:griplab-backend',
  providerDefinitionId: 'provider-def:griplab-backend',
  peerId: 'peer:12D3...',
  lease: { ttlMs: 30_000, renew: true },
  capacity: { sessions: 20, liveChannels: 8 },
});
```

## Provider Claim

A `ProviderClaim` says what this provider instance currently serves.

It SHOULD include:
- provider instance id
- served definitions
- workspace scope
- session selector
- provider group
- owner term
- lease ref

Provider claims MUST be capability checked.

In provisioned environments, a provider claim SHOULD be downstream of a
`ServiceAllocation` and `ServiceInstance`. The claim proves that assigned work
actually became ready; it SHOULD NOT be treated as the first cause of
placement.

Example:

```ts
claimProvider({
  providerInstanceId: 'provider-inst:griplab:boot-17',
  serves: ['exchange:RunCommand', 'model:TerminalPty'],
  workspaceId: 'workspace:repo-alpha',
  selector: {
    sessions: { assignedToMe: true },
    facets: ['facet:griplab'],
  },
  lease: { ttlMs: 30_000, renew: true },
});
```

## Notification Scope

Providers SHOULD NOT be notified of all request instances by default.

Provider observation MUST be reducible from:
- provider claim
- work assignment
- workspace policy
- instance definition
- session affinity
- provider capability

This is the declarative equivalent of endpoint routing plus load-balancer
assignment.

## Session Affinity

Session affinity SHOULD be explicit.

Affinity modes SHOULD include:
- `prefer-same-provider`
- `must-same-provider`
- `same-provider-until-release`
- `no-affinity`

Example:

```ts
declareSessionAffinity({
  sessionId: 'session:alice-browser',
  providerGroup: 'provider-group:griplab-backend',
  preferredProviderPrincipal: 'service:griplab-backend-a',
  stickiness: 'prefer-same-provider',
  fallback: 'same-group',
  stateRequirement: 'warm-cache',
});
```

The model MUST distinguish cache affinity from correctness affinity.

If the provider has only warm cache, failover MAY be acceptable.
If the provider owns non-replicated session state, failover MUST be explicit
and may require migration.

## Assignment Policy

`AssignmentPolicy` declares how work is assigned to providers.

It SHOULD include:
- provider group
- assignment strategy
- rebalance policy
- failover policy
- drain policy
- capacity inputs
- diagnostics requirements

Example:

```ts
declareAssignmentPolicy({
  providerGroup: 'provider-group:griplab-backend',
  strategy: 'sticky-session',
  fallback: 'least-loaded',
  rebalance: 'only-on-failure',
  drain: 'finish-active-work',
});
```

## Work Assignment

A `WorkAssignment` binds concrete work to a provider or group.

It MAY assign:
- one session
- one exchange instance
- one live channel instance
- one content instance
- one provider-owned workspace subset

Example:

```ts
declareWorkAssignment({
  assignmentId: 'assign:session:alice-browser',
  target: 'session:alice-browser',
  providerGroup: 'provider-group:griplab-backend',
  providerInstanceId: 'provider-inst:griplab:boot-17',
  ownerTerm: 12,
  lease: { ttlMs: 30_000, renew: true },
});
```

## Trust

Provider trust MAY begin with JWT, certificate, or external identity proof, but
Glade records SHOULD bind provider authority to Glial-issued capabilities.

JWTs are useful for bootstrap authentication. They SHOULD NOT be the only
authority embedded in long-lived Glade declarations.

## Relationship To Declarations

Provider placement is part of the declaration model.

Definitions describe reusable contracts.
Instances describe concrete work.
Provider claims describe current ability to serve.
Assignments describe who should serve a specific session or instance.
Leases describe whether current service attachments are alive.

At large scale, distributed control-plane controllers reconcile desired,
observed, and decision records. Provider placement records are therefore one
input and output of the broader control plane, not an independent scheduling
system.

## Open Issues

This document does not yet define:
- exact load-balancing algorithm
- provider capacity measurement
- migration of provider-local session state
- split-brain assignment resolution
- how provider claims are indexed for efficient notification
