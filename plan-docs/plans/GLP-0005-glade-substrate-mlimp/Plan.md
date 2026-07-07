# GLP-0005: Glade Substrate V1 — M-LIMP

Status: draft
Owner: maintainer (coordinating)
Affected modules: `glade` (wire IR, rust node, TS client), `taut` (IR
authoring + codegen), `grip-core` (base-tap share declaration +
advertisement, per GQ-5), new `grip-share` package (grip-dev side),
`grip-react-demo` (parity proof), root `plan-docs`

## Canonical checkouts (do not conflate)

| Role | Path | Branch |
| --- | --- | --- |
| grip-core (the one this plan augments) | `/Users/owebeeone/limbo/glial-dev/grip-core` (symlinked from `gryth-dev/grip-core`) | `main` @ 7d52fb1 (delayedupdates-rekey merged in) — clean, no wip2 persistence |
| taut | `/Users/owebeeone/limbo/glial-dev/taut` | as checked out |
| grip-react / demo (parity target) | `/Users/owebeeone/limbo/glial-dev/grip-react-demo` (→ `grip-react`) | on the grip-core `main` above |
| wip2 reference ONLY (superseded projector) | `/Users/owebeeone/limbo/grip-dev/grip-core` | `codex/glial-stumbling-wip2` @ fe51c29 — read, never base on |

The `grip-dev/*` tree is the wip2 world and is reference-only. All GLP-0005
work happens in the `glial-dev` checkout (and its grip-core `main`).

## Authoritative inputs

| Topic | Document |
| --- | --- |
| Substrate contract (what is being built) | `glade/dev-docs/GladeSubstrateV1.md` — §2 model, §3 shapes, §6 server, §11 M-LIMP definition |
| Security punt boundary | `glade/dev-docs/GladeGrythSecurityModelAnalysisPrompt.md` — §4 "punt without foreclosing" |
| Oplog spine guidance | `glade/dev-docs/GladeRustOrbitStrategy.md` (clock/ordering kept; content-addressed store dropped per GQ-9) |
| Wire mechanism | `taut/dev-docs/TautPlan.md`; reference IR `taut/ir/griplab.taut.py`; interop proof `trial/README.md` |
| Terminal slice (post-LIMP target) | `glade/dev-docs/GladeTerminalSliceProposal.md` |
| Superseded approach (do not resurrect) | `GladeSubstrateV1.md` §9 — the `codex/glial-stumbling-wip2` projector |

## Goal

Reach **M-LIMP** as defined in `GladeSubstrateV1.md` §11:

> Two browser (TS) sessions + one rust glade node, all localhost. One `lww`
> value and one append `log` shared between the browsers through the node.
> Node restart resumes from its store. A browser goes offline, keeps writing,
> reconciles on reconnect. `EXCHANGE` and `CHANNEL` proven via a trivial echo
> provider session.

Security is allow-all with retrofit seams present from the first frame:
principal id at `HELLO`, capability-ref slots in the envelope, no-op
enforcement hooks at every frame class.

## Success threshold

Green:
- the §11 scenario passes as a scripted, repeatable test
- Rust and TS produce byte-identical envelopes (golden corpus) and
  byte-identical folded state (fold conformance vectors)
- node restart and browser offline/reconnect lose no acknowledged ops
- equivocation (forked per-origin chain) is detected and surfaced, not silent
- the wip2 projector path is demonstrably unnecessary (P3.S4)

Yellow:
- scenario passes but only with manual steps, or IndexedDB is flaky and the
  local destination ships memory-only
- fold parity holds only for the corpus, with known divergence risks recorded

Red:
- cross-language fold parity cannot be pinned (browser-folds premise fails)
- resume/reconcile cannot be made honest without redesigning the op envelope

## Non-goals (deferred post-LIMP, in rough order)

iroh carrier and multi-node mesh; keyed async/stream bindings + canonical key
derivation; reassembler base + interest regions; grazel authority session;
MV folds (GQ-1); security enforcement (pending analyses per the security
prompt); cached-fold late-joiner optimization beyond the stretch step P1.S5.

## Workstreams and write ownership

Per plan-docs multi-agent rules — one owner per write scope:

