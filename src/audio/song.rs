use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::time::Duration;
use rodio::Decoder;
use crate::file_io;
use crate::file_io::load_file;
#[derive(Clone)]
pub struct Song {
    song_name:String,
    song_path:String,
    total_duration:Duration,
    current_duration:Duration
}


impl Song {
   pub fn new(path:String) -> Result<Song,String> {
       let path_clone = path.clone();
       let p = Path::new(path_clone.as_str());
       let total_duration = mp3_duration::from_path(path.clone()).unwrap();
       //TODO this os string stuff looks jank so look into other solutions or if it matters
       Ok(Song {
           song_name: p.file_name().unwrap().to_os_string().to_str().unwrap().to_string(),
           song_path:path.to_string(),
           total_duration,
           current_duration: Duration::from_secs(0)
       })
   }
    pub fn get_song_path(&self) -> String {
        self.song_path.clone()
    }
    pub fn get_song_name(&self) -> String {
        self.song_name.clone()
    }
    pub fn get_total_duration(&self) -> Duration{
        self.total_duration
    }
    pub fn get_current_duration(&self) -> Duration{
         self.current_duration
    }
    pub fn add_second(&mut self) {
        self.current_duration+=Duration::from_secs(1);
    }
}