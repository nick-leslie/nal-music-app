use std::borrow::{Borrow, BorrowMut};
use std::io::ErrorKind;
use std::ops::Deref;
use std::thread::park_timeout;
use iced::{Background, Color, Column, Container, container, Element, PaneGrid, Text};
use iced::pane_grid::Content;
use iced_native::{Command, Renderer};
use iced_native::user_interface::State;
use iced_native::widget::pane_grid;
use iced_native::widget::pane_grid::{Axis, state};
use crate::event_codes::Message;
use crate::UI::file_widget::{directory_graphic, File_Graphic};

enum PaneState {
    controlsPane,
    filesPane,
}
//TODO find a more genaric way to do this that dose not have lifetimes
pub struct potential_content {
    dire_graphic:Option<directory_graphic>
}
impl potential_content {
    pub fn view(&mut self) -> Element<Message> {
        let testing_text = Text::new("yeet".to_string()).size(40);
        match self.dire_graphic.as_mut() {
            Some(D) => D.view(),
            None => {Column::new().push(testing_text).into()}
        }
        //owned.unwrap().view()

    }
}

pub struct Pane {
    pub panes: pane_grid::State<potential_content>,
    panes_created: usize,
}

impl Pane {
    pub fn new() -> Pane{
        let (mut pane_state,first_pane)  = pane_grid::State::new(
            potential_content {
            dire_graphic:Some(directory_graphic::new("/home/nickl/Music/bigPlaylist".to_string()))
            });
            pane_state.split(Axis::Vertical,&first_pane,potential_content {dire_graphic:None});
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
    fn update(&mut self, message:Message) {
        match message {
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                println!("gameing");
                self.panes.resize(&split, ratio);
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped {
                                 pane,
                                 target,
                             }) => {
                self.panes.swap(&pane, &target);
            }
            Message::PaneDragged(_) => {}
            _ => {}
        }
    }
}
