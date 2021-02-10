extern crate iced;
extern crate walkdir;

use iced::{Element, Column, Button, Text, Length, Image, Row, Align, Application, Command, executor, Subscription, Scrollable, keyboard, time};
use iced_native::subscription;
use iced_native::event::Event::Keyboard;
use iced_native::keyboard::Event::KeyReleased;
use std::path::Path;

use super::frame::Frame;
use super::super::controller::message::Message;

impl Application for Frame {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Frame::new("img/"), Command::none())
    }

    fn title(&self) -> String {
        format!("{}", Path::new(&self.page).file_stem().unwrap().to_str().unwrap())
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Changed => { self.update_pages() }
            Message::PrevPressed => { self.move_prev_page() }
            Message::NextPressed => { self.move_next_page() }
            Message::FileNamePressed(page_num) => { self.move_pressed_page(page_num) }
            _ => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(
            vec![
                time::every(std::time::Duration::from_millis(100)).map(|_| Message::Changed),
                subscription::events().map(|event| {
                    match event {
                        Keyboard(KeyReleased{ key_code: keyboard::KeyCode::Left, modifiers: _ }) => {
                            Message::PrevPressed
                        },
                        Keyboard(KeyReleased{ key_code: keyboard::KeyCode::Right, modifiers: _ })  => {
                            Message::NextPressed
                        },
                        _ => Message::None,
                    }
                })])
    }

    fn view(&mut self) -> Element<Message> {
        let root = self.root.clone();
        let page = root + self.page.to_str().unwrap();
        let image_view =
            Image::new(&page)
                .width(Length::Fill)
                .height(Length::Fill);
        let main_contents =
            Column::new()
                .height(Length::Fill)
                .width(Length::FillPortion(9))
                .align_items(Align::Center)
                .push(image_view);

        let image_pane =
            Scrollable::new(&mut self.image_files)
                .width(Length::Fill)
                .height(Length::Fill);
        let image_pane =
            self.pages.iter().enumerate().zip(self.file_name_buttons.iter_mut()).fold(image_pane, 
                |image_pane, ((page_num, page), button)| {
                    image_pane.push(
                        Button::new(button, Text::new(page.path().file_stem().unwrap().to_str().unwrap()))
                            .on_press(Message::FileNamePressed(page_num))
                            .width(Length::Fill))});
        let right_pane =
            Row::new()
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .push(image_pane);

        let body =
            Row::new()
                .height(Length::FillPortion(19))
                .width(Length::Fill)
                .push(main_contents)
                .push(right_pane);

        let prev_button =
            Button::new(&mut self.prev_button, Text::new("<<"))
                .on_press(Message::PrevPressed);
        let next_button =
            Button::new(&mut self.next_button, Text::new(">>"))
                .on_press(Message::NextPressed);
        let control_buttons =
            Row::new()
                .height(Length::Fill)
                .push(prev_button)
                .push(next_button);
        let footer =
            Column::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(control_buttons);

        Element::from(
            Column::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .push(body)
                .push(footer)
        )
    }
}
