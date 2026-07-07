# Workstreams — GLP-0005

One owner per write scope. WS-B/C/D start only after P0 exit (contract lock).
WS-B and WS-C then proceed in parallel against the corpus, independent of each
other. See `Plan.md` for phase detail.

| WS | Scope | Owns (write) | MUST NOT touch | Inputs | Outputs | Merge risk |
| --- | --- | --- | --- | --- | --- | --- |
| WS-A contract | wire IR, corpus, fold conformance vectors | `taut/ir/glade.taut.py`, `taut/corpus/glade.*`, generated codec targets | grip-core, node/client impls | `GladeSubstrateV1.md` §2/§6, `taut/ir/griplab.taut.py` (pattern) | frozen IR + corpora + codecs | low — single owner; downstream pins to corpus |
| WS-B node | rust glade node + echo provider | `glade/node/` (glade submodule, branch `gladev2`) | IR, client | WS-A corpus | localhost node | medium — submodule pin coordination |
| WS-C client | TS session library | `glade/client-ts/` (glade submodule, branch `gladev2`) | IR, node internals | WS-A corpus + fold vectors | browser session | medium — submodule pin coordination |
| WS-D grip | grip-core share feature + grip-share binder + demo parity | `glial-dev/grip-core` (branch off `main`), `grip-share`, `glial-dev/grip-react-demo` | IR, node, client internals | GQ-5 (base-tap), WS-C session | sharable taps + parity | medium — grip-core submodule pin |
| WS-E integration | M-LIMP harness, doc fold-back, registry/checkpoints | `plan-docs/plans/GLP-0005-*/`, `glade/dev-docs/` updates, root nav files | impl internals | all of the above | green M-LIMP + updated contract | coordinating role only |

## Cross-repo rule

Root `glial-dev` is the primary roll-build checkout (`gladev2` branch +
`gladev2/` tags). The `glade` and `grip-core` submodules get their own
`gladev2` branch when a phase first writes to them; root records the pinned
submodule commit at each phase tag. Shared root navigation files
(`Registry.md`, `ActiveWork.md`) edited only by WS-E.
