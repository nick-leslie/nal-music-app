use std::borrow::BorrowMut;
use std::fs;
use std::fs::ReadDir;
use std::io::Sink;
use std::path::Path;
use iced::{Button, button, Color, Column, Element, Length, Scrollable};
use iced::pure::row;
use iced_native::Renderer;
use iced_native::widget::{Image, Row, Text,scrollable};
use crate::event_codes::Message;
use crate::file_io::is_song;

//TODO refactor this to be the files pane


#[derive(Debug)]
pub struct directory_graphic {
    current_path:String,
    current_dir: ReadDir,
    files:Vec<File_Graphic>,
    scroll_state:scrollable::State
}

impl directory_graphic {
    pub fn new(path:String) -> directory_graphic{
        //TODO add err handling
        let mut dir = fs::read_dir(path.clone()).expect("failed to read dir");
        let mut files = Vec::new();
        for file in dir.borrow_mut() {
            files.push(
                File_Graphic::new(
                    file.as_ref().unwrap().file_name().clone().to_str().unwrap().parse().unwrap(),
                    file.as_ref().unwrap().path().as_os_str().to_str().unwrap().parse().unwrap()
                ));
        }

        directory_graphic {
            current_path: path.clone(),
            current_dir: dir,
            files,
            scroll_state: Default::default()
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        // I make multiple rows by using some to add rows to a vector
        //we need to do it this way because push takes ownership of the row
        //so we use cr to store the current row and then we move it back into the vector
        let mut col = Column::new().padding(10);
        let mut rows = Vec::new();
        let mut i:usize = 0;

        for mut file in self.files.iter_mut(){
            if i % 5 == 0 {
                rows.push( Some(Row::new().spacing(20)));
            }
            let current_row = rows.len()-1;
            let mut cr =  rows[current_row].take().unwrap().push(file.view());
            rows[current_row] = Some(cr);
            i+=1;
        }
        for mut row in rows {
            match row {
                None => { println!("there are extra slots")}
                Some(r) => { col = col.push(r)}
            }
        }
        Scrollable::new(&mut self.scroll_state).push(col).into()

    }
    pub fn get_current_path(&self) -> String {
        self.current_path.clone()
    }
}



#[derive(Debug,Clone,PartialEq)]
enum file_type {
    Dir,
    MP3
}
#[derive(Debug,Clone)]
pub struct File_Graphic {
    name:String,
    Path:String,
    file_type: file_type,
    file_interact: button::State


}

//TODO make this have a  set size per item
impl File_Graphic {
    pub fn new(name:String,path:String) -> File_Graphic {
        let ft = match is_song(path.clone()) {
            true => {file_type::MP3}
            false => {file_type::Dir}
        };


        File_Graphic {
            name: name.clone(),
            Path: path.clone(),
            file_type: ft,
            file_interact: Default::default()
        }
    }
    fn update(&mut self, message:Message) {

    }
    //TODO make a set size for file element so we can dynamicly set the number of rows
     pub fn view(&mut self) -> Element<Message> {
        let filename_txt =Text::new(self.name.as_str()).size(10);
         let mut button_txt = Text::new("");
         if self.file_type == file_type::Dir {
              button_txt = Text::new("change directory");
         } else {
              button_txt = Text::new("add song");
         }
        let file_interact_button = Button::new(&mut self.file_interact,button_txt)
            .on_press(Message::FILE_INTERACTION(self.Path.to_string()));


        Row::new().push(
            Column::new()
                .push(filename_txt)
                .push(file_interact_button))
            .into()
    }
}

