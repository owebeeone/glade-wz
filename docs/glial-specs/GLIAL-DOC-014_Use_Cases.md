# Use Cases & Reference Flows
**Document ID:** GLIAL-DOC-014
**Version:** 1.0
**Status:** Reference
**Dependency:** GLIAL-DOC-001

---

## 1. Scenario: The Intelligent Form Filler

**Context:** User drops a PDF invoice. AI fills the "New Expense" form.

### 1.1 Sequence
1.  **User (Browser):** Drag & Drop PDF.
    * Write: `intent.upload.blob = [Bytes]`
2.  **Glial (Reactor):**
    * Update `session.upload.blob`.
    * Detect Dependency: `view.form.analysis` depends on `blob`.
    * Emit: `EFFECT_REQ { grip: "view.form.analysis", params: { blob: ... } }`
3.  **AI Agent (Provider):**
    * Receive Request.
    * Call LLM: "Extract date, vendor, total from this PDF."
    * Return: `OP { verb: "SET", grip: "view.form.analysis", val: { date: "2023-01-01", total: 100 } }`
4.  **Glial (Reactor):**
    * Update Graph.
    * Broadcast Delta.
5.  **User (Browser):**
    * Form fields auto-populate.
    * Write: `intent.form.submit = true`.

---

## 2. Scenario: The Session Twin (Support)

**Context:** User sees an error. Support Agent "jacks in".

### 2.1 Sequence
1.  **Support Agent:**
    * Connect with `admin` token.
    * Send `MOUNT { graph_id: "user_session_123" }`.
2.  **Glial (Reactor):**
    * Verify `scope:session:read` capability.
    * Send `SNAPSHOT` of User's current graph to Support Agent.
    * Subscribe Support Agent to real-time updates.
3.  **Synchronization:**
    * User scrolls down -> Support Agent sees scroll position update.
    * User types "Helpo" -> Support Agent sees "Helpo".
4.  **Intervention:**
    * Support Agent writes: `intent.form.field_x = "Correct Value"`.
    * Glial propagates to Backend Provider (Validator).
    * Backend Provider accepts and writes Authoritative Value.
    * User sees the correction.
