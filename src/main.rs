/// Pi-Clock: часы с представлением времени через число π.
///
/// 12 часов = 2π, 1 час = π/6, 1 минута = π/360.
/// Два режима отображения: электронные (дробь π) и круговые (ASCII-циферблат).

mod fraction;
mod clock;
mod display;

/// Получение локального времени через libc (без внешних зависимостей).
fn get_local_time() -> (u32, u32, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Системное время раньше UNIX_EPOCH")
        .as_secs() as i64;

    // Используем libc для преобразования в локальное время
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

    let mut tm = std::mem::MaybeUninit::<Tm>::uninit();
    unsafe {
        let result = localtime_r(&secs, tm.as_mut_ptr());
        if result.is_null() {
            panic!("localtime_r вернул NULL");
        }
        let tm = tm.assume_init();
        (tm.tm_hour as u32, tm.tm_min as u32, tm.tm_sec as u32)
    }
}

fn main() {
    let (hour, minute, second) = get_local_time();

    // Преобразуем время в дроби π
    let hour_frac = clock::hour_to_pi_fraction(hour);
    let min_frac = clock::minute_to_pi_fraction(minute);
    let sec_frac = clock::second_to_pi_fraction(second);
    let pi_frac = clock::full_time_to_pi_fraction(hour, minute, second);
    let hour_pos = clock::nearest_hour_position(hour, minute);
    let min_pos = clock::nearest_minute_position(minute);
    let sec_pos = clock::nearest_second_position(second);

    // Формат: (1/2:1/12:1/360)π = 7π/12
    let pi_display = format!("({}:{}:{})π = {}", hour_frac.plain(), min_frac.plain(), sec_frac.plain(), pi_frac);

    // === Электронные часы ===
    println!("╔══════════════════════════════════════╗");
    println!("║         π-ЧАСЫ (электронные)         ║");
    println!("╠══════════════════════════════════════╣");
    println!("║                                      ║");
    println!("║   Время: {:02}:{:02}:{:02}                     ║", hour, minute, second);
    println!("║   π-время: {:<28}║", pi_display);
    println!("║                                      ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // === Круговые часы ===
    println!("╔══════════════════════════════════════════════╗");
    println!("║            π-ЧАСЫ (круговые)                ║");
    println!("╠══════════════════════════════════════════════╣");
    println!("║  * часовая  + минутная  . секундная         ║");
    println!("╠══════════════════════════════════════════════╣");
    println!();
    print!("{}", display::render_circular_clock(hour_pos, min_pos, sec_pos, &pi_frac));
    println!("╚══════════════════════════════════════════════╝");
}
