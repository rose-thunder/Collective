use iced::{Sandbox, Settings};

mod app;
mod gui;

fn main() -> iced::Result {
    gui::CollectiveGui::start()
}
