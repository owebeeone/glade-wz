# Glade Distributed Control Plane

Status: working draft

Purpose: describe how a large multi-tenant Glade deployment can provision,
route, monitor, drain, and scale application providers without introducing one
central scheduler.

## Core Claim

Large Glade deployments SHOULD run the control plane as a distributed Glade
application.

There is still a cluster-control problem:
- deciding desired state
- comparing desired state with observed state
- assigning work to nodes
- enforcing quota and resource limits
- publishing routes
- monitoring health
- draining and rolling back services

But there SHOULD NOT be one live global master process that owns all truth.
Instead, authorized controllers SHOULD read and write signed, leased Glade
records in system declaration spaces.

The stable model is:

```text
desired declarations -> observed records -> decision records -> node execution
```

## Non-Goal

This document does not define one scheduler algorithm.

It defines the record families, controller responsibilities, and authority
boundaries needed to make several scheduler implementations possible.

## Control Responsibilities Split

Traditional centralized scheduler responsibilities SHOULD split into separate
controller roles.

| Responsibility | Glade control-plane role |
| --- | --- |
| Desired deployment state | `DeploymentPlan`, `ServiceDefinition`, `EnvironmentOverlay` |
| Start and stop work | `ServiceAllocation` consumed by a node agent |
| Fleet growth and shrink | `FleetController` plus `NodePool` and `NodeClaim` records |
| User and tenant limits | `AdmissionController`, `QuotaPolicy`, `TenantBudget` |
| Class of service | `ServiceClass`, `PriorityPolicy`, `PreemptionPolicy` |
| Memory and CPU limits | `ResourceLimit` enforced by node agents |
| Network priority | `NetworkClass` mapped by infrastructure policy |
| Health monitoring | `HealthController`, `HealthSample`, leased `ServiceInstance` records |
| Load balancing | `PlacementController`, `LoadSample`, `SessionAssignment`, `ResourceRoute` |
| Draining | `DrainIntent`, `DrainState`, assignment withdrawal |
| Rollout and rollback | `RolloutController`, versioned `DeploymentPlan`, rollback records |

These roles MAY run in one process at small scale. They SHOULD be separable at
large scale.

## Record Families

The control plane SHOULD organize state into four record families.

### Desired Records

Desired records describe what should exist.

Examples:
- `PackageDeclaration`
- `ServiceDefinition`
- `ProviderDefinition`
- `DeploymentPlan`
- `NodePool`
- `QuotaPolicy`
- `ServiceClass`
- `NetworkClass`
- `EnvironmentOverlay`
- `IngressDefinition`

Desired records are usually durable and versioned.

### Observed Records

Observed records describe what is actually present now.

Examples:
- `NodeClaim`
- `NodeCapacity`
- `NodeHealth`
- `LoadSample`
- `ServiceInstance`
- `ProviderClaim`
- `HealthSample`
- `ResourceUsageSample`
- `RouteObservation`

Observed records SHOULD be leased or time-bounded. They expire when the node,
provider, or monitor stops renewing them.

### Decision Records

Decision records describe what an authorized controller currently wants an
actor to do.

Examples:
- `ServiceAllocation`
- `ServiceDeallocation`
- `SessionAssignment`
- `ResourceAssignment`
- `ResourceRoute`
- `DrainIntent`
- `PreemptionIntent`
- `RollbackIntent`

Decision records MUST identify the controller partition and owner term that
created them.

### Runtime Execution

Runtime execution is performed by node-local agents and provider processes.

Examples:
- a node agent starts a container or local process
- a provider process starts serving a share surface
- a routing node updates a fast route index
- a relay starts accepting encrypted traffic

Runtime execution MUST be attributable back to the decision records that caused
it.

## Controllers As A Glade Application

Control-plane controllers SHOULD themselves be Glade participants.

They consume records:
- package declarations
- environment overlays
- node claims
- load samples
- health samples
- quota policies
- existing assignments

They publish records:
- allocations
- assignments
- routes
- drain intents
- rollback intents
- diagnostics

This makes the control plane inspectable and replaceable. A self-hosted
deployment can run simple controllers. A large provider can run highly
optimized controllers over the same semantic record families.

