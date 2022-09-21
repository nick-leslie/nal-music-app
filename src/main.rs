mod audio_player;
mod file_io;
mod UI;
use crate::audio_player::AudioPlayer;
use crate::UI::main_ui::musicPlayer;

fn main() {
    let mut mp = musicPlayer::new();
    mp.player.add_song_from_path("./demoMusic/afterHours.mp3".to_string());
    mp.main_loop();
    //sleep(Duration::from_secs(20));
}

