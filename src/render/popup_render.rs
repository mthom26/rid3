use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders},
    Frame,
};

use crate::config::Config;

pub fn get_block<'a, B>(frame: &mut Frame<B>, config: &Config) -> (Block<'a>, Block<'a>, Rect, Rect)
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

    let block_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(chunks_vertical[1]);

    (
        Block::default()
            .title("Popup One")
            .borders(Borders::ALL)
            .style(Style::default().fg(config.help_border())),
        Block::default()
            .title("Input")
            .borders(Borders::ALL)
            .style(Style::default().fg(config.help_border())),
        block_chunks[0],
        block_chunks[1],
    )
}
