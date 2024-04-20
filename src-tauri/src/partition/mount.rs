use std::path::Path;
pub fn mount(part: String) -> bool {
    // This process should not fail on a live env
    let dir_path = "/mnt/data";
    if !Path::new(dir_path).exists() {
        // Create the directory if it doesn't exist
         let output = std::process::Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(dir_path)
            .output();

            match output{
                Ok(o) => {
                    if o.status.success() {
                        log::info!("created {:#?}", o.stdout);
                       // true
                    } else {
                        log::error!("not created {:#?}", o.stdout);
                        //false
                    }
                }
                Err(e) => {
                    log::error!(" err {:#?}", e);
                    //false
                }
            
            }
    }
    let output = std::process::Command::new("sudo")
    .arg("mount")
    .arg(part)
    .arg(dir_path)
    .output();

    match output{
        Ok(o) => {
            if o.status.success() {
                log::info!("mounted {:#?}", o.stdout);
               true
            } else {
                log::error!("not mounted {:#?}", o.stdout);
                false
            }
        }
        Err(e) => {
            log::error!(" err mounting {:#?}", e);
            false
        }
    
    }
}