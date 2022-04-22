use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};
use tui_logger::TuiWidgetState;

use crate::render::{help_render::render_help, inactive_list_item, logs_render::render_logs};
use crate::state::files_state::FilesState;

const HELP_TEXT: [&str; 3] = ["Files Help", "TODO", "Add hotkeys relevant to files screen"];

pub fn render_files<B>(
    terminal: &mut Terminal<B>,
    state: &mut FilesState,
    show_help: bool,
    logger_state: &TuiWidgetState,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(10)].as_ref())
            .split(size);

        let mut items = vec![ListItem::new("../").style(Style::default().fg(Color::LightGreen))];
        for entry in state.files.iter() {
            let text = entry
                .file_name()
                .into_string()
                .expect("Could not parse OsString");

            let style = if entry.file_type().unwrap().is_dir() {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default().fg(Color::LightGreen)
            };

            items.push(ListItem::new(text).style(style));
        }

        let block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(inactive_list_item());

        f.render_stateful_widget(block, chunks[0], &mut state.files_state);

        let mut log_widget = render_logs();
        log_widget.state(logger_state);
        f.render_widget(log_widget, chunks[1]);

        if show_help {
            render_help(f, &HELP_TEXT);
        }
    })?;

    Ok(())
}
