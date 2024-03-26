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
    #[serde(skip_serializing)] // This if for processing but should not be serialized for config saving
    pub installAlongPartitions: Vec<partition::device::SuggestedPartition>,
    #[serde(skip_serializing)] // This if for processing but should not be serialized for config saving
    pub system_storage_info: Vec<SystemStorageInfo>,
    #[serde(skip_serializing)] // This if for processing but should not be serialized for config saving
    pub system_storage_info_current: Vec<SystemStorageInfo>
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct  SystemStorageInfo{
    pub partitions: Vec<P>
}
fn is_default(s: &String) -> bool {
    s.is_empty()
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct P{
    pub name: Option<String>,
    pub partitionName: Option<String>,
    pub start: Option<i128>,
    pub size: Option<i128>,
    pub action: Option<String>,
    pub end: Option<i128>,
    pub fileSytem: Option<String>
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
            system_storage_info:Vec::new(),
            system_storage_info_current:Vec::new()
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
#[allow(unused)]
pub enum PartitionAction {
    Delete,
    Shrink,
    Create,
    None
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
    pub bootloader: Bootloader,
    pub locale: Locale,
    pub networking: Networking,
    pub users: Value,
    pub rootpass: String,
    pub desktop: String,
    pub theme: String,
    pub displayManager: String,
    pub browser: String,
    pub packagesStore: Value,
    pub extra_packages: Vec<String>,
    pub kernel: String,
    pub snapper: bool,
    pub zramd: bool,
    pub hardened: bool,
    pub flatpak: bool,
    pub params: Params,
    pub terminal: String,
    pub base: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            partition: Partition::default(),
            bootloader: Bootloader::default(),
            locale: Locale::default(),
            networking: Networking::default(),
            users: json!(null),
            rootpass: String::new(),
            desktop: String::new(),
            theme: String::new(),
            displayManager: String::new(),
            browser: String::new(),
            packagesStore: json!(null),
            extra_packages: vec![],
            kernel: String::new(),
            snapper: false,
            zramd: false,
            hardened: false,
            flatpak: false,
            params: Params::default(),
            terminal: String::from("default"),
            base: String::from("arch")
        }
    }
}

impl Config {
    #[allow(dead_code)]
    pub fn from_json_string(v: String) -> Config {
        let mut conf = Config::default();
        let r = partition::utils::unmarshal_json(v.as_str(), &mut conf);
        match r {
            Ok(_) => log::info!("Deserialized config from the frontend"),
            Err(e) => log::error!("error deserializing config from the frontend {:#?}", e),
        };
        conf
    }
}
