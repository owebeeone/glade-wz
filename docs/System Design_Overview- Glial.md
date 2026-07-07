# **System Design & Requirements: Project Glial**

**Version:** 0.8 (Python-First Definitions)

**Status:** Outline / Draft

**Core Concept:** A Distributed, Isomorphic, Mutable Context Graph where the Server is a State Broker, not an Execution Runtime.

## **1\. Executive Summary**

**Glial** is a distributed "Nervous System" for Agentic SaaS. Unlike traditional architectures where the backend server holds the business logic and database connections, Glial acts as a **State Multiplexer**.

**The Paradigm Shift:**

The Glial Server does **not** execute business logic (SQL queries, API calls). It only manages the **Graph Topology** and **State Synchronization**.

* **Real Taps** (Logic/IO) exist *only* on Clients (Effect Plane).  
* **Browser Client:** Provides UI Taps (DOM, User Input).  
* **Backend Client:** A headless Python process that provides Infra Taps (Database, OpenAI, Stripe).  
* **The Glial Server:** Bridges the two, allowing a Browser Grip to transparently drive a Backend Grip via a deterministic State Plane.

## **2\. Topological Architecture**

The system is strictly decomposed into three planes to ensure scalability and determinism.

### **2.1 The Three Planes**

1. **Connection Plane (The Glial Server)**  
   * **Role:** The "Dumb" Switchboard.  
   * **Tech:** Python (FastAPI/Uvicorn) or Go.  
   * **Responsibilities:**  
     * **Termination:** Owns sockets, auth handshakes, and hanging GET loops.  
     * **Routing:** Routes incoming messages to the correct in-process Engine.  
     * **Backpressure:** Manages client quotas and slow-consumer dropping/coalescing.  
     * **Constraint:** *No business logic. No persistence. No tool calls.*  
2. **State Plane (The Grip Engine / Reactor)**  
   * **Role:** The Deterministic Brain.  
   * **Tech:** Python (In-Memory State Machine).  
   * **Responsibilities:**  
     * **Applies Ops:** Processes incoming deltas from connections.  
     * **Computes:** Calculates derived values and tracks dependencies.  
     * **Emits:** Generates effects\_requested events (not execution).  
     * **Constraint:** *Purely in-memory. Deterministic. No IO.*  
3. **Effect Plane (The Clients / Providers)**  
   * **Role:** The Hands (IO & Execution).  
   * **Tech:** TypeScript (Browser), Python (Backend).  
   * **Responsibilities:**  
     * **Executes:** Performs all IO (DB, HTTP, Queues, LLMs, Filesystem).  
     * **Feeds:** Pushes results back as authoritative ops into the State Plane.  
     * **Constraint:** *The only place where side-effects occur.*

### **2.2 The Components**

1. **Glial Relay (Connection Plane Host)**  
   * Public-facing gateway.  
   * Stateless; scales horizontally.  
   * Forwards packets to the specific Reactor holding the user's graph.  
2. **Glial Reactor (State Plane Host)**  
   * Stateful process (sharded by GraphID).  
   * Hosts the GripEngine.  
   * Buffers "Effect Requests" for Providers.  
3. **Glial Clients (Effect Plane Agents)**  
   * **Type A: Browser Client:**  
     * Writes intent.\* grips (e.g., intent.form.submit).  
     * Reads view.\* grips.  
   * **Type B: Backend Client (Provider):**  
     * Subscribes to intent.\*.  
     * Executes logic (DB write).  
     * Writes authoritative.\* grips (e.g., data.user.profile).  
4. **Glial Registry (The Switchboard)**  
   * **Tech:** Redis.  
   * **Data Model:** Map\<GraphID, ReactorIP\>.  
   * **Logic:** Ensures a specific User's graph lives on exactly one Reactor node to prevent state conflicts.

## **3\. Data Architecture: Subgraphs & Security**

The "Graph" is a composition of multiple isolated scopes with strict write permissions enforced at the Connection Plane.

### **3.1 Subgraph Scopes & Permissions**

