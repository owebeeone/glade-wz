# Plan Registry

Status: active index

| Plan | Title | Status | Owner | Branch / Checkout | Affected Modules |
| --- | --- | --- | --- | --- | --- |
| `GLP-0001` | GripLab terminal transport spike | active | current agent | root checkout on `main`; `grip-lab` branch `codex/phase1-node-orbitdb-spike` | root `plan-docs`, `grip-lab` Phase 1 Node sidecar, future Glade transport |
| `GLP-0002` | Multi-Participant Terminal + Typing Avatar | draft | maintainer (coordinating) | root checkout on `main`; per-workstream branches TBD | `glade`, `grip-lab`, provider (`glial-py`/`glial-server`), transport (`rust-libp2p` adapter + hub), root `plan-docs` |
| `GLP-0003` | rust-orbitdb | draft | maintainer (coordinating) | root checkout on `main`; future branch `codex/glp-0003-rust-orbitdb`; new submodule `rust-orbitdb` | root `plan-docs`, future `rust-orbitdb`, host bindings, optional `rust-libp2p` adapter |
| `GLP-0004` | Virtual-VPN Transport Spike (iroh-as-tunnel) | proposed | maintainer (coordinating) | root checkout on `main`; future branch `codex/glp-0004-virtual-vpn-spike` | `griplab_core` (native agent/forwarder), `grip-lab` LCS WSS (forwarded), hub (rendezvous + relay), root `plan-docs` |
| `GLP-0006` | grazel + gryth — the glade supplier program | proposed | maintainer (coordinating) | glade-wz root on `main`; future tags `grazel1/*` | `grazel` (new), `glade-chat`/`glade-gwz`/`glade-files`/`glade-terminal`/`glade-editing` (new suppliers), `glade` (F1, channels, window delivery, stage-2), `glial` (typed manifest, delta path), `taut-shape` (window/crdt/swmr contracts), `ggg-viz` (supplier traces), gryth-wz/`gryth-ui` |
| `GLP-0005` | Glade Substrate V1 — M-LIMP | **finished** | maintainer (coordinating) | root `glial-dev` on `gladev2`; tags `gladev2/p0-start`→`gladev2/p4-mlimp` | `glade` (wire/node/client-ts/grip-share/demo), `taut` (IR+folds+oracles), `grip-core` (base-tap share), root `plan-docs` |
