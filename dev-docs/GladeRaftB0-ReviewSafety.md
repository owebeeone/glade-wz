# Q4-B0 contract/allocation/compiling RED — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeRaftCarrierComparisonContract.md`, `GladeRaftB0Allocation.md`, `GladeRaftB0-AdaptationInventory.md` and `proofs/raft-carrier-comparison/` at root `544c83d8cd07165cfeec2f0db64a78c8417849f3`; DRAFT private contract/allocation/compiling RED gate, dated 2026-10-03.

**Baseline:** Root `544c83d8cd07165cfeec2f0db64a78c8417849f3`; Glade `c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`; discovery `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Reviewed sources were read with `git show <exact SHA>:<path>` and compared with local bytes before execution. All four HEADs matched at start and end.

**Date:** 2026-10-03  
**Axis:** Safety: attack satisfiability, ownership, invalid-input refusal and terminal lifecycle. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — four P2 findings block. I pre-commit to GO on a revision that resolves P2-1–P2-4 as specified.

---

## 0. Evidence base

Read process authority `AGENTS_GWZ.md`, `AGENTS.md` and the full review-loop skill; QualificationPlan §6 supplies the local binding. Read the controlling ProductionIntegrationPlan, QualificationPlan, AdoptionContract RA-001–012, ImplementationEvaluation, Q4-CarrierAudit, LibraryBoundaryAndTestingPolicy, PackageArchitecture and BuildEntry.

Inspected the complete comparison contract, allocation and adaptation inventory; API `lib.rs:1–415`; refusing providers; constructor/compiler witnesses; all B0 consumer modules; fixture sources, scheduler and transport; oracle and fixture/RPC tests; manifests, lockfile, architecture policy and check scripts.

Verified all 50 inventory entries plus the inventory itself against committed/local bytes. Inventory SHA-256 is `d25e9045fa21f11cb513ca6e72712d57bdaeec49d2c0b5018312067f801b3a66`. Final verification also confirmed unchanged contract/adaptation bytes. Ignored pre-existing `__pycache__` files were excluded.

Executed the six README commands exactly:

- API witnesses: 2 passed.
- Provider compiler witnesses: 2 passed.
- Constructor/fixture/oracle/RPC witnesses: 15 passed.
- `b0_election`: exit 101, 20 ordinary behavioral assertion failures; zero ignored/filtered.
- `check.sh`: architecture, empty global allowlist, source boundaries, five architecture negatives and formatting passed.
- Four-package all-target Clippy with denied warnings passed.

Read historical `rpc-red.log`: four compiling assertion failures against refusing response behavior. Independently replayed current RPC GREEN; historical RED was not reconstructed.

Inspected pinned OpenRaft `engine_config.rs:44–60`, `config.rs:256–257`, `raft_core.rs:1440–1507`, ticker and entropy call-site search. Findings below are source-derived counterexamples; no new tests or custom builds were added.

## 1. Findings

### [P2-1] B0-03 requires a runtime entropy failure where OpenRaft performs no entropy read

**Location:** `spec/src/b0/sources.rs:126–164`; comparison contract §2 and allocation’s engineering-profile cadence.

**Invariant:** Common consumers must be satisfiable without changing upstream sampling behavior.

**Counterexample:** A faithful adapted OpenRaft constructs EngineConfig, samples its timeout once, and elects normally. B0-03 then injects `Fault::Entropy` into node 1 and requires `EntropyFailed` within 100 clock/work rounds. The pin’s only election-timeout sampling call is EngineConfig construction. Runtime elections reuse the stored timer configuration; node 1 may also remain leader, whose election handler returns immediately. No next entropy access exists in this schedule. The test inevitably reaches `assert!(failed)` with `failed == false`.

**Impact:** A faithful candidate cannot pass the mandatory unchanged consumer. Passing encourages artificial probing or resampling explicitly prohibited by the contract.

**Required correction:** Retain identical exported scenario selection, but make fault cuts follow each pin’s actual access footprint. Test OpenRaft entropy failure during construction/reconstruction and prove no runtime resampling; induce raft-rs runtime failure through a documented reset path.

**Closure test:** A cadence-faithful witness must satisfy the revised cut selection; an OpenRaft mutant adding runtime entropy access must fail. Refusing providers must remain ordinary behavioral RED.

### [P2-2] Endpoint stop and incarnation validity are not shared across handles

**Location:** `spec/src/fixture.rs:183–188`; `fixture/transport.rs:47–86,172–243,327–344`.

**Invariant:** Stop is terminal for a node scope; stale incarnation controls refuse before queue mutation.

**Counterexample:** Obtain two handles with `fixture.endpoint(scope)`. Each receives a separate `Rc<RefCell<bool>>`. Stop the first, then call `emit` or `request` on the second: it remains unstopped and inserts new traffic/RPC ownership. Alternatively, retain an old-incarnation endpoint, construct replacement inputs, then emit through the old endpoint. `emit_inner` checks neither issuer registration nor current incarnation. Its issued message can be released and taken by a current peer because those paths also omit current issuer validation.

**Impact:** Stopped or replaced scopes can resume participation through valid fixture APIs, violating the harness’s stale-callback boundary. Correlated replies receive stronger checks, but one-way/request paths do not.

**Required correction:** Store lifecycle/current-scope authority in shared network state; every handle for a scope must observe terminal stop. Validate current issuer/destination scope and relevant token ownership before each mutating operation.

**Closure test:** Two independently obtained handles must both refuse after either stops. Retained old-incarnation handles must refuse emit/request/take/control without changing queues or RPC inventories after replacement.

### [P2-3] A task can survive a successful scope stop during its own poll

**Location:** `spec/src/fixture/work.rs:127–169,204–213`.

**Invariant:** Successful stop leaves every owned task terminal and releases its future.

**Counterexample:** Spawn a future holding a cloned `WorkProbe`. Its selected poll calls that clone’s `stop()` and then returns `Pending`. The scheduler removed the task from `tasks` before invoking the future. Consequently stop cannot see or cancel it and returns success. `poll` then unconditionally reinserts the pending future into the stopped scope. Subsequent polls return `Stopped`; inventory retains nonterminal work indefinitely.

**Impact:** Stop reports completion while retaining unreachable owned work and its resources. This is reachable with the expressly permitted node-scoped shared handles.

**Required correction:** Keep in-flight ownership represented, or reconcile the result against terminal scope state before reinsertion. A stop during polling must discard/cancel the in-flight future exactly once.

**Closure test:** A future that stops its scope and returns `Pending` must leave only terminal inventory entries, run its drop guard once, and remain terminal after a saved waker fires.

### [P2-4] Cancellation runs arbitrary future destructors while holding the scheduler borrow

**Location:** `spec/src/fixture/work.rs:171–188,204–212`.

**Invariant:** Cancellation must safely release futures and their owned joins/cancellation guards.

**Counterexample:** Register a pending future containing a guard whose destructor calls `cancel(child)` through a cloned `WorkProbe`. Cancelling the parent holds `WorkScope`’s mutable `RefCell` borrow and assigns `future = None`, immediately invoking that destructor. The nested cancellation attempts another mutable borrow and panics. The same sequence occurs during scope stop. The API expressly anticipates futures owning cancellation guards.

**Impact:** Normal ownership cleanup can panic instead of returning a typed result and completing shutdown; descendant cleanup may remain incomplete.

**Required correction:** Mark terminal state and extract futures while borrowed, release the borrow, then drop futures/guards. Apply this to both individual cancellation and bulk stop.

**Closure test:** Parent cancellation and scope stop must handle a destructor cancelling another task without panic; both futures must drop exactly once and inventories must be terminal.

## 2. Invariant analysis

The package-role and declared-edge attack found no unjustified dependency or implementation-type leakage in this std-only object. Constructors retain explicit separately created dependencies; refusing providers perform no source initialization. Both concrete labels select the same ten exported consumers.

Registration and waking do not poll inline. Ordinary selected polls advance actual boxed futures. Current RPC tests establish explicit correlation, peer/kind/carrier rejection and terminal reuse refusal; message drops preserve pending RPCs. These properties do not close the lifecycle counterexamples above.

The candidate-free epoch remains separate from opaque full votes. Proposed OpenRaft single-term-leader/singlethreaded/storage-v2 choices are explicitly private. Signed internal time, source-error propagation, Ready/LightReady, task joins and actual source adaptations remain mandatory later proofs.

B1/B2/full comparison remain required; no candidate-drop shortcut, production receipt, default activation or engine acceptance is claimed.

## 3. Risks and next action

GREEN witnesses establish compiler and controlled-fixture properties only. Synthetic detector tests, local global checks and measured scaffold costs establish no actual engine, dependency-source, durability or production assurance.

The next action is one scoped remediation patch addressing P2-1–P2-4 with regression-first evidence, followed by review of the revised settled object. Real engines and production acceptance remain unqualified.