| Scope Prefix | Owner | Write Access | Description |
| :---- | :---- | :---- | :---- |
| intent.\* | Browser | **Public** (Browser/User) | User desires (e.g., "I want to save"). |
| session.\* | Browser | **Public** (Browser/User) | Ephemeral UI state (scroll, focus). |
| data.\* | Backend | **Restricted** (Provider) | Authoritative domain data (User Profile). |
| system.\* | Admin | **Restricted** (Admin) | Feature flags, maintenance mode. |
| view.\* | Engine | **Read-Only** (Derived) | Computed values from the State Plane. |

### **3.2 Protocol Optimization (Batching & Ordering)**

* **The "Tick" Concept:** The Reactor collects incoming messages for X milliseconds (e.g., 16ms).  
* **Coalescing:** If grip:A changes 5 times in one tick, only the final value is broadcast.  
* **Ordering:** Every mutation has a monotonic op\_id.  
* **Reconnection:** Clients send last\_seen\_cursor on connect. Server sends "Deltas since cursor" or "Snapshot \+ New Cursor".

## **4\. The Glial Client API (Distributed Primitives)**

The API mirrors grip-core, adding network-aware Taps. The key distinction is between **Multi-Writer Atoms** and **Single-Writer Providers**.

### **4.1 Distributed Tap Types**

1. **AtomTap (The Shared Variable)**  
   * **Analogy:** React.useState but distributed across the network.  
   * **Cardinality:** **Multi-Writer**.  
   * **Modes:**  
     * **Simple (Last-Write-Wins):** For UI toggles (e.g., "Sidebar Open"). Valid where race conditions are visual only.  
     * **Optimistic CAS (Compare-And-Swap):** For logic requiring consistency (e.g., "Increment Counter", "Claim Ticket").  
       * **Client Flow:**  
         1. **Read:** Get current local value v=5.  
         2. **Optimistic Update:** Render v=6 immediately.  
         3. **Network:** Send CAS { id: "count", old: 5, new: 6 }.  
       * **Server Flow:**  
         1. Check current\_val.  
         2. If 5: Update to 6\. Broadcast 6\.  
         3. If \!= 5: Reject. Send REJECT { id: "count", current: 7 }.  
       * **Client Rebase:**  
         1. Receive REJECT (current is 7).  
         2. **Re-Run Logic:** 7 \+ 1 \= 8\.  
         3. **Update UI:** Flash 8\.  
         4. **Retry:** Send CAS { id: "count", old: 7, new: 8 }.  
       * **Fallback:** If retries \> 3, Force Set (if allowed) or Show Error.  
2. **ProviderTap / DestinationTap (The Source of Truth)**  
   * **Analogy:** React.useEffect fetching data from an API.  
   * **Cardinality:** **Single-Provider**. The Reactor enforces that only *one* active client can register as the Provider for a specific Grip ID.  
   * **Behavior:**  
     * **Registration:** Client sends PROVIDE { grip: "data.clients.list" }.  
     * **Locking:** Server rejects any other client trying to PROVIDE this grip.  
     * **Writing:** Only the Provider can write to this Grip.  
3. **PlaceholderTap (The Consumer)**  
   * **Purpose:** A "Ghost" tap for reading remote data.  
   * **Behavior:**  
     * Automatically created if useGrip() is called without a local provider.  
     * Sends SUBSCRIBE messages to the Engine.

## **5\. The Glial Internal API (Engine Contract)**

The Glial Server (Connection Plane) communicates with the Engine (State Plane) via a strict internal API, not a network socket.

### **5.1 Engine Surface Area**

class GripEngine:  
    def apply\_client\_op(self, connection\_id: str, op: Operation) \-\> None:  
        """Apply a write intent from a connection."""  
        pass

    def subscribe(self, connection\_id: str, selector: str) \-\> None:  
        """Register interest in a set of grips."""  
        pass

    def tick(self) \-\> EngineOutput:  
        """Advance the state machine and flush deltas."""  
        pass

