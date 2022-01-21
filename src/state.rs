use id3::{Tag, TagLike};
use tui::widgets::ListState;

use crate::util;

pub struct State {
    pub files_state: ListState,
    pub files: Vec<Tag>,

    pub details_state: ListState,
    pub details: Vec<String>,
}

impl State {
    pub fn new(initial_tags: Vec<Tag>) -> Self {
        Self {
            files_state: ListState::default(),
            files: initial_tags,
            details_state: ListState::default(),
            details: vec![
                "| Title\n| ".to_string(),
                "| Artist\n| ".to_string(),
                "| Album\n| ".to_string(),
            ],
        }
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

    pub fn next_file(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => util::next(i, self.files.len()),
            None => 0,
        };
        self.files_state.select(Some(i));
        self.update_details();
    }

    pub fn prev_file(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => util::prev(i, self.files.len()),
            None => 0,
        };
        self.files_state.select(Some(i));
        self.update_details();
    }

    pub fn next_detail(&mut self) {
        let i = match self.details_state.selected() {
            Some(i) => util::next(i, self.details.len()),
            None => 0,
        };
        self.details_state.select(Some(i));
    }

    pub fn prev_detail(&mut self) {
        let i = match self.details_state.selected() {
            Some(i) => util::prev(i, self.details.len()),
            None => 0,
        };
        self.details_state.select(Some(i));
    }
}
