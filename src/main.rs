extern crate filetime;

use std::env;
use std::fs;
use filetime::FileTime;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const THREE_MONTHS: u64 = 7884000;

fn file_age(mtime: u64) -> u64 {
    let now = SystemTime::now();
    let current_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    current_time - mtime
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("Usage is: backup-rs directory");}

    let directory = fs::read_dir(&args[1]).unwrap();

    for file in directory {
        let file = file.unwrap();
        let metadata = match fs::metadata(file.path()) {
            Ok(val) => val,
            Err(_)  => panic!("Panic reading file metadata")   
        };

        let mtime = Duration::from_secs(FileTime::from_last_modification_time(&metadata).seconds()).as_secs();

        if file_age(mtime) > THREE_MONTHS {
            fs::remove_file(file.path());
            println!("File: {} was deleted", file.path().display());
        }

    }

}