// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::f32::consts::E;
use std::process::{self, Command};
use std::{env, str};

slint::include_modules!();

// Todo:
// - Add app icon
// - Add confirmation prompt
// - Make initsystem agnostic
// - Force initial floating state in window managers
// - Add icons, using gtk or qt theme

const VERSION: &str = "1.1.0";

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

    let app_window: AppWindow = AppWindow::new()?;

    app_window.on_request_shutdown({
        move || {
            Command::new("shutdown")
                .arg("now")
                .spawn()
                .expect("Reboot command failed");
        }
    });

    app_window.on_request_reboot({
        move || {
            Command::new("reboot")
                .spawn()
                .expect("Reboot command failed");
        }
    });

    app_window.on_request_suspend({
        move || {
            // suspend will remain systemd dependent for now

            // current best solution is probably to check which initsystem
            // is in use, then run the appropriate suspend command

            /* Command::new("systemctl")
            .arg("suspend")
            .spawn()
            .expect("Suspend command failed"); */

            // the following command should return the initsys in use:
            // ps -p 1 -o comm=

            let initsys_check = Command::new("ps")
                .arg("-p")
                .arg("1")
                .arg("-o")
                .arg("comm=")
                .output()
                .expect("Initsys check failed");

            let initsys_result = str::from_utf8(&initsys_check.stdout).unwrap();

            println!("Result: {}", initsys_result); // prints initsys

            

            process::exit(0); // so app window doesn't remain open after suspending
        }
    });

    app_window.on_request_cancel({
        move || {
            process::exit(0);
        }
    });

    app_window.run()?;

    Ok(())
}
