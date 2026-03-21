/// Модуль преобразования времени в дробь π.
///
/// Формулы:
///   1 час = π/6  (24 часа = 4π)
///   1 минута = π/360
///   Итого: time = hour/6 + minute/360  (коэффициент при π)

use crate::fraction::Fraction;

/// Переводит часы и минуты в коэффициент при π (сокращённая дробь).
///
/// Пример: 3:30 → 3/6 + 30/360 = 1/2 + 1/12 = 7/12 → отображается как "7π/12"
/// Пример: 20:00 → 20/6 = 10/3 → отображается как "10π/3"
pub fn time_to_pi_fraction(hour: u32, minute: u32) -> Fraction {
    let hour_frac = Fraction::new(hour as i64, 6);
    let min_frac = Fraction::new(minute as i64, 360);
    hour_frac.add(min_frac)
}

/// Возвращает π-дробь только для часов: hour * π/6
pub fn hour_to_pi_fraction(hour: u32) -> Fraction {
    Fraction::new(hour as i64, 6)
}

/// Возвращает π-дробь только для минут: minute * π/360
pub fn minute_to_pi_fraction(minute: u32) -> Fraction {
    Fraction::new(minute as i64, 360)
}

/// Определяет ближайшую из 12 позиций на циферблате (0..11) для часовой стрелки.
/// Позиция определяется по hour % 12 (циферблат повторяется каждые 12 часов).
pub fn nearest_hour_position(hour: u32, minute: u32) -> usize {
    let total_minutes = (hour % 12) as f64 * 60.0 + minute as f64;
    // 720 минут / 12 позиций = 60 минут на позицию
    ((total_minutes / 60.0).round() as usize) % 12
}

/// Определяет ближайшую из 12 позиций на циферблате (0..11) для минутной стрелки.
/// Каждая позиция соответствует 5 минутам (60 минут / 12 позиций).
pub fn nearest_minute_position(minute: u32) -> usize {
    ((minute as f64 / 5.0).round() as usize) % 12
}

/// Возвращает π-дробь только для секунд: second * π/21600
/// (1 минута = π/360, 1 секунда = π/21600)
pub fn second_to_pi_fraction(second: u32) -> Fraction {
    Fraction::new(second as i64, 21600)
}

/// Переводит часы, минуты и секунды в полный коэффициент при π.
pub fn full_time_to_pi_fraction(hour: u32, minute: u32, second: u32) -> Fraction {
    let hour_frac = Fraction::new(hour as i64, 6);
    let min_frac = Fraction::new(minute as i64, 360);
    let sec_frac = Fraction::new(second as i64, 21600);
    hour_frac.add(min_frac).add(sec_frac)
}

/// Определяет ближайшую из 12 позиций на циферблате (0..11) для секундной стрелки.
/// Каждая позиция соответствует 5 секундам (60 секунд / 12 позиций).
pub fn nearest_second_position(second: u32) -> usize {
    ((second as f64 / 5.0).round() as usize) % 12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_thirty() {
        // 3:30 → 7π/12
        let f = time_to_pi_fraction(3, 30);
        assert_eq!(f, Fraction::new(7, 12));
    }

    #[test]
    fn test_midnight() {
        // 0:00 → 0
        let f = time_to_pi_fraction(0, 0);
        assert_eq!(f, Fraction::new(0, 1));
    }

    #[test]
    fn test_six_oclock() {
        // 6:00 → π
        let f = time_to_pi_fraction(6, 0);
        assert_eq!(f, Fraction::new(1, 1));
    }

    #[test]
    fn test_twenty_oclock() {
        // 20:00 → 20/6 = 10/3 → 10π/3
        let f = time_to_pi_fraction(20, 0);
        assert_eq!(f, Fraction::new(10, 3));
    }

    #[test]
    fn test_three_oclock() {
        // 3:00 → π/2
        let f = time_to_pi_fraction(3, 0);
        assert_eq!(f, Fraction::new(1, 2));
    }

    #[test]
    fn test_nine_oclock() {
        // 9:00 → 3π/2
        let f = time_to_pi_fraction(9, 0);
        assert_eq!(f, Fraction::new(3, 2));
    }

    #[test]
    fn test_position() {
        assert_eq!(nearest_hour_position(0, 0), 0);
        assert_eq!(nearest_hour_position(3, 0), 3);
        assert_eq!(nearest_hour_position(6, 0), 6);
        assert_eq!(nearest_hour_position(9, 0), 9);
    }
}
