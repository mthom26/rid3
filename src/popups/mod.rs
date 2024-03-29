use crossterm::event::KeyEvent;
use tui::widgets::{List, ListState, Paragraph};

use crate::{
    configuration::{actions::Action, Config},
    state::AppEvent,
};

pub mod double_input;
pub mod help;
pub mod single_input;
pub mod template;

pub enum PopupRender<'a> {
    Help(List<'a>),
    SingleInput((List<'a>, Paragraph<'a>, ListState, usize)),
    DoubleInput((List<'a>, Paragraph<'a>, ListState, usize)),
    TemplateInput((List<'a>, Paragraph<'a>, usize)),
}

pub enum PopupData {
    SingleInput(String),
    DoubleInput(String, String),
    TemplateInput(String),
}

pub enum PopupHelpType {
    DoubleInput,
}

pub trait Popup {
    fn handle_input(&mut self, key: &KeyEvent, action: Action) -> AppEvent;
    fn get_widget(&self, config: &Config) -> PopupRender;
}
