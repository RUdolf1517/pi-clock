//! Компактный 12-часовой циферблат с Unicode-стрелками.

use crate::{clock, fraction::Fraction};
use std::f64::consts::TAU;

const WIDTH: usize = 45;
const HEIGHT: usize = 17;
const RADIUS_X: f64 = 15.0;
const RADIUS_Y: f64 = 6.0;
const LABEL_RADIUS_X: f64 = 18.5;
const LABEL_RADIUS_Y: f64 = 7.5;
const CENTER_X: f64 = 21.0;
const CENTER_Y: f64 = 8.0;
const LABELS: [&str; 12] = [
    "0", "π/6", "π/3", "π/2", "2π/3", "5π/6",
    "π", "7π/6", "4π/3", "3π/2", "5π/3", "11π/6",
];

#[derive(Clone, Copy, Default)]
enum Ink {
    #[default]
    Plain,
    Ring,
    Tick,
    Hour,
    Minute,
    Second,
    Center,
}

#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    ink: Ink,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', ink: Ink::Plain }
    }
}

fn angle_for_turn(turn: f64) -> f64 {
    turn.rem_euclid(1.0) * TAU
}

/// Угол отсчитывается от отметки 0 сверху по часовой стрелке.
fn circle_xy(angle: f64, rx: f64, ry: f64) -> (f64, f64) {
    (CENTER_X + rx * angle.sin(), CENTER_Y - ry * angle.cos())
}

fn put(grid: &mut [Vec<Cell>], x: f64, y: f64, ch: char, ink: Ink) {
    let xi = x.round() as isize;
    let yi = y.round() as isize;
    if yi >= 0 && yi < HEIGHT as isize && xi >= 0 && xi < WIDTH as isize {
        grid[yi as usize][xi as usize] = Cell { ch, ink };
    }
}

fn hand_glyph(angle: f64) -> char {
    let dx = angle.sin();
    let dy = -angle.cos();
    if dx.abs() > 2.0 * dy.abs() {
        '─'
    } else if dy.abs() > 2.0 * dx.abs() {
        '│'
    } else if dx * dy < 0.0 {
        '╱'
    } else {
        '╲'
    }
}

fn draw_hand(grid: &mut [Vec<Cell>], turn: f64, tip: char, length: f64, ink: Ink) {
    let angle = angle_for_turn(turn);
    let steps = (RADIUS_X * length).ceil() as usize;
    let shaft = hand_glyph(angle);
    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let (x, y) = circle_xy(angle, RADIUS_X * length * t, RADIUS_Y * length * t);
        put(grid, x, y, if step == steps { tip } else { shaft }, ink);
    }
}

/// Рисует циферблат без оформления терминала (подходит для GUI и тестов).
pub fn render_circular_clock(hour: u32, minute: u32, second: u32, pi_fraction: &Fraction) -> String {
    render_clock(hour, minute, second, pi_fraction, false)
}

/// Цветная версия циферблата для TUI.
pub fn render_circular_clock_colored(
    hour: u32,
    minute: u32,
    second: u32,
    pi_fraction: &Fraction,
) -> String {
    render_clock(hour, minute, second, pi_fraction, true)
}

fn render_clock(
    hour: u32,
    minute: u32,
    second: u32,
    pi_fraction: &Fraction,
    colored: bool,
) -> String {
    let mut grid = vec![vec![Cell::default(); WIDTH]; HEIGHT];

    // Точечная окружность.
    for degree in 0..360 {
        let (x, y) = circle_xy(degree as f64 * TAU / 360.0, RADIUS_X, RADIUS_Y);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if grid[yi][xi].ch == ' ' {
            grid[yi][xi] = Cell { ch: '·', ink: Ink::Ring };
        }
    }

    // 12 часовых меток и подписи углов.
    for pos in 0..12 {
        let angle = pos as f64 * TAU / 12.0;
        let (x, y) = circle_xy(angle, RADIUS_X, RADIUS_Y);
        let marker = if pos % 3 == 0 { '◉' } else { '○' };
        put(&mut grid, x, y, marker, Ink::Tick);

        let (lx, ly) = circle_xy(angle, LABEL_RADIUS_X, LABEL_RADIUS_Y);
        let label = LABELS[pos];
        let cells = label.chars().count() as isize;
        let anchor_x = lx.round() as isize;
        let start_x = if angle.sin() < -0.1 {
            anchor_x - cells + 1
        } else if angle.sin().abs() <= 0.1 {
            anchor_x - cells / 2
        } else {
            anchor_x
        };
        let row = ly.round() as isize;
        for (i, ch) in label.chars().enumerate() {
            let col = start_x + i as isize;
            if row >= 0 && row < HEIGHT as isize && col >= 0 && col < WIDTH as isize {
                grid[row as usize][col as usize] = Cell { ch, ink: Ink::Plain };
            }
        }
    }

    // Стрелки непрерывно перемещаются в течение каждой минуты и секунды.
    draw_hand(&mut grid, clock::second_turn(second), '◦', 1.0, Ink::Second);
    draw_hand(&mut grid, clock::minute_turn(minute, second), '◇', 0.82, Ink::Minute);
    draw_hand(&mut grid, clock::hour_turn(hour, minute, second), '◆', 0.62, Ink::Hour);
    put(&mut grid, CENTER_X, CENTER_Y, '⊙', Ink::Center);

    let mut result = String::new();
    for row in &grid {
        let end = row.iter().rposition(|cell| cell.ch != ' ').map_or(0, |index| index + 1);
        for cell in &row[..end] {
            if colored {
                result.push_str(ink_code(cell.ink));
            }
            result.push(cell.ch);
            if colored && !matches!(cell.ink, Ink::Plain) {
                result.push_str("\x1b[0m");
            }
        }
        result.push('\n');
    }
    result.push_str(&format!("  12-часовой циферблат · полное π-время: {pi_fraction}\n"));
    result
}

fn ink_code(ink: Ink) -> &'static str {
    match ink {
        Ink::Plain => "",
        Ink::Ring => "\x1b[38;5;240m",
        Ink::Tick => "\x1b[1;38;5;75m",
        Ink::Hour => "\x1b[1;38;5;213m",
        Ink::Minute => "\x1b[1;38;5;45m",
        Ink::Second => "\x1b[1;38;5;220m",
        Ink::Center => "\x1b[1;37m",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_clock_and_pi_value() {
        let output = render_circular_clock(3, 30, 45, &Fraction::new(281, 480));
        assert!(output.contains("12-часовой циферблат"));
        assert!(output.contains("281π/480"));
        assert!(output.contains("⊙"));
        assert!(output.contains('╱') || output.contains('╲') || output.contains('─') || output.contains('│'));
    }

    #[test]
    fn colored_renderer_adds_ansi_colors() {
        let output = render_circular_clock_colored(3, 30, 45, &Fraction::new(281, 480));
        assert!(output.contains("\x1b[1;38;5;213m"));
        assert!(output.contains("\x1b[0m"));
    }

    #[test]
    fn angle_wraps_at_twelve() {
        assert_eq!(angle_for_turn(0.0), angle_for_turn(1.0));
        assert_eq!(angle_for_turn(-0.25), angle_for_turn(0.75));
    }
}