## Visibility Application

A Glade deployment SHOULD include a control-plane console as an ordinary Glade
application.

The console SHOULD consume desired, observed, decision, health, lease, route,
and diagnostic records. It SHOULD publish projections for UI use and publish
authorized intent records for operator actions.

The console MUST NOT be a hidden source of truth. It is a visibility and
operations layer over Glade records.

The first development implementation SHOULD include this console even if it is
only a local dev server with in-memory projections. The console is how the
system demonstrates that provisioning and placement behavior is explainable.

## Controller Roles

| Controller | Responsibility |
| --- | --- |
| `AdmissionController` | Accepts or rejects packages, deployments, sessions, and provider claims based on trust, namespace, quota, and policy. |
| `PlacementController` | Chooses which eligible node should run a service instance or own a resource/session. |
| `FleetController` | Grows, shrinks, and classifies the available node pools. |
| `HealthController` | Interprets health samples, lease expiry, failed probes, and readiness state. |
| `RouteController` | Publishes and withdraws routes from assignments and provider claims. |
| `RolloutController` | Performs canaries, rolling updates, drain, rollback, and version transitions. |
| `QuotaController` | Tracks tenant, user, package, service, and class-of-service consumption. |
| `NetworkPolicyController` | Maps declared network classes to provider-specific network behavior. |
| `GarbageCollector` | Removes expired, orphaned, or superseded records and derived indexes. |

Controllers MAY be combined, but their record outputs SHOULD remain distinct.

## Node Agent Contract

The node agent is the node-local execution participant.

It SHOULD:
- start with local bootstrap config and node identity
- validate the genesis bundle
- join the system declaration space
- publish `NodeClaim`, `NodeCapacity`, and `NodeHealth`
- watch `ServiceAllocation` records assigned to it
- start and stop service processes or containers
- enforce resource limits
- provide secret and certificate references to authorized processes
- publish `ServiceInstance` and health records
- withdraw or let leases expire during shutdown

It MUST NOT self-elect to run arbitrary provider work.

A node may be homogeneous at the hardware level, but it still needs an
allocation record before it starts tenant work.

## Placement To Claim Flow

Provider claims are not the first cause.

The expected flow is:

```text
ServiceDefinition declares what can be run.
DeploymentPlan declares what should be running.
NodeClaim declares what a node is eligible to host.
PlacementController selects a node.
ServiceAllocation tells the node to start work.
NodeAgent starts the process or container.
ServiceInstance records that the process exists.
ProviderClaim records that the process is ready to serve definitions.
RouteController publishes ResourceRoute or SessionAssignment.
Clients and peers use p2p transport to reach the selected provider.
```

This prevents a random machine from waking up and saying it serves a tenant's
database query surface.

## Bug Database Example

For a `BugsQuery` service:

```text
ServiceDefinition:
  service:bugs-backend serves exchange:BugsQuery

DeploymentPlan:
  bugs-backend min 3 max 200 placement node_pool general_workers

NodeClaim:
  node Z can_host provider_group bugs_backend

ServiceAllocation:
  run service:bugs-backend instance bugs-17 on node Z

ServiceInstance:
  bugs-17 is running on node Z with peer id peer:12D3...

ProviderClaim:
  bugs-17 serves exchange:BugsQuery for tenant acme, lease 30s

ResourceRoute:
  tenant acme BugsQuery traffic routes to bugs-17, fallback bugs-18
```

The p2p substrate answers how to reach `peer:12D3...`.

The Glade control plane answers why that peer is the correct target.

## Authority And Partitioning

Controllers SHOULD be partitioned.

Useful partition keys include:
- package id
- tenant id
- workspace id
- provider group
- service id
- resource id hash
- region or failure domain

Each partition SHOULD have an owner term or lease. Only the current owner for a
partition SHOULD publish decision records for that partition.

Other controllers MAY observe the same records and be ready to take over if
the owner lease expires.

Decision records SHOULD include:
- controller id
- partition key
- owner term
- decision epoch
- expiry or lease
- policy version used
- source desired-record hashes

This provides a way to resolve split-brain and audit scheduling decisions.

