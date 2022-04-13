use tui::style::{Color, Modifier, Style};

pub mod files_render;
pub mod frames_render;
pub mod help_render;
pub mod logs_render;
pub mod main_render;

pub fn list_item() -> Style {
    Style::default().fg(Color::LightGreen)
}

// Style for currently focused selected list item
pub fn active_list_item() -> Style {
    Style::default()
        .fg(Color::LightYellow)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD)
}

// Style for unfocused selected list item
pub fn inactive_list_item() -> Style {
    Style::default()
        .fg(Color::LightGreen)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD)
}
