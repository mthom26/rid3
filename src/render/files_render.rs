use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{
    configuration::Config,
    logger::Logger,
    render::{
        basic, border, list_active, list_directory, render_logs, render_popup,
        window_title,
    },
    state::files_state::{FilesState, FilesStateItem},
};

pub fn files_render<B>(
    terminal: &mut Terminal<B>,
    log_state: &Logger,
    app_config: &Config,
    show_logs: bool,
    state: &mut FilesState,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let chunks = if show_logs {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(10)].as_ref())
                .split(size)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(0)].as_ref())
                .split(size)
        };

        let mut items = vec![];
        for entry in state.files.iter() {
            let (text, style) = match entry {
                FilesStateItem::DirEntry(entry) => (
                    entry
                        .file_name()
                        .into_string()
                        .expect("Could not parse OsString"),
                    if entry.file_type().unwrap().is_dir() {
                        list_directory(app_config)
                    } else {
                        basic(app_config)
                    },
                ),
                FilesStateItem::Parent => ("../".to_owned(), list_directory(app_config)),
            };

            items.push(ListItem::new(text).style(style));
        }

        let title = if let Some(s) = state.current_dir.to_str() {
            format!("Files - {}", s)
        } else {
            "Files".to_string()
        };

        let block = List::new(items)
            .block(
                Block::default()
                    .title(Span::styled(title, window_title(app_config)))
                    .style(border(app_config))
                    .borders(Borders::ALL),
            )
            .highlight_style(list_active(app_config));

        f.render_stateful_widget(block, chunks[0], &mut state.files_state);

        // Logs
        if show_logs {
            let log_block = render_logs(app_config, log_state);
            f.render_widget(log_block, chunks[1]);
        }

        // Popup
        if let Some(popup) = state.popup_widget() {
            render_popup(size, f, popup, app_config);
        }
    })?;

    Ok(())
}
