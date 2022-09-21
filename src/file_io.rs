use std::fs::File;
use std::io::BufReader;
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