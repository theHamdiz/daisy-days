#!/usr/bin/env bash
set -euo pipefail

# Daisy Days - Build Script
# Usage: ./scripts/build.sh [command]
# Commands: build, server, wasm, package, clean, install, help

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$ROOT_DIR"

log_info() { echo -e "\033[0;34m[INFO]\033[0m $1"; }
log_ok() { echo -e "\033[0;32m[OK]\033[0m $1"; }
log_warn() { echo -e "\033[1;33m[WARN]\033[0m $1"; }
log_err() { echo -e "\033[0;31m[ERROR]\033[0m $1"; }

check_wasm_target() {
    if ! rustup target list --installed | grep -q "wasm32-wasip1"; then
        log_warn "wasm32-wasip1 target not installed. Installing..."
        rustup target add wasm32-wasip1
    fi
}

build_server() {
    log_info "Building MCP server (release)..."
    cargo build --release -p daisy_days_mcp
    log_ok "MCP server built: target/release/daisy_days"
}

build_wasm() {
    log_info "Building WASM extension..."
    check_wasm_target
    DAISY_SKIP_SERVER=1 cargo build --release --target wasm32-wasip1
    log_ok "WASM extension built: target/wasm32-wasip1/release/daisy_days_extension.wasm"
}

build_all() {
    log_info "Building Daisy Days extension..."
    build_server
    build_wasm
    log_ok "Build complete!"
}

package() {
    log_info "Packaging extension..."
    build_all

    DIST_DIR="$ROOT_DIR/target/dist"
    mkdir -p "$DIST_DIR"
    cp target/release/daisy_days "$DIST_DIR/" 2>/dev/null || cp target/release/daisy_days.exe "$DIST_DIR/" 2>/dev/null || true
    cp target/wasm32-wasip1/release/daisy_days_extension.wasm "$DIST_DIR/extension.wasm"
    cp extension.toml "$DIST_DIR/"
    cp LICENSE "$DIST_DIR/" 2>/dev/null || true
    cp README.md "$DIST_DIR/" 2>/dev/null || true

    log_ok "Package ready in target/dist/"
    ls -la "$DIST_DIR/"
}

clean() {
    log_info "Cleaning build artifacts..."
    cargo clean
    log_ok "Clean complete"
}

install_local() {
    log_info "Installing to local Zed extensions..."
    build_all

    if [[ "$OSTYPE" == "darwin"* ]]; then
        ZED_EXT_DIR="$HOME/.config/zed/extensions/daisy-days"
    elif [[ "$OSTYPE" == "linux"* ]]; then
        ZED_EXT_DIR="$HOME/.config/zed/extensions/daisy-days"
    else
        log_err "Unknown OS: $OSTYPE"
        exit 1
    fi

    mkdir -p "$ZED_EXT_DIR"
    cp target/wasm32-wasip1/release/daisy_days_extension.wasm "$ZED_EXT_DIR/extension.wasm"
    cp target/release/daisy_days "$ZED_EXT_DIR/" 2>/dev/null || true
    cp extension.toml "$ZED_EXT_DIR/"

    log_ok "Installed to $ZED_EXT_DIR"
}

show_help() {
    cat << EOF
Daisy Days - Build Script

Usage: ./scripts/build.sh [command]

Commands:
  build     Build both MCP server and WASM extension (default)
  server    Build MCP server only
  wasm      Build WASM extension only
  package   Build and package to target/dist/
  install   Build and install to local Zed extensions
  clean     Clean build artifacts
  help      Show this help message

Examples:
  ./scripts/build.sh              # Build everything
  ./scripts/build.sh server       # Build MCP server only
  ./scripts/build.sh wasm         # Build WASM extension only
  ./scripts/build.sh package      # Create distribution package
  ./scripts/build.sh install      # Install to local Zed
EOF
}

case "${1:-build}" in
    build)   build_all ;;
    server)  build_server ;;
    wasm)    build_wasm ;;
    package) package ;;
    install) install_local ;;
    clean)   clean ;;
    help|--help|-h) show_help ;;
    *)
        log_err "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
