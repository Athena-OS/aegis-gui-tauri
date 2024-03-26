use crate::app::{config, global_app, install};
use crate::partition::*;
use bcrypt::{hash, DEFAULT_COST};
use std::thread;
use std::{error::Error, fs::File, io::Write, process::Command};

// Get partitions use the lsblk command to get disks and their partitions
#[tauri::command]
pub fn get_partitions() -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> = Command::new("lsblk")
        .arg("-O") // all fields
        .arg("-J") // json output
        .arg("--bytes") // bytes format
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                Ok(output_str.into())
            } else {
                let var_name = "Command execution failed";
                Err(tauri::Error::InvalidWindowUrl(var_name))
            }
        }
        Err(_) => todo!(),
    }
}

// Get a list of timezones
#[tauri::command]
pub fn get_timezones() -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> =
        Command::new("timedatectl").arg("list-timezones").output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                Ok(output_str.into())
            } else {
                let var_name = "Command execution failed";
                Err(tauri::Error::InvalidWindowUrl(var_name))
            }
        }
        Err(_) => todo!(),
    }
}

// check if system is uefi
#[tauri::command]
pub fn is_uefi() -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> = Command::new("sh")
        .arg("-c")
        .arg("[ -d /sys/firmware/efi ] && echo true || echo false")
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                Ok(output_str.into())
            } else {
                let var_name = "Command execution failed";
                Err(tauri::Error::InvalidWindowUrl(var_name))
            }
        }
        Err(_) => todo!(),
    }
}

// hash password hashes the password
#[tauri::command]
pub fn hash_password(password: &str) -> Result<String, tauri::Error> {
    let hashed = hash(password, DEFAULT_COST)
        .map_err(|_| tauri::Error::InvalidWindowUrl("Failed to hash password"))?;
    Ok(hashed)
}
#[allow(dead_code)]
pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(path)?;
    Ok(file_content)
}

// keymaps
#[tauri::command]
pub fn get_keymaps() -> Result<String, tauri::Error> {
    Ok(include_str!("keymaps").to_string())
}

// locale
#[tauri::command]
pub fn get_locale() -> Result<String, tauri::Error> {
    Ok(include_str!("locale").to_string())
}

#[allow(dead_code)]
pub fn read_file1(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// save the config file in /tmp/conf.file
#[tauri::command]
#[allow(dead_code)]
pub fn install(data: String) {
    // get config and update global store
    global_app::update_config(config::Config::from_json_string(data));
    // start installation in the background
    thread::spawn(move || install::install());
    
}
#[allow(dead_code)]
pub fn save_json(data: &str, filename: &str) -> std::io::Result<()> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(filename)?;
    // Write the data to the file
    file.write_all(data.as_bytes())?;

    Ok(())
}
#[allow(dead_code)]
pub fn read_json(_filename: &str) {}
#[allow(dead_code)]
fn delete_file(filename: &str) -> std::io::Result<()> {
    std::fs::remove_file(filename)?;
    Ok(())
}

#[tauri::command]
#[allow(dead_code)]
pub fn get_gs() -> String {
    global_app::get_global_storage()
        .unwrap_or_default()
        .to_json_string()
}

#[tauri::command]
pub fn human_to_bytes(d: String) -> Result<String, tauri::Error> {
    match utils::human2bytes(&d) {
        Ok(k) => Ok(format!("{:?}", k)),
        Err(_) => todo!(),
    }
}

