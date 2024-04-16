use iced::executor;
use iced::{Application, Command, Settings, Theme, Element};

pub fn main() -> iced::Result {
    Collective::run(Settings::default())
}

struct Collective;

impl Application for Collective {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Collective, Command<Self::Message>) {
        (Collective, Command::none())
    }

    fn title(&self) -> String {
        String::from("Collective")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Hi Mom!".into()
    }
}
