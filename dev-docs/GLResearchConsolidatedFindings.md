# Consolidated, Re-Verified Findings — Rust+Wasm / Local-First Substrate

Consolidation + independent re-verification of three prior research reports.
Synthesized 2026-06-04. Every decision-critical claim below was re-opened against
primary sources (official docs/blogs/repos/changelogs/papers, dates inline) and
adversarially checked — citations were **not** inherited from the reports.

**Source reports** (in `dev-docs/research/`):
- **R1** = `GLRustiesP2PStory.md` — Iroh/CRDT/Leptos, 3-vote verified, hybrid recommendation.
- **R2** = `Rust_Wasm Stack Research Prompt.md` — long-form, strongest detail, also most over-reach.
- **R3** = `deep-research-report (1).md` — a meta-review that treated the brief as "just a text
  attachment"; it explicitly **failed to retrieve primary Iroh sources** and left all Iroh-specific
  questions *unverified*. Useful only for the CRDT/local-first ecosystem layer; weakest on the
  decision-critical Iroh facts.

---

## (a) Executive summary (decision-oriented)

The decision: data/sync substrate for a local-first collaborative dev tool (append logs, file
snapshots/diffs, presence, **concurrent file editing** via deltas) behind a swappable declarative
data-access API. Candidates: (1) isomorphic Rust+wasm, (2) JS/React + adopted CRDT + backend,
(3) hybrid native-Rust substrate + JS browser client.

**Verified bottom line — re-verification *strengthens* R1/R2's hybrid recommendation and kills
R2's pro-isomorphic supporting evidence:**

1. **Iroh is still pre-1.0** — `1.0.0-rc.1` (2026-05-27); the "2H2025" 1.0 target slipped ~1 year.
   Breaking-change cadence is real. **[Verified]**
2. **Iroh's browser transport is alpha and *structurally* relay-only** — browsers can't send raw UDP,
   so hole-punching never runs in-browser; all browser peers relay over WebSocket. Browser-**direct**
   transport (WebTransport `serverCertificateHashes` / WebRTC) is **unshipped and effectively
   dormant**. This is a sandbox constraint, not a maturity gap that simply closes. **[Verified]**
3. **`iroh-docs` is a last-write-wins keyed K/V store synced by RBSR — NOT a text/sequence CRDT.**
   Concurrent file editing requires a real sequence CRDT (yrs/Yjs or Loro) layered on top, with its
   bytes stored inside iroh-docs values. **[Verified]**
4. **The "iroh-docs is built on Willow" claim is FALSE/imprecise** (R2 asserted it; **refuted**).
   iroh-docs cites Meyer's RBSR paper directly and does **not** depend on the `iroh-willow` crate
   (which is a separate, stalled `0.0.1` experiment). Willow and iroh-docs are RBSR *siblings*. **[Verified]**
