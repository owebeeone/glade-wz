# Routing & Lifecycle
**Document ID:** GLIAL-DOC-011
**Version:** 1.0
**Status:** Subsystem Design
**Dependency:** GLIAL-DOC-002, GLIAL-DOC-009

---

## 1. Introduction
This document defines how Glial routes requests to the correct Reactor instance and manages the lifecycle of graphs (Creation, Hibernation, Destruction).

---

## 2. The Registry

### 2.1 Data Model (Redis)
* **Key:** `glial:loc:{graph_id}` -> `ip:port` (TTL: 60s)
* **Key:** `glial:meta:{graph_id}` -> `{"created_at": ..., "owner": ...}`

### 2.2 Placement Strategy
* **Consistent Hashing:** Not used for *location* lookup (to allow rebalancing), but used for *initial placement*.
* **Least Loaded:** Registry tracks connection counts per Reactor. New Graphs are assigned to the least loaded Reactor.

---

## 3. Graph Lifecycle

### 3.1 Creation (The "Mount")
1.  Client Connects with `GraphID`.
2.  Relay checks Registry.
    * **Found:** Proxy to existing Reactor.
    * **Not Found:** Select Reactor (Placement). Send `MOUNT` command to Reactor.
3.  Reactor loads Snapshot (S3) -> Hydrates Memory -> Register in Redis.

### 3.2 Active State
* Reactor holds `GraphLock` in Redis (Heartbeat every 10s).
* If heartbeat fails, other Reactors can claim ownership (Crash Recovery).

### 3.3 Hibernation (The "Freeze")
1.  **Idle Detection:** No connected clients for 10 minutes.
2.  **Snapshot:** Reactor serializes state to S3.
3.  **Unregister:** Remove `glial:loc:{graph_id}` from Redis.
4.  **Free Memory:** Reactor drops `GripEngine` instance.

---

## 4. Rebalancing & Deployment
* **Rolling Updates:**
    1.  New Reactor spawns.
    2.  Old Reactor receives `DRAIN` signal.
    3.  Old Reactor stops accepting new `MOUNTs`.
    4.  As Graphs hibernate, they migrate naturally.
    5.  Force-migrate remaining Graphs (Snapshot -> Handoff) after timeout.

