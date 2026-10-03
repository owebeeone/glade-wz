#!/usr/bin/env python3
"""Reject remote-liveness-for-local-cleanup in an isolated std-only copied fixture.

The real engine/source graphs are absent. Never edits the review object's bytes.
"""
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

root = Path(__file__).resolve().parent
parent = Path(tempfile.mkdtemp(prefix="termination-mutant-", dir=root / "target"))
mutant = parent / "fixture"
shutil.copytree(root, mutant, ignore=shutil.ignore_patterns("target", "evidence", "__pycache__"))
source = mutant / "spec/src/fixture/transport.rs"
text = source.read_text()
needle = "network.owned_rpc(rpc)?;"
assert text.count(needle) == 1, "Mutation must select only Timeout/Cancel validation"
source.write_text(text.replace(needle, "network.rpc(rpc)?;"))
result = subprocess.run(
    ["cargo", "test", "--locked", "--offline", "--manifest-path", str(mutant / "Cargo.toml"),
     "-p", "glade-carrier-spec", "--lib"],
    cwd=mutant, capture_output=True, text=True,
)
print(result.stdout + result.stderr, end="")
expected = {
    f"fixture::transport::termination::{request}_{peer}_peer_{action}"
    for request in ["held", "consumed"]
    for peer in ["stopped", "replaced"]
    for action in ["timeout", "cancel"]
}
observed = set(re.findall(r"^test (\S+) \.\.\. FAILED$", result.stdout, re.MULTILINE))
assert result.returncode == 101, "Mutant must compile and fail behavior"
assert observed == expected, (observed, expected)
assert "test result: FAILED. 19 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out;" in result.stdout
print("Local RPC termination mutant: REJECTED (all eight compiling matrix assertions; no selection bypass)")
