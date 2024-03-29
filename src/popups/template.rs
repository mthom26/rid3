use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{
    configuration::{actions::Action, Config},
    popups::{Popup, PopupData, PopupRender},
    render::{active_border, active_window_title, basic, border, window_title},
    state::AppEvent,
};

pub struct TemplateInput {
    text: String,
    content: String,
    input: String,
    cursor_pos: usize,
}

impl TemplateInput {
    pub fn new(input: &str) -> Self {
        let cursor_pos = input.len();

        Self {
            text: "Text".to_owned(),
            input: input.to_owned(),
            content: input.to_owned(),
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
}

impl Popup for TemplateInput {
    fn handle_input(&mut self, key: &KeyEvent, _action: Action) -> AppEvent {
        match key.code {
            KeyCode::Esc => return AppEvent::ClosePopup,
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.input.remove(self.cursor_pos - 1);
                }
                self.decrement_cursor_pos();
            }
            KeyCode::Left => self.decrement_cursor_pos(),
            KeyCode::Right => self.increment_cursor_pos(),
            KeyCode::Enter => {
                return AppEvent::ClosePopupData(PopupData::TemplateInput(self.input.clone()))
            }
            KeyCode::Char(c) => {
                self.input.insert(self.cursor_pos, c);
                self.increment_cursor_pos();
            }
            _ => {}
        }
        AppEvent::None
    }

    fn get_widget(&self, config: &Config) -> PopupRender {
        let text = format!("┳ {}\n┗ {}\n", self.text, self.content);
        let items = vec![ListItem::new(text)];

        let list = List::new(items)
            .block(
                Block::default()
                    .title(Span::styled("Template", window_title(config)))
                    .style(border(config))
                    .borders(Borders::ALL),
            )
            .style(basic(config));

        let input_block = Paragraph::new(Span::styled(&self.input, basic(config))).block(
            Block::default()
                .title(Span::styled("Input", active_window_title(config)))
                .style(active_border(config))
                .borders(Borders::ALL),
        );

        PopupRender::TemplateInput((list, input_block, self.cursor_pos))
    }
}
