#!/usr/bin/env python3
"""Explicit B0 checkpoint evidence; expected behavioral RED is recorded separately.
No pass is inferred from an expected failure. Logs always retain actual status.
"""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile
import time

root = Path(__file__).resolve().parent
workspace = root.parent.parent
parser = argparse.ArgumentParser()
parser.add_argument("--prefix", required=True, help="fresh evidence prefix; preserves historical checkpoint files")
args = parser.parse_args()
if not re.fullmatch(r"[a-z0-9-]+", args.prefix):
    raise SystemExit("Use a lowercase alphanumeric/hyphen evidence prefix")
prefix = args.prefix + "-"
records = []
base = ["cargo", "test", "--locked", "--offline", "--manifest-path", str(root / "Cargo.toml")]
def run(label, command, expected):
    start = time.perf_counter()
    result = subprocess.run(command, cwd=workspace, capture_output=True, text=True)
    elapsed = time.perf_counter() - start
    (root / "evidence" / f"{prefix}{label}.log").write_text(result.stdout + result.stderr)
    records.append({"label": label, "command": command, "seconds": round(elapsed, 6), "exit_code": result.returncode, "expected_exit_code": expected})
    print(f"{label}: exit={result.returncode}, expected={expected}, {elapsed:.6f}s")
    if result.returncode != expected:
        raise SystemExit(f"Unexpected checkpoint result: {label}; inspect evidence/{label}.log")
    if label.startswith("behavior-red"):
        if not re.search(r"test result: FAILED\. 0 passed; 20 failed; 0 ignored; 0 measured; 0 filtered out;", result.stdout):
            raise SystemExit("Expected exactly 20 compiling ordinary B0 assertion failures")
    return result
run("api-witness", base + ["-p", "glade-carrier-api", "--test", "public_contract"], 0)
run("provider-witness", base + ["-p", "glade-carrier-raft-rs", "-p", "glade-carrier-openraft", "--test", "compiler_contract"], 0)
run("spec-witness", base + ["-p", "glade-carrier-spec", "--lib", "--test", "fixture_plumbing", "--test", "oracle", "--test", "constructor_contract", "--test", "rpc_reply", "--test", "endpoint_lifecycle", "--test", "scheduler_lifecycle"], 0)
run("termination-mutant", ["python3", "-B", str(root / "check-termination-mutant.py")], 0)
run("behavior-red", base + ["-p", "glade-carrier-spec", "--test", "b0_election"], 101)
run("behavior-red-warm", base + ["-p", "glade-carrier-spec", "--test", "b0_election"], 101)
artifacts = run("execution-artifacts", base + ["-p", "glade-carrier-spec", "--test", "b0_election", "--no-run", "--message-format=json"], 0)
executable = next(record["executable"] for line in artifacts.stdout.splitlines() if (record := json.loads(line)).get("reason") == "compiler-artifact" and record.get("executable") and record["target"]["name"] == "b0_election")
run("behavior-red-execution", [executable], 101)
fresh_target = tempfile.mkdtemp(prefix="cold-checkpoint-", dir=root / "target")
run("cold-test-build", base + ["-p", "glade-carrier-spec", "--test", "b0_election", "--no-run", "--target-dir", fresh_target], 0)
run("structural", [str(root / "check.sh")], 0)
run("clippy", ["cargo", "clippy", "--locked", "--offline", "--manifest-path", str(root / "Cargo.toml"), "-p", "glade-carrier-api", "-p", "glade-carrier-spec", "-p", "glade-carrier-raft-rs", "-p", "glade-carrier-openraft", "--all-targets", "--", "-D", "warnings"], 0)
context = {}
for label, command in [("machine", ["uname", "-a"]), ("os", ["sw_vers"]), ("rustc", ["rustc", "-Vv"]), ("cargo", ["cargo", "-V"]), ("python", ["python3", "-V"])]:
    result = subprocess.run(command, capture_output=True, text=True, check=True)
    context[label] = result.stdout.strip()
(root / "evidence" / f"{prefix}measurements.json").write_text(json.dumps({"context": context, "records": records}, indent=2) + "\n")
