

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::str;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OsProber {
    subpath: Option<String>,
    long: Option<String>,
    label: Option<String>,
    type_: Option<String>,
    version: Option<String>,
}
#[allow(dead_code)]
pub fn probe_os() -> Vec<OsProber> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo os-prober")
        .output()
        .expect("Failed to execute os-prober");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in os-prober output");

    output_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let long_with_version = parts.get(1).unwrap_or(&"");
            let long_parts: Vec<&str> = long_with_version.split_whitespace().collect();
            let long = long_parts.get(0).unwrap_or(&"").to_string();
            let version = long_parts.get(1).map(|&s| s.to_string());

            OsProber {
                subpath: Some(parts.get(0).unwrap_or(&"").to_string()),
                long:Some(long),
                label: Some(parts.get(2).unwrap_or(&"").to_string()),
                type_: Some(parts.get(3).unwrap_or(&"").to_string()),
                version,
            }
        })
        .collect()
}
#[allow(dead_code)]
pub fn os_probers_to_string(probers: &[OsProber]) -> String {
    serde_json::to_string(probers).unwrap()
}
#[allow(dead_code)]
pub fn string_to_os_probers(data: &str) -> Vec<OsProber> {
    serde_json::from_str(data).unwrap()
}
