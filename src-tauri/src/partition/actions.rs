use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Partition {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Action {
    Partition,
    Replace,
    Format,
    InstallAlong,
    Install,
}

pub static MINIMUM_SIZE: &str = "50G";


