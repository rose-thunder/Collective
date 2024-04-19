use std::path::PathBuf;

use crate::app::utils::setup_collective_dir;

mod app;
mod gui;

static CONFIG_DIR: PathBuf = setup_collective_dir(dirs::config_dir());

static CACHE_DIR: PathBuf = setup_collective_dir(dirs::cache_dir());

fn main() -> iced::Result {
    gui::CollectiveGui::start()
}
