# Glade Raft Q3 implementation allocation supplement

Date: 2026-10-03. Status: **allocation/lifecycle accepted at root `ccab267c6b23bfec7471923098944048a7959563` after [Consistency](GladeRaftQ3Allocation-ReviewConsistency.md)/[Safety](GladeRaftQ3Allocation-ReviewSafety.md) GO; unchanged member/Gyld tuple in the review ledger. This accepts exact roles/edges, lifecycle declarations and compiling RED providers only, not Q3 algorithms**.

This supplements the accepted [Q3 configuration/snapshot contract](GladeRaftConfigurationSnapshotContract.md), source `ed243db983c485e46a27aa870ec745de16a56d7a`, filed at root `e1c260f6cf992f5d890c2ca454e2323b0d8e78b1`. It proposes the concrete provider allocation and a small injected store-lifecycle contract that the accepted declaration deliberately left for implementation allocation. It MUST pass the new contract gate before any V2 journal, membership or snapshot algorithm is implemented. It does not reopen Q1a/Q2 guarantees or claim Q3 qualification.

## 1. Bounded package allocation

No new package or third-party dependency is proposed. Existing Q1a/Q2 implementations remain in their current modules and keep their contracts/format. Normal dependency direction remains acyclic. Concrete filesystem composition is development-only for the carrier harness and Q3 consumer.

| Package | Role and proposed responsibility | Exact dependency change |
| --- | --- | --- |
| `glade-raft-q3-api` | Contract: accepted checkpoint/session values plus injected store lifecycle below. No filesystem, carrier or provider types. | None; existing normal adoption-api/durability-api only |
| `glade-raft-disk` | Implementation: `v2::V2DiskStore` implements CheckpointStore; `v2::V2StoreFactory` implements StoreLifecycle. Distinct V2 physical format, Q2 unchanged. | Add normal q3-api |
| `glade-raft-adoption-proof` | Harness: `q3::Q3Session` implements QualificationSession with real RawNodes, deterministic FIFO schedules, injected stores/lifecycle and trusted group authority inputs. | Replace accepted dev q3-api with normal q3-api; existing dev disk stays |
| `glade-raft-q3-spec` | Harness: unchanged reusable semantic functions execute through integration-test composition of real Q3 session/V2 store. Refusing historical scaffold remains a labelled non-provider. | Add dev disk and dev adoption-proof; normal q3-api stays |

The new edges and provider inventory in `architecture-policy.json` are **proposals requiring this review**, not automatic policy approval because a checker passes. No old role, edge or gate is relaxed. q3-api owns the smallest useful local storage lifecycle seam rather than putting an interface in the carrier harness and forcing filesystem implementers to compile Raft. StorageAdapter owns physical lifecycle/publication; Records owns semantic replay/configuration; Admission/Policy owns valid bound authority; NodeAssembly composes instances. Discovery supplies no authority. The composition helper lives only under q3-spec's integration tests, not its library or a production path.

A normal proof dependency on q3-api is now necessary because the actual harness implements and consumes its session/checkpoint boundary in library modules; keeping it dev-only would force production-shaped implementation into tests or bypass the injected contract. A normal disk dependency on q3-api is necessary to implement the coherent V2 store and lifecycle. These dependencies do not authorize production reuse of this experiment.

## 2. Proposed lifecycle boundary and ownership

The additional dyn-compatible `StoreLifecycle` trait in q3-api has required operations:

```rust
fn create(&mut self, instance: Instance, initial: Image)
    -> Result<Box<dyn CheckpointStore>, Error>;
fn open(&mut self, instance: Instance, minimum_revision: Option<u64>)
    -> Result<Box<dyn CheckpointStore>, Error>;
```

