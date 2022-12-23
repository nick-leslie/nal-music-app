//TODO goals display the current playlist, be able to reorder playlist, be able to clear current song and play from playlist

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use iced::{Button, button, Column, Element, pane_grid, PaneGrid, Row, Text, Scrollable, Renderer, Length};
use iced_native::widget::button::update;
use iced::pane_grid::{Axis, Content, Direction, TitleBar};
use iced_native::widget::scrollable;
use crate::audio::playlist::Playlist;
use crate::audio::song_info::SongInfo;
use crate::AudioPlayer;
use crate::event_codes::Message;
use crate::UI::pane::Pane;
use crate::UI::playlist_item::PlaylistItem;


pub struct PlaylistWidget {
    songs:Rc<RefCell<Playlist>>,
    songs_ui:Vec<PlaylistItem>, //the main issue is that we want to render this but we dont want to reforge the list every time
    playlist_pane: pane_grid::State<PlaylistItem>,
    first_pane:pane_grid::Pane,
    scrollable:scrollable::State
}

impl PlaylistWidget {
    pub fn new(play_list: Rc<RefCell<Playlist>>) -> PlaylistWidget {
        let (mut pane_state,first_pane)  = pane_grid::State::new(PlaylistItem::new(0,play_list.clone()));
        PlaylistWidget {
            songs:play_list,
            songs_ui: vec![],
            playlist_pane: pane_state,
            first_pane,
            scrollable: Default::default()
        }
    }
    //TODO fails to drag bc its a pane within a pane potentaly use other method
    //this is slow in not relece mode but that could becuse rendering is slow in not relece
    //TODO refactor this to not be panes anymore
    pub fn view(&mut self) -> Element<Message> {
        let mut col = Column::new();
        //TODO we are off by one and our size is off
        //TODO this is bad bc we dont want to recreate the vec every render
        let len = self.songs.as_ref().borrow().get_len();
        self.songs_ui = Vec::new( );
        for i in 1..len {
            self.songs_ui.push(PlaylistItem::new(i,self.songs.clone()));

        }
        for song_item in self.songs_ui.iter_mut() {
            col = col.push(song_item.view());
        }
        col.into()
    }
    pub fn add_song(&mut self,s: SongInfo) { ;
        self.playlist_mut().add_song_info(s);
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
    pub fn get_panegrid(&self) -> &pane_grid::State<PlaylistItem> {
        &self.playlist_pane
    }
}


