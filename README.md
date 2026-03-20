# π-clock

**[English](#english) | [Русский](#русский)**

---

## English

A clock that represents time as fractions of π, written in Rust. Two modes: a terminal clock and a system tray app.

### How it works

Standard time is mapped to radians:

| Unit     | Value  |
|----------|--------|
| 1 hour   | π/6    |
| 1 minute | π/360  |
| 24 hours | 4π     |

The current time is expressed as two reduced fractions — one for hours, one for minutes — displayed in the format `(h:m)π`:

```
3:30  →  hours: 3 × π/6 = π/2   →  fraction: 1/2
         minutes: 30 × π/360 = π/12  →  fraction: 1/12
         combined: π/2 + π/12 = 7π/12

display: (1/2:1/12)π = 7π/12
```

All fractions are always reduced using GCD.

### Preview

**Terminal clock:**
```
╔══════════════════════════════════════╗
║         π-ЧАСЫ (электронные)         ║
╠══════════════════════════════════════╣
║                                      ║
║   Время: 03:30:00                     ║
║   π-время: (1/2:1/12)π = 7π/12         ║
║                                      ║
╚══════════════════════════════════════╝

╔══════════════════════════════════════════════╗
║            π-ЧАСЫ (круговые)                ║
╠══════════════════════════════════════════════╣
║  * часовая  + минутная                      ║
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

    Текущее положение стрелки: 7π/12
```

**System tray (macOS menu bar):**

```
(1/2:1/12)π          ← text in menu bar, updates every minute
```

Clicking the icon opens a popup window with both clock modes.

### Requirements

- Rust 1.80+ (edition 2024)
- macOS or Windows
- No extra dependencies for the terminal binary
- For the tray app: `eframe` + `tray-icon` (automatically downloaded by Cargo)

### Installation

```bash
git clone https://github.com/rudolf1517/pi-clock
cd pi-clock
```

### Running

**Terminal clock:**
```bash
cargo run --bin pi-clock
```

**System tray app** (macOS menu bar / Windows notification area):
```bash
cargo run --bin tray
```

To build optimized release binaries:
```bash
cargo build --release
# binaries will be in ./target/release/pi-clock and ./target/release/tray
```

On macOS you can copy the tray binary to `/usr/local/bin/` and add it to Login Items to auto-start on boot.

### Project structure

```
src/
├── lib.rs        — library root, re-exports modules for the tray binary
├── main.rs       — terminal clock entry point
├── fraction.rs   — Fraction struct: GCD, addition, Display (as nπ/d), plain() (as n/d)
├── clock.rs      — time → π-fraction conversion
├── display.rs    — ASCII circular clock renderer
└── bin/
    └── tray.rs   — system tray app (eframe GUI + tray-icon)
```

### Running tests

```bash
cargo test
```

13 unit tests cover GCD, fraction reduction, addition, Display formatting, and time conversion.

---

## Русский

Часы, которые выражают текущее время через дроби числа π. Написаны на Rust. Два режима: терминальные часы и системный трей.

### Как это работает

Стандартное время переводится в радианы по формулам:

| Единица  | Значение |
|----------|----------|
| 1 час    | π/6      |
| 1 минута | π/360    |
| 24 часа  | 4π       |

Текущее время отображается как две сокращённые дроби — отдельно для часов и минут — в формате `(ч:м)π`:

```
3:30  →  часы: 3 × π/6 = π/2    →  дробь: 1/2
         минуты: 30 × π/360 = π/12  →  дробь: 1/12
         сумма: π/2 + π/12 = 7π/12

отображение: (1/2:1/12)π = 7π/12
```

Все дроби всегда сокращаются через НОД (алгоритм Евклида).

### Предварительный просмотр

**Терминальные часы:**
```
╔══════════════════════════════════════╗
║         π-ЧАСЫ (электронные)         ║
╠══════════════════════════════════════╣
║                                      ║
║   Время: 03:30:00                     ║
║   π-время: (1/2:1/12)π = 7π/12         ║
║                                      ║
╚══════════════════════════════════════╝
```

Круговые часы отображают циферблат от 0 до 2π с часовой (`*`) и минутной (`+`) стрелками.

**Трей (меню-бар macOS):**
```
(1/2:1/12)π          ← текст в меню-баре, обновляется каждую минуту
```

При клике по иконке открывается всплывающее окно с обоими режимами.

### Требования

- Rust 1.80+ (edition 2024)
- macOS или Windows
- Для терминального бинарника: только стандартная библиотека (`std`)
- Для трей-приложения: `eframe` + `tray-icon` (скачиваются через Cargo автоматически)

### Установка

```bash
git clone https://github.com/rudolf1517/pi-clock
cd pi-clock
```

### Запуск

**Терминальные часы:**
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
# готовые файлы: ./target/release/pi-clock и ./target/release/tray
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
├── main.rs       — точка входа терминальных часов
├── fraction.rs   — структура Fraction: НОД, сложение, Display (nπ/d), plain() (n/d)
├── clock.rs      — преобразование времени в π-дроби
├── display.rs    — рендеринг ASCII-циферблата
└── bin/
    └── tray.rs   — трей-приложение (eframe GUI + tray-icon)
```

### Тесты

```bash
cargo test
```

13 юнит-тестов покрывают: НОД, сокращение дробей, сложение, форматирование и преобразование времени.

### Математика подробнее

Дробь π для произвольного момента времени H:M:

```
f(H, M) = H/6 + M/360
```

Вычисляется через сложение рациональных чисел с последующим сокращением:

```
a/b + c/d = (a×d + c×b) / (b×d)
НОД(числитель, знаменатель) → деление обоих на НОД
```

Примеры:

| Время | Часовая дробь | Минутная дробь | π-время       |
|-------|---------------|----------------|---------------|
| 0:00  | 0             | 0              | 0             |
| 3:00  | 1/2           | 0              | π/2           |
| 6:00  | 1             | 0              | π             |
| 9:00  | 3/2           | 0              | 3π/2          |
| 3:30  | 1/2           | 1/12           | 7π/12         |
| 12:00 | 2             | 0              | 2π            |
| 20:35 | 10/3          | 7/72           | 103π/72       |
