# Shaku / Dill evaluation for Glade

Date: 2026-09-09. Status: **bounded measured evaluation; Shaku subsequently selected**.
“Shaju” is interpreted as Shaku. The owner's correction is important: GWZ's
application context was a recovery measure after wiring became unmanageable,
not the pattern Glade should automatically reproduce.

Subsequent owner decision, 2026-09-09: **select Shaku** and record it in the
architecture declaration. GDL-048 supersedes the evaluation's selection-open status, not
the measured findings or integration limits below. Production integration remains
unimplemented; this report is preserved as the selection evidence.

## Recommendation

**Prefer Shaku for the next small composition/async-bridge witness. Do not adopt
Dill's default scoped composition for Glade on the present evidence.** Both can
inject one coordinated fake and keep independent tests isolated. Shaku provides
stronger early wiring rejection in the cases tested; Dill's flexible runtime
catalogue leaves more important checks to application discipline.

This is not a recommendation to build another DI framework, pass a universal
context everywhere, or change Glade's public async traits. A failed Shaku witness
would reopen selection; keep the framework confined to assembly/factory adapters.

## What was actually evaluated

[Runnable probes](../../glade/dev-docs/di-eval/README.md): Shaku **0.6.3**, Dill
**0.17.0**, pinned transitive lockfile, Rust **1.96.0** on Apple Silicon macOS.
23 runtime characterization tests and four expected compilation failures were run.
These are small synthetic wiring tests plus one actual Glade lifecycle-trait
compatibility check—not a migrated Glade subsystem or production validation.

| Architectural question | Shaku 0.6.3 | Dill 0.17.0 |
|---|---|---|
| How is wiring declared? | Derive component, mark injected fields, declare a typed module; generated trait bounds check available dependencies | Annotate components/interfaces/scopes, register builders in a runtime `Catalog`, resolve by type/interface |
| Replace a dependency once for cooperating callers? | Module builder's component override; observed shared fake identity and subsequent fake-clock changes | Register one fake provider in the test catalog; observed shared identity. Per-builder field overrides also work, but repeat them inconsistently and consistency is lost |
| Independent tests and parent/child sharing? | Fresh modules isolate fakes. Explicit shared parent submodule shares its clock while children retain distinct readers | Fresh catalogs isolate fakes. Sequential `TransactionCache` use shares within a child and separates siblings, subject to the hazards below |
| Missing registration? | Tested missing binding fails compilation, E0277 | `validate()` reports missing dependency; `build()` does not require successful validation; resolution can still fail |
| Constructor cycle? | The tested derive-based A↔B graph fails compilation, E0275; diagnostic is a trait-bound overflow, not a friendly graph explanation | `validate()` accepts the tested singleton cycle. We deliberately did not resolve it; recursive singleton locking is a deadlock risk inferred from source |
| Multiple implementations of one port? | Keyed multibinding tested with typed peer/client keys; missing key remains a runtime lookup result | Multiple bindings supported, but singular resolution is ambiguous. Tested `validate()` accepts this ambiguous graph. Selecting a named role remains composition work |
| Avoid starting unused real services in tests? | Default eager module constructs other registered components even when another service is overridden. `#[lazy]` successfully deferred unused construction; separate test modules/submodules can omit real providers | Resolution is lazy by builder/scope design. Build-time registration is not by itself resource startup; already-created values are, of course, already created |
| Independent contract crates? | Direct foreign `dyn Clock` fails the Shaku `Interface` bound even with `Any + Send + Sync`. Assembly-only facade traits work in the probe | Independent trait works; macro/cast registration is supplied on an assembly-owned implementation wrapper, not added to the contract crate |

