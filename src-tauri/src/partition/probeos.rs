use serde::{Deserialize, Serialize};
use std::process::Command;
use std::str;
use tokio::task;
use tokio::time::{timeout, Duration};
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OsProber {
    pub subpath: Option<String>,
    pub long: Option<String>,
    pub label: Option<String>,
    pub type_: Option<String>,
    pub version: Option<String>,
    pub raw: Option<String>,
    pub can_install_along: Option<bool>,
}



pub async fn probe_os_async() -> Vec<OsProber> {
    let output = task::spawn_blocking(|| {
        Command::new("sh")
            .arg("-c")
            .arg("sudo os-prober")
            .output()
            .expect("Failed to execute os-prober")
    })
    .await
    .expect("Failed to run blocking task");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in os-prober output");

    output_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let long_with_version = parts.get(1).unwrap_or(&"");
            let long_parts: Vec<&str> = long_with_version.split_whitespace().collect();
            let long = long_parts.get(0).unwrap_or(&"").to_string();
            let version = long_parts.get(1).map(|s| s.to_string());

            OsProber {
                subpath: Some(parts.get(0).unwrap_or(&"").to_string()),
                long: Some(long),
                label: Some(parts.get(2).unwrap_or(&"").to_string()),
                type_: Some(parts.get(3).unwrap_or(&"").to_string()),
                version,
                raw: Some(output_str.to_string()),
                can_install_along: None,
            }
        })
        .collect()
}

pub async fn run_probe_os_with_timeout() -> Vec<OsProber> {
    match timeout(Duration::from_secs(10), probe_os_async()).await {
        Ok(result) => result,
        Err(_) => vec![],
    }
}