This is a meaningful injected physical lifecycle seam, not a marker or authorization interface. It contains only existing accepted contract values. The public compiler consumer invokes both operations through `dyn StoreLifecycle` without path/carrier types. Its `create` corresponds to explicit exclusive physical creation with a complete initial coherent image; `open` corresponds to retained-history recovery with an independently supplied revision floor. Both operations return the accepted store contract rather than a concrete adapter.

`disk::v2::V2DiskStore::create_new(&Path, Instance, Image)` and `open(&Path, Instance, Option<u64>)` are concrete composition operations. `V2StoreFactory::new(PathBuf)` receives an existing owned root and maps the bounded node ID to `node-{id}` under it. Construction MUST NOT create directories, infer authority from empty paths, read ambient environment, select a different root or reset existing files. Each store retains its exclusive OS file handle until drop. Paths/files/format remain private to this adapter. Existing Q2 journal and adapter behavior are unchanged.

`Q3Session::recover(BTreeMap<u64, Box<dyn CheckpointStore>>, Box<dyn StoreLifecycle>, Vec<u64>)` receives initial retained stores, physical lifecycle, and explicit trusted configuration authority principals. The profile binds scope 7/group 70/application profile 2 and authorities `[1,2]`; any foreign/unsupported authority input MUST refuse before serving. The fixture composition alone supplies genesis authority for initial voters `[1,2,3]`. The numeric vector is neither cryptographic evidence nor discovery input.

The HOST MUST validate initial fixture authority or a committed, bound existing-group AddLearner intent **before invoking create**. In particular, an unauthorized/wrong-group/uncommitted join cannot call the factory or create a learner path. The physical factory does not authenticate an intent and its successful structural creation does not grant voting, home, serving or new-group authority. Tests MUST observe both the host's absence of a create call/file for denied joins and successful creation only after the real existing group's committed accepted learner-add entry. Injection by the trusted assembly is a profile boundary, not a security capability exposed to arbitrary callers.

For learner 4 creation after accepted AddLearner, host authorization MUST come from the existing group's retained committed accepted join intent, validated before calling create. The new local Image MUST be structurally coherent with that bound group, but MUST NOT claim that the receiver has applied the join cut merely because the host observed it elsewhere. The receiver MAY start with term/vote/commit/applied zero, no checkpoint or suffix, and the ORIGINAL authorized genesis ConfState `[1,2,3]`/version 0, solely as a private nonserving catch-up follower. This is creation of a newly owned learner path after existing-group authorization, never a new group genesis, missing-store reset or replacement of retained history. The host MUST prevent this incomplete receiver from campaigning, voting, providing quorum/home readiness, serving or releasing outcomes; it may consume only the validated catch-up protocol until the actual original log or authorized snapshot installs the join configuration and complete application state. Bootstrap context and the externally validated join are host-owned instance evidence, not authority inferred from the empty local image. Restart MUST recover the same existing-group admission from available retained group history and reopen the learner path; inability to validate it blocks participation rather than creating/resetting storage.

A validated complete prefix or checkpoint through the join MAY instead initialize the new image, but that seeded receiver MUST NOT be cited as witnessing actual snapshot installation at the same join cut: raft-rs may reject or fast-forward a matching cut. The combined actual-transfer witness MUST use a receiver which lacks the snapshot's original term/cut, then durably receive the fresh learner-add snapshot at S plus a later original suffix. Host validation precedes creating any serving/eligible RawNode; the private incomplete follower is explicitly excluded from those roles until real catch-up. No fabricated applied counters or old-cut ConfState rewrite is permitted. After compaction, the transferred authorized checkpoint MUST contain 4 at its actual learner-add cut; an older cached checkpoint excluding 4 is rejected.

For `Control::Restart`, the host MUST drop all old store handles before invoking open, retain the explicit bound instances and externally trusted floors, and validate/replay all opened images before constructing any serving RawNode. It MUST NOT substitute load on the old live handles for physical reopen. A missing/empty/corrupt/old-format/wrong-bound required file blocks serving; open never creates it. A joining learner's absent file must be handled only by an already validated committed join and explicit new-path creation, never as replacement genesis. Unknown in-flight membership remains unknown until original retained outcomes/history establish its state.

