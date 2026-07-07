# Glade Declaration Package

Status: working draft

Purpose: define how Glade declarations are authored as static, analyzable
artifacts and compiled into signed runtime declarations and language bindings.

## Core Claim

Declarations SHOULD be data, not application code.

Application and provider code SHOULD be constrained by declaration packages.
Generated bindings MAY make those declarations convenient to use, but the
declaration package remains the source of truth.

The rule is:

```text
declaration package -> validated/signed records -> generated bindings -> constrained runtime use
```

## Why Packages Matter

Static declaration packages allow tooling to:
- validate whether an app can deploy
- inspect required capabilities and secrets
- reject non-conforming applications
- estimate infrastructure requirements
- detect unsupported scale-mode assumptions
- generate typed helpers
- verify provider and client code references known declarations
- compare development intent with production policy
- apply or reject environment-specific overlays before admission

This lets application authors express deployment intent early, while
infrastructure providers can accept, reject, or constrain that intent before
anything runs.

## Package Layout

A package MAY look like:

```text
glade.pkg/
  package.toml
  definitions/
    app.griplab.toml
    facet.terminal.toml
    facet.file-editor.toml
    provider.griplab-backend.toml
    exchange.run-command.toml
    model.terminal-pty.toml
    model.mutable-file.toml
  deployment/
    service.griplab-backend.toml
    deployment.dev.toml
    deployment.prod.toml
    environment.local-dev.toml
    environment.hosted.toml
  policy/
    required-capabilities.toml
    session-affinity.toml
  schemas/
    RunCommandRequest.schema.json
    RunCommandResponse.schema.json
```

This layout is illustrative. The exact layout is an open decision.

## Source Formats

Authoring formats SHOULD be human-readable.

Initial candidates:
- TOML for operational declarations
- JSON Schema for request, response, and payload schemas
- JSON for generated canonical artifacts

The signed runtime form SHOULD use a canonical format suitable for hashing and
signature verification.

Human-authored files and signed runtime records do not need to be the same
format.

## Manifest

The package manifest SHOULD define:
- package id
- package version
- issuer
- target Glade feature version
- supported scale modes
- definition entries
- schema entries
- dependency entries
- signing policy
- generated binding targets
- optional environment overlay entries

Example:

```toml
package_id = "pkg:griplab"
version = "0.1.0"
issuer = "principal:griplab-team"
glade_feature_version = "0.1"
target_scale_modes = ["mode1", "mode2", "mode3"]

[bindings]
typescript = "src/generated/glade-defs.ts"
python_module = "griplab_glade_defs"
python_stubs = "griplab_glade_defs.pyi"
```

## Scale-Mode Declarations

A package SHOULD declare the scale modes it is intended to support.

Example:

```toml
[scale]
target_modes = ["mode1", "mode2", "mode3"]
requires = ["session-affinity", "provider-placement", "live-channel"]
disallows = ["single-process-only"]
```

Infrastructure providers MAY reject a package whose declared behavior is
incompatible with the target scale mode.

## Environment Overlays

Environment overlays SHOULD be authored as data and imported separately from
core application declarations.

They are the provisional home for:
- local LAN TLS defaults
- enterprise TLS and trust-anchor references
- hosted-provider namespace admission policy
- public ingress listener policy
- join-routing defaults
- provider-specific deployment constraints

They SHOULD NOT change application, workspace, exchange, content, or facet
semantics.

Local development packages MAY include a permissive default overlay. Hosted
providers SHOULD be able to replace, constrain, or reject that overlay during
admission.

Secrets MUST be referenced, not embedded. For example, an overlay may reference
`secret:local-dev/tls-key`, a provider secret id, or a local certificate path
under a policy-controlled rule, but it MUST NOT contain raw private key data.

## Compilation

The declaration compiler SHOULD:
- parse source files
- validate schemas
- resolve dependencies
- check internal references
- check target scale-mode compatibility
- apply infrastructure policy if supplied
- produce canonical declaration records
- sign or prepare records for signing
- compute package and record hashes
- generate language bindings

