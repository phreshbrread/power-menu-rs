// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::error::Error;
use std::process::{self, Command};

slint::include_modules!();

// Todo:
// - Add actual power controls
// - Make initsys agnostic
// - Force initially floating state in window managers
// - Add icons, using gtk or qt theme

const VERSION: &str = "0.1.0";

const HELP_MESSAGE: &str = "Usage: power-menu-rs [options]

Options:
    --help          Show this message
    --version       Show current version";

fn main() -> Result<(), Box<dyn Error>> {
    // args[0] is the path
    // Following are actual arguments

    let args: Vec<String> = env::args().collect(); // Collect cmd args

    if args.len() != 1 {
        if args[1] == "--version" {
            println!("power-menu-rs version {}", VERSION);
        } else {
            println!("{}", HELP_MESSAGE);
        }

        process::exit(0);
    }

    let ui: AppWindow = AppWindow::new()?;

    ui.on_request_shutdown({
        move || {
            println!("Shutdown");
        }
    });

    ui.on_request_reboot({
        move || {
            println!("Reboot");
        }
    });

    ui.on_request_suspend({
        move || {
            println!("Suspend");
        }
    });

    ui.on_request_cancel({
        move || {
            process::exit(0);
        }
    });

    ui.run()?;

    Ok(())
}
