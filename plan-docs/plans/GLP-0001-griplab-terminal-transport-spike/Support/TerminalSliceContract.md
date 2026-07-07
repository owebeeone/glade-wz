# Terminal Slice Contract

Status: disposable spike contract
Plan: `GLP-0001`

Purpose: define the smallest terminal surface needed to test libp2p and replay
behavior for GripLab.

This is not final Glade architecture.

## Slice

```text
OpenTerminal exchange
  -> TerminalPty live channel
  -> TerminalOutput append log
  -> Grip terminal tap shape
  -> console diagnostics
```

## V0 Assumptions

- One provider owns one PTY instance.
- PTY state is provider-local and non-migratable.
- Input and resize are low-latency live-channel frames.
- Output is live-streamed and appended to a replay log.
- Reattach resumes from a cursor when available.
- Missing replay history is a diagnostic, not a fatal error.
- Resize is last-writer-wins.
- Collaboration policy is out of scope.

## Surface IDs

| Surface | Kind | ID |
| --- | --- | --- |
| Open terminal | exchange | `exchange:griplab.terminal.open` |
| Terminal PTY | live channel | `live:griplab.terminal.pty` |
| Terminal output | append log | `log:griplab.terminal.output` |
| Terminal diagnostic | diagnostic | `diag:griplab.terminal` |

## Frame Envelope

Use disposable JSON frames:

```json
{
  "frame_id": "frame:dev-0001",
  "kind": "terminal_output",
  "transport_peer_id": "peer:libp2p-id",
  "session_id": "session:dev-a",
  "terminal_id": "terminal:dev-1",
  "log_entry_cid": "zdpu...",
  "local_seq": 1,
  "ts_ms": 1730000000000,
  "payload": {}
}
```

`transport_peer_id` is transport-only. It is not a principal or authorization
identity.

## Frame Kinds

| Kind | Direction | Purpose |
| --- | --- | --- |
| `open_terminal` | client -> provider | Request a new terminal instance. |
| `open_terminal_result` | provider -> client | Publish terminal id and initial cursor. |
| `terminal_input` | client -> provider | Send typed bytes. |
| `terminal_resize` | client -> provider | Send cols/rows. |
| `terminal_output` | provider -> client | Send output bytes or text chunk. |
| `terminal_log_append` | provider -> log | Append output to replay history. |
| `terminal_attach` | client -> provider | Attach to live channel from optional cursor. |
| `terminal_close` | either | Close the live channel or PTY. |
| `terminal_diag` | either | Report diagnostic state. |

## Payload Sketches

### `open_terminal`

```json
{
  "cwd": "/workspace",
  "command": "/bin/zsh",
  "env": {},
  "cols": 120,
  "rows": 32
}
```

### `open_terminal_result`

```json
{
  "terminal_id": "terminal:dev-1",
  "live_surface_id": "live:griplab.terminal.pty",
  "output_log_id": "log:griplab.terminal.output",
  "cursor": "cursor:terminal:dev-1:origin"
}
```

### `terminal_input`

```json
{
  "bytes_b64": "bHMK"
}
```

`bytes_b64` is only the disposable JSON live-frame encoding. The replay log
MUST store terminal output as native bytes in the OrbitDB/IPLD payload.

### `terminal_resize`

```json
{
  "cols": 100,
  "rows": 28
}
```

### `terminal_output`

```json
{
  "bytes_b64": "Li4u",
  "cursor": "cursor:terminal:dev-1:zdpu...",
  "local_seq": 42
}
```

### `terminal_diag`

```json
{
  "code": "replay_unavailable",
  "severity": "warn",
  "message": "Requested cursor is no longer available.",
  "cursor": "cursor:terminal:dev-1:zdpu..."
}
```

## Replay Cursor

Cursor format for the spike:

```text
cursor:<terminal-id>:<entry-cid>
```

Rules:

- the authoritative replay cursor is the last-seen OrbitDB entry CID
- `origin` means no entry has been seen yet
- `local_seq` MAY be emitted for single-writer diagnostics only
- `local_seq` MUST NOT be required for replay
- terminal output SHOULD be coalesced into bounded log-entry windows before
  append
- attach with no cursor starts live-only plus available recent history
- attach with an old unavailable cursor emits `terminal_diag`
- attach with a valid cursor replays entries after the last-seen entry CID

## Lifecycle

```text
open -> live -> attach -> input/output/resize -> close
                  \
                   -> refresh -> attach(cursor) -> replay -> live
```

Provider crash behavior:

- emit diagnostic if possible
- live channel closes
- replay remains available only if the provider/log harness kept it
- creating a new terminal is explicit

## Grip Tap Shape

Mock and Glade-backed terminal taps should keep this shape:

```ts
type TerminalTap = {
  terminals: TerminalSummary[];
  activeTerminalId: string | null;
  output: TerminalOutputView;
  status: TerminalStatus;
  open(input: OpenTerminalInput): Promise<string>;
  attach(terminalId: string, cursor?: string): Promise<void>;
  sendInput(terminalId: string, bytes: Uint8Array): void;
  resize(terminalId: string, cols: number, rows: number): void;
  close(terminalId: string): void;
};
```

The UI should not know whether the tap is mock-backed, websocket-backed, or
libp2p-backed.

## Console Rows

Minimum visible rows:

| Row | Fields |
| --- | --- |
| provider | provider id, transport peer id, status |
| terminal | terminal id, provider id, cwd, command, status |
| live channel | terminal id, attached sessions, open/closed |
| output log | terminal id, latest entry cid, optional local seq, retention state |
| diagnostic | code, severity, terminal id, timestamp |

## Measurements

Record:

- p50 input echo latency
- p95 input echo latency
- burst-output behavior
- resize delivery behavior
- refresh/reattach behavior
- slow-reader behavior

## Non-Goals

- final Glade envelope
- final capability proof
- terminal collaboration policy
- provider migration
- durable storage
- generated helpers
- production security
