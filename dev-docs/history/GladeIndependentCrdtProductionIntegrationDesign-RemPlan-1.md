# IC-3A semantic design — merged remediation 1

Date: 2026-10-04. Status: **initial dual NO-GO; one merged semantic revision authorized, no implementation**.
Reviewed root `4d641dd179e5b0bd94e84df9cb334d7a21dae371`, unchanged Glade/Glial/discovery/Gyld baseline pins.
Consistency reported three P2s; Safety one P2. Their observation-persistence roots
converged independently. Four IDs represent three unique roots: two architectural,
one nonarchitectural. Initial remediation count zero; this authorizes round1.
No third architectural root is established. Historical stopped objects remain intact.

| Finding IDs | Disposition | Required closure witness |
| --- | --- | --- |
| Consistency P2-1 / Safety P2-1 — failed observation/loss writes leave no durable restart discriminator | Accept. Add a bounded, owned durable ingress/receive guard established BEFORE history-bearing consumption. Define establishment failure, partial/unknown received history, atomic settlement with exact observation/classification, every floor/slot fallback and reopen cut, drain/cancel ownership, honest completion and Y independence. Guard failure prevents consumption; unresolved guard survives old-image fallback. No process-only unavailable flag, post-failure marker, sender retry or empty inventory substitutes for durable uncertainty. Retain no kernel-clear semantics. | Semantic state/crash table and exact paired no-input/observed-loss sequence now; typed behavioral RED and real-process guard/receive/obligation/write/loss/floor/slot kill boundaries later. Peer absent after transient storage recovery MUST NOT yield false completeness. Fully settled guard/observation positive control and functional Y. |
| Consistency P2-2 — signed identity omits declaration and parameter/key schema | Accept. Bind authoritative declaration hash/version and canonical parameter/key schema hash/identity/version in authenticated creation/descriptor, trusted provision/open, proof/query and negotiated tuple. Map EVERY accepted identity-table component to an exact field or immutable versioned-profile constant. Preserve inner Op and existing Pure consumers via explicit contract/data adaptation, not silent weakened identity. | Four isolated mutations: declaration hash/version, schema identity/version. Each changes descriptor identity or refuses before open/admission/exchange; equal resource names/key bytes cannot bypass. Exact accepted-table mapping. |
| Consistency P2-3 — remote-use prerequisite omits Rust/TS/Python canonical vectors | Accept; nonarchitectural. Add stable requirement and mandatory pre-remote-use pinned three-language canonical bytes/digests for new descriptor/proof/transfer representations, frozen Op controls and malformed/noncanonical negatives. Browser activation deferral does not defer this prerequisite. | Checklist refuses missing language/vector set; three independent consumers agree on positive/edge exact bytes/hashes and reject specified malformed vectors before IC3C remote use. Pins/assertion identities required. |

Sole drafter edits design and plan as one patch. No typed/API/source/provider changes
before semantic acceptance; archive reports stay verbatim. Current accepted
contracts/types/roles/source/tests remain untouched. Specify any actual new scoped
semantic amendment exactly, with required typed interface/allocation witness.

The correction changes a material durable receive boundary and authenticated
namespace composition. Under review-loop step5, fresh full Consistency/Safety axes
are REQUIRED on the newly settled tuple; this is remediation1 of the SAME object,
not a root/cap reset. Originating reviewers MUST independently verify their own
original counterexamples/closure table on that corrected tuple as well. Reports
are peer blind per round; prior reports/merged plan are legitimate inputs.
Any reviewer-classified third new architectural root on this object triggers
STOP for owner redesign-or-accept decision. Do not draft another patch at that cap.
No self-closure, push, desk/live change or bypass to B/C.
