# glade-razel — razel-facing grazel suppliers (reserved stub — G-razel deferral)

Status: RESERVED STUB, deliberately deferred (2026-07-12) by ruling **G-razel**
(`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md` §VII).
**Premise corrected.** The prior stub claimed razel's "request taxonomy /
protocol does not exist yet" — that is **FALSE**. The canonical `razel-wire-api`
**0.1.0** protocol EXISTS: **10 methods**, a ratified taut IR
(`razel-dev/razel-wire-api/protocol/razel.taut.py`, RATIFIED by
`RazelV4CommsProtocolLockdown.md` 2026-07-07), fail-closed wire (nothing escapes
the IR; malformed input is a typed error, not a panic). What is deferred is NOT
the protocol — it is every **glade supplier surface** over razel. Common
contract: `dev-docs/glade/GladeSupplierModel.md`.

## 1. The premise correction (why "no protocol" was wrong, and what still holds)

The wait-for-razel discipline was right; its stated reason was not. Razel already
ships a pinned, ratified wire. So deferral here is **not** "wait for a protocol
to be born" — it is a deliberate **product / surface** decision:

**10 wire methods ≠ 10 product features at equal maturity.** The IR is one
plane (the cli↔daemon comms plane); a glade supplier surface is a different
plane (peer-facing, authorized, declared). Only a subset of the 10 is even
implemented end-to-end, and none of them is a settled grade supplier. Mapping
the IR method-for-method onto glade suppliers would repeat exactly the mistake
`glade-gwz` had to unwind (an inventory copied mechanically from a protocol).

## 2. Per-method reality — `razel-wire-api` 0.1.0 (G-razel / §VII)

Reproduced from the ruling. Classification is by **current maturity**, not by the
wire label. Candidate `razel.*` names below are **RESERVATIONS ONLY** — not
declared, not granted, and **not compatibility commitments**.

| Protocol method | Current reality | Deferred treatment |
| --- | --- | --- |
| `build` | Implemented end-to-end | Candidate future `razel.build.submit` exchange; **not** specified or declared yet. |
| `events.subscribe` | Implemented invocation log | Candidate future `razel.build.events`; **not** specified or declared yet. |
| `hello`, `version`, `ping` | Implemented protocol/control methods | Keep **inside the provider adapter** (compatibility, health, readiness); **never** exposed as user supplier features. |
| `run` | Authored; returns typed `Unsupported` | Reserve `razel.run.submit`; do **not** declare or grant in v1. |
| `query` | Authored; returns typed `Unsupported` | Reserve read-only target/deps/rdeps/somepath query contracts; do **not** declare in v1. |
| `affected` | Authored; returns typed `Unsupported` | Reserve `razel.affected`; do **not** declare in v1. |
| `cancel` | Authored; the synchronous v1 build **cannot** be cancelled | Reserve `razel.operation.cancel`; declare **only after** the daemon has real interrupt semantics AND a terminal `cancelled` event. |
| `shutdown` | Implemented daemon lifecycle control | Provider-local administration only; MUST **NOT** be a peer-facing glade supplier surface. |

`ws.relations` (member-repo dependency graph — the slot declared in
glade-workspaces §2.2/§3) **stays reserved** until razel's dependency query
(`query`) is implemented. Glade MUST NOT invent a second dependency model to
fill it early.

## 3. The ruling — defer until the gwz family WORKS (G-razel / §VII)

**Defer ALL razel-facing grazel supplier interfaces until the GWZ supplier
family is working.** "Working" means every one of these passes:

- the generated GWZ interface **inventory** (`glade-gwz`);
- **provider composition** (attach/declare per surface);
- **per-surface authorization**;
- **operation-result closure** (the closing replicated result record); and
- at least one **real local AND forwarded** integration path.

**Before that gate:** no `razel.*` interface is normative, generated, declared
in a `.glade` composition, granted, or implemented. The candidate names in §2
are reservations only. No traces, no plan steps, no bindings beyond those names.

## 4. After the gate — how this expands (not a mechanical copy)

Once the GWZ gate passes: **re-read the then-pinned razel protocol** and make a
**new grain / capability ruling** from its actually-supported methods (the same
rule that produced the gwz split — one interface per distinct UI contract AND
capability class). **Reuse** gwz's supplier kit, `ProviderCallContext`
propagation, operation-result closure, and per-surface authorization tests. The
current 10-method inventory MUST NOT be copied mechanically — maturity, not the
IR shape, decides what becomes a surface.

## 5. Dependencies + user-testable-when

- **Depends on:** the **GWZ family** reaching "working" (§3 — the gate);
  glade-workspaces (selection / stable-ID authority, E-ws-1); glade-users (B3
  attribution); `razel-wire-api` 0.1.0 (external; canonical home `razel-dev`,
  pinned — not this repo's to design).
- **User-testable when:** after the gate AND a new §4 ruling, I run a razel build
  on the selected workspace and watch structured progress stream. **Nothing here
  is user-testable in v1** — the surfaces do not exist until the trigger fires.

## 6. Expansion trigger

This spec expands **when the GWZ supplier gate passes** (§3's "working"
definition) — NOT before, and NOT (per §1's correction) when "razel publishes a
protocol," which has already happened. Until the gate: reserved names only.
