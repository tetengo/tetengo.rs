# Lists the Rust program sources.
# Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org>

from pathlib import Path


def list_rs_files(base_path: Path) -> list[Path]:
    return [p for p in base_path.rglob("*.rs")]
