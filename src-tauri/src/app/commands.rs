use crate::app::{config, global_app, install};
use crate::partition::*;
use async_std::task;
use std::{fs::File, io::Write, process::Command};

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
                Err(tauri::Error::WebviewLabelAlreadyExists(
                    var_name.to_string(),
                ))
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
                Err(tauri::Error::WebviewLabelAlreadyExists(
                    var_name.to_string(),
                ))
            }
        }
        Err(_) => todo!(),
    }
}

// Get a list of x11 keymaps
#[tauri::command]
pub fn get_x11_keymaps() -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> = Command::new("localectl")
        .arg("list-x11-keymap-layouts")
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                Ok(output_str.into())
            } else {
                let var_name = "Command execution failed";
                Err(tauri::Error::WebviewLabelAlreadyExists(
                    var_name.to_string(),
                ))
            }
        }
        Err(_) => todo!(),
    }
}
// check if system is uefi
#[tauri::command]
pub fn is_uefi() -> Result<String, tauri::Error> {
    println!("is uefi run");
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
                Err(tauri::Error::WebviewLabelAlreadyExists(
                    var_name.to_string(),
                ))
            }
        }
        Err(e) => {
            println!("failed running is uefi {}", e);
            let var_name = "Failed running is uefi";
            Err(tauri::Error::WebviewLabelAlreadyExists(
                var_name.to_string(),
            ))
        }
    }
}

// hash password hashes the password
#[tauri::command]
pub fn hash_password(password: &str) -> Result<String, tauri::Error> {
    let output: Result<std::process::Output, std::io::Error> = Command::new("openssl")
        .arg("passwd")
        .arg("-6")
        .arg(password)
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                Ok(output_str.into())
            } else {
                let var_name = "Command execution failed";
                Err(tauri::Error::WebviewLabelAlreadyExists(
                    var_name.to_string(),
                ))
            }
        }
        Err(_) => todo!(),
    }
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

// save the config file in /tmp/conf.file
#[tauri::command]
pub fn install(data: String) {
    // get config and update global store
    global_app::update_config(config::Config::from_json_string(data));
    // start installation in the background
    task::spawn(async move { install::install().await });
}

pub fn save_json(data: &str, filename: &str) -> std::io::Result<()> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(filename)?;
    // Write the data to the file
    file.write_all(data.as_bytes())?;

    Ok(())
}

#[tauri::command]
pub fn get_gs() -> String {
    let mut gs = gs::GlobalStorage::new();
    gs.probe();
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

#[tauri::command]
pub fn save_luks_passphrase(d: String) {
    let mut file = match File::create("/tmp/luks") {
        Ok(o) => o,
        Err(_) => return,
    };
    let _ = file.write_all(d.as_bytes());
}

// Read athena logs
pub fn read_athena_logs() -> String {
    match std::fs::read_to_string("/tmp/aegis.log") {
        Ok(o) => o,
        Err(_) => String::new(),
    }
}

#[tauri::command]
pub fn share_logs() -> Result<String, tauri::Error> {
    // These are any logs before that backend is reached. Simply the GUI logs.
    let mut al = match std::fs::read_to_string("prefix.log") {
        Ok(o) => o,
        Err(_) => String::new(),
    };
    // The backend logs
    al = format!("{al} \n {}", read_athena_logs());

    let output: Result<std::process::Output, std::io::Error> = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(r#"echo "{}" | nc termbin.com 9999"#, al))
        .output();
    match output {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            Ok(s.into())
        }
        Err(e) => {
            Err(tauri::Error::Anyhow(e.into()))
        }
    }
}

#[tauri::command]
pub fn get_all_logs() -> Result<String, tauri::Error> {
    // These are any logs before that backend is reached. Simply the GUI logs.
    let mut al = match std::fs::read_to_string("prefix.log") {
        Ok(o) => o,
        Err(_) => String::new(),
    };
    // The backend logs
    al = format!("{al} \n {}", read_athena_logs());
    Ok(al)
}
