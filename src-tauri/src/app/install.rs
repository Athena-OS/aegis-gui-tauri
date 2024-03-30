use crate::app::*;
use crate::partition;
use crate::partition::*;
use log::*;
use std::{io::*, process::*};

pub async fn install() {
    // We first partition the disks.
    match do_partitions() {
        Ok(_) => info!(""),
        Err(_) => return,
    };
    // After successful partitioning, we save the config file
    match save_config() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    let config = global_app::get_config().unwrap_or_default();

    match config.base.as_str() {
        "arch" => match install_arch() {
            Ok(_) => info!(""),
            Err(_) => return,
        },
        "nixos" => match install_arch() {
            Ok(_) => info!(""),
            Err(_) => return,
        },
        &_ => todo!(),
    }
    /*match do_install_base() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_locale() {
        Ok(_) => info!(""),
        Err(_) => return,
    };
    match do_install_packages() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_gen_fstab() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_boot_loader() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_networking() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_zramd() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_hardened() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_desktops() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_themes() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_display_managers() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_browsers() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_terminals() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_setup_snapper() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    match do_flatpak() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    do_users();

    match do_nix() {
        Ok(_) => info!(""),
        Err(_) => return,
    };

    do_cuda();

    do_spotify();

    do_cherrytree();

    do_flameshot();

    do_busybox();

    do_toybox();

    do_config();

    do_shells();

    do_enable_services();*/
}
#[allow(dead_code)]
fn do_partitions() -> std::result::Result<bool, Box<dyn std::error::Error>> {
    info!("[AEGIS TAURI] partition disks for installation.");
    // The default should never be called.
    let config = global_app::get_config().unwrap_or_default();
    info!("Block device to use : {}", config.partition.device);
    info!("Partitioning mode : {:?}", config.partition.mode);
    info!("Partitioning for EFI : {}", config.partition.efi);
    let def_device = &partition::device::Device::default();
    match config.partition.mode.as_str() {
        "install-along" => {
            // Get global storage. Should never call default
            let gs = global_app::get_global_storage().unwrap_or_default();
            let kname = &config.partition.installAlongPartitions[0].kname;
            // device being used for installation
            let device = gs
                .devices
                .iter()
                .find(|&d| d.name == Some(utils::get_disk_id(kname)))
                .unwrap_or(def_device);
            match partition::device::partition_install_along(
                config.partition.installAlongPartitions,
                device.clone(),
            ) {
                Ok(_) => {
                    info!("partitioning successful");
                    global_app::update_progress();
                    // continue installation
                    Ok(true)
                }
                Err(r) => {
                    // notify the frontend of the installation failure.
                    global_app::emit_global_event("install-fail", "");
                    error!("error partioning for install along {:#?}", r);
                    // Return error to be used to exit installation.
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error partioning for install along",
                    )))
                }
            }
        }
        "manual" => {
            // Get the partitions that needs deleting and delete them
            for storage_info in &config.partition.system_storage_info_current {
                for partition in &storage_info.partitions {
                    if partition.action == Some(String::from("delete")) {
                        let partition_name = partition.partitionName.clone().unwrap_or_default();
                        let pn = device::get_partition_number(&partition_name) as i32;
                        let dev = format!("/dev/{}", utils::get_disk_id(&partition_name));

                        match device::delete_partition(&dev, pn) {
                            Ok(true) => info!("Partition deleted successfully"),
                            Ok(false) => {
                                error!("Deleting partition failed.");
                                global_app::emit_global_event("install-fail", "");
                                return Err(Box::new(std::io::Error::new(
                                    std::io::ErrorKind::AddrInUse,
                                    "",
                                )));
                            }
                            Err(e) => {
                                error!("Deleting partition failed with error: {:#?}", e);
                                global_app::emit_global_event("install-fail", "");
                                return Err(Box::new(e));
                            }
                        }
                    }
                }
            }

            // Get the partitions that needs shrinking/expanding and perform the shrinking expanding action
            for storage_info in &config.partition.system_storage_info {
                for item in &storage_info.partitions {
                    if item.action == Some(String::from("shrink")) {
                        // Here we shrink the partition
                        // Get partition number
                        let pn = device::get_partition_number(
                            &item.partitionName.clone().unwrap_or(String::new()),
                        );
                        // Get the device
                        let dev = format!(
                            "/dev/{}{}",
                            config.partition.device,
                            utils::get_disk_id(
                                &item.partitionName.clone().unwrap_or(String::new())
                            )
                        );
                        // Convert size to Human readerble form. Should never unwrap
                        //let s =  item.size
                        let size = utils::bytes2human(item.size.unwrap_or(0) as f64)
                            .unwrap_or(String::from("1GB"));
                        // Delete the partition
                        match device::resize_partition(&dev, pn, &size) {
                            Ok(_) => info!("partition resized successfully"),
                            Err(e) => {
                                error!("Resizing partition failed with error: {:#?}", e);
                                global_app::emit_global_event("install-fail", "");
                                return Err(Box::new(e));
                            }
                        };
                    }
                }
            }

            // With shrinked partitions, we can create new partitions now (Free spaces created out of deletion are ignored)
            // Get the partitions that needs shrinking/expanding and perform the shrinking expanding action
            for storage_info in &config.partition.system_storage_info {
                for item in &storage_info.partitions {
                    if item.action == Some(String::from("create")) {
                        // Here we create the partition
                        println!("{:#?}", item);
                        // Get start and end in human readable form.
                        let start = utils::bytes2human((item.start.unwrap_or(1024) * 512) as f64)
                            .unwrap_or(String::from("1GB"));
                        let end = utils::bytes2human((item.end.unwrap_or(1024) * 512) as f64)
                            .unwrap_or(String::from("1GB"));
                        // Get the device
                        let dev = format!(
                            "/dev/{}",
                            config.partition.device
                        );
                        //Get the fstype
                        let fstype = item.fileSytem.clone().unwrap_or(String::from("ext4"));
                        // Create the partition
                        match device::create_partition(&dev, &fstype, &start, &end) {
                            Ok(_) => info!("partition  successfully"),
                            Err(e) => {
                                error!("Creating partition failed with error: {:#?}", e);
                                global_app::emit_global_event("install-fail", "");
                                return Err(Box::new(e));
                            }
                        };
                    }
                }
            }

            // Here, Manual partitioning is done
            Ok(true)
        }
        // We dont need any partition action here
        &_ => {
            info!("not handled");
            Ok(true)
        }
    }
}
#[allow(dead_code)]
fn do_install_base() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing the base.");
    let args = vec![String::from("athena-aegis"), String::from("install-base")];
    run_command(args)
}
#[allow(dead_code)]
fn do_install_packages() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing packages.");
    let config = global_app::get_config().unwrap_or_default();
    let args = vec![
        String::from("athena-aegis"),
        String::from("install-packages"),
        String::from("--kernel"),
        config.kernel.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_gen_fstab() -> std::io::Result<()> {
    info!("[AEGIS TAURI] generating fstab.");
    let args = vec![String::from("athena-aegis"), String::from("genfstab")];
    run_command(args)
}
#[allow(dead_code)]
fn do_setup_snapper() -> std::io::Result<()> {
    info!("[AEGIS TAURI] setting up snapper.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing snapper : {}", config.snapper);
    if config.snapper {
        let args = vec![String::from("athena-aegis"), String::from("setup-snapper")];
        run_command(args)
    } else {
        global_app::update_progress();
        Ok(())
    }
}
#[allow(dead_code)]
fn do_boot_loader() -> std::io::Result<()> {
    info!("[AEGIS TAURI] setting up boot loader.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing bootloader : {}", config.bootloader.r#type);
    info!("Installing bootloader to : {}", config.bootloader.location);
    let args = vec![
        String::from("athena-aegis"),
        String::from("bootloader"),
        config.bootloader.r#type.clone(),
        config.bootloader.location.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_locale() -> std::io::Result<()> {
    info!("[AEGIS TAURI] setting up locale.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Adding Locales : {:?}", config.locale.locale);
    info!("Using keymap : {}", config.locale.virtkeymap);
    info!("Setting timezone : {}", config.locale.timezone);
    let mut args: Vec<String> = vec![
        String::from("athena-aegis"),
        String::from("locale"),
        config.locale.virtkeymap,
        config.locale.timezone,
    ];
    args.extend(config.locale.locale);
    run_command(args)
}
#[allow(dead_code)]
fn do_networking() -> std::io::Result<()> {
    info!("[AEGIS TAURI] setting up network service.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Hostname : {}", config.networking.hostname);
    info!("Enabling ipv6 : {}", config.networking.ipv6);
    let mut args = vec![String::from("athean-aegis"), String::from("networking")];
    if config.networking.ipv6 {
        args.push(String::from("--ipv6"))
    }
    args.push(config.networking.hostname);
    run_command(args)
}
#[allow(dead_code)]
fn do_zramd() -> std::io::Result<()> {
    info!("[AEGIS TAURI] setting up zramd.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Enabling zramd : {}", config.zramd);
    if config.zramd {
        let args = vec![String::from("athena-aegis"), String::from("zramd")];
        run_command(args)
    } else {
        global_app::update_progress();
        Ok(())
    }
}
#[allow(dead_code)]
fn do_hardened() -> std::io::Result<()> {
    info!("[AEGIS TAURI] perfoming hardening.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Hardening system : {}", config.hardened);
    if config.hardened {
        let args = vec![String::from("athena-aegis"), String::from("hardened")];
        run_command(args)
    } else {
        global_app::update_progress();
        Ok(())
    }
}
#[allow(dead_code)]
fn do_users() {
    info!("[AEGIS TAURI] setting up users. Configring users and passowords.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_nix() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing nix package manager");
    let args = vec![String::from("athena-aegis"), String::from("nix")];
    run_command(args)
}
#[allow(dead_code)]
fn do_flatpak() -> std::io::Result<()> {
    info!("[AEGIS TAURI] Installing Flatpak and enabling FlatHub.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing snapper : {}", config.flatpak);
    if config.flatpak {
        let args = vec![String::from("athena-aegis"), String::from("hardened")];
        run_command(args)
    } else {
        global_app::update_progress();
        Ok(())
    }
}
#[allow(dead_code)]
fn do_cuda() {
    info!("[AEGIS TAURI] installing CUDA.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_spotify() {
    info!("[AEGIS TAURI] installing spotify.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_cherrytree() {
    info!("[AEGIS TAURI] installing cherrytree.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_flameshot() {
    info!("[AEGIS TAURI] installing flameshot.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_busybox() {
    info!("[AEGIS TAURI] installing busybox.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_toybox() {
    info!("[AEGIS TAURI] installing toybox.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_config() {
    info!("[AEGIS TAURI] reading config.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_desktops() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing desktop.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing desktop : {:?}", config.desktop);
    let args = vec![
        String::from("athena-aegis"),
        String::from("desktops"),
        config.desktop.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_themes() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing themes.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing theme : {:?}", config.theme);
    let args = vec![
        String::from("athena-aegis"),
        String::from("themes"),
        config.theme.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_display_managers() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing display manager.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing display manager : {:?}", config.displayManager);
    let args = vec![
        String::from("athena-aegis"),
        String::from("displaymanagers"),
        config.displayManager.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_shells() {
    info!("[AEGIS TAURI] installing shells.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn do_browsers() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing browsers.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing browser : {:?}", config.browser);
    let args = vec![
        String::from("athena-aegis"),
        String::from("browsers"),
        config.displayManager.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_terminals() -> std::io::Result<()> {
    info!("[AEGIS TAURI] installing terminals.");
    let config = global_app::get_config().unwrap_or_default();
    info!("Installing terminals : {:?}", config.terminal);
    let args = vec![
        String::from("athena-aegis"),
        String::from("terminals"),
        config.terminal.clone(),
    ];
    run_command(args)
}
#[allow(dead_code)]
fn do_enable_services() {
    info!("[AEGIS TAURI] enabling services.");
    global_app::update_progress();
}
#[allow(dead_code)]
fn save_config() -> std::result::Result<bool, Box<dyn std::error::Error>> {
    let mut config = match global_app::get_config() {
        Ok(c) => c,
        Err(e) => {
            error!("failed to get config from global storage:{}", e);
            global_app::emit_global_event("install-fail", "");
            // exit if config is not present
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error partioning for install along",
            )));
        }
    };
    let def_device = &partition::device::Device::default();
    match config.partition.mode.as_str() {
        // install along
        "install-along" => {
            // Get global storage
            let gs = partition::gs::GlobalStorage::new();
            let kname = &config.partition.installAlongPartitions[0].kname;
            // device being used for installation
            let device = gs
                .devices
                .iter()
                .find(|&d| d.name == Some(utils::get_disk_id(kname)))
                .unwrap_or(def_device);
            // update config
            config.partition.mode = String::from("Manual");
            config.partition.device = device
                .path
                .as_ref()
                .unwrap_or(&String::from(""))
                .to_string();
            let mut partition: Vec<String> = device
                .partitions
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|item| {
                    // Check if the path is Some or None and format accordingly
                    format!(
                        "none:{}:none",
                        item.path.as_ref().unwrap_or(&"".to_string())
                    )
                })
                .collect();
            partition.push(format!(
                "/mnt:{}{}:btrfs",
                device.path.as_ref().unwrap_or(&String::from("")),
                device
                    .partitions
                    .as_ref()
                    .map(|p| p.clone())
                    .unwrap_or(vec![])
                    .len()
                    + 1
            ));
            config.partition.partitions = serde_json::to_value(partition).unwrap_or_default();
            info!("saving config. config.");
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(e) => {
                    error!("error converting config to string with error: {:?}", e);
                    // send install event failure
                    global_app::emit_global_event("install-fail", "");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error saving config",
                    )));
                }
            };
            match commands::save_json(&config_str, "/tmp/config.json") {
                Ok(_) => {
                    global_app::update_progress();
                    Ok(true)
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        // auto
        "auto" => {
            info!("saving config. config.");
            config.partition.mode = String::from("Auto");
            // Partitions ingnored since  the device will be formatted anyway.
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(e) => {
                    error!("error converting config to string with error: {:?}", e);
                    // send install event failure
                    global_app::emit_global_event("install-fail", "");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error saving config",
                    )));
                }
            };
            match commands::save_json(&config_str, "/tmp/config.json") {
                Ok(_) => {
                    global_app::update_progress();
                    Ok(true)
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        // replace partition
        "replace-partition" => {
            let mut partition: Vec<String> = config
                .partition
                .system_storage_info
                .iter()
                .flat_map(|s| {
                    s.partitions.iter().map(|item| {
                        if item.name == Some(String::from("Athena OS")) {
                            format!(
                                "/mnt:/dev/{}:btrfs",
                                item.partitionName.as_ref().unwrap_or(&"".to_string())
                            )
                        } else {
                            format!(
                                "none:/dev/{}:none",
                                item.partitionName.as_ref().unwrap_or(&"".to_string())
                            )
                        }
                    })
                })
                .collect();
            partition = partition
                .iter()
                .filter(|item| item.contains(&config.partition.device))
                .cloned()
                .collect();

            config.partition.mode = String::from("Manual");
            config.partition.partitions = serde_json::to_value(partition).unwrap_or_default();
            info!("saving config. config");
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(e) => {
                    error!("error converting config to string with error: {:?}", e);
                    // send install event failure
                    global_app::emit_global_event("install-fail", "");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error saving config",
                    )));
                }
            };
            match commands::save_json(&config_str, "/tmp/config.json") {
                Ok(_) => {
                    global_app::update_progress();
                    Ok(true)
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        // replace partition
        "manual" => {
            let mut partition: Vec<String> = config
                .partition
                .system_storage_info
                .iter()
                .flat_map(|s| {
                    s.partitions.iter().map(|item| {
                        if item.name == Some(String::from("Athena OS")) {
                            // Check if its a new partition
                            if item.action == Some(String::from("create")) {
                                // Get the created device part number
                                let pn = device::get_partition_number(
                                    &item.partitionName.clone().unwrap_or(String::new()),
                                );
                                format!("/mnt:/dev/{}{}:btrfs", config.partition.device, pn)
                            } else {
                                format!(
                                    "/mnt:/dev/{}:btrfs",
                                    item.partitionName.as_ref().unwrap_or(&"".to_string())
                                )
                            }
                        } else {
                            format!(
                                "none:/dev/{}:none",
                                item.partitionName.as_ref().unwrap_or(&"".to_string())
                            )
                        }
                    })
                })
                .collect();
            partition = partition
                .iter()
                .filter(|item| item.contains(&config.partition.device))
                .cloned()
                .collect();

            config.partition.mode = String::from("Manual");
            //println!("partition {:#?}", partition);
            config.partition.partitions = serde_json::to_value(partition).unwrap_or_default();
            info!("saving config. config.");
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(e) => {
                    error!("error converting config to string with error: {:?}", e);
                    // send install event failure
                    global_app::emit_global_event("install-fail", "");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error saving config",
                    )));
                }
            };
            match commands::save_json(&config_str, "/tmp/config.json") {
                Ok(_) => {
                    global_app::update_progress();
                    Ok(true)
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        &_ => {
            info!("not handled");

            Ok(true)
        }
    }
}
#[allow(dead_code)]
fn run_command(args: Vec<String>) -> std::io::Result<()> {
    match std::process::Command::new("sudo")
        .args(args.clone())
        .stdout(Stdio::piped()) // Capture standard output
        .stderr(Stdio::piped()) // Capture standard error
        .spawn()
    {
        Ok(mut child) => {
            // Handle stdout
            if let Some(ref mut stdout) = child.stdout {
                let stdout_reader = BufReader::new(stdout);
                for line in stdout_reader.lines() {
                    match line {
                        Ok(line) => info!("[ATHENA AEGIS]: {}", line),
                        Err(e) => error!("[AEGIS TAURI] Error reading stdout line: {}", e),
                    }
                }
            }

            // Handle stderr
            if let Some(ref mut stderr) = child.stderr {
                let stderr_reader = BufReader::new(stderr);
                for line in stderr_reader.lines() {
                    match line {
                        Ok(line) => {
                            error!("stderr: {}", line);
                            // Send Fails event
                            global_app::emit_global_event("install-fail", "");
                        }
                        Err(e) => {
                            error!("Error reading stderr line: {}", e);
                            global_app::emit_global_event("install-fail", "");
                        }
                    }
                }
            }

            // Wait for command to complete
            match child.wait() {
                Ok(output) => {
                    info!(
                        "[AEGIS TAURI] running args {:?} finished with status: {}",
                        args, output
                    );
                    // update progress
                    global_app::update_progress();
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to wait on child process: {}", e);
                    global_app::emit_global_event("install-fail", "");
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Failed to spawn child process: {}", e);
            global_app::emit_global_event("install-fail", "");
            Err(e)
        }
    }
}

#[allow(dead_code)]
fn install_extra_packages() -> std::io::Result<()> {
    athena_aegis::install::install(athena_aegis::PackageManager::Pacman, vec![]);
    Ok(())
}

#[allow(dead_code)]
fn install_arch() -> std::io::Result<()> {
    info!("[AEGIS TAURI] Athena OS with Arch Linux base.");
    let args = vec![
        String::from("aegis-arch"),
        String::from("config"),
        String::from("/tmp/config.json"),
    ];
    run_command(args)
}

#[allow(dead_code)]
fn install_nix() -> std::io::Result<()> {
    info!("[AEGIS TAURI] Athena OS with Nixos Linux base.");
    let args = vec![
        String::from("aegis-nix"),
        String::from("config"),
        String::from("/tmp/config.json"),
    ];
    run_command(args)
}
