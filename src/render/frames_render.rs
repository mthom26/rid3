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
        basic, border, list_active, render_logs, render_popup, window_title,
    },
    state::{frame_data::SUPPORTED_FRAMES, frames_state::FramesState},
};

pub fn frames_render<B>(
    terminal: &mut Terminal<B>,
    log_state: &Logger,
    app_config: &Config,
    show_logs: bool,
    state: &mut FramesState,
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

        // Frames list
        let frames: Vec<ListItem> = SUPPORTED_FRAMES
            .iter()
            .map(|frame| ListItem::new(frame.name).style(basic(app_config)))
            .collect();

        let block = List::new(frames)
            .block(
                Block::default()
                    .title(Span::styled("Frames", window_title(app_config)))
                    .style(border(app_config))
                    .borders(Borders::ALL),
            )
            .highlight_style(list_active(app_config));

        f.render_stateful_widget(block, chunks[0], &mut state.frames_state);

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
