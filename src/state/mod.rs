use std::{
    fs::{self, DirEntry},
    path::Path,
};

use id3::Tag;

pub mod files_state;
pub mod frames_state;
pub mod main_state;
use main_state::Entry;

pub enum ScreenState {
    Main,
    Files,
    Frames,
}

pub enum AppEvent {
    None,
    Quit,
    AddFiles(Vec<Entry>),
    NewScreenState(ScreenState),
    ToggleHelp,
    HideHelp,
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

// Update ScreenState from any state
fn update_screen_state(c: char) -> AppEvent {
    match c {
        '1' => AppEvent::NewScreenState(ScreenState::Main),
        '2' => AppEvent::NewScreenState(ScreenState::Files),
        '3' => AppEvent::NewScreenState(ScreenState::Frames),
        _ => AppEvent::None,
    }
}
