use id3::TagLike;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::Terminal,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::state::{
    files_state::FilesState,
    main_state::{Focus, MainState},
};

pub fn render_main<B>(
    terminal: &mut Terminal<B>,
    state: &mut MainState,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(size);

        let items: Vec<ListItem> = state
            .files
            .iter()
            .map(|item| {
                let text = match item.1.title() {
                    Some(t) => t,
                    None => "!Unknown Artist!",
                };
                ListItem::new(text).style(Style::default().fg(Color::LightGreen))
            })
            .collect();

        let left_block = List::new(items)
            .block(Block::default().title("Left").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(left_block, chunks[0], &mut state.files_state);

        let chunks_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(chunks[1]);

        let right_items: Vec<ListItem> = state
            .details
            .iter()
            .map(|item| ListItem::new(item.clone()).style(Style::default().fg(Color::LightGreen)))
            .collect();
        let right_block = List::new(right_items)
            .block(Block::default().title("Left").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(right_block, chunks_right[0], &mut state.details_state);

        let text = Span::raw(&state.input);
        let input_block =
            Paragraph::new(text).block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_block, chunks_right[1]);

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
    })?;

    Ok(())
}

pub fn render_files<B>(
    terminal: &mut Terminal<B>,
    state: &mut FilesState,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let mut items = vec![ListItem::new("../").style(Style::default().fg(Color::LightGreen))];
        for entry in state.files.iter() {
            let text = entry
                .file_name()
                .into_string()
                .expect("Could not parse OsString");

            items.push(ListItem::new(text).style(Style::default().fg(Color::LightGreen)));
        }

        let block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(block, size, &mut state.files_state);
    })?;

    Ok(())
}