## Health And Leases

Liveness SHOULD be lease-based.

The model SHOULD distinguish:
- node liveness
- service process liveness
- provider readiness
- provider health
- route health
- workload health

A running process is not necessarily ready to serve. A healthy provider is not
necessarily the right provider for a session. Routing MUST depend on assignment
and readiness, not just process existence.

Health checks MAY include:
- explicit readiness probes
- provider claim renewal
- load sample freshness
- route observation
- application-specific diagnostics

If a lease expires, downstream decision and route records MUST be withdrawn,
invalidated, or superseded by the responsible controller.

## Quota And Resource Enforcement

Quota and resource control are admission and node-agent responsibilities.

The declaration model SHOULD support:
- tenant quota
- user quota
- package quota
- service instance limits
- CPU limits
- memory limits
- disk limits
- network bandwidth limits
- live-channel limits
- exchange-rate limits

Node agents MUST enforce hard runtime limits where the host supports it. If a
process exceeds a hard memory limit, the node agent MAY terminate it and MUST
publish diagnostics.

Controllers SHOULD treat quota and limit violations as scheduling inputs.

## Class Of Service And Network Class

Class of service SHOULD be declared separately from transport mechanics.

Examples:
- `best_effort`
- `batch`
- `interactive`
- `latency_critical`
- `bulk_transfer`

Network class SHOULD also be declared as policy, not hard-coded into
application logic.

An environment or provider overlay MAY map network classes to:
- port ranges
- p2p protocol names
- relay pools
- QoS tags
- bandwidth limits
- priority queues

The application can declare that terminal input/output is interactive. The
provider environment decides how that maps onto network infrastructure.

## Routing And P2P

The p2p substrate is the data-plane reachability layer.

It SHOULD NOT decide placement.

Control-plane records decide:
- which provider owns a session
- which provider owns a resource
- which route is preferred
- which fallback routes are allowed
- which relays may carry encrypted traffic

P2P routing then carries traffic to the chosen peer, route, relay, or provider
group.

At large scale, derived route indexes MAY be used. Those indexes MUST be
derived from Glade records and MUST be invalidated when assignments, claims, or
capabilities change.

## Monster-Scale Shape

A large provider deployment MAY look like:

```text
many genesis/seed nodes
replicated system declaration spaces
sharded desired/observed/decision record stores
partitioned control-plane controllers
regional route indexes
node agents on every host
provider fleets per service group
relays and routing nodes without content access
durable logs for audit and replay
compiled indexes for hot-path routing
```

The control plane SHOULD be asynchronous. Ordinary data-plane traffic SHOULD
not synchronously consult the full declaration space on every packet or
request.

Hot paths MAY use:
- route caches
- assignment tables
- provider indexes
- capability validation caches
- regional queues
- append logs

Those hot-path structures MUST remain explainable from declaration and
decision records.

## Self-Hosted And Local Modes

The same model SHOULD collapse for smaller deployments.

In local development:
- one process MAY host genesis, controller, node agent, and provider
- node eligibility MAY be implicit
- placement MAY pick the local node
- route indexes MAY be in memory
- TLS and admission MAY use a local environment overlay

In a small team mesh:
- multiple app managers SHOULD be used
- node agents MAY be ordinary developer machines
- leases and provider claims SHOULD still be real
- routing publications SHOULD still expire

The semantic records SHOULD be the same even when the implementation is
collapsed.

## Relationship To Glade Kernel

The Glade kernel MUST NOT embed a global scheduler.

The kernel SHOULD provide:
- signed records
- declaration validation
- leases and expiry semantics
- capability checks
- replication primitives
- diagnostics

Distributed control-plane applications provide:
- admission logic
- placement logic
- rollout logic
- quota accounting
- route publication
- environment-specific enforcement

This keeps Glade small enough to implement while still allowing large-scale
deployments.

## Open Issues

This document does not yet define:
- controller election protocol
- owner-term format
- exact lease conflict resolution
- scheduling algorithm
- quota accounting model
- network class taxonomy
- cross-region failover behavior
- derived index invalidation protocol
- split-brain repair policy
