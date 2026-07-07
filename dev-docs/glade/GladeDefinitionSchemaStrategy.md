# Glade Definition Schema Strategy

Status: outline draft

Purpose: define how Glade should approach a real machine-readable schema for
declaration packages and definition records before writing detailed
field-by-field schemas.

## Core Claim

Glade definitions SHOULD be schema-first.

Python dataclasses, Pydantic models, TypeScript types, and provider/client
helpers SHOULD be generated from or validated against the schema. They SHOULD
NOT be the canonical source of truth.

The declaration package remains data. Generated code is a convenience and
conformance layer.

## Initial Recommendation

Use this pipeline for the first real schema pass:

```text
human-authored YAML/TOML/JSON files
  -> JSON-compatible declaration objects
  -> JSON Schema validation
  -> canonical signed JSON records
  -> generated Python and TypeScript bindings
```

This keeps the first implementation practical:
- human-readable source files
- machine-readable validation
- Python and TypeScript generation
- simple CI and package validation
- compatibility with signed declaration records

Protobuf MAY be useful later for compact transport or generated runtime APIs,
but it SHOULD NOT be the first authoring format because Glade declarations
need to be readable, reviewable, policy-checkable, and easy to diff.

## Why Not Python Dataclasses As Canonical Source

Python dataclasses or Pydantic models are useful, but Python code should not be
the declaration source of truth.

Reasons:
- Glade needs JS/TS and Python parity.
- Infrastructure providers need to validate packages without executing app
  code.
- Declaration packages should be reviewable as static artifacts.
- Signed records need canonical serialization independent of Python runtime
  behavior.
- p2p peers should validate records from schema and signatures, not imported
  Python modules.

Python SHOULD still get a strong developer experience:
- dynamic module-load generation from signed packages
- generated `.pyi` stubs
- generated Pydantic/dataclass wrappers if useful
- runtime hash checks against signed definitions

## Format Options

| Option | Usefulness | Concern |
| --- | --- | --- |
| JSON Schema | Strong first choice for validation, tooling, TS/Python generation, and policy inspection. | Needs separate canonicalization/signing rules. |
| YAML/TOML | Good human authoring formats. | Need deterministic conversion to canonical records. |
| Python dataclasses/Pydantic | Good generated or helper layer for Python. | Code-first, Python-biased if canonical. |
| TypeScript types | Good generated client/helper layer. | Not sufficient for Python/provider validation. |
| Protobuf | Good for compact wire/runtime APIs later. | Poor primary authoring format and less pleasant for review/policy analysis. |
| CUE/TypeSpec | Potential future schema authoring tools. | Adds tooling surface before core schema is settled. |

## Schema Families

The real schema set SHOULD be split into families rather than one large
unstructured schema.

Initial families:

- package manifest
- genesis and bootstrap
- declaration record envelope
- trust and capability
- application bundle and facet definitions
- provider definitions and provider instances
- provisioning and deployment plans
- exchange definitions and exchange instances
- content model definitions
- live channel definitions
- log definitions
- mutable content definitions
- CRDT content definitions
- routing publications
- session affinity and work assignment
- retention and lifecycle
- diagnostics and observations
- scale-mode declarations

## Use-Case Outline

The first schema pass SHOULD be checked against four concrete declaration
packages.

### 1. GripLab

Primary pressure:
- p2p Mode 1
- multiple app managers
- terminal live channels
- command logs
- mutable file content
- provider placement
- session affinity
- AI agent delegation

Schema families exercised:
- application bundle
- facet definitions
- provider definitions
- live channel
- log
- mutable content
- provisioning
- routing publication
- capability delegation

### 2. Large Client/Server Application

Primary pressure:
- exchange-heavy API surface
- many backend services
- service deployment and rollback
- provider groups
- compiled routing indexes
- high-QPS hot path

Schema families exercised:
- exchange definitions
- provider placement
- service definitions
- deployment plans
- assignment policy
- scale modes
- runtime planning and estimation

### 3. Maps-Style Application

Primary pressure:
- read-mostly massive content
- geospatial windows
- chunks and cache policy
- external source authority
- high fanout
- CDN-like distribution

Schema families exercised:
- content model definitions
- windowed content
- chunk refs
- retention and cache policy
- source-of-truth declaration
- routing publication
- scale modes

### 4. Salesforce + Slack Application Space

Primary pressure:
- application space sharing
- multiple external authorities
- identity bridging
- delegated AI access
- audit and attribution
- cross-app facets over shared environment state

Schema families exercised:
- application bundle
- workspace mounts
- external provider definitions
- exchange definitions
- stream/event definitions
- delegated capabilities
- audit diagnostics
- identity mapping

## Schema Shape Outline

The detailed schema should probably start with these root objects:

```text
DeclarationPackageManifest
DeclarationRecordEnvelope
GenesisBundle
SystemDeclaration
ApplicationBundleDefinition
FacetDefinition
ProviderDefinition
ServiceDefinition
DeploymentPlan
ExchangeDefinition
ContentModelDefinition
LiveChannelDefinition
LogDefinition
MutableContentDefinition
CapabilityGrant
RoutingPublication
SessionAffinityBinding
WorkAssignment
RetentionPolicy
ScaleModeSupport
```

Each object SHOULD have:
- stable id
- kind discriminator
- schema version
- package or genesis reference
- issuer or principal reference
- capability requirements
- policy references
- dependency references where applicable
- lifecycle or retention metadata where applicable

## Canonical Runtime Form

Human-authored files SHOULD compile into canonical records.

The canonical runtime form MUST define:
- deterministic field ordering or canonical serialization
- hash calculation
- signature coverage
- schema version reference
- package hash
- dependency hashes
- validation behavior for unknown fields

The exact canonical format is still open. Initial candidates:
- canonical JSON
- DAG-CBOR or another canonical binary form later
- protobuf only after deciding how it maps to signing and human-authored
  declarations

## Generated Bindings

The schema toolchain SHOULD generate:

- TypeScript types
- TypeScript constants and helpers
- Python runtime loaders
- Python `.pyi` stubs
- optional Python dataclass/Pydantic wrappers
- JSON Schema validators
- provider stubs
- client helpers

Generated bindings MUST embed or expose:
- package hash
- definition hashes
- schema version
- supported feature set

## Validation Stages

Validation SHOULD occur in stages:

1. source syntax validation
2. schema validation
3. cross-reference validation
4. package dependency validation
5. scale-mode validation
6. infrastructure policy validation
7. canonical record validation
8. signature validation
9. generated binding drift check

## First Schema Milestone

The first useful schema milestone SHOULD define:

- package manifest
- declaration record envelope
- application bundle
- facet definition
- provider definition
- exchange definition
- live channel definition
- log definition
- mutable content definition
- service definition
- deployment plan
- capability requirement
- scale-mode support

That is enough to sketch GripLab as a real package without solving every
advanced production case.

## Open Issues

This document does not yet decide:
- JSON Schema dialect
- canonical JSON versus canonical binary representation
- YAML versus TOML for authoring
- exact generated Python runtime-loader API
- whether Pydantic is mandatory or optional
- whether protobuf is used for runtime transport
- schema package dependency/versioning rules
