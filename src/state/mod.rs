use id3::Tag;

pub mod files_state;
pub mod main_state;

pub enum ScreenState {
    Main,
    Files,
}

pub enum AppEvent {
    None,
    Quit,
    AddFiles(Vec<Tag>),
}
