mod hyprviewlib;
use crate::hyprviewlib::{get_hyprctl_monitors_output, run_hyprctl_monitors_command, Monitor};
use clap::Parser;
use home::home_dir;
use std::fs;

/// Control hyprland second monitor position
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Duplicate screen
    #[arg(short, long, action)]
    duplicate: bool,

    /// Second screen on top
    #[arg(short, long, action)]
    top: bool,

    /// Second screen on the bottom
    #[arg(short, long, action)]
    bottom: bool,

    /// Second screen on the left
    #[arg(short, long, action)]
    left: bool,
}

fn main() {
    // Get input arguments
    let args = Args::parse();

    // Run the command and capture the output
    let output = get_hyprctl_monitors_output();

    // Get home and cache paths
    let cache = home_dir()
        .expect("Unnable to find home!?")
        .join(".cache/hyprview.json");

    println!("{:?}", cache);
    // Check if the command was successful
    if output.status.success() {
        // Parse the JSON output into a vector of Monitor structs
        let mut monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout).unwrap();

        // Check if there is a second monitor conected
        // TODO: Use another command to double check howmany monitors are connected
        if monitors.len() < 2 {
            // Check if there is data of monitors in cache
            // Read it if there is data to read
            // This is needed because after mirroring,
            // hyprctl monitor no longer lists the second moniotr
            let data =
                fs::read_to_string(cache.clone()).expect("Error, just one monitor plugged in");
            monitors = serde_json::from_str(&data).unwrap();
        }

        if args.duplicate {
            println!("Duplicate Screen");
            let command = format!(
                "{},{}x{}@{},0x0,1,mirror,{}",
                monitors[1].name,
                monitors[1].width,
                monitors[1].height,
                monitors[1].refreshRate,
                monitors[0].name
            );
            run_hyprctl_monitors_command(command);

            // Save data of monitors in cache
            // This is needed because after mirroring,
            // hyprctl monitor no longer lists the second moniotr
            println!("{}", serde_json::to_string(&monitors).unwrap());
            fs::write(cache.clone(), serde_json::to_string(&monitors).unwrap());
        }
        if args.top {
            println!("Second Monitor on top");
            let command1 = format!(
                "{},{}x{}@{},{}x{},{}",
                monitors[0].name,
                monitors[0].width,
                monitors[0].height,
                monitors[0].refreshRate,
                0,
                monitors[1].height,
                monitors[0].scale
            );
            let command2 = format!(
                "{},{}x{}@{},{}x{},{}",
                monitors[1].name,
                monitors[1].width,
                monitors[1].height,
                monitors[1].refreshRate,
                0,
                0,
                monitors[1].scale
            );
            run_hyprctl_monitors_command(command1);
            run_hyprctl_monitors_command(command2);
        }
        if args.bottom {
            println!("Second Monitor on the bottom");
            let command1 = format!(
                "{},{}x{}@{},{}x{},{}",
                monitors[0].name,
                monitors[0].width,
                monitors[0].height,
                monitors[0].refreshRate,
                0,
                0,
                monitors[0].scale
            );
            let command2 = format!(
                "{},{}x{}@{},{}x{},{}",
                monitors[1].name,
                monitors[1].width,
                monitors[1].height,
                monitors[1].refreshRate,
                0,
                monitors[0].height,
                monitors[1].scale
            );
            run_hyprctl_monitors_command(command1);
            run_hyprctl_monitors_command(command2);
        }
        if args.left {
            println!("Second Monitor on the left");
            let command1 = format!(
                "{},{}x{}@{},{}x{},{}",
                monitors[0].name,
                monitors[0].width,
                monitors[0].height,
                monitors[0].refreshRate,
                monitors[1].width,
                0,
                monitors[0].scale
            );
            let command2 = format!(
                "{},{}x{}@{},{}x{},{}",
                monitors[1].name,
                monitors[1].width,
                monitors[1].height,
                monitors[1].refreshRate,
                0,
                0,
                monitors[1].scale
            );
            run_hyprctl_monitors_command(command1);
            run_hyprctl_monitors_command(command2);
        }
    } else {
        eprintln!("Error running hyprctl command: {:?}", output.status);
    }
}
