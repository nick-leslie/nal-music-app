use std::path::Path;
use std::time::Duration;
use iced::pane_grid;

#[derive(Clone,Debug)]
pub enum Message {
    PLAY,
    PAUSE,
    SECOND_ELAPSED,
    NONE,
    TICK,
    CHANGE_VOL(f32),
    FILE_INTERACTION(String),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    ReArrange(usize,usize)
    //TODO playlist reorder command

}