use crossterm::event::KeyCode;
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::popups::{Popup, PopupData, PopupRender};
use crate::state::AppEvent;

pub struct DoubleInput {
    description: String,
    value: String,
    input: String,
    list_state: ListState,
    cursor_pos: usize,
    input_focused: bool,
}

impl DoubleInput {
    pub fn new(description: &str, value: &str) -> Self {
        let list_state = ListState::default();
        // list_state.select(Some(0));

        Self {
            input: "".to_owned(),
            description: description.to_owned(),
            value: value.to_owned(),
            list_state,
            cursor_pos: 0,
            input_focused: false,
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

    fn next(&mut self) {
        if let Some(i) = self.list_state.selected() {
            match i {
                0 => self.list_state.select(Some(1)),
                1 => self.list_state.select(Some(0)),
                _ => unreachable!(),
            }
        } else {
            self.list_state.select(Some(0))
        }
    }

    fn prev(&mut self) {
        if let Some(i) = self.list_state.selected() {
            match i {
                0 => self.list_state.select(Some(1)),
                1 => self.list_state.select(Some(0)),
                _ => unreachable!(),
            }
        } else {
            self.list_state.select(Some(1))
        }
    }

    fn toggle_focus(&mut self) {
        self.input_focused = !self.input_focused
    }

    fn set_cursor_pos(&mut self) {
        self.cursor_pos = self.input.len();
    }
}

impl Popup for DoubleInput {
    fn handle_input(&mut self, key: &crossterm::event::KeyEvent) -> AppEvent {
        if !self.input_focused {
            match key.code {
                KeyCode::Esc => return AppEvent::ClosePopup,
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                KeyCode::Enter => {
                    if let Some(i) = self.list_state.selected() {
                        if i == 0 {
                            self.input = self.description.clone();
                        } else {
                            self.input = self.value.clone();
                        }
                        self.set_cursor_pos();
                        self.toggle_focus();
                    }
                }
                KeyCode::Char('w') => {
                    return AppEvent::ClosePopupData(PopupData::DoubleInput(
                        self.description.clone(),
                        self.value.clone(),
                    ))
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc => {
                    self.input.clear();
                    self.cursor_pos = 0;
                    self.toggle_focus();
                }
                KeyCode::Backspace => {
                    if self.cursor_pos > 0 {
                        self.input.remove(self.cursor_pos - 1);
                    }
                    self.decrement_cursor_pos();
                }
                KeyCode::Left => self.decrement_cursor_pos(),
                KeyCode::Right => self.increment_cursor_pos(),
                KeyCode::Enter => {
                    if let Some(i) = self.list_state.selected() {
                        if i == 0 {
                            self.description = self.input.clone();
                        } else {
                            self.value = self.input.clone();
                        }
                        self.input.clear();
                        self.cursor_pos = 0;
                        self.toggle_focus();
                    }
                }
                KeyCode::Char(c) => {
                    self.input.insert(self.cursor_pos, c);
                    self.increment_cursor_pos();
                }
                _ => {}
            }
        }
        AppEvent::None
    }

    fn get_widget(&self) -> PopupRender {
        let text_one = format!("┳ {}\n┗ {}\n", "Description", self.description);
        let text_two = format!("┳ {}\n┗ {}\n", "Value", self.value);
        let items = vec![ListItem::new(text_one), ListItem::new(text_two)]; // TODO - Add style from config

        let list = List::new(items)
            .block(
                Block::default().title("Popup One").borders(Borders::ALL), // .style(Style::default().fg(config.help_border()))
            )
            .highlight_style(Style::default().bg(Color::Red)); // TODO - Proper styling

        let input_block = Paragraph::new(Span::raw(&self.input)).block(
            Block::default().title("Popup One").borders(Borders::ALL), // .style(Style::default().fg(config.help_border()))
        );

        PopupRender::DoubleInput((list, input_block, self.list_state.clone(), self.cursor_pos))
    }
}