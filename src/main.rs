mod viewer;

use iced::{Application, Settings};

use crate::viewer::view::frame::Frame;

fn main() -> iced::Result {
    Frame::run(Settings::default())
}
