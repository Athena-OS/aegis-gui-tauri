use crate::partition::{self};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Partition {
    pub device: String,
    pub mode: String,
    pub efi: bool,
    pub swap: bool,
    #[serde(skip_serializing_if = "is_default", default)]
    pub swap_size: String,
    pub partitions: Value,
    #[serde(skip_serializing)]
    // This if for processing but should not be serialized for config saving
    pub installAlongPartitions: Vec<partition::device::SuggestedPartition>,
    #[serde(skip_serializing)]
    // This if for processing but should not be serialized for config saving
    pub system_storage_info: Vec<SystemStorageInfo>,
    #[serde(skip_serializing)]
    // This if for processing but should not be serialized for config saving
    pub system_storage_info_current: Vec<SystemStorageInfo>,
    pub encrypt_check: bool,
    #[serde(skip_serializing)]
    pub new_ptable: bool,
    #[serde(skip_serializing)]
    pub new_pt_file_system: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SystemStorageInfo {
    pub partitions: Vec<P>,
}
fn is_default(s: &String) -> bool {
    s.is_empty()
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct P {
    pub name: Option<String>,
    pub partitionName: Option<String>,
    pub start: Option<i128>,
    pub size: Option<i128>,
    pub action: Option<String>,
    pub end: Option<i128>,
    pub fileSytem: Option<String>,
}
impl Default for Partition {
    fn default() -> Partition {
        Partition {
            device: String::new(),
            mode: String::new(),
            efi: false,
            swap: false,
            swap_size: String::new(),
            partitions: json!(null),
            installAlongPartitions: Vec::new(),
            system_storage_info: Vec::new(),
            system_storage_info_current: Vec::new(),
            encrypt_check: false,
            new_ptable: false,
            new_pt_file_system: String::new(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Bootloader {
    #[serde(rename = "type")]
    pub r#type: String,
    pub location: String,
}

impl Default for Bootloader {
    fn default() -> Bootloader {
        Bootloader {
            r#type: String::new(),
            location: String::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Locale {
    pub locale: Vec<String>,
    pub timezone: String,
    pub virtkeymap: String,
    pub x11keymap: String,
}
impl Default for Locale {
    fn default() -> Locale {
        Locale {
            locale: vec![],
            timezone: String::new(),
            virtkeymap: String::new(),
            x11keymap: String::new(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Networking {
    pub hostname: String,
    pub ipv6: bool,
}

impl Default for Networking {
    fn default() -> Networking {
        Networking {
            hostname: String::new(),
            ipv6: false,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Params {
    pub cores: i32,
    pub jobs: i32,
    pub keep: bool,
}

impl Default for Params {
    fn default() -> Params {
        Params {
            cores: 0,
            jobs: 0,
            keep: false,
        }
    }
}
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Config {
    pub partition: Partition,
    pub bootloader: Value,
    pub locale: Value,
    pub networking: Value,
    pub users: Value,
    pub rootpass: Value,
    pub desktop: Value,
    pub theme: Value,
    pub displaymanager: Value,
    pub browser: Value,
    #[serde(skip_serializing)]
    pub packagesStore: Value,
    pub extra_packages: Value,
    pub kernel: Value,
    pub snapper: Value,
    pub zramd: Value,
    pub hardened: Value,
    pub flatpak: Value,
    //#[serde(skip_serializing_if = "serialize_params", default)]
    pub params: Value,
    pub terminal: Value,
    #[serde(skip_serializing)]
    pub base: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            partition: Partition::default(),
            bootloader: json!(null),
            locale: json!(null),
            networking: json!(null),
            users: json!(null),
            rootpass: json!(null),
            desktop: json!(null),
            theme: json!(null),
            displaymanager: json!(null),
            browser: json!(null),
            packagesStore: json!(null),
            extra_packages: json!(null),
            kernel: json!(null),
            snapper: json!(null),
            zramd: json!(null),
            hardened: json!(null),
            flatpak: json!(null),
            params: json!(null),
            terminal: json!(null),
            base: String::from("arch"),
        }
    }
}

impl Config {
    pub fn from_json_string(v: String) -> Config {
        let mut conf = Config::default();
        let _ = partition::utils::unmarshal_json(v.as_str(), &mut conf);
        conf
    }
}
