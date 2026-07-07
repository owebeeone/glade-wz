# Glade Instance Lifecycle

Status: working draft

Purpose: define how concrete Glade instances are created, renewed, claimed,
completed, expired, retained, and cleaned up.

## Core Claim

Definitions are durable contracts. Instances are concrete uses of those
contracts.

Instances MUST have explicit lifecycle and retention semantics. This is
especially important for request-like exchanges where thousands of short-lived
instances may exist for one reusable definition.

## Instance Classes

Initial instance classes SHOULD include:

| Class | Example | Typical Lifetime |
| --- | --- | --- |
| `exchange-instance` | one `BugsQuery` request | ephemeral, then retained briefly |
| `content-instance` | one shared file | durable |
| `log-instance` | one command output log | durable or retained |
| `live-channel-instance` | one terminal session | leased, with optional retained log |
| `crdt-instance` | one collaborative text document | durable |

## Lifecycle States

Instances SHOULD distinguish:

- `declared`
- `claimable`
- `claimed`
- `active`
- `completed`
- `failed`
- `cancelled`
- `expired`
- `retained`
- `collected`

Not every instance kind needs every state, but each kind MUST define which
states are valid.

## Creation

An instance is created by an `Instance` declaration.

Creation MUST validate:
- referenced definition exists and is current
- declaring principal has instantiate capability
- workspace policy allows this instance kind
- requested lifetime and retention are allowed
- payload or content refs match the declared schema

## Claiming

Instances that require side-effectful work SHOULD be claimed before execution.

Claiming MUST validate:
- provider has claim capability
- claim kind matches the definition
- claim owner term is current
- conflicting claims are resolved by policy
- lease is active if required

Examples:
- a database provider claims an exchange instance
- a terminal host claims a live channel instance
- a file authority claims write sequencing for a mutable content instance

## Renewal

Leased instances and claims MUST be renewed before expiry.

If a lease expires:
- the claim SHOULD become stale
- the instance MAY become claimable again
- diagnostics SHOULD record uncertainty if side effects may have occurred

## Completion

Completion means the responsible provider has declared no more work for the
instance under the current attempt or owner term.

Completion MUST NOT imply:
- requester observation
- content garbage collection
- absence of side effects from stale attempts

Completion records SHOULD reference:
- final output or result refs
- owner term
- generation
- final sequence
- diagnostics if any uncertainty remains

## Expiry And Retention

Expiry decides whether an instance is still active or claimable.

Retention decides how long records and content survive after the active
lifecycle ends.

Retention policy SHOULD cover:
- declaration records
- claim and lease records
- progress records
- output or response content
- diagnostic records
- observation records

## Cleanup

Cleanup MUST be policy-driven.

Peers SHOULD NOT delete records merely because they are locally inconvenient.
They SHOULD collect records only when policy says the instance or related
content is collectible.

In p2p mode, cleanup may be local, partial, or delayed. Therefore content
availability and record retention MUST be separate concerns.

## One-Off Files

A file share is typically a durable content instance:

- definition: `model:MutableFile`
- instance: `file:src/app.ts`
- lifecycle: durable
- retention: persist until explicit deletion
- updates: content heads, patches, invalidations

The file instance may still have leased write authority or provider claims.

## Many Ephemeral Requests

An exchange definition may create many ephemeral instances:

- definition: `exchange:BugsQuery`
- instances: `req:...`
- lifecycle: declared, claimed, active, completed, retained, collected
- retention: short TTL after completion

This is how Glade models repeated endpoint-like use without pretending each
request is a permanent share.

## Streams And Terminals

A terminal is typically a leased live-channel instance with an optional log:

- live instance: `pty:session-8842`
- log instance: `log:pty:session-8842`
- live lifecycle: leased and active until closed or expired
- log lifecycle: retained according to workspace policy

The hot path is the live channel. Replay and audit use the log sidecar.

## Failure Semantics

Lifecycle records MUST preserve uncertainty.

Examples:
- provider lease expired after starting work
- result was published but requester did not observe it
- live channel ended due to transport loss
- content head changed but some chunks are unavailable

These states SHOULD produce diagnostics rather than collapsing into a single
`failed` status.
