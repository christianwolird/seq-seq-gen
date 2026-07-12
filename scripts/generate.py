#!/usr/bin/env python3

import subprocess
import sys
from pathlib import Path


if len(sys.argv) != 2:
    print("Usage: python scripts/generate.py <target number of terms>")
    sys.exit(1)

repo_root = Path(__file__).resolve().parent.parent
src_file = repo_root / "src" / "dense_bitmap_generate.rs"
bin_dir = repo_root / "target"
bin_file = bin_dir / "dense_bitmap_generate"

bin_dir.mkdir(parents=True, exist_ok=True)

compile_needed = (
    not bin_file.exists()
    or src_file.stat().st_mtime > bin_file.stat().st_mtime
)

if compile_needed:
    subprocess.run(
        ["rustc", "-O", str(src_file), "-o", str(bin_file)],
        check=True,
    )

subprocess.run(
    [str(bin_file), sys.argv[1]],
    cwd=repo_root,
    check=True,
)
