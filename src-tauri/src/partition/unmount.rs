
pub fn unmount(path: String) -> bool {
    println!("{:#?}", path);
    let output = std::process::Command::new("sudo")
        .arg("umount")
        .arg(&path)
        //.stderr(std::process::Stdio::null())
        .output();
    match output {
        Ok(o) => {
            if o.status.success() {
                println!("{:#?}", o.stdout);
                true
            } else {
                println!("{:#?}", o.stdout);
                false
            }
        }
        Err(e) => {
            println!("{:#?}", e);
            false
        }
    }
}
