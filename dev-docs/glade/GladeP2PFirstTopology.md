# Glade P2P-First Topology

Status: working draft

Purpose: define the first real Glade topology as p2p-first. HTTP and
client/server modes may still exist as test harnesses, but GripLab requires the
architecture to work without treating one HTTP server as the only authority.

## Core Claim

Glade v1 MUST be p2p-first.

That does not mean every hard p2p feature must be implemented immediately. It
means the core model MUST NOT depend on central server authority for validity.

The first implementation SHOULD still be narrow:
- one genesis bundle
- one workspace
- a small set of peers
- one declaration space
- one terminal/live-channel case
- one exchange case
- one mutable file case

## Roles

| Role | Meaning |
| --- | --- |
| `Genesis Distributor` | Any source that can provide the signed genesis bundle and required capability material. |
| `Peer` | A transport participant, typically with a libp2p `PeerId`. |
| `Principal` | An authorization identity. This MUST NOT be assumed to equal the transport peer id. |
| `Provider` | A principal or session that claims responsibility for serving definitions or instances. |
| `Relay` | A peer that carries traffic without necessarily being able to decrypt content. |
| `Replica` | A peer-local materialization of declaration or content state. |

## Identity Split

Glade MUST distinguish:
- transport identity
- principal identity
- session identity
- provider identity
- workspace identity
- share identity

In libp2p deployments, `PeerId` is transport identity. It MAY be bound to a
principal by certificate or capability proof, but it MUST NOT be treated as the
authorization identity by default.

## Join Flow

A peer joins by obtaining:
- a genesis bundle
- bootstrap peer information
- capability material for the workspaces or shares it may access
- transport credentials if separate from principal credentials

The peer then:
1. validates the genesis bundle
2. connects to bootstrap peers
3. subscribes to or requests the relevant declaration space
4. validates replicated declarations locally
5. opens only shares and content models authorized by its capabilities

## Declaration Replication

The declaration space SHOULD replicate over p2p transport.

Replication MUST preserve:
- record identity
- signatures or proof references
- policy version references
- causal or sequence metadata needed for validation
- expiry and revocation metadata

Replication MAY use libp2p streams, pubsub, delegated routing, or a custom
protocol. The transport choice MUST NOT change declaration validity.

## Provider Registration

A provider participates by declaring or claiming definitions.

Example:
- service declares it can serve `exchange:BugsQuery`
- file host claims `model:MutableFile` instances under a workspace
- terminal host claims `model:TerminalPty` instances

Provider claims MUST be:
- signed or otherwise proof-bearing
- capability checked
- lease bounded
- attributable to a principal

In a provisioned deployment, a provider claim SHOULD also reference the
`ServiceAllocation` or `ServiceInstance` that caused the provider to exist.
The p2p mesh may carry the claim, but the claim does not by itself decide
where tenant work should run.

## Work Routing

Work routing is declaration-driven.

Clients instantiate work by writing instance declarations. Providers observe
matching instances and claim work according to policy.

The system SHOULD support many ephemeral instances for one durable definition,
such as many simultaneous `BugsQuery` request instances.

## Live Channels

Live channels SHOULD use direct streams where possible.

For terminal-like cases:
- declarations establish identity, authority, and diagnostics
- libp2p streams carry hot-path bytes
- optional append logs provide replay and reattach

Live stream transport MUST be treated as separate from replicated declaration
records.

## Relays And Content Privacy

A relay MAY carry encrypted traffic without content capability.

Therefore:
- routing participation MUST NOT imply read access
- content SHOULD be encrypted at workspace or share level
- metadata exposure MUST be explicitly accepted or mitigated

## HTTP Role

HTTP MAY be used for:
- local API conformance tests
- debugging tools
- bridge or fallback deployments
- bootstrap distribution

HTTP MUST NOT define the core v1 authority model.

## First Implementation Shape

The first p2p implementation SHOULD prove:
- signed genesis loading
- declaration replication between peers
- provider claim validation
- exchange instance lifecycle
- terminal live stream plus log sidecar
- mutable file instance with authoritative patches

This is enough to validate the model without solving full mesh generality.
