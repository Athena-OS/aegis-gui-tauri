use std::fs;
use std::process::Command;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

pub fn sane_block_devices() -> Vec<String> {
    let mut devices = Vec::new();
    let paths = fs::read_dir("/sys/block").unwrap();
    
    for path in paths {
        let path = path.unwrap().path();
        let device_name = path.file_name().unwrap().to_string_lossy().into_owned();
        
        // You might want to filter out certain devices here, similar to checking the MAJOR number
        // For example, to skip loop devices, you might check if the device name starts with "loop"
        if !device_name.starts_with("loop") {
            devices.push(device_name);
        }
    }
    
    devices
}



/// Runs the `bcache-super-show` command and returns its output as a String.
pub fn run_bcache_super_show(device: &str) -> Option<String> {
    let output = Command::new("bcache-super-show")
        .arg(device)
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

/// Parses bcache superblock data from a string, returning a HashMap.
pub fn parse_superblock_data(data: &str) -> HashMap<String, String> {
    let mut bcache_super = HashMap::new();
    let line_regex = Regex::new(r"^(.*):\s*(.*)$").unwrap();
    for line in data.lines() {
        if let Some(caps) = line_regex.captures(line) {
            bcache_super.insert(caps[1].to_string(), caps[2].to_string());
        }
    }
    bcache_super
}

/// Checks if the given device path is a bcache backing device.
pub fn is_backing(device: &str) -> bool {
    Path::new(&format!("/sys/class/block/{}/bcache/label", device)).exists()
}

/// Checks if the given device path is a bcache caching device.
pub fn is_caching(device: &str) -> bool {
    Path::new(&format!("/sys/class/block/{}/bcache/cache_replacement_policy", device)).exists()
}