# .glade Keyword Hierarchy

Status: example note

Purpose: show how the terminal `.glade` slice can be understood as a hierarchy
instead of a flat concept graph.

The dense graph is useful for traceability, but it is hard to read because every
keyword touches multiple architectural concepts. A better authoring model is
closer to a programming language:

```text
package                  namespace / module
  workspace              security boundary declaration
    capability           policy atom
  application            app-facing declaration
    facet ref            app includes a UI/agent slice
    provider ref         app expects this provider definition
  facet                  UI/agent-facing class
    creates              what this facet may instantiate
    consumes             what this facet may read/use
    requires capability  authority needed by this facet
  provider               provider-definition class
    serves               definitions this provider can satisfy
  service                concrete runtime attachment
  exchange               bounded-work class
    request              input schema
    creates              source instance created by the work
    returns handle       identity returned to later operations
  source                 real resource class
    handle               stable route/replay identity
    live_channel         hot-path interactive view
    stream               live event/byte view
    log                  durable replay view
    materialized         derived UI/read model
```

## Proposed Mental Model

`.glade` should have four semantic levels:

1. **Namespace level**: `package`, `import`, `version`, `issuer`.
2. **Declaration level**: `workspace`, `application`, `facet`, `provider`,
   `service`, `exchange`, `source`.
3. **Member level**: `capability`, `serves`, `creates`, `consumes`, `request`,
   `handle`, `live_channel`, `stream`, `log`, `materialized`.
4. **Annotation level**: `id`, `legacy method`, `requires capability`,
   `authority`, `retention`, `affinity`, `payload`, `cursor`, `coalesce`,
   `replay`, `decode`.

This is similar to:

```text
C++ namespace -> class -> member -> attribute/convention
```

For Glade:

```text
package -> declaration block -> member block -> semantic annotation
```

That distinction matters because not every keyword should be a top-level
architectural concept. Some keywords declare durable objects. Some only
constrain those objects.

## Terminal Slice

In the terminal example, the hierarchy should read as:

```text
package griplab_terminal
  workspace repo
    capability open_terminal
    capability control_terminal
    capability read_terminal

  application griplab_terminal
    facet terminal
    provider griplab_terminal_backend

  facet terminal
    creates source TerminalSession
    consumes live_channel TerminalPty
    consumes stream TerminalLiveOutput
    consumes log TerminalOutput
    consumes materialized TerminalScreen

  provider griplab_terminal_backend
    serves exchange OpenTerminal
    serves source TerminalSession
    serves live_channel TerminalPty
    serves stream TerminalLiveOutput
    serves log TerminalOutput
    serves materialized TerminalScreen

  exchange OpenTerminal
    request { target_ref, cols, rows, shell }
    creates source TerminalSession
    returns handle TerminalHandle

  source TerminalSession
    handle TerminalHandle
    live_channel TerminalPty
    stream TerminalLiveOutput
    log TerminalOutput
    materialized TerminalScreen
```

The key split is that `source TerminalSession` is the class-like resource
declaration, while `live_channel`, `stream`, `log`, and `materialized` are views
or members on that resource.

