use lazy_static::lazy_static;
use std::fmt;
use std::{
    any::Any,
    collections::HashMap
};


// Size in bytes
pub fn resize_ext(path: &str, size: i64) -> Result<bool, std::io::Error> {
    let output: Result<std::process::Output, std::io::Error> = std::process::Command::new("sudo")
        .arg("resize2fs")
        .arg(path)
        .arg(format!("{}k", size))
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => todo!(),
    }
}

// Size in bytes
pub fn resize_ntfs(path: &str, size: i64) -> Result<bool, std::io::Error> {
    let output: Result<std::process::Output, std::io::Error> = std::process::Command::new("sudo")
        .arg("ntfsresize")
        .arg("-f")
        .arg("-s")
        .arg(format!("{}k", size))
        .arg(path)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => todo!(),
    }
}
type ResizeFn = fn(&str, i64) -> Result<bool, std::io::Error>;
lazy_static! {
    pub static ref RESIZERS: HashMap<&'static str, ResizeFn> = {
        let mut m = HashMap::new();
        m.insert("ext2", resize_ext as ResizeFn);
        m.insert("ext3", resize_ext as ResizeFn);
        m.insert("ext4", resize_ext as ResizeFn);
        m.insert("ntfs", resize_ntfs as ResizeFn);
        m
    };
}



pub fn perform_resize(kname: &str, resize: HashMap<String, Box<dyn Any>>) {
    let path = format!("/dev/{}", kname);
    let binding = Some("ext4".to_string());
    let fstype = match resize.get("fstype") {
        Some(value) => match value.downcast_ref::<Option<String>>() {
            Some(option_string) => option_string.clone().unwrap_or_else(|| "ext4".to_string()),
            None => "ext4".to_string(),
        },
        None => binding.clone().unwrap_or_else(|| "ext4".to_string()),
    };
    let binding2: f64 = 100000000000000000000000000000000000000000.0;
    let size: f64 = match resize.get("size") {
        Some(value) => *value.downcast_ref::<f64>().unwrap_or(&binding2),
        None => binding2,
    };
    let direction = resize.get("direction");
    println!(
        "Resizing {} of type {:#?} {:#?} to {:#?}",
        path, fstype, direction, size
    );

    if let Some(&resize_function) = RESIZERS.get(fstype.as_str()) {
        match resize_function(path.as_str(), size.round() as i64) {
            Ok(_) => {
               
            }
            Err(_e) => {
                
            }
        }
    } else {
        eprintln!("No resize function found for fstype");
    }
}


pub fn unmarshal_json<T: for<'de> serde::Deserialize<'de>>(
    data: &str,
    v: &mut T,
) -> Result<(), serde_json::Error> {
    *v = serde_json::from_str(data)?;
    Ok(())
}


pub fn marshal_json<T: serde::Serialize>(v: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(v)
}

#[derive(Debug)]
pub enum SizeParseError {
    InvalidInput(String),
    NegativeSize(String),
}

impl fmt::Display for SizeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SizeParseError::InvalidInput(ref cause) => write!(f, "Invalid input: {}", cause),
            SizeParseError::NegativeSize(ref cause) => {
                write!(f, "Size cannot be negative: {}", cause)
            }
           
        }
    }
}


pub fn human2bytes(size: &str) -> Result<f64, SizeParseError> {
    let mut size = size.trim().to_uppercase();
    let mut multiplier = 1f64;

    let units = [
        ("B", 1f64),
        ("K", 1024f64),
        ("M", 1048576f64),
        ("G", 1073741824f64),
        ("T", 1099511627776f64),
    ];

    for (unit, mult) in &units {
        if size.ends_with(unit) {
            size = size.trim_end_matches(unit).to_string();
            multiplier = *mult;
            break;
        }
    }

    let number: f64 = size
        .parse()
        .map_err(|_| SizeParseError::InvalidInput(size.clone()))?;
    if number < 0.0 {
        return Err(SizeParseError::NegativeSize(size));
    }

    Ok(number * multiplier)
}


pub fn bytes2human(size: f64) -> Result<String, SizeParseError> {
    if size < 0.0 {
        return Err(SizeParseError::NegativeSize(size.to_string()));
    }

    let units = [
        ("B", 1f64),
        ("K", 1024f64),
        ("M", 1048576f64),
        ("G", 1073741824f64),
        ("T", 1099511627776f64),
    ];
    let mut unit = "B";
    let mut divisor = 1f64;

    for &(u, div) in units.iter().rev() {
        if size >= div {
            unit = u;
            divisor = div;
            break;
        }
    }

    Ok(format!("{:.0}{}", size / divisor, unit))
}

pub fn get_disk_id(partition_id: &str) -> String {
    partition_id
        .chars()
        .rev()
        .skip_while(|c| c.is_digit(10)) // Skip digits from the end
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}
