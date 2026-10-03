#!/bin/sh
set -eu
proof_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
workspace_root=$(CDPATH= cd -- "$proof_root/../.." && pwd)
checker_manifest="$workspace_root/glade-discover/tools/architecture-check/Cargo.toml"
cargo run --quiet --locked --offline --manifest-path "$checker_manifest" --target-dir "$proof_root/target/architecture-check" -- "$proof_root"
python3 -B "$workspace_root/glade/scripts/checks/check_process_globals.py" --repo "$proof_root" --allowlist "$proof_root/process_globals_allowlist.json"
python3 -B "$proof_root/check-source.py" "$workspace_root/glade/scripts/checks/check_process_globals.py" "$proof_root"
python3 -B "$proof_root/check-architecture-negatives.py" "$proof_root" "$proof_root/target/architecture-check/debug/glade-architecture-check"
cargo fmt --manifest-path "$proof_root/Cargo.toml" --all -- --check
