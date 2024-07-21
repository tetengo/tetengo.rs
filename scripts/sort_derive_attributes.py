#! /usr/bin/python
# Sorts the traits listed for #derive attributes.
# Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org>

import re
import sys
from pathlib import Path

import list_sources


def main(args: list[str]):
    if len(args) < 1:
        print("Usage: sort_derive_attributes.py base_directory_path", file=sys.stderr)
        sys.exit(0)

    paths = list_sources.list_rs_files(Path(sys.argv[1]))
    for path in paths:
        sort_derive_attributes(path)


def sort_derive_attributes(path: Path):
    input_lines = load_from_file(path)
    output_lines = []
    while len(input_lines) > 0:
        input_line = input_lines.pop(0)

        single_sorted = sort_single_derive_attributes(input_line)
        if single_sorted:
            output_lines.extend(single_sorted)
            continue

        multiple_sorted = sort_multiple_derive_attributes(input_line, input_lines)
        if multiple_sorted:
            output_lines.extend(multiple_sorted)
            continue

        output_lines.append(input_line)
    save_to_file(output_lines, path)


def load_from_file(path: Path) -> list[str]:
    with path.open(mode="r", encoding="UTF-8") as file:
        return file.readlines()


def save_to_file(lines: list[str], path: Path):
    with path.open(mode="w", encoding="UTF-8") as file:
        file.writelines(lines)


SINGLE_DERIVE_PATTERN = re.compile(r"\s*#\[\s*derive\s*\((?P<traits>.+)\)\s*\]\s*")

MULTIPLE_DERIVE_BEGIN_PATTERN = re.compile(r"\s*#\[\s*derive\s*\(\s*")

MULTIPLE_DERIVE_END_PATTERN = re.compile(r"\s*\)\s*\]\s*")


def sort_single_derive_attributes(input_line: str) -> list[str] | None:
    matched = SINGLE_DERIVE_PATTERN.fullmatch(input_line)
    if not matched:
        return None
    traits = [
        t
        for t in map(lambda x: x.strip(), matched.group("traits").split(","))
        if len(t) > 0
    ]
    traits.sort()
    output_line = "#[derive({})]\n".format(", ".join(traits))
    return [output_line]


def sort_multiple_derive_attributes(
    first_input_line: str, rest_input_lines: list[str]
) -> list[str] | None:
    matched = MULTIPLE_DERIVE_BEGIN_PATTERN.fullmatch(first_input_line)
    if not matched:
        return None
    indent = first_input_line[0 : first_input_line.find("#")]
    output_lines = [first_input_line]
    traits = []
    while True:
        input_line = rest_input_lines.pop(0)
        if MULTIPLE_DERIVE_END_PATTERN.fullmatch(input_line):
            break
        for trait in input_line.split(","):
            trait = trait.strip()
            if len(trait) > 0:
                traits.append(trait.strip())
    traits.sort()
    for trait in traits:
        output_lines.append("{}    {},\n".format(indent, trait))
    output_lines.append(input_line)
    return output_lines


if __name__ == "__main__":
    main(sys.argv[1:])
