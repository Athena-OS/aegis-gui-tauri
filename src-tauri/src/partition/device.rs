use serde::{Serialize, Deserialize};
use crate::partition::{
    actions,
    probeos,
};

#[derive(PartialEq, Deserialize, Serialize, Debug)]
struct Device {
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

}

impl Default for Device {
    fn default() -> Device {
        Device{
        use_percentage: None,
        kname:None,
        size:None,
        used:None,
        possible_actions: None,
        can_install_along: None,
        has_os:None,
        os_details:None ,
        install_candidate: None
        }
    }
}

impl Device {
    pub fn candidate_for_install_along(&self) -> bool {
        match &self.possible_actions{
            Some(action_list) => {
                if action_list.contains(&actions::Action::InstallAlong) {
                    true
                }else{
                    false
                }
            }
            None => false
        }
    }
}

pub fn format_device() -> Result<String, std::io::Error> {
    Ok(String::new())
}