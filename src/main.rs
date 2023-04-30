mod backend_controller;
mod main_window;

use backend_controller::NodeBackendController;
use iced::futures::TryFutureExt;
use iced::widget::{column, container, text};
use iced::Subscription;
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use iced_aw::native::cupertino::cupertino_spinner::CupertinoSpinner;
use main_window::main_menu::MenuMessage;
use main_window::main_widget::MainWidgetMessage;
use main_window::MainWindow;

pub fn main() -> iced::Result {
    MintlayerNodeGUI::run(Settings {
        antialiasing: true,
        exit_on_close_request: false,
        try_opengles_first: true,
        ..Settings::default()
    })
}

enum MintlayerNodeGUI {
    Loading,
    Loaded(NodeBackendController, MainWindow),
    IntializationError(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<NodeBackendController, String>),
    EventOccurred(iced::Event),
    ShuttingDownFinished,
    MenuMessage(MenuMessage),
    MainWidgetMessage(MainWidgetMessage),
}

fn gui_shutdown(_controller: &mut NodeBackendController) -> Command<Message> {
    Command::none()
}

impl Application for MintlayerNodeGUI {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MintlayerNodeGUI::Loading,
            Command::perform(
                NodeBackendController::initialize().map_err(|e| e.to_string()),
                Message::Loaded,
            ),
        )
    }

    fn title(&self) -> String {
        match self {
            MintlayerNodeGUI::Loading => ("Node - Loading...").to_string(),
            MintlayerNodeGUI::Loaded(_d, _w) => {
                format!("Node")
            }
            MintlayerNodeGUI::IntializationError(_) => "Mintlayer initialization error".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            MintlayerNodeGUI::Loading => match message {
                Message::Loaded(Ok(controller)) => {
                    *self =
                        MintlayerNodeGUI::Loaded(controller.clone(), MainWindow::new(controller));
                    Command::none()
                }
                Message::Loaded(Err(e)) => {
                    *self = MintlayerNodeGUI::IntializationError(e);
                    Command::none()
                }
                Message::EventOccurred(event) => {
                    if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                        panic!("Attempted shutdown during initialization")
                    } else {
                        // While the screen is loading, ignore all events
                        Command::none()
                    }
                }
                Message::ShuttingDownFinished => Command::none(),
                Message::MenuMessage(_) => Command::none(),
                Message::MainWidgetMessage(_) => Command::none(),
            },
            MintlayerNodeGUI::Loaded(ref mut controller, ref mut w) => match message {
                Message::Loaded(_) => unreachable!("Already loaded"),
                Message::EventOccurred(event) => {
                    if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                        // TODO: this event doesn't cover the case of closing the Window through Cmd+Q in MacOS
                        gui_shutdown(controller)
                    } else {
                        Command::none()
                    }
                }
                Message::ShuttingDownFinished => iced::window::close(),
                Message::MenuMessage(menu_msg) => {
                    w.main_menu.update(menu_msg).map(Message::MenuMessage)
                }
                Message::MainWidgetMessage(main_widget_msg) => w
                    .main_widget
                    .update(main_widget_msg)
                    .map(Message::MainWidgetMessage),
            },
            MintlayerNodeGUI::IntializationError(_) => match message {
                Message::Loaded(_) => Command::none(),
                Message::EventOccurred(event) => {
                    if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                        iced::window::close()
                    } else {
                        Command::none()
                    }
                }
                Message::ShuttingDownFinished => iced::window::close(),
                Message::MenuMessage(_) => Command::none(),
                Message::MainWidgetMessage(_) => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            MintlayerNodeGUI::Loading => container(
                CupertinoSpinner::new()
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .into(),

            MintlayerNodeGUI::Loaded(state, w) => w.view(state),

            MintlayerNodeGUI::IntializationError(e) => {
                let error_box = column![
                    iced::widget::text("Node initialization failed".to_string()).size(32),
                    iced::widget::text(e.to_string()).size(20),
                    iced::widget::button(text("Close")).on_press(Message::ShuttingDownFinished)
                ]
                .align_items(iced::Alignment::Center)
                .spacing(5);

                container(error_box)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::events().map(Message::EventOccurred)
    }
}
