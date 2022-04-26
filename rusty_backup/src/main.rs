extern crate serde;
extern crate quick_xml;

use std::error::Error;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Config {
    #[serde(rename = "backup", default)]
    backups: Vec<Backup>
}
#[derive(Debug, Deserialize)]
struct Backup {
    count: usize,
    backup_path: PathBuf,
    file: PathBuf 
}

impl Config {
    fn new<P: AsRef<Path>>(p: P) -> Result<Config, Box<dyn Error>> {
        let f = fs::File::open(p.as_ref())?;
        let reader = BufReader::new(f);
        let doc: Config = quick_xml::de::from_reader(reader).expect("config file is incorrect");
        Ok(doc)
    }
}

fn backup<P: AsRef<Path>>(backup_count: usize, backup_dir: P, source_file: P) -> Result<String, Box<dyn Error>>{
    
    // Setup
    let filename = source_file.as_ref()
                        .file_stem().ok_or("source_file is not a file type")?
                        .to_str().unwrap();
    let ext = source_file.as_ref()
                        .extension().ok_or("source_file is not a file type")?
                        .to_str().unwrap();

    // Copy file with timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let timestamp_name = format!("{}-{}.{}", filename, now, ext);
    let timestamp_file = backup_dir.as_ref().join(timestamp_name);
    fs::copy(source_file.as_ref(), timestamp_file)?;

    // Delete old files
    let re = Regex::new(format!(r"{}-\d+.{}", filename, ext).as_str()).unwrap();
    let files: Vec<PathBuf> = fs::read_dir(backup_dir)?
                                .filter_map(|path| match re.is_match(path.as_ref().unwrap().path().to_str().unwrap()) {
                                    true => Some(path.unwrap().path()),
                                    false => None,
                                })
                                .collect();

    if files.len() > backup_count {
        for i in 0..(files.len() - backup_count) {
            fs::remove_file(files[i].clone())?;
        }
    }
    let file = format!("{}.{}", filename, ext);
    Ok(file)
}

fn main() {

    let config_file = std::env::args().nth(1).expect("no config file provided");
    let config = Config::new(config_file).expect("error with config xml");
    // println!("{:?}", config);
    let backup_results: Vec<Result<String, Box<dyn Error>>> = config.backups.iter().map(|b| backup(b.count, b.backup_path.as_path(), b.file.as_path())).collect();

    for r in backup_results {
        match r {
            Ok(f) => println!("file backup success:\t{}", f),
            Err(e) => println!("file backup failed:\t{}", e.as_ref()),
        }
    }
}
