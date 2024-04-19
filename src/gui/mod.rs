pub mod style;
pub mod views;
pub mod widgets;

use iced::{Alignment, Application, Command, Element, Length, Renderer, Theme};

#[derive(Default, Debug, Clone)]
enum View {
    #[default]
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
    SettingsPressed,
    TagsPressed,
    FilePressed,
    TagsAction(TagsMessage),
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

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
            #[allow(clippy::option_if_let_else)]
            Message::LoadFile(file_list) => {
                self.selected_file = match &self.selected_file {
                    Some(s_file) => {
                        // try to reload last selected file
                        file_list.iter().cloned()
                    }
                    None => file_list.first().cloned(),
                };
                self.file_list = file_list;

                #[allow(unused_must_use)]
                {
                    self.update(Message::SettingsAction(SettingsMessage));
                }

                self.update(Message::TagsAction(TagsMessage::LoadTagList(true)))
            }
            Message::TagsPressed => {
                self.view = View::Tags;
                Command::none()
            }
            Message::SettingsPressed => {
                self.view = View::Settings;
                Command::none()
            }
            Message::FilePressed => {
                self.view = View::File;
                Command::none()
            }
            Message::TagsAction(msg) => self
                .tags_view
                .update(&mut self.settings_view, msg)
                .map(Message::TagsAction),
            Message::SettingsAction(msg) => self
                .settings_view
                .update(&mut self.settings_view, msg)
                .map(Message::SettingsAction),
            Message::FileAction(msg) => self
                .file_view
                .update(&mut self.settings_view, msg)
                .map(Message::FileAction),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let navigation_container =
            nav_menu(&self.file_list, self.selected_file.clone(), &self.tags_view);

        let selected_file = self.selected_file.clone().unwrap_or_default();
        let main_container = match self.view {
            View::Tags => self
                .tags_view
                .view(&self.settings_view, &selected_file)
                .map(Message::TagsAction),
            View::Settings => self.settings_view.view().map(Message::SettingsAction),
            View::File => self.file_view.view(&selected_file).map(Message::FileAction),
        };

        column![navigation_container, main_container]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}

impl CollectiveGui {}