### **5.3 Issue: Type Safety Across Languages (Python-First Definitions)**

* **Scenario:** Divergence between Backend (Python) and Frontend (TS) data structures leads to runtime errors.  
* **Solution: Python-as-Schema (Reflection).**  
  * **Source of Truth:** Python modules define the entire Grip Catalog.  
  * **Data Structures (Values):** Grip payloads are defined as frozen=True dataclasses. This enforces immutability and provides clear type hints.  
  * **Grip Identities:** Defined as instantiated objects or class attributes with rich metadata.  
  * **AI Context:** Every Grip definition includes a description field tailored for AI comprehension (e.g., *"This grip controls the visibility of the checkout modal. Set to true only when cart is not empty."*).  
  * **Translation:** A build-time tool (glial-gen) imports the Python modules, reflects on the types/docstrings, and generates:  
    * grips.ts: TypeScript interfaces and Grip constants.  
    * agent\_tools.json: A tool definition file for the AI Agent.

## **6\. Async State & Diagnostics**

Since IO is decoupled, the system must explicitly handle "The Gap" between request and result.

### **6.1 First-Class Grip States**

1. absent: No provider is registered.  
2. connecting: Provider registered but not yet connected.  
3. loading: Provider received request, executing IO.  
4. live: Data is fresh.  
5. stale: Provider disconnected, data is old.  
6. error: Provider failed.

## **7\. Core Use Cases (Revised)**

### **7.1 The Intelligent Form Filler (AI-Driven Data Entry)**

* **Goal:** User uploads a PDF; AI automatically fills a form.  
* **Flow:**  
  1. **Browser (Effect):** User drags file. Writes intent.upload.file \= \<Blob\>.  
  2. **Engine (State):** Accepts intent. Emits effect\_request: parse\_document(file).  
  3. **Backend (Provider):** Subscribed to effect\_request. Receives blob. Calls LLM (IO).  
  4. **Backend (Provider):** Writes data.form.fields \= { name: "John" } (Authoritative).  
  5. **Engine (State):** Updates graph. Emits deltas to Browser.  
  6. **Browser (Effect):** Reacts to deltas. Renders "John" in the name field.

### **7.2 The "Co-Pilot" Debugger (Session Twin)**

* **Goal:** Support staff fixes a user error live.  
* **Flow:**  
  1. **Support Agent (Effect):** Connects to User's GraphID with read\_only scope.  
  2. **Engine (State):** Streams session.\* deltas to Support Agent.  
  3. **Support Agent (Effect):** Sees mirrored form. Notices typo. Writes intent.suggest\_fix \= { field: "date", value: "2023-10-10" }.  
  4. **Backend (Provider):** Validates fix. Writes data.form.date \= "2023-10-10".  
  5. **Engine (State):** Propagates change to User. User sees fixed date.

## **8\. Application Architecture & Context Allocation**

A non-trivial Glial application is structured as a hierarchy of contexts that are "mounted" and "unmounted" based on lifecycle events.

### **8.1 The Context Hierarchy (The "Onion" Model)**

The Graph for a connected user is not a flat list; it is a composition of layered contexts.

1. **The Socket Context (scope:session):**  
   * **Created:** Immediately upon WebSocket connection.  
   * **Content:** session.id, session.browser\_capabilities, session.route.path.  
   * **State:** Anonymous. Contains the "Login Form" state.  
2. **The Identity Context (scope:user):**  
   * **Created:** After successful authentication (JWT exchange).  
   * **Mechanism:** The Glial Server "Mounts" the persisted User Graph *into* the Session Graph.  
   * **Content:** data.user.profile, data.user.settings.  
   * **Persistence:** Changes here are Drip-fed to the Backend Client for DB storage.  
3. **The Workspace/Team Context (scope:team):**  
   * **Created:** When session.route.path enters /workspace/123.  
   * **Content:** data.projects.list, data.team.members.  
   * **Sharing:** This subgraph is strictly shared among all users who are members of Team 123\.

