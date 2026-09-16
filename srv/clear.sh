#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Rust & Cargo Build Artifacts
rm -rf "$SCRIPT_DIR/Rust/target"
rm -f "$SCRIPT_DIR/main"
find "$SCRIPT_DIR/Rust" -type f \( -name "*.rs.bk" -o -name "*.rmeta" \) -delete 2>/dev/null || true


# HTML Build Artifacts (Preserve .gitkeep & prompt.md)
rm -f "$SCRIPT_DIR/html/main.html"
if [ -d "$SCRIPT_DIR/html/static" ]; then
    find "$SCRIPT_DIR/html/static" -mindepth 1 ! -name ".gitkeep" ! -name "prompt.md" -delete 2>/dev/null || true
fi

# Var Storage & DB (Preserve .gitkeep & prompt.md)
if [ -d "$SCRIPT_DIR/var" ]; then
    find "$SCRIPT_DIR/var" -mindepth 1 ! -name ".gitkeep" ! -name "prompt.md" -delete 2>/dev/null || true
fi

# Logs
find "$SCRIPT_DIR" -type f -name "*.log" -delete 2>/dev/null || true
