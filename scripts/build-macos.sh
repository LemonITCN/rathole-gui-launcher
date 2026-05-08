#!/usr/bin/env bash
# Build the launcher for macOS.
#
# Usage:
#   scripts/build-macos.sh              build for the host architecture
#   scripts/build-macos.sh --silicon    Apple Silicon (aarch64-apple-darwin)
#   scripts/build-macos.sh --intel      Intel (x86_64-apple-darwin)
#   scripts/build-macos.sh --universal  fat bundle (universal-apple-darwin)
#
# Output:
#   src-tauri/target/<triple>/release/bundle/macos/*.app
#   src-tauri/target/<triple>/release/bundle/dmg/*.dmg

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BLUE='\033[0;34m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; NC='\033[0m'
step() { printf "\n${BLUE}==>${NC} %s\n" "$1"; }
ok()   { printf "${GREEN}✓${NC}  %s\n" "$1"; }
warn() { printf "${YELLOW}!${NC}  %s\n" "$1"; }
err()  { printf "${RED}✗${NC}  %s\n" "$1" >&2; }

TARGET=""
case "${1:-}" in
  --silicon)   TARGET="aarch64-apple-darwin" ;;
  --intel)     TARGET="x86_64-apple-darwin" ;;
  --universal) TARGET="universal-apple-darwin" ;;
  --help|-h)
    sed -n '2,11p' "$0" | sed 's/^# \{0,1\}//'
    exit 0
    ;;
  "") ;;
  *) err "Unknown option: $1"; exit 1 ;;
esac

step "Checking prerequisites"

if [[ "$(uname)" != "Darwin" ]]; then
  err "This script must be run on macOS."
  exit 1
fi

if ! xcode-select -p >/dev/null 2>&1; then
  err "Xcode Command Line Tools are missing."
  echo "    Install them with: xcode-select --install"
  exit 1
fi
ok "Xcode Command Line Tools"

if ! command -v node >/dev/null 2>&1; then
  err "Node.js is not installed. Install Node 18+ from https://nodejs.org or via nvm."
  exit 1
fi
ok "Node.js $(node --version)"

if ! command -v cargo >/dev/null 2>&1; then
  err "Rust toolchain not found. Install it from https://rustup.rs"
  exit 1
fi
ok "Rust $(rustc --version | awk '{print $2}')"

if [[ -n "$TARGET" ]]; then
  if [[ "$TARGET" == "universal-apple-darwin" ]]; then
    for triple in aarch64-apple-darwin x86_64-apple-darwin; do
      if ! rustup target list --installed | grep -q "^$triple$"; then
        warn "Installing Rust target $triple"
        rustup target add "$triple"
      fi
    done
  else
    if ! rustup target list --installed | grep -q "^$TARGET$"; then
      warn "Installing Rust target $TARGET"
      rustup target add "$TARGET"
    fi
  fi
  ok "Rust target $TARGET ready"
fi

step "Installing npm dependencies"
if [[ -f package-lock.json ]]; then
  npm ci
else
  npm install
fi
ok "Dependencies installed"

step "Building Tauri bundle"
if [[ -n "$TARGET" ]]; then
  npx tauri build --target "$TARGET"
else
  npx tauri build
fi

step "Done"
BUNDLE_BASE="src-tauri/target"
if [[ -n "$TARGET" ]]; then
  BUNDLE_BASE="$BUNDLE_BASE/$TARGET"
fi
BUNDLE_BASE="$BUNDLE_BASE/release/bundle"

if [[ -d "$BUNDLE_BASE" ]]; then
  echo
  echo "Artifacts:"
  find "$BUNDLE_BASE" -maxdepth 3 \( -name "*.app" -o -name "*.dmg" \) -print | sed 's/^/  /'
  echo
  echo "Drop the rathole binary next to the .app, then double-click to launch."
else
  warn "Bundle directory not found at $BUNDLE_BASE"
fi
