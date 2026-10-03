# Q4-B0 remediation 1

Date: 2026-10-03. Status: **implementation authorized; closure pending**.
Reviewed root `544c83d8cd07165cfeec2f0db64a78c8417849f3`, Glade
`c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`, discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.
[Consistency](GladeRaftB0-ReviewConsistency.md) and
[Safety](GladeRaftB0-ReviewSafety.md) returned NO-GO. Three root causes converged
blindly; Safety additionally found stop during a selected task poll. All seven
finding IDs map to four corrections in one patch. No finding is self-closed.

| Original finding | Disposition | Regression/closure obligation |
| --- | --- | --- |
| Consistency P2-1; Safety P2-1 | Preserve each carrier's actual entropy access footprint in common B0-03. Constructor failure for both; OpenRaft post-construction poisoning MUST establish no additional entropy access. Runtime fatal tests MUST target an actual clock/work access. raft-rs MUST deliberately reach a documented reset before asserting entropy failure. | Compiling RED footprint-faithful witness and extra-runtime-draw mutant; then GREEN for faithful once-only sampling and mutant rejection. Both providers retain all ten unchanged case IDs and ordinary RED. Real adapted source witnesses remain mandatory later. |
| Consistency P2-2; Safety P2-2 | Centralize shared endpoint lifecycle/current incarnation authority. All mutations MUST validate live issuer/destination, actual issued token/ownership and terminal state before changing counters, queues or RPC ownership. | Compiling RED using separately obtained endpoint handles, shared stop, stale handles and held tokens after replacement; emit/request/release/duplicate/take/control refuse with unchanged inventory. Valid replacement and all correlation tests remain GREEN. |
| Safety P2-3 | Reconcile an in-flight task result against terminal scope state before reinserting a future. | Compiling RED future stops its own scope during poll and returns Pending. Stop leaves terminal inventory, drops the future once, and saved wake cannot revive it. |
| Consistency P2-3; Safety P2-4 | Detach futures and mark terminal state under borrow, then release scheduler borrow before running arbitrary future destructors, for cancel and bulk stop. | Compiling RED parent future destructor cancels/inspects child work. Individual cancellation and stop complete without panic; both futures drop once; all inventories terminal; later work refuses. |

The correction MUST retain the public contract, four std-only allocation, upstream
sampling/algorithm semantics and no-global rules. No real engines or dependencies
are installed. Before implementation the drafter MUST record actual compiling
behavioral RED against the prior fixtures. Compiler/import failures do not count.
Run affected fixture/oracle/compiler witnesses, all 20 ordinary provider RED cases,
structural gate and all-target denied-warning Clippy. Record new actual counts,
measurements and exact root-relative inventory; do not overwrite prior RED logs.
Both originating reviewers MUST independently verify their original counterexamples
on a settled revision before closure. If the patch changes shared public interface,
architecture or lifecycle contract rather than correcting existing enforcement,
use fresh full reviewers as required by the review-loop skill, retaining original
finder closure. One architectural remediation round is used; two-round cap applies.

This accepts no automatic election, source adaptation, B0 runtime, B1/B2/B3,
canonical profile, production provider, migration or activation.
