# IC-3C independent node qualification usage

Status: UNREVIEWED frozen candidate consumer surface for the owner-requested handoff. Integrated directed interruption qualification and final IC-3ABC review remain pending. This is a disposable, explicitly provisioned development profile. Default starts retain their existing legacy behavior. IC-4 browser activation, live settings migration, deployment/key provisioning, Raft and general repair are outside this profile.

## Start and stop

Build the existing binary with `cargo build --locked --offline --manifest-path glade/node/Cargo.toml --bin glade-node`. Start the explicit route with:

```sh
glade/node/target/debug/glade-node --independent-config /absolute/disposable/owned.profile 4010
```

The port is optional and retains Settings' existing default of `0`, asking the local client listener for an ephemeral port. Supply an explicit fixed port when configuring a direct peer address. Do not mix this form with a legacy STORE positional, `--boot`, operator/app/network/peer files, recovery/grant/lease flags or declarations. Missing/repeated config values, relative paths, unknown/mixed forms and invalid ports refuse before opening either route. The existing no-argument usage output names this form; this guide does not introduce a `--help` command. The flag selects the actual assembled node route without requiring `GLADE_NODE_ASSEMBLED`; an explicitly invalid existing environment setting still refuses.

Startup reads the profile once, constructs one owned NodeAssembly scope and Iroh carrier, exclusively opens/validates the disk/floor pair, replays native callbacks and selected remote inbox entries, then starts the existing client service. `independent ready <resource>`, `independent peer <endpoint>@<address>` and `listening <port>` report progress. Listening is the usable local service boundary; an earlier log is not a write receipt. A failed start exits unsuccessfully and MUST NOT be treated as a fallback to legacy admission.

On Unix, SIGTERM or SIGINT starts cooperative shutdown: stop fresh local/exchange work, close owned sockets/links, await the same receive futures and native continuations, settle exact observation or conservative loss, then close Records before releasing root/floor locks and carrier. A clean exit means these owned releases completed. A close error/Pending retains obligations and reports an unsuccessful stop. Do not delete roots or floor files to make close succeed. SIGKILL is a real crash and provides no drained receive capability.

## Provisioning file and custody

There is no shipped profile-generator command. A trusted development controller writes an absolute regular non-symlink file using the existing public Rust recovery codec. The closed local-only file is `encode(&fields: Vec<Bytes>, &DecodeLimits)` with the following twelve ordered fields. Each encoded typed field uses `glade_crdt_recovery_codec::encode`; seed/digest fields are raw bytes. This file does not add a remote encoding.

| Index | Required content |
| --- | --- |
| 0 | Raw UTF-8 bytes `glade/ic3/node-qualification/v1` |
| 1 | Encoded `ReplicaOpen`, including the genuine signed declaration/identity, exact owner, plan/invocation namespaces and finite signed limits |
| 2 | Encoded genuine root-signed `SignedRecord` policy cut |
| 3 | Encoded `Vec<SignedRecord>` of genuine root-signed writer origin certificates |
| 4 | Raw 32-byte local node signing seed |
| 5 | Raw 32-byte local Iroh endpoint seed |
| 6 | Encoded `Vec<String>` containing exactly two absolute separately owned paths: data root, independently trusted floor root |
| 7 | Raw 32-byte persistent registration digest |
| 8 | Encoded `bool`: true only for genuinely fresh roots; false for supported reopen |
| 9 | Encoded `(Option<TimeInterval>, u64)` current trusted uncertainty interval and trusted time floor |
| 10 | Encoded `Vec<(Digest,(Digest,String))>` of authorized peer node key, TLS endpoint key and exact direct address, e.g. `<endpoint-hex>@127.0.0.1:4011` |
| 11 | Encoded `(u64,u64)` retry tick milliseconds and receive/send deadline milliseconds |

The outer reader caps the file at 4 MiB, 65536 items, depth64 and 512-byte names. Peer count is at most16; node and endpoint mappings MUST be unique, nonzero, not the local identities and consistent with the Iroh address endpoint. Tick/deadline MUST each be 1..60000. All inner instance/storage/decode/inbox/inventory/guard limits are required finite injected values; capacity is a refusal boundary, never permission to truncate history. The profile contains private seed material: its custody belongs to the trusted development controller. Qualification uses disposable scratch seeds, not a live key or store.

The injected current policy/clock is the explicit immutable development observation in fields2/9. The real OS timer bounds network work but does not become authentication time. A missing/uncertain/expired trusted cut refuses when the stronger provider contract requires it. This profile does not promise production wall-clock sourcing or automatic live policy refresh; those remain controller/provider work. Genuine signatures, exact declaration/schema/profile/version hashes, writer possession, historical admission validity and persisted observation floors remain enforced.

For fresh=true, both roots MUST be empty/fresh, expected_floor is the fresh zero digest, and original creation intent/owner/registration/namespaces MUST be supplied. Provisioning writes the new independent private-format compatibility marker in that fresh disposable data root. It does not migrate or seal a legacy store. For reopen, preserve the same paths, registration, signed identity, owner and plan/invocation namespaces and set fresh=false. The node obtains expected_floor from the separately locked authoritative trusted floor bytes; it does not silently replace an explicitly wrong logical namespace or accept a peer/cached-image floor. The narrower Rust host's restoration recipe remains in the PersistenceUsage guide.

Copied data without its legitimate trusted custody, wrong/concurrently owned roots, corrupted/truncated images, rollback against the independent floor and unresolved physical observation markers refuse. Arbitrary rollback of data and trusted floor together is outside the qualification assumption. Never edit a floor, reuse a creation identity for a new store, or remove a compatibility/uncertainty marker as recovery.

