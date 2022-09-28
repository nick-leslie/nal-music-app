use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::fs;
use std::fs::File;
use std::io::{BufReader, sink};
use std::iter::successors;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use crate::file_io;
use mp3_duration;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use iced::window::icon::Error::DimensionsMismatch;
use crate::audio::song::Song;
use crate::event_codes::Message;

pub struct AudioPlayer {
    sink:rodio::Sink,
    stream:OutputStream,
    stream_handle:OutputStreamHandle,
    song_queue:Vec<Song>,
    sender:SyncSender<Message>,
}

impl AudioPlayer {
    pub fn new(sender:SyncSender<Message>) -> AudioPlayer {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        AudioPlayer {
            sink:rodio::Sink::try_new(&stream_handle).unwrap(),
            stream:_stream,
            stream_handle,
            song_queue:Vec::new(),
            sender,
        }
    }
    pub fn play_sink(self:&Self) {
        println!("number of tracks:{}",self.sink.len());
        self.sink.play();
    }
    pub fn pause_sink(self:&Self) {
        self.sink.pause();
    }
    //this func adds a audio file from a given path
    pub fn add_song_from_path(&mut self, path: String) {
       self.add_song(Song::new(path).expect("failed to unwrap song"));
    }
    pub fn add_song(&mut self,s:Song) {

        let mut file = match file_io::load_file(s.get_song_path().parse().unwrap()) {
            Ok(t) => t,
            Err(E) => return
        };
        let decoder: Decoder<BufReader<File>> = match Decoder::new(file) {
            Ok(t) => t,
            Err(E) => {
                println!("failed to decode file:{}",E);
                return;
                //return Err("cant decode song".parse().unwrap());
            }
        };


        let sender = self.sender.clone();
        let source = decoder.periodic_access(Duration::from_secs(1),move |_| {
            sender.send(Message::SECOND_ELAPSED).expect("TODO: panic message");
        });
        if self.sink.len() == 0 {
            self.sink.pause();
        }
        self.sink.append(source);
        self.song_queue.push(s);
        println!("{}",self.song_queue.len())
    }
    pub fn queue_len(&self) -> usize{
        self.sink.len()
    }
    pub fn duration_of_song(&self) -> Duration {
        if self.song_queue.len() > 0 {
            self.song_queue[0].get_total_duration()
        } else {
            Duration::from_secs(0)
        }    }
    pub fn current_time(&self) -> Duration {
        if self.song_queue.len() > 0 {
            self.song_queue[0].get_current_duration()
        } else {
            Duration::from_secs(0)
        }
    }
    pub fn is_paused(&self) -> bool{
        self.sink.is_paused()
    }

    pub fn get_current_song(&self) -> &str {
        return if self.song_queue.len() > 0 {
            self.song_queue[0].get_song_name()
        } else {
            "no song playing"
        }
    }
    pub fn elapsed_second(&mut self) {
        if self.song_queue.len() > 0 {
            self.song_queue[0].add_second();
            println!("current second:{} current song:{}", self.song_queue[0].get_current_duration().as_secs(), self.song_queue[0].get_song_name());
            if self.sink.len() < self.song_queue.len() {
                self.song_queue.remove(0);
                println!("{}", self.song_queue.len())
            }
        }
    }
}
