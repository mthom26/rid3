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
            "Tab" => Ok(KeyCode(XTermKeyCode::Tab)),
            c if c.len() == 1 => Ok(KeyCode(XTermKeyCode::Char(c.chars().next().unwrap()))),
            _ => Err(serde::de::Error::custom("Invalid KeyCode")),
        }
    }
}

#[derive(Debug)]
enum Action {
    Up,
    Down,
    Quit,
    Back,
    SwitchFocus,
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
            "up" => Ok(Action::Up),
            "down" => Ok(Action::Down),
            "quit" => Ok(Action::Quit),
            "switch_focus" => Ok(Action::SwitchFocus),
            _ => Err(serde::de::Error::custom("Invalid action")),
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

        while let Some((key, value)) = map.next_entry().unwrap() {
            // Note the value and key are switched here because we want
            // the `config.toml` entries to read as `Action: Keycode`
            // but the HashMap should have a key of `KeyCode` and value
            // of `ActionMap`
            values.insert(value, key);
        }

        Ok(ActionMap(values))
    }
}
