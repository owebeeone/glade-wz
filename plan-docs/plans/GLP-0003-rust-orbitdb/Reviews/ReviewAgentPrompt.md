# Review Agent Prompt

Use this prompt for independent review agents critiquing `GLP-0003`.

```text
You are reviewing the rust-orbitdb plan in:

  plan-docs/plans/GLP-0003-rust-orbitdb/

Read these files first:

  Plan.md
  Checkpoints.md
  Decisions.md
  Risks.md
  Workstreams.md
  Support/SimulatorArchitecture.md
  Support/NetworkingRuntimeStudy.md
  Support/SecurityAuditPlan.md
  Support/RequirementTrace.md

Do not implement. Provide a critique.

Review stance:

  - Prioritize correctness, testability, security, and architecture risks.
  - Treat D0003 and D0004 as high-priority requirements:
      D0003: simulator is a product module.
      D0004: JS OrbitDB must test the simulator.
  - Challenge whether the roll-build checkpoints are small enough for agents
    and humans to reason about without context overload.
  - Look for hidden coupling to libp2p in core/sync/document APIs.
  - Look for libp2p-shaped substrate vocabulary, not only libp2p Rust types.
  - Look for accidental downstream product naming in rust-orbitdb scope.
  - Look for missing multi-writer document CRDT acceptance gates.
  - Look for places where "million scale" is disconnected from small-N tests.
  - Look for missing security/audit/proof artifacts.
  - Look for missing scenario DSL, trace comparison, and exact-to-compressed
    invariant bridge requirements.
  - Look for weaknesses in the Tokio/mio C10k analysis and runtime trade-offs.

Return:

  1. Findings ordered by severity, with file and line references where possible.
  2. Open questions that block implementation.
  3. Checkpoints that should be split further.
  4. Requirements that lack clear tests.
  5. Decisions that need stronger wording or reversal.
  6. Any suggested edits, written as concise patch guidance rather than full
     replacement prose unless a replacement is necessary.

Do not praise the plan generally. If no issue exists in a category, say so
briefly and move on.
```
