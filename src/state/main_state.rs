use std::{fs, path::PathBuf};

use crossterm::event::{KeyCode, KeyEvent};
use id3::{Content, Frame, Tag, TagLike, Version};
use log::{debug, warn};
use tui::widgets::ListState;

use crate::state::{frame_data::id_to_name, update_screen_state, AppEvent};
use crate::util;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Details,
    Input,
}

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub filename: String,
    pub tag: Tag,
    pub selected: bool,
}

impl Entry {
    pub fn new(path: PathBuf, tag: Tag) -> Self {
        let filename = path
            .to_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .to_string();

        Self {
            path,
            filename,
            tag,
            selected: false,
        }
    }
}

pub struct MainState {
    pub focus: Focus,

    pub files_state: ListState,
    pub files: Vec<Entry>,

    // TODO - `details_state` is longer than `details` so when setting input,
    //        removing frames, etc. this has to be taken into account when indexing
    //        into the relevant vectors. This is cumbersome and if changed again
    //        will need to be fixed. Should be a better way to do it...
    pub details_filename: String,
    pub details_state: ListState,
    pub details: Vec<Frame>,

    pub input: String,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            focus: Focus::Files,
            files_state: ListState::default(),
            files: vec![],
            details_filename: String::new(),
            details_state: ListState::default(),
            details: vec![],
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
                KeyCode::Char('c') => self.remove_all_files(),
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
                KeyCode::Char('d') => {
                    if self.focus == Focus::Details {
                        self.remove_frame();
                    } else if self.focus == Focus::Files {
                        self.remove_files();
                    }
                }
                KeyCode::Char('f') => return update_screen_state('3'),
                KeyCode::Char('u') => self.update_filenames(),
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
        let index = match self.files_state.selected() {
            Some(i) => i,
            None => {
                debug!("files_state not selected");
                return;
            }
        };

        self.details_filename = self.files[index].filename.clone();

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
        // TODO - Customise this sort
        new_details.sort();
        self.details = new_details;

        // Check old `details_state` isn't referring to an index outside `new_details` length
        if let Some(i) = self.details_state.selected() {
            if self.details.len() < i {
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
                // Populate input field
                if let Some(i) = self.details_state.selected() {
                    if i == 0 {
                        self.input = self.details_filename.clone();
                    } else {
                        self.input = match self.details[i - 1].content().text() {
                            Some(s) => s.to_string(),
                            None => "".to_string(),
                        };
                    }
                }
                self.focus = Focus::Input;
            }
        }
    }

    fn set_input(&mut self) {
        // If index is 0 here the filename is highlighted so handle
        // it and return early
        if let Some(0) = self.details_state.selected() {
            if let Some(i) = self.files_state.selected() {
                self.files[i].filename = self.input.clone();
            }

            self.input = "".to_string();
            self.focus = Focus::Details;
            self.update_details();
            return;
        }

        let new_frame = match self.details_state.selected() {
            Some(i) => {
                let id = self.details[i - 1].id();
                let new_frame = Frame::text(id, &self.input);
                self.details[i - 1] = new_frame.clone();
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
        self.next();
    }

    // Remove selected frame from all selected files
    fn remove_frame(&mut self) {
        let id = match self.details_state.selected() {
            Some(0) => {
                debug!("Not a frame");
                return;
            }
            Some(i) => self.details[i - 1].id(),
            _ => unreachable!(),
        };

        for file in &mut self.files {
            if file.selected {
                file.tag.remove(id);
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i].tag.remove(id);
        }

        self.update_details();
    }

    pub fn add_frame(&mut self, id: &str) {
        debug!("Adding frame {}", id);
        let frame = Frame::text(id, "");

        for entry in self.files.iter_mut() {
            if entry.selected {
                // TODO - Check if frame already exists, if it does
                //        don't overwrite existing content
                entry.tag.add_frame(frame.clone());
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i].tag.add_frame(frame);
        }

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
            // When input is focused allow next details item to be selected
            Focus::Details | Focus::Input => {
                let i = match self.details_state.selected() {
                    Some(i) => util::next(i, self.details.len() + 1),
                    None => 0,
                };
                self.details_state.select(Some(i));
            }
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
                    Some(i) => util::prev(i, self.details.len() + 1),
                    None => 0,
                };
                self.details_state.select(Some(i));
            }
            _ => {}
        }
    }

    fn remove_all_files(&mut self) {
        self.files.clear();
        self.details.clear();
        self.files_state = ListState::default();
        self.details_state = ListState::default();
    }

    // Remove all selected files
    fn remove_files(&mut self) {
        self.files = self
            .files
            .iter()
            .filter(|file| !file.selected)
            .map(|file| file.clone())
            .collect();
    }

    pub fn add_files(&mut self, files: &mut Vec<Entry>) {
        'outer: for new_entry in files.iter() {
            for entry in self.files.iter() {
                if entry.path == new_entry.path {
                    warn!("Duplicate path");
                    continue 'outer;
                }
            }
            // info!("Adding entry");
            self.files.push(new_entry.clone());
        }
    }

    // Write updated tags to files
    fn write_tags(&mut self) -> Result<(), anyhow::Error> {
        for entry in self.files.iter_mut() {
            entry.tag.write_to_path(&entry.path, Version::Id3v24)?;

            // Rename the file, for now the extension must be included
            // when the user enters the new filename
            let mut new_path = entry.path.clone();
            new_path.set_file_name(&entry.filename);
            fs::rename(&entry.path, &new_path)?;
            entry.path = new_path;
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

    // Temporary function to update the entry filename with the track number
    // and title
    fn update_filenames(&mut self) {
        for entry in self.files.iter_mut() {
            if entry.selected {
                let frames = get_frames(&entry.tag);
                let mut track_num = "".to_string();
                let mut title = "".to_string();

                for (name, content) in frames.iter() {
                    match &name[..] {
                        "Title" => title = content.clone(),
                        "Track" => track_num = content.clone(),
                        _ => {}
                    }
                }

                let track_num = if track_num.len() < 2 {
                    // Add a leading zero if necessary
                    format!("0{}", track_num)
                } else {
                    track_num
                };

                if track_num.is_empty() {
                    debug!("Track number frame was empty, not renaming file");
                    continue;
                }
                if title.is_empty() {
                    debug!("Title frame was empty, not renaming file");
                    continue;
                }

                let new_filename = format!("{} {}.mp3", track_num, title);
                entry.filename = new_filename;
            }
        }
        self.update_details();
    }
}

fn get_frames(tag: &Tag) -> Vec<(String, String)> {
    let mut frames = vec![];
    for frame in tag.frames() {
        let name = if let Ok(name) = id_to_name(frame.id()) {
            name
        } else {
            debug!("Skipping frame");
            continue;
        };

        let text: String = match frame.content() {
            Content::Text(txt) => txt.clone(),
            _ => unreachable!(),
        };
        frames.push((name, text));
    }

    frames
}
