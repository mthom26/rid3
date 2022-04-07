use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent};
use id3::{Frame, Tag, TagLike, Version};
use log::debug;
use tui::widgets::ListState;

use crate::state::{update_screen_state, AppEvent};
use crate::util;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Details,
    Input,
}

pub struct Entry {
    pub path: PathBuf,
    pub tag: Tag,
    pub selected: bool,
}

impl Entry {
    pub fn new(path: PathBuf, tag: Tag) -> Self {
        Self {
            path,
            tag,
            selected: false,
        }
    }
}

// TODO
// - Check for duplicate Paths when adding new entries
pub struct MainState {
    pub focus: Focus,

    pub files_state: ListState,
    pub files: Vec<Entry>,

    pub details_state: ListState,
    // pub details: Vec<String>,
    pub details: Vec<Frame>,

    pub input: String,
}

impl MainState {
    pub fn new(initial_tags: Vec<Entry>) -> Self {
        Self {
            focus: Focus::Files,
            files_state: ListState::default(),
            files: initial_tags,
            details_state: ListState::default(),
            details: vec![
                // "| Title\n| ".to_string(),
                // "| Artist\n| ".to_string(),
                // "| Album\n| ".to_string(),
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
                KeyCode::Char('c') => self.clear_files(),
                KeyCode::Char('w') => self.write_tags().expect("Could not write tags"),
                KeyCode::Char('h') => return AppEvent::ToggleHelp,
                KeyCode::Char('s') => {
                    if self.focus == Focus::Files {
                        self.select_entry()
                    }
                }
                KeyCode::Char('a') => {
                    if self.focus == Focus::Files {
                        self.select_all_entries()
                    }
                }
                KeyCode::Char(c) => return update_screen_state(c),
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                KeyCode::Tab => self.switch_focus(),
                KeyCode::Enter => self.switch_input(),
                _ => {}
            },
        }
        AppEvent::HideHelp // Hide help on user input
    }

    fn update_details(&mut self) {
        let index = self.files_state.selected().unwrap(); // This shouldn't fail right?
        let mut new_details = vec![];
        for frame in self.files[index].tag.frames() {
            // Only handle text frames
            if frame.id().starts_with("T") {
                // Don't handle user defined text frames
                if frame.id() != "TXXX" {
                    new_details.push(frame.clone());
                }
            }
        }
        self.details = new_details;
        // Check old `details_state` isn't referring to an index outside `new_details` length
        if let Some(i) = self.details_state.selected() {
            if self.details.len() - 1 < i {
                self.details_state.select(Some(0));
            }
        }
    }

    fn switch_focus(&mut self) {
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

    fn switch_input(&mut self) {
        match self.focus {
            Focus::Input => {}
            _ => {
                self.focus = Focus::Input;
            }
        }
    }

    fn set_input(&mut self) {
        let new_frame = match self.details_state.selected() {
            Some(i) => {
                let id = self.details[i].id();
                let new_frame = Frame::text(id, &self.input);
                self.details[i] = new_frame.clone();
                new_frame
            }
            _ => unreachable!(),
        };

        for file in &mut self.files {
            if file.selected {
                file.tag.add_frame(new_frame.clone());
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i].tag.add_frame(new_frame);
        }

        self.input = "".to_string();
        self.focus = Focus::Details;
        self.update_details();
    }

    fn next(&mut self) {
        if self.files.is_empty() {
            return;
        }
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

    fn prev(&mut self) {
        if self.files.is_empty() {
            return;
        }
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

    fn clear_files(&mut self) {
        self.files.clear();
        self.details.clear();
        self.files_state = ListState::default();
        self.details_state = ListState::default();
    }

    pub fn add_files(&mut self, files: &mut Vec<Entry>) {
        self.files.append(files);
    }

    // Write updated tags to files
    fn write_tags(&mut self) -> Result<(), anyhow::Error> {
        for entry in self.files.iter() {
            entry.tag.write_to_path(&entry.path, Version::Id3v24)?;
        }

        Ok(())
    }

    // Toggle selection of highlighted entry
    fn select_entry(&mut self) {
        match self.files_state.selected() {
            Some(i) => {
                self.files[i].selected = !self.files[i].selected;
                debug!(
                    "{:?} selected: {}",
                    self.files[i].path, self.files[i].selected
                );
            }
            None => {}
        }
    }

    // Select all files in list, if all are already selected then deselect them
    fn select_all_entries(&mut self) {
        let mut any_unselected = false;
        for entry in &self.files {
            if !entry.selected {
                any_unselected = true;
                break;
            }
        }
        for entry in &mut self.files {
            entry.selected = any_unselected;
        }
        debug!("All selected: {}", any_unselected);
    }
}