## Local qualification ingress

The existing Listening service dispatches the explicitly independent instance to the same retained DiskSession and native core callbacks. It bypasses legacy holder/home/Store append. The private control is not a general browser API. Each connection sends a canonical genuinely writer-signed `LocalChallengeBody` with role `local-append-request`, exact instance/identity/resource/node/requester/writer key and operation digest. The server issues a fresh genuinely node-signed challenge; the writer returns its exact signed response. Only then does the same session establish a durable owned guard and consume the length-prefixed canonical `TransferBundle` Candidate. The client sends original certificate, policy, permit and operation proof; it MUST NOT supply Facts, a receipt or a presealed local admission.

The local guard source names the actual authorized holding/admitting node adapter. The certified append-only writer remains the original signed requester and does not need replica.hold. This distinction does not apply to peer history: peer guard source MUST be the genuine authenticated remote holder bound to its TLS exporter. Foreign/revoked/expired writer, wrong domain/body role/node/declaration, missing node hold/admit authority and quota refuse. Transport authentication alone supplies no append permission.

A successful response is the original canonical Accepted bundle and original LocalProcessRestart receipt. A lost reply is retryable with identical operation/proof bytes after a fresh possession exchange, including after supported reopen; the returned admission/receipt is the original one. Reconnection or remote import does not upgrade it to quorum/storage-device durability. `independent local committed <digest>` is logged only after actual native durable acceptance. `independent local outcomes: [...]` preserves the actual core refusal/pending distinction in diagnostics; EOF/refused control is not a successful receipt. The private wire does not introduce a structured general UI status DTO.

Local raw intent is retained before native submission. Its bytes are different from the subsequently sealed historical Accepted bundle; exact original local query and terminal custody justify classification. Startup does not replay an old local intent using cached possession: the writer must authenticate afresh and retry its exact request.

## Automatic duplex and read completeness

Peers use the actual Iroh CarrierLink/TLS endpoint plus signed fresh scoped HELLO, endpoint/node/exporter/roles and exact declaration tuple. Identity alone is insufficient; both holders require the current scoped replica.hold policy. Exact frame reads prevent HELLO from prefetching later history. After fresh HELLO confirmation, both sides negotiate local receive ownership using the same closed history-free signed ExchangeHello body with exact dialer/acceptor ready or busy role and the peer challenge nonce. A busy link refuses before any history guard; it never preempts an already started receive. Both drivers remain able to initiate content-triggered work, including a new edit at the former acceptor. A collision uses bounded node-order scheduling backoff, not an authority timeout. Before every history header/body read, the session establishes its owned durable receive guard.

Each finite round exchanges an immutable signed full inventory, then a signed exact missing-reference request and unchanged canonical bundles in both directions. An empty manifest has zero pages and zero advertised payload bytes. A nonempty bounded manifest has one prequalified indexed InventoryPage (index0/count1); its digest binds the header and complete entries. Inventory covers original Accepted receipts/admissions, original Candidate custody, security evidence, fork pairs and same-slot rivals, including exact previously retained peer originals. Head-only or eligible-only inventory is insufficient. It refuses overflow instead of dropping entries. This bounded no-GC profile eventually stops at its configured retained-history/guard quotas.

The receiver durably saves the exact full manifest and bundle before native dispatch. Classification requires the exact earlier full manifest, authenticated source/channel and actual native historical custody; request pages do not substitute for full inventory. Fully settled remote inbox entries resume through the same native offer/core callbacks on startup even when their source peer is absent. Already retained entries are not reminted. Same-instance unknown/busy custody parks later history; independent instances have separate sessions/roots/issuance namespaces and can progress.

Reconnect work comes from committed history content, not image generation; guard/time checkpoints alone do not cause endless rounds. Successful inbound and outbound rounds update their peer cache; completing an inbound collision does not release another owned outbound worker. The service closes links and joins all owned workers on normal and error exits. `independent exchanged <count>` means that round retained and dispatched those missing bundles; a round with no missing work need not print that line. It is not a global-completeness or remote write acknowledgement. `independent idle <digest> complete=<bool>` reports successful configured-peer rounds at that unchanged full-history snapshot, no owned exchange worker, and a validated cut with no Active guard, permanent loss, missing adapter obligation or native continuation. `complete=true` additionally requires the combined cut to be complete; `complete=false` can describe settled, known kernel incompleteness such as a candidate awaiting an ancestor. Neither value proves global discovery. Qualification waits for matching settled idle snapshots on both nodes before stopping a normal healed pair; stopping during another started receive may honestly retain permanent loss. The log is an observation, not a receive capability or permission to clear an older guard.

A combined read cut accounts for kernel state AND all retained inventory/inbox/guard/attempt obligations. Fork custody can contain two original Accepted rows while its eligible projection is empty; pass eligible operations to the released text engine, not raw custody rows. Earlier unknown/permanent loss, sticky kernel incomplete state, omitted digest/ancestor, unresolved marker or orphan Active guard keeps complete_local=false. A later empty/latest inventory cannot clear it.

After SIGKILL during a receive, the Active guard remains pending/incomplete. Restart, lock ownership, timeout, peer absence and lease change do not manufacture the old drained permit. Ordinary valid app state may remain readable, but new guarded input/clean close can remain unavailable/Pending. Fully settled selected inbox work is a different case and can resume automatically. Preserve roots and report the exact stop; this profile provides no generic reset, live migration, arbitrary rollback repair or garbage collection.
