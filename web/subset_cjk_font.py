#!/usr/bin/env python3
"""Build the common-glyph subset loaded before the official regional font."""

from __future__ import annotations

import argparse
import hashlib
import subprocess
import tempfile
from pathlib import Path


UPSTREAM_SHA256 = "faa6c9df652116dde789d351359f3d7e5d2285a2b2a1f04a2d7244df706d5ea9"


def gb2312_codepoints() -> list[int]:
    characters = {chr(codepoint) for codepoint in range(0x20, 0x7F)}

    for lead in range(0xA1, 0xF8):
        for trail in range(0xA1, 0xFF):
            try:
                characters.add(bytes((lead, trail)).decode("gb2312"))
            except UnicodeDecodeError:
                pass

    return sorted(ord(character) for text in characters for character in text)


def build_core(source: Path, output: Path, codepoints: list[int]) -> None:
    temporary = output.with_suffix(".tmp.otf")

    with tempfile.NamedTemporaryFile(mode="w", encoding="ascii") as characters:
        characters.write(",".join(f"U+{value:04X}" for value in codepoints))
        characters.flush()

        subprocess.run(
            [
                "pyftsubset",
                str(source),
                f"--output-file={temporary}",
                f"--unicodes-file={characters.name}",
                "--layout-features=*",
                "--name-IDs=*",
                "--name-legacy",
                "--name-languages=*",
                "--notdef-glyph",
                "--recommended-glyphs",
                "--no-recalc-timestamp",
            ],
            check=True,
        )

    temporary.replace(output)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("core_output", type=Path)
    arguments = parser.parse_args()

    source_hash = hashlib.sha256(arguments.source.read_bytes()).hexdigest()
    if source_hash != UPSTREAM_SHA256:
        raise SystemExit(
            f"unexpected source SHA-256: {source_hash}; expected {UPSTREAM_SHA256}"
        )

    core_codepoints = gb2312_codepoints()
    arguments.core_output.parent.mkdir(parents=True, exist_ok=True)
    build_core(arguments.source, arguments.core_output, core_codepoints)

    output_hash = hashlib.sha256(arguments.core_output.read_bytes()).hexdigest()
    print(
        f"wrote {arguments.core_output} from {len(core_codepoints)} requested codepoints "
        f"({arguments.core_output.stat().st_size} bytes, SHA-256 {output_hash})"
    )


if __name__ == "__main__":
    main()
