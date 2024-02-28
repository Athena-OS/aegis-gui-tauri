use crate::partition::{actions, probeos, unmount, utils};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{any::Any, collections::HashMap, io, process::Command, str};

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    // Percentage Usage
    pub use_percentage: Option<String>,
    // Device KNAME
    pub kname: Option<String>,
    // Device Size(In human form eg 20G)
    pub size: Option<String>,
    // Used space. Necessary for install along
    pub used: Option<String>,
    // A list of possible actions
    pub possible_actions: Option<Vec<actions::Action>>,
    // can install along
    pub can_install_along: Option<bool>,
    // has os
    pub has_os: Option<bool>,
    // os details
    pub os_details: Option<probeos::OsProber>,
    // install candindate
    pub install_candidate: Option<bool>,
    // device name
    pub name: Option<String>,
    // Device partitions
    pub partitions: Option<Vec<Partition>>,
    //parttable type
    pub pttype: Option<String>,

    #[serde(rename = "id")]
    pub display_name: Option<String>,
}

impl Default for Device {
    fn default() -> Device {
        Device {
            use_percentage: None,
            kname: None,
            name: None,
            size: None,
            used: None,
            possible_actions: None,
            can_install_along: None,
            has_os: None,
            os_details: None,
            install_candidate: None,
            partitions: None,
            pttype: None,
            display_name: None,
        }
    }
}

