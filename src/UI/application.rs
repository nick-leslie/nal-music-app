use std::borrow::BorrowMut;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;
use iced::{Application, Button, button, Column, Command, Element, executor, ProgressBar, Row, Subscription, Text, time};
use iced::pure::{column, row, scrollable};
use crate::{AudioPlayer, event_codes, file_io};
use crate::audio::song::Song;
use crate::event_codes::Message;
use crate::UI::file_widget::{directory_graphic, File_Graphic};

pub struct Player {
    pub ap:AudioPlayer,
    sender:SyncSender<Message>,
    receiver:Receiver<Message>,

    // The local state of the two buttons
    play_button: button::State,
    pause_button: button::State,
    current_files:directory_graphic
}



impl Application for Player {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Player, Command<Self::Message>) {
        let (sender, receiver): (SyncSender<Message>, Receiver<Message>)  = mpsc::sync_channel(100);
        let mut ap = AudioPlayer::new(sender.clone());
        ap.add_song_from_path("./demoMusic/afterHours.mp3".to_string());
        //let song =Song::new(").unwrap();
        //ap.add_song_from_path(./demoMusic/afterHours.mp3".to_string());
        (Player {
            ap,
            sender,
            receiver,
            play_button: Default::default(),
            pause_button: Default::default(),
            current_files: directory_graphic::new("./src".to_string())
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
            Message::CHANGE_DIRECTORY(path) => {
                if file_io::check_if_dir(path.clone()) == true {
                    self.current_files = directory_graphic::new(path.clone());
                }
            }
            _ => ()
        }
        Command::none()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        //this genrates a tic every 500 ms
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            Message::TICK
        })
    }
    //TODO figure make look good
    //TODO convert time left to a slider
    //TODO find a way to add songs to playlists
    fn view(&mut self) -> Element<Self::Message> {

        // buttons
        let play_button = Button::new(&mut self.play_button, Text::new("play")).on_press(Message::PLAY);
        let pause_buttion = Button::new(&mut self.pause_button, Text::new("pause")).on_press(Message::PAUSE);
        let current_song_text = Text::new(self.ap.get_current_song()).size(50);
        // duration bar row
        let seconds_played_txt = Text::new(self.ap.current_time().as_secs().to_string()).size(40);
        let duration_bar = ProgressBar::new(0.0..=self.ap.duration_of_song().as_secs() as f32, self.ap.current_time().as_secs() as f32);
        let total_duration_timer =Text::new(self.ap.duration_of_song().as_secs().to_string()).size(40);


        Column::new()
            .push(Row::new()
                .push(play_button)
                .push(pause_buttion)
                .push(current_song_text))
            .push(Row::new().padding([5, 5, 15, 15])
                .push(seconds_played_txt)
                .push(duration_bar)
                .push(total_duration_timer))
            .push(self.current_files.veiw())
            //.push(thing)
            .into()
    }
}