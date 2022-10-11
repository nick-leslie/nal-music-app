use std::borrow::BorrowMut;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;
use iced::{Application, Background, Button, button, Color, Column, Command, container, Container, Element, executor, Length, pane_grid, PaneGrid, ProgressBar, Renderer, Row, Subscription, Text, time};
use iced::pane_grid::Content;
use iced::pure::{column, row, scrollable, widget};
use crate::{AudioPlayer, event_codes, file_io};
use crate::audio::song::Song;
use crate::event_codes::Message;
use crate::file_io::{get_dir_parent, is_song};
use crate::UI::file_widget::{directory_graphic, File_Graphic};
use iced_lazy::responsive;
use crate::UI::pane::Pane;

pub struct Player {
    pub ap:AudioPlayer,
    sender:SyncSender<Message>,
    receiver:Receiver<Message>,

    // The local state of the two buttons
    play_button: button::State,
    pause_button: button::State,
    step_back_button:button::State,
    current_files:directory_graphic,
    panes:Pane
    //pane_state: iced_native::widget::pane_grid::state::State<PaneState>,
}

enum PaneState {
    SomePane,
    AnotherKindOfPane,
}

impl Application for Player {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Player, Command<Self::Message>) {
        let (sender, receiver): (SyncSender<Message>, Receiver<Message>)  = mpsc::sync_channel(100);
        let mut ap = AudioPlayer::new(sender.clone());
        let (mut pane_state,_)  = pane_grid::State::new(PaneState::SomePane);
        (Player {
            ap,
            sender,
            receiver,
            play_button: Default::default(),
            pause_button: Default::default(),
            step_back_button: Default::default(),
            current_files: directory_graphic::new("/home/nickl/Music/bigPlaylist".to_string()),
            panes: Pane::new()
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
                    self.current_files = directory_graphic::new(path.clone());
                } else {
                    if is_song(path.clone()) {
                        self.ap.add_song_from_path(path.clone())
                    }
                }
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
        let play_button = Button::new(&mut self.play_button, Text::new("play")).on_press(Message::PLAY);
        let pause_button = Button::new(&mut self.pause_button, Text::new("pause")).on_press(Message::PAUSE);
        let current_song_text = Text::new(self.ap.get_current_song()).size(50);
        // duration bar row
        let seconds_played_txt = Text::new(self.ap.current_time().as_secs().to_string()).size(40);
        let duration_bar = ProgressBar::new(0.0..=self.ap.duration_of_song().as_secs() as f32, self.ap.current_time().as_secs() as f32);
        let total_duration_timer =Text::new(self.ap.duration_of_song().as_secs().to_string()).size(40);
        let step_back_button = Button::new(&mut self.step_back_button,Text::new("step back")).on_press(Message::FILE_INTERACTION(get_dir_parent(self.current_files.get_current_path())));

        let file_content =  Element::from(Column::new().push(self.current_files.view()));




        let overall_col = Column::new()
            .push(Row::new()
                .push(play_button)
                .push(pause_button)
                .push(current_song_text))
            .push(Row::new().padding([5, 5, 15, 15])
                .push(seconds_played_txt)
                .push(duration_bar)
                .push(total_duration_timer)).push(step_back_button)
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