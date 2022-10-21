use std::path::Iter;
use rodio::{Sample, Source};
use crate::audio::song::SongInfo;
use crate::audio::song_source::SongSource;
//TODO bug on thrid play could be to do with swaping
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
    //TODO for what ever reason this is not the right info
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

    //TODO this is causing issues its probobly with how I am re arranging the playlist
    pub fn rearrange_playlist(&mut self, target_song:usize, mut move_location:usize) {
        //TODO all of this is bad think of a better way to do this
        if move_location > self.song_queue.len() - 1 {
            move_location = self.song_queue.len() - 1;
        }
        let song_src = self.song_queue.remove(target_song);
        self.song_queue.insert(move_location, song_src);

        //TODO I think we need to use the -1 becuse we still need to use the song info through the duration of the song so in throry song info queue could be 1 longer than
        if self.song_info_queue.len() > self.song_queue.len() {
            let song_info = self.song_info_queue.remove(target_song+1);
            self.song_info_queue.insert(move_location+1, song_info)
        } else if self.song_queue.len() == self.song_info_queue.len() {
            let song_info = self.song_info_queue.remove(target_song);
            self.song_info_queue.insert(move_location, song_info)
        }
    }
    pub fn vec_ref_mut(&mut self) -> &mut Vec<SongInfo> {
        &mut self.song_info_queue
    }
}

