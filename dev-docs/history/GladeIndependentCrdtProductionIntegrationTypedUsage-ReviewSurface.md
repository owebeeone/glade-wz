# IC-3A2 typed API consumer guide — SURFACE-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtProductionIntegrationTypedUsage.md` at `c3608e286272e7de412f8de52496cbfc0a355a75`, candidate for independent review; associated External Gyld IC3 capture documentation and help.

**Baseline:**

| Repository | Exact revision |
|---|---|
| Glade workspace root | `c3608e286272e7de412f8de52496cbfc0a355a75` |
| Glade | `8b0595551dd90f32da4698fff9358aa30dbf2548` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `03428fb36649d71541fd76f2471533e384be8209` |

Committed documentation was read using `git show` at these revisions. All five `HEAD` values matched the tuple at START and END.

**Date:** 2026-10-04

**Axis:** Surface — cold consumer comprehension of API ownership, lifecycle, defaults, refusal and recovery, plus the capture helper’s first-day interface. Independent, adversarial, read-only. The other axes run in parallel; nothing here relies on their reports. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0/P1/P2 findings; two nonblocking P3 documentation findings. This verdict covers the cold usage surface only and grants no production activation or physical-provider qualification.

---

## 0. Evidence base

Read:

- Canonical Surface prompt, in full.
- Root `AGENTS.md` and `AGENTS_GWZ.md`; Glade and External Gyld `AGENTS.md`. Pinned `AGENTS.md` lookups for Glial and Glade-discover returned “path does not exist”; a scoped instruction-file inventory found none there.
- `/Users/owebeeone/.claude/skills/review-loop/SKILL.md` and its canonical reviewer template.
- The complete TypedUsage guide, lines 1–103:
  - construction and limits, 5–11;
  - open/load authority, 13–19;
  - finite receive workflow and example, 21–73;
  - abandonment and persistence outcomes, 75–87;
  - storage attempts, reconstruction and close, 89–95;
  - representation/evidence boundaries, 97–103.
- External Gyld `examples/README.md`, particularly the existing source-qualified capture documentation at 59–86, IC1 overlay at 1121–1144, and new IC3 capture section at 1146–1165. No linked declaration, sidecar, graph, ledger or evidence artifact was opened.

Ran only the permitted helper help command from the External Gyld application checkout:

```text
PYTHONPATH=src:. python3 -B scripts/capture_glade_crdt_production.py --help
```

It succeeded and displayed:

```text
usage: capture_glade_crdt_production.py [-h] [--repository REPOSITORY]
                                        --output OUTPUT

IC3 allocation overlay; no runtime satisfaction or mutable sibling dependency.

options:
  -h, --help            show this help message and exit
  --repository REPOSITORY
  --output OUTPUT
```

No source inspection, builds, tests, capture execution, network requests, live actions, writes or Git mutations were performed.

## 1. Findings

### [P3-1] The unresolved-session handoff lacks a documented shutdown/reopen sequence

**Location:** TypedUsage lines 58–67, 77–87 and 95.

**Root classification:** Nonarchitectural documentation root. The evidence establishes an incomplete consumer recipe, not a missing or incorrect implementation contract.

**Violated invariant:** A cold consumer must be able to follow failure or cancellation through safe shutdown, disposal and reopening without guessing how retained ownership transfers.

**Reproduction by inspection:**

1. On the explicitly hypothetical qualified provider, `begin_ingress` succeeds.
2. Receive finishes with an error, or classification fails. The example returns `PersistResult::Refused`, dropping its local permit while telling the reader that the host retains the guard or recovery must retain loss.
3. The caller follows the close guidance. Line 95 says `Pending` applies while guards remain unresolved.
4. The guide names reopen/load/reconstruction for uncertain persistence, but supplies no corresponding handoff sequence for this error path: whether the caller retains the session, retries close, may dispose of a pending session, or must invoke a particular host recovery operation before reopening.

The same question arises after cancelling a started continuation. The prohibitions are clear; the next permitted shutdown action is not.

**Impact:** A caller using this guide alone can stall during error cleanup or guess how to relinquish exclusive store custody. The guide does not authorize erasing guards or fabricating permits, so this is a bounded usability defect rather than evidence of a destructive API defect.

**Required correction:** Add a concise outcome-to-next-action table covering receive failure, classification failure, started cancellation, uncertain persistence and `CloseResult::Pending`. State the supported disposal/reopen boundary, how original bindings remain available, and which steps belong to the injected host. If qualified recovery is unavailable at A2, state the caller’s explicit stop condition instead of implying an immediately executable recovery recipe.

