use std::fs;
use std::fs::{File, FileType, metadata};
use std::io::BufReader;
use std::path::Path;

pub fn load_file(path:String) -> Result<BufReader<File>,String> {
    let attempted_file = File::open(path);
    return match attempted_file {
        Ok(T) => Ok(BufReader::new(T)),
        Err(E) => {
            println!("failed to load file:{}",E);
            Err("failed to load file".parse().unwrap())
        }
    }
}
pub fn check_if_dir(path:String) -> bool {
    let md = metadata(path);
    match md {
        Ok(data) => data.is_dir(),
        Err(E) => false
    }
}
