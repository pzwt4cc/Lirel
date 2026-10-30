#![cfg_attr(windows, windows_subsystem = "windows")]

mod backend;
mod gui;
mod utils;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    slint::set_xdg_app_id("lirel")?;
    gui::setup(&ui);
    ui.run()
}
