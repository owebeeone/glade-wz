# Glial Sync Protocol (GSP)
**Document ID:** GLIAL-DOC-002
**Version:** 1.0
**Status:** Protocol Specification
**Dependency:** GLIAL-DOC-001

---

## 1. Introduction

The **Glial Sync Protocol (GSP)** is the low-level transport layer for the Glial system. It defines how a Connection Plane (Server) and an Effect Plane (Client) exchange messages over a persistent connection (WebSocket) or an emulation layer (Hanging GET).

### 1.1 Scope
GSP handles:
* Session Establishment (Handshake).
* Operation Streaming (Deltas).
* Flow Control (Backpressure & Heartbeats).
* Session Recovery (Reconnects).
* RPC-style invocation of remote taps (request/response correlation).

GSP **does not** handle:
* The semantics of specific operations (see Doc 4: AtomTap).
* The resolution of dependencies (see Doc 9: State Plane).

---

## 2. Wire Format

All GSP messages are serialized as JSON (text mode) or MsgPack (binary mode). The outer envelope is identical for both.

### 2.1 The Envelope
Every message typically follows this structure:

```json
[
  "TYPE",       // Message Type Code (string/int)
  "PAYLOAD",    // The content (object/array)
  "META"        // Optional metadata (request_id, trace_id)
]
```

### 2.2 Message Types (Standard Registry)

| Code | Name | Direction | Description |
| :--- | :--- | :--- | :--- |
| `HI` | `HELLO` | C -> S | Client initiating handshake. |
| `OK` | `WELCOME` | S -> C | Server accepting session. |
| `RESYNC` | `RESYNC_REQUEST` | C -> S | Client requesting replay from a known cursor/op. |
| `SUB` | `SUBSCRIBE` | C -> S | Client asking for a set of Grips. |
| `UNSUB` | `UNSUBSCRIBE` | C -> S | Client stopping interest. |
| `OP` | `OPERATION` | C <-> S | A mutation or delta (e.g., SET, APPEND). |
| `SNAP` | `SNAPSHOT` | S -> C | Full state dump for a scope. |
| `EFFECT_REQ` | `EFFECT_REQUEST` | S -> C | State Plane asks a Provider to perform work. |
| `RPC_REQ` | `RPC_REQUEST` | C <-> S | Correlated RPC request frame (caller -> Relay, Relay -> Provider). |
| `RPC_RES` | `RPC_RESPONSE` | C <-> S | Correlated response for a prior `RPC_REQ`. |
| `RPC_CANCEL` | `RPC_CANCEL` | C <-> S | Cancellation request for an in-flight RPC. |
| `ACK` | `ACKNOWLEDGE` | C <-> S | Confirming receipt of op_id (Flow Control). |
| `ERR` | `ERROR` | C <-> S | Protocol or routing/security error. |
| `PING` | `PING` | C <-> S | Keep-alive. |

---

## 3. Session Lifecycle

### 3.1 Handshake
1.  **Client Connects:** Opens WebSocket to `wss://api.glial.io/v1/socket`.
2.  **Client Hello:** Sends `["HI", { "token": "JWT...", "caps": ["gzip"], "cursor": "last_op_105" }]`.
3.  **Server Verification:**
    * Validates JWT (see Doc 3).
    * Checks `last_op_105` against the Reactor's history buffer.
4.  **Server Welcome:**
    * **Scenario A (Clean Resume):** `["OK", { "session_id": "sess_abc", "resume": true }]`.
    * **Scenario B (Full Reset):** `["OK", { "session_id": "sess_new", "resume": false }]` followed immediately by `SNAP`.

### 3.2 Operation Streaming & Ordering
* **Monotonicity:** Every `OP` from the Server has a monotonically increasing `op_id` (integer or lexicographical string).
* **Gaps:** If a Client receives `op_101` and then `op_103`, it detects a gap. It MUST send `["RESYNC", { "from": "op_101" }]` or disconnect and reconnect.
* **Batching:** The Server MAY coalesce multiple Ops into a single transport frame if they occur within the same "Tick". The wire format treats the payload as an Array of Ops in this case.

### 3.3 Flow Control
* **Server -> Client:** If the underlying TCP buffer fills (slow client), the Connection Plane MUST drop the connection (to protect Server memory). It should NOT drop random messages.
* **Client -> Server:** Clients must respect `429 Too Many Requests` if sent as an `ERR` frame.

### 3.4 RPC Correlation Rules
* Every `RPC_REQ` MUST include a caller-generated `rpc_id` unique within the caller connection.
* Every terminal RPC outcome MUST include that same `rpc_id` in either:
    * `RPC_RES` (`ok: true|false`) for application-level completion.
    * `ERR` (`ref_rpc`) for protocol/security/routing failures.
* The Relay MUST keep correlation state until terminal outcome or timeout.
* If timeout occurs before terminal outcome, the Relay MUST return `ERR` with `code: "E_RPC_TIMEOUT"` and `ref_rpc`.

---

## 4. Subscriptions

Clients do not receive the whole graph. They subscribe to specific **Scopes** or **Selectors**.

* `["SUB", { "id": "sub_1", "selector": "session.*" }]`
* `["SUB", { "id": "sub_2", "selector": "view.dashboard.*" }]`

The Server guarantees that all `OP` messages related to these Grips will be forwarded.

---

## 5. Remote Tap RPC

RPC supports imperative calls that are not naturally modeled as subscriptions.

### 5.1 Caller Request
```json
["RPC_REQ", {
  "rpc_id": "rpc_01HZX2...",
  "tap": "rpc.user.create",
  "args": { "email": "a@example.com" },
  "timeout_ms": 5000,
  "idempotency_key": "idem_2e0d...",
  "expect": "unary"
}, {
  "trace_id": "t_abc"
}]
```

### 5.2 Server Routing & Enforcement
* Relay MUST authenticate caller and enforce capabilities before forwarding (Doc 3).
* Relay MUST resolve target Provider using registration/selection rules (Doc 5).
* Relay MUST append caller context when forwarding to Provider:
    * `caller.sub`
    * `caller.connection_id`
    * `caller.capabilities` (or an equivalent signed/filtered representation)

### 5.3 Provider Response
Success:
```json
["RPC_RES", {
  "rpc_id": "rpc_01HZX2...",
  "ok": true,
  "result": { "user_id": "u_123" }
}, {
  "trace_id": "t_abc"
}]
```

Application failure:
```json
["RPC_RES", {
  "rpc_id": "rpc_01HZX2...",
  "ok": false,
  "error": { "code": "E_VALIDATION", "msg": "email already used" },
  "retryable": false
}]
```

Protocol/security failure:
```json
["ERR", { "code": "E_ACCESS_DENIED", "ref_rpc": "rpc_01HZX2..." }]
```

### 5.4 Ordering, State, and Idempotency
* RPC response ordering is independent from `OP` ordering.
* `RPC_RES(ok=true)` does NOT imply state mutation unless associated `OP` frames are applied.
* Callers SHOULD provide `idempotency_key` for non-read-only RPCs.
* Relay/Provider MAY deduplicate repeated requests that share `(caller, tap, idempotency_key)`.

---

## 6. Extensions
GSP supports protocol extensions via the `META` field.
* **Tracing:** `trace_id` for distributed debugging.
* **Priority:** `prio: "high"` for critical UI signals.
