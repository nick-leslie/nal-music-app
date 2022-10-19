use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::io::ErrorKind;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::park_timeout;
use iced::{Background, Color, Column, Container, container, Element, PaneGrid, Text};
use iced::futures::try_join;
use iced::pane_grid::Content;
use iced_native::{Command, Renderer};
use iced_native::user_interface::State;
use iced_native::widget::pane_grid;
use iced_native::widget::pane_grid::{Axis, state};
use crate::audio::song::SongInfo;
use crate::event_codes::Message;
use crate::AudioPlayer;
use crate::UI::controls_widget::ControlPanel;
use crate::UI::file_widget::{directory_graphic, File_Graphic};
use crate::UI::pane::PaneState::controlsPane;
use crate::UI::playlist_widget::PlaylistWidget;
use crate::audio::playlist::Playlist;

enum PaneState {
    controlsPane,
    filesPane,
}
//TODO find a more genaric way to do this that dose not have lifetimes
pub struct potential_content {
    dire_graphic:Option<directory_graphic>,
    playlist_graphic:Option<PlaylistWidget>
}

impl potential_content {
    pub fn view(&mut self) -> Element<Message> {
        let testing_text = Text::new("yeet".to_string()).size(40);
        let mut final_element:Element<Message> = Column::new().push(testing_text.clone()).into();

        if self.dire_graphic.is_some() {
            match self.dire_graphic.as_mut() {
                Some(d) => final_element = d.view(),
                None => {}
            }
        } else if self.playlist_graphic.is_some() {
            match self.playlist_graphic.as_mut() {
                Some(p) => final_element = p.view(),
                None => {}
            };
        }
        final_element
        //owned.unwrap().view()
    }
    pub fn change_dir(&mut self,path:String) -> bool {
        if self.dire_graphic.is_some() {
            self.dire_graphic = Some(directory_graphic::new(path.clone()));
            return true
        }
        false
    }
    pub fn update_playlist(&mut self,ap: &mut AudioPlayer) -> bool {
        if self.playlist_graphic.is_some() {
            let mut playlist = self.playlist_graphic.take().unwrap();
            playlist.match_ap(ap);
            self.playlist_graphic = Some(playlist);
            return true;
        }
        false
    }
}

pub struct Pane {
    pub panes: pane_grid::State<potential_content>,
    panes_created: usize,
}

impl Pane {
    pub fn new(playlist: Rc<RefCell<Playlist>>) -> Pane{
        let (mut pane_state,first_pane)  = pane_grid::State::new(
            potential_content {
                dire_graphic:Some(directory_graphic::new("/home/nickl/Music/bigPlaylist".to_string())),
                playlist_graphic: None
            });
            pane_state.split(Axis::Vertical,&first_pane,potential_content {
                dire_graphic:None,
                playlist_graphic:Some(PlaylistWidget::new(playlist))
            });
        Pane {
            panes:pane_state,
            panes_created: 1
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        //let mut movedContent = Column::new();
        let pane_grid =
            PaneGrid::new(&mut self.panes, |pane, graphics| {
                Content::new(Container::new(graphics.view()))
            })
                .on_drag(Message::PaneDragged)
                .on_resize(10, Message::PaneResized);
        Column::new().push(pane_grid).into()
    }
}
