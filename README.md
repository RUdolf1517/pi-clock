# π-clock

**[English](#english) | [Русский](#русский)**

---

## English

A Rust CLI clock that expresses local time as rational multiples of π. All app actions go through one executable: `pi-clock`.

### Time model

The 24-hour day maps to four turns: `1 hour = π/6`, `1 minute = π/360`, `1 second = π/21600`. For `H:M:S`, the coefficient is `H/6 + M/360 + S/21600`; fractions are reduced automatically. The analog dial is 12-hour and its hands move smoothly, so the 0 and 12 positions coincide. The numeric π value still follows the full 24-hour day.

Example at 03:30:45:

```text
(1/2:1/12:1/480)π = 221π/480
```

### Install

Requirements: Rust/Cargo 1.85 or newer. Run the platform installer from the repository directory:

```bash
# Linux / macOS
./install.sh

# Windows PowerShell
.\install.ps1
```

The installer bootstraps Rust if needed, builds the single `pi-clock` CLI, and installs it to `~/.local/bin` (Linux/macOS) or `%LOCALAPPDATA%\Programs\pi-clock` (Windows). Add the directory to `PATH` if the installer says it is missing. To include the tray command, use `./install.sh --with-tray` or `./install.ps1 -WithTray`.

### CLI commands

```text
pi-clock [tui]               launch the interactive terminal UI (default)
pi-clock tray                launch the system tray app
pi-clock install [--path DIR] install this executable
pi-clock help                show command help
pi-clock version             show version
```

The tray is optional and must be compiled with the `tray-app` feature. From the repository, for example:

```bash
cargo run --bin pi-clock -- help
```

For Cargo, features belong before the `--` that separates Cargo arguments from app arguments. Use:

```bash
cargo run --features tray-app --bin pi-clock -- tray
```

In the TUI, press `q` or `Esc` to quit (`Ctrl+C` also works on Unix), `c` to toggle color, and `?` to show or hide help. Unix terminals accept keys immediately; on Windows, press Enter after a command. Building the tray requires native platform libraries used by `eframe` and `tray-icon`.

### Versioning and releases

`Cargo.toml` is the source of truth for the SemVer version (`MAJOR.MINOR.PATCH`); `pi-clock version` prints it. Record changes in `CHANGELOG.md`. To publish, update the package version and changelog, then push a matching `vX.Y.Z` tag. The release workflow verifies the tag, tests/builds Linux, macOS, and Windows binaries, and creates a GitHub release with the packaged executables.

### Tests

```bash
cargo test
```

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
install.sh        Linux/macOS installer
install.ps1       Windows installer
.github/workflows/ci.yml      cross-platform tests on push/PR
.github/workflows/release.yml tag-driven SemVer release builds
CHANGELOG.md      release notes

---

## Русский

CLI-часы на Rust, которые показывают локальное время через рациональные коэффициенты числа π. Все действия запускаются из одного исполняемого файла: `pi-clock`.

### Модель времени

Сутки в 24-часовом формате соответствуют четырём оборотам: `1 час = π/6`, `1 минута = π/360`, `1 секунда = π/21600`. Для `ЧЧ:ММ:СС` коэффициент равен `Ч/6 + М/360 + С/21600`; дроби автоматически сокращаются. Аналоговый циферблат 12-часовой, стрелки движутся плавно, позиции 0 и 12 совпадают. Числовое π-время учитывает полные 24 часа.

Пример для 03:30:45:

```text
(1/2:1/12:1/480)π = 221π/480
```

### Установка

Требуется Rust/Cargo 1.85 или новее. Запустите установщик из каталога проекта:

```bash
# Linux / macOS
./install.sh

# Windows PowerShell
.\install.ps1
```

Скрипт при необходимости установит Rust, соберёт единый CLI `pi-clock` и установит его в `~/.local/bin` (Linux/macOS) или `%LOCALAPPDATA%\Programs\pi-clock` (Windows). Если установщик сообщит, что каталога нет в `PATH`, добавьте его. Чтобы включить команду трея, запустите `./install.sh --with-tray` или `./install.ps1 -WithTray`.

### Команды CLI

```text
pi-clock [tui]               запустить интерактивный TUI (по умолчанию)
pi-clock tray                запустить приложение в системном трее
pi-clock install [--path DIR] установить этот исполняемый файл
pi-clock help                показать справку
pi-clock version             показать версию
```

Трей необязателен и собирается с feature `tray-app`. Примеры запуска из каталога проекта:

```bash
cargo run --bin pi-clock -- help
cargo run --features tray-app --bin pi-clock -- tray
```

В TUI: `q` или `Esc` — выход (`Ctrl+C` также работает в Unix), `c` — переключить цвет, `?` — показать/скрыть справку. В Unix-терминалах клавиши работают сразу; в Windows команду нужно подтвердить клавишей Enter. Для трея нужны системные библиотеки, требуемые `eframe` и `tray-icon`.

### Версии и релизы

Единственный источник версии SemVer (`MAJOR.MINOR.PATCH`) — поле `version` в `Cargo.toml`; команду можно проверить через `pi-clock version`. Изменения фиксируются в `CHANGELOG.md`. Для релиза обновите версию и changelog, затем отправьте соответствующий тег `vX.Y.Z`. Release workflow проверит совпадение версии, соберёт бинарники для Linux, macOS и Windows и создаст GitHub Release.

### Тесты

```bash
cargo test
```

### Структура проекта

```text
src/
├── lib.rs        библиотечные модули
├── main.rs       диспетчер CLI: tui, tray, install, help, version
├── tui.rs        интерактивный TUI и управление терминалом
├── tray.rs       необязательное приложение в трее (feature tray-app)
├── fraction.rs   арифметика рациональных чисел и форматирование π
├── clock.rs      перевод времени, локальное время, положение стрелок
└── display.rs    компактный 12-часовой текстовый циферблат
install.sh        установщик Linux/macOS
install.ps1       установщик Windows
```