5. **No verified isomorphic-*Rust*-core→native+wasm production precedent exists.** Every example R2
   cited decomposes: 1Password = Rust core + React UI (typeshare); Figma = isomorphic *C++* (not Rust);
   Zed = native Rust GPUI + wasm-for-extensions-only; Tailscale = Go core (the "Tailscale uses
   isomorphic Rust+wasm" claim is **false**). **[Verified / Refuted]**
6. **The leading Rust→wasm SPA framework (Leptos) downshifted to "lightly maintained / complete"**
   on 2026-05-08 (creator's own words). Dioxus is the more active option; Yew cadence is ~stale. **[Verified]**
7. **CRDT cross-language wire-compat is real**: yrs (Rust) documents binary-protocol compat with Yjs
   (JS); pycrdt is a yrs binding (Python). This Yjs/yrs/pycrdt family is the safe hybrid default. **[Verified]**

**Decision implication:** the evidence points **away from a pure isomorphic Rust+wasm SPA** and
**toward the hybrid** (native Rust substrate: iroh native peers that hole-punch + a sequence CRDT for
editable text; JS/React browser client over iroh's relay or a thin gateway). The browser was always
going to be relayed, so the relay-only constraint is acceptable for the browser slice. The swappable
declarative seam means a future Rust+wasm client swap costs consumers nothing — revisit when
browser-direct transport ships and the frameworks stabilize.

---

## (b) Consolidated findings

Format: **claim — grade — primary source(s) + date — reports asserting / agreement.**
Grades: **Verified** (multiple independent primary sources) · **Single-source** · **Vendor-sourced**
(project-owned; fine for design intent, flagged) · **Contested** · **Refuted** · **Stale** (dated, may move).

### Iroh — release & direction

1. **Iroh is pre-1.0; 1.0 targeted for 2H2025 and slipped ~1 year; now at release-candidate.**
   — **Verified** — [iroh.computer/blog/road-to-1-0](https://www.iroh.computer/blog/road-to-1-0) (Oct 28, 2024, "committing to releasing iroh 1.0 in the second half of 2025"); crates.io / GitHub releases show `1.0.0-rc.1` (2026-05-27), `1.0.0-rc.0` (2026-05-07), 0.9x series through 2025–2026 — 1.0 **stable** not yet shipped as of 2026-06-04.
   — R1 (1.0 unshipped), R2 (pre-1.0, 0.x churn). **Agree.** (R3: unverified.)

2. **n0 abandoned IPFS/Bitswap/DHT for performance reasons; concrete numbers confirmed.**
   — **Verified (vendor-sourced numbers)** — [iroh.computer/blog/a-new-direction-for-iroh](https://www.iroh.computer/blog/a-new-direction-for-iroh) (Feb 17, 2023): gateway "needs at least 2,000 simultaneous p2p connections"; "~1,000 messages to retrieve a single 256KB block"; DHT queries "on the order of seconds." Bitswap removed (discussion [#707](https://github.com/n0-computer/iroh/discussions/707): "Bitswap is already gone"); declined libp2p Kademlia in favor of purpose-built content routing.
   — R1 (numbers, claims 2/5/6/7), R2 (Bitswap/DHT rationale + async-Rust angle). **Agree.**
   — *Correction:* the numbers live in the **blog**, not in discussion #707 (R1 attributed to both). Numbers are n0's own (design rationale, not independent audit).

3. **Async-Rust/embedded-DB (redb, `!Send`, async-Drop) friction was a real engineering driver.**
   — **Verified (vendor-sourced)** — [iroh.computer/blog/async-rust-challenges-in-iroh](https://www.iroh.computer/blog/async-rust-challenges-in-iroh) (Jul 31, 2024). Genuine; but it's about embedded-DB-in-async-Rust generally — the "caused by the IPFS model" framing in R2 is loose/contextual, not stated.
   — R2 only.

### Iroh — browser / wasm (the central risk)

4. **Browser support landed alpha: 0.32.0 "Browsers Alpha" (Feb 4, 2025); wasm compile official 0.33.**
   — **Verified** — [iroh-0-32-0…](https://www.iroh.computer/blog/iroh-0-32-0-browser-alpha-qad-and-n0-future) (Feb 4, 2025); [iroh-0-33-0…](https://www.iroh.computer/blog/iroh-0-33-0-browsers-and-discovery-and-0-RTT-oh-my) (**Feb 25, 2025** — R1 said Feb 24, off by one day).
   — R1, R2. **Agree.**

5. **Browser transport is *structurally* relay-only: no raw UDP in the sandbox → no in-browser
   hole-punching → all browser peers relay over WebSocket (E2E-encrypted; relay is a dumb pipe).**
   — **Verified** — [docs.iroh.computer/deployment/wasm-browser-support](https://docs.iroh.computer/deployment/wasm-browser-support): "All connections from browsers… need to flow via a relay server" because browsers "don't support sending UDP packets… inside the browser sandbox." Corroborated by the 0.32 blog ("won't try to hole-punch… not possible in browsers without deeply integrating with WebRTC"). Framed as sandbox-structural, not a temporary gap.
   — R1 (claims 9/11/12, "structural"), R2 (§3.1, "draconian sandbox"). **Agree.** (R3: supports browser caution only at a *general*, non-Iroh-specific level.)

6. **Browser-DIRECT transport (WebTransport `serverCertificateHashes` / WebRTC) is NOT shipped;
   speculative/dormant.**
   — **Verified** — docs page lists them only as things n0 "may" do; tracking issue [#2799](https://github.com/n0-computer/iroh/issues/2799) is **closed/done but covered only the relay-only wasm milestone** (0.33), not direct transport. No release through `1.0.0-rc.1` ships it.
   — R1 ("considered but unshipped"), R2 ("stalling Phase 3… deeply entrenched W3C constraints"). **Agree.**
   — *Caveat:* recent comment timeline on #2799 could not be fully extracted; no docs/release evidence of direct transport landing.

7. **n0 frames browser/wasm as a complement to native/server, not a standalone production platform.**
   — **Verified** — docs: "most applications will use iroh browser support as an additional feature to
   complement existing deployments to desktops, native apps or servers." Roadmap context: [iroh-and-the-web](https://www.iroh.computer/blog/iroh-and-the-web) (Jul 1, 2024 — predates the alpha; intent not shipped state).
   — R1 (claim 13), R2. **Agree.**

### Iroh-docs — semantics (decisive for the editing requirement)

8. **`iroh-docs` is a multi-dimensional LWW keyed K/V store — entries are `(namespace, author, key)`
   → BLAKE3 hash + size + timestamp; per-(author,key) conflicts resolve newest-timestamp-wins. It is
   NOT a text/sequence CRDT (no character-level merge).**
   — **Verified (with nuance)** — [iroh-docs README](https://github.com/n0-computer/iroh-docs) + `src/sync.rs` (`InsertError::NewerEntryExists`, "newer than an existing entry for the same key and author"); [docs.iroh.computer/protocols/documents](https://docs.iroh.computer/protocols/documents) (`single_latest_per_key()` collapses across authors at *query time*).
   — R1 (claim 3, "keyed LWW, not text CRDT"), R2 (Fact-Check 2, "LWW K/V map, architecturally misleading"). **Agree.**
   — *Nuance:* LWW is per **(namespace, author, key)**, not bare key — different authors' writes to the
     same key both persist; the "latest" collapse is a query helper, not a storage merge. Also: docs call
     documents "built on CRDTs," so frame the negative as **"LWW register store, no sequence/text merge,"**
     not a blunt "not a CRDT."

9. **Sync = Range-Based Set Reconciliation (RBSR), Aljoscha Meyer 2022 (arXiv:2212.13567);
   fully-in-sync peers exchange a single fingerprint → true delta sync, no full re-pull.**
   — **Verified** — iroh-docs README (cites Meyer + links arXiv:2212.13567); docs.iroh.computer/protocols/documents ("fully-in-sync peers only need to exchange a single fingerprint to confirm it"). Satisfies the brief's "don't re-pull full content" *for keyed data*.
   — R1 (claims 4/21), R2 (§2, with the logarithmic-round-trip mechanics). **Agree.**

### CRDT / local-first data layer

10. **Cross-language wire-compat is real in the Yjs family: yrs (Rust) documents *binary-protocol*
    compatibility with Yjs (JS); pycrdt is a yrs binding (Python).**
    — **Verified** — [y-crdt README](https://github.com/y-crdt/y-crdt): "aims to maintain behavior and binary protocol compatibility with Yjs, therefore projects using Yjs/Yrs should be able to interoperate." [pycrdt](https://github.com/y-crdt/pycrdt): "CRDTs based on Yrs."
    — R1 (claim 19), R2 (§6.1), R3 (Yjs network-agnostic). **Agree.**
    — *Nuance:* compat is documented at the **yrs↔Yjs** layer; the pycrdt↔Yjs leg is *inferred* (pycrdt binds
      yrs). "aims to maintain" = best-effort intent, not a guarantee; yrs is a re-implementation, not an FFI port.

11. **Browser bundle weight favors plain Yjs decisively: Yjs 69 KB / ywasm 678 KB / Automerge 1.74 MB
    (raw); gz 20 KB / 214 KB / 604 KB.**
    — **Verified, exact — but PARTISAN source** — [dmonad/crdt-benchmarks](https://github.com/dmonad/crdt-benchmarks) (byte-exact match). Maintainer = Kevin Jahns, **author of Yjs**; bundle-size is an axis Yjs wins by design. Numbers are raw artifacts (partisanship doesn't bias them) but workload comparisons in the same suite should be read with care.
    — R1 (claim 16), R2 (§6.1). **Agree.**

12. **Loro is a genuine text/sequence CRDT (Fugue), 1.0 shipped 2024-10-21; B4 benchmark replays
    259,778 ops.**
    — **Verified** — Loro 1.0 changelog/loro.dev (v1.0.0, 2024-10-21; npm loro-crdt@1.0.7 2024-10-23);
      README "Text Editing with Fugue" (arXiv:2305.00583); [loro.dev/docs/performance](https://loro.dev/docs/performance) "259,778 operations totally."
    — R1 (claims 17/18), R2 (§6.2). **Agree on 1.0 + Fugue + B4.**
    — *Single-source/inferred:* (a) **Peritext** lives in Loro's *separate* `crdt-richtext` repo — whether 1.0's
      shipped rich-text uses it isn't stated in the core README (R2 asserts Peritext confidently → downgrade to
      single-source). (b) R1's "traded encoding speed for fwd/back compat before 1.0" is **inferred**, not a
      documented Loro statement (the 1.0 note says "stable encoding format"; the speed delta is consistent but
      the causal framing is not primary).

13. **Automerge: current (v3.x, 2026), Rust core via FFI to JS+Wasm+C, columnar storage,
    transport-agnostic (`automerge-repo`).**
    — **Verified** — [github.com/automerge/automerge](https://github.com/automerge/automerge) (v3.2.6, 2026-04-22; README "core Rust… exposed via FFI in javascript+WASM, C"); [automerge.org binary-format spec](https://automerge.org/automerge-binary-format-spec/) (columnar); [automerge-repo](https://github.com/automerge/automerge-repo) (pluggable networking/storage).
    — R2 (§6), R3 (Verified, transport-agnostic). **Agree.**

### Rust→wasm SPA frameworks

14. **Leptos downshifted to "lightly maintained / complete" on 2026-05-08 (creator's own words).**
    — **Verified** — [leptos-rs/leptos#4707](https://github.com/leptos-rs/leptos/issues/4707) "Status Update - May 2026" by gbj (Greg Johnston), May 8 2026, verbatim: "Leptos is not abandoned but will be lightly maintained going forward," "I consider Leptos complete," "shipped every major feature on any of my roadmaps," "do not expect to do significant new development." Latest stable 0.8.x; 0.9.0-alpha tagged 2026-05-19.
    — R1 (claims 14/15, primary-sourced, unanimous). R2 ("Leptos production-ready, with caveats" — *partly
      superseded* by this; APIs settled but maintainer momentum dropped). **R1 is the more current/accurate read.**

15. **Dioxus actively maintained (more so than Leptos now); Yew cadence ~stale.**
    — **Verified / Single-source** — Dioxus releases through May 2026 (v0.7.9 2026-05-08, v0.8.0-alpha 2026-05-19); Yew latest v0.23.0 (2025-03-10, >1yr stale).
    — R1 ("Dioxus more actively evolving"), R2 (framework comparison table). **Agree directionally.**
    — *Caveat:* the comparative DX/bundle-size/SSR-maturity table in R2 (§4.1) leans on **secondary 2026
      comparison write-ups** (reintech/rustify) — treat the comparison framing as secondary, the
      maintenance-status facts as primary.

### Isomorphic Rust in production

16. **No verified isomorphic-*Rust*-core→native+wasm production precedent. Every cited example
    decomposes.**
    — **Verified (the decomposition) / Refuted (the precedent)** —
      • **1Password**: Rust core ("Brain") compiled to wasm for crypto/filling; **UI is TypeScript/React**;
        [typeshare](https://github.com/1Password/typeshare) generates TS types from Rust structs. Rust core + non-Rust UI.
      • **Figma**: renderer is **C++→wasm** ([figma.com/blog/webassembly-cut-figmas-load-time-by-3x](https://www.figma.com/blog/webassembly-cut-figmas-load-time-by-3x/)); same C++ core also native — a **genuine isomorphic same-core native+wasm pattern, but in C++, not Rust.**
      • **Zed**: native Rust desktop on **GPUI**; wasm only for sandboxed extensions ([zed.dev/blog/zed-decoded-extensions](https://zed.dev/blog/zed-decoded-extensions)). Not a browser app.
    — R1 (Finding G: "none survived verification — open"), R2 (§5: claimed them as precedents but its *own
      detail* shows the decomposition), R3 ("unsupported in retrieved sources"). **R1/R3 correct; R2's
      framing over-reaches but its facts agree.**
    — *Note for design intent:* Figma proves the **one-core→native+wasm pattern works in production** — just
      in C++. The 1Password "Rust core + typeshare'd TS UI" is the **closest validated template for the hybrid.**

17. **"Tailscale uses isomorphic Rust+wasm" — FALSE.**
    — **Refuted** — Tailscale core is **Go (~95%)** ([github.com/tailscale/tailscale](https://github.com/tailscale/tailscale)); [tailscale-rs](https://github.com/tailscale/tailscale-rs) is an explicitly "preview, experimental… unstable and insecure" tsnet **binding**, **DERP-relay-only, no NAT traversal** ([blog](https://tailscale.com/blog/tailscale-rs-rust-tsnet-library-preview), Apr 15 2026) — and a *native* binding, not a browser-wasm stack.
    — R1 ("unverified, treat as unconfirmed"), R2 (Fact-Check 4: FALSE), R3 ("unsupported"). **Agree** —
      R2/R3 land it as outright false; R1 was merely unable to confirm. **Now firmly refuted.**

---

## (c) Contradictions & open gaps

**Resolved contradictions between the reports:**

- **"iroh-docs built on Willow" — R2 said TRUE (via `iroh-willow` crate); R1 said imprecise (RBSR
  sibling, not Willow).** → **R1 is correct; R2 refuted.** iroh-docs 0.100.0 has **no `iroh-willow`
  dependency**; `iroh-willow` is a separate `0.0.1` experiment last published 2025-02-07 ("not released
  yet"), repo-co-located but unused by iroh-docs. iroh-docs cites Meyer/RBSR directly. Willow and
  iroh-docs share the RBSR lineage but are siblings, not parent/child.

- **Tailscale precedent — R1 "unconfirmed" vs R2/R3 "false."** → **Refuted (false)**, with primary
  evidence. R1 was under-confident here.

- **Leptos — R2 "production-ready" vs R1 "lightly maintained."** → Not strictly contradictory but R1
  is **more current**: APIs are settled (R2's point stands) *and* core-maintainer development has
  stopped (R1's point, now primary-verified). Use R1's framing.

**Open gaps / unanswered decision-critical questions:**

1. **No verified isomorphic-*Rust* production precedent** for one core crate → native + browser-wasm
   sharing real logic. Closest analogs are C++ (Figma) or "Rust core + non-Rust UI" (1Password). The
   pure-Rust-UI pattern remains **unproven at production scale** — treat as a real risk, not a solved path.
2. **Browser-direct transport date** — n0 has **no committed date** for WebTransport
   `serverCertificateHashes` / WebRTC. Until it ships, browser iroh stays relay-only. This is the single
   most important thing to watch; it gates any future pure-Rust-wasm client.
3. **Relay operating cost/latency at this tool's scale** — unquantified by all three reports. Browser
   traffic is always relayed; the real $/latency profile of running iroh relays is unknown and should be
   prototyped before committing.
4. **Sequence-CRDT choice for the editing slice (yrs/Yjs vs Loro)** — unresolved. yrs/Yjs/pycrdt wins on
   wire-compat + bundle weight + cross-language; Loro wins on benchmarked text/tree perf and movable-tree
   semantics (good for file-tree sync) but its headline numbers are vendor-sourced (see below). Needs a
   hands-on bake-off behind the iroh-docs/blobs seam.
5. **GuardianDB as a production iroh-docs user** (R2's one concrete adoption proof) was **not
   independently re-verified** this round — treat as single-source/unconfirmed.
6. **R3 contributes ~nothing on the Iroh-specific decision** — it never retrieved primary Iroh sources
   and left every Iroh question "unverified rather than false." Do not rely on R3 for the substrate
   decision; its value is limited to the CRDT/local-first ecosystem framing (e.g. correcting "Yjs is
   centralized-first" → Yjs is network-agnostic).

---

## (d) Claims that did NOT survive re-verification

- ❌ **"iroh-docs is built on Willow / uses the `iroh-willow` crate"** (R2, Fact-Check 1: "Verdict TRUE").
  **Refuted** — no `iroh-willow` dependency in iroh-docs; it cites Meyer/RBSR directly. iroh-willow is a
  stalled 0.0.1 experiment.

- ❌ **"Automerge synchronizes via RIBLT / rateless set reconciliation"** (R2, §6.1 table + §6.2).
  **Refuted** — Automerge production sync uses a **Bloom-filter + heads** protocol (automerge.org sync
  docs; `sync/bloom.rs`, 1% FPR). RIBLT is an unrelated MIT research protocol (arXiv:2402.02668) used in
  the literature to *attack* Automerge's Bloom-filter sync, not to implement it.

- ❌ **"Loro processes 250,000 operations 19× faster than Yjs (66 ms vs 1,270 ms)"** (R2, §6.2).
  **Contested / mislabeled** — the 66 ms vs 1,270 ms figures are real on loro.dev/docs/performance but
  belong to **B4×100 (~26 million ops)**, *not* 250 k ops. B4 itself (259,778 ops) shows different
  numbers. Ratio plausible, dataset label wrong by ~100×, and the source is **vendor-sourced (loro.dev)**.

- ❌ **Per-operation memory figures "Yjs ~30 B / Automerge ~50 / Loro ~25 / diamond-types ~12"** (R2,
  §6.1 table). **Not located in any primary source** — appear to derive from a secondary blog (taskade-style
  aggregation). Primary sources give only non-comparable metrics (version-vector sizes, document sizes).
  Drop or attribute as secondary.

- ❌ **"Tailscale uses isomorphic Rust+wasm"** (brief's premise; R1 left unconfirmed). **Refuted** — Go
  core; tailscale-rs is an experimental DERP-only native binding.

- ❌ **Isomorphic-Rust production examples (1Password / Figma / Zed / Cloudflare) as evidence for a
  pure-Rust-UI isomorphic stack** (R2, §5 framing). **Refuted as stated** — each decomposes to Rust-core +
  non-Rust-UI, native-only, or C++ (not Rust). R2's own detail supports the decomposition; only its
  headline framing over-reached.

- ⚠️ **R2's "Loro uses Peritext in 1.0"** (§6.2) — **downgraded to single-source/unconfirmed**: Peritext
  is in Loro's separate `crdt-richtext` repo; the core README confirms Fugue, not Peritext-in-1.0.

- ⚠️ **R1's "Loro traded encoding speed for fwd/back compat before 1.0"** — **downgraded to inferred**:
  consistent with benchmark deltas but not a documented Loro causal statement.

---

## (e) Caveats (read before committing)

- **Time-sensitive / fast-moving.** Every Iroh browser/wasm fact is dated Feb 2025 – June 2026 and the
  maturity gap may narrow quickly. Iroh is at `1.0.0-rc.1` (2026-05-27) — **re-check release status and
  browser-direct transport before committing.**
- **Vendor-sourced rationale (fine for design intent, not independent audit):** the IPFS-pivot numbers
  (2000/1000/256 KB/seconds), the iroh-docs/RBSR single-fingerprint claim, Loro's 1.0/encoding notes, and
  **all Loro performance numbers** come from project-owned primary sources. Good for "what they designed
  for," not for "independently measured."
- **Partisan benchmark:** `crdt-benchmarks` (bundle sizes) is maintained by Yjs's author. Raw byte counts
  are unbiased; workload/perf comparisons in the same suite are not neutral. Loro's perf page is likewise
  Loro's own — and at least one headline figure is mislabeled (see (d)).
- **Secondary framing:** the Rust-frontend DX/ecosystem/hiring comparisons (R2 §4) and the per-op memory
  table lean on 2026 comparison blogs, not primary docs. The maintenance-status facts (Leptos #4707, Dioxus/
  Yew release cadence) are primary; the *comparative* framing around them is secondary.
- **R3 reliability:** R3 mostly reviewed *the brief itself* and did not perform the Iroh primary-source
  sweep — its "unverified" verdicts reflect its own retrieval gap, not the state of the world. The Iroh
  facts above (R1/R2 + this round's re-verification) supersede R3's "open" status on those points.
- **One date nit:** iroh 0.33 released **Feb 25, 2025** (R1 said Feb 24). Immaterial to the decision.
- **Unverified adoption proof:** GuardianDB (R2's production iroh-docs user) was not re-verified this round.

---

### Appendix — grade tally

| Finding | Grade |
|---|---|
| 1 Iroh pre-1.0, 1.0 slipped to rc | Verified |
| 2 IPFS/Bitswap/DHT pivot + numbers | Verified (vendor numbers) |
| 3 Async-Rust/redb friction | Verified (vendor) |
| 4 Browser alpha 0.32/0.33 | Verified |
| 5 Structurally relay-only browser | Verified |
| 6 Browser-direct transport unshipped | Verified |
| 7 Browser = complement, not standalone | Verified |
| 8 iroh-docs = LWW keyed K/V, not text CRDT | Verified (nuanced) |
| 9 RBSR / Meyer 2022 / single-fingerprint | Verified |
| 10 yrs↔Yjs binary-protocol compat | Verified (pycrdt leg inferred) |
| 11 Bundle sizes | Verified, partisan |
| 12 Loro real CRDT / Fugue / 1.0 / B4 | Verified (Peritext+tradeoff downgraded) |
| 13 Automerge v3 / columnar / FFI / repo | Verified |
| 14 Leptos lightly maintained (#4707) | Verified |
| 15 Dioxus active / Yew stale | Verified/Single-source |
| 16 No isomorphic-Rust precedent | Verified (decomposition) / Refuted (precedent) |
| 17 Tailscale isomorphic Rust+wasm | Refuted (false) |
| — "iroh-docs built on Willow" | **Refuted** |
| — "Automerge syncs via RIBLT" | **Refuted** |
| — "Loro 250k ops 19× faster" | **Contested / mislabeled, vendor** |
| — Per-op memory table | **Not in primary source / secondary** |
