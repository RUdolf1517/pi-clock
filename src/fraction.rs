/// Работа с рациональными коэффициентами при π.
use std::fmt;

/// Дробь `numerator / denominator` в несократимом виде.
/// Знаменатель всегда положительный.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    /// Создаёт дробь и сокращает её. Нулевой знаменатель считается ошибкой.
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Self::from_i128(numerator as i128, denominator as i128)
    }

    fn from_i128(mut numerator: i128, mut denominator: i128) -> Self {
        assert!(denominator != 0, "Знаменатель не может быть нулём");

        if numerator == 0 {
            return Self { numerator: 0, denominator: 1 };
        }
        if denominator < 0 {
            numerator = numerator.checked_neg().expect("Переполнение при нормализации дроби");
            denominator = -denominator;
        }

        let divisor = gcd(numerator.unsigned_abs(), denominator as u128) as i128;
        let numerator = numerator / divisor;
        let denominator = denominator / divisor;
        Self {
            numerator: i64::try_from(numerator).expect("Числитель дроби не помещается в i64"),
            denominator: i64::try_from(denominator).expect("Знаменатель дроби не помещается в i64"),
        }
    }

    /// Складывает дроби с промежуточными вычислениями в i128.
    pub fn add(self, other: Fraction) -> Fraction {
        let left = (self.numerator as i128) * (other.denominator as i128);
        let right = (other.numerator as i128) * (self.denominator as i128);
        let numerator = left.checked_add(right).expect("Переполнение при сложении дробей");
        let denominator = (self.denominator as i128) * (other.denominator as i128);
        Fraction::from_i128(numerator, denominator)
    }

    /// Форматирует дробь как число без π: `0`, `1`, `1/6`.
    pub fn plain(&self) -> String {
        match (self.numerator, self.denominator) {
            (0, _) => "0".to_string(),
            (n, 1) => n.to_string(),
            (n, d) => format!("{n}/{d}"),
        }
    }
}

/// Отображение дроби как коэффициента при π.
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.numerator, self.denominator) {
            (0, _) => write!(f, "0"),
            (1, 1) => write!(f, "π"),
            (-1, 1) => write!(f, "-π"),
            (n, 1) => write!(f, "{n}π"),
            (1, d) => write!(f, "π/{d}"),
            (-1, d) => write!(f, "-π/{d}"),
            (n, d) => write!(f, "{n}π/{d}"),
        }
    }
}

/// НОД неотрицательных целых чисел (алгоритм Евклида).
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_and_reduction() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(Fraction::new(6, 12), Fraction { numerator: 1, denominator: 2 });
        assert_eq!(Fraction::new(0, 10), Fraction { numerator: 0, denominator: 1 });
        assert_eq!(Fraction::new(-3, 6), Fraction { numerator: -1, denominator: 2 });
        assert_eq!(Fraction::new(3, -6), Fraction { numerator: -1, denominator: 2 });
    }

    #[test]
    fn supports_i64_min_and_large_intermediates() {
        assert_eq!(Fraction::new(i64::MIN, 2), Fraction { numerator: i64::MIN / 2, denominator: 1 });
        assert_eq!(Fraction::new(i64::MAX, 2).add(Fraction::new(i64::MAX, 2)), Fraction::new(i64::MAX, 1));
    }

    #[test]
    fn addition_and_display() {
        assert_eq!(Fraction::new(1, 2).add(Fraction::new(1, 12)), Fraction::new(7, 12));
        assert_eq!(format!("{}", Fraction::new(0, 1)), "0");
        assert_eq!(format!("{}", Fraction::new(1, 1)), "π");
        assert_eq!(format!("{}", Fraction::new(2, 1)), "2π");
        assert_eq!(format!("{}", Fraction::new(1, 6)), "π/6");
        assert_eq!(format!("{}", Fraction::new(7, 12)), "7π/12");
        assert_eq!(format!("{}", Fraction::new(-1, 1)), "-π");
        assert_eq!(Fraction::new(7, 12).plain(), "7/12");
    }

    #[test]
    #[should_panic(expected = "Знаменатель не может быть нулём")]
    fn rejects_zero_denominator() {
        Fraction::new(1, 0);
    }
}
