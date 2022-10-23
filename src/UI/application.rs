use std::borrow::BorrowMut;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;
use iced::{Application, Background, Button, button, Color, Column, Command, container, Container, Element, executor, Length, pane_grid, PaneGrid, ProgressBar, Renderer, Row, Subscription, Text, time};
use iced::pane_grid::Content;
use iced::pure::{column, row, scrollable, widget};
use crate::{AudioPlayer, event_codes, file_io};
use crate::audio::song_info::SongInfo;
use crate::event_codes::Message;
use crate::file_io::{get_dir_parent, is_song};
use crate::UI::file_widget::{directory_graphic, File_Graphic};
use iced_lazy::responsive;
use crate::UI::controls_widget::ControlPanel;
use crate::UI::pane::Pane;

pub struct Player {
    pub ap:AudioPlayer,
    sender:SyncSender<Message>,
    receiver:Receiver<Message>,
    current_files:directory_graphic,
    control_panel:ControlPanel,
    panes:Pane
    //pane_state: iced_native::widget::pane_grid::state::State<PaneState>,
}

impl Application for Player {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Player, Command<Self::Message>) {
        let (sender, receiver): (SyncSender<Message>, Receiver<Message>)  = mpsc::sync_channel(100);
        let mut ap = AudioPlayer::new(sender.clone());
        let pane = Pane::new(ap.get_play_list());
        (Player {
            ap,
            sender,
            receiver,
            current_files: directory_graphic::new("/home/nickl/Music/bigPlaylist".to_string()),
            control_panel: ControlPanel::new(),
            panes: pane
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PLAY => {
                self.ap.play_sink();
            },
            Message::PAUSE => {
                self.ap.pause_sink();
            },
            //this handles ticks
            Message::TICK => {
                self.ap.done_check();
                match self.receiver.try_recv() {
                    Ok(m) => {
                        match m {
                            Message::SECOND_ELAPSED => {
                                self.ap.elapsed_second();
                            }
                            _ => {}
                        }
                    }
                    Err(e) => ()

                }
            }
            Message::FILE_INTERACTION(path) => {
                //TODO make it work with relative paths
                if file_io::check_if_dir(path.clone()) == true {
                    //TODO this is bad for preformce I dont want to iterate every time I need the dir
                    for panes_and_content in self.panes.panes.iter_mut() {
                        if panes_and_content.1.change_dir(path.clone()) == true {
                            break;
                        }
                    }
                    self.current_files = directory_graphic::new(path.clone());
                } else {
                    if is_song(path.clone()) {
                        self.ap.add_song_from_path(path.clone());
                        //TODO fix this this is bad desine becuse what if you had 1 in 100 panes
                        // //you can only get away with this cause its 2
                        for panes_and_content in self.panes.panes.iter_mut() {
                            if panes_and_content.1.update_playlist(&mut self.ap) {
                                break;
                            }
                        }
                    }
                }
            }
            Message::CHANGE_VOL(vol) => {
                self.ap.set_vol(vol)
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.panes.resize(&split, ratio);
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped {
                                     pane,
                                     target,
                                 }) => {
                self.panes.panes.swap(&pane, &target);
            }
            Message::PaneDragged(_) => {}
            Message::ReArrange(target,loc) =>  {
                self.ap.playlist_mut().rearrange_playlist(target,loc)
            }
            _ => ()
        }
        Command::none()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        //this genrates a tic every 500 ms
        iced::time::every(Duration::from_millis(500)).map(|_| {
            Message::TICK
        })
    }
    //TODO figure make look good
    //TODO convert time left to a slider
    //TODO find a way to add songs to playlists
    fn view(&mut self) -> Element<Self::Message> {

        // buttons

        let overall_col = Column::new()
            .push(self.control_panel.view(&mut self.ap))
            .push(self.panes.view());
            //.push(file_content)

        Container::new(overall_col).width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(1.0,1.0,1.0))),
            border_width: 2.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            ..Default::default()
        }
    }
}