mod backend;
mod gui;
mod utils;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    gui::setup(&ui);
    ui.run()
}