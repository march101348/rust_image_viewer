extern crate iced;

use iced::{button, Sandbox, Element, Column, Button, Text, Settings, Container, Length, Image, Row, Align};

fn main() {
    Frame::run(Settings::default())
}

pub struct Frame {
    page: u32,
    prev_button: button::State,
    next_button: button::State,
}

impl Sandbox for Frame {
    type Message = Message;

    fn new() -> Self {
        Frame {
            page: 1,
            prev_button: button::State::default(),
            next_button: button::State::default(),
        }
    }

    fn title(&self) -> String {
        format!("{} page", self.page)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PrevPressed => {
                self.page -= 1;
            }
            Message::NextPressed => {
                self.page += 1;
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Element::from(Column::new()
            .align_items(Align::Center)
            .push(Image::new("src/img/chika.jpg")
                .width(Length::Fill)
                      .height(Length::Fill), )
            .push(Row::new()
                .push(Button::new(&mut self.prev_button, Text::new("PREV"))
                    .on_press(Message::PrevPressed),
                )
                .push(Button::new(&mut self.next_button, Text::new("NEXT"))
                    .on_press(Message::NextPressed),
                )
            )
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    PrevPressed,
    NextPressed,
}