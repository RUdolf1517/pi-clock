//! Интерактивный полноэкранный интерфейс терминала.

use crate::{clock, display};
use std::io::{self, IsTerminal, Write};
#[cfg(not(unix))]
use std::io::BufRead;
#[cfg(unix)]
use std::io::Read;
use std::time::{Duration, Instant};

#[cfg(unix)]
struct Terminal {
    saved_stty: String,
}

#[cfg(not(unix))]
struct Terminal {
    keys: std::sync::mpsc::Receiver<char>,
}

impl Terminal {
    fn enter() -> io::Result<Self> {
        #[cfg(unix)]
        {
            use std::process::{Command, Stdio};
            if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "запустите pi-clock в интерактивном терминале (не через pipe)",
                ));
            }
            // Явно наследуем stdin: stty должен видеть управляющий TTY, даже
            // когда stdout процесса захватывается для чтения исходных настроек.
            let saved = Command::new("stty")
                .arg("-g")
                .stdin(Stdio::inherit())
                .output()?;
            if !saved.status.success() {
                let detail = String::from_utf8_lossy(&saved.stderr).trim().to_string();
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    format!("не удалось прочитать настройки TTY через stty: {detail}"),
                ));
            }
            let saved_stty = String::from_utf8_lossy(&saved.stdout).trim().to_string();
            let status = Command::new("stty")
                .args(["-echo", "-icanon", "-isig", "min", "0", "time", "1"])
                .stdin(Stdio::inherit())
                .status()?;
            if !status.success() {
                return Err(io::Error::other("Не удалось переключить терминал в режим клавиш"));
            }
            let terminal = Self { saved_stty };
            let mut stdout = io::stdout().lock();
            stdout.write_all(b"\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H")?;
            stdout.flush()?;
            Ok(terminal)
        }
        #[cfg(not(unix))]
        {
            // В Windows клавиши вводятся с Enter: стандартная библиотека Rust не
            // предоставляет переносимый режим чтения консоли без буферизации.
            let (tx, keys) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    match line {
                        Ok(line) => {
                            if let Some(ch) = line.chars().next() {
                                if tx.send(ch).is_err() { break; }
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
            print!("\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H");
            io::stdout().flush()?;
            Ok(Self { keys })
        }
    }

    fn read_key(&self) -> io::Result<Option<char>> {
        #[cfg(unix)]
        {
            let mut byte = [0u8; 1];
            match io::stdin().read(&mut byte)? {
                0 => Ok(None),
                _ => Ok(Some(byte[0] as char)),
            }
        }
        #[cfg(not(unix))]
        {
            match self.keys.recv_timeout(Duration::from_millis(100)) {
                Ok(key) => Ok(Some(key)),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Ok(None),
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Ok(None),
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            let _ = std::process::Command::new("stty").arg(&self.saved_stty).status();
        }
        let _ = io::stdout().write_all(b"\x1b[0m\x1b[?25h\x1b[?1049l");
        let _ = io::stdout().flush();
    }
}

/// Запускает TUI. Ошибки отображаются в обычном терминале после восстановления режима.
pub fn run() {
    if let Err(error) = run_inner() {
        eprintln!("Не удалось запустить π-clock TUI: {error}");
    }
}

fn run_inner() -> io::Result<()> {
    let terminal = Terminal::enter()?;
    let mut colored = true;
    let mut show_help = false;
    let mut last_frame: Option<Instant> = None;
    let mut last_time = (u32::MAX, u32::MAX, u32::MAX);

    loop {
        if let Some(key) = terminal.read_key()? {
            match key.to_ascii_lowercase() {
                'q' | '\u{1b}' | '\u{3}' => break,
                'c' => { colored = !colored; last_time = (u32::MAX, u32::MAX, u32::MAX); }
                '?' => { show_help = !show_help; last_time = (u32::MAX, u32::MAX, u32::MAX); }
                _ => {}
            }
        }

        if last_frame.is_none_or(|instant| instant.elapsed() >= Duration::from_millis(200)) {
            let (hour, minute, second) = clock::local_time().map_err(io::Error::other)?;
            if (hour, minute, second) != last_time {
                draw(hour, minute, second, colored, show_help)?;
                last_time = (hour, minute, second);
                last_frame = Some(Instant::now());
            }
        }
    }
    Ok(())
}

fn draw(hour: u32, minute: u32, second: u32, colored: bool, show_help: bool) -> io::Result<()> {
    let pi_fraction = clock::full_time_to_pi_fraction(hour, minute, second);
    let hour_part = clock::hour_to_pi_fraction(hour).plain();
    let minute_part = clock::minute_to_pi_fraction(minute).plain();
    let second_part = clock::second_to_pi_fraction(second).plain();
    let digital = format!("({hour_part}:{minute_part}:{second_part})π = {pi_fraction}");
    let face = display::render_circular_clock(hour, minute, second, &pi_fraction);
    let mut out = String::from("\x1b[H\x1b[2J");
    if colored {
        out.push_str("\x1b[1;96m╭─ π-ЧАСЫ ─────────────────────────────────────────╮\x1b[0m\n");
        out.push_str(&format!("\x1b[96m│\x1b[0m Время: {hour:02}:{minute:02}:{second:02}  ·  полный 24-часовой ход (0…4π)\n"));
        out.push_str(&format!("\x1b[96m│\x1b[0m π-время: \x1b[1;93m{digital}\x1b[0m\n"));
        out.push_str("\x1b[96m│\x1b[0m Циферблат 12-часовой: позиции 0 и 12 часов совпадают\n");
    } else {
        out.push_str("╭─ π-ЧАСЫ ─────────────────────────────────────────╮\n");
        out.push_str(&format!("│ Время: {hour:02}:{minute:02}:{second:02}  ·  полный 24-часовой ход (0…4π)\n"));
        out.push_str(&format!("│ π-время: {digital}\n"));
        out.push_str("│ Циферблат 12-часовой: позиции 0 и 12 часов совпадают\n");
    }
    out.push_str(&face);
    if colored {
        out.push_str("\x1b[96m*\x1b[0m часовая   \x1b[92m+\x1b[0m минутная   \x1b[93m.\x1b[0m секундная   │ q выход · c цвет · ? справка\n");
    } else {
        out.push_str("* часовая   + минутная   . секундная   │ q выход · c цвет · ? справка\n");
    }
    if show_help {
        out.push_str("Стрелки движутся плавно; аналоговый циферблат повторяется каждые 12 часов.\n");
        out.push_str("Нажмите q или Esc для выхода, c — переключить цвет, ? — скрыть справку.\n");
    }
    let mut stdout = io::stdout().lock();
    stdout.write_all(out.as_bytes())?;
    stdout.flush()
}