The Q3 session keeps only injected per-instance state. Fault controls are one-shot instance inputs using the already accepted `FaultPoint` values. Process-global exceptions remain empty. Test roots are explicitly constructed in integration test code and passed to the provider, which never reads ambient process inputs. Child-kill orchestration remains external Python with a minimal explicit environment, as Q2; no production Rust child spawn/global exception is added.

## 3. Source boundaries and carrier obligations

Current scaffold files are `disk/src/v2.rs` and `proof/src/q3.rs`; every provider operation refuses NotQualified. A private File field states the intended owned-handle lifetime shape but refusing constructors do not open a file. Factory methods only wire injected paths to those refusing constructors. No working journal, session, permissive model or algorithm is present.

At a green implementation checkpoint, cohesive modules SHOULD separate V2 physical publication/locking, bounded framing, whole-journal recovery and structural transitions. The Q3 host SHOULD separate FIFO composition, carrier Storage/Ready lifecycle, ordered application/configuration, private original-history/checkpoint codec and recovery. Existing Q2 modules MUST NOT be copied into a combined file that mixes protocol, I/O and orchestration. The split-files skill was read during allocation; no existing source relocation is warranted while its working behavior remains unchanged. New implementations MUST be reviewed for cohesion before growing past one responsibility, using syntax-aware rust-split where a mechanical move is needed and preserving attributes/owning scopes. Green tests precede relocation/refactor; formatting is a distinct pass.

Real carrier details that constrain implementation/tests:

- Accepted configuration entries call `apply_conf_change` with their exact retained ConfChangeV2; refused entries MUST NOT call it. An empty ConfChangeV2 means leave-joint, not refusal. Applied advancement clears the refused pending configuration frontier. Nested/wrong leave admission MUST refuse before the carrier can rewrite its proposal as an empty normal entry and destroy the intent context.
- Initial explicit campaign commits the real index-1 noop. A new learner MUST restore the actual original genesis configuration/history or a validated snapshot containing itself before replaying later changes. Applying only AddLearner onto empty ConfState cannot reconstruct group authority.
- A custom Q3 `Storage` implementation or wrapper MUST expose the actual retained full checkpoint index/term/ConfState/application bytes and correct first/last/log boundaries. `MemStorage::snapshot` returns current commit/ConfState with empty data and cannot represent the historical learner-add checkpoint plus later suffix by itself. The wrapper belongs to the actual carrier harness, not q3-api or disk.
- Matching index/term snapshot fast-forward returns false and skips carrier snapshot installation; it MUST still preserve full application evidence. True restoration, fast-forward and stale/recipient rejection MUST be separate actual carrier tests.
- Snapshot installation clears memory entries; host restores the coherent retained suffix and HardState, including historical removed-voter votes. Any retained compaction boundary MUST agree with the actual applied/checkpoint cut; never compact beyond applied or fabricate a new ConfState at an old snapshot index.
- Ready/LightReady persistence, candidate validation, application and outgoing message release MUST follow the accepted Q3 ordering. No step/propose/campaign interleaves with an outstanding Ready. Snapshot delivery status MUST be reported as the carrier requires; a stalled snapshot probe is not learner readiness.

These observations come from the locally cached pinned raft-rs 0.7.0 source and are implementation constraints, not substitute qualification evidence.

## 4. TDD and default selection

The accepted nineteen tests in `q3-spec/tests/configuration_snapshot.rs` keep their names and shared semantic functions; their provider bindings now point to dev-composed concrete V2/Q3 scaffolds through `tests/common`. They remain selected by the default command: no ignore, cfg exclusion, default-member change or filter hides an unfinished case. Initially all fail at NotQualified while creating their real provider. This demonstrates composition/compiler shape only; assertions after construction remain unexecuted specifications.

