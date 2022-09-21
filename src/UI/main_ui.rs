use fltk::{app, button, group, prelude::*, window::Window};
use fltk::app::{Receiver, Sender};
use fltk::dialog::BeepType::Message;
use crate::AudioPlayer;
use crate::UI::event_codes::{PAUSE, PLAY};
use fltk_theme::{ColorTheme, color_themes,widget_themes, WidgetTheme, ThemeType};

pub struct musicPlayer {
    app:app::App,
    pub player:AudioPlayer,
    sender:Sender<i32>,
    receiver:Receiver<i32>
}

impl musicPlayer {
    pub fn new() -> musicPlayer {
        let app = app::App::default();
        
        let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
        widget_theme.apply();
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();


        let mut wind = Window::new(100, 100, 400, 300, "My Window");
        let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
        let mut play = button::Button::default().with_label("play");
        let mut pause = button::Button::default().with_label("pause");
        let (sender, receiver) = app::channel::<i32>();

        play.emit(sender, PLAY);
        pause.emit(sender, PAUSE);


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
                    PLAY => self.player.play_sink(),
                    PAUSE => self.player.pause_sink(), // Here we basically do nothing
                    _ => ()
                }
            }
        }
    }
}