Example workflow:

```text
glade-decl validate glade.pkg
glade-decl plan glade.pkg --target mode3
glade-decl estimate glade.pkg --workload expected.toml
glade-decl sign glade.pkg --key issuer.key
glade-decl generate glade.pkg
```

## Generated Bindings

Generated bindings SHOULD provide typed convenience over declarations.

They MAY include:
- typed ids
- exchange clients
- provider stubs
- request and response validators
- capability constants
- content model helpers
- service definition helpers
- declaration hashes

Generated bindings MUST NOT become the source of truth. They are derived from
the package.

## Python Module-Load Generation

Python MAY load definitions dynamically at module import time.

This works well because Python can construct definition objects from signed
declaration data at runtime.

Example:

```python
from glade_decl import load_package

defs = load_package("glade.pkg/manifest.signed.json")

BugsQuery = defs.exchange("exchange:BugsQuery")
RunCommand = defs.exchange("exchange:RunCommand")
MutableFile = defs.content_model("model:MutableFile")
GripLabBackend = defs.provider("provider-def:griplab-backend")
```

`load_package` SHOULD:
- verify package signature
- validate package hash
- validate schemas
- check supported Glade feature version
- construct definition objects
- expose package hash and definition hashes
- fail loudly if the package is invalid

Python packages MAY also provide generated `.pyi` stubs so IDEs and type
checkers can see the dynamic definitions.

## TypeScript Build-Time Generation

TypeScript SHOULD use build-time generation.

This allows static type checking, bundler compatibility, and editor support.

Example:

```text
glade-decl generate-ts glade.pkg --out src/generated/glade-defs.ts
```

Generated TypeScript SHOULD embed or export:
- package hash
- definition ids
- definition hashes
- typed exchange helpers
- schema validators
- capability constants

Example use:

```ts
import { BugsQuery } from './generated/glade-defs';

await glade.exchange(BugsQuery).request({ status: 'open' });
```

## Runtime Enforcement

Runtime behavior SHOULD be constrained by declarations.

Examples:
- provider cannot claim an undeclared exchange
- UI cannot request a capability absent from the package or granted policy
- service cannot publish routing for an undeclared provider group
- app cannot claim scale-mode support contradicted by its declarations

Provider claims and client requests SHOULD reference package and definition
hashes. Connected peers MAY reject behavior that references unknown,
unsupported, revoked, or mismatched declarations.

## Drift Prevention

Tooling SHOULD detect drift between source declarations, signed records, and
generated bindings.

Possible checks:
- regenerated bindings match checked-in bindings
- package hash in bindings matches signed manifest
- provider imports definitions from the signed package
- runtime claims reference the same definition hash as the package
- TS and Python generated definitions agree on ids and schema hashes

## Infrastructure Policy Validation

Infrastructure providers SHOULD be able to validate packages before accepting
them.

Policy validation MAY reject packages because:
- required capabilities are not allowed
- service images are untrusted
- scale-mode claims are unsupported
- live channels are forbidden in a hosting tier
- provider placement rules conflict with infra policy
- declared capacity requirements exceed quota
- session affinity requires unavailable state migration

## Estimation And Planning

The declaration package SHOULD expose enough information for rough planning.

Estimators MAY use:
- provider capacity declarations
- scaling policy
- expected workload files
- session affinity requirements
- live channel counts
- storage retention policy
- content/log size estimates

The output can be approximate, but the declarations should make the estimate
possible before deployment.

## Relationship To Runtime Declarations

Source declaration packages produce runtime declarations.

Runtime declarations may then be:
- published into a system declaration space
- referenced by genesis
- imported by another declaration collection
- used by an application manager
- used to validate provider and client behavior

The package is the authoring artifact. The signed declaration records are the
runtime trust artifact.

## Open Issues

This document does not yet define:
- exact package layout
- canonical signed representation
- TOML versus JSON authoring boundaries
- schema language choice beyond JSON Schema
- generated binding naming rules
- whether generated files are checked in or regenerated in every build
- package dependency resolution
- package registry/distribution model
