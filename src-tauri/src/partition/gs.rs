use crate::partition::{
    device::{probe_devices, Device},
    probeos::{probe_os, OsProber},
    utils,
};
use std::{fmt, str};
use serde::{Deserialize, Serialize};

// This struct stores the storage information details
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GlobalStorage {
    // The operating systems installed
    pub operating_systems: Vec<OsProber>,
    // The disks(devices) in the system
    pub devices: Vec<Device>,
}

impl Default for GlobalStorage {
    fn default() -> GlobalStorage {
        GlobalStorage {
            operating_systems: vec![],
            devices: vec![],
        }
    }
}

impl GlobalStorage {
    #[allow(dead_code)]
    pub fn new() -> GlobalStorage {
        GlobalStorage::default()
    }
    #[allow(dead_code)]
    pub fn probe_os(&mut self) {
        self.operating_systems = probe_os();
    }
    #[allow(dead_code)]
    pub fn probe_devices(&mut self) {
        self.devices = probe_devices(&self.operating_systems);
    }
    #[allow(dead_code)]
    pub fn to_json_string(&self) -> String {
        utils::marshal_json(&self).unwrap()
    }
    #[allow(dead_code)]
    pub fn from_json_string(data: String) -> Result<GlobalStorage, serde_json::Error> {
        let mut gl = GlobalStorage::default();
        let v = utils::unmarshal_json(data.as_str(), &mut gl);
        match v {
            Ok(_) => Ok(gl),
            Err(e) => Err(e),
        }
    }
    #[allow(dead_code)]
    pub fn probe(&mut self) {        
        self.probe_os();
        self.probe_devices();
    }
}

// Implementing Display for GlobalStorage
impl fmt::Display for GlobalStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write your formatted string to `f`
        // This is a simple example; adjust it according to your struct's fields
        let gs = self;
        write!(f, "{}", gs.to_json_string())
    }
}
