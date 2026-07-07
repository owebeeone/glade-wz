# The Rust + WebAssembly P2P / local-first stack — trajectory, production-readiness, and viability

Research report. Compiled 2026-06-04. Question: should glade bet on an isomorphic Rust+wasm
stack (one shared core crate → native server + browser wasm client) as the data/sync substrate
for a collaborative dev tool, versus a JS/TS (React) frontend with an adopted CRDT plus a
separate backend? All factual claims below survived 3-vote adversarial verification against
primary sources; dates and dissent are flagged inline.

## Executive summary

The post-IPFS Rust P2P ecosystem (Iroh/n0) is real and improving, but as of mid-2026 it is
**pre-1.0 and its browser story is alpha and structurally relay-only** — browsers cannot send
raw UDP from the sandbox, so iroh's hole-punching never runs in the browser; every browser
connection is relayed via WebSocket. iroh-docs is **not** a text/sequence CRDT — it is a
multi-dimensional last-write-wins keyed key-value store synced by range-based set reconciliation
(RBSR), so concurrent collaborative text editing still needs a real sequence CRDT (Yjs/yrs,
Loro, Automerge) layered on top. The Rust→wasm SPA frameworks are usable but high-churn, and the
leading one (Leptos) downshifted to "lightly maintained / feature-complete" in May 2026. The
defensible bet for this use case is a **hybrid**: a native Rust substrate (iroh peers that
hole-punch + a real CRDT for the editable text) fronted by a JS/TS client over iroh's relay or a
thin gateway — not a pure isomorphic Rust+wasm SPA today. Revisit the pure-Rust client when
browser-direct transport ships and the frameworks stabilize.

## Findings

### A. Iroh / n0 direction — why IPFS was abandoned (high confidence)

n0 abandoned the IPFS-interop-focused iroh in its **February 2023 redirection** because the
architecture could not scale: a gateway needed **~2,000 simultaneous p2p connections**, sent
**~1,000 messages to fetch a single 256KB block**, and took **seconds to resolve DHT queries**
[claim 2]. The IPFS-lineage DHT was a chronic, widely-complained-about performance problem,
corroborated by IPFS/libp2p's own issue trackers [claim 6]. Bitswap was removed as part of this
pivot [claim 5], and rather than adopt the libp2p Kademlia DHT, iroh planned to redesign content
routing purpose-built for its architecture [claim 7]. This confirms the brief's premise that the
IPFS/OrbitDB lineage is a Rust dead end.

- Sources: https://www.iroh.computer/blog/a-new-direction-for-iroh (Feb 17, 2023);
  https://github.com/n0-computer/iroh/discussions/707 (Feb 21, 2023).

### B. Iroh is pre-1.0; 1.0 was targeted for 2H2025 and had not shipped (high confidence)

In its **Road to 1.0** post (Oct 28, 2024) n0 committed to releasing iroh 1.0 in 2H2025,
confirming iroh was not stable/1.0 at that date [claim 0]. Later changelog evidence shows
1.0.0-rc.0 plus still-pre-1.0 versions (0.90, 0.94, 0.95) appearing around/after the target,
i.e. 1.0 remained unreleased through 2H2025. API stability and a finalized spec were explicitly
listed as milestones still required before 1.0.

- Source: https://www.iroh.computer/blog/road-to-1-0 (Oct 28, 2024).

### C. Browser / wasm maturity — the central risk: alpha and structurally relay-only (high confidence)

This is the make-or-break slice. As of **Oct 2024**, shipping iroh in browsers was an unfinished
pre-1.0 milestone, not a shipped capability [claim 1]. Preliminary browser support landed in
**iroh 0.32.0 (Feb 4, 2025)**, branded "Browsers Alpha" and requiring a dependency on a git
branch rather than a stable release [claim 8]. wasm compilation support became official only in
**iroh 0.33 (Feb 24, 2025)** and is "recent and still maturing" [claim 10].