Eight new `disk/tests/v2_lifecycle.rs` cases compile against the actual V2 API and deliberately fail before algorithms. They cover create/sync/reopen/exact identity, missing-open no creation, empty/old-format/wrong-instance no reset, held lock/exclusive creation, invalid instance before creation, trusted old-image floor/no-floor honesty, all five publication fault controls, and whole-frame capacity. The initial old-format case is a labelled incompatible byte fixture; implementation MUST add a real previously qualified Q2 journal compatibility test rather than presenting this label as canonical Q2 evidence.

Before changing additional behavior, the owning package MUST add/run its actual regression RED. GREEN requires the real provider/RawNode/I/O path, never replacement with a permissive model. The existing Application may gain an internal dynamic-membership input after observed RED, but its public Q1a apply/retry contract MUST remain unchanged. Existing Q1a/Q2 tests MUST remain green. No fixture assertion may be weakened because an implementation initially rejects it.

Commands from repository root (set `PROTOC` to the already cached compatible protobuf 3 executable when compiling the carrier):

```sh
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-disk --test v2_lifecycle
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-spec --test configuration_snapshot
proofs/raft-adoption/check.sh
cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --all-targets -- -D warnings
```

The q3-api path remains the bounded deterministic contract/compiler loop. V2 actual I/O is a separate tier; the Q3 consumer compiles the actual carrier and exercises multi-node disk composition. Actual SIGKILL, exhaustive adversaries and cross-node lifecycle scenarios are separate system-assurance tiers. Their measured cold/warm/execution costs MUST be recorded at implementation acceptance; no performance budget is inferred from this refusing scaffold. Default workspace tests remain deliberately RED until all real providers are qualified.

## 5. Q3a/Q3b progression and complete exit witnesses

After this supplement's GO/GO, Q3a builds V2 coherent structural storage plus uncompacted authorized learner/joint/application recovery; Q3b builds the complete semantic snapshot codec, actual carrier install/fast-forward/rejection, suffix and compaction. Each starts actual RED and ends its affected checks. Neither slice alone closes Q3. The implementation MUST satisfy every row in accepted contract §7; the following allocates ownership and records cases beyond the initial nineteen/eight specifications.

| Area and owner | Mandatory actual witness before combined acceptance |
| --- | --- |
| V2 storage/lifecycle, disk | Exclusive create; full file/parent sync; held lock/drop/reopen; old real Q2/empty/missing/wrong-bound/profile no reset; canonical full ConfState structural grammar; full-frame bounds and checked term/index/revision/count conversions; legal committed/uncommitted/same-cut transitions; valid-old replacement with/without independent floor |
| Host admission/configuration, proof | Unauthorized/wrong-group joins call no create; accepted committed join creates once; canonical ConfChangeV2; actual nonvoting complete learner; incoming-only/outgoing-only joint partitions cannot produce receipt; explicit leave/nested/invalid variants; expected config version and principal namespaces; retained deterministic stale readiness/home refusals at every replica |
| Resource and configuration history, proof | Exact full application/configuration intent/receipt/envelope retry and typed original-index replay; actual initial noop; changed-byte conflict/missing distinct; original refused key remains refused after movement/retirement/new successful exit; joint restart preserves current outgoing-home and vote/pending history |
| Complete checkpoint semantics, proof | Versioned complete original applied prefix plus all maps; replay reconstruction compared against resources/names/tombstones/policy/frontier/original outcomes/configuration; validly encoded single-field payload/home/generation/command/result/index/name/tombstone/policy/ConfState/cut adversaries quarantine, not only malformed bytes |
| Carrier snapshot/suffix, proof | Actual install plus matching-term fast-forward/stale and recipient rejection; regenerate learner-add checkpoint containing learner, then replay later real normal/config suffix; coherent Storage first/last/term/cut; committed-but-unapplied advanced durably before serve; uncommitted suffix never applied and actual Raft reconciliation remains possible |
| Publication/lifecycle faults, disk + proof | All five faults with snapshot/configuration/suffix as one coherent record; pre-write usable prior; post-write poisoned; full/prior/quarantine reopen justified by bytes; actual Ready and LightReady no failed lifecycle messages/apply/receipt; snapshot candidate never exposed before sync; actual term MAX/MAX-1 refusal |
| Actual process termination, external oracle + proof | SIGKILL after acknowledged application/configuration and after durable joint/snapshot before apply/reply; parent retains full original Command/ConfigIntent/Receipt/Entry bytes or independently complete expected result; fresh lookup/exact retry/typed replay equal originals; recovering two fresh values alone is insufficient |
| Combined Q3 case, q3-spec + concrete composition | Authorized learner restored through actual checkpoint at learner-add plus suffix, complete durable/applied/policy/retirement comparison, joint commitment/restart exact original retry and explicit leave using both data-bearing authority majorities |

