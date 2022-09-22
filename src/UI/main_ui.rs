use std::thread::Thread;
use fltk::{app, browser, button, group, prelude::*, window::Window};
use crate::AudioPlayer;
use crate::UI::event_codes::Message;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use fltk::valuator::{HorNiceSlider, ValueSlider};
use fltk_theme::{ColorTheme, color_themes,widget_themes, WidgetTheme, ThemeType};

pub struct musicPlayer {
    app:app::App,
    pub player:AudioPlayer,
    sender:SyncSender<Message>,
    receiver:Receiver<Message>,
    elements:windowElements
}
pub struct windowElements {
    slider:ValueSlider
}

impl musicPlayer {
    pub fn new() -> musicPlayer {
        let app = app::App::default();
        let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
        widget_theme.apply();
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();
        let mut wind = Window::new(1000, 1000, 1000, 1000, "My Window");
        wind.make_resizable(true);



        let flex = group::Flex::default().with_size(100, 100).row();
        let mut play = button::Button::default().with_label("play");
        let mut pause = button::Button::default().with_label("pause");

        let mut time_left = ValueSlider::default().with_size(100,100);
        time_left.set_minimum(0.);
        time_left.set_maximum(100.);
        flex.end();
        /* file browser stuff
        let mut b = browser::FileBrowser::new(10, 10, 900 - 20, 300 - 20, "");
        let widths = &[50, 50, 50, 70, 70, 40, 40, 70, 70, 50];
        b.set_column_widths(widths);
        b.set_column_char('\t');
        */
         */
        let (sender, receiver): (SyncSender<Message>, Receiver<Message>)  = mpsc::sync_channel(100);
        //messaging creator
        let play_sender = sender.clone();
        let pause_sender = sender.clone();
        play.set_callback(move |_| {
            play_sender.send(Message::PLAY).unwrap();
        });
        pause.set_callback(move |_| {
            pause_sender.send(Message::PAUSE).unwrap();
        });

        wind.end();
        wind.show();
        let player_sender = sender.clone();
        let mut player =  AudioPlayer::new(player_sender);
        let elements = windowElements {
            slider:time_left
        };
        musicPlayer {
            app,
            player,
            sender,
            receiver,
            elements
        }

    }

    pub fn main_loop(self:&mut Self) {
        while self.app.wait() {
            self.app.redraw();
            match self.receiver.try_recv() {
                Ok(msg) => {
                    match msg {
                        Message::PLAY => {
                            if self.player.is_paused() {
                                self.player.play_sink();
                                self.elements.slider.set_maximum(self.player.duration_of_song().as_secs() as f64);
                            }
                        },
                        Message::PAUSE => self.player.pause_sink(), // Here we basically do nothing
                        Message::SECOND_ELAPSED=> {
                            self.player.elapsed_second();
                            self.elements.slider.set_value(self.player.current_time().as_secs() as f64)
                        },
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
}