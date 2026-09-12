# Injection relationships — Glade candidate 1, revision 2

2026-09-09. Owner requested the missing graph facts alongside the
[Shaku/Dill evaluation](DependencyInjectionEvaluation.md). These are proposed
architecture declarations, not a selected framework or implemented runtime.

The [current `.gyld.py`](/Users/owebeeone/limbo/gyld-wz/gyld/examples/glade-architecture.gyld.py)
and [v2 snapshot](/Users/owebeeone/limbo/gyld-wz/gyld/artifacts/glade-architecture-v2/snapshot.json)
now contain six explicit binding recipes. Each names its port, provider, consumers,
assembly owner and shared instance scope. Binding occurrences are **not services**.

| Binding recipe | Contract | Selected provider | Consumers |
|---|---|---|---|
| `clock_binding` | ClockPort | Runtime | Admission, Directory |
| `peer_carrier_binding` | CarrierPort | IrohAdapter | Sessions, peer role |
| `client_carrier_binding` | CarrierPort | WebSocketAdapter | Sessions, client role |
| `record_transport_binding` | TransportPort | Same IrohAdapter occurrence | Records |
| `directory_host_binding` | RecordHostPort | Records | Directory facade |
| `directory_profile_binding` | RecordProfilePort | DirectoryRules | Records |

All six inherit `AssembledBy[NodeAssembly]` and `SharedWithin[NodeScope]`.
The binding occurrence distinguishes a role; matching a port type alone does not.
The two Iroh-facing recipes MUST share their provider within the same node scope.
The clock binding MUST be substituted once for all named consumers in a test-node
composition; a sibling test-node gets a distinct scope. This does not define a
runtime config format or infer a global singleton from a library class.

Seven relationship kinds were added in the **example vocabulary**, not Gyld core:
`BindsPort`, `SelectsProvider`, `ForConsumers`, `SharedWithin`, `AssembledBy`,
`CleanupOwnedBy`, `DrainsBefore`. Existing `UsesScope` remains coarse context;
it is not silently reinterpreted as a resolved binding.

## Construction and cleanup are different graphs

Directory formerly supplied both the facade and the pure record-profile port.
That hid a possible constructor cycle: Directory needs Records while Records
needs Directory's profile. Revision 2 introduces **DirectoryRules**, independently
constructible pure rules. The intended sequence is rules → Records → live facade,
subject to other dependencies. This need not become another crate; the existing
discovery transaction/equivalence requirement remains unchanged.

Runtime is explicitly proposed to supervise cleanup of Records, Sessions, Iroh,
WebSocket and Storage. Records drains before storage/peer-carrier release; Sessions
drains before peer/client-carrier release. These are **partial ordering constraints**,
not a total serial shutdown algorithm. Independent cleanup may still run concurrently.
Cache scope does not discharge this async obligation.

## Evidence and limits

- V2 retains **24 responsibility allocations, 27 concerns and 107 obligations**.
  It adds one logical rules participant and six binding-recipe occurrences.
- Four new regression tests first failed against v1, then passed with the declared
  bindings/rules/cleanup and an unresolved-provider rejection witness. All eight
  Glade architecture tests and the 34-test selected architecture suite pass; the
  existing package architecture gate passes. Ruff and capture-host Mypy pass.
- Expanded Mypy over the whole architecture test module still reports ten diagnostics
  in previously existing helper/test code (`check_architecture.py`, `test_fast.py`,
  the old temporary-directory cleanup test). Those were not suppressed or repaired
  in this scope. No claim of a fully green repository type check is made.
- Capture verifies structural references, requirement preservation and the declared
  code-dependency DAG. It does **not** verify all constructor/provider bindings,
  cache identity, provider conformance, actual shutdown or absence of hidden I/O.
  These six recipes are a bounded slice, not a complete constructor graph.
- Broader session/account/operation scope rules, full shutdown phases, ownership
  of escaped handles, failure cleanup and framework selection remain open.
- No weights, optimization score, new graph engine or production Glade code changed.
  V1 exports and the previously shown v1 interactive views remain historical;
  the [new full SVG](/Users/owebeeone/limbo/gyld-wz/gyld/artifacts/glade-architecture-v2/full-graph.svg)
  is generated separately. No v1 result was overwritten.

V2 digest: `61c853bd17bcc4837f54f34b7e41fb8b09abaeed2116a31b7ebabe6215497a29`.
The snapshot and its `inputs.json` / `annotations.json` preserve the exact capture.
