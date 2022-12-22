pub mod files_state;
pub mod frame_data;
pub mod frames_state;
pub mod main_state;
use main_state::Entry;

pub enum AppEvent {
    AddFiles(Vec<Entry>),
    AddFrame(&'static str),
    SwitchScreen(ScreenState),
    ClosePopup,
    Quit,
    None,
}

pub enum ScreenState {
    Main,
    Files,
    Frames,
}

fn update_screen_state(screen_state: ScreenState) -> AppEvent {
    AppEvent::SwitchScreen(screen_state)
}