The declaration/registration mechanisms are documented by the projects:
[Shaku](https://docs.rs/shaku/latest/shaku/),
[Dill](https://docs.rs/dill/latest/dill/). Specific observed outcomes above are
reproducible in [Shaku probes](../../glade/dev-docs/di-eval/tests/shaku.rs) and
[Dill probes](../../glade/dev-docs/di-eval/tests/dill.rs), not inferred from marketing.

## Dill findings that matter for tests

1. **A child catalog is not an override layer.** Registering another `Clock`
   underneath a parent with a real `Clock` makes singular resolution ambiguous;
   it does not transparently substitute the fake. Build a selected test composition
   before resolution instead of assuming child registration shadows its parent.
2. **A parent singleton can capture a child's dependency.** The probe registers
   the reader only in the parent and different clocks in two children. Both child
   validations succeed. Resolving the reader through child 1 first causes child 2
   to receive the same reader with child 1's clock. This is not inevitable in every
   Dill design; it is an unsafe composition that validation did not reject.
3. **Transaction-cache first creation is not atomic.** Two coordinated first
   callers of the same built-in transaction scope/cache obtain different instances.
   The implementation separates lookup, factory execution and cache insertion.
   This matters if “one per scope” must also mean one under concurrent access.
   The probe tests the built-in scope directly, not a complete async application.
4. **Validation is partial and opt-in.** It caught missing dependencies and one
   singleton→transient scope inversion, but not the tested ambiguity or cycle.
   A passing validation call cannot serve as a complete composition proof.

Source corroboration in the installed 0.17.0 crate: `catalog_builder.rs::validate`,
`catalog_impl.rs::builders_for`, `specs.rs::OneOf`, `scopes.rs::{Singleton,Cached}`
and `cache.rs`. See the upstream
[catalog builder](https://docs.rs/dill/latest/dill/struct.CatalogBuilder.html) and
[scope API](https://docs.rs/dill/latest/dill/scopes/index.html).
No upstream bug report, dependency patch or production workaround was submitted.

## Async and “lifetime” are a separate boundary

DI scope determines **which instance is reused and where**. Rust borrow lifetimes
determine reference validity. Neither means “await resource shutdown before releasing
its dependencies.” Both libraries' ordinary construction APIs inspected here are
synchronous. Async initialization can happen in an explicit bootstrap step followed
by registration; failure, cancellation and partial construction need an owner.

Both Arc-escape probes show a service surviving container drop. Dropping a container
is therefore not even a guarantee that all its services have been destroyed, much
less that their async work has drained. A future resource-ownership witness MUST
preserve [Glade's lifecycle contract](../../glade/contracts/lifecycle-api/src/lib.rs),
including retained unfinished shutdown work and truthful completion reports.

The actual `ManagedResource` trait returns `impl Future` and is **not dyn compatible**;
the compiler confirmed E0038. `Invoker` also uses a generic future/associated binding.
This does not make the traits wrong or rule out either framework: concrete injection
can work, while runtime-polymorphic injection needs a reviewed object-safe bridge or
generic composition. `ManagedResource` is `Send` with mutable shutdown, not a blanket
`Send + Sync` shared service. Do not add `Sync`, locks or boxed public futures merely
to please a container. See Rust's
[dyn-compatibility rules](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility).

Dill's optional Tokio task-local catalog is an additional lookup mechanism, not an
async cleanup supervisor. It was not enabled in these probes. No claim is made
about spawned-task context propagation, cancellation safety or real-I/O startup.

## Fast feedback and dependency pressure

| Offline clean-target build + tests | Unchanged warm repeat | Runtime tests |
|---|---|---|
| Shaku: **3.49s** | **0.03s** | 9, reported execution 0.00s rounded |
| Dill: **5.32s** | **0.03s** | 14, reported execution 0.00s rounded |

One sequential sample each, debug profile, cached downloaded sources, elapsed
Cargo-inclusive wall time. These unequal tiny fixtures are not a performance ranking
or proof of whole-project scaling. Initial network install time is excluded.

The common witness port crate has **zero third-party dependencies**. Shaku adds
`anymap2` and its derive machinery; Dill adds derive machinery, `indoc`, `multimap`
(with Serde in this resolution) and `thiserror`. Both published manifests declare
MIT/Apache-2.0 licensing alternatives. Shaku declares Rust 1.88 minimum; Dill has
no `rust-version` declaration in the inspected release. Only Rust 1.96 was tested.
This records package metadata, not a complete licensing/security/maintenance audit.

**DI does not fix a monolithic test target.** Overriding a real component or making
it lazy can avoid its execution without removing its compile/link dependency. Core
unit tests SHOULD instantiate the component with supplied narrow fakes directly;
small composition tests exercise the selected injector. Real-adapter conformance
and integration remain separately selected. No full-workspace test run is needed
to edit a deterministic leaf library, and a mock does not prove real durability.

## Proposed acceptance for the next witness, not completed work

| ID | Required evidence |
|---|---|
| DI-E01 | One fake clock/store/carrier selection reaches every relevant caller; real-provider construction and hidden I/O are rejected in the fast composition |
| DI-E02 | Equal scoped binding → shared identity, independent test scope → isolation, also during concurrent first access |
| DI-E03 | Missing/ambiguous role and construction cycle rejected before useful startup; no late service-locator surprises |
| DI-E04 | The selected real async Glade port remains usable without framework imports in its contract/pure libraries |
| DI-E05 | Failure or cancellation after partial async startup retains ownership and cleans up correctly; escaped handles cannot be mistaken for completed shutdown |
| DI-E06 | Peer and client carrier roles select distinct providers; multiple ports intended to share one adapter actually do |
| DI-E07 | Test compilation, execution and affected-consumer selection stay isolated; public API changes retain contract review |

Next bounded experiment: Shaku assembles a small real-contract slice with supplied
fakes and an async resource owner, including cancellation/failure. This evaluation
does **not** authorize adapting production APIs or adopting dependencies. Dill
could be reconsidered after the failing requirements above are addressed and rerun.

The corresponding [graph revision](InjectionGraphRefinement.md) captures selection,
scope and lifecycle facts now. It is not a completed DI semantic validator.
