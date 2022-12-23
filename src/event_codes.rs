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
    SWAP_SONGS(usize),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    PaneClicked(pane_grid::Pane),
    ReArrange(usize,usize)
    //TODO playlist reorder command

}