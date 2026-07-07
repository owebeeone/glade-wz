# Decisions — GLP-0004

Plan-local decision record and the reasoning arc that produced this spike.
Date: 2026-06-05

## D-01: Use iroh for connectivity only, not as a data layer

**Decision.** iroh provides identity + NAT traversal + relay. The data/app layer
is the **existing WSS LCS protocol**, forwarded over an iroh bi-stream. We do
**not** use iroh-docs/blobs/gossip as the terminal/log data model.

**Why.** Treating iroh-docs as an append-log accreted adapters, each a symptom of
abstraction misfit (iroh-docs is an LWW keyed **set** + RBSR, not an ordered log):
- seq-key encoding `{prefix}/{seq:020}` to fake ordering;
- an out-of-order **assembler** in the consumer (blobs arrive unordered over the
  relay) — built and verified this session;
- an **ephemeral** MemStore (the wasm build drops the redb/fs store) → no cache,
  re-sync from zero every subscribe;
- blob-per-chunk overhead (60 chunks × ~60 B = 60 ContentReady round-trips).

Forwarding a reliable QUIC stream makes ordering **free** and moves cache/resume
to a tractable **app-protocol** concern ("I have up to offset N, send from
there") instead of fighting blob sync. Adapter count → 0.

## D-02: Keep the browser out of the P2P layer

**Decision.** The browser talks `ws://localhost` to a small **native agent**.
No iroh in the browser.

**Why.** Every hard edge this session was "the browser can't X": no UDP/QUIC
(relay-only), no threads (`spawn_local` vs OS threads), no fs (MemStore vs redb),
~1.6 MB brotli of crypto/QUIC it otherwise wouldn't need. The browser is the
lowest-capability peer and dominates the architecture; design it first. WSS to a
local agent is trivial and ships ~0 extra bytes.

## D-03: Per-connection forwarding, not a full overlay VPN

**Decision.** `ssh -L`-over-iroh / dumbpipe granularity. No tun device, no
virtual IPs, no userspace netstack.

**Why.** GripLab needs to forward *specific* things (the LCS WSS connection, test
ports), not arbitrary host traffic. Per-connection forwarding needs no
privileges and no netstack — the difference between "build a pipe" (a week) and
"build Tailscale" (a year). The "virtual VPN" is the mental model; the build is a
pipe. n0's `dumbpipe` is the existence proof.

## D-04: Discovery is the hub roster; iroh is for traversal

**Decision.** The hub holds the roster and hands out node-ids/tickets. iroh's
own discovery (pkarr) and any DHT are unused.

**Why.** The target is a **bounded** huddle, not the open internet. "Who is in my
huddle" is a roster. mDNS is LAN-only and the persona is distributed-home (not
same LAN); the DHT is the known dead end (archived rust-ipfs; iroh dropped it).
The genuinely hard part for distributed-home is **reachability/NAT traversal**,
not discovery — and that is exactly what iroh is for.

## D-05: Why not the alternatives

- **Pure hub + SSH tunnels (no iroh).** Reaches the same place for *connectivity*
  (outbound to a public hub always works through NAT), and SSH is a great
  developer-audience primitive (identity + transport + firewall-punch). But: the
  current "hub SSH **into** collaborator" assumes **inbound** reachability, which
  home NAT denies — the direction must flip to outbound, and you then hand-roll
  persistent reverse tunnels (reconnect, port allocation, Windows sshd) per peer.
  iroh eats that ops surface and, over QUIC, avoids TCP-over-TCP meltdown.
- **iroh/libp2p as full P2P incl. browser + CRDT.** Over-engineered for a
  hub-centric team tool: pays for hubless/offline-first/DHT properties the
  persona doesn't have, and drags the iroh stack into the browser.
- **libp2p specifically for discovery.** Its edge over iroh is the discovery menu
  (mDNS/DHT/gossipsub) — i.e. the LAN-only or dead-end parts. Choosing it for
  discovery is choosing it for its weakest dimension here.

The tell: optimizing the iroh path this session converged on "run the relay on
the hub and SSH-tunnel every node to it" — i.e. a central router over SSH. When
the sophisticated path collapses into the simple one, the simple one was the
answer; this spike formalizes the simple one with iroh doing the NAT work.

## D-06: Phase 1 v2pn is a loadable module, not a standalone daemon

**Decision.** The v2pn (iroh tunnel + per-connection forwarder) ships as a
**loadable native Rust module (pyo3)**, loaded by the LCS — repurposing the
existing `griplab_core_py` module mechanism from "iroh-docs producer" to
"v2pn tunnel." It is **not** a standalone daemon / separate process for Phase 1.

**Why.** A spike tests the core claim (forward WSS over an iroh tunnel through
NAT), not the process/deploy model. The loadable path reuses what already works
(maturin/abi3, the per-platform artifact store, `native_loader`, copy-on-startup,
bootstrap shipping) and keeps lifecycle under the LCS — the fastest route to the
answer.

