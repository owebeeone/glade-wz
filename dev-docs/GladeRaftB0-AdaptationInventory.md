# Q4-B0 adaptation call-graph inventory

**The draft’s explicit failure contract requires API and error-path adaptations in both pins. Replacing their RNG implementations alone is insufficient.** This report inventories source paths; it does not establish runtime behavior, approve the untracked scaffold, or select a production carrier.

Inspected: the new `GladeRaftCarrierComparisonContract.md` draft and the previously audited pinned sources:

- raft-rs: `10c6e9db6792b85c81784e44fc278f895d5f0ab0`, crate `0.7.0`.
- OpenRaft: `8815cdba2826f74e848acef361ad03f93bb1c3f8`, crate `0.9.25`.

No writes, Git operations, builds or tests were performed.

## raft-rs: entropy ownership and error propagation

**Source facts.** Construction follows:

```text
RawNode::new [raw_node.rs:302–304]
  → Raft::new [raft.rs:327]
  → become_follower [391]
  → reset [1128]
  → reset_randomized_election_timeout [992]
  → thread_rng().gen_range(min..max) [2810]
```

`RawNode::new` and `Raft::new` already return `Result`; `become_follower`, `reset` and timeout reset return `()`. The owned entropy capability must therefore reach `Raft::new` before its initial transition, and construction failure must propagate without returning the partially assembled node. [Constructor source](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L327)

The complete transition fan-out is:

| Path | Exact locations in `raft.rs` |
|---|---|
| Follower transition → reset | `become_follower:1126–1131` |
| Candidate transition → reset | `become_candidate:1145–1155` |
| Leader transition → reset | `become_leader:1195–1205` |
| Campaign → candidate; single-node poll | `campaign:1261–1274` |
| Pre-vote win → campaign; vote win → leader; loss → follower | `poll:2219–2253` |
| Higher-term message → follower | `step:1388,1390` |
| Leader quorum loss → follower | `step_leader:2018` |
| Candidate append/heartbeat/snapshot → follower | `step_candidate:2270,2275,2280` |
| Pending committed configuration → follower | `maybe_commit_by_vote:2174–2216` |
| Defensive snapshot restoration → follower | `restore:2564–2578` |

`become_pre_candidate` does **not** call reset (`1168–1187`); preserving upstream draw frequency means it must not acquire a new entropy draw merely because its signature changes. [Transition/poll source](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L1126)

Automatic election errors currently have an additional obstacle:

```text
RawNode::tick → Raft::tick → tick_election → step(MsgHup)
                                      → hup → campaign → poll/transitions
```

`RawNode::tick`, `Raft::tick` and both tick handlers return `bool`. The handlers discard `step` errors at `1089`, `1105` and `1120`. `hup:1516`, `campaign:1261`, `poll:2219`, `maybe_commit_by_vote:2174`, `restore:2564`, and `handle_snapshot:2529` also lack a source-failure result channel. Public `step` already returns `Result`, but these nested paths must propagate the new error into it. [Tick source](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L1068)

**Proposed engineering choice, untested:** make the owned bounded sampler fallible; propagate a distinct source error through reset/transitions and the listed callers, including fallible ticks. Acquire and validate the timeout before installing it or changing reset-owned term/vote/leader fields: upstream `reset` currently changes those fields at `987–991` before drawing at `992`. A later failure may follow other legitimate step mutations; the host must poison participation and suppress further outputs rather than assume complete step rollback.

The current `Error` enum has no source variant (`errors.rs:6–50`). A reviewed distinct variant avoids presenting entropy exhaustion as storage failure or proposal rejection. A standard infallible `rand::Rng::gen_range` facade cannot preserve the required `Result` semantics. [Reset/entropy source](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L985)

## OpenRaft: context propagation inventory

**Source facts.** `Raft::new:230–324` currently assembles:

```text
static oneshot [247]
→ Tick::spawn [249]
→ EngineConfig::new [265]
→ StorageHelper::new/get_initial_state [268–269]
→ Engine::new [272]
→ Worker::spawn [274]
→ static spawn(core.main) [307]
→ RaftInner [309–324]
```

