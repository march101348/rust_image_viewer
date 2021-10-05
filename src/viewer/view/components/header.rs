use iced::{button, Button, Column, Command, Element, Length, Text};

#[derive(Debug, Clone, Copy)]
pub enum HeaderMessage {
    FileOpenPressed,
}

/// ヘッダーメニュー
pub struct Header {
    file_open_button_state: button::State,
}

impl Header {
    pub fn new() -> Self {
        Header {
            file_open_button_state: button::State::default(),
        }
    }

    pub fn update(&mut self, message: HeaderMessage) -> Command<HeaderMessage> {
        match message {
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<HeaderMessage> {
        let file_open_button = Column::new()
            .push(
                Button::new(&mut self.file_open_button_state, Text::new("folder open"))
                    .on_press(HeaderMessage::FileOpenPressed),
            )
            .width(Length::Fill);
        file_open_button.into()
    }
}