The hard constraint is **structural, not a maturity gap that simply closes with time**: browsers
cannot send raw UDP packets to IP addresses from inside the sandbox [claims 11, 12], so iroh's
hole-punching cannot be ported. Therefore **all browser connections must flow through a relay
server** over WebSocket — effectively "relay only" mode, with no direct P2P / hole-punching for
browser peers [claims 9, 11, 12]. End-to-end encryption still holds (the relay is a dumb
encrypted pipe), but the headline "direct P2P hole-punching" benefit does not apply to browser
clients. Native peers still hole-punch. n0's own docs frame browser/wasm as a **complementary
add-on to native/server deployments, not a first-class standalone production platform** [claim
13]. Direct browser paths (WebTransport with `serverCertificateHashes`, or WebRTC) are
considered but unshipped.

- Sources: https://www.iroh.computer/blog/road-to-1-0;
  https://www.iroh.computer/blog/iroh-0-32-0-browser-alpha-qad-and-n0-future (Feb 4, 2025);
  https://docs.iroh.computer/deployment/wasm-browser-support;
  https://www.iroh.computer/blog/iroh-0-33-0-browsers-and-discovery-and-0-RTT-oh-my (Feb 24, 2025);
  https://github.com/n0-computer/iroh/issues/2799.

### D. iroh-docs is a keyed LWW K/V store synced by RBSR — NOT a text CRDT (high confidence)

This is the most important fact-check for the collaborative-editing requirement. iroh-docs is a
**multi-dimensional key-value document store** — entries are `(namespace, author, key)` tuples
whose value is a BLAKE3 hash (plus size and timestamp) pointing at content in the blobs store
[claims 3, 20]. It is "built on CRDTs" in the keyed sense and resolves per-key conflicts by
timestamp (**last-write-wins**); different authors' entries coexist. **It is not a text/sequence
CRDT** and does not provide character-level concurrent-edit merge semantics [claim 3].

Sync uses **range-based set reconciliation (RBSR)**, based on **Aljoscha Meyer's 2022 paper
(arXiv:2212.13567)**, cited directly by the README — not framed as "Willow" per se, though Willow
shares the same Meyer RBSR lineage [claims 4, 21]. RBSR recursively partitions entry sets and
compares fingerprints; **fully-in-sync peers exchange a single fingerprint to confirm
convergence** — i.e. true delta sync, no full re-pull, which directly satisfies the brief's
"don't re-fetch full content" requirement *for keyed data* [claim 21].

**Fact-check resolution:** "iroh-docs is built on Willow" → imprecise; it cites Meyer/RBSR
directly. "iroh-docs is a multi-writer CRDT" → true only as keyed LWW, NOT as a text/sequence
CRDT; collaborative text editing still needs Yjs/yrs/Loro/Automerge on top.

- Sources: https://github.com/n0-computer/iroh-docs;
  https://docs.iroh.computer/protocols/documents; https://arxiv.org/abs/2212.13567.

### E. CRDT / local-first data layer — Yjs/yrs family, Loro, Automerge (high confidence)

For the actual editable-text and structured-data semantics, the mature options are the
Yjs/yrs/Loro/Automerge family:

- **Cross-language interop is real in the Yjs family.** `pycrdt` provides Python bindings for
  **yrs**, the Rust port of **Yjs** (JavaScript), which aims to maintain behavior and binary
  protocol compatibility — establishing a wire-compatible **JS / Rust / Python** implementation
  family sharing one CRDT model [claim 19]. This is the strongest cross-language story and maps
  cleanly onto a hybrid (Rust native + JS browser + possible Python tooling).
- **Bundle size strongly favors plain Yjs for the browser.** Yjs ~69 KB vs ywasm ~678 KB vs
  Automerge ~1.74 MB (uncompressed); gzipped Yjs still wins decisively (~20 KB vs ~214 KB vs
  ~604 KB) [claim 16]. For a wasm-conscious browser client, a wasm CRDT carries real weight.
- **Loro's benchmarked strength is collaborative text editing.** Its headline B4 workload
  replays a character-by-character real-world trace of **259,778 ops** (182,315 insertions,
  77,463 deletions → 104,852 chars) [claim 18] — i.e. Loro is genuinely a text/sequence CRDT,
  unlike iroh-docs. Note Loro deliberately **traded away peak encoding speed for forward/backward
  compatibility ahead of 1.0**, adopting a more extensible (slower) encoding than its 2023-era
  "loro-old" build [claim 17] — a maturity signal (stabilizing the wire format) but a
  self-reported design rationale, not an independent benchmark.

