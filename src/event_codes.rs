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
    FILE_INTERACTION(String),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    //TODO playlist reorder command

}