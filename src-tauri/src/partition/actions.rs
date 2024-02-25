
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Partition {

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Action {
    Partition,
    Replace,
    Format,
    InstallAlong,
}

pub static MinimumSize: &str = "20G";