use std::path::Path;
use iced::{Color, Column, Element, Length};
use crate::event_codes::Message;

enum file_type {
    Dir,
    MP3
}

pub struct File_Graphic {
    name:String,
    Path:String,
    file_type: file_type

}


impl File_Graphic {
    pub fn new(path:String) -> File_Graphic {
        File_Graphic {
            name: "".to_string(),
            Path: path.clone(),
            file_type: file_type::Dir
        }

    }
}

// impl<Message, Renderer> Widget<Message, Renderer> for File_Graphic where Renderer: renderer::Renderer, {
//     fn width(&self) -> Length {
//         Length::Shrink
//     }
//
//     fn height(&self) -> Length {
//         Length::Shrink
//     }
//
//     fn layout(
//         &self,
//         _renderer: &Renderer,
//         _limits: &layout::Limits,
//     ) -> layout::Node {
//         layout::Node::new(Size::new(self.radius * 2.0, 4 * 2.0))
//     }
//
//     fn draw(
//         &self,
//         renderer: &mut Renderer,
//         _style: &renderer::Style,
//         layout: Layout<'_>,
//         _cursor_position: Point,
//         _viewport: &Rectangle,
//     ) {
//         renderer.fill_quad(
//             renderer::Quad {
//                 bounds: layout.bounds(),
//                 border_radius: self.radius,
//                 border_width: 0.0,
//                 border_color: Color::TRANSPARENT,
//             },
//             Color::BLACK,
//         );
//     }
// }
// impl<'a, Message, Renderer> From<File_Graphic> for Element<'a, Message>
//     where
//         Renderer: renderer::Renderer,
// {
//     fn from(f: File_Graphic) -> Self {
//         Self::new(f)
//     }
// }