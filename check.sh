#! /bin/sh
# Checks this package
# Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>

set -e

./scripts/check.sh
./scripts/sort_derive_attributes.py .
cargo verify-project
cargo fmt --check
cargo clippy --all-targets
cargo doc
cargo build --all-targets
cargo test --all-targets --quiet
