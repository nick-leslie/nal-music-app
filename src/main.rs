mod file_io;
mod UI;
mod audio;
mod icedUI;
pub mod event_codes;

use iced::{Application,Settings};
use audio::audio_player::AudioPlayer;
use crate::UI::main_ui::musicPlayer;
use crate::icedUI::application::Player;

fn main()  {
    // let mut mp = musicPlayer::new();
    // mp.player.add_song_from_path("./demoMusic/afterHours.mp3".to_string());
    // mp.main_loop();
    //sleep(Duration::from_secs(20));
    Player::run(Settings::default()).expect("something went wrong");
}
