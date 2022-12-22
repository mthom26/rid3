use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent};
use id3::{frame::ExtendedText, Content, Frame, Tag, TagLike};
use log::{info, warn};
use tui::widgets::ListState;

use crate::{
    popups::{help::HelpPopup, Popup},
    state::{update_screen_state, AppEvent, ScreenState},
    util,
};

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Details,
    // Edit,
    // EditInput,
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

pub enum DetailItem {
    FileName(String),
    Frame(Frame),
}

pub struct MainState {
    pub popup_stack: Vec<Box<dyn Popup>>,

    pub focus: Focus,

    pub files_state: ListState,
    pub files: Vec<Entry>,

    pub details_state: ListState,
    pub details: Vec<DetailItem>,
}

impl MainState {
    pub fn new() -> Self {
        let popup_stack: Vec<Box<dyn Popup>> = vec![];

        Self {
            popup_stack,
            focus: Focus::Files,
            files_state: ListState::default(),
            files: vec![],
            details_state: ListState::default(),
            details: vec![],
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
                KeyCode::Char('c') => self.remove_all_files(),
                KeyCode::Char('q') => return AppEvent::Quit,
                KeyCode::Char('s') => match self.focus {
                    Focus::Files => self.select_entry(),
                    _ => {}
                },
                KeyCode::Char('a') => match self.focus {
                    Focus::Files => self.select_all_entries(),
                    _ => {}
                },
                KeyCode::Char('d') => match self.focus {
                    Focus::Files => self.remove_files(),
                    Focus::Details => self.remove_frames(),
                },
                KeyCode::Char('h') => {
                    let help = Box::new(HelpPopup::new(
                        "Main Help".to_owned(),
                        vec!["Hello".to_owned(), "Main Help".to_owned()],
                    ));
                    self.popup_stack.push(help);
                }
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                KeyCode::Tab => self.switch_focus(),
                _ => {}
            }
        }
        AppEvent::None
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Files => {
                if self.files.is_empty() {
                    return;
                }
                let i = match self.files_state.selected() {
                    Some(i) => util::next(i, self.files.len()),
                    None => 0,
                };
                self.files_state.select(Some(i));
                self.update_details();
            }
            Focus::Details => {
                if self.details.is_empty() {
                    return;
                }
                let i = match self.details_state.selected() {
                    Some(i) => util::next(i, self.details.len()),
                    None => 0,
                };
                self.details_state.select(Some(i))
            }
        }
    }

    pub fn prev(&mut self) {
        match self.focus {
            Focus::Files => {
                if self.files.is_empty() {
                    return;
                }
                let i = match self.files_state.selected() {
                    Some(i) => util::prev(i, self.files.len()),
                    None => 0,
                };
                self.files_state.select(Some(i));
                self.update_details();
            }
            Focus::Details => {
                if self.details.is_empty() {
                    return;
                }
                let i = match self.details_state.selected() {
                    Some(i) => util::prev(i, self.details.len()),
                    None => 0,
                };
                self.details_state.select(Some(i));
            }
        }
    }

    // Add files from AppEvent::AddFiles(files)
    pub fn add_files(&mut self, files: Vec<Entry>) {
        'outer: for new_entry in files.iter() {
            for entry in self.files.iter() {
                if entry.path == new_entry.path {
                    warn!("Duplicate path");
                    continue 'outer;
                }
            }
            // TODO - remove this clone
            self.files.push(new_entry.clone());
        }
    }

    // Remove all files
    fn remove_all_files(&mut self) {
        self.files.clear();
        self.details.clear();
        self.files_state = ListState::default();
        self.details_state = ListState::default();
    }

    // Remove all selected files
    // TODO - Remove highlighted that is not selected but is highlighted
    fn remove_files(&mut self) {
        self.files = self
            .files
            .iter()
            .filter(|file| !file.selected)
            .map(|file| file.clone())
            .collect();
        // TODO - This causes a panic when all files are deleted
        self.update_details();
    }

    // Toggle selection of highlighted entry
    fn select_entry(&mut self) {
        match self.files_state.selected() {
            Some(i) => {
                self.files[i].selected = !self.files[i].selected;
                info!(
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
        info!("All selected: {}", any_unselected);
    }

    fn update_details(&mut self) {
        let index = match self.files_state.selected() {
            Some(i) => i,
            None => {
                warn!("files_state not selected");
                return;
            }
        };

        // TODO - Fix panic here when `remove_files` is called and files is zero length
        let file_name = self.files[index].filename.clone();
        let mut new_details = vec![DetailItem::FileName(file_name)];
        for frame in self.files[index].tag.frames() {
            // Only handle text frames
            if frame.id().starts_with("T") {
                new_details.push(DetailItem::Frame(frame.clone()));
            }
        }
        // TODO - Implement `Ord` for DetailItem and customise this sort
        // new_details.sort();
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
        }
    }

    pub fn add_frame(&mut self, id: &str) {
        info!("Adding frame {}", id);
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

    // Remove selected frame from all selected files
    fn remove_frames(&mut self) {
        let id = if let Some(i) = self.details_state.selected() {
            match &self.details[i] {
                DetailItem::FileName(_) => {
                    warn!("Not a frame");
                    return;
                }
                DetailItem::Frame(frame) => frame.id(),
            }
        } else {
            unreachable!();
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

    pub fn popup_widget(&self) -> Option<&Box<dyn Popup>> {
        self.popup_stack.last()
    }
}
