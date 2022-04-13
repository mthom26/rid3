use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::ListState;

use crate::{
    state::{update_screen_state, AppEvent},
    util,
};

pub struct FramesState {
    pub frames_state: ListState,
    pub frames: Vec<&'static str>,
}

impl FramesState {
    pub fn new() -> Self {
        let frames = vec!["Title", "Artist", "Album", "Track", "Date"];
        Self {
            frames_state: ListState::default(),
            frames,
        }
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        match key.code {
            KeyCode::Char('q') => return AppEvent::Quit,
            KeyCode::Char('h') => return AppEvent::ToggleHelp,
            KeyCode::Char(c) => return update_screen_state(c),
            KeyCode::Up => self.prev(),
            KeyCode::Down => self.next(),
            _ => {}
        }
        AppEvent::HideHelp
    }

    pub fn next(&mut self) {
        let i = match self.frames_state.selected() {
            Some(i) => util::next(i, self.frames.len()),
            None => 0,
        };
        self.frames_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.frames_state.selected() {
            Some(i) => util::prev(i, self.frames.len()),
            None => 0,
        };
        self.frames_state.select(Some(i));
    }
}
