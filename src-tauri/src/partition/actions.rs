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
#[allow(dead_code)]
pub static MINIMUM_SIZE: &str = "20G";

/*impl std::marker::Copy for std::option::Option<Vec<Action>> {

}*/
