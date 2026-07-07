# Glial Application Definition Model

Status: working draft

Purpose: define how application definitions, facet definitions, provider
contracts, and related declaration collections are organized, stored, shared,
and updated.

## Core Claim

An application definition SHOULD be modeled as a collection of declarations,
not one monolithic document.

This allows Glial to:
- share definitions independently
- update facets without replacing the whole application
- reuse provider definitions across applications
- mount the same capability in multiple environments
- replicate definition sets across genesis or system declaration spaces

## Definition Collection

An application definition collection is a named, versioned set of related
declarations.

It MAY include:
- application bundle declaration
- facet definitions
- provider definitions
- exchange definitions
- content model definitions
- workspace templates
- capability requirements
- UI placement hints
- version compatibility rules

The collection SHOULD be addressable as a manifest with content references to
its member declarations.

## Hierarchical Definitions

Definitions SHOULD support hierarchy.

Example:

```text
ApplicationBundle: GripLab
  Facet: RepoExplorer
  Facet: Terminal
  Facet: FileEditor
  Facet: AgentPanel
  ProviderDefinition: GripLabBackend
  ExchangeDefinition: RunCommand
  ContentModelDefinition: MutableFile
  ContentModelDefinition: TerminalPty
  LogModelDefinition: CommandOutputLog
```

Hierarchy is organizational. Each child declaration SHOULD still have its own
identity, version, policy, and capability requirements.

## Application Bundle

An `ApplicationBundleDeclaration` describes a composed capability surface.

It SHOULD include:
- application bundle id
- version
- required facets
- optional facets
- required provider groups
- required exchange definitions
- required content model definitions
- workspace template requirements
- capability requirements
- compatibility constraints

Example:

```ts
declareApplicationBundle({
  applicationBundleId: 'app:griplab',
  version: '0.1.0',
  facets: [
    'facet:repo-explorer',
    'facet:terminal',
    'facet:file-editor',
    'facet:agent-panel',
  ],
  providers: ['provider-def:griplab-backend'],
  definitions: [
    'exchange:RunCommand',
    'model:TerminalPty',
    'model:MutableFile',
    'model:AppendLog',
  ],
  capabilityRequirements: ['workspace.mount', 'file.read', 'terminal.control'],
});
```

## Facet Definition

A `FacetDefinition` declares what an application facet consumes and produces.

It SHOULD include:
- facet id
- version
- consumed share types
- produced share types
- required capabilities
- optional capabilities
- provider roles if any
- agent delegation behavior

Facets SHOULD NOT be treated as security boundaries. They request access;
Glial evaluates access through session and workspace capabilities.

## Sharing Definition Collections

Definition collections SHOULD be shareable across genesis or system
declaration spaces by reference.

Possible sharing modes:
- copy by content hash
- mount by signed manifest reference
- mirror from trusted registry
- subscribe to update channel

Each mode MUST define:
- who trusts the source
- how updates are validated
- whether local policy can override imported definitions
- whether imported definitions can grant capabilities directly

Imported definitions SHOULD NOT automatically receive authority to change local
policy unless explicitly allowed by the local genesis or system policy.

## Storage

Application definition collections SHOULD be stored as system declarations or
content-addressed declaration manifests.

Large assets, UI bundles, or code artifacts SHOULD be stored as content
references rather than inline declaration payloads.

The declaration record SHOULD preserve:
- manifest id
- version
- content hash
- signature
- issuer
- compatibility metadata
- dependency references

## Dependency Model

Application definitions MAY depend on other definition collections.

Dependencies SHOULD include:
- dependency id
- version constraint
- required or optional flag
- compatibility policy
- trusted issuer requirement

Dependency resolution MUST be deterministic within one system declaration
space.

## Update Model

Updates SHOULD be represented by new declaration versions.

An update SHOULD define:
- superseded version
- migration behavior
- compatibility with existing instances
- rollback behavior
- whether sessions may continue using the old version
- whether providers must drain or restart

## Relationship To Genesis

Genesis SHOULD NOT contain whole application definitions except for minimal
bootstrap cases.

Genesis SHOULD point to the system declaration space and trust roots that can
validate application definition collections.

The system declaration space can then provision new applications dynamically.

## Open Issues

This document does not yet define:
- exact manifest serialization
- cross-genesis trust negotiation
- definition registry discovery
- UI/code artifact loading policy
- whether imported application collections can carry executable code
