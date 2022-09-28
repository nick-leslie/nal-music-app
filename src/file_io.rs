use std::fs;
use std::fs::{File, FileType};
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
// pub fn load_dir_from_string(dir_path: String) {
//     let mut paths = fs::read_dir(dir_path).unwrap();
//
//     // for path in paths {
//     //     if path.as_ref().unwrap().file_type().unwrap().is_dir() {
//     //         println!("{:?}", path.unwrap().file_name())
//     //     }
//     // }
//     let what_r_u = paths.nth(1).unwrap().unwrap().file_name();
//     println!("{:?}",what_r_u);
//
// }
// pub fn load_dir_from_path(dir_path:&Path) {
//     let paths = fs::read_dir(dir_path);
//
//     // for path in paths {
//     //     if path.as_ref().unwrap().file_type().unwrap().is_dir() {
//     //         println!("{:?}", path.unwrap().file_name())
//     //     }
//     // }
// }