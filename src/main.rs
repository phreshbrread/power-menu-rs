// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::process;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui: AppWindow = AppWindow::new()?;

    ui.on_request_cancel({
        move || {
            process::exit(0);
        }
    });

    ui.run()?;




    Ok(())
}
