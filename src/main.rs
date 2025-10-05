// For windows subsystem, relase mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;

    ui.run()?;
    Ok(())
}
