use crate::partition::{
    device::{probe_devices, Device, Partition},
    probeos::{probe_os, OsProber},
    utils,
};
use serde::{Deserialize, Serialize};
use std::{fmt, str};

// This struct stores the storage information details
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GlobalStorage {
    // The operating systems installed
    pub operating_systems: Vec<OsProber>,
    // The disks(devices) in the system
    pub devices: Vec<Device>,
    // all partitions for all the devices
    pub partitions: Vec<Partition>,
}

impl Default for GlobalStorage {
    fn default() -> GlobalStorage {
        GlobalStorage {
            operating_systems: vec![],
            devices: vec![],
            partitions: vec![],
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
        for device in &self.devices {
            if let Some(ps) = &device.partitions {
                for p in ps {
                    self.partitions.push(p.clone());
                }
            }
        }
        for os in &mut self.operating_systems {
            if let Some(index) = self.partitions.iter().position(|x| {
                if let (Some(kname), Some(sp)) = (&x.kname, &os.subpath) {
                    format!("/dev/{}", kname) == *sp
                } else {
                    false
                }
            }) {
                // Now you can borrow `self.partitions[index]` as mutable
                os.can_install_along = Some(self.partitions[index].candidate_for_install_along());
            }
        }
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
