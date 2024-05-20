use std::path::Path;
pub fn mount(part: String) -> bool {
    // This process should not fail on a live env
    let dir_path = "/mnt/data";
    if !Path::new(dir_path).exists() {
        // Create the directory if it doesn't exist
        let _output = std::process::Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(dir_path)
            .output();
    }
    let output = std::process::Command::new("sudo")
        .arg("mount")
        .arg(part)
        .arg(dir_path)
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_e) => false,
    }
}
