use std::time::Duration;
use iced::pane_grid;

#[derive(Clone,Copy,Debug)]
pub enum Message {
    PLAY,
    PAUSE,
    SECOND_ELAPSED,
    NONE,
    TICK,
}