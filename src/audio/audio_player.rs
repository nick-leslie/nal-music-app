use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::fs;
use std::fs::File;
use std::io::{BufReader, sink};
use std::iter::successors;
use std::ops::Add;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use crate::file_io;
use mp3_duration;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use iced::window::icon::Error::DimensionsMismatch;
use rodio::source::ChannelVolume;
use crate::audio::song::SongInfo;
use crate::event_codes::Message;
use crate::audio::playlist;
use crate::audio::playlist::Playlist;
use crate::audio::song_source::SongSource;

pub struct AudioPlayer {
    sink:rodio::Sink,
    stream:OutputStream,
    stream_handle:OutputStreamHandle,
    song_queue:Rc<RefCell<Playlist>>,
    sender:SyncSender<Message>,
    //TODO make this one arc usize that we use to represent the number of items in queue
    current_done_signal:Option<Arc<AtomicUsize>>
}

impl AudioPlayer {
    pub fn new(sender:SyncSender<Message>) -> AudioPlayer {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        AudioPlayer {
            sink:rodio::Sink::try_new(&stream_handle).unwrap(),
            stream:_stream,
            stream_handle,
            song_queue:Rc::new(RefCell::new(Playlist::new())),
            sender,
            current_done_signal: None
        }
    }
    pub fn play_sink(&mut self) {
        println!("number of tracks:{}",self.sink.len());
        if self.sink.len() == 0 {
            let mut song= self.playlist_mut().give_next_song();
            match song{
                Some(mut s) => {
                    let (source,done_signal) = s.give_source();
                    self.sink.append(source);
                    self.current_done_signal = Some(done_signal);
                },
                None => {}
            }
        }
        self.sink.play();
    }
    pub fn pause_sink(self:&Self) {
        self.sink.pause();
    }
    //this func adds a audio file from a given path
    pub fn add_song_from_path(&mut self, path: String) {
        self.add_song(SongInfo::new(path).expect("failed to unwrap song"));
    }

    pub fn add_song(&mut self,s: SongInfo) {

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
        let source = decoder.periodic_access(Duration::from_secs(1),move |src|
            {
            sender.send(Message::SECOND_ELAPSED).expect("TODO: panic message");
        }).convert_samples();
        if self.sink.len() == 0 {
            self.sink.pause();
        }
        //self.sink.append(source);
        self.playlist_mut().add_song_info(s);
        self.playlist_mut().add_song(SongSource::new(source));
        //self.song_queue.as_ref().borrow_mut().add_song_info(s);
        println!("{}",self.playlist_len())
    }

    pub fn queue_len(&self) -> usize{
        self.sink.len()
    }
    pub fn playlist_len(&self) -> usize {
        self.song_queue.as_ref().borrow().get_len()
    }
    pub fn playlist_mut(&mut self) -> RefMut<Playlist> {
        self.song_queue.as_ref().borrow_mut()
    }
    pub fn playlist(&self) -> Ref<Playlist> {
        self.song_queue.as_ref().borrow()
    }
    pub fn duration_of_song(&self) -> Duration {
        if self.playlist_len() > 0 {
            self.playlist().get_current_song().get_total_duration()
        } else {
            Duration::from_secs(0)
        }    }
    pub fn current_time(&self) -> Duration {
        if self.playlist_len() > 0 {
            self.playlist().get_current_song().get_current_duration()
        } else {
            Duration::from_secs(0)
        }
    }
    pub fn is_paused(&self) -> bool{
        self.sink.is_paused()
    }
    pub fn get_play_list(&mut self) -> Rc<RefCell<Playlist>>{
        self.song_queue.clone()
    }
    pub fn get_current_song(&self) -> String {
        return if self.playlist_len() > 0 {
            let song_name = self.playlist().get_current_song().get_song_name();
            song_name
        } else {
            "no song playing".to_string()
        }
    }
    //TODO we arnt proporly marking song as doen when swaping
    pub fn elapsed_second(&mut self) {
        if self.playlist_len() > 0 {
            self.playlist_mut().add_second();
            println!("current second:{} current song:{}", self.playlist().get_current_song().get_current_duration().as_secs(), self.playlist().get_current_song().get_song_name());
            //TODO refactor this to not use match and instead just have one Arc<usize> the length of the queue

            // if self.sink.len() < self.playlist_len() {
            //     println!("removing for some reason");
            //     self.playlist_mut().remove_first();
            //     println!("{}", self.playlist_len())
            // }
        }
    }
    pub fn done_check(&mut self) {
        match self.current_done_signal.take() {
            Some(done_signal) => {
                let is_done = done_signal.load(Ordering::Relaxed);
                if is_done == 0 {
                    println!("removing for some reason");
                    self.playlist_mut().remove_first();
                    self.play_sink();
                    println!("{}", self.playlist_len())
                } else {
                    self.current_done_signal = Some(done_signal);
                }
            }
            None => {}
        }
    }
}