# Provider Registration & Selection
**Document ID:** GLIAL-DOC-005
**Version:** 1.0
**Status:** Contract Specification
**Dependency:** GLIAL-DOC-001, GLIAL-DOC-002

---

## 1. Introduction

Providers are the "Hands" of Glial. This document defines how a Provider (Backend Client) announces its presence and how the System decides which Provider handles a request.

---

## 2. Registration

### 2.1 The "PROVIDE" Message
Upon connection, a Backend Client sends a registration frame:

```json
["PROVIDE", {
  "group": "user-service",
  "grips": ["data.user.*", "data.auth.*"],
  "priority": 10
}]
```

### 2.2 Provider Groups
Providers are organized into **Groups**.
* **Load Balancing:** If 5 providers register for `user-service`, the Glial Server round-robins Effect Requests among them.
* **Failover:** If a Provider disconnects, the Server immediately re-routes pending requests to another member of the group.

---

## 3. Leasing & Exclusive Locks

For some Grips (e.g., a specific User's real-time trading session), we need **Stickiness**.

### 3.1 The Lease
A Provider can request an **Exclusive Lease** on a Scope.
* `["LEASE", { "scope": "data.trading.session.123" }]`
* If granted, *only* this Provider receives requests for this scope.
* Used to ensure linearizability of complex backend logic.

---

## 4. Lifecycle

### 4.1 Heartbeats
Providers must send `PING` every 5 seconds.
If 3 heartbeats are missed, the Server:
1.  Mark Provider as `DEAD`.
2.  Triggers **Re-Election** for any Leases held.
3.  Updates Grip Status to `stale` or `connecting` for affected clients.