**Retained:** pyo3 / maturin / abi3 / `native_loader` / artifact shipping. The
iroh-docs *producer* role is replaced by the *tunnel* role inside the same module.

**Still dropped:** `griplab-core-wasm`. The browser talks `ws://localhost` to its
**local LCS** (which loads the v2pn module); no iroh in the browser, so the wasm
consumer is unnecessary regardless of the module-vs-daemon choice.

**Deferred (explicitly not this spike):** standalone daemon + separate process +
supervision; LCS-in-Rust. Both are productionization choices to revisit *after*
the core claim holds. The earlier "drop pyo3, go standalone daemon" framing was
overreach for a spike — recorded here so we don't relitigate it.

## D-07: The v2pn replaces the SSH tunnel connectivity — any node ↔ any node

**Decision.** With the v2pn validated (`scratch/iroh-tcp-test`), iroh becomes the
connectivity layer for the whole fleet and the SSH reverse-tunnel machinery is
**retired**:

- **Gone:** the hub SSH-ing *into* each collaborator + persistent per-peer
  `-R`/`-L` tunnels to route the control plane — the fragile part of the bootstrap
  (quoting bugs, "remote port forwarding failed", and the inbound-reachability
  assumption that never worked for home NAT).
- **Instead:** every node runs iroh, dialing **outbound** to the hub-hosted relay.
  Any node reaches any other by `EndpointId` — hole-punched direct when possible,
  via the relay otherwise. Forwarded channels (LCS WSS, test ports) ride iroh
  streams.

**Connectivity is any-to-any and guaranteed** *because* the relay is the backstop:
every node reaches it outbound (always open), so even peers that can't hole-punch
still connect. Direct LAN-local paths are proven (same-LAN, cross-platform);
cross-NAT *direct* success across the internet is the only unmeasured item, and
it's relay-backstopped — so **no connectivity pressure-test is needed to proceed**
(it's a hub-bandwidth question, Open Question #1, not a connectivity risk).

iroh also gives **authentication** for free — every connection is mutually
authenticated by `EndpointId` (the node's public key). It does **not** give
authorization (D-08). SSH may still do initial *provisioning* (get the agent onto
a box, start it) — separate from connectivity, out of scope here.

## D-08: Add an authorization layer (membership), hub-brokered

**Decision.** Authentication is iroh's (you cryptographically know *who* the peer
is). The product adds **authorization**: a node decides whether an incoming
`EndpointId` is *allowed* to connect / reach a session. The **hub holds the
per-huddle roster** of permitted node ids; nodes enforce it per connection.
Membership *establishment* (how a node joins the roster — ssh-key enrolment, a hub
invite/token, etc.) is the trust-bootstrap part, to be designed.

This is the substantive remaining work once the v2pn lands: **connectivity is
solved; access control is the build.** Likely its own plan.

## Relationship to other plans

- **`GLP-0001` / `Phase1Libp2pTest`** (transport-as-data-layer): direction
  superseded for the terminal logstream. Connectivity learnings retained.
- **`GLP-0003` rust-orbitdb** (CRDT data layer): this approach **reduces** the
  need for a CRDT log for terminal output (the log rides a reliable stream + an
  app-level resume protocol). It does NOT auto-supersede GLP-0003 — collaborative
  *editing* (concurrent text) may still want a CRDT. Flagged for the maintainer
  to reconcile; not changed here.

## Session learnings worth keeping (condensed)

1. The "nothing streams" blocker was a **reactive-framework** bug
   (`grip-core` `delayedUpdates` never cleared → home-param changes never re-keyed
   the tap), **not** the transport. Debug top-down before bottom-up.
2. Instrumentation is a system that lies: `console.debug` unhooked, wrong grip
   context, hook lost on reload, reading the SESSIONS snapshot vs the live grip.
   Verify the instrument before trusting a negative.
3. Environment is a confounder: two vites on one port (IPv4-service/IPv6-mock)
   silently flipped data mode between calls. Pin one deterministic target.
4. Perf, measured (`scratch/iroh-tcp-test`): relay connect n0 ~4.5 s → local relay
   **~0.2 s**; direct UDP + LAN-local routing reliable & cross-platform; latency
   adds no tax (p50 ~3.6 ms on 5 GHz). **Throughput is link/radio-bound, not iroh**
   — iroh ≈ pure TCP in every direction (up to 28.6 MB/s on a good link); the low
   1.9–3 MB/s figures were a 2.4 GHz misconfig + a Pi's weak WiFi RX, not the
   transport.
5. Isomorphic Rust unifies the **core**, not the **edges** (threads, storage,
   crypto, transport) — which maps onto declaration/producer/consumer.
6. The declarative seam (`SESSION_OUTPUT`) held across websocket → iroh → back;
   substrate change costs zero consumer rewrites. Its value shows up exactly at
   substitution.
