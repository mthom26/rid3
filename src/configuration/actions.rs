use std::collections::HashMap;

use crossterm::event::KeyCode as XTermKeyCode;
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Hash, PartialEq, Eq)]
struct KeyCode(XTermKeyCode);

impl<'de> Deserialize<'de> for KeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyCodeVisitor)
    }
}

struct KeyCodeVisitor;

impl<'de> Visitor<'de> for KeyCodeVisitor {
    type Value = KeyCode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("KeyCodeVisitor Error")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "Up" => Ok(KeyCode(XTermKeyCode::Up)),
            "Down" => Ok(KeyCode(XTermKeyCode::Down)),
            "Left" => Ok(KeyCode(XTermKeyCode::Left)),
            "Right" => Ok(KeyCode(XTermKeyCode::Right)),
            "Esc" => Ok(KeyCode(XTermKeyCode::Esc)),
            "Tab" => Ok(KeyCode(XTermKeyCode::Tab)),
            "Backspace" => Ok(KeyCode(XTermKeyCode::Backspace)),
            "Enter" => Ok(KeyCode(XTermKeyCode::Enter)),
            "Home" => Ok(KeyCode(XTermKeyCode::Home)),
            "End" => Ok(KeyCode(XTermKeyCode::End)),
            "PageUp" => Ok(KeyCode(XTermKeyCode::PageUp)),
            "PageDown" => Ok(KeyCode(XTermKeyCode::PageDown)),
            "BackTab" => Ok(KeyCode(XTermKeyCode::BackTab)),
            "Delete" => Ok(KeyCode(XTermKeyCode::Delete)),
            "Insert" => Ok(KeyCode(XTermKeyCode::Insert)),
            c if c.len() == 1 => Ok(KeyCode(XTermKeyCode::Char(c.chars().next().unwrap()))),
            // TODO - Handle F1-F12 keys
            k => Err(serde::de::Error::custom(format!("Invalid KeyCode `{}`", k))),
        }
    }
}

// Currently different states use the same KeyCode for different actions. Either merge these actions
// into one in the General Actions section (then the user would only be able to rebind all of them to
// the same key) or have separate keys for these actions even where one key for multiple actions
// would be convenient.
//
// TODO - Let one KeyCode map to multiple actions
#[derive(Debug)]
enum Action {
    // General Actions
    Prev,
    Next,
    Quit, // Quit the application
    Back, // Exit current context (popup, input, etc...) without saving
    SwitchFocus,
    ToggleLogs,
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
            a => Err(serde::de::Error::custom(format!("Invalid action `{}`", a))),
        }
    }
}

#[derive(Debug)]
pub struct ActionMap(HashMap<KeyCode, Action>);

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
