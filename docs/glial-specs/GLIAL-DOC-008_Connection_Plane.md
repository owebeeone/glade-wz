# Connection Plane Implementation
**Document ID:** GLIAL-DOC-008
**Version:** 1.0
**Status:** Subsystem Design
**Dependency:** GLIAL-DOC-002, GLIAL-DOC-003

---

## 1. Introduction
The **Connection Plane** (Glial Relay) is the stateless frontend of the system. It manages high-concurrency WebSocket connections, protocol parsing, authentication enforcement, and message routing to the State Plane.

---

## 2. Architecture & Stack
* **Language:** Python 3.12+ (Asyncio) or Go (Future Optimization).
* **Server:** FastAPI + Uvicorn (WebSockets) / Hypercorn.
* **Scale Strategy:** Horizontal Autoscaling behind a Layer 4/7 Load Balancer (AWS ALB / Cloudflare).

### 2.1 Component Diagram
```
[Client] <--> [LB] <--> [Connection Manager]
                              |
                        [Auth Guard (DOC-003)]
                              |
                        [Router (DOC-011)]
                              |
                        [ZeroMQ / IPC]
                              |
                        [State Plane (Reactor)]
```

---

## 3. Connection Lifecycle

### 3.1 Handshake Handling
1.  **Upgrade:** Accept `Upgrade: websocket`.
2.  **Await Hello:** Set a 5s timeout for the `HI` frame (GSP).
3.  **Validate:** Pass JWT to `AuthGuard`.
4.  **Assign ID:** Generate `conn_id` (UUIDv4).
5.  **Route:** Query `Registry` (Doc 11) for the target Reactor.
6.  **Bind:** Establish a bidirectional stream (Queue/ZMQ) to the specific Reactor instance.

### 3.2 Heartbeats & Keep-Alives
* **Server-Side:** Send `PING` every 30s.
* **Timeout:** If no `PONG` (or data) within 60s, terminate connection.
* **Hanging GET:** For environments blocking WS, implement a long-polling fallback loop `/v1/sync/poll`.

---

## 4. Backpressure & Quotas

### 4.1 Per-Connection Queues
* Maintain an outgoing `asyncio.Queue` per socket.
* **High Water Mark (Soft):** 100 pending messages. Trigger `Coalescing` (merge pending OPs for same Grip).
* **Max Limit (Hard):** 500 pending messages. Drop connection with `ERR_QuotaExceeded`.

### 4.2 Fanout Optimization
* **Broadcasts:** When a Reactor emits a broadcast delta (e.g., `user.status` change), the Connection Plane receives *one* message.
* **Local Fanout:** The Connection Manager iterates over local sockets subscribed to that GraphID and enqueues the frame. This reduces IPC overhead.

---

## 5. Security Integration
* **TLS:** Terminated at LB or Relay.
* **Rate Limiting:** Token Bucket per IP (100 req/sec) and per GraphID (1000 ops/sec).
* **Payload Inspection:** Enforce max payload size (e.g., 1MB) before passing to State Plane.

