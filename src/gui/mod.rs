pub mod style;
pub mod views;
pub mod widgets;

use iced::{Application, Command, Element, Settings, Theme};

fn main() -> iced::Result {
    CollectiveGui::run(Settings::default())
}

pub struct CollectiveGui;

impl Application for CollectiveGui {
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (CollectiveGui, Command::none())
    }

    fn title(&self) -> String {
        String::from("Collective")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        "Welcome to Collective".into()
    }
}
