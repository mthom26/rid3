use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Terminal,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::render::{
    active_list_item, help_render::render_help, inactive_list_item, logs_render::render_logs,
};
use crate::state::main_state::{Focus, MainState};

const HELP_TEXT: [&str; 3] = ["Main Help", "TODO", "Add hotkeys relevant to main screen"];

pub fn render_main<B>(
    terminal: &mut Terminal<B>,
    state: &mut MainState,
    show_help: bool,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let c = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(10)].as_ref())
            .split(size);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(c[0]);

        let items: Vec<ListItem> = state
            .files
            .iter()
            .map(|item| {
                let text = match item.0.to_str() {
                    Some(t) => t
                        .split("/")
                        .last()
                        .unwrap_or("!Problem unwrapping filename!"),
                    None => "!Unknown Artist!",
                };
                ListItem::new(text).style(Style::default().fg(Color::LightGreen))
            })
            .collect();

        let left_block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Files => active_list_item(),
                _ => inactive_list_item(),
            });

        f.render_stateful_widget(left_block, chunks[0], &mut state.files_state);

        let chunks_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(chunks[1]);

        let right_items: Vec<ListItem> = state
            .details
            .iter()
            .map(|item| {
                let text = format!("| {}\n| {}\n", item.name(), item.content());
                ListItem::new(text).style(Style::default().fg(Color::LightGreen))
            })
            .collect();
        let right_block = List::new(right_items)
            .block(Block::default().title("Details").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Details => active_list_item(),
                _ => inactive_list_item(),
            });
        f.render_stateful_widget(right_block, chunks_right[0], &mut state.details_state);

        let text = Span::raw(&state.input);
        let input_block =
            Paragraph::new(text).block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_block, chunks_right[1]);

        f.render_widget(render_logs(), c[1]);

        // Render cursor
        match state.focus {
            Focus::Input => {
                f.set_cursor(
                    chunks_right[1].x + state.input.len() as u16 + 1,
                    chunks_right[1].y + 1,
                );
            }
            _ => {}
        }

        if show_help {
            render_help(f, &HELP_TEXT);
        }
    })?;

    Ok(())
}
