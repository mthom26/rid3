use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

use crate::config::Config;

pub fn render_help<B>(frame: &mut Frame<B>, items: &[&str], config: &Config)
where
    B: Backend,
{
    let size = frame.size();

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

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(*item).style(Style::default().fg(config.list_item_fg())))
        .collect();

    let block = List::new(list_items)
        .block(Block::default().title("Help").borders(Borders::ALL))
        .style(Style::default().fg(config.help_border()))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(Clear, chunks_vertical[1]);
    frame.render_widget(block, chunks_vertical[1]);
}
