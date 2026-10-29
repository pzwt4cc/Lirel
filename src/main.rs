mod ui;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lirel",
        options,
        Box::new(|_cc| Box::new(ui::App::default())),
    );
}
