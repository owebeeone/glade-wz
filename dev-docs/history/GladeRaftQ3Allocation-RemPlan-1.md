# Q3 allocation remediation 1 — learner bootstrap

Date: 2026-10-03. Allocation accepted source `ccab267c6b23bfec7471923098944048a7959563`; filing `c6beda7b2c94fef4060f7bc9a069b876fd2aa7d9`. The implementer found one source-confirmed contract satisfiability defect before implementing learner construction. This is not an implementation-review closure or self-acceptance.

Original §2 required every new learner initial Image already to contain the accepted join configuration/applied cut. The required actual incoming snapshot at that same join cut then hits raft-rs 0.7.0's matching term/cut fast-forward path, returning no installed snapshot Ready. The normative combined test cannot be implemented faithfully by assigning counters or pretending creation was snapshot transfer.

Disposition: allow a newly owned, externally authorized learner path to start as a private nonserving follower with the exact original authorized genesis ConfState and zero local cut, only after the existing group's committed accepted join has been validated before create. Original log or genuine snapshot catch-up installs actual join/data. Block campaign/vote/quorum/home/serving/outcomes until appropriate validated restoration; restart must revalidate existing-group admission, never infer it from emptiness or reset missing files. Already seeded receivers remain legal for other paths, but cannot claim actual installation at the same snapshot cut.

Closure: source-confirmed actual RawNode regression comparing seeded matching-cut fast-forward/no snapshot Ready with an unseeded known-group receiver's actual incoming snapshot Ready. Full Q3 combined transfer, host admission/no-create-on-denial, restart and readiness tests remain implementation obligations. No trait, normal/dev dependency, magic/schema, authority universe, public Q1 contract or production profile changes. Fresh Consistency/Safety review of the corrected pinned supplement is required because the bootstrap mutation boundary changed. One allocation remediation round; no algorithm acceptance is claimed.

The document review excludes all in-progress uncommitted Q3 algorithms and unrelated dirty members; reviewers use pinned sources. Independent algorithm work may continue, but the changed learner-bootstrap algorithm waits for renewed GO/GO. All actual matrix evidence and final dual Code/State review remain mandatory.


## Observed carrier counterexample evidence

`proof/tests/q3_seeded_snapshot.rs` first asserted that the receiver already seeded at S=5 would produce an incoming snapshot Ready at S; this compiled and failed behaviorally. The corrected source explicitly asserts absence for that seeded receiver and presence of a genuine snapshot Ready for the original-genesis nonserving receiver; 1 PASS. Exact command: `PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-adoption-proof --test q3_seeded_snapshot`. Owner independently reran 1 PASS. This uses actual RawNode/MemStorage/Snapshot and explicitly labelled carrier-only opaque data; it establishes no application validation, authorization or physical durability. The full host tests remain mandatory. No learner algorithm change has been made while the clarification awaits review.


## Independent closure

Corrected supplement at root `d4589feb02ad86b92f58686da35c8a216c370c44`, unchanged member/Gyld pins, received fresh Consistency/Safety GO with zero additional findings. Both reviewers independently traced the original source counterexample and classify it as one bounded allocation/consumer satisfiability defect; no additional architectural root cause. One allocation remediation round. This closes the declaration conflict only, not learner algorithms or any implementation matrix row. See the verbatim bootstrap reviews and final qualification-ledger disposition.
