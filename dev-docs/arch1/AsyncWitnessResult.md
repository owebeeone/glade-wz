# The async witness — result for `async_witness`

Date: 2026-09-22. Written against glade `559cb2c` and glade-wz root `1a4fced`.

**This document records what was run and what it showed. It records no ruling.**
The plan (`dev-docs/arch1/AsyncWitnessPlan.md`, §6, Phase 4) says so in one line:
"The owner records `shaku_confirmed` or `selection_reopened` on the graph. **An
agent does not.**" Nothing here recommends an outcome, and nothing was written to
the decision graph.

**How to read the paths.** Paths starting `ports/`, `fast/` or `real/`, and the
bare files `check.sh`, `arch002-fixture.sh`, `Cargo.toml` and
`architecture-policy.json`, are inside the witness workspace at
`glade/dev-docs/async-witness/`. Paths starting `crates/` are in the sdax-rs
checkout at the pinned rev `ccf06e76a90e22a454471a71f0cf6f5cb878baac`. Paths
starting `iroh-1.2.0/`, `noq-1.3.0/` or `shaku-0.6.3/` are in the local Cargo
registry. A path starting `gyld/` is in the `gyld-wz` workspace. Everything else
is relative to `glade-wz/`.

---

## 1. The question, and the short answer to each criterion

`async_witness` asks whether the chosen wiring survives a real async port,
start-up and cleanup. The wiring was already chosen: Shaku for assembly
(GDL-048, matrix row R27) and sdax-rs for lifecycle (GDL-049, matrix row R28).
The witness does not pick between them. It is an experiment on the pair, with
four acceptance criteria that already existed, and the plan wrote down before
the experiment ran what each criterion's evidence would have to look like. The
sharp one is DI-E04: the real port has to stay usable without framework imports
in the pure crates.

The plan adds one rule so the answer cannot be misread. An sdax-rs defect is not
a Shaku failure (§8.1). Step 3.4's differential — the same plan run with and
without the Shaku part — is what separates the two.

| Criterion | Finding |
|---|---|
| DI-E01 | met, with caveats 2, 3 and 7 |
| DI-E02 | met |
| DI-E03 | met, with caveats 1 and 6 |
| DI-E04 | met, with caveats 4 and 5 |

Each finding is a statement about the **criterion**, measured against the two
columns of the plan's §8.2, and nothing more. It is not the owner's ruling. The
ruling is the owner's, from this document.

AR-08 is reported separately in §4 below, against R28 and Q10, not against
`async_witness`. That is the plan's own instruction (§8.3).

The caveats are numbered once, in §5, and every criterion section names the ones
that apply to it by number. §8.4 of the plan is why: "A caveat must be written
into the Phase 4 result document with its evidence. An unrecorded caveat is a
failure."

---

## 2. Per criterion

### DI-E01

**The criterion, verbatim** (`dev-docs/arch1/DependencyInjectionEvaluation.md:133`):

> One fake clock/store/carrier selection reaches every relevant caller;
> real-provider construction and hidden I/O are rejected in the fast composition

**What §8.2 says would confirm it:**

> One fake instance is observed at every declared consumer by `Arc::ptr_eq`; the
> real-provider construction counter reads 0; `async-witness-fast` declares no
> tokio, iroh or socket dependency, so hidden I/O cannot compile into it

**What §8.2 says would reopen the selection:**

> Shaku cannot express a single override reaching all consumers for a Glade
> contract trait; or eager construction of a real provider cannot be prevented by
> `#[lazy]` or a module split; or preventing it requires changing a Glade contract

**What was run.** The `fast` member, which has no runtime and no socket, over one
shared composition declared in `fast/src/lib.rs`: three ports (`ClockPort`,
`CarrierPort`, `StorePort`), three consumers (`Session`, `Journal`, `Supervisor`),
and a stand-in real provider registered for each port that counts its own
construction and panics in every port method.

```sh
cargo test --locked --offline -p async-witness-fast --lib --tests
```

Six tests in `fast/tests/di_e01_single_selection.rs`:
`one_clock_selection_reaches_every_declared_caller`,
`one_carrier_selection_reaches_every_declared_caller`,
`one_store_selection_reaches_every_declared_caller`,
`the_consumers_themselves_are_one_occurrence_each`,
`no_stand_in_real_provider_is_constructed_in_this_process` and
`the_fast_composition_names_no_socket_api`. One more in
`fast/tests/di_e01_eager_construction.rs`:
`a_registered_stand_in_is_built_at_module_build_unless_it_is_lazy`, in a separate
test binary because the counters are process-wide.

**What it showed, clause by clause against the confirm column.**

*One fake instance at every declared consumer, by `Arc::ptr_eq`.* Yes. The clock
selection is the same `Arc` at the session, the journal, the supervisor's session
and the supervisor's journal. The carrier selection is the same `Arc` at the
session, the supervisor and the supervisor's session. The store selection is the
same `Arc` at the journal and the supervisor's journal. Pointer identity is not
the whole claim in these tests: the one fake clock is advanced by 45 ms and every
consumer then reads 1 045; three sends from three call sites all land, in order,
in the one recording fake carrier; two appends from two call sites produce
sequence numbers 1 and 2 in the one fake store.

*The real-provider construction counter reads 0.* Yes. `real_provider_builds()`
is 0 after the whole composition has been built, resolved and used. The counter is
paid for, not vacuously zero: the separate binary of caveat 2 shows a registered
stand-in constructing itself when it is not overridden, so the zero is not the
zero of a counter that never moves.

*`async-witness-fast` declares no tokio, iroh or socket dependency.* Yes.
`fast/Cargo.toml` declares exactly two dependencies, `async-witness-ports` and
`shaku = "=0.6.3"`. The architecture gate is an allowlist, so any third entry is
an error without anyone having thought to forbid it
(`glade-discover/tools/architecture-check/src/lib.rs:299`), and it reads
`cargo metadata`, so no `#[cfg]` can reach around it.

