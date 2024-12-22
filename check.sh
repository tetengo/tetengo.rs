#! /bin/sh
# Checks this package
# Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>

set -e

./kogyan/scripts/check.sh

./kogyan/scripts/sort_derive_attributes.py .

./kogyan/scripts/check_use_order.py .

cargo verify-project

cargo fmt --check

cargo clippy --all-targets

cargo doc

cargo build --all-targets

cargo test --all-targets --quiet
