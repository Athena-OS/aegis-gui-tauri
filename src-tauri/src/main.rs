// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// sudo lshw -c disk -c storage
// df | grep -v ^/dev/loop

use std::{
    error::Error,
    process::{Command, Stdio},
};

#[tauri::command]
fn get_storage_devices() -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> = Command::new("pkexec")
        .arg("--user")
        .arg("root")
        .arg("fdisk")
        .arg("-l")
        .arg("--bytes")
        .stdout(Stdio::piped())
        .output();

    println!("{:?}", output);

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

#[tauri::command]
fn get_partitions_file_systems() -> Result<String, tauri::Error>{
    let output: Result<std::process::Output, std::io::Error> = Command::new("df")
        .arg("-T")
        .stdout(Stdio::piped())
        .output();

    println!("{:?}", output);

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
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_storage_devices, get_partitions_file_systems])
        .plugin(tauri_plugin_system_info::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
