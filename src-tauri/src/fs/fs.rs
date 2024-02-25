use std::{fs::File, io::Write, path::Path};

#[allow(dead_code)]
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// save the config file in /tmp/conf.file
#[tauri::command]
#[allow(dead_code)]
pub fn save_conf(data: String) {
    let p: &'static str = "/tmp/config.json";
    let path = Path::new(p);
    if path.exists() {
        println!("The file exists.");
        // delete file
        delete_file(p).expect("unable to delete file")
    } else {
        println!("The file exists");
    }
    println!("{}", data);
}
#[allow(dead_code)]
pub fn save_json(data: &str, filename: &str) -> std::io::Result<()> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(filename)?;

    // Write the data to the file
    file.write_all(data.as_bytes())?;

    Ok(())
}
#[allow(dead_code)]
pub fn read_json(_filename: &str) {}
#[allow(dead_code)]
fn delete_file(filename: &str) -> std::io::Result<()> {
    std::fs::remove_file(filename)?;
    Ok(())
}
