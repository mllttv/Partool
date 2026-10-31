#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Stop running server process if active
pkill -f "$SCRIPT_DIR/main" 2>/dev/null || true

# Rust & Cargo Build Artifacts
rm -rf "$SCRIPT_DIR/Rust/target"
rm -f "$SCRIPT_DIR/main"
find "$SCRIPT_DIR/Rust" -type f \( -name "*.rs.bk" -o -name "*.rmeta" \) -delete 2>/dev/null || true

# WASM & Static Package Artifacts
rm -rf "$SCRIPT_DIR/html/pkg"
rm -rf "$SCRIPT_DIR/html/static/node_modules"

# Var Storage & DB (Preserve .gitkeep, prompt.md & keys)
if [ -d "$SCRIPT_DIR/var" ]; then
    find "$SCRIPT_DIR/var" -mindepth 1 ! -name ".gitkeep" ! -name "prompt.md" ! -path "$SCRIPT_DIR/var/keys" ! -path "$SCRIPT_DIR/var/keys/*" -delete 2>/dev/null || true
fi

# Tmp Storage (Preserve .gitkeep)
if [ -d "$SCRIPT_DIR/tmp" ]; then
    find "$SCRIPT_DIR/tmp" -mindepth 1 ! -name ".gitkeep" -delete 2>/dev/null || true
fi

# Logs
find "$SCRIPT_DIR" -type f -name "*.log" -delete 2>/dev/null || true
