mod ui;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([400.0, 300.0])
        .with_min_inner_size([400.0, 300.0]);
    eframe::run_native(
        "Lirel",
        options,
        Box::new(|_cc| Box::new(ui::App::default())),
    );
}
