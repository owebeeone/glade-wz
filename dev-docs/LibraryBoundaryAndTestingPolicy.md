# Library Boundary and Testing Policy

Date: 2026-09-05. Owner: root engineering.

Status: **owner-requested engineering requirements for this workspace; reusable recommendation elsewhere**. A different project MUST explicitly adopt this policy before treating it as mandatory. The exact package decomposition of any system is a separate architectural decision.

Purpose: keep libraries independently understandable, replaceable where appropriate, and cheap to compile and test. An interface inside a tightly coupled implementation package is not sufficient isolation. This policy is language-independent; section 5 describes Rust enforcement.

Agents working on libraries in this workspace MUST read this document before introducing a library, changing a public boundary, adding a dependency, or changing test selection. Existing libraries MAY migrate incrementally under named, reviewed exceptions; this document does not claim that every repository already conforms.

## 1. Classification and ownership

Every library MUST have an explicit architectural classification. There is no reliable automatic definition of “non-trivial”; line counts and the presence of any trait are inadequate substitutes for architectural intent.

| Role | Responsibility | Required boundary |
|---|---|---|
| Contract | Public behavior and boundary data | Named interfaces/traits with meaningful required operations; no concrete implementation dependency |
| Implementation | A replaceable service, policy, or algorithm implementation | Implements a named contract and runs its behavioral conformance suite |
| Pure | Deterministic transformation or state machine | Explicit input/output contract and deterministic tests; a trait is not mandatory if it adds no substitution value |
| Protocol/data | Canonical values, schemas, encoding, decoding | Versioned representation contract and compatibility tests; no artificial marker trait |
| Integration | Composition, lifecycle, I/O orchestration | Narrow injected ports, focused integration tests, and a documented reason for concrete dependencies |
| Harness/tool | Simulation, system-test orchestration, or development tooling | Kept outside production dependency paths; bounded fast mode separate from exhaustive runs |

A classifier MUST reject an unknown library until its role is recorded. A non-service classification MUST include a rationale. Changing a classification, adding an exception, or relaxing an allowlist is a policy change requiring review, not a routine way to make a failing check green.

## 2. Requirements

These IDs are stable anchors for project-specific enforcement and tests. “Review” in the final column identifies a requirement that a structural checker alone cannot prove.

| ID | Requirement | Verification |
|---|---|---|
| LBT-001 | Every in-scope library MUST have an explicit, reviewed role. An unknown library MUST fail the architecture gate. | Package inventory versus classification; negative fixture for a new unclassified library |
| LBT-002 | A replaceable implementation MUST implement its named contract. A contract MUST name meaningful required operations; an empty or unrelated marker interface MUST NOT satisfy the rule. | Trait/interface syntax, compiler-checked implementation, conformance tests; semantic relevance requires review |
| LBT-003 | Contracts MUST NOT depend on concrete implementations. Pure/protocol libraries MUST NOT acquire integration or harness dependencies. Dependency direction MUST be acyclic. | Manifest graph and language build; explicit allowed edges |
| LBT-004 | Libraries MUST depend on the smallest practical contracts, not a global “common” package. Public boundary types MUST NOT expose transport/database/runtime implementation types. | Dependency gate plus public API review; transitive type leakage requires additional tooling or review |
| LBT-005 | Normal, build, optional, target-specific, and development dependencies MUST be considered. Renaming a dependency MUST NOT bypass checks. Test helpers MUST NOT pull the entire node/application into a library's fast test path. | Cargo-equivalent metadata; dependency fixtures across kinds, aliases, and targets |
| LBT-006 | Behavioral contracts MUST state inputs, outcomes, failure modes, lifecycle, and guarantees. A successful call MUST NOT imply an unstated durability, replication, authorization, or completeness guarantee. | Requirement-to-test mapping and failure tests; review |
| LBT-007 | Every behavior change MUST begin with a failing test; bug fixes MUST begin with a regression test. Tests MUST cover success, failure, and edge conditions. | RED/GREEN evidence, conformance suite, review |
| LBT-008 | Pure-library tests MUST use deterministic time, randomness, and delivery schedules. They MUST NOT use real sleeps, external services, or a running application. | Injected inputs/ports, reproducible cases, targeted checks and review |
| LBT-009 | Each implementation MUST run applicable shared contract tests against itself. Test doubles MUST be contract-faithful; mocks alone MUST NOT be presented as production-adapter verification. | Reusable suite plus concrete adapter tests; compiler checks |
| LBT-010 | A package MUST expose a small, measured fast-test command. Full-system, stress, and fuzz suites MUST be separate from the ordinary local edit/test loop. | Explicit commands and measured budgets; selection checks |
| LBT-011 | Public-contract changes MUST test affected consumers. Internal changes SHOULD run the owning package's unit/conformance tests first; wider checks follow actual impact. | Dependency-aware CI selection and review, not “only changed files” heuristics |
| LBT-012 | Exceptions MUST identify the exact package/boundary, rationale, review owner, and retirement condition. Broad exemptions for future packages MUST NOT be used. | Policy/exception review and stale-entry checks |

Trait signatures are only the structural part of LBT-002. A `lookup()` method returning an empty value forever may compile but still violate the contract. Relevant behavior belongs in conformance tests. No checker can determine the intended architecture or defeat an agent that is permitted to rewrite its policy and tests without review.

## 3. Package structure and dependency inversion

