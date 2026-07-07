# Glade Declaration Model

Status: working draft

Purpose: define the root declaration language used by Glade. This is the
foundation for p2p-first Glade, where peers need to validate shared behavior
without assuming one central HTTP server is the source of truth.

## Core Claim

Glade is declaration driven.

Participants do not primarily call endpoints. They declare reusable contracts,
instantiate concrete shared objects or work items, claim responsibility, and
observe results.

The root rule is:

- declarations describe desired or allowed shared shape
- instances describe concrete participation
- claims describe current responsibility
- observations describe what was seen
- content records carry data

## Root Concepts

| Concept | Meaning |
| --- | --- |
| `Genesis` | The signed starting declaration set for one environment or workspace. |
| `DeclarationSpace` | The replicated control space where declarations, claims, leases, and observations are exchanged. |
| `Definition` | A reusable contract such as an exchange type, content model, live channel type, or provider capability. |
| `Instance` | One concrete use of a definition, such as one request, one file share, one terminal, or one command log. |
| `Source` | A concrete resource or state producer that may expose multiple flow-typed views, such as a PTY with live bytes and an output log. |
| `Handle` | Stable identity returned or declared for a concrete source instance so commands, live channels, logs, and materialized views bind to the same resource. |
| `Claim` | A statement that a principal currently accepts responsibility for a definition, instance, role, or authority. |
| `Lease` | A time-bounded liveness record attached to a claim or ephemeral instance. |
| `Capability` | A permission-bearing grant that allows a principal to define, instantiate, claim, read, write, host, delegate, or decrypt. |
| `Policy` | A rule set used to validate declarations and claims. |
| `Observation` | A record that some principal or session observed a declaration, publication, response, or content cursor. |
| `Retention` | Rules describing expiry, history, replay, cleanup, and diagnostic retention. |

## Definition And Instance Split

Glade MUST distinguish reusable definitions from concrete instances.

This mirrors the useful part of HTTP:
- endpoint definition: reusable contract
- request: one concrete instance

Glade generalizes this beyond HTTP.

Examples:

| Case | Definition | Instance |
| --- | --- | --- |
| database query | `exchange:BugsQuery` | `req:01J...` |
| file share | `model:MutableFile` | `file:src/app.ts` |
| terminal | `model:TerminalPty` | `pty:session-8842` |
| command output | `model:AppendLog` | `log:run-8842` |
| collaborative document | `model:CrdtText` | `doc:reports/live-edit` |

Definitions SHOULD be durable. Instances MAY be durable, leased, ephemeral, or
completed-and-retained according to policy.

## Source, View, And Handle Binding

Glade MUST distinguish a source from its views.

A `Source` is the concrete thing that produces or owns state. Examples include:
- one PTY process
- one command run
- one repository file
- one chat room
- one collaboration document

A source MAY expose multiple flow-typed views. For example, one terminal source
can expose:
- a live channel for input, resize, close, and hot-path output
- an append log for durable output replay
- a materialized terminal-screen view for UI rendering

When an operation creates a source, the operation MUST return or publish a
stable handle. The handle MUST include enough identity to bind all views and
controls for that source without consulting a stale materialized list.

For provider-hosted sources, the handle SHOULD include:
- `source_id` or source instance id
- `provider_id`
- `provider_instance_id` if runtime attachment matters
- `owner_term`
- `workspace_id`
- `session_id` if participant session identity matters
- target identity, such as `target_ref`, `target_id`, or `repo_path`
- canonical view ids or instance ids for related logs, streams, and channels

Handles are not merely UI convenience. Generated taps, provider helpers, and
reconnect logic SHOULD use handles as request keys and route keys. A materialized
view such as a session list MAY display handles, but it SHOULD NOT be the only
source of truth for routing a just-created resource.

Materialized views MUST be declared as derived from canonical flows or sources.
They MAY cache decoded, aggregated, filtered, or windowed state. They MUST NOT
silently replace canonical logs or live channels when replay, ordering, or byte
integrity matters.

## Genesis

`Genesis` is the trust and declaration root.

Genesis MUST define:
- `genesis_id`
- workspace or environment identity
- root principals or trust anchors
- accepted signature and certificate schemes
- allowed declaration kinds
- allowed definition kinds
- allowed claim kinds
- initial policy version
- default retention policy
- key or capability distribution policy

