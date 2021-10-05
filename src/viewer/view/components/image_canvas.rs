use iced::{Command, Element, Image, Length, Row};

#[derive(Debug, Clone, Copy)]
pub enum ImageCanvasMessage {}

pub struct ImageCanvas;

impl ImageCanvas {
    pub fn new() -> Self {
        ImageCanvas
    }

    pub fn update(&mut self, message: ImageCanvasMessage) -> Command<ImageCanvasMessage> {
        match message {
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self, file_paths: Vec<String>) -> Element<ImageCanvasMessage> {
        let canvas = file_paths
            .iter()
            .map(|path| Image::new(path).width(Length::FillPortion(1)))
            .fold(Row::new(), |row, image| row.push(image));
        canvas.into()
    }
}
