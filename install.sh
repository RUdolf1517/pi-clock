#!/usr/bin/env bash
# Собирает и устанавливает CLI pi-clock. Передайте --with-tray для команды tray.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WITH_TRAY=0
for arg in "$@"; do
  case "$arg" in
    --with-tray) WITH_TRAY=1 ;;
    -h|--help)
      echo "Использование: ./install.sh [--with-tray]"
      exit 0
      ;;
    *) echo "Неизвестный параметр: $arg" >&2; exit 2 ;;
  esac
done

if ! command -v cargo >/dev/null 2>&1; then
  if ! command -v curl >/dev/null 2>&1; then
    echo "Не найден cargo; установите Rust с https://rustup.rs и повторите запуск." >&2
    exit 1
  fi
  echo "Rust не найден. Устанавливаю минимальный Rust toolchain через rustup…"
  rustup_init="$(mktemp)"
  trap 'rm -f "$rustup_init"' EXIT
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs -o "$rustup_init"
  sh "$rustup_init" -y --profile minimal
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

build_args=(--bin pi-clock)
if (( WITH_TRAY )); then build_args+=(--features tray-app); fi
cargo build --release --manifest-path "$ROOT/Cargo.toml" "${build_args[@]}"

install_dir="${PI_CLOCK_INSTALL_DIR:-$HOME/.local/bin}"
"$ROOT/target/release/pi-clock" install --path "$install_dir"
if [[ ":${PATH}:" != *":${install_dir}:"* ]]; then
  printf 'Добавьте каталог в PATH: export PATH="%s:$PATH"\n' "$install_dir"
fi
printf '\nЗапуск: pi-clock (TUI) · pi-clock help (справка)\n'
if (( WITH_TRAY )); then printf 'Трей включён: pi-clock tray\n'; fi
