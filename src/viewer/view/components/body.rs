use iced::{Align, Column, Command, Element, Length};

use super::{
    body_footer::{BodyFooter, BodyFooterMessage},
    body_header::{BodyHeader, BodyHeaderMessage},
    image_canvas::{ImageCanvas, ImageCanvasMessage},
};

#[derive(Debug, Clone, Copy)]
pub enum BodyMessage {
    ImageCanvasMessage(ImageCanvasMessage),
    BodyHeaderMessage(BodyHeaderMessage),
    BodyFooterMessage(BodyFooterMessage),
}

/// 画像ビューアメインパーツ
pub struct Body {
    canvas: ImageCanvas,
    header: BodyHeader,
    footer: BodyFooter,
}

impl Body {
    pub fn new() -> Self {
        Body {
            canvas: ImageCanvas::new(),
            header: BodyHeader::new(),
            footer: BodyFooter::new(),
        }
    }

    pub fn update(&mut self, message: BodyMessage) -> Command<BodyMessage> {
        match message {
            BodyMessage::ImageCanvasMessage(msg) => {
                self.canvas.update(msg);
            }
            BodyMessage::BodyHeaderMessage(msg) => {
                match msg {
                    BodyHeaderMessage::Single => {}
                    BodyHeaderMessage::Double => {}
                    BodyHeaderMessage::DoubleWithFront => {}
                }
                self.header.update(msg);
            }
            BodyMessage::BodyFooterMessage(msg) => {
                self.footer.update(msg);
            }
        }
        Command::none()
    }

    pub fn view(&mut self, pages: Vec<String>) -> Element<BodyMessage> {
        let image_view = Column::new()
            .push(
                self.canvas
                    .view(pages)
                    .map(|message| BodyMessage::ImageCanvasMessage(message)),
            )
            .height(Length::FillPortion(18))
            .width(Length::Fill)
            .align_items(Align::Center);

        let header = Column::new()
            .push(
                self.header
                    .view()
                    .map(|message| BodyMessage::BodyHeaderMessage(message)),
            )
            .height(Length::FillPortion(1))
            .width(Length::Fill);

        let footer = Column::new()
            .push(
                self.footer
                    .view()
                    .map(|message| BodyMessage::BodyFooterMessage(message)),
            )
            .height(Length::FillPortion(1))
            .width(Length::Fill)
            .align_items(Align::Center);

        let body = Column::new()
            .push(header)
            .push(image_view)
            .push(footer)
            .into();

        body
    }
}
