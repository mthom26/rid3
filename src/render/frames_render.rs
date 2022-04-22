use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};
use tui_logger::TuiWidgetState;

use crate::render::{
    help_render::render_help, inactive_list_item, list_item, logs_render::render_logs,
};
use crate::state::{frame_data::SUPPORTED_FRAMES, frames_state::FramesState};

const HELP_TEXT: [&str; 3] = [
    "Frames Help",
    "TODO",
    "Add hotkeys relevant to frame select screen",
];

pub fn render_frames<B>(
    terminal: &mut Terminal<B>,
    state: &mut FramesState,
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

        let chunks_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks[0]);

        let frames: Vec<ListItem> = SUPPORTED_FRAMES
            .iter()
            .map(|frame| ListItem::new(frame.name).style(list_item()))
            .collect();

        let frames_block = List::new(frames)
            .block(Block::default().title("Frames").borders(Borders::ALL))
            .highlight_style(inactive_list_item());

        // TODO - Add description and info for the highlighted frame
        let description_block = Block::default()
            .title("Frame Description")
            .borders(Borders::ALL);

        f.render_stateful_widget(frames_block, chunks_top[0], &mut state.frames_state);
        f.render_widget(description_block, chunks_top[1]);
        let mut log_widget = render_logs();
        log_widget.state(logger_state);
        f.render_widget(log_widget, chunks[1]);

        if show_help {
            render_help(f, &HELP_TEXT);
        }
    })?;

    Ok(())
}
