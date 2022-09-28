use std::time::Duration;

#[derive(Clone,Copy,Debug)]
pub enum Message {
    PLAY,
    PAUSE,
    SECOND_ELAPSED,
    NONE,
    TICK
}