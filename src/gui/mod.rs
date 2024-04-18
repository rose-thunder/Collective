pub mod style;
pub mod views;
pub mod widgets;

use iced::{Application, Command, Element, Theme};

#[derive(Default, Debug, Clone)]
enum View {
    #[default]
    About,
    File,
    Tags,
    Settings,
}

#[derive(Default, Clone)]
pub struct CollectiveGui {
    view: View,
    file_view: FileView,
    tags_view: TagsView,
    settings_view: SettingsView,
    file_list: Vec<Folder>,
    selected_file: Option<Folder>, // index of file_list
}

#[derive(Debug, Clone)]
pub enum Message {
    AboutPressed,
    SettingsPressed,
    TagsPressed,
    FilePressed,
    TagsAction(TagsMessage),
    AboutAction(AboutMessage),
    SettingsAction(SettingsMessage),
    FileAction(FileMessage),
    LoadFile(Vec<Folder>),
}

impl Application for CollectiveGui {
    type Theme = Theme;
    type Flags = ();
    type Message = Message;
    type Executor = iced::executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::batch([Command::perform(get_file_list(), Message::LoadFile)]),
        )
    }
    
    fn title(&self) -> String {
        String::from("Collective")
    }

    fn update(&mut self, msg:Message) -> Command<Self::Message> {
        match msg {
            #[allow(clippy::option_if_let_else)]
            Message::LoadFile(file_list) => {
                self.selected_file = match &self.selected_file {
                    Some(s_file) => {
                        // try to reload last selected file
                        file_list.iter().cloned()
                    },
                    None => file_list.first().cloned(),
                };
                self.file_list = file_list;

                #[allow(unused_must_use)]
                {
                    self.update(Message::SettingsAction(SettingsMessage));
                }

                self.update(Message::TagsAction(TagsMessage::LoadTagList(true))}
            Message::TagsPressed => {
                self.view = View::Tags;
                Command::none()
            }
            Message::AboutPressed => {
                self.view = View::About;
                Command::none()
            }
            Message::SettingsPressed => {
                self.view = View::Settings;
                Command::none()
            }
        }
    }
}

impl CollectiveGui {}
