# Glial: System Overview & Vocabulary
**Document ID:** GLIAL-DOC-001
**Version:** 1.0
**Status:** Core Spine (Normative)
**Dependency:** None

---

## 1. Introduction

**Glial** is a distributed "Nervous System" for Agentic SaaS applications. It provides a real-time, isomorphic, mutable context graph that synchronizes state between a User's browser, a Backend infrastructure, and AI Agents.

### 1.1 The Core Problem
Traditional web architectures suffer from the "Request/Response" impedance mismatch. The frontend "pulls" data from the backend, manages a local cache, and manually synchronizes state. This becomes exponentially more complex when adding AI Agents that need to "see" and "manipulate" the user's session state in real-time.

### 1.2 The Glial Solution
Glial inverts this model. It acts as a **State Multiplexer**. It does not execute business logic; it manages a deterministic graph of dependencies.
* **State** is a living graph, not a database table.
* **Logic** runs on the edges (Clients/Providers), not in the core.
* **Synchronization** is automatic and push-based ("Drips").
* **Remote Execution Interface** is Tap/RPC-native: authorized remote taps can be invoked over the same Glial protocol, reducing custom CRUD endpoint surface.

---

## 2. System Architecture: The Three Planes

Glial enforces a strict separation of concerns into three distinct planes. This separation is the primary mechanism for scalability and determinism.

### 2.1 The Connection Plane (The "Switchboard")
* **Component:** Glial Server / Relay.
* **Responsibility:** Connection termination, routing, and backpressure.
* **Behavior:**
    * Terminates WebSockets and manages Hanging GET loops.
    * Performs initial Authentication (Handshake).
    * Routes messages to the correct *State Plane* shard based on GraphID.
    * Enforces rate limits and quotas.
* **Constraint:** **Zero Business Logic.** The Connection Plane never parses the payload of a Drip beyond routing headers. It never accesses a database.

### 2.2 The State Plane (The "Brain")
* **Component:** Glial Reactor / Grip Engine.
* **Responsibility:** Deterministic graph execution.
* **Behavior:**
    * Maintains the in-memory Context Graph for active sessions.
    * Processes incoming *Operations* (Deltas).
    * Calculates derived values (Dependencies).
    * Emits *Effect Requests* when a node requires external data.
* **Constraint:** **Zero IO.** The State Plane is pure software. It cannot make HTTP calls, query databases, or read files. It is strictly a state machine.

### 2.3 The Effect Plane (The "Hands")
* **Component:** Glial Clients (Providers).
* **Responsibility:** Side-effects, IO, and Business Logic.
* **Behavior:**
    * **Browser Client:** Renders UI, captures user input (Intent).
    * **Backend Client:** Connects to Databases, LLMs, Payment Gateways.
    * Receives *Effect Requests* from the State Plane.
    * Executes logic and writes *Authoritative Data* back to the graph.
* **Constraint:** **Exclusive IO.** All side-effects must happen here.

---

## 3. Core Vocabulary

These terms are normative across all Glial specifications.

### 3.1 The Graph Objects
* **Graph (Context Graph):** The totality of state for a specific session or user. It is a directed graph where nodes are data points and edges are dependencies.
* **Scope:** A namespace within the Graph that defines lifecycle and permissions (e.g., `session.*`, `user.*`, `system.*`).
* **Grip:** A named node in the graph (e.g., `user.profile.email`). It has a unique ID and a typed value.
* **Tap:** A connection point to a Grip.
    * **Source Tap:** Feeds data into the graph (e.g., from a DB).
    * **Sink Tap:** Consumes data from the graph (e.g., to render UI).
* **Drip:** A discrete unit of change (Delta) propagating through the graph.

### 3.2 The Actors
* **Provider:** An entity in the Effect Plane that registers itself as the *Authoritative Source* for a specific set of Grips (e.g., "I provide `data.users.*`").
* **Client:** An entity in the Effect Plane that connects to the Glial Server. A Client can be a Consumer (Browser) or a Provider (Backend Service), or both.
* **Agent:** An autonomous Client (usually Python-based) that observes the graph and manipulates `intent` or `data` nodes to achieve a goal.

### 3.3 The Protocol Concepts
* **Intent:** A write operation initiated by a non-authoritative client (e.g., Browser writes `intent.save`). It is a *request*, not a fact.
* **Effect Request:** A signal from the State Plane to a Provider asking for work to be done (e.g., "Load User 123").
* **Authority:** The privilege to write to restricted Scopes (`data.*`, `system.*`). Only authenticated Providers hold Authority.
* **Remote Tap RPC:** A correlated request/response call over Glial transport used for imperative operations that do not fit pure state subscription semantics, while still obeying scope/capability checks and auditability requirements.

---

## 4. Deployment Topologies

Glial supports different deployment models while maintaining the Three-Plane logic.

### 4.1 Single-Process (Development / Simple)
* **Structure:** The Connection Plane and State Plane run in the same Python process (Asyncio).
* **Use Case:** Local development, small tools, low-traffic apps.
* **State:** Held in process memory.

### 4.2 Distributed / Sharded (Production)
* **Relay Fleet:** A layer of stateless Connection Plane servers (Nginx/Go/Python) handling WebSockets.
* **Reactor Fleet:** A layer of stateful State Plane servers, sharded by `GraphID`.
* **Registry:** A generic key-value store (Redis) mapping `GraphID -> ReactorIP`.
* **Use Case:** High-scale SaaS.

### 4.3 Sidecar / Mesh (Enterprise)
* **Structure:** Glial runs as a sidecar to existing microservices.
* **Use Case:** Integrating Glial into legacy architectures where the "Provider" is a legacy Java/Go service connecting to the local Glial sidecar.

---

## 5. Non-Goals

To maintain focus, Glial explicitly avoids certain responsibilities:

1.  **Glial is NOT a Database:** It does not guarantee long-term durability. It is a *coordination* layer. Persistent data must be written to a real DB by a Provider.
2.  **Glial is NOT a Job Queue:** While it buffers Effect Requests, it is designed for real-time interactivity (<100ms), not long-running background jobs (hours).
3.  **Glial is NOT an IDP (Identity Provider):** It consumes JWTs/Tokens but does not manage user passwords or MFA.

---
