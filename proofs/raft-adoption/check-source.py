#!/usr/bin/env python3
"""Token-aware local rule: this fixture workspace has no conditional attributes.

Rust parsing/format checking enforces braced control-flow syntax. Tokenization
strips literals/comments and scans every source, including disabled files.
Production cfg migration and dependency sources are outside this local gate.
"""
import importlib.util
from pathlib import Path
import sys

spec = importlib.util.spec_from_file_location("global_check", sys.argv[1])
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
root = Path(sys.argv[2])
errors = []
for crate in ("api", "durability-api", "disk", "proof"):
    for path in sorted((root / crate).rglob("*.rs")):
        tokens = [token.text for token in module.lex(path.read_text())]
        for index in range(len(tokens) - 2):
            if tokens[index:index + 2] == ["#", "["] and tokens[index + 2] in ("cfg", "cfg_attr"):
                errors.append(f"{path.relative_to(root)}: conditional attribute forbidden in this fixture profile")
if errors:
    sys.exit("\n".join(errors))
print("Explicit source boundaries: PASS (token scan, no conditional attributes)")
