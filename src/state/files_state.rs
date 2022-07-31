use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use crossterm::event::{KeyCode, KeyEvent};
use id3::Tag;
use tui::widgets::ListState;

use crate::{
    state::{update_screen_state, AppEvent, Entry},
    util::{self, sort_files},
};

pub struct FilesState {
    pub current_dir: PathBuf,
    pub files_state: ListState,
    pub files: Vec<fs::DirEntry>,
}

impl FilesState {
    pub fn new() -> Result<Self, anyhow::Error> {
        let current_dir = env::current_dir()?;
        let mut files = get_entries(&current_dir)?;
        sort_files(&mut files);

        Ok(FilesState {
            current_dir,
            files_state: ListState::default(),
            files,
        })
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        match key.code {
            KeyCode::Char('q') => return AppEvent::Quit,
            KeyCode::Char('a') => return self.add_all_files().expect("Could not add files"),
            KeyCode::Char('s') => return self.add_file().expect("Could not add file"),
            KeyCode::Char('b') => self.parent_dir().expect("Could not enter parent directory"),
            KeyCode::Char('h') => return AppEvent::ToggleHelp,
            KeyCode::Char(c) => return update_screen_state(c),
            KeyCode::Up => self.prev(),
            KeyCode::Down => self.next(),
            KeyCode::Enter => {
                if let Some(i) = self.files_state.selected() {
                    if i == 0 {
                        self.parent_dir().expect("Could not enter parent directory");
                    } else {
                        self.enter_dir(i).expect("Could not enter directory");
                    }
                }
            }
            _ => {}
        }
        AppEvent::HideHelp // Hide help on user input
    }

    pub fn next(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => util::next(i, self.files.len() + 1),
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => util::prev(i, self.files.len() + 1),
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    fn enter_dir(&mut self, index: usize) -> Result<(), anyhow::Error> {
        let index = index - 1; // ListState has one more entry than the Vector of dir entries
        if self.files[index].file_type()?.is_dir() {
            let path = self.files[index].path();
            let files = get_entries(&path)?;

            self.current_dir = path;
            self.files = files;
            sort_files(&mut self.files);
            self.files_state = ListState::default();
            self.files_state.select(Some(0));
        }

        Ok(())
    }

    fn parent_dir(&mut self) -> Result<(), anyhow::Error> {
        match self.current_dir.parent() {
            Some(path) => {
                let files = get_entries(path)?;

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

    // Append selected file to MainState files
    fn add_file(&mut self) -> Result<AppEvent, anyhow::Error> {
        match self.files_state.selected().unwrap() {
            0 => Ok(AppEvent::None),
            i => {
                // `self.files` has one less item than `self.files_state` so
                // need to subtract one from index here
                let tag = get_tag(&self.files[i - 1])?;
                Ok(AppEvent::AddFiles(tag))
            }
        }
    }

    // Append all files to MainState files
    fn add_all_files(&mut self) -> Result<AppEvent, anyhow::Error> {
        let tags = get_tags(&self.files[..])?;
        Ok(AppEvent::AddFiles(tags))
    }
}

// Get a Vec of (Path, Tags) from a Vec of DirEntrys
fn get_tags(entries: &[DirEntry]) -> Result<Vec<Entry>, anyhow::Error> {
    let tags = entries
        .iter()
        .filter_map(|entry| match entry.path().is_dir() {
            false => Some(Entry::new(
                entry.path(),
                Tag::read_from_path(entry.path()).expect("Could not read Tag"),
            )),
            true => None,
        })
        .collect();

    Ok(tags)
}

// Get a Vec of (Path, Tags) from a DirEntry, returning a Vec here for convenience
fn get_tag(entry: &DirEntry) -> Result<Vec<Entry>, anyhow::Error> {
    if entry.path().is_dir() {
        Ok(vec![])
    } else {
        Ok(vec![Entry::new(
            entry.path(),
            Tag::read_from_path(entry.path()).expect("Could not read Tag"),
        )])
    }
}

// Get a Vec of DirEntrys from a Path, filters out everything except .mp3 and other directories
fn get_entries(path: &Path) -> Result<Vec<DirEntry>, anyhow::Error> {
    let files = fs::read_dir(&path)?
        .filter_map(|rdir| {
            let rdir = rdir.unwrap();
            if rdir.file_type().unwrap().is_dir() {
                return Some(rdir);
            } else if let Some(ext) = rdir.path().extension() {
                if ext.to_str() == Some("mp3") {
                    return Some(rdir);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        })
        .collect();

    Ok(files)
}
