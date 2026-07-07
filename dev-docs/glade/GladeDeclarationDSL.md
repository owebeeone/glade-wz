# Glade Declaration DSL

Status: early sketch

Purpose: explore a purpose-built declaration language for Glade packages.

This is not a formal grammar yet. It is a working syntax used to expose what
we need to declare for real applications.

## Core Claim

The Glade DSL should be declarative, analyzable, and non-executable.

It should describe:
- application bundles
- workspaces
- facets
- providers
- services
- exchanges
- content models
- resource sources and handles
- streams and live channels
- logs
- materialized views
- legacy adapter mappings
- capabilities
- placement and affinity
- scale-mode intent
- package imports
- environment overlays for admission, ingress, and TLS references

It should not contain arbitrary code.

## Design Rules

The DSL SHOULD:
- compile to canonical declaration records
- support generated Python and TypeScript bindings
- preserve stable ids
- be readable in code review
- be statically analyzable by infrastructure providers
- support overlays for development and production
- support imports for composing packages from separate files
- support comments for human review without changing semantics

The DSL SHOULD NOT:
- execute user code
- hide dynamic behavior in scripts
- depend on one runtime language
- make transport mechanics the semantic contract

## Early Syntax Shape

The first syntax uses blocks and named declarations.

Example:

```glade
package griplab version "0.1.0" {
  id "pkg:griplab"
  issuer "principal:griplab-team"

  import "./griplab/workspace.glade"
  import "./griplab/application.glade"
  import "./griplab/provider.glade"
  import "./griplab/scale.mode1.glade"
}
```

The imported files then define workspaces, applications, providers, services,
and scale/deployment declarations separately.

This syntax is intentionally compact. The compiled IR should carry the
precise record kinds, ids, hashes, policy references, source file references,
and capability references.

## Source, Handle, And View Syntax

The DSL MUST be able to declare one concrete source that exposes multiple
flow-typed views. This is required for resources such as terminals where one
open operation creates a live channel, an output log, and derived UI state.

Example shape:

```glade
source TerminalSession {
  id "source:terminal-session"

  opened_by exchange OpenTerminal

  handle fields provider_id, provider_instance_id, owner_term,
    session_id, terminal_id, target_ref, pty_channel_id, output_log_id

  lifecycle provider_local non_migratable
  affinity must_same_provider_until_close

  live_channel TerminalPty {
    input bytes TerminalInput
    control TerminalResize
    control TerminalClose
    output stream TerminalLiveOutput
  }

  stream TerminalLiveOutput {
    payload bytes
    delivery best_effort
    replay none
  }

  log TerminalOutput {
    source stream TerminalLiveOutput
    payload bytes
    append_order provider_sequenced
    cursor provider_seq
    coalesce max_bytes 65536 max_latency 20ms
    replay from_cursor
  }

  materialized TerminalScreen {
    source log TerminalOutput
    decode terminal_bytes
    retention latest_only
  }
}
```

The exact grammar is provisional. The semantic requirement is not provisional:
generated bindings MUST be able to derive the request key, route key, replay
cursor, and control target from the declared handle rather than from a separate
materialized list such as `CommandSessions`.

Legacy method mappings MAY adapt older protocols to target Glade semantics, but
the mapping MUST say whether the legacy surface is canonical or compatibility
only. For example, a legacy full-output snapshot stream MAY feed a materialized
view, but it MUST NOT be treated as a byte-oriented append log unless an adapter
declares how snapshots become append records and cursors.

## Comments

Glade source files SHOULD support three comment forms:

```glade
// line comment
# line comment
/* block comment */
```

`//` and `#` comments SHOULD run from the marker to the end of the current
line.

`/* ... */` comments MAY span multiple lines. Block comments SHOULD NOT nest.

Comment markers inside quoted strings MUST be treated as string content.

Comments MAY appear anywhere whitespace may appear. Comments MUST NOT create
declaration records, MUST NOT change canonical IR, and MUST NOT affect
semantic hashes or signatures. Tooling MAY preserve comments in source maps,
formatting output, generated documentation, and diagnostics.

## Aliases And Ids

Most declarations have both a block alias and a canonical `id`.

Example:

```glade
workspace repo {
  id "workspace:repo"
}
```

`repo` is the local alias. It exists to make source files readable and to
allow concise references inside the same package declaration graph, including
across imported package-local files.

`workspace:repo` is the canonical declaration id. It MUST be stable across
source refactors and alias changes. It SHOULD be unique within the compiled
package graph after imports are resolved.

A non-package declaration id is package-scoped unless the DSL later introduces
an explicitly absolute id form. The fully qualified identity of the example
above is therefore:

```text
pkg:griplab#workspace:repo
```

The package `id` is different: it identifies the package itself and SHOULD be
globally meaningful within the trust/provisioning environment.

Generated bindings, diagnostics, persisted references, signatures, and remote
protocol records SHOULD prefer canonical ids or fully qualified ids over
source aliases.

## Imports

Packages SHOULD be composable from multiple `.glade` files.

Example:

```glade
package griplab version "0.1.0" {
  id "pkg:griplab"
  issuer "principal:griplab-team"

  import "./griplab/workspace.glade"
  import "./griplab/application.glade"
  import "./griplab/provider.glade"
  import "./griplab/scale.mode1.glade"
}
```

Imports SHOULD be declarative includes. They MUST NOT execute code.

Imported files SHOULD be parsed into the same package declaration graph, with
stable source references preserved for diagnostics and signing. In this sketch,
imports are package-local declaration-graph includes, not runtime module loads.

Scale and deployment declarations SHOULD be importable independently from
workspace and application declarations so that deployment intent does not have
to be mixed into the application workspace definition.

Environment declarations SHOULD also be importable independently.

Environment imports MAY define:
- namespace admission policy
- local or hosted ingress listeners
- TLS certificate and trust-anchor references
- join routing defaults
- provider-specific deployment overlays

Environment imports MUST NOT redefine application semantics, workspace
semantics, content models, exchange contracts, or facet contracts.

TLS private keys and other secrets MUST NOT be embedded directly in `.glade`
source. Environment declarations SHOULD reference secrets, files, certificate
stores, or provider-managed secret ids.

Self-hosted and LAN deployments MAY use permissive local defaults, including
wildcard local namespaces and locally managed TLS trust anchors. Hosted
multi-tenant providers SHOULD apply stricter admission policy and MAY reject
packages whose namespace claims cannot be validated.

The exact environment declaration syntax is provisional. The important
boundary is that local TLS, public ingress, hosted namespace admission, and
enterprise deployment policy live in swappable environment imports rather than
inside workspace or application definitions.

## Current Open Syntax Questions

- How are ids quoted or normalized?
- Are block names symbolic aliases, stable ids, or both?
- Should imports later support external package references, or only
  package-local declaration includes?
- Can imported files define multiple packages, or only contribute to the
  current package?
- How are environment overlays expressed?
- How are local-dev defaults selected without contaminating production
  package signatures?
- How much type syntax belongs in the DSL versus external schemas?
- Should payload schemas be inline, JSON Schema references, or DSL-defined?
- What is the final source/handle/materialized-view grammar?
- How does a legacy snapshot stream declare that it is compatibility-only for a
  target append-log contract?
- Which request-key fields must generated Grip Share adapters derive from each
  handle?
- What is the exact canonical IR shape?

These are open. The GripLab example exists to pressure these questions.
