# Schema & Compatibility Contract
**Document ID:** GLIAL-DOC-007
**Version:** 1.0
**Status:** Contract Specification
**Dependency:** GLIAL-DOC-001, GLIAL-DOC-002

---

## 1. Introduction

Glial enforces a "Code-as-Contract" approach. The schema is defined in Python and projected to other languages (TypeScript). This document defines the rules for this schema to ensure compatibility.

---

## 2. The Type System

Glial types are a subset of Python types that map cleanly to JSON.

### 2.1 Primitives
* `str`, `int`, `float`, `bool`, `None`.
* `Blob` (Base64 encoded string with mime-type metadata).

### 2.2 Composites
* `List[T]`
* `Dict[str, T]`
* `Dataclass` (Maps to JSON Object).

---

## 3. Validation

### 3.1 Who Validates?
* **Connection Plane:** Basic JSON syntax check only.
* **State Plane:** Checks basic type conformity (e.g., "Expected int, got string") if schema metadata is loaded.
* **Provider (Effect Plane):** **MUST** perform deep validation (business rules).
    * If a Provider receives valid JSON that violates logic (e.g., `age: -5`), it must return a Terminal Error.

---

## 4. Versioning & Evolution

### 4.1 Forward Compatibility
* Clients MUST ignore unknown fields in a Dataclass payload.
* This allows the Backend to add new fields (e.g., `user.avatar_url`) without breaking older Frontend clients.

### 4.2 Grip ID Stability
* Grip IDs (e.g., `data.user.profile`) are immutable constants.
* Renaming a Grip is a breaking change requiring a migration (or a new Grip ID).

---

## 5. Codegen Artifacts

The `glial-gen` tool ensures the contract is respected.

### 5.1 Python (Source)
```python
@dataclass(frozen=True)
class UserProfile:
    """The public profile of a user."""
    email: str
    age: int
```

### 5.2 TypeScript (Target)
```typescript
/** The public profile of a user. */
export interface UserProfile {
    email: string;
    age: number;
}
```

### 5.3 AI Tools (Target)
```json
{
  "name": "UserProfile",
  "description": "The public profile of a user.",
  "schema": { ... }
}
```
