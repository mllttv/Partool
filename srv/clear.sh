#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Rust
rm -rf "$SCRIPT_DIR/Rust/target"
rm -f "$SCRIPT_DIR/main"

# HTML
rm -f "$SCRIPT_DIR/html/main.html"
if [ -d "$SCRIPT_DIR/html/static" ]; then
    find "$SCRIPT_DIR/html/static" -mindepth 1 ! -name ".gitkeep" -delete
fi

# Tmp
if [ -d "$SCRIPT_DIR/tmp" ]; then
    find "$SCRIPT_DIR/tmp" -mindepth 1 ! -name ".gitkeep" -delete
fi