| WS | Scope | Write scope |
| --- | --- | --- |
| WS-A contract | wire IR, corpus, conformance vectors | `glade/wire/` (or layout per P0.S1), `taut/` only via explicit contract change |
| WS-B node | rust glade node + echo provider | `glade/node/` |
| WS-C client | TS session library | `glade/client-ts/` |
| WS-D grip | grip-core share declaration + grip-share binder + demo parity | `glial-dev/grip-core` (branch off `main` @ 7d52fb1), `grip-share` package, `glial-dev/grip-react-demo` branch |
| WS-E integration | M-LIMP scenario harness, doc folds-back | `plan-docs/plans/GLP-0005-*/`, `glade/dev-docs/` updates |

WS-B/C/D start only after P0 exit (contract lock). WS-B and WS-C may proceed
in parallel against the corpus without each other.

## Phases

### P0 — Contract lock (WS-A)

Wire changes after P0 exit are explicit contract changes recorded in
`Decisions.md`, never silent edits.

| Step | Work | Exit qualifier |
| --- | --- | --- |
| P0.S1 | Module layout decision: where `glade.taut.py`, corpus, generated codecs, node and client live. Default proposal: `glade/wire/`, `glade/node/`, `glade/client-ts/`; `grip-share` beside `grip-core` (ratifies GQ-5). | Layout recorded in `Decisions.md`; directories exist. |
| P0.S2 | Author the IR: op envelope `(origin, seq, prev-hash, causal refs, payload)` per GQ-9; frames `HELLO` (session id, heads, principal-id seam), `SUBSCRIBE`/`UNSUBSCRIBE`, `OPS`, `HEADS`, `EXCHANGE` req/resp, `CHANNEL` open/data/close, `ERROR`/ack; chunking + priority-class markers; capability-ref slots present and unused. | IR passes `taut.ir.validate`; field-by-field review against `GladeSubstrateV1.md` §2/§6 recorded; security seams checked against security prompt §4. |
| P0.S3 | Golden corpus: encode/decode vectors for every frame and envelope edge cases (empty causal refs, max chunk, unicode, zero-length payload, heads with gaps). | Corpus committed; taut Python reference round-trips byte-exact. |
| P0.S4 | Rust codec via `taut.gen.rust`; TS codec (generated if available, else hand-written against corpus, as `trial/` did). | Rust and TS pass the full corpus byte-exact in CI-able test runs. |
| P0.S5 | Fold conformance vectors for `lww` and `log`: concurrent writes, lamport+origin tiebreaks, out-of-order arrival, duplicate delivery, equivocation case (expected outcome: detection, not a fold result). | Vectors committed with a reference implementation; expected outputs reviewed by hand for at least the tiebreak cases. |

**P0 exit:** corpus + vectors green in Python reference and at least one
compiled target; IR frozen for M-LIMP.

### P1 — glade-node, rust, localhost (WS-B)

| Step | Work | Exit qualifier |
| --- | --- | --- |
| P1.S1 | Crate skeleton + storage: per-`(share, origin)` append log, heads index, restart-safe. Boring storage (fs segments or sqlite — record choice in `Decisions.md`). | Store unit tests: append, scan-from-seq, heads, kill-and-restart retains acknowledged ops. |
| P1.S2 | WS carrier: framing, `HELLO`/resume, heads exchange, gap ship both directions. | Integration test: test client disconnects, writes elsewhere, reconnects, both sides converge; no acknowledged op lost. |
| P1.S3 | Subscription routing: `(share, glade id) → sessions` table, fan-out minus origin; two-class priority (small/control preempts `OPS` backfill) with chunked frames. | Two test clients see each other's ops; origin never echoed its own op; a keystroke-class frame is not queued behind a > 1 MB backfill in the scheduler test. |
| P1.S4 | Per-origin chain verification on append: `prev-hash` checked; equivocation (same `(origin, seq)`, different hash) rejected with a diagnostic frame. | Forked-log test: node rejects, surfaces `ERROR`, does not propagate either fork silently. |
| P1.S5 (stretch) | Opaque cached-fold blobs: store + serve snapshot+tail to late joiners. M-LIMP MAY fall back to full replay (retention = infinite). | Either: late-joiner snapshot+tail test green, or explicit fallback note in `State.md`. |
| P1.S6 | Echo provider session: attaches as a session, serves one `EXCHANGE` (echo request/response) and one `CHANNEL` (echo bytes). | Round-trip test through the node from a test client; correlation ids honored; channel close propagates. |

**P1 exit:** P1.S1–S4 + S6 green; node runs the localhost role in §11
end-to-end with test clients.

### P2 — glade-client-ts (WS-C)

