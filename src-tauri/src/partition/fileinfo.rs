use crate::partition::utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct FileSystemInfo {
    filesystem: String,
    size: String,
    used: String,
    available: String,
    use_percentage: String,
    mounted_on: String,
}

#[allow(dead_code)]
pub fn get_file_info() -> Result<String, serde_json::Error> {
    let output = std::process::Command::new("df")
        .arg("-h") // Human-readable sizes
        .output()
        .expect("Failed to execute command");

    let output_str = std::str::from_utf8(&output.stdout).unwrap_or("");

    let mut fs_infos: Vec<FileSystemInfo> = Vec::new();

    for (i, line) in output_str.lines().enumerate() {
        if i == 0 {
            continue;
        } // Skip headers

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        } // Basic validation

        let fs_info = FileSystemInfo {
            filesystem: parts[0].to_string(),
            size: parts[1].to_string(),
            used: parts[2].to_string(),
            available: parts[3].to_string(),
            use_percentage: parts[4].to_string(),
            mounted_on: parts[5..].join(" "), // Handle mount points with spaces
        };

        fs_infos.push(fs_info);
    }

    utils::marshal_json(&fs_infos)
}
