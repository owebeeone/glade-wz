# Glade Scale Modes

Status: working draft

Purpose: define how the same Glade/Glial declaration model spans from a small
development setup to a large, high-throughput service with auto-provisioning
and few single points of failure.

## Core Claim

Declarations are the source of truth. They are not necessarily the hot path.

The same declaration model SHOULD apply at every scale, but the runtime
implementation MAY collapse components together at small scale and split them
apart at large scale.

The API and semantics SHOULD remain stable while the operational topology
changes.

## Planes

The system SHOULD distinguish these planes:

| Plane | Purpose |
| --- | --- |
| `trust plane` | genesis, policy, capability grants, revocation |
| `semantic control plane` | definitions, instances, claims, leases, observations |
| `operational control plane` | provisioning, placement, routing, scaling, rollout, rollback |
| `data plane` | live streams, logs, chunks, blobs, file patches, CRDT ops |

At small scale, one process MAY host multiple planes.
At large scale, these planes SHOULD be separately scalable.

## Scale Mode 0: In-Process Development

Mode 0 is the smallest useful local mode.

Expected shape:
- one process
- in-memory declaration space
- fake or loopback p2p
- optional local provider
- optional local Grip runtime
- optional local control-plane console

Acceptable tradeoffs:
- single point of failure is acceptable
- durability may be limited
- declarations may be seeded from local config
- app manager may run in-process

Non-negotiable:
- semantics SHOULD still be declaration-shaped
- promotion to Mode 1 SHOULD NOT require UI rewrites

## Scale Mode 1: Small Dev Mesh

Mode 1 is a real p2p mesh for development and early team use.

Expected shape:
- one or more genesis nodes
- one workspace or small number of workspaces
- multiple app managers
- several cooperating peers
- one or more provider backends
- one control-plane console
- direct or relayed p2p where available

Multiple app managers SHOULD be used even in this mode. They are a useful
partitioning tool and should be exercised early.

App managers MAY partition by:
- application bundle
- workspace
- provider group
- session set
- development node

Acceptable tradeoffs:
- some control paths may still be simple
- app manager coordination may be coarse
- load balancing may be minimal
- failover may be manual or lease-based

Non-negotiable:
- app manager outputs MUST be declarations
- provider claims MUST be leased
- session affinity MUST be explicit
- routing publications MUST expire or be revocable

## Scale Mode 2: Team Service

Mode 2 supports a team or small organization.

Expected shape:
- replicated system declaration state
- multiple app managers with explicit ownership or partitioning
- provider groups
- session affinity and assignment policy
- routing nodes
- persistent logs and content storage

At this scale, the system SHOULD introduce:
- assignment policy
- service drain
- basic rollback
- provider capacity reporting
- diagnostics for stale leases and failed claims

Single points of failure SHOULD be reduced for:
- routing
- provider instances
- app manager partitions
- content storage

## Scale Mode 3: Managed Production

Mode 3 is a production managed service.

Expected shape:
- replicated genesis/system declaration spaces
- multiple app managers with defined coordination
- provisioning backends
- rolling deploys
- service autoscaling
- regional or zone-aware routing
- indexed declaration views
- durable content and replay storage

At this scale, declarations remain authoritative, but runtime systems SHOULD
use compiled views such as:
- provider routing indexes
- assignment tables
- capability validation caches
- content availability indexes
- stream routing tables

The compiled views MUST be derived from declarations and MUST be invalidated or
updated when relevant declarations change.

## Scale Mode 4: Large High-QPS Service

Mode 4 targets large production scale, including very high request volume.

Expected shape:
- sharded declaration spaces
- regional routing and provider fleets
- replicated trust and policy material
- partitioned app managers
- automated provisioning and rollback
- durable event/log infrastructure
- no single traffic-path SPOF for ordinary operation

At this scale, declaration spaces SHOULD NOT sit synchronously in the hot path
for every request.

Instead:
- durable definitions live in declaration spaces
- claims, routes, and assignments are compiled into fast indexes
- exchange instances may be backed by regional queues or logs
- live streams use direct or regional data-plane routing
- content transfer uses chunk/blob infrastructure
- app managers reconcile desired and actual state asynchronously
- distributed control-plane controllers partition admission, placement, route,
  rollout, health, quota, and garbage-collection work

The system MUST preserve a way to audit runtime behavior back to declarations.

## What Must Stay Stable

Across all modes, these semantics SHOULD NOT change:
- declaration validation
- capability checks
- definition and instance identity
- provider claim semantics
- session affinity semantics
- retention semantics
- content model semantics
- observation and diagnostics meaning

## What May Change By Scale

These implementation details MAY change:
- transport backend
- storage backend
- routing index implementation
- provider placement algorithm
- app manager partitioning strategy
- content replication strategy
- queue/log infrastructure
- cache strategy

Changing those details MUST NOT change the declared meaning of the system.

## SPOF Policy

Single points of failure are acceptable only when the scale mode explicitly
allows them.

Mode 0 MAY have many SPOFs.
Mode 1 MAY have limited SPOFs but SHOULD exercise multiple app managers.
Mode 2 SHOULD reduce SPOFs for common team workflows.
Mode 3 SHOULD avoid SPOFs for production-critical flows.
Mode 4 MUST avoid single traffic-path SPOFs for ordinary operation.

The design SHOULD distinguish:
- bootstrap SPOF
- control-plane SPOF
- data-plane SPOF
- routing SPOF
- provider SPOF
- storage SPOF

## P2P Meaning By Scale

`p2p-first` does not mean all peers do all jobs.

It means:
- peers validate declarations where possible
- transport identity is distinct from authorization identity
- direct peer communication is supported where useful
- relay-only peers can exist without content access
- the architecture does not require one central HTTP authority

At large scale, p2p may be implemented by managed provider fleets and routing
nodes as much as by ad hoc user devices.

## Development Implication

The first executable slice SHOULD target Mode 1, not Mode 0 alone.

That means:
- at least two app manager partitions
- at least one provider backend
- at least one browser/client session
- one rudimentary control-plane console
- one genesis/system declaration space
- one terminal live channel
- one command log
- one mutable file model

Mode 0 remains valuable for tests and local iteration, but Mode 1 is the first
architecture-valid target for GripLab.

## Open Issues

This document does not define:
- exact app manager partitioning algorithm
- sharding model for declaration spaces
- regional routing model
- high-QPS exchange queue implementation
- cross-region trust replication
- cost and quota controls
- distributed control-plane election and owner-term protocol
