use crossterm::event::{KeyCode, KeyEvent};
use log::warn;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::state::AppEvent;

pub mod double_input;
pub mod help;
pub mod single_input;

pub enum PopupRender<'a> {
    One(List<'a>),
    Two(Block<'a>),
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

#[derive(Clone, Debug)]
pub struct TestPopupOne {
    pub data: Vec<String>,
}

impl Popup for TestPopupOne {
    fn handle_input(&mut self, _key: &KeyEvent) -> AppEvent {
        warn!("Popup 1");
        AppEvent::None
    }

    fn get_widget(&self) -> PopupRender {
        let items: Vec<ListItem> = self
            .data
            .iter()
            .map(|item| ListItem::new(item.clone()))
            .collect();

        PopupRender::One(
            List::new(items).block(Block::default().title("Popup 1").borders(Borders::ALL)),
        )
    }
}

#[derive(Clone, Debug)]
pub struct TestPopupTwo {
    pub hello: String,
}

impl Popup for TestPopupTwo {
    fn handle_input(&mut self, key: &KeyEvent) -> AppEvent {
        warn!("Popup 2");
        match key.code {
            KeyCode::Char(c) => {
                self.hello.push(c);
                AppEvent::None
            }
            KeyCode::Enter => AppEvent::ClosePopup,
            _ => AppEvent::None,
        }
    }

    fn get_widget(&self) -> PopupRender {
        let title = format!("Popup 2: {}", self.hello);
        PopupRender::Two(Block::default().title(title).borders(Borders::ALL))
    }
}
