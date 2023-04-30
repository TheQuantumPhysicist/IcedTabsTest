use iced::Element;

use crate::{backend_controller::NodeBackendController, Message};

pub mod main_menu;
pub mod main_widget;

pub struct MainWindow {
    pub main_menu: main_menu::MainMenu,
    pub main_widget: main_widget::MainWidget,
}

impl MainWindow {
    pub fn new(backend_controller: NodeBackendController) -> Self {
        Self {
            main_menu: main_menu::MainMenu::new(backend_controller.clone()),
            main_widget: main_widget::MainWidget::new(backend_controller),
        }
    }

    pub fn view(
        &self,
        backend_controller: &NodeBackendController,
    ) -> Element<'_, Message, iced::Renderer> {
        let c = iced::widget::column![
            self.main_menu
                .view(backend_controller)
                .map(Message::MenuMessage),
            self.main_widget
                .view(backend_controller)
                .map(Message::MainWidgetMessage)
        ];

        c.into()
    }
}
