use std::{env, fs};

use tui::widgets::ListState;

pub struct FilesState {
    pub files_state: ListState,
    pub files: Vec<fs::DirEntry>,
}

impl FilesState {
    pub fn new() -> Result<Self, anyhow::Error> {
        let current_dir = env::current_dir()?;
        let files: Vec<fs::DirEntry> = fs::read_dir(current_dir)?
            .map(|rdir| rdir.unwrap())
            .collect();

        Ok(FilesState {
            files_state: ListState::default(),
            files,
        })
    }
}
