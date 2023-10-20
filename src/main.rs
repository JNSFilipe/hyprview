use std::process::{Command, Output};
use std::str::FromStr;

#[derive(Debug, Default)]
struct Monitor {
    id: String,
    resolution: String,
    position: String,
    frame_rate: f64,
    description: String,
    make: String,
    model: String,
    serial: String,
    active_workspace: u32,
    special_workspace: String,
    reserved: String,
    scale: f64,
    transform: u32,
    focused: bool,
    dpms_status: u32,
    vrr: u32,
}

impl FromStr for Monitor {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monitor = Monitor::default();

        for line in s.lines() {
            let mut toks = line.splitn(2, ' ').fuse();
            let key = toks.next().unwrap().trim();
            let value = toks.next().unwrap();
            match key {
                "Monitor" => {
                    monitor.id = value.to_string();
                }
                "description:" => {
                    monitor.description = value.to_string();
                }
                "make:" => {
                    monitor.make = value.to_string();
                }
                "model:" => {
                    monitor.model = value.to_string();
                }
                "serial:" => {
                    monitor.active_workspace = value
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                    monitor.serial = value.to_string();
                }
                "active" => {
                    monitor.active_workspace = value
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                }
                "special" => {
                    let mut aux = value.split(' ').fuse();
                    let _ = aux.next();
                    let sw = aux.next().unwrap();
                    monitor.special_workspace = sw.to_string();
                }
                "reserved:" => {
                    let mut aux = value.splitn(2, ' ').fuse();
                    let _ = aux.next();
                    let rw = aux.next().unwrap();
                    monitor.reserved = rw.to_string();
                }
                "scale:" => {
                    monitor.scale = value.parse().unwrap_or(0.0);
                }
                "transform:" => {
                    monitor.transform = value.parse().unwrap_or(0);
                }
                "focused:" => {
                    monitor.focused = value == "yes";
                }
                "dpmsStatus:" => {
                    monitor.dpms_status = value.parse().unwrap_or(0);
                }
                "vrr:" => {
                    monitor.vrr = value.parse().unwrap_or(0);
                }
                _ => {
                    if key.contains("@") {
                        let mut aux = key.split("@").fuse();
                        let res = aux.next().unwrap();
                        let fps = aux.next().unwrap();

                        let mut aux = value.split(" ").fuse();
                        let _ = aux.next();
                        let pos = aux.next().unwrap();

                        monitor.resolution = res.to_string();
                        monitor.frame_rate = fps.parse().unwrap();
                        monitor.position = pos.to_string();
                    }
                }
            }
        }
        Ok(monitor)
    }
}

fn get_monitor_info() -> Option<Vec<Monitor>> {
    // Specify the command you want to run
    // let command = "hyprctl monitors";

    // Use the Command struct to create and configure the command
    let output: Output = Command::new("hyprctl")
        .arg("monitors")
        .output()
        .expect("Failed to execute command. You sure hyprctl is installed?");

    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to a string
        let output_string = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", output_string);

        let monitors: Vec<Monitor> = output_string
            .split("\n\n")
            .filter(|&block| !block.trim().is_empty())
            .map(|block| block.parse().unwrap())
            .collect();

        return Some(monitors);
    } else {
        eprintln!(
            "Command failed with error:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        None
    }
}

fn main() {
    let monitors = get_monitor_info().unwrap();

    if monitors.len() < 2 {
        println!("No external monitor connected!")
    }

    for (i, monitor) in monitors.iter().enumerate() {
        println!("Monitor {}: {:?}", i, monitor);
    }
}