| Step | Work | Exit qualifier |
| --- | --- | --- |
| P2.S1 | Session core: origin identity, own log (seq + prev-hash), append API. | Unit tests; envelope bytes match corpus. |
| P2.S2 | Folds `lww` + `log`. | Full fold conformance vectors pass byte-identical to reference. |
| P2.S3 | Local destination: memory store with the heads protocol; then IndexedDB. | Offline append + restart hydration tests; IndexedDB flake risk assessed (yellow path: ship memory-only, record it). |
| P2.S4 | WS destination: connect, resume, reconcile against the rust node. | Two TS sessions converge through the node on `lww` + `log` bindings. |
| P2.S5 | Partition test: scripted offline → local writes → reconnect → converge. | Test green and repeatable; no suppression-style global state anywhere in the apply path (origin filtering only). |

**P2 exit:** all steps green against the P1 node.

### P3 — grip-share bindings (WS-D)

| Step | Work | Exit qualifier |
| --- | --- | --- |
| P3.S1 | grip-core base feature (GQ-5): `share: (glade id, shape, authority?)` on tap config; uniform capture/apply hooks with per-class defaults; grok advertisement enumeration. Protocol-free — core imports no glade types; zero cost when no session attached. | Core unit tests: declared tap advertises, undeclared doesn't; hooks round-trip on AtomValueTap; no behavior change for share-free apps (existing grip-core test suite untouched and green). |
| P3.S2 | grip-share binder: walks advertised taps, binds to a session by glade id; glade-ID manifest generation + pin check (GQ-6 first ratification). AtomValueTap `value` (lww): local set → op append; remote op → apply with origin filtering. | Two grok instances converge via in-process loopback session; no echo loops; consumer code unchanged; manifest committed, rename-refactor test shows stable ids, drift fails visibly. |
| P3.S3 | Log-shaped tap binding: append + replay-from-cursor into a grip. | Replay test from cold and from cursor. |
| P3.S4 | Demo parity: `grip-react-demo` session state (count, tab, calc display, weather provider) shared between two browsers via the node — **without** the wip2 projector path. | §11 user-visible behavior reproduced; `GladeSubstrateV1.md` §8 proof target 3 checked off; projector path confirmed unused. |

**P3 exit:** demo parity green; GQ-6 ratified or amended in
`GladeSubstrateV1.md` from implementation experience (GQ-5 resolved
2026-06-13: base-tap feature).

### P4 — M-LIMP integration proof (WS-E)

| Step | Work | Exit qualifier |
| --- | --- | --- |
| P4.S1 | Full §11 scenario as one scripted harness: two browsers, node, `lww` + `log`, node restart, browser offline/reconnect, echo exchange/channel. | Single command, repeatable, CI-able; green. |
| P4.S2 | Latency/burst sanity: keystroke-class frame latency under sustained log backfill on localhost; record numbers. | Numbers recorded in `State.md`; control-frame latency under backfill stays within an interactive budget (target ≤ 10 ms localhost; measured, not asserted). |
| P4.S3 | Retro + fold-back: contract deviations folded into `GladeSubstrateV1.md`; GQ-7 ratified or amended; security seam list cross-checked against the analyses' Q8 output if available; post-LIMP phase order confirmed. | Doc updates merged; go/no-go for the post-LIMP list recorded in `Decisions.md`; plan moved toward archive or extended. |

**P4 exit = M-LIMP.** The substrate exists; post-LIMP work (iroh, keyed
bindings, reassembler, grazel, security enforcement) gets its own plan(s).

## Risks

| Risk | Mitigation |
| --- | --- |
| TS codegen maturity in taut (Rust gen exists; TS may be hand-written) | corpus-first: hand TS codec is acceptable if corpus pins it (precedent: `trial/`) |
| Cross-language fold divergence (JS number semantics, map ordering) | fold conformance vectors in P0.S5 before any client work; deterministic CBOR only in payloads |
| IndexedDB flakiness | memory destination first; IndexedDB is its own step with a recorded yellow path |
| Scope creep into security, keys, or reassembler | non-goals list; security prompt owns that scope; keyed bindings explicitly post-LIMP |
| Contract drift after P0 | wire changes only via `Decisions.md` entries; corpus is the arbiter |
| wip2 entanglement (a *different* checkout, `grip-dev/grip-core`, runs the old projector demo) | all work is in `glial-dev/grip-core` @ `main` (clean of wip2 code); `grip-dev` tree is read-only reference per `GladeSubstrateV1.md` §9 and the checkout map above |
