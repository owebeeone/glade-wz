# Glade Raft implementation qualification

Date: 2026-10-03. Status: **experiment carrier selected; production selection open**.
The [adoption contract](GladeRaftAdoptionContract.md) selects the obligations;
this document compares implementations of Raft, not Raft against Chubby/Multi-Paxos.
The [qualification plan](GladeRaftQualificationPlan.md) controls execution.

## Candidates and recommendation

| Candidate | Why evaluate it | Obligations and limits |
| --- | --- | --- |
| TiKV raft-rs, exact crate `raft 0.7.0` | Synchronous `RawNode` lets a small harness supply every message and persistence step. Suitable for explicit adversarial schedules without adding an async runtime. | Host must process Ready/LightReady correctly and provide storage, transport and deterministic application. Fixed-group success does not prove membership, automatic elections, snapshots or physical durability. |
| OpenRaft, inspected release `0.9.25` | A serious production candidate with application/network/storage and runtime interfaces, plus documented cluster lifecycle. | Larger runtime/adapter qualification scope for this first witness. Its documentation warns of pre-1.0 API and disk-format upgrade changes; qualify exact release, runtime, storage and migration behavior. No comparative executable results yet. |

Use raft-rs **only as the first proof carrier**. The reason is control of the
experiment boundary, not a claim that it is safer, better maintained or cheaper
to integrate. OpenRaft remains on the production shortlist. Neither candidate
has passed Glade's production qualification. The source/dependency and runtime
obligations below can change that choice.

`RawNode::ready` forbids state-changing calls while Ready is outstanding.
`advance` requires complete processing and returns LightReady, which must also
be processed before advancing application progress. The proof will persist to
its named memory store before sending dependent messages, apply only committed
entries, and preserve full command/outcome history. This establishes ordering
under memory retention, not a disk guarantee.
[raft-rs 0.7.0 RawNode documentation](https://docs.rs/raft/0.7.0/raft/raw_node/struct.RawNode.html).

OpenRaft documents runtime/network interfaces, cluster formation, dynamic
membership and upgrade guidance. Its API-status warning explicitly distinguishes
on-disk data changes from API changes. Those are relevant production obligations,
not proof of a particular adapter's fsync or power-loss safety.
[OpenRaft 0.9.25 documentation](https://docs.rs/openraft/0.9.25/openraft/),
[maintainer release history](https://github.com/databendlabs/openraft/releases).

## Pinned proof edges and source audit

The independent workspace pins `raft = 0.7.0` with defaults disabled and
`protobuf-codec`, `protobuf = 2.28.0`, `slog = 2.8.2` with defaults disabled, and
its Cargo lockfile. The API has no dependencies. Inject an instance-owned discard
logger; do not install a global logging hook. The fixture codec is private and
is not a Glade wire contract. Existing production manifests remain unchanged.

Source inspection of the downloaded `raft-0.7.0/src/raft.rs` found
`rand::thread_rng()` in `reset_randomized_election_timeout` (line 2810).
Construction/state transitions can call the reset even though the experiment
uses manual campaigns and never ticks. Thus repeatable explicit schedules do
**not** mean the dependency's randomness is injected. This is an open production
boundary defect; the proof's empty local process-global allowlist does not waive
it or cover dependency sources. An approved injectable randomness solution or
explicit dependency treatment, with automatic-election tests, is required.
[Versioned upstream source](https://docs.rs/raft/0.7.0/src/raft/raft.rs.html).

The build helper `protobuf-build 0.14.1` accepts only major version 3 protoc and
has no arm64-macOS bundled selection. The proof explicitly supplies its bundled
x86-64 libprotoc 3.9.0 on this translation-capable machine. Portability and this
aging build chain remain production qualification costs, not silently patched
or broadened dependency approvals. See [fixture commands](../proofs/raft-adoption/README.md).

## Production comparison gate

Both candidates MUST be tested against the same application contract and fault
journeys before final selection. Record actual engineering cost and measurements,
rather than scores inferred from their README. The gate MUST cover:

- persistence ordering and application/outcome replay across real I/O failures;
- full-data quorum and lagging successor readiness, beyond metadata agreement;
- automatic election scheduling, explicit clocks/randomness and pause recovery;
- authenticated genesis, learners/joint membership, snapshots and compaction;
- deterministic policy/retry/retirement state, permission-sensitive disclosure;
- Glade receipt/API compatibility, exclusion of old writers and upgrade/rollback;
- exact dependency license/security inventory and supported storage/runtime profile.

Neither algorithm precedent nor a storage test suite closes physical power-loss
qualification. The next production crate decision follows these witnesses; this
tranche starts the common application proof using raft-rs.
