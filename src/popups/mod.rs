use crossterm::event::KeyEvent;
use tui::widgets::{List, ListState, Paragraph};

use crate::state::AppEvent;

pub mod double_input;
pub mod help;
pub mod single_input;

pub enum PopupRender<'a> {
    Help(List<'a>),
    SingleInput((List<'a>, Paragraph<'a>, ListState, usize)),
    DoubleInput((List<'a>, Paragraph<'a>, ListState, usize)),
}

pub enum PopupData {
    SingleInput(String),
    DoubleInput(String, String),
}

pub trait Popup {
    fn handle_input(&mut self, key: &KeyEvent) -> AppEvent;
    fn get_widget(&self) -> PopupRender; // TODO - Add `config` here for styling
}
