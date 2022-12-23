use crossterm::event::KeyCode;
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::popups::{Popup, PopupRender};
use crate::state::AppEvent;

pub struct SingleInput {
    text: String,
    content: String,
    input: String,
    list_state: ListState,
    cursor_pos: usize,
}

impl SingleInput {
    pub fn new(input: &str) -> Self {
        let cursor_pos = input.len();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            text: "Text".to_owned(),
            input: input.to_owned(),
            content: input.to_owned(),
            list_state,
            cursor_pos,
        }
    }

    fn increment_cursor_pos(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    fn decrement_cursor_pos(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn set_cursor_pos(&mut self) {
        self.cursor_pos = self.input.len();
    }
}

impl Popup for SingleInput {
    fn handle_input(&mut self, key: &crossterm::event::KeyEvent) -> AppEvent {
        match key.code {
            KeyCode::Esc => AppEvent::ClosePopup,
            _ => AppEvent::None,
        }
    }

    fn get_widget(&self) -> PopupRender {
        let text = format!("┳ {}\n┗ {}\n", self.text, self.content);
        let items = vec![ListItem::new(text)]; // TODO - Add style from config

        let list = List::new(items)
            .block(
                Block::default().title("Popup One").borders(Borders::ALL), // .style(Style::default().fg(config.help_border()))
            )
            .highlight_style(Style::default().bg(Color::Red)); // TODO - Proper styling

        let input_block = Paragraph::new(Span::raw(&self.input)).block(
            Block::default().title("Popup One").borders(Borders::ALL), // .style(Style::default().fg(config.help_border()))
        );

        PopupRender::SingleInput((list, input_block, self.list_state.clone(), self.cursor_pos))
    }
}
