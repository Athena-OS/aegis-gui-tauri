
use std::{
    process::Command,
    path::Path,
    error::Error,
    fs::File,
    io::{ Write}
};
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// save the config file in /tmp/conf.file
#[tauri::command]
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

pub fn save_json(data: &str, filename: &str) -> std::io::Result<(),> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(filename)?;

    // Write the data to the file
    file.write_all(data.as_bytes())?;

    Ok(())
}

pub fn read_json(filename: &str){}

fn delete_file(filename: &str) -> std::io::Result<()> {
    std::fs::remove_file(filename)?;
    Ok(())
}