#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

cargo test --all-features
cargo test --no-default-features
cargo test --no-default-features --features "tracing"
cargo test --no-default-features --features "implementation"
