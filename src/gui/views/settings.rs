use crate::gui::style;

#[derive(Debug, Clone)]
pub struct Settings {
    pub general: GeneralSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: Config::load_config_file().general,
        }
    }
}
