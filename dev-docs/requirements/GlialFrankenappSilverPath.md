# Glial Frankenapp Silver Path

Status: working draft

Purpose: define the second product path: composing multiple applications and
AI participation into one live environment without export/import workflows.

## Core Claim

The silver path is cross-application composition.

It should validate that Glial can mount separate application facets over shared
workspaces and Glade shares while preserving trust, consent, attribution, and
external authority boundaries.

It should not block the GripLab golden path.

## Product Story

The target story is:

```text
mount CRM workspace + mount Slack workspace + mount AI agent facet
-> link customer, conversation, and task context
-> let the user and agent act over the live shared environment
```

The result is not two apps in iframes. It is one working environment with
multiple facets over governed shared state.

## Candidate Scenario

A Salesforce-like CRM plus Slack-like conversation system is a useful silver
path example.

It pressures:
- multiple external authorities
- external identity mapping
- object linking across systems
- event streams from external apps
- bounded exchanges into external APIs
- delegated AI action
- consent and audit
- per-workspace visibility
- cross-app facet composition

## Minimal Silver Path Shape

The first non-implementation sketch SHOULD include:
- CRM workspace mount
- messaging workspace mount
- customer/account object share
- conversation thread share
- task/action exchange
- AI agent facet with delegated read and propose-action capability
- audit log surface

The first real implementation SHOULD wait until the GripLab golden path proves
declarations, generated bindings, provider claims, local console visibility,
and one AI participant flow.

## Required Boundaries

The silver path MUST keep these boundaries explicit:

| Boundary | Meaning |
| --- | --- |
| External authority | CRM and messaging systems remain authoritative for their own data. |
| Glial environment | Composes mounted workspaces, facets, sessions, and capabilities. |
| Glade shares | Represent live projections, exchanges, logs, links, and delegated action records. |
| Grip Share | Maps local UI state into declared share surfaces. |
| Agent session | Acts only through delegated capabilities and auditable records. |

## Plausible Story For Now

The plausible story is:
- external connectors are providers
- external objects are represented as source-backed Glade content or streams
- actions into external systems are bounded exchanges
- cross-app links are explicit shared records
- AI agents receive delegated capabilities to read context and propose actions
- user approval can be required before external side effects
- audit records attribute actions to human, agent, facet, and provider

This is enough to avoid designing the core substrate into a siloed app model.

## Hard Problems To Expand Later

These are real, but not golden-path blockers:
- external identity linking
- account/team/project tenancy model
- OAuth and enterprise identity provider integration
- consent and approval workflows
- external API rate limits and quotas
- object schema drift across SaaS systems
- data residency and deletion rules
- marketplace or connector registry
- app/facet compatibility negotiation

## Expansion Trigger

Expand this path when one of these becomes true:
- GripLab has a working Glade-backed provider surface.
- The AI agent flow needs a second external authority.
- A real customer workflow requires cross-application object linking.
- The declaration package model needs connector registry or marketplace
  semantics.

Until then, keep this path as a design pressure test, not the main build path.

## Open Issues

This document does not yet define:
- connector package format
- cross-application object link schema
- external identity mapping model
- user consent record shape
- approval policy
- audit event schema
- connector marketplace or registry behavior
