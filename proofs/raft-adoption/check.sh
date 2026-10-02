#!/bin/sh
set -eu
proof_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
workspace_root=$(CDPATH= cd -- "$proof_root/../.." && pwd)
cargo run --quiet --locked --offline --manifest-path "$workspace_root/glade-discover/tools/architecture-check/Cargo.toml" -- "$proof_root"
python3 "$workspace_root/glade/scripts/checks/check_process_globals.py" --repo "$proof_root" --allowlist "$proof_root/process_globals_allowlist.json"
python3 "$proof_root/check-source.py" "$workspace_root/glade/scripts/checks/check_process_globals.py" "$proof_root"
cargo fmt --manifest-path "$proof_root/Cargo.toml" --all -- --check
