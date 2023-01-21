use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    style::Style,
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{
    configuration::{actions::Action, Config},
    popups::{Popup, PopupData, PopupRender},
    state::AppEvent,
};

pub struct SingleInput {
    text: String,
    content: String,
    input: String,
    list_state: ListState,
    cursor_pos: usize,
}

impl SingleInput {
    // TODO - 'text' is currently the frame id, convert to readable name
    pub fn new(text: &str, input: &str) -> Self {
        let cursor_pos = input.len();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            text: text.to_owned(),
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
}

impl Popup for SingleInput {
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
                return AppEvent::ClosePopupData(PopupData::SingleInput(self.input.clone()))
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

        let list = List::new(items).block(Block::default().title("Frame").borders(Borders::ALL));

        let input_block = Paragraph::new(Span::raw(&self.input)).block(
            Block::default()
                .title("Input")
                .borders(Borders::ALL)
                .style(Style::default().fg(config.help_border())),
        );

        PopupRender::SingleInput((list, input_block, self.list_state.clone(), self.cursor_pos))
    }
}
