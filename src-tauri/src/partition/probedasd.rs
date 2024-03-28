/*use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use udev::Enumerator;

#[derive(Debug)]
#[allow(dead_code)]
struct DasdInfo {
    blocksize: Option<u32>,
    cylinders: Option<u32>,
    device_id: String,
    disk_layout: Option<String>,
    name: String,
    tracks_per_cylinder: Option<u32>,
    dasd_type: Option<String>,
}
#[allow(dead_code)]
fn probe() -> Result<HashMap<String, DasdInfo>, Box<dyn std::error::Error>> {
    let mut dasds = HashMap::new();
    let mut context = Enumerator::new()?;
    for device in context.scan_devices()? {
        if device.property_value("DEVTYPE") == Some(std::ffi::OsStr::new("disk")) {
            if let Some(devname) = device.devnode() {
                if let Some(dasd_info) = get_dasd_info(&device) {
                    dasds.insert(devname.to_string_lossy().into_owned(), dasd_info);
                }
            }
        }
    }
    Ok(dasds)
}
#[allow(dead_code)]
fn get_dasd_info(device: &udev::Device) -> Option<DasdInfo> {
    let name = device.devnode()?.to_str()?.to_owned();
    let device_id = device
        .property_value("ID_PATH")
        .unwrap_or_default()
        .to_string_lossy()
        .replace("ccw-", "");
    let dasdview_output = dasdview(&name)?;
    let disk_layout = disk_format(&dasdview_output);
    let blocksize = find_val_int("blocksize", &dasdview_output);
    let dasd_type = find_val("type", &dasdview_output);
    let cylinders = find_val_int("number of cylinders", &dasdview_output);
    let tracks_per_cylinder = find_val_int("tracks per cylinder", &dasdview_output);

    Some(DasdInfo {
        name,
        device_id,
        disk_layout,
        blocksize,
        dasd_type,
        cylinders,
        tracks_per_cylinder,
    })
}
#[allow(dead_code)]
fn dasdview(devname: &str) -> Option<String> {
    let output = Command::new("dasdview")
        .arg("--extended")
        .arg(devname)
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}
#[allow(dead_code)]
fn find_val(pattern: &str, content: &str) -> Option<String> {
    let regex = Regex::new(&format!(r"{}:\s+hex\s+\w+\s+dec\s+(?P<value>\d+)", pattern)).ok()?;
    regex
        .captures(content)
        .and_then(|cap| cap.name("value").map(|value| value.as_str().to_string()))
}
#[allow(dead_code)]
fn find_val_int(pattern: &str, content: &str) -> Option<u32> {
    find_val(pattern, content)?.parse().ok()
}
#[allow(dead_code)]
fn disk_format(content: &str) -> Option<String> {
    let regex = Regex::new(r"^format\s+:\s+.+\s+(?P<value>\w+\s\w+)$").ok()?;
    regex.captures(content).and_then(|cap| {
        cap.name("value")
            .map(|value| match value.as_str().to_lowercase().as_str() {
                "cdl formatted" => Some("cdl".to_string()),
                "ldl formatted" => Some("ldl".to_string()),
                "not formatted" => Some("not-formatted".to_string()),
                _ => None,
            })
            .flatten()
    })
}
*/