**Which column the evidence falls in.** All three clauses of the confirm column
are satisfied. None of the three reopen conditions arose: one override did reach
every consumer; eager construction of a registered provider **is** preventable,
by `#[lazy]` or by an override; and no Glade contract was changed to get any of
it (§5, "what is never a caveat").

**Caveats that apply: 2, 3 and 7.**

---

### DI-E02

**The criterion, verbatim** (`dev-docs/arch1/DependencyInjectionEvaluation.md:134`):

> Equal scoped binding → shared identity, independent test scope → isolation, also
> during concurrent first access

**What §8.2 says would confirm it:**

> `ptr_eq` true within one module and false across two independently built
> modules; two threads racing the first resolve of a `#[lazy]` component observe
> exactly one construction

**What §8.2 says would reopen the selection:**

> More than one construction under concurrent first access; or two independently
> built modules share an instance

**What was run.** Three tests in `fast/tests/di_e02_scope_identity.rs`:
`one_scope_hands_out_one_occurrence_of_everything`,
`two_independently_built_scopes_share_nothing` and
`a_concurrent_first_access_of_a_lazy_component_constructs_once`.

**What it showed, clause by clause.**

*`ptr_eq` true within one module.* Yes, for every port, for every consumer, and
for repeated resolutions of the same interface.

*`ptr_eq` false across two independently built modules.* Yes, and also by
behaviour, which is what a test composition actually needs: a frame sent through
scope A's carrier is invisible to scope B's; advancing A's clock to 15 leaves B's
at 20; an append in A's store cannot be scanned from B's.

*Two threads racing the first resolve of a `#[lazy]` component observe exactly one
construction.* Yes. The store is registered `#[lazy]` and overridden with a
counting factory, so the counter reads 0 after `build()` and the race really is
the first access. Two threads meet at a `std::sync::Barrier` and both resolve. The
construction counter reads 1 and both threads hold the same `Arc`. Shaku's
`thread_safe` feature backs a lazy slot with `std::sync::OnceLock`
(`shaku-0.6.3/src/lib.rs:41-50`), which is the reason this is witnessable at all;
the evaluation recorded Dill failing the same probe.

**Which column the evidence falls in.** Both clauses of the confirm column are
satisfied and neither reopen condition arose.

**Caveats that apply: none.** The limits of the race probe are real and are in
§6, not here: a race that passes is evidence that this construction is atomic, not
a proof that every interleaving was explored.

---

### DI-E03

**The criterion, verbatim** (`dev-docs/arch1/DependencyInjectionEvaluation.md:135`):

> Missing/ambiguous role and construction cycle rejected before useful startup; no
> late service-locator surprises

**What §8.2 says would confirm it:**

> The missing binding fails to compile (E0277), the constructor cycle fails to
> compile (E0275), an ambiguous role requires a key and a missing key is an absent
> map entry; no string-keyed or global resolution exists anywhere in the witness

**What §8.2 says would reopen the selection:**

> Any of the three is accepted at compile time and surfaces only at run time; or
> selecting a role requires a service locator

**What was run.** Three examples that must fail to compile, kept behind a
`negative` feature so the gate cannot sweep them in and invert their meaning. They
are run by hand and the diagnostics are read, not the exit codes. Re-run on
2026-09-22 at this head:

```sh
cargo check --locked --offline --features negative --example missing_binding
cargo check --locked --offline --features negative --example construction_cycle
cargo check --locked --offline --features negative --example ambiguous_role
cargo run   --locked --offline --features negative --example cfg_scanner_gap
```

Plus four tests in `fast/tests/keyed_roles.rs`:
`two_recipes_over_one_contract_select_two_distinct_providers`,
`a_role_no_recipe_registered_is_an_absent_entry_not_a_panic`,
`the_record_transport_recipe_shares_the_peer_occurrence` and
`a_second_resolution_of_the_map_is_the_same_occurrence`.

**What it showed, clause by clause.**

*The missing binding fails to compile, E0277.* Yes. `fast/examples/missing_binding.rs`
registers the consumer and neither of the two ports it injects. The compiler
reports `the trait bound MissingPorts: HasComponent<(dyn Clock + 'static)> is not
satisfied`, the same for `dyn Carrier`, and a third E0277 falling out of those
two, `MissingPorts cannot be shared between threads safely`. The module type
cannot be constructed at all, so there is no run in which a port is missing.

*The constructor cycle fails to compile, E0275.* Yes.
`fast/examples/construction_cycle.rs` has admission needing a quota and the quota
needing admission. The compiler reports `overflow evaluating the requirement
Cyclic: HasComponent<(dyn Admission + 'static)>`. It is rejected while the module
is compiled, not by a deadlock or a stack overflow at `build()`.

*An ambiguous role requires a key, and a missing key is an absent map entry.* Yes,
with a correction to what the plan predicted. `fast/examples/ambiguous_role.rs`
registers two keyed occurrences of one interface and then asks for the interface
twice. Asking the natural way reports **E0599**, `no method named resolve found for
struct Ambiguous`, because a multibound interface gets no `HasComponent` impl at
all and so there is no `resolve` method to call. Naming the bound explicitly
reports the **E0277** the plan expected. Both call sites are in the fixture. The
run-time half is `a_role_no_recipe_registered_is_an_absent_entry_not_a_panic`: a
role no recipe registered is `None` from the map, not a panic.

*No string-keyed or global resolution anywhere in the witness.* Yes. `resolve` is
generic in the interface type and `resolve_map` is generic in the key type, so a
name cannot be smuggled in as data. The witness's key type is an enum,
`CarrierRole`, declared in `fast/src/lib.rs`.

**Which column the evidence falls in.** All four clauses of the confirm column are
satisfied. Neither reopen condition arose: none of the three is accepted at
compile time, and selecting a role needs a typed key rather than a locator.

**Caveats that apply: 1 and 6.**

---

### DI-E04

**The criterion, verbatim** (`dev-docs/arch1/DependencyInjectionEvaluation.md:136`):

> The selected real async Glade port remains usable without framework imports in
> its contract/pure libraries

**What §8.2 says would confirm it:**

