use iced::{button, scrollable, Button, Column, Command, Element, Length, Row, Scrollable, Text};

use crate::viewer::values::page::Pages;

#[derive(Debug, Clone, Copy)]
pub enum ImageButtonsMessage {
    FolderRootChanged(usize),
    ToggleVisibleButtonPressed(bool),
    ImageButtonPressed(usize),
    FileNumChanged(usize),
}

/// ページ選択用サイドボタン
pub struct ImageButtons {
    visible_button_state: button::State,
    buttons_state: Vec<button::State>,
    scroll_state: scrollable::State,
}

impl ImageButtons {
    pub fn new(file_num: usize) -> Self {
        ImageButtons {
            visible_button_state: button::State::default(),
            buttons_state: (0..file_num).map(|_| button::State::default()).collect(),
            scroll_state: scrollable::State::default(),
        }
    }

    pub fn update(&mut self, message: ImageButtonsMessage) -> Command<ImageButtonsMessage> {
        match message {
            ImageButtonsMessage::FolderRootChanged(file_num) => {
                self.buttons_state = (0..file_num).map(|_| button::State::default()).collect();
            }
            ImageButtonsMessage::FileNumChanged(file_num) => {
                if file_num != self.buttons_state.len() {
                    self.buttons_state = (0..file_num).map(|_| button::State::default()).collect();
                }
            }
            ImageButtonsMessage::ImageButtonPressed(_) => {}
            ImageButtonsMessage::ToggleVisibleButtonPressed(_) => {}
        }
        Command::none()
    }

    pub fn view(
        &mut self,
        pages: &Pages,
        selected: Vec<usize>,
        is_visible_buttons: bool,
    ) -> Element<ImageButtonsMessage> {
        let image_pane = if is_visible_buttons {
            let file_names: Vec<String> = pages.0.iter().map(|pg| pg.stem()).collect();

            let scroll = Scrollable::new(&mut self.scroll_state);
            let buttons = file_names
                .iter()
                .enumerate()
                .zip(self.buttons_state.iter_mut())
                .fold(scroll, |image_pane, ((page_num, page), button)| {
                    image_pane.push(if selected.iter().find(|num| num == &&page_num) != None {
                        Button::new(button, Text::new(page)).width(Length::Fill)
                    } else {
                        Button::new(button, Text::new(page))
                            .on_press(ImageButtonsMessage::ImageButtonPressed(page_num))
                            .width(Length::Fill)
                    })
                })
                .width(Length::FillPortion(9));

            let toggle_button = Column::new()
                .push(
                    Button::new(&mut self.visible_button_state, Text::new(">>"))
                        .on_press(ImageButtonsMessage::ToggleVisibleButtonPressed(
                            !is_visible_buttons,
                        ))
                        .width(Length::Fill),
                )
                .width(Length::FillPortion(1));
            Row::new().push(toggle_button).push(buttons)
        } else {
            let toggle_button = Column::new()
                .push(
                    Button::new(&mut self.visible_button_state, Text::new("<<"))
                        .on_press(ImageButtonsMessage::ToggleVisibleButtonPressed(
                            !is_visible_buttons,
                        ))
                        .width(Length::Fill),
                )
                .width(Length::FillPortion(1));
            Row::new().push(toggle_button)
        };
        Column::new().push(image_pane).into()
    }
}
