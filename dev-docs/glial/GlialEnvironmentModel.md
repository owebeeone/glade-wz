# Glial Environment Model

Status: working draft

Purpose: define the deeper Glial model needed to support cross-application
working environments, shared live data, and direct agent participation without
falling back to application silos.

## Core Claim

Glial SHOULD NOT treat one application as the primary collaboration boundary.

Glial SHOULD treat the live working environment as primary.

That environment may contain:
- many application facets
- many human and agent sessions
- many mounted workspaces
- many Glade shares

This is necessary if users and agents are expected to:
- work across tools without export/import loops
- share data across application boundaries in one UI
- let agents act directly on the same live state as a user
- build new features on the fly using the same mounted data

## Primary Model

Glial needs the following first-class nouns.

| Term | Meaning |
| --- | --- |
| `Environment` | The top-level live working world for one user, team, or task context. |
| `Workspace` | One collaboration and security domain mounted into an `Environment`. |
| `Session` | One attached actor runtime inside an `Environment`. A session may be human-facing or headless. |
| `Facet` | One application module, tool, panel, service surface, or projection that operates over mounted shares. |
| `Capability` | A grant that allows a session or facet to read, write, control, host, or decrypt some workspace or share. |
| `Share` | One Glade collaboration object inside one workspace. |

## Why This Split Matters

This split is intended to fix two bad models:

- application silo model
  data belongs to one application and must be exported or bridged manually
- transport-equals-authorization model
  a peer that can participate in comms can also see content

Glial needs neither of those.

The working rule is:

- applications are facets
- workspaces are collaboration and security islands
- shares are Glade collaboration objects inside those islands

## Environment Structure

One `Environment` MAY mount many `Workspace` values.

Examples:
- `workspace:repo-alpha`
- `workspace:bugs-alpha`
- `workspace:user-private`

One environment MAY mount many `Facet` values over the same mounted
workspaces.

Examples:
- editor facet
- terminal facet
- bug list facet
- AI copilot facet
- review facet

This means the editor and AI facet can both operate on the same file share,
the same selection share, and the same terminal share if capability allows it.

## Session Model

A `Session` attaches to an environment, not only to one application.

A session may be:
- a browser tab
- a local desktop client
- a backend worker
- an AI agent

Sessions do not own the data model. They are participants with capabilities.

One environment MAY contain:
- one human session
- many human sessions
- one or more headless agent sessions

## Facet Model

A `Facet` is not a security boundary and not a data silo.

A facet is a projection or tool surface over the same mounted workspaces and
shares.

Examples:
- an editor facet presents file shares and selection shares
- a bug facet presents exchange-based query shares
- a terminal facet presents live channels and logs
- an AI facet reads selections, writes file patches, tails logs, and issues
  exchange requests

This is the key move that allows Glial to meld applications instead of merely
hosting several apps side by side.

## Workspace Model

A `Workspace` is the primary Glial collaboration and security boundary.

It answers:
- who may mount this workspace
- which capabilities they receive
- which shares exist inside it
- how its content should be encrypted or protected

One environment MAY mount several fully disjoint workspaces.

That allows one collaborator to see:
- workspace `repo-alpha`
- workspace `bugs-alpha`

while another collaborator sees:
- workspace `repo-beta`

without changing the higher environment model.

## Capability Model

Participation in transport MUST NOT imply permission to read content.

Glial therefore needs explicit capability issuance for:
- workspace mount
- share open
- read
- write
- control
- host
- delegate
- decrypt

Capabilities MAY be scoped:
- to one workspace
- to one share
- to one content model inside a share
- to one time window
- to one delegated agent task

## Agent Model

Agents are sessions, not a special side-channel.

This means an AI assistant can:
- mount the same environment
- receive delegated capabilities
- read the same selection or file shares
- write patches to the same mutable content shares
- tail the same terminal or log shares
- issue the same exchange requests

This is what removes the export, analyze, import workflow.

## Security And P2P Direction

In a future p2p deployment:

- peers may carry or relay encrypted traffic
- only sessions with the right capability material can decrypt a workspace or
  share
- workspaces SHOULD be the default encryption domains
- shares MAY use narrower per-share keys where needed

This means:
- a peer can participate in comms without seeing content
- a collaborator can mount one workspace without learning another
- one environment can still project many workspaces into one user experience

## Glial To Glade Mapping

Glial does not replace Glade. It maps environment concepts onto Glade
mechanics.

| Glial Concept | Maps To In Glade |
| --- | --- |
| `Workspace` | one collaboration namespace containing many `Share` values |
| `Share` | one Glade `Share` with one or more `ContentShare` models |
| `Facet` | an application-facing consumer or producer over existing Glade shares |
| `Session` | a principal-attached participant with capabilities to open or mutate shares |
| `Capability` | permission and decryption material required to access a workspace or share |

Glial owns:
- environment composition
- workspace mounting
- facet registration
- session attachment
- capability issuance
- delegation

Glade owns:
- share identity
- exchange mechanics
- log mechanics
- mutable content mechanics
- live channel mechanics
- replay, invalidation, and observation

## Example

One environment:

- `env:alice-dev`

Mounted workspaces:

- `workspace:repo-alpha`
- `workspace:bugs-alpha`
- `workspace:alice-private`

Mounted facets:

- `facet:editor`
- `facet:terminal`
- `facet:bugs`
- `facet:ai-copilot`

Sessions:

- `session:alice-browser`
- `session:assistant-refactor`

The editor and AI copilot may both open:

- `share:file:src/app.ts`
- `share:selection:editor-main`

The terminal and AI copilot may both open:

- `share:terminal:dev-server`
- `share:log:test-run-8842`

The bug facet and AI copilot may both use:

- `share:exchange:BugsQuery`

The data is shared because the environment mounts the same workspaces and
shares. It is not shared because the applications happen to know how to bridge
one another.

## Practical Direction

Glial SHOULD move away from `HybridApp` as the dominant mental model and
toward:
- environment
- workspace
- facet
- session
- capability

`HybridApp` MAY still exist as a packaging or deployment concept, but it
should not remain the primary collaboration model if Glial is expected to
support cross-application live work and direct agent participation.
