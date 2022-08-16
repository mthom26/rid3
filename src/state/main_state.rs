use std::{fs, path::PathBuf};

use crossterm::event::{KeyCode, KeyEvent};
use id3::{frame::ExtendedText, Content, Frame, Tag, TagLike, Version};
use log::{debug, warn};
use tui::widgets::ListState;

use crate::state::{frame_data::id_to_name, update_screen_state, AppEvent};
use crate::util;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Details,
    Edit,
    EditInput,
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

#[derive(Default)]
pub struct PopupState {
    pub state: ListState,
    // Currently each tuple in items contains text to identify the input
    // and the value of the input (text, value)
    pub items: Vec<(String, String)>,

    pub cursor_pos: usize,
    pub input: String,
}

impl PopupState {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => util::next(i, self.items.len()),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => util::prev(i, self.items.len()),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn increment_cursor_pos(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    fn decrement_cursor_pos(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn set_cursor_pos(&mut self) {
        self.cursor_pos = self.input.len();
    }

    fn set_input(&mut self) {
        match self.state.selected() {
            Some(i) => {
                self.items[i].1 = self.input.clone();
                self.input.clear();
                self.set_cursor_pos();
            }
            None => {}
        }
    }

    fn populate_input(&mut self) {
        if let Some(i) = self.state.selected() {
            self.input = self.items[i].1.clone();
            self.set_cursor_pos();
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
    pub cursor_pos: usize,

    pub popup: PopupState,
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
            cursor_pos: 0,
            popup: PopupState::default(),
        }
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        match self.focus {
            Focus::Edit => match key.code {
                KeyCode::Esc => self.switch_focus(),
                KeyCode::Up => self.popup.prev(),
                KeyCode::Down => self.popup.next(),
                KeyCode::Enter => self.switch_focus_popup_input(),
                KeyCode::Char('h') => return AppEvent::ToggleHelp,
                KeyCode::Char('w') => self.set_frame(),
                _ => {}
            },
            Focus::EditInput => match key.code {
                KeyCode::Char(c) => {
                    self.popup.input.insert(self.popup.cursor_pos, c);
                    self.popup.increment_cursor_pos();
                }
                KeyCode::Backspace => {
                    if self.popup.cursor_pos > 0 {
                        self.popup.input.remove(self.popup.cursor_pos - 1);
                    }
                    self.popup.decrement_cursor_pos();
                }
                KeyCode::Left => self.popup.decrement_cursor_pos(),
                KeyCode::Right => self.popup.increment_cursor_pos(),
                KeyCode::Enter => {
                    self.popup.set_input();
                    self.focus = Focus::Edit;
                }
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
                KeyCode::Enter => self.edit_frame(),
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
                new_details.push(frame.clone());
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
            Focus::Edit => self.focus = Focus::Details,
            Focus::EditInput => self.focus = Focus::Edit,
        }
    }

    fn switch_focus_popup_input(&mut self) {
        self.popup.populate_input();
        self.focus = Focus::EditInput;
    }

    fn set_frame(&mut self) {
        // If index is 0 here the filename is highlighted so handle
        // it and return early
        if let Some(0) = self.details_state.selected() {
            if let Some(i) = self.files_state.selected() {
                self.files[i].filename = self.popup.items[0].1.clone();
            }

            self.focus = Focus::Details;
            self.update_details();
            return;
        }

        // Here we are adding a new frame, but there can be multiple User defined text
        // frames in one tag. If a new frame is added with a new description the old frame
        // must be removed.
        let mut old_frame_description: Option<String> = None;

        // Create new frame
        let new_frame = match self.details_state.selected() {
            Some(i) => {
                let id = self.details[i - 1].id();
                // TODO check frame type by id and write appropriate data to frame
                match id {
                    "TXXX" => {
                        let previous_description = self.details[i - 1]
                            .content()
                            .extended_text()
                            .unwrap()
                            .description
                            .clone();
                        let description = self.popup.items[0].1.clone();
                        let value = self.popup.items[1].1.clone();

                        // Adding new description, need to delete old frame
                        if previous_description != description {
                            old_frame_description = Some(previous_description);
                        }

                        // Check for empty fields
                        if description.is_empty() || value.is_empty() {
                            warn!("User defined text frame contained an empty field, not adding new frame");
                            return;
                        }

                        let content = Content::ExtendedText(ExtendedText { description, value });
                        let new_frame = Frame::with_content(id, content);
                        self.details[i - 1] = new_frame.clone();
                        new_frame
                    }
                    _ => {
                        let text = self.popup.items[0].1.clone();
                        if text.is_empty() {
                            warn!("Text frame contained an empty field, not adding new frame");
                            return;
                        }

                        let new_frame = Frame::text(id, text);
                        self.details[i - 1] = new_frame.clone();
                        new_frame
                    }
                }
            }
            _ => unreachable!(),
        };

        // Update selected files
        for file in &mut self.files {
            if file.selected {
                file.tag.add_frame(new_frame.clone());
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i].tag.add_frame(new_frame);
        }

        // Remove old TXXX frame
        if old_frame_description.is_some() {
            for file in &mut self.files {
                if file.selected {
                    file.tag
                        .remove_extended_text(Some(&old_frame_description.clone().unwrap()), None);
                }
            }
            if let Some(i) = self.files_state.selected() {
                self.files[i]
                    .tag
                    .remove_extended_text(Some(&old_frame_description.unwrap()), None);
            }
        }

        self.focus = Focus::Details;
        self.update_details();
        self.next();
    }

    fn edit_frame(&mut self) {
        match self.focus {
            _ => {
                if let Some(i) = self.details_state.selected() {
                    if i == 0 {
                        // Filename selected spawn appropriate popup
                        let filename = self.details_filename.clone();
                        self.focus = Focus::Edit;
                        self.popup = PopupState {
                            items: vec![("Filename".to_string(), filename)],
                            ..Default::default()
                        };
                    } else {
                        match self.details[i - 1].id() {
                            // Spawn appropriate popup for frame
                            "TXXX" => {
                                let default_text = ExtendedText {
                                    description: "".to_string(),
                                    value: "".to_string(),
                                };
                                let text = self.details[i - 1]
                                    .content()
                                    .extended_text()
                                    .unwrap_or(&default_text);

                                self.focus = Focus::Edit;
                                self.popup = PopupState {
                                    items: vec![
                                        ("Description".to_string(), text.description.to_string()),
                                        ("Value".to_string(), text.value.to_string()),
                                    ],
                                    ..Default::default()
                                };
                            }
                            t if t.starts_with("T") => {
                                // Any frame that starts with `T` should only have one text field
                                // except for `TXXX`
                                let text = self.details[i - 1]
                                    .content()
                                    .text()
                                    .expect("Could not get frame text");
                                self.focus = Focus::Edit;
                                self.popup = PopupState {
                                    items: vec![("Text".to_string(), text.to_string())],
                                    ..Default::default()
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
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
        let frame = match id {
            "TXXX" => Frame::with_content(
                id,
                Content::ExtendedText(ExtendedText {
                    description: "Description".to_string(),
                    value: "Value".to_string(),
                }),
            ),
            _ => Frame::text(id, ""),
        };

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
            Focus::Details => {
                let i = match self.details_state.selected() {
                    Some(i) => util::next(i, self.details.len() + 1),
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
            // TODO - Handle ExtendedText
            _ => unimplemented!(),
        };
        frames.push((name, text));
    }

    frames
}
