use iced::{button, Button, Column, Command, Element, Row, Text};

#[derive(Debug, Clone, Copy)]
pub enum BodyFooterMessage {
    NextPressed,
    PrevPressed,
}

pub struct BodyFooter {
    pub prev_button: button::State,
    pub next_button: button::State,
}

impl BodyFooter {
    pub fn new() -> Self {
        BodyFooter {
            prev_button: button::State::default(),
            next_button: button::State::default(),
        }
    }

    pub fn update(&mut self, message: BodyFooterMessage) -> Command<BodyFooterMessage> {
        match message {
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<BodyFooterMessage> {
        let prev_button = Button::new(&mut self.prev_button, Text::new("<<"))
            .on_press(BodyFooterMessage::PrevPressed);
        let next_button = Button::new(&mut self.next_button, Text::new(">>"))
            .on_press(BodyFooterMessage::NextPressed);
        let control_buttons = Row::new().push(prev_button).push(next_button);
        let footer = Column::new().push(control_buttons).into();
        footer
    }
}
