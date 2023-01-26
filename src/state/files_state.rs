use std::{
    cmp::Ordering,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use crossterm::event::KeyEvent;
use id3::Tag;
use log::{error, warn};
use tui::widgets::ListState;

use crate::{
    configuration::{actions::Action, Config},
    popups::{help::HelpPopup, Popup},
    state::{main_state::Entry, update_screen_state, AppEvent, ScreenState},
    util, LOGGER,
};

pub enum FilesStateItem {
    Parent,
    DirEntry(DirEntry),
}

pub struct FilesState {
    pub popup_stack: Vec<Box<dyn Popup>>,

    pub current_dir: PathBuf,
    pub files_state: ListState,
    pub files: Vec<FilesStateItem>,

    pub show_hidden_dirs: bool,

    help_text: Vec<String>,
}

impl FilesState {
    pub fn new(dir: PathBuf) -> Result<Self, anyhow::Error> {
        let mut files: Vec<FilesStateItem> = get_entries(&dir, false)?;
        sort_files(&mut files);

        let popup_stack: Vec<Box<dyn Popup>> = vec![];

        Ok(Self {
            popup_stack,
            current_dir: dir,
            files,
            files_state: ListState::default(),
            show_hidden_dirs: false,
            help_text: vec![],
        })
    }

    pub fn handle_input(
        &mut self,
        key: &KeyEvent,
        actions: &Vec<Action>,
        show_logs: &mut bool,
    ) -> AppEvent {
        let action = if actions.len() == 1 {
            actions[0]
        } else {
            let mut action = Action::None;
            for a in actions.iter() {
                if *a == Action::AddAllFiles
                    || *a == Action::AddFile
                    || *a == Action::ParentDir
                    || *a == Action::EnterDir
                    || *a == Action::HiddenDir
                {
                    action = *a;
                    break;
                }
            }
            action
        };

        if let Some(popup) = self.popup_stack.last_mut() {
            match popup.handle_input(key, action) {
                AppEvent::ClosePopup => {
                    // Need to return relevant data from popup here, probably another enum...
                    let _p = self.popup_stack.pop().unwrap();
                }
                AppEvent::SwitchScreen(s) => return update_screen_state(s),
                _ => {}
            }
        } else {
            match action {
                Action::Quit => return AppEvent::Quit,
                Action::ScreenOne => return update_screen_state(ScreenState::Main),
                Action::ScreenTwo => return update_screen_state(ScreenState::Files),
                Action::ScreenThree => return update_screen_state(ScreenState::Frames),
                Action::ToggleLogs => *show_logs = !*show_logs,
                Action::LogsPrev => LOGGER.prev(),
                Action::LogsNext => LOGGER.next(),
                Action::Help => self.spawn_help_popup(),
                Action::Prev => self.prev(),
                Action::Next => self.next(),
                Action::AddAllFiles => return self.add_all_files().expect("Could not add files"),
                Action::AddFile => return self.add_file().expect("Could not add file"),
                Action::ParentDir => self.parent_dir().expect("Could not enter parent directory"),
                Action::EnterDir => {
                    if let Some(i) = self.files_state.selected() {
                        if i == 0 {
                            self.parent_dir().expect("Could not enter parent directory");
                        } else {
                            if let Err(e) = self.enter_dir(i) {
                                warn!("{}", e);
                            }
                        }
                    }
                }
                Action::HiddenDir => {
                    self.show_hidden_dirs = !self.show_hidden_dirs;
                    if let Err(e) = self.refresh_dir() {
                        warn!("{}", e);
                    }
                }
                _ => {}
            }
        }
        AppEvent::None
    }

    pub fn next(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => util::next(i, self.files.len()),
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => util::prev(i, self.files.len()),
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    fn enter_dir(&mut self, index: usize) -> Result<(), anyhow::Error> {
        match &self.files[index] {
            FilesStateItem::DirEntry(entry) => {
                if entry.file_type()?.is_dir() {
                    let path = entry.path();
                    let files = get_entries(&path, self.show_hidden_dirs)?;

                    self.current_dir = path;
                    self.files = files;
                    sort_files(&mut self.files);
                    self.files_state = ListState::default();
                    self.files_state.select(Some(0));
                } else {
                    warn!("Not a directory!");
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn parent_dir(&mut self) -> Result<(), anyhow::Error> {
        match self.current_dir.parent() {
            Some(path) => {
                let files = get_entries(path, self.show_hidden_dirs)?;

                self.current_dir = path.to_path_buf();
                self.files = files;
                sort_files(&mut self.files);
                self.files_state = ListState::default();
                self.files_state.select(Some(0));
            }
            None => { /* Must be at root */ }
        }

        Ok(())
    }

    // Rebuild files list of current directory
    fn refresh_dir(&mut self) -> Result<(), anyhow::Error> {
        let files = get_entries(&self.current_dir, self.show_hidden_dirs)?;
        self.files = files;
        sort_files(&mut self.files);
        self.files_state = ListState::default();
        self.files_state.select(Some(0));
        Ok(())
    }

    // Append selected file to MainState files
    fn add_file(&mut self) -> Result<AppEvent, anyhow::Error> {
        match self.files_state.selected().unwrap() {
            0 => Ok(AppEvent::None),
            i => {
                let tag = get_tags(&self.files[i..i + 1])?;
                Ok(AppEvent::AddFiles(tag))
            }
        }
    }

    // Append all files to MainState files
    fn add_all_files(&mut self) -> Result<AppEvent, anyhow::Error> {
        let tags = get_tags(&self.files[1..])?;
        Ok(AppEvent::AddFiles(tags))
    }

    pub fn popup_widget(&self) -> Option<&Box<dyn Popup>> {
        self.popup_stack.last()
    }

    pub fn spawn_help_popup(&mut self) {
        self.popup_stack.push(Box::new(HelpPopup::new(
            "Files Help".to_owned(),
            self.help_text.clone(),
        )));
    }

    pub fn update_help_text(&mut self, config: &Config) {
        let quit = config.get_key(&Action::Quit).unwrap();
        let parent_dir = config.get_key(&Action::ParentDir).unwrap();
        let add = config.get_key(&Action::AddFile).unwrap();
        let add_all = config.get_key(&Action::AddAllFiles).unwrap();

        self.help_text = vec![
            format!("`{}` - Quit", util::display_keycode(quit)),
            format!(
                "`{}` - Change to parent directory",
                util::display_keycode(parent_dir)
            ),
            format!("`{}` - Add highlighted file", util::display_keycode(add)),
            format!("`{}` - Add all files", util::display_keycode(add_all)),
        ];
    }
}

// Get a Vec of (Path, Tags) from a Vec of DirEntrys
fn get_tags(entries: &[FilesStateItem]) -> Result<Vec<Entry>, anyhow::Error> {
    let tags = entries
        .iter()
        .filter_map(|entry| match entry {
            FilesStateItem::DirEntry(entry) => match entry.path().is_dir() {
                false => {
                    let tag = match Tag::read_from_path(entry.path()) {
                        Ok(tag) => tag,
                        Err(id3::Error {
                            kind: id3::ErrorKind::NoTag,
                            ..
                        }) => {
                            warn!("File has no id3 tag, adding empty tag");
                            Tag::new()
                        }
                        Err(e) => {
                            error!("Failed to add file - {}", e);
                            return None;
                        }
                    };
                    Some(Entry::new(entry.path(), tag))
                }
                true => None,
            },
            FilesStateItem::Parent => unreachable!(),
        })
        .collect();

    Ok(tags)
}

// Get a Vec<FilesStateItem> from a Path, filters out everything except .mp3 and other directories
fn get_entries(path: &Path, show_hidden_dirs: bool) -> Result<Vec<FilesStateItem>, anyhow::Error> {
    let mut files = vec![FilesStateItem::Parent]; // Add `../` item
    let mut entries = fs::read_dir(&path)?
        .filter_map(|rdir| {
            let rdir = rdir.unwrap();
            if rdir.file_type().unwrap().is_dir() {
                if rdir.file_name().to_str().unwrap().starts_with(".") && !show_hidden_dirs {
                    return None;
                } else {
                    return Some(FilesStateItem::DirEntry(rdir));
                }
            } else if let Some(ext) = rdir.path().extension() {
                if ext.to_str() == Some("mp3") {
                    return Some(FilesStateItem::DirEntry(rdir));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        })
        .collect();

    files.append(&mut entries);

    Ok(files)
}

// Sort a list of `FilesStateItem`, directories first then by filename
pub fn sort_files(files: &mut Vec<FilesStateItem>) {
    files.sort_by(|a, b| {
        match (a, b) {
            // First item should always be `../`
            // Apparently `b` here is the first item in the Vec not `a`
            (_, FilesStateItem::Parent) => return Ordering::Greater,
            (FilesStateItem::DirEntry(a), FilesStateItem::DirEntry(b)) => {
                match (
                    a.file_type().unwrap().is_dir(),
                    b.file_type().unwrap().is_dir(),
                ) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => a
                        .file_name()
                        .to_ascii_lowercase()
                        .cmp(&b.file_name().to_ascii_lowercase()),
                }
            }
            _ => unreachable!(),
        }
    });
}
