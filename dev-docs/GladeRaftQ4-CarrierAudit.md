# Q4 pinned carrier fact audit

**Neither unmodified pinned carrier currently establishes instance-owned injected election entropy and clocks. Both can carry the protected application history, but both need further executable qualification before production selection.** The accepted raft-rs Q3 proof remains evidence for its private profile; it does not select the production implementation.

This was read-only inspection. No repository files, dependencies, Git state or production sources were changed. OpenRaft’s crate was downloaded and extracted only under `/tmp/glade-q4-carrier-audit.jaiDwP`.

## Inspected objects

Controlling documents read:

- `AGENTS_GWZ.md`, `AGENTS.md`
- `GladeRaftQualificationPlan.md`
- `GladeRaftAdoptionContract.md`
- `GladeRaftImplementationEvaluation.md`
- The review-loop skill’s helper-role instructions

Pinned carrier sources:

| Carrier | Inspected source | Source identity |
|---|---|---|
| raft-rs | Cached `raft-0.7.0`, plus `raft-proto-0.7.0` | Crate VCS metadata: `10c6e9db6792b85c81784e44fc278f895d5f0ab0` |
| OpenRaft | Downloaded `openraft-0.9.25.crate` | Crate VCS metadata: `8815cdba2826f74e848acef361ad03f93bb1c3f8`, `path_in_vcs = openraft` |

OpenRaft archive SHA-256:

```text
a97014fb78acb77be3a40ac2da305f6dd3a6b243f3a908ace87d29b3972eaafd
```

