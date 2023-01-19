use std::collections::HashMap;

use crossterm::event::KeyCode;
use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub struct AppKeyCode(pub KeyCode);

impl<'de> Deserialize<'de> for AppKeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(AppKeyCodeVisitor)
    }
}

struct AppKeyCodeVisitor;

impl<'de> Visitor<'de> for AppKeyCodeVisitor {
    type Value = AppKeyCode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("KeyCodeVisitor Error")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "Up" => Ok(AppKeyCode(KeyCode::Up)),
            "Down" => Ok(AppKeyCode(KeyCode::Down)),
            "Left" => Ok(AppKeyCode(KeyCode::Left)),
            "Right" => Ok(AppKeyCode(KeyCode::Right)),
            "Esc" => Ok(AppKeyCode(KeyCode::Esc)),
            "Tab" => Ok(AppKeyCode(KeyCode::Tab)),
            "Backspace" => Ok(AppKeyCode(KeyCode::Backspace)),
            "Enter" => Ok(AppKeyCode(KeyCode::Enter)),
            "Home" => Ok(AppKeyCode(KeyCode::Home)),
            "End" => Ok(AppKeyCode(KeyCode::End)),
            "PageUp" => Ok(AppKeyCode(KeyCode::PageUp)),
            "PageDown" => Ok(AppKeyCode(KeyCode::PageDown)),
            "BackTab" => Ok(AppKeyCode(KeyCode::BackTab)),
            "Delete" => Ok(AppKeyCode(KeyCode::Delete)),
            "Insert" => Ok(AppKeyCode(KeyCode::Insert)),
            c if c.len() == 1 => Ok(AppKeyCode(KeyCode::Char(c.chars().next().unwrap()))),
            // TODO - Handle F1-F12 keys
            k => Err(serde::de::Error::custom(format!("Invalid KeyCode `{}`", k))),
        }
    }
}

// TODO - Currently multiple actions can be assigned to one KeyCode. The user
//        can assign the same KeyCode for an action in the `General Actions`
//        section and one of the other `State Actions` sections which would
//        introduce the potential for returning two or more relevant actions
//        from one key press causing unpredictable behaviour in the app.
//        Need to check for these conflicts when building the config.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Action {
    // General Actions
    Prev,
    Next,
    Quit, // Quit the application
    Back, // Exit current context (popup, input, etc...) without saving
    SwitchFocus,
    ToggleLogs,
    LogsPrev,
    LogsNext,
    ScreenOne,
    ScreenTwo,
    ScreenThree,
    Help, // Get help popup

    // MainState Actions
    RemoveFiles,   // Remove files
    WriteTags,     // Save new tags to files
    SelectCurrent, // Select currently highlighted item
    SelectAll,     // Select all highlighted items
    Remove,        // Remove currently selected files/frames
    SpawnPopup,

    // FilesState Actions
    AddAllFiles, // Add all files
    AddFile,     // Add highlighted file
    ParentDir,   // Move to parent directory
    EnterDir,

    // FramesState Actions
    AddFrame,

    // Popup Actions
    SelectField, // Select the highlighted frame field
    SaveChanges, // Save changes to the frame

    // A variant for no action here is easier than using an
    // Option<Action> elsewhere in the app
    None,
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ActionVisitor)
    }
}

struct ActionVisitor;

impl<'de> Visitor<'de> for ActionVisitor {
    type Value = Action;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("ActionVisitor Error")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            // General Actions
            "up" => Ok(Action::Prev),
            "down" => Ok(Action::Next),
            "quit" => Ok(Action::Quit),
            "back" => Ok(Action::Back),
            "switch_focus" => Ok(Action::SwitchFocus),
            "toggle_logs" => Ok(Action::ToggleLogs),
            "logs_prev" => Ok(Action::LogsPrev),
            "logs_next" => Ok(Action::LogsNext),
            "screen_one" => Ok(Action::ScreenOne),
            "screen_two" => Ok(Action::ScreenTwo),
            "screen_three" => Ok(Action::ScreenThree),
            "help" => Ok(Action::Help),
            // MainState Actions
            "remove_files" => Ok(Action::RemoveFiles),
            "write_tags" => Ok(Action::WriteTags),
            "select_current" => Ok(Action::SelectCurrent),
            "select_all" => Ok(Action::SelectAll),
            "remove" => Ok(Action::Remove),
            "spawn_popup" => Ok(Action::SpawnPopup),
            // FilesState Actions
            "add_file" => Ok(Action::AddFile),
            "add_all_files" => Ok(Action::AddAllFiles),
            "parent_directory" => Ok(Action::ParentDir),
            "enter_directory" => Ok(Action::EnterDir),
            // FramesState Actions
            "add_frame" => Ok(Action::AddFrame),
            // Popup Actions
            "select_field" => Ok(Action::SelectField),
            "save_changes" => Ok(Action::SaveChanges),
            a => Err(serde::de::Error::custom(format!("Invalid action `{}`", a))),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ActionMap(pub HashMap<Action, AppKeyCode>);

// This custom deserialization is not needed as we are no longer
// trying to convert the ActionMap to HashMap<KeyCode, Vec<Action>>
// in here. It is done later outside of serde.
/*
impl<'de> Deserialize<'de> for ActionMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ActionMapVisitor)
    }
}

struct ActionMapVisitor;

impl<'de> Visitor<'de> for ActionMapVisitor {
    type Value = ActionMap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("ActionMapVisitor Error")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut values = HashMap::new();

        while let Some((key, value)) = map.next_entry()? {
            // Note the value and key are switched here because we want
            // the `config.toml` entries to read as `Action: Keycode`
            // but the HashMap should have a key of `KeyCode` and value
            // of `ActionMap`
            values.insert(value, key);
        }

        Ok(ActionMap(values))
    }
}
*/
