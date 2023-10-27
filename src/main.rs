mod hyprviewlib;
use crate::hyprviewlib::{Monitor, run_hyprctl_command};

fn main() {
    // Run the command and capture the output
    let output = run_hyprctl_command();

    // Check if the command was successful
    if output.status.success() {
        // Parse the JSON output into a vector of Monitor structs
        let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout).unwrap();

        if monitors.len() > 2 {
            for monitor in monitors {
                println!("{:#?}", monitor);
            }
        } else {
            eprintln!("Error, just one monitor plugged in");
        }
    } else {
        eprintln!("Error running hyprctl command: {:?}", output.status);
    }
}


