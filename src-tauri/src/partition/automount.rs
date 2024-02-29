use dbus::{
    blocking::{BlockingSender, Connection},
    message::Message,
};
use std::{string::String, time::Duration};

#[allow(dead_code)]
pub struct AutoMountInfo {
    pub has_solid: bool,
    pub was_solid_module_auto_loaded: bool,
}

#[allow(dead_code)]
pub fn kded_call(method: &str, args: Vec<&dyn dbus::arg::RefArg>) -> Message {
    Message::new_method_call("org.kde.kded5", "/kded", "org.kde.kded5", method)
        .unwrap()
        .append_ref(&args)
}

#[allow(dead_code)]
pub fn enable_solid_automount(dbus: &Connection, enable: bool) {
    let method = if enable { "loadModule" } else { "unloadModule" };
    let arg = String::from("device_automounter");
    let args = vec![&arg as &dyn dbus::arg::RefArg];
    let msg = kded_call(method, args);
    let reply = dbus.send_with_reply_and_block(msg, Duration::from_millis(2000));

    match reply {
        Ok(r) => println!("Reply: {:?}", r.get_items()),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
pub fn query_solid_automount(dbus: &Connection) -> AutoMountInfo {
    let arg = String::from("device_automounter");
    let args = vec![&arg as &dyn dbus::arg::RefArg];
    let msg = kded_call("isModuleAutoloaded", args);
    let reply = dbus.send_with_reply_and_block(msg, Duration::from_millis(2000));

    match reply {
        Ok(r) => {
            let result: bool = r.get1().unwrap_or(false);
            AutoMountInfo {
                has_solid: true,
                was_solid_module_auto_loaded: result,
            }
        }
        Err(e) => {
            println!("Error querying Solid: {}", e);
            AutoMountInfo {
                has_solid: false,
                was_solid_module_auto_loaded: false,
            }
        }
    }
}
