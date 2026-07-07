# Decisions

Plan: `GLP-0001`

| ID | Decision | Rationale |
| --- | --- | --- |
| `GLP-0001-D001` | Treat this as a throw-away spike. | The goal is to test libp2p plus terminal behavior, not produce stable Glade architecture. |
| `GLP-0001-D002` | Use low green/yellow/red thresholds. | We need directional evidence quickly before committing to deeper implementation. |
| `GLP-0001-D003` | Keep terminal collaboration semantics out of scope. | The straight-line interactive case is the hard product proof for this spike. |
| `GLP-0001-D004` | Use Rust provider as the first implementation track. | Browser-to-Rust teaches more about the eventual deployable Glade node path. Node.js and gateway paths remain fallbacks. |
| `GLP-0001-D005` | Use an OrbitDB-shaped in-memory append log, not `rust-orbitdb`, for Phase 1. | `rust-orbitdb` is still bootstrap-stage. Phase 1 needs transport/replay evidence now; OrbitDB informs the log shape, while real oplog/sync compatibility belongs to `GLP-0003`. |
| `GLP-0001-D006` | Supersede the immediate Rust-first track with a Node.js libp2p + OrbitDB sidecar in `grip-lab`. | The user redirected Phase 1 to p2p communications only while leaving the current GripLab backend in place. Node.js OrbitDB gives faster evidence on libp2p, Helia, OrbitDB, ACL, and replication friction. Rust provider work moves later. |
| `GLP-0001-D007` | Add `--allow-any-writer` only as a Phase 1 OrbitDB test setting. | Default OrbitDB write access allows only the database creator. The open ACL separates replication testing from Glade authority design, but it MUST NOT become production authorization. |
| `GLP-0001-D008` | Use a persistent Level blockstore for the Node sidecar. | OrbitDB metadata persisted across runs while Helia's default blockstore did not, causing missing-block failures on restart. The sidecar MUST keep Helia blocks and OrbitDB metadata persistent together during the spike. |
| `GLP-0001-D009` | Store terminal output log payloads as native bytes and cursor by OrbitDB entry CID. | Base64/UTF-8 payloads inflate and corrupt byte-stream semantics, while `seq` is only single-writer-safe. Phase 1 MUST keep live bytes off OrbitDB, coalesce output before append, treat per-session vs shared DB as a simulator input, and avoid a Helia/IPFS archive tier for logs. |
| `GLP-0001-D010` | Split collaborator bootstrap into tunnel, sync, and runtime-supervisor layers once the Node sidecar is added. | The current SSH process conflates forwards with remote child lifetime. Sidecar bootstrap needs named client/sidecar ports, optional bidirectional sidecar forwarding, exact payload sync, and restarts that do not tear down the control path. |
