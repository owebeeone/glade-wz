# Glial Authorization Layer
**Document ID:** GLIAL-DOC-003
**Version:** 1.0
**Status:** Contract Specification
**Dependency:** GLIAL-DOC-001, GLIAL-DOC-002

---

## 1. Introduction

Security in Glial is not an afterthought; it is enforced at the **Connection Plane**. The Authorization Layer defines how Clients prove their identity and what Capabilities they hold.

---

## 2. The Capability Model

Glial uses a **Capability-Based Security** model. A Client does not just have a "Role"; it holds a set of signed Capabilities.

### 2.1 Capability Token
The Auth Token (JWT) passed in the `HI` handshake contains a `caps` claim:

```json
{
  "sub": "user_123",
  "caps": [
    "scope:session:write",
    "scope:intent:write",
    "scope:view:read",
    "scope:data:read"
  ]
}
```

### 2.2 Scope Enforcement
The Connection Plane maintains a generic enforcement table. When an `OP` or `RPC_REQ` arrives:

1.  **Parse Op:** Extract Target Grip (e.g., `data.user.email`) and Verb (e.g., `WRITE`).
2.  **Match Scope:** Identify that `data.user.email` belongs to the `data` scope.
3.  **Check Cap:** Does the connection possess `scope:data:write`?
    * **Browser Client:** Typically has `scope:intent:write` but **NOT** `scope:data:write`.
    * **Backend Provider:** Validates with a Service Key granting `scope:data:write`.

For `RPC_REQ`, enforcement MUST include:
1.  **Target Resolution:** Resolve the target tap and implied scope/resource.
2.  **Capability Check:** Validate invoke privilege for that tap/scope (for example `scope:data:rpc` or equivalent policy mapping).
3.  **Context Binding:** Bind caller identity/capabilities to the forwarded request so the Provider can enforce business-level authorization.

---

## 3. Enforcement Points

### 3.1 Ingress (Write Protection)
* **Location:** Connection Plane (Glial Server).
* **Action:** If a Client tries to write/invoke on a Scope it lacks capabilities for, the Server:
    1.  Drops the message.
    2.  Sends an `["ERR", { "code": "E_ACCESS_DENIED" }]` frame.
    3.  Logs the violation to the Audit Stream.

### 3.2 Egress (Read Protection)
* **Location:** State Plane (Reactor) -> Connection Plane.
* **Action:** When a subscription matches a Grip, the Engine checks if the Client has `READ` permission for that Scope. If not, the Grip is implicitly "masked" (returns `null` or `absent`) or the subscription is rejected.

---

## 4. Audit Logging
Every `ERR` of type `E_ACCESS_DENIED` must be logged with:
* `connection_id`
* `user_id`
* `target_grip`
* `op_type`
* `timestamp`

This allows for intrusion detection (e.g., a hacked client trying to overwrite system flags).