> `async-witness-ports` compiles and is usable with `glade-wire` as its only
> dependency while a real iroh-backed provider fills its `CarrierPort`; the gate
> fails closed on an injected `shaku` entry; `cargo tree --invert` finds no
> contract-role package

**What §8.2 says would reopen the selection:**

> The real port cannot be reached without a framework type appearing in a contract
> or pure crate's manifest or public signature; or making it work requires adding
> `Sync`, a lock or a boxed public future to a Glade contract

**What was run.** The real provider is `WitnessCarrier` in
`real/src/peer_carrier.rs`, over the node's own
`glade_node::iroh_carrier::PeerEndpoint`, unmodified. Two witness nodes bind
localhost QUIC endpoints, one dials the other, both complete the node-to-node
HELLO, and one real glade `Frame` crosses the witness's `CarrierPort`
(`real/tests/peer_carrier.rs`,
`two_witness_nodes_bind_dial_hello_and_exchange_one_frame`). The wall itself is
checked three ways by `check.sh`, which the gate runs on every invocation, and by
`arch002-fixture.sh`.

**What it showed, clause by clause.**

*One dependency, with a real provider behind the port.* Yes.
`cargo tree -p async-witness-ports` is two lines: the crate and `glade-wire`.
`check.sh` also asserts that `glade-wire` is itself still a one-line tree, so the
pure crate the port's types come from has not acquired anything either.

*The gate fails closed on an injected `shaku` entry.* Yes. `arch002-fixture.sh`
copies the workspace, injects `shaku` into the contract crate's manifest in the
copy, and passes only on the exact line `ARCH-002 async-witness-ports: undeclared
dependency normal:shaku`, after a positive control on the untouched copy. Re-run
on 2026-09-22: it produced that line, in 1.06 s. It never touches the live
manifests or the lockfile. Each of its refusal branches was produced on purpose
during Phase 3, including one with `tokio` injected instead, which it rejected as
the wrong diagnostic rather than accepting a non-zero exit.

*`cargo tree --invert` finds no contract-role package.* Yes. `check.sh` inverts the
tree from six frameworks — `shaku`, `sdax`, `sdax-tokio`, `sdax-testkit`, `tokio`
and `iroh` — and fails if `async-witness-ports` appears in any of them. It does
not appear in any of them. This is the check the manifest half cannot make, because
the checker shells `cargo metadata --no-deps` and so sees workspace members only
(`glade-discover/tools/architecture-check/src/lib.rs:255-256`).

*No framework type in a public signature.* Yes, by inspection, which is what §8.2's
second reopen clause needs and no tool here provides. `ports/src/lib.rs` names
`shaku`, `sdax` and `tokio` exactly three times, all in doc comments saying the
crate must not name them. Every type in a public signature is `std` or
`glade-wire`: `FrameType`, `i64`, `String`, `Vec<u8>`, `Pin<Box<dyn Future +
Send>>`, `Result`, and the crate's own `CarrierError` and `StoreError`. The
`std::sync::Mutex` inside `FakeCarrier` and `FakeStore` is a private field.

*No `Sync`, lock or boxed public future added to a Glade contract.* Yes. No Glade
contract was changed at all; see §5, "what is never a caveat". The ports crate's
own `Any + Send + Sync` bound and its boxed `PortFuture` are pre-existing facts
about the witness's own crate and are caveats 4 and 5, not this clause.

**Which column the evidence falls in.** All three clauses of the confirm column are
satisfied, and neither reopen condition arose.

**Caveats that apply: 4 and 5.**

---

## 3. The differential, and the governing distinction

§8.1 sets the rule the differential exists to serve. A failure caused by the
injector or its bridge records `selection_reopened`. A failure caused by sdax-rs
records nothing on `async_witness` at all; it is an R28/Q10 matter and the plan
says to stop and report rather than reopen the injector. Step 3.4's differential
is what tells the two apart:

> If the identical sdax plan is `is_clean()` and frees the port **without** the
> Shaku module step, and is not clean or does not free the port **with** it, the
> fault is the bridge's and `selection_reopened` is recorded. If both runs fail the
> same way, the fault is not Shaku's.

**What was compared.** `real/tests/differential.rs` has one function, `observe`,
taking one `bool`. The `bool` reaches exactly one place, `PeerHarness::with_shaku_module`,
which adds one node to the plan. The plan, the bodies, the budgets, the runtime and
the order of the observations are shared. The structural half of that claim is
asserted statically in `real/tests/shaku_assembly.rs` by
`the_module_step_adds_a_node_and_changes_no_other_edge`: with the module the plan
has exactly one more node and exactly three more edges, and every edge of the plain
plan survives.

Each run is compared on: whether the report is `is_clean()`; its `outcome`; the
number of faults, cleanup failures and ambiguous records; the names in
`incomplete`; the exported output; what `TokioRuntime::shutdown` answered; what
`tracked()` read afterwards; whether each endpoint's UDP port came back; which
releases completed, as a set; the three parent/child pairs the declaration
constrains; and the Shaku construction counter.

**What it showed.** The two runs did not diverge. Every compared field is equal.
The comparison is not vacuous — the tests also assert the values compared:
`outcome` is `Ok`, the output is `Some(1)`, `shutdown` is `Ok(())`, `tracked()` is
0, `incomplete` is empty, the provider-build counter is 0, all four resources
released, and all three ordered pairs answered child-before-parent. Both runs were
clean and both freed every port they bound, in either running order
(`the_answer_does_not_depend_on_which_side_runs_first`) and over three repeated
A/B pairs (`the_answer_repeats`). No fault of any kind arose, so §8.1 has nothing
to attribute to either side.

**Why the release sequence was not compared, and the partial order was.** The
first version of the differential compared the *sequence* in which releases
completed, and the two sides appeared to diverge: `["Served", "Dialed",
"Acceptor", "Dialer"]` against `["Dialed", "Served", "Dialer", "Acceptor"]`. What
gave it away is that the sequence also varied between runs of the **same**
configuration — three distinct total orders across six runs. Nothing was wrong.
`Served` and `Dialed` share no `needs` path, and neither do `Acceptor` and
`Dialer`, so `release_order()` reports both pairs `unordered`
(`crates/sdax/src/view.rs:485`) and the engine is free to finish them in either
order. That is AR-08's third clause, "Concurrent independent cleanup can
progress", observed live.

