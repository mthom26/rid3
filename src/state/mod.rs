pub mod files_state;
pub mod frame_data;
pub mod frames_state;
pub mod main_state;
use main_state::Entry;

pub enum ScreenState {
    Main,
    Files,
    Frames,
}

pub enum AppEvent {
    None,
    Quit,
    AddFiles(Vec<Entry>),
    NewScreenState(ScreenState),
    ToggleHelp,
    HideHelp,
    AddFrame(&'static str),
}

// Update ScreenState from any state
fn update_screen_state(c: char) -> AppEvent {
    match c {
        '1' => AppEvent::NewScreenState(ScreenState::Main),
        '2' => AppEvent::NewScreenState(ScreenState::Files),
        '3' => AppEvent::NewScreenState(ScreenState::Frames),
        _ => AppEvent::None,
    }
}
