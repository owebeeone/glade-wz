# Effect Plane Implementation
**Document ID:** GLIAL-DOC-010
**Version:** 1.0
**Status:** Subsystem Design
**Dependency:** GLIAL-DOC-006, GLIAL-DOC-005, GLIAL-DOC-007

---

## 1. Introduction
The **Effect Plane** consists of the SDKs and Runtimes that allow developers to build Providers and Clients. It abstracts the GSP protocol into ergonomic APIs.

---

## 2. Provider SDK (`glial-py`)

### 2.1 API Surface
The SDK encourages a declarative style similar to FastAPI.

```python
app = GlialApp(token="SERVICE_KEY")

@app.provider("data.user.profile")
async def get_user_profile(intent: UserProfileIntent, ctx: Context):
    """
    Fetches user profile from Postgres.
    """
    user_id = intent.user_id
    # DB Logic
    profile = await db.fetch_profile(user_id)
    return profile # SDK automatically sends OP:SET
```

### 2.2 Execution Model
* **Concurrency:** Asyncio event loop.
* **Effect Loop:**
    1.  Connect WebSocket to Glial Relay.
    2.  Send `PROVIDE` registration.
    3.  Listen for `EFFECT_REQ`.
    4.  Spawn `Task` for handler.
    5.  Send `OP` or `ERR` response.

### 2.3 Reliability Features
* **Auto-Reconnect:** Exponential backoff (1s, 2s, 4s...) on connection loss.
* **Lease Renewal:** Background task to renew `LEASE` every 5s.
* **Validation:** Pydantic validation of incoming `intent` payloads against the Python Schema.

---

## 3. Browser SDK (`glial-ts`)

### 3.1 React Integration
```typescript
const [profile, status] = useGrip(UserProfile);

// Writing Intent
const updateProfile = () => {
    profile.writeIntent({ name: "Alice" }); // Optimistic Update
}
```

### 3.2 Internals
* **SocketWorker:** Runs GSP in a WebWorker to prevent UI blocking.
* **Cache:** `Map<GripID, Value>`.
* **Optimistic UI:**
    * Store `OptimisticValue` overlay on top of `AuthoritativeValue`.
    * Clear overlay when `ACK` or new `OP` arrives from Server.