So the differential compares the partial order the plan declares: which releases
completed, and `released_before(child, parent)` for each of the three pairs that
are genuinely ordered. The exclusion is reading the declaration rather than
excusing a difference:
`the_uncompared_pairs_are_the_ones_the_plan_declares_unordered` asserts
statically that the pairs left out are exactly the ones `release_order()` calls
unordered. A total order is not an observable of this system.

**What a divergence would have meant.** If the plain run had been clean and freed
its ports while the assembled run had not, §8.1 makes that the bridge's fault and
`selection_reopened` the record. If both had failed the same way, the fault would
not have been Shaku's and the plan says to report it against R28/Q10 instead.
Neither happened.

---

## 4. AR-08, reported separately against R28 and Q10

§8.3 says AR-08 "is witnessed by Steps 2.1–2.4, 3.2 and 3.4 and is reported
**against R28/Q10**, not against `async_witness`". R28
(`dev-docs/GladeBuyBuildMatrix.md:113`) names the residue as "Plan authoring for
the node's resources and services, the AR-08 witness on sdax-rs, the async bridge
to Shaku-built components". Q10 (`dev-docs/GladeBuyBuildMatrix.md:157`) sets the lean: "Run the AR-08 and
DI-E01..E04 witness on sdax-rs plus Shaku; keep Tokio primitives as the fallback
if it fails."

The clause under test, verbatim (`dev-docs/arch1/RuntimeAndAssurance.md:126`):

> AR-08 | Parent resource remains owned while child cleanup is pending; partial
> shutdown retries without double-release. Concurrent independent cleanup can
> progress; no invisible detached work.

§8.3 lists six things that must hold. Each is below with what showed it.

| §8.3 clause | Supported? | Evidence |
|---|---|---|
| Reverse order is asserted statically by `release_order().before(...)` | yes | `real/tests/release_order.rs`: `child_cleanup_finishes_before_its_parent_begins`, `the_parent_is_never_released_before_its_child`, `two_independent_branches_may_release_concurrently`, `the_declared_edges_are_the_only_edges`, `asserting_the_order_runs_no_body`. Against the real port, `real/tests/peer_carrier.rs`: `the_link_is_released_before_the_endpoint_that_produced_it` |
| A clean run reports `is_clean()`, never merely `outcome == Ok` | yes | `real/tests/lifecycle.rs`: `a_clean_drain_reports_is_clean_and_not_merely_ok`. The distinction is not rhetorical: `an_expired_budget_names_the_resource_it_abandoned` ends `Outcome::Ok` and is **not** clean |
| An expired budget lists the unfinished resource in `report.incomplete` | yes | `an_expired_budget_names_the_resource_it_abandoned`: `incomplete` has exactly one record and it is the endpoint. `Report::incomplete` is "Obligations the shutdown budget abandoned" (`crates/sdax/src/report.rs:333`) |
| A cancelled run still executes the release graph | yes | `a_cancelled_run_still_executes_the_release_graph`: outcome `Cancelled`, not clean, no cleanup failures, and the whole release graph ran in order anyway |
| `TokioRuntime::tracked()` is 0 afterwards | yes, with the qualification below | `real/tests/ownership.rs`: `a_clean_run_leaves_no_tracked_task`, and `tracked_counts_the_run_while_it_is_still_in_flight` so the zero is not vacuous. `a_dropped_run_still_drains_through_a_tracked_drainer` shows a dropped run leaves one tracked drainer and still releases |
| The recorded UDP port can be re-bound | yes | `real/tests/peer_release.rs`: `a_clean_run_frees_every_port_it_bound`. The check is falsifiable: `the_release_check_can_answer_still_bound` holds a port of its own and makes the helper spend the whole two-second bound saying so |

Two further pieces of AR-08 evidence, from Step 2.3. The witness built a concrete
`ManagedResource` whose cleanup is an sdax run and put it through
`glade-lifecycle-api`'s own LC-001..LC-006 suite unchanged
(`real/tests/managed_resource.rs`: `lc_001_shutdown_is_terminal_and_idempotent`
through `lc_006_cancel_does_not_drain_work`). The suite's own negative still
fires: `rejects_false_cleanup_success` panics when an adapter closes cleanly and
then claims `Open`. And `a_retry_does_not_repeat_completed_cleanup` shows the
"partial shutdown retries without double-release" half: a retry re-attempts only
what the ledger still shows outstanding.

### Two findings that qualify all of this, plainly

**A leaked endpoint clone or `Connection` is invisible to the sdax report.** In
the variant where one clone of the acceptor's `PeerEndpoint` escapes the
composition, the run is `is_clean()`, `report.incomplete` is empty,
`TokioRuntime::shutdown` answers `Ok(())` and `tracked()` reads 0 — and the
acceptor's UDP port is still bound at the two-second bound. It frees within
microseconds of the clone being dropped. An escaped link `Connection` behaves the
same way: held for 2.005 s, free 41 µs after the drop. Both are in
`real/tests/peer_release.rs`
(`an_escaped_clone_keeps_the_port_bound_until_it_is_dropped` and
`an_escaped_connection_holds_the_port_as_an_endpoint_clone_does`). Nothing sdax
can see is wrong, because nothing sdax can see *is* wrong: the release ran, the
obligation was discharged, and a clone the engine never knew about outlived it.
Of §8.3's six clauses, **only the re-bind sees a leak.** `is_clean()` is necessary
and not sufficient. That is why the plan chose a port whose leak is observable
from outside the process.