Prefer a composition root that assembles concrete implementations. Callers and implementations both depend on a small contract, while callers do not import the concrete service package. Interfaces for external actions SHOULD be owned by the consuming boundary or a small dedicated contract package.

Separate a contract package when it creates a useful compilation/replacement boundary. Do not create one package per method, split tightly coupled invariants artificially, or manufacture traits for data structures and pure functions just to satisfy a mechanical rule.

Explicit events and effects can be a better boundary for a deterministic state machine than a collection of object-style traits. The host performs I/O; the state machine consumes explicit outcomes. Boundary translation belongs at integration points rather than in a universal shared type package.

The following are distinct:

- Compile-time dependency: which package must be available to compile another?
- Runtime cooperation: which components exchange data or invoke a supplied implementation?
- Wire compatibility: which versions can communicate across processes/languages?
- Deployment: which roles happen to run in the same executable or machine?

An in-process interface does not itself define a stable cross-language protocol, plugin ABI, or security boundary.

## 4. Fast feedback and test selection

Each adopting project MUST record its fast command, machine/toolchain context, and measured budget. Measure test execution, warm incremental build plus tests, and cold build separately. Do not describe an unmeasured target as an achieved performance guarantee.

Budget values remain project-specific; this policy deliberately does not impose a universal seconds threshold. A fast test target MUST not silently accumulate exhaustive scenarios as the project grows. Shared test utilities SHOULD be development-only dependencies and SHOULD themselves be small.

A small, deterministic contract-test helper MAY itself be classified as pure, depending only on contract/data libraries and used through a development dependency. It MUST NOT conceal a dependency on concrete adapters or a full-system harness. The classification follows its dependency and behavior contract, not simply the fact that its callers are tests.

| Change | Immediate development checks | Wider verification |
|---|---|---|
| Internal implementation | Owning package's unit and relevant conformance tests | Focused integration tests where behavior crosses a boundary |
| Public interface or boundary data | Contract tests, implementations, affected consumers | Compatibility matrix and relevant end-to-end scenarios |
| Concrete I/O adapter | Adapter unit/conformance tests | Actual I/O, restart and fault-injection tests |
| Composition/wiring | Targeted multi-component tests | Relevant end-to-end slice |
| System assurance | Not mandatory on every minor local edit | Full suite, stress, fuzz and scale runs in explicit CI/release jobs |

Fast selection MUST NOT eliminate system assurance. Contract changes require more verification than isolated implementation changes. Passing an isolated suite does not prove distributed correctness, performance, or production durability.

## 5. Rust-specific application

A Rust **trait** is the equivalent of an interface. A required trait method has a signature and no default body. A type supplies the implementation through `impl Trait for Type`. A generic trait bound uses compile-time dispatch; `dyn Trait` enables runtime dispatch where the trait is dyn-compatible. Rust generally uses composition rather than class inheritance. [Rust traits](https://doc.rust-lang.org/book/ch10-02-traits.html), [trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).

A library **crate** is a compilation boundary. A module within a crate is not an independent package/test-build boundary. Introducing a trait without separating unwanted manifest dependencies will not remove their compilation cost.

An adopting Rust repository SHOULD use complementary checks:

1. **Inventory/graph gate:** inspect Cargo metadata; reject unclassified libraries and forbidden declared dependency edges, including renamed, optional, target, build, and dev dependencies.
2. **Source structure gate:** parse Rust syntax rather than search for the string `trait`; require named, public, non-empty contracts and explicit implementation declarations. Comments, dead code, and marker traits are not evidence.
3. **Compiler witness:** compile the real implementation against its actual trait, normally through a conformance test's generic bound. The compiler resolves types, visibility, aliases, and feature combinations that a source linter cannot fully resolve.
4. **Behavioral conformance:** execute the same applicable invariants against each implementation, including failure and edge cases.
5. **CI invocation:** run the architecture gate as a failing job. A plain `cargo build` does not automatically run architecture tests; it only catches trait mismatches that actual code references.

`cargo test -p <package>` selects the package's tests, but may compile dependencies. It does not automatically run every affected consumer's tests. A project needs an explicit impact-selection policy. [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html), [Cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html).

Checks MUST report their coverage honestly. A declared-dependency allowlist is not a resolved third-party transitive-graph audit. Source parsing is not macro expansion, semantic type checking, API-leak analysis, or behavioral proof. Feature-specific/generated contracts need an explicit compiler-verification design rather than a blanket exemption.

Protecting the gate from deliberate removal additionally requires repository review/branch-protection policy. Adding a workflow does not configure a required merge check in repository hosting settings.

## 6. Adoption and current enforcement

The first implementation of the architecture gate is present in `glade-discover`, scoped to library members returned by that repository's primary Cargo workspace. Its standalone checker MUST NOT become a runtime dependency. Other repositories, nested independent workspaces, and non-Rust libraries are not automatically covered. Local scope, commands, exception review and enforcement limits are documented in [LibraryBoundaryChecks.md](../glade-discover/dev-docs/LibraryBoundaryChecks.md).

The current Glade target decomposition and rollout belong in [GladePackageArchitecture.md](GladePackageArchitecture.md). The shared rule belongs here; project-specific documents SHOULD link to it rather than fork the policy. Current enforcement status, commands, exceptions, and limits belong in each adopting repository's local documentation.

Handoff instruction for another agent: read this document, inventory your project's existing boundaries and test commands, propose explicit classifications, and implement only the in-scope adoption work requested by your user. Do not infer authorization to refactor packages merely from receiving this policy.
