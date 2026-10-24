//! Компактный текстовый 12-часовой циферблат.

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

fn angle_for_turn(turn: f64) -> f64 {
    turn.rem_euclid(1.0) * TAU
}

/// Угол отсчитывается от отметки 0 сверху по часовой стрелке.
fn circle_xy(angle: f64, rx: f64, ry: f64) -> (f64, f64) {
    (CENTER_X + rx * angle.sin(), CENTER_Y - ry * angle.cos())
}

fn put(grid: &mut [Vec<char>], x: f64, y: f64, ch: char) {
    let xi = x.round() as isize;
    let yi = y.round() as isize;
    if yi >= 0 && yi < HEIGHT as isize && xi >= 0 && xi < WIDTH as isize {
        grid[yi as usize][xi as usize] = ch;
    }
}

fn draw_hand(grid: &mut [Vec<char>], turn: f64, glyph: char, tip: char, length: f64) {
    let angle = angle_for_turn(turn);
    let steps = (length * 2.0).ceil() as usize;
    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let (x, y) = circle_xy(angle, RADIUS_X * length * t, RADIUS_Y * length * t);
        put(grid, x, y, if step == steps { tip } else { glyph });
    }
}

/// Рисует циферблат с плавно движущимися стрелками.
/// Положение циферблата 12-часовое: 0 и 12 часов совпадают.
pub fn render_circular_clock(
    hour: u32,
    minute: u32,
    second: u32,
    pi_fraction: &Fraction,
) -> String {
    let mut grid = vec![vec![' '; WIDTH]; HEIGHT];

    // Окружность.
    for degree in 0..360 {
        let (x, y) = circle_xy(degree as f64 * TAU / 360.0, RADIUS_X, RADIUS_Y);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if grid[yi][xi] == ' ' {
            grid[yi][xi] = '·';
        }
    }

    // Часовые отметки и подписи углов.
    for pos in 0..12 {
        let angle = pos as f64 * TAU / 12.0;
        let (x, y) = circle_xy(angle, RADIUS_X, RADIUS_Y);
        put(&mut grid, x, y, '○');

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
                grid[row as usize][col as usize] = ch;
            }
        }
    }

    // Сначала короткая секундная, затем минутная и часовая стрелки.
    draw_hand(&mut grid, clock::second_turn(second), '.', '◦', 1.0);
    draw_hand(&mut grid, clock::minute_turn(minute, second), '+', '◇', 0.82);
    draw_hand(&mut grid, clock::hour_turn(hour, minute, second), '*', '◆', 0.62);
    put(&mut grid, CENTER_X, CENTER_Y, '⊙');

    let mut result = String::new();
    for row in &grid {
        let line: String = row.iter().collect();
        result.push_str(line.trim_end());
        result.push('\n');
    }
    result.push_str(&format!("  12-часовой циферблат · полное π-время: {pi_fraction}\n"));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_clock_and_pi_value() {
        let output = render_circular_clock(3, 30, 45, &Fraction::new(221, 480));
        assert!(output.contains("12-часовой циферблат"));
        assert!(output.contains("221π/480"));
        assert!(output.contains("⊙"));
    }

    #[test]
    fn angle_wraps_at_twelve() {
        assert_eq!(angle_for_turn(0.0), angle_for_turn(1.0));
        assert_eq!(angle_for_turn(-0.25), angle_for_turn(0.75));
    }
}
