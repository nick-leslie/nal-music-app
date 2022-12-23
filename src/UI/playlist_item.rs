use std::cell::RefCell;
use std::rc::Rc;
use iced::{button, Button, Element, Row, Text};
use iced_native::Command;
use crate::audio::playlist::Playlist;
use crate::event_codes::Message;

//TODO make this a pane and use drag events to re order
pub struct PlaylistItem {
    loc:usize,
    playlist:Rc<RefCell<Playlist>>,
    swap_button:button::State
}

impl PlaylistItem {
    pub fn new(loc:usize,playlist:Rc<RefCell<Playlist>>) -> PlaylistItem {
        PlaylistItem {
            loc,
            playlist,
            swap_button:Default::default()
        }
    }
    //try using panes and pane orderings to order playlist
    pub fn view(&mut self) -> Element<Message> {
        let playlist_barrow = self.playlist.as_ref().borrow();
        let mut txt_str = "no song".parse().unwrap();
        match playlist_barrow.get_song_info(self.loc) {
            Some(info) => {
                txt_str = info.get_song_name();
            }
            _ => {}
        }
        let txt = Text::new(txt_str);
        let swp_button = Button::new(&mut self.swap_button, Text::new("swap loc")).on_press(Message::SWAP_SONGS(self.loc));
        Row::new().push(txt).push(swp_button).into()
    }
    pub fn get_loc(&self) -> usize {
        self.loc
    }
}
