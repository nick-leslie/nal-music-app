mod audio_player;
mod file_io;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    println!("Hello, world!");
    //player.play_sink();
    let mut player = audio_player::AudioPlayer::new();
    player.add_song_from_path("./demoMusic/afterHours.mp3".to_string());
    player.play_sink();
    UI::main_window::open_window();
    while player.queue_len() > 0 {
        
    }
    //sleep(Duration::from_secs(20));
}