The ticker is spawned before initial timeout sampling and storage recovery complete. Explicit construction failure therefore requires cancellation/join ownership for already registered work, or reviewed deferred registration that preserves successful execution ordering. [Constructor](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/raft/mod.rs#L230)

Required context recipients and paths:

| Recipient/path | Source locations |
|---|---|
| Initial entropy | `config/config.rs:256–257` → `engine/engine_config.rs:45–56` |
| Recovery timestamp | `storage/helper.rs:49`, `65–67`, `155–163` |
| Engine candidate timestamp | `engine/engine_impl.rs:118–132` |
| Vote-request lease checks | `engine/engine_impl.rs:282–307` |
| Vote update/touch | `engine/handler/vote_handler/mod.rs:94–123` |
| Ticker spawn, clock and deadline registration | `core/tick.rs:56–88` |
| Leader heartbeat initialization | `core/raft_core.rs:149–151`, caller `1631` |
| API/notify/election/heartbeat time | `core/raft_core.rs:1157,1248,1284,1302,1441` |
| Replication spawn and backoff | `replication/mod.rs:160–203,329,413,595–604` |
| Append RPC deadline | `replication/mod.rs:437–457` |
| Vote RPC spawn/deadline | `core/raft_core.rs:1082–1088` |
| Additional child work | `core/raft_core.rs:372,440,868,1674` |
| State-machine/snapshot tasks | `core/sm/worker.rs:43–58,177–191` |
| Snapshot sender/deadlines | `replication/mod.rs:758,789`; `network/snapshot_transport.rs:163,199,278` |
| Wait timeout | `raft/raft_inner.rs:192` |
| Metrics elapsed time | `core/raft_core.rs:558` → `instant.rs:42–47` |
| Diagnostic wall/local time | `display_ext/display_instant.rs:45–58` |

The static conveniences in `type_config/util.rs:21–67` must stop serving as clock/work accessors. Descendant futures need cloned **instance** handles, including nested snapshot tasks. [Runtime conveniences](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/type_config/util.rs#L17)

## Concrete OpenRaft interface mismatches

These are source-derived incompatibilities with an unchanged API:

- `RaftTypeConfig` requires `Copy + Default + Ord` (`type_config.rs:49–50`). It is type configuration, not a suitable container for a caller-owned shared runtime handle. Keep types there; pass context separately.
- `AsyncRuntime` requires `Default`; its `spawn`, sleep, timeout and RNG methods are static. `spawn` returns a handle directly, `Sleep::Output = ()`, and RNG access is infallible (`async_runtime.rs:25–90`). Registration failure cannot be represented without adapting these signatures/result types.
- Join handles require `OptionalSync + Unpin`; boxed unit tasks therefore need reviewed typed result channels and compatible join wrappers, not just task erasure.
- `Instant` requires `Copy`, infallible arithmetic operators, static `now`, and infallible `elapsed` (`instant.rs:16–47`). Domain/overflow failures cannot travel through those operators.
- `handle_vote_req` explicitly computes `now - lease - 1ms` (`engine_impl.rs:288`). A zero-based unsigned logical instant cannot represent that early-time value without a checked internal representation/mapping.
- `update_vote` returns only protocol rejection errors (`vote_handler/mod.rs:94`). Source failure must not become `ByVote` or `ByLastLogId`.
- `Fatal` contains only `StorageError`, `Panicked`, `Stopped` (`error.rs:133–145`). `handle_api_msg` returns `()` (`raft_core.rs:1149`); callers at `955,1007` cannot propagate a new fatal directly. `handle_notify` already returns `Result<_, Fatal>` (`1239`).
- RPC timeout handling treats every timeout error as ordinary expiry (`raft_core.rs:1088–1100`); source/scheduler failure must remain distinguishable.
- Shutdown calls `report_metrics` (`raft_core.rs:230`), which currently reads time through `elapsed`; a failed clock must not prevent reporting the original failure.

[Runtime constraints](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/async_runtime.rs#L25), [instant constraints](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/instant.rs#L16)

**Proposed engineering choice, untested:** retain carrier type configuration, introduce a separately supplied context, make source/work operations fallible, and add a dedicated local fatal-source notification/result path across core and child tasks. Use checked time operations at the identified call sites; review an internal instant representation supporting upstream pre-epoch calculations. Format logical instants without clock reads. Track and cancel/join all descendant work by incarnation.

This requires broader plumbing than an RNG swap. Whether the proposed representation, result wrappers and cleanup ordering compile and preserve upstream behavior remains a B0 RED/GREEN and review obligation.