#!/bin/sh
# One-line binary install: curl -fsSL https://raw.githubusercontent.com/RUdolf1517/pi-clock/main/install.sh | sh
set -eu

REPO=${PI_CLOCK_REPO:-RUdolf1517/pi-clock}
BRANCH=${PI_CLOCK_BRANCH:-main}
VERSION=${PI_CLOCK_VERSION:-}
INSTALL_DIR=${PI_CLOCK_INSTALL_DIR:-"$HOME/.local/bin"}

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      cat <<'EOF'
Usage: install.sh

Downloads and verifies the latest pi-clock release for Linux/macOS.
Set PI_CLOCK_VERSION (without the leading v) to pin a release.
Set PI_CLOCK_INSTALL_DIR to change the destination.
If no release is available, Cargo is used when installed.
EOF
      exit 0
      ;;
    *) printf 'pi-clock-install: unknown option: %s\n' "$arg" >&2; exit 2 ;;
  esac
done

say() { printf '%s\n' "$*"; }
err() { printf 'pi-clock-install: %s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1; }
download() {
  if need curl; then curl -fsSL "$1" -o "$2"
  elif need wget; then wget -qO "$2" "$1"
  else return 1
  fi
}

os=$(uname -s)
arch=$(uname -m)
case "$os" in
  Linux)  target_os=unknown-linux-gnu ;;
  Darwin) target_os=apple-darwin ;;
  *) err "unsupported OS: $os (supported: Linux and macOS)" ;;
esac
case "$arch" in
  x86_64|amd64)  target_arch=x86_64 ;;
  arm64|aarch64) target_arch=aarch64 ;;
  *) err "unsupported architecture: $arch" ;;
esac

asset="pi-clock-${target_arch}-${target_os}.tar.gz"
if [ -n "$VERSION" ]; then
  release_url="https://github.com/${REPO}/releases/download/v${VERSION}"
else
  release_url="https://github.com/${REPO}/releases/latest/download"
fi
mkdir -p "$INSTALL_DIR"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

# Prefer a prebuilt HTTPS GitHub Release binary: no Rust toolchain is needed.
if download "$release_url/$asset" "$tmp/$asset" 2>/dev/null; then
  if download "$release_url/$asset.sha256" "$tmp/$asset.sha256" 2>/dev/null; then
    if need sha256sum; then (cd "$tmp" && sha256sum -c "$asset.sha256")
    elif need shasum; then (cd "$tmp" && shasum -a 256 -c "$asset.sha256")
    else err "need sha256sum or shasum to verify the downloaded binary"
    fi
  else
    err "downloaded release has no SHA-256 checksum: $asset.sha256"
  fi
  tar -C "$tmp" -xzf "$tmp/$asset"
  [ -f "$tmp/pi-clock" ] || err "release archive does not contain pi-clock"
  chmod 755 "$tmp/pi-clock"
  "$tmp/pi-clock" install --path "$INSTALL_DIR"
  say "installed version: $("$INSTALL_DIR/pi-clock" version)"
else
  if ! need cargo; then
    err "no prebuilt release for $target_arch-$target_os and cargo is not installed. See https://github.com/${REPO}/releases"
  fi
  say "release binary unavailable; building from source with cargo…"
  if [ "$os" = Darwin ]; then
    cargo install --git "https://github.com/${REPO}" --branch "$BRANCH" --root "$tmp/cargo" --force --features tray-app
  else
    cargo install --git "https://github.com/${REPO}" --branch "$BRANCH" --root "$tmp/cargo" --force
  fi
  [ -x "$tmp/cargo/bin/pi-clock" ] || err "cargo did not produce pi-clock"
  "$tmp/cargo/bin/pi-clock" install --path "$INSTALL_DIR"
  say "installed version: $("$INSTALL_DIR/pi-clock" version)"
fi

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) say "add to PATH: export PATH=\"$INSTALL_DIR:\$PATH\"" ;;
esac
say "run: pi-clock (TUI) · pi-clock help (commands)"
