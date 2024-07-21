#! /bin/sh
# Checks the scripts
# Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>

for f in $(dirname $0)/*.py; do
    echo "Checking $(basename $f)..."
    mypy "$f"
    black "$f"
    isort --profile black "$f"
done
