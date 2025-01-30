use crate::app::*;
use crate::partition;
use crate::partition::*;

pub async fn install() {
    // We first partition the disks.
    match do_partitions() {
        Ok(_) => {},
        Err(_) => return,
    };
    // After successful partitioning, we save the config file
    match save_config() {
        Ok(_) => {},
        Err(_) => return,
    };

    let config = global_app::get_config().unwrap_or_default();

    match config.base.as_str() {
        "arch" => match install_arch() {
            Ok(_) => {},
            Err(_) => return,
        },
        "nixos" => match install_nix() {
            Ok(_) => {},
            Err(_) => return,
        },
        &_ => todo!(),
    }
}

fn do_partitions() -> std::result::Result<bool, Box<dyn std::error::Error>> {
    // The default should never be called.
    let mut config = global_app::get_config().unwrap_or_default();
    let def_device = &partition::device::Device::default();
    match config.partition.mode.as_str() {
        "install-along" => {
            // Get global storage. Should never call default
            let gs = global_app::get_global_storage().unwrap_or_default();
            let kname = &config.clone().partition.installAlongPartitions[0].kname;
            // device being used for installation
            let device = gs
                .devices
                .iter()
                .find(|&d| d.name == Some(utils::get_disk_id(kname)))
                .unwrap_or(def_device);
            match partition::device::partition_install_along(
                config.clone().partition.installAlongPartitions,
                device.clone(),
            ) {
                Ok(_) => {
                    let mut gs = partition::gs::GlobalStorage::new();
                    gs.probe();
                    // probe partitions
                    let device = gs
                        .devices
                        .iter()
                        .find(|&d| d.name == Some(utils::get_disk_id(kname)))
                        .unwrap_or(def_device);
                    let partition = device.partitions.clone().unwrap_or(vec![]);
                    if partition.len() > 1 {
                        // Update the config
                        config.partition.partitions = serde_json::to_value(vec![
                            format!(
                                "/mnt/:/dev/{kname}{}:ext4:{}",
                                partition.len(),
                                config.partition.encrypt_check
                            ),
                            format!(
                                "/mnt/boot:/dev/{kname}{}:vfat:{}",
                                partition.len() - 1,
                                false
                            ),
                        ])
                        .unwrap_or_default();
                        global_app::update_config(config.clone())
                    }
                    global_app::update_progress();
                    // continue installation
                    Ok(true)
                }
                Err(_) => {
                    // notify the frontend of the installation failure.
                    global_app::emit_global_event("install-fail", "");
                    // Return error to be used to exit installation.
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error partioning for install along",
                    )))
                }
            }
        }
        "manual" => {
            // New partition table
            if config.partition.new_ptable {
                // create new partition table
                let dt = match config.partition.new_pt_file_system.as_str() {
                    "mdos" => device::DeviceType::MDOS,
                    "gpt" => device::DeviceType::GPT,
                    &_ => device::DeviceType::GPT,
                };
                match device::clear_partition_device(config.partition.device.clone(), dt) {
                    Ok(_) => {
                        // Create partitions now that the partition table was created successfully
                        for storage_info in &config.partition.system_storage_info {
                            for item in &storage_info.partitions {
                                //if item.action == Some(String::from("create")) {
                                // Here we create the partition
                                println!("{:#?}", item);
                                // Get start and end in human readable form.
                                let start =
                                    utils::bytes2human((item.start.unwrap_or(1024) * 512) as f64)
                                        .unwrap_or(String::from("1GB"));
                                let end =
                                    utils::bytes2human((item.end.unwrap_or(1024) * 512) as f64)
                                        .unwrap_or(String::from("1GB"));
                                // Get the device
                                let dev = format!("/dev/{}", config.partition.device.clone());
                                //Get the fstype
                                let fstype = item.fileSytem.clone().unwrap_or(String::from("ext4"));
                                // Create the partition
                                match device::create_partition(&dev, &fstype, &start, &end) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        global_app::emit_global_event("install-fail", "");
                                        return Err(Box::new(e));
                                    }
                                };
                                //}
                            }
                        }
                    }
                    Err(e) => return Err(Box::new(e)),
                }
            } else {
                // Get the partitions that needs deleting and delete them
                for storage_info in &config.partition.system_storage_info_current {
                    for partition in &storage_info.partitions {
                        if partition.action == Some(String::from("delete")) {
                            let partition_name =
                                partition.partitionName.clone().unwrap_or_default();
                            let pn = device::get_partition_number(&partition_name) as i32;
                            let dev = format!("/dev/{}", utils::get_disk_id(&partition_name));

                            match device::delete_partition(&dev, pn) {
                                Ok(true) => {},
                                Ok(false) => {
                                    global_app::emit_global_event("install-fail", "");
                                    return Err(Box::new(std::io::Error::new(
                                        std::io::ErrorKind::AddrInUse,
                                        "",
                                    )));
                                }
                                Err(e) => {
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
                                Ok(_) => {},
                                Err(e) => {
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
                            let start =
                                utils::bytes2human((item.start.unwrap_or(1024) * 512) as f64)
                                    .unwrap_or(String::from("1GB"));
                            let end = utils::bytes2human((item.end.unwrap_or(1024) * 512) as f64)
                                .unwrap_or(String::from("1GB"));
                            // Get the device
                            let dev = format!("/dev/{}", config.partition.device);
                            //Get the fstype
                            let fstype = item.fileSytem.clone().unwrap_or(String::from("ext4"));
                            // Create the partition
                            match device::create_partition(&dev, &fstype, &start, &end) {
                                Ok(_) => {},
                                Err(e) => {
                                    global_app::emit_global_event("install-fail", "");
                                    return Err(Box::new(e));
                                }
                            };
                        }
                    }
                }
            }
            // Here, Manual partitioning is done
            Ok(true)
        }
        // We dont need any partition action here
        &_ => {
            Ok(true)
        }
    }
}


