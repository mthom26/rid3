use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::ListState;

use crate::{
    popups::{help::HelpPopup, Popup},
    state::{frame_data::SUPPORTED_FRAMES, update_screen_state, AppEvent, ScreenState},
    util,
};

pub struct FramesState {
    pub popup_stack: Vec<Box<dyn Popup>>,

    pub frames_state: ListState,
}

impl FramesState {
    pub fn new() -> Self {
        let mut frames_state = ListState::default();
        frames_state.select(Some(0));

        Self {
            popup_stack: vec![],
            frames_state,
        }
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        if let Some(popup) = self.popup_stack.last_mut() {
            match popup.handle_input(key) {
                AppEvent::ClosePopup => {
                    // Need to return relevant data from popup here, probably another enum...
                    let _p = self.popup_stack.pop().unwrap();
                }
                AppEvent::SwitchScreen(s) => return update_screen_state(s),
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('1') => return update_screen_state(ScreenState::Main),
                KeyCode::Char('2') => return update_screen_state(ScreenState::Files),
                KeyCode::Char('3') => return update_screen_state(ScreenState::Frames),
                KeyCode::Char('q') => return AppEvent::Quit,
                // KeyCode::Char('h') => return AppEvent::ToggleHelp,
                KeyCode::Char('a') => return AppEvent::AddFrame(self.frame_id()),
                KeyCode::Char('h') => {
                    let help = Box::new(HelpPopup::new(
                        "Frames Help".to_owned(),
                        vec!["Hello".to_owned(), "Frames Help".to_owned()],
                    ));
                    self.popup_stack.push(help);
                }
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                _ => {}
            }
        }
        AppEvent::None
    }

    pub fn next(&mut self) {
        let i = match self.frames_state.selected() {
            Some(i) => util::next(i, SUPPORTED_FRAMES.len()),
            None => 0,
        };
        self.frames_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.frames_state.selected() {
            Some(i) => util::prev(i, SUPPORTED_FRAMES.len()),
            None => 0,
        };
        self.frames_state.select(Some(i));
    }

    fn frame_id(&self) -> &'static str {
        match self.frames_state.selected() {
            Some(i) => SUPPORTED_FRAMES[i].id,
            None => unreachable!(),
        }
    }

    pub fn popup_widget(&self) -> Option<&Box<dyn Popup>> {
        self.popup_stack.last()
    }
}
