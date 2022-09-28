mod file_io;
mod audio;
mod UI;
pub mod event_codes;

use iced::{Application,Settings};
use audio::audio_player::AudioPlayer;
use crate::UI::application::Player;

fn main()  {
    //file_io::load_dir_from_string("./demoMusic".parse().unwrap());
    //file_io::load_dir_from_path()
    Player::run(Settings::default()).expect("something went wrong");
}
