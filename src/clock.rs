//! Преобразование локального времени в дроби π и координаты 12-часового циферблата.

use crate::fraction::Fraction;

/// Часы и минуты в коэффициенте при π: `hour / 6 + minute / 360`.
pub fn time_to_pi_fraction(hour: u32, minute: u32) -> Fraction {
    Fraction::new(i64::from(hour), 6).add(Fraction::new(i64::from(minute), 360))
}

pub fn hour_to_pi_fraction(hour: u32) -> Fraction {
    Fraction::new(i64::from(hour), 6)
}

pub fn minute_to_pi_fraction(minute: u32) -> Fraction {
    Fraction::new(i64::from(minute), 360)
}

pub fn second_to_pi_fraction(second: u32) -> Fraction {
    Fraction::new(i64::from(second), 21600)
}

/// Полное 24-часовое время в коэффициенте при π (за сутки набегает 4π).
pub fn full_time_to_pi_fraction(hour: u32, minute: u32, second: u32) -> Fraction {
    hour_to_pi_fraction(hour)
        .add(minute_to_pi_fraction(minute))
        .add(second_to_pi_fraction(second))
}

/// Непрерывное положение часовой стрелки в оборотах от 0 до 1.
pub fn hour_turn(hour: u32, minute: u32, second: u32) -> f64 {
    ((hour % 12) as f64 + minute as f64 / 60.0 + second as f64 / 3600.0) / 12.0
}

/// Непрерывное положение минутной стрелки в оборотах от 0 до 1.
pub fn minute_turn(minute: u32, second: u32) -> f64 {
    (minute as f64 + second as f64 / 60.0) / 60.0
}

/// Непрерывное положение секундной стрелки в оборотах от 0 до 1.
pub fn second_turn(second: u32) -> f64 {
    second as f64 / 60.0
}

/// Ближайшая часовая отметка циферблата (0..11), сохранена для старых клиентов API.
pub fn nearest_hour_position(hour: u32, minute: u32) -> usize {
    (((hour % 12) as u64 + (minute as u64 + 30) / 60) % 12) as usize
}

/// Ближайшая минутная отметка (одна из 12, шаг пять минут).
pub fn nearest_minute_position(minute: u32) -> usize {
    (((minute as u64 + 2) / 5) % 12) as usize
}

/// Ближайшая секундная отметка (одна из 12, шаг пять секунд).
pub fn nearest_second_position(second: u32) -> usize {
    (((second as u64 + 2) / 5) % 12) as usize
}

/// Получает локальное время средствами ОС.
#[cfg(all(unix, target_pointer_width = "64"))]
pub fn local_time() -> Result<(u32, u32, u32), String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Не удалось получить системное время: {e}"))?
        .as_secs() as i64;

    // POSIX struct tm для 64-битных macOS/Linux. Используем localtime_r,
    // чтобы не обращаться к разделяемому статическому буферу localtime.
    #[repr(C)]
    struct Tm {
        tm_sec: i32,
        tm_min: i32,
        tm_hour: i32,
        tm_mday: i32,
        tm_mon: i32,
        tm_year: i32,
        tm_wday: i32,
        tm_yday: i32,
        tm_isdst: i32,
        tm_gmtoff: i64,
        tm_zone: *const i8,
    }

    unsafe extern "C" {
        fn localtime_r(time: *const i64, result: *mut Tm) -> *mut Tm;
    }

    let mut result = std::mem::MaybeUninit::<Tm>::uninit();
    let ptr = unsafe { localtime_r(&seconds, result.as_mut_ptr()) };
    if ptr.is_null() {
        return Err("ОС не смогла преобразовать время в местное".to_string());
    }
    let tm = unsafe { result.assume_init() };
    Ok((tm.tm_hour as u32, tm.tm_min as u32, tm.tm_sec as u32))
}

#[cfg(target_os = "windows")]
pub fn local_time() -> Result<(u32, u32, u32), String> {
    #[repr(C)]
    struct SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLocalTime(system_time: *mut SystemTime);
    }

    let mut result = std::mem::MaybeUninit::<SystemTime>::uninit();
    unsafe { GetLocalTime(result.as_mut_ptr()) };
    let tm = unsafe { result.assume_init() };
    Ok((u32::from(tm.hour), u32::from(tm.minute), u32::from(tm.second)))
}

#[cfg(not(any(all(unix, target_pointer_width = "64"), target_os = "windows")))]
pub fn local_time() -> Result<(u32, u32, u32), String> {
    Err("Эта операционная система пока не поддерживается".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_conversion() {
        assert_eq!(time_to_pi_fraction(3, 30), Fraction::new(7, 12));
        assert_eq!(time_to_pi_fraction(0, 0), Fraction::new(0, 1));
        assert_eq!(time_to_pi_fraction(6, 0), Fraction::new(1, 1));
        assert_eq!(time_to_pi_fraction(20, 0), Fraction::new(10, 3));
        assert_eq!(full_time_to_pi_fraction(3, 30, 45), Fraction::new(221, 480));
        assert_eq!(full_time_to_pi_fraction(23, 59, 59), Fraction::new(86399, 21600));
    }

    #[test]
    fn analog_positions_repeat_every_twelve_hours_and_move_smoothly() {
        assert_eq!(hour_turn(0, 0, 0), hour_turn(12, 0, 0));
        assert_eq!(hour_turn(3, 0, 0), 0.25);
        assert_eq!(hour_turn(3, 30, 0), 0.2916666666666667);
        assert_eq!(minute_turn(15, 30), 0.25833333333333336);
        assert_eq!(second_turn(30), 0.5);
    }
}
