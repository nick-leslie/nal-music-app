use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::time::Duration;
use iced::{Application, Button, button, Column, Command, Element, executor, Subscription, Text,time,};
use crate::{AudioPlayer, event_codes};
use crate::event_codes::Message;

pub struct Player {
    pub ap:AudioPlayer,
    sender:SyncSender<Message>,
    receiver:Receiver<Message>,

    // The local state of the two buttons
    play_button: button::State,
    pause_button: button::State,
}


impl Application for Player {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Player, Command<Self::Message>) {
        let (sender, receiver): (SyncSender<Message>, Receiver<Message>)  = mpsc::sync_channel(100);
        let mut ap = AudioPlayer::new(sender.clone());
        ap.add_song_from_path("./demoMusic/afterHours.mp3".to_string());
        (Player {
            ap,
            sender,
            receiver,
            play_button: Default::default(),
            pause_button: Default::default()
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
                self.ap.play_sink();
            },
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
                    Err(e) => println!("{e}")

                }
            }
            _ => ()
        }
        Command::none()
    }
    fn subscription(&self) -> Subscription<Self::Message> {

        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            Message::TICK
        })
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                Button::new(&mut self.play_button, Text::new("play"))
                    .on_press(Message::PLAY),
            )
            .push(
                // We show the value of the counter here
                Text::new(self.ap.current_time().as_secs().to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.pause_button, Text::new("pause"))
                    .on_press(Message::PAUSE),
            ).into()
    }
}