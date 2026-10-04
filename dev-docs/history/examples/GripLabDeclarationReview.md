# GripLab Declaration Review

Status: example review

Purpose: explain the first GripLab `.glade` declaration sketch and what it
exposes about the declaration language.

## Source Code Surface

The sketch is based on the current GripLab service and React client.

Backend methods observed in the local service include:
- `workspace.status.subscribe`
- `chat.subscribe`
- `peer.presence.subscribe`
- `tree.subscribe`
- `file.subscribe`
- `file.window.update`
- `sessions.subscribe`
- `session.output.subscribe`
- `cmd.run`
- `term.open`
- `term.input`
- `term.resize`
- `term.close`

The main handler lives in
`/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/services/griplab_service/src/griplab_service/local_client/app.py`.

The React taps consume those service surfaces through Grip in:
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/fileContentTap.ts`
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/treeTap.ts`
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/sessionsTap.ts`
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/sessionOutputTap.ts`
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/diffContentTap.ts`
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/peersTap.ts`
- `/Users/owebeeone/limbo/grip-pyrolyze-dev/grip-lab/src/lab/serviceTaps/chatMessagesTap.ts`

## What The Sketch Defines

The first declaration sketch defines:
- one package: `pkg:griplab`
- one application: `app:griplab`
- one workspace: `workspace:repo`
- imported files for workspace, application, provider, and scale declarations
- several facets over the same workspace
- one provider definition: `provider-def:griplab-backend`
- one service definition: `service:griplab-backend`
- one sticky-session assignment policy
- one provisional local development environment overlay
- stream definitions for presence, tree, status, file windows, diffs, sessions
- exchange definitions for command run, terminal open, chat post, settings,
  and admin restart
- source definition for terminal sessions
- live-channel definition for terminal PTY
- live-output stream definition for terminal bytes
- log definitions for command, terminal, and chat output
- materialized terminal-screen view for UI rendering

## Declaration File Roles

`GripLab.glade` is the root package manifest. It defines the package id,
issuer, and imports. It should stay small because it composes the declaration
graph rather than describing all behavior inline.

`griplab/workspace.glade` defines the collaboration/security boundary and the
capability atoms available inside that boundary. Its `id` is package-scoped:
`pkg:griplab#workspace:repo`.

`griplab/application.glade` defines the user-facing app and its facets. A facet
is the level where Grip taps, UI panels, or agent tools can later map onto
shared Glade surfaces.

`griplab/provider.glade` defines provider capability, deployable service shape,
assignment policy, and the share surfaces the provider can serve.

`griplab/scale.mode1.glade` defines deployment envelopes. Keeping it imported
separately avoids mixing workspace semantics with scale and placement policy.

`griplab/env.local-dev.glade` defines provisional LAN/self-hosted admission,
ingress, and TLS references. Keeping it imported separately makes local TLS and
hosted-provider routing policy swappable without changing the application
model.

## What The Sketch Proves

The DSL needs first-class declarations for at least:
- `package`
- `workspace`
- `application`
- `facet`
- `provider`
- `service`
- `assignment`
- `stream`
- `content`
- `exchange`
- `live_channel`
- `log`

GripLab is not just request/response. Even this small app already needs:
- snapshot streams
- windowed content streams
- derived diff streams
- append logs
- live terminal channels
- provider placement
- session affinity
- legacy protocol mapping

## Current Versus Target Model

The imported provider declarations use `legacy method` mappings to connect
current service methods to future Glade declarations.

This is deliberate. The current service has websocket methods. The target
model has declarations and generated bindings.

For example:
- `cmd.run` maps to `exchange RunCommand`
- `term.open` maps to `exchange OpenTerminal`, which creates
  `source TerminalSession` and returns `handle TerminalHandle`
- `term.input` and `term.resize` map to live-channel controls
- `session.output.subscribe` maps to a compatibility output surface; target
  Glade semantics are `stream TerminalLiveOutput` plus `log TerminalOutput`
- `file.subscribe` maps to windowed content over `WorkspaceFile`

## Hard Questions Exposed

### 1. Is `stream` too broad?

The sketch uses `stream` for presence, tree, workspace status, file windows,
diffs, and sessions. Those have different semantics.

The schema may need sub-kinds:
- `snapshot_stream`
- `window_stream`
- `derived_stream`
- `presence_stream`
- `index_stream`

### 2. How do logs relate to streams?

Current `session.output.subscribe` sends full output snapshots. The intended
Glade model is append chunks plus replay cursor.

The declaration needs to express both:
- current legacy protocol mapping
- target content/log semantics

The Node Phase 1 terminal work made this sharper: a legacy full-output snapshot
stream cannot be treated as the same contract as a durable byte append log. The
DSL needs an explicit compatibility mapping when a legacy snapshot feeds a
materialized UI view while the target Glade contract remains append-oriented.

### 3. Where does Grip mapping live?

The package declarations name application and data surfaces. They do not map
individual Grip taps or Grip keys.

That mapping should probably live in a Grip Share adapter declaration or
generated binding layer, not in the core app package.

The adapter declaration must still be generated from Glade-visible semantics.
For terminal output, the generated adapter needs to know the handle fields that
form the request key and route key. Inferring peer/session/target from a
materialized sessions list is not reliable enough.

### 4. Does `workspace repo` represent one repo or one developer workspace?

The current app can target repo paths within one service workspace. The DSL
needs to decide whether `workspace:repo` is:
- one actual repo
- one developer machine workspace
- one collaboration/security domain containing many repos

This affects capability and encryption boundaries.

### 5. How much legacy admin surface belongs in the app declaration?

Methods like `admin.restart`, `debug.perf.get`, and peer bootstrap operations
are real service surfaces, but they may belong to an operator/admin package
rather than the main GripLab app definition.

## Immediate DSL Pressure

The GripLab example suggests the DSL needs:
- aliases that compile to stable ids
- import statements that compose one package from separate files
- `legacy method` metadata for migration
- capability requirements on facets and operations
- source and handle declarations for multi-view resources
- stream event shape declarations
- log replay declarations
- materialized-view declarations for decoded or aggregated UI state
- provider/service split
- assignment policy declarations
- comments or notes that do not affect canonical hashes

## Next Iteration

The next useful pass should choose one thin vertical slice and define the
schema for it:

```text
package -> application -> facet terminal -> provider -> service
exchange OpenTerminal -> source TerminalSession
source TerminalSession -> live_channel TerminalPty
source TerminalSession -> stream TerminalLiveOutput
source TerminalSession -> log TerminalOutput
source TerminalSession -> materialized TerminalScreen
```

That slice should be enough to generate Python provider helpers and TypeScript
client helpers for the terminal path.

The generated helpers should prove:
- `OpenTerminal` returns a stable terminal handle
- input, resize, close, live output, replay log, and materialized screen all bind
  to the same handle
- output storage is raw bytes with declared cursor semantics
- UTF-8/xterm rendering is a materialization policy, not the log payload
- legacy `session.output.subscribe` is marked as compatibility-only unless an
  adapter declares snapshot-to-append conversion