Genesis MAY also seed durable definitions such as:
- exchange definitions
- content model definitions
- provider roles
- facet contracts

In a p2p-first system, peers MUST be able to validate a declaration against a
genesis reference without asking one central server whether the declaration is
valid.

## DeclarationSpace

A `DeclarationSpace` is the replicated control space for declarations.

It carries:
- definitions
- instances
- claims
- leases
- observations
- diagnostic declarations
- supersession and revocation records

The declaration space MUST be separate from large content payloads. It MAY
contain small inline payloads, but large output, files, blobs, or logs SHOULD
be represented by content references.

## Definition Declaration

A `Definition` declares a reusable contract.

Minimum fields SHOULD include:
- `definition_id`
- `definition_kind`
- `workspace_id`
- `declared_by`
- `schema_ref` or inline schema
- `allowed_instance_kinds`
- `authority_model`
- `retention_default`
- `required_capabilities`
- `policy_ref`

Example:

```ts
declareDefinition({
  definitionId: 'exchange:BugsQuery',
  definitionKind: 'exchange',
  workspaceId: 'workspace:bugs-alpha',
  requestType: 'BugsQueryRequest',
  responseType: 'BugsQueryResponse',
  cardinality: 'many-instances',
  authorityModel: 'provider-claimed',
  retentionDefault: {
    completedTtlMs: 60_000,
    failedTtlMs: 300_000,
  },
  requiredCapabilities: {
    instantiate: ['bugs.query'],
    claim: ['bugs.provider'],
  },
});
```

## Instance Declaration

An `Instance` declares one concrete use of a definition.

Minimum fields SHOULD include:
- `instance_id`
- `definition_id`
- `instance_kind`
- `workspace_id`
- `declared_by`
- `input_ref` or inline input
- `handle_ref` or inline handle if this instance creates or binds a source
- `lifetime`
- `retention`
- `required_capabilities`

`lifetime` SHOULD distinguish at least:
- `durable`
- `leased`
- `ephemeral`
- `completed`
- `expired`

Example:

```ts
declareInstance({
  instanceId: 'req:01JABC...',
  definitionId: 'exchange:BugsQuery',
  instanceKind: 'exchange-instance',
  workspaceId: 'workspace:bugs-alpha',
  input: { status: 'open', limit: 200 },
  replyTo: 'reply:req:01JABC...',
  lifetime: 'ephemeral',
  retention: { completedTtlMs: 60_000 },
});
```

## Claim Declaration

A `Claim` declares current responsibility.

Minimum fields SHOULD include:
- `claim_id`
- `claim_kind`
- `target_ref`
- `claimed_by`
- `owner_term`
- `lease_ref`
- `capability_ref`
- `claim_state`

`claim_kind` SHOULD distinguish:
- `serve-definition`
- `serve-instance`
- `sequence-writes`
- `host-content`
- `host-live-channel`
- `retain-content`
- `relay-transport`

Example:

```ts
claimDefinition({
  definitionId: 'exchange:BugsQuery',
  providerId: 'service:bugs-db',
  claimKind: 'serve-definition',
  ownerTerm: 42,
  lease: { ttlMs: 30_000, renew: true },
});
```

## Declaration Actions

The root declaration API SHOULD expose actions similar to:

- `propose`
- `accept`
- `reject`
- `supersede`
- `revoke`
- `claim`
- `release`
- `observe`
- `delegate`
- `expire`

Ergonomic APIs MAY wrap these root actions, but the root record semantics
SHOULD remain visible for debugging, conformance, and p2p validation.

## Policy Validation

Every declaration MUST be validated against:
- the referenced genesis
- the active policy version
- the declaring principal
- the capability proof
- the declaration kind
- the target workspace
- expiry and revocation state

A peer MUST be able to reject declarations that are well-formed but not
authorized.

## Cleanup

Every instance MUST have explicit retention semantics.

Glade MUST NOT rely on callers remembering to delete ephemeral request nodes.

Retention policy SHOULD cover:
- completed instances
- failed instances
- abandoned leased instances
- diagnostic records
- response payloads
- logs and replay content
- observation records

## Boundary

This model defines the declaration language. It does not define:
- cryptographic details
- exact wire envelope
- libp2p stream protocols
- Grip adapter behavior
- UI APIs

Those belong in companion documents.
