use std::fs::File;
use std::io::{BufReader, sink};
use rodio::{Decoder, OutputStream, OutputStreamHandle};
use crate::file_io;
pub struct AudioPlayer {
     pub sink:rodio::Sink,
    stream:OutputStream,
    stream_handle:OutputStreamHandle,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        AudioPlayer {
            sink:rodio::Sink::try_new(&stream_handle).unwrap(),
            stream:_stream,
            stream_handle,
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
    pub fn add_song_from_path(self:&Self,path: String) {
        let file = match file_io::load_file(path) {
            Ok(t) => t,
            Err(E) =>  {
                return;
            },
        };

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
        self.sink.append(decoder);

    }
    pub fn queue_len(&self) -> usize{
        self.sink.len()
    }
    //this adds an audio from a pre created source
    fn add_audio_source(self:&Self,source:rodio::Decoder<File>) {
        self.sink.append(source)
    }
}