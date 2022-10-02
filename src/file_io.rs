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
//TODO figure out how to step backwards from a relative path
pub fn get_dir_parent(path:String) -> String{
    let p =Path::new(path.as_str());
    let parent_string = match p.parent() {
        None => { "".parse().unwrap() }
        Some(parent) => { parent.to_str().expect("failed to make string").parse().unwrap() }
    };
    parent_string
}
pub fn is_song(path:String) -> bool {
    let potential_song = Path::new(path.as_str());
    let is_song =
        potential_song.is_file() &&
        potential_song.extension().map(|e| e.to_str().unwrap() == "mp3").unwrap_or(false);

    is_song
}