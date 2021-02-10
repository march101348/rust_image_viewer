use iced::{Settings, Application};

use image_viewer::viewer::view::frame::Frame;

fn main() -> iced::Result {
    Frame::run(Settings::default())
}
