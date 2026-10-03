#!/usr/bin/env python3
"""All local Rust files, including orphan/disabled paths: lexical cfg scope check
plus rustfmt's syntax parser for braced Rust control flow. This private profile
has no conditional attributes. Dependency/vendor source audits remain open.
"""
import importlib.util
from pathlib import Path
import subprocess
import sys

spec = importlib.util.spec_from_file_location("global_check", sys.argv[1])
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
root = Path(sys.argv[2])
errors = []
count = 0
for crate in ("api", "spec", "raft-rs", "openraft"):
    for path in sorted((root / crate).rglob("*.rs")):
        count += 1
        tokens = [token.text for token in module.lex(path.read_text())]
        for index in range(len(tokens) - 2):
            if tokens[index:index + 2] == ["#", "["] and tokens[index + 2] in ("cfg", "cfg_attr"):
                errors.append(f"{path.relative_to(root)}: conditional attribute forbidden in this local fixture profile")
        result = subprocess.run(["rustfmt", "--check", "--edition", "2024", str(path)], capture_output=True, text=True)
        if result.returncode:
            errors.append(f"{path.relative_to(root)}: syntax/format failure\n{result.stderr}{result.stdout}")
if errors:
    sys.exit("\n".join(errors))
print(f"Explicit source boundaries: PASS ({count} Rust files, all paths, no conditional attributes; Rust syntax requires braces)")
