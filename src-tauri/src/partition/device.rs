use crate::partition::{actions, probeos, utils};
use serde::{Deserialize, Serialize};
use std::{io, process::Command, str};
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    // Percentage Usage
    use_percentage: Option<String>,
    // Device KNAME
    kname: Option<String>,
    // Device Size(In human form eg 20G)
    size: Option<String>,
    // Used space. Necessary for install along
    used: Option<String>,
    // A list of possible actions
    possible_actions: Option<Vec<actions::Action>>,
    // can install along
    can_install_along: Option<bool>,
    // has os
    has_os: Option<bool>,
    // os details
    os_details: Option<probeos::OsProber>,
    // install candindate
    install_candidate: Option<bool>,
    // device name
    name: Option<String>,
    // Device partitions
    partitions: Option<Vec<Partition>>,
    //parttable type
    pttype: Option<String>,
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
        }
    }
}

impl Device {
    #[allow(dead_code)]
    pub fn candidate_for_install_along(&self) -> bool {
        match &self.possible_actions {
            Some(action_list) => {
                if action_list.contains(&actions::Action::InstallAlong) {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}
#[allow(dead_code)]
pub fn get_device_list() -> Vec<Device> {
    let dl = get_disk_info().expect("unable to get device info");
    let mut devices: Vec<Device> = vec![];
    let _ = utils::unmarshal_json(&dl, &mut devices);

    for device in &mut devices {
        let kname = match &device.kname {
            Some(kname) => kname,
            None => "",
        };
        device.use_percentage = Some(disk_percentage_usage(kname.to_string()));
    }

    devices
}
#[allow(dead_code)]
pub fn probe_devices() -> Vec<Device> {
    get_device_list()
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
        .arg("lsblk -d -o NAME,TYPE,SIZE,VENDOR,MODEL,SERIAL,TRAN,KNAME,PTTYPE -J | jq '[.blockdevices[] | select(.type==\"disk\")]'")
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
    mounted_on: Option<String>,
    // Used space
    used: Option<i128>,
    // Available space
    available: Option<i128>,
    // percentage used
    use_percentage: Option<String>,
    // has os(For install alongside and replace functions)
    has_os: Option<bool>,
    // replacable(has os and minimum space)
    can_install_along: Option<bool>,
    os_details: Option<probeos::OsProber>,
}

fn disk_percentage_usage(kname: String) -> String {
    let cmd = format!("df | grep '{}' | awk '{{print $5}}'", kname);
    let out = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        //.arg("grep")
        //.arg(&kname)
        .output()
        .expect("unable to run command");
    println!("{:?}", out);
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
