use fltk::{app, button, group, prelude::*, window::Window};
use fltk::app::{Receiver, Sender};

use crate::AudioPlayer;
use crate::UI::event_codes::Message;
use fltk_theme::{ColorTheme, color_themes,widget_themes, WidgetTheme, ThemeType};

pub struct musicPlayer {
    app:app::App,
    pub player:AudioPlayer,
    sender:Sender<Message>,
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


        let (sender, receiver) = app::channel::<Message>();

        play.emit(sender, Message::PLAY);
        pause.emit(sender, Message::PAUSE);


        flex.end();
        wind.end();
        wind.show();
        let mut player =  AudioPlayer::new();

        musicPlayer {
            app,
            player,
            sender,
            receiver
        }

    }

    pub fn main_loop(self:&mut Self) {
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    Message::PLAY => self.player.play_sink(),
                    Message::PAUSE => self.player.pause_sink(), // Here we basically do nothing
                    _ => ()
                }
            }
            self.player.on_loop();
        }
    }
}