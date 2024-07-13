use crate::partition::{
    device::{probe_devices, Device, Partition},
    probeos::{run_probe_os_with_timeout, OsProber},
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
    pub fn new() -> GlobalStorage {
        GlobalStorage::default()
    }
    

    pub fn probe_os(&mut self) {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let result = runtime.block_on(run_probe_os_with_timeout());
        self.operating_systems = result;
    }
    
    
    pub fn probe_devices(&mut self) {
        self.devices = probe_devices(&self.operating_systems);
    }
    
    
    pub fn to_json_string(&self) -> String {
        utils::marshal_json(&self).unwrap_or(String::new())
    }
    
    
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
                os.can_install_along = Some(self.partitions[index].candidate_for_install_along());
            }
        }
    }
}

impl fmt::Display for GlobalStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gs = self;
        write!(f, "{}", gs.to_json_string())
    }
}