### **8.2 The Drip Resolution Lifecycle (Server-Side Resolution)**

How the graph reacts when a Client requests data that requires parameters from *another* part of the graph.

1. **Phase 1: Intent (Client-Side Optimistic):**  
   * Client changes session.filter.category to "Books".  
   * Client *may* optimistically render a "Loading..." state or a predicted list.  
   * Client pushes session.filter.category \= "Books" to Glial Server.  
2. **Phase 2: Topology Check (Server-Side):**  
   * Glial Server receives the drip.  
   * It updates the Session Graph: filter.category \= "Books".  
   * It identifies that view.products.list (a Proxy Node) depends on filter.category.  
3. **Phase 3: Effect Request (Server-Side Orchestration):**  
   * **Optimization:** The Server *already has* the value of filter.category. It does **not** need to ask the Client for it.  
   * The Server bundles the dependencies: { category: "Books" }.  
   * The Server sends an EffectRequest to the Backend Provider: GetProducts(category="Books").  
4. **Phase 4: Authoritative Write (Provider-Side):**  
   * Backend Client receives request. Executes DB Query.  
   * Backend Client writes data.products.list \= \[...\] to the Server.  
5. **Phase 5: Reconciliation (Server-Side):**  
   * Server updates data.products.list.  
   * Server pushes the new list to the Browser Client.  
   * **Result:** The Browser Client receives the new list *without* ever knowing that an API call happened. It just saw two state changes: filter changed, then list changed.

## **9\. Implementation Nuances (Security & Performance)**

### **9.1 Security & Validation**

* **Interceptor Layer:** The Connection Plane enforces that Browsers can *only* write to intent.\* or session.\*.  
* **Schema Validation:** Providers must validate intent payloads before executing effects.

### **9.2 Backpressure & Slow Clients**

* **Problem:** One slow tab causes memory buildup in the Engine.  
* **Solution:** Per-connection output queues in the Glial Server.  
  * If queue \> HighWaterMark: Drop intermediate deltas (coalesce).  
  * If queue \> MaxLimit: Terminate connection (Force Reconnect/Snapshot).

## **10\. Developer Experience (DX)**

### **10.1 "Zero-API" Architecture**

* **Backend:** Define a Provider: @provider("data.users.list") def get\_users(intent): ...  
* **Frontend:** Use the Grip: const users \= useGrip("data.users.list");  
* **RPC Path:** For imperative commands that are not natural state subscriptions, invoke an authorized remote tap via Glial RPC (same transport, correlated request/response).  
* **Result:** The network, auth, and state synchronization are abstracted away, and most app-specific CRUD endpoints are eliminated.

### **10.2 "Spreadsheet Consistency"**

* Dependencies are tracked automatically. If data.user.role changes to "Viewer", the view.admin\_panel.visible node recalculates in the State Plane and pushes false to the Browser immediately.

### **10.3 "Code-as-Contract" (Python \-\> TS)**

* **Workflow:** Developers define data models in src/grips/definitions.py using Python dataclasses.  
* **Automation:** A watcher detects changes and runs glial-gen.  
* **Output:**  
  * src/grips/definitions.ts is updated instantly.  
  * Frontend developers get Intellisense for free.  
  * AI Agents get updated context descriptions for free.

### **10.4 "RPC-to-Remote-Taps" (Endpoint Elimination)**

* **Requirement:** Glial supports RPC-style invocation of remote taps over the same authenticated channel used for state sync.  
* **Security:** RPCs obey the same scope/capability model as grip writes.  
* **Observability:** Every RPC has request/response correlation ids and audit metadata.  
* **Scope:** Keep traditional HTTP endpoints only for control-plane concerns (auth/token issuance, webhook ingress, health, metrics, admin operations).

## **11\. Next Steps**

1. Implement the **Python-to-TS Reflection Generator** (glial-gen).  
2. Define the initial **Grip Catalog** for the pilot application.  
3. Build the **Glial Relay** prototype (FastAPI \+ Websockets) enforcing the Three-Plane boundary.
