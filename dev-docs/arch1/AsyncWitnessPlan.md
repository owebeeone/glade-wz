# The async witness — plan for `async_witness`

Date: 2026-09-21. Status: **DRAFT for the owner's review. Nothing here is a ruling.**
No code, manifest, decision graph or other document was changed to write this. It was
produced read-only against the workzones as they stand on 2026-09-21. It plans one
bounded experiment and nothing beyond it.

Prerequisites, both already decided: **Shaku** as the injector (GDL-048, matrix row
R27) and **sdax-rs** as the lifecycle library (`LifecycleCompositionRuling`,
2026-09-21, confirming GDL-049). This plan does not reopen either.

---

## 1. The question, in plain words

`async_witness` asks: **does the chosen wiring actually survive a real async port,
start-up and cleanup?**

It is not a preference between options. It is an experiment with two outcomes, and
the experiment decides which one is recorded:

- **`shaku_confirmed`** — the witness passes. Shaku stays exactly where the ruling
  put it, at assembly only. Nothing else has to move.
- **`selection_reopened`** — the witness fails. The injector choice goes back on the
  table, and everything already wired through assembly is reconsidered with it.

The stakes are stated in the graph: Shaku builds synchronously, is not a lifecycle
supervisor, and needs a reviewed bridge for runtime-polymorphic injection. E04 is
the sharp one — the real port must stay usable without framework imports in the
pure crates.

One thing this plan adds, because the code demands it: **an sdax-rs defect is not a
Shaku failure.** Section 8 separates the two so the result cannot be misattributed.

---

## 2. The acceptance criteria, verbatim

These already exist. They are used as written and no new ones are invented.

### 2.1 From `arch1/DependencyInjectionEvaluation.md:129-136`

Section heading, verbatim: "Proposed acceptance for the next witness, not completed work".

| ID | Required evidence |
|---|---|
| DI-E01 | One fake clock/store/carrier selection reaches every relevant caller; real-provider construction and hidden I/O are rejected in the fast composition |
| DI-E02 | Equal scoped binding → shared identity, independent test scope → isolation, also during concurrent first access |
| DI-E03 | Missing/ambiguous role and construction cycle rejected before useful startup; no late service-locator surprises |
| DI-E04 | The selected real async Glade port remains usable without framework imports in its contract/pure libraries |

DI-E05, DI-E06 and DI-E07 (`DependencyInjectionEvaluation.md:137-139`) are **not** part
of this witness. Section 10 says so and says why the overlap with DI-E05 is handled.

### 2.2 From `GladeBuyBuildMatrix.md:133-138`

> The **next witness** (DI-E01..E04) must show one injected provider scope on every
> caller path, a rejected real-I/O bypass, one coordinated fake per test composition,
> and the selected real async port usable without framework imports in contracts.
> A failed witness reopens the selection. Under GDL-049 the lifecycle side of that
> witness runs on sdax-rs: Shaku builds the components, sdax-rs owns their
> acquisition, run and reverse-order release.

Row R27 (`GladeBuyBuildMatrix.md:112`) records the decision and its build residue:
"Assembly-only use; six binding recipes; the still-open async-port/startup/cleanup
witness (DI-E01..E04)". Row R28 (`:113`) names the lifecycle residue: "Plan authoring
for the node's resources and services, the AR-08 witness on sdax-rs, the async bridge
to Shaku-built components". Row Q10 (`:157`) sets the lean: "Run the AR-08 and
DI-E01..E04 witness on sdax-rs plus Shaku; keep Tokio primitives as the fallback if
it fails."

### 2.3 From `DecisionLog.md`

GDL-048 (`:88-94`) selects Shaku "for assembly, not a production Cargo installation,
public-trait change, completed async bridge or automatic lifecycle guarantee".
GDL-049 (`:73-86`) directs sdax-rs, notes "Glade can consume it as a Git dependency
now, from crates.io only after publication", and records that "this records a
direction, not a completed integration or a lifecycle conformance result".

### 2.4 AR-08, from `arch1/RuntimeAndAssurance.md:126`

> AR-08 | Parent resource remains owned while child cleanup is pending; partial
> shutdown retries without double-release. Concurrent independent cleanup can
> progress; no invisible detached work.

The surrounding contract (`RuntimeAndAssurance.md:90-96`) is the fuller statement:
every externally running activity belongs to a scope before it can outlive its
caller; shutdown stops admission, drains or cooperatively cancels children, then
releases the resources they use; parents outlive dependents; deadline expiry returns
unfinished ownership; dropping a polled future means neither rollback nor completed
cleanup.

---

## 3. Facts established from the code

Everything in this section was read, not inferred. Paths are absolute.

### 3.1 The node today names neither library

`/Users/owebeeone/limbo/glade-wz/glade/node/Cargo.toml:14-24` — the node's entire
dependency list is `glade-wire` (path), `sha2 0.10`, `tokio 1` and `iroh 1.2`.
No `shaku`, no `sdax`. The only Shaku in the tree is the evaluation harness:
`/Users/owebeeone/limbo/glade-wz/glade/dev-docs/di-eval/Cargo.toml:19`,
`shaku = { version = "=0.6.3", optional = true }`. `sdax` appears nowhere in `glade/`.

The node's lockfile is untracked (`glade/node/.gitignore:2`, `/Cargo.lock`), which is
why `VersionPinRuling` put the iroh floor in the manifest.

### 3.2 Which crates are the "contract/pure libraries" DI-E04 means

Eight crates exist today with no framework dependency. These are the wall DI-E04
protects.

| Crate | Manifest | Dependencies |
|---|---|---|
| `glade-lifecycle-api` | `glade/contracts/lifecycle-api/Cargo.toml` | none |
| `glade-persistence-api` | `glade/contracts/persistence-api/Cargo.toml` | none |
| `glade-invocation-api` | `glade/contracts/invocation-api/Cargo.toml` | none |
| `glade-subscription-api` | `glade/contracts/subscription-api/Cargo.toml` | none |
| `glade-sync-api` | `glade/contracts/sync-api/Cargo.toml` | none |
| `glade-binding-api` | `glade/contracts/binding-api/Cargo.toml:11-12` | `glade-decl` only |
| `glade-decl` | `glade-decl-rs/Cargo.toml:10-11` | none ("A leaf") |
| `glade-wire` | `glade/wire-rs/Cargo.toml:10-12` | none ("No dependencies: the codec is self-contained") |

`glade-client` (`glade/client-rs/Cargo.toml:11-13`) is **not** in this set: it depends
on `tokio`. It is the supplier SDK of `Components.md:60`, an implementation, not a
pure library. `glade-node` is likewise an integration crate.

Six of the eight are already under a mechanical gate: `glade/contracts/architecture-policy.json`
classifies them with explicit dependency allowlists, and `glade/contracts/check.sh:25-31`
runs the checker over them. `glade-wire` and `glade-decl` are **not** under any gate today.
That is a real gap and Phase 1 closes it for the witness's purposes.