fn save_config() -> std::result::Result<bool, Box<dyn std::error::Error>> {
    let mut config = match global_app::get_config() {
        Ok(c) => c,
        Err(_) => {
            global_app::emit_global_event("install-fail", "");
            // exit if config is not present
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error partioning for install along",
            )));
        }
    };
    match config.partition.mode.as_str() {
        // install along
        "install-along" => {
            config.partition.mode = String::from("Manual");
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(_) => {
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
            config.partition.mode = String::from("Auto");
            config.partition.device = format!("/dev/{}", config.partition.device);
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(_) => {
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
            config.partition.mode = String::from("Replace");
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(_) => {
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
            config.partition.partitions = serde_json::to_value(partition).unwrap_or_default();
            let config_str = match utils::marshal_json(&config) {
                Ok(s) => s,
                Err(_) => {
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
            Ok(true)
        }
    }
}

fn install_arch() -> std::io::Result<()> {
    let args = vec![
        String::from("aegis-arch"),
        String::from("config"),
        String::from("/tmp/config.json"),
    ];
    run_command3(args)
}

fn install_nix() -> std::io::Result<()> {
    let args = vec![
        String::from("aegis-nix"),
        String::from("config"),
        String::from("/tmp/config.json"),
    ];
    run_command3(args)
}

fn run_command3(args: Vec<String>) -> std::io::Result<()> {
    let child_thread = std::thread::spawn(move || {
        let output = match std::process::Command::new("sudo")
            .args(args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
        {
            Ok(o) => o,
            Err(_) => {
                global_app::emit_global_event("install-fail", "");
                return;
            }
        };

        if output.status.success() {
            println!("fail");
            global_app::emit_global_event("install-success", "");
        } else {
            global_app::emit_global_event("install-fail", "");
        }
    });

    child_thread.join().expect("Failed to join child thread");
    Ok(())
}
