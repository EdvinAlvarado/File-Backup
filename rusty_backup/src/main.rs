use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;


fn backup<P: AsRef<Path>>(backup_count_limit: usize, backup_path: P, file_path: P) -> Result<String, Box<dyn Error>> {

    // Setup
    let filename = file_path.as_ref().file_stem().ok_or("file path does not point to a file")?
                    .to_str().unwrap();
    let ext = file_path.as_ref().extension().ok_or("file path does not point to a file")?
                .to_str().unwrap();
    
    // Copy file with timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let name_timestamped = format!("{}-{}.{}", filename, now, ext);
    let timestamp_file_path = backup_path.as_ref().join(name_timestamped);
    fs::copy(file_path.as_ref(), timestamp_file_path)?;

    // Delete old files
    let re = Regex::new(format!(r"{}-\d+.{}", filename, ext).as_str()).unwrap();
    let files: Vec<PathBuf> = fs::read_dir(backup_path)?.filter_map(|p| match re.is_match(p.as_ref().unwrap().path().to_str().unwrap()) {
        true => Some(p.unwrap().path()),
        false => None,
    }).collect();

    if files.len() > backup_count_limit {
        for i in 0..(files.len() - backup_count_limit) {
            fs::remove_file(files[i].clone())?;
        }
    }
    let file = format!("{}.{}", filename, ext);
    Ok(file)
}

fn main() {
    let backups = [
        // Example
        backup(10, r"", r""),
        backup(10, r"", r""),
    ];

    for r in backups {
        match r {
            Ok(f) => println!("file backup success:\t{}", f),
            Err(e) => println!("file backup failed:\t{}", e.as_ref()),
        }
    }
}
