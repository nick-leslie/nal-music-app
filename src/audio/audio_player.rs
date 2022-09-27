use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::fs;
use std::fs::File;
use std::io::{BufReader, sink};
use std::ops::Add;
use std::path::Path;
use std::time::Duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use crate::file_io;
use mp3_duration;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use crate::event_codes::Message;

pub struct AudioPlayer {
    sink:rodio::Sink,
    stream:OutputStream,
    stream_handle:OutputStreamHandle,
    duration_queue: Vec<Duration>,
    current_duration:Duration,
    sender:SyncSender<Message>,
}

impl AudioPlayer {
    pub fn new(sender:SyncSender<Message>) -> AudioPlayer {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        AudioPlayer {
            sink:rodio::Sink::try_new(&stream_handle).unwrap(),
            stream:_stream,
            stream_handle,
            duration_queue:Vec::new(),
            current_duration:Duration::new(0,0),
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
        let mut file = match file_io::load_file(path) {
            Ok(t) => t,
            Err(E) =>  {
                return;
            },
        };
        let duration = mp3_duration::from_read(&mut file).unwrap();
        self.duration_queue.push(duration);
        println!("{}",duration.as_secs());
        let decoder: Decoder<BufReader<File>> = match Decoder::new(file) {
            Ok(t) => t,
            Err(E) => {
                println!("failed to decode file:{}",E);
                return;
            }
        };
        println!("should have added new audio source");
        if self.sink.len() == 0 {
            self.sink.pause();
        }
        let sender = self.sender.clone();
        let source = decoder.periodic_access(Duration::from_secs(1),move |_| {
            sender.send(Message::SECOND_ELAPSED).expect("TODO: panic message");
        });
        self.sink.append(source);

    }
    pub fn queue_len(&self) -> usize{
        self.sink.len()
    }
    pub fn duration_of_song(&self) -> Duration {
         self.duration_queue[0]
    }
    pub fn current_time(&self) -> Duration {
        self.current_duration
    }
    pub fn is_paused(&self) -> bool{
        self.sink.is_paused()
    }
    pub fn elapsed_second(&mut self) {
        self.current_duration += Duration::from_secs(1);
        println!("current second:{}",self.current_duration.as_secs());
        if self.sink.len() > self.duration_queue.len() {
            self.duration_queue.pop();
            self.current_duration = Duration::from_secs(0);
            println!("reseting")
        }
    }
}