All nineteen shared specifications, real V2 adapter tests, additional owning-carrier regressions, actual crash oracle/self-tests and preserved Q1a/Q2 regression tiers MUST pass together. Architecture, source/cfg boundary, process-global, formatting and all-target Clippy checks MUST pass with zero new exceptions. Code/State reviewers MUST inspect one settled committed source tuple and the complete named profile evidence. Physical journal records remain append-only and are not reclaimed; complete application history remains bounded only by refusal at the 16 MiB whole-record capacity. There is no app-memory/startup/physical disk savings claim.

## 6. Allocation checkpoint evidence

Before this supplement, the new lifecycle compiler consumer failed with unresolved StoreLifecycle, then compiled after the declaration/refusing providers. On this proposed allocation:

- q3-api: **3 PASS**, including dyn lifecycle consumer and existing exhaustive typed replay/Create fixtures.
- q3-spec configuration_snapshot: **19 RED**, all NotQualified during concrete-provider construction, no ignored/filtered cases.
- disk v2_lifecycle: **8 RED** against refusing create/open; later fault/reopen assertions are compiling obligations, not passed safety claims.
- Existing Q1a/Q2 API, proof and disk conformance selections: **51 PASS**, including the two accepted Create compatibility tests. The two separately ignored Q2 disk lifecycle witnesses and external SIGKILL tier were not rerun at this source-only allocation checkpoint.
- Combined selected targets compile with `--no-run`; all-target Clippy PASS.
- Owned architecture/source/format/process-global gate PASS; **23 Rust files, zero exceptions**. Dependency changes remain DRAFT despite structural PASS.

Cached protoc used for actual carrier compilation: `/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64`. Existing Q2 process crash evidence is preserved, not repeated or relabelled as Q3. Implementation timing/full fault/process evidence remains open. Root owner files the exact source tuple, peer-blind prompts/reports and review closure in the existing qualification ledger. No push, production activation or desktop rebuild is authorized by this bounded qualification work.


## Learner bootstrap clarification — pending renewed review

Implementation exposed a satisfiability defect in the original §2 seeded-receiver requirement: a receiver already at the exact required learner-add snapshot cut cannot demonstrate actual installation of that snapshot. The local carrier checks a matching index/term and fast-forwards without restoring snapshot state (raft-rs 0.7.0 raft.rs:2609–2624). A private authorized but incomplete follower therefore must be permitted to receive the original log/snapshot before acquiring an applied join cut. This is a bounded clarification of creation/bootstrap, not permission for empty-store authority, destructive reset, fabricated readiness or production genesis. [The merged disposition](GladeRaftQ3Allocation-RemPlan-1.md) records the counterexample and exact closure obligation. Fresh Consistency/Safety review on a pinned document/source object precedes the corresponding learner algorithm change. Existing in-progress algorithms are outside that review object; original allocation remains accepted except this conflicting paragraph awaits renewed verdicts.
