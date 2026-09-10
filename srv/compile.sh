#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Rust
cd "$SCRIPT_DIR/Rust"
cargo build --release
cp target/release/main "$SCRIPT_DIR/main"

# Run
cd "$SCRIPT_DIR"
./main

