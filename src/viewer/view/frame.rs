extern crate iced;
extern crate nfd;
extern crate notify;
extern crate walkdir;

use iced::{Row, time};
use nfd::Response;
use std::path::Path;

use iced::{
    executor, keyboard, Application, Clipboard, Column, Command, Element, Length, Subscription,
};
use iced_native::event::Event::Keyboard;
use iced_native::keyboard::Event::KeyReleased;
use iced_native::subscription;

// use crate::viewer::model::file_notification::{watch, FileEvent};
use crate::viewer::values::page::Pages;

use super::components::body::{Body, BodyMessage};
use super::components::body_footer::BodyFooterMessage;
use super::components::body_header::BodyHeaderMessage;
use super::components::header::{Header, HeaderMessage};
use super::components::image_buttons::{ImageButtons, ImageButtonsMessage};

/// 閲覧中ページ番号
pub enum ViewPageNums {
    Single(i32),
    Double(i32),
    DoubleWithFront(i32),
}

impl ViewPageNums {
    fn get_inner(&self) -> i32 {
        match self {
            &ViewPageNums::Single(num) => num,
            &ViewPageNums::Double(num) => num,
            &ViewPageNums::DoubleWithFront(num) => num,
        }
    }

    fn get_same_type(&self, num: i32) -> Self {
        match self {
            ViewPageNums::Single(_) => ViewPageNums::Single(num),
            ViewPageNums::Double(_) => ViewPageNums::Double(num / 2 * 2),
            ViewPageNums::DoubleWithFront(_) => ViewPageNums::DoubleWithFront((num + 1) / 2 * 2),
        }
    }

    fn reset(&self) {
        self.get_same_type(0);
    }

    fn page_nums(&self) -> Vec<i32> {
        match self {
            &ViewPageNums::Single(num) => vec![num],
            &ViewPageNums::Double(num) => vec![num, num + 1],
            &ViewPageNums::DoubleWithFront(num) => vec![num - 1, num],
        }
    }
}

/// アプリケーションroot
pub struct Frame {
    root: String,
    pages: Pages,
    view_on: ViewPageNums,
    header: Header,
    body: Body,
    buttons: ImageButtons,
    is_visible_buttons: bool,
}

impl Frame {
    pub fn new(root: &str) -> Self {
        let root = String::from(root);
        let pages = Pages::new(&root);
        let buttons = ImageButtons::new(pages.0.len());
        Frame {
            root,
            pages,
            view_on: ViewPageNums::Single(0),
            header: Header::new(),
            body: Body::new(),
            buttons,
            is_visible_buttons: true,
        }
    }

    pub fn update_pages(&mut self) {
        self.pages = Pages::new(&self.root);
        self.move_specified_page(self.view_on.get_inner());
    }

    pub fn move_prev_page(&mut self) {
        let page_num = match self.view_on {
            ViewPageNums::Single(num) => {
                if num - 1 < 0 {
                    self.pages.0.len() as i32 - 1
                } else {
                    num - 1
                }
            }
            ViewPageNums::Double(num) | ViewPageNums::DoubleWithFront(num) => {
                if num - 2 < 0 {
                    self.pages.0.len() as i32 - 1
                } else {
                    num - 2
                }
            }
        };
        self.move_specified_page(page_num);
    }

    pub fn move_next_page(&mut self) {
        let page_num = match self.view_on {
            ViewPageNums::Single(num) => {
                if num + 1 >= self.pages.0.len() as i32 {
                    0
                } else {
                    num + 1
                }
            }
            ViewPageNums::Double(num) => {
                if num + 2 >= self.pages.0.len() as i32 {
                    0
                } else {
                    num + 2
                }
            }
            ViewPageNums::DoubleWithFront(num) => {
                if num + 2 >= (self.pages.0.len() + 1) as i32 {
                    0
                } else {
                    num + 2
                }
            }
        };
        self.move_specified_page(page_num);
    }

    pub fn move_pressed_page(&mut self, page_num: i32) {
        self.move_specified_page(page_num);
    }

    fn move_specified_page(&mut self, page_num: i32) {
        if page_num < 0 || page_num >= self.pages.0.len() as i32 {
            self.init_viewing();
            return;
        }
        self.view_on = self.view_on.get_same_type(page_num);
    }

    fn init_viewing(&mut self) {
        self.view_on.reset();
    }

    fn on_file_open_pressed(&mut self) -> bool {
        let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
            panic!(e);
        });
        match result {
            Response::Okay(root) => {
                self.root = root;
                self.init_viewing();
                self.update_pages();
                true
            }
            Response::OkayMultiple(files) => false,
            Response::Cancel => false,
        }
    }

    fn on_view_type_changed(&mut self, msg: ViewPageNums) {
        self.view_on = msg.get_same_type(self.view_on.get_inner());
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    None,
    FilesChanged,
    LeftKeyPressed,
    RightKeyPressed,
    HeaderMessage(HeaderMessage),
    BodyMessage(BodyMessage),
    ImageButtonsMessage(ImageButtonsMessage),
}

