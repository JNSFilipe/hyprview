use serde_derive::{Deserialize, Serialize};
use std::process::{Command, Output};

// Define the struct for the Monitor data
#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: u32,
    pub height: u32,
    pub refreshRate: f64,
    pub x: u32,
    pub y: u32,
    pub activeWorkspace: Workspace,
    pub specialWorkspace: Workspace,
    pub reserved: Vec<u32>,
    pub scale: f64,
    pub transform: u32,
    pub focused: bool,
    pub dpmsStatus: bool,
    pub vrr: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    id: u32,
    name: String,
}

pub fn get_hyprctl_monitors_output() -> Output {
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("monitors")
        .output()
        .expect("Failed to execute hyprctl command");

    output
}

pub fn run_hyprctl_monitors_command(arg: String) {
    let command = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(arg)
        .output()
        .expect("Failed to execute hyprctl command");
}
