# Checkpoints — GLP-0006

Roll-build tag prefix: `grazel1/` (root glade-wz; members pin via gwz lock).
Tag a phase only when its milestone is met, the demo tab(s) verify live, and
remaining ambiguities are minor.

| Tag | Phase | Goal (milestone) | Verification | Status |
| --- | --- | --- | --- | --- |
| `grazel1/p0-foundations` | P0 | a supplier can exist: vocab+model doc, F1 live minting, rust wire client, supplier kit, typed manifest + tab chassis, grazel skeleton boots, principals minimal (attribution live in dir.principals) | all repo gates green; two-node live mint + route outside tests; typed-manifest TS compile wall demonstrated | **done 2026-07-12** — glade d872838 (node 56 · wire-rs 3 · client-rs 6 · grip-share 18 · client-ts 9 · demo build), glial 4dfbd23 (60 + tsc), grazel 56d9a32 (15 + integration); create-to-target E2E + kit-over-real-client (GAP-13 closed); compile wall proven twice (glial @ts-expect-error gate, demo TS2339 probe); live two-tab demo verified. Tag `grazel1/p0-foundations` READY — not minted (tags are Gianni's call) |
| `grazel1/p1-first-suppliers` | P1 | chat + gwz suppliers live in demo tabs; grazel composes them and serves gryth-ui | demo tabs verified live (two participants); gryth-ui shows chat+gwz panels via glial | **done 2026-07-12** — S1-S3 (glade-chat 9238d21 · glade-gwz e53c87d · compose cbb19ba/61617d5; atlas 241; demo tabs live) + S4 (gryth-ui bffbdd0 on branch glp-0006-p1s4-gryth-panels, 74 tests, grazel-served live E2E). Tag `grazel1/p1-first-suppliers` READY — not minted (Gianni's call). gryth-ui integration = Gianni merges the branch |
| `grazel1/p2-stage2` | P2 | invite → grant → enforced access live; allow-all ends | stage-2 traces + INV-4/INV-5 as live behavior; audit sweep re-run green | pending |
| `grazel1/p3-heavy-shapes` | P3 | files (windowed + blobs) + live terminal (WINCH) in tabs | window corpus green ×langs; terminal live session survives resize; large-binary path never chains ops | pending |
| `grazel1/p4-editing` | P4 | collaborative editing in gryth-ui | crdt/swmr corpus green; cursor-stable remote deltas live | pending |
