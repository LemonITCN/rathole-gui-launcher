#!/usr/bin/env bash
# Build the launcher for Linux (x86_64).
#
# Usage:
#   scripts/build-linux.sh
#
# Output:
#   src-tauri/target/release/rathole-gui-launcher       (raw binary)
#   src-tauri/target/release/bundle/deb/*.deb
#   src-tauri/target/release/bundle/appimage/*.AppImage
#   src-tauri/target/release/bundle/rpm/*.rpm           (only when rpmbuild is installed)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BLUE='\033[0;34m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; NC='\033[0m'
step() { printf "\n${BLUE}==>${NC} %s\n" "$1"; }
ok()   { printf "${GREEN}✓${NC}  %s\n" "$1"; }
warn() { printf "${YELLOW}!${NC}  %s\n" "$1"; }
err()  { printf "${RED}✗${NC}  %s\n" "$1" >&2; }

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  sed -n '2,11p' "$0" | sed 's/^# \{0,1\}//'
  exit 0
fi

step "Checking prerequisites"

if [[ "$(uname)" != "Linux" ]]; then
  err "This script must be run on Linux."
  exit 1
fi

if ! command -v node >/dev/null 2>&1; then
  err "Node.js is not installed. Install Node 18+ from your package manager or nvm."
  exit 1
fi
ok "Node.js $(node --version)"

if ! command -v cargo >/dev/null 2>&1; then
  err "Rust toolchain not found. Install it from https://rustup.rs"
  exit 1
fi
ok "Rust $(rustc --version | awk '{print $2}')"

if ! command -v pkg-config >/dev/null 2>&1; then
  err "pkg-config is required."
  echo "    Debian / Ubuntu: sudo apt-get install -y pkg-config"
  echo "    Fedora:          sudo dnf install -y pkgconf-pkg-config"
  echo "    Arch:            sudo pacman -S pkgconf"
  exit 1
fi
ok "pkg-config"

REQUIRED_PKGS=(webkit2gtk-4.1 javascriptcoregtk-4.1 libsoup-3.0 gtk+-3.0)
MISSING=()
for p in "${REQUIRED_PKGS[@]}"; do
  pkg-config --exists "$p" 2>/dev/null || MISSING+=("$p")
done
if (( ${#MISSING[@]} > 0 )); then
  err "Missing system development libraries: ${MISSING[*]}"
  cat <<HINT

Install them with one of:

  Debian / Ubuntu:
    sudo apt-get update && sudo apt-get install -y \\
      libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev \\
      libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf

  Fedora:
    sudo dnf install -y webkit2gtk4.1-devel javascriptcoregtk4.1-devel \\
      libsoup3-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel patchelf

  Arch / Manjaro:
    sudo pacman -S --needed webkit2gtk-4.1 libsoup3 gtk3 libappindicator-gtk3 \\
      librsvg patchelf
HINT
  exit 1
fi
ok "GTK / WebKit2GTK 4.1 development libraries"

step "Installing npm dependencies"
if [[ -f package-lock.json ]]; then
  npm ci
else
  npm install
fi
ok "Dependencies installed"

step "Building Tauri bundle"
npx tauri build

step "Done"
BUNDLE_BASE="src-tauri/target/release/bundle"
RAW_BIN="src-tauri/target/release/rathole-gui-launcher"

echo
echo "Artifacts:"
[[ -f "$RAW_BIN" ]] && echo "  $RAW_BIN"
if [[ -d "$BUNDLE_BASE" ]]; then
  find "$BUNDLE_BASE" -maxdepth 3 \( -name "*.deb" -o -name "*.AppImage" -o -name "*.rpm" \) -print | sed 's/^/  /'
fi
echo
echo "The .AppImage is a self-contained executable."
echo "Drop the rathole binary into the launcher's working directory before launching."