`arch1/Components.md:21-33` and `arch1/InjectionGraphRefinement.md:12-19` name a larger
set of *proposed* packages (`glade-binding`, `glade-policy`, `glade-runtime` …) and six
binding recipes over ports (`ClockPort`, `CarrierPort`, `TransportPort`, `RecordHostPort`,
`RecordProfilePort`). **None of those ports exists as a Rust trait.** They are architecture
declarations. The witness must therefore declare the port it uses; it cannot import one.

### 3.3 Which real async ports exist

| Candidate | Where | Async? | Verdict |
|---|---|---|---|
| iroh peer carrier | `glade/node/src/iroh_carrier.rs` | yes — `bind`, `dial`, `accept` | **selected** |
| WebSocket carrier | `glade/node/src/ws.rs` | yes — `accept`, `connect`, `read`, `send_binary` | runner-up |
| Store | `glade/node/src/store.rs` | **no** — fully synchronous | unusable |
| Clock | — | **no trait exists** | unusable |

**The store is not a port.** `Store::open` (`store.rs:98`), `append` (`:136`) and `scan`
(`:234`) are synchronous blocking file I/O; `grep "async fn" node/src/store.rs` returns
nothing. It is held behind a `tokio::sync::Mutex` and locked from async code
(`server.rs:83`, `shared.store.lock().await`), so blocking file I/O already runs on the
executor. That is a finding for AR-08's bounded-executor clause, not a witness target.

**There is no clock port.** `now_ms()` is a free function reading the wall clock
directly: `glade/node/src/sysdir.rs:206-211`. It is called at write sites only
(`claims.rs:163`, `claims.rs:250`, `sysdir.rs:177`). The read path is already correct
by construction — `RegistryApi::who_serves` (`glade/node/src/registry.rs:199`, in the
`RegistryApi` trait at `:191`) takes `now_ms: i64` as a **parameter**, and the doc
comment at `:197-198` says lease expiry "is evaluated at read time, never inside the
fold; highest live epoch wins". So DI-E01's "fake clock" has
nothing to substitute in the existing code; the witness must declare a clock port of
its own, and it is the *write* sites that would eventually consume it.

#### The selected real async Glade port: the iroh peer carrier

`glade/node/src/iroh_carrier.rs`, type `PeerEndpoint` (`:67`). Four reasons:

1. **Its acquisition is genuinely async and fallible.** `PeerEndpoint::bind()` (`:75`)
   and `bind_with()` (`:86`) await a real UDP socket bind through `bind_endpoint()`
   (`:32-40`). `dial()` (`:108`) awaits a QUIC connect plus a HELLO handshake.
   Nothing else in the tree has a start-up that can fail this way.
2. **It already carries AR-08's exact clause in a comment.** `iroh_carrier.rs:63-65`:
   the endpoint "**MUST outlive every `PeerLink` it produces** — dropping the last
   handle closes the endpoint and tears down live connections." That is "parent
   resource remains owned while child cleanup is pending", written by hand as a
   footgun warning. `mesh.rs:125-126` calls it "the R2 footgun".
3. **It has a real, bounded, drain-shaped teardown.** `iroh::Endpoint::close()` is
   `pub async fn` and drains connections
   (`~/.cargo/registry/src/index.crates.io-*/iroh-1.2.0/src/endpoint.rs:1719`), with a
   documented worst case of about three seconds and the warning that not waiting makes
   remote ends see failures. `is_closed()` (`:1724`) maps onto a phase. Crucially,
   `endpoint.rs:1717-1718`: "the underlying UDP sockets are only closed once all clones
   of the respective `Endpoint` are dropped." `PeerEndpoint` is `#[derive(Clone)]`
   (`iroh_carrier.rs:66`) over that Arc-backed handle. **An escaped clone is directly
   observable as a socket that is still bound**, which turns the evaluation's abstract
   "a service can outlive container drop via `Arc`" into a falsifiable test.
4. **It is the port the binding graph gives two roles.** `InjectionGraphRefinement.md:15,17`
   gives `peer_carrier_binding` and `record_transport_binding` the *same* IrohAdapter
   occurrence, and `:23` says the two "MUST share their provider within the same node
   scope". DI-E01's "reaches every relevant caller" has a real target here.

**Runner-up: the WebSocket carrier** (`glade/node/src/ws.rs:94` `accept`, `:121` `connect`).
It needs only tokio, builds in seconds, and carries the same `Frame` bytes, so it would
make a cheaper witness; but its acquisition is not itself async-fallible in the same
way — the `TcpListener` is bound by the caller — so it would witness less of start-up.
It stays the fallback if iroh's build cost or resolution makes Phase 4 impractical.

### 3.4 What the node does about shutdown today: nothing

There is **no** `impl ManagedResource` anywhere outside the contract's own test fixture
(`glade/contracts/lifecycle-api/tests/public_contract.rs:11`). The node has no orderly
shutdown path at all:

- `Server::run` (`server.rs:95-103`) loops forever and `tokio::spawn`s each connection
  handler, detached.
- Per-session teardown ends with `wtask.abort()` (`server.rs:273`) — queued outbound
  bytes are dropped, not drained, and nothing is reported.
- Enabling the mesh detaches an accept loop (`mesh.rs:124`) which detaches a task per
  link (`mesh.rs:131`); `run_link` (`mesh.rs:158`) detaches three more (`:166`, `:175`,
  `:178`) and one on the acceptor path (`:193`).

That is five nested untracked spawns per peer link, none cancellable, none drained.
It is a precise statement of the distance to AR-08, and it is why the witness is a
separate crate rather than a change to the node.

### 3.5 Shaku, exactly as the evaluation assumed it

Version `=0.6.3` (`di-eval/Cargo.toml:19`, `di-eval/Cargo.lock:124-125`), with
`shaku_derive 0.6.3` and `anymap2 0.13.0`. Read from the vendored source at
`~/.cargo/registry/src/index.crates.io-*/shaku-0.6.3/`:

- **It is synchronous, with no async surface whatsoever.** `grep -rn "async fn\|Future" src/`
  returns **zero matches**. `ModuleBuilder::build(self) -> M` (`src/module/module_builder.rs:83`)
  is a plain function. `with_component_override` (`:48`) takes an **already-constructed**
  `Box<I>`; `ComponentFn` (`src/component.rs:70`) is
  `Box<dyn (FnOnce(&mut ModuleBuildContext<M>) -> Box<I>) + Send + Sync>` — a synchronous
  closure. **Shaku cannot acquire an async resource.** Acquisition must happen first and
  be handed to the builder as an override.
- `rust-version = "1.88"` (`Cargo.toml:14`). The installed toolchain is 1.96.0, matching
  the evaluation's tested toolchain.
- **`#[lazy]` first access is atomic.** With the default `thread_safe` feature, lazy
  components use `std::sync::OnceLock` (`src/lib.rs:41-50`). This is exactly what Dill
  lacked, so DI-E02's "also during concurrent first access" is witnessable rather than
  hopeless.

#### The bridge the evaluation meant

`DependencyInjectionEvaluation.md:95-97` says runtime-polymorphic injection "needs a
reviewed object-safe bridge or generic composition" but does not name one. **It already
exists in the probes.** `di-eval/tests/shaku.rs:9-14`:

```rust
// Assembly-only facades: foreign dyn port traits do not implement Shaku's
// Interface, even with Any + Send + Sync. Keep that dependency out of ports.
trait Clock: ClockPort + shaku::Interface {}
impl<T: ClockPort> Clock for T {}
```

An assembly-local facade trait with the framework bound as a supertrait, a blanket impl
over every concrete port implementation, and stable trait upcasting back to the port at
the use site (`di-eval/tests/shaku.rs:44-45`, `Arc<dyn Clock>` → `Arc<dyn ClockPort>`).
The port crate never names Shaku.

**Why the facade is unavoidable, from the source.** `shaku-0.6.3/src/component.rs:47-55`
declares `Interface` through `trait_alias!`, which expands (`src/trait_alias.rs:5-9`) to
`trait Interface: Any + Send + Sync {}` plus `impl<T: Any + Send + Sync> Interface for T {}`.
There is **no `?Sized`** on that blanket impl, so a foreign trait object such as
`dyn ClockPort` can never acquire `Interface` no matter what its supertraits are. That is
the E0277 in `di-eval/examples/foreign_port_shaku.rs`, and it is structural, not a
version accident.

#### The hard limit the bridge cannot cross

`ManagedResource` (`glade/contracts/lifecycle-api/src/lib.rs:57-60`):

```rust
pub trait ManagedResource: Send {
    fn phase(&self) -> Phase;
    fn shutdown(&mut self, request: Shutdown) -> impl Future<Output = ShutdownReport> + Send;
}
```

Three facts collide:

1. It is **not dyn-compatible** — `impl Future` in return position. The compiler confirmed
   E0038 and the probe is retained: `di-eval/examples/native_dyn.rs:3`.
2. Shaku's `Interface` requires `Send + Sync`; `ManagedResource` is `Send` only and takes
   `&mut self`.
3. The evaluation forbids the obvious workaround (`DependencyInjectionEvaluation.md:96-97`):
   "Do not add `Sync`, locks or boxed public futures merely to please a container."

**Therefore a `ManagedResource` can never be a Shaku component interface.** This is not a
defect to work around; it is the shape of the answer. Shaku injects the shared, `Sync`
service *views*; sdax-rs separately owns the `&mut`, `Send`-only resource *handles*.
The witness's whole job is to show those two can meet at one object without either
side corrupting the other.

The existing conformance suite is already generic and `&mut`-based
(`glade/contracts/lifecycle-api/src/conformance.rs:9,18,34,46,59`), driven by a noop
waker with no runtime, covering LC-001..LC-006 plus a `rejects_false_cleanup_success`
negative (`tests/public_contract.rs:70-107`). A concrete adapter can run it unchanged.

### 3.6 sdax-rs as it really is

Both checkouts were read. **The library source is byte-identical between them.**

#### Which line to pin

| | `/Users/owebeeone/limbo/sdax-wz-authoring/sdax-rs` | `/Users/owebeeone/limbo/sdax-wz-diagnostics/sdax-rs` |
|---|---|---|
| `HEAD` | `7ec4b67` "Complete component binding and lifecycle guidance" | `4599e3d` "test: cover complete report diagnostics at string boundary" |
| branch | `main` | `main` |
| `origin/main` | **`ce339a5`** | **`ce339a5`** |
| ahead by | 2 commits | 1 commit |
| touches `*/src/` | **no** — one test file + two docs | **no** — one test file + one doc |

They are independent clones that **diverged at `ce339a5`**; neither object store contains
the other's HEAD. There is no mainline tip among them: they are two sibling lanes both
called `main`.

**Pin `rev = "ce339a59eefcd136562b16f4f20035c29ef0c988"`.** It is `origin/main` in both
clones, it is the only one of the three commits reachable from the remote, and because
both lanes changed only tests and documents, the library you compile is identical at all
three. Nothing is lost. The authoring lane (`7ec4b67`) is the line to follow *afterwards*,
because it owns `docs/AI-Authoring.md`, the reference an implementer will read.

Two cautions. `origin/main` is as the local remote-tracking refs record it — confirm with
`git ls-remote` before writing the pin, since no network command was run. And the
`code-complete` tag resolves to `8147239`, six commits behind `ce339a5`; do not pin it,
and do not move it.

#### Crates, versions and dependencies

`sdax-rs/Cargo.toml:1-14` — three members, `version 0.1.0`, `edition 2021`,
`rust-version 1.75`, `unsafe_code = "forbid"`. There are **no feature flags anywhere** in
the workspace.

- **`sdax`** — `crates/sdax/Cargo.toml` is 20 lines with **no `[dependencies]` section at
  all**. Confirmed three ways: the manifest, the lockfile entry (`Cargo.lock:77-79`, no
  `dependencies` key), and every `use` in `crates/sdax/src/` resolving to `std`/`crate`/
  `super`. Stronger than "std-only": it is zero-dependency, with no `futures` crate.
- **`sdax-tokio`** — `crates/sdax-tokio/Cargo.toml:28-29` pins
  `tokio = "=1.53.1"` and `tokio-util = "=0.7.19"` **exactly**, with
  `default-features = false, features = ["rt","sync","time"]`.
- **`sdax-testkit`** — `crates/sdax-testkit/Cargo.toml:12` sets `publish = false`. It can
  come from Git but never from crates.io. If the witness wants `FakeClock`, it is a
  permanent Git dev-dependency.

Its `Cargo.lock` **is** tracked.

#### What the public API offers, against AR-08's list

| AR-08 clause | sdax-rs surface | Where |
|---|---|---|
| owned work | `.resource()`, `.step()`, `.service()`, `.effect()`, `.component()`, `.template()` on `PlanBuilder` | `crates/sdax/src/builder.rs:242-331` |
| ownership handover | `Held<T>`, mintable only by `Cx::hold` / `Cx::hold_value`; registration is **poll-atomic** — the `Arc` is registered in the same poll that observes the effect completing | `crates/sdax/src/cx/acquisition.rs:46-52`, `:104-110` |
| admission | engine state; the author-visible refusal is `SpawnError::ScopeStopping`, "the scope is stopping and admits no new instances" | `crates/sdax/src/host/engine/admit.rs:12-21`; `crates/sdax/src/cx/instances.rs:117` |
| drain | `impl Future for Running<Out>` — awaiting it *is* the drain; `Running::ready()` waits for steady state instead | `crates/sdax-tokio/src/running.rs:330-331`, `:278` |
| drain (runtime) | `TokioRuntime::shutdown(budget) -> Result<(), usize>`; `Err(n)` = budget expired with `n` tasks live | `crates/sdax-tokio/src/lib.rs:216-222` |
| no detached work | every spawn registered in a `TaskTracker`; `TokioRuntime::tracked() -> usize` | `crates/sdax-tokio/src/lib.rs:77`, `:197` |
| cancellation | `Running::cancel()` — "the release graph still runs"; `cx.stop()`, `cx.is_stopping()`, `cx.until_stop()`, `cx.timeout()` | `crates/sdax-tokio/src/running.rs:262`; `crates/sdax/src/cx.rs:341-373` |
| cancellation mode | `CancelMode::Abrupt` vs cooperative with a grace, per node via `.cooperative(grace)` | `crates/sdax/src/policy.rs:196-202`; `crates/sdax/src/builder.rs:507` |
| reverse-order teardown | **derived, not declared** — the reverse of the `needs` DAG, transitively closed | `crates/sdax/src/view.rs:120-148` |
| partial order, asserted statically | `ReleaseOrder::before(a,b)`, `unordered(a,b)`; reached via `Plan::inspect()`, which runs no bodies | `crates/sdax/src/view.rs:477`, `:485`, `:535` |
| truthful reports | `Report { outcome, output, faults, cleanup_failures, incomplete, ambiguous, trace }` | `crates/sdax/src/report.rs:320-338` |
| honest "not finished" | `Report::incomplete` — "obligations the shutdown budget abandoned"; `is_clean()` requires **every** list empty, because "`Outcome::Ok` alone is not enough" | `crates/sdax/src/report.rs:333`, `:354-362` |
| deterministic time | `FakeClock` with `advance()`, plus `ScriptedDriver`, an independent invariant checker and a Monte Carlo generator | `sdax-testkit/src/clock.rs:8-40`, `src/driver.rs:22`, `src/invariants.rs:68`, `src/mc/` |

