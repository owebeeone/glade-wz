# Glade Exchange Semantics

Status: working draft

Purpose: define the bounded-work exchange pattern that runs on top of the Glade
share substrate without collapsing back into a hidden RPC model.

## Core Claim

Request/reply remains a valid mental model, but it is not a Glade primitive.

Glade does not provide:
- send-request
- await-response
- transport-level reply routing

Glade provides share primitives that allow a request/reply exchange pattern to
be represented as shared state.

This matters because a real Glade exchange must model:
- substrate replication state
- ownership and claiming
- provider execution
- publication of results
- requester observation
- diagnostics from Glade, provider, or mesh instability

A single request document with one lifecycle field is too weak for that.

## 1. Exchange Pattern

An exchange is a bounded unit of work represented by an exchange bundle inside
Glade shared state.

Current working model:

- a requester writes intent into a provider-visible share
- a provider or provider group observes that intent
- one provider claims one attempt under an ownership term
- the provider publishes zero or more response events or artifacts
- the requester observes those results through ordinary share replication

The exchange pattern is therefore:
- built from Glade share primitives
- observable
- replayable
- diagnosable
- tolerant of partial uncertainty

It is not a hidden side-channel transport abstraction.

## 2. Exchange Bundle

The exchange bundle is keyed by one stable `request_id`.

One exchange bundle MAY contain:
- one `RequestIntent`
- zero or more `RequestAttempt` records
- zero or more `ClaimRecord` values
- zero or more `ProviderProgress` events
- zero or more `PublicationRecord` values
- zero or more `ObservationRecord` values
- zero or more `DiagnosticEvent` values

This bundle is a logical grouping, not necessarily one serialized object.

## 3. Orthogonal State Planes

Glade MUST treat the following planes separately.

### 3.1 Intent Plane

Intent answers:
- what work was requested
- against which target or content reference
- with which declared parameters
- with which reply destination

Intent does not answer:
- whether any provider has seen it
- whether any provider has completed it
- whether any requester has observed a result

### 3.2 Substrate Plane

Substrate state answers:
- whether the exchange bundle is durably present in Glade shared state
- whether the relevant provider-visible share is stable
- whether the reply destination is currently writable or readable

Substrate state does not answer:
- whether provider execution has succeeded

### 3.3 Ownership Plane

Ownership state answers:
- which provider claimed which attempt
- under which `owner_term`
- whether the claim is current, stale, released, or uncertain

Ownership state does not answer:
- whether provider work completed
- whether published results were observed

### 3.4 Execution Plane

Execution state answers:
- whether a provider started work
- whether it emitted progress
- whether it completed, failed, or was interrupted

Execution state does not answer:
- whether the result publication succeeded
- whether the requester has seen the result

### 3.5 Publication Plane

Publication state answers:
- what response events or result references were written
- in what sequence
- under which attempt and owner term
- in which generation

Publication state does not answer:
- whether the requester has confirmed receipt

### 3.6 Observation Plane

Observation state answers:
- what the requester has definitely observed
- which response sequence or artifact version is locally confirmed

Observation state is especially important under disconnect, reconnect, and
resync. A published result is not the same as an observed result.

### 3.7 Diagnostics Plane

Diagnostics answer:
- whether Glade substrate instability affected the exchange
- whether provider-local errors occurred
- whether upstream source instability occurred
- whether outcomes are certain or uncertain

Diagnostics do not replace the other planes. They annotate risk, ambiguity, and
operational state.

## 4. Exchange Records

The exchange pattern SHOULD use record types with distinct responsibilities.

### 4.1 `RequestIntent`

`RequestIntent` describes the logical request.

Minimum fields SHOULD include:
- `request_id`
- `operation`
- `target_ref`
- `input`
- `reply_to`
- `requester_ref`
- `created_at`
- `ttl` or expiry policy

`RequestIntent` SHOULD be immutable after creation except for explicitly
allowed fields such as cancellation.

### 4.2 `RequestAttempt`

`RequestAttempt` describes one execution try for a request.

Minimum fields SHOULD include:
- `request_id`
- `attempt_id`
- `created_at`
- `attempt_reason`
- `supersedes_attempt_id` if this is a retry or takeover attempt

Retry SHOULD create a new `attempt_id`, not mutate the identity of the
original request.

### 4.3 `ClaimRecord`

`ClaimRecord` describes ownership of an attempt.

Minimum fields SHOULD include:
- `request_id`
- `attempt_id`
- `provider_id`
- `owner_term`
- `claim_state`
- `claimed_at`

`claim_state` SHOULD distinguish at least:
- `claimed`
- `released`
- `stale`
- `uncertain`

### 4.4 `ProviderProgress`

`ProviderProgress` captures execution-local progress signals.

Minimum fields SHOULD include:
- `request_id`
- `attempt_id`
- `provider_id`
- `owner_term`
- `progress_kind`
- `detail`
- `emitted_at`

`progress_kind` SHOULD distinguish at least:
- `queued`
- `running`
- `blocked`
- `partial`
- `completed`
- `failed`
- `cancelled`

### 4.5 `PublicationRecord`

`PublicationRecord` describes published response material.

