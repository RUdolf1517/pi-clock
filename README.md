# π-clock

**[English](#english) | [Русский](#русский)**

---

## English

A clock that represents time as fractions of π, written in Rust. Two modes: an animated terminal clock and a system tray app.

### How it works

Standard time is mapped to radians:

| Unit      | Value   |
|-----------|---------|
| 1 hour    | π/6     |
| 1 minute  | π/360   |
| 1 second  | π/21600 |
| 24 hours  | 4π      |

Each unit is converted to a reduced fraction independently, then displayed in the format `(h:m:s)π`:

```
3:30:45  →  hours:   3  × π/6     = π/2      →  1/2
             minutes: 30 × π/360   = π/12     →  1/12
             seconds: 45 × π/21600 = π/480    →  1/480
             combined: π/2 + π/12 + π/480 = 221π/480

display: (1/2:1/12:1/480)π = 221π/480
```

All fractions are always reduced using GCD (Euclidean algorithm).

### Preview

**Terminal clock** (animated, updates every second, Ctrl+C to quit):
```
╔══════════════════════════════════════╗
║         π-ЧАСЫ (электронные)         ║
╠══════════════════════════════════════╣
║                                      ║
║   Время: 03:30:45                     ║
║   π-время: (1/2:1/12:1/480)π = 221π/480║
║                                      ║
╚══════════════════════════════════════╝

╔══════════════════════════════════════════════╗
║            π-ЧАСЫ (круговые)                ║
╠══════════════════════════════════════════════╣
║  * часовая  + минутная  . секундная         ║
╠══════════════════════════════════════════════╣

                      [0]
       11π/6     ·····◎·····    π/6
             ·○··           ··○·
          ····                 ····
  5π/3   ··                       ··  π/3
        ○·                         ·○
       ·                             ·
      ··   * *                       ··
      ·         * *                   ·
3π/2  ○               ⊙* * * * * *◆  [π/2]
      ·                               ·
      ··                             ··
       ·                             ·
        ○·                         ·○
  4π/3   ··                       ··  2π/3
          ····                 ····
             ·○··           ··○·
         7π/6    ·····○·····    5π/6

                     π

    Текущее положение стрелки: 221π/480

  Ctrl+C для выхода
╚══════════════════════════════════════════════╝
```

**System tray (macOS menu bar):**
```
(1/2:1/12:1/480)π     ← text in menu bar, updates every second
```

Clicking the icon opens a popup window with both clock modes. The popup also updates every second — all three hands move in real time.

### Requirements

- Rust 1.80+ (edition 2024)
- macOS or Windows
- No extra dependencies for the terminal binary
- For the tray app: `eframe` + `tray-icon` (automatically downloaded by Cargo)

### Installation

```bash
git clone https://github.com/RUdolf1517/pi-clock
cd pi-clock
```

### Running

**Terminal clock** (animated, Ctrl+C to quit):
```bash
cargo run --bin pi-clock
```

**System tray app** (macOS menu bar / Windows notification area):
```bash
cargo run --bin tray
```

Release build (optimized):
```bash
cargo build --release
# binaries: ./target/release/pi-clock  and  ./target/release/tray
```

**Autostart on macOS:**
1. `cargo build --release`
2. Copy `target/release/tray` somewhere permanent, e.g. `~/Applications/pi-clock`
3. Open **System Settings → General → Login Items**
4. Click `+` and add the `tray` binary

**Autostart on Windows:**
1. `cargo build --release`
2. Create a shortcut to `target\release\tray.exe`
3. Press `Win + R`, type `shell:startup`
4. Move the shortcut into the startup folder

### Project structure

```
src/
├── lib.rs        — library root, re-exports modules for the tray binary
├── main.rs       — terminal clock entry point (animated loop)
├── fraction.rs   — Fraction struct: GCD, addition, Display (nπ/d), plain() (n/d)
├── clock.rs      — time → π-fraction conversion (h, m, s)
├── display.rs    — ASCII circular clock renderer (3 hands)
└── bin/
    └── tray.rs   — system tray app (eframe GUI + tray-icon)
```

### Tests

```bash
cargo test
```

13 unit tests cover GCD, fraction reduction, addition, Display formatting, and time conversion.

### Math

The full π-fraction for a moment H:M:S:

```
f(H, M, S) = H/6 + M/360 + S/21600
```

Fraction addition with reduction:

```
a/b + c/d = (a×d + c×b) / (b×d),  then divide by GCD
```

Examples:

| Time    | Hours | Minutes | Seconds | Combined      |
|---------|-------|---------|---------|---------------|
| 0:00:00 | 0     | 0       | 0       | 0             |
| 3:00:00 | 1/2   | 0       | 0       | π/2           |
| 6:00:00 | 1     | 0       | 0       | π             |
| 9:00:00 | 3/2   | 0       | 0       | 3π/2          |
| 3:30:00 | 1/2   | 1/12    | 0       | 7π/12         |
| 12:00:00| 2     | 0       | 0       | 2π            |
| 20:35:00| 10/3  | 7/72    | 0       | 103π/72       |
| 3:30:45 | 1/2   | 1/12    | 1/480   | 221π/480      |

---

## Русский

Часы, которые выражают текущее время через дроби числа π. Написаны на Rust. Два режима: анимированные терминальные часы и системный трей.

### Как это работает

Стандартное время переводится в радианы по формулам:

| Единица  | Значение |
|----------|----------|
| 1 час    | π/6      |
| 1 минута | π/360    |
| 1 секунда| π/21600  |
| 24 часа  | 4π       |

Каждая единица переводится в отдельную сокращённую дробь, затем все три отображаются в формате `(ч:м:с)π`:

```
3:30:45  →  часы:    3  × π/6     = π/2    →  дробь: 1/2
             минуты:  30 × π/360   = π/12   →  дробь: 1/12
             секунды: 45 × π/21600 = π/480  →  дробь: 1/480
             сумма: π/2 + π/12 + π/480 = 221π/480

отображение: (1/2:1/12:1/480)π = 221π/480
```

Все дроби всегда сокращаются через НОД (алгоритм Евклида).

### Предварительный просмотр

**Терминальные часы** (анимация, обновляются каждую секунду, выход — Ctrl+C):
```
╔══════════════════════════════════════╗
║         π-ЧАСЫ (электронные)         ║
╠══════════════════════════════════════╣
║                                      ║
║   Время: 03:30:45                     ║
║   π-время: (1/2:1/12:1/480)π = 221π/480║
║                                      ║
╚══════════════════════════════════════╝
```

Круговые часы отображают циферблат от 0 до 2π с тремя стрелками:
- `*` — часовая, `◆` — наконечник
- `+` — минутная, `◇` — наконечник
- `.` — секундная, `◦` — наконечник

**Трей (меню-бар macOS):**
```
(1/2:1/12:1/480)π     ← текст в меню-баре, обновляется каждую секунду
```

При клике по иконке открывается всплывающее окно с обоими режимами. Все три стрелки движутся в реальном времени.

### Требования

- Rust 1.80+ (edition 2024)
- macOS или Windows
- Для терминального бинарника: только стандартная библиотека (`std`)
- Для трей-приложения: `eframe` + `tray-icon` (скачиваются через Cargo автоматически)

### Установка

```bash
git clone https://github.com/RUdolf1517/pi-clock
cd pi-clock
```

### Запуск

**Терминальные часы** (анимация, Ctrl+C для выхода):
```bash
cargo run --bin pi-clock
```

**Трей-приложение** (меню-бар macOS / область уведомлений Windows):
```bash
cargo run --bin tray
```

Сборка оптимизированных бинарников:
```bash
cargo build --release
# готовые файлы: ./target/release/pi-clock  и  ./target/release/tray
```

**Автозапуск на macOS:**
1. Собери release-бинарник: `cargo build --release`
2. Скопируй `target/release/tray` в удобное место, например `~/Applications/pi-clock`
3. Открой **Системные настройки → Основные → Объекты входа**
4. Нажми `+` и добавь бинарник `tray`

**Автозапуск на Windows:**
1. Собери: `cargo build --release`
2. Создай ярлык для `target\release\tray.exe`
3. Нажми `Win + R`, введи `shell:startup`
4. Перенеси ярлык в открывшуюся папку автозагрузки

### Структура проекта

```
src/
├── lib.rs        — библиотечный корень, экспортирует модули для трея
├── main.rs       — точка входа терминальных часов (анимированный цикл)
├── fraction.rs   — структура Fraction: НОД, сложение, Display (nπ/d), plain() (n/d)
├── clock.rs      — преобразование времени в π-дроби (ч, м, с)
├── display.rs    — рендеринг ASCII-циферблата (3 стрелки)
└── bin/
    └── tray.rs   — трей-приложение (eframe GUI + tray-icon)
```

### Тесты

```bash
cargo test
```

13 юнит-тестов покрывают: НОД, сокращение дробей, сложение, форматирование и преобразование времени.

### Математика подробнее

Полная π-дробь для момента времени Ч:М:С:

```
f(Ч, М, С) = Ч/6 + М/360 + С/21600
```

Сложение дробей с сокращением:

```
a/b + c/d = (a×d + c×b) / (b×d),  затем деление на НОД
```

Примеры:

| Время    | Часы  | Минуты | Секунды | Итого         |
|----------|-------|--------|---------|---------------|
| 0:00:00  | 0     | 0      | 0       | 0             |
| 3:00:00  | 1/2   | 0      | 0       | π/2           |
| 6:00:00  | 1     | 0      | 0       | π             |
| 9:00:00  | 3/2   | 0      | 0       | 3π/2          |
| 3:30:00  | 1/2   | 1/12   | 0       | 7π/12         |
| 12:00:00 | 2     | 0      | 0       | 2π            |
| 20:35:00 | 10/3  | 7/72   | 0       | 103π/72       |
| 3:30:45  | 1/2   | 1/12   | 1/480   | 221π/480      |