**Closure check:** Retrace these paths from the revised guide alone. Each must end at a named permitted operation or an explicit unavailable-stage stop condition, without assuming that dropping a permit/session resolves durable ownership. No physical host or crash test is required to close this documentation finding.

### [P3-2] The capture helper omits its input default and generated-output workflow

**Location:** Actual `capture_glade_crdt_production.py --help`, option entries; External Gyld `examples/README.md` lines 1153–1165.

**Root classification:** Nonarchitectural documentation/help root. No incompatible command redesign is established.

**Violated invariant:** Every exposed option must state its required/default meaning; a first-day capture user must know how to select inputs, inspect the result and dispose of generated output.

**Reproduction by inspection:**

1. Run the allowed help command.
2. `--output` is visibly required, but both option descriptions are empty.
3. `--repository` is optional and absent from the IC3 README recipe. Neither surface states its default, what directory it denotes, or how a relative value is resolved.
4. The README supplies a fresh-directory creation command and existing-directory refusal, but identifies no generated entry point to inspect and no disposal guidance.

The reader therefore has to guess whether `--repository` means the Gyld fixture checkout, a Glade checkout or another root. The statement that no sibling workzone is read is useful but does not define this option.

**Impact:** The caller cannot deliberately select or diagnose the input root from the published surface. After creation, discovering the useful output requires directory exploration; cleanup requires an unstated assumption about what the generated directory owns.

**Required correction:** Give `--repository` an explicit meaning, default and path-resolution rule in help and the adjacent README. Explain the output’s fresh-directory requirement and principal inspection entry point. State how to discard only the generated capture directory. A dedicated remove command is unnecessary for ordinary generated files.

**Closure check:** Capture and assert the help text’s option descriptions/default information in the helper’s appropriate test suite, and perform a documentation-only create→inspect→discard walkthrough. The reviewer need not execute capture to verify that the revised surface answers these questions.

## 2. Invariant analysis

The principal safety and compatibility attacks failed:

- **Stage honesty holds.** Lines 3 and 15 state that assembled `open_replica` refuses with `Fault::Unavailable`. The guide does not promise an operational store, node flag or activation command. The hypothetical qualified-provider workflow is explicitly distinguished.
- **Configuration has no hidden success defaults.** Required construction inputs and finite bounds are stated. `DecodeLimits` refuses zero or nesting above 128; `IngressAuthority` zero capacity issues nothing. Evidence operations have no success default.
- **Load authority is distinguishable from decoding.** A structural Rust value cannot be promoted into validated recovery by the consumer. The authoritative floor and retained intent remain host responsibilities.
- **Receive authorization precedes consumption.** The documented ordering is begin→owned permit→factory→await→classify→settle. Foreign ownership is checked before factory invocation, and all spawned work must be joined before completion.
- **Cancellation does not become abandonment.** Factory invocation is the start boundary, including an unpolled future. Timeout, dropped future and restart cannot establish never-started work.
- **Uncertainty cannot silently become noncommit.** `Unknown` retains intent and obligations, consumes the permit and forbids replacement issuance or fabricated retry. Exact checkpoint retry depends on recovered binding.
- **Close is explicitly qualified.** The two inherited method names are acknowledged and the combined method call is given. Close does not delete terminal records or reset issuance floors. P3-1 concerns the missing consumer handoff sequence around this otherwise explicit boundary.
- **Completeness claims remain local and conservative.** Reconstruction does not assert global convergence; active/lost guards, missing history and sticky incompleteness survive reconnection and later empty inventory.
- **Capture placement and naming fit the existing family.** The helper sits among source-qualified `scripts/capture_glade_*` examples. Help and README identify an allocation overlay and deny runtime satisfaction. Existing-directory refusal provides an understandable creation boundary.
- **Cryptographic and durability claims are bounded.** Symbolic vectors, shape checks and capture allocations do not assert actual authentication, power-loss durability or two-node exchange.

## 3. Risks and next action

This review cannot establish implementation correctness or executable recovery semantics: source and runtime qualification were deliberately outside the evidence base. Actual crypto, trusted time, physical storage, duplex exchange and IC4 remain deferred.

Accounting for this initial A2 typed Surface review is **zero new architectural roots, two new nonarchitectural roots, zero remediation rounds**. The accepted semantic object’s recorded **two architectural roots, one nonarchitectural root and one merged remediation** are unchanged. No third architectural root or cap-triggering semantic counterexample was identified; neither P3 finding closes itself.

The next action is for the lane owner to file this report verbatim and retain P3-1/P3-2 as nonblocking documentation work in the verdict merge.