Minimum fields SHOULD include:
- `request_id`
- `attempt_id`
- `owner_term`
- `generation`
- `seq`
- `payload_ref` or inline payload handle
- `publication_kind`
- `published_at`

`publication_kind` SHOULD distinguish at least:
- `event`
- `partial-result`
- `final-result`
- `artifact-ref`
- `error`

### 4.6 `ObservationRecord`

`ObservationRecord` captures what a requester or other consumer definitely saw.

Minimum fields SHOULD include:
- `request_id`
- `observer_id`
- `observed_generation`
- `observed_seq`
- `observed_at`

`ObservationRecord` MAY be omitted in some minimal deployments, but the
architecture MUST still distinguish publication from observation.

### 4.7 `DiagnosticEvent`

`DiagnosticEvent` captures warnings, degradations, and uncertainty.

Minimum fields SHOULD include:
- `request_id`
- `attempt_id` if relevant
- `source_plane`
- `diagnostic_kind`
- `certainty`
- `detail`
- `emitted_at`

`source_plane` SHOULD distinguish at least:
- `substrate`
- `ownership`
- `execution`
- `publication`
- `observation`
- `upstream`

`certainty` SHOULD distinguish at least:
- `confirmed`
- `suspected`
- `unknown`

## 5. Exchange Guarantees

The exchange model MUST be explicit about what each plane can guarantee.

### 5.1 What intent can guarantee

If a requester sees its own `RequestIntent` durably in the share, it can
reasonably say:
- the request was declared
- the request identity exists
- providers attached to the relevant visibility scope may now observe it

It cannot yet say:
- a provider claimed it
- a provider completed it
- a result exists

### 5.2 What ownership can guarantee

If a valid `ClaimRecord` exists under the current `owner_term`, the system can
say:
- one provider is currently the recognized owner for that attempt

It cannot yet say:
- the provider is healthy
- work has completed
- no older provider performed side effects before being fenced

### 5.3 What publication can guarantee

If a `PublicationRecord` exists with accepted ownership term and sequence, the
system can say:
- some result material was published into shared state

It cannot yet say:
- the requester observed it
- the result is final unless the publication kind says so

### 5.4 What observation can guarantee

If an `ObservationRecord` exists or equivalent derived observation is confirmed,
the system can say:
- the relevant observer has definitely seen at least that generation and
  sequence

Observation is the only plane that can close the loop for end-to-end receipt.

## 6. Uncertainty Cases

The exchange model MUST preserve uncertainty instead of flattening it into one
status field.

Examples:

- Provider completed work, but publication path is unstable:
  - execution may be `completed`
  - publication may be `uncertain`
  - diagnostics may report `substrate` instability

- Request was claimed, but provider disappeared mid-execution:
  - ownership may be `stale` or `uncertain`
  - execution outcome may be `unknown`
  - retry may be required with a new attempt

- Result exists in shared state, but requester was offline:
  - publication may be `complete`
  - observation remains `not-confirmed`

- Retry begins before previous attempt certainty is resolved:
  - old attempt remains part of the bundle
  - new attempt receives a new `attempt_id`
  - accepted results are gated by current ownership and attempt validity rules

## 7. Generations and Sequencing

Glade exchange semantics MUST distinguish:
- logical request identity
- execution attempt identity
- publication generation
- publication sequence

Current working rule:

- `request_id` identifies the logical work request
- `attempt_id` identifies one execution try
- `generation` identifies one coherent response stream or artifact lineage
- `seq` orders response events within one generation

This prevents retry, replay, and progressive delivery from becoming ambiguous.

## 8. Relationship To Interest Semantics

Exchange semantics and interest semantics are related but different.

### Exchange semantics are for:
- bounded units of work
- prompts or commands to a provider
- derived artifact creation
- explicit cancellation or retry

### Interest semantics are for:
- long-lived observation of mutable sources
- ongoing synchronization
- shared source binding and projection maintenance

They may interact:
- an exchange may produce a new `Source Binding`
- an exchange may create a new share or artifact that later participates in
  interest semantics

But exchange semantics MUST NOT replace interest semantics.

## 9. Diagnostics Taxonomy

Diagnostics SHOULD distinguish at least:

- substrate instability
- ownership conflict
- provider-local execution failure
- upstream source failure
- publication failure
- observation uncertainty
- timeout or expiry
- cancellation

Diagnostics SHOULD also distinguish:
- hard failure
- soft degradation
- ambiguity or uncertainty

## 10. Minimal First Slice

The first implementation slice SHOULD stay narrow:

- one provider-visible inbox share
- one requester-visible reply share
- one active owner at a time
- one progressive response generation per accepted attempt
- explicit diagnostics instead of implicit retries

The first slice does not need every optimization. It does need hard semantics
for identity, ownership, publication, and uncertainty.

## 11. Open Questions

This document sharpens, but does not fully close, these design questions:

- how observation confirmation should be represented in minimal deployments
- whether response publication uses the same share as request intent or a
  separate reply share by default
- how garbage collection and retention work for old attempts and diagnostics
- how much of the diagnostics plane is core Glade versus provider-contributed

Related open decisions remain tracked in
`/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md`.
