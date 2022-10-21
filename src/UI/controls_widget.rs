use iced::{button, Button, Column, Element, Row, Text};
use iced_native::widget::ProgressBar;
use crate::AudioPlayer;
use crate::event_codes::Message;
use crate::event_codes::Message::ReArrange;

pub struct ControlPanel {
    play_button: button::State,
    pause_button: button::State,
    re_arrange: button::State,
}

impl ControlPanel {
    pub fn new() -> ControlPanel {
        ControlPanel {
            play_button: Default::default(),
            pause_button: Default::default(),
            re_arrange: Default::default()
        }
    }
    pub fn view(&mut self,ap:&mut AudioPlayer) -> Element<Message> {
        let play_button = Button::new(&mut self.play_button, Text::new("play")).on_press(Message::PLAY);
        let pause_button = Button::new(&mut self.pause_button, Text::new("pause")).on_press(Message::PAUSE);
        let current_song_text = Text::new(ap.get_current_song()).size(50);
        // duration bar row
        let seconds_played_txt = Text::new(ap.current_time().as_secs().to_string()).size(40);
        //TODO bug with duration bar becuse of playlist changes
        let duration_bar = ProgressBar::new(0.0..= ap.duration_of_song().as_secs() as f32, ap.current_time().as_secs() as f32);
        let total_duration_timer =Text::new(ap.duration_of_song().as_secs().to_string()).size(40);


        //TODO we make a bug here becuse 1 and 2 might be larger than playlist
        let re_arrange_buttion =  Button::new(&mut self.re_arrange,Text::new("swap 2 and 3")).on_press(ReArrange(0,1));
        let overall_col = Column::new()
            .push(Row::new()
                .push(play_button)
                .push(pause_button)
                .push(current_song_text))
            .push(Row::new().padding([5, 5, 15, 15])
                      .push(seconds_played_txt)
                      .push(duration_bar))
            .           push(total_duration_timer)
            .push(Row::new().push(re_arrange_buttion))
            .into();
        overall_col
    }
}