use iced::{
    alignment,
    widget::{button, container, text},
    Color, Command, Element, Length,
};
use iced_aw::menu::{CloseCondition, ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};

use crate::backend_controller::NodeBackendController;

#[derive(Debug, Clone)]
pub enum MenuMessage {
    NoOp,
    Exit,
}

pub struct MainMenu {
    _backend_controller: NodeBackendController,
}

impl MainMenu {
    pub fn new(backend_controller: NodeBackendController) -> Self {
        Self {
            _backend_controller: backend_controller,
        }
    }

    pub fn view(
        &self,
        _backend_controller: &NodeBackendController,
    ) -> Element<'_, MenuMessage, iced::Renderer> {
        let file_menu = make_menu_file();
        let help_menu = make_menu_help();

        let menu_bar = MenuBar::new(vec![file_menu, help_menu])
            .item_width(ItemWidth::Uniform(180))
            .item_height(ItemHeight::Uniform(25))
            .spacing(4.0)
            .bounds_expand(30)
            .path_highlight(Some(PathHighlight::MenuActive))
            .close_condition(CloseCondition {
                leave: true,
                click_outside: false,
                click_inside: false,
            });

        let c = iced::widget::column![container(menu_bar)];

        c.into()
    }

    pub fn update(&self, msg: MenuMessage) -> Command<MenuMessage> {
        match msg {
            MenuMessage::NoOp => Command::none(),
            MenuMessage::Exit => iced::window::close(),
        }
    }
}

struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border_radius: 4.0,
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, MenuMessage, iced::Renderer>>,
    msg: MenuMessage,
) -> button::Button<'a, MenuMessage, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

fn labeled_button<'a>(
    label: &str,
    msg: MenuMessage,
) -> button::Button<'a, MenuMessage, iced::Renderer> {
    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}

fn menu_item<'a>(label: &str, msg: MenuMessage) -> MenuTree<'a, MenuMessage, iced::Renderer> {
    MenuTree::new(
        labeled_button(label, msg)
            .width(Length::Fill)
            .height(Length::Fill),
    )
}

fn make_menu_file<'a>() -> MenuTree<'a, MenuMessage, iced::Renderer> {
    let root = MenuTree::with_children(
        labeled_button("File", MenuMessage::NoOp),
        vec![
            menu_item("Settings", MenuMessage::NoOp),
            menu_item("Exit", MenuMessage::Exit),
        ],
    )
    .width(110);

    root
}

fn make_menu_help<'a>() -> MenuTree<'a, MenuMessage, iced::Renderer> {
    let root = MenuTree::with_children(
        labeled_button("Help", MenuMessage::NoOp),
        vec![menu_item("About", MenuMessage::NoOp)],
    )
    .width(110);

    root
}
