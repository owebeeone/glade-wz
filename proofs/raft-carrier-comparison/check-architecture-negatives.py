#!/usr/bin/env python3
"""Exercise the adopted checker against actual mutated local manifest inventories.
Generated fixtures are temporary beneath this authorized subtree; no checked-in
policy is weakened. Registry dependencies are never introduced.
"""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

root = Path(sys.argv[1])
checker = Path(sys.argv[2])
with tempfile.TemporaryDirectory(prefix="architecture-negatives-", dir=root / "target") as directory:
    fixture = Path(directory)
    for member in ("api", "spec", "raft-rs", "openraft"):
        shutil.copytree(root / member, fixture / member)
    shutil.copyfile(root / "Cargo.toml", fixture / "Cargo.toml")
    policy = json.loads((root / "architecture-policy.json").read_text())
    extra = fixture / "unclassified"
    (extra / "src").mkdir(parents=True)
    (extra / "Cargo.toml").write_text('[package]\nname="unclassified"\nversion="0.0.0"\nedition="2024"\n')
    (extra / "src/lib.rs").write_text('pub struct Payload;\n')
    manifest = (fixture / "Cargo.toml").read_text().replace('"openraft"]', '"openraft", "unclassified"]')
    (fixture / "Cargo.toml").write_text(manifest)
    (fixture / "architecture-policy.json").write_text(json.dumps(policy))
    subprocess.run(["cargo", "generate-lockfile", "--offline"], cwd=fixture, check=True, capture_output=True)
    result = subprocess.run([str(checker), str(fixture)], capture_output=True, text=True)
    assert result.returncode != 0 and "ARCH-001" in result.stderr, result.stderr
    print("Architecture negative: PASS (unknown library rejected as ARCH-001)")
    policy["packages"]["unclassified"] = {"role": "protocol", "reason": "Negative fixture payload only", "dependencies": []}
    (fixture / "architecture-policy.json").write_text(json.dumps(policy))
    original = (fixture / "api/Cargo.toml").read_text()
    for label, declaration in [
        ("normal", '[dependencies]\ninnocent = { package="unclassified", path="../unclassified" }\n'),
        ("build", '[build-dependencies]\ninnocent = { package="unclassified", path="../unclassified" }\n'),
        ("dev", '[dev-dependencies]\ninnocent = { package="unclassified", path="../unclassified" }\n'),
        ("optional-target-renamed", '[target.\'cfg(windows)\'.dependencies]\ninnocent = { package="unclassified", path="../unclassified", optional=true }\n'),
    ]:
        (fixture / "api/Cargo.toml").write_text(original + '\n' + declaration)
        subprocess.run(["cargo", "generate-lockfile", "--offline"], cwd=fixture, check=True, capture_output=True)
        result = subprocess.run([str(checker), str(fixture)], capture_output=True, text=True)
        assert result.returncode != 0 and "ARCH-002" in result.stderr, result.stderr
        print(f"Architecture negative: PASS ({label} forbidden real-name edge rejected as ARCH-002)")
