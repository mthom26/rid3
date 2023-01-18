use crossterm::event::KeyEvent;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::{
    configuration::actions::Action,
    popups::{Popup, PopupRender},
    state::AppEvent,
};

pub struct HelpPopup {
    title: String,
    data: Vec<String>,
}

impl HelpPopup {
    pub fn new(title: String, data: Vec<String>) -> Self {
        Self { title, data }
    }
}

impl Popup for HelpPopup {
    fn handle_input(&mut self, _key: &KeyEvent, _action: Action) -> AppEvent {
        // Just close the help popup on any input
        AppEvent::ClosePopup
    }

    fn get_widget(&self) -> PopupRender {
        let items: Vec<ListItem> = self
            .data
            .iter()
            .map(|item| ListItem::new(item.clone()))
            .collect();

        PopupRender::Help(
            List::new(items).block(
                Block::default()
                    .title(self.title.clone())
                    .borders(Borders::ALL),
            ),
        )
    }
}
