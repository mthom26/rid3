use std::{env, fs, path::PathBuf};

use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::ListState;

use crate::state::AppEvent;
use crate::util;

pub struct FilesState {
    pub current_dir: PathBuf,
    pub files_state: ListState,
    pub files: Vec<fs::DirEntry>,
}

impl FilesState {
    pub fn new() -> Result<Self, anyhow::Error> {
        let current_dir = env::current_dir()?;
        let files: Vec<fs::DirEntry> = fs::read_dir(&current_dir)?
            .map(|rdir| rdir.unwrap())
            .collect();

        Ok(FilesState {
            current_dir,
            files_state: ListState::default(),
            files,
        })
    }

    pub fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        match key.code {
            KeyCode::Char('q') => return AppEvent::Quit,
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

        AppEvent::None
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

            let files = fs::read_dir(&path)?.map(|rdir| rdir.unwrap()).collect();

            self.current_dir = path;
            self.files = files;
            self.files_state = ListState::default();
            self.files_state.select(Some(0));
        }

        Ok(())
    }

    fn parent_dir(&mut self) -> Result<(), anyhow::Error> {
        match self.current_dir.parent() {
            Some(p) => {
                let files: Vec<fs::DirEntry> = fs::read_dir(p)?.map(|rdir| rdir.unwrap()).collect();

                self.current_dir = p.to_owned();
                self.files = files;
                self.files_state = ListState::default();
                self.files_state.select(Some(0));
            }
            None => { /* Must be at root */ }
        }

        Ok(())
    }

    // Append selected file to MainState files
    fn add_file() {
        // TODO
    }

    // Append all files to MainState files
    fn add_all_files() {
        // TODO
    }
}