impl Application for Frame {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Frame::new("."), Command::none())
    }

    fn title(&self) -> String {
        format!(
            "{}",
            Path::new(
                &self
                    .pages
                    .0
                    .get(self.view_on.get_inner() as usize)
                    .map(|pg| pg.stem())
                    .clone()
                    .unwrap_or_else(|| String::from("no image files"))
            )
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
        )
    }

    fn update(&mut self, message: Self::Message, _: &mut Clipboard) -> Command<Message> {
        match message {
            Message::FilesChanged => {
                self.update_pages();
                self.buttons
                    .update(ImageButtonsMessage::FileNumChanged(self.pages.0.len()));
            }
            Message::LeftKeyPressed => self.move_prev_page(),
            Message::RightKeyPressed => self.move_next_page(),
            Message::HeaderMessage(msg) => {
                match msg {
                    HeaderMessage::FileOpenPressed => {
                        let root_changed = self.on_file_open_pressed();
                        if root_changed {
                            self.buttons
                                .update(ImageButtonsMessage::FolderRootChanged(self.pages.0.len()));
                        }
                    }
                }
                self.header.update(msg);
            }
            Message::BodyMessage(msg) => {
                match msg {
                    BodyMessage::ImageCanvasMessage(_) => {}
                    BodyMessage::BodyHeaderMessage(msg) => {
                        let msg = match msg {
                            BodyHeaderMessage::Single => ViewPageNums::Single(0),
                            BodyHeaderMessage::Double => ViewPageNums::Double(0),
                            BodyHeaderMessage::DoubleWithFront => ViewPageNums::DoubleWithFront(0),
                        };
                        self.on_view_type_changed(msg);
                    }
                    BodyMessage::BodyFooterMessage(BodyFooterMessage::NextPressed) => {
                        self.move_next_page();
                    }
                    BodyMessage::BodyFooterMessage(BodyFooterMessage::PrevPressed) => {
                        self.move_prev_page();
                    }
                }
                self.body.update(msg);
            }
            Message::ImageButtonsMessage(msg) => {
                match msg {
                    ImageButtonsMessage::ImageButtonPressed(page_num) => {
                        self.move_pressed_page(page_num as i32);
                    }
                    ImageButtonsMessage::ToggleVisibleButtonPressed(is_visible) => {
                        self.is_visible_buttons = is_visible;
                    }
                    _ => {}
                }
                self.buttons.update(msg);
            }
            _ => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            // watch(&self.root).map(|event| match event {
            //     FileEvent::Changed => {
            //         println!("changed!!!!!");
            //         Message::FilesChanged
            //     }
            // }),
            time::every(std::time::Duration::from_millis(100)).map(|_| Message::FilesChanged),
            subscription::events().map(|event| match event {
                Keyboard(KeyReleased {
                    key_code: keyboard::KeyCode::Left,
                    modifiers: _,
                }) => Message::LeftKeyPressed,
                Keyboard(KeyReleased {
                    key_code: keyboard::KeyCode::Right,
                    modifiers: _,
                }) => Message::RightKeyPressed,
                _ => Message::None,
            }),
        ])
    }

    fn view(&mut self) -> Element<Message> {
        let selected: Vec<usize> = self
            .view_on
            .page_nums()
            .into_iter()
            .filter(|num| num >= &0 && num < &(self.pages.0.len() as i32))
            .map(|num| num as usize)
            .collect();
        let mut pages: Vec<String> = selected
            .iter()
            .map(|num| self.pages.0.get(*num).unwrap().path())
            .collect();
        pages.reverse();

        let header = Column::new()
            .push(self.header.view().map(|msg| Message::HeaderMessage(msg)))
            .width(Length::Fill);

        let body = Column::new()
            .push(
                self.body
                    .view(pages)
                    .map(|message| Message::BodyMessage(message)),
            )
            .width(Length::FillPortion(if self.is_visible_buttons {
                8
            } else {
                80
            }));

        let buttons = Column::new()
            .push(
                self.buttons
                    .view(&self.pages, selected, self.is_visible_buttons)
                    .map(|msg| Message::ImageButtonsMessage(msg)),
            )
            .width(Length::FillPortion(2));

        Column::new()
            .push(header.width(Length::Fill))
            .push(
                Row::new()
                    .push(body)
                    .push(buttons)
                    .height(Length::Fill)
                    .width(Length::Fill),
            )
            .into()
    }
}