**`tracked() == 0` does not prove nothing was abandoned.** When a shutdown budget
expires, sdax aborts the async task as well as listing it
(`crates/sdax/src/host/engine/cleanup.rs:310-333`), so
`TokioRuntime::shutdown` still answers `Ok(())` and the count still falls to zero.
`report.incomplete` is the abandonment signal, not the task count.
`an_abandoned_obligation_is_reported_and_leaves_nothing_running` is the test that
makes this concrete, and the witness therefore reads `tracked()` together with an
empty `report.incomplete`, never on its own.

**Caveat 3 applies to AR-08 as well as to DI-E01.**

---

## 5. The caveats, numbered, with their evidence

§8.4 defines a caveat as "an observation that is undesirable, **already recorded
in the evaluation**, and does not prevent the criterion being met", and then says
"A caveat must be written into the Phase 4 result document with its evidence. An
unrecorded caveat is a failure." These are the seven.

### Caveat 1 — the cycle diagnostic names a bound, not the loop

*Applies to DI-E03.* Already recorded at
`dev-docs/arch1/DependencyInjectionEvaluation.md:39`: the A-to-B graph "fails
compilation, E0275; diagnostic is a trait-bound overflow, not a friendly graph
explanation".

Evidence: `fast/examples/construction_cycle.rs` declares two components that need
each other. `cargo check --locked --offline --features negative --example
construction_cycle` reports `error[E0275]: overflow evaluating the requirement
Cyclic: HasComponent<(dyn Admission + 'static)>`, four errors in all. A reader is
told the composition is impossible. A reader is not told which two services form
the loop. DI-E03 asks only that the cycle be rejected before useful startup, and
it is.

### Caveat 2 — eager construction needs an override or `#[lazy]`

*Applies to DI-E01.* Already recorded at
`dev-docs/arch1/DependencyInjectionEvaluation.md:41`: "Default eager module
constructs other registered components even when another service is overridden.
`#[lazy]` successfully deferred unused construction". §8.4 says using either is a
caveat.

Evidence, ordered so the readings cannot interleave, in
`fast/tests/di_e01_eager_construction.rs`
(`a_registered_stand_in_is_built_at_module_build_unless_it_is_lazy`). The
construction counter for the stand-in store reads:

| Point | Counter |
|---|---|
| before anything is built | 0 |
| after `EagerComposition::builder().build()` | 1 |
| after `LazyComposition::builder().build()` | 1, unchanged — the lazy build deferred it |
| after the first `resolve()` of that lazy component | 2 |

Nothing in either composition injects the store's interface — it is an
unreferenced registration — and it is built anyway unless it is `#[lazy]`. The two
components that *were* overridden stayed unbuilt in both, which is the half DI-E01
relies on.

The consequence, stated plainly: DI-E01 passes because every port a test binds is
overridden, not because Shaku prunes what a composition does not use. A
composition that registers a real provider it does not override gets that provider
built.

### Caveat 3 — the `Arc` escape, and what the clean differential really rests on

*Applies to DI-E01 and AR-08*, which is the scope §8.4 gives it. Already recorded
at `dev-docs/arch1/DependencyInjectionEvaluation.md:85-86`: "Both Arc-escape probes
show a service surviving container drop. Dropping a container is therefore not even
a guarantee that all its services have been destroyed". §8.4 says this is a caveat
**only if** the witness's own composition demonstrably prevents it, and Step 3.2's
re-bind is the proof; if the port cannot be re-bound it is not a caveat but a
DI-E01 and AR-08 failure.

Evidence that the composition prevents it: `a_clean_run_frees_every_port_it_bound`
frees both ports. `a_module_that_outlives_the_step_holds_no_socket` stashes the
built Shaku module so it survives the whole run, and both ports still free in
microseconds. Evidence that the check can fail:
`an_escaped_clone_keeps_the_port_bound_until_it_is_dropped` holds the port for the
whole bound.

**And now the part that has to be read together with it.** The plan's §4.2
predicted the risk at this seam — "The Shaku module holds `Arc` clones of things
derived from the endpoint, and an escaped clone keeps the UDP socket bound" — and
the risk did not arise, for a reason §4.2 does not state. The provider gives its
handles up **by value**. `WitnessEndpoint` owns its `PeerEndpoint` as a
`Mutex<Option<..>>` and the release body takes it out and calls
`PeerEndpoint::close(self)`; `WitnessCarrier` does the same for the link's
`SendStream`, `RecvStream` and `Connection` (`real/src/peer_carrier.rs`).

That shape was forced by sdax, not by Shaku. An sdax release body receives an
`Arc<T>`, and for an **async** release the engine leaves that `Arc` in its slots
rather than taking the value out; it takes the value out only for a `by_drop`
release (`crates/sdax/src/host/bodies.rs:404-421`). A provider that closed a
*clone* would leave a live clone in the slots for as long as the run's storage
lives.

**So, plainly: a provider that closed a clone would have left the port bound with
or without Shaku.** The clean differential therefore depends on a discipline in
the provider, not on Shaku. A module that deliberately outlives its step holds no
socket, but only because the carrier it holds owns nothing by then. The contrast
is the endpoint clone of Step 3.2 — a handle the engine never owned — which does
keep the port bound.

This is also reported against R28/Q10 in §7: an async release body cannot be
handed the owned value.

### Caveat 4 — `async-witness-ports` requires `Any + Send + Sync`

*Applies to DI-E04.* `ClockPort`, `CarrierPort` and `StorePort` all declare
`Any + Send + Sync` as supertraits (`ports/src/lib.rs`), and the crate's own doc
comment says the `Sync` is what lets the facade
`trait Carrier: CarrierPort + shaku::Interface` compile.

The bound is not *only* Shaku's. An injected port is shared as
`Arc<dyn CarrierPort>` across tasks on a multi-thread runtime, which needs `Sync`
whatever assembles it, and `Any` is a `'static` bound that any `'static` type
already satisfies. All three are `std` traits and the crate never names Shaku. But
the coincidence is a coincidence, and it is honest to record that the port was
written knowing what `shaku::Interface` demands — which is a blanket impl over
every `T: Any + Send + Sync` with no `?Sized` (`shaku-0.6.3/src/component.rs:47-55`).

