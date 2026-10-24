/// Библиотечный корень: экспортирует модули для использования в tray-бинарнике.
pub mod fraction;
pub mod clock;
pub mod display;
pub mod tui;
#[cfg(feature = "tray-app")]
pub mod tray;