Takeaway: iroh-docs handles keyed/presence/membership/append-style data well via RBSR delta
sync; the distributed **file editing** requirement needs a real sequence CRDT (Yjs/yrs or Loro)
layered on top, and the Yjs family's JS/Rust/Python wire-compat makes it the natural fit for a
hybrid.

- Sources: https://github.com/y-crdt/pycrdt; https://github.com/y-crdt/y-crdt;
  https://github.com/dmonad/crdt-benchmarks; https://loro.dev/docs/performance;
  https://github.com/zxch3n/crdt-benchmarks.

### F. Rust→wasm SPA frameworks — usable but high-churn; Leptos downshifted (high confidence on Leptos; medium on the rest)

- **Leptos is now lightly maintained.** In a **May 8, 2026 status update (issue #4707)**, creator
  Greg Johnston (gbj) stated Leptos is "not abandoned but will be lightly maintained going
  forward," that he "consider[s] Leptos complete" having "shipped every major feature on any of
  my roadmaps," and does "not expect to do significant new development" [claims 14, 15]. This
  directly refutes a glib "Leptos is production-ready and actively developed" claim: it can run
  production sites and APIs are largely settled, but core-maintainer momentum has dropped.
- Framework churn across the family is real (Leptos 0.6→0.7→0.8 with a breaking 0.9 pending),
  wasm debugging requires shims and is below mainstream JS DX, and none of Leptos/Yew/Dioxus has
  React-scale component ecosystems or hiring pool. (These specifics carry medium confidence —
  the underlying maintenance-status facts are primary-sourced and unanimous; the comparative DX
  framing draws on secondary 2026 comparison write-ups.)

- Sources: https://github.com/leptos-rs/leptos/issues/4707 (May 8, 2026).

### G. Isomorphic Rust (one core crate → native + wasm) in production (low confidence — see caveats)

The brief asked to verify production examples (1Password, Cloudflare, Tailscale, Zed) and to
check the specific misconception that **Tailscale uses isomorphic Rust+wasm**. No claim about any
of these isomorphic-Rust production examples survived 3-vote verification in this round, so this
report cannot assert them. In particular, the **"Tailscale uses this stack" claim is unverified
here and should be treated as unconfirmed** (Tailscale's core is predominantly Go; any Rust+wasm
attribution needs independent confirmation before relying on it). Treat the isomorphic-Rust
production-precedent question as **open**.

## 12–24 month trajectory

- **Iroh:** expect 1.0 / RC stabilization to land (slipping past the original 2H2025 target),
  custom protocols and discovery to mature, and incremental browser improvements. Browser-direct
  transport (WebTransport `serverCertificateHashes`, possibly WebRTC) is the key thing to watch;
  until it ships, browser iroh stays relay-only. iroh-docs remains keyed-LWW + RBSR, not a text
  CRDT.
- **CRDTs:** Yjs/yrs/pycrdt family stays the safe, wire-compatible cross-language default; Loro
  matures post-1.0 with stabilized encoding; the local-first movement continues to recommend
  adopting a proven CRDT rather than rolling one.
- **Rust→wasm frameworks:** Leptos coasts on light maintenance; Dioxus is the more actively
  evolving option; none closes the React ecosystem/hiring gap in this window.

## Risk list — betting on Rust-wasm-in-the-browser now

1. **Browser transport is alpha and structurally relay-only.** No browser hole-punching; direct
   paths unshipped. (iroh docs; 0.32 alpha Feb 2025; 0.33 Feb 2025.) [claims 8–13]
2. **Iroh is pre-1.0** with ongoing breaking-change cadence; 1.0 slipped past 2H2025. [claim 0]
3. **iroh-docs ≠ collaborative text editing.** You still need a separate sequence CRDT for file
   editing; iroh-docs alone is keyed LWW. [claims 3, 4]
4. **Framework churn / maintainer risk.** Leptos lightly maintained as of May 2026; repeated
   breaking releases; thin ecosystems vs React. [claims 14, 15]
5. **wasm debugging friction** — stack traces need shims; DX below mainstream JS.
6. **Hiring** — Rust-wasm frontend talent pool is a fraction of React's.
7. **wasm CRDT bundle weight** — a wasm CRDT (ywasm/Automerge) is far heavier than plain Yjs in
   the browser. [claim 16]
8. **No verified isomorphic-Rust+wasm production precedent** surfaced this round. [Finding G]

## Recommendation for the use case

The evidence points away from a **pure isomorphic Rust+wasm SPA** for a browser collaborative dev
tool right now. The browser transport is alpha and relay-only, the leading Rust→wasm framework
just downshifted, and there is no verified production precedent for the isomorphic pattern in
this round.

The defensible bet is a **hybrid**:

- **Native Rust substrate.** Use iroh native peers (which *do* hole-punch) + iroh-docs (RBSR
  delta sync) for keyed/presence/membership/append data — terminal logs, presence, collaborators,
  file snapshots-as-blobs. This satisfies "don't re-pull full content" for keyed data via RBSR's
  single-fingerprint convergence.
- **A real sequence CRDT for editable text.** Layer Yjs/yrs (or Loro) for concurrent file
  editing; the Yjs↔yrs↔pycrdt wire-compatible family is the cross-language sweet spot.
- **JS/TS (React) browser client** talking to the substrate over iroh's relay or a thin gateway —
  React's ecosystem, debugging, and hiring pool dominate for the frontend, and the relay-only
  browser constraint is acceptable because the browser was always going to be relayed anyway.

This matches the brief's "swappable substrate behind a declarative data-access API" framing: keep
consumers declaring *what* data and *what* share semantics; let the provider hide the iroh + CRDT
plumbing. Revisit a Rust+wasm client when browser-direct transport ships and Leptos/Dioxus
stabilize — the declarative seam means that swap costs the consumer nothing.

## Forums & people directory

(Curated from the brief; activity/centrality noted. Verification of community-channel specifics
was thin this round — treat links as starting points, not verified-current.)

- **Local-first movement:** localfirst.fm podcast; Ink & Switch (the originating research lab);
  Local-First Conf; local-first community Discord/Zulip. Central figures: Martin Kleppmann,
  Geoffrey Litt, Peter van Hardenberg.
- **Iroh / n0:** n0 Discord + the iroh blog (https://www.iroh.computer/blog) and GitHub
  (n0-computer). Maintainers: b5 (Brendan O'Brien), matheus23 (Philipp Krüger) on the wasm work.
- **CRDT libraries:** Yjs — Kevin Jahns (dmonad); Automerge — Ink & Switch / Martin Kleppmann
  et al.; Loro — Zixuan Chen (zxch3n).
- **CRDT research circle:** Martin Kleppmann; Aljoscha Meyer (Willow / RBSR, arXiv:2212.13567).
- **Recurring venues:** Hacker News, lobste.rs, r/rust, project Discords/Zulips.

## Caveats & open questions

**Caveats.**
- **Time-sensitive / fast-moving:** every iroh browser/wasm fact is dated Feb 2025–mid 2026 and
  the maturity gap may narrow quickly; re-check before committing.
- **Vendor-sourced rationale:** the IPFS-pivot performance numbers, Loro's encoding trade-off,
  and the iroh-docs/RBSR descriptions are from project-owned primary sources (appropriate for
  design-intent claims, but not independent audits).
- **Partisan benchmark:** crdt-benchmarks is maintained by Yjs's author; bundle-size figures are
  raw artifacts so the partisanship doesn't bias them, but treat workload comparisons with care.
- **Secondary framing:** the comparative Rust-frontend DX/ecosystem framing leans on 2026
  comparison write-ups, not primary docs.
- **Unverified:** isomorphic-Rust production examples (incl. the Tailscale claim) did not survive
  verification here — do not rely on them without independent confirmation.

**Open questions.**
1. Is there any *verified* production deployment using one Rust core crate compiled to both
   native and browser wasm, and what do they actually share across the boundary?
2. Has iroh shipped (or set a concrete date for) browser-direct transport via WebTransport
   `serverCertificateHashes` or WebRTC — and does it materially change the relay-only verdict?
3. What is the real operational cost and latency profile of running iroh relays at the scale this
   tool needs, given browser traffic is always relayed?
4. Which sequence-CRDT pairing (yrs+Yjs vs Loro) best fits the file-editing slice once integrated
   behind iroh-docs/blobs for keyed data, in terms of wire-compat and memory at scale?