**This is not a Glade contract change.** `async-witness-ports` is the witness's own
crate, it dates from Phase 0, and `git log -- ports/` is two commits, `b2b6779` and
`8a7ed44`, both Phase 0. §8.2's reopen clause is about adding `Sync` "to a Glade
contract"; none was touched.

### Caveat 5 — `async-witness-ports` boxes its port futures

*Applies to DI-E04.* `PortFuture<'a, T>` is
`Pin<Box<dyn Future<Output = T> + Send + 'a>>` (`ports/src/lib.rs`), a boxed public
future in the witness's own port.

It is boxed to stay dyn-compatible: `impl Future` in return position is not, which
is E0038 and which is exactly why `glade-lifecycle-api`'s `ManagedResource`
(`glade/contracts/lifecycle-api/src/lib.rs:57-60`) is not dyn-compatible. sdax-rs
boxes every trait future for the same stated reason. `Pin`, `Box` and `Future` are
all `std`.

**This is not a Glade contract change either.** `ManagedResource` still returns
`impl Future` and was not touched. Same two Phase 0 commits as caveat 4.

### Caveat 6 — the ambiguity diagnostic, and what Shaku's diagnostics cannot tell apart

*Applies to DI-E03.* The plan predicted E0277 for the ambiguous role. The natural
call site reports **E0599** instead: `no method named resolve found for struct
Ambiguous`, because a multibound interface gets no `HasComponent` impl at all, so
there is no `resolve` method. E0277 appears only where the bound is named
explicitly. Both call sites are in `fast/examples/ambiguous_role.rs` and both
diagnostics were reproduced on 2026-09-22.

The wider point is the one worth carrying forward: `missing_binding` and
`ambiguous_role` report the same *kind* of failure for opposite causes — nothing
bound and two things bound. **Shaku's diagnostics do not distinguish absence from
ambiguity.** DI-E03 is still met, because the ambiguity fails to compile, which is
what the criterion asks.

### Caveat 7 — the `std::net` claim is a source assertion, not a manifest fact

*Applies to DI-E01.* The plan's §4.4 said hidden I/O would be proved absent
mechanically, because "`async-witness-fast` does not depend on tokio, iroh or
`std::net` at all". Two of those three are manifest facts and the gate decides
them. `std::net` is not: it is in the standard library, so a blocking socket needs
no manifest entry at all and no dependency check can see it.

What replaced it: `the_fast_composition_names_no_socket_api` in
`fast/tests/di_e01_single_selection.rs` pulls the crate's one source file in with
`include_str!` at compile time and asserts it names none of `std::net`,
`TcpStream`, `TcpListener` or `UdpSocket`. That is a **textual assertion over one
file**, not a proof about the whole crate, and it would also trip on the string
appearing in a comment. It is weaker than the manifest half and is recorded as
such.

### What is never a caveat

§8.4's last paragraph: "Any change to a Glade public contract made to satisfy the
container. `DependencyInjectionEvaluation.md:96-97` is explicit: 'Do not add
`Sync`, locks or boxed public futures merely to please a container.' If the bridge
needs one, that is `selection_reopened`, stated plainly."

The file the plan names there is
`dev-docs/arch1/DependencyInjectionEvaluation.md:96-97`, and it reads as quoted.

**No Glade contract was changed.** `git -C glade log --oneline 4467696..HEAD --
contracts/ wire-rs/` returns nothing across all 23 commits of the witness's range.

One change was made outside the witness workspace, to the node: `22e3cd3`, "Give
PeerEndpoint a graceful close", 69 insertions in `glade/node/src/iroh_carrier.rs`
and no other file. The plan's Phase 0 update records it as owner-ruled and as "the
one exception to §10's 'the node is not edited'". It was needed because
`PeerEndpoint` could only be dropped: it had no `close()` and its endpoint field is
private, so the witness could not await iroh's drain. It is a change the node needs
anyway for a graceful shutdown it does not have today. It is not a contract
change and it was not made to please a container.

---

## 6. What the witness does not show

- **One machine, one toolchain.** Every figure and every run is from the machine
  and toolchain named in the witness `README.md`'s "Measured — Step 4.1" section.
  Nothing was run on a second machine, on Linux, or on another rustc.
- **Localhost QUIC only.** The two witness endpoints bind loopback addresses and
  dial each other directly. No relay, no address lookup, no NAT, no real network.
  The relay and discovery questions are elsewhere on the graph and the witness
  touches neither.
