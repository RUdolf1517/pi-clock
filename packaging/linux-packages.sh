#!/bin/sh
# Build .deb and .rpm packages from the platform release archives.
# Usage: packaging/linux-packages.sh VERSION DIST_DIR
set -eu

VERSION=${1:?usage: linux-packages.sh VERSION DIST_DIR}
DIST=${2:?usage: linux-packages.sh VERSION DIST_DIR}
NFPM_VERSION=${NFPM_VERSION:-2.48.0}
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
MAN_PAGE="$SCRIPT_DIR/../man/pi-clock.1"

command -v curl >/dev/null 2>&1 || { echo "need curl to download nfpm" >&2; exit 1; }
command -v tar >/dev/null 2>&1 || { echo "need tar" >&2; exit 1; }
mkdir -p "$DIST"

workdir=$(mktemp -d)
trap 'rm -rf "$workdir"' EXIT HUP INT TERM
nfpm_archive="nfpm_${NFPM_VERSION}_Linux_x86_64.tar.gz"
curl --retry 3 -fsSL \
  "https://github.com/goreleaser/nfpm/releases/download/v${NFPM_VERSION}/${nfpm_archive}" \
  -o "$workdir/$nfpm_archive"
tar -xzf "$workdir/$nfpm_archive" -C "$workdir" nfpm
nfpm="$workdir/nfpm"
chmod +x "$nfpm"

package_one() {
  target=$1
  deb_arch=$2
  rpm_arch=$3
  archive="$DIST/pi-clock-${target}.tar.gz"
  [ -f "$archive" ] || { echo "missing release archive: $archive" >&2; exit 1; }

  package_dir="$workdir/$target"
  mkdir -p "$package_dir"
  tar -xzf "$archive" -C "$package_dir"
  binary="$package_dir/pi-clock"
  [ -f "$binary" ] || { echo "archive does not contain pi-clock: $archive" >&2; exit 1; }
  chmod 755 "$binary"

  config="$package_dir/nfpm.yaml"
  cat > "$config" <<EOF
name: pi-clock
arch: $deb_arch
platform: linux
version: $VERSION
release: "1"
section: utils
priority: optional
maintainer: RUdolf1517 <rudolfzinovev@protonmail.com>
vendor: RUdolf1517
description: Terminal clock that displays time as rational multiples of pi
homepage: https://github.com/RUdolf1517/pi-clock
license: MIT
contents:
  - src: $binary
    dst: /usr/bin/pi-clock
    file_info:
      mode: 0755
EOF
  if [ -f "$MAN_PAGE" ]; then
    cat >> "$config" <<EOF
  - src: $MAN_PAGE
    dst: /usr/share/man/man1/pi-clock.1
EOF
  fi

  "$nfpm" pkg --packager deb --config "$config" --target "$DIST/pi-clock_${deb_arch}.deb"
  "$nfpm" pkg --packager rpm --config "$config" --target "$DIST/pi-clock.${rpm_arch}.rpm"
}

package_one x86_64-unknown-linux-gnu amd64 x86_64
package_one aarch64-unknown-linux-gnu arm64 aarch64
