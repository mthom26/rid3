use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    terminal::Frame,
    widgets::Clear,
};

use crate::{
    configuration::Config,
    popups::{Popup, PopupRender},
};

pub mod files_render;
pub mod frames_render;
mod logs;
pub mod main_render;
use logs::render_logs;

pub fn render_popup<B>(size: Rect, f: &mut Frame<B>, popup: &Box<dyn Popup>, config: &Config)
where
    B: Backend,
{
    let chunks_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    let chunks_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(chunks_horizontal[1]);

    f.render_widget(Clear, chunks_vertical[1]);

    let w = popup.get_widget(config);
    match w {
        PopupRender::Help(help) => {
            f.render_widget(help, chunks_vertical[1]);
        }
        PopupRender::SingleInput((list, input, mut state, cursor_pos)) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(chunks_vertical[1]);

            f.render_stateful_widget(list, chunks[0], &mut state);
            f.render_widget(input, chunks[1]);
            f.set_cursor(chunks[1].x + cursor_pos as u16 + 1, chunks[1].y + 1);
        }
        PopupRender::DoubleInput((list, input, mut state, cursor_pos)) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(chunks_vertical[1]);

            f.render_stateful_widget(list, chunks[0], &mut state);
            f.render_widget(input, chunks[1]);
            f.set_cursor(chunks[1].x + cursor_pos as u16 + 1, chunks[1].y + 1);
        }
        PopupRender::TemplateInput((list, input, cursor_pos)) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(chunks_vertical[1]);

            f.render_widget(list, chunks[0]);
            f.render_widget(input, chunks[1]);
            f.set_cursor(chunks[1].x + cursor_pos as u16 + 1, chunks[1].y + 1);
        }
    }
}

// Styles
pub fn basic(config: &Config) -> Style {
    Style::default().fg(config.basic_fg()).bg(config.basic_bg())
}

pub fn border(config: &Config) -> Style {
    Style::default().fg(config.window_border())
}

pub fn active_border(config: &Config) -> Style {
    Style::default().fg(config.active_border())
}

pub fn window_title(config: &Config) -> Style {
    Style::default().fg(config.window_title())
}

pub fn active_window_title(config: &Config) -> Style {
    Style::default().fg(config.active_window_title())
}

pub fn list_highlighted(config: &Config) -> Style {
    Style::default()
        .fg(config.list_highlighted_fg())
        .bg(config.list_highlighted_bg())
        .add_modifier(Modifier::BOLD)
}

pub fn list_active(config: &Config) -> Style {
    Style::default()
        .fg(config.list_active_fg())
        .bg(config.list_active_bg())
        .add_modifier(Modifier::BOLD)
}

pub fn list_directory(config: &Config) -> Style {
    Style::default().fg(config.list_directory())
}
