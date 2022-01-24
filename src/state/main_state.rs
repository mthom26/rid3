use crossterm::event::{KeyCode, KeyEvent};
use id3::{Tag, TagLike};
use tui::widgets::ListState;

use crate::state::AppEvent;
use crate::util;

pub enum Focus {
    Files,
    Details,
    Input,
}

pub struct MainState {
    pub focus: Focus,

    pub files_state: ListState,
    pub files: Vec<Tag>,

    pub details_state: ListState,
    pub details: Vec<String>,

    pub input: String,
}

impl MainState {
    pub fn new(initial_tags: Vec<Tag>) -> Self {
        Self {
            focus: Focus::Files,
            files_state: ListState::default(),
            files: initial_tags,
            details_state: ListState::default(),
            details: vec![
                "| Title\n| ".to_string(),
                "| Artist\n| ".to_string(),
                "| Album\n| ".to_string(),
            ],
            input: "".to_string(),
        }
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        match self.focus {
            Focus::Input => match key.code {
                KeyCode::Char(c) => self.input.push(c),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => self.set_input(),
                KeyCode::Esc => self.switch_focus(),
                _ => {}
            },
            _ => match key.code {
                KeyCode::Char('q') => return AppEvent::Quit,
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                KeyCode::Tab => self.switch_focus(),
                KeyCode::Enter => self.switch_input(),
                _ => {}
            },
        }
        AppEvent::None
    }

    fn update_details(&mut self) {
        let index = self.files_state.selected().unwrap(); // This shouldn't fail right?

        let title = match self.files[index].title() {
            Some(t) => format!("| Title\n| {}", t),
            None => "!No title!".to_string(),
        };
        let artist = match self.files[index].artist() {
            Some(t) => format!("| Artist\n| {}", t),
            None => "!No artist!".to_string(),
        };
        let album = match self.files[index].album() {
            Some(t) => format!("| Album\n| {}", t),
            None => "!No album!".to_string(),
        };

        self.details = vec![title, artist, album];
    }

    pub fn switch_focus(&mut self) {
        match self.focus {
            Focus::Files => {
                // On switching focus to the details list for the first time
                // select the first item
                if self.details_state.selected() == None {
                    self.details_state.select(Some(0));
                }
                self.focus = Focus::Details
            }
            Focus::Details => self.focus = Focus::Files,
            Focus::Input => self.focus = Focus::Files,
        }
    }

    pub fn switch_input(&mut self) {
        match self.focus {
            Focus::Input => {}
            _ => {
                self.focus = Focus::Input;
            }
        }
    }

    pub fn set_input(&mut self) {
        match self.files_state.selected() {
            Some(i) => {
                match self.details_state.selected() {
                    // 0 - Title, 1 - Artist, 2 - Album, this needs to be improved...
                    Some(0) => self.files[i].set_title(&self.input),
                    Some(1) => self.files[i].set_artist(&self.input),
                    Some(2) => self.files[i].set_album(&self.input),
                    _ => {}
                }
                self.input = "".to_string();
                self.focus = Focus::Details;
                self.update_details();
            }
            None => {}
        }
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Files => {
                let i = match self.files_state.selected() {
                    Some(i) => util::next(i, self.files.len()),
                    None => 0,
                };
                self.files_state.select(Some(i));
                self.update_details();
            }
            Focus::Details => {
                let i = match self.details_state.selected() {
                    Some(i) => util::next(i, self.details.len()),
                    None => 0,
                };
                self.details_state.select(Some(i));
            }
            _ => {}
        }
    }

    pub fn prev(&mut self) {
        match self.focus {
            Focus::Files => {
                let i = match self.files_state.selected() {
                    Some(i) => util::prev(i, self.files.len()),
                    None => 0,
                };
                self.files_state.select(Some(i));
                self.update_details();
            }
            Focus::Details => {
                let i = match self.details_state.selected() {
                    Some(i) => util::prev(i, self.details.len()),
                    None => 0,
                };
                self.details_state.select(Some(i));
            }
            _ => {}
        }
    }
}