The versioned official documentation warns that OpenRaft’s pre-1.0 upgrades can change both APIs and on-disk types. That is an explicit compatibility cost, not evidence against its correctness. [OpenRaft 0.9.25 API status](https://docs.rs/openraft/0.9.25/openraft/#api-status)

## Election entropy and clocks

### raft-rs 0.7.0

The protocol clock already has a usable explicit boundary: `RawNode::tick()` advances one logical tick. The host can own the monotonic clock, pause behavior and scheduling without placing wall-clock access inside Raft. Automatic elections run through `Raft::tick_election()`, which increments elapsed ticks and steps `MsgHup` when the election deadline passes. [Pinned `raw_node.rs`, lines 338–344](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raw_node.rs#L338), [pinned `raft.rs`, lines 1068–1090](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L1068)

The entropy boundary is defective for this workspace:

- `reset_randomized_election_timeout()` directly invokes `rand::thread_rng().gen_range(...)` at `raft.rs:2810`.
- `reset()` invokes it at `raft.rs:992`, so avoiding election ticks does not remove this dependency access.
- The hidden public setter at `raft.rs:474` overwrites the timeout **after** such accesses; it does not inject the source used by resets. [Pinned reset and entropy implementation](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L985), [timeout setter](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L470)

**Concrete faithful route:** a reviewed carrier-source adaptation introduces an instance-owned entropy provider supplied before construction and used by every timeout reset. The host continues supplying ticks. The injected provider needs the same bounded-range semantics; callers must not merely overwrite the chosen timeout afterward. This is a proposed adaptation, not an existing upstream feature or permission to modify dependencies.

The default logger also contains `Once` and `static mut LOGGER`, but the accepted feature selection disables `default-logger` and supplies an instance logger. That avoids this production feature path; it does not erase conditional source debt. [Pinned default logger](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/lib.rs#L572)

### OpenRaft 0.9.25

OpenRaft exposes `AsyncRuntime`, but its operations are **static**:

```rust
fn spawn(...);
fn sleep(duration: Duration);
fn sleep_until(deadline: Self::Instant);
fn thread_rng() -> Self::ThreadLocalRng;
```

`Instant::now()` is also static. `Raft::new(id, config, network, log_store, state_machine)` accepts no runtime, clock or entropy instance. `RaftTypeConfig` selects a runtime **type**. These are useful replacement seams, but they do not pass an injected node-owned handle through the running system. [Pinned runtime trait](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/async_runtime.rs#L25), [instant trait](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/instant.rs#L36), [constructor](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/raft/mod.rs#L230)

The default implementation uses `rand::thread_rng()` at `async_runtime.rs:164`. Configuration samples an election timeout through `RT::thread_rng()` at `config/config.rs:257`; `EngineConfig::new` stores it in timer configuration at `engine/engine_config.rs:46–56`. In the inspected source, the random-timeout generator is called during engine configuration construction; I did not find per-election calls to this generator. [Pinned RNG implementation](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/async_runtime.rs#L163), [timeout sampling](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/engine/engine_config.rs#L44)

Automatic ticking uses `C::now()` and `C::sleep_until()`. Election handling examines vote modification time, leader lease, randomized timeout and the additional timeout for a greater known log. A comparison must exercise these actual rules, including pause/resume, rather than replacing them with manual leader selection. [Pinned ticker](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/core/tick.rs#L81), [election handler](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/core/raft_core.rs#L1440)

There is another ambient-clock seam: `DisplayInstant::fmt()` reads `SystemTime::now()` and `T::now()`, then defaults to conversion through `chrono::Local`. Merely replacing election RNG does not close clock access throughout this carrier. [Pinned instant formatting](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/display_ext/display_instant.rs#L39)

**Concrete faithful route:** a reviewed adaptation passes an owned runtime/context handle into construction and propagates it through storage initialization, engine/vote handlers, ticker, core and replication tasks. Time/sleep/timeout/spawn calls must use that handle, and initial election entropy must receive it. Formatting must use logical instants or an explicitly injected conversion. A custom runtime implemented through globals or thread-locals does not satisfy the requirement. A static factory returning a fixed seeded RNG also does not demonstrate separately injected instance state.

This route has a broader source footprint than raft-rs’s entropy seam. That is a source-based observation, not a measured total integration-cost ranking.

## Persistence, application and retained outcomes

| Boundary | raft-rs 0.7.0 | OpenRaft 0.9.25 |
|---|---|---|
| Durable protocol state | Host processes `Ready`: snapshot, entries, HardState, dependent messages | `save_vote` must persist before returning; `append` reports durable completion through `LogFlushed` |
| Applied progress | Host applies committed entries and correctly processes `LightReady` before advancing application progress | State-machine worker applies committed entries; returned results are sent to awaiting clients |
| Outcome retention | Application-owned | Application-owned |
| Physical durability | Adapter/profile obligation | Adapter/profile obligation |

raft-rs explicitly separates messages that require persisted state, exposes `must_sync`, and states that persistence usually requires operations such as `fsync`. `advance_append_async` means cache/readability plus a later persistence notification; it is not a durability acknowledgment. [Pinned Ready interfaces](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raw_node.rs#L197), [advance and async persistence](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raw_node.rs#L645)

OpenRaft’s v2 storage contract requires serialized vote/log writes, consecutive logs, durable vote completion and durable append callbacks. Core `append_to_log` awaits that callback; local replication progress advances afterward. [Pinned v2 storage contract](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/storage/v2.rs#L43), [core append completion](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/core/raft_core.rs#L709)

OpenRaft permits either persistent state-machine application or persistent snapshots plus reconstruction. `save_committed` is optional. Glade must select and test the stronger concrete profile it needs rather than treating these optional defaults as its receipt guarantee. [Pinned application/persistence choices](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/storage/v2.rs#L84), [apply contract](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/storage/v2.rs#L175)

Neither carrier automatically retains Glade’s canonical request identity, exact command bytes, accepted/refused terminal outcomes, disclosure evidence or retirement fences. OpenRaft returning `Vec<C::R>` and sending those values after application does not establish retained retry history. Its core removes response channels when sending results. Those obligations belong in the durable application machine and snapshots/replay. [Pinned result dispatch](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/core/raft_core.rs#L769)

## Membership and snapshots

raft-rs supplies `ConfChangeV2` proposal/application and explicit joint enter/leave handling. The host applies an accepted configuration change when applying its committed entry and retains the resulting configuration coherently with recovery. It already supports the accepted Q3 witness; real authenticated admission and production configuration-intent recovery remain additional obligations. [Pinned configuration application](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raw_node.rs#L394), [joint handling](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/raft.rs#L2757)

OpenRaft’s `change_membership` performs joint then uniform changes. Its documentation/source explicitly permits remaining joint if leadership is lost or the caller crashes before the second change. `add_learner(blocking=true)` waits for leader-observed replication catch-up. This does **not** establish successor application of the complete policy/retry cut required by RA-005. [Pinned membership API and implementation](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/raft/impl_raft_blocking_write.rs#L25)

OpenRaft membership APIs accept membership changes and return responses, but do not accept Glade’s `ConfigKey`/canonical intent identity. A wrapper must demonstrate durable intent binding, accepted and refused outcomes, and exact recovery across the two stages; matching the current final membership alone cannot recover the original outcome.

Snapshot differences:

- raft-rs exchanges a protocol `Snapshot` with application-owned data and host-owned persistence/install sequencing. `Storage::snapshot` must satisfy the requested frontier. [Pinned storage snapshot interface](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/src/storage.rs#L152)
- OpenRaft provides builder, receive, install and current-snapshot interfaces. Metadata includes last log ID, last membership and snapshot ID. Installation must replace state and expose the installed snapshot before returning. [Pinned snapshot metadata](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/storage/mod.rs#L40), [installation contract](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/storage/v2.rs#L217)
- OpenRaft’s ordinary snapshot data implements Tokio async read/write/seek; `generic-snapshot-data` removes that shape and requires application transport handling. Neither shape supplies a complete Glade snapshot codec or validates its identity/policy/outcome coherence. [Pinned snapshot data types](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/type_config.rs#L67)

## Codec, runtime and inventory costs

**raft-rs:** inspected manifest uses Apache-2.0 and supports protobuf/prost codec feature paths. The accepted proof uses protobuf; `raft-proto` adds `protobuf-build` as a build dependency. Switching codecs is a separately qualified dependency change, not an automatic escape from the current build-tool obligations. [Pinned manifests](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/Cargo.toml), [raft-proto manifest](https://github.com/tikv/raft-rs/blob/10c6e9db6792b85c81784e44fc278f895d5f0ab0/proto/Cargo.toml)

**OpenRaft:** crate license is `MIT OR Apache-2.0`. Mandatory manifest dependencies include `anyerror`, `byte-unit`, `chrono`, `clap`, `derive_more`, `futures`, `maplit`, `openraft-macros`, `rand`, `thiserror`, `tokio`, `tracing`, `tracing-futures`, and `validit`. Tokio remains mandatory for a custom `AsyncRuntime`; core also directly uses Tokio synchronization and selection. `serde` is optional and adds serialization bounds; it does not select a canonical wire codec. Direct v2 custom storage requires `storage-v2`, because the traits otherwise remain sealed. [Pinned OpenRaft manifest](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/Cargo.toml), [v2 sealing](https://github.com/databendlabs/openraft/blob/8815cdba2826f74e848acef361ad03f93bb1c3f8/openraft/src/storage/v2.rs#L25)

The existing proof lockfile contains **39 registry packages**. All had cached manifest license fields. Notable expressions include:

- `raft`, `raft-proto`, `protobuf-build`: Apache-2.0
- `protobuf`, `protobuf-codegen`, `bytes`, `getset`: MIT
- `slog`: MPL-2.0 OR MIT OR Apache-2.0
- `unicode-ident`: `(MIT OR Apache-2.0) AND Unicode-3.0`
- `wasi`: Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT
- `zerocopy` and derive: BSD-2-Clause OR Apache-2.0 OR MIT

**Inventory limits:** manifest license fields are declarations, not a completed notice audit. The existing lockfile includes build/target/proof edges, not a reviewed production graph. OpenRaft has no resolved comparison lockfile here; its version ranges are not exact transitive pins. I did not run vulnerability scanning, inspect every dependency’s disabled source branches, or qualify bundled protoc/security/platform support. Production license/security compliance remains open.

## Cheapest faithful comparison witness before selection

The smallest useful next object is a **private carrier comparison**, preserving the accepted application semantics and leaving production dependencies unchanged:

1. Define the common application/outcome/fault observations first, with compiling RED consumers. Existing `CommittedMachine` and Q3 conformance functions provide reusable starting points, but Q3’s stored-entry format includes original carrier bytes; it cannot simply be relabeled as a carrier-neutral wire format.
2. Specify and independently review the instance-owned entropy/time adaptations for both pinned carriers. Preserve upstream algorithm behavior. Check dependency source branches as well as wrapper code; an empty repository allowlist does not audit dependencies.
3. Run both through the same complete application commands, identities, policy inputs and expected terminal outcomes. A common scheduled transport owns delivery/drop/duplicate/reorder decisions; OpenRaft’s async RPC futures must be driven by controlled runtime events.
4. Start with fixed three data-bearing voters and **automatic** election/failover. Exercise split votes, minority isolation, delayed votes/append replies, paused leader recovery and independent node schedules.
5. Add the same real-I/O cuts: vote/log durability, commit publication, application/outcome publication, reply loss, restart and storage error. Require exact outcomes after leader change and reconstruction; add mutants for early durable callbacks, missing outcomes and acceptance before durable application evidence.
6. Add learner/joint transitions, lost replies at both joint stages, application-cut readiness, snapshot install/publication/compaction/restart, retirement and disclosure-sensitive exact retry.
7. Record engineering changes, runtime/dependency inventory, test duration, warm build-plus-test and cold build independently. Do not substitute “fewer adapters” or README claims for these measurements.

This comparison remains a bounded private witness. Real cryptographic evidence, independent machines/storage domains, Glade receipt compatibility, baseline cut, legacy-writer exclusion, rollback fencing and external sinks still require their own Q4 evidence.

## Commands and limits of this inspection

Representative executed commands:

```sh
rg --files -g 'AGENTS*' -g '*QualificationPlan*' \
  -g '*AdoptionContract*' -g '*ImplementationEvaluation*'
cat AGENTS_GWZ.md AGENTS.md \
  dev-docs/GladeRaftQualificationPlan.md \
  dev-docs/GladeRaftAdoptionContract.md \
  dev-docs/GladeRaftImplementationEvaluation.md
rg --files /Users/owebeeone/.cargo/registry/src
mktemp -d /tmp/glade-q4-carrier-audit.XXXXXX
curl --proto '=https' -LsSf \
  https://static.crates.io/crates/openraft/openraft-0.9.25.crate \
  -o "$task_tmp/openraft-0.9.25.crate"
tar -xzf "$task_tmp/openraft-0.9.25.crate" -C "$task_tmp"
sha256sum /tmp/glade-q4-carrier-audit.jaiDwP/openraft-0.9.25.crate
```

Additional `rg`/`nl` inspections located the cited seams. An inline Python `tomllib` read joined the existing proof lockfile with cached manifest license fields. Official versioned documentation was browsed; some individual docs.rs pages were unavailable, so the downloaded pinned source supplied the detailed evidence.

No carrier implementation, comparison build, test run, fault run or production-selection recommendation was produced by this fact audit.