- **The DI-E02 race is a probe, not a search.** Two threads meet at a barrier and
  both resolve once, per test run. The lane owner separately recorded running the
  compiled binary 200 times without a failure (glade `aa3c5da`'s commit message);
  that figure is not in the witness `README.md` and was not re-run in Phase 4. A
  passing race is evidence that this construction is atomic. It is not an
  exhaustive interleaving search, and two threads and a barrier are a probe rather
  than a proposed Glade threading architecture.
- **DI-E05, DI-E06 and DI-E07 are not witnessed**
  (`dev-docs/arch1/DependencyInjectionEvaluation.md:137-139`; the plan's §10 says
  so and says why). DI-E06 is partly exercised incidentally by Step 1.5 — two
  roles over one contract, one shared occurrence, in `fast/tests/keyed_roles.rs` —
  and is **not** claimed. DI-E05 overlaps AR-08 heavily and Steps 2.2 and 3.4
  produce evidence toward it, but that evidence is reported as AR-08 under R28 and
  DI-E05 is **not** claimed complete.
- **DI-E01 could not be witnessed against the node's own clock or store.** The
  node has no clock trait at all — `now_ms()` is a free function reading the wall
  clock — and its `Store` is synchronous blocking file I/O with no `async fn`
  anywhere in it (plan §3.3). So the witness declared its own `ClockPort` and
  `StorePort` and nothing claims otherwise. Only the carrier has a real Glade
  provider behind it.
- **The architecture checker has a `#[cfg]` blind spot, and it is wider than the
  plan stated.** `fn conditional`
  (`glade-discover/tools/architecture-check/src/lib.rs:97-102`) tests whether an
  attribute's path is `cfg` or `cfg_attr` and never evaluates the condition, so an
  always-true `#[cfg(all())]` is skipped exactly like a never-true `#[cfg(any())]`.
  The checker's own refusal of `#[path]` modules sits behind the same guard
  (`glade-discover/tools/architecture-check/src/lib.rs:211-214`), so
  `#[cfg_attr(all(), path = "hidden.rs")]` compiles a whole module
  the scanner never examines and the gate still reports `PASS`. This was measured
  in both directions and the fixture is `fast/examples/cfg_scanner_gap.rs`, which
  breaks the standing rule on conditional compilation on purpose and says so at its
  head. **What it weakens** is the trait half of the gate, `ARCH-003`. **What it
  does not weaken** is the dependency half, which is read from `cargo metadata` and
  cannot be reached by any `#[cfg]` — so DI-E01's and DI-E04's manifest claims are
  unaffected. The checker belongs to `glade-discover` and the witness did not
  change it.
- **INV-5 and INV-6 in sdax-testkit have no independent falsification** — the core
  derives the release order from the edge set, so its own checker cannot disagree
  with it. That is why the witness asserts release order from the Glade side, with
  its own `release_order().before(...)` assertions, rather than relying on sdax's
  checker.
- **The `ManagedResource` adapter is a harness object**, not a proposed Glade
  supervisor. It lives in a test target because `glade-lifecycle-api` is a
  dev-dependency of `async-witness-real`.
- **The wall figures are from a loaded machine.** The owner's own application
  instance was running throughout and was not touched. See the README section for
  the load averages beside each measurement.

---

## 7. Findings outside the question, for the owner

None of these is a Shaku result and none of them changes a criterion. They are
reported against R28 and Q10, or as plain observations.

1. **sdax's async release cannot be handed the owned value.** A release body
   receives an `Arc<T>`. The engine takes the value out of its slots only for a
   `by_drop` release; for an async release the `Arc` stays in the slots
   (`crates/sdax/src/host/bodies.rs:404-421`). So a resource whose cleanup consumes
   the handle — as `PeerEndpoint::close(self)` does, because iroh frees the UDP
   socket only when every clone is gone (`iroh-1.2.0/src/endpoint.rs:1717-1718`) —
   has to own it behind interior mutability and take it out itself. The witness does
   that in `real/src/peer_carrier.rs`. Anything Glade builds on sdax with a
   consuming close will meet the same shape.

2. **sdax runs a run's release graph exactly once.** There is no API to re-run a
   completed run's cleanup and no retry attribute reaches a release body.
   `ManagedResource` meanwhile requires that "repeated shutdown MUST retry
   unfinished cleanup without repeating completed irreversible cleanup"
   (`glade/contracts/lifecycle-api/src/lib.rs:39-41`). So the retry loop lives
   **above** sdax: the Step 2.3 adapter owns the retry and sdax owns each attempt,
   over only what the ledger still shows outstanding
   (`real/tests/managed_resource.rs`, `a_retry_does_not_repeat_completed_cleanup`).
   A Glade supervisor would need the same loop. This is not a defect.

3. **`sdax_testkit::FakeClock::sleep` registers no waker.** It returns `Pending`
   from a `poll_fn` that only compares an atomic against a deadline
   (`crates/sdax-testkit/src/clock.rs:59-71`), so a task sleeping on it on a
   multi-thread tokio runtime never wakes unless something else polls it. Every
   witness test is multi-thread, because `TokioRuntime::new` panics on a
   current-thread handle. The witness therefore declared its own `WitnessClock`
   (`real/src/two_clocks.rs`), which records wakers, wakes them when advanced, and
   reports how many registered deadlines an advance crossed. That last property is
   what lets `the_engine_deadline_falls_where_the_port_clock_says_it_should` say
   *where* the engine's budget expired: ticks one to nine cross no deadline, the
   tenth crosses exactly one, and the port's own `FakeClock` reads 500 ms for the
   same wait. Anyone driving sdax deterministically on a multi-thread runtime will
   hit this.

4. **`TokioRuntime::current_thread_no_background_drain` exists and the witness did
   not use it** (`crates/sdax-tokio/src/lib.rs:154`). The plan did not mention it.
   Every witness test uses `TokioRuntime::new` on a multi-thread handle instead. It
   is named here so a later reader knows the choice was made rather than missed.

5. **`Err(n)` from `TokioRuntime::shutdown` is reachable only through a blocking
   step.** `shutdown(budget)` answers `Err(self.tracker.len())` when the budget
   expires with tasks still live (`crates/sdax-tokio/src/lib.rs:216-222`). But
   `abandon` aborts the async task as well as recording it, and excludes only
   `BlockingStep` and `Template`, because "a blocking body cannot be aborted"
   (`crates/sdax/src/host/engine/cleanup.rs:310-333`). So an abandoned async
   obligation shows up in `report.incomplete` and the task count still falls to
   zero. The shape that would answer `Err(n)` is a blocking step — and the plan
   already names where Glade has one: the node's synchronous `Store` is held behind
   a `tokio::sync::Mutex` and locked from async code, so blocking file I/O already
   runs on the executor (plan §3.3). That is where `Err(n)` would bite, and tokio's
   runtime drop then waits for it with no budget.

6. **The checker's `#[cfg_attr(..., path = ...)]` bypass in `glade-discover`.**
   Described in §6 above. It is a real hole in the trait half of an existing gate
   that is used by the six contract packages today, not only by the witness. The
   witness filed an executable fixture for it and changed no checker code. It is
   the owner's to decide what to do about.

7. **iroh 1.2.0 runs on `noq` 1.3.0, not quinn.** `cargo tree -p iroh --depth 1`
   lists `noq`, `noq-proto` and `noq-udp`, and `cargo tree --invert quinn` answers
   that the package specification matched no packages in this workspace at all. A
   passage in the witness was originally derived from quinn's endpoint driver and
   had to be corrected and re-grounded on a measurement. noq stops its endpoint
   driver when the connection map is empty **and** either the handle count is zero
   **or** `close` has been called (`noq-1.3.0/src/endpoint.rs:471-476`), which
   settles nothing about the socket in either direction — hence the measurement.
   Anything in Glade that reasons about iroh internals should cite noq.

8. **The port-release lag.** The node's own `close()` tests measure 6 to 10 ms
   between `close` resolving and the port being free, with the node's whole suite
   running in parallel (`glade/node/src/iroh_carrier.rs:218-221`). The witness
   measured 4.7 to 15.2 µs for the same thing — three orders of magnitude less. The
   difference is not a faster machine: the node's test asks immediately after
   `close` resolves, whereas a witness run has a second endpoint to release and a
   report to assemble in between, so iroh's driver has already wound down by the
   time the check happens. Both bound the wait at two seconds, which is the right
   posture: the bound is there for the case where the driver has *not* wound down.

---

## 8. What was built, and how to rerun it

**The workspace.** `glade/dev-docs/async-witness/`, a separate Cargo workspace with
its own tracked `Cargo.lock`, following `glade/dev-docs/di-eval/` exactly. It is
never referenced from `glade/node/Cargo.toml`. Dependency flows one way: the
witness path-depends on `glade-node` and `glade-wire`; neither gains anything.

**The members.**

| Member | Role | Dependencies | Tests |
|---|---|---|---|
| `async-witness-ports` | contract | `glade-wire` only | 7 |
| `async-witness-fast` | harness | ports, `shaku =0.6.3` | 18 |
| `async-witness-real` | harness | ports, shaku, `sdax`, `sdax-tokio`, `glade-node`, `glade-wire`, `iroh`, tokio; dev `sdax-testkit`, `glade-lifecycle-api` | 46 |

**The gate**, from the witness workspace directory:

```sh
sh check.sh
```

It runs the architecture checker, then `arch002-fixture.sh`, then the `glade-wire`
purity assertion, then six `cargo tree --invert` inversions, then
`cargo test --lib --tests`, `cargo fmt -- --check` and
`cargo clippy -- -D warnings`. 71 tests. It deliberately omits
`--all-features --all-targets`, because sweeping the must-fail examples in would
invert their meaning.

**The fixtures run by hand** — three that must fail to compile, one that must
compile, and the gate's own `ARCH-002` fixture. Read the diagnostics, not the exit
codes:

```sh
cargo check --locked --offline --features negative --example missing_binding      # E0277
cargo check --locked --offline --features negative --example construction_cycle   # E0275
cargo check --locked --offline --features negative --example ambiguous_role       # E0599, then E0277
cargo run   --locked --offline --features negative --example cfg_scanner_gap      # MUST compile
sh arch002-fixture.sh                                                             # one exact ARCH-002
```

All five were re-run on 2026-09-22 at this head and behaved as recorded.

**The measurements.** The full table, with its conditions and its load averages,
is in the witness `README.md` under "Measured — Step 4.1". The headline figures,
on an Apple M3 Pro with 12 cores under real desktop load, `dev` profile,
rustc 1.96.0:

| | min | median | max | repeats |
|---|---|---|---|---|
| `cargo test -p async-witness-ports --lib --tests`, warm | 0.09 s | 0.09 s | 0.21 s | 5 |
| `cargo test -p async-witness-fast --lib --tests`, warm | 0.11 s | 0.11 s | 0.14 s | 5 |
| `cargo test -p async-witness-real --lib --tests`, warm | 2.95 s | 3.04 s | 3.39 s | 5 |
| `sh check.sh`, warm, all 71 tests | 4.92 s | 7.47 s | 8.05 s | 5 |
| Cold build of `real` alone, empty target dir | — | 42.88 s | — | 1 |

The cold `real` build produced 1.6 GiB of artefacts; cold `fast` alone took 7.10 s
for 71 MiB. **None of these is a budget.** The plan names no seconds threshold and
`dev-docs/LibraryBoundaryAndTestingPolicy.md:68` says the policy "deliberately does
not impose a universal seconds threshold". They are measurements, per
`dev-docs/LibraryBoundaryAndTestingPolicy.md:66` — "Do not describe an unmeasured
target as an achieved performance guarantee."

**The commit range.** `4467696..559cb2c` in `glade`, 23 commits, from the workspace
skeleton to this document's measurement section. The glade-wz root locks it at
`1a4fced`.

**No stop trigger of §9.2 fired.** The sdax rev was on the remote and fetchable;
the bridge compiled without any remediation round; nothing required editing
`glade-lifecycle-api` or any other contract; no sdax-rs defect blocked Phase 2; the
real target's build cost did not make Phase 3 impractical, so the WebSocket
runner-up was never substituted; and the fast target missed no budget, because the
plan names none.

---

## 9. What the owner records

The decision graph offers two alternatives under `AsyncWitness`. In its own words
(`gyld/examples/glade-decisions.gyld.py`):

> **`ShakuConfirmed`** — "The witness passes and Shaku stays where the ruling put
> it, at assembly only. Nothing else has to move."

> **`SelectionReopened`** — "The witness fails and the injector choice goes back on
> the table. Everything already wired through assembly is reconsidered with it."

What choosing each would mean next, neutrally:

- **`ShakuConfirmed`** closes R27's residue on the injector question. The remaining
  R27 work named in the matrix — assembly-only use and the six binding recipes —
  stays as it is, and the node's own composition root becomes design work that can
  start from the witness's shape rather than from an open question. The caveats in
  §5 stay on the record as things a real composition has to handle, particularly
  caveat 2 and caveat 3's by-value discipline.
- **`SelectionReopened`** puts the injector back on the table and, in the graph's
  own words, reconsiders everything already wired through assembly with it. The
  alternatives R27 examined — Dill and a hand-rolled context bag — would come back
  into scope, which the plan's §10 says is the only door to that work.

**The ruling is the owner's.** This document states no verdict on `async_witness`
and nothing was recorded on the decision graph.

Separately from the ruling, §4's AR-08 result and §7's findings are reported
against R28 and Q10 and are the owner's to file there.
