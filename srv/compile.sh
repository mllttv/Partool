#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

BUILD_FLAG=""
PROFILE="debug"
[[ " $* " =~ " --release " ]] && BUILD_FLAG="--release" && PROFILE="release"

export RUSTFLAGS="--remap-path-prefix=$HOME=/root --remap-path-prefix=$SCRIPT_DIR=/srv"

# 1. Rust WASM Client (SPA Hydration)
echo "Building Leptos WASM Client ($PROFILE)..."
cd "$SCRIPT_DIR/Rust"
mold -run cargo build --lib --target wasm32-unknown-unknown --no-default-features --features hydrate $BUILD_FLAG
mkdir -p "$SCRIPT_DIR/html/pkg"
wasm-bindgen --target web --out-dir "$SCRIPT_DIR/html/pkg" --no-typescript "target/wasm32-unknown-unknown/$PROFILE/htmlvm.wasm"
[ "$PROFILE" = "debug" ] && wasm-opt -O1 --strip-dwarf -o "$SCRIPT_DIR/html/pkg/htmlvm_bg.wasm" "$SCRIPT_DIR/html/pkg/htmlvm_bg.wasm"
[ "$PROFILE" = "release" ] && wasm-opt -Oz --all-features --strip-debug --strip-producers -o "$SCRIPT_DIR/html/pkg/htmlvm_bg.wasm" "$SCRIPT_DIR/html/pkg/htmlvm_bg.wasm"

# 2. Rust Native Server (Fullstack Monolith / SSR)
echo "Building Native Server Binary ($PROFILE)..."
mold -run cargo build --bin main --features ssr $BUILD_FLAG
rm -f "$SCRIPT_DIR/main"
cp "target/$PROFILE/main" "$SCRIPT_DIR/main"
[ "$PROFILE" = "release" ] && strip --strip-all "$SCRIPT_DIR/main" && objcopy --remove-section .comment "$SCRIPT_DIR/main" 2>/dev/null || true

# 4. Run
cd "$SCRIPT_DIR"
[[ " $* " =~ " --no-run " ]] || ./main
