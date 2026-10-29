mod ui;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(400.0, 300.0));
    eframe::run_native(
        "Lirel",
        options,
        Box::new(|_cc| Box::new(ui::App::default())),
    );
}
