// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::error::Error;
use std::process::{self, Command};

slint::include_modules!();

// Todo:
// - Add app icon
// - Add confirmation prompt
// - Make initsystem agnostic
// - Force initial floating state in window managers
// - Add icons, using gtk or qt theme

const VERSION: &str = "1.0.0";

const HELP_MESSAGE: &str = "Usage: power-menu-rs [options]

Options:
    --help          Show this message
    --version       Show current version";

fn main() -> Result<(), Box<dyn Error>> {
    // args[0] is the path
    // following are actual arguments

    let args: Vec<String> = env::args().collect(); // collect cmd args

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
            Command::new("systemctl")
                .arg("poweroff")
                .spawn()
                .expect("Shutdown command failed");
        }
    });

    ui.on_request_reboot({
        move || {
            Command::new("systemctl")
                .arg("reboot")
                .spawn()
                .expect("Reboot command failed");
        }
    });

    ui.on_request_suspend({
        move || {
            Command::new("systemctl")
                .arg("suspend")
                .spawn()
                .expect("Suspend command failed");
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
