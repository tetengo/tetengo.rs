#! /usr/bin/python
# Sorts the traits listed for #derive attributes.
# Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org>

import sys
from pathlib import Path

import list_sources


def main(args: list[str]):
    package_root = Path(sys.argv[0]) / Path("..")
    paths = list_sources.list_rs_files(package_root)
    for path in paths:
        print("Path: {}".format(path))


if __name__ == "__main__":
    main(sys.argv[1:])
