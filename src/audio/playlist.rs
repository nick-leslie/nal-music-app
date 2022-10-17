use std::path::Iter;
use crate::audio::song::Song;

pub struct Playlist {
    song_queue: Vec<Song>
}


impl Playlist {
    pub fn new() -> Playlist {
        Playlist {
            song_queue: Vec::new()
        }
    }
    pub fn add_song(&mut self,song:Song) {
        self.song_queue.push(song);
    }
    pub fn get_current_song(&self) -> &Song {
        &self.song_queue[0]
    }
    pub fn get_len(&self) -> usize {
        self.song_queue.len()
    }
    pub fn add_second(&mut self) {
        self.song_queue[0].add_second();
    }
    pub fn remove_first(&mut self) {
        if self.song_queue.len() > 0 {
            self.song_queue.remove(0);
        }
    }
    pub fn vec_ref(&mut self) -> &Vec<Song> {
        &self.song_queue
    }
    pub fn vec_ref_mut(&mut self) -> &mut Vec<Song> {
        &mut self.song_queue
    }
}

