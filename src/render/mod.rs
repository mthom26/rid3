use tui::style::{Modifier, Style};

use crate::config::Config;

pub mod files_render;
pub mod frames_render;
pub mod help_render;
pub mod logs_render;
pub mod main_render;

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
