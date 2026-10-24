# π-clock

**[English](#english) | [Русский](#русский)**

---

## English

A Rust clock that expresses local time as rational multiples of π. One `pi-clock` command opens the terminal dial and, in tray-enabled builds, starts the system-tray app alongside it.

### Time model

The 24-hour day maps to four turns: `1 hour = π/6`, `1 minute = π/360`, `1 second = π/21600`. For `H:M:S`, the coefficient is `H/6 + M/360 + S/21600`; fractions are reduced automatically. The analog dial is 12-hour and its hands move smoothly, so the 0 and 12 positions coincide. The numeric π value still follows the full 24-hour day.

Example at 03:30:45:

```text
(1/2:1/12:1/480)π = 281π/480
```

### Install

**Linux / macOS** — downloads and verifies the latest prebuilt release; no Rust needed:

```bash
curl -fsSL https://raw.githubusercontent.com/RUdolf1517/pi-clock/master/install.sh | sh
```

**Windows PowerShell** — x64 release:

```powershell
irm https://raw.githubusercontent.com/RUdolf1517/pi-clock/master/install.ps1 | iex
```

The scripts install to `~/.local/bin` on Linux/macOS and `%LOCALAPPDATA%\Programs\pi-clock` on Windows. Set `PI_CLOCK_VERSION` (for example `0.1.0`, without the `v` prefix) to pin a release or `PI_CLOCK_INSTALL_DIR` to choose another destination. If no matching release asset exists, the Unix script builds from source when Cargo is installed; the Windows script does the same. Otherwise, see the [releases page](https://github.com/RUdolf1517/pi-clock/releases).

Release binaries: Linux x86_64/aarch64 (TUI), macOS Apple Silicon/Intel (TUI + tray), Windows x64 (TUI + tray). The Linux release intentionally avoids GUI system-library dependencies; to build the tray there, compile with `--features tray-app`.

### CLI commands

```text
pi-clock [tui]               launch the TUI and tray (if included in this build)
pi-clock --no-tray            launch only the TUI
pi-clock tray                launch only the system-tray app
pi-clock install [--path DIR] install this executable
pi-clock help                show command help
pi-clock version             show version
```

The default Linux release is TUI-only to avoid GUI system-library dependencies; the macOS and Windows releases include the tray. Build with `--features tray-app` to enable automatic tray startup in any supported build. Use `pi-clock --no-tray` (or `pi-clock tui --no-tray`) to suppress the tray.

From the repository:

```bash
cargo run --features tray-app --bin pi-clock --       # TUI + tray
cargo run --bin pi-clock -- --no-tray                 # TUI only
cargo run --bin pi-clock -- help
```

In the TUI, press `q` or `Esc` to quit (`Ctrl+C` also works on Unix), `c` to toggle color, and `?` to show or hide help. Unix terminals accept keys immediately; on Windows, press Enter after a command. Building the tray requires native platform libraries used by `eframe` and `tray-icon`.

### Versioning and releases

`Cargo.toml` is the source of truth for the SemVer version (`MAJOR.MINOR.PATCH`); `pi-clock version` prints it. Record changes in `CHANGELOG.md`. Update the package version and changelog, then push a matching `vX.Y.Z` tag. The release workflow verifies the tag, builds platform archives and SHA-256 checksums, and creates a GitHub release.

### Tests and CI

```bash
cargo test
```

GitHub Actions runs tests and CLI smoke checks on Linux, macOS, and Windows for every push and pull request.

### Project structure

```text
src/
├── lib.rs        library modules
├── main.rs       CLI dispatcher: tui, tray, install, help, version
├── tui.rs        interactive terminal UI and terminal-mode handling
├── tray.rs       optional system-tray app (tray-app feature)
├── fraction.rs   rational arithmetic and π formatting
├── clock.rs      time conversion, local-time access, hand positions
└── display.rs    compact 12-hour text dial
install.sh        Linux/macOS binary installer
install.ps1       Windows binary installer
.github/workflows/ci.yml      cross-platform tests on push/PR
.github/workflows/release.yml tag-driven SemVer releases
CHANGELOG.md      release notes
```

---

## Русский

CLI-часы на Rust, которые показывают локальное время через рациональные коэффициенты числа π. Все команды запускаются из одного исполняемого файла: `pi-clock`.

### Модель времени

Сутки в 24-часовом формате соответствуют четырём оборотам: `1 час = π/6`, `1 минута = π/360`, `1 секунда = π/21600`. Для `ЧЧ:ММ:СС` коэффициент равен `Ч/6 + М/360 + С/21600`; дроби автоматически сокращаются. Аналоговый циферблат 12-часовой, стрелки движутся плавно, позиции 0 и 12 совпадают. Числовое π-время учитывает полные 24 часа.

Пример для 03:30:45:

```text
(1/2:1/12:1/480)π = 281π/480
```

### Установка

**Linux / macOS** — скачивает последнюю готовую сборку и проверяет SHA-256; Rust не нужен:

```bash
curl -fsSL https://raw.githubusercontent.com/RUdolf1517/pi-clock/master/install.sh | sh
```

**Windows PowerShell** — готовая сборка для x64:

```powershell
irm https://raw.githubusercontent.com/RUdolf1517/pi-clock/master/install.ps1 | iex
```

Скрипты устанавливают приложение в `~/.local/bin` (Linux/macOS) и `%LOCALAPPDATA%\Programs\pi-clock` (Windows). Переменная `PI_CLOCK_VERSION` (например, `0.1.0`, без префикса `v`) позволяет выбрать версию, а `PI_CLOCK_INSTALL_DIR` — каталог установки. Если подходящего релизного бинарника нет, скрипты соберут его из исходников при наличии Cargo; иначе откройте [страницу релизов](https://github.com/RUdolf1517/pi-clock/releases).

Готовые сборки: Linux x86_64/aarch64 (TUI), macOS Apple Silicon/Intel (TUI + трей), Windows x64 (TUI + трей). В Linux-сборке нет GUI-зависимостей; чтобы собрать там трей, используйте feature `tray-app`.

### Команды CLI

```text
pi-clock [tui]               запустить TUI и трей (если он включён в сборку)
pi-clock --no-tray            запустить только TUI
pi-clock tray                запустить только системный трей
pi-clock install [--path DIR] установить этот исполняемый файл
pi-clock help                показать справку
pi-clock version             показать версию
```

В готовых релизах Linux включён только TUI — без GUI-зависимостей; релизы macOS и Windows включают трей. Чтобы включить автозапуск трея, соберите программу с `--features tray-app`. Для запуска одного TUI используйте `pi-clock --no-tray` или `pi-clock tui --no-tray`.

Запуск из каталога проекта:

```bash
cargo run --features tray-app --bin pi-clock --       # TUI + трей
cargo run --bin pi-clock -- --no-tray                 # только TUI
cargo run --bin pi-clock -- help
```

В TUI: `q` или `Esc` — выход (`Ctrl+C` также работает в Unix), `c` — переключить цвет, `?` — показать/скрыть справку. В Unix-терминалах клавиши работают сразу; в Windows команду нужно подтвердить клавишей Enter. Для сборки трея нужны системные библиотеки `eframe` и `tray-icon`.

### Версии и релизы

Единственный источник версии SemVer (`MAJOR.MINOR.PATCH`) — поле `version` в `Cargo.toml`; её показывает `pi-clock version`. Изменения фиксируются в `CHANGELOG.md`. Обновите версию и changelog, затем отправьте тег `vX.Y.Z`. Release workflow проверит соответствие версии, соберёт архивы и SHA-256, затем создаст GitHub Release.

### Тесты и CI

```bash
cargo test
```

GitHub Actions автоматически тестирует проект и CLI-команды на Linux, macOS и Windows при каждом push и pull request.
