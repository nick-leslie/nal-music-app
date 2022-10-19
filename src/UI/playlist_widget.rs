//TODO goals display the current playlist, be able to reorder playlist, be able to clear current song and play from playlist

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use iced::{Column, Element, Text};
use iced::keyboard::KeyCode::V;
use iced_native::widget::button::update;
use crate::audio::playlist::Playlist;
use crate::audio::song::SongInfo;
use crate::AudioPlayer;
use crate::event_codes::Message;



//TODO mutextes frezzes the entire UI audio thread still works
//TODO go back to the old method

pub struct PlaylistWidget {
    //TODO figure out how to sync this up with the song_queue in the audio_player
    songs:Rc<RefCell<Playlist>>
    //TODO potentaly store an arc reffrence
}

impl PlaylistWidget {
    pub fn new(play_list: Rc<RefCell<Playlist>>) -> PlaylistWidget {
        PlaylistWidget {
            songs:play_list
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let mut col = Column::new();
        for song in self.songs.as_ref().borrow_mut().vec_ref().iter() {
            let song_txt = Text::new(song.get_song_name().to_string());
            col = col.push(song_txt)
        }
        col.into()
    }
    pub fn add_song(&mut self,s: SongInfo) { ;
        self.playlist_mut().add_song_info(s);
    }
    pub fn match_ap(&mut self,ap:&mut AudioPlayer) {
        println!("should be matching");
        self.songs = ap.get_play_list();
    }
    pub fn playlist_len(&self) -> usize {
        self.songs.as_ref().borrow().get_len()
    }
    pub fn playlist_mut(&mut self) -> RefMut<Playlist> {
        self.songs.as_ref().borrow_mut()
    }
    pub fn playlist(&self) -> Ref<Playlist> {
        self.songs.as_ref().borrow()
    }
}