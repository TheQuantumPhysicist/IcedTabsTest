mod tabs;

use iced::{Command, Element};

use crate::backend_controller::NodeBackendController;

#[derive(Debug, Clone)]
pub enum MainWidgetMessage {
    NoOp,
    TabsMessage(tabs::TabsMessage),
}

pub struct MainWidget {
    tabs: tabs::TabsWidget,
}

impl MainWidget {
    pub fn new(backend_controller: NodeBackendController) -> Self {
        Self {
            tabs: tabs::TabsWidget::new(backend_controller),
        }
    }

    pub fn view(
        &self,
        backend_controller: &NodeBackendController,
    ) -> Element<'_, MainWidgetMessage, iced::Renderer> {
        self.tabs
            .view(backend_controller)
            .map(MainWidgetMessage::TabsMessage)
    }

    pub fn update(&mut self, msg: MainWidgetMessage) -> Command<MainWidgetMessage> {
        match msg {
            MainWidgetMessage::NoOp => Command::none(),
            MainWidgetMessage::TabsMessage(tabs_message) => self
                .tabs
                .update(tabs_message)
                .map(MainWidgetMessage::TabsMessage),
        }
    }
}
