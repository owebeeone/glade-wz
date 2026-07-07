# Glade Record Envelope

Status: working draft

Purpose: define the verifiable record shape needed for p2p-first Glade
declarations, claims, observations, and content references.

## Core Claim

Every important Glade control-plane record MUST be self-describing enough that
a peer can validate it without trusting the transport path.

Transport delivery says only that bytes arrived. The record envelope says
whether the bytes are meaningful in a Glade declaration space.

## Envelope Fields

Every declaration-space record SHOULD carry:

- `record_id`
- `record_kind`
- `schema_version`
- `genesis_ref`
- `workspace_id`
- `declaration_space_id`
- `principal_id`
- `session_id` if relevant
- `capability_ref` or embedded proof reference
- `policy_ref`
- `created_at`
- `expires_at` if applicable
- `supersedes_ref` if applicable
- `revokes_ref` if applicable
- `causal_refs` or sequence metadata
- `payload_hash`
- `signature`

The payload SHOULD be validated separately from the envelope. The envelope
binds the payload to identity, policy, and trust context.

## Record Kinds

Initial record kinds SHOULD include:

- `genesis`
- `definition`
- `instance`
- `claim`
- `lease`
- `observation`
- `diagnostic`
- `capability-grant`
- `capability-revocation`
- `content-head`
- `content-chunk-ref`
- `content-invalidation`

## Identity Requirements

`record_id` MUST be stable and unique within the declaration space.

`genesis_ref` MUST identify the genesis bundle or genesis hash against which
the record is validated.

`workspace_id` MUST identify the workspace security domain. It MUST NOT be
inferred only from transport topic or peer connection.

`principal_id` MUST identify the actor responsible for the declaration. It
MUST NOT be replaced by transport peer id unless a separate binding exists.

## Signature And Hashing

The signature SHOULD cover:
- canonical envelope fields
- canonical payload hash
- genesis reference
- policy reference
- capability reference

Canonical serialization MUST be defined before cross-language implementations
are considered compatible.

The initial architecture SHOULD plan for JS/TS and Python conformance tests
that verify:
- canonical encoding
- hash calculation
- signature verification
- schema validation
- rejection of tampered records

## Capability Binding

A record that exercises authority MUST bind to a capability.

Examples:
- defining a new exchange definition
- instantiating a request
- claiming provider authority
- writing a content head
- publishing invalidation
- granting delegated access

The envelope MUST make it possible to decide whether the referenced capability
permits the record.

## Sequencing And Causality

Some record kinds need ordering.

Glade SHOULD avoid pretending all records have one global order.

Initial ordering scopes SHOULD include:
- per declaration space
- per definition
- per instance
- per source handle
- per content head
- per live channel
- per authority owner term

Records that depend on previous state SHOULD carry either:
- `base_ref`
- `owner_term`
- `generation`
- `seq`
- explicit causal references

The required fields depend on the record kind.

Append-log definitions MUST declare their cursor semantics. A cursor MAY be a
provider sequence, byte offset, content id, causal head, or substrate-native
entry id, but consumers MUST NOT infer cursor meaning from transport delivery
order. If a log is derived from a live stream, the log record SHOULD bind to the
source handle and owner term that produced the bytes.

## Expiry, Supersession, And Revocation

The envelope MUST support:
- expiry for leases and ephemeral instances
- supersession for updated definitions or policies
- revocation for capability grants and delegated rights

Peers MUST define how they treat records that are:
- expired
- superseded
- revoked
- valid but no longer current
- malformed
- well-formed but unauthorized

## Metadata Leakage

Envelope fields can leak information even when payloads are encrypted.

The design SHOULD decide which fields are visible to relay-only peers.

Potential mitigations include:
- opaque workspace and share ids
- encrypted payloads
- encrypted or blinded topic names
- minimizing human-readable operation names on transport paths

This document does not solve privacy policy, but it requires the record format
to acknowledge metadata exposure as an architectural concern.
