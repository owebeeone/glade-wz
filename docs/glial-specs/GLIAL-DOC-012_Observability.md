# Observability & Debugging
**Document ID:** GLIAL-DOC-012
**Version:** 1.0
**Status:** Subsystem Design
**Dependency:** GLIAL-DOC-002, GLIAL-DOC-003, GLIAL-DOC-006

---

## 1. Introduction
Distributed graphs are notoriously hard to debug. Glial treats observability as a first-class citizen using **OpenTelemetry (OTEL)** and a custom **Time-Travel Audit Log**.

---

## 2. Tracing

### 2.1 Span Propagation
* **Trace Context:** Passed in GSP `META` envelope.
* **Key Spans:**
    * `ws_ingress` (Relay)
    * `engine_tick` (Reactor)
    * `effect_req` (Reactor -> Provider)
    * `provider_exec` (Provider)

### 2.2 Resolution Traces
When a Grip resolves, the Engine emits a `ResolutionTrace` event:
```json
{
  "event": "resolution",
  "grip": "data.user.profile",
  "trigger": "op_105 (session.user_id)",
  "provider": "user-service-01",
  "latency": 45
}
```

---

## 3. The Audit Log (Time Travel)
* **Concept:** A linear history of all Ops applied to a Graph.
* **Storage:** Append-only log (e.g., Kafka / Kinesis -> S3 Parquet).
* **Replay:** Debugging tools can load the log into a local `GripEngine` and replay tick-by-tick to reproduce race conditions.

---

## 4. Metrics (Prometheus)
* `glial_active_graphs`: Gauge.
* `glial_ops_per_sec`: Counter.
* `glial_effect_latency`: Histogram (labels: `scope`, `provider_group`).
* `glial_quota_drops`: Counter (Backpressure activity).

