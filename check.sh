#! /bin/sh
# Checks this package
# Copyright 2023 kaoru  https://www.tetengo.org/

set -e

cargo verify-project
cargo fmt --check
cargo clippy --tests
cargo doc
cargo build --all-targets
cargo test --all-targets
