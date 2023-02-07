pub mod files_state;
pub mod frame_data;
pub mod frames_state;
pub mod main_state;
use main_state::Entry;

use crate::popups::PopupData;

pub enum AppEvent {
    AddFiles(Vec<Entry>),
    AddFrame(&'static str),
    SwitchScreen(ScreenState),
    ClosePopupData(PopupData),
    UpdateConfig,
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
