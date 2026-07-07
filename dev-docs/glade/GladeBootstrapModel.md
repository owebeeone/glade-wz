# Glade Bootstrap Model

Status: working draft

Purpose: define how Glade can use Glade declarations to provision higher-level
Glade, Glial, provider, and application behavior without becoming circular or
self-invalidating.

## Core Claim

Glade MAY provision Glade through shared declarations, but only above a minimal
bootstrap kernel.

The bootstrap kernel is the non-circular base. It is the smallest trusted
runtime required for a peer to load, validate, and enter a Glade declaration
space.

After that point, Glade can dynamically provision:
- trust and capability documents
- policy documents
- provider definitions
- facet manifests
- application bundles
- workspace definitions
- exchange and content definitions

## Bootstrap Boundary

The bootstrap kernel MUST be small and stable.

It MUST know how to:
- load a genesis bundle
- verify signatures
- validate the base record envelope
- recognize base declaration kinds
- evaluate minimum capability and policy rules
- reject unknown mandatory features
- enter a declaration space

It SHOULD NOT know application-level definitions, provider-specific contracts,
Grip mappings, or product workflows.

## Authority Bands

Glade SHOULD distinguish three authority bands.

| Band | Meaning |
| --- | --- |
| `bootstrap` | Minimal trusted verifier built into the peer runtime. |
| `system` | Glade-managed declarations that define trust, policy, schemas, providers, facets, and app bundles. |
| `application` | Ordinary workspace data such as files, logs, exchanges, terminals, and live collaboration content. |

This split avoids making application declarations part of the trusted base.

## Genesis

Genesis is the bridge between the bootstrap kernel and the system declaration
space.

Genesis MUST identify:
- genesis id or hash
- bootstrap schema version
- root trust anchors
- system declaration space id
- accepted signature schemes
- base policy version
- mandatory feature set
- initial capability issuers

Genesis MAY seed initial system declarations directly, or it MAY point to
their content-addressed records.

## System Declaration Space

The system declaration space is where Glade provisions itself above the
bootstrap kernel.

It MAY contain:
- `TrustRootDeclaration`
- `PrincipalDeclaration`
- `CapabilityGrant`
- `CapabilityRevocation`
- `PolicyDeclaration`
- `SchemaDeclaration`
- `ProviderDefinition`
- `FacetDefinition`
- `ApplicationBundleDeclaration`
- `WorkspaceTemplateDeclaration`
- `ContentModelDefinition`
- `ExchangeDefinition`

System declarations MUST be validated by bootstrap-level rules plus the active
system policy.

## Dynamic Application Onboarding

New applications SHOULD be introduced as declarations, not code-level
hardcoding.

Example flow:

```text
provider declares it can serve exchange:BugsQuery
facet declares it consumes BugsQuery and file shares
application bundle declares those facets belong together
workspace policy declares which principals may mount the bundle
sessions observe the declarations and attach permitted facets
```

This lets Glial expose new capabilities dynamically while Glade remains the
shared declaration substrate.

## Self-Provisioning Rule

A declaration MUST NOT define the rules that validate itself.

Policy, schema, and trust updates MUST be validated under the previously
accepted policy version or an explicit bootstrap-approved transition rule.

This means:
- current policy validates proposed next policy
- current schema validates proposed next schema
- current trust root validates proposed trust-root update
- peers that cannot understand a mandatory update MUST fail closed or enter a
  defined compatibility mode

## Versioning

System declarations MUST be versioned.

Versioned objects SHOULD include:
- policy versions
- schema versions
- content model versions
- exchange definition versions
- facet manifest versions
- application bundle versions

Version changes SHOULD define:
- compatibility rules
- migration behavior
- rollback behavior
- whether old instances remain valid
- whether new instances require the new version

## Feature Negotiation

Peers SHOULD advertise supported bootstrap and system feature sets.

A peer MUST reject or quarantine declarations requiring mandatory features it
does not understand.

Optional features MAY be ignored if the declaration remains valid without them.

## Revocation And Recovery

Bootstrap and system-level revocation are high-impact operations.

The model MUST define behavior for:
- revoked root principals
- revoked capability issuers
- revoked provider definitions
- revoked facet definitions
- compromised policy versions
- recovery from conflicting system declarations

Peers SHOULD preserve diagnostics when system-level uncertainty occurs.

## Relationship To Glial

Glial consumes the system declaration space to discover:
- available application bundles
- available facets
- workspace templates
- provider capabilities
- trust and capability policy

Glial then maps those system declarations into environments, workspaces,
sessions, facets, and capabilities.

## Relationship To Glade Runtime

The Glade runtime uses the bootstrap kernel to enter the system declaration
space.

The system declaration space then tells the runtime:
- which definitions exist
- which providers may claim work
- which content models are available
- which policies apply
- which application-level declaration spaces may be opened

## Non-Goals

This document does not define:
- exact cryptographic algorithms
- canonical serialization
- libp2p protocol names
- UI composition APIs
- Grip adapter behavior

It defines the bootstrapping boundary that keeps self-provisioning from
becoming circular.
