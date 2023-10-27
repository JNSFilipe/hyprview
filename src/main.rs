mod hyprviewlib;
use crate::hyprviewlib::{Monitor, get_hyprctl_monitors_output, run_hyprctl_monitors_command};
use clap::Parser;

/// Control hyprland second monitor position
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Duplicate screen
    #[arg(short, long, action)]
    duplicate: bool,
}

fn main() {
    // Get input arguments
    let args = Args::parse();

    // Run the command and capture the output
    let output = get_hyprctl_monitors_output();

    // Check if the command was successful
    if output.status.success() {
        // Parse the JSON output into a vector of Monitor structs
        let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout).unwrap();

        if monitors.len() > 1 {
            if args.duplicate {
                println!("Duplicate Screen");
                let command = format!("{},{}x{}@{},0x0,1,mirror,{}", 
                    monitors[1].name, 
                    monitors[1].width,
                    monitors[1].height,
                    monitors[1].refreshRate,
                    monitors[0].name 
                );
                println!("{}", command);
            }
        } else {
            eprintln!("Error, just one monitor plugged in");
        }
    } else {
        eprintln!("Error running hyprctl command: {:?}", output.status);
    }
}