All trait futures are boxed on purpose: `crates/sdax/src/contracts.rs:9-10` — "Futures
are boxed in every trait signature: the crate's MSRV predates `async fn` in traits, and
these traits must stay usable as `dyn`." `Clock` (`:72-78`) and `Observer` (`:137-151`)
are dyn-compatible and used as `dyn`; `Runtime` (`:109-124`) has an associated type and
is always a generic bound.

**The teardown order is derived from typed `needs` edges, not from registration order and
not from an explicit "drains before" edge.** `InjectionGraphRefinement.md:41-45` already
speaks in partial-order terms ("Records drains before storage/peer-carrier release …
These are **partial ordering constraints**, not a total serial shutdown algorithm"), and
`ReleaseOrder::unordered_pairs()` is the direct expression of that. The two models agree.

#### What is NOT there

- **No DI integration of any kind.** `grep -rni 'shaku'` and `grep -rni 'ManagedResource'`
  over the whole repository return **zero matches**. There is no bridge, no adapter, and
  nothing planned. The bridge is entirely Glade-side work.
- Stage 3 is the **last stage the project defines** (`AGENTS.md:153-162`, all four rows
  marked done). There is no Stage 4.
- No `todo!`, `unimplemented!`, `TODO` or `FIXME` anywhere in `crates/`.
- Open items the project itself lists (`dev-docs/QueuedWork.md:14-20`): the fast-loop
  budget is ~4.8 s against a stated ~0.55 s; exhaustive schedule enumeration `S-02` has
  never run; `panic = "abort"` behaviour is stated but unexecuted; three substrate probes
  are deferred.
- One honest limit that matters here (`sdax-testkit/src/invariants.rs:27-32`): the release-
  order invariants INV-5 and INV-6 have no negative fixture, because the core derives the
  order *from* the edge set, so "the two computations cannot disagree unless the core's
  derivation regresses". They are regression guards, not independent falsification.
- **Unpublished, SSH-only, and anonymous access is explicitly unverified.**
  `README.md:19` — "The crates are not yet published on crates.io";
  `dev-docs/ReleaseReadiness-2026-09-08.md:106` — "No release tag, publication or
  repository visibility change has been made"; `:97-99` requires making the repository
  public "if anonymous Git installation is intended", and `RELEASE.md:70-72` says the
  existing consumer test "does not establish anonymous access to the actual GitHub
  repository".

#### Version compatibility with the selected port — checked

`sdax-tokio` pins `tokio =1.53.1` and `tokio-util =0.7.19`. `iroh 1.2.0` requires
`tokio ^1.44.1` and `tokio-util ^0.7`
(`~/.cargo/registry/src/index.crates.io-*/iroh-1.2.0/Cargo.toml:348-362`). **They unify**,
and `tokio 1.53.1` is already in the local registry cache. `glade-node` declares
`tokio = "1"`, which accepts it. Feature unification means iroh's larger feature set wins
over sdax-tokio's deliberately minimal one — a build-cost effect, not a correctness one.

Two toolchain floors follow: `iroh 1.2.0` declares `rust-version = "1.91"` and
`edition = "2024"`; `shaku 0.6.3` declares `1.88`. The witness workspace's effective MSRV
is **1.91**. Installed: 1.96.0.

---

## 4. The design of the witness

### 4.1 Where it lives

`/Users/owebeeone/limbo/glade-wz/glade/dev-docs/async-witness/`, a **separate Cargo
workspace** with its own tracked `Cargo.lock`, following the precedent of
`glade/dev-docs/di-eval/` exactly. It is never referenced from `glade/node/Cargo.toml`.
Dependency flows one way only: the witness path-depends on `glade-node`; the node gains
nothing. Nothing is wired into the node binary and the demo is untouched.

Three members, because the fast path must not drag the node in — `LibraryBoundaryAndTestingPolicy.md:36`
(LBT-005): "Test helpers MUST NOT pull the entire node/application into a library's fast
test path."

| Member | Role | Dependencies | Purpose |
|---|---|---|---|
| `async-witness-ports` | contract | `glade-wire` only | `ClockPort`, `CarrierPort` and deterministic fakes. **Zero framework dependencies — this crate is the DI-E04 wall.** |
| `async-witness-fast` | harness | ports, `shaku =0.6.3` | DI-E01, E02, E03. No tokio, no iroh, no sdax. Milliseconds. |
| `async-witness-real` | harness | ports, shaku, `sdax`, `sdax-tokio`, `glade-node`, tokio | DI-E04 and AR-08 against the real endpoint. Slow, run separately. |

`async-witness-ports` names `glade-wire` because the carrier port carries `Frame` bytes,
and `glade-wire` is itself zero-dependency. That is the point: the port is expressible
without importing anything framework-shaped.

### 4.2 How Shaku assembly and sdax-rs lifecycle meet

They meet in one order, and the order is forced by Section 3.5's collision:

```
sdax-rs acquires  →  Shaku assembles over the acquired handles  →  sdax-rs releases, in reverse
```

- **sdax-rs acquires first.** An sdax `resource` node awaits `PeerEndpoint::bind_with()`
  inside the `cx.hold(...)` factory, so the engine owns the cleanup in the same poll that
  the bind completes. Shaku cannot do this: `build()` is synchronous and
  `with_component_override` takes an already-constructed value.
- **Shaku assembles second, from inside the sdax step that needs the handle.** A `step`
  node `.needs(endpoint_key)` receives `Arc<PeerEndpoint>` and calls
  `Module::builder().with_component_override::<dyn Carrier>(...).build()`. The module is
  a plain value produced by a step; Shaku never learns about lifecycles.
- **sdax-rs releases in reverse**, derived from the `needs` edges: the module step
  depends on the endpoint resource, so the module is torn down before
  `Endpoint::close()` is awaited. That is AR-08's parent/child clause, obtained by
  construction rather than by discipline.

The sharp risk lives at this seam. The Shaku module holds `Arc` clones of things derived
from the endpoint, and an escaped clone keeps the UDP socket bound
(`iroh-1.2.0/src/endpoint.rs:1717-1718`). The witness's central test is therefore a
**differential**: run the identical sdax plan twice, once with the Shaku module step and
once without. If the report is `is_clean()` and the port is free without Shaku but not
with it, the bridge leaks and that is a Shaku result, not an sdax one.

### 4.3 The bridge

Exactly the mechanism already proven in `di-eval/tests/shaku.rs:9-14`, now applied to a
port the witness declares rather than a synthetic one, and living **only** in
`async-witness-fast` / `async-witness-real`:

- a facade trait in the assembly crate, `trait Carrier: CarrierPort + shaku::Interface {}`;
- a blanket impl over concrete implementations, `impl<T: CarrierPort> Carrier for T {}`;
- trait upcasting at the use site to hand callers back `Arc<dyn CarrierPort>`.

`ManagedResource` is **not** bridged. Nothing is made `Sync`, no public future is boxed,
and no Glade contract changes. The `ManagedResource` adapter is a concrete type owned by
`async-witness-real` and runs the existing conformance suite by generic instantiation.

The two roles of `InjectionGraphRefinement.md:14,17` use Shaku's keyed multibinding, as
probed at `di-eval/tests/shaku.rs:176-212`: one `Role` key type, two keyed registrations,
one shared occurrence, and a missing key is an absent map entry rather than a panic.

### 4.4 The fakes

Deterministic, dependency-free, in `async-witness-ports`, mirroring
`di-eval/ports/src/lib.rs:19-36`: a `FakeClock` over an atomic, a fake store, and a fake
carrier that records frames. No sleeps, no sockets, no files — LBT-008
(`LibraryBoundaryAndTestingPolicy.md:39`).

**There are two clocks and the witness must not conflate them.** sdax-rs has its own
injected `Clock` (`sdax/src/contracts.rs:72-78`) on which all backoffs, `within`
deadlines and the shutdown budget are measured, and `sdax-testkit` supplies a `FakeClock`
for it. Glade's clock port is a separate, Glade-owned thing. Phase 3 wires
`sdax_testkit::FakeClock` to the engine and the witness's own fake to the port, and
asserts they agree where a test advances both. Using sdax's `Clock` as Glade's clock port
would put a framework type in a contract crate and fail DI-E04 by construction.

**Hidden I/O is proved absent mechanically, not by inspection.** `async-witness-fast`
does not depend on tokio, iroh or `std::net` at all, so a provider that opens a socket
cannot compile into that target. Real-provider construction is counted with an atomic,
as at `di-eval/tests/shaku.rs:90-105`, and asserted to be zero.

### 4.5 How "no framework imports in the contract crates" is checked mechanically

By the existing fast source/dependency lint, not by reading.
`/Users/owebeeone/limbo/glade-wz/glade-discover/tools/architecture-check/` is a `syn`-based
checker (`src/lib.rs:1-2`: "Fast source/dependency architecture lint") that shells
`cargo metadata --format-version 1 --no-deps --locked --offline` (`src/main.rs:6-14`) and
compares declared dependencies against a per-package allowlist in
`architecture-policy.json`. It compiles nothing.

Three properties make it the right instrument here:

1. **It is an allowlist, not a denylist.** `ARCH-002 … undeclared dependency`
   (`src/lib.rs:299`) fires on anything not listed. `shaku` or `sdax` appearing in
   `async-witness-ports` is an error without anyone having thought to forbid it.
2. **It sees dependency kinds a reviewer forgets.** `src/lib.rs:255-256`: "This checks
   declarations, **including inactive optional/target/build/dev dependencies**." The key
   is `kind:name`, so `dev:shaku` and a target-specific entry are both caught. This is
   LBT-005 (`LibraryBoundaryAndTestingPolicy.md:36`) enforced by a program.
3. **It is manifest-level, so no `#[cfg]` can bypass it.** A `use shaku::…` behind any
   conditional still needs the manifest entry. This satisfies the standing rule that
   checks must not be defeatable by a disabled branch.

Two gaps to close rather than pretend away:

- `--no-deps` means **transitive** framework reachability is invisible. A contract crate
  depending on a permitted pure crate that itself depends on `shaku` would pass. The
  witness adds a second, equally cheap assertion: `cargo tree --invert shaku` and
  `--invert sdax` from the witness workspace must list no contract-role package.
- The scanner **skips items under `#[cfg]`/`#[cfg_attr]`** (`src/lib.rs:97-102`,
  `fn conditional`), so `ARCH-003`'s trait-method checks can be silently weakened by a
  conditional attribute. That does not affect the dependency check, but it is a real
  limitation of the trait half and Step 1.3 files a negative fixture that demonstrates it
  rather than leaving it undocumented.

`glade-wire` and `glade-decl` are brought under a policy file for the first time, because
they are the pure crates the port's types come from.

---

## 5. Dependency posture

Nothing is added to `glade/node/Cargo.toml`. Every dependency below lives in the witness
workspace alone.

```toml
# glade/dev-docs/async-witness/real/Cargo.toml — illustrative, not yet written
[dependencies]
sdax       = { git = "ssh://git@github.com/owebeeone/sdax-rs", package = "sdax",       rev = "ce339a59eefcd136562b16f4f20035c29ef0c988" }
sdax-tokio = { git = "ssh://git@github.com/owebeeone/sdax-rs", package = "sdax-tokio", rev = "ce339a59eefcd136562b16f4f20035c29ef0c988" }
shaku      = "=0.6.3"
glade-node = { path = "../../../node" }

[dev-dependencies]
sdax-testkit = { git = "ssh://git@github.com/owebeeone/sdax-rs", package = "sdax-testkit", rev = "ce339a59eefcd136562b16f4f20035c29ef0c988" }
```

- **sdax-rs from Git at an explicit `rev`**, as `LifecycleCompositionRuling` requires.
  All three crates pin the **same** rev; a mismatch is a failure case
  (`sdax-rs/RELEASE.md:70`).
- **Shaku pinned exactly** at `=0.6.3`, the evaluation's measured baseline, matching
  `di-eval/Cargo.toml:19`. A caret range would silently retest a different library.
- **The witness workspace tracks its own `Cargo.lock`.** The ruling's stated reason —
  "lockfiles are not tracked in these repositories" — is true of the **node**
  (`glade/node/.gitignore:2`) but not of the evaluation workspaces:
  `glade/contracts/Cargo.lock` and `glade/dev-docs/di-eval/Cargo.lock` are both tracked,
  and di-eval added an explicit `!Cargo.lock` to its `.gitignore` to make sure of it.
  This **strengthens** the ruling rather than contradicting it: the `rev` remains the only
  pin a fresh clone resolves from and is still required, and the tracked lockfile pins the
  exact transitive set on top. `check.sh` then runs `--locked --offline`, as the existing
  gates do.

**A blocking prerequisite the ruling could not have known.** `ce339a5` is the only commit
reachable from the remote; `7ec4b67` and `4599e3d` are unpushed local lane tips. And the
remote is SSH-only with anonymous access explicitly unverified
(`sdax-rs/dev-docs/ReleaseReadiness-2026-09-08.md:97-99`). Before Phase 0 can finish, the
owner must confirm the rev on the remote and decide the access story. This is an owner
action; the plan does not assume it.

**Update, 2026-09-21, after the owner pushed.** `origin/main` is now
`ccf06e76a90e22a454471a71f0cf6f5cb878baac`, and it contains both lane tips, `7ec4b67` and
`4599e3d`. **Pin `ccf06e7`, not `ce339a5`**, in all three entries above. Against the
reading this plan was written from (`7ec4b67`) the library differs by 109 added lines, all
in `crates/sdax/src` (`required_output.rs` is new, `recovery.rs` grew, `lib.rs` gained
three lines); `sdax-tokio` and `sdax-testkit` are unchanged, so §3.6 stands. Step 2.2
should check whether the new required-output rule changes what `report.is_clean()` means.
The owner has also decided the repository can be public. As of this note it is still
private (an anonymous `git ls-remote` is refused), so Step 0.0 is half done: the rev is
confirmed, and the access story is decided but not yet in effect. Until the visibility
changes the URL stays the SSH form above; once it is public, use
`https://github.com/owebeeone/sdax-rs` so that a fresh clone needs no credentials.

---

## 6. The phased plan

Phases are milestones: each is a coherent increment that stands on its own and leaves the
tree green. Steps are one goal each, budgeted to an **aspirational < 500 LOC** (a target,
not a limit). Steps are written so different agents can take different ones; coupling is
named where it exists.

**The standard exit check, referenced below as `GATE`.** Unless a step says otherwise:
`cargo test --locked --offline` for the touched package, then the witness workspace's
`check.sh` (architecture gate, `cargo fmt --check`, `cargo clippy -- -D warnings`), and
the six existing contract packages still pass `sh glade/contracts/check.sh`.

### Phase 0 — the witness workspace exists and resolves (foundational; gates everything)

Goal: an empty but complete, gated, reproducible workspace. No witness logic yet.

| Step | Goal | Touches | Test / observable result | Depends on |
|---|---|---|---|---|
| 0.0 | **Owner action, not an agent step.** Confirm `ce339a5` is on `github.com/owebeeone/sdax-rs`; decide SSH credentials or public visibility. *(2026-09-21: the rev to pin is now `ccf06e7` and the owner chose public visibility, not yet in effect; see the update in §5.)* | nothing in these repos | `git ls-remote` shows the rev; a scratch crate outside all workzones resolves the three Git deps | — |
| 0.1 | Create the workspace skeleton: root `Cargo.toml` with three members, `.gitignore` with `/target/` and `!Cargo.lock`, a README stating harness-not-production | new `glade/dev-docs/async-witness/` | `cargo metadata --no-deps --locked --offline` succeeds; `Cargo.lock` is tracked | — |
| 0.2 | `async-witness-ports`: `ClockPort`, `CarrierPort`, `FakeClock`, fake carrier. Zero framework dependencies | ports crate | Compiles with `glade-wire` as its only dependency; `cargo tree -p async-witness-ports` shows one edge | 0.1 |
| 0.3 | Adopt the architecture gate: `architecture-policy.json` classifying the three members plus `glade-wire` and `glade-decl`, and a `check.sh` copied from `glade/contracts/check.sh` | policy + script | Gate passes; a scratch edit adding `shaku` to the ports manifest makes it fail with `ARCH-002` | 0.1 |
| 0.4 | Resolve the pinned dependencies: add sdax, sdax-tokio, shaku and `glade-node` to the two harness crates with no code using them yet | two harness manifests | `cargo metadata --locked` resolves; `tokio` unifies to `1.53.1`; the lockfile is committed | 0.1, 0.0 |

0.2, 0.3 and 0.4 are independent of each other once 0.1 exists. 0.4 stops at 0.0.

### Phase 1 — the fast composition: DI-E01, DI-E02, DI-E03

Goal: the three criteria that need no async port, no runtime and no sdax, decided on
Glade's own contract types rather than synthetic ones. Milliseconds. Shaku alone.

| Step | Goal | Touches | Test / observable result | Depends on |
|---|---|---|---|---|
| 1.1 | The bridge over the witness's real ports: facade traits, blanket impls, upcasting at the use site | `async-witness-fast` | The module resolves `Arc<dyn Carrier>` and hands back `Arc<dyn CarrierPort>`; `async-witness-ports` still has one dependency | 0.2, 0.4 |
| 1.2 | **DI-E01**: one fake selection reaches every declared consumer; real-provider construction counted and zero; no I/O reachable | `async-witness-fast` tests | `Arc::ptr_eq` holds across every consumer; the construction counter is 0; the crate has no tokio/iroh/net dependency at all | 1.1 |
| 1.3 | **DI-E03**: a missing binding, a construction cycle and an unkeyed ambiguous role each fail to compile; no string-keyed resolution exists. Plus the negative fixture for the `#[cfg]` scanner gap of §4.5 | `async-witness-fast` examples, README table | Three `cargo check` examples fail with E0277, E0275, E0277; diagnostics inspected, not just exit codes | 1.1 |
| 1.4 | **DI-E02**: shared identity within a scope, isolation across scopes, and one construction under concurrent first access of a `#[lazy]` component | `async-witness-fast` tests | `ptr_eq` true within, false across; two threads racing first resolve observe exactly one construction | 1.1 |
| 1.5 | Keyed roles: peer and client select distinct providers while the two iroh-facing recipes share one occurrence | `async-witness-fast` tests | `resolve_map()` gives two roles; the shared occurrence is `ptr_eq` across both | 1.1 |

1.2, 1.3, 1.4 and 1.5 are **four parallel steps** once 1.1 exists. They share only the
module definition and touch separate test files.

### Phase 2 — sdax-rs owns a fake resource: AR-08 without the real port

Goal: prove the lifecycle library does what AR-08 needs, with fakes, before any socket is
involved. Pure sdax. **Independent of Phase 1** — a different agent can run this in
parallel.

| Step | Goal | Touches | Test / observable result | Depends on |
|---|---|---|---|---|
| 2.1 | A parent/child resource plan over fakes; assert reverse order **statically**, with no runtime | `async-witness-real` tests | `Plan::inspect().release_order().before(child, parent)` is true; two independent branches are `unordered` | 0.4 |
| 2.2 | Run it: drain to completion, then cancel, then expire the budget | `async-witness-real` tests | `report.is_clean()` on the clean run; the cancelled run still runs the release graph; the expired run lists the resource in `report.incomplete` | 2.1 |
| 2.3 | The `ManagedResource` adapter: a concrete type wrapping one sdax run, mapping `Phase`/`Mode`/`Shutdown`/`ShutdownReport` onto `Outcome`/`incomplete`/`cleanup_failures` | `async-witness-real` | The existing LC-001..LC-006 suite from `glade-lifecycle-api` passes against it unchanged, and `rejects_false_cleanup_success` still panics | 2.2 |
| 2.4 | Ownership honesty: `TokioRuntime::tracked()` returns 0 after shutdown; a dropped polled shutdown leaves ownership recoverable | `async-witness-real` tests | LC-005 passes on the real adapter; `TokioRuntime::shutdown(budget)` returns `Ok(())` | 2.3 |

2.1 can start as soon as 0.4 lands; it needs nothing from Phase 1.

### Phase 3 — the real async port: DI-E04 and AR-08 together

Goal: the criterion the graph calls the sharp one, against the real iroh endpoint.
Needs both earlier phases.

| Step | Goal | Touches | Test / observable result | Depends on |
|---|---|---|---|---|
| 3.1 | A `CarrierPort` implementation over `PeerEndpoint`, acquired inside `cx.hold(...)` so the engine owns cleanup poll-atomically | `async-witness-real` | Two witness nodes bind, dial, complete HELLO and exchange one `Frame`, driven by an sdax plan | 2.3, 0.2 |
| 3.2 | **AR-08 for real**: the endpoint is released last; the socket is actually free afterwards | `async-witness-real` tests | After `report.is_clean()`, re-binding the recorded port succeeds — proving no clone escaped | 3.1 |
| 3.3 | Shaku assembles over the acquired handle, from inside an sdax step that `.needs` it | `async-witness-real` | The module step's release is ordered before the endpoint's, asserted by `release_order().before(...)` | 3.1, 1.1 |
| 3.4 | **The differential** — the same plan with and without the Shaku module step | `async-witness-real` tests | Both runs are `is_clean()` and both free the port. A divergence is the answer to `async_witness` | 3.2, 3.3 |
| 3.5 | **DI-E04**: the port crate stays framework-free while a real iroh-backed provider fills it | policy file, negative fixture | Gate green; a fixture adding `shaku` to the ports manifest fails `ARCH-002`; `cargo tree --invert shaku` lists no contract-role package | 3.1, 0.3 |
| 3.6 | The two clocks: `sdax-testkit::FakeClock` drives the engine, the witness's fake drives the port; neither leaks into `async-witness-ports` | `async-witness-real` tests | Advancing both agrees; the gate still shows one dependency on the ports crate | 3.1 |

3.2 and 3.5 are independent of each other after 3.1. 3.4 is the gate for Phase 4.

### Phase 4 — the verdict

| Step | Goal | Touches | Test / observable result | Depends on |
|---|---|---|---|---|
| 4.1 | Measure and record: fast target, real target, cold build, on a named toolchain and machine | witness README | A measured table, per `LibraryBoundaryAndTestingPolicy.md:66` — "Do not describe an unmeasured target as an achieved performance guarantee" | Phase 3 |
| 4.2 | Write the result against §8, criterion by criterion, with the caveats named | a new `arch1/` result document | Every one of DI-E01..E04 has a stated verdict and its evidence; no criterion is silently skipped | 4.1 |

The owner records `shaku_confirmed` or `selection_reopened` on the graph. **An agent does
not.**

### What can run in parallel

- **Phase 0 gates everything.** 0.4 in particular stops at the owner action 0.0.
- **Phases 1 and 2 are fully independent after 0.4.** Different crates, different files,
  different agents, different days. This is the main parallel opportunity.
- Within Phase 1: 1.1 first, then **{1.2, 1.3, 1.4, 1.5} in parallel**.
- Within Phase 2: strictly sequential, 2.1 → 2.2 → 2.3 → 2.4.
- Within Phase 3: 3.1 first, then **{3.2, 3.5, 3.6} in parallel**; 3.3 also needs 1.1;
  3.4 needs 3.2 and 3.3.
- Phase 4 needs all of Phase 3.

---

## 7. Standing code rules the implementation must follow

These are the owner's, restated so no step has to go looking:

- **Every control-flow body is a braced block.** In Rust, `if`, `else`, `for`, `while`
  and `loop` bodies are `{ ... }`, including single-statement and empty ones. An
  `else if` chain is allowed with braced branch bodies.
- **Conditional compilation only inside an explicit boundary.** Use `cfg_if::cfg_if!` or
  an enclosing platform module. Never a bare `#[cfg(...)]` or conditional `cfg_attr` on an
  individual import or a single unbraced declaration. Unconditional imports stay outside
  the conditional section. Ordinary non-conditional attributes are not affected.
- **Deleting or moving a declaration takes its attributes and owning scope with it.** A
  condition must never silently transfer to the next declaration.
- **Prefer fast syntax-aware source checks** over review, and they must inspect disabled
  branches too. §4.5's manifest-level check satisfies this, and its two limitations are
  named there rather than hidden.
- **TDD, Rule 0** (`glade-wz/AGENTS.md:3-8`): a failing test before the code, the smallest
  change to pass, refactor only green, a regression test for every bug. The di-eval
  precedent records RED before GREEN (`di-eval/README.md:47-50`); this witness does the
  same.
- **Do not relax a classification or a dependency allowlist to make a check pass**
  (`glade-wz/AGENTS.md:31-32`). Record it and get it reviewed.

---

## 8. The decision rule

This exists so the result cannot be argued afterwards. It is written before the
experiment runs.

### 8.1 The governing distinction

`async_witness` is a question about **Shaku**. Its outcomes move the injector, not the
lifecycle library.

- A failure that is **caused by the injector or its bridge** records
  **`selection_reopened`**.
- A failure that is **caused by sdax-rs** records nothing on `async_witness`. It is a
  `lifecycle_composition` matter (Q10 / R28), and the correct action is to **stop and
  report**, not to reopen the injector.
- Step 3.4's differential is what separates them. If the identical sdax plan is
  `is_clean()` and frees the port **without** the Shaku module step, and is not clean or
  does not free the port **with** it, the fault is the bridge's and `selection_reopened`
  is recorded. If both runs fail the same way, the fault is not Shaku's.

### 8.2 Per criterion

| | Records `shaku_confirmed` | Records `selection_reopened` |
|---|---|---|
| **DI-E01** | One fake instance is observed at every declared consumer by `Arc::ptr_eq`; the real-provider construction counter reads 0; `async-witness-fast` declares no tokio, iroh or socket dependency, so hidden I/O cannot compile into it | Shaku cannot express a single override reaching all consumers for a Glade contract trait; or eager construction of a real provider cannot be prevented by `#[lazy]` or a module split; or preventing it requires changing a Glade contract |
| **DI-E02** | `ptr_eq` true within one module and false across two independently built modules; two threads racing the first resolve of a `#[lazy]` component observe exactly one construction | More than one construction under concurrent first access; or two independently built modules share an instance |
| **DI-E03** | The missing binding fails to compile (E0277), the constructor cycle fails to compile (E0275), an ambiguous role requires a key and a missing key is an absent map entry; no string-keyed or global resolution exists anywhere in the witness | Any of the three is accepted at compile time and surfaces only at run time; or selecting a role requires a service locator |
| **DI-E04** | `async-witness-ports` compiles and is usable with `glade-wire` as its only dependency while a real iroh-backed provider fills its `CarrierPort`; the gate fails closed on an injected `shaku` entry; `cargo tree --invert` finds no contract-role package | The real port cannot be reached without a framework type appearing in a contract or pure crate's manifest or public signature; or making it work requires adding `Sync`, a lock or a boxed public future to a Glade contract |

### 8.3 AR-08, recorded separately

AR-08 is witnessed by Steps 2.1–2.4, 3.2 and 3.4 and is reported **against R28/Q10**, not
against `async_witness`. It passes when: reverse order is asserted statically by
`release_order().before(...)`; a clean run reports `is_clean()`, never merely
`outcome == Ok`; an expired budget lists the unfinished resource in `report.incomplete`;
a cancelled run still executes the release graph; `TokioRuntime::tracked()` is 0
afterwards; and the recorded UDP port can be re-bound.

### 8.4 What counts as a pass with caveats

A caveat is an observation that is undesirable, **already recorded in the evaluation**,
and does not prevent the criterion being met. These three are caveats, not failures:

1. **The cycle diagnostic is poor.** E0275 is a trait-bound overflow, not a graph
   explanation — already recorded at `DependencyInjectionEvaluation.md:39`. The cycle is
   still rejected before startup, which is what DI-E03 asks.
2. **Eager construction needs help.** Shaku's default module constructs registered
   components even when another is overridden; `#[lazy]` or a separate module defers it —
   already recorded at `DependencyInjectionEvaluation.md:41`. Using either is a caveat.
3. **`Arc` escape is possible in principle.** Both frameworks allow it
   (`DependencyInjectionEvaluation.md:85-86`). It is a caveat **only if** the witness's own
   composition demonstrably prevents it — Step 3.2's re-bind is the proof. If the port
   cannot be re-bound, it is not a caveat; it is a DI-E01 and AR-08 failure.

**What is never a caveat.** Any change to a Glade public contract made to satisfy the
container. `DependencyInjectionEvaluation.md:96-97` is explicit: "Do not add `Sync`, locks
or boxed public futures merely to please a container." If the bridge needs one, that is
`selection_reopened`, stated plainly.

A caveat must be written into the Phase 4 result document with its evidence. An unrecorded
caveat is a failure.

---

## 9. Risks and the time box

### 9.1 Time box

**Three working days of agent time**, phase by phase: Phase 0 half a day, Phases 1 and 2
one day in parallel, Phase 3 one day, Phase 4 half a day. Phases 1 and 2 running in
parallel is what makes this fit.

### 9.2 Stop and report rather than push on

Each of these ends the work immediately, with a written report and no further attempts:

| Trigger | Why stopping is right |
|---|---|
| `ce339a5` is not on the remote, or the Git dependency cannot be fetched | The ruling's posture cannot be executed. This is an owner action (0.0), not an engineering problem to route around with a `path` dependency, which would defeat the pin |
| The bridge does not compile after **two** remediation rounds | Two rounds is the estate's cap. A third is evidence, and the evidence is `selection_reopened` |
| Making it work would require editing `glade-lifecycle-api` or any other contract | §8.4. Report the required change; do not make it |
| An sdax-rs defect blocks Phase 2 | Not a Shaku result. Report against R28/Q10; do not record `selection_reopened` |
| The real target's build cost makes Phase 3 impractical | Report the measurement and propose the WebSocket runner-up of §3.3. Do not silently substitute it |
| The fast target misses a budget | Record the measurement (`LibraryBoundaryAndTestingPolicy.md:66`). Do not optimise; an unmeasured target was never a guarantee |

### 9.3 Risks

| Risk | Disposition |
|---|---|
| The sdax rev is unreachable, or the repository is private with unverified anonymous access | Named as blocking prerequisite 0.0. `ReleaseReadiness-2026-09-08.md:97-99` already says anonymous access is unestablished |
| `sdax-tokio`'s exact `=1.53.1` pin conflicts with a future iroh | Checked today: iroh 1.2.0 wants `^1.44.1` and they unify. Re-check on any iroh bump; the `=` pin means sdax-rs, not Glade, decides the tokio version |
| `TokioRuntime::new` **panics** on a `current_thread` handle (`sdax-tokio/src/lib.rs:114-125`) | Every witness test uses `#[tokio::test(flavor = "multi_thread")]`, as the node's own iroh tests already do (`iroh_carrier.rs:137`, `:159`). `sdax-tokio` disables tokio's `macros` feature, so the witness needs its own tokio dev-dependency with `macros` |
| iroh's real teardown takes seconds (`endpoint.rs:1700-1704`) | Budget the real target accordingly; it is not the fast target. Do not shorten the budget to make a test quick — that turns a drain into an abandonment, which is exactly what `report.incomplete` exists to reveal |
| The witness proves a synthetic port rather than a Glade one | `async-witness-ports` carries `glade-wire` `Frame` bytes and the provider is the node's own `PeerEndpoint`. Both are real |
| INV-5/INV-6 have no independent falsification in sdax-testkit (`invariants.rs:27-32`) | Step 2.1 asserts release order from the **Glade** side via `release_order().before(...)`, independently of sdax's own checker |
| The node's blocking `Store` is locked from async code (`server.rs:83`) | Out of scope for this witness; recorded in §3.3 as an AR-08 bounded-executor finding for later |
| An agent "fixes" a contract to make the bridge compile | §7 and §8.4 forbid it; the stop rule catches it |

---

## 10. Out of scope, explicitly

- **No production wiring of the node.** `glade/node/Cargo.toml` is not edited, no module
  is added to `glade/node/src/`, and the demo is untouched (`GladeBuildEntry.md:19`:
  "keep the demo untouched").
- **No publication of sdax-rs**, no tag, no visibility change, no `crates.io` push. Its
  publication is a shared blocker recorded in GDL-049 and is an owner decision.
- **No git operation on sdax-rs.** Neither lane is merged, rebased, pushed or tagged by
  this plan. The `code-complete` tag is not moved.
- **No re-evaluation of other injectors.** Dill, a hand-rolled context bag and everything
  else stay where R27 left them **unless** §8 records `selection_reopened`, which is the
  only door to that work.
- **No re-opening of `lifecycle_composition`.** sdax-rs is ruled. This witness tests the
  wiring, not the choice.
- **DI-E05, DI-E06 and DI-E07 are not witnessed here.** DI-E06 is partly exercised
  incidentally by Step 1.5 (two roles, one shared occurrence) but is not claimed. DI-E05
  overlaps AR-08 heavily — failure or cancellation after partial async startup — and
  Steps 2.2 and 3.4 produce evidence toward it, but it is reported as AR-08 evidence
  under R28 and **not** claimed as DI-E05 complete.
- **No change to any public contract**, no new Glade crate outside the witness workspace,
  and no adoption of the witness's policy file as a repository-wide gate.
- **No `ClockPort` introduced into the node**, even though §3.3 shows where one would go.
  That is design work the witness may inform but does not perform.

---

## 11. What this document is

A plan for one bounded experiment, written read-only, for the owner to correct. It selects
no dependency, changes no manifest, and records no outcome on the decision graph. The
result of running it belongs in a separate document and the ruling belongs to the owner.
