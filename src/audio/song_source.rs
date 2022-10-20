use std::iter::Take;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use rodio::{Sample, Source, source};

pub struct SongSource<S> {
    source: Option<Box<dyn Source<Item = S> + Send>>,
    done_signal:Arc<AtomicUsize>
}
//TODO figure out how to intianshiate
impl<S: Sample> SongSource<S> {
    pub fn new<T>(source:T) -> SongSource<S>
        where T:Source<Item = S> + Send + 'static,
    {
        let mut done_signal:Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));
        let source_with_done = source::Done::new(source,done_signal.clone());
        SongSource {
            source:Some( Box::new(source_with_done)),
            done_signal
        }
    }
    pub fn give_source(&mut self) -> (Box<dyn Source<Item = S> + Send>,Arc<AtomicUsize>) {
        let source_box = self.source.take().unwrap();
        (source_box,self.done_signal.clone())
    }
}