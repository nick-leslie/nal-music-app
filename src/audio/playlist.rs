use std::path::Iter;
use rodio::{Sample, Source};
use crate::audio::song::SongInfo;
use crate::audio::song_source::SongSource;

pub struct Playlist {
    song_info_queue: Vec<SongInfo>,
    song_queue: Vec<SongSource<f32>>
}


impl Playlist {
    pub fn new() -> Playlist {
        Playlist {
            song_info_queue: Vec::new(),
            song_queue: Vec::new()
        }
    }
    pub fn add_song_info(&mut self, song: SongInfo) {
        self.song_info_queue.push(song);
    }
    pub fn add_song(&mut self, song: SongSource<f32>)
    {
        self.song_queue.push(song);
    }
    pub fn give_next_song(&mut self) -> Option<SongSource<f32>>{
        if self.song_queue.len() > 0 {
            let song = self.song_queue.remove(0);
            return Some(song)
        }
        None
    }
    pub fn get_current_song(&self) -> &SongInfo {
        &self.song_info_queue[0]
    }
    pub fn get_len(&self) -> usize {
        self.song_info_queue.len()
    }
    pub fn add_second(&mut self) {
        self.song_info_queue[0].add_second();
    }
    pub fn remove_first(&mut self) {
        if self.song_info_queue.len() > 0 {
            self.song_info_queue.remove(0);
        }
    }
    pub fn vec_ref(&mut self) -> &Vec<SongInfo> {
        &self.song_info_queue
    }
    pub fn vec_ref_mut(&mut self) -> &mut Vec<SongInfo> {
        &mut self.song_info_queue
    }
}

