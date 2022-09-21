mod audio_player;
mod file_io;

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::BufReader;
use std::mem;
use std::thread::sleep;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source};
use fltk::{app, button, group, prelude::*, window::Window};
use fltk::dialog::BeepType::Message;
use crate::audio_player::AudioPlayer;

fn main() {
    let mut player =  audio_player::AudioPlayer::new();
    player.add_song_from_path("./demoMusic/afterHours.mp3".to_string());

    let a = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "My Window");
    let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
    let mut play = button::Button::default().with_label("play");
    let mut pause = button::Button::default().with_label("pause");
    let (s, r) = app::channel::<i32>();

    play.emit(s, 1);
    pause.emit(s, 0);

    // This is equivalent to calling but.set_callback(move |_| s.send(true));

    flex.end();
    wind.end();
    wind.show();
    println!("Hello, world!");
    //player.play_sink();

    //player.play_sink();
    // while player.queue_len() > 0 {
    //
    // }


    //a.run().unwrap();
    while a.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                1 => player.play_sink(),
                0 => player.pause_sink(), // Here we basically do nothing
                _ => ()
            }
        }
    }
    //sleep(Duration::from_secs(20));
}

