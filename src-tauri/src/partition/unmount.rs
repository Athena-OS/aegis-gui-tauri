use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
//use std::io::{self, Read};
use std::io::BufRead;
#[allow(dead_code)]
pub fn do_umount(mountpoint: &Path, recursive: bool, private: bool) -> std::io::Result<bool> {
    let mp = fs::canonicalize(mountpoint)?;
    let mut ret = false;

    // Load mount points from /proc/mounts
    let mountpoints: Vec<PathBuf> = fs::read_to_string("/proc/mounts")?
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .map(|mp| PathBuf::from(mp))
        .collect();

    if private {
        for curmp in &mountpoints {
            if curmp == &mp || curmp.starts_with(&mp) {
                let _ = Command::new("mount")
                    .args(&["--make-private", curmp.to_str().unwrap()])
                    .output()?;
            }
        }
    }

    for curmp in mountpoints.iter().rev() {
        if curmp == &mp || (recursive && curmp.starts_with(&mp)) {
            let _ = Command::new("umount")
                .arg(curmp.to_str().unwrap())
                .output()?;
            if curmp == &mp {
                ret = true;
            }
        }
    }

    Ok(ret)
}
#[allow(dead_code)]
pub fn is_mounted(
    target: &Path,
    _src: Option<&Path>,
    _opts: Option<&str>,
) -> std::io::Result<bool> {
    let mounts_file = std::fs::File::open("/proc/mounts")?;
    let reader = std::io::BufReader::new(mounts_file);

    let target = target.canonicalize()?;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 1 {
            // The mount point is the second field in the /proc/mounts line
            let mount_point = PathBuf::from(parts[1]);

            // Check if the current line's mount point matches the target
            if mount_point == target {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
