use iced::{Application, Settings};

mod app;
mod gui;

pub fn main() -> iced::Result {
    gui::CollectiveGui::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1920.0, 1080.0),
            resizable: true,
            decorations: true,
            ..iced::window::Settings::default()
        },
        default_text_size: iced::Pixels(18.0),
        ..Settings::default()
    })
}
