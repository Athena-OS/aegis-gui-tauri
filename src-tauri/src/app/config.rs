use crate::partition;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Partition {
    pub device: String,
    pub mode: String,
    pub efi: bool,
    pub swap: bool,
    pub swap_size: String,
    pub partitions: Value,
    pub installAlongPartitions: Vec<partition::device::SuggestedPartition>,
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
        println!("{}", v);
        let r = partition::utils::unmarshal_json(v.as_str(), &mut conf);
        match r {
            Ok(_) => println!("good"),
            Err(e) => println!("{:#?}", e),
        };
        conf
    }
}
