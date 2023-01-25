use crossterm::event::KeyEvent;
use tui::{
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{
    configuration::{actions::Action, Config},
    popups::{Popup, PopupRender},
    render::{basic, border, window_title},
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

    fn get_widget(&self, config: &Config) -> PopupRender {
        let items: Vec<ListItem> = self
            .data
            .iter()
            .map(|item| ListItem::new(item.clone()))
            .collect();

        PopupRender::Help(
            List::new(items)
                .block(
                    Block::default()
                        .title(Span::styled(self.title.clone(), window_title(config)))
                        .style(border(config))
                        .borders(Borders::ALL),
                )
                .style(basic(config)),
        )
    }
}
