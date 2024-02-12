// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{
    process::Command,
    path::Path,
    fs,
    error::Error
};
use bcrypt::{hash, DEFAULT_COST};
// Get partitions use the lsblk command to get disks and their partitions
#[tauri::command]
fn get_partitions() -> Result<String, tauri::Error> {
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
fn get_timezones() -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> = Command::new("timedatectl")
    .arg("list-timezones")
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
        Err(_)=>todo!()
    }
}

// check if system is uefi
#[tauri::command]
fn is_uefi() -> Result<String, tauri::Error> {
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
        Err(_)=>todo!()
    }
}

// save the config file in /tmp/conf.file
#[tauri::command]
fn save_conf(data: String) {
    let path = Path::new("/tmp/config.json");
    if path.exists() {
        println!("The file exists.");
    } else {
        println!("The file exists");
    }
    println!("{}", data);
}

// hash password hashes the password
#[tauri::command]
fn hash_password(password: &str) -> Result<String, tauri::Error> {
    let hashed = hash(password, DEFAULT_COST)
        .map_err(|_| tauri::Error::InvalidWindowUrl("Failed to hash password"))?;
    Ok(hashed)
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    Ok(file_content)
}

// keymaps
#[tauri::command]
fn get_keymaps() -> Result<String, tauri::Error> {
    let keymap_content = include_str!("keymaps");
    Ok(keymap_content.to_string())
}     

// locale
#[tauri::command]
fn get_locale() -> Result<String, tauri::Error> {
    let locale_content = include_str!("locale");
    Ok(locale_content.to_string())
} 
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_partitions, is_uefi, get_timezones, save_conf, hash_password, get_keymaps, get_locale])
        .plugin(tauri_plugin_system_info::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

