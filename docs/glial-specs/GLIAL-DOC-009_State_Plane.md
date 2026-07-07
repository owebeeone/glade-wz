# State Plane Implementation
**Document ID:** GLIAL-DOC-009
**Version:** 1.0
**Status:** Subsystem Design
**Dependency:** GLIAL-DOC-002, GLIAL-DOC-007, GLIAL-DOC-006

---

## 1. Introduction
The **State Plane** (Reactor) is the deterministic engine that holds the mutable graph. It executes the `GripEngine` runtime, resolves dependencies, and manages the lifecycle of data within a session.

---

## 2. Grip Engine Runtime

### 2.1 The Core Loop (`tick`)
The engine operates on a discrete event loop tick (e.g., 16ms or 60Hz equivalent).

```python
class GripEngine:
    def tick(self):
        # 1. Apply Buffer
        while self.op_buffer:
            op = self.op_buffer.popleft()
            self._apply_op(op)

        # 2. Recompute Dirty Nodes
        resolved_ops = self._recalc_graph()

        # 3. Emit Effects
        self._dispatch_effects()

        # 4. Return Deltas
        return resolved_ops
```

### 2.2 Dependency Graph (DAG)
* **Structure:** `Dict[GripID, Set[GripID]]` (Forward and Reverse dependencies).
* **Resolution:** Topological Sort is *not* strictly required per tick if using a "Dirty Propagation" set.
    * Mark mutated node as Dirty.
    * Add children to Dirty Set.
    * Re-evaluate Dirty Set in dependency order.

### 2.3 PlaceholderTap Resolution
* When a `PlaceholderTap` (Proxy) is encountered:
    1.  Check if `Authoritative Source` is registered (Doc 5).
    2.  If yes, evaluate `Destination Params`.
    3.  Emit `EFFECT_REQ` (Doc 6) to the Provider.
    4.  Set Grip State to `loading` (if not already `live`).

---

## 3. Persistence & Snapshots

### 3.1 Snapshot Strategy
* **Format:** JSON (via Dataclass serialization).
* **Trigger:**
    * **Time-based:** Every 5 minutes.
    * **Op-count:** Every 1000 Ops.
    * **Shutdown:** On Reactor shutdown/rebalance.
* **Storage:** S3 or Blob Store (via Interface).

### 3.2 Hydration
* On `Mount`, fetch latest Snapshot.
* Replay any Ops from the `Write Ahead Log` (WAL) if using a durable queue, or simply load Snapshot if accepting "lossy" session state (standard for ephemeral sessions).

---

## 4. Conflict Resolution
* **AtomTaps:** Apply Logic defined in Doc 4 (LWW, CAS).
* **Version Vectors:** (Optional Extension) Store Vector Clock per Grip to detect split-brain merges if multi-master is enabled later. MVP uses Single-Leader per GraphID.

