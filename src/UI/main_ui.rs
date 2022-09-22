use std::thread::Thread;
use fltk::{app, button, group, prelude::*, window::Window};
use crate::AudioPlayer;
use crate::UI::event_codes::Message;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use fltk_theme::{ColorTheme, color_themes,widget_themes, WidgetTheme, ThemeType};

pub struct musicPlayer {
    app:app::App,
    pub player:AudioPlayer,
    sender:SyncSender<Message>,
    receiver:Receiver<Message>
}

impl musicPlayer {
    pub fn new() -> musicPlayer {
        let app = app::App::default();
        let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
        widget_theme.apply();
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();
        let mut wind = Window::new(100, 100, 400, 300, "My Window");
        wind.make_resizable(true);
        let flex = group::Flex::default().with_size(100, 100).row();
        let mut play = button::Button::default().with_label("play");
        let mut pause = button::Button::default().with_label("pause");


        let (sender, receiver): (SyncSender<Message>, Receiver<Message>)  = mpsc::sync_channel(100);

        let play_sender = sender.clone();
        let pause_sender = sender.clone();
        play.set_callback(move |_| {
            play_sender.send(Message::PLAY).unwrap();
        });
        pause.set_callback(move |_| {
            pause_sender.send(Message::PAUSE).unwrap();
        });

        flex.end();
        wind.end();
        wind.show();
        let player_sender = sender.clone();
        let mut player =  AudioPlayer::new(player_sender);

        musicPlayer {
            app,
            player,
            sender,
            receiver
        }

    }

    pub fn main_loop(self:&mut Self) {
        while self.app.wait() {
            self.app.redraw();
            match self.receiver.try_recv() {
                Ok(msg) => {
                    match msg {
                        Message::PLAY => self.player.play_sink(),
                        Message::PAUSE => self.player.pause_sink(), // Here we basically do nothing
                        Message::SECOND_ELAPSED=>self.player.elapsed_second(),
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
}