impl Device {
    #[allow(dead_code)]
    pub fn candidate_for_install_along(&mut self) -> bool {
        let cfia = match &self.possible_actions {
            Some(action_list) => {
                if action_list.contains(&actions::Action::InstallAlong) {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        self.can_install_along = Some(cfia);
        cfia
    }

    #[allow(dead_code)]
    pub fn populate_partitions(&mut self) {
        let binding = String::new();
        let kname = match &self.kname {
            Some(kname) => kname,
            None => &binding,
        };
        self.partitions = Some(get_partitions(&kname));
    }

    #[allow(dead_code)]
    pub fn populate_possible_actions(&mut self, os_data: &Vec<probeos::OsProber>) {
        let mut possible_actions: Vec<actions::Action> = vec![];
        // Any device can be formatted or partitioned
        possible_actions.push(actions::Action::Partition);
        possible_actions.push(actions::Action::Format);
        //check for space (Candidate for install)
        let disk_size = match &self.size {
            Some(s) => utils::human2bytes(&s).unwrap_or(0.0),
            None => 0.0,
        };
        let min_size = utils::human2bytes(actions::MINIMUM_SIZE).unwrap_or(0.0);
        if disk_size > min_size {
            possible_actions.push(actions::Action::Install);
            self.install_candidate = Some(true)
        }
        // check if disk has an installed os
        if os_data.len() > 0 {
            let binding = String::new();
            let kname = match &self.kname {
                Some(kname) => kname,
                None => &binding,
            };
            let os = os_data.iter().find(|item| {
                <std::option::Option<std::string::String> as Clone>::clone(&item.subpath)
                    .expect("empty subpath")
                    .contains(&*kname)
            });
            self.os_details = os.cloned();
            match os {
                Some(_) => {
                    possible_actions.push(actions::Action::Replace);
                    if disk_size > min_size {
                        possible_actions.push(actions::Action::InstallAlong);
                        self.can_install_along = Some(true);
                    }
                }
                None => {}
            };
        }
        self.possible_actions = Some(possible_actions);
    }
}
#[allow(dead_code)]
pub fn get_device_list(os: &Vec<probeos::OsProber>) -> Vec<Device> {
    let dl = get_disk_info().expect("unable to get device info");
    let mut devices: Vec<Device> = vec![];
    let _ = utils::unmarshal_json(&dl, &mut devices);

    for device in &mut devices {
        let b = String::new();
        let kname = match &device.kname {
            Some(kn) => kn,
            None => &b,
        };
        device.use_percentage = Some(disk_percentage_usage(kname.to_string()));
    }
    for device in &mut devices {
        device.populate_partitions();
        device.populate_possible_actions(os);
        if let Some(partitions) = &mut device.partitions {
            for partition in partitions {
                partition.populate_possible_actions(os)
            }
        }
    }

    devices
}
#[allow(dead_code)]
pub fn probe_devices(os: &Vec<probeos::OsProber>) -> Vec<Device> {
    get_device_list(os)
}

#[allow(dead_code)]
pub enum DeviceType {
    GPT,
    MDOS,
}
#[allow(dead_code)]
pub fn clear_partition_device(
    device: String,
    device_type: DeviceType,
) -> Result<String, std::io::Error> {
    let dt = match device_type {
        DeviceType::GPT => "gpt",
        DeviceType::MDOS => "mdos",
    };
    let output = std::process::Command::new("sudo")
        .arg("parted")
        .arg(device.to_string())
        .arg("--script")
        .arg("--")
        .arg("mklabel")
        .arg(dt)
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::AddrNotAvailable,
                    "Unable to clear partitions",
                ))
            }
        }
        Err(e) => Err(e),
    }
}
#[allow(dead_code)]
pub fn get_disk_info() -> Result<String, std::io::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lsblk -d -O -J | jq '[.blockdevices[] | select(.type==\"disk\")]'")
        .output()?;

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap_or("");
        Ok(stdout.to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to execute command");
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Partition {
    // Used space
    #[serde(rename = "fsused")]
    pub used: Option<String>,
    // Available space
    #[serde(rename = "fsavail")]
    pub available: Option<String>,
    // has os(For install alongside and replace functions)
    pub has_os: Option<bool>,
    // replacable(has os and minimum space)
    pub can_install_along: Option<bool>,
    // details of the installed OS if any
    pub os_details: Option<probeos::OsProber>,
    // disk name
    pub disk_name: Option<String>,
    //partion name
    // #[serde(rename = "kname")]
    pub partition_name: Option<String>,
    // start of the partition
    pub start: Option<u64>,
    // end of the partition
    pub end: Option<u64>,
    // percentage space used
    #[serde(rename = "fsuse%")]
    pub used_percentage: Option<String>,
    // size of the disk
    #[serde(rename = "size")]
    pub size: Option<String>,
    // partition's filesystem
    #[serde(rename = "fstype")]
    pub file_system: Option<String>,
    // partition's mountpoint
    #[serde(rename = "mountpoint")]
    pub mounted_on: Option<String>,
    // partition's flag
    pub partition_flags: Option<String>,

    #[serde(rename = "id")]
    pub display_name: Option<String>,

    pub possible_actions: Option<Vec<actions::Action>>,

    pub install_candidate: Option<bool>,

    pub kname: Option<String>,

    pub suggested_partitions: Option<Vec<SuggestedPartition>>,
}

impl Default for Partition {
    fn default() -> Self {
        Partition {
            used: None,
            available: None,
            has_os: None,
            can_install_along: None,
            os_details: None,
            disk_name: None,
            partition_name: None,
            start: None,
            end: None,
            used_percentage: None,
            size: None,
            file_system: None,
            mounted_on: None,
            partition_flags: None,
            display_name: None,
            possible_actions: None,
            install_candidate: None,
            kname: None,
            suggested_partitions: None,
        }
    }
}
impl Partition {
    pub fn match_self(&self, sp: Vec<SuggestedPartition>) -> bool {
        /*if let Some(s_p) = self.suggested_partitions{
            if
        }else{
            false
        }*/
        false
    }
    #[allow(dead_code)]
    pub fn candidate_for_install_along(&mut self) -> bool {
        let cfia = match &self.possible_actions {
            Some(action_list) => {
                if action_list.contains(&actions::Action::InstallAlong) {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        self.can_install_along = Some(cfia);
        cfia
    }
    #[allow(dead_code)]
    pub fn populate_possible_actions(&mut self, os_data: &Vec<probeos::OsProber>) {
        let mut possible_actions: Vec<actions::Action> = vec![];
        // Any device can be formatted or partitioned
        possible_actions.push(actions::Action::Partition);
        possible_actions.push(actions::Action::Format);
        //check for space (Candidate for install)
        let disk_size = match &self.size {
            Some(s) => utils::human2bytes(&s).unwrap_or(0.0),
            None => 0.0,
        };
        let used_size = match &self.used {
            Some(s) => utils::human2bytes(&s).unwrap_or(0.0),
            None => 0.0,
        };
        let min_size = utils::human2bytes(actions::MINIMUM_SIZE).unwrap_or(0.0);
        if disk_size > min_size {
            possible_actions.push(actions::Action::Install);
            self.install_candidate = Some(true)
        }
        // check if disk has an installed os
        if os_data.len() > 0 {
            let binding = String::new();
            let kname = match &self.kname {
                Some(kname) => kname,
                None => &binding,
            };
            let os = os_data.iter().find(|item| {
                <std::option::Option<std::string::String> as Clone>::clone(&item.subpath)
                    .expect("empty subpath")
                    .contains(&*kname)
            });
            self.os_details = os.cloned();
            match os {
                Some(_) => {
                    possible_actions.push(actions::Action::Replace);
                    if disk_size > min_size {
                        possible_actions.push(actions::Action::InstallAlong);
                        self.can_install_along = Some(true);
                        self.calculate_sizes_for_install_along(disk_size, min_size, used_size);
                    }
                }
                None => {}
            };
        }
        self.possible_actions = Some(possible_actions);
    }

    // Auto suggest partition sizes for install along
    #[allow(dead_code)]
    pub fn calculate_sizes_for_install_along(
        &mut self,
        disk_size: f64,
        min_size: f64,
        used_size: f64,
    ) {
        // calculate the free size after taking into account minimun required and used
        let free_size = disk_size - (min_size + used_size);
        let b = String::new();
        let other_os = SuggestedPartition {
            label: String::from("other_os"),
            minimum_size: used_size,
            maximum_size: used_size + free_size,
            suggested_size: used_size + free_size / 2.0,
            kname: <std::option::Option<std::string::String> as Clone>::clone(&self.kname)
                .unwrap_or(b.clone()),
        };
        let athena = SuggestedPartition {
            label: String::from("athena"),
            minimum_size: min_size,
            maximum_size: min_size + free_size,
            suggested_size: min_size + free_size / 2.0,
            kname: <std::option::Option<std::string::String> as Clone>::clone(&self.kname)
                .unwrap_or(b.clone()),
        };
        self.suggested_partitions = Some(vec![athena, other_os]);
    }
}
fn disk_percentage_usage(kname: String) -> String {
    let cmd = format!("df | grep '{}' | awk '{{print $5}}'", kname);
    let out = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("unable to run command");
    let s = String::from_utf8(out.stdout).expect("unable to get output");
    sum_percentages(&s)
}

fn sum_percentages(input: &str) -> String {
    let result: Result<i32, _> = input
        .lines() // Split the input string into lines
        .map(|line| line.trim_end_matches('%')) // Remove the '%' character from the end of each line
        .map(|number_str| number_str.parse::<i32>()) // Parse each number as i32
        .collect::<Result<Vec<_>, _>>() // Collect results into a Vec, or an Err if any
        .map(|vec| vec.iter().sum()); // Sum up all numbers if no error occurred

    match result {
        Ok(sum) => format!("{}%", sum), // Format the sum as a percentage string
        Err(_) => String::from("Error parsing input"), // Return an error message if parsing failed
    }
}

#[allow(dead_code)]
fn remove_partition(partition: &str) -> Result<(), String> {
    // Ensure the partition string is in the correct format
    let partition = if partition.starts_with("/dev/") {
        partition.to_string()
    } else {
        format!("/dev/{}", partition)
    };

    // Execute the `parted` command to remove the partition
    // Note: This assumes the partition is always on /dev/sda and might need adjustment for other disks
    let output = Command::new("sudo")
        .arg("parted")
        .arg("--script") // Avoids interactive prompts
        .arg(partition.rsplitn(2, '/').last().unwrap_or("")) // Gets the disk device, like `sda` from `/dev/sda1`
        .arg("rm")
        .arg(partition.rsplitn(2, '/').next().unwrap_or("")) // Gets the partition number, like `1` from `sda1`
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_partitions(disk_name: &str) -> Vec<Partition> {
    let output = Command::new("lsblk")
        .arg("-J")
        .arg("-O")
        //.arg("NAME,SIZE,TYPE,MOUNTPOINT,FSTYPE,START,END")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&output_str).unwrap();
    let mut partitions: Vec<Partition> = Vec::new();
    if let Some(devices) = json["blockdevices"].as_array() {
        for device in devices {
            if let (Some(name), Some(children)) =
                (device["name"].as_str(), device["children"].as_array())
            {
                if name.starts_with(disk_name) {
                    for child in children {
                        let p: Partition =
                            serde_json::from_value(child.clone()).expect("unable to get partition");
                        partitions.push(p);
                    }
                }
            }
        }
    }

    partitions
}

// This is the suggested partition table for install along
// if a partition has OS, it should be resized but the its remaining size should be larger than the used space
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct SuggestedPartition {
    pub maximum_size: f64,
    pub minimum_size: f64,
    pub label: String,
    pub suggested_size: f64,
    pub kname: String,
}
impl Default for SuggestedPartition {
    fn default() -> Self {
        SuggestedPartition {
            maximum_size: 0.0,
            minimum_size: 0.0,
            label: String::new(),
            suggested_size: 0.0,
            kname: String::new(),
        }
    }
}
#[allow(dead_code)]
pub fn partition_install_along(
    parts: Vec<SuggestedPartition>,
    device: Device,
) -> Result<bool, std::io::Error> {
    // get partition to shrink
    let def_sp = SuggestedPartition::default();
    let pts = parts
        .iter()
        .find(|&d| d.label == "other_os")
        .unwrap_or(&def_sp);
    let partitions = device.partitions.unwrap_or(vec![]);
    let def_part = Partition::default();
    let part = partitions
        .iter()
        .find(|&d| d.kname == Some(pts.kname.clone()))
        .unwrap_or(&def_part);
    // unmount the partition
    if unmount::unmount(String::from(format!("/dev/{}", pts.kname))) {
        // unmount successful
        let mut resize: HashMap<String, Box<dyn Any>> = HashMap::new();
        resize.insert(String::from("fstype"), Box::new(part.file_system.clone()));
        resize.insert(String::from("size"), Box::new(pts.suggested_size));
        utils::perform_resize(&pts.kname, resize);

        Ok(true)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Unounting",
        ))
    }
}
