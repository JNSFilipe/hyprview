use serde_derive::Deserialize;
use std::process::{Command, Output};

// Define the struct for the Monitor data
#[derive(Debug, Deserialize)]
pub struct Monitor {
    id: u32,
    name: String,
    description: String,
    make: String,
    model: String,
    serial: String,
    width: u32,
    height: u32,
    refreshRate: f64,
    x: u32,
    y: u32,
    activeWorkspace: Workspace,
    specialWorkspace: Workspace,
    reserved: Vec<u32>,
    scale: f64,
    transform: u32,
    focused: bool,
    dpmsStatus: bool,
    vrr: bool,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    id: u32,
    name: String,
}

pub fn run_hyprctl_command() -> Output {
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("monitors")
        .output()
        .expect("Failed to execute hyprctl command");

    output
}
