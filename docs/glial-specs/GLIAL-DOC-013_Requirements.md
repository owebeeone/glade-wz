# Glial Requirements & Non-Functional Requirements (NFRs)
**Document ID:** GLIAL-DOC-013
**Version:** 1.0
**Status:** Product Requirements
**Dependency:** GLIAL-DOC-001

---

## 1. Goals
* **Agentic Native:** First-class support for AI agents manipulating UI state.
* **Zero-API Frontend:** Frontends should not write fetch logic.
* **Isomorphic State:** State moves seamlessly between Server and Client.
* **Endpoint Minimization:** Application features should use Grip/Tap contracts instead of custom CRUD endpoints wherever possible.

## 2. Functional Requirements
* **FR-01:** System MUST support multi-writer shared state (AtomTaps).
* **FR-02:** System MUST support single-writer authoritative state (ProviderTaps).
* **FR-03:** System MUST resume sessions across network interruptions without data loss (Gap Detection).
* **FR-04:** AI Agents MUST be able to observe and inject "Intent" into any user session (Permissions permitting).
* **FR-05:** System MUST support RPC-style invocation of authorized remote taps over the Glial protocol (request/response correlation) so application logic can be executed without introducing app-specific HTTP CRUD endpoints.
* **FR-06:** RPC-style remote tap invocation MUST enforce the same scope/capability authorization model as standard OP writes and MUST produce auditable request and response metadata.

---

## 3. Non-Functional Requirements (NFRs)

### 3.1 Performance
* **Latency:** End-to-end propagation (Client A -> Server -> Client B) < 100ms (p95).
* **Throughput:** Support 1,000 Ops/sec per GraphID.
* **Scale:** Support 10,000 active concurrent connections per Relay node (c5.large).

### 3.2 Reliability
* **Availability:** 99.9% uptime for the Connection Plane.
* **Durability:** No acknowledged Op should be lost during a graceful shutdown.

### 3.3 Security
* **Isolation:** A compromised Provider MUST NOT be able to write to Scopes it does not own.
* **Encryption:** All data in transit MUST be TLS 1.3 encrypted.

---

## 4. Rollout Phases
1.  **Alpha:** Single-process Python implementation. In-memory only.
2.  **Beta:** Distributed Relay/Reactor. Redis Registry.
3.  **GA:** Multi-region support. S3 Snapshots.
