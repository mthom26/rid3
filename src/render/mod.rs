use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    terminal::Frame,
    widgets::Clear,
};

use crate::{
    config::Config,
    popups::{Popup, PopupRender},
};

pub mod files_render;
pub mod frames_render;
mod logs;
pub mod main_render;
use logs::render_logs;

pub fn render_popup<B>(size: Rect, f: &mut Frame<B>, popup: &Box<dyn Popup>)
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

    let w = popup.get_widget();
    match w {
        PopupRender::One(list) => {
            f.render_widget(list, chunks_vertical[1]);
        }
        PopupRender::Two(block) => {
            f.render_widget(block, chunks_vertical[1]);
        }
        PopupRender::Help(help) => {
            f.render_widget(help, chunks_vertical[1]);
        }
    }
}

// Style for list item
pub fn list_item(config: &Config) -> Style {
    Style::default()
        .fg(config.list_item_fg())
        .bg(config.list_item_bg())
}

// Style for currently focused selected list item
pub fn active_list_item(config: &Config) -> Style {
    Style::default()
        .fg(config.active_list_item_fg())
        .bg(config.active_list_item_bg())
        .add_modifier(Modifier::BOLD)
}

// Style for unfocused selected list item
pub fn inactive_list_item(config: &Config) -> Style {
    Style::default()
        .fg(config.inactive_list_item_fg())
        .bg(config.inactive_list_item_bg())
        .add_modifier(Modifier::BOLD)
}
