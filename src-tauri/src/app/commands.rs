// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::config;
use crate::partition;
use bcrypt::{hash, DEFAULT_COST};
use std::{error::Error, process::Command};
use std::{fs::File, io::Write, path::Path};
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
    let keymap_content = include_str!("keymaps");
    Ok(keymap_content.to_string())
}

// locale
#[tauri::command]
pub fn get_locale() -> Result<String, tauri::Error> {
    let locale_content = include_str!("locale");
    Ok(locale_content.to_string())
}

#[allow(dead_code)]
pub fn read_file1(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// save the config file in /tmp/conf.file
#[tauri::command]
#[allow(dead_code)]
pub fn save_conf(data: String) {
    let config = config::Config::from_json_string(data.clone());
    let def_device = &partition::device::Device::default();
    println!("{:#?}", config);
    match config.partition.mode.as_str() {
        "install-along" => {
            let mut gs = partition::gs::GlobalStorage::new();
            gs.probe();
            let kname = &config.partition.installAlongPartitions[0].kname;
            let device = gs
                .devices
                .iter()
                .find(|&d| d.name == Some(get_disk_id(kname)))
                .unwrap_or(def_device);
            match partition::device::partition_install_along(
                config.partition.installAlongPartitions,
                device.clone(),
            ) {
                Ok(_) => {
                    println!("partitioning successful");
                }
                Err(r) => {
                    println!("error partioning for install along {:#?}", r);
                }
            }
        }
        &_ => todo!(),
    }
    let p: &'static str = "/tmp/config.json";
    let path = Path::new(p);
    if path.exists() {
        println!("The file exists.");
        // delete file
        delete_file(p).expect("unable to delete file");
    } else {
        println!("The file exists");
    }
    println!("{}", data);
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
    let mut gs = partition::gs::GlobalStorage::new();
    gs.probe();
    gs.to_json_string()
}

#[tauri::command]
pub fn human_to_bytes(d: String) -> Result<String, tauri::Error> {
    match partition::utils::human2bytes(&d) {
        Ok(k) => Ok(format!("{:?}", k)),
        Err(_) => todo!(),
    }
}

fn get_disk_id(partition_id: &str) -> String {
    partition_id
        .chars()
        .rev()
        .skip_while(|c| c.is_digit(10)) // Skip digits from the end
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}
