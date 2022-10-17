mod file_io;
mod audio;
mod UI;
pub mod event_codes;

use iced::{Application,Settings};
use audio::audio_player::AudioPlayer;
use crate::UI::application::Player;

fn main()  {
    Player::run(Settings::default()).expect("something went wrong");
}

