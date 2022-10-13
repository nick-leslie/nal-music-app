//TODO goals display the current playlist, be able to reorder playlist, be able to clear current song and play from playlist

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use iced::{Column, Element, Text};
use iced::keyboard::KeyCode::V;
use iced_native::widget::button::update;
use crate::audio::song::Song;
use crate::AudioPlayer;
use crate::event_codes::Message;



//TODO mutextes frezzes the entire UI audio thread still works
//TODO go back to the old method

pub struct PlaylistWidget {
    //TODO figure out how to sync this up with the song_queue in the audio_player
    songs:Vec<Song>
    //TODO potentaly store an arc reffrence
}

impl PlaylistWidget {
    pub fn new(play_list: Vec<Song>) -> PlaylistWidget {
        PlaylistWidget {
            songs:play_list
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let mut col = Column::new();
        for song in self.songs.iter() {
            let song_txt = Text::new(song.get_song_name().to_string());
            col = col.push(song_txt)
        }
        col.into()
    }
    pub fn add_song(&mut self,s:Song) { ;
        self.songs.push(s);
    }
    pub fn match_ap(&mut self,ap:&mut AudioPlayer) {
        println!("should be matching");
        self.songs = ap.get_play_list();
    }
}