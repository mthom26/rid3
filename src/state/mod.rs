use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use id3::Tag;

pub mod files_state;
pub mod main_state;

pub enum ScreenState {
    Main,
    Files,
}

pub enum AppEvent {
    None,
    Quit,
    AddFiles(Vec<(PathBuf, Tag)>),
}

// Get a Vec of (Path, Tags) from a Vec of DirEntrys
fn get_tags(entries: &Vec<DirEntry>) -> Result<Vec<(PathBuf, Tag)>, anyhow::Error> {
    let tags = entries
        .iter()
        .filter_map(|entry| match entry.path().is_dir() {
            false => Some((
                entry.path(),
                Tag::read_from_path(entry.path()).expect("Could not read Tag"),
            )),
            true => None,
        })
        .collect();

    Ok(tags)
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
