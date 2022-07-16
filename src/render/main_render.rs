use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use tui_logger::TuiWidgetState;

use crate::config::Config;
use crate::render::{
    active_list_item, help_render::render_help, inactive_list_item, list_item,
    logs_render::render_logs,
};
use crate::state::main_state::{Focus, MainState};

const HELP_TEXT: [&str; 7] = [
    "`q` - Quit",
    "`c` - Remove all files",
    "`d` - Remove selected files/frame",
    "`s` - Select highlighted file",
    "`a` - Select/Deselect all files",
    "`u` - Update file names",
    "`w` - Write changes",
];

pub fn render_main<B>(
    terminal: &mut Terminal<B>,
    state: &mut MainState,
    show_help: bool,
    logger_state: &TuiWidgetState,
    config: &Config,
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
                let text = match item.path.to_str() {
                    Some(t) => t
                        .split("/")
                        .last()
                        .unwrap_or("!Problem unwrapping filename!"),
                    None => "!Unknown Artist!",
                };
                ListItem::new(text).style(match item.selected {
                    true => inactive_list_item(config),
                    false => list_item(config),
                })
            })
            .collect();

        let left_block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Files => active_list_item(config),
                _ => inactive_list_item(config),
            });

        f.render_stateful_widget(left_block, chunks[0], &mut state.files_state);

        let chunks_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(chunks[1]);

        let filename = format!("┳ Filename\n┗ {}\n", state.details_filename);
        let mut right_items = vec![ListItem::new(filename).style(list_item(config))];

        for item in state.details.iter().map(|item| {
            let text = format!("┳ {}\n┗ {}\n", item.name(), item.content());
            ListItem::new(text).style(list_item(config))
        }) {
            right_items.push(item);
        }

        let right_block = List::new(right_items)
            .block(Block::default().title("Details").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Details => active_list_item(config),
                _ => inactive_list_item(config),
            });
        f.render_stateful_widget(right_block, chunks_right[0], &mut state.details_state);

        let text = Span::raw(&state.input);
        let input_block =
            Paragraph::new(text).block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_block, chunks_right[1]);

        let mut log_widget = render_logs(config);
        log_widget.state(logger_state);
        f.render_widget(log_widget, c[1]);

        // Render cursor
        match state.focus {
            Focus::Input => {
                f.set_cursor(
                    chunks_right[1].x + state.cursor_pos as u16 + 1,
                    chunks_right[1].y + 1,
                );
            }
            _ => {}
        }

        if show_help {
            render_help(f, &HELP_TEXT, config);
        }
    })?;

    Ok(())
}
