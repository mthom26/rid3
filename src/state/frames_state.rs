use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::ListState;

use crate::{
    state::{frame_data::SUPPORTED_FRAMES, update_screen_state, AppEvent},
    util,
};

pub struct FramesState {
    pub frames_state: ListState,
}

impl FramesState {
    pub fn new() -> Self {
        let mut frames_state = ListState::default();
        frames_state.select(Some(0));

        Self { frames_state }
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        match key.code {
            KeyCode::Char('q') => return AppEvent::Quit,
            KeyCode::Char('h') => return AppEvent::ToggleHelp,
            KeyCode::Char('a') => return AppEvent::AddFrame(self.frame_id()),
            KeyCode::Char(c) => return update_screen_state(c),
            KeyCode::Up => self.prev(),
            KeyCode::Down => self.next(),
            _ => {}
        }
        AppEvent::HideHelp
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
}
