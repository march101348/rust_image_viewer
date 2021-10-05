use iced::{button, Button, Column, Command, Element, Row, Text};

#[derive(Debug, Clone, Copy)]
pub enum BodyHeaderMessage {
    Single,
    Double,
    DoubleWithFront,
}

pub struct BodyHeader {
    single_button_state: button::State,
    double_button_state: button::State,
    double_front_button_state: button::State,
}

impl BodyHeader {
    pub fn new() -> Self {
        BodyHeader {
            single_button_state: button::State::default(),
            double_button_state: button::State::default(),
            double_front_button_state: button::State::default(),
        }
    }

    pub fn update(&mut self, message: BodyHeaderMessage) -> Command<BodyHeaderMessage> {
        match message {
            BodyHeaderMessage::Single => {}
            BodyHeaderMessage::Double => {}
            BodyHeaderMessage::DoubleWithFront => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<BodyHeaderMessage> {
        let single_button = Button::new(&mut self.single_button_state, Text::new("single"))
            .on_press(BodyHeaderMessage::Single);
        let double_button = Button::new(&mut self.double_button_state, Text::new("double"))
            .on_press(BodyHeaderMessage::Double);
        let double_front_button = Button::new(
            &mut self.double_front_button_state,
            Text::new("double front"),
        )
        .on_press(BodyHeaderMessage::DoubleWithFront);
        let buttons = Row::new()
            .push(single_button)
            .push(double_button)
            .push(double_front_button);
        Column::new().push(buttons).into()
    }
}
