// Pi-Clock: трей-приложение.
// macOS: показывает π-время текстом прямо в меню-баре.
// Windows: показывает иконку в трее, π-время в тултипе.
// Клик по иконке открывает всплывающее окно с обоими типами часов.

use std::time::Duration;
use eframe::egui;
use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use crate::{clock, display};

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn FreeConsole() -> i32;
}

fn get_local_time() -> (u32, u32, u32) {
    clock::local_time().expect("Не удалось получить локальное время")
}

// ─── Генерация иконки (32×32 белый круг) ─────────────────────────────────────

fn make_icon() -> tray_icon::Icon {
    const SIZE: u32 = 32;
    let mut rgba = vec![0u8; (SIZE * SIZE * 4) as usize];
    let cx = SIZE as f64 / 2.0;
    let cy = SIZE as f64 / 2.0;
    let r = SIZE as f64 / 2.0 - 1.0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            if (dx * dx + dy * dy).sqrt() <= r {
                let i = ((y * SIZE + x) * 4) as usize;
                rgba[i] = 200; rgba[i+1] = 200; rgba[i+2] = 255; rgba[i+3] = 255;
            }
        }
    }
    tray_icon::Icon::from_rgba(rgba, SIZE, SIZE).expect("Не удалось создать иконку")
}

// ─── Состояние приложения ────────────────────────────────────────────────────

struct TrayApp {
    /// Иконка в трее — должна жить всё время работы приложения
    _tray: TrayIcon,
    popup_open: bool,
    /// Кэшированная метка π-времени (обновляется каждую минуту)
    pi_label: String,
}

impl TrayApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (hour, minute, second) = get_local_time();
        let pi_label = format!(
            "({}:{}:{})π",
            clock::hour_to_pi_fraction(hour).plain(),
            clock::minute_to_pi_fraction(minute).plain(),
            clock::second_to_pi_fraction(second).plain()
        );

        let tray = TrayIconBuilder::new()
            .with_icon(make_icon())
            .with_title(&pi_label)      // macOS: текст в меню-баре
            .with_tooltip(&pi_label)    // Windows: тултип при наведении
            .build()
            .expect("Не удалось создать иконку в трее");

        // Прячем корневое окно — оно нужно только как якорь event loop
        cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));

        Self { _tray: tray, popup_open: false, pi_label }
    }

    /// Обновляет текст иконки каждую секунду
    fn refresh_label(&mut self) {
        let (h, m, s) = get_local_time();
        let new_label = format!(
            "({}:{}:{})π",
            clock::hour_to_pi_fraction(h).plain(),
            clock::minute_to_pi_fraction(m).plain(),
            clock::second_to_pi_fraction(s).plain()
        );
        if new_label != self.pi_label {
            self.pi_label = new_label.clone();
            let _ = self._tray.set_title(Some(&new_label));
            let _ = self._tray.set_tooltip(Some(&new_label));
        }
    }
}

impl eframe::App for TrayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Обновляем метку π-времени
        self.refresh_label();

        // 2. Проверяем события трея (клик → открыть попап)
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if matches!(event, TrayIconEvent::Click { .. }) {
                self.popup_open = true;
            }
        }

        // 3. Просыпаемся каждую секунду для обновления секундной стрелки
        ctx.request_repaint_after(Duration::from_secs(1));

        // 4. Рендерим всплывающее окно, если открыто
        if self.popup_open {
            let (hour, minute, second) = get_local_time();
            let pi_frac = clock::full_time_to_pi_fraction(hour, minute, second);
            let hour_frac = clock::hour_to_pi_fraction(hour);
            let min_frac = clock::minute_to_pi_fraction(minute);
            let sec_frac = clock::second_to_pi_fraction(second);
            let ascii = display::render_circular_clock(hour, minute, second, &pi_frac);
            let digital = format!(
                "({}:{}:{})π = {}",
                hour_frac.plain(),
                min_frac.plain(),
                sec_frac.plain(),
                pi_frac
            );
            let time_str = format!("{:02}:{:02}:{:02}", hour, minute, second);

            let mut popup_open = self.popup_open;

            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("pi_popup"),
                egui::ViewportBuilder::default()
                    .with_title("π-Clock")
                    .with_inner_size([480.0, 460.0])
                    .with_resizable(false),
                |ctx, _class| {
                    // Перерисовываем попап каждую секунду независимо от root viewport
                    ctx.request_repaint_after(Duration::from_secs(1));

                    // Закрыть по Escape
                    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    // Закрыть по кнопке окна
                    if ctx.input(|i| i.viewport().close_requested()) {
                        popup_open = false;
                    }

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(10.0);

                        // ── Электронные часы ──────────────────────────────────
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Время: {}", time_str))
                                    .size(16.0)
                            );
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(&digital)
                                    .size(28.0)
                                    .strong()
                                    .monospace()
                            );
                        });

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // ── Круговые часы (ASCII) ─────────────────────────────
                        ui.label(
                            egui::RichText::new(&ascii)
                                .monospace()
                                .size(11.0)
                        );

                        ui.add_space(4.0);
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("* часовая   + минутная   . секундная")
                                    .size(11.0)
                                    .weak()
                            );
                        });
                    });
                },
            );

            self.popup_open = popup_open;
        }

        // Корневое окно — скрытое, ничего не рендерит
        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}

// ─── Точка входа ─────────────────────────────────────────────────────────────

pub fn run() -> eframe::Result {
    #[cfg(windows)]
    unsafe {
        // Отсоединяем CLI от консоли при запуске системного трея.
        let _ = FreeConsole();
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_visible(false)          // корневое окно невидимо
            .with_inner_size([1.0, 1.0])  // минимальный размер
            .with_decorations(false),     // без рамки
        ..Default::default()
    };

    eframe::run_native(
        "pi-clock-tray",
        options,
        Box::new(|cc| Ok(Box::new(TrayApp::new(cc)))),
    )
}
