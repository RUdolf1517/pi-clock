/// ASCII-циферблат от 0 до 2π.
///
/// 12 позиций по кругу, стрелка указывает на текущее время.
/// Позиция 0 (0 радиан) — вверху, далее по часовой стрелке.

use crate::fraction::Fraction;
use std::f64::consts::PI;

/// Ширина и высота символьной сетки
const WIDTH: usize = 45;
const HEIGHT: usize = 21;

/// Радиусы (с учётом пропорций символов ~2:1)
const RADIUS_X: f64 = 16.0;
const RADIUS_Y: f64 = 8.0;

/// Радиусы для подписей (чуть дальше от круга)
const LABEL_RADIUS_X: f64 = 19.0;
const LABEL_RADIUS_Y: f64 = 9.5;

/// Центр сетки
const CENTER_X: f64 = 22.0;
const CENTER_Y: f64 = 10.0;

/// Подписи для 12 позиций циферблата
const LABELS: [&str; 12] = [
    "0", "π/6", "π/3", "π/2", "2π/3", "5π/6",
    "π", "7π/6", "4π/3", "3π/2", "5π/3", "11π/6",
];

/// Угол позиции (от верха, по часовой стрелке).
/// Позиция 0 = верх (−π/2 в стандартных координатах).
fn position_angle(pos: usize) -> f64 {
    (pos as f64) * PI / 6.0
}

/// Координаты точки на окружности по углу (от верха, по часовой стрелке)
fn circle_xy(angle: f64, rx: f64, ry: f64) -> (f64, f64) {
    let x = CENTER_X + rx * angle.sin();
    let y = CENTER_Y - ry * angle.cos();
    (x, y)
}

/// Рисует круговые часы и возвращает многострочную строку.
/// hour_pos  — позиция часовой стрелки (0..11)
/// min_pos   — позиция минутной стрелки (0..11)
/// sec_pos   — позиция секундной стрелки (0..11)
pub fn render_circular_clock(hour_pos: usize, min_pos: usize, sec_pos: usize, pi_fraction: &Fraction) -> String {
    let mut grid = vec![vec![' '; WIDTH]; HEIGHT];

    // 1. Рисуем окружность точками
    for i in 0..360 {
        let angle = (i as f64) * PI / 180.0;
        let (x, y) = circle_xy(angle, RADIUS_X, RADIUS_Y);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if xi < WIDTH && yi < HEIGHT && grid[yi][xi] == ' ' {
            grid[yi][xi] = '·';
        }
    }

    // 2. Рисуем отметки на 12 позициях
    for pos in 0..12 {
        let angle = position_angle(pos);
        let (x, y) = circle_xy(angle, RADIUS_X, RADIUS_Y);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if xi < WIDTH && yi < HEIGHT {
            let ch = match (pos == hour_pos, pos == min_pos, pos == sec_pos) {
                (true, true, _)   => '◉', // часовая + минутная
                (true, false, _)  => '●', // только часовая
                (false, true, _)  => '◎', // только минутная
                (false, false, true)  => '◌', // только секундная
                (false, false, false) => '○',
            };
            grid[yi][xi] = ch;
        }
    }

    // 3. Рисуем секундную стрелку (.) — первой, перекрывается минутной и часовой
    let sec_angle = position_angle(sec_pos);
    let steps = 7;
    for s in 1..=steps {
        let t = s as f64 / steps as f64;
        let (x, y) = circle_xy(sec_angle, RADIUS_X * t, RADIUS_Y * t);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if xi < WIDTH && yi < HEIGHT {
            grid[yi][xi] = if s == steps { '◦' } else { '.' };
        }
    }

    // 4. Рисуем минутную стрелку (+) — поверх секундной
    let min_angle = position_angle(min_pos);
    let steps = 7;
    for s in 1..=steps {
        let t = s as f64 / steps as f64;
        let (x, y) = circle_xy(min_angle, RADIUS_X * t, RADIUS_Y * t);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if xi < WIDTH && yi < HEIGHT {
            grid[yi][xi] = if s == steps { '◇' } else { '+' };
        }
    }

    // 5. Рисуем часовую стрелку (*) — поверх минутной
    let hour_angle = position_angle(hour_pos);
    for s in 1..=steps {
        let t = s as f64 / steps as f64;
        let (x, y) = circle_xy(hour_angle, RADIUS_X * t, RADIUS_Y * t);
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if xi < WIDTH && yi < HEIGHT {
            grid[yi][xi] = if s == steps { '◆' } else { '*' };
        }
    }

    // 6. Центр
    let cx = CENTER_X.round() as usize;
    let cy = CENTER_Y.round() as usize;
    grid[cy][cx] = '⊙';

    // 7. Размещаем подписи позиций
    for pos in 0..12 {
        let angle = position_angle(pos);
        let (lx, ly) = circle_xy(angle, LABEL_RADIUS_X, LABEL_RADIUS_Y);
        let label = LABELS[pos];
        let lxi = lx.round() as isize;
        let lyi = ly.round() as usize;

        if lyi >= HEIGHT {
            continue;
        }

        // Для позиций слева — выравниваем по правому краю к точке привязки
        // Для позиций справа — по левому краю
        let start_col = if angle.sin() < -0.1 {
            // Левая сторона: текст заканчивается у точки привязки
            (lxi - label.len() as isize + 1).max(0) as usize
        } else if angle.sin().abs() <= 0.1 {
            // Верх/низ: центрируем
            (lxi - label.len() as isize / 2).max(0) as usize
        } else {
            // Правая сторона: текст начинается от точки привязки
            lxi.max(0) as usize
        };

        // Подпись выделяем скобками, если совпадает с любой стрелкой
        let decorated = if pos == hour_pos || pos == min_pos || pos == sec_pos {
            format!("[{}]", label)
        } else {
            label.to_string()
        };

        for (i, ch) in decorated.chars().enumerate() {
            let col = start_col + i;
            if col < WIDTH {
                grid[lyi][col] = ch;
            }
        }
    }

    // Собираем сетку в строку
    let mut result = String::new();
    for row in &grid {
        let line: String = row.iter().collect();
        result.push_str(line.trim_end());
        result.push('\n');
    }

    // Добавляем текущее значение под циферблатом
    result.push_str(&format!("\n    Текущее положение стрелки: {}\n", pi_fraction));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_does_not_panic() {
        let frac = Fraction::new(1, 2);
        let output = render_circular_clock(3, 6, 0, &frac);
        assert!(!output.is_empty());
        assert!(output.contains("π/2")); // позиция 3 = π/2
    }
}
