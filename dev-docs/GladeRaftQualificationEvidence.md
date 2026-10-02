# Glade Raft qualification evidence

Date: 2026-10-03. Status: **Q0 draft checkpoint; implementation not yet reviewed**.
The [plan](GladeRaftQualificationPlan.md) and
[contract](GladeRaftAdoptionContract.md) control claims. Exact reviewed source
revisions and reports will be recorded in the review-cycle ledger.

## TDD and build evidence

1. The API consumer test was written against absent declarations: expected
   E0432 missing imports. This was declaration RED only.
2. Minimal std-only API declarations and an intentionally refusing harness were
   added. API `public_contract` compiles and passes **2 tests**; its recorder is
   a substitution/type witness, not a conforming application implementation.
3. `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml
   -p glade-raft-adoption-proof --test qualification` with the compatible PROTOC
   compiles and produces **0 passed, 11 failed**. Assertions fail because no
   retained applied response exists, create never accepts and the direct noop
   returns `Err(NotQualified)` instead of `Ok(None)`. This is behavioral RED.
   No application or RawNode driver is implemented in the Q0 checkpoint.
4. `proofs/raft-adoption/check.sh` passes its syntax/metadata architecture gate,
   two-source empty-allowlist process-global scan, token-aware conditional scope
   check and Rust formatting check. These structural passes do not mean the
   behavioral contract passes.

Build setup failures are separate evidence: offline dependency cache initially
lacked protobuf; missing protoc and the newer major-32 compiler also failed
before running tests. The bundled protobuf-build 0.14.1 macOS x86-64 compiler
prints `libprotoc 3.9.0` and runs on this arm64 machine. The exact supplied path
and repeatable commands are in [the fixture README](../proofs/raft-adoption/README.md).
Rust toolchain is 1.96.0. No build failure is counted as behavioral RED.

## Initial executable specifications

`proof/tests/qualification.rs` names RA-linked schedules for: ordered competing
creates; leadership versus home; dropped reply/retry after failover; changed
retry and principal namespaces; old-leader minority and post-move suffix fencing;
isolated successor lag and catch-up; ordered revocation/current disclosure;
retirement/exact historical retry; wrong scope/unsupported effect; two-voter
loss; contiguous application/noop/exact replay/gap/conflicting replay.

These are intended assertions pending implementation, not passing results. The
full plan retains unqualified bootstrap, crypto, metadata witnesses, separate
BeginMove/Activate, capacity, disk and production integration obligations.
