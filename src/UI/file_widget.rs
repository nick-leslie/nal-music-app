use std::borrow::BorrowMut;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;
use iced::{Color, Column, Element, Length};
use iced_native::widget::{Row, Text};
use crate::event_codes::Message;

#[derive(Debug)]
pub struct directory_graphic {
    current_dir: ReadDir,
    files:Vec<File_Graphic>
}

impl directory_graphic {
    pub fn new(path:String) -> directory_graphic{
        //TODO add err handling

        let mut dir = fs::read_dir(path).expect("failed to read dir");
        let mut files = Vec::new();
        for file in dir.borrow_mut() {
            files.push(
                File_Graphic::new(
                    file.as_ref().unwrap().file_name().clone().to_str().unwrap().parse().unwrap(),
                    file.as_ref().unwrap().path().as_os_str().to_str().unwrap().parse().unwrap()
                ));
        }

        directory_graphic {
            current_dir: dir,
            files
        }
    }
    pub fn veiw(& mut self) -> Element<Message> {

        let dire_structure = self.files.iter_mut().enumerate().fold(
            Row::new().spacing(20),
            |row, (i,f)| {
                let t = f.view();
                row.push(t)
            });
        dire_structure.into()
    }
}


#[derive(Debug,Clone)]
enum file_type {
    Dir,
    MP3
}
#[derive(Debug,Clone)]
pub struct File_Graphic {
    name:String,
    Path:String,
    file_type: file_type

}


impl File_Graphic {
    pub fn new(name:String,path:String) -> File_Graphic {
        File_Graphic {
            name: name.clone(),
            Path: path.clone(),
            file_type: file_type::Dir
        }
    }
    fn update(&mut self, message:Message) {

    }
     pub fn view(&mut self) -> Element<Message> {
        let filename_txt =Text::new(self.name.as_str()).size(40);

        Column::new().push(filename_txt).into()
    }
}

