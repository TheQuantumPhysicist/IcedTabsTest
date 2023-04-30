use iced::{
    alignment,
    widget::{container, text},
    Command, Element, Length,
};
use iced_aw::tab_bar::TabLabel;

use crate::backend_controller::NodeBackendController;

use super::{Tab, TabsMessage};

#[derive(Debug, Clone)]
pub enum SummaryMessage {
    NoOp,
}

pub struct SummaryTab {
    controller: NodeBackendController,
}

impl SummaryTab {
    pub fn new(controller: NodeBackendController) -> Self {
        SummaryTab { controller }
    }

    pub fn update(&mut self, message: SummaryMessage) -> Command<SummaryMessage> {
        match message {
            SummaryMessage::NoOp => Command::none(),
        }
    }
}

impl Tab for SummaryTab {
    type Message = TabsMessage;

    fn title(&self) -> String {
        String::from("Summary")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
        // TabLabel::IconText(Icon::User.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        genesis_block_label_field(&self.controller)
    }
}

pub fn genesis_block_label_field<'a>(
    _backend_controller: &NodeBackendController,
) -> Element<'a, TabsMessage, iced::Renderer> {
    let main_widget = text(&format!("Hello: {}", "world"))
        .width(Length::Fill)
        .size(25)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);

    let c = container(main_widget);

    c.into()
}
