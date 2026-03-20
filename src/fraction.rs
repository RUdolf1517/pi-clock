/// Модуль для работы с дробями.
/// Представляет коэффициент при π в виде несократимой дроби.

use std::fmt;

/// Наибольший общий делитель (алгоритм Евклида)
fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Дробь: numerator / denominator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    /// Создаёт новую дробь и сразу сокращает её.
    /// Знаменатель всегда положительный.
    pub fn new(numerator: i64, denominator: i64) -> Self {
        assert!(denominator != 0, "Знаменатель не может быть нулём");
        let mut f = Fraction { numerator, denominator };
        f.simplify();
        f
    }

    /// Сокращение дроби: делим на НОД, знак переносим в числитель
    fn simplify(&mut self) {
        if self.numerator == 0 {
            self.denominator = 1;
            return;
        }
        let g = gcd(self.numerator, self.denominator);
        self.numerator /= g;
        self.denominator /= g;
        // Знаменатель всегда положительный
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
    }

    /// Сложение двух дробей: a/b + c/d = (a*d + c*b) / (b*d)
    pub fn add(self, other: Fraction) -> Fraction {
        let num = self.numerator * other.denominator + other.numerator * self.denominator;
        let den = self.denominator * other.denominator;
        Fraction::new(num, den)
    }

    /// Форматирует дробь как просто число, без π.
    /// Примеры: "0", "1", "2", "1/6", "7/12"
    pub fn plain(&self) -> String {
        match (self.numerator, self.denominator) {
            (0, _) => "0".to_string(),
            (n, 1) => format!("{}", n),
            (n, d) => format!("{}/{}", n, d),
        }
    }
}

/// Отображение дроби как коэффициента при π.
/// Примеры: "0", "π", "2π", "π/6", "7π/12"
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.numerator, self.denominator) {
            (0, _) => write!(f, "0"),
            (1, 1) => write!(f, "π"),
            (-1, 1) => write!(f, "-π"),
            (n, 1) => write!(f, "{}π", n),
            (1, d) => write!(f, "π/{}", d),
            (-1, d) => write!(f, "-π/{}", d),
            (n, d) => write!(f, "{}π/{}", n, d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(100, 75), 25);
    }

    #[test]
    fn test_simplify() {
        assert_eq!(Fraction::new(6, 12), Fraction { numerator: 1, denominator: 2 });
        assert_eq!(Fraction::new(0, 10), Fraction { numerator: 0, denominator: 1 });
        assert_eq!(Fraction::new(-3, 6), Fraction { numerator: -1, denominator: 2 });
    }

    #[test]
    fn test_add() {
        // 1/2 + 1/12 = 7/12
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 12);
        assert_eq!(a.add(b), Fraction::new(7, 12));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Fraction::new(0, 1)), "0");
        assert_eq!(format!("{}", Fraction::new(1, 1)), "π");
        assert_eq!(format!("{}", Fraction::new(2, 1)), "2π");
        assert_eq!(format!("{}", Fraction::new(1, 6)), "π/6");
        assert_eq!(format!("{}", Fraction::new(7, 12)), "7π/12");
        assert_eq!(format!("{}", Fraction::new(-1, 1)), "-π");
    }

    #[test]
    fn test_plain() {
        assert_eq!(Fraction::new(0, 1).plain(), "0");
        assert_eq!(Fraction::new(1, 2).plain(), "1/2");
        assert_eq!(Fraction::new(7, 12).plain(), "7/12");
        assert_eq!(Fraction::new(2, 1).plain(), "2");
    }
}
